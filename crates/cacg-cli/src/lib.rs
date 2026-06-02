//! Library face of the `kb` CLI.
//!
//! Holds the clap parser definitions for the 14 subcommands and the
//! shared `unimplemented_subcommand(verb)` helper for reserved verbs
//! whose Rust runtime is still pending. Integration tests
//! introspect the parser tree directly via [`Cli::command`]. The
//! binary's `main.rs` is a thin dispatcher that calls into this
//! library.
//!
//! Each subcommand's `Args` struct mirrors Python `cacg.cli`'s
//! argparse surface: positionals in declaration order with the same
//! `nargs` semantics, long flags with matching defaults +
//! required-ness, mutually-exclusive groups via `clap::ArgGroup`, and
//! the hidden `--skip-lint` alias on `verify`'s `--unsafe-skip-lint`
//! (both flags map to the same `skip_lint` field).

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::doc_markdown,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::needless_pass_by_value,
    clippy::map_unwrap_or,
    clippy::uninlined_format_args
)]

pub mod round_summary;

use std::path::PathBuf;

use clap::{ArgGroup, Args, Parser, Subcommand};

pub use clap::CommandFactory;

/// Top-level parser. `kb [SUBCOMMAND] ...` -- 14 subcommands total,
/// matching Python's argparse surface 1:1.
#[derive(Parser, Debug)]
#[command(
    name = "kb",
    version,
    about = "CACG (Content-Addressable Card Graph) CLI — Rust port."
)]
pub struct Cli {
    /// Selected subcommand.
    #[command(subcommand)]
    pub cmd: Cmd,
}

/// The complete `kb` subcommand surface. Mirrors the historical CLI's
/// argparse subparsers 1:1. Implemented verbs dispatch to Rust
/// modules; reserved verbs dispatch to [`unimplemented_subcommand`]
/// which exits 1.
#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Parse a PDF into a hash-pinned chunk manifest.
    Ingest(IngestArgs),
    /// Scaffold a new card from the canonical template.
    New(NewArgs),
    /// Run layer-1 mechanical lint on cards. Mandatory
    /// `--source-matrix` per the trust kernel.
    Lint(LintArgs),
    /// Run layer-2 content verification. Mandatory
    /// `--source-matrix`.
    Verify(VerifyArgs),
    /// Print or validate per-card `.history.jsonl` (unimplemented).
    History(HistoryArgs),
    /// Build cards_manifest.json + summaries.json + INDEX.md + per-card
    /// history from a directory of `.md` cards. The only natively
    /// implemented subcommand today; threads a `DeterminismContext`
    /// from `KB_FROZEN_CLOCK` so two runs with the env set produce
    /// byte-identical artifacts.
    Index(IndexArgs),
    /// Retract a card via tombstone history event (unimplemented).
    Retract(RetractArgs),
    /// Retract a source: add source_id to retracted_sources (unimplemented).
    #[command(name = "retract-source")]
    RetractSource(RetractSourceArgs),
    /// Retract a single chunk: add chunk_id to retracted_chunks.
    #[command(name = "retract-chunk")]
    RetractChunk(RetractChunkArgs),
    /// Generate a permissive source_matrix.json (unimplemented).
    #[command(name = "scaffold-matrix")]
    ScaffoldMatrix(ScaffoldMatrixArgs),
    /// Synthesize a permissive source role-map (unimplemented).
    #[command(name = "scaffold-role-map")]
    ScaffoldRoleMap(ScaffoldRoleMapArgs),
    /// In-memory BM25 search over summaries.json. Mandatory
    /// `--source-matrix`.
    Search(SearchArgs),
    /// Show a single card's frontmatter + summary + citations.
    /// Mandatory `--source-matrix`.
    Show(ShowArgs),
    /// Populate empty card summaries via retrieval-surface migration
    /// (unimplemented).
    #[command(name = "migrate-summaries")]
    MigrateSummaries(MigrateSummariesArgs),
}

/// `kb history <card> [--validate]`.
#[derive(Args, Debug)]
pub struct HistoryArgs {
    /// Path to a single card file.
    pub card: PathBuf,
    /// Validate the chain instead of printing.
    #[arg(long)]
    pub validate: bool,
}

/// `kb index [cards_dir] [--out OUT]`.
#[derive(Args, Debug)]
pub struct IndexArgs {
    /// Cards directory to walk recursively.
    #[arg(default_value = "cards")]
    pub cards_dir: PathBuf,
    /// Output directory for cards_manifest.json + summaries.json
    /// + INDEX.md.
    #[arg(long, default_value = "out")]
    pub out: PathBuf,
}

/// `kb ingest <pdf> [--config PATH] [--out OUT] [--source-id ID]`.
#[derive(Args, Debug)]
pub struct IngestArgs {
    /// Path to PDF source.
    pub pdf: PathBuf,
    /// Optional ingest config (YAML). Loads `chunking.target_tokens`
    /// / `chunking.overlap_tokens` / `chunking.max_pages_per_chunk`.
    /// Not yet implemented in the Rust `kb ingest` dispatcher;
    /// passing `--config` currently exits with `CACG-CLI-004`.
    #[arg(long)]
    pub config: Option<PathBuf>,
    /// Output directory.
    #[arg(long, default_value = "out")]
    pub out: PathBuf,
    /// Source identifier override.
    #[arg(long = "source-id")]
    pub source_id: Option<String>,
}

/// `kb new <reading_id> <slug> [--cards-dir DIR] [--force] [--title T]`.
#[derive(Args, Debug)]
pub struct NewArgs {
    /// Reading namespace (e.g. `reading_01`).
    pub reading_id: String,
    /// Card slug.
    pub slug: String,
    /// Cards directory root.
    #[arg(long = "cards-dir", default_value = "cards")]
    pub cards_dir: PathBuf,
    /// Overwrite existing card file with template.
    #[arg(long)]
    pub force: bool,
    /// Override default title text.
    #[arg(long)]
    pub title: Option<String>,
}

/// `kb lint [card | --all-readings] [--cards-dir DIR] [--check-orphans]
///   [--chunks-manifest PATH] [--journal PATH] --source-matrix PATH`.
///
/// The `card` positional and `--all-readings` flag form a required
/// mutually-exclusive group: exactly one must be supplied.
#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("lint_target")
        .required(true)
        .args(["card", "all_readings"])
))]
pub struct LintArgs {
    /// Path to a single card file (one of `card` or `--all-readings`).
    pub card: Option<PathBuf>,
    /// Lint every card under `--cards-dir` (one of `card` or
    /// `--all-readings`).
    #[arg(long = "all-readings")]
    pub all_readings: bool,
    /// Root cards directory.
    #[arg(long = "cards-dir", default_value = "cards")]
    pub cards_dir: PathBuf,
    /// Opt-in: emit orphan-card diagnostics.
    #[arg(long = "check-orphans")]
    pub check_orphans: bool,
    /// Path to chunks_manifest.json produced by `kb ingest`.
    #[arg(long = "chunks-manifest", default_value = "out/chunks_manifest.json")]
    pub chunks_manifest: PathBuf,
    /// Path to lint_journal.jsonl (default: alongside chunks-manifest).
    #[arg(long)]
    pub journal: Option<PathBuf>,
    /// Required source matrix path.
    #[arg(long = "source-matrix", required = true)]
    pub source_matrix: PathBuf,
}

/// `kb verify [card | --round-summary PATH] [--allow-retracted] ...
///   --source-matrix PATH`.
///
/// Has up to two mutex groups: `verify_target` (card vs
/// --round-summary, required) and `verify_semantic`
/// (--semantic vs --semantic-judge, optional). The
/// `verify_semantic` group + the `--semantic-judge` flag itself
/// are gated behind the `b2-llm-judge` Cargo feature; default
/// builds expose neither. The hidden alias `--skip-lint` maps
/// to the same `skip_lint` field as `--unsafe-skip-lint`.
#[derive(Args, Debug)]
#[command(
    group(
        ArgGroup::new("verify_target")
            .required(true)
            .args(["card", "round_summary"])
    ),
)]
#[cfg_attr(
    feature = "b2-llm-judge",
    command(
        group(
            ArgGroup::new("verify_semantic")
                .required(false)
                .args(["semantic", "semantic_judge"])
        )
    )
)]
pub struct VerifyArgs {
    /// Path to a single card file (one of `card` or `--round-summary`).
    pub card: Option<PathBuf>,
    /// Treat input as a round-summary directory.
    #[arg(long = "round-summary")]
    pub round_summary: Option<PathBuf>,
    /// Include retracted cards in verification.
    #[arg(long = "allow-retracted")]
    pub allow_retracted: bool,
    /// Path to chunks_manifest.json produced by `kb ingest`.
    #[arg(long = "chunks-manifest", default_value = "out/chunks_manifest.json")]
    pub chunks_manifest: PathBuf,
    /// Enable fuzzy matching during quote verification.
    #[arg(long)]
    pub fuzzy: bool,
    /// Path to lint_journal.jsonl.
    #[arg(long)]
    pub journal: Option<PathBuf>,
    /// Semantic verifier engine name.
    #[arg(long)]
    pub semantic: Option<String>,
    /// Run the semantic-judge variant of the semantic verifier.
    /// Only exposed when the crate is built with the
    /// `b2-llm-judge` Cargo feature; under default features
    /// the flag is invisible to clap and an attempt to pass
    /// it produces clap's "unexpected argument" error
    /// (exit 2 at argv parse time).
    #[cfg(feature = "b2-llm-judge")]
    #[arg(long = "semantic-judge")]
    pub semantic_judge: bool,
    /// Skip the layer-1 lint preflight before verification.
    /// `--skip-lint` is a hidden alias for the same field.
    #[arg(long = "unsafe-skip-lint", alias = "skip-lint")]
    pub skip_lint: bool,
    /// Required source matrix path.
    #[arg(long = "source-matrix", required = true)]
    pub source_matrix: PathBuf,
}

/// `kb retract <card> [--out OUT]`.
#[derive(Args, Debug)]
pub struct RetractArgs {
    /// Card identifier or path to retract.
    pub card: String,
    /// Output directory.
    #[arg(long, default_value = "out")]
    pub out: PathBuf,
}

/// `kb retract-source <source_id> [--cards-dir DIR] [--out OUT]`.
#[derive(Args, Debug)]
pub struct RetractSourceArgs {
    /// Source identifier to retract.
    pub source_id: String,
    /// Root cards directory.
    #[arg(long = "cards-dir")]
    pub cards_dir: Option<PathBuf>,
    /// Output directory.
    #[arg(long, default_value = "out")]
    pub out: PathBuf,
}

/// `kb retract-chunk <chunk_id> [--cards-dir DIR] [--out OUT]`.
#[derive(Args, Debug)]
pub struct RetractChunkArgs {
    /// Chunk identifier to retract.
    pub chunk_id: String,
    /// Root cards directory.
    #[arg(long = "cards-dir")]
    pub cards_dir: Option<PathBuf>,
    /// Output directory.
    #[arg(long, default_value = "out")]
    pub out: PathBuf,
}

/// `kb scaffold-matrix [--cards-dir DIR] [--cards-manifest PATH]
///   [--chunks-manifest PATH] [--out PATH]`.
#[derive(Args, Debug)]
pub struct ScaffoldMatrixArgs {
    /// Root cards directory.
    #[arg(long = "cards-dir")]
    pub cards_dir: Option<PathBuf>,
    /// Path to cards_manifest.json.
    #[arg(long = "cards-manifest", default_value = "out/cards_manifest.json")]
    pub cards_manifest: PathBuf,
    /// Path to chunks_manifest.json.
    #[arg(long = "chunks-manifest", default_value = "out/chunks_manifest.json")]
    pub chunks_manifest: PathBuf,
    /// Output path for the synthesized source_matrix.json.
    #[arg(long, default_value = "out/source_matrix.json")]
    pub out: PathBuf,
}

/// `kb scaffold-role-map [--cards-dir DIR] [--cards-manifest PATH]
///   [--force] [--out PATH] --reading READING_ID`.
#[derive(Args, Debug)]
pub struct ScaffoldRoleMapArgs {
    /// Root cards directory.
    #[arg(long = "cards-dir", default_value = "cards")]
    pub cards_dir: PathBuf,
    /// Path to cards_manifest.json.
    #[arg(long = "cards-manifest", default_value = "out/cards_manifest.json")]
    pub cards_manifest: PathBuf,
    /// Overwrite existing role-map file.
    #[arg(long)]
    pub force: bool,
    /// Output path for the role-map file.
    #[arg(long)]
    pub out: Option<PathBuf>,
    /// Required reading id.
    #[arg(long, required = true)]
    pub reading: String,
}

/// `kb search <query> [--json] --source-matrix PATH [--summaries PATH]
///   [--top-k N]`.
#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Search query.
    pub query: String,
    /// Emit results as JSON.
    #[arg(long)]
    pub json: bool,
    /// Required source matrix path.
    #[arg(long = "source-matrix", required = true)]
    pub source_matrix: PathBuf,
    /// Path to summaries.json.
    #[arg(long, default_value = "out/summaries.json")]
    pub summaries: PathBuf,
    /// Maximum number of results. Signed integer to mirror Python
    /// argparse's `type=int`, which accepts negative values cleanly;
    /// the runtime clamps via `max(value, 0)` at the call site. Using
    /// `u32` here would reject `--top-k -1` with a clap usage error
    /// (exit 2) for an argv that Python parses cleanly. `allow_hyphen_values`
    /// ensures clap accepts `-1` as the value rather than interpreting
    /// the leading `-` as a short-flag introducer.
    #[arg(long = "top-k", default_value = "10", allow_hyphen_values = true)]
    pub top_k: i64,
}

/// `kb show <card_id> [--allow-retracted] [--cards-manifest PATH]
///   [--path PATH] --source-matrix PATH`.
#[derive(Args, Debug)]
pub struct ShowArgs {
    /// Card identifier to show.
    pub card_id: String,
    /// Include retracted cards.
    #[arg(long = "allow-retracted")]
    pub allow_retracted: bool,
    /// Path to cards_manifest.json.
    #[arg(long = "cards-manifest", default_value = "out/cards_manifest.json")]
    pub cards_manifest: PathBuf,
    /// Override card path.
    #[arg(long)]
    pub path: Option<PathBuf>,
    /// Required source matrix path.
    #[arg(long = "source-matrix", required = true)]
    pub source_matrix: PathBuf,
}

/// `kb migrate-summaries [cards_dir] [--auto-heuristic] [--out OUT]
///   [--strict]`.
///
/// Has an optional mutex group `migrate_mode` between
/// `--auto-heuristic` and `--strict`.
#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("migrate_mode")
        .required(false)
        .args(["auto_heuristic", "strict"])
))]
pub struct MigrateSummariesArgs {
    /// Root cards directory.
    #[arg(default_value = "cards")]
    pub cards_dir: PathBuf,
    /// Use heuristic for missing summaries.
    #[arg(long = "auto-heuristic")]
    pub auto_heuristic: bool,
    /// Output directory.
    #[arg(long, default_value = "out")]
    pub out: PathBuf,
    /// Strict mode (fail on missing summaries).
    #[arg(long)]
    pub strict: bool,
}

/// Shared exit point for subcommands whose Rust implementation is
/// pending. Prints a stable diagnostic to stderr that downstream
/// tooling can grep for (`CACG-CLI-NOT-IMPLEMENTED-<verb>`) and
/// exits 1, treating "recognized command, runtime not implemented"
/// as an operational failure rather than a usage error.
pub fn unimplemented_subcommand(verb: &str) -> ! {
    eprintln!("CACG-CLI-NOT-IMPLEMENTED-{verb}: native Rust implementation pending");
    std::process::exit(1);
}

/// Return the underlying `clap::Command` used by the binary. Lets
/// integration tests introspect the parser tree (subcommands, flags,
/// required-ness, defaults, mutex groups) without spawning the binary.
#[must_use]
pub fn command() -> clap::Command {
    Cli::command()
}
