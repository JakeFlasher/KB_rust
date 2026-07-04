

All four documents are attached in full, so I worked directly from them — no fetching needed. M2's hand-off makes the next step explicit: it names the exact three seams built for its consumer — `stage1`/`stage2` scores for the decision path, `FactorWeights` + `spearman_ic` for the nightly weight refresh, and `PanelBuilder` for calibration panels — and assigns that consumer to "M3 scope alongside `hkq-signal`'s confirmation/meta-label layer and `hkq-risk`'s `size_book`."

# Milestone 3 — The Decision Layer (`hkq-signal` + `hkq-risk`)

**Why this is next.** M1 gave us facts (the lake), M2 gave us opinions (scores). Nothing yet converts an opinion into an *action*, and — just as important — nothing closes the two feedback loops the report demands: the ICIR weight refresh (§3.2/§3.4) and the meta-label α̂ calibration (§3.5). The engine milestone (M4) is a clock and a socket; if the decision math lands inside it, we end up debugging statistics, Decimal accounting, and async timing simultaneously — exactly the failure mode the M1→M2 ordering avoided. So M3 delivers the entire path *score → confirmation χ → meta-label filter → Decimal sizing → stop levels → kill-switch state*, plus the post-close attribution mathematics, all as pure, synchronously-testable functions. When M4 arrives, `ScoreFreeze`→`Entry` is a straight pipe through code that is already proven.

**In scope:** the `hkq-signal` crate (confirmation tracker, `AlphaMap` regime-bucketed meta-label with fitting, the §3.5 three-fold entry filter with a typed audit trail, per-factor daily rank-IC attribution and IC-panel maintenance, `FactorWeights` re-export closing M2's ownership note); the `hkq-risk` crate (config-driven cost model with the Decimal/f64 twin split, `size_book` with board-lot floors, participation caps and the no-leverage Decimal invariant, protective-stop math and a `StopBook`, the `watch<RiskState>` kill switch); and wiring `backfill_bars_1m` into `hkq-nightly` — the M2 gap that unblocks `rv`/`rv_5d`/`lav` and the α̂ label. **Deferred:** `hkq-exec` and the engine state machine (M4), `hkq-validate`'s CUSUM producer and quarterly fits (M5), VHSI ingestion (buckets degrade gracefully below), and scores-partition persistence (decided with the engine that produces scores).

Engineering decisions beyond the blueprint sketch, briefly: the entry filter follows the report *literally* — the top-`m` per sector is the proposal set, and a name outside it is rejected even if it would pass every gate (the blueprint's `take(m).filter(...)` had the same semantics; I make it explicit) — and every rejection is a typed `Rejection` carrying the numbers that killed it, because §6's post-close attribution needs reasons, not booleans. `AlphaMap` is conservative at cold start by design: unknown bucket → global → 0 bps, and 0 bps never clears `c_i + m*`, so an uncalibrated system trades nothing (the blueprint's shadow-period stance, mechanized); an explicit `flat()` prior exists for operators who choose otherwise, and `vhsi_tercile` is `Option` so the missing-VHSI feed degrades to the global bucket instead of fabricating a regime. The confirmation tracker keys on bar-open times and uses the first *post-09:30* bar close, so the 09:30 print is structurally untradeable (§3.5). Attribution's label is `ln(P₁₅:₄₅/P₀₉:₄₅)` from 1-minute bars — the exact §3.5 horizon — and ICs are computed on the `{f}_z` columns the combiner actually consumes (falling back to raw `f` if absent); the attribution *logic* lands here while its *scheduling* is M4's `PostClose` phase, because honest ICs need the live-frozen scores, and recomputing morning factors at night would fabricate them. In `hkq-risk`, the API surface is total (`Option`s, never panics): the blueprint's `Decimal::from_f64(...).unwrap()` on a NaN σ is fixed by routing all stop math through one guarded helper whose degenerate answer is stop = entry (the documented immediate-exit guard); the f64→Decimal conversion happens exactly once per name with `ToZero` rounding so the Σnotional ≤ equity invariant holds by construction and is still re-checked in Decimal. The cost floor is built from `CostCfg` — the blueprint's hardcoded `20.0 + 2.2` violated hkq-core's own no-magic-numbers rule — and stamp duty ceils to the dollar per contract note. The kill switch wraps `Arc<watch::Sender>` (clonable across its two blueprint producers without gambling on tokio's `Sender: Clone` history) with first-reason-wins semantics via `send_if_modified`. Finally, the Bars1m step is wired in the *binary*, not `hkq-data::ingest` — M1 crates stay frozen, per-code failures isolate, and the partition write stays atomic and idempotent.

```text
hkq/
├── Cargo.toml                        (updated: members)
└── crates/
    ├── hkq-signal/
    │   ├── Cargo.toml
    │   └── src/{lib,error,confirm,alpha,decision,attribution}.rs
    ├── hkq-risk/
    │   ├── Cargo.toml
    │   └── src/{lib,cost,sizing,stops,state}.rs
    └── hkq-nightly/
        ├── Cargo.toml                (updated: polars)
        └── src/main.rs               (updated: bars_1m backfill step)
```

## Workspace

```toml
# Cargo.toml
[workspace]
resolver = "2"
members = [
  "crates/hkq-core", "crates/hkq-data", "crates/hkq-factors",
  "crates/hkq-signal", "crates/hkq-risk", "crates/hkq-nightly",
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
# Feature set unchanged from M2 — M3 adds no new expression surface.
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

## `hkq-signal`

```toml
# crates/hkq-signal/Cargo.toml
[package]
name = "hkq-signal"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-factors = { path = "../hkq-factors" }
polars.workspace = true
chrono.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
tracing.workspace = true
```

```rust
// crates/hkq-signal/src/lib.rs
#![forbid(unsafe_code)]
//! The decision layer (report §3.5): confirmation χ, the regime-bucketed
//! meta-label α̂, the three-fold entry filter, and post-close attribution
//! (per-factor daily rank ICs → the IC panel that feeds `FactorWeights`).
//!
//! Design invariants:
//! - Everything here is pure w.r.t. its inputs. No I/O, no clocks, no channels:
//!   the engine (M4) owns WHEN; this crate owns WHETHER and WHY.
//! - The filter's answer is never a bare boolean. Every rejected candidate
//!   carries a typed `Rejection` with the numbers that killed it — §6's
//!   post-close attribution consumes reasons, not booleans.
//! - Cold start is conservative by construction: an unfitted `AlphaMap` predicts
//!   0 bps, and 0 bps never clears c_i + m*. An uncalibrated system trades
//!   nothing (the blueprint's shadow-period stance, mechanized).

pub mod alpha;
pub mod attribution;
pub mod confirm;
pub mod decision;
pub mod error;

pub use alpha::AlphaMap;
pub use confirm::ConfirmationTracker;
pub use decision::{entry_filter, Candidate, EntryDecision, Rejection};
pub use error::SignalError;

// M2 ownership note honored: the factor crate OWNS the ICIR machinery; the
// signal crate re-exports it as the blueprint's original seam expected.
pub use hkq_factors::icir::{spearman_ic, FactorWeights};
```

```rust
// crates/hkq-signal/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignalError {
    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),
    #[error("input contract violated: {0}")]
    Contract(&'static str),
    #[error("insufficient data: {0}")]
    Insufficient(&'static str),
}
```

```rust
// crates/hkq-signal/src/confirm.rs
//! Confirmation statistic χ (§3.5): χ_i = sgn(score_i)·r_i[09:30,09:45)/σ15m.
//!
//! Pure state machine: the engine feeds it (code, bar-open time, close) during
//! the Confirmation phase; timing lives in M4, arithmetic lives here.
//!
//! Convention: bars are keyed by their OPEN time (M1 model contract). The first
//! accepted bar opens at ≥09:30, so its close is a ~09:31 mark — the 09:30 print
//! itself is structurally untradeable, which is exactly §3.5's instruction.
use hkq_core::ids::StockCode;
use hkq_core::session::SessionTimes;
use chrono::NaiveTime;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ConfirmationTracker {
    window_from: NaiveTime, // inclusive: 09:30
    window_to: NaiveTime,   // exclusive: 09:45
    first: HashMap<StockCode, f64>,
    last: HashMap<StockCode, f64>,
}

impl Default for ConfirmationTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfirmationTracker {
    pub fn new() -> Self {
        let s = SessionTimes::get();
        Self { window_from: s.open, window_to: s.entry, first: HashMap::new(), last: HashMap::new() }
    }

    /// Absorb a 1-minute bar (by open time, HKT clock). Out-of-window and
    /// degenerate prices are ignored; duplicates keep first-first / last-last.
    pub fn absorb(&mut self, code: StockCode, bar_open_hkt: NaiveTime, close: f64) {
        if !(close.is_finite() && close > 0.0) {
            return;
        }
        if bar_open_hkt < self.window_from || bar_open_hkt >= self.window_to {
            return;
        }
        self.first.entry(code).or_insert(close);
        self.last.insert(code, close);
    }

    /// r_i[09:30,09:45): ln(last/first) over the accumulated window marks.
    pub fn window_return(&self, code: StockCode) -> Option<f64> {
        let f = *self.first.get(&code)?;
        let l = *self.last.get(&code)?;
        let r = (l / f).ln();
        r.is_finite().then_some(r)
    }

    /// χ = sgn(score)·r/σ15m. None ⇒ no in-window data ⇒ the candidate FAILS
    /// confirmation (no data, no trade — conservative by policy).
    pub fn chi(&self, code: StockCode, score: f64, sigma_15m: f64) -> Option<f64> {
        Some(score.signum() * self.window_return(code)? / sigma_15m.max(1e-9))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn t(h: u32, m: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, 0).unwrap()
    }

    #[test]
    fn window_edges_and_return() {
        let mut c = ConfirmationTracker::new();
        let code = StockCode(700);
        c.absorb(code, t(9, 29), 99.0);   // pre-window: ignored
        c.absorb(code, t(9, 30), 100.0);  // first accepted mark
        c.absorb(code, t(9, 40), 101.0);
        c.absorb(code, t(9, 45), 150.0);  // 09:45 bar is exclusive: ignored
        let r = c.window_return(code).unwrap();
        assert!((r - (101f64 / 100.0).ln()).abs() < 1e-12);
        let chi = c.chi(code, 2.0, 0.01).unwrap();
        assert!(chi > 0.0);
        assert!((chi - r / 0.01).abs() < 1e-9);
    }

    #[test]
    fn no_data_means_none() {
        let c = ConfirmationTracker::new();
        assert!(c.window_return(StockCode(5)).is_none());
        assert!(c.chi(StockCode(5), 1.0, 0.01).is_none());
    }
}
```

```rust
// crates/hkq-signal/src/alpha.rs
//! The meta-label map (§3.5): nightly OLS of realized r[09:45→15:45] on the
//! composite score within regime buckets (VHSI tercile × IVU tercile).
//! α̂_i = a + b·score_i, in return units; exposed in bps for the cost comparison.
//!
//! Missing-regime policy: vhsi_tercile is Option — the VHSI feed doesn't exist
//! yet (M3 honest gap), and a missing regime must degrade to the GLOBAL bucket,
//! never fabricate one. Unknown bucket AND no global fit ⇒ 0 bps ⇒ the filter
//! never trades: cold start is conservative by construction.
use crate::attribution::R_FWD;
use crate::error::SignalError;
use hkq_factors::cols;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const VHSI_TERCILE: &str = "vhsi_tercile";
pub const GLOBAL_BUCKET: &str = "global";

fn key(vhsi: u8, ivu: u8) -> String {
    format!("{vhsi}-{ivu}")
}

/// Regime-bucketed linear alpha model. Serializable: persisted as JSON next to
/// the lake and reloaded at PreMarket (M4).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlphaMap {
    /// bucket ("v-i" or "global") → (a, b) in RETURN units.
    pub coef: BTreeMap<String, (f64, f64)>,
}

impl AlphaMap {
    /// Explicit operator prior: a flat α̂ of `alpha_bps` regardless of score.
    /// For deliberate warm starts only — the default is the conservative zero.
    pub fn flat(alpha_bps: f64) -> Self {
        let mut coef = BTreeMap::new();
        coef.insert(GLOBAL_BUCKET.to_string(), (alpha_bps / 1e4, 0.0));
        Self { coef }
    }

    /// α̂ in bps. Fallback chain: exact bucket → global → (0, 0).
    pub fn expected_alpha_bps(&self, vhsi: Option<u8>, ivu: u8, score: f64) -> f64 {
        let bucket = vhsi.and_then(|v| self.coef.get(&key(v, ivu)));
        let (a, b) = bucket
            .or_else(|| self.coef.get(GLOBAL_BUCKET))
            .copied()
            .unwrap_or((0.0, 0.0));
        (a + b * score) * 1e4
    }

    /// Fit from the calibration panel: [score, r_fwd, vhsi_tercile (nullable u32),
    /// ivu_tercile (u32)]. Buckets need ≥ min_obs finite rows; the global fit uses
    /// every finite row (including null-VHSI rows) and is the documented fallback.
    pub fn fit(panel: &DataFrame, min_obs: usize) -> Result<Self, SignalError> {
        let score = panel.column(cols::SCORE)?.as_materialized_series().f64()?.clone();
        let r = panel.column(R_FWD)?.as_materialized_series().f64()?.clone();
        let vhsi = panel.column(VHSI_TERCILE)?.as_materialized_series().u32()?.clone();
        let ivu = panel.column(cols::IVU_TERCILE)?.as_materialized_series().u32()?.clone();

        let mut global: Vec<(f64, f64)> = Vec::new();
        let mut buckets: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
        for i in 0..panel.height() {
            let (Some(x), Some(y), Some(iv)) = (score.get(i), r.get(i), ivu.get(i)) else {
                continue;
            };
            if !(x.is_finite() && y.is_finite()) {
                continue;
            }
            global.push((x, y));
            if let Some(v) = vhsi.get(i) {
                buckets
                    .entry(key(v.min(255) as u8, iv.min(255) as u8))
                    .or_default()
                    .push((x, y));
            }
        }

        let mut coef = BTreeMap::new();
        if let Some(ab) = ols(&global, min_obs) {
            coef.insert(GLOBAL_BUCKET.to_string(), ab);
        }
        for (k, v) in buckets {
            if let Some(ab) = ols(&v, min_obs) {
                coef.insert(k, ab);
            }
        }
        Ok(Self { coef })
    }
}

/// Simple-regression OLS with a variance guard: zero-variance scores collapse
/// to (ȳ, 0) — the honest "score carries no information here" answer.
fn ols(xy: &[(f64, f64)], min_obs: usize) -> Option<(f64, f64)> {
    let n = xy.len();
    if n < min_obs.max(2) {
        return None;
    }
    let nf = n as f64;
    let mx = xy.iter().map(|(x, _)| x).sum::<f64>() / nf;
    let my = xy.iter().map(|(_, y)| y).sum::<f64>() / nf;
    let (mut sxx, mut sxy) = (0.0, 0.0);
    for (x, y) in xy {
        sxx += (x - mx) * (x - mx);
        sxy += (x - mx) * (y - my);
    }
    if sxx <= 1e-18 {
        return Some((my, 0.0));
    }
    let b = sxy / sxx;
    Some((my - b * mx, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    #[test]
    fn fit_recovers_bucket_and_global() {
        let n = 80usize;
        let score: Vec<f64> = (0..n).map(|i| -0.02 + 0.0005 * i as f64).collect();
        let r: Vec<f64> = score.iter().map(|s| 0.001 + 0.5 * s).collect();
        // Half the rows carry a VHSI tercile; the rest are feed-gap rows.
        let vhsi: Vec<Option<u32>> = (0..n).map(|i| (i % 2 == 0).then_some(1u32)).collect();
        let panel = df!(
            "score" => score,
            "r_fwd" => r,
            "vhsi_tercile" => vhsi,
            "ivu_tercile" => vec![1u32; n],
        )
        .unwrap();
        let m = AlphaMap::fit(&panel, 20).unwrap();
        let (a, b) = m.coef.get("1-1").copied().unwrap();
        assert!((a - 0.001).abs() < 1e-9 && (b - 0.5).abs() < 1e-9);
        let (ga, gb) = m.coef.get(GLOBAL_BUCKET).copied().unwrap();
        assert!((ga - 0.001).abs() < 1e-9 && (gb - 0.5).abs() < 1e-9);
        // Fallbacks: missing bucket → global; both use the same true model here.
        let e1 = m.expected_alpha_bps(Some(1), 1, 0.02);
        let e2 = m.expected_alpha_bps(None, 1, 0.02);
        assert!((e1 - (0.001 + 0.5 * 0.02) * 1e4).abs() < 1e-6);
        assert!((e1 - e2).abs() < 1e-6);
    }

    #[test]
    fn cold_start_is_zero_and_flat_prior_works() {
        let m = AlphaMap::default();
        assert_eq!(m.expected_alpha_bps(Some(2), 0, 1.0), 0.0);
        let m = AlphaMap::flat(40.0);
        assert!((m.expected_alpha_bps(None, 0, 123.0) - 40.0).abs() < 1e-9);
    }

    #[test]
    fn serde_roundtrip() {
        let m = AlphaMap::flat(25.0);
        let s = serde_json::to_string(&m).unwrap();
        let back: AlphaMap = serde_json::from_str(&s).unwrap();
        assert!((back.expected_alpha_bps(None, 1, 0.0) - 25.0).abs() < 1e-9);
    }
}
```

```rust
// crates/hkq-signal/src/decision.rs
//! §3.5 — the three-fold entry filter at 09:45. The ranking model proposes,
//! the calibrated conditional-expectation model disposes.
//!
//! Report-literal semantics: the proposal set is the top-m BY SCORE per selected
//! sector; a name outside the top-m is rejected as SectorQuotaFull even if it
//! would pass every gate. (The blueprint's `take(m).filter(...)` had exactly
//! these semantics; this makes them explicit and auditable.)
use crate::alpha::AlphaMap;
use hkq_core::config::TradeCfg;
use hkq_core::ids::{SectorId, StockCode};
use std::collections::BTreeMap;

/// One frozen candidate at 09:45. The engine (M4) assembles these from the
/// stage-2 frame + ConfirmationTracker + hkq-risk's cost floor.
#[derive(Debug, Clone)]
pub struct Candidate {
    pub code: StockCode,
    pub sector: SectorId,
    pub score: f64,
    pub sigma_15m: f64,
    pub lav: f64,
    /// c_i at tentative size (hkq-risk::CostModel::floor_bps), in bps.
    pub cost_floor_bps: f64,
    /// r[09:30,09:45) from ConfirmationTracker; None ⇒ no in-window data.
    pub r_0930_0945: Option<f64>,
    /// None ⇒ VHSI feed absent ⇒ AlphaMap global bucket (documented degradation).
    pub vhsi_tercile: Option<u8>,
    pub ivu_tercile: u8,
}

/// Why a candidate did NOT trade — the attribution job's raw material.
#[derive(Debug, Clone, PartialEq)]
pub enum Rejection {
    /// Outside the top-m proposal set of its sector (§3.5).
    SectorQuotaFull,
    /// Long-only book: score ≤ 0 (or non-finite) never proposes.
    NonPositiveScore,
    /// No bars observed in [09:30, 09:45) — no data, no trade.
    NoConfirmationData,
    /// χ ≤ 0: the market did not confirm the score's direction.
    FailedConfirmation { chi: f64 },
    /// α̂ failed the net-edge hurdle α̂ > c_i + m* (§3.5).
    BelowCostHurdle { alpha_bps: f64, hurdle_bps: f64 },
}

#[derive(Debug)]
pub struct EntryDecision<'a> {
    pub accepted: Vec<&'a Candidate>,
    pub rejected: Vec<(&'a Candidate, Rejection)>,
}

pub fn entry_filter<'a>(
    cands: &'a [Candidate],
    alpha: &AlphaMap,
    m_per_sector: usize,
    trade: &TradeCfg,
) -> EntryDecision<'a> {
    let mut by_sector: BTreeMap<u16, Vec<&Candidate>> = BTreeMap::new();
    for c in cands {
        by_sector.entry(c.sector.0).or_default().push(c);
    }

    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for (_sector, mut v) in by_sector {
        v.sort_by(|a, b| b.score.total_cmp(&a.score));
        for (idx, c) in v.into_iter().enumerate() {
            if idx >= m_per_sector {
                rejected.push((c, Rejection::SectorQuotaFull));
                continue;
            }
            if !(c.score.is_finite() && c.score > 0.0) {
                rejected.push((c, Rejection::NonPositiveScore));
                continue;
            }
            let Some(r) = c.r_0930_0945.filter(|r| r.is_finite()) else {
                rejected.push((c, Rejection::NoConfirmationData));
                continue;
            };
            // (i) confirmation: χ = sgn(score)·r/σ15m > 0 (score > 0 ⇒ sgn = 1).
            let chi = c.score.signum() * r / c.sigma_15m.max(1e-9);
            if chi <= 0.0 {
                rejected.push((c, Rejection::FailedConfirmation { chi }));
                continue;
            }
            // (ii) meta-label: α̂ > c_i + m*, all in bps (§3.5).
            let alpha_bps = alpha.expected_alpha_bps(c.vhsi_tercile, c.ivu_tercile, c.score);
            let hurdle_bps = c.cost_floor_bps + trade.margin_bps;
            if !(alpha_bps.is_finite() && alpha_bps > hurdle_bps) {
                rejected.push((c, Rejection::BelowCostHurdle { alpha_bps, hurdle_bps }));
                continue;
            }
            accepted.push(c);
        }
    }
    EntryDecision { accepted, rejected }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hkq_core::config::HalfDayMode;

    fn trade_cfg() -> TradeCfg {
        TradeCfg {
            margin_bps: 10.0,
            stop_sigma15m_mult: 2.5,
            participation_cap: 0.02,
            half_day_mode: HalfDayMode::Skip,
            reuse_unsettled_proceeds: false,
        }
    }

    fn cand(code: u32, sector: u16, score: f64, r: Option<f64>) -> Candidate {
        Candidate {
            code: StockCode(code),
            sector: SectorId(sector),
            score,
            sigma_15m: 0.01,
            lav: 0.1,
            cost_floor_bps: 25.0,
            r_0930_0945: r,
            vhsi_tercile: None,
            ivu_tercile: 1,
        }
    }

    #[test]
    fn three_fold_gate_and_quota() {
        let cands = vec![
            cand(1, 1, 3.0, Some(0.01)),   // top-1: passes everything
            cand(2, 1, 2.0, Some(-0.01)),  // top-2: fails χ
            cand(3, 1, 1.5, Some(0.02)),   // rank 3: outside quota even though it passes
            cand(4, 2, -0.5, Some(0.01)),  // other sector: negative score
            cand(5, 2, 1.0, None),         // other sector: no confirmation data
        ];
        let alpha = AlphaMap::flat(100.0); // 100 bps ≫ 25 + 10 hurdle
        let d = entry_filter(&cands, &alpha, 2, &trade_cfg());
        assert_eq!(d.accepted.len(), 1);
        assert_eq!(d.accepted[0].code, StockCode(1));
        let find = |code: u32| {
            d.rejected.iter().find(|(c, _)| c.code == StockCode(code)).map(|(_, r)| r.clone())
        };
        assert!(matches!(find(2), Some(Rejection::FailedConfirmation { .. })));
        assert_eq!(find(3), Some(Rejection::SectorQuotaFull));
        assert_eq!(find(4), Some(Rejection::NonPositiveScore));
        assert_eq!(find(5), Some(Rejection::NoConfirmationData));
    }

    #[test]
    fn cost_hurdle_rejects_and_cold_start_trades_nothing() {
        let cands = vec![cand(1, 1, 3.0, Some(0.01))];
        // Thin edge: 30 bps < 25 + 10.
        let d = entry_filter(&cands, &AlphaMap::flat(30.0), 2, &trade_cfg());
        assert!(d.accepted.is_empty());
        assert!(matches!(d.rejected[0].1, Rejection::BelowCostHurdle { .. }));
        // Cold start: default AlphaMap ⇒ 0 bps ⇒ nothing ever trades.
        let d = entry_filter(&cands, &AlphaMap::default(), 2, &trade_cfg());
        assert!(d.accepted.is_empty());
    }
}
```

```rust
// crates/hkq-signal/src/attribution.rs
//! Post-close attribution (runbook §6): the realized §3.5 label from 1-minute
//! bars and the per-factor daily rank-IC row that the nightly job appends to the
//! IC panel — the exact input of `FactorWeights::from_ic_panel` (M2).
//!
//! Scheduling note (deliberate): the LOGIC lives here; the SCHEDULING is the
//! engine's PostClose phase (M4). Honest ICs need the scores frozen live at
//! 09:29:30 — recomputing morning factors at night would fabricate them.
use crate::error::SignalError;
use chrono::{NaiveDate, NaiveTime, Timelike};
use hkq_factors::cols::{self, base};
use hkq_factors::icir::spearman_ic;
use hkq_factors::realized::bod5_expr;
use polars::prelude::*;

const E: f64 = std::f64::consts::E;

/// Realized forward-return column: ln(P_last/P_first) over [from, to).
pub const R_FWD: &str = "r_fwd";

/// HKT clock time → time-of-day 5-minute slot (matches realized::bod5_expr).
pub fn bod5_of(t: NaiveTime) -> i64 {
    (t.hour() * 60 + t.minute()) as i64 / 5
}

/// Per-(code, date) realized window return from 1-minute bars: first/last close
/// in [from, to) HKT. With the §3.5 defaults (09:45, 15:45) this is the
/// meta-label horizon; the last included bar OPENS 15:44 ⇒ its close ≈ 15:45.
pub fn realized_window_returns(bars_1m: LazyFrame, from: NaiveTime, to: NaiveTime) -> LazyFrame {
    let (lo, hi) = (bod5_of(from), bod5_of(to));
    bars_1m
        .sort_by_exprs([col(base::CODE), col(base::TS_MS)], Default::default())
        .with_column(bod5_expr())
        .filter(col(cols::BOD5).gt_eq(lit(lo)).and(col(cols::BOD5).lt(lit(hi))))
        .group_by([col(base::CODE), col(base::DATE)])
        .agg([(col(cols::C1M).last().log(E) - col(cols::C1M).first().log(E)).alias(R_FWD)])
}

/// One IC-panel row for `date`: Spearman rank IC of each factor against R_FWD
/// across the frozen cross-section. Prefers the `{f}_z` column (what the
/// combiner actually consumed), falls back to raw `f`. Fewer than 3 valid pairs
/// ⇒ null IC — an honest "no data" day; from_ic_panel filters non-finite (M2).
pub fn daily_ic_row(
    date: NaiveDate,
    scores: &DataFrame,
    realized: &DataFrame,
    factors: &[&str],
) -> Result<DataFrame, SignalError> {
    if factors.is_empty() {
        return Err(SignalError::Contract("factors list must be non-empty"));
    }
    let joined = scores
        .clone()
        .lazy()
        .join(
            realized.clone().lazy(),
            [col(base::CODE)],
            [col(base::CODE)],
            JoinArgs::new(JoinType::Inner),
        )
        .collect()?;

    let fwd_ca = joined.column(R_FWD)?.as_materialized_series().f64()?.clone();
    let fwd: Vec<f64> = (0..joined.height())
        .map(|i| fwd_ca.get(i).unwrap_or(f64::NAN))
        .collect();

    let mut out: Vec<Column> = vec![Series::new(base::DATE.into(), vec![date.to_string()]).into_column()];
    for f in factors {
        let zname = cols::z(f);
        let s = joined
            .column(zname.as_str())
            .or_else(|_| joined.column(f))?
            .as_materialized_series()
            .f64()?
            .clone();
        let pred: Vec<f64> = (0..joined.height())
            .map(|i| s.get(i).unwrap_or(f64::NAN))
            .collect();
        let ic = spearman_ic(&pred, &fwd); // pairwise-finite; None if degenerate
        out.push(Series::new(format!("ic_{f}").into(), vec![ic]).into_column());
    }
    Ok(DataFrame::new(out)?)
}

/// Append-only IC-panel maintenance. Schema is a CONTRACT: a changed factor set
/// must start a new panel version, not silently widen this one.
pub fn append_ic_history(
    history: Option<DataFrame>,
    row: DataFrame,
) -> Result<DataFrame, SignalError> {
    match history {
        None => Ok(row),
        Some(h) => h
            .vstack(&row)
            .map_err(|_| SignalError::Contract("IC panel schema changed — version the panel")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use hkq_core::session::hk;
    use polars::df;

    fn ts(h: u32, m: u32) -> i64 {
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        hk(d, NaiveTime::from_hms_opt(h, m, 0).unwrap()).timestamp_millis()
    }

    #[test]
    fn label_window_is_0945_to_1545_exclusive() {
        let lf = df!(
            "code" => vec![700u32; 4],
            "date" => vec!["2026-07-03"; 4],
            "ts_ms" => vec![ts(9, 40), ts(9, 45), ts(15, 44), ts(15, 45)],
            "c" => vec![90.0, 100.0, 110.0, 999.0],
        )
        .unwrap()
        .lazy();
        let out = realized_window_returns(
            lf,
            NaiveTime::from_hms_opt(9, 45, 0).unwrap(),
            NaiveTime::from_hms_opt(15, 45, 0).unwrap(),
        )
        .collect()
        .unwrap();
        assert_eq!(out.height(), 1);
        let r = out.column("r_fwd").unwrap().as_materialized_series()
            .f64().unwrap().get(0).unwrap();
        // 09:40 pre-window and the 15:45 bar are excluded: ln(110/100).
        assert!((r - (110f64 / 100.0).ln()).abs() < 1e-12);
    }

    #[test]
    fn ic_row_signs_and_append() {
        let scores = df!(
            "code" => vec![1u32, 2, 3, 4],
            "s1_z" => vec![-1.0, -0.2, 0.4, 1.3],  // aligned with r_fwd  → IC +1
            "x1_z" => vec![2.0, 1.0, 0.0, -1.0],   // anti-aligned        → IC −1
        )
        .unwrap();
        let realized = df!(
            "code" => vec![1u32, 2, 3, 4],
            "r_fwd" => vec![-0.02, -0.01, 0.01, 0.03],
        )
        .unwrap();
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        let row = daily_ic_row(d, &scores, &realized, &["s1", "x1"]).unwrap();
        let g = |n: &str| row.column(n).unwrap().as_materialized_series()
            .f64().unwrap().get(0).unwrap();
        assert!((g("ic_s1") - 1.0).abs() < 1e-12);
        assert!((g("ic_x1") + 1.0).abs() < 1e-12);

        let hist = append_ic_history(None, row.clone()).unwrap();
        let hist = append_ic_history(Some(hist), row).unwrap();
        assert_eq!(hist.height(), 2);
    }
}
```

## `hkq-risk`

```toml
# crates/hkq-risk/Cargo.toml
[package]
name = "hkq-risk"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
tokio.workspace = true
rust_decimal.workspace = true
rust_decimal_macros.workspace = true
serde.workspace = true
thiserror.workspace = true
tracing.workspace = true
```

```rust
// crates/hkq-risk/src/lib.rs
#![forbid(unsafe_code)]
//! Risk layer (report §1, §3.6): the cost floor c_i, Decimal position sizing
//! with board-lot and participation constraints, protective-stop math, and the
//! kill-switch state channel.
//!
//! The numerics boundary (blueprint invariant): statistical inputs arrive as
//! f64; the f64→Decimal conversion happens EXACTLY ONCE per name, inside
//! `size_book`, rounding toward zero. After that point everything is Decimal
//! and no float ever touches an order. The API surface is total: degenerate
//! inputs produce fewer orders (or stop = entry), never panics.

pub mod cost;
pub mod sizing;
pub mod state;
pub mod stops;

pub use cost::CostModel;
pub use sizing::{size_book, SizedOrder, SizingInput};
pub use state::{HaltReason, KillSwitch, RiskState};
pub use stops::{protective_stop, StopBook};
```

```rust
// crates/hkq-risk/src/cost.rs
//! The §1 cost stack, built from CostCfg — no magic numbers (hkq-core's own
//! rule; the blueprint sketch hardcoded 20.0 + 2.2 and is corrected here).
//! Twin representation by design: f64 for the signal-side floor c_i (feeds the
//! α̂ > c_i + m* comparison), Decimal for statutory accounting (attribution).
use hkq_core::config::CostCfg;
use hkq_core::error::CoreError;
use hkq_core::money::Cash;
use rust_decimal::{prelude::FromPrimitive, Decimal, RoundingStrategy};
use rust_decimal_macros::dec;

#[derive(Debug, Clone)]
pub struct CostModel {
    // Signal side (f64, bps).
    stamp_bps_per_side: f64,
    fees_bps_roundtrip: f64,
    kappa: f64,
    // Accounting side (Decimal, rates).
    stamp_rate_per_side: Decimal,
    fees_rate_roundtrip: Decimal,
}

impl CostModel {
    pub fn from_cfg(c: &CostCfg) -> Result<Self, CoreError> {
        let to_rate = |bps: f64| -> Result<Decimal, CoreError> {
            Ok(Decimal::from_f64(bps).ok_or(CoreError::BadFloat(bps))? / dec!(10000))
        };
        Ok(Self {
            stamp_bps_per_side: c.stamp_bps_per_side,
            fees_bps_roundtrip: c.fees_bps_roundtrip,
            kappa: c.impact_kappa,
            stamp_rate_per_side: to_rate(c.stamp_bps_per_side)?,
            fees_rate_roundtrip: to_rate(c.fees_bps_roundtrip)?,
        })
    }

    /// §1: c_i = 2·stamp + fees_rt + s_i + κ·σ_i·√(q_i/ADV_i), in bps.
    /// Signal-side twin — used ONLY for the α̂ comparison, never for accounting.
    pub fn floor_bps(&self, spread_bps: f64, sigma_cc: f64, q_shares: f64, adv_shares: f64) -> f64 {
        2.0 * self.stamp_bps_per_side
            + self.fees_bps_roundtrip
            + spread_bps.max(0.0)
            + self.kappa * sigma_cc.abs() * (q_shares.max(0.0) / adv_shares.max(1.0)).sqrt() * 1e4
    }

    /// Statutory stamp duty per contract note: rate × |notional|, rounded UP to
    /// the whole dollar (§4 cost realism).
    pub fn stamp_duty(&self, notional: Cash) -> Cash {
        Cash((notional.0.abs() * self.stamp_rate_per_side).ceil())
    }

    /// Exchange/levy fees for one side (half the round trip), 2 dp banker's.
    pub fn fees_per_side(&self, notional: Cash) -> Cash {
        Cash(
            (notional.0.abs() * self.fees_rate_roundtrip / dec!(2))
                .round_dp_with_strategy(2, RoundingStrategy::MidpointNearestEven),
        )
    }

    /// Full deterministic round trip (entry + exit), Decimal: duty both sides
    /// (buyer and seller each pay per contract note) + fees both sides.
    pub fn round_trip_cost(&self, entry_notional: Cash, exit_notional: Cash) -> Cash {
        Cash(
            self.stamp_duty(entry_notional).0
                + self.stamp_duty(exit_notional).0
                + self.fees_per_side(entry_notional).0
                + self.fees_per_side(exit_notional).0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> CostCfg {
        CostCfg { stamp_bps_per_side: 10.0, fees_bps_roundtrip: 2.2, impact_kappa: 0.1 }
    }

    #[test]
    fn floor_matches_hand_math() {
        let m = CostModel::from_cfg(&cfg()).unwrap();
        // 20 + 2.2 + 5 + 0.1·0.02·√(10⁴/10⁶)·10⁴ = 27.2 + 2 = 29.2
        let f = m.floor_bps(5.0, 0.02, 10_000.0, 1_000_000.0);
        assert!((f - 29.2).abs() < 1e-9);
    }

    #[test]
    fn stamp_ceils_to_the_dollar() {
        let m = CostModel::from_cfg(&cfg()).unwrap();
        let duty = m.stamp_duty(Cash(dec!(123456.78)));
        assert_eq!(duty.0, dec!(124)); // 123.45678 → ceil → 124
        let duty = m.stamp_duty(Cash(dec!(-50000)));
        assert_eq!(duty.0, dec!(50)); // sign-agnostic, exact
    }

    #[test]
    fn round_trip_composes() {
        let m = CostModel::from_cfg(&cfg()).unwrap();
        let n = Cash(dec!(100000));
        let rt = m.round_trip_cost(n, n);
        // duty 100 + 100, fees 11.00 + 11.00
        assert_eq!(rt.0, dec!(222.00));
    }
}
```

```rust
// crates/hkq-risk/src/sizing.rs
//! §3.6 sizing: w_i ∝ 1/LAV_i, Σw = 1 (cash account, no leverage), board-lot
//! floors, participation ≤ cap·projected interval volume, stop at −mult·σ15m.
//!
//! Invariant chain (why Σnotional ≤ equity holds by CONSTRUCTION): weights sum
//! to 1 → each Decimal target is rounded TOWARD ZERO → lot flooring only shrinks
//! → participation capping only shrinks. The Decimal re-check is defense in
//! depth, not the mechanism.
use hkq_core::config::TradeCfg;
use hkq_core::ids::StockCode;
use hkq_core::money::{BoardLot, Cash, LotQty, Px};
use rust_decimal::{prelude::FromPrimitive, Decimal, RoundingStrategy};

#[derive(Debug, Clone)]
pub struct SizingInput {
    pub code: StockCode,
    /// 09:45 marketable reference price.
    pub ref_px: Px,
    pub lot: BoardLot,
    /// §3.6 inverse-LAV weight input (f64: statistics side).
    pub lav: f64,
    pub sigma_15m: f64,
    /// Seasonal profile × today's surprise multiplier (shares).
    pub projected_interval_vol: f64,
}

#[derive(Debug, Clone)]
pub struct SizedOrder {
    pub code: StockCode,
    pub qty: LotQty,
    pub limit: Px,
    /// −mult·σ15m from entry, snapped DOWN to a valid tick; degenerate σ ⇒
    /// stop = entry (immediate-exit guard, per blueprint).
    pub stop: Px,
    pub target_cash: Cash,
}

/// Returns the sized book and the residual cash — cash is a position (§3.2).
pub fn size_book(
    equity: Cash,
    inputs: &[SizingInput],
    cfg: &TradeCfg,
) -> (Vec<SizedOrder>, Cash) {
    if inputs.is_empty() || equity.0 <= Decimal::ZERO {
        return (vec![], equity);
    }
    // 1) Weights in f64 (statistics), normalized. LAV floor keeps z finite.
    let inv: Vec<f64> = inputs.iter().map(|i| 1.0 / i.lav.max(1e-9)).collect();
    let z: f64 = inv.iter().sum();

    let mut used = Decimal::ZERO;
    let mut orders = Vec::with_capacity(inputs.len());

    for (i, inp) in inputs.iter().enumerate() {
        // 2) f64 → Decimal exactly once per name; round DOWN — never size up.
        let w = Decimal::from_f64(inv[i] / z).unwrap_or(Decimal::ZERO);
        let target = Cash((equity.0 * w).round_dp_with_strategy(2, RoundingStrategy::ToZero));

        // 3) Board-lot floor, then participation cap q_i ≤ cap·projected volume.
        let Some(qty) = LotQty::floor_from_cash(target, inp.ref_px, inp.lot) else {
            continue; // < 1 lot at this weight: the name drops out, cash absorbs it
        };
        let cap = inp.projected_interval_vol * cfg.participation_cap;
        let cap_shares = if cap.is_finite() && cap >= 1.0 { cap.floor() as u64 } else { 0 };
        let Some(qty) = qty.cap_shares(cap_shares) else {
            continue; // cap below one lot: untradeable today
        };

        let notional = qty.notional(inp.ref_px);
        // 4) Defense-in-depth invariant check, all Decimal.
        if used + notional.0 > equity.0 {
            tracing::warn!(code = %inp.code, "sizing invariant guard tripped; skipping name");
            continue;
        }
        used += notional.0;

        let stop = crate::stops::protective_stop(inp.ref_px, inp.sigma_15m, cfg.stop_sigma15m_mult);
        orders.push(SizedOrder { code: inp.code, qty, limit: inp.ref_px, stop, target_cash: notional });
    }
    (orders, Cash(equity.0 - used))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hkq_core::config::HalfDayMode;
    use rust_decimal_macros::dec;

    fn cfg() -> TradeCfg {
        TradeCfg {
            margin_bps: 10.0,
            stop_sigma15m_mult: 2.5,
            participation_cap: 0.02,
            half_day_mode: HalfDayMode::Skip,
            reuse_unsettled_proceeds: false,
        }
    }

    fn inp(code: u32, lav: f64, proj_vol: f64) -> SizingInput {
        SizingInput {
            code: StockCode(code),
            ref_px: Px::from_f64_quote(10.0).unwrap(),
            lot: BoardLot(500),
            lav,
            sigma_15m: 0.01,
            projected_interval_vol: proj_vol,
        }
    }

    #[test]
    fn inverse_lav_lots_caps_and_residual() {
        let equity = Cash(dec!(1000000));
        // inv-LAV 10 vs 5 ⇒ weights 2/3, 1/3. A's participation cap binds hard.
        let inputs = vec![inp(1, 0.1, 1_000_000.0), inp(2, 0.2, 100_000_000.0)];
        let (orders, residual) = size_book(equity, &inputs, &cfg());
        assert_eq!(orders.len(), 2);
        // A: target 666,666.66 → 133 lots (66,500 sh) → cap 2%·1M = 20,000 sh → 40 lots.
        assert_eq!(orders[0].qty.shares(), 20_000);
        assert_eq!(orders[0].target_cash.0, dec!(200000.0));
        // B: target 333,333.33 → 66 lots = 33,000 sh = 330,000 HKD.
        assert_eq!(orders[1].qty.shares(), 33_000);
        // Residual = 1,000,000 − 200,000 − 330,000.
        assert_eq!(residual.0, dec!(470000.0));
        // Stop: 10·(1 − 2.5·0.01) = 9.75, already on-tick in the ≤10 band.
        assert_eq!(orders[0].stop.get(), dec!(9.75));
    }

    #[test]
    fn degenerate_inputs_never_panic() {
        let equity = Cash(dec!(100000));
        let mut a = inp(1, f64::NAN, f64::NAN); // NaN LAV → floor; NaN cap → drop
        a.sigma_15m = f64::NAN;
        let (orders, residual) = size_book(equity, &[a], &cfg());
        assert!(orders.is_empty());
        assert_eq!(residual.0, dec!(100000));
        // Zero-equity and empty-book cases are total too.
        let (o, r) = size_book(Cash(dec!(0)), &[inp(1, 0.1, 1e9)], &cfg());
        assert!(o.is_empty());
        assert_eq!(r.0, dec!(0));
        let (o, r) = size_book(equity, &[], &cfg());
        assert!(o.is_empty());
        assert_eq!(r.0, dec!(100000));
    }

    #[test]
    fn nan_sigma_gives_entry_stop_guard() {
        let equity = Cash(dec!(1000000));
        let mut a = inp(1, 0.1, 1e9);
        a.sigma_15m = f64::NAN;
        let (orders, _) = size_book(equity, &[a], &cfg());
        assert_eq!(orders[0].stop, orders[0].limit); // immediate-exit guard
    }
}
```

```rust
// crates/hkq-risk/src/stops.rs
//! Protective-stop math (§3.6) and the pure stop-tracking book the engine's
//! bar handler consults. Touch rule: low ≤ stop triggers (conservative exit).
use hkq_core::ids::StockCode;
use hkq_core::money::Px;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use std::collections::HashMap;

/// stop = entry·(1 − mult·σ15m), snapped DOWN to a valid HKEX tick. Degenerate
/// σ/mult (non-finite, ≤ 0, or ≥ 100% move) ⇒ stop = entry — the blueprint's
/// immediate-exit guard: any tick below entry stops the position out.
pub fn protective_stop(entry: Px, sigma_15m: f64, mult: f64) -> Px {
    let frac = mult * sigma_15m;
    if !(frac.is_finite() && frac > 0.0) {
        return entry;
    }
    let Some(d) = Decimal::from_f64(frac) else { return entry };
    match Px::new(entry.get() * (Decimal::ONE - d)) {
        Ok(p) => p.snap_down_to_tick(),
        Err(_) => entry, // frac ≥ 1 ⇒ non-positive stop ⇒ guard
    }
}

/// Armed stops per name. Pure and synchronous — the engine (M4) feeds bar lows
/// and acts on breaches; backtest and live share this exact logic.
#[derive(Debug, Default)]
pub struct StopBook {
    stops: HashMap<StockCode, Px>,
}

impl StopBook {
    pub fn arm(&mut self, code: StockCode, stop: Px) {
        self.stops.insert(code, stop);
    }

    pub fn disarm(&mut self, code: StockCode) -> Option<Px> {
        self.stops.remove(&code)
    }

    pub fn stop_of(&self, code: StockCode) -> Option<Px> {
        self.stops.get(&code).copied()
    }

    /// True iff an armed stop exists and `low` touched or pierced it.
    pub fn breached(&self, code: StockCode, low: Px) -> bool {
        self.stops.get(&code).is_some_and(|s| low.get() <= s.get())
    }

    pub fn is_empty(&self) -> bool {
        self.stops.is_empty()
    }

    pub fn len(&self) -> usize {
        self.stops.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn stop_levels_and_snapping() {
        let entry = Px::from_f64_quote(10.0).unwrap();
        let s = protective_stop(entry, 0.01, 2.5);
        assert_eq!(s.get(), dec!(9.75));
        // Off-tick raw stop snaps down: 100·(1−0.0333)=96.67 → tick 0.05 → 96.65.
        let entry = Px::from_f64_quote(100.0).unwrap();
        let s = protective_stop(entry, 0.0333, 1.0);
        assert_eq!(s.get(), dec!(96.65));
        // Degenerate σ ⇒ entry guard.
        assert_eq!(protective_stop(entry, f64::NAN, 2.5), entry);
        assert_eq!(protective_stop(entry, 0.5, 2.5), entry); // ≥100% move
    }

    #[test]
    fn stop_book_touch_rule() {
        let mut book = StopBook::default();
        let code = StockCode(700);
        book.arm(code, Px::from_f64_quote(9.75).unwrap());
        assert!(book.breached(code, Px::from_f64_quote(9.75).unwrap()));  // touch
        assert!(book.breached(code, Px::from_f64_quote(9.74).unwrap()));  // pierce
        assert!(!book.breached(code, Px::from_f64_quote(9.76).unwrap()));
        assert!(!book.breached(StockCode(5), Px::from_f64_quote(1.0).unwrap()));
        book.disarm(code);
        assert!(!book.breached(code, Px::from_f64_quote(1.0).unwrap()));
    }
}
```

```rust
// crates/hkq-risk/src/state.rs
//! The kill switch: a `watch<RiskState>` written by exactly two producers — the
//! CUSUM monitor (hkq-validate, M5) and the operator console — and read by every
//! actor (blueprint). Any transition to Halted ⇒ cancel resting orders and, if
//! past 09:45, run the exit program immediately (engine behavior, M4).
//!
//! `Arc<watch::Sender>` rather than relying on `watch::Sender: Clone` — the
//! two-producer requirement must not depend on tokio version history. Semantics:
//! first reason wins; a halt is never overwritten or downgraded intraday.
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::watch;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum HaltReason {
    /// Operator console command.
    Operator,
    /// §4 CUSUM breach on the rolling IC (pre-registered kill threshold).
    CusumIcBreach,
    /// Nightly reconciliation breach / quarantined partition.
    ReconBreach,
    /// Live feed integrity failure (stale, gapped, or contradictory data).
    FeedIntegrity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RiskState {
    Normal,
    Halted { reason: HaltReason },
}

impl RiskState {
    pub fn halted(&self) -> bool {
        matches!(self, RiskState::Halted { .. })
    }
}

#[derive(Debug, Clone)]
pub struct KillSwitch {
    tx: Arc<watch::Sender<RiskState>>,
}

impl KillSwitch {
    pub fn new() -> (Self, watch::Receiver<RiskState>) {
        let (tx, rx) = watch::channel(RiskState::Normal);
        (Self { tx: Arc::new(tx) }, rx)
    }

    pub fn subscribe(&self) -> watch::Receiver<RiskState> {
        self.tx.subscribe()
    }

    /// Halt with first-reason-wins semantics. Returns true iff THIS call
    /// performed the Normal → Halted transition.
    pub fn halt(&self, reason: HaltReason) -> bool {
        let flipped = self.tx.send_if_modified(|s| {
            if s.halted() {
                false
            } else {
                *s = RiskState::Halted { reason };
                true
            }
        });
        if flipped {
            tracing::error!(?reason, "KILL SWITCH: risk state HALTED");
        }
        flipped
    }

    pub fn current(&self) -> RiskState {
        *self.tx.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn first_reason_wins_and_receivers_observe() {
        let (ks, mut rx) = KillSwitch::new();
        assert_eq!(ks.current(), RiskState::Normal);
        assert!(!rx.borrow().halted());

        // Second producer via clone (the blueprint's two-writer topology).
        let ks2 = ks.clone();
        assert!(ks.halt(HaltReason::Operator));
        assert!(!ks2.halt(HaltReason::CusumIcBreach)); // no overwrite

        rx.changed().await.unwrap();
        assert_eq!(*rx.borrow(), RiskState::Halted { reason: HaltReason::Operator });
        assert_eq!(ks.current(), RiskState::Halted { reason: HaltReason::Operator });
    }
}
```

## `hkq-nightly` — Bars1m backfill wired in

```toml
# crates/hkq-nightly/Cargo.toml
[package]
name = "hkq-nightly"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-data = { path = "../hkq-data" }
tokio.workspace = true
anyhow.workspace = true
chrono.workspace = true
chrono-tz.workspace = true
polars.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
```

```rust
// crates/hkq-nightly/src/main.rs
//! T−1 18:00 nightly job: EOD + flows + mainland prints + 1-minute bars → lake.
//! Usage: hkq-nightly <strategy.toml> [YYYY-MM-DD] [--force]
//!
//! M3 addition: the bars_1m backfill step (closing M2's declared gap — the
//! provider method existed since M1 but nothing called it). Wired in the BINARY,
//! not hkq-data::ingest, so M1 crates stay frozen; per-code failures isolate;
//! the partition write stays atomic and idempotent.
use anyhow::Context;
use chrono::{NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg, ids::StockCode,
               session::{DayKind, TradingCalendar}};
use hkq_data::{cfg::load_sources, eastmoney::{load_ah_map, EastMoneyClient},
               ingest::{NightlyIngest, StepOutcome, StepReport}, lake::{Dataset, Lake},
               provider::{FlowProvider, IntradayFeed, LinkedMarketFeed}, tiger::TigerClient};
use polars::prelude::DataFrame;

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

/// Backfill today's completed 1-minute bars for the whole universe into ONE
/// Bars1m partition. Idempotent skip; per-code failure isolation; a partition
/// is written only if at least one name produced bars.
async fn step_bars_1m(
    lake: &Lake, feed: &TigerClient, codes: &[StockCode], date: NaiveDate, force: bool,
) -> StepReport {
    let name = "bars_1m";
    if !force && lake.exists(Dataset::Bars1m, date) {
        return StepReport { name, outcome: StepOutcome::SkippedExisting };
    }
    tracing::info!(n_codes = codes.len(), %date, "bars_1m backfill starting (rate-limited)");
    let mut acc: Option<DataFrame> = None;
    let mut failures = 0usize;
    for &code in codes {
        match feed.backfill_bars_1m(code, date).await {
            Ok(df) if df.height() > 0 => {
                acc = Some(match acc.take() {
                    None => df,
                    Some(mut a) => match a.vstack_mut(&df) {
                        Ok(_) => a,
                        Err(e) => {
                            tracing::warn!(%code, error = %e, "bars_1m vstack failed; dropping code");
                            failures += 1;
                            a
                        }
                    },
                });
            }
            Ok(_) => tracing::debug!(%code, "no 1m bars (halt/suspension?)"),
            Err(e) => {
                tracing::warn!(%code, error = %e, "bars_1m backfill failed for code");
                failures += 1;
            }
        }
    }
    let outcome = match acc {
        Some(mut df) => match lake.write_partition(Dataset::Bars1m, date, &mut df, "tiger:kline_1m", 1) {
            Ok(()) => StepOutcome::Written { rows: df.height() },
            Err(e) => StepOutcome::Failed(e.to_string()),
        },
        None => StepOutcome::Failed(format!(
            "no 1m bars for any of {} codes ({failures} hard failures)", codes.len()
        )),
    };
    StepReport { name, outcome }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("info".parse()?))
        .init();

    let mut args = std::env::args().skip(1);
    let cfg_path = args.next().unwrap_or_else(|| "config/strategy.toml".into());
    let mut date: Option<NaiveDate> = None;
    let mut force = false;
    for a in args {
        if a == "--force" { force = true; }
        else { date = Some(a.parse().context("date must be YYYY-MM-DD")?); }
    }

    let cfg = StrategyCfg::load(&cfg_path)?;
    let sources = load_sources(&cfg_path)?;
    let calendar = FileCalendar::load(&cfg.ops.calendar_path)?;
    let date = date.unwrap_or_else(|| Utc::now().with_timezone(&Hong_Kong).date_naive());

    if calendar.day_kind(date) == DayKind::Closed {
        tracing::info!(%date, "market closed; nothing to ingest");
        return Ok(());
    }

    let codes = load_universe_codes(&cfg.ops.universe_codes_path)?;
    let tiger = TigerClient::new(sources.tiger.context("[sources.tiger] is required")?)?;

    let em = match (sources.eastmoney, &cfg.ops.ah_map_path) {
        (Some(em_cfg), Some(ah_path)) => Some(EastMoneyClient::new(em_cfg, load_ah_map(ah_path)?)),
        (Some(em_cfg), None) => {
            tracing::warn!("eastmoney configured without ops.ah_map_path; flows only");
            Some(EastMoneyClient::new(em_cfg, [(StockCode(0), String::new())].into_iter().collect()))
        }
        _ => None,
    };
    let ah_codes: Vec<StockCode> = cfg.ops.ah_map_path.as_ref()
        .and_then(|p| load_ah_map(p).ok())
        .map(|m| m.keys().copied().collect())
        .unwrap_or_default();

    let lake = Lake::new(&cfg.ops.lake_root);
    let ingest = NightlyIngest {
        lake: &lake,
        eod: &tiger,
        flows: em.as_ref().map(|e| e as &dyn FlowProvider),
        linked: em.as_ref().map(|e| e as &dyn LinkedMarketFeed),
    };

    let mut report = ingest.run(date, &codes, &ah_codes, force).await;

    // M3: 1-minute bars — the raw material of rv/rv_5d/lav (§3.0), the seasonal
    // volume profiles and IVU (§3.3), and the §3.5 meta-label window returns.
    let bars = step_bars_1m(&lake, &tiger, &codes, date, force).await;
    match &bars.outcome {
        StepOutcome::Failed(e) => tracing::error!(step = bars.name, error = %e, "nightly step FAILED"),
        o => tracing::info!(step = bars.name, outcome = ?o, "nightly step done"),
    }
    report.steps.push(bars);

    // Close reconciliation still needs an INDEPENDENT official-close source
    // (HKEX daily quotes / OMD-C EOD, report §5) — unchanged M1 gap, logged loudly.
    tracing::warn!("recon skipped: independent official-close source not configured (M1 gap)");

    anyhow::ensure!(report.all_ok(), "one or more nightly steps failed: {report:?}");
    tracing::info!(%date, "nightly ingest complete");
    Ok(())
}
```

## Honest gaps and hand-off to Milestone 4

Four gaps, each now a wiring fact rather than a math fact. First, scores persistence: attribution's inputs must be the *live-frozen* stage-2 frame, which only the engine possesses; M4's `PostClose` phase calls `realized_window_returns → daily_ic_row → append_ic_history` and decides the scores-partition dataset (a two-line `Dataset` variant) at the same time — deferring that enum change kept every M1/M2 crate byte-identical this milestone. Second, VHSI: nothing ingests it, so `Candidate.vhsi_tercile` is `None` in practice and `AlphaMap` runs on IVU-global buckets — the degradation is explicit in types, not discovered in production. Third, the kill switch ships with both producer *handles* but only one real producer (the operator path); the CUSUM monitor that computes the §4 statistic and calls `halt(CusumIcBreach)` is `hkq-validate` scope. Fourth, `Candidate.cost_floor_bps` needs a tentative size before sizing exists — the engine estimates participation from `equity/(K·m)` at the reference price, feeds `CostModel::floor_bps`, and re-checks after `size_book`; both calls are already total functions here. Milestone 4 — the engine — is now almost entirely glue: its `ScoreFreeze → Confirmation → Entry` phases are `sector_composite → select_sectors → stage2_scores → ConfirmationTracker → entry_filter → size_book`, its bar handler is `StopBook::breached`, its every actor holds a `watch<RiskState>` receiver, and its `PostClose` is the attribution pipeline above. The decision layer it orchestrates is, as of this milestone, fully built and fully tested.
