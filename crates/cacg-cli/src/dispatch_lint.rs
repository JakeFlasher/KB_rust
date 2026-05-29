//! `kb lint` dispatch module.

use std::path::PathBuf;
use std::process::ExitCode;

use cacg_core::diagnostic::Diagnostic;
use cacg_core::lint::layer1::{lint_card, lint_directory, LintDirectoryError};
use cacg_core::source_matrix::AuthSpec;

use cacg_cli::LintArgs;

pub(crate) fn dispatch_lint(args: LintArgs) -> ExitCode {
    let chunks_manifest_path = args.chunks_manifest;
    let journal_path: PathBuf = args.journal.unwrap_or_else(|| {
        chunks_manifest_path.parent().map_or_else(
            || PathBuf::from("lint_journal.jsonl"),
            |p| p.join("lint_journal.jsonl"),
        )
    });

    let auth = AuthSpec::from_optional_path(Some(&args.source_matrix));
    let auth_arg = if auth.enabled() { Some(&auth) } else { None };

    let (diagnostics, failed) = if args.all_readings {
        let cards_dir = &args.cards_dir;
        // Mirror Python `_cmd_lint` directory-shape preflight at
        // `legacy_python_oracle/src/cacg/cli.py:808-815`: missing path and non-directory
        // path both surface as `CACG-CLI-001` with the same stderr
        // shape that Python emits.
        if !cards_dir.exists() {
            eprintln!(
                "CACG-CLI-001: cards directory not found: {}",
                cards_dir.display()
            );
            return ExitCode::FAILURE;
        }
        if !cards_dir.is_dir() {
            eprintln!(
                "CACG-CLI-001: cards_dir is not a directory: {}",
                cards_dir.display()
            );
            return ExitCode::FAILURE;
        }
        match lint_directory(cards_dir, &chunks_manifest_path, &journal_path, auth_arg) {
            Ok(outcome) => (outcome.diagnostics, outcome.failed),
            Err(LintDirectoryError::NonDir(p)) => {
                eprintln!(
                    "CACG-CLI-001: cards_dir is not a directory: {}",
                    p.display()
                );
                return ExitCode::FAILURE;
            }
            Err(LintDirectoryError::Io { path, source }) => {
                eprintln!(
                    "CACG-CLI-001: cards directory I/O error at {}: {source}",
                    path.display()
                );
                return ExitCode::FAILURE;
            }
            Err(LintDirectoryError::Journal(err)) => {
                eprintln!("CACG-JNL-001: {err}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        let Some(card_path) = args.card else {
            eprintln!("CACG-CLI-003: kb lint requires either <card> or --all-readings");
            return ExitCode::FAILURE;
        };
        match lint_card(
            &card_path,
            &chunks_manifest_path,
            &journal_path,
            None,
            auth_arg,
        ) {
            Ok(o) => (o.diagnostics, o.failed),
            Err(err) => {
                eprintln!("CACG-JNL-001: {err}");
                return ExitCode::FAILURE;
            }
        }
    };

    if !failed {
        return ExitCode::SUCCESS;
    }
    emit_lint_diagnostics(&diagnostics);
    ExitCode::FAILURE
}

pub(crate) fn emit_lint_diagnostics(diagnostics: &[Diagnostic]) {
    for diag in diagnostics {
        let file = diag.file.as_deref().unwrap_or("?");
        eprintln!("{file}: {code} {msg}", code = diag.code, msg = diag.message);
        for hint in &diag.hints {
            emit_hint_line(hint);
        }
    }
}

/// Render one hint object to stderr in the shape Python `_cmd_verify`
/// emits (`legacy_python_oracle/src/cacg/cli.py:953-968`). BM25 "did you mean" hints carry a
/// `chunk_id` key; Layer-3 semantic hints carry `semantic_verdict`.
/// Both shapes coexist on the same diagnostic.
pub(crate) fn emit_hint_line(hint: &serde_json::Value) {
    if let Some(chunk_id) = hint.get("chunk_id").and_then(|v| v.as_str()) {
        let score = render_hint_number(hint.get("score"));
        eprintln!("  hint chunk_id={chunk_id} score={score}");
    } else if let Some(verdict) = hint.get("semantic_verdict").and_then(|v| v.as_str()) {
        let score = render_hint_number(hint.get("semantic_score"));
        let mode = hint
            .get("semantic_mode")
            .and_then(|v| v.as_str())
            .unwrap_or("?");
        eprintln!("  hint semantic_verdict={verdict} score={score} mode={mode}");
    }
}

/// Render a `serde_json::Value` number using its canonical JSON-text
/// form. The journal payload already carries the round-trip-stable
/// serde representation, so echoing `Number::to_string()` matches
/// Python's `repr(float)` for the values both implementations write
/// (BM25 scores rounded to 6 decimals; semantic_score taken verbatim
/// from the cache).
fn render_hint_number(value: Option<&serde_json::Value>) -> String {
    match value {
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(serde_json::Value::String(s)) => s.clone(),
        _ => String::from("?"),
    }
}
