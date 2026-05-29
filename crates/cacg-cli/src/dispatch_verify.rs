//! `kb verify` dispatch module.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use cacg_cli::round_summary::{
    verify_round_summary, RoundSummaryResult, Verdict, VerifyRoundSummaryError,
};
use cacg_cli::VerifyArgs;
use cacg_core::diagnostic::{codes, Diagnostic, Severity};
use cacg_core::retraction::RetractionSpec;
use cacg_core::source_matrix::AuthSpec;
use cacg_core::verify::{verify_one_card, SemanticEvaluator};
use cacg_semantic::SemanticCache;

use crate::dispatch_lint::emit_lint_diagnostics;

/// Translate `--semantic <path>` / `--semantic-judge` CLI args into
/// an optional boxed [`SemanticEvaluator`]. Mirrors Python
/// `_build_semantic_spec` (`legacy_python_oracle/src/cacg/cli.py:843-883`).
///
/// Returns:
/// * `Ok(None)` when neither flag was supplied (default verify path).
/// * `Ok(Some(boxed))` when `--semantic <path>` loads a B1 cache,
///   OR when `--semantic-judge` (only under
///   `--features b2-llm-judge`) constructs a B2 LLM-judge adapter
///   over an Anthropic `HaikuClient`.
/// * `Err(ExitCode)` after emitting `CACG-MAN-001` on a missing /
///   malformed `--semantic` cache file.
///
/// For `--semantic-judge`, the function does NOT validate
/// `ANTHROPIC_API_KEY` at this point — a missing or empty key,
/// a transport timeout, a malformed Anthropic response, and any
/// other judge-time `JudgeError` surface at Layer-3 firing time
/// as a `CACG-VERIFY-002` diagnostic with severity = Error via
/// [`SemanticEvaluationError`], NOT as a startup `CACG-MAN-001`.
/// Adapter construction failures (Tokio current-thread runtime
/// build — exceedingly rare in practice) ARE startup-class
/// infrastructure errors and emit `CACG-MAN-001` here before any
/// verification begins.
fn build_semantic_evaluator(
    args: &VerifyArgs,
) -> Result<Option<Box<dyn SemanticEvaluator>>, ExitCode> {
    if let Some(cache_path_str) = args.semantic.as_deref() {
        let cache_path = Path::new(cache_path_str);
        if !cache_path.is_file() {
            eprintln!(
                "CACG-MAN-001: semantic cache not found or not a regular file: {}",
                cache_path.display(),
            );
            return Err(ExitCode::FAILURE);
        }
        return match SemanticCache::load(cache_path) {
            Ok(cache) => Ok(Some(Box::new(cache) as Box<dyn SemanticEvaluator>)),
            Err(err) => {
                eprintln!("CACG-MAN-001: cannot load semantic cache: {err}");
                Err(ExitCode::FAILURE)
            }
        };
    }
    #[cfg(feature = "b2-llm-judge")]
    if args.semantic_judge {
        use cacg_semantic::b2::{B2Evaluator, HaikuClient, LlmJudgeClient};
        use std::sync::Arc;
        let client: Arc<dyn LlmJudgeClient + Send + Sync> =
            Arc::new(HaikuClient::with_components_using_env_key());
        return match B2Evaluator::new(client) {
            Ok(adapter) => Ok(Some(Box::new(adapter) as Box<dyn SemanticEvaluator>)),
            Err(err) => {
                eprintln!("CACG-MAN-001: failed to construct B2 LLM-judge adapter: {err}",);
                Err(ExitCode::FAILURE)
            }
        };
    }
    Ok(None)
}

/// `kb verify <card>` dispatcher. Mirrors Python
/// `legacy_python_oracle/src/cacg/cli.py::_cmd_verify`: the single-card path delegates
/// to the canonical `cacg_core::verify::verify_one_card` runner,
/// which owns the exactly-one-journal-event cardinality contract.
/// The `--round-summary` branch dispatches to
/// `dispatch_verify_round_summary` (native), which runs the same
/// runner per cited card. Layer-3 `--semantic <path>` wires the
/// B1 cache-as-oracle backend; `--semantic-judge` (only under
/// `--features b2-llm-judge`) wires the B2 LLM-judge adapter
/// over an Anthropic Haiku client. Both flow through
/// [`build_semantic_evaluator`] and emerge as
/// `Option<Box<dyn SemanticEvaluator>>`. The `verify_one_card`
/// runner builds the BM25 hint cache internally, so a failed
/// `CACG-VERIFY-001` carries BM25 "did you mean" hints in its
/// diagnostic payload.
pub(crate) fn dispatch_verify(args: VerifyArgs) -> ExitCode {
    if let Some(summary_path) = &args.round_summary {
        return dispatch_verify_round_summary(summary_path.clone(), args);
    }

    let Some(card_path) = args.card.clone() else {
        eprintln!("CACG-CLI-003: kb verify requires either <card> or --round-summary");
        return ExitCode::FAILURE;
    };

    // Card-path shape preflight (BL-20260518-shape-check-fs-inputs).
    // A missing / directory / dangling-symlink card path is a CLI input
    // error: report `CACG-CLI-001` "file not found" at the boundary,
    // BEFORE building the Layer-3 evaluator, deriving the sibling
    // `cards_manifest.json`, or touching the shared lint journal. The
    // contracted `kb verify <missing> --source-matrix <m>` command omits
    // `--chunks-manifest`, so the derived manifest AND the journal both
    // default to the root `out/`; a stale/malformed manifest there would
    // otherwise fail-close with `CACG-MAN-001`, and a pre-existing
    // corrupt journal with `CACG-JNL-001` — either of which would shadow
    // the real diagnostic. Like the sibling `CACG-CLI-003` input error
    // above, a nonexistent path is reported directly: it has no card
    // identity to verify or journal, so it never enters the
    // verification/journal pipeline (the core `verify_one_card` runner
    // still journals a missing card when invoked directly). `is_file()`
    // (not `exists()`) routes a directory or dangling symlink here too,
    // and reusing the canonical `CLI_001` code + diagnostic formatter
    // keeps the wire shape `<path>: CACG-CLI-001 file not found: <path>`
    // identical to the runner's own preflight.
    if !card_path.is_file() {
        let diag = Diagnostic::new(
            codes::CLI_001,
            Severity::Error,
            format!("file not found: {}", card_path.display()),
        )
        .with_file(card_path.display().to_string());
        emit_lint_diagnostics(&[diag]);
        return ExitCode::FAILURE;
    }

    // Build the optional Layer-3 evaluator BEFORE consuming any
    // `args` fields by move so a bad `--semantic <path>` fails fast
    // with `CACG-MAN-001` (mirrors Python `_build_semantic_spec`).
    let semantic_spec = match build_semantic_evaluator(&args) {
        Ok(spec) => spec,
        Err(code) => return code,
    };
    let semantic_arg: Option<&dyn SemanticEvaluator> = semantic_spec
        .as_deref()
        .map(|s| s as &dyn SemanticEvaluator);

    let chunks_manifest_path = args.chunks_manifest;
    // The verify journal shares the lint journal file: Python
    // defaults to `<chunks_manifest>.parent / "lint_journal.jsonl"`.
    let journal_path: PathBuf = args.journal.unwrap_or_else(|| {
        chunks_manifest_path.parent().map_or_else(
            || PathBuf::from("lint_journal.jsonl"),
            |p| p.join("lint_journal.jsonl"),
        )
    });

    let auth = AuthSpec::from_optional_path(Some(&args.source_matrix));
    let auth_arg = if auth.enabled() { Some(&auth) } else { None };

    // Derive cards_manifest.json from the chunks-manifest directory.
    // A present-but-malformed manifest is fail-closed: surface
    // CACG-MAN-001 rather than silently treating it as
    // no-retractions (mirrors Python `_cmd_verify`). Reached only for an
    // EXISTING card, so manifest validation is never weakened by the
    // missing-card shortcut above
    // (BL-20260522-port-pydantic-validators-not-just-fields).
    let cards_manifest_path = chunks_manifest_path.parent().map_or_else(
        || PathBuf::from("cards_manifest.json"),
        |p| p.join("cards_manifest.json"),
    );
    let retraction = match RetractionSpec::from_cards_manifest_lenient(
        &cards_manifest_path,
        args.allow_retracted,
    ) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("CACG-MAN-001: cards_manifest.json could not be loaded: {e}");
            return ExitCode::FAILURE;
        }
    };
    let retraction_arg = if retraction.enabled() {
        Some(&retraction)
    } else {
        None
    };

    match verify_one_card(
        &card_path,
        &chunks_manifest_path,
        &journal_path,
        args.fuzzy,
        args.skip_lint,
        None,
        auth_arg,
        retraction_arg,
        semantic_arg,
        None,
    ) {
        Ok(result) => {
            if result.verified {
                ExitCode::SUCCESS
            } else {
                emit_lint_diagnostics(&result.diagnostics);
                ExitCode::FAILURE
            }
        }
        Err(err) => {
            eprintln!("CACG-JNL-001: {err}");
            ExitCode::FAILURE
        }
    }
}

/// `kb verify --round-summary <summary>` dispatcher. Mirrors Python
/// `legacy_python_oracle/src/cacg/cli.py::_cmd_verify_round_summary` (lines 973-1058) on
/// the stdout/stderr surface, exit-code ladder, and diagnostic-code
/// translation.
///
/// The path shape pre-check uses `is_file()` per
/// `BL-20260518-shape-check-fs-inputs`: directories AND missing paths
/// land on `CACG-CLI-001` rather than panicking on the inner
/// `read_to_string`. Manifest existence is NOT pre-checked here -
/// structural-only outcomes (N/A sentinel, missing-section + non-KB-
/// relevant, missing-section + KB-relevant raising `CACG-RS-001`)
/// never load the manifest, mirroring Python.
fn dispatch_verify_round_summary(summary_path: PathBuf, args: VerifyArgs) -> ExitCode {
    if !summary_path.is_file() {
        eprintln!(
            "CACG-CLI-001: round summary not found or not a regular file: {}",
            summary_path.display(),
        );
        return ExitCode::FAILURE;
    }

    let semantic_spec = match build_semantic_evaluator(&args) {
        Ok(spec) => spec,
        Err(code) => return code,
    };
    let semantic_arg: Option<&dyn SemanticEvaluator> = semantic_spec
        .as_deref()
        .map(|s| s as &dyn SemanticEvaluator);

    let chunks_manifest_path = args.chunks_manifest;
    let journal_path: PathBuf = args.journal.unwrap_or_else(|| {
        chunks_manifest_path.parent().map_or_else(
            || PathBuf::from("lint_journal.jsonl"),
            |p| p.join("lint_journal.jsonl"),
        )
    });

    let result = match verify_round_summary(
        &summary_path,
        &chunks_manifest_path,
        &journal_path,
        args.fuzzy,
        Some(&args.source_matrix),
        args.allow_retracted,
        None,
        semantic_arg,
    ) {
        Ok(r) => r,
        Err(VerifyRoundSummaryError::ReadSummary { path, source }) => {
            eprintln!(
                "CACG-CLI-001: cannot read round summary {}: {source}",
                path.display(),
            );
            return ExitCode::FAILURE;
        }
        Err(VerifyRoundSummaryError::Retraction(e)) => {
            eprintln!("CACG-MAN-001: cards_manifest.json could not be loaded: {e}");
            return ExitCode::FAILURE;
        }
        Err(VerifyRoundSummaryError::Runner(e)) => {
            eprintln!("CACG-JNL-001: {e}");
            return ExitCode::FAILURE;
        }
    };

    emit_round_summary_result(&result)
}

/// Translate a `RoundSummaryResult` to stdout/stderr per Python
/// parity and return the corresponding `ExitCode`.
fn emit_round_summary_result(result: &RoundSummaryResult) -> ExitCode {
    if result.is_na {
        println!("N/A acknowledged");
        return ExitCode::SUCCESS;
    }
    if result.section_missing {
        if result.kb_relevant {
            eprintln!("CACG-RS-001: ## Knowledge Consulted section missing on KB-relevant work",);
            return exit_code_from_int(2);
        }
        println!("(no Knowledge Consulted section; round not KB-relevant)");
        return ExitCode::SUCCESS;
    }
    for verdict in &result.paths {
        let mut line = format!("{}: {}", verdict.path, verdict.verdict.as_str());
        if !verdict.detail.is_empty() {
            line.push_str(&format!(" ({})", verdict.detail));
        }
        if verdict.verdict == Verdict::Verified {
            println!("{line}");
        } else {
            eprintln!("{line}");
        }
    }
    exit_code_from_int(result.exit_code())
}

fn exit_code_from_int(code: i32) -> ExitCode {
    // `ExitCode::from` accepts u8; clamp 0..=255 (callers only emit
    // 0, 1, or 2).
    let clamped = u8::try_from(code).unwrap_or(1);
    ExitCode::from(clamped)
}
