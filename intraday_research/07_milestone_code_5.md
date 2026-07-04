

All six documents are attached in full, so I worked directly from them — no fetching needed. M4's hand-off names two M5 candidates: the signed Tiger venue and `hkq-validate`. I'm deliberately re-sequencing: **validate ships first**. The reason is not taste but protocol: the kill switch still has exactly one real producer (the operator console), and the report's §4 is explicit that decay is an *operational event* with a pre-registered kill threshold — routing a single real order through a signed venue before the CUSUM producer exists would put capital behind a system with no immune response. Meanwhile the paper tier from M4 only *counts* as the report's mandated shadow period if its output is actually monitored: the Attribution/Scores/Fills partitions the engine started writing at PostClose are currently write-only. And every deliverable here is pure math over the lake — zero vendor-verification surface — which keeps the milestone discipline intact (the venue is *all* verification surface, and it lands next, plugging into an already-guarded system).

# Milestone 5 — The Statistical Immune System (`hkq-validate` + kill-switch closure)

**Why this is next.** M1–M4 built a machine that trades; nothing yet decides whether it *should keep trading*. Three loops are dangling: the CUSUM monitor (the kill switch's second producer, blueprint topology), the §4 promotion gates (purged CV, NW t-stats, DSR against an honest trials registry), and the quarterly estimation jobs whose outputs the engine has been consuming as priors since M2 (identity `RegimeGate`, θ₁=θ₂=1, v* prior, ρ=1 AH betas). All of their *inputs* now exist and grow daily — live-frozen `Scores`, `Fills`, `Attribution`, `MainlandPrints` — precisely because M4 persisted what cannot be recomputed. M5 closes those loops as pure, synchronously-tested functions plus two thin binaries' worth of wiring.

**In scope:** the `hkq-validate` crate (purged walk-forward splits with embargo; Newey–West t; skew/kurtosis/block-bootstrap Sharpe CIs; Deflated Sharpe with expected-max-SR under N trials; the latching CUSUM state machine with persistent state and idempotent replay; the hash-chained append-only trials registry that makes DSR's N honest; the composite-score daily rank-IC series from `Scores`⨝realized; daily PnL from `Fills`; quarterly fits for θ₁/θ₂, v*, the IVU regime-gate matrix, and per-name AH betas), the `hkq-validate` binary (`cusum` / `fit-quarterly` / `report` jobs), CUSUM wired into `hkq-live` as the kill switch's second producer (a startup gate: the IC row for today only exists at PostClose, so a breach is decidable — and latched — before any order intent), and the engine's one marked landing point activated: PreMarket now loads `_state/regime_gate.json` exactly as it loads the alpha map. **Deferred:** the signed Tiger venue + partial-fill pacer accounting (M6 — now unblocked *and* gated), backtest replay (M7), VHSI ingestion (unchanged typed degradation), per-sector A50 betas (no persisted A50 stream exists yet — a data-milestone fact), Hansen SPA (needs benchmark return series that aren't data yet), and CPCV (lands with its first model-selection consumer).

Engineering decisions beyond the blueprint sketch, briefly. The CUSUM monitors the **composite score's** daily rank IC recomputed from the persisted `Scores` + `Bars1m` label window — not a new column in the attribution row — so M4's engine stays byte-identical and the monitor's input is exactly what the AlphaMap already trains on; the state is a JSON file with a `last_date` watermark (idempotent re-runs, deterministic full-history replay on first arm) and a **latched** breach: only an operator edit un-halts, which is what "pre-registered kill" means. Pre-registration itself is materialized as a best-effort registry record of {μ₀, k, h} the first time the monitor arms — the registry, not a memo, is the audit trail. Registry records are hash-chained (SHA-1 over previous-hash + canonical record) so N in the DSR can't be quietly shrunk by deleting lines; `verify_chain` is a job, not a promise. Config follows the `[sources]` precedent: a crate-owned `[validate]` table parsed from the same TOML — **no new `StrategyCfg` fields**, because M3/M4 construct that struct literally in sibling tests and the type contract is load-bearing. Fit outputs split by consumption mechanism: the gate matrix has a typed in-engine slot (`NightlyState.gate`), so the fit writes `_state/regime_gate.json` and PreMarket auto-loads it (corrupt ⇒ identity + loud error); scalar recommendations (θ₁/θ₂, v*) are *reported*, registry-logged, and promoted by the operator editing `strategy.toml` — config stays the single source of truth, per hkq-core's own rule. The gate fit is mechanized as per-tercile pooled rank-ICs (date-demeaned labels), mean-one normalized and clamped to [0,2] — a documented simplification of the report's threshold regression, isolated in one function like σ15m was. v* is chosen by grid-search over `vs_auct_i` quantiles maximizing the mean daily IC of the reconstructed X1, evaluated with purged walk-forward: per-fold argmax on train, OOS IC on test, recommendation = median of fold argmaxes — the splits module is used in anger, not shipped as decoration. Strategy Sharpe for the `report` job comes from `Fills`-derived daily realized PnL (the book is flat daily by construction, so per-day realized PnL is complete, and SR is invariant to the constant-equity normalization). Everything degrades honestly: no scores history ⇒ `Insufficient` and the live gate logs-and-continues (cold-start shadow must not be blocked by its own monitor); schema drift ⇒ loud polars errors, never defaults.

```text
hkq/
├── Cargo.toml                        (updated: member)
└── crates/
    ├── hkq-data/src/lake.rs          (append: Lake::root accessor)
    ├── hkq-engine/src/premarket.rs   (append: gate_state_path/load_gate; 1-line swap)
    ├── hkq-validate/
    │   ├── Cargo.toml
    │   └── src/{lib,error,cfg,splits,stats,dsr,cusum,ic,pnl,registry,fits,main}.rs
    └── hkq-live/
        ├── Cargo.toml                (updated: hkq-validate)
        └── src/main.rs               (updated: CUSUM startup gate — second producer)
```

## Workspace

```toml
# Cargo.toml
[workspace]
resolver = "2"
members = [
  "crates/hkq-core", "crates/hkq-data", "crates/hkq-factors",
  "crates/hkq-signal", "crates/hkq-risk", "crates/hkq-exec",
  "crates/hkq-engine", "crates/hkq-validate", "crates/hkq-nightly",
  "crates/hkq-live",
]

[workspace.package]
edition = "2021"
rust-version = "1.83"

[workspace.dependencies]
tokio        = { version = "1.38", features = ["full"] }
tokio-stream = "0.1"
reqwest      = { version = "0.12", default-features = false, features = ["json", "gzip", "rustls-tls"] }
serde        = { version = "1", features = ["derive"] }
serde_json   = "1"
# Feature set unchanged since M2 — M5 adds no new expression surface.
polars       = { version = "0.46", features = [
  "lazy", "parquet", "dtype-date", "dtype-datetime",
  "ewma", "rank", "log", "abs", "rolling_window", "clip", "sign",
  "is_in", "round_series", "pct_change", "cum_agg", "partition_by",
  "semi_anti_join", "temporal", "dynamic_group_by",
] }
rust_decimal        = { version = "1.35", features = ["serde-str"] }
rust_decimal_macros = "1.35"
chrono       = { version = "0.4", features = ["serde"] }
chrono-tz    = "0.9"
nalgebra     = "0.33"
thiserror    = "1"
anyhow       = "1"
async-trait  = "0.1"
futures      = "0.3"
governor     = "0.6"
tracing      = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
rsa          = { version = "0.9", features = ["sha1"] }
sha1         = { version = "0.10", features = ["oid"] }
base64       = "0.22"
toml         = "0.8"
```

## Surgical patches to frozen crates

Two append-blocks and one line, in the M4 style — everything else in M1–M4 stays byte-identical.

```rust
// (append inside crates/hkq-data/src/lake.rs `impl Lake`)
    /// Root accessor. Validate-owned state files live under `<root>/_state`
    /// (alpha_map.json, regime_gate.json, cusum.json, trials.jsonl, ah_beta.parquet).
    pub fn root(&self) -> &Path { &self.root }
```

```rust
// (append inside crates/hkq-engine/src/premarket.rs, alongside alpha_state_path/load_alpha)
/// Contract shared with hkq-validate's quarterly fit job: the gate matrix lands at
/// `<lake_root>/_state/regime_gate.json`. (Duplicated constant by design — the
/// dependency arrow runs live-binary → {engine, validate}, never between them.)
pub fn gate_state_path(lake_root: &Path) -> PathBuf {
    lake_root.join("_state").join("regime_gate.json")
}

/// X4 gate (§3.3): identity until hkq-validate's first threshold fit (M2's
/// documented cold start). Corrupt file ⇒ identity + loud error; the engine runs.
fn load_gate(lake_root: &Path) -> RegimeGate {
    let path = gate_state_path(lake_root);
    match std::fs::read(&path) {
        Ok(bytes) => match serde_json::from_slice::<RegimeGate>(&bytes) {
            Ok(g) => { tracing::info!(path = %path.display(), "regime gate loaded"); g }
            Err(e) => {
                tracing::error!(error = %e, "regime gate corrupt; IDENTITY fallback");
                RegimeGate::default()
            }
        },
        Err(_) => { tracing::info!("no regime gate state: identity (cold start)"); RegimeGate::default() }
    }
}
```

And in `NightlyState::load`, replace the single line

```rust
            weights_s, weights_x, alpha, gate: RegimeGate::default(),
```

with

```rust
            weights_s, weights_x, alpha, gate: load_gate(&cfg.ops.lake_root),
```

(`RegimeGate`, `Path`, `serde_json`, and `tracing` are already imported in `premarket.rs`; the existing PreMarket test passes unchanged — no state file in its temp root means the identity path.)

## `hkq-validate`

```toml
# crates/hkq-validate/Cargo.toml
[package]
name = "hkq-validate"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-data = { path = "../hkq-data" }
hkq-factors = { path = "../hkq-factors" }
hkq-signal = { path = "../hkq-signal" }
hkq-risk = { path = "../hkq-risk" }
polars.workspace = true
nalgebra.workspace = true
chrono.workspace = true
chrono-tz.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
tracing.workspace = true
sha1.workspace = true
toml.workspace = true
anyhow.workspace = true
tracing-subscriber.workspace = true
```

```rust
// crates/hkq-validate/src/lib.rs
#![forbid(unsafe_code)]
//! The §4 protocol as a crate, not a notebook: purged splits, NW t-stats, DSR
//! against an honest trials registry, the CUSUM kill producer, and the quarterly
//! estimation jobs whose outputs the engine has consumed as priors since M2.
//!
//! Design invariants:
//! - Every statistic is a pure function of frames/slices; the ONLY I/O is the
//!   lake (read), the `_state` directory (fit artifacts, CUSUM state), and the
//!   hash-chained trials registry (append).
//! - Monitoring consumes the SAME persisted artifacts the learning loops train
//!   on (Scores ⨝ realized label window): no recomputed morning factors, no
//!   fabricated ICs — the M3/M4 honesty rule extended to the monitor.
//! - Degradation is typed: missing history ⇒ `Insufficient` (callers continue,
//!   loudly); schema drift ⇒ polars errors (nobody continues).
//! - The CUSUM breach LATCHES. Un-halting is an operator edit of the state
//!   file, never code. That is what "pre-registered kill threshold" means.

pub mod cfg;
pub mod cusum;
pub mod dsr;
pub mod error;
pub mod fits;
pub mod ic;
pub mod pnl;
pub mod registry;
pub mod splits;
pub mod stats;

pub use cfg::{load_validate, ValidateCfg};
pub use cusum::{startup_gate, CusumOutcome, CusumParams, CusumState};
pub use dsr::{deflated_sharpe, expected_max_sharpe};
pub use error::ValidateError;
pub use registry::TrialsRegistry;
pub use splits::{purged_walk_forward, Split};
```

```rust
// crates/hkq-validate/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidateError {
    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),
    #[error(transparent)]
    Data(#[from] hkq_data::error::DataError),
    #[error(transparent)]
    Factor(#[from] hkq_factors::FactorError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("state serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("config: {0}")]
    Config(String),
    #[error("input contract violated: {0}")]
    Contract(&'static str),
    #[error("insufficient data: {0}")]
    Insufficient(&'static str),
    #[error("validate state: {0}")]
    State(String),
}
```

```rust
// crates/hkq-validate/src/cfg.rs
//! `[validate]` table of strategy.toml — crate-owned config per the hkq-data
//! `[sources]` precedent. Deliberately NOT a StrategyCfg field: M3/M4 construct
//! that struct literally in sibling tests, and the type contract is load-bearing.
//!
//! The CUSUM triple (mu0, k, h) is the PRE-REGISTERED kill rule (§4). It lives
//! in version-controlled config; the registry records it when first armed.
use crate::cusum::CusumParams;
use crate::error::ValidateError;
use serde::Deserialize;
use std::path::{Path, PathBuf};

fn d_mu0() -> f64 { 0.02 }
fn d_k() -> f64 { 0.005 }
fn d_h() -> f64 { 0.15 }
fn d_window() -> i64 { 550 }     // calendar days ≈ 2y trading (θ/v*/gate window, §3.2)
fn d_min_obs() -> usize { 120 }
fn d_grid() -> usize { 17 }
fn d_folds() -> usize { 5 }
fn d_embargo() -> usize { 5 }    // §4: 5-day embargo

#[derive(Debug, Clone, Deserialize)]
pub struct ValidateCfg {
    /// Healthy-regime mean daily rank IC (CUSUM reference μ₀).
    #[serde(default = "d_mu0")]
    pub cusum_mu0: f64,
    /// CUSUM slack k — half the IC shift the monitor is tuned to detect.
    #[serde(default = "d_k")]
    pub cusum_k: f64,
    /// Pre-registered kill threshold h: S_t > h ⇒ HALT, latched.
    #[serde(default = "d_h")]
    pub cusum_h: f64,
    #[serde(default = "d_window")]
    pub fit_window_days: i64,
    #[serde(default = "d_min_obs")]
    pub fit_min_obs: usize,
    #[serde(default = "d_grid")]
    pub vstar_grid: usize,
    #[serde(default = "d_folds")]
    pub cv_folds: usize,
    #[serde(default = "d_embargo")]
    pub cv_embargo: usize,
    /// Defaults to `<lake_root>/_state/trials.jsonl`.
    #[serde(default)]
    pub trials_registry_path: Option<PathBuf>,
}

impl Default for ValidateCfg {
    fn default() -> Self {
        Self {
            cusum_mu0: d_mu0(), cusum_k: d_k(), cusum_h: d_h(),
            fit_window_days: d_window(), fit_min_obs: d_min_obs(),
            vstar_grid: d_grid(), cv_folds: d_folds(), cv_embargo: d_embargo(),
            trials_registry_path: None,
        }
    }
}

impl ValidateCfg {
    pub fn cusum_params(&self) -> CusumParams {
        CusumParams { mu0: self.cusum_mu0, k: self.cusum_k, h: self.cusum_h }
    }

    pub fn registry_path(&self, lake_root: &Path) -> PathBuf {
        self.trials_registry_path.clone()
            .unwrap_or_else(|| lake_root.join("_state").join("trials.jsonl"))
    }
}

#[derive(Debug, Deserialize)]
struct ValidateFile {
    validate: Option<ValidateCfg>,
}

/// Absent `[validate]` table ⇒ documented defaults (the report's priors).
pub fn load_validate(path: impl AsRef<Path>) -> Result<ValidateCfg, ValidateError> {
    let raw = std::fs::read_to_string(path.as_ref())?;
    let f: ValidateFile = toml::from_str(&raw)
        .map_err(|e| ValidateError::Config(format!("{}: {e}", path.as_ref().display())))?;
    Ok(f.validate.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_when_table_absent() {
        let f: ValidateFile = toml::from_str("[ops]\nx = 1\n").unwrap();
        let v = f.validate.unwrap_or_default();
        assert!((v.cusum_mu0 - 0.02).abs() < 1e-12);
        assert_eq!(v.cv_embargo, 5);
        assert!(v.registry_path(Path::new("/lake")).ends_with("_state/trials.jsonl"));
    }
}
```

```rust
// crates/hkq-validate/src/splits.rs
//! §4 purged walk-forward: labels span `horizon` days, so training must end
//! `embargo + horizon` before the test block starts — purge removes label
//! overlap, the embargo kills slower leakage (serially-correlated features).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Split {
    pub train: std::ops::Range<usize>,
    pub test: std::ops::Range<usize>,
}

/// Deviation from the blueprint sketch, documented: the LAST fold absorbs the
/// `n % n_folds` remainder instead of silently dropping those days. Folds whose
/// purged training range would be empty are skipped, not emitted degenerate.
pub fn purged_walk_forward(
    n_days: usize, n_folds: usize, embargo: usize, horizon: usize,
) -> Vec<Split> {
    if n_folds < 2 || n_days == 0 {
        return vec![];
    }
    let fold = n_days / n_folds;
    if fold == 0 {
        return vec![];
    }
    let mut out = Vec::new();
    for k in 1..n_folds {
        let test_start = k * fold;
        let test_end = if k == n_folds - 1 { n_days } else { (k + 1) * fold };
        let train_end = test_start.saturating_sub(embargo + horizon);
        if train_end == 0 {
            continue;
        }
        out.push(Split { train: 0..train_end, test: test_start..test_end });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gap_and_coverage() {
        let s = purged_walk_forward(100, 5, 5, 1);
        assert_eq!(s.len(), 4);
        for sp in &s {
            assert!(sp.train.end + 5 + 1 <= sp.test.start); // purge + embargo respected
            assert!(!sp.train.is_empty() && !sp.test.is_empty());
        }
        assert_eq!(s[0].test, 20..40);
        assert_eq!(s[0].train, 0..14);
        assert_eq!(s.last().unwrap().test, 80..100); // remainder absorbed
    }

    #[test]
    fn degenerate_inputs() {
        assert!(purged_walk_forward(0, 5, 5, 1).is_empty());
        assert!(purged_walk_forward(10, 1, 0, 0).is_empty());
        assert!(purged_walk_forward(3, 5, 0, 0).is_empty()); // fold == 0
        // Embargo so large the early folds have no training data: skipped.
        let s = purged_walk_forward(100, 5, 50, 1);
        assert!(s.iter().all(|sp| sp.train.end >= 1));
        assert!(s.len() < 4);
    }
}
```

```rust
// crates/hkq-validate/src/stats.rs
//! Shared statistical primitives (§4): NW t, moments, Φ, block-bootstrap Sharpe.
//! Total functions: degenerate inputs return None, never panic — house rule.

pub fn mean_sd(v: &[f64]) -> (f64, f64) {
    let n = v.len().max(1) as f64;
    let m = v.iter().sum::<f64>() / n;
    let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>()
        / (v.len().saturating_sub(1)).max(1) as f64)
        .sqrt();
    (m, sd)
}

/// Daily Sharpe (mean/sd). Scale-invariant to constant equity normalization —
/// why Fills-derived PnL (HKD) is a valid SR input for a flat-daily book.
pub fn sharpe(v: &[f64]) -> Option<f64> {
    if v.len() < 2 {
        return None;
    }
    let (m, sd) = mean_sd(v);
    (sd > 0.0 && sd.is_finite()).then(|| m / sd)
}

pub fn skewness(v: &[f64]) -> Option<f64> {
    if v.len() < 3 {
        return None;
    }
    let (m, sd) = mean_sd(v);
    if !(sd > 0.0) {
        return None;
    }
    let n = v.len() as f64;
    Some(v.iter().map(|x| ((x - m) / sd).powi(3)).sum::<f64>() / n)
}

/// NON-excess kurtosis (normal ⇒ 3), the convention the DSR formula expects.
pub fn kurtosis(v: &[f64]) -> Option<f64> {
    if v.len() < 4 {
        return None;
    }
    let (m, sd) = mean_sd(v);
    if !(sd > 0.0) {
        return None;
    }
    let n = v.len() as f64;
    Some(v.iter().map(|x| ((x - m) / sd).powi(4)).sum::<f64>() / n)
}

/// Newey–West t-stat of the mean (lag-window `lags`, Bartlett weights) — the §4
/// significance statistic for daily rank-IC series. Non-finite values are
/// dropped; needs ≥ 8 observations and positive long-run variance.
pub fn newey_west_t(x: &[f64], lags: usize) -> Option<f64> {
    let e: Vec<f64> = x.iter().copied().filter(|v| v.is_finite()).collect();
    let n = e.len();
    if n < 8 {
        return None;
    }
    let nf = n as f64;
    let m = e.iter().sum::<f64>() / nf;
    let d: Vec<f64> = e.iter().map(|v| v - m).collect();
    let mut s = d.iter().map(|v| v * v).sum::<f64>() / nf;
    for l in 1..=lags.min(n - 1) {
        let w = 1.0 - l as f64 / (lags as f64 + 1.0);
        let g = d.iter().zip(&d[l..]).map(|(a, b)| a * b).sum::<f64>() / nf;
        s += 2.0 * w * g;
    }
    (s > 0.0).then(|| m / (s / nf).sqrt())
}

/// Circular block bootstrap of a daily series → (point SR, 2.5%, 97.5%).
/// Deterministic xorshift64* PRNG: reproducible reports, no rand dependency.
pub fn block_bootstrap_sharpe_ci(
    x: &[f64], block: usize, n_boot: usize, seed: u64,
) -> Option<(f64, f64, f64)> {
    let n = x.len();
    if n < 2 * block.max(1) || block == 0 || n_boot < 40 {
        return None;
    }
    let point = sharpe(x)?;
    let mut s = seed | 1;
    let mut next = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut srs: Vec<f64> = Vec::with_capacity(n_boot);
    let mut buf = Vec::with_capacity(n);
    for _ in 0..n_boot {
        buf.clear();
        while buf.len() < n {
            let start = (next() % n as u64) as usize;
            for j in 0..block {
                if buf.len() == n {
                    break;
                }
                buf.push(x[(start + j) % n]);
            }
        }
        if let Some(sr) = sharpe(&buf) {
            srs.push(sr);
        }
    }
    if srs.len() < n_boot / 2 {
        return None;
    }
    srs.sort_by(|a, b| a.total_cmp(b));
    let q = |p: f64| srs[((p * (srs.len() - 1) as f64).round() as usize).min(srs.len() - 1)];
    Some((point, q(0.025), q(0.975)))
}

/// Standard normal CDF via Abramowitz–Stegun 7.1.26 erf (|ε| < 1.5e−7).
pub fn phi(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

fn erf(x: f64) -> f64 {
    let s = x.signum();
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let y = 1.0
        - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
            + 0.254829592)
            * t
            * (-x * x).exp();
    s * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nw_t_basics() {
        assert!(newey_west_t(&[1.0; 30], 5).is_none()); // zero variance
        let alt: Vec<f64> = (0..40).map(|i| if i % 2 == 0 { 1.0 } else { -1.0 }).collect();
        assert!(newey_west_t(&alt, 5).unwrap().abs() < 1e-9); // mean 0
        let drift: Vec<f64> = (0..60).map(|i| 0.05 + 0.01 * ((i % 7) as f64 - 3.0)).collect();
        assert!(newey_west_t(&drift, 5).unwrap() > 2.0);
        assert!(newey_west_t(&[1.0, f64::NAN, 1.0], 2).is_none()); // < 8 finite
    }

    #[test]
    fn moments_and_phi() {
        let sym = [-2.0, -1.0, 0.0, 1.0, 2.0];
        assert!(skewness(&sym).unwrap().abs() < 1e-12);
        assert!(kurtosis(&sym).unwrap() > 1.0);
        assert!((phi(0.0) - 0.5).abs() < 1e-9);
        assert!((phi(1.959964) - 0.975).abs() < 1e-4);
    }

    #[test]
    fn bootstrap_ci_brackets_point() {
        let x: Vec<f64> = (0..250)
            .map(|i| 0.001 + 0.01 * (((i * 2654435761u64 as usize) % 97) as f64 / 48.0 - 1.0))
            .collect();
        let (sr, lo, hi) = block_bootstrap_sharpe_ci(&x, 10, 200, 42).unwrap();
        assert!(lo <= sr && sr <= hi);
        assert!(lo < hi);
    }
}
```

```rust
// crates/hkq-validate/src/dsr.rs
//! Deflated Sharpe Ratio (§4): PSR against SR* = E[max SR] under N effective
//! trials. N and the SR dispersion MUST come from the trials registry — that is
//! the whole point; a hand-typed N is a lie the math cannot detect.
use crate::stats::phi;
use hkq_factors::xsec::norm_ppf;

const EULER: f64 = 0.577_215_664_901_532_9;

/// E[max SR] over `n_trials` independent trials with cross-trial SR variance
/// `var_trials` (Bailey–López de Prado). n < 2 is clamped: one trial deflates
/// nothing, which is exactly why the registry exists.
pub fn expected_max_sharpe(var_trials: f64, n_trials: usize) -> f64 {
    let n = n_trials.max(2) as f64;
    let sd = var_trials.max(0.0).sqrt();
    sd * ((1.0 - EULER) * norm_ppf(1.0 - 1.0 / n)
        + EULER * norm_ppf(1.0 - 1.0 / (n * std::f64::consts::E)))
}

/// DSR = Φ( (SR − SR*)·√(T−1) / √(1 − γ₃·SR + (γ₄−1)/4·SR²) ), γ₄ non-excess.
/// None on degenerate inputs (T < 2, non-finite, or non-positive denominator).
pub fn deflated_sharpe(
    sr: f64, t_obs: usize, skew: f64, kurt: f64, var_trials: f64, n_trials: usize,
) -> Option<f64> {
    if t_obs < 2 || !sr.is_finite() || !skew.is_finite() || !kurt.is_finite() {
        return None;
    }
    let sr_star = expected_max_sharpe(var_trials, n_trials);
    let den = 1.0 - skew * sr + (kurt - 1.0) / 4.0 * sr * sr;
    if !(den > 0.0) {
        return None;
    }
    Some(phi((sr - sr_star) * ((t_obs - 1) as f64).sqrt() / den.sqrt()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sr_star_grows_with_trials_and_dispersion() {
        let a = expected_max_sharpe(0.01, 5);
        let b = expected_max_sharpe(0.01, 50);
        let c = expected_max_sharpe(0.04, 50);
        assert!(b > a && c > b);
        assert!(expected_max_sharpe(0.0, 100).abs() < 1e-12);
    }

    #[test]
    fn dsr_anchors() {
        // At SR == SR*, the deflated probability is exactly one half.
        let sr_star = expected_max_sharpe(0.01, 10);
        let d = deflated_sharpe(sr_star, 252, 0.0, 3.0, 0.01, 10).unwrap();
        assert!((d - 0.5).abs() < 1e-9);
        // Strong SR well above SR* clears the 0.95 promotion gate.
        let d = deflated_sharpe(0.30, 252, 0.0, 3.0, 0.01, 10).unwrap();
        assert!(d > 0.95);
        // Degenerate denominator (absurd moments) is None, not a panic.
        assert!(deflated_sharpe(2.0, 252, 5.0, 1.0, 0.01, 10).is_none());
    }
}
```

```rust
// crates/hkq-validate/src/cusum.rs
//! The §4 CUSUM kill producer: one-sided CUSUM on the composite score's daily
//! rank IC. S_t = max(S_{t−1} + (μ₀ − IC_t) − k, 0); S > h ⇒ HALT, LATCHED.
//!
//! Operational shape: the IC row for day t exists only after PostClose, so the
//! intraday statistic is constant — a breach is decidable at STARTUP, before any
//! order intent. `startup_gate` is therefore the kill switch's second producer
//! (blueprint topology), running in hkq-live between PreMarket assembly and the
//! trading loop. State is persistent, watermarked (idempotent re-runs replay
//! nothing), and the breach survives restarts until an operator edits the file.
use crate::cfg::ValidateCfg;
use crate::error::ValidateError;
use crate::registry::TrialsRegistry;
use hkq_data::lake::Lake;
use hkq_risk::{HaltReason, KillSwitch};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub struct CusumParams {
    pub mu0: f64,
    pub k: f64,
    pub h: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CusumState {
    pub s: f64,
    pub last_date: Option<String>,
    pub breached: bool,
    pub breach_date: Option<String>,
}

impl CusumState {
    /// Apply one IC observation (caller guarantees date ordering). Returns the
    /// post-update breach flag. A latched breach never un-latches here.
    pub fn update(&mut self, p: &CusumParams, date: &str, ic: f64) -> bool {
        if self.breached {
            return true;
        }
        if ic.is_finite() {
            self.s = (self.s + (p.mu0 - ic) - p.k).max(0.0);
        }
        self.last_date = Some(date.to_string());
        if self.s > p.h {
            self.breached = true;
            self.breach_date = Some(date.to_string());
            tracing::error!(s = self.s, h = p.h, date, "CUSUM IC BREACH — latched");
        }
        self.breached
    }
}

/// Watermarked replay: only dates strictly after `last_date` are applied, in
/// order. Returns the number of new points consumed.
pub fn apply_series(
    state: &mut CusumState, p: &CusumParams, series: &[(String, f64)],
) -> usize {
    let mut n = 0usize;
    for (date, ic) in series {
        if state.breached {
            break;
        }
        if let Some(last) = &state.last_date {
            if date.as_str() <= last.as_str() {
                continue;
            }
        }
        state.update(p, date, *ic);
        n += 1;
    }
    n
}

pub fn state_path(lake_root: &Path) -> PathBuf {
    lake_root.join("_state").join("cusum.json")
}

/// Missing file ⇒ fresh state. CORRUPT file ⇒ hard error: silently resetting
/// the monitor would erase a possible latched breach — operator attention.
pub fn load_state(path: &Path) -> Result<CusumState, ValidateError> {
    match std::fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .map_err(|e| ValidateError::State(format!("corrupt cusum state {}: {e}", path.display()))),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(CusumState::default()),
        Err(e) => Err(e.into()),
    }
}

pub fn save_state(path: &Path, state: &CusumState) -> Result<(), ValidateError> {
    let dir = path.parent().ok_or(ValidateError::Contract("cusum state path has no parent"))?;
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(".cusum.tmp");
    std::fs::write(&tmp, serde_json::to_vec_pretty(state)?)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct CusumOutcome {
    pub s: f64,
    pub breached: bool,
    pub new_points: usize,
    pub last_date: Option<String>,
}

/// Recompute the score-IC series from the lake, advance the persistent state,
/// and report. First arming appends the pre-registration record (best-effort —
/// the authoritative pre-registration is the version-controlled config).
pub fn update_from_lake(lake: &Lake, vcfg: &ValidateCfg) -> Result<CusumOutcome, ValidateError> {
    let path = state_path(lake.root());
    let mut st = load_state(&path)?;
    let fresh = st.last_date.is_none() && !st.breached;
    let series = crate::ic::score_ic_series(lake)?;
    let p = vcfg.cusum_params();
    let n = apply_series(&mut st, &p, &series);
    save_state(&path, &st)?;
    if fresh && n > 0 {
        let reg = TrialsRegistry::open(vcfg.registry_path(lake.root()));
        let mut m = BTreeMap::new();
        m.insert("mu0".to_string(), p.mu0);
        m.insert("k".to_string(), p.k);
        m.insert("h".to_string(), p.h);
        if let Err(e) = reg.append("cusum_preregistration", "-", &m, "CUSUM first armed") {
            tracing::warn!(error = %e, "cusum pre-registration record failed (registry io)");
        }
    }
    Ok(CusumOutcome { s: st.s, breached: st.breached, new_points: n, last_date: st.last_date })
}

/// The live binary's startup gate — the SECOND kill-switch producer. A latched
/// or fresh breach flips the watch channel before the engine trades anything.
pub fn startup_gate(
    lake: &Lake, vcfg: &ValidateCfg, ks: &KillSwitch,
) -> Result<CusumOutcome, ValidateError> {
    let out = update_from_lake(lake, vcfg)?;
    if out.breached {
        ks.halt(HaltReason::CusumIcBreach);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latch_watermark_and_reset_floor() {
        let p = CusumParams { mu0: 0.0, k: 0.0, h: 0.5 };
        let mut st = CusumState::default();
        let series = vec![
            ("2026-07-01".to_string(), 1.0),  // healthy: s stays floored at 0
            ("2026-07-02".to_string(), -1.0), // s = 1.0 > h ⇒ breach, latched
            ("2026-07-03".to_string(), 9.0),  // never consumed (latched)
        ];
        let n = apply_series(&mut st, &p, &series);
        assert_eq!(n, 2);
        assert!(st.breached);
        assert_eq!(st.breach_date.as_deref(), Some("2026-07-02"));
        // Idempotent replay: nothing new, breach persists.
        let n = apply_series(&mut st, &p, &series);
        assert_eq!(n, 0);
        assert!(st.breached);
    }

    #[test]
    fn state_file_roundtrip_and_corrupt_is_loud() {
        let dir = std::env::temp_dir().join(format!(
            "hkq_cusum_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let path = dir.join("_state").join("cusum.json");
        assert!(load_state(&path).unwrap().last_date.is_none()); // missing ⇒ fresh
        let st = CusumState { s: 0.2, last_date: Some("2026-07-03".into()),
                              breached: false, breach_date: None };
        save_state(&path, &st).unwrap();
        let back = load_state(&path).unwrap();
        assert_eq!(back.last_date.as_deref(), Some("2026-07-03"));
        std::fs::write(&path, b"{ not json").unwrap();
        assert!(load_state(&path).is_err()); // corrupt ⇒ operator, not default
        std::fs::remove_dir_all(dir).ok();
    }
}
```

```rust
// crates/hkq-validate/src/ic.rs
//! The composite score's daily rank IC — the CUSUM's input — recomputed from
//! the SAME persisted artifacts the AlphaMap trains on: live-frozen Scores and
//! the §3.5 label window from Bars1m. No recomputed morning factors (M3 rule).
use crate::error::ValidateError;
use hkq_core::session::SessionTimes;
use hkq_data::lake::{Dataset, Lake};
use hkq_factors::cols::{self, base};
use hkq_factors::icir::spearman_ic;
use hkq_signal::attribution::{realized_window_returns, R_FWD};
use polars::prelude::*;

/// Pure core: joined frame [date, score, r_fwd] → sorted (date, IC) series.
/// Days with a degenerate cross-section (< 3 valid pairs) drop out honestly.
pub fn ic_from_joined(joined: &DataFrame) -> Result<Vec<(String, f64)>, ValidateError> {
    if joined.height() == 0 {
        return Err(ValidateError::Insufficient("empty scores⨝realized panel"));
    }
    let parts = joined.partition_by([base::DATE], true)?;
    let mut out = Vec::with_capacity(parts.len());
    for p in parts {
        let date = p.column(base::DATE)?.as_materialized_series().str()?
            .get(0).unwrap_or_default().to_string();
        let s = p.column(cols::SCORE)?.as_materialized_series().f64()?.clone();
        let f = p.column(R_FWD)?.as_materialized_series().f64()?.clone();
        let pred: Vec<f64> = (0..p.height()).map(|i| s.get(i).unwrap_or(f64::NAN)).collect();
        let fwd: Vec<f64> = (0..p.height()).map(|i| f.get(i).unwrap_or(f64::NAN)).collect();
        if let Some(ic) = spearman_ic(&pred, &fwd) {
            out.push((date, ic));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Lake wrapper. Missing datasets ⇒ `Insufficient` (cold-start shadow must not
/// be blocked by its own monitor); schema drift inside partitions stays LOUD.
pub fn score_ic_series(lake: &Lake) -> Result<Vec<(String, f64)>, ValidateError> {
    let s = SessionTimes::get();
    let scores = lake.scan(Dataset::Scores)
        .map_err(|_| ValidateError::Insufficient("no scores history (shadow not started?)"))?
        .select([col(base::DATE), col(base::CODE), col(cols::SCORE)]);
    let bars = lake.scan(Dataset::Bars1m)
        .map_err(|_| ValidateError::Insufficient("no bars_1m history (run hkq-nightly)"))?;
    let realized = realized_window_returns(bars, s.entry, s.exit_end);
    let joined = scores
        .join(realized, [col(base::CODE), col(base::DATE)],
              [col(base::CODE), col(base::DATE)], JoinArgs::new(JoinType::Inner))
        .collect()?;
    ic_from_joined(&joined)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    use hkq_core::session::hk;
    use polars::df;

    fn ts(d: NaiveDate, h: u32, m: u32) -> i64 {
        hk(d, NaiveTime::from_hms_opt(h, m, 0).unwrap()).timestamp_millis()
    }

    #[test]
    fn pure_core_signs() {
        let joined = df!(
            "date" => vec!["2026-07-02", "2026-07-02", "2026-07-02",
                           "2026-07-03", "2026-07-03", "2026-07-03"],
            "score" => vec![1.0, 2.0, 3.0, 1.0, 2.0, 3.0],
            "r_fwd" => vec![0.01, 0.02, 0.03, 0.03, 0.02, 0.01],
        ).unwrap();
        let s = ic_from_joined(&joined).unwrap();
        assert_eq!(s.len(), 2);
        assert!((s[0].1 - 1.0).abs() < 1e-12);
        assert!((s[1].1 + 1.0).abs() < 1e-12);
    }

    #[test]
    fn lake_wrapper_roundtrip() {
        let root = std::env::temp_dir().join(format!(
            "hkq_ic_test_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let lake = Lake::new(&root);
        let d1 = NaiveDate::from_ymd_opt(2026, 7, 2).unwrap();
        let d2 = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        for (d, up) in [(d1, true), (d2, false)] {
            let mut scores = df!(
                "date" => vec![d.to_string(); 3],
                "code" => vec![1u32, 2, 3],
                "score" => vec![1.0, 2.0, 3.0],
            ).unwrap();
            lake.write_partition(Dataset::Scores, d, &mut scores, "test", 1).unwrap();
            // Bars: flat at 09:45, ordered/anti-ordered closes at 15:44.
            let last = if up { [101.0, 102.0, 103.0] } else { [103.0, 102.0, 101.0] };
            let mut bars = df!(
                "code" => vec![1u32, 2, 3, 1, 2, 3],
                "date" => vec![d.to_string(); 6],
                "ts_ms" => vec![ts(d, 9, 45); 3].into_iter()
                    .chain(vec![ts(d, 15, 44); 3]).collect::<Vec<_>>(),
                "c" => vec![100.0, 100.0, 100.0, last[0], last[1], last[2]],
            ).unwrap();
            lake.write_partition(Dataset::Bars1m, d, &mut bars, "test", 1).unwrap();
        }
        let s = score_ic_series(&lake).unwrap();
        assert_eq!(s.len(), 2);
        assert!((s[0].1 - 1.0).abs() < 1e-9 && (s[1].1 + 1.0).abs() < 1e-9);

        // End-to-end: the startup gate arms, breaches on day 2, halts the switch.
        let vcfg = crate::cfg::ValidateCfg {
            cusum_mu0: 0.0, cusum_k: 0.0, cusum_h: 0.5, ..Default::default()
        };
        let (ks, _rx) = hkq_risk::KillSwitch::new();
        let out = crate::cusum::startup_gate(&lake, &vcfg, &ks).unwrap();
        assert!(out.breached && ks.current().halted());
        // Idempotent second run: nothing new, still latched.
        let (ks2, _rx2) = hkq_risk::KillSwitch::new();
        let out = crate::cusum::startup_gate(&lake, &vcfg, &ks2).unwrap();
        assert_eq!(out.new_points, 0);
        assert!(out.breached && ks2.current().halted());
        std::fs::remove_dir_all(root).ok();
    }
}
```

```rust
// crates/hkq-validate/src/pnl.rs
//! Daily realized PnL from the Fills dataset. The book is flat every day by
//! construction (M4 terminal invariant), so per-day realized PnL is complete:
//! Σ(sell notional − buy notional − duty − fees). SR on this HKD series equals
//! SR on returns under constant equity — sufficient for the §4 report gates.
use crate::error::ValidateError;
use hkq_data::lake::{Dataset, Lake};
use hkq_factors::cols::base;
use polars::prelude::*;

pub fn pnl_from_fills_df(fills: DataFrame) -> Result<Vec<(String, f64)>, ValidateError> {
    let signed = when(col("side").eq(lit("sell")))
        .then(col("px") * col("shares").cast(DataType::Float64))
        .otherwise(lit(0.0) - col("px") * col("shares").cast(DataType::Float64));
    let df = fills.lazy()
        .group_by([col(base::DATE)])
        .agg([
            signed.sum().alias("__gross"),
            col("duty").sum().alias("__duty"),
            col("fees").sum().alias("__fees"),
        ])
        .with_column((col("__gross") - col("__duty") - col("__fees")).alias("pnl"))
        .sort_by_exprs([col(base::DATE)], Default::default())
        .select([col(base::DATE), col("pnl")])
        .collect()?;
    let dates = df.column(base::DATE)?.as_materialized_series().str()?.clone();
    let pnl = df.column("pnl")?.as_materialized_series().f64()?.clone();
    Ok((0..df.height())
        .filter_map(|i| Some((dates.get(i)?.to_string(), pnl.get(i)?)))
        .collect())
}

pub fn daily_pnl(lake: &Lake) -> Result<Vec<(String, f64)>, ValidateError> {
    let fills = lake.scan(Dataset::Fills)
        .map_err(|_| ValidateError::Insufficient("no fills history"))?
        .collect()?;
    pnl_from_fills_df(fills)
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    #[test]
    fn round_trip_pnl_hand_math() {
        let fills = df!(
            "code" => vec![700u32; 2],
            "date" => vec!["2026-07-03"; 2],
            "ts_ms" => vec![1i64, 2],
            "side" => vec!["buy".to_string(), "sell".to_string()],
            "shares" => vec![1_000u64, 1_000],
            "px" => vec![10.0, 11.0],
            "duty" => vec![10.0, 11.0],
            "fees" => vec![1.1, 1.21],
        ).unwrap();
        let out = pnl_from_fills_df(fills).unwrap();
        assert_eq!(out.len(), 1);
        // 11_000 − 10_000 − 21 duty − 2.31 fees
        assert!((out[0].1 - 976.69).abs() < 1e-9);
    }
}
```

```rust
// crates/hkq-validate/src/registry.rs
//! Append-only, hash-chained trials registry (§4): every evaluated variant is a
//! record, so the N in DSR is honest. JSONL; each record's hash = SHA1(prev_hash
//! ‖ canonical-json(record with hash="")) — deleting or editing any line breaks
//! `verify_chain`. Not cryptographically unforgeable; loudly tamper-EVIDENT.
use crate::error::ValidateError;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trial {
    pub seq: u64,
    pub ts_utc: String,
    pub kind: String,
    pub config_hash: String,
    pub metrics: BTreeMap<String, f64>,
    pub note: String,
    pub prev_hash: String,
    pub hash: String,
}

pub fn sha1_hex(bytes: &[u8]) -> String {
    Sha1::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

pub fn sha1_hex_of_file(path: impl AsRef<Path>) -> Result<String, ValidateError> {
    Ok(sha1_hex(&std::fs::read(path.as_ref())?))
}

fn record_hash(prev: &str, t: &Trial) -> Result<String, ValidateError> {
    let mut body = t.clone();
    body.hash = String::new();
    Ok(sha1_hex(format!("{prev}{}", serde_json::to_string(&body)?).as_bytes()))
}

pub struct TrialsRegistry {
    path: PathBuf,
}

impl TrialsRegistry {
    pub fn open(path: impl AsRef<Path>) -> Self {
        Self { path: path.as_ref().into() }
    }

    pub fn read_all(&self) -> Result<Vec<Trial>, ValidateError> {
        let raw = match std::fs::read_to_string(&self.path) {
            Ok(r) => r,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
            Err(e) => return Err(e.into()),
        };
        raw.lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).map_err(ValidateError::from))
            .collect()
    }

    pub fn n_trials(&self) -> Result<usize, ValidateError> {
        Ok(self.read_all()?.len())
    }

    pub fn append(
        &self, kind: &str, config_hash: &str, metrics: &BTreeMap<String, f64>, note: &str,
    ) -> Result<Trial, ValidateError> {
        let all = self.read_all()?;
        let prev = all.last().map(|t| t.hash.clone()).unwrap_or_else(|| "genesis".into());
        let mut t = Trial {
            seq: all.len() as u64,
            ts_utc: chrono::Utc::now().to_rfc3339(),
            kind: kind.to_string(),
            config_hash: config_hash.to_string(),
            metrics: metrics.clone(),
            note: note.to_string(),
            prev_hash: prev.clone(),
            hash: String::new(),
        };
        t.hash = record_hash(&prev, &t)?;
        if let Some(dir) = self.path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&self.path)?;
        writeln!(f, "{}", serde_json::to_string(&t)?)?;
        tracing::info!(kind, seq = t.seq, "trials registry appended");
        Ok(t)
    }

    /// Recompute the full chain; Err names the first broken record.
    pub fn verify_chain(&self) -> Result<usize, ValidateError> {
        let all = self.read_all()?;
        let mut prev = "genesis".to_string();
        for t in &all {
            if t.prev_hash != prev {
                return Err(ValidateError::State(format!("chain break at seq {}", t.seq)));
            }
            if record_hash(&prev, t)? != t.hash {
                return Err(ValidateError::State(format!("hash mismatch at seq {}", t.seq)));
            }
            prev = t.hash.clone();
        }
        Ok(all.len())
    }

    /// Cross-trial SR dispersion for the DSR: variance of the `sr` metric over
    /// all registered trials carrying one. None until ≥ 2 exist — you cannot
    /// deflate against a trial count you refuse to record.
    pub fn sr_dispersion(&self) -> Result<Option<(f64, usize)>, ValidateError> {
        let srs: Vec<f64> = self.read_all()?.iter()
            .filter_map(|t| t.metrics.get("sr").copied())
            .filter(|v| v.is_finite())
            .collect();
        if srs.len() < 2 {
            return Ok(None);
        }
        let (_, sd) = crate::stats::mean_sd(&srs);
        Ok(Some((sd * sd, srs.len())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!(
            "hkq_registry_{}.jsonl", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    #[test]
    fn append_verify_and_tamper_detection() {
        let path = tmp();
        let reg = TrialsRegistry::open(&path);
        let mut m = BTreeMap::new();
        m.insert("sr".to_string(), 1.0);
        reg.append("backtest", "abc", &m, "v1").unwrap();
        m.insert("sr".to_string(), 0.4);
        reg.append("backtest", "def", &m, "v2").unwrap();
        assert_eq!(reg.verify_chain().unwrap(), 2);
        assert_eq!(reg.n_trials().unwrap(), 2);
        let (var, n) = reg.sr_dispersion().unwrap().unwrap();
        assert_eq!(n, 2);
        assert!(var > 0.0);
        // Tamper with a recorded metric ⇒ chain verification fails loudly.
        let raw = std::fs::read_to_string(&path).unwrap().replace("0.4", "9.4");
        std::fs::write(&path, raw).unwrap();
        assert!(reg.verify_chain().is_err());
        std::fs::remove_file(path).ok();
    }
}
```

```rust
// crates/hkq-validate/src/fits.rs
//! Quarterly estimation jobs (§3.2/§3.3, report "estimation outputs, not
//! constants"). Pure fit functions over frames; thin lake-facing panel builders;
//! atomic artifact writers. Consumption split by mechanism: the gate matrix has
//! a typed engine slot (auto-loaded at PreMarket); scalar recommendations
//! (θ₁/θ₂, v*) are reported + registry-logged and promoted by the OPERATOR
//! editing strategy.toml — config remains the single source of truth.
use crate::error::ValidateError;
use chrono::NaiveDate;
use hkq_core::config::StrategyCfg;
use hkq_core::session::SessionTimes;
use hkq_data::lake::{Dataset, Lake};
use hkq_factors::cols::{self, base};
use hkq_factors::icir::spearman_ic;
use hkq_factors::moments::ewm_sigma_over;
use hkq_factors::panel::PanelBuilder;
use hkq_factors::sector::aggregate_sector_returns;
use hkq_factors::stage2::RegimeGate;
use hkq_signal::attribution::{realized_window_returns, R_FWD};
use nalgebra::{DMatrix, DVector};
use polars::prelude::*;
use std::path::{Path, PathBuf};

const E: f64 = std::f64::consts::E;
/// Gate fits below this many distinct dates are refused (noise, not regimes).
pub const GATE_MIN_DATES: usize = 40;

// ───────────────────────────── θ₁ / θ₂ (S1) ─────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct ThetaFit {
    pub theta1: f64,
    pub theta2: f64,
    pub n: usize,
}

/// Panel regression y = a + b₁·zid₁ + b₂·zon₁ pooled over (sector, date);
/// θ₁ = (b₁)₊, θ₂ = (−b₂)₊ — S1 carries the minus sign structurally (§3.2).
pub fn fit_theta(panel: &DataFrame, min_obs: usize) -> Result<Option<ThetaFit>, ValidateError> {
    let y = panel.column("y")?.as_materialized_series().f64()?.clone();
    let zid = panel.column("zid_1")?.as_materialized_series().f64()?.clone();
    let zon = panel.column("zon_1")?.as_materialized_series().f64()?.clone();
    let mut rows: Vec<(f64, f64, f64)> = Vec::with_capacity(panel.height());
    for i in 0..panel.height() {
        if let (Some(a), Some(b), Some(c)) = (y.get(i), zid.get(i), zon.get(i)) {
            if a.is_finite() && b.is_finite() && c.is_finite() {
                rows.push((a, b, c));
            }
        }
    }
    let n = rows.len();
    if n < min_obs.max(10) {
        return Ok(None);
    }
    let x = DMatrix::<f64>::from_row_iterator(n, 3, rows.iter().flat_map(|r| [1.0, r.1, r.2]));
    let yv = DVector::from_iterator(n, rows.iter().map(|r| r.0));
    let Ok(beta) = x.svd(true, true).solve(&yv, 1e-12) else { return Ok(None) };
    Ok(Some(ThetaFit { theta1: beta[1].max(0.0), theta2: (-beta[2]).max(0.0), n }))
}

/// [date, sector, y = r_id_t, zid_1, zon_1] from the lake: enriched members →
/// capped-weight sector returns → sector EWMA σ's → standardized t−1 regressors.
pub fn sector_s1_panel(
    lake: &Lake, cfg: &StrategyCfg, from: NaiveDate, to: NaiveDate,
) -> Result<DataFrame, ValidateError> {
    let static_path = cfg.ops.universe_static_path.as_ref()
        .ok_or(ValidateError::Contract("ops.universe_static_path required for sector fits"))?;
    let statics = LazyFrame::scan_parquet(
        static_path.to_string_lossy().as_ref(),
        ScanArgsParquet {
            hive_options: HiveOptions { enabled: Some(false), ..Default::default() },
            ..Default::default()
        },
    )?
    .select([col(base::CODE), col(cols::SECTOR), col(cols::FLOAT_CAP)])
    .collect()?;

    let pb = PanelBuilder::new(lake, &cfg.factors);
    let enriched = pb.enriched_daily(from, to, Some(statics))?.collect()?;
    let sector_daily = aggregate_sector_returns(enriched, cfg.stage1.member_weight_cap)?;

    let zid = when(col(cols::SIGMA_ID).gt(lit(0.0)))
        .then(col(cols::R_ID) / col(cols::SIGMA_ID))
        .otherwise(lit(NULL));
    let zon = when(col(cols::SIGMA_ON).gt(lit(0.0)))
        .then(col(cols::R_ON) / col(cols::SIGMA_ON))
        .otherwise(lit(NULL));
    Ok(sector_daily.lazy()
        .sort_by_exprs([col(cols::SECTOR), col(base::DATE)], Default::default())
        .with_columns([
            ewm_sigma_over(&cfg.factors, cols::SECTOR, cols::R_ON, cols::SIGMA_ON),
            ewm_sigma_over(&cfg.factors, cols::SECTOR, cols::R_ID, cols::SIGMA_ID),
        ])
        .with_columns([
            zid.shift(lit(1)).over([col(cols::SECTOR)]).alias("zid_1"),
            zon.shift(lit(1)).over([col(cols::SECTOR)]).alias("zon_1"),
        ])
        .select([col(base::DATE), col(cols::SECTOR), col(cols::R_ID).alias("y"),
                 col("zid_1"), col("zon_1")])
        .collect()?)
}

// ─────────────────────────────── v* (S2/X1) ─────────────────────────────────

#[derive(Debug, Clone)]
pub struct DateGroup {
    pub eps: Vec<f64>,
    pub vs: Vec<f64>,
    pub fwd: Vec<f64>,
}

/// Scores ⨝ realized label window: the raw material of the v* and gate fits.
/// Every column here was frozen live at 09:29:30/09:35 — no recomputation.
pub fn scores_realized_panel(lake: &Lake) -> Result<DataFrame, ValidateError> {
    let s = SessionTimes::get();
    let scores = lake.scan(Dataset::Scores)
        .map_err(|_| ValidateError::Insufficient("no scores history"))?
        .select([col(base::DATE), col(base::CODE), col(cols::EPS_GAP), col(cols::VS_AUCT_I),
                 col(cols::IVU_TERCILE), col(cols::X1), col(cols::X2), col(cols::X3),
                 col(cols::SCORE)]);
    let bars = lake.scan(Dataset::Bars1m)
        .map_err(|_| ValidateError::Insufficient("no bars_1m history"))?;
    let out = scores
        .join(realized_window_returns(bars, s.entry, s.exit_end),
              [col(base::CODE), col(base::DATE)],
              [col(base::CODE), col(base::DATE)], JoinArgs::new(JoinType::Inner))
        .collect()?;
    if out.height() == 0 {
        return Err(ValidateError::Insufficient("scores⨝realized panel empty"));
    }
    Ok(out)
}

pub fn date_groups(panel: &DataFrame) -> Result<Vec<(String, DateGroup)>, ValidateError> {
    let parts = panel.partition_by([base::DATE], true)?;
    let mut out = Vec::with_capacity(parts.len());
    for p in parts {
        let date = p.column(base::DATE)?.as_materialized_series().str()?
            .get(0).unwrap_or_default().to_string();
        let g = |n: &str| -> Result<Vec<f64>, ValidateError> {
            let ca = p.column(n)?.as_materialized_series().f64()?.clone();
            Ok((0..p.height()).map(|i| ca.get(i).unwrap_or(f64::NAN)).collect())
        };
        out.push((date, DateGroup {
            eps: g(cols::EPS_GAP)?, vs: g(cols::VS_AUCT_I)?, fwd: g(R_FWD)?,
        }));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Candidate thresholds: quantiles of the pooled finite VS distribution.
pub fn vstar_candidates(groups: &[(String, DateGroup)], n_grid: usize) -> Vec<f64> {
    let mut vs: Vec<f64> = groups.iter()
        .flat_map(|(_, g)| g.vs.iter().copied())
        .filter(|v| v.is_finite())
        .collect();
    vs.sort_by(|a, b| a.total_cmp(b));
    if vs.is_empty() || n_grid == 0 {
        return vec![];
    }
    let mut out: Vec<f64> = (1..=n_grid)
        .map(|i| vs[((i as f64 / (n_grid + 1) as f64) * (vs.len() - 1) as f64).round() as usize])
        .collect();
    out.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    out
}

/// Mean daily rank IC of the reconstructed X1(v) = ε·(φ·1{VS>v} − 1) (§3.3).
pub fn vstar_ic(groups: &[(String, DateGroup)], phi: f64, v: f64) -> Option<f64> {
    let mut ics = Vec::with_capacity(groups.len());
    for (_, g) in groups {
        let x1: Vec<f64> = g.eps.iter().zip(&g.vs)
            .map(|(e, vs)| e * (phi * f64::from(u8::from(*vs > v)) - 1.0))
            .collect();
        if let Some(ic) = spearman_ic(&x1, &g.fwd) {
            ics.push(ic);
        }
    }
    (ics.len() >= 3).then(|| ics.iter().sum::<f64>() / ics.len() as f64)
}

/// Grid argmax with a confirmed-cell mass guard (a threshold nobody crosses is
/// not a threshold). Returns (v*, mean IC at v*).
pub fn fit_vstar(
    groups: &[(String, DateGroup)], phi: f64, grid: &[f64],
) -> Option<(f64, f64)> {
    let mut best: Option<(f64, f64)> = None;
    for &v in grid {
        let confirmed: usize = groups.iter()
            .flat_map(|(_, g)| g.vs.iter())
            .filter(|vs| vs.is_finite() && **vs > v)
            .count();
        if confirmed < 20 {
            continue;
        }
        if let Some(ic) = vstar_ic(groups, phi, v) {
            if best.map_or(true, |(_, b)| ic > b) {
                best = Some((v, ic));
            }
        }
    }
    best
}

// ───────────────────────── IVU regime gate (X4) ─────────────────────────────

/// Per-tercile pooled rank-ICs on date-demeaned labels, mean-one normalized and
/// clamped to [0, 2]. A factor with no positive-IC tercile keeps its identity
/// column. DOCUMENTED SIMPLIFICATION of the report's threshold regression: the
/// terciles already discretize the threshold; this fits only the multipliers.
pub fn fit_regime_gate(panel: &DataFrame) -> Result<RegimeGate, ValidateError> {
    let df = panel.clone().lazy()
        .with_column((col(R_FWD) - col(R_FWD).mean().over([col(base::DATE)])).alias("__fwd_d"))
        .collect()?;
    let terc = df.column(cols::IVU_TERCILE)?.as_materialized_series().u32()?.clone();
    let fwd = df.column("__fwd_d")?.as_materialized_series().f64()?.clone();
    let mut g = [[1.0f64; 3]; 3];
    for (fi, fname) in [cols::X1, cols::X2, cols::X3].iter().enumerate() {
        let x = df.column(fname)?.as_materialized_series().f64()?.clone();
        let mut ics = [0.0f64; 3];
        for t in 0..3u32 {
            let (mut xs, mut ys) = (Vec::new(), Vec::new());
            for i in 0..df.height() {
                if terc.get(i) == Some(t) {
                    xs.push(x.get(i).unwrap_or(f64::NAN));
                    ys.push(fwd.get(i).unwrap_or(f64::NAN));
                }
            }
            ics[t as usize] = spearman_ic(&xs, &ys).unwrap_or(0.0).max(0.0);
        }
        let s: f64 = ics.iter().sum();
        if s > 1e-12 {
            for t in 0..3 {
                g[t][fi] = (3.0 * ics[t] / s).min(2.0);
            }
        }
    }
    Ok(RegimeGate { g })
}

// ─────────────────────────── AH betas (S6/X-link) ───────────────────────────

/// Per-name β of HK overnight return on the A-share 09:25 auction return —
/// replaces the ρ = 1 prior once ≥ min_obs prints exist. Clamped to [0, 2].
pub fn fit_ah_beta(joined: &DataFrame, min_obs: usize) -> Result<DataFrame, ValidateError> {
    let parts = joined.partition_by([base::CODE], true)?;
    let (mut codes, mut betas, mut ns) = (Vec::new(), Vec::new(), Vec::new());
    for p in parts {
        let code = p.column(base::CODE)?.as_materialized_series().u32()?
            .get(0).ok_or(ValidateError::Contract("null code in ah panel"))?;
        let xca = p.column(base::A_OPEN_RET)?.as_materialized_series().f64()?.clone();
        let yca = p.column(cols::R_ON)?.as_materialized_series().f64()?.clone();
        let mut xy: Vec<(f64, f64)> = Vec::with_capacity(p.height());
        for i in 0..p.height() {
            if let (Some(x), Some(y)) = (xca.get(i), yca.get(i)) {
                if x.is_finite() && y.is_finite() {
                    xy.push((x, y));
                }
            }
        }
        if xy.len() < min_obs.max(10) {
            continue;
        }
        let n = xy.len() as f64;
        let mx = xy.iter().map(|(x, _)| x).sum::<f64>() / n;
        let my = xy.iter().map(|(_, y)| y).sum::<f64>() / n;
        let (mut sxx, mut sxy) = (0.0, 0.0);
        for (x, y) in &xy {
            sxx += (x - mx) * (x - mx);
            sxy += (x - mx) * (y - my);
        }
        if sxx <= 1e-12 {
            continue;
        }
        codes.push(code);
        betas.push((sxy / sxx).clamp(0.0, 2.0));
        ns.push(xy.len() as u32);
    }
    Ok(df!(base::CODE => codes, "ah_beta" => betas, "n_obs" => ns)?)
}

/// MainlandPrints ⨝ adjusted HK overnight returns over the fit window.
pub fn ah_panel(lake: &Lake, from: NaiveDate, to: NaiveDate) -> Result<DataFrame, ValidateError> {
    let range = |lf: LazyFrame| lf.filter(
        col(base::DATE).gt_eq(lit(from.to_string()))
            .and(col(base::DATE).lt_eq(lit(to.to_string()))));
    let daily = range(lake.scan(Dataset::DailyBars)
        .map_err(|_| ValidateError::Insufficient("no daily_bars history"))?)
        .sort_by_exprs([col(base::CODE), col(base::DATE)], Default::default())
        .with_column((col(base::ADJ_CLOSE) / col(base::CLOSE)).alias("__ar"))
        .with_column((col(base::OPEN) * col("__ar")).alias("__ao"))
        .with_column(
            (col("__ao").log(E)
                - col(base::ADJ_CLOSE).log(E).shift(lit(1)).over([col(base::CODE)]))
            .alias(cols::R_ON),
        )
        .select([col(base::CODE), col(base::DATE), col(cols::R_ON)]);
    let prints = range(lake.scan(Dataset::MainlandPrints)
        .map_err(|_| ValidateError::Insufficient("no mainland prints history"))?)
        .select([col(base::CODE), col(base::DATE), col(base::A_OPEN_RET)]);
    let out = prints
        .join(daily, [col(base::CODE), col(base::DATE)],
              [col(base::CODE), col(base::DATE)], JoinArgs::new(JoinType::Inner))
        .collect()?;
    if out.height() == 0 {
        return Err(ValidateError::Insufficient("ah panel empty"));
    }
    Ok(out)
}

// ───────────────────────── artifact writers (atomic) ────────────────────────

/// Contract shared with hkq-engine::premarket::gate_state_path.
pub fn gate_state_path(lake_root: &Path) -> PathBuf {
    lake_root.join("_state").join("regime_gate.json")
}

pub fn ah_beta_path(lake_root: &Path) -> PathBuf {
    lake_root.join("_state").join("ah_beta.parquet")
}

pub fn save_gate(lake_root: &Path, gate: &RegimeGate) -> Result<(), ValidateError> {
    let path = gate_state_path(lake_root);
    let dir = path.parent().ok_or(ValidateError::Contract("gate path has no parent"))?;
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(".regime_gate.tmp");
    std::fs::write(&tmp, serde_json::to_vec_pretty(gate)?)?;
    std::fs::rename(&tmp, &path)?;
    tracing::info!(path = %path.display(), "regime gate written");
    Ok(())
}

pub fn save_ah_beta(lake_root: &Path, df: &mut DataFrame) -> Result<(), ValidateError> {
    let path = ah_beta_path(lake_root);
    let dir = path.parent().ok_or(ValidateError::Contract("ah_beta path has no parent"))?;
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(".ah_beta.tmp");
    {
        let f = std::fs::File::create(&tmp)?;
        ParquetWriter::new(&f).finish(df)?;
        f.sync_all()?;
    }
    std::fs::rename(&tmp, &path)?;
    tracing::info!(path = %path.display(), rows = df.height(), "ah betas written");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    fn xorshift(s: &mut u64) -> f64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        ((*s >> 11) as f64 / (1u64 << 53) as f64) - 0.5
    }

    #[test]
    fn theta_recovers_known_coefficients() {
        let mut s = 0x9E3779B97F4A7C15u64;
        let n = 400;
        let zid: Vec<f64> = (0..n).map(|_| xorshift(&mut s) * 2.0).collect();
        let zon: Vec<f64> = (0..n).map(|_| xorshift(&mut s) * 2.0).collect();
        let y: Vec<f64> = (0..n)
            .map(|i| 0.8 * zid[i] - 0.5 * zon[i] + 0.001 * xorshift(&mut s))
            .collect();
        let panel = df!("y" => y, "zid_1" => zid, "zon_1" => zon).unwrap();
        let t = fit_theta(&panel, 100).unwrap().unwrap();
        assert!((t.theta1 - 0.8).abs() < 0.05, "theta1 {}", t.theta1);
        assert!((t.theta2 - 0.5).abs() < 0.05, "theta2 {}", t.theta2);
        // Insufficient rows ⇒ None, keep the config prior.
        assert!(fit_theta(&panel.head(Some(5)), 100).unwrap().is_none());
    }

    #[test]
    fn vstar_recovers_true_threshold() {
        let mut s = 0xDEADBEEFCAFEBABEu64;
        let mut groups = Vec::new();
        for d in 0..40 {
            let (mut eps, mut vs, mut fwd) = (vec![], vec![], vec![]);
            for _ in 0..12 {
                let e = xorshift(&mut s) * 2.0;
                let v = xorshift(&mut s) + 0.5; // uniform [0,1]
                // Confirmed gaps continue, unconfirmed fade — true v* = 0.6.
                let f = e * (if v > 0.6 { 1.0 } else { -1.0 }) * 0.01;
                eps.push(e);
                vs.push(v);
                fwd.push(f);
            }
            groups.push((format!("2026-01-{:02}", d + 1), DateGroup { eps, vs, fwd }));
        }
        let grid = vstar_candidates(&groups, 17);
        assert!(!grid.is_empty());
        let (v, ic) = fit_vstar(&groups, 2.0, &grid).unwrap();
        assert!((v - 0.6).abs() < 0.08, "v* {v}");
        assert!(ic > 0.9);
    }

    #[test]
    fn gate_fit_finds_the_live_regime() {
        // Tercile 2: only x1 works. Terciles 0/1: only x2 works. x3: never.
        let (mut date, mut terc, mut x1, mut x2, mut x3, mut fwd) =
            (vec![], vec![], vec![], vec![], vec![], vec![]);
        for d in 0..2 {
            for t in 0..3u32 {
                for v in 1..=3 {
                    date.push(format!("2026-07-0{}", d + 1));
                    terc.push(t);
                    let vf = v as f64;
                    x1.push(vf);
                    x2.push(vf);
                    x3.push(4.0 - vf);
                    fwd.push(if t == 2 { vf } else { vf }); // monotone in both…
                }
            }
        }
        // …then break x2 in tercile 2 and x1 in terciles 0/1 via shuffles.
        for (i, t) in terc.iter().enumerate() {
            if *t == 2 { x2[i] = 2.0; }         // constant ⇒ IC None ⇒ 0
            else { x1[i] = 2.0; }               // constant ⇒ IC None ⇒ 0
        }
        // x3 anti-monotone ⇒ negative IC ⇒ clamped to 0 in every tercile ⇒ identity.
        let panel = df!(
            "date" => date, "ivu_tercile" => terc,
            "x1" => x1, "x2" => x2, "x3" => x3, "r_fwd" => fwd,
        ).unwrap();
        let g = fit_regime_gate(&panel).unwrap().g;
        assert!((g[2][0] - 2.0).abs() < 1e-9);          // x1 lives only in tercile 2 (clamped)
        assert!(g[0][0].abs() < 1e-9 && g[1][0].abs() < 1e-9);
        assert!((g[0][1] - 1.5).abs() < 1e-9 && (g[1][1] - 1.5).abs() < 1e-9);
        assert!(g[2][1].abs() < 1e-9);
        assert!((0..3).all(|t| (g[t][2] - 1.0).abs() < 1e-9)); // x3 identity
    }

    #[test]
    fn ah_beta_recovery_and_clamp() {
        let mut s = 0x1234_5678_9ABC_DEF1u64;
        let n = 120;
        let x: Vec<f64> = (0..n).map(|_| xorshift(&mut s) * 0.02).collect();
        let y: Vec<f64> = x.iter().map(|v| 1.3 * v + 0.0005 * xorshift(&mut s)).collect();
        let joined = df!(
            "code" => vec![941u32; n],
            "a_open_ret" => x,
            "r_on" => y,
        ).unwrap();
        let out = fit_ah_beta(&joined, 60).unwrap();
        assert_eq!(out.height(), 1);
        let b = out.column("ah_beta").unwrap().as_materialized_series()
            .f64().unwrap().get(0).unwrap();
        assert!((b - 1.3).abs() < 0.1, "beta {b}");
    }
}
```

```rust
// crates/hkq-validate/src/main.rs
//! §4 protocol jobs. Deliberately synchronous — pure batch over the lake.
//!
//! Usage:
//!   hkq-validate <strategy.toml> cusum
//!   hkq-validate <strategy.toml> fit-quarterly [YYYY-MM-DD]
//!   hkq-validate <strategy.toml> report
//!
//! `cusum` exits non-zero on a latched breach so cron/alerting notices; the
//! authoritative runtime producer is hkq-live's startup gate, not this job.
use anyhow::{bail, Context};
use chrono::{Duration, NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use hkq_core::config::StrategyCfg;
use hkq_data::lake::Lake;
use hkq_validate::cfg::{load_validate, ValidateCfg};
use hkq_validate::registry::{sha1_hex_of_file, TrialsRegistry};
use hkq_validate::splits::purged_walk_forward;
use hkq_validate::{cusum, dsr, fits, pnl, stats};
use serde_json::json;
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("info".parse()?))
        .init();

    let mut args = std::env::args().skip(1);
    let cfg_path = args.next()
        .context("usage: hkq-validate <strategy.toml> <cusum|fit-quarterly|report> [YYYY-MM-DD]")?;
    let cmd = args.next().context("missing subcommand: cusum | fit-quarterly | report")?;
    let date_arg: Option<NaiveDate> =
        args.next().map(|s| s.parse()).transpose().context("date must be YYYY-MM-DD")?;

    let cfg = StrategyCfg::load(&cfg_path)?;
    let vcfg = load_validate(&cfg_path)?;
    let lake = Lake::new(&cfg.ops.lake_root);

    match cmd.as_str() {
        "cusum" => job_cusum(&lake, &vcfg),
        "fit-quarterly" => job_fit_quarterly(&cfg, &cfg_path, &vcfg, &lake, date_arg),
        "report" => job_report(&vcfg, &lake),
        other => bail!("unknown subcommand: {other}"),
    }
}

fn job_cusum(lake: &Lake, vcfg: &ValidateCfg) -> anyhow::Result<()> {
    let out = cusum::update_from_lake(lake, vcfg)?;
    tracing::info!(s = out.s, new_points = out.new_points, last = ?out.last_date, "cusum updated");
    if out.breached {
        bail!("CUSUM BREACHED (S = {:.4}) — strategy is HALTED until operator reset", out.s);
    }
    Ok(())
}

fn median(mut v: Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    Some(v[v.len() / 2])
}

fn job_fit_quarterly(
    cfg: &StrategyCfg, cfg_path: &str, vcfg: &ValidateCfg, lake: &Lake,
    date_arg: Option<NaiveDate>,
) -> anyhow::Result<()> {
    let to = date_arg.unwrap_or_else(|| Utc::now().with_timezone(&Hong_Kong).date_naive());
    let from = to - Duration::days(vcfg.fit_window_days);
    let mut report = serde_json::Map::new();
    let mut metrics: BTreeMap<String, f64> = BTreeMap::new();
    report.insert("window_from".into(), json!(from.to_string()));
    report.insert("window_to".into(), json!(to.to_string()));

    // ── θ₁/θ₂ (recommendation; operator promotes into [stage1]) ─────────────
    match fits::sector_s1_panel(lake, cfg, from, to)
        .and_then(|p| fits::fit_theta(&p, vcfg.fit_min_obs))
    {
        Ok(Some(t)) => {
            tracing::info!(theta1 = t.theta1, theta2 = t.theta2, n = t.n, "theta fit");
            report.insert("theta".into(), json!({ "theta1": t.theta1, "theta2": t.theta2, "n": t.n }));
            metrics.insert("theta1".into(), t.theta1);
            metrics.insert("theta2".into(), t.theta2);
        }
        Ok(None) => {
            tracing::warn!("theta fit: insufficient panel — config priors stand");
            report.insert("theta".into(), json!("insufficient"));
        }
        Err(e) => {
            tracing::warn!(error = %e, "theta fit failed");
            report.insert("theta".into(), json!(format!("failed: {e}")));
        }
    }

    // ── v* (purged walk-forward) + regime gate (auto-consumed state file) ───
    match fits::scores_realized_panel(lake) {
        Ok(panel) => {
            let groups = fits::date_groups(&panel)?;
            let grid = fits::vstar_candidates(&groups, vcfg.vstar_grid);
            let sp = purged_walk_forward(groups.len(), vcfg.cv_folds, vcfg.cv_embargo, 1);
            let (mut choices, mut oos) = (Vec::new(), Vec::new());
            for s in &sp {
                if let Some((v, _)) = fits::fit_vstar(&groups[s.train.clone()],
                                                      cfg.stage2.phi, &grid) {
                    choices.push(v);
                    if let Some(ic) = fits::vstar_ic(&groups[s.test.clone()], cfg.stage2.phi, v) {
                        oos.push(ic);
                    }
                }
            }
            match median(choices.clone()) {
                Some(v) => {
                    let oos_ic = if oos.is_empty() { f64::NAN }
                                 else { oos.iter().sum::<f64>() / oos.len() as f64 };
                    tracing::info!(vstar = v, oos_ic, folds = choices.len(), "v* fit (purged WF)");
                    report.insert("vstar".into(),
                        json!({ "recommended": v, "oos_mean_ic": oos_ic, "folds": choices.len() }));
                    metrics.insert("vstar".into(), v);
                    if oos_ic.is_finite() { metrics.insert("vstar_oos_ic".into(), oos_ic); }
                }
                None => match fits::fit_vstar(&groups, cfg.stage2.phi, &grid) {
                    Some((v, ic)) => {
                        tracing::warn!(vstar = v, ic, "v*: history too short for folds — IN-SAMPLE only");
                        report.insert("vstar".into(), json!({ "in_sample_only": v, "ic": ic }));
                    }
                    None => { report.insert("vstar".into(), json!("insufficient")); }
                },
            }

            let n_dates = groups.len();
            if panel.height() >= vcfg.fit_min_obs && n_dates >= fits::GATE_MIN_DATES {
                match fits::fit_regime_gate(&panel) {
                    Ok(gate) => {
                        fits::save_gate(lake.root(), &gate)?;
                        for t in 0..3 {
                            for f in 0..3 {
                                metrics.insert(format!("g{t}{f}"), gate.g[t][f]);
                            }
                        }
                        report.insert("regime_gate".into(), json!(gate.g));
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "gate fit failed; existing state untouched");
                        report.insert("regime_gate".into(), json!(format!("failed: {e}")));
                    }
                }
            } else {
                tracing::warn!(rows = panel.height(), dates = n_dates,
                    "gate fit refused: below floor — identity/previous gate stands");
                report.insert("regime_gate".into(), json!("insufficient"));
            }
        }
        Err(e) => {
            tracing::warn!(error = %e, "scores⨝realized panel unavailable; v*/gate skipped");
            report.insert("vstar".into(), json!(format!("skipped: {e}")));
            report.insert("regime_gate".into(), json!(format!("skipped: {e}")));
        }
    }

    // ── AH betas (artifact; freeze-side consumption is the marked landing) ──
    match fits::ah_panel(lake, from, to).and_then(|p| fits::fit_ah_beta(&p, 60)) {
        Ok(mut df) if df.height() > 0 => {
            fits::save_ah_beta(lake.root(), &mut df)?;
            metrics.insert("ah_names".into(), df.height() as f64);
            report.insert("ah_beta".into(), json!({ "names": df.height() }));
        }
        Ok(_) => { report.insert("ah_beta".into(), json!("no names cleared min_obs")); }
        Err(e) => {
            tracing::warn!(error = %e, "ah beta fit skipped");
            report.insert("ah_beta".into(), json!(format!("skipped: {e}")));
        }
    }

    // ── persist the report + honest trial record ────────────────────────────
    let state_dir = lake.root().join("_state");
    std::fs::create_dir_all(&state_dir)?;
    let report_path = state_dir.join(format!("fit_report_{to}.json"));
    std::fs::write(&report_path, serde_json::to_vec_pretty(&report)?)?;
    tracing::info!(path = %report_path.display(), "fit report written");

    let registry = TrialsRegistry::open(vcfg.registry_path(lake.root()));
    registry.append("quarterly_fit", &sha1_hex_of_file(cfg_path)?, &metrics,
                    "quarterly θ/v*/gate/AH-β refresh")?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn job_report(vcfg: &ValidateCfg, lake: &Lake) -> anyhow::Result<()> {
    let series = pnl::daily_pnl(lake)?;
    let x: Vec<f64> = series.iter().map(|(_, v)| *v).collect();
    if x.len() < 20 {
        tracing::warn!(days = x.len(), "report: fewer than 20 shadow days — treat as anecdote");
    }
    let sr = stats::sharpe(&x);
    let nw = stats::newey_west_t(&x, 5);
    let ci = stats::block_bootstrap_sharpe_ci(&x, 10, 2000, 20260704);
    let (skew, kurt) = (stats::skewness(&x), stats::kurtosis(&x));

    let registry = TrialsRegistry::open(vcfg.registry_path(lake.root()));
    let chain_len = registry.verify_chain()?;
    let dispersion = registry.sr_dispersion()?;
    let dsr_val = match (sr, skew, kurt, dispersion) {
        (Some(s), Some(g3), Some(g4), Some((var, n))) =>
            dsr::deflated_sharpe(s, x.len(), g3, g4, var, n),
        _ => None,
    };

    let out = json!({
        "days": x.len(),
        "sharpe_daily": sr,
        "sharpe_annualized": sr.map(|s| s * 252f64.sqrt()),
        "newey_west_t_lag5": nw,
        "sharpe_ci_95_block_bootstrap": ci.map(|(p, lo, hi)| json!({ "point": p, "lo": lo, "hi": hi })),
        "skewness": skew,
        "kurtosis_nonexcess": kurt,
        "trials_registered": chain_len,
        "sr_dispersion_across_trials": dispersion.map(|(v, n)| json!({ "var": v, "n_with_sr": n })),
        "deflated_sharpe": dsr_val,
        "promotion_gate_dsr_gt_0_95": dsr_val.map(|d| d > 0.95),
        "note": "DSR is None until ≥2 registered trials carry an `sr` metric — record your trials.",
    });
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
```

## `hkq-live` — CUSUM gate wired as the second producer

```toml
# crates/hkq-live/Cargo.toml
[package]
name = "hkq-live"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-data = { path = "../hkq-data" }
hkq-risk = { path = "../hkq-risk" }
hkq-exec = { path = "../hkq-exec" }
hkq-engine = { path = "../hkq-engine" }
hkq-validate = { path = "../hkq-validate" }
tokio.workspace = true
tokio-stream.workspace = true
futures.workspace = true
anyhow.workspace = true
chrono.workspace = true
chrono-tz.workspace = true
rust_decimal.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
```

```rust
// crates/hkq-live/src/main.rs
//! Paper/shadow live binary (report §4: shadow period before capital).
//! Usage: hkq-live <strategy.toml> --equity <HKD> [YYYY-MM-DD]
//!
//! Wiring per the blueprint dataflow: provider streams → LiveMux → engine actor
//! (single md consumer) → ExecCmd → exec actor (PaperVenue) → fills → engine.
//! Kill-switch producers, both real as of M5: the operator console (`halt` +
//! Enter) and hkq-validate's CUSUM startup gate — the IC row for today exists
//! only after PostClose, so a breach is decidable (and latched) BEFORE any
//! order intent. An already-latched breach halts the day at startup.
use anyhow::Context;
use chrono::{NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use futures::StreamExt;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg, ids::StockCode,
               money::Cash, session::SessionTimes};
use hkq_data::{cfg::load_sources, eastmoney::{load_ah_map, EastMoneyClient},
               ingest::LiveMux, lake::Lake, model::MarketEvent,
               provider::{AuctionFeed, IntradayFeed, LinkedMarketFeed},
               tiger::TigerClient};
use hkq_engine::{Channels, NightlyState, RunCfg, TradingDay};
use hkq_exec::{spawn_exec, PaperVenue};
use hkq_risk::{HaltReason, KillSwitch};
use hkq_validate::{cfg::load_validate, cusum};
use rust_decimal::Decimal;
use std::str::FromStr;
use tokio_stream::wrappers::ReceiverStream;

fn load_universe_codes(path: &std::path::Path) -> anyhow::Result<Vec<StockCode>> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("universe codes file {}", path.display()))?;
    let codes: Vec<StockCode> = raw.lines().map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(StockCode::parse)
        .collect();
    anyhow::ensure!(!codes.is_empty(), "empty universe file {}", path.display());
    Ok(codes)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("info".parse()?))
        .init();

    let mut args = std::env::args().skip(1);
    let cfg_path = args.next().unwrap_or_else(|| "config/strategy.toml".into());
    let mut equity: Option<Decimal> = None;
    let mut date: Option<NaiveDate> = None;
    let mut rest = args.peekable();
    while let Some(a) = rest.next() {
        match a.as_str() {
            "--equity" => {
                let v = rest.next().context("--equity needs a value")?;
                equity = Some(Decimal::from_str(&v).context("equity must be a decimal HKD amount")?);
            }
            other => date = Some(other.parse().context("date must be YYYY-MM-DD")?),
        }
    }
    let equity = equity.context("--equity <HKD> is required (runtime param, not config)")?;
    anyhow::ensure!(equity > Decimal::ZERO, "equity must be positive");

    let cfg = StrategyCfg::load(&cfg_path)?;
    let sources = load_sources(&cfg_path)?;
    let calendar = FileCalendar::load(&cfg.ops.calendar_path)?;
    let date = date.unwrap_or_else(|| Utc::now().with_timezone(&Hong_Kong).date_naive());
    let codes = load_universe_codes(&cfg.ops.universe_codes_path)?;

    // 08:45 PreMarket: rebuild all nightly-derived state from the lake.
    let lake = Lake::new(&cfg.ops.lake_root);
    let state = NightlyState::load(&lake, &cfg, date, &calendar)
        .context("premarket assembly failed (is the lake populated by hkq-nightly?)")?;

    // Kill switch: producer 1 is the operator console…
    let (ks, kill_rx) = KillSwitch::new();
    {
        let ks = ks.clone();
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            let mut line = String::new();
            loop {
                line.clear();
                if stdin.read_line(&mut line).is_err() { break; }
                if line.trim().eq_ignore_ascii_case("halt") {
                    ks.halt(HaltReason::Operator);
                }
            }
        });
    }

    // …producer 2 is the §4 CUSUM gate (M5): replay the persisted score-IC
    // history through the latching monitor before any order intent exists.
    let vcfg = load_validate(&cfg_path)?;
    match cusum::startup_gate(&lake, &vcfg, &ks) {
        Ok(o) if o.breached => tracing::error!(s = o.s, last = ?o.last_date,
            "CUSUM breach LATCHED — engine will observe HALT and stand down"),
        Ok(o) => tracing::info!(s = o.s, new_points = o.new_points, last = ?o.last_date,
            "CUSUM gate clear"),
        Err(e) => tracing::warn!(error = %e,
            "CUSUM gate skipped (no scores history yet — cold-start shadow)"),
    }

    // Exec actor: paper venue only until the signed Tiger route lands (M6).
    let (fill_tx, fill_rx) = tokio::sync::mpsc::channel(4096);
    let (exec_tx, _exec_handle) =
        spawn_exec(PaperVenue, cfg.trade.participation_cap, fill_tx, kill_rx.clone());

    // Market data fan-in.
    let tiger = TigerClient::new(sources.tiger.context("[sources.tiger] is required")?)?;
    let (mut mux, md_rx) = LiveMux::new(8192);
    match tiger.subscribe_pos(&codes).await {
        Ok(s) => mux.pump_auction(s),
        Err(e) => tracing::warn!(error = %e, "POS feed unavailable — X2-DISABLED mode (§5)"),
    }
    mux.pump_bars(tiger.subscribe_bars_1m(&codes).await?);

    if let (Some(em_cfg), Some(ah_path)) = (sources.eastmoney, &cfg.ops.ah_map_path) {
        let ah_map = load_ah_map(ah_path)?;
        let ah_codes: Vec<StockCode> = ah_map.keys().copied().collect();
        let em = EastMoneyClient::new(em_cfg, ah_map);
        mux.pump_events(em.subscribe_a50().await?);
        // One-shot 09:25 mainland prints → events.
        let (tx, rx) = tokio::sync::mpsc::channel::<MarketEvent>(1024);
        tokio::spawn(async move {
            let target = hkq_core::session::hk(date, SessionTimes::get().mainland_print)
                .with_timezone(&Utc) + chrono::Duration::seconds(5);
            if let Ok(wait) = (target - Utc::now()).to_std() {
                tokio::time::sleep(wait).await;
            }
            match em.mainland_open_prints(&ah_codes).await {
                Ok(df) => {
                    let code = df.column("code").and_then(|c| Ok(c.as_materialized_series()
                        .u32()?.clone()));
                    let ret = df.column("a_open_ret").and_then(|c| Ok(c.as_materialized_series()
                        .f64()?.clone()));
                    if let (Ok(code), Ok(ret)) = (code, ret) {
                        for i in 0..df.height() {
                            if let (Some(c), Some(r)) = (code.get(i), ret.get(i)) {
                                let _ = tx.send(MarketEvent::MainlandAuctionPrint {
                                    code: StockCode(c), a_open_ret: r,
                                }).await;
                            }
                        }
                    }
                }
                Err(e) => tracing::warn!(error = %e, "mainland prints failed; S6 degrades"),
            }
        });
        mux.pump_events(ReceiverStream::new(rx).boxed());
    } else {
        tracing::warn!("eastmoney/ah_map not configured: S6 and S5 run degraded");
    }

    let day = TradingDay::new(
        cfg, RunCfg { equity: Cash(equity) }, date, Lake::new_from(&lake), state, exec_tx,
    )?;
    let result = day.run_day(&calendar, Channels { md_rx, fill_rx, kill_rx }).await;
    mux.shutdown().await;
    result.map_err(Into::into)
}
```

Config addition (one table; every value is the report's prior, pre-registered by being version-controlled):

```toml
[validate]
cusum_mu0 = 0.02        # healthy-regime mean daily rank IC (μ₀)
cusum_k   = 0.005       # CUSUM slack — half the shift to detect
cusum_h   = 0.15        # PRE-REGISTERED kill threshold; edits are protocol events
fit_window_days = 550   # ≈ 2y trading window for quarterly fits (§3.2)
fit_min_obs = 120
vstar_grid = 17
cv_folds = 5
cv_embargo = 5          # §4 embargo (days); label horizon 1d is purged automatically
# trials_registry_path = "data/lake/_state/trials.jsonl"   (default)
```

## Honest gaps and hand-off to Milestone 6

Six items, each now a named fact. First, the re-sequencing debt comes due: M6 is the signed Tiger venue behind `hkq-exec::Venue` plus partial-fill accounting in the pacer (which still treats submission as sent) — and it now lands inside a system that can refuse to trade, which was the point of doing validate first. Second, scalar promotions are governance, not automation: `fit-quarterly` writes recommendations and registry records for θ₁/θ₂ and v*, but only an operator edit of `strategy.toml` changes behavior — the gate matrix is the sole auto-consumed artifact because it alone has a typed engine slot. Third, `_state/ah_beta.parquet` now exists but `freeze.rs` still applies the ρ=1 prior; consuming it is a two-line join at the marked landing point, deliberately bundled with the next engine-touching milestone rather than done as a drive-by here. Fourth, per-sector A50 betas remain unbuildable for a data reason, not a math reason: no A50 stream is persisted (the `MorningBoard` holds it in memory and drops it), so a future data milestone must add an `A50` dataset before that fit can exist — same story for VHSI and the σ15m EWMA half-life, whose consumer isn't parameterized yet. Fifth, the SPA test against the report's benchmark family (always-cash, HSI open→close, sector-ETF Stage-1) needs benchmark return series that nothing ingests yet; DSR + bootstrap CIs are the shipped promotion gates until then. Sixth, CPCV: purged walk-forward is in production use (the v* fit); the combinatorial variant arrives with its first genuine model-selection consumer (the ML ranking layer, §3.7). The system now has facts (M1), opinions (M2), decisions (M3), a clock (M4), and — as of this milestone — a memory of every trial it ever ran and a reflex that stops it when the edge decays. What it still lacks is a hand that touches the real market, and that is exactly one trait implementation away.
