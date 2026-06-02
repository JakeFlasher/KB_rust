//! xtask: workspace automation tasks.
//!
//! Hosts the parity harness (`xtask parity`), benchmark runner
//! (`xtask bench`), and static-grep lints (`xtask lint-platform-cfg`
//! today; more lints follow when the trust kernel begins).

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::doc_markdown,
    clippy::doc_lazy_continuation,
    clippy::missing_panics_doc,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::float_cmp,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::manual_let_else,
    clippy::single_match_else,
    clippy::if_not_else,
    clippy::needless_raw_string_hashes,
    clippy::needless_borrows_for_generic_args,
    clippy::redundant_closure_for_method_calls,
    clippy::manual_range_contains,
    clippy::naive_bytecount,
    clippy::useless_format,
    clippy::unreadable_literal,
    clippy::explicit_iter_loop,
    clippy::items_after_statements,
    clippy::struct_excessive_bools,
    clippy::type_complexity,
    clippy::bool_to_int_with_if,
    clippy::must_use_candidate,
    clippy::map_unwrap_or,
    clippy::ptr_arg,
    clippy::uninlined_format_args,
    clippy::str_to_string,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::iter_cloned_collect,
    clippy::wildcard_enum_match_arm,
    clippy::inefficient_to_string,
    clippy::collapsible_str_replace,
    clippy::stable_sort_primitive,
    clippy::nonminimal_bool,
    clippy::manual_strip,
    clippy::single_char_pattern,
    clippy::enum_variant_names,
    clippy::assertions_on_constants,
    clippy::unnecessary_join,
    clippy::unnecessary_map_or,
    dead_code,
    missing_docs
)]

mod audit_default_kb_deps;
mod bench_check;
mod cacg_core_deps;
pub mod gate;
mod lints;
mod parity;
mod retrieval_eval;
mod schema_fixtures;
mod semantic_cache_provenance;
mod semantic_eval;
mod threshold_sweep;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "xtask", about = "Workspace automation tasks.")]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Run the unified quality gate: all lint/audit/test/format checks
    /// in sequence. Exits non-zero on any failing non-advisory check.
    /// Use `--report` to run all checks and emit JSON results.
    Gate {
        /// Run all checks (don't stop on first failure) and emit JSON
        /// report to stdout.
        #[arg(long)]
        report: bool,
    },
    /// Run the byte-diff parity harness against the committed fixture
    /// matrix. Returns non-zero on missing artifacts, command failures,
    /// or byte differences in any gating entry.
    Parity {
        /// Corpus directory to run the parity harness against.
        #[arg(long, default_value = "tests/parity_corpus/")]
        corpus: PathBuf,
        /// Output root for the harness's temp publish trees + perf
        /// report.
        #[arg(long, default_value = "target/parity-out/")]
        out: PathBuf,
    },
    /// Run iai-callgrind benchmarks for hot-path non-regression testing.
    /// Requires valgrind.
    Bench {
        /// Compare current instruction counts against committed baselines.
        /// Exits non-zero if any hot path regresses by >5%.
        #[arg(long)]
        check: bool,
    },
    /// Run the retrieval-quality eval gate over the committed eval
    /// cases. Exits non-zero when an expected hit drops out of top-k.
    RetrievalEval {
        /// Path to the eval-cases fixture file.
        #[arg(long, default_value = "tests/retrieval_eval/eval_cases.json")]
        fixtures: PathBuf,
    },
    /// Run the Layer-3 semantic-eval gate against the committed
    /// QM B1 cache. Each case pins a `(chunk_hash,
    /// claim_window_hash)` pair and its expected verdict;
    /// exits non-zero when any case's actual lookup verdict
    /// diverges from the expected value. The fixture schema is
    /// distinct from the retrieval-eval schema so the two suites
    /// cannot drift into a shared shape.
    SemanticEval {
        /// Path to the semantic-eval fixture file.
        #[arg(long, default_value = "tests/semantic_eval/eval_cases.json")]
        fixtures: PathBuf,
    },
    /// Reject source-level platform branches that bypass the documented matrix.
    LintPlatformCfg {
        /// Root directory to scan (default: `crates/cacg-core/src`).
        #[arg(long, default_value = "crates/cacg-core/src")]
        root: PathBuf,
    },
    /// Reject raw `std::fs::rename` call sites outside the centralized
    /// publisher (`cacg_core::atomic_publish`). Any module that moves
    /// files atomically must route through the publisher so the
    /// `.tmp` / `.bak` rollback discipline is preserved.
    LintRenameOutsidePublisher {
        /// Root directories to scan. Defaults to `crates` (recursively
        /// scans every workspace crate's `src/` and `tests/` trees) and
        /// `xtask/src`, so a regression introduced in any current or
        /// future workspace crate is caught by one invocation.
        #[arg(long)]
        root: Vec<PathBuf>,
    },
    /// Reject direct calls to nondeterministic APIs outside the
    /// centralized `cacg_core::determinism::DeterminismContext`. Catches
    /// `OffsetDateTime::now_utc`, `Uuid::new_v4`, `Instant::now`,
    /// `SystemTime::now`, `chrono::Utc::now`, `chrono::Local::now`, and
    /// `tempfile::tempfile`. Test code under any `/tests/` directory is
    /// allowlisted because test fixtures never reach committed artifacts.
    LintDeterminism {
        /// Root directories to scan. Defaults to every workspace crate
        /// source tree (via `crates`) plus `xtask/src`.
        #[arg(long)]
        root: Vec<PathBuf>,
    },
    /// Reject direct trust-critical implementation calls in
    /// `crates/cacg-cli/src/**`. The CLI binary must stay a thin
    /// clap dispatcher over `cacg-core`; SHA-256 / canonical JSON /
    /// append-mode I/O / SQLite / hand-rolled key sorting all belong
    /// in the trust kernel, not the CLI.
    LintTrustLeak {
        /// Root directories to scan. Defaults to `crates/cacg-cli/src`.
        #[arg(long)]
        root: Vec<PathBuf>,
    },
    /// Static-grep gate: reject verify-path runner bypasses in
    /// `crates/cacg-cli/src/round_summary*.rs`. The CLI round-
    /// summary surface MUST route through
    /// `cacg-core::verify::verify_one_card`; direct
    /// `cacg_core::journal::append_entry` or
    /// `cacg_core::verify::layer2::verify_card` calls bypass the
    /// runner's exactly-one-journal-event-per-card contract.
    LintRunnerBypass {
        /// Root directories to scan. Defaults to `crates/cacg-cli/src`.
        #[arg(long)]
        root: Vec<PathBuf>,
    },
    /// Reject `.unwrap()` in production code (non-test `crates/*/src/**/*.rs`).
    /// Secondary grep-based check; primary enforcement is clippy::unwrap_used.
    LintUnwrap {
        /// Root directories to scan. Defaults to `crates`.
        #[arg(long)]
        root: Vec<PathBuf>,
    },
    /// Reject error-discarding patterns (`.unwrap_or_default()`, `.ok()`)
    /// in production code.
    LintErrorSwallow {
        /// Root directories to scan. Defaults to `crates`.
        #[arg(long)]
        root: Vec<PathBuf>,
    },
    /// Report structural quality violations: oversized modules and
    /// functions. Exits non-zero unless `--warn` is passed.
    LintStructural {
        /// Root directories to scan. Defaults to `crates`.
        #[arg(long)]
        root: Vec<PathBuf>,
        /// Advisory mode: report violations without failing.
        #[arg(long)]
        warn: bool,
    },
    /// Regenerate the generated_pydantic_errors fixture suite (AC-C4).
    ///
    /// Regenerates the generated_pydantic_errors manifest using the
    /// Rust-native fixture definitions.
    GenSchemaFixtures,
    /// Reject heavy or boundary-crossing dependencies in cacg-core's
    /// resolved dependency graph. Walks the full transitive non-dev
    /// closure via `cargo metadata` and matches against the RESOLVED
    /// package name so renamed-package aliases, dep-table syntax,
    /// target-specific deps, and transitive forbidden packages all
    /// surface.
    AuditCacgCoreDeps,
    /// Reject embedding / model crates and default-path
    /// network/async crates in the shipped `kb` binary's
    /// resolved dependency graph. Walks the full transitive
    /// non-dev closure of `cacg-cli` under the DEFAULT feature
    /// set (no `--all-features`, no `--no-default-features`) so
    /// the legitimate opt-in LLM-judge build graph is not
    /// inspected. Matches resolved package names so renamed
    /// aliases cannot dodge the gate; the `candle-` prefix
    /// covers the whole family without enumeration.
    AuditDefaultKbDeps,
    /// Audit the generated_pydantic_errors fixture manifest (AC-C4).
    ///
    /// Walks `tests/parity_corpus/generated_pydantic_errors/manifest.json`
    /// and asserts:
    ///   - every required CACG-FM-* + CACG-CITE-* + CACG-SUM-* code category
    ///     has at least one fixture,
    ///   - no duplicate fixture names,
    ///   - every fixture has `oracle_layer` + `expected_code` annotations,
    ///   - every parse-layer fixture in the manifest has a fully-populated
    ///     row (no `skip_reason` placeholders).
    AuditSchemaFixtures {
        /// Path to the manifest (default: tests/parity_corpus/generated_pydantic_errors/manifest.json).
        #[arg(
            long,
            default_value = "tests/parity_corpus/generated_pydantic_errors/manifest.json"
        )]
        manifest: PathBuf,
    },
    /// Static-grep gate: reject workflow-vocabulary identifiers
    /// in the semantic-cache implementation surfaces. The
    /// contribution rules forbid acceptance-criterion codes,
    /// milestone / sub-milestone tags, round identifiers,
    /// task identifiers, and the bare workflow noun from
    /// implementation code, comments, and runtime messages —
    /// see the lint module for the full forbidden-token set.
    /// Default scope is the five files the semantic-cache
    /// builder sequence touches; widening is a deliberate
    /// merge decision via `--root <path>`.
    LintWorkflowLabels {
        /// Files to scan. When empty, the default scan list
        /// (the five semantic-cache implementation surfaces)
        /// is used.
        #[arg(long)]
        root: Vec<PathBuf>,
    },
    /// Structural integrity check on the parity workflow. Verifies
    /// triggers, unconditional parity step, and no bypass patterns.
    LintWorkflowIntegrity,
    /// Audit the committed B1 semantic cache against its
    /// provenance sidecar. Verifies Hash C (cache content),
    /// the frozen 222-paraphrase + 5-negative-fixture count
    /// contract, model identity, and decision threshold. The
    /// cache is frozen (immutable) — Hash B and uv.lock are no
    /// longer verified.
    AuditSemanticCacheProvenance {
        /// Path to the committed cache file.
        #[arg(long, default_value = "out/semantic_cache.json")]
        cache: PathBuf,
        /// Path to the committed provenance sidecar.
        #[arg(long, default_value = "out/semantic_cache.provenance.json")]
        provenance: PathBuf,
        /// Historical uv.lock path (no longer verified; kept for CLI compat).
        #[arg(long, default_value = "/dev/null")]
        uv_lock: PathBuf,
    },
    /// Replay the committed semantic cache's stored scores against
    /// a swept range of decision thresholds. Emits the per-threshold
    /// (pass / fail / abstain) distribution as canonical JSON to
    /// stdout and a brief human-readable summary to stderr. Reads
    /// the provenance sidecar before emitting any rows and refuses
    /// to scan a cache whose committed counts disagree with the
    /// frozen QM label set. Does NOT re-run the embedding model —
    /// the sweep only walks the cache's committed score field.
    ThresholdSweep {
        /// Vertical to sweep. Only `qm` is supported (the only
        /// frozen paraphrase label set).
        #[arg(long, default_value = threshold_sweep::DEFAULT_VERTICAL)]
        vertical: String,
        /// Lower bound of the swept range (inclusive).
        #[arg(long, default_value_t = threshold_sweep::DEFAULT_FROM)]
        from: f64,
        /// Upper bound of the swept range (inclusive).
        #[arg(long, default_value_t = threshold_sweep::DEFAULT_TO)]
        to: f64,
        /// Step size between successive thresholds.
        #[arg(long, default_value_t = threshold_sweep::DEFAULT_STEP)]
        step: f64,
        /// Path to the committed cache to read scores from.
        #[arg(long, default_value = "out/semantic_cache.json")]
        cache: PathBuf,
        /// Path to the committed provenance sidecar used for the
        /// frozen-count preflight.
        #[arg(long, default_value = "out/semantic_cache.provenance.json")]
        provenance: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Some(Cmd::Gate { report }) => {
            eprintln!("xtask gate: running quality gate checks...");
            match gate::run_gate(report) {
                Ok(results) => {
                    let any_failed = results.iter().any(|r| r.status == gate::CheckStatus::Fail);
                    if report {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&results)
                                .expect("Invariant: CheckResult is always serializable")
                        );
                    }
                    let passed = results
                        .iter()
                        .filter(|r| r.status == gate::CheckStatus::Pass)
                        .count();
                    let failed = results
                        .iter()
                        .filter(|r| r.status == gate::CheckStatus::Fail)
                        .count();
                    let advisory = results
                        .iter()
                        .filter(|r| r.status == gate::CheckStatus::AdvisoryWarn)
                        .count();
                    eprintln!("xtask gate: {passed} passed, {failed} failed, {advisory} advisory");
                    if any_failed {
                        ExitCode::FAILURE
                    } else {
                        ExitCode::SUCCESS
                    }
                }
                Err(e) => {
                    eprintln!("xtask gate: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::Parity { corpus, out }) => match parity::run_parity(&corpus, &out) {
            Ok(report) => {
                eprintln!(
                    "xtask parity: {} entries; {} passed, {} failed, {} future-stage",
                    report.summary.total,
                    report.summary.passed,
                    report.summary.failed,
                    report.summary.future_stage,
                );
                for entry in &report.entries {
                    match &entry.status {
                        parity::EntryStatus::Pass => {
                            eprintln!("  PASS  {}", entry.name);
                        }
                        parity::EntryStatus::Fail(reason) => {
                            eprintln!("  FAIL  {}: {reason}", entry.name);
                            for c in &entry.comparisons {
                                if !c.equal {
                                    eprintln!(
                                        "        [{}] {}",
                                        c.name,
                                        c.diff_summary.as_deref().unwrap_or("")
                                    );
                                }
                            }
                        }
                        parity::EntryStatus::FutureStage(milestone) => {
                            eprintln!("  FUTURE({milestone})  {}", entry.name);
                        }
                    }
                }
                if report.summary.failed == 0 {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(err) => {
                eprintln!("xtask parity: {err}");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::Bench { check }) => {
            let mut ws = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            ws.pop();
            let benchmarks = ["bm25_iai", "verify_iai", "lint_iai", "index_iai"];
            let mut any_failed = false;
            for bench_name in &benchmarks {
                eprintln!("xtask bench: running {bench_name}...");
                let status = std::process::Command::new("cargo")
                    .args(["bench", "-p", "cacg-core", "--bench", bench_name])
                    .current_dir(&ws)
                    .status();
                match status {
                    Ok(s) if s.success() => {
                        eprintln!("xtask bench: {bench_name} PASS");
                    }
                    Ok(s) => {
                        eprintln!(
                            "xtask bench: {bench_name} FAIL (exit {})",
                            s.code().unwrap_or(-1)
                        );
                        any_failed = true;
                    }
                    Err(e) => {
                        eprintln!("xtask bench: {bench_name} failed to execute: {e}");
                        any_failed = true;
                    }
                }
            }
            if check {
                eprintln!("xtask bench --check: comparing against committed baselines...");
                match bench_check::check_baselines(&ws) {
                    Ok(results) => {
                        let check_failed = results.iter().any(|r| !r.passed);
                        if check_failed {
                            eprintln!("xtask bench --check: FAILED — regression detected");
                            any_failed = true;
                        } else {
                            eprintln!("xtask bench --check: PASS — all within threshold");
                        }
                    }
                    Err(e) => {
                        eprintln!("xtask bench --check: FAILED — {e}");
                        any_failed = true;
                    }
                }
            }
            if any_failed {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Some(Cmd::RetrievalEval { fixtures }) => match retrieval_eval::run(&fixtures) {
            Ok(true) => {
                eprintln!("xtask retrieval-eval: gate PASSED");
                ExitCode::SUCCESS
            }
            Ok(false) => {
                eprintln!(
                    "xtask retrieval-eval: gate FAILED — an expected hit dropped out of top-k"
                );
                ExitCode::FAILURE
            }
            Err(err) => {
                eprintln!("xtask retrieval-eval: {err:#}");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::SemanticEval { fixtures }) => match semantic_eval::run(&fixtures) {
            Ok(true) => {
                eprintln!("xtask semantic-eval: gate PASSED");
                ExitCode::SUCCESS
            }
            Ok(false) => {
                eprintln!(
                    "xtask semantic-eval: gate FAILED — a committed case's verdict diverged from expected"
                );
                ExitCode::FAILURE
            }
            Err(err) => {
                eprintln!("xtask semantic-eval: {err:#}");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::LintPlatformCfg { root }) => match lints::platform_cfg::lint(&root) {
            Ok(violations) if violations.is_empty() => {
                eprintln!(
                    "xtask lint-platform-cfg: 0 violations under {}",
                    root.display()
                );
                ExitCode::SUCCESS
            }
            Ok(violations) => {
                eprintln!(
                    "xtask lint-platform-cfg: {} violation(s) under {}",
                    violations.len(),
                    root.display()
                );
                for v in &violations {
                    eprintln!(
                        "  {}:{} [{}] {}",
                        v.file.display(),
                        v.line,
                        v.rule,
                        v.message
                    );
                }
                ExitCode::FAILURE
            }
            Err(err) => {
                eprintln!(
                    "xtask lint-platform-cfg: failed to walk {}: {err}",
                    root.display()
                );
                ExitCode::FAILURE
            }
        },
        Some(Cmd::LintDeterminism { root }) => {
            let root = if root.is_empty() {
                lints::determinism::default_scan_roots()
            } else {
                root
            };
            match lints::determinism::lint(&root) {
                Ok(violations) if violations.is_empty() => {
                    let scanned: Vec<String> =
                        root.iter().map(|p| p.display().to_string()).collect();
                    eprintln!(
                        "xtask lint-determinism: 0 violations under {}",
                        scanned.join(", ")
                    );
                    ExitCode::SUCCESS
                }
                Ok(violations) => {
                    eprintln!("xtask lint-determinism: {} violation(s)", violations.len());
                    for v in &violations {
                        eprintln!(
                            "  {}:{} [{}] {}",
                            v.file.display(),
                            v.line,
                            v.rule,
                            v.message
                        );
                    }
                    ExitCode::FAILURE
                }
                Err(err) => {
                    eprintln!("xtask lint-determinism: failed to walk: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::LintTrustLeak { root }) => {
            let root = if root.is_empty() {
                lints::trust_leak::default_scan_roots()
            } else {
                root
            };
            match lints::trust_leak::lint(&root) {
                Ok(violations) if violations.is_empty() => {
                    let scanned: Vec<String> =
                        root.iter().map(|p| p.display().to_string()).collect();
                    eprintln!(
                        "xtask lint-trust-leak: 0 violations under {}",
                        scanned.join(", ")
                    );
                    ExitCode::SUCCESS
                }
                Ok(violations) => {
                    eprintln!("xtask lint-trust-leak: {} violation(s)", violations.len());
                    for v in &violations {
                        eprintln!(
                            "  {}:{} [{}] {}",
                            v.file.display(),
                            v.line,
                            v.rule,
                            v.message
                        );
                    }
                    ExitCode::FAILURE
                }
                Err(err) => {
                    eprintln!("xtask lint-trust-leak: failed to walk: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::LintWorkflowLabels { root }) => {
            let files = if root.is_empty() {
                lints::workflow_labels::default_scan_files()
            } else {
                root
            };
            match lints::workflow_labels::lint(&files) {
                Ok(violations) if violations.is_empty() => {
                    let scanned: Vec<String> =
                        files.iter().map(|p| p.display().to_string()).collect();
                    eprintln!(
                        "xtask lint-workflow-labels: 0 violations across {} file(s) ({})",
                        files.len(),
                        scanned.join(", ")
                    );
                    ExitCode::SUCCESS
                }
                Ok(violations) => {
                    eprintln!(
                        "xtask lint-workflow-labels: {} violation(s)",
                        violations.len()
                    );
                    for v in &violations {
                        eprintln!(
                            "  {}:{} [{}] {}",
                            v.file.display(),
                            v.line,
                            v.rule,
                            v.message
                        );
                    }
                    ExitCode::FAILURE
                }
                Err(err) => {
                    eprintln!("xtask lint-workflow-labels: failed to scan: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::LintWorkflowIntegrity) => {
            let mut ws = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            ws.pop();
            match lints::workflow_integrity::lint(&ws) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{e}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::LintRunnerBypass { root }) => {
            let root = if root.is_empty() {
                lints::runner_bypass::default_scan_roots()
            } else {
                root
            };
            match lints::runner_bypass::lint(&root) {
                Ok(violations) if violations.is_empty() => {
                    let scanned: Vec<String> =
                        root.iter().map(|p| p.display().to_string()).collect();
                    eprintln!(
                        "xtask lint-runner-bypass: 0 violations under {}",
                        scanned.join(", ")
                    );
                    ExitCode::SUCCESS
                }
                Ok(violations) => {
                    eprintln!(
                        "xtask lint-runner-bypass: {} violation(s)",
                        violations.len()
                    );
                    for v in &violations {
                        eprintln!(
                            "  {}:{} [{}] {}",
                            v.file.display(),
                            v.line,
                            v.rule,
                            v.message
                        );
                    }
                    ExitCode::FAILURE
                }
                Err(err) => {
                    eprintln!("xtask lint-runner-bypass: failed to walk: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::LintRenameOutsidePublisher { root }) => {
            let root = if root.is_empty() {
                lints::rename_outside_publisher::default_scan_roots()
            } else {
                root
            };
            match lints::rename_outside_publisher::lint(&root) {
                Ok(violations) if violations.is_empty() => {
                    let scanned: Vec<String> =
                        root.iter().map(|p| p.display().to_string()).collect();
                    eprintln!(
                        "xtask lint-rename-outside-publisher: 0 violations under {}",
                        scanned.join(", ")
                    );
                    ExitCode::SUCCESS
                }
                Ok(violations) => {
                    eprintln!(
                        "xtask lint-rename-outside-publisher: {} violation(s)",
                        violations.len()
                    );
                    for v in &violations {
                        eprintln!(
                            "  {}:{} [{}] {}",
                            v.file.display(),
                            v.line,
                            v.rule,
                            v.message
                        );
                    }
                    ExitCode::FAILURE
                }
                Err(err) => {
                    eprintln!("xtask lint-rename-outside-publisher: failed to walk: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::LintUnwrap { root }) => {
            let root = if root.is_empty() {
                lints::unwrap::default_scan_roots()
            } else {
                root
            };
            match lints::unwrap::lint(&root) {
                Ok(violations) if violations.is_empty() => {
                    let scanned: Vec<String> =
                        root.iter().map(|p| p.display().to_string()).collect();
                    eprintln!(
                        "xtask lint-unwrap: 0 violations under {}",
                        scanned.join(", ")
                    );
                    ExitCode::SUCCESS
                }
                Ok(violations) => {
                    eprintln!("xtask lint-unwrap: {} violation(s)", violations.len());
                    for v in &violations {
                        eprintln!(
                            "  {}:{} [{}] {}",
                            v.file.display(),
                            v.line,
                            v.rule,
                            v.message
                        );
                    }
                    ExitCode::FAILURE
                }
                Err(err) => {
                    eprintln!("xtask lint-unwrap: failed to walk: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::LintErrorSwallow { root }) => {
            let root = if root.is_empty() {
                lints::error_swallow::default_scan_roots()
            } else {
                root
            };
            match lints::error_swallow::lint(&root) {
                Ok(violations) if violations.is_empty() => {
                    let scanned: Vec<String> =
                        root.iter().map(|p| p.display().to_string()).collect();
                    eprintln!(
                        "xtask lint-error-swallow: 0 violations under {}",
                        scanned.join(", ")
                    );
                    ExitCode::SUCCESS
                }
                Ok(violations) => {
                    eprintln!(
                        "xtask lint-error-swallow: {} violation(s)",
                        violations.len()
                    );
                    for v in &violations {
                        eprintln!(
                            "  {}:{} [{}] {}",
                            v.file.display(),
                            v.line,
                            v.rule,
                            v.message
                        );
                    }
                    ExitCode::FAILURE
                }
                Err(err) => {
                    eprintln!("xtask lint-error-swallow: failed to walk: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::LintStructural { root, warn }) => {
            let root = if root.is_empty() {
                lints::structural::default_scan_roots()
            } else {
                root
            };
            match lints::structural::lint(&root, warn) {
                Ok((violations, should_fail)) => {
                    if violations.is_empty() {
                        let scanned: Vec<String> =
                            root.iter().map(|p| p.display().to_string()).collect();
                        eprintln!(
                            "xtask lint-structural: 0 violations under {}",
                            scanned.join(", ")
                        );
                        ExitCode::SUCCESS
                    } else {
                        let mode = if warn { "advisory" } else { "blocking" };
                        eprintln!(
                            "xtask lint-structural: {} violation(s) ({})",
                            violations.len(),
                            mode
                        );
                        for v in &violations {
                            eprintln!(
                                "  {}:{} [{}] {}",
                                v.file.display(),
                                v.line,
                                v.rule,
                                v.message
                            );
                        }
                        if should_fail {
                            ExitCode::FAILURE
                        } else {
                            ExitCode::SUCCESS
                        }
                    }
                }
                Err(err) => {
                    eprintln!("xtask lint-structural: failed to walk: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(Cmd::AuditCacgCoreDeps) => match cacg_core_deps::audit_current_workspace() {
            Ok(violations) if violations.is_empty() => {
                eprintln!(
                        "xtask audit-cacg-core-deps: 0 forbidden packages in cacg-core's resolved dep closure"
                    );
                ExitCode::SUCCESS
            }
            Ok(violations) => {
                eprintln!(
                        "xtask audit-cacg-core-deps: {} forbidden package(s) in cacg-core's resolved dep closure",
                        violations.len()
                    );
                for v in &violations {
                    eprintln!(
                        "  [{}] {}\n    chain: {}",
                        v.package,
                        v.reason,
                        v.chain_hint.join(" -> ")
                    );
                }
                ExitCode::FAILURE
            }
            Err(err) => {
                eprintln!("xtask audit-cacg-core-deps: {err}");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::AuditDefaultKbDeps) => match audit_default_kb_deps::audit_current_workspace() {
            Ok(violations) if violations.is_empty() => {
                eprintln!(
                        "xtask audit-default-kb-deps: 0 forbidden packages in cacg-cli's resolved default-features closure"
                    );
                ExitCode::SUCCESS
            }
            Ok(violations) => {
                eprintln!(
                        "xtask audit-default-kb-deps: {} forbidden package(s) in cacg-cli's resolved default-features closure",
                        violations.len()
                    );
                for v in &violations {
                    eprintln!(
                        "  [{}] {}\n    chain: {}",
                        v.package,
                        v.reason,
                        v.chain_hint.join(" -> ")
                    );
                }
                ExitCode::FAILURE
            }
            Err(err) => {
                eprintln!("xtask audit-default-kb-deps: {err}");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::GenSchemaFixtures) => match schema_fixtures::gen() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("xtask gen-schema-fixtures: {e}");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::AuditSchemaFixtures { manifest }) => match schema_fixtures::audit(&manifest) {
            Ok(report) => {
                eprintln!(
                        "xtask audit-schema-fixtures: {} fixtures clean ({} parse-layer, {} lint-layer)",
                        report.total, report.parse_count, report.lint_count
                    );
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("xtask audit-schema-fixtures: {e}");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::AuditSemanticCacheProvenance {
            cache,
            provenance,
            uv_lock,
        }) => match semantic_cache_provenance::audit(&cache, &provenance, &uv_lock) {
            Ok(report) => {
                eprintln!(
                    "xtask audit-semantic-cache-provenance: {} entries clean ({} paraphrase + {} negative), \
                     hash_b={}..., hash_c={}...",
                    report.entry_count,
                    report.paraphrase_count,
                    report.negative_fixture_count,
                    &report.hash_b[..12],
                    &report.hash_c[..12],
                );
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("xtask audit-semantic-cache-provenance: {e:#}");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::ThresholdSweep {
            vertical,
            from,
            to,
            step,
            cache,
            provenance,
        }) => match threshold_sweep::run(&cache, &provenance, &vertical, from, to, step) {
            Ok(report) => match threshold_sweep::emit_canonical_stdout(&report) {
                Ok(()) => {
                    threshold_sweep::emit_human_summary(&report);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("xtask threshold-sweep: {e:#}");
                    ExitCode::FAILURE
                }
            },
            Err(e) => {
                eprintln!("xtask threshold-sweep: {e:#}");
                ExitCode::FAILURE
            }
        },
        None => {
            eprintln!(
                "xtask: no subcommand given. Available: parity, bench, \
                 lint-platform-cfg, lint-rename-outside-publisher, \
                 lint-determinism, lint-trust-leak, lint-runner-bypass, \
                 gen-schema-fixtures, audit-schema-fixtures, \
                 audit-cacg-core-deps."
            );
            ExitCode::SUCCESS
        }
    }
}
