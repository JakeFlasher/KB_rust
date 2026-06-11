#![allow(clippy::unwrap_used)]
//! End-to-end coverage for the `kb` clap surface:
//! presence of all 14 subcommands in `--help`, mandatory
//! `--source-matrix` enforcement on the trust-kernel four
//! (`lint`, `verify`, `search`, `show`) producing clap's exit-2
//! usage error, and the shared `CACG-CLI-NOT-IMPLEMENTED-<verb>`
//! diagnostic + exit 1 from `unimplemented_subcommand` for the
//! reserved subcommands without a native Rust implementation today.

use std::process::Command;

/// Subcommand names that clap's `kb --help` must list verbatim
/// (kebab-case where Python uses it). Order matches the source.
const ALL_SUBCOMMANDS: &[&str] = &[
    "ingest",
    "new",
    "lint",
    "verify",
    "history",
    "index",
    "retract",
    "retract-source",
    "retract-chunk",
    "scaffold-matrix",
    "scaffold-role-map",
    "search",
    "show",
    "migrate-summaries",
];

/// Subcommands that the trust kernel requires `--source-matrix` on.
/// Invoking these without the flag must exit 2 (argparse-equivalent
/// usage error) so downstream tooling can rely on the contract.
const SOURCE_MATRIX_REQUIRED: &[&str] = &["lint", "verify", "search", "show"];

/// Subcommands whose runtime is intentionally absent in Rust today
/// and must dispatch to the shared `unimplemented_subcommand(verb)`
/// helper. Each of these MUST print
/// `CACG-CLI-NOT-IMPLEMENTED-<verb>: native Rust implementation
/// pending` to stderr and exit 1 when invoked with a parse-clean argv.
const UNIMPLEMENTED_VERBS: &[&str] = &[
    // `lint`, `verify`, `search`, `show`, `ingest`, and `new` are
    // implemented and therefore removed from the unimplemented-verb
    // regression list:
    //   - lint: `cacg_core::lint::layer1::lint_card`
    //   - verify: `cacg_core::verify::verify_one_card`
    //   - search: in-memory via `cacg_search::SummariesIndex`
    //   - show: `cacg_core::card_loader` + `cacg_core::index::CardsManifest`
    //   - ingest: `cacg_ingest::extract_pages` +
    //     `cacg_ingest::manifest::{build,publish}_manifests`
    //   - new: `cacg_core::card_template::build_card_text` +
    //     `dispatch_new` (kb_new.rs covers the dispatcher contract
    //     end-to-end including byte-equality with Python kb new).
    //   - retract-chunk: `cacg_core::retract::retract_chunk` +
    //     `dispatch_retract_chunk` (kb_retract_chunk.rs covers
    //     the manifest-mutation contract end-to-end including
    //     byte-equality with Python kb retract-chunk).
    //   - retract-source: `cacg_core::retract::retract_source` +
    //     `dispatch_retract_source` (kb_retract_source.rs covers the
    //     whole-source takedown contract end-to-end, including the
    //     verify-rejects-retracted-source lifecycle).
    "history",
    "retract",
    "scaffold-matrix",
    "scaffold-role-map",
    "migrate-summaries",
];

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

#[test]
fn kb_help_lists_all_14_subcommands() {
    let output = Command::new(kb_bin())
        .arg("--help")
        .output()
        .expect("spawn kb --help");
    assert!(output.status.success(), "kb --help should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    for sub in ALL_SUBCOMMANDS {
        assert!(
            stdout.contains(sub),
            "kb --help must list subcommand {sub:?}; got:\n{stdout}"
        );
    }
    // Sanity check the count: clap also emits the auto-generated `help`
    // subcommand, so the listing has 15 lines under "Commands:".
    let commands_block_starts = stdout.find("Commands:").expect("Commands: block present");
    let commands_block = &stdout[commands_block_starts..];
    let subcommand_count = ALL_SUBCOMMANDS
        .iter()
        .filter(|s| commands_block.contains(*s))
        .count();
    assert_eq!(
        subcommand_count,
        ALL_SUBCOMMANDS.len(),
        "every subcommand from the catalog must appear under `Commands:`"
    );
}

#[test]
fn source_matrix_required_subcommands_exit_2_when_flag_missing() {
    for verb in SOURCE_MATRIX_REQUIRED {
        // Argv that would parse cleanly if `--source-matrix` were
        // present: every subcommand accepts a single permissive
        // positional via `PermissiveArgs::positionals`. The exit code
        // is therefore attributable to the missing required flag and
        // not to a different parse error.
        let output = Command::new(kb_bin())
            .arg(verb)
            .arg("some_argument")
            .output()
            .unwrap_or_else(|e| panic!("spawn kb {verb} <arg>: {e}"));
        let status = output.status;
        let code = status.code();
        assert_eq!(
            code,
            Some(2),
            "kb {verb} without --source-matrix must exit 2 (clap usage error); status={status:?}, stderr={}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("source-matrix") || stderr.contains("source_matrix"),
            "missing-flag error message must reference --source-matrix; got: {stderr}"
        );
    }
}

/// Build a parse-clean argv for each subcommand: supply every required
/// positional + flag Python argparse demands so the runtime can reach
/// the `unimplemented_subcommand(verb)` dispatcher. The shape per verb
/// mirrors Python's surface:
/// - `new` requires two positionals (`reading_id` + `slug`).
/// - `scaffold-matrix` has no positionals.
/// - `scaffold-role-map` has no positionals but requires `--reading`.
/// - All other verbs accept a single positional.
fn argv_for_parse_clean(verb: &str) -> Vec<String> {
    let mut argv: Vec<String> = Vec::new();
    if SOURCE_MATRIX_REQUIRED.contains(&verb) {
        argv.push("--source-matrix".to_string());
        argv.push("/nonexistent/source_matrix.toml".to_string());
    }
    match verb {
        "new" => {
            argv.push("reading_01".to_string());
            argv.push("synthetic-slug".to_string());
        }
        "scaffold-matrix" => {
            // No positionals.
        }
        "scaffold-role-map" => {
            argv.push("--reading".to_string());
            argv.push("reading_01".to_string());
        }
        _ => {
            argv.push("synthetic_positional".to_string());
        }
    }
    argv
}

#[test]
fn unimplemented_subcommands_print_diagnostic_and_exit_1() {
    for verb in UNIMPLEMENTED_VERBS {
        let argv = argv_for_parse_clean(verb);
        let output = Command::new(kb_bin())
            .arg(verb)
            .args(&argv)
            .output()
            .unwrap_or_else(|e| panic!("spawn kb {verb} {argv:?}: {e}"));
        let status = output.status;
        let code = status.code();
        assert_eq!(
            code,
            Some(1),
            "kb {verb} {argv:?} must exit 1 via unimplemented_subcommand; status={status:?}, stderr={}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        let expected_marker = format!("CACG-CLI-NOT-IMPLEMENTED-{verb}");
        assert!(
            stderr.contains(&expected_marker),
            "kb {verb} stderr must contain {expected_marker:?}; got: {stderr}"
        );
        assert!(
            stderr.contains("native Rust implementation pending"),
            "kb {verb} stderr must contain the pending-runtime sentence; got: {stderr}"
        );
    }
}

#[test]
fn unimplemented_dispatcher_uses_exit_code_1_not_2_or_3() {
    // Unimplemented-dispatcher contract: exit 1 (operational
    // failure), NOT 2 (usage error reserved for clap parse failures)
    // and NOT a new typed code outside 0/1/2. A regression that
    // emits any other code would break the downstream tooling
    // contract that distinguishes parse errors from runtime
    // failures.
    //
    // The probe verb must be one of the still-stubbed
    // `UNIMPLEMENTED_VERBS` (above), invoked with a parse-clean
    // argv so the exit code is attributable to the runtime
    // dispatcher (not to a clap parse failure). `history` plus a
    // positional argument satisfies the unimplemented-dispatcher
    // contract and depends on no filesystem state. (Earlier
    // revisions of this test invoked `kb new reading_01 some-slug`;
    // that verb became NATIVE so a clean tree returned exit 0 — the
    // success path of real card creation — and falsified the
    // regression.)
    let argv = argv_for_parse_clean("history");
    let output = Command::new(kb_bin())
        .arg("history")
        .args(&argv)
        .output()
        .expect("spawn kb history");
    let code = output.status.code().expect("kb history exits with a code");
    assert_eq!(code, 1, "exit code must be exactly 1; got {code}");
    assert_ne!(code, 0, "must not be silent success");
    assert_ne!(code, 2, "must not collide with clap's usage-error code");
}

#[test]
fn kb_search_accepts_negative_top_k_matching_python_argparse() {
    // Python argparse declares `--top-k` with `type=int`, which
    // accepts any signed integer including `-1`; the runtime then
    // clamps via `max(int(args.top_k), 0)` at the call site. The Rust
    // clap surface must mirror that parse contract -- a Python-parse-
    // clean argv with `--top-k -1` must REACH the runtime dispatcher
    // (and fail there on the non-existent inputs, exit 1) instead of
    // being rejected at parse time as a clap usage error (exit 2).
    // Using `u32` for the `top_k` field would fail this test. The
    // negative-`top_k` *clamp behavior* is covered end-to-end by the
    // `kb_search` oracle parity test; this asserts only the parse
    // contract.
    let output = Command::new(kb_bin())
        .arg("search")
        .arg("--source-matrix")
        .arg("/nonexistent/source_matrix.toml")
        .arg("--summaries")
        .arg("/nonexistent/summaries.json")
        .arg("--top-k")
        .arg("-1")
        .arg("query")
        .output()
        .expect("spawn kb search ... --top-k -1");
    let code = output.status.code();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_ne!(
        code,
        Some(2),
        "kb search ... --top-k -1 must NOT be rejected as a clap usage error (exit 2); \
         the `--top-k` field must be a signed integer. status={:?}, stderr={stderr}",
        output.status,
    );
    assert_eq!(
        code,
        Some(1),
        "kb search ... --top-k -1 must reach the runtime dispatcher and fail (exit 1) \
         on the non-existent inputs. status={:?}, stderr={stderr}",
        output.status,
    );
    assert!(
        stderr.contains("CACG-CLI-001"),
        "stderr must carry the missing-summaries runtime diagnostic, proving the argv \
         reached the dispatcher rather than being rejected by clap; got: {stderr}"
    );
}

#[test]
fn kb_help_subcommand_succeeds_for_each_required_source_matrix_verb() {
    // `kb <verb> --help` must succeed (exit 0) and document the
    // required `--source-matrix` flag. This guards against a
    // regression that would make the help-render path itself depend
    // on the required flag being supplied (clap supports this, but
    // we want the standard behavior).
    for verb in SOURCE_MATRIX_REQUIRED {
        let output = Command::new(kb_bin())
            .arg(verb)
            .arg("--help")
            .output()
            .unwrap_or_else(|e| panic!("spawn kb {verb} --help: {e}"));
        assert!(
            output.status.success(),
            "kb {verb} --help should exit 0; got status={:?}",
            output.status
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("--source-matrix"),
            "kb {verb} --help must document --source-matrix; got:\n{stdout}"
        );
    }
}
