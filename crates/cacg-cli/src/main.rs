//! `kb` binary: thin dispatcher over the cacg-core trust kernel.
//!
//! All trust-critical logic lives in cacg-core; this binary only
//! parses args, constructs a `DeterminismContext` from the
//! environment, and dispatches to library functions. The clap parser
//! lives in `cacg_cli::lib` so integration tests can introspect it
//! without spawning the binary. The static-grep `xtask lint-determinism`
//! and `xtask lint-rename-outside-publisher` gates enforce that no
//! nondeterministic API call or raw rename sneaks into this crate.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::doc_markdown,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::uninlined_format_args,
    clippy::needless_pass_by_value,
    clippy::wildcard_enum_match_arm,
    clippy::ptr_arg,
    clippy::map_unwrap_or
)]

mod dispatch_ingest;
mod dispatch_lint;
mod dispatch_new;
mod dispatch_retract;
mod dispatch_search;
mod dispatch_show;
mod dispatch_verify;

use std::path::Path;
use std::process::ExitCode;

use cacg_cli::{unimplemented_subcommand, Cli, Cmd, IndexArgs};
use cacg_core::determinism::DeterminismContext;
use cacg_core::index::{build_index, SummariesManifest};
use clap::Parser;

use dispatch_ingest::dispatch_ingest;
use dispatch_lint::dispatch_lint;
use dispatch_new::dispatch_new;
use dispatch_retract::dispatch_retract_chunk;
use dispatch_search::dispatch_search;
use dispatch_show::dispatch_show;
use dispatch_verify::dispatch_verify;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Index(args) => dispatch_index(&args),
        Cmd::Ingest(args) => dispatch_ingest(&args),
        Cmd::New(args) => dispatch_new(&args),
        Cmd::Lint(args) => dispatch_lint(args),
        Cmd::Verify(args) => dispatch_verify(args),
        Cmd::History(_) => unimplemented_subcommand("history"),
        Cmd::Retract(_) => unimplemented_subcommand("retract"),
        Cmd::RetractSource(args) => dispatch_retract::dispatch_retract_source(&args),
        Cmd::RetractChunk(args) => dispatch_retract_chunk(&args),
        Cmd::ScaffoldMatrix(_) => unimplemented_subcommand("scaffold-matrix"),
        Cmd::ScaffoldRoleMap(_) => unimplemented_subcommand("scaffold-role-map"),
        Cmd::Search(args) => dispatch_search(&args),
        Cmd::Show(args) => dispatch_show(&args),
        Cmd::MigrateSummaries(_) => unimplemented_subcommand("migrate-summaries"),
    }
}

/// `kb index` dispatcher. Builds the trust artifacts via `cacg-core`,
/// then runs the non-trust `summaries.sqlite` FTS5 sidecar publication
/// post-step. Mirrors Python `_cmd_index` + `_publish_summaries_phase_d`.
fn dispatch_index(args: &IndexArgs) -> ExitCode {
    let ctx = DeterminismContext::from_env();
    if let Err(err) = build_index(&args.cards_dir, &args.out, &ctx) {
        eprintln!("kb index: {err}");
        return ExitCode::FAILURE;
    }
    eprintln!(
        "kb index: published {} and {}",
        args.out.join("cards_manifest.json").display(),
        args.out.join("summaries.json").display()
    );
    publish_search_sidecar(&args.out);
    ExitCode::SUCCESS
}

/// Build the `summaries.sqlite` `FTS5` search sidecar as a non-fatal
/// `kb index` post-step. The sidecar is a performance artifact, not a
/// trust artifact: any failure (missing `FTS5` support, an I/O or
/// `SQLite` error) emits a `CACG-FTS-002` informational line and
/// leaves the `cacg-core` trust artifacts intact — `kb search`
/// rebuilds-or-falls-back via the `meta` seal. `SQLite` stays out of
/// `cacg-core`: the build runs here in `cacg-cli` against `cacg-search`.
fn publish_search_sidecar(out_dir: &Path) {
    let summaries_path = out_dir.join("summaries.json");
    let bytes = match std::fs::read(&summaries_path) {
        Ok(b) => b,
        Err(err) => {
            eprintln!(
                "CACG-FTS-002: cannot read summaries.json for the search sidecar \
                 (non-fatal; kb search falls back to in-memory BM25): {err}"
            );
            return;
        }
    };
    let manifest: SummariesManifest = match serde_json::from_slice(&bytes) {
        Ok(m) => m,
        Err(err) => {
            eprintln!(
                "CACG-FTS-002: cannot parse summaries.json for the search sidecar \
                 (non-fatal; kb search falls back to in-memory BM25): {err}"
            );
            return;
        }
    };
    // Validate the manifest at this trust boundary before building the
    // sidecar — a parseable but invariant-invalid `summaries.json`
    // (duplicate summary id, non-canonical `source_ids`) must NOT be
    // sealed into a sidecar that bypasses the in-memory loader's checks.
    if let Err(err) = cacg_search::validate_manifest(&manifest) {
        eprintln!(
            "CACG-FTS-002: summaries.json failed validation; the search sidecar \
             was not built (non-fatal; kb search falls back to in-memory BM25): {err}"
        );
        return;
    }
    match cacg_search::build_sidecar(out_dir, &bytes, &manifest) {
        Ok(Some(_)) => {}
        Ok(None) => eprintln!(
            "CACG-FTS-002: runtime SQLite lacks FTS5 support; summaries.sqlite \
             sidecar skipped (non-fatal; kb search falls back to in-memory BM25)."
        ),
        Err(err) => eprintln!(
            "CACG-FTS-002: summaries.sqlite sidecar build failed (non-fatal; \
             kb search falls back to in-memory BM25): {err}"
        ),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::publish_search_sidecar;

    #[test]
    fn publish_search_sidecar_skips_an_invariant_invalid_summaries_json() {
        // A parseable but invariant-invalid `summaries.json` (here a
        // duplicate summary id) must NOT be sealed into a sidecar. The
        // `kb index` post-step is non-fatal: it emits `CACG-FTS-002`,
        // leaves the out-dir's trust artifacts untouched, and creates
        // no `summaries.sqlite`.
        let dir = tempfile::tempdir().expect("tempdir");
        // Two summaries sharing id "dup" — JSON-shape-valid, but a
        // `validate_manifest` invariant violation. (Literal newlines in
        // the raw string are JSON whitespace.)
        let invalid = r#"{"schema_version":"cacg.v0","summaries":[
{"schema_version":"cacg.v0","id":"dup","title":"A","reading_id":"R1","summary":"summary a","tags":[],"source_ids":["s1"],"path":"cards/a.md","card_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"},
{"schema_version":"cacg.v0","id":"dup","title":"B","reading_id":"R1","summary":"summary b","tags":[],"source_ids":["s1"],"path":"cards/b.md","card_hash":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}]}"#;
        std::fs::write(dir.path().join("summaries.json"), invalid).unwrap();
        let cards_manifest = dir.path().join("cards_manifest.json");
        std::fs::write(&cards_manifest, b"trust-artifact-sentinel").unwrap();

        // Non-fatal: returns normally, prints `CACG-FTS-002`.
        publish_search_sidecar(dir.path());

        assert!(
            !dir.path().join("summaries.sqlite").exists(),
            "an invalid summaries.json must NOT be sealed into a sidecar"
        );
        assert!(
            !dir.path().join("summaries.sqlite.tmp").exists(),
            "no sidecar tmp may be left behind"
        );
        // The post-step never touches the cacg-core trust artifacts.
        assert_eq!(
            std::fs::read(&cards_manifest).unwrap(),
            b"trust-artifact-sentinel"
        );
    }
}
