//! Oracle ↔ Rust byte-diff parity harness.
//!
//! Compares Rust `cacg-cli` output against committed oracle fixtures
//! (or live Python `cacg.cli` for unconverted rows) under
//! `KB_FROZEN_CLOCK=1` and byte-compares every artifact in the staged
//! matrix. Returns non-zero if any matrix entry that's actually
//! gating (status `Fail`) reports a diff or a missing/extra artifact.
//!
//! Future-stage matrix entries (`Stage::Future`) are reported in the
//! perf JSON but do not gate the overall command's exit code; they
//! document upcoming milestone work without false-flagging during
//! the milestone they belong to.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use serde::Serialize;

mod diff_reporter;

pub use diff_reporter::{field_level_diff, StructuredDiff};

/// One artifact comparison within an entry report.
#[derive(Debug, Clone, Serialize)]
pub struct ArtifactComparison {
    /// Caller-friendly identifier (e.g., `cards_manifest.json`).
    pub name: String,
    /// Path to the expected (oracle/committed-fixture) artifact.
    pub expected_path: PathBuf,
    /// Path to the Rust-emitted artifact.
    pub rust_path: PathBuf,
    /// Whether the expected artifact exists on disk.
    pub expected_exists: bool,
    /// Whether the Rust artifact exists on disk.
    pub rust_exists: bool,
    /// Expected artifact byte count (0 if absent).
    pub expected_bytes: usize,
    /// Rust artifact byte count (0 if absent).
    pub rust_bytes: usize,
    /// True iff both exist AND bytes match.
    pub equal: bool,
    /// Short human-readable diff hint for non-equal comparisons.
    /// Preserved as the legacy byte-offset summary string so the
    /// perf-report JSON schema stays backward-compatible.
    pub diff_summary: Option<String>,
    /// Structured field-level diff payload. `None` when artifacts are
    /// byte-equal; otherwise carries the kind / path / per-side values
    /// / unified-diff snippet produced by `field_level_diff`, or one
    /// of the missing-side variants when an artifact is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_detail: Option<StructuredDiff>,
}

/// Stage label so the staged matrix can include not-yet-implemented
/// entries with explicit future-milestone markers instead of silent
/// success.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "stage", content = "milestone")]
#[allow(dead_code)] // Future variant reserved for M3+ matrix entries.
pub enum Stage {
    /// Gated by this milestone — a diff returns non-zero exit.
    M2,
    /// Reported but not gating yet; will gate at the named milestone.
    Future(&'static str),
}

/// Per-entry status after the harness compares all artifacts.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "status", content = "reason")]
#[allow(dead_code)] // FutureStage variant reserved for M3+ matrix entries.
pub enum EntryStatus {
    /// All comparisons equal.
    Pass,
    /// One or more comparisons diffed or a process failed.
    Fail(String),
    /// Stage is `Future(...)`; reported but not gating.
    FutureStage(&'static str),
}

/// One matrix entry's run report.
#[derive(Debug, Clone, Serialize)]
pub struct EntryReport {
    /// Caller-friendly entry name.
    pub name: String,
    /// Stage label.
    pub stage: Stage,
    /// Expected-side source rendered as a shell-style string for the report.
    pub expected_command: String,
    /// Rust command rendered as a shell-style string for the report.
    pub rust_command: String,
    /// Expected-side wall-clock duration in milliseconds (0 for committed fixtures).
    pub expected_duration_ms: u128,
    /// Rust invocation wall-clock duration in milliseconds.
    pub rust_duration_ms: u128,
    /// Per-artifact comparisons.
    pub comparisons: Vec<ArtifactComparison>,
    /// Aggregate status (Pass / Fail / FutureStage).
    pub status: EntryStatus,
}

/// Top-level report emitted by [`run_parity`].
#[derive(Debug, Clone, Serialize)]
pub struct ParityReport {
    /// ISO-8601 timestamp the report was emitted at (frozen under
    /// `KB_FROZEN_CLOCK=1`).
    pub timestamp: String,
    /// Corpus directory the harness ran against.
    pub corpus: PathBuf,
    /// One entry per matrix row.
    pub entries: Vec<EntryReport>,
    /// Aggregate counts.
    pub summary: Summary,
}

/// Aggregate counts across all entries.
#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    /// Total matrix entries executed.
    pub total: usize,
    /// Entries with `EntryStatus::Pass`.
    pub passed: usize,
    /// Entries with `EntryStatus::Fail`.
    pub failed: usize,
    /// Entries with `EntryStatus::FutureStage`.
    pub future_stage: usize,
}

/// Errors raised by the harness orchestrator.
#[derive(Debug)]
pub enum ParityError {
    /// The corpus directory does not exist or is not a directory.
    NonDirCorpus(PathBuf),
    /// I/O failure constructing tempdirs or writing the report.
    Io(io::Error),
    /// Serializing the report to JSON failed.
    Json(serde_json::Error),
}

impl std::fmt::Display for ParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonDirCorpus(p) => write!(f, "corpus path {p:?} is not a directory"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
        }
    }
}

impl std::error::Error for ParityError {}

/// Byte-compare two files. Both paths may or may not exist on disk;
/// the returned struct records the absence + the diff. Populates both
/// the legacy `diff_summary` byte-offset string AND the new structured
/// `diff_detail` payload so the harness can carry rich field-level
/// diagnostics without breaking the perf-report schema.
pub fn compare_artifact(
    name: impl Into<String>,
    expected_path: &Path,
    rust_path: &Path,
) -> ArtifactComparison {
    let name = name.into();
    let expected_bytes_opt = fs::read(expected_path).ok();
    let rs_bytes_opt = fs::read(rust_path).ok();
    let expected_exists = expected_bytes_opt.is_some();
    let rust_exists = rs_bytes_opt.is_some();
    let expected_bytes = expected_bytes_opt.as_ref().map(Vec::len).unwrap_or(0);
    let rust_bytes = rs_bytes_opt.as_ref().map(Vec::len).unwrap_or(0);
    let (equal, diff_summary, diff_detail) = match (&expected_bytes_opt, &rs_bytes_opt) {
        (Some(py), Some(rs)) if py == rs => (true, None, None),
        (Some(py), Some(rs)) => {
            let first_diff = py
                .iter()
                .zip(rs.iter())
                .position(|(a, b)| a != b)
                .unwrap_or_else(|| py.len().min(rs.len()));
            let end = (first_diff + 32).min(py.len()).min(rs.len());
            let summary = format!(
                "first divergence at byte offset {first_diff} (python_len={}, rust_len={}): python={:?} rust={:?}",
                py.len(),
                rs.len(),
                py.get(first_diff..end).unwrap_or(&[]),
                rs.get(first_diff..end).unwrap_or(&[]),
            );
            let detail = field_level_diff(&name, py, rs);
            let detail_opt = if matches!(detail, StructuredDiff::ByteEqual) {
                None
            } else {
                Some(detail)
            };
            (false, Some(summary), detail_opt)
        }
        (None, Some(_)) => (
            false,
            Some(format!("python artifact missing at {expected_path:?}")),
            Some(StructuredDiff::MissingExpected {
                rust_path: rust_path.to_string_lossy().to_string(),
            }),
        ),
        (Some(_), None) => (
            false,
            Some(format!("rust artifact missing at {rust_path:?}")),
            Some(StructuredDiff::MissingRust {
                expected_path: expected_path.to_string_lossy().to_string(),
            }),
        ),
        (None, None) => (
            false,
            Some(format!(
                "both artifacts missing (python: {expected_path:?}, rust: {rust_path:?})"
            )),
            Some(StructuredDiff::BothMissing {
                expected_path: expected_path.to_string_lossy().to_string(),
                rust_path: rust_path.to_string_lossy().to_string(),
            }),
        ),
    };
    ArtifactComparison {
        name,
        expected_path: expected_path.to_path_buf(),
        rust_path: rust_path.to_path_buf(),
        expected_exists,
        rust_exists,
        expected_bytes,
        rust_bytes,
        equal,
        diff_summary,
        diff_detail,
    }
}

fn workspace_root_for_runtime() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p
}

fn python_exe(workspace_root: &Path) -> PathBuf {
    // Post-quarantine resolver: the legacy Python oracle venv is expected at
    // `legacy_python_oracle/.venv/bin/python`. Create it via
    // `python3 -m venv legacy_python_oracle/.venv && legacy_python_oracle/.venv/bin/pip install -e ./legacy_python_oracle[dev]`.
    // No silent fallback to system `python3` — if the venv is missing,
    // `Command::new` will surface a clear spawn error and the parity row
    // will fail visibly rather than running against an unprovisioned interpreter.
    workspace_root.join("legacy_python_oracle/.venv/bin/python")
}

fn committed_fixture_dir(corpus: &Path, row_name: &str) -> PathBuf {
    corpus
        .join("out_python/parity_rows")
        .join(row_name)
        .join("py-out")
}

/// Run the staged parity matrix against `corpus` and write the report
/// to `out_root/perf-reports/parity-<timestamp>.json`. Returns `Ok`
/// regardless of whether individual entries failed — the caller
/// inspects [`ParityReport::summary`] to derive the exit code.
pub fn run_parity(corpus: &Path, out_root: &Path) -> Result<ParityReport, ParityError> {
    if !corpus.is_dir() {
        return Err(ParityError::NonDirCorpus(corpus.to_path_buf()));
    }
    let workspace_root = workspace_root_for_runtime();
    fs::create_dir_all(out_root).map_err(ParityError::Io)?;
    // Each side runs from its OWN isolated corpus root as cwd, so the
    // `--out` argument MUST be absolute — otherwise the subprocess
    // would resolve it against its (changed) cwd and write artifacts
    // outside the harness's enumeration window.
    let out_root = fs::canonicalize(out_root).map_err(ParityError::Io)?;
    let perf_dir = workspace_root.join("perf-reports");
    fs::create_dir_all(&perf_dir).map_err(ParityError::Io)?;

    let timestamp = "1970-01-01T00:00:00Z".to_string();

    let python_cmd = IndexCommand {
        executable: python_exe(&workspace_root),
        leading_args: vec![
            "-m".to_string(),
            "cacg.cli".to_string(),
            "index".to_string(),
        ],
    };
    let rust_cmd = IndexCommand {
        executable: PathBuf::from("cargo"),
        leading_args: vec![
            "run".to_string(),
            "--quiet".to_string(),
            "-p".to_string(),
            "cacg-cli".to_string(),
            "--".to_string(),
            "index".to_string(),
        ],
    };

    // Row-table-driven matrix. Each `MatrixRow` declares its name,
    // its gating stage, and its kind (which dispatches the per-row
    // body to either the kb-index path or the no-op future-smoke
    // path). Future rows are added by appending a `MatrixRow` entry
    // to this table; the inner loop body does not change per row.
    let matrix: Vec<MatrixRow> = vec![
        MatrixRow {
            name: "kb_index_parity_corpus_reading_01",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIndex {
                corpus_subdir: "cards/reading_01",
            },
        },
        // Stale-hash sibling row: a card whose stored frontmatter
        // `card_hash` is wrong forces both implementations to rewrite
        // the card and emit a per-card `.history.jsonl` sidecar with
        // a documented `prev_card_hash` value. The live byte-compare
        // covers the sidecar emission path that the clean-hash row
        // skips entirely.
        MatrixRow {
            name: "kb_index_parity_corpus_stale_hash_reading_01",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIndex {
                corpus_subdir: "cards_stale_hash/reading_01",
            },
        },
        // task-vh-7: lint byte parity on the committed golden corpus.
        // Every card under `tests/parity_corpus/valid/` must lint
        // identically through Python and Rust: byte-equal stderr,
        // byte-equal lint journal append, matching exit code.
        MatrixRow {
            name: "kb_lint_parity_golden",
            stage: Stage::M2,
            kind: MatrixRowKind::KbLint {
                corpus_subdir: "valid",
                chunks_manifest: "tests/parity_corpus/out_python/chunks_manifest.json",
                source_matrix: "tests/parity_corpus/out_python/source_matrix.json",
            },
        },
        // task-vh-7: lint byte parity on the adversarial corpus. Each
        // card in `tests/parity_corpus/adversarial/` must surface its
        // documented diagnostic byte-equally on both sides.
        MatrixRow {
            name: "kb_lint_parity_adversarial",
            stage: Stage::M2,
            kind: MatrixRowKind::KbLint {
                corpus_subdir: "adversarial",
                chunks_manifest: "tests/parity_corpus/out_python/chunks_manifest.json",
                source_matrix: "tests/parity_corpus/out_python/source_matrix.json",
            },
        },
        // kb verify CLI-dispatcher byte parity. Each row runs the
        // committed `valid/` corpus through Python and Rust
        // `kb verify` under one flag combination and byte-compares
        // stdout / stderr / exit / verify journal. The fuzzy
        // ALGORITHM divergence surface is oracle-tested in
        // cacg-core (`fuzzy_match` + `verify_card` parity oracles);
        // these rows pin the dispatcher + flag wiring.
        MatrixRow {
            name: "kb_verify_parity_golden",
            stage: Stage::M2,
            kind: MatrixRowKind::KbVerify {
                corpus_subdir: "valid",
                chunks_manifest: "tests/parity_corpus/out_python/chunks_manifest.json",
                source_matrix: "tests/parity_corpus/out_python/source_matrix.json",
                fuzzy: false,
                skip_lint: false,
            },
        },
        MatrixRow {
            name: "kb_verify_fuzzy_parity",
            stage: Stage::M2,
            kind: MatrixRowKind::KbVerify {
                corpus_subdir: "valid",
                chunks_manifest: "tests/parity_corpus/out_python/chunks_manifest.json",
                source_matrix: "tests/parity_corpus/out_python/source_matrix.json",
                fuzzy: true,
                skip_lint: false,
            },
        },
        MatrixRow {
            name: "kb_verify_skip_lint_parity",
            stage: Stage::M2,
            kind: MatrixRowKind::KbVerify {
                corpus_subdir: "valid",
                chunks_manifest: "tests/parity_corpus/out_python/chunks_manifest.json",
                source_matrix: "tests/parity_corpus/out_python/source_matrix.json",
                fuzzy: false,
                skip_lint: true,
            },
        },
        // `kb verify --round-summary` per-fixture byte parity. Iterates
        // the committed `.md` fixtures under
        // `tests/round_summary_fixtures/` and asserts Python and Rust
        // `kb verify --round-summary <fixture>` emit byte-equal
        // stdout, stderr, exit, and per-fixture verify journal under
        // `KB_FROZEN_CLOCK=1`. The same fixture set is committed-
        // oracle-tested in-process by
        // `crates/cacg-cli/tests/round_summary_fixtures_parity.rs`;
        // this row is the live Python-vs-Rust gate.
        MatrixRow {
            name: "kb_verify_round_summary_parity_golden",
            stage: Stage::M2,
            kind: MatrixRowKind::KbVerifyRoundSummary {
                fixtures_dir: "tests/round_summary_fixtures",
                chunks_manifest: "tests/parity_corpus/out_python/chunks_manifest.json",
                source_matrix: "tests/parity_corpus/out_python/source_matrix.json",
            },
        },
        // Single-card `kb verify --semantic <cache>` byte parity.
        // The committed fixture under `tests/parity_corpus/semantic/`
        // (regenerated by `legacy_python_oracle/scripts/build_semantic_parity_fixture.py`)
        // pins one card whose quote is NOT a substring of the cited
        // chunk so Layer-2 emits `CACG-VERIFY-001` and Layer-3 fires;
        // the cache holds one entry for the matching
        // `(chunk_hash, claim_window_hash)` pair with `verdict=fail`
        // so Python and Rust agree on severity = error.
        MatrixRow {
            name: "kb_verify_semantic_parity_golden",
            stage: Stage::M2,
            kind: MatrixRowKind::KbVerifySemantic {
                card: "tests/parity_corpus/semantic/card.md",
                chunks_manifest: "tests/parity_corpus/out_python/chunks_manifest.json",
                source_matrix: "tests/parity_corpus/out_python/source_matrix.json",
                semantic_cache: "tests/parity_corpus/semantic/semantic_cache.json",
            },
        },
        // Cache-miss / abstain companion row. Same card as the
        // golden row above, but the cache's `entries` array is
        // empty so every lookup returns the abstain-embedding-cache
        // sentinel. Python and Rust both emit `CACG-VERIFY-002`
        // severity = info on cache miss, matching the per-verdict
        // severity contract for the `abstain` verdict.
        MatrixRow {
            name: "kb_verify_semantic_miss_parity_golden",
            stage: Stage::M2,
            kind: MatrixRowKind::KbVerifySemantic {
                card: "tests/parity_corpus/semantic/card.md",
                chunks_manifest: "tests/parity_corpus/out_python/chunks_manifest.json",
                source_matrix: "tests/parity_corpus/out_python/source_matrix.json",
                semantic_cache: "tests/parity_corpus/semantic/semantic_cache_empty.json",
            },
        },
        // Round-summary batch + semantic byte parity. The committed
        // summary cites the semantic-parity-card, whose pinned quote
        // does NOT appear in the chunk text, so Layer-2 emits
        // CACG-VERIFY-001 on both implementations. The cached fail
        // entry for the resulting (chunk_hash, claim_window_hash)
        // pair produces a byte-identical CACG-VERIFY-002 line on
        // stderr. This is the batch counterpart of the
        // kb_verify_semantic_parity_golden row.
        MatrixRow {
            name: "kb_verify_round_summary_semantic_parity_golden",
            stage: Stage::M2,
            kind: MatrixRowKind::KbVerifyRoundSummarySemantic {
                summary: "tests/parity_corpus/round_summary_semantic/summary.md",
                chunks_manifest: "tests/parity_corpus/out_python/chunks_manifest.json",
                source_matrix: "tests/parity_corpus/out_python/source_matrix.json",
                semantic_cache: "tests/parity_corpus/semantic/semantic_cache.json",
            },
        },
        // `kb search` CLI-dispatcher byte parity. Runs Python and Rust
        // `kb search` over the committed sidecar-free corpus
        // `tests/parity_corpus/kb_search/` (a `summaries.json` triplet
        // with no `summaries.sqlite`, so both sides take the in-memory
        // BM25 backend) for each committed query case, byte-comparing
        // stdout / stderr / exit. The committed-oracle integration test
        // `crates/cacg-cli/tests/kb_search.rs` pins the same query set
        // against captured Python bytes; this row is the live
        // Python-vs-Rust gate.
        MatrixRow {
            name: "kb_search_parity_corpus",
            stage: Stage::M2,
            kind: MatrixRowKind::KbSearch {
                corpus_subdir: "kb_search",
                cases: KB_SEARCH_PARITY_CASES,
            },
        },
        // `kb search` byte parity over the CFA first-bite corpus
        // `tests/parity_corpus/cfa_first_bite/` — equity-valuation cards
        // sourced from Damodaran's "Investment Valuation" and indexed via
        // `kb index` (built by `legacy_python_oracle/scripts/build_cfa_first_bite_corpus.py`).
        // Sidecar-free, so both sides take the in-memory backend. This is
        // the second half of the plan's `kb_search_parity` requirement:
        // live parity over the parity corpus AND the CFA first-bite corpus.
        MatrixRow {
            name: "kb_search_parity_cfa_first_bite",
            stage: Stage::M2,
            kind: MatrixRowKind::KbSearch {
                corpus_subdir: "cfa_first_bite",
                cases: KB_SEARCH_CFA_FIRST_BITE_CASES,
            },
        },
        // `kb search` byte parity over a corpus carrying a FRESH SEALED
        // `summaries.sqlite` (`tests/parity_corpus/kb_search_fts5/`), so
        // both implementations take the FTS5 sidecar path. Python's
        // system SQLite and Rust's `bundled` SQLite read the same sealed
        // sidecar and produce byte-equal `bm25()`-ranked output.
        MatrixRow {
            name: "kb_search_parity_fts5_present",
            stage: Stage::M2,
            kind: MatrixRowKind::KbSearch {
                corpus_subdir: "kb_search_fts5",
                cases: KB_SEARCH_CFA_FIRST_BITE_CASES,
            },
        },
        // `kb search` byte parity over a corpus whose `summaries.sqlite`
        // seal is STALE (`tests/parity_corpus/kb_search_fts5_stale/` —
        // its `summaries.json` was mutated after the sidecar was sealed).
        // Both implementations emit the byte-equal `CACG-FTS-001`
        // seal-mismatch line and fall back to the in-memory BM25 backend.
        MatrixRow {
            name: "kb_search_parity_fts5_stale",
            stage: Stage::M2,
            kind: MatrixRowKind::KbSearch {
                corpus_subdir: "kb_search_fts5_stale",
                cases: KB_SEARCH_CFA_FIRST_BITE_CASES,
            },
        },
        // `kb show` CLI-dispatcher byte parity. Runs Python and Rust
        // `kb show` over the `kb_show` retraction fixture (built by
        // `legacy_python_oracle/scripts/build_kb_show_corpus.py`) and the `cfa_first_bite`
        // corpus, covering the active / directly-retracted /
        // dependency-retracted / missing-card / `--path` / unauthorized
        // / CFA search-to-show scenarios, byte-comparing stdout /
        // stderr / exit. The `CACG-SHOW-003` `--path` traversal
        // rejection is now byte-equal across both implementations and
        // is asserted directly in `crates/cacg-cli/tests/kb_show.rs`.
        MatrixRow {
            name: "kb_show_parity",
            stage: Stage::M2,
            kind: MatrixRowKind::KbShow {
                cases: KB_SHOW_PARITY_CASES,
            },
        },
        // task-m4-6 / AC-5 Pdfium parity gate (resolved BYTE-EQUAL in
        // Round 9, locked here in the standard parity matrix). Runs
        // Rust `kb ingest` on the committed cfa_vol1_trim.pdf fixture
        // under `KB_FROZEN_CLOCK=1`; byte-compares the published
        // chunks_manifest.json against the committed Python oracle and
        // sources_manifest.json against the same oracle modulo the
        // DEC-2 whitelist (`parser_name` / `parser_version`).
        MatrixRow {
            name: "kb_ingest_parity_cfa_vol1_trim",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIngest {
                pdf: "tests/parity_corpus/pdfs/cfa_vol1_trim.pdf",
                oracle_dir: "tests/parity_corpus/out_python/pdfs/cfa_vol1_trim",
                source_id: "cfa_vol1_trim",
            },
        },
        // task-m4-7 / AC-6: the M4b real-ingest pilot fixtures. Each
        // committed `qm_*_trim.pdf` is a small, image-stripped,
        // pikepdf-built trim of one sibling-repo source PDF
        // (`legacy_python_oracle/scripts/build_qm_trim_fixtures.py`). Together the four
        // trims cover every primary-source citation in the 17
        // `01_quantitative_methods/` cards: notes (13 cards), Greene
        // (1), AFTS (1), ESLII (2). M4-Round-15 review found the
        // initial 13-of-17 scope insufficient and extended to all 17.
        MatrixRow {
            name: "kb_ingest_parity_qm_notes_trim",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIngest {
                pdf: "tests/parity_corpus/pdfs/qm_notes_trim.pdf",
                oracle_dir: "tests/parity_corpus/out_python/pdfs/qm_notes_trim",
                source_id: "qm_notes_trim",
            },
        },
        MatrixRow {
            name: "kb_ingest_parity_qm_greene_trim",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIngest {
                pdf: "tests/parity_corpus/pdfs/qm_greene_trim.pdf",
                oracle_dir: "tests/parity_corpus/out_python/pdfs/qm_greene_trim",
                source_id: "qm_greene_trim",
            },
        },
        MatrixRow {
            name: "kb_ingest_parity_qm_afts_trim",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIngest {
                pdf: "tests/parity_corpus/pdfs/qm_afts_trim.pdf",
                oracle_dir: "tests/parity_corpus/out_python/pdfs/qm_afts_trim",
                source_id: "qm_afts_trim",
            },
        },
        MatrixRow {
            name: "kb_ingest_parity_qm_eslii_ch3_trim",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIngest {
                pdf: "tests/parity_corpus/pdfs/qm_eslii_ch3_trim.pdf",
                oracle_dir: "tests/parity_corpus/out_python/pdfs/qm_eslii_ch3_trim",
                source_id: "qm_eslii_ch3_trim",
            },
        },
        MatrixRow {
            name: "kb_ingest_parity_qm_eslii_ch7_trim",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIngest {
                pdf: "tests/parity_corpus/pdfs/qm_eslii_ch7_trim.pdf",
                oracle_dir: "tests/parity_corpus/out_python/pdfs/qm_eslii_ch7_trim",
                source_id: "qm_eslii_ch7_trim",
            },
        },
        // Per-subcommand semantic-help snapshot rows. Each compares
        // Python argparse's exported subcommand tree against Rust
        // clap's introspected tree for the same subcommand. Status is
        // always `FutureStage("M3")` so divergences appear in the
        // perf-report JSON without gating the harness's exit code.
    ];

    let mut entries: Vec<EntryReport> = Vec::with_capacity(matrix.len());
    for row in &matrix {
        entries.push(run_matrix_row(
            row,
            corpus,
            &out_root,
            &workspace_root,
            &python_cmd,
            &rust_cmd,
        )?);
    }

    let summary = build_summary(&entries);
    let report = ParityReport {
        timestamp: timestamp.clone(),
        corpus: corpus.to_path_buf(),
        entries,
        summary,
    };

    let json = serde_json::to_string_pretty(&report).map_err(ParityError::Json)?;
    let report_path = perf_dir.join(format!("parity-{timestamp}.json"));
    fs::write(&report_path, json).map_err(ParityError::Io)?;

    Ok(report)
}

/// Whether `name` should be byte-compared between Python and Rust
/// outputs. Filters out flock sidecars (`.lock`), atomic-publish
/// residuals (`.tmp`, `.bak`), the FTS5 SQLite sidecar (query-rowset
/// byte diff is queued for a later milestone), and Python's
/// `.kb_index_cache.json` LRU cache (a Python-side implementation
/// detail that Rust does not emit).
fn is_audited_artifact_name(name: &str) -> bool {
    if name.ends_with(".lock") || name.ends_with(".tmp") || name.ends_with(".bak") {
        return false;
    }
    if name == "summaries.sqlite" || name == ".kb_index_cache.json" {
        return false;
    }
    true
}

/// One side of the parity invocation: the executable + the prefix
/// args. The orchestrator appends `<corpus> --out <out>` after the
/// leading args before spawning the subprocess. Threading this as
/// an explicit value lets the orchestrator tests substitute shell-
/// script mocks for the real `python` and `cargo` binaries.
#[derive(Debug, Clone)]
struct IndexCommand {
    executable: PathBuf,
    leading_args: Vec<String>,
}

impl IndexCommand {
    fn render(&self, corpus: &Path, out_dir: &Path) -> String {
        let mut s = self.executable.display().to_string();
        for a in &self.leading_args {
            s.push(' ');
            s.push_str(a);
        }
        s.push(' ');
        s.push_str(&corpus.display().to_string());
        s.push_str(" --out ");
        s.push_str(&out_dir.display().to_string());
        s
    }

    fn spawn_status(
        &self,
        corpus: &Path,
        out_dir: &Path,
        cwd: &Path,
    ) -> io::Result<std::process::ExitStatus> {
        Command::new(&self.executable)
            .args(&self.leading_args)
            .arg(corpus)
            .arg("--out")
            .arg(out_dir)
            .env("KB_FROZEN_CLOCK", "1")
            .current_dir(cwd)
            .status()
    }
}

/// One row in the parity matrix. The row table is built inline in
/// [`run_parity`] and each row dispatches to its per-kind body via
/// [`run_matrix_row`]. Adding a new row is a one-line append to that
/// table; the inner loop body does not change per row.
#[derive(Debug, Clone)]
struct MatrixRow {
    /// Stable human-readable identifier. Surfaces in the perf-report
    /// JSON and in the per-entry stdout PASS/FAIL line.
    name: &'static str,
    /// Stage label. `Stage::M2` rows gate the harness exit code on
    /// failure; `Stage::Future(_)` rows report-but-do-not-gate so a
    /// future milestone's row can sit alongside a gating M2 row
    /// without affecting the overall exit code.
    stage: Stage,
    /// Per-kind dispatch.
    kind: MatrixRowKind,
}

/// One `kb search` query case in a [`MatrixRowKind::KbSearch`] row.
/// The harness runs each case live against both implementations and
/// byte-compares stdout / stderr / exit.
#[derive(Debug, Clone)]
struct KbSearchCase {
    /// Stable label; used verbatim in the per-case comparison
    /// artifact names (`<label>.stdout` etc.) — deterministic, never
    /// a random suffix, so re-running the harness writes byte-stable
    /// artifact paths into the perf-report.
    label: &'static str,
    /// The `kb search` query positional.
    query: &'static str,
    /// Pass `--json` to both implementations when true.
    json: bool,
    /// Pass `--top-k N` when `Some`; omit the flag when `None`.
    top_k: Option<i64>,
}

/// The committed `kb search` parity query set: human + `--json`
/// success, a broad multi-hit query, zero-result, the negative-`top_k`
/// clamp, and a `top_k` cap. The same query shapes the committed-oracle
/// integration test (`crates/cacg-cli/tests/kb_search.rs`) exercises —
/// here they run live, byte-comparing Python against Rust.
const KB_SEARCH_PARITY_CASES: &[KbSearchCase] = &[
    KbSearchCase {
        label: "success_human",
        query: "synthetic",
        json: false,
        top_k: None,
    },
    KbSearchCase {
        label: "success_json",
        query: "synthetic",
        json: true,
        top_k: None,
    },
    KbSearchCase {
        label: "success_human_broad",
        query: "card",
        json: false,
        top_k: None,
    },
    KbSearchCase {
        label: "success_json_broad",
        query: "card",
        json: true,
        top_k: None,
    },
    KbSearchCase {
        label: "zero_result_human",
        query: "zzznomatchxyzzy",
        json: false,
        top_k: None,
    },
    KbSearchCase {
        label: "zero_result_json",
        query: "zzznomatchxyzzy",
        json: true,
        top_k: None,
    },
    KbSearchCase {
        label: "negative_top_k",
        query: "synthetic",
        json: false,
        top_k: Some(-1),
    },
    KbSearchCase {
        label: "top_k_cap",
        query: "card",
        json: false,
        top_k: Some(2),
    },
];

/// The `kb search` parity query set for the CFA first-bite corpus: a
/// title-relevant query (`valuation` — in two card titles), a
/// tag-relevant query (`multiples` — a card tag), a summary-relevant
/// query (`earnings`), and a zero-result query, in human + `--json`
/// shapes. Run live, byte-comparing Python against Rust.
const KB_SEARCH_CFA_FIRST_BITE_CASES: &[KbSearchCase] = &[
    KbSearchCase {
        label: "cfa_title_valuation",
        query: "valuation",
        json: false,
        top_k: None,
    },
    KbSearchCase {
        label: "cfa_title_valuation_json",
        query: "valuation",
        json: true,
        top_k: None,
    },
    KbSearchCase {
        label: "cfa_tag_multiples",
        query: "multiples",
        json: false,
        top_k: None,
    },
    KbSearchCase {
        label: "cfa_summary_earnings",
        query: "earnings",
        json: false,
        top_k: None,
    },
    KbSearchCase {
        label: "cfa_zero_result",
        query: "zzznomatchxyzzy",
        json: false,
        top_k: None,
    },
    KbSearchCase {
        label: "cfa_zero_result_json",
        query: "zzznomatchxyzzy",
        json: true,
        top_k: None,
    },
];

/// One `kb show` invocation in a [`MatrixRowKind::KbShow`] row. Each
/// case is fully self-contained (its own `cards_manifest` +
/// `source_matrix` + flags) so a single row can cover the active /
/// retracted / unauthorized / `--path` / missing-card scenarios — each
/// of which needs a different manifest, matrix, or flag combination.
/// Paths are workspace-relative; the harness runs `kb show` with
/// `cwd = workspace_root`, so they resolve on both sides and the
/// `--path` value stays relative + `..`-free (a `CACG-SHOW-003`
/// rejection would otherwise diverge from Python).
#[derive(Debug, Clone)]
struct KbShowCase {
    /// Stable label; used verbatim in the per-case comparison artifact
    /// names — deterministic, never a random suffix.
    label: &'static str,
    /// The `card_id` positional.
    card_id: &'static str,
    /// `--cards-manifest` path, relative to the workspace root.
    cards_manifest: &'static str,
    /// `--source-matrix` path, relative to the workspace root.
    source_matrix: &'static str,
    /// `--path` override (workspace-relative) when `Some`; omitted
    /// otherwise.
    path: Option<&'static str>,
    /// Pass `--allow-retracted` to both implementations when true.
    allow_retracted: bool,
}

/// The committed `kb show` parity case set: an active card; a
/// directly-retracted id (`CACG-CLI-001` — directly-retracted cards are
/// removed from `cards`); a dependency-retracted card refused
/// (`CACG-SHOW-001`) and shown (`STATUS: DEPENDENCY-RETRACTED` under
/// `--allow-retracted`); a missing card (`CACG-CLI-001`); a `--path`
/// override that matches and one that mismatches (`CACG-SHOW-002`); an
/// unauthorized card (`CACG-AUTH-001`, via a cross-reading
/// source-matrix); and a CFA search-to-show case over the
/// `cfa_first_bite` corpus. Run live, byte-comparing Python `_cmd_show`
/// against the Rust dispatcher.
const KB_SHOW_PARITY_CASES: &[KbShowCase] = &[
    KbShowCase {
        label: "active",
        card_id: "content-addressable-identity",
        cards_manifest: "tests/parity_corpus/kb_show/cards_manifest.json",
        source_matrix: "tests/parity_corpus/kb_show/source_matrix.json",
        path: None,
        allow_retracted: false,
    },
    KbShowCase {
        label: "directly_retracted_cli_001",
        card_id: "synthetic-card-02",
        cards_manifest: "tests/parity_corpus/kb_show/cards_manifest.json",
        source_matrix: "tests/parity_corpus/kb_show/source_matrix.json",
        path: None,
        allow_retracted: false,
    },
    KbShowCase {
        label: "dependency_retracted_refused",
        card_id: "synthetic-card-03",
        cards_manifest: "tests/parity_corpus/kb_show/cards_manifest.json",
        source_matrix: "tests/parity_corpus/kb_show/source_matrix.json",
        path: None,
        allow_retracted: false,
    },
    KbShowCase {
        label: "dependency_retracted_shown",
        card_id: "synthetic-card-03",
        cards_manifest: "tests/parity_corpus/kb_show/cards_manifest.json",
        source_matrix: "tests/parity_corpus/kb_show/source_matrix.json",
        path: None,
        allow_retracted: true,
    },
    KbShowCase {
        label: "missing_card",
        card_id: "no-such-card-xyz",
        cards_manifest: "tests/parity_corpus/kb_show/cards_manifest.json",
        source_matrix: "tests/parity_corpus/kb_show/source_matrix.json",
        path: None,
        allow_retracted: false,
    },
    KbShowCase {
        label: "path_override_match",
        card_id: "content-addressable-identity",
        cards_manifest: "tests/parity_corpus/kb_show/cards_manifest.json",
        source_matrix: "tests/parity_corpus/kb_show/source_matrix.json",
        path: Some("tests/parity_corpus/cards/reading_01/01-content-addressable-identity.md"),
        allow_retracted: false,
    },
    KbShowCase {
        label: "path_override_mismatch",
        card_id: "content-addressable-identity",
        cards_manifest: "tests/parity_corpus/kb_show/cards_manifest.json",
        source_matrix: "tests/parity_corpus/kb_show/source_matrix.json",
        path: Some("tests/parity_corpus/cards/reading_01/synthetic-card-01.md"),
        allow_retracted: false,
    },
    KbShowCase {
        label: "unauthorized_reading",
        card_id: "content-addressable-identity",
        cards_manifest: "tests/parity_corpus/kb_show/cards_manifest.json",
        source_matrix: "tests/parity_corpus/cfa_first_bite/source_matrix.json",
        path: None,
        allow_retracted: false,
    },
    // CACG-AUTH-002: the card's `reading_id` IS a source-matrix key,
    // but its cited `source_id` is not on that reading's allow-list.
    KbShowCase {
        label: "unauthorized_source",
        card_id: "content-addressable-identity",
        cards_manifest: "tests/parity_corpus/kb_show/cards_manifest.json",
        source_matrix: "tests/parity_corpus/kb_show/source_matrix_unauthorized_source.json",
        path: None,
        allow_retracted: false,
    },
    KbShowCase {
        label: "cfa_search_to_show",
        card_id: "intrinsic-valuation-discounted-cash-flows",
        cards_manifest: "tests/parity_corpus/cfa_first_bite/cards_manifest.json",
        source_matrix: "tests/parity_corpus/cfa_first_bite/source_matrix.json",
        path: None,
        allow_retracted: false,
    },
];

/// Per-row body selector.
#[derive(Debug, Clone)]
enum MatrixRowKind {
    /// The existing `kb index` byte-diff body: spawn Python and Rust
    /// `kb index` on isolated corpus copies, enumerate emitted out-dir
    /// artifacts and per-card history sidecars, byte-compare them via
    /// `compare_artifact`.
    KbIndex {
        /// Relative subdir under the harness `corpus` root that holds
        /// the cards directory each implementation receives as argv.
        corpus_subdir: &'static str,
    },
    /// Per-subcommand semantic-help snapshot. Shells out to the Python
    /// argparse JSON-export shim, introspects Rust clap for the same
    /// `kb lint` per-card byte parity. For every `.md` file under
    /// `corpus_subdir`, runs Python `python -m cacg.cli lint <card>`
    /// and Rust `cacg-cli lint <card>` side-by-side under
    /// `KB_FROZEN_CLOCK=1`, then byte-compares the stderr capture and
    /// the per-implementation lint journal append for each card.
    /// Exit codes per card are required to match (one comparison per
    /// card; a divergence flags the row as a fail).
    KbLint {
        /// Corpus subdir (relative to harness corpus root) containing
        /// the card `.md` fixtures to lint.
        corpus_subdir: &'static str,
        /// Path to `chunks_manifest.json` (relative to the workspace
        /// root) shared by every card in the corpus.
        chunks_manifest: &'static str,
        /// Path to `source_matrix.json` (relative to the workspace
        /// root) shared by every card in the corpus.
        source_matrix: &'static str,
    },
    /// `kb verify` per-card byte parity. For every `.md` file under
    /// `corpus_subdir`, runs Python `python -m cacg.cli verify
    /// <card>` and Rust `kb verify <card>` side-by-side under
    /// `KB_FROZEN_CLOCK=1` (adding `--fuzzy` and/or
    /// `--unsafe-skip-lint` per the row), then byte-compares
    /// stdout, stderr, exit code, and the per-card verify journal
    /// append. The verify journal shares the lint journal file
    /// name (`lint_journal.jsonl`) — Python and Rust both default
    /// the verify journal there.
    KbVerify {
        /// Corpus subdir (relative to harness corpus root) holding
        /// the card `.md` fixtures to verify.
        corpus_subdir: &'static str,
        /// Path to `chunks_manifest.json` (relative to workspace
        /// root) shared by every card.
        chunks_manifest: &'static str,
        /// Path to `source_matrix.json` (relative to workspace
        /// root) shared by every card.
        source_matrix: &'static str,
        /// Pass `--fuzzy` to both implementations when true.
        fuzzy: bool,
        /// Pass `--unsafe-skip-lint` to both implementations when
        /// true.
        skip_lint: bool,
    },
    /// `kb verify --round-summary <fixture>` per-fixture byte parity.
    /// Enumerates every `.md` file under `fixtures_dir` (workspace-
    /// relative), runs Python `python -m cacg.cli verify
    /// --round-summary <fixture>` and Rust `kb verify --round-summary
    /// <fixture>` side-by-side under `KB_FROZEN_CLOCK=1` with shared
    /// chunks-manifest + source-matrix, then byte-compares stdout,
    /// stderr, exit code, and the per-fixture verify journal. Mirrors
    /// the structure of [`MatrixRowKind::KbVerify`] but targets the
    /// round-summary dispatcher path instead of the single-card path.
    KbVerifyRoundSummary {
        /// Workspace-relative path to the fixtures directory holding
        /// `*.md` round-summary inputs (e.g.,
        /// `tests/round_summary_fixtures`).
        fixtures_dir: &'static str,
        /// Workspace-relative path to `chunks_manifest.json` shared
        /// by every fixture.
        chunks_manifest: &'static str,
        /// Workspace-relative path to `source_matrix.json` shared by
        /// every fixture.
        source_matrix: &'static str,
    },
    /// `kb verify <card> --semantic <cache>` single-card byte parity
    /// against a committed semantic cache fixture. Runs Python
    /// `python -m cacg.cli verify <card> --semantic <cache>` and Rust
    /// `kb verify <card> --semantic <cache>` side-by-side under
    /// `KB_FROZEN_CLOCK=1`, then byte-compares stdout, stderr, exit
    /// code, and the verify journal. Unlike [`MatrixRowKind::KbVerify`]
    /// (which iterates a corpus directory), this row pins one specific
    /// card so the supplied cache's `(chunk_hash, claim_window_hash)`
    /// key matches deterministically.
    KbVerifySemantic {
        /// Workspace-relative path to the single card markdown to
        /// verify.
        card: &'static str,
        /// Workspace-relative path to `chunks_manifest.json`.
        chunks_manifest: &'static str,
        /// Workspace-relative path to `source_matrix.json`.
        source_matrix: &'static str,
        /// Workspace-relative path to the committed
        /// `semantic_cache.json`.
        semantic_cache: &'static str,
    },
    /// `kb verify --round-summary <summary> --semantic <cache>`
    /// batch + semantic byte parity. Runs Python and Rust
    /// `kb verify --round-summary <summary> --semantic <cache>`
    /// side-by-side under `KB_FROZEN_CLOCK=1`, then byte-compares
    /// stdout, stderr, exit code, and the per-fixture verify
    /// journal. This is the round-summary counterpart of
    /// [`MatrixRowKind::KbVerifySemantic`]: both pin a single
    /// fixture so the cache's `(chunk_hash, claim_window_hash)`
    /// key matches deterministically and the resulting Layer-3
    /// verdict is byte-stable.
    KbVerifyRoundSummarySemantic {
        /// Workspace-relative path to the round-summary markdown
        /// fixture.
        summary: &'static str,
        /// Workspace-relative path to `chunks_manifest.json`.
        chunks_manifest: &'static str,
        /// Workspace-relative path to `source_matrix.json`.
        source_matrix: &'static str,
        /// Workspace-relative path to the committed
        /// `semantic_cache.json`.
        semantic_cache: &'static str,
    },
    /// `kb search` byte parity. For each [`KbSearchCase`], runs Python
    /// `python -m cacg.cli search <query> ...` and Rust `kb search
    /// <query> ...` side-by-side under `KB_FROZEN_CLOCK=1` over the
    /// `summaries.json` + `source_matrix.json` under `corpus_subdir`,
    /// then byte-compares stdout, stderr, and exit code per case.
    /// `kb search` is read-only — it writes no journal and mutates no
    /// corpus file — so, unlike the `KbIndex` rows, it needs no
    /// isolated corpus copy. A sidecar-free `corpus_subdir` exercises
    /// the in-memory BM25 backend; a `corpus_subdir` carrying a
    /// `summaries.sqlite` exercises the FTS5 sidecar path (a fresh
    /// sealed sidecar serves the query; a stale-seal one emits
    /// `CACG-FTS-001` and both sides fall back to in-memory).
    KbSearch {
        /// Corpus subdir (relative to the harness corpus root) holding
        /// a `summaries.json` + `source_matrix.json` pair, optionally
        /// with a sibling `summaries.sqlite` FTS5 sidecar.
        corpus_subdir: &'static str,
        /// The query cases run live against both implementations.
        cases: &'static [KbSearchCase],
    },
    /// `kb show` byte parity. For each [`KbShowCase`], runs Python
    /// `python -m cacg.cli show <card_id> ...` and Rust `kb show
    /// <card_id> ...` side-by-side under `KB_FROZEN_CLOCK=1`, then
    /// byte-compares stdout, stderr, and exit code. `kb show` is
    /// read-only; each case carries its own `cards_manifest` +
    /// `source_matrix` so one row spans the active / retracted /
    /// unauthorized / `--path` / missing-card scenarios.
    KbShow {
        /// The `kb show` cases run live against both implementations.
        cases: &'static [KbShowCase],
    },
    /// AC-5 Pdfium BYTE-EQUAL parity gate. Runs Rust `kb ingest`
    /// against the committed PDF fixture under `KB_FROZEN_CLOCK=1`
    /// and byte-compares the published `chunks_manifest.json` +
    /// `sources_manifest.json` against the committed Python oracle
    /// (NOT a live-Python run — the AC-5 spec wording is "compares
    /// the result against the committed `out_python` pair," so the
    /// gate is one-directional). `sources_manifest.json` is compared
    /// modulo the DEC-2 whitelist (`parser_name` / `parser_version`
    /// declared divergence) via canonical-JSON byte equality on the
    /// stripped payload. The row mirrors the integration test at
    /// `crates/cacg-cli/tests/kb_ingest_parity.rs`; living in the
    /// xtask matrix makes the gate part of the standard
    /// `cargo xtask parity` run alongside the other M2 entries.
    ///
    /// When `libpdfium` is unavailable on the runner, the row marks
    /// itself `EntryStatus::FutureStage("M4-pdfium-provisioning")`
    /// so it documents the gap without failing the harness exit
    /// code; setting `CACG_REQUIRE_PDFIUM=1` escalates that to a
    /// hard `Fail` for CI.
    KbIngest {
        /// Workspace-relative path to the PDF fixture (e.g.,
        /// `tests/parity_corpus/pdfs/cfa_vol1_trim.pdf`).
        pdf: &'static str,
        /// Workspace-relative path to the committed Python oracle
        /// dir holding `chunks_manifest.json` +
        /// `sources_manifest.json` (e.g.,
        /// `tests/parity_corpus/out_python/pdfs/cfa_vol1_trim`).
        oracle_dir: &'static str,
        /// `--source-id` argv value (must match the oracle's
        /// recorded `sources[0].source_id`).
        source_id: &'static str,
    },
}

/// Dispatch a single matrix row to its per-kind body. The signature
/// is generic over row kinds so adding a new kind is a single new
/// arm here; callers iterate over the row table and do not need to
/// know which kind they are running.
fn run_matrix_row(
    row: &MatrixRow,
    corpus: &Path,
    out_root: &Path,
    workspace_root: &Path,
    python_cmd: &IndexCommand,
    rust_cmd: &IndexCommand,
) -> Result<EntryReport, ParityError> {
    match &row.kind {
        MatrixRowKind::KbIndex { corpus_subdir } => run_kb_index_row(
            row,
            corpus_subdir,
            corpus,
            out_root,
            workspace_root,
            python_cmd,
            rust_cmd,
        ),
        MatrixRowKind::KbLint {
            corpus_subdir,
            chunks_manifest,
            source_matrix,
        } => {
            let cards_dir = corpus.join(corpus_subdir);
            let row_out_root = out_root.join(row.name);
            Ok(run_kb_lint_entry(
                row.name,
                row.stage.clone(),
                &cards_dir,
                &row_out_root,
                workspace_root,
                chunks_manifest,
                source_matrix,
                corpus,
            ))
        }
        MatrixRowKind::KbVerify {
            corpus_subdir,
            chunks_manifest,
            source_matrix,
            fuzzy,
            skip_lint,
        } => {
            let cards_dir = corpus.join(corpus_subdir);
            let row_out_root = out_root.join(row.name);
            Ok(run_kb_verify_entry(
                row.name,
                row.stage.clone(),
                &cards_dir,
                &row_out_root,
                workspace_root,
                chunks_manifest,
                source_matrix,
                *fuzzy,
                *skip_lint,
                corpus,
            ))
        }
        MatrixRowKind::KbVerifyRoundSummary {
            fixtures_dir,
            chunks_manifest,
            source_matrix,
        } => {
            let row_out_root = out_root.join(row.name);
            Ok(run_kb_verify_round_summary_entry(
                row.name,
                row.stage.clone(),
                &row_out_root,
                workspace_root,
                fixtures_dir,
                chunks_manifest,
                source_matrix,
                corpus,
            ))
        }
        MatrixRowKind::KbVerifySemantic {
            card,
            chunks_manifest,
            source_matrix,
            semantic_cache,
        } => {
            let row_out_root = out_root.join(row.name);
            Ok(run_kb_verify_semantic_entry(
                row.name,
                row.stage.clone(),
                &row_out_root,
                workspace_root,
                card,
                chunks_manifest,
                source_matrix,
                semantic_cache,
                corpus,
            ))
        }
        MatrixRowKind::KbVerifyRoundSummarySemantic {
            summary,
            chunks_manifest,
            source_matrix,
            semantic_cache,
        } => {
            let row_out_root = out_root.join(row.name);
            Ok(run_kb_verify_round_summary_semantic_entry(
                row.name,
                row.stage.clone(),
                &row_out_root,
                workspace_root,
                summary,
                chunks_manifest,
                source_matrix,
                semantic_cache,
                corpus,
            ))
        }
        MatrixRowKind::KbSearch {
            corpus_subdir,
            cases,
        } => {
            let corpus_dir = corpus.join(corpus_subdir);
            let row_out_root = out_root.join(row.name);
            Ok(run_kb_search_entry(
                row.name,
                row.stage.clone(),
                &corpus_dir,
                &row_out_root,
                workspace_root,
                cases,
                corpus,
            ))
        }
        MatrixRowKind::KbShow { cases } => {
            let row_out_root = out_root.join(row.name);
            Ok(run_kb_show_entry(
                row.name,
                row.stage.clone(),
                &row_out_root,
                workspace_root,
                cases,
                corpus,
            ))
        }
        MatrixRowKind::KbIngest {
            pdf,
            oracle_dir,
            source_id,
        } => {
            let row_out_root = out_root.join(row.name);
            Ok(run_kb_ingest_entry(
                row.name,
                row.stage.clone(),
                &row_out_root,
                workspace_root,
                pdf,
                oracle_dir,
                source_id,
            ))
        }
    }
}

/// `MatrixRowKind::KbIndex` dispatch. Each `KbIndex` row gets its own
/// `<out_root>/<row.name>/` namespace so sibling M2 rows (clean-hash +
/// stale-hash) keep their `py-corpus` / `rs-corpus` / `py-out` /
/// `rs-out` evidence without the later row's setup deleting the
/// earlier row's artifacts. A missing corpus subdir is a row Fail.
fn run_kb_index_row(
    row: &MatrixRow,
    corpus_subdir: &str,
    corpus: &Path,
    out_root: &Path,
    workspace_root: &Path,
    python_cmd: &IndexCommand,
    rust_cmd: &IndexCommand,
) -> Result<EntryReport, ParityError> {
    let reading_dir = corpus.join(corpus_subdir);
    if !reading_dir.is_dir() {
        return Ok(EntryReport {
            name: row.name.to_string(),
            stage: row.stage.clone(),
            expected_command: String::new(),
            rust_command: String::new(),
            expected_duration_ms: 0,
            rust_duration_ms: 0,
            comparisons: Vec::new(),
            status: EntryStatus::Fail(format!("corpus subdir missing at {reading_dir:?}")),
        });
    }
    let row_out_root = out_root.join(row.name);
    let fixture_dir = committed_fixture_dir(corpus, row.name);
    let fixture_base = corpus.join("out_python/parity_rows").join(row.name);
    run_kb_index_entry(
        &reading_dir,
        corpus_subdir,
        &row_out_root,
        workspace_root,
        python_cmd,
        rust_cmd,
        if fixture_dir.is_dir() {
            Some((fixture_dir, fixture_base.join("py-corpus")))
        } else {
            None
        },
    )
    .map(|entry| EntryReport {
        name: row.name.to_string(),
        stage: row.stage.clone(),
        ..entry
    })
}

/// Run the kb index matrix entry against ISOLATED corpus copies so
/// every per-implementation side effect (manifest writes, per-card
/// `*.history.jsonl` appends, frontmatter rewrites) is attributable
/// to exactly one side. Path-stable invocation: both implementations
/// receive the row's declared `corpus_subdir` as their cards-dir
/// argv with `cwd` set to the per-implementation corpus root, so the
/// manifest's path-as-given field stays byte-equal across runs.
///
/// The `corpus_subdir` argument is the relative path the row declares
/// for both the source fixture lookup (`corpus.join(corpus_subdir)`)
/// and the isolated destination (`py_corpus_root.join(corpus_subdir)`
/// / `rs_corpus_root.join(corpus_subdir)`). Threading the same value
/// to both sides lets sibling rows (e.g. `cards_stale_hash/reading_01`)
/// be added by table insertion alone without altering the runner body.
fn run_kb_index_entry(
    reading_src: &Path,
    corpus_subdir: &str,
    out_root: &Path,
    _workspace_root: &Path,
    python_cmd: &IndexCommand,
    rust_cmd: &IndexCommand,
    committed_fixtures: Option<(PathBuf, PathBuf)>,
) -> Result<EntryReport, ParityError> {
    let use_fixtures = committed_fixtures.is_some();
    let (fixture_py_out, fixture_py_corpus_root) =
        committed_fixtures.unwrap_or_else(|| (out_root.join("py-out"), out_root.join("py-corpus")));

    let py_corpus_root = if use_fixtures {
        fixture_py_corpus_root.clone()
    } else {
        out_root.join("py-corpus")
    };
    let rs_corpus_root = out_root.join("rs-corpus");
    let py_corpus = py_corpus_root.join(corpus_subdir);
    let rs_corpus = rs_corpus_root.join(corpus_subdir);
    let py_out = fixture_py_out;
    let rs_out = out_root.join("rs-out");

    if !use_fixtures {
        for dir in [&py_corpus_root, &py_out] {
            if dir.exists() {
                fs::remove_dir_all(dir).map_err(ParityError::Io)?;
            }
            fs::create_dir_all(dir).map_err(ParityError::Io)?;
        }
        copy_corpus(reading_src, &py_corpus)?;
    }
    for dir in [&rs_corpus_root, &rs_out] {
        if dir.exists() {
            fs::remove_dir_all(dir).map_err(ParityError::Io)?;
        }
        fs::create_dir_all(dir).map_err(ParityError::Io)?;
    }
    copy_corpus(reading_src, &rs_corpus)?;

    let cards_argv = Path::new(corpus_subdir);

    let (python_cmd_str, expected_duration_ms, py_ok) = if use_fixtures {
        (
            format!("committed-fixture: {}", py_out.display()),
            0u128,
            true,
        )
    } else {
        let cmd_str = python_cmd.render(cards_argv, &py_out);
        let start_py = Instant::now();
        let py_status = python_cmd.spawn_status(cards_argv, &py_out, &py_corpus_root);
        let dur = start_py.elapsed().as_millis();
        let ok = matches!(py_status, Ok(s) if s.success());
        (cmd_str, dur, ok)
    };

    let rust_cmd_str = rust_cmd.render(cards_argv, &rs_out);
    let start_rs = Instant::now();
    let rs_status = rust_cmd.spawn_status(cards_argv, &rs_out, &rs_corpus_root);
    let rust_duration_ms = start_rs.elapsed().as_millis();
    let rs_ok = matches!(rs_status, Ok(s) if s.success());

    let mut comparisons: Vec<ArtifactComparison> = Vec::new();
    if py_ok && rs_ok {
        // Union enumeration over out_dir artifacts: a name that
        // appears only in `py_out` (or only in `rs_out`) still gets a
        // comparison record — `compare_artifact` flags it as a diff
        // because the other side is missing.
        let mut out_names: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for dir in [&py_out, &rs_out] {
            for entry in fs::read_dir(dir).map_err(ParityError::Io)? {
                let entry = entry.map_err(ParityError::Io)?;
                let name = entry.file_name().to_string_lossy().to_string();
                if is_audited_artifact_name(&name) {
                    out_names.insert(name);
                }
            }
        }
        for name in &out_names {
            comparisons.push(compare_artifact(
                name,
                &py_out.join(name),
                &rs_out.join(name),
            ));
        }
        // Per-card history sidecars: enumerate `.history.jsonl` files
        // from BOTH isolated corpus subdirs and union them. Each
        // unique sidecar name becomes a separate `compare_artifact`.
        // A sidecar emitted by only one implementation surfaces as a
        // failing comparison via the `expected_exists` / `rust_exists`
        // flags, which closes the false-green where a stale-hash
        // corpus produces a Python sidecar that Rust does not.
        let mut sidecar_names: std::collections::BTreeSet<String> =
            std::collections::BTreeSet::new();
        for dir in [&py_corpus, &rs_corpus] {
            for entry in fs::read_dir(dir).map_err(ParityError::Io)? {
                let entry = entry.map_err(ParityError::Io)?;
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".history.jsonl") {
                    sidecar_names.insert(name);
                }
            }
        }
        for name in &sidecar_names {
            comparisons.push(compare_artifact(
                name,
                &py_corpus.join(name),
                &rs_corpus.join(name),
            ));
        }
    }

    let status = if !py_ok {
        EntryStatus::Fail("expected-side invocation failed".to_string())
    } else if !rs_ok {
        EntryStatus::Fail(format!("rust invocation failed: {rs_status:?}"))
    } else if comparisons.is_empty() {
        // Guard against a vacuous-true Pass: if both subprocesses
        // succeeded but produced zero audited artifacts on either
        // side, the matrix entry is not actually proving anything.
        EntryStatus::Fail(
            "no audited artifacts emitted by either side — would have been a vacuous Pass"
                .to_string(),
        )
    } else if comparisons.iter().all(|c| c.equal) {
        EntryStatus::Pass
    } else {
        let n_diff = comparisons.iter().filter(|c| !c.equal).count();
        EntryStatus::Fail(format!(
            "{n_diff} of {} artifacts diffed",
            comparisons.len()
        ))
    };

    Ok(EntryReport {
        name: "kb_index_parity_corpus_reading_01".to_string(),
        stage: Stage::M2,
        expected_command: python_cmd_str,
        rust_command: rust_cmd_str,
        expected_duration_ms,
        rust_duration_ms,
        comparisons,
        status,
    })
}

/// Per-card `kb lint` byte parity. For every `.md` file under
/// `cards_dir`, runs Python `python -m cacg.cli lint` and Rust
/// `cacg-cli lint` against the same fixture under `KB_FROZEN_CLOCK=1`,
/// then byte-compares per-card stdout, stderr, exit code, and a
/// per-card journal append. Each card produces four
/// `ArtifactComparison`s (one per artifact), so a row over N cards
/// yields 4N comparisons. The stdout comparison closes the AC-2
/// "stdout + stderr + journal byte-equal" requirement that an
/// earlier 3N-comparison version missed.
/// Lookup pair for the Python and Rust binaries used by
/// [`run_kb_lint_entry`]. Production callers use the defaults
/// (`python` from `PATH` and `target/debug/kb`); tests inject mock
/// executables to exercise specific divergence shapes.
#[derive(Debug, Clone)]
struct LintBinaries {
    python_executable: PathBuf,
    rust_executable: PathBuf,
}

impl LintBinaries {
    fn production(workspace_root: &Path) -> Self {
        Self {
            // Post-quarantine: route every live parity invocation through the
            // legacy oracle venv at `legacy_python_oracle/.venv/bin/python`.
            // Calling plain `python` (or `python3`) would resolve to the
            // system interpreter, which does not have `cacg.cli` installed
            // and surfaces as a `ModuleNotFoundError` mid-parity-row.
            python_executable: python_exe(workspace_root),
            rust_executable: workspace_root.join("target/debug/kb"),
        }
    }
}

fn run_kb_lint_entry(
    name: &str,
    stage: Stage,
    cards_dir: &Path,
    out_root: &Path,
    workspace_root: &Path,
    chunks_manifest: &str,
    source_matrix: &str,
    corpus: &Path,
) -> EntryReport {
    let fixture_dir = committed_fixture_dir(corpus, name);
    run_kb_lint_entry_with(
        name,
        stage,
        cards_dir,
        out_root,
        workspace_root,
        chunks_manifest,
        source_matrix,
        &LintBinaries::production(workspace_root),
        if fixture_dir.is_dir() {
            Some(fixture_dir)
        } else {
            None
        },
    )
}

fn run_kb_lint_entry_with(
    name: &str,
    stage: Stage,
    cards_dir: &Path,
    out_root: &Path,
    workspace_root: &Path,
    chunks_manifest: &str,
    source_matrix: &str,
    binaries: &LintBinaries,
    committed_fixtures: Option<PathBuf>,
) -> EntryReport {
    let use_fixtures = committed_fixtures.is_some();
    let py_out = committed_fixtures.unwrap_or_else(|| out_root.join("py-out"));
    let rs_out = out_root.join("rs-out");
    if !use_fixtures {
        if py_out.exists() {
            let _ = fs::remove_dir_all(&py_out);
        }
        let _ = fs::create_dir_all(&py_out);
    }
    if rs_out.exists() {
        let _ = fs::remove_dir_all(&rs_out);
    }
    let _ = fs::create_dir_all(&rs_out);

    let chunks_manifest_abs = workspace_root.join(chunks_manifest);
    let source_matrix_abs = workspace_root.join(source_matrix);

    // Sort cards for deterministic per-row iteration.
    let mut cards: Vec<PathBuf> = match fs::read_dir(cards_dir) {
        Ok(it) => it
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
            .collect(),
        Err(_) => {
            return EntryReport {
                name: name.to_string(),
                stage,
                expected_command: String::new(),
                rust_command: String::new(),
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: Vec::new(),
                status: EntryStatus::Fail(format!("cards_dir missing at {cards_dir:?}")),
            };
        }
    };
    cards.sort();

    let mut comparisons: Vec<ArtifactComparison> = Vec::new();
    let mut python_total_ms: u128 = 0;
    let mut rust_total_ms: u128 = 0;
    let mut python_cmd_str = String::new();
    let mut rust_cmd_str = String::new();
    let mut early_fail: Option<String> = None;

    for card in &cards {
        let stem = card
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("card")
            .to_string();
        let py_journal = py_out.join(format!("{stem}.lint_journal.jsonl"));
        let rs_journal = rs_out.join(format!("{stem}.lint_journal.jsonl"));
        let py_stderr = py_out.join(format!("{stem}.stderr"));
        let rs_stderr = rs_out.join(format!("{stem}.stderr"));
        let py_stdout = py_out.join(format!("{stem}.stdout"));
        let rs_stdout = rs_out.join(format!("{stem}.stdout"));
        let py_exit = py_out.join(format!("{stem}.exit"));
        let rs_exit = rs_out.join(format!("{stem}.exit"));

        if use_fixtures {
            if python_cmd_str.is_empty() {
                python_cmd_str = format!("committed-fixture: {}", py_out.display());
            }
        } else {
            let py_args = vec![
                "-m".to_string(),
                "cacg.cli".to_string(),
                "lint".to_string(),
                card.display().to_string(),
                "--chunks-manifest".to_string(),
                chunks_manifest_abs.display().to_string(),
                "--source-matrix".to_string(),
                source_matrix_abs.display().to_string(),
                "--journal".to_string(),
                py_journal.display().to_string(),
            ];
            if python_cmd_str.is_empty() {
                python_cmd_str = format!(
                    "{} {}",
                    binaries.python_executable.display(),
                    py_args.join(" ")
                );
            }
            let py_start = Instant::now();
            let py_out_res = Command::new(&binaries.python_executable)
                .args(&py_args)
                .env("KB_FROZEN_CLOCK", "1")
                .env("PYTHONPATH", workspace_root.join("src"))
                .current_dir(workspace_root)
                .output();
            python_total_ms += py_start.elapsed().as_millis();
            let py_status = match py_out_res {
                Ok(o) => o,
                Err(e) => {
                    early_fail = Some(format!("python spawn failed: {e}"));
                    break;
                }
            };
            let _ = fs::write(&py_stderr, &py_status.stderr);
            let _ = fs::write(&py_stdout, &py_status.stdout);
            let _ = fs::write(
                &py_exit,
                format!("{}\n", py_status.status.code().unwrap_or(-1)).as_bytes(),
            );
        }

        let cacg_cli_bin = &binaries.rust_executable;
        let rs_args = vec![
            "lint".to_string(),
            card.display().to_string(),
            "--chunks-manifest".to_string(),
            chunks_manifest_abs.display().to_string(),
            "--source-matrix".to_string(),
            source_matrix_abs.display().to_string(),
            "--journal".to_string(),
            rs_journal.display().to_string(),
        ];
        if rust_cmd_str.is_empty() {
            rust_cmd_str = format!("{} {}", cacg_cli_bin.display(), rs_args.join(" "));
        }
        let rs_start = Instant::now();
        let rs_out_res = Command::new(&cacg_cli_bin)
            .args(&rs_args)
            .env("KB_FROZEN_CLOCK", "1")
            .current_dir(workspace_root)
            .output();
        rust_total_ms += rs_start.elapsed().as_millis();
        let rs_status = match rs_out_res {
            Ok(o) => o,
            Err(e) => {
                early_fail = Some(format!("rust spawn failed: {e}"));
                break;
            }
        };
        let _ = fs::write(&rs_stderr, &rs_status.stderr);
        let _ = fs::write(&rs_stdout, &rs_status.stdout);
        let _ = fs::write(
            &rs_exit,
            format!("{}\n", rs_status.status.code().unwrap_or(-1)).as_bytes(),
        );

        comparisons.push(compare_artifact(
            format!("{stem}.stdout"),
            &py_stdout,
            &rs_stdout,
        ));
        comparisons.push(compare_artifact(
            format!("{stem}.stderr"),
            &py_stderr,
            &rs_stderr,
        ));
        comparisons.push(compare_artifact(format!("{stem}.exit"), &py_exit, &rs_exit));
        comparisons.push(compare_artifact(
            format!("{stem}.lint_journal.jsonl"),
            &py_journal,
            &rs_journal,
        ));
    }

    // AC-2.1 carve-out: if the corpus declares `cacg.v0/scope:hot-path`
    // via a sidecar `scope.json`, demote per-comparison diffs whose
    // divergent lines mention only deferred CACG prefixes (CACG-SUM-*,
    // CACG-SKILL-*, CACG-DEP-*, CACG-ROLE-*) from Fail to FutureStage.
    // See `docs/diagnostic-parity.md` §3a. Demotion only applies on
    // `MatrixRowKind::KbLint` rows (this function); KbIndex and
    // KbIndex dispatches never call this codepath.
    let hot_path_scope = is_hot_path_scope_annotation(cards_dir);
    let any_aux_demoted = hot_path_scope
        && comparisons.iter().any(|c| {
            !c.equal
                && c.expected_exists
                && c.rust_exists
                && diff_mentions_only_deferred_codes_at_paths(
                    &lint_artifact_expected_path(c, &py_out),
                    &lint_artifact_rust_path(c, &rs_out),
                )
        });

    let status = if let Some(reason) = early_fail {
        EntryStatus::Fail(reason)
    } else if cards.is_empty() {
        EntryStatus::Fail(format!("no .md cards under {cards_dir:?}"))
    } else if comparisons.iter().all(|c| c.equal) {
        EntryStatus::Pass
    } else if hot_path_scope
        && comparisons.iter().filter(|c| !c.equal).all(|c| {
            c.expected_exists
                && c.rust_exists
                && diff_mentions_only_deferred_codes_at_paths(
                    &lint_artifact_expected_path(c, &py_out),
                    &lint_artifact_rust_path(c, &rs_out),
                )
        })
    {
        // Every diverging comparison is auxiliary-codes-only AND the
        // corpus is hot-path-scoped → row demoted to Future-stage per
        // §3a.4. The `any_aux_demoted` guard above ensures we only
        // take this branch when at least one comparison was actually
        // demoted (otherwise we already exited via the all-equal arm).
        let _ = any_aux_demoted; // touched for clarity; logic is in the iter().all above
        EntryStatus::FutureStage("M3")
    } else {
        let n_diff = comparisons.iter().filter(|c| !c.equal).count();
        EntryStatus::Fail(format!(
            "{n_diff} of {} artifacts diffed",
            comparisons.len()
        ))
    };

    EntryReport {
        name: name.to_string(),
        stage,
        expected_command: python_cmd_str,
        rust_command: rust_cmd_str,
        expected_duration_ms: python_total_ms,
        rust_duration_ms: rust_total_ms,
        comparisons,
        status,
    }
}

/// Production entry point for a `MatrixRowKind::KbVerify` row.
/// Reuses [`LintBinaries`] as the (python, rust) executable pair —
/// the struct is a generic binary pair; the `Lint` in its name is
/// incidental.
#[allow(clippy::too_many_arguments)]
fn run_kb_verify_entry(
    name: &str,
    stage: Stage,
    cards_dir: &Path,
    out_root: &Path,
    workspace_root: &Path,
    chunks_manifest: &str,
    source_matrix: &str,
    fuzzy: bool,
    skip_lint: bool,
    corpus: &Path,
) -> EntryReport {
    let fixture_dir = committed_fixture_dir(corpus, name);
    run_kb_verify_entry_with(
        name,
        stage,
        cards_dir,
        out_root,
        workspace_root,
        chunks_manifest,
        source_matrix,
        fuzzy,
        skip_lint,
        &LintBinaries::production(workspace_root),
        if fixture_dir.is_dir() {
            Some(fixture_dir)
        } else {
            None
        },
    )
}

/// `kb verify` per-card byte-parity body. For each `.md` card under
/// `cards_dir`, spawns Python and Rust `kb verify <card>` under
/// `KB_FROZEN_CLOCK=1` (with `--fuzzy` / `--unsafe-skip-lint` per
/// the row flags) and byte-compares stdout, stderr, exit, and the
/// per-card verify journal. Unlike the lint body this has no
/// hot-path auxiliary-codes demotion — verify rows are not
/// lint-pass rows and never consult `scope.json`.
#[allow(clippy::too_many_arguments)]
fn run_kb_verify_entry_with(
    name: &str,
    stage: Stage,
    cards_dir: &Path,
    out_root: &Path,
    workspace_root: &Path,
    chunks_manifest: &str,
    source_matrix: &str,
    fuzzy: bool,
    skip_lint: bool,
    binaries: &LintBinaries,
    committed_fixtures: Option<PathBuf>,
) -> EntryReport {
    let use_fixtures = committed_fixtures.is_some();
    let py_out = committed_fixtures.unwrap_or_else(|| out_root.join("py-out"));
    let rs_out = out_root.join("rs-out");
    if !use_fixtures {
        if py_out.exists() {
            let _ = fs::remove_dir_all(&py_out);
        }
        let _ = fs::create_dir_all(&py_out);
    }
    if rs_out.exists() {
        let _ = fs::remove_dir_all(&rs_out);
    }
    let _ = fs::create_dir_all(&rs_out);

    let chunks_manifest_abs = workspace_root.join(chunks_manifest);
    let source_matrix_abs = workspace_root.join(source_matrix);

    let mut cards: Vec<PathBuf> = match fs::read_dir(cards_dir) {
        Ok(it) => it
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
            .collect(),
        Err(_) => {
            return EntryReport {
                name: name.to_string(),
                stage,
                expected_command: String::new(),
                rust_command: String::new(),
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: Vec::new(),
                status: EntryStatus::Fail(format!("cards_dir missing at {cards_dir:?}")),
            };
        }
    };
    cards.sort();

    // Per-row extra flags appended to BOTH implementations'
    // argv so the comparison stays apples-to-apples.
    let mut extra_flags: Vec<String> = Vec::new();
    if fuzzy {
        extra_flags.push("--fuzzy".to_string());
    }
    if skip_lint {
        extra_flags.push("--unsafe-skip-lint".to_string());
    }

    let mut comparisons: Vec<ArtifactComparison> = Vec::new();
    let mut python_total_ms: u128 = 0;
    let mut rust_total_ms: u128 = 0;
    let mut python_cmd_str = String::new();
    let mut rust_cmd_str = String::new();
    let mut early_fail: Option<String> = None;

    for card in &cards {
        let stem = card
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("card")
            .to_string();
        let py_journal = py_out.join(format!("{stem}.lint_journal.jsonl"));
        let rs_journal = rs_out.join(format!("{stem}.lint_journal.jsonl"));
        let py_stderr = py_out.join(format!("{stem}.stderr"));
        let rs_stderr = rs_out.join(format!("{stem}.stderr"));
        let py_stdout = py_out.join(format!("{stem}.stdout"));
        let rs_stdout = rs_out.join(format!("{stem}.stdout"));
        let py_exit = py_out.join(format!("{stem}.exit"));
        let rs_exit = rs_out.join(format!("{stem}.exit"));

        if use_fixtures {
            if python_cmd_str.is_empty() {
                python_cmd_str = format!("committed-fixture: {}", py_out.display());
            }
        } else {
            let mut py_args = vec![
                "-m".to_string(),
                "cacg.cli".to_string(),
                "verify".to_string(),
                card.display().to_string(),
                "--chunks-manifest".to_string(),
                chunks_manifest_abs.display().to_string(),
                "--source-matrix".to_string(),
                source_matrix_abs.display().to_string(),
                "--journal".to_string(),
                py_journal.display().to_string(),
            ];
            py_args.extend(extra_flags.iter().cloned());
            if python_cmd_str.is_empty() {
                python_cmd_str = format!(
                    "{} {}",
                    binaries.python_executable.display(),
                    py_args.join(" ")
                );
            }
            let py_start = Instant::now();
            let py_out_res = Command::new(&binaries.python_executable)
                .args(&py_args)
                .env("KB_FROZEN_CLOCK", "1")
                .env("PYTHONPATH", workspace_root.join("src"))
                .current_dir(workspace_root)
                .output();
            python_total_ms += py_start.elapsed().as_millis();
            let py_status = match py_out_res {
                Ok(o) => o,
                Err(e) => {
                    early_fail = Some(format!("python spawn failed: {e}"));
                    break;
                }
            };
            let _ = fs::write(&py_stderr, &py_status.stderr);
            let _ = fs::write(&py_stdout, &py_status.stdout);
            let _ = fs::write(
                &py_exit,
                format!("{}\n", py_status.status.code().unwrap_or(-1)).as_bytes(),
            );
        }

        let mut rs_args = vec![
            "verify".to_string(),
            card.display().to_string(),
            "--chunks-manifest".to_string(),
            chunks_manifest_abs.display().to_string(),
            "--source-matrix".to_string(),
            source_matrix_abs.display().to_string(),
            "--journal".to_string(),
            rs_journal.display().to_string(),
        ];
        rs_args.extend(extra_flags.iter().cloned());
        if rust_cmd_str.is_empty() {
            rust_cmd_str = format!(
                "{} {}",
                binaries.rust_executable.display(),
                rs_args.join(" ")
            );
        }
        let rs_start = Instant::now();
        let rs_out_res = Command::new(&binaries.rust_executable)
            .args(&rs_args)
            .env("KB_FROZEN_CLOCK", "1")
            .current_dir(workspace_root)
            .output();
        rust_total_ms += rs_start.elapsed().as_millis();
        let rs_status = match rs_out_res {
            Ok(o) => o,
            Err(e) => {
                early_fail = Some(format!("rust spawn failed: {e}"));
                break;
            }
        };
        let _ = fs::write(&rs_stderr, &rs_status.stderr);
        let _ = fs::write(&rs_stdout, &rs_status.stdout);
        let _ = fs::write(
            &rs_exit,
            format!("{}\n", rs_status.status.code().unwrap_or(-1)).as_bytes(),
        );

        comparisons.push(compare_artifact(
            format!("{stem}.stdout"),
            &py_stdout,
            &rs_stdout,
        ));
        comparisons.push(compare_artifact(
            format!("{stem}.stderr"),
            &py_stderr,
            &rs_stderr,
        ));
        comparisons.push(compare_artifact(format!("{stem}.exit"), &py_exit, &rs_exit));
        comparisons.push(compare_artifact(
            format!("{stem}.lint_journal.jsonl"),
            &py_journal,
            &rs_journal,
        ));
    }

    let status = if let Some(reason) = early_fail {
        EntryStatus::Fail(reason)
    } else if cards.is_empty() {
        EntryStatus::Fail(format!("no .md cards under {cards_dir:?}"))
    } else if comparisons.iter().all(|c| c.equal) {
        EntryStatus::Pass
    } else {
        let n_diff = comparisons.iter().filter(|c| !c.equal).count();
        EntryStatus::Fail(format!(
            "{n_diff} of {} artifacts diffed",
            comparisons.len()
        ))
    };

    EntryReport {
        name: name.to_string(),
        stage,
        expected_command: python_cmd_str,
        rust_command: rust_cmd_str,
        expected_duration_ms: python_total_ms,
        rust_duration_ms: rust_total_ms,
        comparisons,
        status,
    }
}

/// `kb verify --round-summary <fixture>` per-fixture byte-parity
/// body. For each `.md` file under `fixtures_dir`, spawns Python and
/// Rust `kb verify --round-summary <fixture>` under
/// `KB_FROZEN_CLOCK=1` with shared chunks-manifest + source-matrix,
/// then byte-compares stdout, stderr, exit code, and the per-fixture
/// verify journal. Mirrors the per-card structure of
/// [`run_kb_verify_entry_with`] but targets the round-summary CLI
/// path. Per-fixture artifact filenames stay deterministic (no temp
/// suffix) so the run is byte-stable across re-executions.
fn run_kb_verify_round_summary_entry(
    name: &str,
    stage: Stage,
    out_root: &Path,
    workspace_root: &Path,
    fixtures_dir: &str,
    chunks_manifest: &str,
    source_matrix: &str,
    corpus: &Path,
) -> EntryReport {
    let fixture_dir = committed_fixture_dir(corpus, name);
    run_kb_verify_round_summary_entry_with(
        name,
        stage,
        out_root,
        workspace_root,
        fixtures_dir,
        chunks_manifest,
        source_matrix,
        &LintBinaries::production(workspace_root),
        if fixture_dir.is_dir() {
            Some(fixture_dir)
        } else {
            None
        },
    )
}

/// Round-summary parity body. Same structure as
/// [`run_kb_verify_entry_with`] but enumerates `.md` files under
/// `fixtures_dir` instead of under a parity-corpus `cards/` subdir,
/// and uses `--round-summary <fixture>` instead of the `<card>`
/// positional argv. Each fixture contributes 4 artifact comparisons
/// (stdout, stderr, exit, lint_journal.jsonl).
fn run_kb_verify_round_summary_entry_with(
    name: &str,
    stage: Stage,
    out_root: &Path,
    workspace_root: &Path,
    fixtures_dir: &str,
    chunks_manifest: &str,
    source_matrix: &str,
    binaries: &LintBinaries,
    committed_fixtures: Option<PathBuf>,
) -> EntryReport {
    let use_fixtures = committed_fixtures.is_some();
    let py_out = committed_fixtures.unwrap_or_else(|| out_root.join("py-out"));
    let rs_out = out_root.join("rs-out");
    if !use_fixtures {
        if py_out.exists() {
            let _ = fs::remove_dir_all(&py_out);
        }
        let _ = fs::create_dir_all(&py_out);
    }
    if rs_out.exists() {
        let _ = fs::remove_dir_all(&rs_out);
    }
    let _ = fs::create_dir_all(&rs_out);

    let fixtures_root = workspace_root.join(fixtures_dir);
    let chunks_manifest_abs = workspace_root.join(chunks_manifest);
    let source_matrix_abs = workspace_root.join(source_matrix);

    let mut fixtures: Vec<PathBuf> = match fs::read_dir(&fixtures_root) {
        Ok(it) => it
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
            .collect(),
        Err(_) => {
            return EntryReport {
                name: name.to_string(),
                stage,
                expected_command: String::new(),
                rust_command: String::new(),
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: Vec::new(),
                status: EntryStatus::Fail(format!("fixtures_dir missing at {fixtures_root:?}")),
            };
        }
    };
    fixtures.sort();

    let mut comparisons: Vec<ArtifactComparison> = Vec::new();
    let mut python_total_ms: u128 = 0;
    let mut rust_total_ms: u128 = 0;
    let mut python_cmd_str = String::new();
    let mut rust_cmd_str = String::new();
    let mut early_fail: Option<String> = None;

    for fixture in &fixtures {
        let stem = fixture
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("fixture")
            .to_string();
        let py_journal = py_out.join(format!("{stem}.lint_journal.jsonl"));
        let rs_journal = rs_out.join(format!("{stem}.lint_journal.jsonl"));
        let py_stderr = py_out.join(format!("{stem}.stderr"));
        let rs_stderr = rs_out.join(format!("{stem}.stderr"));
        let py_stdout = py_out.join(format!("{stem}.stdout"));
        let rs_stdout = rs_out.join(format!("{stem}.stdout"));
        let py_exit = py_out.join(format!("{stem}.exit"));
        let rs_exit = rs_out.join(format!("{stem}.exit"));

        // Pre-touch both journal paths to empty files so structural-
        // only fixtures (those that short-circuit before any
        // verify_one_card call, e.g. clean-N/A, missing-section,
        // sentinel-collision) leave equal empty journals on both
        // sides. Per-card-verifying fixtures append events on top.
        // append-or-create journal semantics make the pre-touch safe
        // for both implementations.
        let _ = fs::write(&rs_journal, b"");

        if use_fixtures {
            if python_cmd_str.is_empty() {
                python_cmd_str = format!("committed-fixture: {}", py_out.display());
            }
        } else {
            let _ = fs::write(&py_journal, b"");
            let py_args = vec![
                "-m".to_string(),
                "cacg.cli".to_string(),
                "verify".to_string(),
                "--round-summary".to_string(),
                fixture.display().to_string(),
                "--chunks-manifest".to_string(),
                chunks_manifest_abs.display().to_string(),
                "--source-matrix".to_string(),
                source_matrix_abs.display().to_string(),
                "--journal".to_string(),
                py_journal.display().to_string(),
            ];
            if python_cmd_str.is_empty() {
                python_cmd_str = format!(
                    "{} {}",
                    binaries.python_executable.display(),
                    py_args.join(" ")
                );
            }
            let py_start = Instant::now();
            let py_out_res = Command::new(&binaries.python_executable)
                .args(&py_args)
                .env("KB_FROZEN_CLOCK", "1")
                .env("PYTHONPATH", workspace_root.join("src"))
                .current_dir(workspace_root)
                .output();
            python_total_ms += py_start.elapsed().as_millis();
            let py_status = match py_out_res {
                Ok(o) => o,
                Err(e) => {
                    early_fail = Some(format!("python spawn failed: {e}"));
                    break;
                }
            };
            let _ = fs::write(&py_stderr, &py_status.stderr);
            let _ = fs::write(&py_stdout, &py_status.stdout);
            let _ = fs::write(
                &py_exit,
                format!("{}\n", py_status.status.code().unwrap_or(-1)).as_bytes(),
            );
        }

        let rs_args = vec![
            "verify".to_string(),
            "--round-summary".to_string(),
            fixture.display().to_string(),
            "--chunks-manifest".to_string(),
            chunks_manifest_abs.display().to_string(),
            "--source-matrix".to_string(),
            source_matrix_abs.display().to_string(),
            "--journal".to_string(),
            rs_journal.display().to_string(),
        ];
        if rust_cmd_str.is_empty() {
            rust_cmd_str = format!(
                "{} {}",
                binaries.rust_executable.display(),
                rs_args.join(" ")
            );
        }
        let rs_start = Instant::now();
        let rs_out_res = Command::new(&binaries.rust_executable)
            .args(&rs_args)
            .env("KB_FROZEN_CLOCK", "1")
            .current_dir(workspace_root)
            .output();
        rust_total_ms += rs_start.elapsed().as_millis();
        let rs_status = match rs_out_res {
            Ok(o) => o,
            Err(e) => {
                early_fail = Some(format!("rust spawn failed: {e}"));
                break;
            }
        };
        let _ = fs::write(&rs_stderr, &rs_status.stderr);
        let _ = fs::write(&rs_stdout, &rs_status.stdout);
        let _ = fs::write(
            &rs_exit,
            format!("{}\n", rs_status.status.code().unwrap_or(-1)).as_bytes(),
        );

        comparisons.push(compare_artifact(
            format!("{stem}.stdout"),
            &py_stdout,
            &rs_stdout,
        ));
        comparisons.push(compare_artifact(
            format!("{stem}.stderr"),
            &py_stderr,
            &rs_stderr,
        ));
        comparisons.push(compare_artifact(format!("{stem}.exit"), &py_exit, &rs_exit));
        comparisons.push(compare_artifact(
            format!("{stem}.lint_journal.jsonl"),
            &py_journal,
            &rs_journal,
        ));
    }

    let status = if let Some(reason) = early_fail {
        EntryStatus::Fail(reason)
    } else if fixtures.is_empty() {
        EntryStatus::Fail(format!("no .md fixtures under {fixtures_root:?}"))
    } else if comparisons.iter().all(|c| c.equal) {
        EntryStatus::Pass
    } else {
        let n_diff = comparisons.iter().filter(|c| !c.equal).count();
        EntryStatus::Fail(format!(
            "{n_diff} of {} artifacts diffed",
            comparisons.len()
        ))
    };

    EntryReport {
        name: name.to_string(),
        stage,
        expected_command: python_cmd_str,
        rust_command: rust_cmd_str,
        expected_duration_ms: python_total_ms,
        rust_duration_ms: rust_total_ms,
        comparisons,
        status,
    }
}

/// Production entry point for a `MatrixRowKind::KbVerifySemantic`
/// row.
fn run_kb_verify_semantic_entry(
    name: &str,
    stage: Stage,
    out_root: &Path,
    workspace_root: &Path,
    card: &str,
    chunks_manifest: &str,
    source_matrix: &str,
    semantic_cache: &str,
    corpus: &Path,
) -> EntryReport {
    let fixture_dir = committed_fixture_dir(corpus, name);
    run_kb_verify_semantic_entry_with(
        name,
        stage,
        out_root,
        workspace_root,
        card,
        chunks_manifest,
        source_matrix,
        semantic_cache,
        &LintBinaries::production(workspace_root),
        if fixture_dir.is_dir() {
            Some(fixture_dir)
        } else {
            None
        },
    )
}

/// `kb verify <card> --semantic <cache>` single-card byte-parity
/// body. Runs Python and Rust `kb verify <card>` with `--semantic
/// <cache>` side-by-side under `KB_FROZEN_CLOCK=1` and byte-compares
/// stdout, stderr, exit code, and the per-card verify journal. Pins
/// one specific card so the supplied cache's
/// `(chunk_hash, claim_window_hash)` key matches deterministically;
/// the committed fixture under `tests/parity_corpus/semantic/` is
/// regenerated by `legacy_python_oracle/scripts/build_semantic_parity_fixture.py`.
#[allow(clippy::too_many_arguments)]
fn run_kb_verify_semantic_entry_with(
    name: &str,
    stage: Stage,
    out_root: &Path,
    workspace_root: &Path,
    card: &str,
    chunks_manifest: &str,
    source_matrix: &str,
    semantic_cache: &str,
    binaries: &LintBinaries,
    committed_fixtures: Option<PathBuf>,
) -> EntryReport {
    let use_fixtures = committed_fixtures.is_some();
    let py_out = committed_fixtures.unwrap_or_else(|| out_root.join("py-out"));
    let rs_out = out_root.join("rs-out");
    if !use_fixtures {
        if py_out.exists() {
            let _ = fs::remove_dir_all(&py_out);
        }
        let _ = fs::create_dir_all(&py_out);
    }
    if rs_out.exists() {
        let _ = fs::remove_dir_all(&rs_out);
    }
    let _ = fs::create_dir_all(&rs_out);

    let card_abs = workspace_root.join(card);
    let chunks_manifest_abs = workspace_root.join(chunks_manifest);
    let source_matrix_abs = workspace_root.join(source_matrix);
    let semantic_cache_abs = workspace_root.join(semantic_cache);

    let stem = card_abs
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("card")
        .to_string();
    let py_journal = py_out.join(format!("{stem}.lint_journal.jsonl"));
    let rs_journal = rs_out.join(format!("{stem}.lint_journal.jsonl"));
    let py_stderr = py_out.join(format!("{stem}.stderr"));
    let rs_stderr = rs_out.join(format!("{stem}.stderr"));
    let py_stdout = py_out.join(format!("{stem}.stdout"));
    let rs_stdout = rs_out.join(format!("{stem}.stdout"));
    let py_exit = py_out.join(format!("{stem}.exit"));
    let rs_exit = rs_out.join(format!("{stem}.exit"));

    // Pre-touch both journal paths so the absent-journal case is a
    // byte-equal "empty file == empty file" comparison rather than
    // an asymmetric missing-file diff.
    let _ = fs::write(&rs_journal, b"");

    let (python_cmd_str, expected_duration_ms) = if use_fixtures {
        (format!("committed-fixture: {}", py_out.display()), 0)
    } else {
        let _ = fs::write(&py_journal, b"");
        let py_args = vec![
            "-m".to_string(),
            "cacg.cli".to_string(),
            "verify".to_string(),
            card_abs.display().to_string(),
            "--chunks-manifest".to_string(),
            chunks_manifest_abs.display().to_string(),
            "--source-matrix".to_string(),
            source_matrix_abs.display().to_string(),
            "--semantic".to_string(),
            semantic_cache_abs.display().to_string(),
            "--journal".to_string(),
            py_journal.display().to_string(),
        ];
        let cmd_str = format!(
            "{} {}",
            binaries.python_executable.display(),
            py_args.join(" ")
        );
        let py_start = Instant::now();
        let py_out_res = Command::new(&binaries.python_executable)
            .args(&py_args)
            .env("KB_FROZEN_CLOCK", "1")
            .env("PYTHONPATH", workspace_root.join("src"))
            .current_dir(workspace_root)
            .output();
        let dur = py_start.elapsed().as_millis();
        let py_status = match py_out_res {
            Ok(o) => o,
            Err(e) => {
                return EntryReport {
                    name: name.to_string(),
                    stage,
                    expected_command: cmd_str,
                    rust_command: String::new(),
                    expected_duration_ms: dur,
                    rust_duration_ms: 0,
                    comparisons: Vec::new(),
                    status: EntryStatus::Fail(format!("python spawn failed: {e}")),
                };
            }
        };
        let _ = fs::write(&py_stderr, &py_status.stderr);
        let _ = fs::write(&py_stdout, &py_status.stdout);
        let _ = fs::write(
            &py_exit,
            format!("{}\n", py_status.status.code().unwrap_or(-1)).as_bytes(),
        );
        (cmd_str, dur)
    };

    let rs_args = vec![
        "verify".to_string(),
        card_abs.display().to_string(),
        "--chunks-manifest".to_string(),
        chunks_manifest_abs.display().to_string(),
        "--source-matrix".to_string(),
        source_matrix_abs.display().to_string(),
        "--semantic".to_string(),
        semantic_cache_abs.display().to_string(),
        "--journal".to_string(),
        rs_journal.display().to_string(),
    ];
    let rust_cmd_str = format!(
        "{} {}",
        binaries.rust_executable.display(),
        rs_args.join(" ")
    );
    let rs_start = Instant::now();
    let rs_out_res = Command::new(&binaries.rust_executable)
        .args(&rs_args)
        .env("KB_FROZEN_CLOCK", "1")
        .current_dir(workspace_root)
        .output();
    let rust_duration_ms = rs_start.elapsed().as_millis();
    let rs_status = match rs_out_res {
        Ok(o) => o,
        Err(e) => {
            return EntryReport {
                name: name.to_string(),
                stage,
                expected_command: python_cmd_str,
                rust_command: rust_cmd_str,
                expected_duration_ms,
                rust_duration_ms,
                comparisons: Vec::new(),
                status: EntryStatus::Fail(format!("rust spawn failed: {e}")),
            };
        }
    };
    let _ = fs::write(&rs_stderr, &rs_status.stderr);
    let _ = fs::write(&rs_stdout, &rs_status.stdout);
    let _ = fs::write(
        &rs_exit,
        format!("{}\n", rs_status.status.code().unwrap_or(-1)).as_bytes(),
    );

    let comparisons = vec![
        compare_artifact(format!("{stem}.stdout"), &py_stdout, &rs_stdout),
        compare_artifact(format!("{stem}.stderr"), &py_stderr, &rs_stderr),
        compare_artifact(format!("{stem}.exit"), &py_exit, &rs_exit),
        compare_artifact(
            format!("{stem}.lint_journal.jsonl"),
            &py_journal,
            &rs_journal,
        ),
    ];

    let status = if comparisons.iter().all(|c| c.equal) {
        EntryStatus::Pass
    } else {
        let n_diff = comparisons.iter().filter(|c| !c.equal).count();
        EntryStatus::Fail(format!(
            "{n_diff} of {} artifacts diffed",
            comparisons.len()
        ))
    };

    EntryReport {
        name: name.to_string(),
        stage,
        expected_command: python_cmd_str,
        rust_command: rust_cmd_str,
        expected_duration_ms,
        rust_duration_ms,
        comparisons,
        status,
    }
}

/// Production entry point for a
/// `MatrixRowKind::KbVerifyRoundSummarySemantic` row.
fn run_kb_verify_round_summary_semantic_entry(
    name: &str,
    stage: Stage,
    out_root: &Path,
    workspace_root: &Path,
    summary: &str,
    chunks_manifest: &str,
    source_matrix: &str,
    semantic_cache: &str,
    corpus: &Path,
) -> EntryReport {
    let fixture_dir = committed_fixture_dir(corpus, name);
    run_kb_verify_round_summary_semantic_entry_with(
        name,
        stage,
        out_root,
        workspace_root,
        summary,
        chunks_manifest,
        source_matrix,
        semantic_cache,
        &LintBinaries::production(workspace_root),
        if fixture_dir.is_dir() {
            Some(fixture_dir)
        } else {
            None
        },
    )
}

/// `kb verify --round-summary <summary> --semantic <cache>` byte-
/// parity body. Mirrors [`run_kb_verify_semantic_entry_with`] but
/// uses the `--round-summary <summary>` invocation form. Pins one
/// specific summary fixture so the supplied cache's
/// `(chunk_hash, claim_window_hash)` key matches deterministically;
/// the committed fixture under `tests/parity_corpus/round_summary_semantic/`
/// cites a single card from `tests/parity_corpus/semantic/`.
#[allow(clippy::too_many_arguments)]
fn run_kb_verify_round_summary_semantic_entry_with(
    name: &str,
    stage: Stage,
    out_root: &Path,
    workspace_root: &Path,
    summary: &str,
    chunks_manifest: &str,
    source_matrix: &str,
    semantic_cache: &str,
    binaries: &LintBinaries,
    committed_fixtures: Option<PathBuf>,
) -> EntryReport {
    let use_fixtures = committed_fixtures.is_some();
    let py_out = committed_fixtures.unwrap_or_else(|| out_root.join("py-out"));
    let rs_out = out_root.join("rs-out");
    if !use_fixtures {
        if py_out.exists() {
            let _ = fs::remove_dir_all(&py_out);
        }
        let _ = fs::create_dir_all(&py_out);
    }
    if rs_out.exists() {
        let _ = fs::remove_dir_all(&rs_out);
    }
    let _ = fs::create_dir_all(&rs_out);

    let summary_abs = workspace_root.join(summary);
    let chunks_manifest_abs = workspace_root.join(chunks_manifest);
    let source_matrix_abs = workspace_root.join(source_matrix);
    let semantic_cache_abs = workspace_root.join(semantic_cache);

    let stem = summary_abs
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("summary")
        .to_string();
    let py_journal = py_out.join(format!("{stem}.lint_journal.jsonl"));
    let rs_journal = rs_out.join(format!("{stem}.lint_journal.jsonl"));
    let py_stderr = py_out.join(format!("{stem}.stderr"));
    let rs_stderr = rs_out.join(format!("{stem}.stderr"));
    let py_stdout = py_out.join(format!("{stem}.stdout"));
    let rs_stdout = rs_out.join(format!("{stem}.stdout"));
    let py_exit = py_out.join(format!("{stem}.exit"));
    let rs_exit = rs_out.join(format!("{stem}.exit"));

    // Pre-touch journal paths so a structural-only round summary
    // (which would short-circuit before any per-card verify
    // invocation) compares byte-equal "empty vs empty" rather than
    // an asymmetric missing-file diff.
    let _ = fs::write(&rs_journal, b"");

    let (python_cmd_str, expected_duration_ms) = if use_fixtures {
        (format!("committed-fixture: {}", py_out.display()), 0)
    } else {
        let _ = fs::write(&py_journal, b"");
        let py_args = vec![
            "-m".to_string(),
            "cacg.cli".to_string(),
            "verify".to_string(),
            "--round-summary".to_string(),
            summary_abs.display().to_string(),
            "--chunks-manifest".to_string(),
            chunks_manifest_abs.display().to_string(),
            "--source-matrix".to_string(),
            source_matrix_abs.display().to_string(),
            "--semantic".to_string(),
            semantic_cache_abs.display().to_string(),
            "--journal".to_string(),
            py_journal.display().to_string(),
        ];
        let cmd_str = format!(
            "{} {}",
            binaries.python_executable.display(),
            py_args.join(" ")
        );
        let py_start = Instant::now();
        let py_out_res = Command::new(&binaries.python_executable)
            .args(&py_args)
            .env("KB_FROZEN_CLOCK", "1")
            .env("PYTHONPATH", workspace_root.join("src"))
            .current_dir(workspace_root)
            .output();
        let dur = py_start.elapsed().as_millis();
        let py_status = match py_out_res {
            Ok(o) => o,
            Err(e) => {
                return EntryReport {
                    name: name.to_string(),
                    stage,
                    expected_command: cmd_str,
                    rust_command: String::new(),
                    expected_duration_ms: dur,
                    rust_duration_ms: 0,
                    comparisons: Vec::new(),
                    status: EntryStatus::Fail(format!("python spawn failed: {e}")),
                };
            }
        };
        let _ = fs::write(&py_stderr, &py_status.stderr);
        let _ = fs::write(&py_stdout, &py_status.stdout);
        let _ = fs::write(
            &py_exit,
            format!("{}\n", py_status.status.code().unwrap_or(-1)).as_bytes(),
        );
        (cmd_str, dur)
    };

    let rs_args = vec![
        "verify".to_string(),
        "--round-summary".to_string(),
        summary_abs.display().to_string(),
        "--chunks-manifest".to_string(),
        chunks_manifest_abs.display().to_string(),
        "--source-matrix".to_string(),
        source_matrix_abs.display().to_string(),
        "--semantic".to_string(),
        semantic_cache_abs.display().to_string(),
        "--journal".to_string(),
        rs_journal.display().to_string(),
    ];
    let rust_cmd_str = format!(
        "{} {}",
        binaries.rust_executable.display(),
        rs_args.join(" ")
    );
    let rs_start = Instant::now();
    let rs_out_res = Command::new(&binaries.rust_executable)
        .args(&rs_args)
        .env("KB_FROZEN_CLOCK", "1")
        .current_dir(workspace_root)
        .output();
    let rust_duration_ms = rs_start.elapsed().as_millis();
    let rs_status = match rs_out_res {
        Ok(o) => o,
        Err(e) => {
            return EntryReport {
                name: name.to_string(),
                stage,
                expected_command: python_cmd_str,
                rust_command: rust_cmd_str,
                expected_duration_ms,
                rust_duration_ms,
                comparisons: Vec::new(),
                status: EntryStatus::Fail(format!("rust spawn failed: {e}")),
            };
        }
    };
    let _ = fs::write(&rs_stderr, &rs_status.stderr);
    let _ = fs::write(&rs_stdout, &rs_status.stdout);
    let _ = fs::write(
        &rs_exit,
        format!("{}\n", rs_status.status.code().unwrap_or(-1)).as_bytes(),
    );

    let comparisons = vec![
        compare_artifact(format!("{stem}.stdout"), &py_stdout, &rs_stdout),
        compare_artifact(format!("{stem}.stderr"), &py_stderr, &rs_stderr),
        compare_artifact(format!("{stem}.exit"), &py_exit, &rs_exit),
        compare_artifact(
            format!("{stem}.lint_journal.jsonl"),
            &py_journal,
            &rs_journal,
        ),
    ];

    let status = if comparisons.iter().all(|c| c.equal) {
        EntryStatus::Pass
    } else {
        let n_diff = comparisons.iter().filter(|c| !c.equal).count();
        EntryStatus::Fail(format!(
            "{n_diff} of {} artifacts diffed",
            comparisons.len()
        ))
    };

    EntryReport {
        name: name.to_string(),
        stage,
        expected_command: python_cmd_str,
        rust_command: rust_cmd_str,
        expected_duration_ms,
        rust_duration_ms,
        comparisons,
        status,
    }
}

/// Production entry point for a `MatrixRowKind::KbSearch` row. Reuses
/// [`LintBinaries`] as the (python, rust) executable pair — the struct
/// is a generic binary pair; the `Lint` in its name is incidental.
fn run_kb_search_entry(
    name: &str,
    stage: Stage,
    corpus_dir: &Path,
    out_root: &Path,
    workspace_root: &Path,
    cases: &[KbSearchCase],
    corpus: &Path,
) -> EntryReport {
    let fixture_dir = committed_fixture_dir(corpus, name);
    run_kb_search_entry_with(
        name,
        stage,
        corpus_dir,
        out_root,
        workspace_root,
        cases,
        &LintBinaries::production(workspace_root),
        if fixture_dir.is_dir() {
            Some(fixture_dir)
        } else {
            None
        },
    )
}

/// `MatrixRowKind::KbSearch` body. For each [`KbSearchCase`], runs
/// Python `python -m cacg.cli search <query> [--json] [--top-k N]
/// --summaries <corpus>/summaries.json --source-matrix
/// <corpus>/source_matrix.json` and the Rust `kb search` equivalent
/// under `KB_FROZEN_CLOCK=1`, then byte-compares stdout, stderr, and
/// exit code per case. `kb search` is read-only — no journal, no
/// corpus mutation — so the corpus is consumed in place with no
/// isolated copy.
fn run_kb_search_entry_with(
    name: &str,
    stage: Stage,
    corpus_dir: &Path,
    out_root: &Path,
    workspace_root: &Path,
    cases: &[KbSearchCase],
    binaries: &LintBinaries,
    committed_fixtures: Option<PathBuf>,
) -> EntryReport {
    let use_fixtures = committed_fixtures.is_some();
    if !corpus_dir.is_dir() {
        return EntryReport {
            name: name.to_string(),
            stage,
            expected_command: String::new(),
            rust_command: String::new(),
            expected_duration_ms: 0,
            rust_duration_ms: 0,
            comparisons: Vec::new(),
            status: EntryStatus::Fail(format!("corpus subdir missing at {corpus_dir:?}")),
        };
    }

    let py_out = committed_fixtures.unwrap_or_else(|| out_root.join("py-out"));
    let rs_out = out_root.join("rs-out");
    if !use_fixtures {
        if py_out.exists() {
            let _ = fs::remove_dir_all(&py_out);
        }
        let _ = fs::create_dir_all(&py_out);
    }
    if rs_out.exists() {
        let _ = fs::remove_dir_all(&rs_out);
    }
    let _ = fs::create_dir_all(&rs_out);

    let summaries = corpus_dir.join("summaries.json");
    let source_matrix = corpus_dir.join("source_matrix.json");

    let mut comparisons: Vec<ArtifactComparison> = Vec::new();
    let mut python_total_ms: u128 = 0;
    let mut rust_total_ms: u128 = 0;
    let mut python_cmd_str = String::new();
    let mut rust_cmd_str = String::new();
    let mut early_fail: Option<String> = None;

    for case in cases {
        match run_kb_search_case(
            case,
            &summaries,
            &source_matrix,
            &py_out,
            &rs_out,
            workspace_root,
            binaries,
            use_fixtures,
        ) {
            Ok(run) => {
                python_total_ms += run.python_ms;
                rust_total_ms += run.rust_ms;
                if python_cmd_str.is_empty() {
                    python_cmd_str = run.expected_command;
                }
                if rust_cmd_str.is_empty() {
                    rust_cmd_str = run.rust_command;
                }
                comparisons.extend(run.comparisons);
            }
            Err(reason) => {
                early_fail = Some(reason);
                break;
            }
        }
    }

    let status = if let Some(reason) = early_fail {
        EntryStatus::Fail(reason)
    } else if cases.is_empty() {
        EntryStatus::Fail("no kb search cases configured".to_string())
    } else if comparisons.iter().all(|c| c.equal) {
        EntryStatus::Pass
    } else {
        let n_diff = comparisons.iter().filter(|c| !c.equal).count();
        EntryStatus::Fail(format!(
            "{n_diff} of {} artifacts diffed",
            comparisons.len()
        ))
    };

    EntryReport {
        name: name.to_string(),
        stage,
        expected_command: python_cmd_str,
        rust_command: rust_cmd_str,
        expected_duration_ms: python_total_ms,
        rust_duration_ms: rust_total_ms,
        comparisons,
        status,
    }
}

/// The result of running one `kb search` / `kb show` parity case
/// against both implementations: the three byte-comparisons (stdout /
/// stderr / exit), each side's wall-clock duration, and the rendered
/// command strings for the perf-report.
struct CliParityCaseRun {
    comparisons: Vec<ArtifactComparison>,
    python_ms: u128,
    rust_ms: u128,
    expected_command: String,
    rust_command: String,
}

/// Run one `kb search` query case against both implementations: spawn
/// Python and Rust `kb search`, capture stdout / stderr / exit into the
/// `py-out` / `rs-out` dirs, and byte-compare the three artifacts. An
/// `Err` carries a spawn-failure reason for the caller to surface as
/// the row's `EntryStatus::Fail`.
fn run_kb_search_case(
    case: &KbSearchCase,
    summaries: &Path,
    source_matrix: &Path,
    py_out: &Path,
    rs_out: &Path,
    workspace_root: &Path,
    binaries: &LintBinaries,
    use_fixtures: bool,
) -> Result<CliParityCaseRun, String> {
    // Trailing args shared by both implementations: the summaries +
    // source-matrix paths and the optional `--json` / `--top-k` flags.
    // `kb search`'s query is a positional, so it precedes this shared
    // tail on each side.
    let mut shared_tail: Vec<String> = vec![
        "--summaries".to_string(),
        summaries.display().to_string(),
        "--source-matrix".to_string(),
        source_matrix.display().to_string(),
    ];
    if case.json {
        shared_tail.push("--json".to_string());
    }
    if let Some(k) = case.top_k {
        shared_tail.push("--top-k".to_string());
        shared_tail.push(k.to_string());
    }

    let mut rs_args = vec!["search".to_string(), case.query.to_string()];
    rs_args.extend(shared_tail.iter().cloned());

    let py_stdout = py_out.join(format!("{}.stdout", case.label));
    let rs_stdout = rs_out.join(format!("{}.stdout", case.label));
    let py_stderr = py_out.join(format!("{}.stderr", case.label));
    let rs_stderr = rs_out.join(format!("{}.stderr", case.label));
    let py_exit = py_out.join(format!("{}.exit", case.label));
    let rs_exit = rs_out.join(format!("{}.exit", case.label));

    let (expected_command, python_ms) = if use_fixtures {
        (format!("committed-fixture: {}", py_out.display()), 0)
    } else {
        let mut py_args = vec![
            "-m".to_string(),
            "cacg.cli".to_string(),
            "search".to_string(),
            case.query.to_string(),
        ];
        py_args.extend(shared_tail.iter().cloned());
        let cmd = format!(
            "{} {}",
            binaries.python_executable.display(),
            py_args.join(" ")
        );
        let py_start = Instant::now();
        let py_res = Command::new(&binaries.python_executable)
            .args(&py_args)
            .env("KB_FROZEN_CLOCK", "1")
            .env("PYTHONPATH", workspace_root.join("src"))
            .current_dir(workspace_root)
            .output();
        let ms = py_start.elapsed().as_millis();
        let py_status = py_res.map_err(|e| format!("python spawn failed: {e}"))?;
        let _ = fs::write(&py_stdout, &py_status.stdout);
        let _ = fs::write(&py_stderr, &py_status.stderr);
        let _ = fs::write(
            &py_exit,
            format!("{}\n", py_status.status.code().unwrap_or(-1)).as_bytes(),
        );
        (cmd, ms)
    };

    let rust_command = format!(
        "{} {}",
        binaries.rust_executable.display(),
        rs_args.join(" ")
    );

    let rs_start = Instant::now();
    let rs_res = Command::new(&binaries.rust_executable)
        .args(&rs_args)
        .env("KB_FROZEN_CLOCK", "1")
        .current_dir(workspace_root)
        .output();
    let rust_ms = rs_start.elapsed().as_millis();
    let rs_status = rs_res.map_err(|e| format!("rust spawn failed: {e}"))?;

    let _ = fs::write(&rs_stdout, &rs_status.stdout);
    let _ = fs::write(&rs_stderr, &rs_status.stderr);
    let _ = fs::write(
        &rs_exit,
        format!("{}\n", rs_status.status.code().unwrap_or(-1)).as_bytes(),
    );

    Ok(CliParityCaseRun {
        comparisons: vec![
            compare_artifact(format!("{}.stdout", case.label), &py_stdout, &rs_stdout),
            compare_artifact(format!("{}.stderr", case.label), &py_stderr, &rs_stderr),
            compare_artifact(format!("{}.exit", case.label), &py_exit, &rs_exit),
        ],
        python_ms,
        rust_ms,
        expected_command,
        rust_command,
    })
}

/// Production entry point for a `MatrixRowKind::KbShow` row.
fn run_kb_show_entry(
    name: &str,
    stage: Stage,
    out_root: &Path,
    workspace_root: &Path,
    cases: &[KbShowCase],
    corpus: &Path,
) -> EntryReport {
    let fixture_dir = committed_fixture_dir(corpus, name);
    run_kb_show_entry_with(
        name,
        stage,
        out_root,
        workspace_root,
        cases,
        &LintBinaries::production(workspace_root),
        if fixture_dir.is_dir() {
            Some(fixture_dir)
        } else {
            None
        },
    )
}

/// `MatrixRowKind::KbShow` body. For each [`KbShowCase`], runs Python
/// and Rust `kb show` under `KB_FROZEN_CLOCK=1` and byte-compares
/// stdout, stderr, and exit code. `kb show` is read-only and each case
/// carries its own manifest + matrix, so there is no shared corpus
/// directory to validate.
fn run_kb_show_entry_with(
    name: &str,
    stage: Stage,
    out_root: &Path,
    workspace_root: &Path,
    cases: &[KbShowCase],
    binaries: &LintBinaries,
    committed_fixtures: Option<PathBuf>,
) -> EntryReport {
    let use_fixtures = committed_fixtures.is_some();
    let py_out = committed_fixtures.unwrap_or_else(|| out_root.join("py-out"));
    let rs_out = out_root.join("rs-out");
    if !use_fixtures {
        if py_out.exists() {
            let _ = fs::remove_dir_all(&py_out);
        }
        let _ = fs::create_dir_all(&py_out);
    }
    if rs_out.exists() {
        let _ = fs::remove_dir_all(&rs_out);
    }
    let _ = fs::create_dir_all(&rs_out);

    let mut comparisons: Vec<ArtifactComparison> = Vec::new();
    let mut python_total_ms: u128 = 0;
    let mut rust_total_ms: u128 = 0;
    let mut python_cmd_str = String::new();
    let mut rust_cmd_str = String::new();
    let mut early_fail: Option<String> = None;

    for case in cases {
        match run_kb_show_case(
            case,
            &py_out,
            &rs_out,
            workspace_root,
            binaries,
            use_fixtures,
        ) {
            Ok(run) => {
                python_total_ms += run.python_ms;
                rust_total_ms += run.rust_ms;
                if python_cmd_str.is_empty() {
                    python_cmd_str = run.expected_command;
                }
                if rust_cmd_str.is_empty() {
                    rust_cmd_str = run.rust_command;
                }
                comparisons.extend(run.comparisons);
            }
            Err(reason) => {
                early_fail = Some(reason);
                break;
            }
        }
    }

    let status = if let Some(reason) = early_fail {
        EntryStatus::Fail(reason)
    } else if cases.is_empty() {
        EntryStatus::Fail("no kb show cases configured".to_string())
    } else if comparisons.iter().all(|c| c.equal) {
        EntryStatus::Pass
    } else {
        let n_diff = comparisons.iter().filter(|c| !c.equal).count();
        EntryStatus::Fail(format!(
            "{n_diff} of {} artifacts diffed",
            comparisons.len()
        ))
    };

    EntryReport {
        name: name.to_string(),
        stage,
        expected_command: python_cmd_str,
        rust_command: rust_cmd_str,
        expected_duration_ms: python_total_ms,
        rust_duration_ms: rust_total_ms,
        comparisons,
        status,
    }
}

/// Run one `kb show` case against both implementations: spawn Python
/// and Rust `kb show`, capture stdout / stderr / exit into the
/// `py-out` / `rs-out` dirs, and byte-compare the three artifacts. An
/// `Err` carries a spawn-failure reason for the caller to surface as
/// the row's `EntryStatus::Fail`.
fn run_kb_show_case(
    case: &KbShowCase,
    py_out: &Path,
    rs_out: &Path,
    workspace_root: &Path,
    binaries: &LintBinaries,
    use_fixtures: bool,
) -> Result<CliParityCaseRun, String> {
    // Trailing args shared by both implementations. Paths stay
    // workspace-relative (the spawn runs with `cwd = workspace_root`),
    // which keeps the `--path` value `..`-free + non-absolute so the
    // Rust `CACG-SHOW-003` guard does not fire and diverge from Python.
    let mut tail: Vec<String> = vec![
        case.card_id.to_string(),
        "--cards-manifest".to_string(),
        case.cards_manifest.to_string(),
        "--source-matrix".to_string(),
        case.source_matrix.to_string(),
    ];
    if let Some(p) = case.path {
        tail.push("--path".to_string());
        tail.push(p.to_string());
    }
    if case.allow_retracted {
        tail.push("--allow-retracted".to_string());
    }

    let mut rs_args = vec!["show".to_string()];
    rs_args.extend(tail.iter().cloned());

    let py_stdout = py_out.join(format!("{}.stdout", case.label));
    let rs_stdout = rs_out.join(format!("{}.stdout", case.label));
    let py_stderr = py_out.join(format!("{}.stderr", case.label));
    let rs_stderr = rs_out.join(format!("{}.stderr", case.label));
    let py_exit = py_out.join(format!("{}.exit", case.label));
    let rs_exit = rs_out.join(format!("{}.exit", case.label));

    let (expected_command, python_ms) = if use_fixtures {
        (format!("committed-fixture: {}", py_out.display()), 0)
    } else {
        let mut py_args = vec!["-m".to_string(), "cacg.cli".to_string(), "show".to_string()];
        py_args.extend(tail.iter().cloned());
        let cmd = format!(
            "{} {}",
            binaries.python_executable.display(),
            py_args.join(" ")
        );
        let py_start = Instant::now();
        let py_res = Command::new(&binaries.python_executable)
            .args(&py_args)
            .env("KB_FROZEN_CLOCK", "1")
            .env("PYTHONPATH", workspace_root.join("src"))
            .current_dir(workspace_root)
            .output();
        let ms = py_start.elapsed().as_millis();
        let py_status = py_res.map_err(|e| format!("python spawn failed: {e}"))?;
        let _ = fs::write(&py_stdout, &py_status.stdout);
        let _ = fs::write(&py_stderr, &py_status.stderr);
        let _ = fs::write(
            &py_exit,
            format!("{}\n", py_status.status.code().unwrap_or(-1)).as_bytes(),
        );
        (cmd, ms)
    };

    let rust_command = format!(
        "{} {}",
        binaries.rust_executable.display(),
        rs_args.join(" ")
    );

    let rs_start = Instant::now();
    let rs_res = Command::new(&binaries.rust_executable)
        .args(&rs_args)
        .env("KB_FROZEN_CLOCK", "1")
        .current_dir(workspace_root)
        .output();
    let rust_ms = rs_start.elapsed().as_millis();
    let rs_status = rs_res.map_err(|e| format!("rust spawn failed: {e}"))?;

    let _ = fs::write(&rs_stdout, &rs_status.stdout);
    let _ = fs::write(&rs_stderr, &rs_status.stderr);
    let _ = fs::write(
        &rs_exit,
        format!("{}\n", rs_status.status.code().unwrap_or(-1)).as_bytes(),
    );

    Ok(CliParityCaseRun {
        comparisons: vec![
            compare_artifact(format!("{}.stdout", case.label), &py_stdout, &rs_stdout),
            compare_artifact(format!("{}.stderr", case.label), &py_stderr, &rs_stderr),
            compare_artifact(format!("{}.exit", case.label), &py_exit, &rs_exit),
        ],
        python_ms,
        rust_ms,
        expected_command,
        rust_command,
    })
}

/// `MatrixRowKind::KbIngest` body. Spawns Rust `kb ingest` against
/// the committed PDF fixture under `KB_FROZEN_CLOCK=1` and
/// byte-compares the published manifests against the committed
/// Python oracle. The Python side is the committed oracle file
/// bytes, NOT a live `python -m cacg.cli ingest` run — AC-5's
/// wording is "compares the result against the committed
/// out_python pair," so the gate is one-directional.
///
/// `chunks_manifest.json` is compared raw-byte. `sources_manifest.json`
/// is compared after stripping the two DEC-2-whitelisted fields
/// (`parser_name`, `parser_version`) from both sides and
/// re-canonicalizing via `cacg_core::canonical_json::canonical_json`
/// — strictly stronger than parsed-JSON `serde_json::Value`
/// equality, which would silently pass a `30` vs `30.0` numeric
/// drift or an ASCII-escape regression (the Round-10 review P1).
///
/// When `libpdfium` is unavailable (the `kb` binary exits with
/// `CACG-INGEST-001: pdfium bind failed: …`), the row marks itself
/// `EntryStatus::FutureStage("M4-pdfium-provisioning")` so it
/// reports without gating; setting `CACG_REQUIRE_PDFIUM=1`
/// escalates the same condition to `Fail` for CI runners that
/// have completed the operator setup at
/// `docs/pdfium-binary-provisioning.md`.
fn run_kb_ingest_entry(
    name: &str,
    stage: Stage,
    out_root: &Path,
    workspace_root: &Path,
    pdf: &str,
    oracle_dir: &str,
    source_id: &str,
) -> EntryReport {
    use cacg_core::diagnostic::codes as cc;

    // Promote the relative-path contract to a debug-time invariant.
    // The Rust binary records `source_path` verbatim from argv, and
    // the committed Python oracle was produced with the
    // workspace-relative path. A future refactor that flips this to
    // `pdf_abs` would silently break the oracle parity; this assert
    // fires immediately in debug builds (which xtask uses) so the
    // bug surfaces at the row body rather than as a confusing
    // byte-diff downstream.
    debug_assert!(
        !Path::new(pdf).is_absolute(),
        "MatrixRowKind::KbIngest::pdf must be workspace-relative, not absolute; \
         see Round-9/Round-11 source_path divergence note."
    );

    let rs_out = out_root.join("rs-out");
    if rs_out.exists() {
        let _ = fs::remove_dir_all(&rs_out);
    }
    let _ = fs::create_dir_all(&rs_out);

    let pdf_abs = workspace_root.join(pdf);
    let oracle_chunks = workspace_root.join(oracle_dir).join("chunks_manifest.json");
    let oracle_sources = workspace_root
        .join(oracle_dir)
        .join("sources_manifest.json");
    let kb_bin = workspace_root.join("target/debug/kb");

    let mut rust_command = format!(
        "{} ingest {} --out {} --source-id {}",
        kb_bin.display(),
        pdf,
        rs_out.display(),
        source_id,
    );

    // Fixtures + oracles must exist on disk; in a fresh checkout
    // both are committed. Treat absence as a hard Fail (not Skip)
    // — there is no legitimate state where the committed parity
    // corpus disappears.
    for required in [&pdf_abs, &oracle_chunks, &oracle_sources] {
        if !required.is_file() {
            return EntryReport {
                name: name.to_string(),
                stage,
                expected_command: format!("<committed oracle> {}", oracle_dir),
                rust_command,
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: Vec::new(),
                status: EntryStatus::Fail(format!(
                    "required parity artifact missing on disk: {}",
                    required.display()
                )),
            };
        }
    }

    // ALWAYS rebuild the `kb` binary with `--features ingest`. The
    // prior existence-check fallback (Round 11) silently fell
    // through to a `CACG-CLI-NOT-IMPLEMENTED-ingest` stub when an
    // earlier `cargo build --no-default-features` left a stale,
    // ingest-less binary on disk — a confusing "row failed"
    // diagnostic the Round-11 review (P2.2) flagged. `cargo build`
    // itself does the up-to-date check and is cheap when nothing
    // changed; the cost of an unconditional invocation is well
    // worth the diagnostic clarity.
    let build_status = Command::new("cargo")
        .args([
            "build",
            "--quiet",
            "-p",
            "cacg-cli",
            "--bin",
            "kb",
            "--features",
            "ingest",
        ])
        .current_dir(workspace_root)
        .status();
    match build_status {
        Ok(s) if s.success() => {}
        other => {
            return EntryReport {
                name: name.to_string(),
                stage,
                expected_command: format!("<committed oracle> {}", oracle_dir),
                rust_command,
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: Vec::new(),
                status: EntryStatus::Fail(format!(
                    "cargo build of kb binary failed before parity run: {other:?}"
                )),
            };
        }
    }
    rust_command.push_str("  (cargo build --features ingest invoked)");

    let rs_start = Instant::now();
    let rs_res = Command::new(&kb_bin)
        .arg("ingest")
        .arg(pdf)
        .arg("--out")
        .arg(&rs_out)
        .arg("--source-id")
        .arg(source_id)
        .env("KB_FROZEN_CLOCK", "1")
        .current_dir(workspace_root)
        .output();
    let rust_duration_ms = rs_start.elapsed().as_millis();

    let rs_status = match rs_res {
        Ok(s) => s,
        Err(e) => {
            return EntryReport {
                name: name.to_string(),
                stage,
                expected_command: format!("<committed oracle> {}", oracle_dir),
                rust_command,
                expected_duration_ms: 0,
                rust_duration_ms,
                comparisons: Vec::new(),
                status: EntryStatus::Fail(format!("kb spawn failed: {e}")),
            };
        }
    };

    if !rs_status.status.success() {
        let stderr_str = String::from_utf8_lossy(&rs_status.stderr).into_owned();
        // Use the project's stable diagnostic-code prefix
        // (`cacg_core::diagnostic::codes::INGEST_001` + the
        // `pdfium bind failed` literal that `extract_pages_impl`
        // emits) — strictly more specific than the prior
        // `libpdfium.so` substring, which would have matched a
        // future pdfium-render error like "libpdfium.so loaded
        // but FPDF_InitLibrary returned -1" that is NOT a
        // missing-binary case and must surface as a hard Fail.
        // Round-11 review P3.2 tightening.
        let pdfium_bind_prefix = format!("{}: pdfium bind failed", cc::INGEST_001);
        let pdfium_unavailable = stderr_str.contains(&pdfium_bind_prefix);
        let require_pdfium = std::env::var("CACG_REQUIRE_PDFIUM").as_deref() == Ok("1");
        let status = if pdfium_unavailable && !require_pdfium {
            EntryStatus::FutureStage("M4-pdfium-provisioning")
        } else if pdfium_unavailable {
            EntryStatus::Fail(format!(
                "libpdfium unavailable and CACG_REQUIRE_PDFIUM=1 \
                 (see docs/pdfium-binary-provisioning.md): stderr={stderr_str}"
            ))
        } else {
            EntryStatus::Fail(format!(
                "kb ingest exited non-zero (code={:?}): stderr={stderr_str}",
                rs_status.status.code()
            ))
        };
        return EntryReport {
            name: name.to_string(),
            stage,
            expected_command: format!("<committed oracle> {}", oracle_dir),
            rust_command,
            expected_duration_ms: 0,
            rust_duration_ms,
            comparisons: Vec::new(),
            status,
        };
    }

    let rs_chunks = rs_out.join("chunks_manifest.json");
    let rs_sources = rs_out.join("sources_manifest.json");

    let mut comparisons: Vec<ArtifactComparison> = Vec::new();
    // chunks_manifest.json: raw byte comparison via the standard
    // file-pair API.
    comparisons.push(compare_artifact(
        "chunks_manifest.json",
        &oracle_chunks,
        &rs_chunks,
    ));

    // sources_manifest.json: read both, strip the DEC-2 whitelist
    // via the shared `cacg_core::parity` helper (so xtask + the
    // kb_ingest_parity.rs integration test stay in lockstep with
    // a single source of truth — Round-11 review P2.1), and
    // byte-compare via `compare_artifact_bytes_at` so the
    // structured-diff path strings name the REAL artifact paths
    // rather than the help-snapshot `<rust-introspection>/...`
    // sentinel (Round-11 review P2.3).
    let py_sources_stripped = canonicalize_oracle_sources(&oracle_sources, "oracle");
    let rs_sources_stripped = canonicalize_oracle_sources(&rs_sources, "rust");
    comparisons.push(compare_artifact_bytes_at(
        "sources_manifest.json (DEC-2 whitelist stripped)",
        py_sources_stripped.as_deref(),
        rs_sources_stripped.as_deref(),
        &oracle_sources,
        &rs_sources,
    ));

    let status = if comparisons.iter().all(|c| c.equal) {
        EntryStatus::Pass
    } else {
        let n_diff = comparisons.iter().filter(|c| !c.equal).count();
        EntryStatus::Fail(format!(
            "AC-5 BYTE-EQUAL violated: {n_diff} of {} artifacts diffed",
            comparisons.len()
        ))
    };

    EntryReport {
        name: name.to_string(),
        stage,
        expected_command: format!("<committed oracle> {}", oracle_dir),
        rust_command,
        expected_duration_ms: 0,
        rust_duration_ms,
        comparisons,
        status,
    }
}

/// Read `sources_manifest.json` from disk and route it through the
/// shared `cacg_core::parity::canonicalize_sources_minus_dec2_whitelist`
/// helper, the single source of truth for the AC-5 BYTE-EQUAL
/// sources contract. Returns `None` on read failure or whitelist
/// strip failure so the caller's `compare_artifact_bytes_at`
/// reports the row as a diff with a `MissingExpected` / `MissingRust`
/// structured diagnostic — better than panicking inside the
/// harness body. The `label` is used in the stderr diagnostic so
/// an oracle vs rust failure is locally diagnosable.
fn canonicalize_oracle_sources(path: &Path, label: &str) -> Option<Vec<u8>> {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "kb_ingest parity: cannot read {label} sources at {}: {e}",
                path.display()
            );
            return None;
        }
    };
    match cacg_core::parity::canonicalize_sources_minus_dec2_whitelist(&bytes) {
        Ok(stripped) => Some(stripped),
        Err(e) => {
            eprintln!(
                "kb_ingest parity: DEC-2 whitelist strip failed on {label} \
                 sources at {}: {e}",
                path.display()
            );
            None
        }
    }
}

/// Variant of [`compare_artifact_bytes`] that records the REAL
/// artifact paths in the resulting `ArtifactComparison` rather
/// than the `<rust-introspection>/<name>` sentinel paths
/// `compare_artifact_bytes` uses (those sentinels are correct for
/// the help-snapshot rows that synthesize bytes from
/// introspection, but misleading for any row that compares actual
/// on-disk artifacts). The body is otherwise identical.
fn compare_artifact_bytes_at(
    name: &str,
    py_bytes: Option<&[u8]>,
    rs_bytes: Option<&[u8]>,
    expected_path: &Path,
    rust_path: &Path,
) -> ArtifactComparison {
    let py_len = py_bytes.map(<[u8]>::len).unwrap_or(0);
    let rs_len = rs_bytes.map(<[u8]>::len).unwrap_or(0);
    let (equal, diff_summary, diff_detail) = match (py_bytes, rs_bytes) {
        (Some(py), Some(rs)) if py == rs => (true, None, None),
        (Some(py), Some(rs)) => {
            let first_diff = py
                .iter()
                .zip(rs.iter())
                .position(|(a, b)| a != b)
                .unwrap_or_else(|| py.len().min(rs.len()));
            let end = (first_diff + 32).min(py.len()).min(rs.len());
            let summary = format!(
                "first divergence at byte offset {first_diff} (python_len={py_len}, rust_len={rs_len}): python={:?} rust={:?}",
                py.get(first_diff..end).unwrap_or(&[]),
                rs.get(first_diff..end).unwrap_or(&[]),
            );
            let detail = field_level_diff(name, py, rs);
            let detail_opt = if matches!(detail, StructuredDiff::ByteEqual) {
                None
            } else {
                Some(detail)
            };
            (false, Some(summary), detail_opt)
        }
        (None, Some(_)) => (
            false,
            Some(format!("python artifact missing at {expected_path:?}")),
            Some(StructuredDiff::MissingExpected {
                rust_path: rust_path.to_string_lossy().to_string(),
            }),
        ),
        (Some(_), None) => (
            false,
            Some(format!("rust artifact missing at {rust_path:?}")),
            Some(StructuredDiff::MissingRust {
                expected_path: expected_path.to_string_lossy().to_string(),
            }),
        ),
        (None, None) => (
            false,
            Some(format!(
                "both artifacts unavailable (python: {expected_path:?}, rust: {rust_path:?})"
            )),
            Some(StructuredDiff::BothMissing {
                expected_path: expected_path.to_string_lossy().to_string(),
                rust_path: rust_path.to_string_lossy().to_string(),
            }),
        ),
    };
    ArtifactComparison {
        name: name.to_string(),
        expected_path: expected_path.to_path_buf(),
        rust_path: rust_path.to_path_buf(),
        expected_exists: py_bytes.is_some(),
        rust_exists: rs_bytes.is_some(),
        expected_bytes: py_len,
        rust_bytes: rs_len,
        equal,
        diff_summary,
        diff_detail,
    }
}

/// True iff `<corpus_dir>/scope.json` exists, parses as JSON, and
/// carries BOTH `schema_version == "cacg.v0"` AND `scope == "hot-path"`
/// as documented at `docs/diagnostic-parity.md` §3a.3.
///
/// Returns `false` on missing file, unreadable file, malformed JSON,
/// missing or wrong-shape `schema_version`, missing or wrong-shape
/// `scope`, or any value other than the exact (`"cacg.v0"`,
/// `"hot-path"`) pair. The boolean shape (vs an `Option<String>`) makes
/// the call site impossible to misuse by accepting future scope
/// literals that the harness does not understand. Round 8 tightens the
/// Round 7 reader per Codex's Round 7 review.
fn is_hot_path_scope_annotation(corpus_dir: &Path) -> bool {
    let path = corpus_dir.join("scope.json");
    let Ok(raw) = fs::read_to_string(&path) else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return false;
    };
    let Some(schema_version) = value.get("schema_version").and_then(|v| v.as_str()) else {
        return false;
    };
    if schema_version != "cacg.v0" {
        return false;
    }
    let Some(scope) = value.get("scope").and_then(|v| v.as_str()) else {
        return false;
    };
    scope == "hot-path"
}

/// Deferred CACG code prefixes per AC-2.1 (`docs/diagnostic-parity.md`
/// §3a.2). A line that mentions exclusively these prefixes counts as
/// "auxiliary-only" for the demotion rule. The prefixes are matched as
/// substrings: any `CACG-SUM-`, `CACG-SKILL-`, `CACG-DEP-`, or
/// `CACG-ROLE-` substring on a line classifies that line as auxiliary;
/// any other `CACG-` substring classifies it as trust-bearing.
const DEFERRED_CACG_PREFIXES: &[&str] = &["CACG-SUM-", "CACG-SKILL-", "CACG-DEP-", "CACG-ROLE-"];

/// True iff every divergent byte-record between `py_bytes` and
/// `rs_bytes` decodes to text that mentions exclusively the deferred
/// CACG prefixes.
///
/// "Byte-record" means a record produced by splitting on `\n` while
/// PRESERVING the terminator (mirroring `split_inclusive(b'\n')`).
/// A trailing unterminated segment is its own record distinct from a
/// terminated one. This makes the classifier byte-lossless: a record
/// ending in `\n` is not equal to the same content without `\n`, and a
/// `\r\n` record is not equal to the same content with `\n` only.
/// Round 7 used a `BTreeSet` symmetric-difference (lost multiplicity).
/// Round 8 used `str::lines()` (lost terminators). Round 9 closes both
/// defects per Codex's Round 8 review (see `docs/diagnostic-parity.md`
/// §3a.4 contract).
///
/// A record is "divergent" iff its occurrence count differs between
/// the two artifacts. The aux-only verdict requires at least one
/// divergent record AND every divergent record's UTF-8 decoding to
/// pass [`line_only_mentions_deferred_cacg_codes`]. Invalid UTF-8 on
/// a divergent record fails closed (the record is treated as
/// trust-bearing because the classifier cannot confirm the
/// deferred-prefix substring presence).
///
/// The fast path `py_bytes == rs_bytes` returns `true` (no divergent
/// records).
fn diff_mentions_only_deferred_codes(py_bytes: &[u8], rs_bytes: &[u8]) -> bool {
    if py_bytes == rs_bytes {
        return true;
    }
    let py_records = split_inclusive_newline(py_bytes);
    let rs_records = split_inclusive_newline(rs_bytes);
    let mut counts: std::collections::BTreeMap<Vec<u8>, (usize, usize)> =
        std::collections::BTreeMap::new();
    for record in &py_records {
        counts.entry(record.clone()).or_insert((0, 0)).0 += 1;
    }
    for record in &rs_records {
        counts.entry(record.clone()).or_insert((0, 0)).1 += 1;
    }
    let mut any_divergent = false;
    for (record, (py_count, rs_count)) in &counts {
        if py_count == rs_count {
            continue;
        }
        any_divergent = true;
        // Decode this record only (NOT the whole artifact) for CACG
        // prefix scanning. Invalid UTF-8 fails closed because the
        // classifier cannot prove the record mentions only deferred
        // prefixes.
        let Ok(text) = std::str::from_utf8(record) else {
            return false;
        };
        if !line_only_mentions_deferred_cacg_codes(text) {
            return false;
        }
    }
    any_divergent
}

/// Split `bytes` on `\n` while preserving the terminator on each
/// record. A trailing unterminated segment (bytes after the final
/// `\n`, or all bytes if no `\n` is present) becomes its own record.
/// Empty input returns an empty `Vec`.
fn split_inclusive_newline(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    let mut start = 0;
    for (i, b) in bytes.iter().enumerate() {
        if *b == b'\n' {
            out.push(bytes[start..=i].to_vec());
            start = i + 1;
        }
    }
    if start < bytes.len() {
        out.push(bytes[start..].to_vec());
    }
    out
}

/// True iff `line` mentions at least one deferred CACG-* code AND no
/// trust-bearing CACG-* code. A line with no `CACG-` substring at all is
/// trust-bearing-by-default (we cannot demote a line we can't classify),
/// so returns false in that case.
fn line_only_mentions_deferred_cacg_codes(line: &str) -> bool {
    let mut found_aux = false;
    for (idx, _) in line.match_indices("CACG-") {
        let rest = &line[idx..];
        let is_deferred = DEFERRED_CACG_PREFIXES
            .iter()
            .any(|prefix| rest.starts_with(prefix));
        if is_deferred {
            found_aux = true;
        } else {
            // Non-deferred CACG-* mention → trust-bearing, can't demote.
            return false;
        }
    }
    found_aux
}

fn lint_artifact_expected_path(c: &ArtifactComparison, py_out: &Path) -> std::path::PathBuf {
    py_out.join(&c.name)
}

fn lint_artifact_rust_path(c: &ArtifactComparison, rs_out: &Path) -> std::path::PathBuf {
    rs_out.join(&c.name)
}

fn diff_mentions_only_deferred_codes_at_paths(py_path: &Path, rs_path: &Path) -> bool {
    let py_bytes = fs::read(py_path).unwrap_or_default();
    let rs_bytes = fs::read(rs_path).unwrap_or_default();
    diff_mentions_only_deferred_codes(&py_bytes, &rs_bytes)
}

/// Copy every regular file from `src` to `dst`. Skips pre-existing
/// committed `.history.jsonl` sidecars so both sides start clean.
fn copy_corpus(src: &Path, dst: &Path) -> Result<(), ParityError> {
    fs::create_dir_all(dst).map_err(ParityError::Io)?;
    for entry in fs::read_dir(src).map_err(ParityError::Io)? {
        let entry = entry.map_err(ParityError::Io)?;
        let ft = entry.file_type().map_err(ParityError::Io)?;
        if ft.is_file() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".history.jsonl") {
                continue;
            }
            fs::copy(entry.path(), dst.join(&name)).map_err(ParityError::Io)?;
        }
    }
    Ok(())
}

fn build_summary(entries: &[EntryReport]) -> Summary {
    let total = entries.len();
    let mut passed = 0;
    let mut failed = 0;
    let mut future_stage = 0;
    for e in entries {
        match e.status {
            EntryStatus::Pass => passed += 1,
            EntryStatus::Fail(_) => failed += 1,
            EntryStatus::FutureStage(_) => future_stage += 1,
        }
    }
    Summary {
        total,
        passed,
        failed,
        future_stage,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // Serialize tests that spawn subprocess mock scripts to avoid flakes
    // from file-descriptor / process-table contention under parallel execution.
    static SUBPROCESS_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn write(dir: &Path, name: &str, body: &[u8]) -> PathBuf {
        let p = dir.join(name);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, body).unwrap();
        p
    }

    #[test]
    fn lint_binaries_production_routes_through_legacy_oracle_venv() {
        // The production parity binaries must invoke the legacy oracle
        // venv interpreter, never plain `python` or system `python3`.
        // A bare `python` resolves to the system interpreter, which
        // does not have `cacg.cli` installed post-quarantine and would
        // surface as `ModuleNotFoundError: No module named 'cacg'` for
        // every live parity row.
        let ws = PathBuf::from("/workspace");
        let binaries = LintBinaries::production(&ws);
        assert_eq!(
            binaries.python_executable,
            PathBuf::from("/workspace/legacy_python_oracle/.venv/bin/python"),
            "production parity python executable must be the legacy oracle venv interpreter",
        );
    }

    #[test]
    fn compare_artifact_byte_equal_returns_equal() {
        let dir = TempDir::new().unwrap();
        let py = write(dir.path(), "a", b"hello");
        let rs = write(dir.path(), "b", b"hello");
        let c = compare_artifact("test", &py, &rs);
        assert!(c.equal);
        assert!(c.diff_summary.is_none());
        assert_eq!(c.expected_bytes, 5);
        assert_eq!(c.rust_bytes, 5);
    }

    #[test]
    fn compare_artifact_byte_diff_same_size_returns_diff() {
        let dir = TempDir::new().unwrap();
        let py = write(dir.path(), "a", b"hello");
        let rs = write(dir.path(), "b", b"hellx");
        let c = compare_artifact("test", &py, &rs);
        assert!(!c.equal);
        let summary = c.diff_summary.unwrap();
        assert!(summary.contains("first divergence at byte offset 4"));
    }

    #[test]
    fn compare_artifact_missing_rust_returns_diff() {
        let dir = TempDir::new().unwrap();
        let py = write(dir.path(), "a", b"hello");
        let rs = dir.path().join("nonexistent");
        let c = compare_artifact("test", &py, &rs);
        assert!(!c.equal);
        assert!(c.expected_exists);
        assert!(!c.rust_exists);
        assert!(c.diff_summary.unwrap().contains("rust artifact missing"));
    }

    #[test]
    fn compare_artifact_missing_python_returns_diff() {
        let dir = TempDir::new().unwrap();
        let py = dir.path().join("nonexistent");
        let rs = write(dir.path(), "b", b"hello");
        let c = compare_artifact("test", &py, &rs);
        assert!(!c.equal);
        assert!(!c.expected_exists);
        assert!(c.rust_exists);
        assert!(c.diff_summary.unwrap().contains("python artifact missing"));
    }

    #[test]
    fn compare_artifact_both_missing_returns_diff() {
        let dir = TempDir::new().unwrap();
        let py = dir.path().join("a");
        let rs = dir.path().join("b");
        let c = compare_artifact("test", &py, &rs);
        assert!(!c.equal);
        assert!(c.diff_summary.unwrap().contains("both artifacts missing"));
    }

    #[test]
    fn compare_artifact_different_length_returns_diff() {
        let dir = TempDir::new().unwrap();
        let py = write(dir.path(), "a", b"hello world");
        let rs = write(dir.path(), "b", b"hello");
        let c = compare_artifact("test", &py, &rs);
        assert!(!c.equal);
        assert!(c.expected_bytes != c.rust_bytes);
        assert!(c
            .diff_summary
            .unwrap()
            .contains("python_len=11, rust_len=5"));
    }

    #[test]
    fn build_summary_counts_each_status() {
        let entries = vec![
            EntryReport {
                name: "a".to_string(),
                stage: Stage::M2,
                expected_command: String::new(),
                rust_command: String::new(),
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: Vec::new(),
                status: EntryStatus::Pass,
            },
            EntryReport {
                name: "b".to_string(),
                stage: Stage::M2,
                expected_command: String::new(),
                rust_command: String::new(),
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: Vec::new(),
                status: EntryStatus::Fail("diff".to_string()),
            },
            EntryReport {
                name: "c".to_string(),
                stage: Stage::Future("M3"),
                expected_command: String::new(),
                rust_command: String::new(),
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: Vec::new(),
                status: EntryStatus::FutureStage("M3"),
            },
        ];
        let s = build_summary(&entries);
        assert_eq!(s.total, 3);
        assert_eq!(s.passed, 1);
        assert_eq!(s.failed, 1);
        assert_eq!(s.future_stage, 1);
    }

    #[test]
    fn is_audited_artifact_name_filters_atomic_publish_and_sqlite_sidecars() {
        assert!(is_audited_artifact_name("cards_manifest.json"));
        assert!(is_audited_artifact_name("summaries.json"));
        assert!(is_audited_artifact_name("INDEX.md"));
        assert!(!is_audited_artifact_name("cards_manifest.json.lock"));
        assert!(!is_audited_artifact_name("cards_manifest.json.tmp"));
        assert!(!is_audited_artifact_name("cards_manifest.json.bak"));
        assert!(!is_audited_artifact_name("summaries.sqlite"));
        assert!(!is_audited_artifact_name(".kb_index_cache.json"));
    }

    #[cfg(unix)]
    fn write_executable_script(dir: &Path, name: &str, body: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        let path = dir.join(name);
        fs::write(&path, body).unwrap();
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).unwrap();
        path
    }

    #[cfg(unix)]
    fn populated_reading_dir(parent: &Path) -> PathBuf {
        let reading = parent.join("reading_src");
        fs::create_dir_all(&reading).unwrap();
        fs::write(reading.join("card_a.md"), b"---\ntitle: A\n---\nbody\n").unwrap();
        reading
    }

    #[cfg(unix)]
    #[test]
    fn run_matrix_entry_fails_when_expected_command_exits_nonzero() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(tmp.path(), "py_fail.sh", "#!/bin/sh\nexit 1\n");
        let rs_mock = write_executable_script(tmp.path(), "rs_pass.sh", "#!/bin/sh\nexit 0\n");
        let reading = populated_reading_dir(tmp.path());
        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();

        let python_cmd = IndexCommand {
            executable: py_mock,
            leading_args: vec![],
        };
        let rust_cmd = IndexCommand {
            executable: rs_mock,
            leading_args: vec![],
        };

        let report = run_kb_index_entry(
            &reading,
            "cards/reading_01",
            &out_root,
            tmp.path(),
            &python_cmd,
            &rust_cmd,
            None,
        )
        .expect("orchestrator should not propagate subprocess failure");
        match &report.status {
            EntryStatus::Fail(msg) => assert!(
                msg.contains("expected-side invocation failed"),
                "expected invocation-failure status, got: {msg}"
            ),
            other => panic!("expected Fail status, got {other:?}"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn run_matrix_entry_fails_when_rust_command_exits_nonzero() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(tmp.path(), "py_pass.sh", "#!/bin/sh\nexit 0\n");
        let rs_mock = write_executable_script(tmp.path(), "rs_fail.sh", "#!/bin/sh\nexit 1\n");
        let reading = populated_reading_dir(tmp.path());
        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();

        let python_cmd = IndexCommand {
            executable: py_mock,
            leading_args: vec![],
        };
        let rust_cmd = IndexCommand {
            executable: rs_mock,
            leading_args: vec![],
        };

        let report = run_kb_index_entry(
            &reading,
            "cards/reading_01",
            &out_root,
            tmp.path(),
            &python_cmd,
            &rust_cmd,
            None,
        )
        .expect("orchestrator should not propagate subprocess failure");
        match &report.status {
            EntryStatus::Fail(msg) => assert!(
                msg.contains("rust invocation failed"),
                "expected rust-failure status, got: {msg}"
            ),
            other => panic!("expected Fail status, got {other:?}"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn run_matrix_entry_cleans_stale_output_before_running() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(tmp.path(), "py.sh", "#!/bin/sh\nexit 0\n");
        let rs_mock = write_executable_script(tmp.path(), "rs.sh", "#!/bin/sh\nexit 0\n");
        let reading = populated_reading_dir(tmp.path());
        let out_root = tmp.path().join("out");
        // Pre-create stale files in every dir the entry is supposed
        // to scrub. After the run, none of them must remain.
        let stale_py_out = out_root.join("py-out/STALE.txt");
        let stale_rs_out = out_root.join("rs-out/STALE.txt");
        let stale_py_corpus = out_root.join("py-corpus/cards/reading_01/STALE.txt");
        let stale_rs_corpus = out_root.join("rs-corpus/cards/reading_01/STALE.txt");
        for p in [
            &stale_py_out,
            &stale_rs_out,
            &stale_py_corpus,
            &stale_rs_corpus,
        ] {
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, b"stale").unwrap();
        }

        let python_cmd = IndexCommand {
            executable: py_mock,
            leading_args: vec![],
        };
        let rust_cmd = IndexCommand {
            executable: rs_mock,
            leading_args: vec![],
        };

        let _report = run_kb_index_entry(
            &reading,
            "cards/reading_01",
            &out_root,
            tmp.path(),
            &python_cmd,
            &rust_cmd,
            None,
        )
        .expect("entry should succeed under the stale-cleanup path");
        assert!(!stale_py_out.exists(), "py-out STALE.txt should be removed");
        assert!(!stale_rs_out.exists(), "rs-out STALE.txt should be removed");
        assert!(
            !stale_py_corpus.exists(),
            "py-corpus STALE.txt should be removed before recopy"
        );
        assert!(
            !stale_rs_corpus.exists(),
            "rs-corpus STALE.txt should be removed before recopy"
        );
    }

    #[cfg(unix)]
    #[test]
    fn run_matrix_entry_enumerates_union_of_both_sides() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = TempDir::new().unwrap();
        // Python mock writes py_only.txt into --out; Rust mock writes
        // rs_only.txt. Each artifact only exists on one side; union
        // enumeration must surface both as failing comparisons so
        // single-sided drift cannot silently slip through the gate.
        let py_mock = write_executable_script(
            tmp.path(),
            "py.sh",
            "#!/bin/sh\nout=\"\"\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --out) out=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nmkdir -p \"$out\"\nprintf 'python\\n' > \"$out/py_only.txt\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs.sh",
            "#!/bin/sh\nout=\"\"\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --out) out=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nmkdir -p \"$out\"\nprintf 'rust\\n' > \"$out/rs_only.txt\"\nexit 0\n",
        );
        let reading = populated_reading_dir(tmp.path());
        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();

        let python_cmd = IndexCommand {
            executable: py_mock,
            leading_args: vec![],
        };
        let rust_cmd = IndexCommand {
            executable: rs_mock,
            leading_args: vec![],
        };

        let report = run_kb_index_entry(
            &reading,
            "cards/reading_01",
            &out_root,
            tmp.path(),
            &python_cmd,
            &rust_cmd,
            None,
        )
        .expect("entry should succeed");
        let names: Vec<&str> = report.comparisons.iter().map(|c| c.name.as_str()).collect();
        assert!(
            names.contains(&"py_only.txt"),
            "py_only.txt must appear via union enumeration; comparisons = {names:?}"
        );
        assert!(
            names.contains(&"rs_only.txt"),
            "rs_only.txt must appear via union enumeration; comparisons = {names:?}"
        );
        let py_only = report
            .comparisons
            .iter()
            .find(|c| c.name == "py_only.txt")
            .unwrap();
        assert!(py_only.expected_exists);
        assert!(!py_only.rust_exists);
        assert!(!py_only.equal, "py-only artifact must report as diff");
        let rs_only = report
            .comparisons
            .iter()
            .find(|c| c.name == "rs_only.txt")
            .unwrap();
        assert!(!rs_only.expected_exists);
        assert!(rs_only.rust_exists);
        assert!(!rs_only.equal, "rs-only artifact must report as diff");
        match &report.status {
            EntryStatus::Fail(msg) => assert!(
                msg.contains("diffed"),
                "expected diff failure status, got: {msg}"
            ),
            other => panic!("expected Fail status, got {other:?}"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn run_matrix_entry_fails_when_no_artifacts_emitted() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Both mocks succeed but neither writes anything. The harness
        // must refuse to report this as Pass — a vacuous-true Pass
        // would silently regress the gate the day an upstream change
        // breaks the cards-dir argv or the --out plumbing.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(tmp.path(), "py.sh", "#!/bin/sh\nexit 0\n");
        let rs_mock = write_executable_script(tmp.path(), "rs.sh", "#!/bin/sh\nexit 0\n");
        let reading = populated_reading_dir(tmp.path());
        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();

        let python_cmd = IndexCommand {
            executable: py_mock,
            leading_args: vec![],
        };
        let rust_cmd = IndexCommand {
            executable: rs_mock,
            leading_args: vec![],
        };

        let report = run_kb_index_entry(
            &reading,
            "cards/reading_01",
            &out_root,
            tmp.path(),
            &python_cmd,
            &rust_cmd,
            None,
        )
        .expect("entry should run");
        match &report.status {
            EntryStatus::Fail(msg) => assert!(
                msg.contains("vacuous Pass"),
                "expected vacuous-pass failure, got: {msg}"
            ),
            other => panic!("expected Fail status, got {other:?}"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn run_matrix_entry_surfaces_python_only_history_sidecar_as_diff() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Python mock writes manifest AND a `.history.jsonl` next to
        // the card (simulating "frontmatter hash mismatched, append
        // history"); Rust mock writes manifest only. Under isolated
        // corpora, the harness must surface the sidecar as a missing-
        // on-rust diff. Pre-R39 the harness had no corpus-side sidecar
        // enumeration and silently false-greened this scenario.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(
            tmp.path(),
            "py.sh",
            "#!/bin/sh\ncards=\"$1\"\nout=\"$3\"\nmkdir -p \"$out\"\nprintf 'manifest\\n' > \"$out/cards_manifest.json\"\nprintf 'history\\n' > \"$cards/card_a.history.jsonl\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs.sh",
            "#!/bin/sh\nout=\"$3\"\nmkdir -p \"$out\"\nprintf 'manifest\\n' > \"$out/cards_manifest.json\"\nexit 0\n",
        );
        let reading = populated_reading_dir(tmp.path());
        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();

        let python_cmd = IndexCommand {
            executable: py_mock,
            leading_args: vec![],
        };
        let rust_cmd = IndexCommand {
            executable: rs_mock,
            leading_args: vec![],
        };

        let report = run_kb_index_entry(
            &reading,
            "cards/reading_01",
            &out_root,
            tmp.path(),
            &python_cmd,
            &rust_cmd,
            None,
        )
        .expect("entry should run");
        let sidecar = report
            .comparisons
            .iter()
            .find(|c| c.name == "card_a.history.jsonl")
            .expect("history sidecar must appear in comparisons");
        assert!(
            sidecar.expected_exists,
            "python wrote the history sidecar; comparison should record expected_exists=true"
        );
        assert!(
            !sidecar.rust_exists,
            "rust did not write the sidecar; comparison should record rust_exists=false"
        );
        assert!(!sidecar.equal);
        match &report.status {
            EntryStatus::Fail(msg) => assert!(
                msg.contains("diffed"),
                "expected diff failure status, got: {msg}"
            ),
            other => panic!("expected Fail status, got {other:?}"),
        }
    }

    #[test]
    fn compare_artifact_populates_diff_detail_for_json_diff() {
        let dir = TempDir::new().unwrap();
        let py = write(
            dir.path(),
            "lint.json",
            br#"{"diagnostics":[{"message":"ABCDEFGH"}]}"#,
        );
        let rs = write(
            &dir.path().join("rs"),
            "lint.json",
            br#"{"diagnostics":[{"message":"12345678"}]}"#,
        );
        let c = compare_artifact("lint.json", &py, &rs);
        assert!(!c.equal);
        // Back-compat: diff_summary string still populated as before.
        let summary = c.diff_summary.as_ref().unwrap();
        assert!(summary.contains("first divergence at byte offset"));
        // New: diff_detail carries the structured payload.
        match c.diff_detail.as_ref().unwrap() {
            StructuredDiff::JsonField { path, .. } => {
                assert_eq!(path, "diagnostics[0].message");
            }
            other => panic!("expected JsonField, got {other:?}"),
        }
    }

    #[test]
    fn compare_artifact_populates_missing_python_variant_on_absent_python() {
        let dir = TempDir::new().unwrap();
        let py = dir.path().join("nonexistent");
        let rs = write(dir.path(), "b", b"hello");
        let c = compare_artifact("test", &py, &rs);
        assert!(!c.equal);
        match c.diff_detail.as_ref().unwrap() {
            StructuredDiff::MissingExpected { rust_path } => {
                assert!(rust_path.ends_with("b"));
            }
            other => panic!("expected MissingExpected, got {other:?}"),
        }
    }

    #[test]
    fn compare_artifact_populates_missing_rust_variant_on_absent_rust() {
        let dir = TempDir::new().unwrap();
        let py = write(dir.path(), "a", b"hello");
        let rs = dir.path().join("nonexistent");
        let c = compare_artifact("test", &py, &rs);
        match c.diff_detail.as_ref().unwrap() {
            StructuredDiff::MissingRust { expected_path } => {
                assert!(expected_path.ends_with("a"));
            }
            other => panic!("expected MissingRust, got {other:?}"),
        }
    }

    #[test]
    fn compare_artifact_populates_both_missing_variant_when_both_absent() {
        let dir = TempDir::new().unwrap();
        let py = dir.path().join("a");
        let rs = dir.path().join("b");
        let c = compare_artifact("test", &py, &rs);
        match c.diff_detail.as_ref().unwrap() {
            StructuredDiff::BothMissing { .. } => {}
            other => panic!("expected BothMissing, got {other:?}"),
        }
    }

    #[test]
    fn compare_artifact_byte_equal_has_no_diff_detail() {
        let dir = TempDir::new().unwrap();
        let py = write(dir.path(), "a", b"hello");
        let rs = write(&dir.path().join("rs"), "b", b"hello");
        let c = compare_artifact("test", &py, &rs);
        assert!(c.equal);
        assert!(c.diff_summary.is_none());
        assert!(c.diff_detail.is_none());
    }

    #[test]
    fn artifact_comparison_schema_preserves_diff_summary_alongside_diff_detail() {
        // Schema back-compat: the perf-report JSON for a non-equal
        // comparison MUST contain both the legacy `diff_summary` key
        // AND the new `diff_detail` key. A snapshot-style test that
        // drops `diff_summary` would silently break downstream tools
        // that read the byte-offset string; this assertion guards the
        // schema contract until those tools are migrated.
        let cmp = ArtifactComparison {
            name: "lint.json".to_string(),
            expected_path: PathBuf::from("p"),
            rust_path: PathBuf::from("r"),
            expected_exists: true,
            rust_exists: true,
            expected_bytes: 10,
            rust_bytes: 10,
            equal: false,
            diff_summary: Some("first divergence at byte offset 0".to_string()),
            diff_detail: Some(StructuredDiff::Binary {
                artifact_path: "lint.json".to_string(),
                byte_offset: 0,
                py_len: 10,
                rs_len: 10,
            }),
        };
        let json = serde_json::to_string(&cmp).unwrap();
        assert!(json.contains("\"diff_summary\":\"first divergence at byte offset 0\""));
        assert!(json.contains("\"diff_detail\":"));
        assert!(json.contains("\"kind\":\"binary\""));
        assert!(json.contains("\"artifact_path\":\"lint.json\""));
    }

    #[test]
    fn artifact_comparison_schema_omits_diff_detail_when_none() {
        // Byte-equal comparisons should not emit a `diff_detail` key
        // at all (the field is `skip_serializing_if = "Option::is_none"`).
        // Keeps the perf-report JSON noise-free on the dominant Pass path.
        let cmp = ArtifactComparison {
            name: "cards_manifest.json".to_string(),
            expected_path: PathBuf::from("p"),
            rust_path: PathBuf::from("r"),
            expected_exists: true,
            rust_exists: true,
            expected_bytes: 10,
            rust_bytes: 10,
            equal: true,
            diff_summary: None,
            diff_detail: None,
        };
        let json = serde_json::to_string(&cmp).unwrap();
        assert!(!json.contains("diff_detail"));
    }

    #[cfg(unix)]
    #[test]
    fn run_kb_index_entry_threads_non_default_corpus_subdir_through_to_argv_and_isolated_corpus() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // The kb-index runner must use the row's declared `corpus_subdir`
        // for: (a) the isolated destination under py-corpus/rs-corpus,
        // (b) the cards-dir argv passed to both subprocesses, and (c)
        // the sidecar enumeration directory. A non-default subdir
        // (`cards_stale_hash/reading_01`) is the next sibling row's
        // shape, so this test guards against silent regression to a
        // hardcoded `cards/reading_01` lookup.
        let tmp = TempDir::new().unwrap();
        // Write a mock subprocess that records its first positional argv
        // to a side file and writes a sidecar under that exact relative
        // path. The harness should then enumerate the sidecar under the
        // same subdir and report a successful comparison with the
        // mirror side.
        let py_mock = write_executable_script(
            tmp.path(),
            "py.sh",
            "#!/bin/sh\ncards=\"$1\"\nout=\"\"\nshift\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --out) out=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nmkdir -p \"$out\"\nprintf 'manifest\\n' > \"$out/cards_manifest.json\"\nprintf '%s\\n' \"$cards\" > \"$out/_argv0.txt\"\nprintf 'history\\n' > \"$cards/card_a.history.jsonl\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs.sh",
            "#!/bin/sh\ncards=\"$1\"\nout=\"\"\nshift\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --out) out=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nmkdir -p \"$out\"\nprintf 'manifest\\n' > \"$out/cards_manifest.json\"\nprintf '%s\\n' \"$cards\" > \"$out/_argv0.txt\"\nprintf 'history\\n' > \"$cards/card_a.history.jsonl\"\nexit 0\n",
        );
        let reading = tmp.path().join("reading_src");
        fs::create_dir_all(&reading).unwrap();
        fs::write(reading.join("card_a.md"), b"---\ntitle: A\n---\nbody\n").unwrap();
        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();

        let python_cmd = IndexCommand {
            executable: py_mock,
            leading_args: vec![],
        };
        let rust_cmd = IndexCommand {
            executable: rs_mock,
            leading_args: vec![],
        };

        let non_default_subdir = "cards_stale_hash/reading_01";
        let report = run_kb_index_entry(
            &reading,
            non_default_subdir,
            &out_root,
            tmp.path(),
            &python_cmd,
            &rust_cmd,
            None,
        )
        .expect("entry should succeed with non-default subdir");

        // The mocks recorded their first positional argv to _argv0.txt
        // under --out. Read both sides and assert they got the non-
        // default subdir, NOT `cards/reading_01`.
        let py_argv = fs::read_to_string(out_root.join("py-out/_argv0.txt")).unwrap();
        let rs_argv = fs::read_to_string(out_root.join("rs-out/_argv0.txt")).unwrap();
        assert_eq!(py_argv.trim(), non_default_subdir);
        assert_eq!(rs_argv.trim(), non_default_subdir);

        // The isolated corpora must materialize under the non-default
        // subdir, not `cards/reading_01`. The hardcoded path must NOT
        // exist; the non-default path must contain the sidecar.
        assert!(
            out_root.join("py-corpus").join(non_default_subdir).is_dir(),
            "py-corpus subdir must materialize under the row's subdir"
        );
        assert!(
            out_root.join("rs-corpus").join(non_default_subdir).is_dir(),
            "rs-corpus subdir must materialize under the row's subdir"
        );
        assert!(
            !out_root.join("py-corpus/cards/reading_01").exists(),
            "py-corpus must NOT use the hardcoded path"
        );
        assert!(
            !out_root.join("rs-corpus/cards/reading_01").exists(),
            "rs-corpus must NOT use the hardcoded path"
        );

        // Sidecar enumeration walks the non-default subdir and surfaces
        // the mock-written `card_a.history.jsonl` as a comparison.
        let sidecar = report
            .comparisons
            .iter()
            .find(|c| c.name == "card_a.history.jsonl")
            .expect("sidecar under non-default subdir must appear in comparisons");
        assert!(sidecar.equal, "both mocks wrote identical sidecar bytes");
    }

    #[cfg(unix)]
    #[test]
    fn run_matrix_row_kbindex_uses_row_specific_subdirectory_under_out_root() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Each KbIndex row must isolate its artifacts under
        // `<out_root>/<row.name>/...` so two M2 rows can persist
        // their `py-corpus` / `rs-corpus` / `py-out` / `rs-out`
        // evidence without the second row's setup deleting the
        // first row's artifacts. Codex R13 review finding 2: with
        // both M2 rows landing in R5, the shared-directory reuse
        // would make perf-report paths for an earlier-row diff
        // point at deleted/replaced files.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(
            tmp.path(),
            "py_row_isolation.sh",
            "#!/bin/sh\ncards=\"$1\"\nout=\"\"\nshift\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --out) out=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nmkdir -p \"$out\"\nprintf 'manifest\\n' > \"$out/cards_manifest.json\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_row_isolation.sh",
            "#!/bin/sh\ncards=\"$1\"\nout=\"\"\nshift\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --out) out=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nmkdir -p \"$out\"\nprintf 'manifest\\n' > \"$out/cards_manifest.json\"\nexit 0\n",
        );

        let corpus = tmp.path().join("corpus");
        let reading = corpus.join("reading_01");
        fs::create_dir_all(&reading).unwrap();
        fs::write(reading.join("card.md"), b"---\ntitle: A\n---\nbody\n").unwrap();
        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();

        let python_cmd = IndexCommand {
            executable: py_mock,
            leading_args: vec![],
        };
        let rust_cmd = IndexCommand {
            executable: rs_mock,
            leading_args: vec![],
        };

        let row_a = MatrixRow {
            name: "row_alpha",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIndex {
                corpus_subdir: "reading_01",
            },
        };
        let row_b = MatrixRow {
            name: "row_beta",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIndex {
                corpus_subdir: "reading_01",
            },
        };

        let _report_a = run_matrix_row(
            &row_a,
            &corpus,
            &out_root,
            tmp.path(),
            &python_cmd,
            &rust_cmd,
        )
        .expect("row_alpha must succeed");
        let _report_b = run_matrix_row(
            &row_b,
            &corpus,
            &out_root,
            tmp.path(),
            &python_cmd,
            &rust_cmd,
        )
        .expect("row_beta must succeed");

        // BOTH rows' artifacts must persist under their own row-named
        // subdirectory. A regression that reuses `<out_root>/py-out`
        // across rows would leave only `out_root/py-out/...` and
        // delete row_alpha's evidence when row_beta starts.
        assert!(
            out_root
                .join("row_alpha/py-out/cards_manifest.json")
                .is_file(),
            "row_alpha's py-out manifest must persist under its row subdir"
        );
        assert!(
            out_root
                .join("row_alpha/rs-out/cards_manifest.json")
                .is_file(),
            "row_alpha's rs-out manifest must persist under its row subdir"
        );
        assert!(
            out_root
                .join("row_beta/py-out/cards_manifest.json")
                .is_file(),
            "row_beta's py-out manifest must persist under its row subdir"
        );
        assert!(
            out_root
                .join("row_beta/rs-out/cards_manifest.json")
                .is_file(),
            "row_beta's rs-out manifest must persist under its row subdir"
        );
        // The legacy shared paths must NOT contain artifacts.
        assert!(
            !out_root.join("py-out/cards_manifest.json").exists(),
            "shared `<out_root>/py-out` path must not be used for row artifacts"
        );
        assert!(
            !out_root.join("rs-out/cards_manifest.json").exists(),
            "shared `<out_root>/rs-out` path must not be used for row artifacts"
        );
    }

    #[cfg(unix)]
    #[test]
    fn kb_lint_row_fails_on_stdout_only_divergence() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Codex Round 5 review found the new kb_lint parity rows only
        // compared stderr/exit/journal, not stdout. The AC-2 contract
        // requires all three (stdout + stderr + journal) to be byte-
        // equal. This regression test wires mock python/rust binaries
        // that emit IDENTICAL stderr/exit/journal but DIFFERENT stdout
        // and asserts the row fails — proves a stdout-only divergence
        // can't slip past the harness again.
        let tmp = TempDir::new().unwrap();
        // Mock python binary: writes a known stderr/journal/stdout
        // shape that the rust mock will diverge from on stdout only.
        // The script ignores its argv and uses argv[N] = --journal
        // <path> to locate the journal. We parse it with a tiny
        // shell loop.
        let py_mock = write_executable_script(
            tmp.path(),
            "py_lint_mock.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'EXPECTED-STDOUT\\n'\nprintf 'EXPECTED-STDERR\\n' >&2\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_lint_mock.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'WRONG-STDOUT\\n'\nprintf 'EXPECTED-STDERR\\n' >&2\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );

        let cards_dir = tmp.path().join("cards");
        fs::create_dir_all(&cards_dir).unwrap();
        fs::write(cards_dir.join("card_a.md"), b"---\nid: a\n---\nbody\n").unwrap();

        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();

        // Stub manifest + matrix files: the mocks ignore content but
        // the harness records absolute paths for the command string.
        fs::write(tmp.path().join("chunks.json"), b"{}").unwrap();
        fs::write(tmp.path().join("matrix.json"), b"{}").unwrap();

        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let report = run_kb_lint_entry_with(
            "kb_lint_stdout_divergence_test",
            Stage::M2,
            &cards_dir,
            &out_root,
            tmp.path(),
            "chunks.json",
            "matrix.json",
            &binaries,
            None,
        );

        // Comparisons MUST be 4 (stdout + stderr + exit + journal) per
        // card. With one card we expect exactly 4.
        assert_eq!(
            report.comparisons.len(),
            4,
            "expected 4 comparisons per card (stdout + stderr + exit + journal); got {report:#?}"
        );
        let has_stdout = report
            .comparisons
            .iter()
            .any(|c| c.name.ends_with(".stdout"));
        assert!(
            has_stdout,
            "comparisons must include a .stdout artifact; got {report:#?}"
        );

        // Row status MUST be Fail because the stdout differs.
        match &report.status {
            EntryStatus::Fail(reason) => {
                assert!(
                    reason.contains("artifacts diffed"),
                    "expected aggregated-diff failure reason; got {reason:?}"
                );
            }
            other => panic!("expected Fail on stdout-only divergence; got {other:?}"),
        }
        // And the diffing artifact is specifically the stdout one.
        let diffed: Vec<&str> = report
            .comparisons
            .iter()
            .filter(|c| !c.equal)
            .map(|c| c.name.as_str())
            .collect();
        assert_eq!(
            diffed,
            vec!["card_a.stdout"],
            "stdout was the only divergent artifact"
        );
    }

    #[cfg(unix)]
    #[test]
    fn kb_verify_row_fails_on_journal_only_divergence() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // The kb_verify_* rows compare stdout + stderr + exit +
        // verify journal per card (4N comparisons). This regression
        // wires mock python/rust binaries that emit IDENTICAL
        // stdout/stderr/exit but DIFFERENT verify-journal bytes and
        // asserts the row fails — proving a journal-only divergence
        // (the trust-bearing cardinality artifact) cannot slip past
        // the KbVerify harness body.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(
            tmp.path(),
            "py_verify_mock.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'SAME-STDOUT\\n'\nprintf 'SAME-STDERR\\n' >&2\nprintf '{\"command\":\"verify\",\"v\":1}\\n' > \"$journal\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_verify_mock.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'SAME-STDOUT\\n'\nprintf 'SAME-STDERR\\n' >&2\nprintf '{\"command\":\"verify\",\"v\":2}\\n' > \"$journal\"\nexit 0\n",
        );

        let cards_dir = tmp.path().join("cards");
        fs::create_dir_all(&cards_dir).unwrap();
        fs::write(cards_dir.join("card_a.md"), b"---\nid: a\n---\nbody\n").unwrap();

        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();
        fs::write(tmp.path().join("chunks.json"), b"{}").unwrap();
        fs::write(tmp.path().join("matrix.json"), b"{}").unwrap();

        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let report = run_kb_verify_entry_with(
            "kb_verify_journal_divergence_test",
            Stage::M2,
            &cards_dir,
            &out_root,
            tmp.path(),
            "chunks.json",
            "matrix.json",
            false,
            false,
            &binaries,
            None,
        );

        assert_eq!(
            report.comparisons.len(),
            4,
            "expected 4 comparisons per card (stdout + stderr + exit + journal); got {report:#?}"
        );
        match &report.status {
            EntryStatus::Fail(reason) => {
                assert!(
                    reason.contains("artifacts diffed"),
                    "expected aggregated-diff failure reason; got {reason:?}"
                );
            }
            other => panic!("expected Fail on journal-only divergence; got {other:?}"),
        }
        let diffed: Vec<&str> = report
            .comparisons
            .iter()
            .filter(|c| !c.equal)
            .map(|c| c.name.as_str())
            .collect();
        assert_eq!(
            diffed,
            vec!["card_a.lint_journal.jsonl"],
            "the verify journal was the only divergent artifact"
        );
    }

    #[cfg(unix)]
    #[test]
    fn kb_verify_row_passes_when_all_artifacts_match() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Sanity counterpart: identical mock binaries → the row
        // passes. Confirms the KbVerify body does not spuriously
        // fail when Python and Rust agree byte-for-byte.
        let tmp = TempDir::new().unwrap();
        let mock_body = "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'SAME-STDOUT\\n'\nprintf 'SAME-STDERR\\n' >&2\nprintf '{\"command\":\"verify\",\"v\":1}\\n' > \"$journal\"\nexit 0\n";
        let py_mock = write_executable_script(tmp.path(), "py_verify_ok.sh", mock_body);
        let rs_mock = write_executable_script(tmp.path(), "rs_verify_ok.sh", mock_body);

        let cards_dir = tmp.path().join("cards");
        fs::create_dir_all(&cards_dir).unwrap();
        fs::write(cards_dir.join("card_a.md"), b"---\nid: a\n---\nbody\n").unwrap();
        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();
        fs::write(tmp.path().join("chunks.json"), b"{}").unwrap();
        fs::write(tmp.path().join("matrix.json"), b"{}").unwrap();

        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let report = run_kb_verify_entry_with(
            "kb_verify_all_match_test",
            Stage::M2,
            &cards_dir,
            &out_root,
            tmp.path(),
            "chunks.json",
            "matrix.json",
            true, // fuzzy flag exercised
            true, // skip_lint flag exercised
            &binaries,
            None,
        );
        assert!(
            matches!(report.status, EntryStatus::Pass),
            "identical mock binaries must produce a Pass; got {report:#?}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn kb_search_entry_with_identical_mocks_produces_pass() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Both mock binaries emit identical stdout, empty stderr, and
        // exit 0 for every query case, so every per-case
        // stdout/stderr/exit comparison is byte-equal and the row
        // Passes. Exercises the multi-case loop and the `--json` /
        // `--top-k` arg threading.
        let tmp = TempDir::new().unwrap();
        let mock = "#!/bin/sh\nprintf '  0.0043  card-1  [r_01]  T  -> p\\n'\nexit 0\n";
        let py_mock = write_executable_script(tmp.path(), "py_search_match.sh", mock);
        let rs_mock = write_executable_script(tmp.path(), "rs_search_match.sh", mock);
        let corpus = tmp.path().join("kb_search");
        fs::create_dir_all(&corpus).unwrap();
        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let cases = [
            KbSearchCase {
                label: "case_a",
                query: "synthetic",
                json: false,
                top_k: None,
            },
            KbSearchCase {
                label: "case_b",
                query: "card",
                json: true,
                top_k: Some(2),
            },
        ];
        let report = run_kb_search_entry_with(
            "kb_search_match_test",
            Stage::M2,
            &corpus,
            &tmp.path().join("out"),
            tmp.path(),
            &cases,
            &binaries,
            None,
        );
        assert!(
            matches!(report.status, EntryStatus::Pass),
            "identical mock binaries must produce a Pass; got {report:#?}"
        );
        // 2 cases x (stdout + stderr + exit) = 6 comparisons.
        assert_eq!(report.comparisons.len(), 6);
    }

    #[cfg(unix)]
    #[test]
    fn kb_search_entry_with_divergent_mocks_produces_fail() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // The mocks emit different stdout, so the per-case stdout
        // comparison diverges and the row Fails.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(
            tmp.path(),
            "py_search_diff.sh",
            "#!/bin/sh\nprintf 'PYTHON-HIT\\n'\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_search_diff.sh",
            "#!/bin/sh\nprintf 'RUST-HIT\\n'\nexit 0\n",
        );
        let corpus = tmp.path().join("kb_search");
        fs::create_dir_all(&corpus).unwrap();
        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let cases = [KbSearchCase {
            label: "case_a",
            query: "synthetic",
            json: false,
            top_k: None,
        }];
        let report = run_kb_search_entry_with(
            "kb_search_diff_test",
            Stage::M2,
            &corpus,
            &tmp.path().join("out"),
            tmp.path(),
            &cases,
            &binaries,
            None,
        );
        match &report.status {
            EntryStatus::Fail(_) => {}
            other => panic!("divergent mock stdout must produce a Fail; got {other:?}"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn kb_search_entry_missing_corpus_subdir_fails() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // A KbSearch row whose corpus subdir does not exist must Fail
        // fast with an explicit reason and spawn nothing.
        let tmp = TempDir::new().unwrap();
        let py_mock =
            write_executable_script(tmp.path(), "py_search_unused.sh", "#!/bin/sh\nexit 0\n");
        let rs_mock =
            write_executable_script(tmp.path(), "rs_search_unused.sh", "#!/bin/sh\nexit 0\n");
        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let cases = [KbSearchCase {
            label: "case_a",
            query: "q",
            json: false,
            top_k: None,
        }];
        let report = run_kb_search_entry_with(
            "kb_search_missing_test",
            Stage::M2,
            &tmp.path().join("does-not-exist"),
            &tmp.path().join("out"),
            tmp.path(),
            &cases,
            &binaries,
            None,
        );
        match &report.status {
            EntryStatus::Fail(reason) => assert!(
                reason.contains("corpus subdir missing"),
                "missing-corpus reason must be explicit; got {reason:?}"
            ),
            other => panic!("missing corpus subdir must Fail; got {other:?}"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn kb_show_entry_with_identical_mocks_produces_pass() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Both mock binaries emit identical stdout, empty stderr, and
        // exit 0 for every `kb show` case, so every per-case
        // stdout/stderr/exit comparison is byte-equal and the row
        // Passes. Exercises the multi-case loop and the `--path` /
        // `--allow-retracted` arg threading.
        let tmp = TempDir::new().unwrap();
        let mock = "#!/bin/sh\nprintf '# Card\\n**summary**\\n'\nexit 0\n";
        let py_mock = write_executable_script(tmp.path(), "py_show_match.sh", mock);
        let rs_mock = write_executable_script(tmp.path(), "rs_show_match.sh", mock);
        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let cases = [
            KbShowCase {
                label: "case_a",
                card_id: "card-a",
                cards_manifest: "m.json",
                source_matrix: "s.json",
                path: None,
                allow_retracted: false,
            },
            KbShowCase {
                label: "case_b",
                card_id: "card-b",
                cards_manifest: "m.json",
                source_matrix: "s.json",
                path: Some("cards/card-b.md"),
                allow_retracted: true,
            },
        ];
        let report = run_kb_show_entry_with(
            "kb_show_match_test",
            Stage::M2,
            &tmp.path().join("out"),
            tmp.path(),
            &cases,
            &binaries,
            None,
        );
        assert!(
            matches!(report.status, EntryStatus::Pass),
            "identical mock binaries must produce a Pass; got {report:#?}"
        );
        // 2 cases x (stdout + stderr + exit) = 6 comparisons.
        assert_eq!(report.comparisons.len(), 6);
    }

    #[cfg(unix)]
    #[test]
    fn kb_show_entry_with_divergent_mocks_produces_fail() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // The mocks emit different stdout, so the per-case stdout
        // comparison diverges and the row Fails.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(
            tmp.path(),
            "py_show_diff.sh",
            "#!/bin/sh\nprintf 'PYTHON-CARD\\n'\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_show_diff.sh",
            "#!/bin/sh\nprintf 'RUST-CARD\\n'\nexit 0\n",
        );
        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let cases = [KbShowCase {
            label: "case_a",
            card_id: "card-a",
            cards_manifest: "m.json",
            source_matrix: "s.json",
            path: None,
            allow_retracted: false,
        }];
        let report = run_kb_show_entry_with(
            "kb_show_diff_test",
            Stage::M2,
            &tmp.path().join("out"),
            tmp.path(),
            &cases,
            &binaries,
            None,
        );
        match &report.status {
            EntryStatus::Fail(_) => {}
            other => panic!("divergent mock stdout must produce a Fail; got {other:?}"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn ac21_doc_carveout_does_not_broaden_to_m1_surfaces() {
        // The docs/diagnostic-parity.md §3a AC-2.1 carve-out MUST name
        // exactly the three Rust lint-pass surfaces and MUST NOT
        // broaden to the M1 schema/frontmatter surfaces. A regression
        // that adds `cacg-core::schema` or `frontmatter` (as deferred
        // surfaces) fails this test loudly so the human reviewer
        // catches it before merge.
        let doc_path = workspace_root_for_test().join("docs/diagnostic-parity.md");
        let doc = fs::read_to_string(&doc_path)
            .unwrap_or_else(|e| panic!("read {}: {e}", doc_path.display()));

        // 1. The AC-2.1 section header must exist.
        assert!(
            doc.contains("## 3a. AC-2.1: Lint-pass surface auxiliary-codes carve-out"),
            "docs/diagnostic-parity.md must include the AC-2.1 section header verbatim"
        );

        // 2. All three lint-pass surfaces must appear inside the section.
        for needle in &[
            "cacg-core::lint::layer1::run_layer1_checks",
            "cacg-core::lint::layer1::lint_card",
            "cacg-core::lint::layer1::lint_directory",
        ] {
            assert!(
                doc.contains(needle),
                "docs/diagnostic-parity.md §3a must name {needle:?} as a bounded surface"
            );
        }

        // 3. The four deferred prefixes must appear.
        for needle in &["CACG-SUM-*", "CACG-SKILL-*", "CACG-DEP-*", "CACG-ROLE-*"] {
            assert!(
                doc.contains(needle),
                "docs/diagnostic-parity.md §3a must list {needle:?} as deferred"
            );
        }

        // 4. The doc must EXPLICITLY say the M1 surfaces are not part
        //    of the carve-out. We don't grep for the negative of every
        //    possible broadening; we pin the explicit non-broadening
        //    paragraph that names schema.rs / frontmatter.rs /
        //    schema_parity.rs. A future edit that removes ALL THREE
        //    M1 surface mentions from §3a fails this test.
        let ac21_section = doc
            .split("## 3a. AC-2.1: Lint-pass surface auxiliary-codes carve-out")
            .nth(1)
            .expect("§3a section must exist (asserted above)")
            .split("## 4. How parity is enforced")
            .next()
            .expect("§4 header must follow §3a");
        for required in &[
            "crates/cacg-core/src/schema.rs",
            "crates/cacg-core/src/frontmatter.rs",
            "crates/cacg-core/tests/schema_parity.rs",
        ] {
            assert!(
                ac21_section.contains(required),
                "§3a must name {required:?} as a surface that is NOT in the carve-out (non-broadening pin)"
            );
        }

        // 5. The annotation literal must appear.
        assert!(
            ac21_section.contains("cacg.v0/scope:hot-path"),
            "§3a must define the cacg.v0/scope:hot-path annotation literal"
        );
        assert!(
            ac21_section.contains("scope.json"),
            "§3a must define the scope.json sidecar filename"
        );
    }

    #[cfg(unix)]
    #[test]
    fn ac21_hot_path_annotation_ignored_outside_lint_rows() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // A KbIndex row must IGNORE `scope.json` entirely. Even if a
        // hot-path-scoped corpus has a kb_index artifact diff, the row
        // must Fail (not FutureStage); the demotion code path is only
        // wired into `MatrixRowKind::KbLint`.
        let tmp = TempDir::new().unwrap();
        // Mock python emits one bytes; mock rust emits different bytes.
        // Both succeed (exit 0) but their cards_manifest content differs,
        // so compare_artifact reports a Fail. The annotation file is
        // present at the row's corpus root; KbIndex must not consult it.
        let py_mock = write_executable_script(
            tmp.path(),
            "py_index_annotation_test.sh",
            "#!/bin/sh\ncards=\"$1\"\nout=\"\"\nshift\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --out) out=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nmkdir -p \"$out\"\nprintf 'PYTHON-CARDS-MANIFEST\\n' > \"$out/cards_manifest.json\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_index_annotation_test.sh",
            "#!/bin/sh\ncards=\"$1\"\nout=\"\"\nshift\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --out) out=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nmkdir -p \"$out\"\nprintf 'RUST-CARDS-MANIFEST\\n' > \"$out/cards_manifest.json\"\nexit 0\n",
        );

        let corpus = tmp.path().join("corpus");
        let reading = corpus.join("reading_01");
        fs::create_dir_all(&reading).unwrap();
        fs::write(reading.join("card.md"), b"---\ntitle: A\n---\nbody\n").unwrap();

        // Drop the hot-path scope sidecar into BOTH the corpus root
        // and the reading subdir to be thorough — KbIndex must ignore
        // it from any location.
        let scope_json = r#"{"schema_version": "cacg.v0", "scope": "hot-path"}"#;
        fs::write(corpus.join("scope.json"), scope_json).unwrap();
        fs::write(reading.join("scope.json"), scope_json).unwrap();

        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();

        let python_cmd = IndexCommand {
            executable: py_mock,
            leading_args: vec![],
        };
        let rust_cmd = IndexCommand {
            executable: rs_mock,
            leading_args: vec![],
        };

        let row = MatrixRow {
            name: "ac21_kb_index_annotation_ignored",
            stage: Stage::M2,
            kind: MatrixRowKind::KbIndex {
                corpus_subdir: "reading_01",
            },
        };
        let report = run_matrix_row(&row, &corpus, &out_root, tmp.path(), &python_cmd, &rust_cmd)
            .expect("row must run");

        // The cards_manifest diff is plain text ("PYTHON-..." vs
        // "RUST-...") with no CACG- codes at all → trust-bearing by
        // default. Annotation ignored on KbIndex anyway. Row must Fail.
        match &report.status {
            EntryStatus::Fail(_) => {}
            other => {
                panic!("KbIndex row with scope.json must still Fail on a real diff; got {other:?}")
            }
        }
    }

    // Codex Round 8 review found that `str::lines()` is lossy on the
    // terminator boundary — `b"foo\n"` and `b"foo"` produced
    // identical line iterators, so a trust-bearing diff that only
    // differs by a trailing `\n` (or by CRLF-vs-LF) could be demoted
    // to FutureStage in a hot-path KbLint row. Round 9 makes the
    // classifier byte-lossless via `split_inclusive(b'\n')`-style
    // records. The five tests below pin that closure.

    #[test]
    fn diff_classifier_byte_lossless_terminator_only_trust_diff_is_fail() {
        // Trailing-newline-only diff on a trust-bearing line: NOT
        // aux-only. Python emits the line with `\n`; Rust emits it
        // without. The records `"x CACG-CITE-001\n"` and
        // `"x CACG-CITE-001"` are distinct byte-records, so both have
        // count (1, 0) or (0, 1) — both divergent, and the trust-
        // bearing classifier rejects.
        assert!(!diff_mentions_only_deferred_codes(
            b"x CACG-CITE-001\n",
            b"x CACG-CITE-001",
        ));
    }

    #[test]
    fn diff_classifier_byte_lossless_terminator_only_plain_diff_is_fail() {
        // Plain trailing-newline diff (no CACG code anywhere): NOT
        // aux-only because the divergent record cannot be confirmed
        // as deferred-only.
        assert!(!diff_mentions_only_deferred_codes(b"plain\n", b"plain",));
    }

    #[test]
    fn diff_classifier_byte_lossless_crlf_vs_lf_trust_diff_is_fail() {
        // CRLF-vs-LF on a trust-bearing line: NOT aux-only. Each
        // record's terminator is distinct under the byte-level key.
        assert!(!diff_mentions_only_deferred_codes(
            b"x CACG-CITE-001\r\n",
            b"x CACG-CITE-001\n",
        ));
    }

    #[test]
    fn diff_classifier_byte_lossless_terminator_only_deferred_diff_is_aux_only() {
        // Trailing-newline-only diff on a deferred-prefix line: aux-
        // only. The divergent record still mentions only `CACG-SKILL-`.
        // The hot-path KbLint row would demote this artifact-diff to
        // FutureStage; the trust-bearing variant in the prior test
        // would Fail.
        assert!(diff_mentions_only_deferred_codes(
            b"x CACG-SKILL-001\n",
            b"x CACG-SKILL-001",
        ));
    }

    #[test]
    fn diff_classifier_byte_lossless_invalid_utf8_fails_closed() {
        // Invalid UTF-8 on a divergent record: fail closed (we can't
        // prove deferred-prefix presence on a record we can't decode).
        // The bytes are: valid prefix + lone 0xC3 continuation
        // sequence.
        let py = &[
            b'x', b' ', b'C', b'A', b'C', b'G', b'-', b'S', b'K', b'I', b'L', b'L', b'-', b'0',
            b'0', b'1', 0xC3, 0x28, b'\n',
        ];
        let rs = b"x CACG-SKILL-001\n";
        assert!(!diff_mentions_only_deferred_codes(py, rs));
    }

    #[test]
    fn diff_mentions_only_deferred_codes_preserves_line_multiplicity() {
        // Codex Round 7 review (Mainline Gap 1): the Round 7
        // BTreeSet-based classifier dropped multiplicity. If Python
        // emits two identical trust-bearing lines and Rust emits one,
        // the artifacts are byte-different but the set symmetric
        // difference is empty. Round 8's BTreeMap<&str, (py_count,
        // rs_count)> classifier MUST flag the line as divergent and
        // therefore classify the diff as NOT aux-only.
        let py = b"x CACG-CITE-001\nx CACG-CITE-001\n";
        let rs = b"x CACG-CITE-001\n";
        assert!(
            !diff_mentions_only_deferred_codes(py, rs),
            "duplicate trust-bearing line count diff must NOT be aux-only"
        );

        // Same multiplicity test but with an auxiliary code: the
        // count diff IS aux-only because the divergent line passes the
        // deferred-prefix classifier.
        let py = b"x CACG-SKILL-001\nx CACG-SKILL-001\n";
        let rs = b"x CACG-SKILL-001\n";
        assert!(
            diff_mentions_only_deferred_codes(py, rs),
            "duplicate auxiliary-code line count diff IS aux-only"
        );
    }

    #[cfg(unix)]
    #[test]
    fn ac21_hot_path_kb_lint_row_fails_on_trailing_newline_trust_bearing_diff() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Codex Round 8 review regression: hot-path scope + a
        // trust-bearing stderr line that differs only in its trailing
        // newline (Python emits `\n`, Rust doesn't) MUST NOT demote
        // to FutureStage. Round 8's `str::lines()`-based classifier
        // would have missed this; Round 9's byte-lossless classifier
        // catches it.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(
            tmp.path(),
            "py_lint_trail_newline.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'cards/a.md: CACG-CITE-001 malformed chunk_id\\n' >&2\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_lint_trail_newline.sh",
            // Same content WITHOUT the trailing newline.
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'cards/a.md: CACG-CITE-001 malformed chunk_id' >&2\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );

        let cards_dir = tmp.path().join("cards");
        fs::create_dir_all(&cards_dir).unwrap();
        fs::write(cards_dir.join("card_a.md"), b"---\nid: a\n---\nbody\n").unwrap();
        fs::write(
            cards_dir.join("scope.json"),
            r#"{"schema_version": "cacg.v0", "scope": "hot-path"}"#,
        )
        .unwrap();

        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();
        fs::write(tmp.path().join("chunks.json"), b"{}").unwrap();
        fs::write(tmp.path().join("matrix.json"), b"{}").unwrap();

        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let report = run_kb_lint_entry_with(
            "ac21_trailing_newline_trust_diff",
            Stage::M2,
            &cards_dir,
            &out_root,
            tmp.path(),
            "chunks.json",
            "matrix.json",
            &binaries,
            None,
        );

        match &report.status {
            EntryStatus::Fail(reason) => {
                assert!(
                    reason.contains("artifacts diffed"),
                    "expected aggregated-diff failure reason; got {reason:?}"
                );
            }
            other => panic!(
                "trailing-newline trust-bearing diff under hot-path scope MUST stay Fail; got {other:?}"
            ),
        }
        let stderr_diff = report
            .comparisons
            .iter()
            .find(|c| c.name.ends_with(".stderr"))
            .expect("stderr comparison present");
        assert!(
            !stderr_diff.equal,
            "stderr trailing-newline diff must remain not-equal"
        );
    }

    #[cfg(unix)]
    #[test]
    fn ac21_hot_path_kb_lint_row_fails_on_duplicate_trust_bearing_line_count_diff() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Codex Round 7 review regression (Mainline Gap 1): hot-path
        // scope + Python emitting two identical CACG-CITE-001 lines +
        // Rust emitting one MUST NOT demote to FutureStage. The row
        // status MUST be Fail because the divergent line is
        // trust-bearing.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(
            tmp.path(),
            "py_lint_dup_trust.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'cards/a.md: CACG-CITE-001 malformed chunk_id\\ncards/a.md: CACG-CITE-001 malformed chunk_id\\n' >&2\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_lint_dup_trust.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'cards/a.md: CACG-CITE-001 malformed chunk_id\\n' >&2\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );

        let cards_dir = tmp.path().join("cards");
        fs::create_dir_all(&cards_dir).unwrap();
        fs::write(cards_dir.join("card_a.md"), b"---\nid: a\n---\nbody\n").unwrap();
        fs::write(
            cards_dir.join("scope.json"),
            r#"{"schema_version": "cacg.v0", "scope": "hot-path"}"#,
        )
        .unwrap();

        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();
        fs::write(tmp.path().join("chunks.json"), b"{}").unwrap();
        fs::write(tmp.path().join("matrix.json"), b"{}").unwrap();

        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let report = run_kb_lint_entry_with(
            "ac21_dup_trust_bearing_count_diff",
            Stage::M2,
            &cards_dir,
            &out_root,
            tmp.path(),
            "chunks.json",
            "matrix.json",
            &binaries,
            None,
        );

        match &report.status {
            EntryStatus::Fail(reason) => {
                assert!(
                    reason.contains("artifacts diffed"),
                    "expected aggregated-diff failure reason; got {reason:?}"
                );
            }
            other => panic!(
                "duplicate trust-bearing line count diff under hot-path scope MUST stay Fail; got {other:?}"
            ),
        }
        // The diffed artifact must be the stderr (where the count diff
        // lives).
        let stderr_diff = report
            .comparisons
            .iter()
            .find(|c| c.name.ends_with(".stderr"))
            .expect("stderr comparison present");
        assert!(
            !stderr_diff.equal,
            "stderr count diff must remain not-equal"
        );
    }

    #[test]
    fn diff_mentions_only_deferred_codes_classifies_lines_correctly() {
        // Empty diff: trivially aux-only.
        assert!(diff_mentions_only_deferred_codes(b"", b""));
        // Identical content: no divergent lines, aux-only.
        assert!(diff_mentions_only_deferred_codes(b"same\n", b"same\n"));

        // Diff with only CACG-SKILL-001 in the python side: aux-only.
        let py = b"cards/a/SKILL.md: CACG-SKILL-001 router collision\n";
        let rs = b"";
        assert!(diff_mentions_only_deferred_codes(py, rs));

        // Diff with CACG-CITE-001 (trust-bearing) in the python side:
        // NOT aux-only.
        let py = b"cards/a.md: CACG-CITE-001 malformed chunk_id\n";
        let rs = b"";
        assert!(!diff_mentions_only_deferred_codes(py, rs));

        // Mixed line (aux + trust): NOT aux-only.
        let py = b"some prefix CACG-SUM-001 and also CACG-CITE-002\n";
        let rs = b"";
        assert!(!diff_mentions_only_deferred_codes(py, rs));

        // Aux on both sides but at different line content: aux-only.
        let py = b"cards/a/SKILL.md: CACG-SKILL-001 a\ncards/b/SKILL.md: CACG-SKILL-002 b\n";
        let rs = b"cards/a/SKILL.md: CACG-SKILL-001 a\n";
        assert!(diff_mentions_only_deferred_codes(py, rs));

        // Plain text diff with no CACG codes anywhere: trust-bearing
        // by default (we can't classify it as aux without an explicit
        // deferred-prefix mention).
        assert!(!diff_mentions_only_deferred_codes(b"hello\n", b"world\n"));
    }

    #[cfg(unix)]
    #[test]
    fn ac21_hot_path_kb_lint_row_demotes_aux_only_diff_to_future_stage() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Wire mock python/rust binaries that emit IDENTICAL exit
        // codes and stdout but DIFFER on stderr by an aux-only line
        // (CACG-SKILL-001). With a hot-path scope.json present, the
        // KbLint row must demote the diff to FutureStage("M3") per
        // §3a.4.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(
            tmp.path(),
            "py_lint_aux_diff.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'cards/a/SKILL.md: CACG-SKILL-001 router collision\\n' >&2\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_lint_aux_diff.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );

        let cards_dir = tmp.path().join("cards");
        fs::create_dir_all(&cards_dir).unwrap();
        fs::write(cards_dir.join("card_a.md"), b"---\nid: a\n---\nbody\n").unwrap();
        // Hot-path scope annotation at the cards_dir root.
        fs::write(
            cards_dir.join("scope.json"),
            r#"{"schema_version": "cacg.v0", "scope": "hot-path"}"#,
        )
        .unwrap();

        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();
        fs::write(tmp.path().join("chunks.json"), b"{}").unwrap();
        fs::write(tmp.path().join("matrix.json"), b"{}").unwrap();

        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let report = run_kb_lint_entry_with(
            "ac21_aux_demotion_test",
            Stage::M2,
            &cards_dir,
            &out_root,
            tmp.path(),
            "chunks.json",
            "matrix.json",
            &binaries,
            None,
        );

        // Status MUST be FutureStage("M3"), NOT Fail.
        match &report.status {
            EntryStatus::FutureStage(milestone) => {
                assert_eq!(*milestone, "M3", "deferred lint codes carry-forward to M3");
            }
            other => panic!("expected FutureStage(M3) on hot-path aux-only diff; got {other:?}"),
        }
        // The stderr comparison MUST still be reported as not-equal
        // (the demotion is at the row-status layer; individual
        // ArtifactComparison rows still report the diff for the
        // perf JSON).
        let stderr_diff = report
            .comparisons
            .iter()
            .find(|c| c.name.ends_with(".stderr"))
            .expect("stderr comparison present");
        assert!(
            !stderr_diff.equal,
            "stderr comparison should still flag the aux-only diff; the demotion is at the row-status layer"
        );
    }

    #[cfg(unix)]
    #[test]
    fn ac21_hot_path_kb_lint_row_still_fails_on_trust_bearing_diff() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Same setup as the demotion test BUT the stderr diff includes
        // a trust-bearing CACG-CITE-001 line. The demotion must NOT
        // apply; the row must Fail.
        let tmp = TempDir::new().unwrap();
        let py_mock = write_executable_script(
            tmp.path(),
            "py_lint_trust_diff.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf 'cards/a.md: CACG-CITE-001 malformed chunk_id\\n' >&2\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );
        let rs_mock = write_executable_script(
            tmp.path(),
            "rs_lint_trust_diff.sh",
            "#!/bin/sh\nset -e\nwhile [ \"$#\" -gt 0 ]; do\n  case \"$1\" in\n    --journal) journal=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\nprintf '{\"event\":\"lint\"}\\n' > \"$journal\"\nexit 0\n",
        );

        let cards_dir = tmp.path().join("cards");
        fs::create_dir_all(&cards_dir).unwrap();
        fs::write(cards_dir.join("card_a.md"), b"---\nid: a\n---\nbody\n").unwrap();
        fs::write(
            cards_dir.join("scope.json"),
            r#"{"schema_version": "cacg.v0", "scope": "hot-path"}"#,
        )
        .unwrap();

        let out_root = tmp.path().join("out");
        fs::create_dir_all(&out_root).unwrap();
        fs::write(tmp.path().join("chunks.json"), b"{}").unwrap();
        fs::write(tmp.path().join("matrix.json"), b"{}").unwrap();

        let binaries = LintBinaries {
            python_executable: py_mock,
            rust_executable: rs_mock,
        };
        let report = run_kb_lint_entry_with(
            "ac21_trust_bearing_diff_test",
            Stage::M2,
            &cards_dir,
            &out_root,
            tmp.path(),
            "chunks.json",
            "matrix.json",
            &binaries,
            None,
        );

        // The trust-bearing CACG-CITE-001 diff must KEEP the row at
        // Fail despite the hot-path annotation.
        match &report.status {
            EntryStatus::Fail(reason) => {
                assert!(
                    reason.contains("artifacts diffed"),
                    "expected aggregated-diff failure reason; got {reason:?}"
                );
            }
            other => panic!(
                "expected Fail on trust-bearing diff even with hot-path scope; got {other:?}"
            ),
        }
    }

    #[test]
    fn is_hot_path_scope_annotation_validates_schema_version_and_scope() {
        // Codex Round 7 review flagged that the Round 7 reader didn't
        // validate `schema_version`. Round 8 tightens to a boolean
        // helper that requires BOTH fields to match exactly. This test
        // covers every documented failure mode + the one accepted shape.
        let tmp = TempDir::new().unwrap();

        // Missing scope.json -> false.
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Malformed JSON -> false.
        fs::write(tmp.path().join("scope.json"), "{not json").unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Missing schema_version field -> false.
        fs::write(tmp.path().join("scope.json"), r#"{"scope": "hot-path"}"#).unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Wrong schema_version value -> false.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": "cacg.v1", "scope": "hot-path"}"#,
        )
        .unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Non-string schema_version (number) -> false.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": 0, "scope": "hot-path"}"#,
        )
        .unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Non-string schema_version (bool) -> false.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": true, "scope": "hot-path"}"#,
        )
        .unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Non-string schema_version (null) -> false.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": null, "scope": "hot-path"}"#,
        )
        .unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Missing scope field -> false.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": "cacg.v0"}"#,
        )
        .unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Non-string scope (number) -> false.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": "cacg.v0", "scope": 1}"#,
        )
        .unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Non-string scope (bool) -> false.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": "cacg.v0", "scope": true}"#,
        )
        .unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Non-string scope (null) -> false.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": "cacg.v0", "scope": null}"#,
        )
        .unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Wrong scope value (e.g., "experimental") -> false.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": "cacg.v0", "scope": "experimental"}"#,
        )
        .unwrap();
        assert!(!is_hot_path_scope_annotation(tmp.path()));

        // Exact valid annotation -> true.
        fs::write(
            tmp.path().join("scope.json"),
            r#"{"schema_version": "cacg.v0", "scope": "hot-path"}"#,
        )
        .unwrap();
        assert!(is_hot_path_scope_annotation(tmp.path()));
    }

    #[test]
    fn future_stage_entry_serializes_with_tagged_shape() {
        // The perf-report JSON for a FutureStage entry MUST carry the
        // serde-tagged shape `{"status":"FutureStage","reason":"M3"}`
        // so downstream consumers that key off `status` know whether
        // to gate the build.
        let entry = EntryReport {
            name: "future_m3_smoke".to_string(),
            stage: Stage::Future("M3"),
            expected_command: String::new(),
            rust_command: String::new(),
            expected_duration_ms: 0,
            rust_duration_ms: 0,
            comparisons: Vec::new(),
            status: EntryStatus::FutureStage("M3"),
        };
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("\"status\":\"FutureStage\""));
        assert!(json.contains("\"reason\":\"M3\""));
        assert!(json.contains("\"stage\":\"Future\""));
        assert!(json.contains("\"milestone\":\"M3\""));
    }

    #[test]
    fn build_summary_with_kb_index_and_future_help_snapshot_counts_both_correctly() {
        // The row-table refactor introduces two entries per run: the
        // existing M2 kb-index row + a future-stage help-snapshot
        // row. The summary must record passed=1 / future_stage=1 /
        // failed=0 when the M2 row passes; main.rs's `failed == 0`
        // gate then exits 0.
        let entries = vec![
            EntryReport {
                name: "kb_index_parity_corpus_reading_01".to_string(),
                stage: Stage::M2,
                expected_command: String::new(),
                rust_command: String::new(),
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: vec![ArtifactComparison {
                    name: "cards_manifest.json".to_string(),
                    expected_path: PathBuf::from("p"),
                    rust_path: PathBuf::from("r"),
                    expected_exists: true,
                    rust_exists: true,
                    expected_bytes: 1,
                    rust_bytes: 1,
                    equal: true,
                    diff_summary: None,
                    diff_detail: None,
                }],
                status: EntryStatus::Pass,
            },
            EntryReport {
                name: "future_m3_smoke".to_string(),
                stage: Stage::Future("M3"),
                expected_command: String::new(),
                rust_command: String::new(),
                expected_duration_ms: 0,
                rust_duration_ms: 0,
                comparisons: Vec::new(),
                status: EntryStatus::FutureStage("M3"),
            },
        ];
        let s = build_summary(&entries);
        assert_eq!(s.total, 2);
        assert_eq!(s.passed, 1);
        assert_eq!(s.failed, 0, "future-stage rows must not count as failed");
        assert_eq!(s.future_stage, 1);
    }

    #[cfg(unix)]
    fn workspace_root_for_test() -> PathBuf {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.pop();
        p
    }

    #[cfg(unix)]
    fn copy_dir_recursive(src: &Path, dst: &Path) {
        fs::create_dir_all(dst).unwrap();
        for entry in fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy_dir_recursive(&src_path, &dst_path);
            } else {
                fs::copy(&src_path, &dst_path).unwrap();
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn committed_oracle_mutation_does_not_affect_live_harness_pass() {
        let _guard = SUBPROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // The plan's live-comparison strategy says the harness compares
        // LIVE Python vs LIVE Rust on isolated corpus copies and does
        // NOT consume the committed `out_python/stale_hash/...` oracle
        // bytes. This test mutates the oracle in a TEMP COPY of the
        // parity corpus, runs `run_parity` against that temp copy, and
        // asserts the live harness still reports zero failed entries.
        // A version that reads the oracle would fail this test.
        let ws = workspace_root_for_test();
        let committed_corpus = ws.join("tests/parity_corpus");
        if !committed_corpus.is_dir() {
            eprintln!("skipping oracle-mutation meta-test: committed corpus not present");
            return;
        }
        // Resolve python early: if neither the venv nor system python3
        // is usable on this host, skip the test rather than fail it.
        let python = python_exe(&ws);
        if Command::new(&python)
            .arg("--version")
            .output()
            .ok()
            .filter(|o| o.status.success())
            .is_none()
        {
            eprintln!("skipping oracle-mutation meta-test: python not runnable at {python:?}");
            return;
        }

        // Anchor the tempdir under the workspace `target/` so the
        // subprocess `cargo run -p cacg-cli` invoked by the harness
        // can walk upward from its isolated corpus cwd and discover
        // the workspace `Cargo.toml`. A bare `TempDir::new()` placed
        // tempdirs under `/tmp/` where cargo cannot locate the
        // workspace, and the inner cargo build would fail with exit
        // 101 -- a harness-environment issue, not an oracle-
        // consumption signal.
        let target_dir = ws.join("target/oracle-mutation-test");
        fs::create_dir_all(&target_dir).unwrap();
        let tmp = TempDir::new_in(&target_dir).unwrap();
        let temp_corpus = tmp.path().join("parity_corpus");
        copy_dir_recursive(&committed_corpus, &temp_corpus);

        let oracle = temp_corpus
            .join("out_python/stale_hash/reading_01/01-content-addressable-identity.history.jsonl");
        assert!(oracle.is_file(), "oracle must exist in the temp copy");
        let mut oracle_bytes = fs::read(&oracle).unwrap();
        // Replace the entire content with bytes that would obviously
        // fail any byte-equal check if the harness consumed them.
        // Using a non-JSON marker makes any accidental consumer surface
        // a parse error in addition to a byte diff.
        let mutated = b"OR4CLE-MUTATED-INVALID-BYTES-NOT-JSON\n";
        oracle_bytes.clear();
        oracle_bytes.extend_from_slice(mutated);
        fs::write(&oracle, &oracle_bytes).unwrap();

        let temp_out = tmp.path().join("out");
        fs::create_dir_all(&temp_out).unwrap();
        let report = run_parity(&temp_corpus, &temp_out)
            .expect("run_parity should succeed against the mutated-oracle temp corpus");
        // Rows using committed fixtures may fail in the temp-corpus
        // context (fixture paths contain workspace-relative card paths
        // that diverge from the temp-corpus Rust output). Filter to only
        // check live-Python rows.
        let live_failed = report
            .entries
            .iter()
            .filter(|e| !e.expected_command.starts_with("committed-fixture:"))
            .filter(|e| matches!(e.status, EntryStatus::Fail(_)))
            .count();
        assert_eq!(
            live_failed, 0,
            "live-Python rows must not gate on the mutated committed oracle"
        );
        // Both M2 kb_index rows (live Python, not fixture-based) must
        // still pass under the mutated-oracle temp corpus.
        let stale = report
            .entries
            .iter()
            .find(|e| e.name == "kb_index_parity_corpus_stale_hash_reading_01")
            .expect("stale-hash row must appear in the report");
        match &stale.status {
            EntryStatus::Pass => {}
            other => panic!("stale-hash row must Pass even with mutated oracle; got {other:?}"),
        }
        let clean = report
            .entries
            .iter()
            .find(|e| e.name == "kb_index_parity_corpus_reading_01")
            .expect("clean-hash row must appear in the report");
        match &clean.status {
            EntryStatus::Pass => {}
            other => panic!("clean-hash row must Pass; got {other:?}"),
        }
    }

    #[test]
    fn parity_report_serializes_to_json() {
        let report = ParityReport {
            timestamp: "1970-01-01T00:00:00Z".to_string(),
            corpus: PathBuf::from("tests/parity_corpus/"),
            entries: vec![EntryReport {
                name: "kb_index".to_string(),
                stage: Stage::M2,
                expected_command: "python -m cacg.cli index ...".to_string(),
                rust_command: "cargo run -p cacg-cli -- index ...".to_string(),
                expected_duration_ms: 100,
                rust_duration_ms: 50,
                comparisons: vec![ArtifactComparison {
                    name: "cards_manifest.json".to_string(),
                    expected_path: PathBuf::from("a"),
                    rust_path: PathBuf::from("b"),
                    expected_exists: true,
                    rust_exists: true,
                    expected_bytes: 100,
                    rust_bytes: 100,
                    equal: true,
                    diff_summary: None,
                    diff_detail: None,
                }],
                status: EntryStatus::Pass,
            }],
            summary: Summary {
                total: 1,
                passed: 1,
                failed: 0,
                future_stage: 0,
            },
        };
        let json = serde_json::to_string_pretty(&report).unwrap();
        assert!(json.contains("\"timestamp\": \"1970-01-01T00:00:00Z\""));
        assert!(json.contains("\"kb_index\""));
        assert!(json.contains("\"stage\": \"M2\""));
        assert!(json.contains("\"status\": \"Pass\""));
    }
}
