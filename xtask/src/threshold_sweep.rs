//! `xtask threshold-sweep`: replay the committed semantic cache's
//! stored scores against a swept range of decision thresholds and
//! emit the per-threshold (pass / fail / abstain) distribution.
//!
//! The sweep does NOT invoke the embedding model and does NOT need
//! Python or `sentence-transformers` installed: it reads only the
//! committed cache JSON bytes (`score` fields) plus the committed
//! provenance sidecar (frozen counts + threshold). The output is
//! consumed by the QM-vertical threshold calibration document; the
//! chosen threshold is locked into the cache builder, the audit's
//! pinned-value check, and a sweep-side range assertion so all
//! three surfaces drift together.
//!
//! Output shape (stdout): a canonical-JSON object with `vertical`,
//! the swept range parameters, `total_entries`, and an array of
//! `rows`, each row `{ "abstain": 0, "fail": n, "pass": n, "threshold": f }`.
//! The `abstain` column is always `0` for the in-cache score sweep
//! (B1 cache entries store a binary pass/fail verdict), but the
//! column is reserved in the schema so the future B2 LLM-judge
//! cache shape can populate it without a downstream-tooling churn.
//! Stderr carries a brief human-readable summary so an operator
//! can eyeball the knee of the distribution without parsing JSON.
//!
//! Frozen-shape preflight: the sweep reads the committed
//! provenance sidecar before emitting any rows and asserts the
//! frozen-count contract (222 paraphrase + 5..=10 negative + total
//! arithmetic + cache.entries.len() agreement). A non-canonical
//! cache (different paraphrase count, drifted entry total)
//! produces a failure BEFORE any sweep output, matching the QM
//! frozen-label contract for the calibration sweep.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use cacg_core::canonical_json::canonical_json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::semantic_cache_provenance::{
    EXPECTED_PARAPHRASE_COUNT, EXPECTED_SCHEMA_VERSION, NEGATIVE_FIXTURE_COUNT_MAX,
    NEGATIVE_FIXTURE_COUNT_MIN,
};

/// Only the QM vertical has a frozen paraphrase label set committed
/// to the workspace. Building a sweep against any other vertical
/// would silently scan empty data.
pub const SUPPORTED_VERTICALS: &[&str] = &["qm"];

/// Default lower bound of the swept range (inclusive).
pub const DEFAULT_FROM: f64 = 0.40;
/// Default upper bound of the swept range (inclusive).
pub const DEFAULT_TO: f64 = 0.99;
/// Default step between successive thresholds.
pub const DEFAULT_STEP: f64 = 0.01;
/// Default vertical (the only frozen QM label set).
pub const DEFAULT_VERTICAL: &str = "qm";

/// Integer scale used to walk the threshold range. `from`, `to`, and
/// `step` are converted to ticks of `1 / SCALE_FACTOR` so the
/// iteration is exact in integer space and the emitted threshold
/// values round-trip cleanly through `f64::to_string()` (e.g. 0.5
/// stays as the string `0.5`, not `0.5000000000001`).
const SCALE_FACTOR: f64 = 1_000.0;

#[derive(Debug, Deserialize)]
struct CacheShape {
    schema_version: String,
    entries: Vec<CacheEntry>,
}

#[derive(Debug, Deserialize)]
struct CacheEntry {
    score: f64,
}

/// Minimal strict-shape deserializer for the provenance sidecar.
/// Reads only the fields the sweep preflight needs; the full audit
/// owns the strict-deny-unknown contract over the same file.
#[derive(Debug, Deserialize)]
struct ProvenanceShape {
    schema_version: String,
    #[allow(dead_code)]
    threshold: f64,
    entry_count: usize,
    paraphrase_count: usize,
    negative_fixture_count: usize,
}

/// One row in the sweep report.
#[derive(Debug, Clone, Copy)]
pub struct SweepRow {
    /// Threshold value at which `pass`, `fail`, and `abstain` are
    /// counted.
    pub threshold: f64,
    /// Number of cache entries where `score >= threshold`.
    pub pass: usize,
    /// Number of cache entries where `score < threshold`.
    pub fail: usize,
    /// Number of cache entries with an abstain verdict. Always `0`
    /// for the B1 in-cache score sweep (the cache stores a binary
    /// pass/fail verdict at build time). Reserved for the future
    /// B2 LLM-judge cache shape.
    pub abstain: usize,
}

/// The full sweep report. Serialized as canonical JSON to stdout.
#[derive(Debug, Clone)]
pub struct SweepReport {
    /// Vertical that was swept (matches `SUPPORTED_VERTICALS`).
    pub vertical: String,
    /// Cache file the scores were read from (informational).
    pub cache_path: PathBuf,
    /// Provenance file the frozen counts were read from
    /// (informational).
    pub provenance_path: PathBuf,
    /// Lower bound of the swept range (inclusive).
    pub from: f64,
    /// Upper bound of the swept range (inclusive).
    pub to: f64,
    /// Step size between successive thresholds.
    pub step: f64,
    /// Total cache entries scanned (constant across rows).
    pub total_entries: usize,
    /// Per-threshold (pass, fail, abstain) counts.
    pub rows: Vec<SweepRow>,
}

/// Run a threshold sweep against the cache at `cache_path`.
///
/// Reads the committed provenance sidecar at `provenance_path` and
/// asserts the frozen-count contract BEFORE emitting any rows. A
/// non-frozen cache (paraphrase count drift, entry-count arithmetic
/// drift, cache/provenance disagreement) fails preflight with no
/// rows emitted.
pub fn run(
    cache_path: &Path,
    provenance_path: &Path,
    vertical: &str,
    from: f64,
    to: f64,
    step: f64,
) -> Result<SweepReport> {
    validate_args(vertical, from, to, step)?;

    let cache_bytes = std::fs::read(cache_path)
        .with_context(|| format!("read cache at {}", cache_path.display()))?;
    let cache: CacheShape = serde_json::from_slice(&cache_bytes)
        .with_context(|| format!("parse cache at {}", cache_path.display()))?;
    if cache.schema_version != EXPECTED_SCHEMA_VERSION {
        bail!(
            "unexpected cache schema_version {:?}; expected {:?}",
            cache.schema_version,
            EXPECTED_SCHEMA_VERSION
        );
    }

    let provenance_bytes = std::fs::read(provenance_path)
        .with_context(|| format!("read provenance at {}", provenance_path.display()))?;
    let provenance: ProvenanceShape = serde_json::from_slice(&provenance_bytes)
        .with_context(|| format!("parse provenance at {}", provenance_path.display()))?;

    preflight(&cache, &provenance)?;

    let scores: Vec<f64> = cache.entries.iter().map(|e| e.score).collect();
    let rows = compute_rows(&scores, from, to, step)?;

    Ok(SweepReport {
        vertical: vertical.to_string(),
        cache_path: cache_path.to_path_buf(),
        provenance_path: provenance_path.to_path_buf(),
        from,
        to,
        step,
        total_entries: scores.len(),
        rows,
    })
}

fn validate_args(vertical: &str, from: f64, to: f64, step: f64) -> Result<()> {
    if !SUPPORTED_VERTICALS.contains(&vertical) {
        bail!("unsupported vertical {vertical:?}; supported: {SUPPORTED_VERTICALS:?}");
    }
    if !(step > 0.0 && step.is_finite()) {
        bail!("step must be a finite positive number; got {step}");
    }
    if !(from.is_finite() && to.is_finite()) {
        bail!("from and to must be finite; got from={from}, to={to}");
    }
    if from > to {
        bail!("from ({from}) must be <= to ({to})");
    }
    // Reject values that do not lie on the integer-tick grid the
    // iteration assumes. Without this check, `compute_rows` rounds
    // each value to the nearest tick (e.g. `step = 0.0015` becomes
    // 2 ticks = `0.002`) but the emitted report still echoes the
    // ORIGINAL operator-supplied number, which can mislead
    // calibration. Rejecting non-aligned inputs at the boundary is
    // both honest and strictly safer than silently snapping.
    ensure_tick_aligned("from", from)?;
    ensure_tick_aligned("to", to)?;
    ensure_tick_aligned("step", step)?;
    // Cross-axis alignment: the iteration emits ticks
    // `from_tick, from_tick + step_tick, ...` while
    // `tick <= to_tick`. If `(to_tick - from_tick)` is not an
    // exact integer multiple of `step_tick`, the loop stops
    // short of `to` and the emitted report's `to` field
    // misrepresents the sweep. Per-axis ticking-alignment is
    // necessary but not sufficient — `0.40 / 0.99 / 0.02` is
    // tick-aligned on every axis but `(990 - 400) % 20 == 10`,
    // so the sweep stops at `0.98` while the report still
    // claims `to = 0.99`. Reject the input at the boundary so
    // the misleading-report path is unreachable.
    let from_tick = to_tick(from);
    let to_tick_value = to_tick(to);
    let step_tick = to_tick(step);
    if step_tick > 0 {
        let span = to_tick_value - from_tick;
        if span % step_tick != 0 {
            bail!(
                "(to - from) = {span_value} is not an integer multiple of step = {step}; \
                 the sweep would stop short of `to` ({to}) and the report would \
                 misrepresent the upper bound. Adjust `to` or `step` so the range is \
                 step-aligned.",
                span_value = to - from,
            );
        }
    }
    Ok(())
}

/// Round `value * SCALE_FACTOR` to the nearest integer tick. The
/// validator and the row-computation loop both walk the threshold
/// range in tick-space so float-accumulation drift is impossible;
/// centralizing the conversion here keeps the two sites in lock-step.
fn to_tick(value: f64) -> i64 {
    (value * SCALE_FACTOR).round() as i64
}

/// True when `value * SCALE_FACTOR` is within a small absolute
/// epsilon of an integer. The epsilon (`1e-6`) covers the f64
/// rounding error that accumulates from typical decimal string
/// parsing (e.g. `0.5_f64 * 1_000.0` is exactly `500.0`, but
/// `0.123_f64 * 1_000.0` carries a few ULPs of error). It is
/// orders of magnitude tighter than the tick size (`1e-3`), so
/// genuinely non-aligned values like `0.0015` are still detected.
fn ensure_tick_aligned(name: &str, value: f64) -> Result<()> {
    let scaled = value * SCALE_FACTOR;
    let rounded = scaled.round();
    if (scaled - rounded).abs() > 1e-6 {
        bail!(
            "{name} ({value}) is not aligned to the {tick} tick grid; \
             supply a value that is an exact multiple of {tick}",
            tick = 1.0 / SCALE_FACTOR,
        );
    }
    Ok(())
}

/// Assert the cache + provenance pair satisfies the frozen QM
/// label-set contract. Any failure here is the calibration
/// sweep's frozen-label negative gate: a non-frozen label set
/// MUST refuse to sweep.
fn preflight(cache: &CacheShape, provenance: &ProvenanceShape) -> Result<()> {
    if provenance.schema_version != EXPECTED_SCHEMA_VERSION {
        bail!(
            "provenance schema_version mismatch: expected {:?}, got {:?}",
            EXPECTED_SCHEMA_VERSION,
            provenance.schema_version
        );
    }
    if provenance.paraphrase_count != EXPECTED_PARAPHRASE_COUNT {
        bail!(
            "non-frozen QM label set: provenance.paraphrase_count must be \
             {EXPECTED_PARAPHRASE_COUNT}; got {}. The threshold sweep \
             refuses to scan a cache whose paraphrase count differs from \
             the frozen QM label set.",
            provenance.paraphrase_count
        );
    }
    if !(NEGATIVE_FIXTURE_COUNT_MIN..=NEGATIVE_FIXTURE_COUNT_MAX)
        .contains(&provenance.negative_fixture_count)
    {
        bail!(
            "provenance.negative_fixture_count must be in [{NEGATIVE_FIXTURE_COUNT_MIN}, \
             {NEGATIVE_FIXTURE_COUNT_MAX}]; got {}",
            provenance.negative_fixture_count
        );
    }
    let expected_total = provenance.paraphrase_count + provenance.negative_fixture_count;
    if provenance.entry_count != expected_total {
        bail!(
            "provenance.entry_count must equal paraphrase_count + \
             negative_fixture_count = {expected_total}; got entry_count = {}",
            provenance.entry_count
        );
    }
    if cache.entries.len() != provenance.entry_count {
        bail!(
            "cache.entries.len() = {} does not match provenance.entry_count = {}",
            cache.entries.len(),
            provenance.entry_count
        );
    }
    Ok(())
}

fn compute_rows(scores: &[f64], from: f64, to: f64, step: f64) -> Result<Vec<SweepRow>> {
    // Walk the threshold range in integer ticks to avoid
    // float-accumulation drift. The tick size is `1 / SCALE_FACTOR`
    // so a `step` of 0.01 becomes 10 ticks per integer unit.
    let from_tick = to_tick(from);
    let to_tick_value = to_tick(to);
    let step_tick = to_tick(step);
    if step_tick <= 0 {
        bail!(
            "step too small for the iteration scale ({step}); minimum is {}",
            1.0 / SCALE_FACTOR
        );
    }

    let mut rows = Vec::new();
    let mut tick = from_tick;
    while tick <= to_tick_value {
        let threshold = (tick as f64) / SCALE_FACTOR;
        let pass = scores.iter().filter(|&&s| s >= threshold).count();
        let fail = scores.len() - pass;
        // B1 in-cache score sweep never emits abstain — every entry
        // stored a binary pass/fail verdict at build time. The
        // column is reserved by the verdict-distribution contract
        // for the future LLM-judge cache shape.
        let abstain = 0usize;
        rows.push(SweepRow {
            threshold,
            pass,
            fail,
            abstain,
        });
        tick += step_tick;
    }
    Ok(rows)
}

/// Emit the report to stdout as canonical JSON (sorted keys,
/// minimal separators, UTF-8). Downstream tooling can parse this
/// directly.
pub fn emit_canonical_stdout(report: &SweepReport) -> Result<()> {
    let rows_value: Vec<Value> = report
        .rows
        .iter()
        .map(|r| {
            json!({
                "threshold": r.threshold,
                "pass": r.pass,
                "fail": r.fail,
                "abstain": r.abstain,
            })
        })
        .collect();
    let payload = json!({
        "vertical": report.vertical,
        "cache_path": report.cache_path.display().to_string(),
        "provenance_path": report.provenance_path.display().to_string(),
        "from": report.from,
        "to": report.to,
        "step": report.step,
        "total_entries": report.total_entries,
        "rows": rows_value,
    });
    let text = canonical_json(&payload)
        .map_err(|e| anyhow::anyhow!("canonical-JSON serialize failed: {e}"))?;
    println!("{text}");
    Ok(())
}

/// Print a brief, human-readable summary to stderr. Strides the
/// rows so the summary stays compact even on long sweeps.
pub fn emit_human_summary(report: &SweepReport) {
    eprintln!(
        "threshold-sweep[{}]: scanned {} entries across [{}, {}] step {}; {} rows",
        report.vertical,
        report.total_entries,
        report.from,
        report.to,
        report.step,
        report.rows.len(),
    );
    if report.rows.is_empty() {
        return;
    }
    let stride = (report.rows.len() / 10).max(1);
    let last_idx = report.rows.len() - 1;
    for (i, row) in report.rows.iter().enumerate() {
        if i % stride == 0 || i == last_idx {
            let pass_pct = if report.total_entries > 0 {
                100.0 * (row.pass as f64) / (report.total_entries as f64)
            } else {
                0.0
            };
            eprintln!(
                "  t={:.3}: pass={:>4} fail={:>4} abstain={:>4} ({:.1}% pass)",
                row.threshold, row.pass, row.fail, row.abstain, pass_pct
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Build a 227-entry cache + matching (222, 5, 227) provenance
    /// so the preflight passes. The first 5 entries are forced to
    /// score = 0.0 (mimicking the committed negative fixtures);
    /// the remaining 222 take their scores from `paraphrase_scores`
    /// (padded to 222 with `pad_score` if shorter, panic if
    /// longer).
    fn write_committed_shape_pair(
        dir: &Path,
        paraphrase_scores: &[f64],
        pad_score: f64,
    ) -> (PathBuf, PathBuf) {
        assert!(
            paraphrase_scores.len() <= 222,
            "test helper expects at most 222 paraphrase scores"
        );
        let mut entry_scores: Vec<f64> = vec![0.0; 5];
        entry_scores.extend_from_slice(paraphrase_scores);
        while entry_scores.len() < 227 {
            entry_scores.push(pad_score);
        }
        write_pair_raw(dir, &entry_scores, 222, 5, 227)
    }

    /// Write a cache + provenance pair with explicit counts. Used
    /// by negative tests to drift specific fields.
    fn write_pair_raw(
        dir: &Path,
        entry_scores: &[f64],
        paraphrase_count: usize,
        negative_fixture_count: usize,
        provenance_entry_count: usize,
    ) -> (PathBuf, PathBuf) {
        let entries_json: Vec<Value> = entry_scores
            .iter()
            .enumerate()
            .map(|(i, score)| {
                json!({
                    "chunk_hash": format!("{i:064}"),
                    "claim_window_hash": format!("{:064}", i + 10_000),
                    "verdict": if *score >= 0.5 { "pass" } else { "fail" },
                    "score": score,
                })
            })
            .collect();
        let cache_body = json!({
            "schema_version": "cacg.v0",
            "entries": entries_json,
        });
        let cache_path = dir.join("semantic_cache.json");
        fs::write(&cache_path, serde_json::to_string(&cache_body).unwrap()).unwrap();

        let provenance_body = json!({
            "schema_version": "cacg.v0",
            "threshold": 0.5,
            "entry_count": provenance_entry_count,
            "paraphrase_count": paraphrase_count,
            "negative_fixture_count": negative_fixture_count,
        });
        let provenance_path = dir.join("semantic_cache.provenance.json");
        fs::write(
            &provenance_path,
            serde_json::to_string(&provenance_body).unwrap(),
        )
        .unwrap();
        (cache_path, provenance_path)
    }

    #[test]
    fn monotone_pass_count_does_not_grow_as_threshold_rises() {
        let dir = TempDir::new().unwrap();
        let (cache, provenance) =
            write_committed_shape_pair(dir.path(), &[0.10, 0.30, 0.50, 0.70, 0.90], 0.0);
        let report = run(&cache, &provenance, "qm", 0.00, 1.00, 0.10).unwrap();
        let mut prev_pass = report.rows[0].pass;
        for row in &report.rows[1..] {
            assert!(
                row.pass <= prev_pass,
                "pass count must not grow as threshold rises; got {} then {} at t={}",
                prev_pass,
                row.pass,
                row.threshold
            );
            prev_pass = row.pass;
        }
        assert_eq!(report.total_entries, 227);
        // At t=0.00 every entry passes (227); at t=1.00 none do.
        assert_eq!(report.rows.first().unwrap().pass, 227);
        assert_eq!(report.rows.last().unwrap().pass, 0);
    }

    #[test]
    fn pass_fail_abstain_partition_at_every_row() {
        let dir = TempDir::new().unwrap();
        let (cache, provenance) =
            write_committed_shape_pair(dir.path(), &[0.10, 0.45, 0.55, 0.80], 0.0);
        let report = run(&cache, &provenance, "qm", 0.00, 1.00, 0.05).unwrap();
        for row in &report.rows {
            assert_eq!(
                row.pass + row.fail + row.abstain,
                report.total_entries,
                "pass + fail + abstain = total invariant broken at t={}",
                row.threshold
            );
            // B1 score sweep never produces abstain.
            assert_eq!(
                row.abstain, 0,
                "B1 cache score sweep must emit abstain=0; got {row:?}"
            );
        }
    }

    #[test]
    fn non_qm_vertical_is_refused() {
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let err = run(&cache, &provenance, "history", 0.4, 0.9, 0.1).unwrap_err();
        assert!(
            format!("{err:#}").contains("unsupported vertical"),
            "got error: {err:#}"
        );
    }

    #[test]
    fn validate_args_rejects_non_tick_aligned_from() {
        // `from=0.5005` is between two ticks (0.500 and 0.501).
        // The runner used to silently snap to the nearest tick;
        // the validator now refuses the input outright.
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let err = run(&cache, &provenance, "qm", 0.5005, 0.6, 0.01).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("from") && msg.contains("0.5005") && msg.contains("0.001"),
            "expected from + value + tick size in the diagnostic: {msg}"
        );
    }

    #[test]
    fn validate_args_rejects_non_tick_aligned_to() {
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let err = run(&cache, &provenance, "qm", 0.5, 0.6005, 0.01).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("to") && msg.contains("0.6005") && msg.contains("0.001"),
            "expected to + value + tick size in the diagnostic: {msg}"
        );
    }

    #[test]
    fn validate_args_rejects_non_tick_aligned_step() {
        // Codex's literal example: `--step 0.0015` would silently
        // become 2 ticks = `0.002`. The validator must refuse so
        // the emitted report cannot mislead by echoing 0.0015.
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let err = run(&cache, &provenance, "qm", 0.5, 0.6, 0.0015).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("step") && msg.contains("0.0015") && msg.contains("0.001"),
            "expected step + value + tick size in the diagnostic: {msg}"
        );
    }

    #[test]
    fn validate_args_accepts_locked_threshold_and_default_step() {
        // The committed locked threshold (0.5) and the default
        // sweep step (0.01) are exact multiples of the 0.001 tick
        // grid. The tighter validation must continue to accept
        // them so the committed calibration surface keeps
        // working unchanged.
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        run(&cache, &provenance, "qm", 0.5, 0.5, 0.01)
            .expect("locked threshold + default step must remain accepted");
    }

    #[test]
    fn validate_args_accepts_dense_thousandth_grid() {
        // Tighter alignment at the tick size itself must work
        // (e.g. 0.001 step). This prevents an over-tight
        // validator from rejecting legitimate fine sweeps.
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let report = run(&cache, &provenance, "qm", 0.498, 0.502, 0.001)
            .expect("fine 0.001-spaced sweep must be accepted");
        // 0.498, 0.499, 0.500, 0.501, 0.502 → 5 rows.
        assert_eq!(report.rows.len(), 5);
    }

    #[test]
    fn validate_args_rejects_range_not_multiple_of_step() {
        // Codex's literal example. Each axis is tick-aligned
        // individually (0.40, 0.99, 0.02 are all exact
        // multiples of 0.001), but (0.99 - 0.40) = 0.59 is
        // not a multiple of 0.02. Without this validation
        // the loop would stop at 0.98 while the emitted
        // report still claims `to=0.99`.
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let err = run(&cache, &provenance, "qm", 0.40, 0.99, 0.02).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("0.99"),
            "diagnostic must name the requested `to` value: {msg}"
        );
        assert!(
            msg.contains("0.02") || msg.contains("step"),
            "diagnostic must name the offending step: {msg}"
        );
        assert!(
            msg.contains("step-aligned") || msg.contains("multiple of step"),
            "diagnostic must explain the alignment requirement: {msg}"
        );
    }

    #[test]
    fn validate_args_accepts_step_aligned_default_range() {
        // The committed default sweep range (0.4 → 0.99 step 0.01)
        // gives (990 - 400) % 10 == 0, so the new cross-axis check
        // must continue to accept it. Otherwise the canonical
        // calibration surface — the committed locked-threshold
        // sweep — would silently regress.
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let report = run(&cache, &provenance, "qm", 0.40, 0.99, 0.01)
            .expect("committed default range must remain accepted");
        // 0.40, 0.41, …, 0.99 = 60 rows.
        assert_eq!(report.rows.len(), 60);
    }

    #[test]
    fn validate_args_accepts_zero_width_range() {
        // A `from == to` sweep (single-row lookup at the locked
        // threshold) has span 0 and `0 % step_tick == 0` for any
        // positive step. The new check must not reject this
        // existing usage pattern.
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let report = run(&cache, &provenance, "qm", 0.5, 0.5, 0.01)
            .expect("from == to must continue to be accepted");
        assert_eq!(report.rows.len(), 1);
        assert!((report.rows[0].threshold - 0.5).abs() < 1e-9);
    }

    #[test]
    fn non_frozen_paraphrase_count_is_rejected_pre_emission() {
        let dir = TempDir::new().unwrap();
        // 221 paraphrase + 5 negative = 226 entries; provenance
        // says paraphrase_count=221, which is one short of the
        // frozen QM label set.
        let entry_scores: Vec<f64> = vec![0.0; 226];
        let (cache, provenance) = write_pair_raw(dir.path(), &entry_scores, 221, 5, 226);
        let err = run(&cache, &provenance, "qm", 0.40, 0.99, 0.01).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("paraphrase_count"),
            "expected paraphrase_count diagnostic: {msg}"
        );
        assert!(
            msg.contains("222"),
            "expected the frozen 222 value in the diagnostic: {msg}"
        );
        assert!(
            msg.contains("221"),
            "expected the actual 221 value in the diagnostic: {msg}"
        );
    }

    #[test]
    fn entry_count_arithmetic_mismatch_is_rejected() {
        let dir = TempDir::new().unwrap();
        // 222 + 5 = 227 but provenance.entry_count = 228 (broken
        // arithmetic).
        let entry_scores: Vec<f64> = vec![0.0; 227];
        let (cache, provenance) = write_pair_raw(dir.path(), &entry_scores, 222, 5, 228);
        let err = run(&cache, &provenance, "qm", 0.40, 0.99, 0.01).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("entry_count"),
            "expected entry_count diagnostic: {msg}"
        );
    }

    #[test]
    fn cache_entries_len_mismatch_is_rejected() {
        let dir = TempDir::new().unwrap();
        // Provenance says 227 entries; cache has 226.
        let entry_scores: Vec<f64> = vec![0.0; 226];
        let (cache, provenance) = write_pair_raw(dir.path(), &entry_scores, 222, 5, 227);
        let err = run(&cache, &provenance, "qm", 0.40, 0.99, 0.01).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("cache.entries.len()") || msg.contains("entry_count"),
            "expected cache.entries.len mismatch diagnostic: {msg}"
        );
    }

    #[test]
    fn negative_fixture_count_outside_envelope_is_rejected() {
        let dir = TempDir::new().unwrap();
        // 222 + 4 = 226 (below the 5..=10 envelope).
        let entry_scores: Vec<f64> = vec![0.0; 226];
        let (cache, provenance) = write_pair_raw(dir.path(), &entry_scores, 222, 4, 226);
        let err = run(&cache, &provenance, "qm", 0.40, 0.99, 0.01).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("negative_fixture_count"),
            "expected negative_fixture_count diagnostic: {msg}"
        );
    }

    #[test]
    fn committed_provenance_shape_is_accepted() {
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5; 222], 0.0);
        let report = run(&cache, &provenance, "qm", 0.40, 0.99, 0.01).unwrap();
        assert_eq!(report.total_entries, 227);
        assert!(!report.rows.is_empty());
    }

    #[test]
    fn unknown_schema_version_is_rejected() {
        let dir = TempDir::new().unwrap();
        // Hand-roll a malformed cache without going through the
        // committed-shape helper, since the helper hardcodes the
        // expected schema version.
        let cache_path = dir.path().join("semantic_cache.json");
        let cache_body = json!({
            "schema_version": "cacg.v999-future",
            "entries": [],
        });
        fs::write(&cache_path, serde_json::to_string(&cache_body).unwrap()).unwrap();
        let provenance_path = dir.path().join("semantic_cache.provenance.json");
        let provenance_body = json!({
            "schema_version": "cacg.v0",
            "threshold": 0.5,
            "entry_count": 227,
            "paraphrase_count": 222,
            "negative_fixture_count": 5,
        });
        fs::write(
            &provenance_path,
            serde_json::to_string(&provenance_body).unwrap(),
        )
        .unwrap();
        let err = run(&cache_path, &provenance_path, "qm", 0.40, 0.99, 0.01).unwrap_err();
        assert!(
            format!("{err:#}").contains("schema_version"),
            "got: {err:#}"
        );
    }

    #[test]
    fn provenance_schema_version_mismatch_is_rejected() {
        let dir = TempDir::new().unwrap();
        let cache_path = dir.path().join("semantic_cache.json");
        let entries_json: Vec<Value> = (0..227)
            .map(|i| {
                json!({
                    "chunk_hash": format!("{i:064}"),
                    "claim_window_hash": format!("{:064}", i + 10_000),
                    "verdict": "pass",
                    "score": 0.5,
                })
            })
            .collect();
        let cache_body = json!({
            "schema_version": "cacg.v0",
            "entries": entries_json,
        });
        fs::write(&cache_path, serde_json::to_string(&cache_body).unwrap()).unwrap();
        let provenance_path = dir.path().join("semantic_cache.provenance.json");
        let provenance_body = json!({
            "schema_version": "cacg.v999-future",
            "threshold": 0.5,
            "entry_count": 227,
            "paraphrase_count": 222,
            "negative_fixture_count": 5,
        });
        fs::write(
            &provenance_path,
            serde_json::to_string(&provenance_body).unwrap(),
        )
        .unwrap();
        let err = run(&cache_path, &provenance_path, "qm", 0.40, 0.99, 0.01).unwrap_err();
        assert!(
            format!("{err:#}").contains("schema_version"),
            "got: {err:#}"
        );
    }

    #[test]
    fn zero_or_negative_step_is_rejected() {
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let err = run(&cache, &provenance, "qm", 0.4, 0.9, 0.0).unwrap_err();
        assert!(format!("{err:#}").contains("step"), "got: {err:#}");
        let err = run(&cache, &provenance, "qm", 0.4, 0.9, -0.1).unwrap_err();
        assert!(format!("{err:#}").contains("step"), "got: {err:#}");
    }

    #[test]
    fn from_greater_than_to_is_rejected() {
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let err = run(&cache, &provenance, "qm", 0.9, 0.4, 0.1).unwrap_err();
        assert!(format!("{err:#}").contains("must be <="), "got: {err:#}");
    }

    #[test]
    fn threshold_values_round_trip_cleanly_through_serde() {
        let dir = TempDir::new().unwrap();
        let (cache, provenance) = write_committed_shape_pair(dir.path(), &[0.5], 0.0);
        let report = run(&cache, &provenance, "qm", 0.40, 0.99, 0.01).unwrap();
        // The integer-tick walk must emit threshold values whose
        // f64::to_string() form is short (e.g. "0.5", "0.41"), not
        // "0.41000000000000003". Spot-check the boundary points.
        let first = format!("{}", report.rows.first().unwrap().threshold);
        let mid = report
            .rows
            .iter()
            .find(|r| (r.threshold - 0.50).abs() < f64::EPSILON)
            .unwrap();
        let mid_text = format!("{}", mid.threshold);
        assert_eq!(first, "0.4", "expected clean 0.4, got {first}");
        assert_eq!(mid_text, "0.5", "expected clean 0.5, got {mid_text}");
    }

    #[test]
    fn locked_threshold_lies_in_default_sweep_range() {
        // Cross-check that the audit's pinned threshold lies inside
        // the default sweep range. If a future revision raises the
        // locked threshold above DEFAULT_TO (or drops it below
        // DEFAULT_FROM), the default sweep would no longer produce
        // a row at the locked value and this test catches that.
        use crate::semantic_cache_provenance::EXPECTED_THRESHOLD;
        assert!(
            EXPECTED_THRESHOLD >= DEFAULT_FROM,
            "locked threshold {EXPECTED_THRESHOLD} is below default sweep lower bound {DEFAULT_FROM}"
        );
        assert!(
            EXPECTED_THRESHOLD <= DEFAULT_TO,
            "locked threshold {EXPECTED_THRESHOLD} is above default sweep upper bound {DEFAULT_TO}"
        );
        // Also assert the locked value is representable as a
        // multiple of the default step so a row at exactly the
        // locked value appears in the default sweep.
        let scaled = (EXPECTED_THRESHOLD * SCALE_FACTOR).round() as i64;
        let step_ticks = (DEFAULT_STEP * SCALE_FACTOR).round() as i64;
        assert_eq!(
            scaled % step_ticks,
            0,
            "locked threshold {EXPECTED_THRESHOLD} is not a clean multiple of default step {DEFAULT_STEP}"
        );
    }

    /// Static cross-check that the frozen semantic cache's locked
    /// threshold remains the Rust audit's `EXPECTED_THRESHOLD`.
    #[test]
    fn frozen_threshold_matches_committed_provenance() {
        use crate::semantic_cache_provenance::EXPECTED_THRESHOLD;
        assert!(
            (EXPECTED_THRESHOLD - 0.5).abs() < f64::EPSILON,
            "EXPECTED_THRESHOLD must be 0.5 (the locked decision threshold \
             from the frozen semantic cache). Got {EXPECTED_THRESHOLD}."
        );
    }
}
