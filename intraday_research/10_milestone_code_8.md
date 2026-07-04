 
All nine documents are attached in full, so I worked directly from them — no fetching needed (I have search only, and it isn't required here). M7's hand-off names exactly what remains: "What it lacks is judgment about what the mirror shows: the as-of snapshotting, κ calibration, and benchmark family that turn replays into the report's full §4 promotion verdict — and every one of those now has its input data being written daily." Of those three, the benchmark family is still blocked by data facts (nothing ingests HSI or sector-ETF series — unchanged owner, a data milestone), but the other two are pure math over artifacts the system has been writing since M4–M7. That is the milestone.

# Milestone 8 — The Judgment (as-of state reconstruction + κ calibration: the §4 promotion protocol)

**Why this is next.** M5 pre-registered DSR > 0.95 as *the* promotion gate, and M7 finally made `sr`-carrying trials exist — but every trial the mirror can generate is stamped, by M7's own loud API note, "a mechanical counterfactual with today's learned state." The gate is fed, but with evidence the protocol itself disqualifies: a replayed 2025 day scored by 2026's alpha map is look-ahead by construction, so the binding constraint on §4 is no longer trial *existence* (M7 solved that) but trial *honesty*. Second, every replay PnL — hence every `sr` in the registry, hence the DSR itself — settles at prices computed from κ, which has shipped as a hand-set config prior since M1 while the report's §1 says "calibrated from fills"; fills carrying real prices began existing with M6's signed venue, and the completed-bar reference they must be measured against is in the lake. Both gaps close with zero vendor surface — this milestone lands entirely in `hkq-validate` and the `hkq-backtest` binary while the M6 paper-vs-tiger VERIFY day proceeds operationally. And there is a structural reason it must be built *now, this way*: it requires **no patches to any frozen crate**. The M5/M7 seams — state files keyed off `cfg.ops.lake_root`, the `NightlyState::load(lake, cfg, …)` parameter split, date-bounded ICIR weights — were designed for exactly this consumer, and M8 is the proof-by-composition that the seam architecture was right.

**In scope:** point-in-time state reconstruction in `hkq-validate` (`asof.rs`: the alpha map, regime gate, and AH-β *as the machine had them on morning D*, recomputed from lake partitions strictly before D — reconstruction, not snapshots, because every one of those artifacts is a pure function of dated partitions); the as-of walk-forward mode in `hkq-backtest` (`--asof`: per replayed day, materialize the as-of state into the sandbox `_state`, then run the *unchanged* `run_replay` with its state reads pointed there — trials recorded as kind `walkforward`, the promotion-grade species); the κ calibration job (`kappa.rs` + `hkq-validate fit-kappa`: realized adverse cost per fill against the SimExec-identical completed-bar VWAP reference, through-origin regression on the model's own impact regressor, per-participation-bucket slopes as the √-shape diagnostic, registry-logged — promoted only by an operator edit of `[costs] impact_kappa`, per the M5 governance precedent); and an `asof-state` audit job that materializes one date's reconstruction under `_asof/<date>/`, never touching production `_state`. **Deferred:** the *self-hosted* walk-forward (learning from the sandbox's own replayed history rather than production history — it needs a weights-source seam in the engine and its genuine consumer is config-divergent experimentation under CPCV/§3.7); Hansen SPA benchmarks, VHSI, A50, quote/VCM replay, and `hkq-recon` (all unchanged data-blocked deferrals with unchanged owners); promotion *enforcement* (governance stays with the operator, by design, unchanged).

Engineering decisions beyond the blueprint sketch, briefly. **Reconstruction beats snapshotting**: a snapshot job would only cover dates after it first ran, while refitting on data `< D` works retroactively over the whole recorded history and is deterministic — same lake, same state, every time. The reconstruction is **shadow-anchored and mostly exact**: the engine's PostClose refit at D−1 fit the alpha map on scores ≤ D−1, which is precisely `fit(scores < D)`, so the as-of alpha is bit-faithful to what live had; ICIR weights were made as-of at the source by M7's `load_weights` patch and need no file at all; AH-β reuses the M5 fit over the same trailing window ending at D−1; only the gate carries a documented cadence caveat — the operator's actual quarterly refresh timing is not a lake fact, so the reconstruction is the "refit nightly" counterfactual with the *same* refusal floors the quarterly job applies (and the registry has logged every real `quarterly_fit`'s matrix since M5, so a cadence-faithful replay is a named refinement, not a lost cause). **No engine surgery** is the load-bearing observation: inside `NightlyState::load`, every windowed input (enriched panels, `iev_bar20`, `sb_z`) is already point-in-time via the `lake` parameter and its ≤ prev-day windows; the *only* today-state inputs are the three `_state` files, and all three are keyed off `cfg.ops.lake_root` — so handing `run_replay` a config clone whose `lake_root` is the sandbox root (while panels still read from the `prod_lake` parameter) makes the entire assembly as-of. The materializer **always rewrites all three files atomically**, so a stale artifact from a previous date can never leak into the next replayed day; the sandbox's own PostClose alpha refit harmlessly no-ops (the sandbox has no `Bars1m`, the M4 refit warns and skips — already true of every M7 replay). For κ: the reference is **SimExec's rule verbatim** — the latest *completed* bar's VWAP (turnover/volume, close fallback) at fill time, i.e. the bar opening one minute before the fill's own — and σ_cc/ADV come from the *previous* panel row, the numbers the machine itself had that morning; fills that can't be referenced honestly (no prior bar, unwarmed σ/ADV, no daily-bar row yet — run after `hkq-nightly`) are dropped and counted, never imputed. The estimator is the through-origin slope on x = σ_cc·√(q/ADV)·10⁴ so the coefficient *is* κ in the model's own units, with a free-intercept diagnostic (a persistent intercept blames the half-spread estimate, not κ) and equal-count participation buckets (flat bucket slopes ⇒ the √-impact shape holds). The Fills schema carries no venue tag, so the job documents its one honesty requirement loudly: point it at a lake whose fills came from the venue you mean to calibrate — paper fills execute at their own limit and measure mark noise, not impact. Duplicated constants (`alpha_state_path`, `ALPHA_MIN_OBS`) follow the M5 precedent with the same comment: the dependency arrow runs live-binary → {engine, validate}, never between them.

```text
hkq/
├── Cargo.toml                        (unchanged — M8 adds no workspace surface)
└── crates/
    ├── hkq-validate/
    │   └── src/{lib,error,asof,kappa,main}.rs
    │                                 (asof.rs, kappa.rs NEW; lib/error/main updated;
    │                                  cfg, splits, stats, dsr, cusum, ic, pnl,
    │                                  registry, fits stay byte-identical)
    └── hkq-backtest/
        └── src/main.rs               (updated: --asof walk-forward mode)
```

## Workspace

`Cargo.toml` is byte-identical to M7: no new members, no new dependencies, no new polars features, and — for the first time since M1 — **no patches to any frozen crate**. Both crate manifests are also unchanged. That is itself the milestone's structural claim: the promotion protocol is pure composition over seams that already exist.

## `hkq-validate` — updated shell files

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
    /// M8: the as-of alpha reconstruction runs hkq-signal's own fit.
    #[error(transparent)]
    Signal(#[from] hkq_signal::SignalError),
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
// crates/hkq-validate/src/lib.rs
#![forbid(unsafe_code)]
//! The §4 protocol as a crate, not a notebook: purged splits, NW t-stats, DSR
//! against an honest trials registry, the CUSUM kill producer, the quarterly
//! estimation jobs whose outputs the engine has consumed as priors since M2 —
//! and, as of M8, the promotion protocol proper: point-in-time reconstruction
//! of the learned state (the honest half of a walk-forward the M7 mirror was
//! missing) and the κ calibration that turns the §1 cost model from prior into
//! estimate.
//!
//! Design invariants:
//! - Every statistic is a pure function of frames/slices; the ONLY I/O is the
//!   lake (read), the `_state` directory (fit artifacts, CUSUM state), and the
//!   hash-chained trials registry (append).
//! - Monitoring and reconstruction consume the SAME persisted artifacts the
//!   learning loops train on: no recomputed morning factors, no fabricated
//!   ICs, no imputed fills — the M3/M4 honesty rule, everywhere.
//! - Degradation is typed: missing history ⇒ `Insufficient` (callers continue,
//!   loudly); schema drift ⇒ polars errors (nobody continues).
//! - The CUSUM breach LATCHES. Un-halting is an operator edit of the state
//!   file, never code. That is what "pre-registered kill threshold" means.
//! - Scalar promotions (θ, v*, κ) are OPERATOR config edits. Jobs report and
//!   registry-log; they never mutate strategy.toml or production `_state`.

pub mod asof;
pub mod cfg;
pub mod cusum;
pub mod dsr;
pub mod error;
pub mod fits;
pub mod ic;
pub mod kappa;
pub mod pnl;
pub mod registry;
pub mod splits;
pub mod stats;

pub use asof::{materialize_asof_state, AsofReport};
pub use cfg::{load_validate, ValidateCfg};
pub use cusum::{startup_gate, CusumOutcome, CusumParams, CusumState};
pub use dsr::{deflated_sharpe, expected_max_sharpe};
pub use error::ValidateError;
pub use kappa::{fit_kappa, kappa_panel, KappaFit};
pub use registry::TrialsRegistry;
pub use splits::{purged_walk_forward, Split};
```

## `hkq-validate` — the as-of reconstruction

```rust
// crates/hkq-validate/src/asof.rs
//! Point-in-time state reconstruction (M8). The M7 mirror replays any recorded
//! day through the identical code path — but it consumes TODAY'S learned state
//! (alpha map, regime gate, AH-β), so its verdicts are current-state
//! counterfactuals, branded as such on the replay API. This module closes that
//! gap with no snapshot infrastructure at all: every learned artifact the
//! engine loads at PreMarket is a PURE FUNCTION of dated lake partitions, so
//! the state the machine had on morning D is RECOMPUTABLE — fit the same
//! functions on data STRICTLY BEFORE D:
//!
//! - alpha map:  `AlphaMap::fit` over Scores ⨝ realized, date < D. The engine's
//!   PostClose refit at D−1 fit on scores ≤ D−1 — the SAME panel — so this is
//!   bit-faithful to what live had, including the min-obs cold-start floor.
//! - regime gate: the M5 per-tercile fit, date < D, with the SAME refusal
//!   floors as the quarterly job (identity below them). Cadence caveat, stated
//!   once: the operator's actual quarterly refresh timing is not a lake fact;
//!   this reconstructs the "refit nightly" counterfactual, the only gate that
//!   is a pure function of the lake. (The registry has logged every real
//!   quarterly_fit matrix since M5 — a cadence-faithful replay of those
//!   records is a named refinement, not built here.)
//! - AH-β: the M5 regression over the same trailing window, ending at D−1.
//! - ICIR weights: NO file needed — M7 date-bounded `load_weights` at the
//!   source, so every replay is already as-of for weights.
//!
//! Reconstruction beats literal snapshotting: it works retroactively over the
//! whole recorded history (a snapshot job would only cover dates after it
//! first ran) and it is deterministic — same lake, same state, every time.
//!
//! `materialize_asof_state` writes the three files into `<state_root>/_state/`
//! — the EXACT filenames PreMarket loads — always all three, atomically, so a
//! stale artifact from a previous materialization can never leak into the next
//! replayed day. The walk-forward driver (hkq-backtest --asof) points the
//! replay's state reads at that root; production `_state` is never touched by
//! anything in this module.
use crate::cfg::ValidateCfg;
use crate::error::ValidateError;
use crate::fits;
use chrono::{Duration, NaiveDate};
use hkq_core::session::SessionTimes;
use hkq_data::lake::{Dataset, Lake};
use hkq_factors::cols::{self, base};
use hkq_factors::stage2::RegimeGate;
use hkq_signal::alpha::VHSI_TERCILE;
use hkq_signal::attribution::realized_window_returns;
use hkq_signal::AlphaMap;
use polars::prelude::*;
use std::path::{Path, PathBuf};

/// PreMarket's alpha-map location, duplicated by design (M5 precedent: the
/// dependency arrow runs live-binary → {engine, validate}, never between them).
pub fn alpha_state_path(lake_root: &Path) -> PathBuf {
    lake_root.join("_state").join("alpha_map.json")
}

/// The engine's PostClose refit floor (hkq-engine::premarket::ALPHA_MIN_OBS),
/// duplicated by design: reconstruction must apply the SAME floor the live
/// refit applied, or the as-of map would warm up on dates where history didn't.
pub const ALPHA_MIN_OBS: usize = 60;

/// AH-β observation floor — the value the M5 `fit-quarterly` job applies.
pub const AH_MIN_OBS: usize = 60;

/// The alpha map as of `date`'s 08:45. Missing history ⇒ the conservative
/// default (predicts 0 bps ⇒ the §3.5 gate trades nothing) — the machine's own
/// documented cold start, reproduced rather than papered over.
pub fn asof_alpha(lake: &Lake, date: NaiveDate) -> Result<AlphaMap, ValidateError> {
    let (Ok(scores), Ok(bars)) = (lake.scan(Dataset::Scores), lake.scan(Dataset::Bars1m)) else {
        return Ok(AlphaMap::default()); // no shadow history yet: cold start
    };
    let s = SessionTimes::get();
    let panel = scores
        .select([col(base::DATE), col(base::CODE), col(cols::SCORE), col(cols::IVU_TERCILE)])
        .filter(col(base::DATE).lt(lit(date.to_string())))
        .join(
            realized_window_returns(bars, s.entry, s.exit_end),
            [col(base::CODE), col(base::DATE)],
            [col(base::CODE), col(base::DATE)],
            JoinArgs::new(JoinType::Inner),
        )
        .with_column(lit(NULL).cast(DataType::UInt32).alias(VHSI_TERCILE))
        .collect()?;
    Ok(AlphaMap::fit(&panel, ALPHA_MIN_OBS)?)
}

/// The regime gate as of `date`: identity below the SAME floors the quarterly
/// job applies (`vcfg.fit_min_obs` rows, `fits::GATE_MIN_DATES` distinct dates).
pub fn asof_gate(
    lake: &Lake, vcfg: &ValidateCfg, date: NaiveDate,
) -> Result<RegimeGate, ValidateError> {
    let panel = match fits::scores_realized_panel(lake) {
        Ok(p) => p,
        Err(ValidateError::Insufficient(_)) => return Ok(RegimeGate::default()),
        Err(e) => return Err(e),
    };
    let panel = panel
        .lazy()
        .filter(col(base::DATE).lt(lit(date.to_string())))
        .collect()?;
    let n_dates = fits::date_groups(&panel)?.len();
    if panel.height() < vcfg.fit_min_obs || n_dates < fits::GATE_MIN_DATES {
        tracing::info!(rows = panel.height(), dates = n_dates, %date,
            "as-of gate below fit floors — IDENTITY (the cold start history had)");
        return Ok(RegimeGate::default());
    }
    fits::fit_regime_gate(&panel)
}

/// Per-name AH βs as of `date`: the M5 fit over `vcfg.fit_window_days`, ending
/// the day before `date`. Empty frame on missing/short history — the engine's
/// per-name ρ = 1 prior stands, exactly as it did historically.
pub fn asof_ah_beta(
    lake: &Lake, vcfg: &ValidateCfg, date: NaiveDate,
) -> Result<DataFrame, ValidateError> {
    let empty = || -> Result<DataFrame, ValidateError> {
        Ok(df!(
            base::CODE => Vec::<u32>::new(),
            "ah_beta" => Vec::<f64>::new(),
            "n_obs" => Vec::<u32>::new(),
        )?)
    };
    let Some(to) = date.pred_opt() else { return empty() };
    let from = to - Duration::days(vcfg.fit_window_days);
    match fits::ah_panel(lake, from, to).and_then(|p| fits::fit_ah_beta(&p, AH_MIN_OBS)) {
        Ok(df) => Ok(df),
        Err(ValidateError::Insufficient(_)) => empty(),
        Err(e) => Err(e),
    }
}

#[derive(Debug, Clone)]
pub struct AsofReport {
    pub date: NaiveDate,
    /// Fitted alpha buckets (0 ⇒ conservative map: the replayed day trades nothing).
    pub alpha_buckets: usize,
    pub gate_is_identity: bool,
    /// Names with a fitted β (all others keep the ρ = 1 prior).
    pub ah_names: usize,
}

/// Rebuild and write the COMPLETE as-of state for `date` into
/// `<state_root>/_state/` — the exact files PreMarket loads. All three are
/// always (re)written, atomically, so nothing stale survives between dates.
/// `state_root` is a sandbox root by contract; this function knows nothing of
/// (and can never touch) the production `_state`.
pub fn materialize_asof_state(
    prod: &Lake, vcfg: &ValidateCfg, date: NaiveDate, state_root: &Path,
) -> Result<AsofReport, ValidateError> {
    let alpha = asof_alpha(prod, date)?;
    let gate = asof_gate(prod, vcfg, date)?;
    let mut ahb = asof_ah_beta(prod, vcfg, date)?;

    let path = alpha_state_path(state_root);
    let dir = path.parent().ok_or(ValidateError::Contract("alpha path has no parent"))?;
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(".alpha_map.tmp");
    std::fs::write(&tmp, serde_json::to_vec_pretty(&alpha)?)?;
    std::fs::rename(&tmp, &path)?;

    fits::save_gate(state_root, &gate)?;
    fits::save_ah_beta(state_root, &mut ahb)?;

    let report = AsofReport {
        date,
        alpha_buckets: alpha.coef.len(),
        gate_is_identity: gate.g == RegimeGate::default().g,
        ah_names: ahb.height(),
    };
    tracing::info!(%date, alpha_buckets = report.alpha_buckets,
        gate_identity = report.gate_is_identity, ah_names = report.ah_names,
        root = %state_root.display(), "as-of state materialized");
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;
    use hkq_core::session::hk;
    use polars::df;
    use std::path::PathBuf;

    fn tmp(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hkq_asof_{tag}_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    fn ts(d: NaiveDate, h: u32, m: u32) -> i64 {
        hk(d, NaiveTime::from_hms_opt(h, m, 0).unwrap()).timestamp_millis()
    }

    /// One scored day: 40 names, score_i = i/100, label r = 0.001 + slope·score,
    /// realized via first/last closes inside the §3.5 window.
    fn write_scored_day(lake: &Lake, d: NaiveDate, slope: f64) {
        let n = 40u32;
        let codes: Vec<u32> = (1..=n).collect();
        let scores: Vec<f64> = (1..=n).map(|i| i as f64 * 0.01).collect();
        let mut sdf = df!(
            "date" => vec![d.to_string(); n as usize],
            "code" => codes.clone(),
            "score" => scores.clone(),
            "ivu_tercile" => vec![1u32; n as usize],
        ).unwrap();
        lake.write_partition(Dataset::Scores, d, &mut sdf, "test", 1).unwrap();
        let (mut c_, mut t_, mut cl) = (vec![], vec![], vec![]);
        for (i, code) in codes.iter().enumerate() {
            let r = 0.001 + slope * scores[i];
            c_.push(*code); t_.push(ts(d, 9, 45)); cl.push(100.0);
            c_.push(*code); t_.push(ts(d, 15, 44)); cl.push(100.0 * r.exp());
        }
        let mut bdf = df!(
            "code" => c_,
            "date" => vec![d.to_string(); 2 * n as usize],
            "ts_ms" => t_,
            "c" => cl,
        ).unwrap();
        lake.write_partition(Dataset::Bars1m, d, &mut bdf, "test", 1).unwrap();
    }

    #[test]
    fn alpha_asof_is_bounded_strictly_before_date() {
        let root = tmp("alpha");
        let lake = Lake::new(&root);
        let d1 = NaiveDate::from_ymd_opt(2026, 6, 29).unwrap();
        let d2 = NaiveDate::from_ymd_opt(2026, 6, 30).unwrap();
        let d3 = NaiveDate::from_ymd_opt(2026, 7, 2).unwrap(); // regime break AFTER the as-of date
        write_scored_day(&lake, d1, 0.5);
        write_scored_day(&lake, d2, 0.5);
        write_scored_day(&lake, d3, -0.5);

        // As of 2026-07-01: only the slope-0.5 days are the past.
        let asof = asof_alpha(&lake, NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()).unwrap();
        let e = asof.expected_alpha_bps(None, 1, 0.02);
        assert!((e - (0.001 + 0.5 * 0.02) * 1e4).abs() < 1.0, "asof bps {e}");

        // As of 2026-07-03 the flipped day joins the fit: the map must differ.
        let later = asof_alpha(&lake, NaiveDate::from_ymd_opt(2026, 7, 3).unwrap()).unwrap();
        assert!(later.expected_alpha_bps(None, 1, 0.02) < e - 5.0);

        // Before any history: the conservative cold start, not an error.
        let cold = asof_alpha(&lake, d1).unwrap();
        assert_eq!(cold.expected_alpha_bps(None, 1, 1.0), 0.0);
        std::fs::remove_dir_all(root).ok();
    }

    #[test]
    fn gate_asof_identity_below_floors() {
        let root = tmp("gate");
        let lake = Lake::new(&root);
        let vcfg = ValidateCfg::default();
        // No scores history at all ⇒ Insufficient ⇒ identity, never an error.
        let g = asof_gate(&lake, &vcfg, NaiveDate::from_ymd_opt(2026, 7, 1).unwrap()).unwrap();
        assert_eq!(g.g, RegimeGate::default().g);
        std::fs::remove_dir_all(root).ok();
    }

    #[test]
    fn ah_beta_asof_recovers_and_excludes_the_asof_date() {
        let root = tmp("ahb");
        let lake = Lake::new(&root);
        let vcfg = ValidateCfg::default();
        // 70 consecutive days of exact β = 1.3 structure: r_on = 1.3·a exactly
        // (open_t = close_{t−1}·exp(1.3·a_t), flat intraday, adj == close).
        let mut d = NaiveDate::from_ymd_opt(2026, 3, 2).unwrap();
        let mut close_prev = 100.0f64;
        for i in 0..70 {
            let a = if i % 2 == 0 { 0.01 } else { -0.008 };
            let open = close_prev * (1.3f64 * a).exp();
            let mut daily = df!(
                "code" => vec![941u32],
                "date" => vec![d.to_string()],
                "open" => vec![open], "high" => vec![open], "low" => vec![open],
                "close" => vec![open], "adj_close" => vec![open],
                "volume" => vec![1.0e6], "turnover" => vec![1.0e8],
            ).unwrap();
            lake.write_partition(Dataset::DailyBars, d, &mut daily, "test", 1).unwrap();
            let mut prints = df!(
                "code" => vec![941u32],
                "a_open_ret" => vec![a],
                "date" => vec![d.to_string()],
            ).unwrap();
            lake.write_partition(Dataset::MainlandPrints, d, &mut prints, "test", 1).unwrap();
            close_prev = open;
            d = d.succ_opt().unwrap();
        }
        // Poison the as-of date itself: β would explode if it leaked in.
        let poison = d;
        let open = close_prev * (-25.0f64 * 0.01).exp();
        let mut daily = df!(
            "code" => vec![941u32], "date" => vec![poison.to_string()],
            "open" => vec![open], "high" => vec![open], "low" => vec![open],
            "close" => vec![open], "adj_close" => vec![open],
            "volume" => vec![1.0e6], "turnover" => vec![1.0e8],
        ).unwrap();
        lake.write_partition(Dataset::DailyBars, poison, &mut daily, "test", 1).unwrap();
        let mut prints = df!(
            "code" => vec![941u32], "a_open_ret" => vec![0.01],
            "date" => vec![poison.to_string()],
        ).unwrap();
        lake.write_partition(Dataset::MainlandPrints, poison, &mut prints, "test", 1).unwrap();

        let out = asof_ah_beta(&lake, &vcfg, poison).unwrap();
        assert_eq!(out.height(), 1);
        let b = out.column("ah_beta").unwrap().as_materialized_series()
            .f64().unwrap().get(0).unwrap();
        assert!((b - 1.3).abs() < 0.05, "beta {b}");
        std::fs::remove_dir_all(root).ok();
    }

    #[test]
    fn materialize_always_overwrites_all_three() {
        let prod_root = tmp("mat_prod");
        let state_root = tmp("mat_state");
        let prod = Lake::new(&prod_root);
        let vcfg = ValidateCfg::default();
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();

        // Plant a STALE warm alpha where the sandbox `_state` will live.
        std::fs::create_dir_all(state_root.join("_state")).unwrap();
        std::fs::write(
            alpha_state_path(&state_root),
            serde_json::to_vec_pretty(&AlphaMap::flat(100.0)).unwrap(),
        ).unwrap();

        // Cold production lake ⇒ conservative state must REPLACE the stale file.
        let rep = materialize_asof_state(&prod, &vcfg, d, &state_root).unwrap();
        assert_eq!(rep.alpha_buckets, 0);
        assert!(rep.gate_is_identity);
        assert_eq!(rep.ah_names, 0);
        let back: AlphaMap = serde_json::from_slice(
            &std::fs::read(alpha_state_path(&state_root)).unwrap()).unwrap();
        assert_eq!(back.expected_alpha_bps(None, 1, 5.0), 0.0);
        assert!(state_root.join("_state").join("regime_gate.json").exists());
        assert!(state_root.join("_state").join("ah_beta.parquet").exists());
        std::fs::remove_dir_all(prod_root).ok();
        std::fs::remove_dir_all(state_root).ok();
    }
}
```

## `hkq-validate` — the κ calibration

```rust
// crates/hkq-validate/src/kappa.rs
//! κ calibration (M8; report §1: "impact_kappa — κ, calibrated from fills";
//! M7 hand-off item three). The §4 cost model prices every simulated fill at
//!
//!   px = ref · (1 ± (s/2 + κ·σ_cc·√(q/ADV)) / 10⁴)
//!
//! and κ has shipped as a config PRIOR since M1. Real fills carry real prices;
//! the same-convention reference — the latest COMPLETED 1-minute bar's VWAP at
//! fill time, SimExec's exact rule — is in the lake; σ_cc and ADV are the §1
//! per-name inputs the machine itself used that morning. Each fill therefore
//! yields one honest observation of realized adverse cost:
//!
//!   e = sign·(px/ref − 1)·10⁴ − s/2      (bps beyond the half-spread)
//!   x = σ_cc·√(q/ADV)·10⁴                (the model's impact regressor)
//!
//! and κ̂ is the through-origin slope Σex/Σx², with per-participation-bucket
//! slopes as the shape diagnostic (flat buckets ⇒ the √-impact law holds) and
//! a free-intercept fit as the attribution diagnostic (a persistent intercept
//! blames the half-spread estimate, not κ).
//!
//! Governance per the M5 precedent: this job REPORTS and registry-logs κ̂;
//! only an operator edit of `[costs] impact_kappa` changes behavior. One
//! honesty requirement, stated loudly: the Fills schema carries no venue tag,
//! so point this job at a lake whose fills came from the venue you mean to
//! calibrate — paper fills execute at their own limit and measure mark noise,
//! not impact. Run AFTER hkq-nightly: fills on dates without a daily-bar row
//! cannot be given prior-day σ/ADV and are dropped (and counted), not imputed.
use crate::error::ValidateError;
use chrono::{Duration, NaiveDate};
use hkq_core::config::StrategyCfg;
use hkq_data::lake::{Dataset, Lake};
use hkq_factors::cols::{self, base};
use hkq_factors::panel::PanelBuilder;
use polars::prelude::*;

/// Realized adverse cost beyond the half-spread, in bps.
pub const E_ADVERSE: &str = "e_adverse_bps";
/// The model's impact regressor σ_cc·√(q/ADV)·10⁴ — the slope on it IS κ.
pub const X_IMPACT: &str = "x_impact";
pub const PARTICIPATION: &str = "participation";
/// Statics contract column (OpsCfg.universe_static_path schema).
const SPREAD_BPS: &str = "spread_bps";
/// Same calendar warmup the engine uses before σ/ADV's first consumer.
const WARMUP_CAL_DAYS: i64 = 420;
/// Below this many usable fills the fit refuses: a κ from a handful of fills
/// is a prior wearing a costume.
pub const KAPPA_MIN_FILLS: usize = 30;

/// One row per matched fill: [date, code, side, shares, participation,
/// x_impact, e_adverse_bps]. Reference = the latest COMPLETED bar's VWAP at
/// fill time; σ_cc/ADV = the PREVIOUS panel row — the numbers the machine had
/// that morning. Unmatchable fills are dropped and counted, never imputed.
pub fn kappa_panel(lake: &Lake, cfg: &StrategyCfg) -> Result<DataFrame, ValidateError> {
    let fills = lake.scan(Dataset::Fills)
        .map_err(|_| ValidateError::Insufficient("no fills history"))?
        .select([
            col(base::CODE), col(base::DATE), col(base::TS_MS), col("side"),
            col("shares").cast(DataType::Float64), col("px"),
        ])
        // Latest completed bar at fill time = the bar OPENING one minute before
        // the fill's own bar. (Positive epoch ms ⇒ the Int64 cast floors,
        // whether `/` is int or float division — the M2 binning convention.)
        .with_column(
            ((col(base::TS_MS) / lit(60_000i64)).cast(DataType::Int64) * lit(60_000i64)
                - lit(60_000i64))
            .alias("__ref_open"),
        );

    let refs = lake.scan(Dataset::Bars1m)
        .map_err(|_| ValidateError::Insufficient("no bars_1m history"))?
        .select([
            col(base::CODE), col(base::DATE),
            col(base::TS_MS).alias("__ref_open"),
            // SimExec's reference rule, verbatim: VWAP when sane, else close.
            when(col(base::TURNOVER).gt(lit(0.0)).and(col(base::VOLUME).gt(lit(0.0))))
                .then(col(base::TURNOVER) / col(base::VOLUME))
                .otherwise(col(cols::C1M))
                .alias("__ref"),
        ]);

    let joined = fills
        .join(
            refs,
            [col(base::CODE), col(base::DATE), col("__ref_open")],
            [col(base::CODE), col(base::DATE), col("__ref_open")],
            JoinArgs::new(JoinType::Left),
        )
        .collect()?;
    let n_all = joined.height();
    if n_all == 0 {
        return Err(ValidateError::Insufficient("fills dataset is empty"));
    }

    // Fill-date range → the enriched-panel window (with engine-style warmup).
    let dstr = joined.column(base::DATE)?.as_materialized_series().str()?.clone();
    let (mut lo, mut hi): (Option<NaiveDate>, Option<NaiveDate>) = (None, None);
    for i in 0..n_all {
        let Some(d) = dstr.get(i).and_then(|s| s.parse::<NaiveDate>().ok()) else { continue };
        lo = Some(lo.map_or(d, |x| x.min(d)));
        hi = Some(hi.map_or(d, |x| x.max(d)));
    }
    let (Some(lo), Some(hi)) = (lo, hi) else {
        return Err(ValidateError::Insufficient("no parseable fill dates"));
    };

    // σ_cc / ADV as KNOWN on the fill morning: the previous panel row, exactly
    // the slice PreMarket hands the engine (row t carries t−1's values here).
    let pb = PanelBuilder::new(lake, &cfg.factors);
    let known = pb
        .enriched_daily(lo - Duration::days(WARMUP_CAL_DAYS), hi, None)?
        .select([col(base::CODE), col(base::DATE), col(cols::SIGMA_CC), col(cols::ADV_SHARES)])
        .sort_by_exprs([col(base::CODE), col(base::DATE)], Default::default())
        .with_columns([
            col(cols::SIGMA_CC).shift(lit(1)).over([col(base::CODE)]).alias("__sigma_known"),
            col(cols::ADV_SHARES).shift(lit(1)).over([col(base::CODE)]).alias("__adv_known"),
        ])
        .select([col(base::CODE), col(base::DATE), col("__sigma_known"), col("__adv_known")]);

    // Per-name static spread; absent file/column ⇒ the conservative config
    // fallback — the SAME default SimExec and the candidate cost floor use.
    let spread_fallback = cfg.universe.max_median_spread_bps;
    let statics: Option<DataFrame> = match &cfg.ops.universe_static_path {
        Some(p) => {
            let scan = LazyFrame::scan_parquet(
                p.to_string_lossy().as_ref(),
                ScanArgsParquet {
                    hive_options: HiveOptions { enabled: Some(false), ..Default::default() },
                    ..Default::default()
                },
            );
            match scan.and_then(|lf| lf.select([col(base::CODE), col(SPREAD_BPS)]).collect()) {
                Ok(df) => Some(df),
                Err(e) => {
                    tracing::warn!(error = %e,
                        "statics unreadable; s/2 uses the conservative config fallback");
                    None
                }
            }
        }
        None => None,
    };

    let mut lf = joined.lazy().join(
        known,
        [col(base::CODE), col(base::DATE)],
        [col(base::CODE), col(base::DATE)],
        JoinArgs::new(JoinType::Left),
    );
    lf = match statics {
        Some(s) => lf.join(s.lazy(), [col(base::CODE)], [col(base::CODE)],
                           JoinArgs::new(JoinType::Left)),
        None => lf.with_column(lit(NULL).cast(DataType::Float64).alias(SPREAD_BPS)),
    };

    let sign = when(col("side").eq(lit("sell"))).then(lit(-1.0)).otherwise(lit(1.0));
    let out = lf
        .with_column(col(SPREAD_BPS).fill_null(lit(spread_fallback)).alias(SPREAD_BPS))
        .with_columns([
            (sign * (col("px") / col("__ref") - lit(1.0)) * lit(1e4)
                - col(SPREAD_BPS) / lit(2.0))
                .alias(E_ADVERSE),
            (col("shares") / col("__adv_known")).alias(PARTICIPATION),
            (col("__sigma_known") * (col("shares") / col("__adv_known")).sqrt() * lit(1e4))
                .alias(X_IMPACT),
        ])
        .filter(
            col("__ref").gt(lit(0.0))
                .and(col("px").gt(lit(0.0)))
                .and(col("__sigma_known").gt(lit(0.0)))
                .and(col("__adv_known").gt(lit(0.0)))
                .and(col("shares").gt(lit(0.0))),
        )
        .select([col(base::DATE), col(base::CODE), col("side"), col("shares"),
                 col(PARTICIPATION), col(X_IMPACT), col(E_ADVERSE)])
        .collect()?;

    let dropped = n_all - out.height();
    if dropped > 0 {
        tracing::warn!(matched = out.height(), dropped,
            "kappa panel: fills without an honest reference/σ/ADV were dropped");
    }
    if out.height() == 0 {
        return Err(ValidateError::Insufficient(
            "no fill matched a completed bar + warmed σ/ADV (run hkq-nightly first?)"));
    }
    Ok(out)
}

#[derive(Debug, Clone)]
pub struct BucketFit {
    pub part_lo: f64,
    pub part_hi: f64,
    pub n: usize,
    /// Within-bucket through-origin slope — flat across buckets ⇒ √-impact holds.
    pub kappa: f64,
}

#[derive(Debug, Clone)]
pub struct KappaFit {
    pub kappa: f64,
    pub se: f64,
    pub t: f64,
    pub n: usize,
    /// Free-intercept diagnostics: a persistent non-zero intercept means the
    /// half-spread term is misestimated — do not launder it into κ.
    pub intercept_bps: f64,
    pub slope_free: f64,
    pub buckets: Vec<BucketFit>,
}

/// Through-origin κ̂ = Σex/Σx² with SE, t, free-intercept diagnostics, and
/// equal-count participation buckets. None below KAPPA_MIN_FILLS usable rows.
pub fn fit_kappa(panel: &DataFrame, n_buckets: usize) -> Result<Option<KappaFit>, ValidateError> {
    let e = panel.column(E_ADVERSE)?.as_materialized_series().f64()?.clone();
    let x = panel.column(X_IMPACT)?.as_materialized_series().f64()?.clone();
    let p = panel.column(PARTICIPATION)?.as_materialized_series().f64()?.clone();
    let mut rows: Vec<(f64, f64, f64)> = Vec::with_capacity(panel.height());
    for i in 0..panel.height() {
        if let (Some(ei), Some(xi), Some(pi)) = (e.get(i), x.get(i), p.get(i)) {
            if ei.is_finite() && xi.is_finite() && pi.is_finite() && xi > 0.0 {
                rows.push((ei, xi, pi));
            }
        }
    }
    let n = rows.len();
    if n < KAPPA_MIN_FILLS {
        return Ok(None);
    }
    let sxx: f64 = rows.iter().map(|(_, x, _)| x * x).sum();
    let sxe: f64 = rows.iter().map(|(e, x, _)| e * x).sum();
    if !(sxx > 0.0) {
        return Ok(None);
    }
    let kappa = sxe / sxx;
    let ssr: f64 = rows.iter().map(|(e, x, _)| (e - kappa * x).powi(2)).sum();
    let se = (ssr / (n as f64 - 1.0) / sxx).sqrt();
    let t = if se > 0.0 { kappa / se } else { f64::INFINITY };

    // Free-intercept OLS — the "is s/2 right?" diagnostic.
    let nf = n as f64;
    let mx = rows.iter().map(|(_, x, _)| x).sum::<f64>() / nf;
    let me = rows.iter().map(|(e, _, _)| e).sum::<f64>() / nf;
    let (mut cxx, mut cxe) = (0.0f64, 0.0f64);
    for (e, x, _) in &rows {
        cxx += (x - mx) * (x - mx);
        cxe += (x - mx) * (e - me);
    }
    let slope_free = if cxx > 1e-12 { cxe / cxx } else { kappa };
    let intercept_bps = me - slope_free * mx;

    // Equal-count participation buckets, sorted by q/ADV.
    let mut by_part = rows.clone();
    by_part.sort_by(|a, b| a.2.total_cmp(&b.2));
    let nb = n_buckets.max(1).min(n);
    let mut buckets = Vec::with_capacity(nb);
    for b in 0..nb {
        let lo_i = b * n / nb;
        let hi_i = (((b + 1) * n / nb).max(lo_i + 1)).min(n);
        let seg = &by_part[lo_i..hi_i];
        let bxx: f64 = seg.iter().map(|(_, x, _)| x * x).sum();
        let bxe: f64 = seg.iter().map(|(e, x, _)| e * x).sum();
        if bxx > 0.0 {
            buckets.push(BucketFit {
                part_lo: seg.first().map(|r| r.2).unwrap_or(0.0),
                part_hi: seg.last().map(|r| r.2).unwrap_or(0.0),
                n: seg.len(),
                kappa: bxe / bxx,
            });
        }
    }
    Ok(Some(KappaFit { kappa, se, t, n, intercept_bps, slope_free, buckets }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveTime};
    use hkq_core::config::*;
    use hkq_core::session::hk;
    use polars::df;

    fn xorshift(s: &mut u64) -> f64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        ((*s >> 11) as f64 / (1u64 << 53) as f64) - 0.5
    }

    #[test]
    fn kappa_recovered_through_origin_with_flat_buckets() {
        let mut s = 0x9E3779B97F4A7C15u64;
        let n = 400;
        let (mut e, mut x, mut p) = (vec![], vec![], vec![]);
        for _ in 0..n {
            let part = (xorshift(&mut s) + 0.5).max(1e-4) * 0.02; // (0, 0.02]
            let xi = 0.02 * part.sqrt() * 1e4;                    // σ_cc = 2%
            let ei = 0.12 * xi + xorshift(&mut s) * 0.5;          // κ_true = 0.12
            p.push(part); x.push(xi); e.push(ei);
        }
        let panel = df!(
            "e_adverse_bps" => e, "x_impact" => x, "participation" => p,
        ).unwrap();
        let f = fit_kappa(&panel, 4).unwrap().unwrap();
        assert!((f.kappa - 0.12).abs() < 0.01, "kappa {}", f.kappa);
        assert!(f.t > 10.0);
        assert!(f.intercept_bps.abs() < 0.5);
        assert_eq!(f.n, 400);
        assert_eq!(f.buckets.len(), 4);
        for b in &f.buckets {
            assert!((b.kappa - 0.12).abs() < 0.05, "bucket kappa {}", b.kappa);
            assert!(b.part_lo <= b.part_hi);
        }
        // Below the mass floor: refuse rather than bless noise.
        assert!(fit_kappa(&panel.head(Some(10)), 4).unwrap().is_none());
    }

    fn cfg_for(root: &std::path::Path) -> StrategyCfg {
        StrategyCfg {
            universe: UniverseCfg { min_median_turnover_hkd: 0.0, min_price_hkd: 0.0,
                min_listed_days: 0, max_median_spread_bps: 35.0 },
            factors: FactorCfg { ewma_halflife_days: 2.0, ewma_min_obs: 2, amihud_window: 3,
                rv_days: 2, lav_gamma: 0.3, seasonal_vol_days: 2, ivu_tercile_window: 4 },
            stage1: Stage1Cfg { theta1: 1.0, theta2: 1.0, eta: 0.25, vs_threshold: 0.5,
                leadlag_window: 250, fdr_q: 0.10, icir_window: 250, icir_shrink_delta: 0.10,
                top_k_sectors: 2, sigma_min_gate: 0.1, member_weight_cap: 0.9 },
            stage2: Stage2Cfg { phi: 2.0, zeta: 0.5, vs_threshold_stock: 0.5, beta_window: 250,
                winsor_pct: 0.01,
                ortho_order: vec!["x5".into(), "x3".into(), "x2".into(), "x1".into(),
                                  "x6".into(), "x7".into()],
                names_per_sector: 2 },
            trade: TradeCfg { margin_bps: 10.0, stop_sigma15m_mult: 2.5,
                participation_cap: 0.02, half_day_mode: HalfDayMode::Skip,
                reuse_unsettled_proceeds: false },
            costs: CostCfg { stamp_bps_per_side: 10.0, fees_bps_roundtrip: 2.2,
                impact_kappa: 0.1 },
            ops: OpsCfg { lake_root: root.into(), calendar_path: root.into(),
                universe_codes_path: root.into(), ah_map_path: None,
                universe_static_path: None, log_json: false },
        }
    }

    #[test]
    fn panel_references_the_prior_completed_bar_and_prior_day_inputs() {
        let root = std::env::temp_dir().join(format!(
            "hkq_kappa_panel_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&root).unwrap();
        let lake = Lake::new(&root);
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();

        // Warmup daily bars (alternating closes ⇒ σ_cc > 0), plus the fill date
        // itself — κ calibration runs AFTER hkq-nightly by contract.
        let mut day = NaiveDate::from_ymd_opt(2026, 6, 19).unwrap();
        let mut days = vec![];
        while days.len() < 10 {
            if !matches!(day.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun) && day < d {
                days.push(day);
            }
            day = day.succ_opt().unwrap();
        }
        days.push(d);
        for (i, dd) in days.iter().enumerate() {
            let px = 10.0 + 0.1 * ((i % 2) as f64);
            let mut daily = df!(
                "code" => vec![700u32],
                "date" => vec![dd.to_string()],
                "open" => vec![px], "high" => vec![px + 0.05], "low" => vec![px - 0.05],
                "close" => vec![px], "adj_close" => vec![px],
                "volume" => vec![1.0e6], "turnover" => vec![px * 1.0e6],
            ).unwrap();
            lake.write_partition(Dataset::DailyBars, *dd, &mut daily, "test", 1).unwrap();
        }

        // Reference bar (09:44, VWAP exactly 10.0 ≠ its close) + the fill-minute bar.
        let ts = |h: u32, m: u32, sec: u32|
            hk(d, NaiveTime::from_hms_opt(h, m, sec).unwrap()).timestamp_millis();
        let mut bars = df!(
            "code" => vec![700u32; 2],
            "date" => vec![d.to_string(); 2],
            "ts_ms" => vec![ts(9, 44, 0), ts(9, 45, 0)],
            "o" => vec![10.0, 10.0], "h" => vec![10.0, 10.1],
            "l" => vec![10.0, 10.0], "c" => vec![10.05, 10.02],
            "volume" => vec![1_000.0, 500.0],
            "turnover" => vec![10_000.0, 5_100.0],
        ).unwrap();
        lake.write_partition(Dataset::Bars1m, d, &mut bars, "test", 1).unwrap();

        // Two fills at 09:45:30 AT the reference price: e must be exactly −s/2
        // on BOTH sides (the sign flips both terms).
        let mut fills = df!(
            "code" => vec![700u32; 2],
            "date" => vec![d.to_string(); 2],
            "ts_ms" => vec![ts(9, 45, 30); 2],
            "side" => vec!["buy".to_string(), "sell".to_string()],
            "shares" => vec![1_000u64, 1_000],
            "px" => vec![10.0, 10.0],
            "duty" => vec![10.0, 10.0],
            "fees" => vec![1.1, 1.1],
        ).unwrap();
        lake.write_partition(Dataset::Fills, d, &mut fills, "test", 1).unwrap();

        let cfg = cfg_for(&root);
        let panel = kappa_panel(&lake, &cfg).unwrap();
        assert_eq!(panel.height(), 2);
        let e = panel.column(E_ADVERSE).unwrap().as_materialized_series().f64().unwrap().clone();
        let x = panel.column(X_IMPACT).unwrap().as_materialized_series().f64().unwrap().clone();
        let p = panel.column(PARTICIPATION).unwrap().as_materialized_series()
            .f64().unwrap().clone();
        for i in 0..2 {
            // px == ref (VWAP 10.0, NOT the 10.05 close) ⇒ e = −s/2 = −17.5
            // (no statics file ⇒ the 35 bps config fallback).
            assert!((e.get(i).unwrap() + 17.5).abs() < 1e-9, "e {:?}", e.get(i));
            // ADV from the PRIOR day's row: constant 1e6 ⇒ q/ADV = 1e-3 exactly.
            assert!((p.get(i).unwrap() - 1.0e-3).abs() < 1e-12);
            assert!(x.get(i).unwrap().is_finite() && x.get(i).unwrap() > 0.0);
        }
        std::fs::remove_dir_all(root).ok();
    }
}
```

## `hkq-validate` — the binary, two new jobs

```rust
// crates/hkq-validate/src/main.rs
//! §4 protocol jobs. Deliberately synchronous — pure batch over the lake.
//!
//! Usage:
//!   hkq-validate <strategy.toml> cusum
//!   hkq-validate <strategy.toml> fit-quarterly [YYYY-MM-DD]
//!   hkq-validate <strategy.toml> report
//!   hkq-validate <strategy.toml> asof-state [YYYY-MM-DD]     (M8: audit artifact)
//!   hkq-validate <strategy.toml> fit-kappa                   (M8: κ from fills)
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
use hkq_validate::{asof, cusum, dsr, fits, kappa, pnl, stats};
use serde_json::json;
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("info".parse()?))
        .init();

    let mut args = std::env::args().skip(1);
    let cfg_path = args.next().context(
        "usage: hkq-validate <strategy.toml> <cusum|fit-quarterly|report|asof-state|fit-kappa> [YYYY-MM-DD]")?;
    let cmd = args.next()
        .context("missing subcommand: cusum | fit-quarterly | report | asof-state | fit-kappa")?;
    let date_arg: Option<NaiveDate> =
        args.next().map(|s| s.parse()).transpose().context("date must be YYYY-MM-DD")?;

    let cfg = StrategyCfg::load(&cfg_path)?;
    let vcfg = load_validate(&cfg_path)?;
    let lake = Lake::new(&cfg.ops.lake_root);

    match cmd.as_str() {
        "cusum" => job_cusum(&lake, &vcfg),
        "fit-quarterly" => job_fit_quarterly(&cfg, &cfg_path, &vcfg, &lake, date_arg),
        "report" => job_report(&vcfg, &lake),
        "asof-state" => job_asof_state(&vcfg, &lake, date_arg),
        "fit-kappa" => job_fit_kappa(&cfg, &cfg_path, &vcfg, &lake),
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
            report.insert("theta".into(),
                json!({ "theta1": t.theta1, "theta2": t.theta2, "n": t.n }));
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

    // ── AH betas (artifact; freeze-side consumption landed in M6) ───────────
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
        "note": "DSR is None until ≥2 registered trials carry an `sr` metric — record your trials. Promotion-grade trials are kind=walkforward (hkq-backtest --asof).",
    });
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

/// M8: audit materialization of one date's reconstructed state. Writes under
/// `<lake_root>/_asof/<date>/_state/…` — NEVER production `_state`, which only
/// the engine's own PostClose refit and the quarterly fit job may touch.
fn job_asof_state(vcfg: &ValidateCfg, lake: &Lake, date_arg: Option<NaiveDate>) -> anyhow::Result<()> {
    let date = date_arg.unwrap_or_else(|| Utc::now().with_timezone(&Hong_Kong).date_naive());
    let root = lake.root().join("_asof").join(date.to_string());
    let rep = asof::materialize_asof_state(lake, vcfg, date, &root)?;
    println!("{}", serde_json::to_string_pretty(&json!({
        "date": rep.date.to_string(),
        "alpha_buckets": rep.alpha_buckets,
        "gate_is_identity": rep.gate_is_identity,
        "ah_names": rep.ah_names,
        "state_root": root.display().to_string(),
        "note": "audit artifact — the walk-forward (hkq-backtest --asof) materializes its own copy per replayed day",
    }))?);
    Ok(())
}

/// M8: κ from fills (§1 "calibrated from fills"). Reports + registry-logs;
/// promotion is an OPERATOR edit of `[costs] impact_kappa` (M5 precedent).
fn job_fit_kappa(
    cfg: &StrategyCfg, cfg_path: &str, vcfg: &ValidateCfg, lake: &Lake,
) -> anyhow::Result<()> {
    let panel = match kappa::kappa_panel(lake, cfg) {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!(error = %e, "kappa panel unavailable — nothing to calibrate");
            return Ok(());
        }
    };
    match kappa::fit_kappa(&panel, 4)? {
        Some(f) => {
            let out = json!({
                "kappa_hat": f.kappa,
                "se": f.se,
                "t": f.t,
                "n_fills": f.n,
                "intercept_bps": f.intercept_bps,
                "slope_free": f.slope_free,
                "buckets": f.buckets.iter().map(|b| json!({
                    "part_lo": b.part_lo, "part_hi": b.part_hi, "n": b.n, "kappa": b.kappa
                })).collect::<Vec<_>>(),
                "current_prior": cfg.costs.impact_kappa,
                "note": "promotion is an OPERATOR edit of [costs] impact_kappa; flat bucket κ's ⇒ the √-impact shape holds; a large intercept blames the half-spread estimate, not κ; calibrate against a lake whose fills came from the REAL venue (paper fills measure mark noise)",
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
            let reg = TrialsRegistry::open(vcfg.registry_path(lake.root()));
            let mut m = BTreeMap::new();
            m.insert("kappa_hat".to_string(), f.kappa);
            m.insert("kappa_t".to_string(), f.t);
            m.insert("n_fills".to_string(), f.n as f64);
            m.insert("intercept_bps".to_string(), f.intercept_bps);
            reg.append("kappa_fit", &sha1_hex_of_file(cfg_path)?, &m,
                       "κ calibration: venue fills vs completed-bar VWAP reference")?;
        }
        None => tracing::warn!(rows = panel.height(), floor = kappa::KAPPA_MIN_FILLS,
            "kappa fit refused: too few usable fills — the config prior stands"),
    }
    Ok(())
}
```

## `hkq-backtest` — the as-of walk-forward mode

```rust
// crates/hkq-backtest/src/main.rs
//! The mirror's binary (blueprint bin: hkq-backtest).
//! Usage: hkq-backtest <strategy.toml> --equity <HKD> --from YYYY-MM-DD
//!        [--to YYYY-MM-DD] [--out <dir>] [--asof]
//!
//! Two modes, one code path. DEFAULT: the M7 current-state counterfactual —
//! today's learned state replayed against history (validates the machine,
//! studies costs; never promotion evidence). --asof (M8): the point-in-time
//! walk-forward — before each replayed day D, the learned state the machine
//! had on morning D (alpha map, regime gate, AH-β; ICIR weights are as-of at
//! the source since M7) is reconstructed from production data strictly before
//! D and materialized into the sandbox `_state`; the replay's state reads are
//! pointed there via the config clone, while panels still read the production
//! lake. Same TradingDay, same exec actor, same virtual clock — only the
//! state's PROVENANCE changes, which is the entire point.
//!
//! Trials: every run appends to the PRODUCTION registry — kind "walkforward"
//! for --asof runs (the promotion-grade species the DSR gate should weigh),
//! kind "backtest" for current-state runs. DSR's N only ever grows.
//!
//! Sandbox hygiene: PnL summaries cover every date ever replayed into the out
//! root. NEVER mix modes in one out root — use a fresh --out per experiment.
use anyhow::Context;
use chrono::NaiveDate;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg, money::Cash,
               session::{DayKind, TradingCalendar}};
use hkq_data::lake::Lake;
use hkq_engine::replay::run_replay;
use hkq_validate::asof::materialize_asof_state;
use hkq_validate::cfg::load_validate;
use hkq_validate::registry::{sha1_hex_of_file, TrialsRegistry};
use hkq_validate::{pnl, stats};
use rust_decimal::Decimal;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;

const USAGE: &str = "usage: hkq-backtest <strategy.toml> --equity <HKD> --from YYYY-MM-DD \
                     [--to YYYY-MM-DD] [--out <dir>] [--asof]";

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("info".parse()?))
        .init();

    let mut args = std::env::args().skip(1);
    let cfg_path = args.next().context(USAGE)?;
    let mut equity: Option<Decimal> = None;
    let mut from: Option<NaiveDate> = None;
    let mut to: Option<NaiveDate> = None;
    let mut out: Option<PathBuf> = None;
    let mut asof = false;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--equity" => {
                let v = args.next().context("--equity needs a value")?;
                equity = Some(Decimal::from_str(&v).context("equity must be a decimal HKD amount")?);
            }
            "--from" => {
                let v = args.next().context("--from needs a date")?;
                from = Some(v.parse().context("--from must be YYYY-MM-DD")?);
            }
            "--to" => {
                let v = args.next().context("--to needs a date")?;
                to = Some(v.parse().context("--to must be YYYY-MM-DD")?);
            }
            "--out" => out = Some(PathBuf::from(args.next().context("--out needs a path")?)),
            "--asof" => asof = true,
            other => anyhow::bail!("unknown argument '{other}'\n{USAGE}"),
        }
    }
    let equity = equity.context("--equity <HKD> is required")?;
    anyhow::ensure!(equity > Decimal::ZERO, "equity must be positive");
    let from = from.context("--from <YYYY-MM-DD> is required (backtests never default to today)")?;
    let to = to.unwrap_or(from);
    anyhow::ensure!(from <= to, "--from must be ≤ --to");

    // The paused-time runtime is the replay tier's virtual-clock engine:
    // sleeps auto-advance whenever the system is idle. Current-thread keeps
    // the event interleaving deterministic.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .start_paused(true)
        .build()
        .context("building the paused-time replay runtime")?;
    runtime.block_on(run(cfg_path, from, to, out, equity, asof))
}

async fn run(
    cfg_path: String, from: NaiveDate, to: NaiveDate, out: Option<PathBuf>,
    equity: Decimal, asof: bool,
) -> anyhow::Result<()> {
    let cfg = StrategyCfg::load(&cfg_path)?;
    let vcfg = load_validate(&cfg_path)?;
    let calendar = FileCalendar::load(&cfg.ops.calendar_path)?;
    let prod_lake = Lake::new(&cfg.ops.lake_root);
    let out_root = out.unwrap_or_else(|| cfg.ops.lake_root.join("_backtest"));
    anyhow::ensure!(
        out_root != cfg.ops.lake_root,
        "--out must differ from ops.lake_root: a replay must never write into the production lake"
    );

    // M8: in as-of mode the replay's STATE reads (alpha map, regime gate,
    // AH-β) come from the sandbox `_state`, re-materialized per date below;
    // panel reads still come from `prod_lake` (the parameter — the load-
    // bearing split in NightlyState::load), and ICIR weights are date-bounded
    // at the source (M7). The file config's root remains the only production
    // root, and it is checked above — the sandbox can never be it.
    let replay_cfg = if asof {
        let mut c = cfg.clone();
        c.ops.lake_root = out_root.clone();
        c
    } else {
        cfg.clone()
    };

    let (mut ok_days, mut failed_days) = (0usize, 0usize);
    let mut d = from;
    while d <= to {
        if calendar.day_kind(d) == DayKind::Closed {
            d = d.succ_opt().context("date overflow")?;
            continue;
        }
        if asof {
            // Point-in-time state for D, from production data STRICTLY before
            // D — always all three files, so nothing stale leaks across days.
            match materialize_asof_state(&prod_lake, &vcfg, d, &out_root) {
                Ok(rep) => tracing::info!(date = %d, alpha_buckets = rep.alpha_buckets,
                    gate_identity = rep.gate_is_identity, ah_names = rep.ah_names,
                    "as-of state materialized"),
                Err(e) => {
                    failed_days += 1;
                    tracing::error!(date = %d, error = %e,
                        "as-of materialization FAILED; day skipped");
                    d = d.succ_opt().context("date overflow")?;
                    continue;
                }
            }
        }
        match run_replay(&prod_lake, &replay_cfg, &out_root, d, Cash(equity), &calendar).await {
            Ok(r) => {
                ok_days += 1;
                tracing::info!(date = %r.date, events = r.events, bars = r.bars,
                    auction_snaps = r.auction_snaps, mainland_prints = r.mainland_prints,
                    scored_names = r.scored_names, fills = r.fills,
                    cash_day = (r.scored_names == 0), asof, "replay day complete");
            }
            Err(e) => {
                failed_days += 1;
                tracing::error!(date = %d, error = %e, "replay day FAILED; continuing range");
            }
        }
        d = d.succ_opt().context("date overflow")?;
    }
    anyhow::ensure!(ok_days > 0, "no day in {from}..={to} replayed successfully");

    // Sandbox-wide realized PnL through the §4 path. NOTE: this covers every
    // date ever replayed into this out root, not only this run — use a fresh
    // --out per experiment, and NEVER mix --asof and current-state runs in one.
    let out_lake = Lake::new(&out_root);
    match pnl::daily_pnl(&out_lake) {
        Ok(series) => {
            let x: Vec<f64> = series.iter().map(|(_, v)| *v).collect();
            let total: f64 = x.iter().sum();
            let sr = stats::sharpe(&x);
            let mode = if asof { "asof_walk_forward" } else { "current_state_counterfactual" };
            let summary = serde_json::json!({
                "mode": mode,
                "days_replayed_this_run": ok_days,
                "days_failed_this_run": failed_days,
                "sandbox_root": out_root.display().to_string(),
                "sandbox_days_with_fills": x.len(),
                "sandbox_total_pnl_hkd": total,
                "sharpe_daily": sr,
                "sharpe_annualized": sr.map(|s| s * 252f64.sqrt()),
                "note": "point hkq-validate `report` at a strategy.toml whose ops.lake_root is the sandbox for bootstrap CIs and the DSR promotion gate; only --asof runs are promotion-grade evidence",
            });
            println!("{}", serde_json::to_string_pretty(&summary)?);

            // §4 honesty: a backtest is a TRIAL. Record it in the PRODUCTION
            // registry so the DSR's N can only grow — never quietly shrink.
            if let Some(sr) = sr {
                let reg = TrialsRegistry::open(vcfg.registry_path(&cfg.ops.lake_root));
                let mut m = BTreeMap::new();
                m.insert("sr".to_string(), sr);
                m.insert("days".to_string(), x.len() as f64);
                m.insert("total_pnl_hkd".to_string(), total);
                m.insert("asof".to_string(), if asof { 1.0 } else { 0.0 });
                let kind = if asof { "walkforward" } else { "backtest" };
                let note = format!(
                    "{} {from}..={to} → {}",
                    if asof { "as-of walk-forward" } else { "current-state replay" },
                    out_root.display()
                );
                match reg.append(kind, &sha1_hex_of_file(&cfg_path)?, &m, &note) {
                    Ok(t) => tracing::info!(seq = t.seq, kind,
                        "trial recorded — DSR's N grows honestly"),
                    Err(e) => tracing::warn!(error = %e, "trials registry append failed"),
                }
            }
        }
        Err(e) => tracing::warn!(error = %e,
            "no realized PnL in the sandbox (all cash days?) — nothing to report"),
    }
    Ok(())
}
```

*Thinking...*

> Looking at the documents the user has provided, they're describing a Rust-based HK intraday trading framework organized across multiple milestones, with M1 covering the core data layer and M2 focusing on the factor engine.
> 
> The user's prompt appears to have two conflicting task statements—the first references an M9 document that isn't actually attached, while the second asks me to evaluate whether the framework is complete and either build the next milestone or conduct a code review. Since only M1 through M8 are provided, I need to assess the current state of the framework and determine what comes next.
> 
> Looking at the blueprint against what's been built, I can see that most of the core crates and binaries are implemented through M8, but there's one critical gap: hkq-recon, the reconciliation binary, has never been built. The milestone docs consistently flag this as blocked on an independent official-close data source—a dependency that's been outstanding since M1. This is the only blueprint binary still missing, which means the framework is technically incomplete until that's resolved. The independent official-close source needs to be something other than Tiger to avoid circularity. EastMoney could work since it supports HK securities through its push2 API, and the codebase already has the pattern for parsing EastMoney data. Yahoo is research-gated and can't be used in production binaries. HKEX has an official daily quotes endpoint but it's undocumented. The cleanest approach fits the existing pattern: extend EastMoney with an `official_closes` capability using a config URL template.
> 
> For Milestone 9, I need to define the scope carefully around the `hkq-recon` binary and how to wire in the EastMoney implementation. The challenge is that M1-M8 crates are frozen, so modifications follow a surgical append pattern rather than direct edits. Adding a new config field to EastMoneyCfg with `#[serde(default)]` would keep backward compatibility with existing configs, but I need to check whether modifying a frozen struct counts as a breaking change under the established precedent.
> 
> The `hkq-recon` binary itself would load the config and calendar, fetch official closes from the independent provider for a given date, run the existing `reconcile_closes` logic, and quarantine any breached partitions while reporting missing entries. There's also the question of whether the recon process should trigger the `HaltReason::ReconBreach` variant that's been defined since M3 but never actually produced by anything — though since recon runs as a separate nightly process, the mechanics of how it would halt the main system need clarification.
> 
> The cleanest approach mirrors the CUSUM pattern: recon writes a latched breach marker to `_state/recon.json` when it detects a breach, and `hkq-live`'s startup gate checks that file before proceeding. This gives `ReconBreach` its producer and keeps the halt logic consistent across different monitoring systems. The question then becomes where to house the recon gate logic — it belongs in `hkq-validate` (which already owns state conventions and monitoring) rather than splitting it between `hkq-data` (which has the recon math) and `hkq-risk` (which owns the kill switch). The `hkq-recon` binary itself would live in its own crate and depend on `hkq-data` for the reconciliation logic, while `hkq-live` reads the latch state through `hkq-validate`. Settling on hkq-validate::recon_gate as the home for the latch — it mirrors the cusum precedent and keeps monitoring and reconstruction artifacts together. Now I'm designing the official-close provider: a new trait in hkq-data that outputs [code, close_official], with an EastMoney implementation that pulls from a configurable URL template, maps HK secids to EastMoney's scheme, and parses the f12 (code) and f2 (close price) fields into a simple versioned parser.
> 
> For the secid mapping, I'm adding two defaulted config fields to EastMoneyCfg — `hk_close_url_template` and `hk_secid_prefix` — which keeps old configs working while enabling the new capability. This is a surgical patch in the established style. I'm considering whether EastMoney is independent enough for production or if we should eventually swap in HKEX's official EOD data behind the same trait, but for now the provider-agnostic design lets us defer that choice.
> 
> On the nightly wiring side, the blueprint keeps `hkq-recon` as a separate binary, so I'll leave hkq-nightly untouched except for a one-line patch to update the stale "recon skipped" log message to point at the separate recon binary. For the reconciliation universe, I need to decide whether to compare against all codes in the partition or just the official universe codes.
> 
> On artifacts and state, I'm moving away from per-date report files toward a minimal state file at `_state/recon.json` that tracks breach status, the breach date if latched, and the last clean date — operators can un-latch by editing the file directly, same pattern as CUSUM. For the live gate, I'll add a call to `recon_gate::startup_gate` in hkq-live's main.rs alongside the existing cusum gate check.
> 
> Now I'm reconsidering whether hkq-recon should be PATH A (build it) or PATH B (defer it). The user's criterion flags missing critical modules, and hkq-recon has been a standing gap since M1. Earlier milestones deferred it as a "data acquisition" problem — no independent source configured — but looking closer, the codebase already has an EastMoney client with quote templates, so the acquisition mechanism exists in-pattern. What was actually missing was the capability trait, the binary itself, and the latch/gate logic — those are code facts, not data facts. The source question (HKEX OMD-C vs. EastMoney) was the blocker, but the M1 precedent shows how to wire an undocumented endpoint behind config with recon as a tripwire.
> 
> So I'm weighing two options: build hkq-recon with official closes (closes the oldest gap, completes the blueprint's binary list, produces HaltReason::ReconBreach, and implements the §5 data integrity mandate that quarantines bad partitions), or pivot to benchmark ingestion plus the Hansen SPA test (completes §4, needs HSI and sector ETF series ingestion). The recon path feels foundational since the entire learning stack consumes lake partitions and a silent bad close corrupts everything downstream.
> 
> Looking at precedence: build what unblocks the most with inputs that already exist and minimal vendor surface. Recon wins—it needs just one quote endpoint for ~500 codes' closes, whereas SPA requires new persistent benchmark datasets. I'm also considering whether to close the Fills venue-tag gap (add a venue column to fills_frame for κ calibration), but that expands scope into schema versioning, and M9 discipline calls for one coherent theme per milestone.
> 
> So Milestone 9 is **The Tripwire**: `hkq-recon` plus independent official-close capability with a recon latch gate. Now I'm designing the code in detail—adding a new workspace member, defining an OfficialCloseProvider trait in the data layer that fetches closing-auction closes from EastMoney (which are live quotes after market close but can't serve historical dates), and validating that the requested date matches today.
> 
> The provider will error if asked for historical closes, since EastMoney only has same-day data. The recon binary defaults to today and runs in the evening cron chain after nightly, so the operator needs to know it'll fail if run after midnight HKT. I'm also adding config fields to the EastMoney client for the batch quote URL template and HK market prefix.
> 
> Adding config fields... Implementing the trait method to fetch official closes from EastMoney, which validates that the date is today (since the live-quote endpoint only serves same-day data), then batches the stock codes into chunks of around 80 to keep URLs manageable, builds the secid format, parses the response, and returns a DataFrame with the official closing prices.
> 
> Now I'm thinking through the reconciliation logic — when official data comes back, some codes might be missing if they're halted, and the recon process needs to distinguish between codes we're missing versus codes the official source is missing. The clean state requires no price breaches and no missing codes from our side, but I'm weighing whether missing data should quarantine the partition or just trigger a loud warning for operator investigation, since the philosophy elsewhere favors degradation over blocking. So missing_official is just a warning since vendors drop suspended names anyway. For breaches—price contradictions—I'm quarantining the data and latching the gate to halt trading until an operator manually resets. missing_ours is a non-zero exit with loud logging but no latch, since it's an ingestion gap that degrades panels rather than corrupts prices.
> 
> Now I'm building the recon latch module in Rust. Defining a state struct that tracks whether a breach occurred, when it happened, and the most recent clean reconciliation date for the audit trail.
> 
> Defining the core functions: loading and saving state atomically, latching a breach into the state file, recording clean reconciliations (which never un-latch), and a startup gate that halts trading if a breach is latched. Mirroring the cusum pattern with tests for persistence, non-un-latching, halt behavior, and loud errors on corrupt files.
> 
> Then wiring the new recon binary crate with its dependencies and main entry point that runs at 18:30 nightly, comparing official closes against stored daily bars and quarantining plus latching any price deltas exceeding one tick. A quarantined partition creates a silent gap in the data that panels would bridge across, which is why the latch halts trading until the operator re-ingests and clears it. I'm adding a recon gate check in the live engine startup that reads the latch state and either logs a halt condition or clears the gate, with error handling for corrupt state files.
> 
> Now I'm deciding how to handle corrupt latch files — unlike cusum which warns and continues on missing data, a corrupt recon latch could hide a breach, so the safer approach is to halt on any unreadable state and force operator attention.
> 
> I'm implementing this by having startup_gate itself call the kill switch with ReconBreach when load fails, then return the error to hkq-live which logs it (already halted). For hkq-nightly, I'm replacing the recon warning with a single-line pointer to the independent hkq-recon tool. I'm also adding the EastMoney close URL template and secid prefix to the config, then building out unit tests for the parser with a helper function to format secids from prefix and code.
> 
> For the async official_closes tests, I need to handle the date guard and unsupported paths without making actual HTTP calls — the date check happens before any network request, and I can construct an EastMoneyCfg directly in the test using toml parsing to fill in all the required fields with serde defaults. Since hkq-data already has tokio as a workspace dependency, I can use #[tokio::test] for async tests. I'll add a test module to eastmoney.rs covering the unsupported paths and parser logic, then move on to testing the recon_gate roundtrip and latch persistence.
> 
> For the binary itself, I'll keep main.rs thin and push testable logic into libraries — consistent with how other binaries in the project are structured. The real value is in testing reconcile_closes, which wasn't unit-tested in the earlier milestone. I'll add an integration-style test to hkq-data/src/recon.rs that writes a small daily_bars partition to a temp lake with matching, breaching, and missing records, then verifies the report fields account for tick-aware price differences correctly.
> 
> On the schema side, I need to ensure the official DataFrame has the right columns — code as u32 and close_official as f64. The provider returns those plus a date column, which is harmless. I'll add a constant for "close_official" to model::cols and use it in both the eastmoney provider and recon to keep things consistent, even though recon.rs is already using the literal. For the official_closes logic, I'm thinking through how to handle the response items that carry the code as a string (f12 field) — I'll parse those via StockCode and collect the code-price pairs, letting recon handle any extras defensively.
> 
> Building a HashSet of requested codes to filter the parsed rows prevents false alarms from codes outside our universe. If parsing yields zero rows, I'll return a SchemaDrift error like the southbound provider does. I'm also including the DATE column for provenance consistency with other providers. For chunking the HK secids, I'm setting a chunk size around 80 to respect URL length limits while staying well within what the push2 endpoint can handle, then iterating through chunks, formatting each secid with the prefix, fetching the data, parsing it by schema version, and filtering to only the requested codes.
> 
> Handling suspended stocks (which return "-" for price) naturally skips them since the parser can't convert that to a float. I'm keeping error propagation tight — any transport error fails the whole call since recon is a tripwire and partial data weakens the check. For the main recon binary, I'm importing the necessary modules from hkq-core and hkq-data, and since hkq-validate depends on hkq-data, having hkq-recon depend on both avoids any circular dependency. I'll follow the established pattern of duplicating the universe loader helper across binaries like hkq-nightly and hkq-live already do.
> 
> The flow checks if the date is a trading day, ensures the daily bars partition exists, loads the EastMoney config and A/H mapping, fetches official closes, then reconciles them against the lake data. If breaches are found, the partition gets quarantined and a detailed error is raised; otherwise the clean status is recorded.
> 
> I'm verifying the lake.root() method exists and thinking through the recon_gate functions — latch_breach loads the state, sets breached=true on first occurrence without overwriting, while record_clean updates the last_clean_date to the maximum of existing or current date. For the worst breach detail, I'm using max_by_key on the breaches iterator with abs_diff as the key, since Decimal is Copy and Ord.
> 
> Now I'm building the recon_gate module with the ReconState struct and helper functions to load and save state from a JSON file in the lake's _state directory.
> 
> Writing module functions... Checking whether hkq-validate already has the Lake dependency it needs, and confirming the error variants are already available so no changes to error.rs are required. I'll skip adding a recon-gate-status job to main.rs to keep it unchanged. Now I need to update the workspace Cargo.toml to include the new hkq-recon crate as a member. For the Eastmoney config changes, I'll need to insert the new fields into the EastMoneyCfg struct and update the default implementation — I'll present this as a precise block insertion after the existing fields rather than reprinting the whole file. I'll use fully qualified `std::collections::HashSet` to avoid needing a new import, keeping the patch append-only. I'm adding a `CLOSE_OFFICIAL` constant to the model's cols module with documentation, then referencing it in eastmoney.rs while recon.rs keeps its literal string. Now I'm writing the test module for recon.rs that validates tick-aware breach detection and missing data sets. I'm verifying the tick logic for the breach detection: at 70.10 the tick size is 0.050, so a difference of 0.10 exceeds that threshold and correctly flags as a breach, while 60.05 differs by exactly one tick which doesn't breach. The reconciliation report correctly counts three checked entries, identifies the one breach, marks 1299 as missing from our data, and returns false for clean since there's a breach and missing entries. Now I'm looking at a test for the Eastmoney parser...
> 
> Writing parser test... Writing test assertions...
> 
> I'm verifying that the guard conditions work correctly—the template-missing case returns Unsupported for today's date, and the historical date case also returns Unsupported even with the template present. I should also add an empty codes guard that returns a Config error before checking other conditions, and I need to make sure the date parameter stays explicit in the signature for clarity and future extensibility. Now I'm ready to apply the same guard pattern to the hkq-live patch right after the CUSUM gate match. Writing Cargo.toml... Setting up the main async entry point...
> 
> I'm loading the EastMoney configuration and A/H stock mapping, then fetching official closing prices for the universe codes. After reconciling the closes against our lake data, I'm building a summary report that captures the reconciliation results—how many records checked, any price breaches found, and which stocks are missing from either source. If there are breaches, I'm quarantining the daily bars partition for that date. Checking the borrow semantics — worst is an Option reference into report, and I'm using it after the quarantine call without any mutations, so that's fine. The abs_diff field is Copy, so keying by value works. Quarantine happens before the latch write, and both operations are idempotent enough that the order is safe. The DayKind check correctly skips only Closed days, leaving HalfDay valid for recon. The DataFrame height() method is available without extra imports, and the JSON serialization handles the Decimal-to-string conversion properly. Now I need to add the hkq-recon member to the workspace TOML and set up the strategy config block for the eastmoney source with the HK close URL template.
> 
> For the milestone narrative, I'm working from ten attached documents (the proposal, report, and milestones 1–8), and there's no 11th document despite the first prompt block mentioning one — so M8 is the current frontier. The PATH A/B instruction is the operative one. I need to frame this in the style of the previous milestone docs: dense opening on why this is next, scope and deferred items, engineering decisions in prose, file tree, code, and honest gaps. I should note upfront that all documents are attached and I don't have a web fetch tool, only search (which isn't needed here). PATH A reveals that the blueprint's bin list includes hkq-recon, which doesn't exist yet.
> 
> The recon binary is the only remaining blueprint component — every other crate and binary exists. The lake's integrity check (the quarantine mandate from §5) has been stubbed since M1 with a nightly warning, and the typed ReconBreach reason in the kill switch (from M3) has no producer. The other named remainders are either blocked by external constraints (SPA benchmarks need new datasets, §3.7 needs a CPCV consumer, venue-tag schema v2 is a data-layer decision) or are governance-by-design. Recon's data acquisition is just one config-templated quote endpoint — the same pattern as the M1 EastMoney setup. The key engineering decision is independence over officialdom: the tripwire needs a source outside Tiger, so EastMoney sits behind a config template with a schema-versioned parser, while HKEX OMD-C EOD remains the canonical production substitute behind the same new trait — applying the M1 precedent to closes. A live-quote endpoint can only certify the session just ended; the provider returns a typed Unsupported for historical dates rather than fabricating history, and an EOD-file provider would handle the archive.
> 
> Breach semantics mirror CUSUM — a persistent JSON latch with first-breach-wins and operator-only reset — because a quarantined partition leaves a gap that rolling spans would silently bridge; the halt is what makes quarantine safe. Unlike CUSUM's cold-start-friendly skip, recon treats a corrupt state file as breached; a clean state must be provable (missing file on cold start is clean by default, only corruption halts). Price contradictions trigger quarantine and latch, while ingest holes page without quarantine since there's nothing to quarantine — data is absent, not wrong — and never latch; missing_official is just informational. The implementation is additive: hkq-live gains one fully-qualified gate call, the nightly stale gap warning becomes a pointer to the new binary, and recon.rs stays byte-identical. The chunked fetch bound is transport bookkeeping (following the M6 VENUE_POLL precedent), not a strategy constant. The honest gap is that the official source is a same-day quote endpoint, so historical backfill recon is impossible until an EOD-file provider like HKEX OMD-C daily quotes lands behind OfficialCloseProv.
> 
> Recon covers DailyBars closes only — bars_1m, flows, and auction partitions have manifests as tripwires but no cross-vendor check; extending recon to flows versus the HKEX Connect page is a named follow-up. Adjusted close is reconciled implicitly only when adjust-mode is raw; once kline_adjust is verified, the official source has no adjusted series, so corporate-action recon needs an actions dataset. The recon latch reset is deliberately manual with no auto-unlatch job by design. I'm defining a chunked fetch constant as a module-level value with a comment explaining it's transport bookkeeping, not a strategy constant, and checking that my DataError variants match the expected types. Now I'm wondering whether the trait implementation needs to normalize duplicates.
> 
> I'll add a defensive deduplication step using a HashSet to track seen codes and skip repeats — it's cheap insurance. The dataframe construction checks for empty results and returns a SchemaDrift error if nothing was parsed, then builds the output with CODE, CLOSE_OFFICIAL, and DATE columns. I'm verifying the EastMoneyClient::new signature takes the config and AhMap, and for the guard test I'm only exercising the Unsupported paths without HTTP calls. For the recon gate test, I'm setting up a temporary directory path with a timestamp.
> 
> Writing test cases... Updating lib.rs to export the recon_gate module and ReconState type, then verifying the Cargo.toml dependencies are already in place. For hkq-nightly, I'm replacing the old warning about the missing close source with an info message directing users to run the separate hkq-recon binary after the nightly job completes, since reconciliation now happens independently and can halt the system if breaches occur.
> 
> Now I'm thinking through the OfficialCloseProvider trait signature — it takes a slice of stock codes and outputs records with code, official close price, and date, with clear documentation about the independence requirement. Mentally walking through the eastmoney implementation: the client struct holds an Arc to Inner which contains the config and HTTP client, async_trait is imported, Hong Kong timezone is available, date comparisons work with NaiveDate, and the code formatting with the prefix works correctly since StockCode implements Display. The config template is safely borrowed via let-else on the Option, and the recon module will call official_closes with the codes slice.
> 
> For hkq-recon's argument parsing, I'm mirroring hkq-nightly's approach — config path first, optional date second, no force flag — and copying over the tracing subscriber initialization. The summary println before bail is useful for cron visibility. I'm skipping an asof-style inspection tool for the validate binary since jq on the JSON output is sufficient.
> 
> For the answer format, I have ten attached documents covering the report through M8, with M9 being the tripwire milestone that completes the missing pieces. I'll lay out PATH A with the specific gaps, define M9's scope and deferred items, explain the engineering decisions, show the file tree and workspace config, then provide the surgical patches for the model columns, provider trait implementation, and remaining code changes.
> 
> Adding the EastMoney config fields... Implementing the trait method... Adding the constant definition for close_official in the model, then wrapping up with honest gaps and a hand-off to the next milestone — noting that adjustment handling is already consistent (raw close stored as-is), corporate action verification remains future work, and sandbox lakes naturally skip reconciliation. I'm committing to PATH A since it's nearly complete, with PATH B (hkq-recon) deferred.
> 
> For the opening, I'll acknowledge that all ten documents are in hand so no fetching is needed, and flag that the eleventh document isn't attached — meaning M8 is the verified frontier. Then I'll tighten the "why next" section: the workspace has three of four binaries, and hkq-recon is the critical gap — it's been named since M1, the spec mandates nightly reconciliation with quarantine on breach, the risk system has carried the typed halt reason since M3 with no producer, and every learning loop trains on unverified lake prices. M7/M8 filed it as "data acquisitions, not code," but that's only half-true — the vendor endpoint is already wired, and the rest (trait, parser, binary, quarantine logic, safety latch) is pure code work.
> 
> The quarantine safety argument is key: a silently-bridged hole in partitions (shifts and rolling windows skip the missing date) means quarantine without a halt is worse than no quarantine at all — the latch and startup gate are what make the §5 mandate actually implementable. I need to be precise about the claim: manifests exist as row/schema tripwires, but no price cross-check happens. I'll note that verifying Bars1m is out of scope. Now I'll write the full section, keeping the code APIs consistent with what's established, double-checking the error types and dependencies, and anchoring the patches exactly — hkq-live goes after the CUSUM startup-gate match block, and the nightly swap replaces the two-line comment block. Test count is five total across the three modules.
> 
> Checking the glob import pattern in the test modules — `use super::*;` brings in both the items defined in the parent module and everything it imported, so StockCode, DataError, and other types are available. For the async test, chrono::Utc comes through the parent's imports as well.
> 
> The test constructs an AhMap (BTreeMap) from the stock code and price pair, and pred_opt works on NaiveDate as expected. The main binary parses the config path and optional date argument cleanly, with the partition existence check happening before any official close fetch, and the summary printed after any breach bailout.
> 
> Now I'm laying out the final response: preamble covering attachments and the missing 11th document, then declaring PATH A with the concrete list of gaps, followed by Milestone 9's rationale and scope. The engineering decisions section flows into the full tree structure, workspace configuration, and all the patches needed across model, provider, eastmoney (two blocks), recon test, nightly swap, and live insert. Then hkq-validate gets its lib.rs and recon_gate.rs in full, hkq-recon crate complete, config addition, and finally the honest gaps leading into Milestone 10.

All ten documents are attached in full, so I worked directly from them — no fetching needed (I have a search tool only, and it isn't required here). One manifest note before anything else: the task list references an eleventh document (`11_milestone_code_9.md`), but it is not among the attachments. The verified frontier is therefore M8 — the judgment — and this assessment builds on exactly that state.

# Assessment — PATH A

The framework is *almost* complete, but not complete, and the gap is not cosmetic. The blueprint's workspace topology ends with four binaries: `hkq-nightly / hkq-live / hkq-backtest / hkq-recon`. Three exist. The fourth has been a named gap in every single hand-off since M1 — the nightly binary still logs `"recon skipped: independent official-close source not configured (M1 gap)"` every night — and three structural facts hang off it: report §5 mandates nightly reconciliation with "|Δ| > 1 tick ⇒ quarantine the partition and page the operator," and that mandate is unimplemented at the binary level; `hkq-risk` has carried a typed `HaltReason::ReconBreach` since M3 with **no producer** (the blueprint's kill-switch topology promised one); and every learning loop the system now runs — ICIR weights, the AlphaMap, κ calibration, the DSR itself — trains on lake partitions whose *prices* no independent source has ever confirmed (the `_manifest.json` tripwire catches schema drift, not wrong closes).

M7/M8 filed recon under "data acquisitions, not code." Examined against the M1 pattern, that judgment is only half-true: the acquisition is one config-templated quote endpoint on a vendor already wired with the rate-limited substrate and schema-versioned-parser discipline (EastMoney), while everything else — the capability trait, the parser, the binary, the quarantine wiring, and the latch that makes quarantine *safe* — is code. So: **PATH A.**

# Milestone 9 — The Tripwire (`hkq-recon` + independent official closes + the recon latch)

**Why this must be next.** Beyond completing the blueprint's binary list, there is a soundness argument specific to this codebase: `Lake::quarantine` has existed and been tested since M1, but quarantining a `DailyBars` partition leaves a *hole* that the factor panels silently bridge — `shift(1).over(code)` and every rolling window simply span the missing date, so `r_on` quietly becomes a two-day return and every derived moment inherits the lie. Quarantine without a trading halt is therefore worse than no quarantine at all. The missing piece is the same latch discipline CUSUM established in M5: breach ⇒ persistent latched state ⇒ `hkq-live` startup gate halts before any order intent ⇒ only an operator edit resumes trading. That closes the §5 mandate, gives `ReconBreach` its producer, and retires the oldest standing gap in the project.

**In scope:** a new `OfficialCloseProvider` capability trait in `hkq-data`; an EastMoney implementation behind a config URL template with a schema-versioned parser (independent of Tiger — a tripwire needs a *different* vendor, and HKEX OMD-C EOD remains the canonical production substitute behind the same trait, per §5); the `recon_gate` module in `hkq-validate` (persistent latch, first-breach-wins, operator-only reset — the CUSUM pattern verbatim); the `hkq-recon` binary (fetch → `reconcile_closes` → quarantine + latch + non-zero exit on breach); the `hkq-live` startup gate as the kill switch's **third** producer; a unit test for M1's never-tested `reconcile_closes`; and retirement of the nightly gap log. **Deferred:** historical-date recon (a live-quote endpoint certifies only the session just ended; an EOD-file provider slots behind the same trait), recon of `Bars1m`/`Flows` against second sources, the SPA benchmark family, venue-tagged Fills, and §3.7 — all unchanged owners.

Engineering decisions beyond the blueprint sketch, briefly. Independence beats officialdom for a tripwire: EastMoney ≠ Tiger is the property that matters (different vendor, transport, failure modes), so it ships now behind config, with the report's canonical HKEX source as a named substitution point — the M1 precedent applied to closes. Same-day honesty: a quote endpoint after the CAS *is* the official close for today but cannot serve history, so the provider refuses historical dates with a typed `Unsupported` instead of fabricating them. Breach versus hole are different failures: price contradictions quarantine + latch; ingest holes (`missing_ours`) page via non-zero exit but neither quarantine (nothing is *wrong*, something is *absent*) nor latch; `missing_official` is informational (vendors drop suspended names). Unlike CUSUM's cold-start-friendly skip, an *unreadable* recon latch halts — a corrupt file could be hiding a breach, and a clean state must be provable; a *missing* file is the documented clean cold start. `cols::CLOSE_OFFICIAL` is added for producers while `recon.rs`'s already-documented literal stays byte-identical. The fetch chunk bound is transport bookkeeping, not a strategy constant (the M6 `VENUE_POLL` precedent). And no engine surgery anywhere: `hkq-live` gains one fully-qualified gate call, `hkq-nightly` swaps one stale log line, everything else is additive.

```text
hkq/
├── Cargo.toml                        (updated: member)
└── crates/
    ├── hkq-data/src/{model,provider,eastmoney,recon}.rs   (surgical appends)
    ├── hkq-validate/src/{lib,recon_gate}.rs               (lib updated; recon_gate NEW)
    ├── hkq-nightly/src/main.rs                            (one-block swap)
    ├── hkq-live/src/main.rs                               (one insert block)
    └── hkq-recon/
        ├── Cargo.toml                                     (NEW)
        └── src/main.rs                                    (NEW — the blueprint's fourth binary)
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
  "crates/hkq-live", "crates/hkq-backtest", "crates/hkq-recon",
]

[workspace.package]
edition = "2021"
rust-version = "1.83"

[workspace.dependencies]
tokio        = { version = "1.38", features = ["full", "test-util"] }
tokio-stream = "0.1"
reqwest      = { version = "0.12", default-features = false, features = ["json", "gzip", "rustls-tls"] }
serde        = { version = "1", features = ["derive"] }
serde_json   = "1"
# Feature set unchanged since M2 — M9 adds no new expression surface.
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

In the M5/M6 style: anchored append/replace blocks; everything else in M1–M8 stays byte-identical.

```rust
// (append inside crates/hkq-data/src/model.rs `pub mod cols`)
    /// M9: official closing-auction print from the INDEPENDENT recon source.
    /// Matches the schema documented on `recon::reconcile_closes` (whose literal
    /// stays byte-identical; this constant is for producers).
    pub const CLOSE_OFFICIAL: &str = "close_official";
```

```rust
// (append inside crates/hkq-data/src/provider.rs)
#[async_trait]
pub trait OfficialCloseProvider: Send + Sync {
    /// Official closes for date `d`, from a source INDEPENDENT of the one that
    /// wrote DailyBars (reconciling a vendor against itself is circular — the
    /// M1 gap this trait closes). Output schema: [code: u32,
    /// close_official: f64, date: str]. Providers that can only certify the
    /// just-ended session (live-quote endpoints) MUST refuse other dates with
    /// `DataError::Unsupported` rather than fabricate history.
    async fn official_closes(&self, codes: &[StockCode], d: NaiveDate)
        -> Result<DataFrame, DataError>;
}
```

```rust
// (insert inside crates/hkq-data/src/eastmoney.rs `pub struct EastMoneyCfg`,
//  directly after `a50_window_enforced`)
    /// M9: batch HK close-quote URL for the recon tripwire; `{secids}`
    /// substituted. Must request float formatting (fltt=2) and fields f12
    /// (code) + f2 (last price — equal to the official closing-auction print
    /// once the session, incl. CAS, has ended). None ⇒ this client provides no
    /// official closes and hkq-recon errors loudly.
    #[serde(default)]
    pub hk_close_url_template: Option<String>,
    /// EastMoney market prefix for HKEX secids ("116" ⇒ "116.00700"). VERIFY.
    #[serde(default = "default_hk_secid_prefix")]
    pub hk_secid_prefix: String,
```

```rust
// (append to crates/hkq-data/src/eastmoney.rs)
fn default_hk_secid_prefix() -> String { "116".into() }

/// URL-length bound for the close ulist (push2 handles ~100 secids comfortably).
/// Transport bookkeeping, not a strategy constant (the M6 VENUE_POLL precedent).
const CLOSE_CHUNK: usize = 80;

/// push2 ulist close parser, schema v1: { data: { diff: [ { f12, f2 } ] } }.
/// Requires fltt=2 so prices arrive as floats; suspended names serve "-" and
/// drop out here (recon reports them as missing_official — informational).
fn parse_ulist_close_v1(v: &Value) -> Result<Vec<(String, f64)>, DataError> {
    let diff = v.pointer("/data/diff").and_then(Value::as_array)
        .ok_or(DataError::SchemaDrift("eastmoney close ulist: /data/diff missing"))?;
    let mut out = Vec::with_capacity(diff.len());
    for item in diff {
        let (Some(code), Some(px)) = (
            item.get("f12").and_then(Value::as_str).map(str::to_string),
            item.get("f2").and_then(Value::as_f64),
        ) else { continue };
        out.push((code, px));
    }
    Ok(out)
}

#[async_trait]
impl OfficialCloseProvider for EastMoneyClient {
    async fn official_closes(&self, codes: &[StockCode], d: NaiveDate)
        -> Result<DataFrame, DataError>
    {
        let inner = &self.inner;
        let Some(tpl) = inner.cfg.hk_close_url_template.as_deref() else {
            return Err(DataError::Unsupported(
                "eastmoney: hk_close_url_template not configured (no recon source)"));
        };
        if codes.is_empty() {
            return Err(DataError::Config("official_closes: empty code list".into()));
        }
        // A live-quote endpoint certifies ONLY the session that just ended.
        // Refusing history is honesty, not a limitation to paper over —
        // historical recon needs an EOD-file provider behind this same trait
        // (HKEX OMD-C daily quotes, report §5).
        let today = Utc::now().with_timezone(&Hong_Kong).date_naive();
        if d != today {
            return Err(DataError::Unsupported(
                "eastmoney official closes are same-day only (run hkq-recon T+0 evening)"));
        }
        let requested: std::collections::HashSet<u32> = codes.iter().map(|c| c.0).collect();
        let mut seen: std::collections::HashSet<u32> = std::collections::HashSet::new();
        let (mut out_codes, mut out_px) =
            (Vec::with_capacity(codes.len()), Vec::with_capacity(codes.len()));
        for chunk in codes.chunks(CLOSE_CHUNK) {
            let secids: Vec<String> = chunk.iter()
                .map(|c| format!("{}.{c}", inner.cfg.hk_secid_prefix))
                .collect();
            let url = tpl.replace("{secids}", &secids.join(","));
            let v: Value = inner.http.get_json(&url, &[]).await?;
            let rows = match inner.cfg.schema_version {
                1 => parse_ulist_close_v1(&v)?,
                _ => return Err(DataError::SchemaDrift(
                    "eastmoney close ulist: unknown schema_version")),
            };
            for (em_code, px) in rows {
                let Some(sc) = StockCode::parse(&em_code) else { continue };
                if px.is_finite() && px > 0.0 && requested.contains(&sc.0) && seen.insert(sc.0) {
                    out_codes.push(sc.0);
                    out_px.push(px);
                }
            }
        }
        if out_codes.is_empty() {
            return Err(DataError::SchemaDrift("eastmoney close ulist: zero rows parsed"));
        }
        let n = out_codes.len();
        Ok(df!(
            cols::CODE => out_codes,
            cols::CLOSE_OFFICIAL => out_px,
            cols::DATE => vec![d.to_string(); n],
        )?)
    }
}

#[cfg(test)]
mod close_tests {
    use super::*;

    #[test]
    fn close_parser_v1_shapes_and_drift() {
        let v = serde_json::json!({ "data": { "diff": [
            { "f12": "00700", "f2": 321.4 },
            { "f12": "00005", "f2": "-" },   // suspended ⇒ skipped
            { "f2": 1.0 },                    // no code ⇒ skipped
        ]}});
        let rows = parse_ulist_close_v1(&v).unwrap();
        assert_eq!(rows, vec![("00700".to_string(), 321.4)]);
        assert!(parse_ulist_close_v1(&serde_json::json!({})).is_err()); // SchemaDrift
    }

    #[tokio::test]
    async fn official_closes_guards_fire_before_any_http() {
        let cfg: EastMoneyCfg = toml::from_str(r#"
            quote_url_template = "http://invalid.test/{secids}"
            a50_url_template = "http://invalid.test/{secid}"
            a50_secid = "1.000001"
            southbound_url_template = "http://invalid.test/{date}"
            southbound_unit = "CNY"
        "#).unwrap();
        let map: AhMap = [(StockCode(700), "1.600941".to_string())].into_iter().collect();
        let today = Utc::now().with_timezone(&Hong_Kong).date_naive();

        // No template ⇒ Unsupported (capability absent), zero HTTP.
        let em = EastMoneyClient::new(cfg.clone(), map.clone());
        assert!(matches!(
            em.official_closes(&[StockCode(700)], today).await.unwrap_err(),
            DataError::Unsupported(_)
        ));

        // Template set but historical date ⇒ Unsupported (same-day only), zero HTTP.
        let mut cfg2 = cfg;
        cfg2.hk_close_url_template = Some("http://invalid.test/{secids}".into());
        let em = EastMoneyClient::new(cfg2, map);
        let yesterday = today.pred_opt().unwrap();
        assert!(matches!(
            em.official_closes(&[StockCode(700)], yesterday).await.unwrap_err(),
            DataError::Unsupported(_)
        ));
    }
}
```

M1 shipped `reconcile_closes` described as tested, but no test was printed; its consumer milestone adds the missing one:

```rust
// (append to crates/hkq-data/src/recon.rs)
#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    #[test]
    fn tick_aware_breaches_and_missing_sets() {
        let root = std::env::temp_dir().join(format!(
            "hkq_recon_test_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let lake = Lake::new(&root);
        let date = chrono::NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        let mut ours = df!(
            "code" => vec![700u32, 5u32, 941u32],
            "close" => vec![321.40f64, 60.00, 70.00],
            "date" => vec![date.to_string(); 3],
        ).unwrap();
        lake.write_partition(Dataset::DailyBars, date, &mut ours, "test", 1).unwrap();

        // 700 exact; 5 off by EXACTLY one tick (0.05 band at 60) ⇒ not a breach
        // (§5: |Δ| > 1 tick); 941 off by two ticks ⇒ breach; 1299 ⇒ missing_ours.
        let official = df!(
            "code" => vec![700u32, 5u32, 941u32, 1299u32],
            "close_official" => vec![321.40f64, 60.05, 70.10, 50.0],
        ).unwrap();
        let rep = reconcile_closes(&lake, &official, date).unwrap();
        assert_eq!(rep.checked, 3);
        assert_eq!(rep.breaches.len(), 1);
        assert_eq!(rep.breaches[0].code, 941);
        assert_eq!(rep.missing_ours, vec![1299]);
        assert!(rep.missing_official.is_empty());
        assert!(!rep.clean());
        std::fs::remove_dir_all(root).ok();
    }
}
```

In `crates/hkq-nightly/src/main.rs`, the M1 gap line finally retires — replace

```rust
    // Close reconciliation still needs an INDEPENDENT official-close source
    // (HKEX daily quotes / OMD-C EOD, report §5) — unchanged M1 gap, logged loudly.
    tracing::warn!("recon skipped: independent official-close source not configured (M1 gap)");
```

with

```rust
    // M9: close reconciliation is its own binary — run `hkq-recon` after this
    // job (T 18:30). A breach quarantines the partition and LATCHES; hkq-live
    // halts at next startup until the operator re-ingests and clears the latch.
    tracing::info!("nightly done; run hkq-recon next (close tripwire, M9)");
```

And in `crates/hkq-live/src/main.rs`, insert directly after the CUSUM startup-gate `match` block (before the `// Tiger client first (M6)` comment) — fully qualified, so no import lines change:

```rust
    // …and producer 3 is the nightly recon latch (M9): a close-reconciliation
    // breach recorded by hkq-recon (quarantined partition) halts the day before
    // any order intent — HaltReason::ReconBreach, typed since M3, now produced.
    match hkq_validate::recon_gate::startup_gate(&lake, &ks) {
        Ok(s) if s.breached => tracing::error!(date = ?s.breach_date, detail = ?s.detail,
            "RECON breach LATCHED — engine will observe HALT and stand down"),
        Ok(s) => tracing::info!(last_clean = ?s.last_clean_date, "recon gate clear"),
        Err(e) => tracing::error!(error = %e,
            "recon latch unreadable — HALTED (a clean state must be provable)"),
    }
```

## `hkq-validate` — the recon latch

```rust
// crates/hkq-validate/src/lib.rs
#![forbid(unsafe_code)]
//! The §4 protocol as a crate, not a notebook: purged splits, NW t-stats, DSR
//! against an honest trials registry, the CUSUM kill producer, the quarterly
//! estimation jobs, the promotion protocol (as-of reconstruction + κ) — and,
//! as of M9, the recon latch: the kill switch's THIRD producer, fed by the
//! hkq-recon tripwire so a quarantined partition halts trading at startup.
//!
//! Design invariants:
//! - Every statistic is a pure function of frames/slices; the ONLY I/O is the
//!   lake (read), the `_state` directory (fit artifacts, CUSUM + recon state),
//!   and the hash-chained trials registry (append).
//! - Monitoring and reconstruction consume the SAME persisted artifacts the
//!   learning loops train on: no recomputed morning factors, no fabricated
//!   ICs, no imputed fills — the M3/M4 honesty rule, everywhere.
//! - Degradation is typed: missing history ⇒ `Insufficient` (callers continue,
//!   loudly); schema drift ⇒ polars errors (nobody continues).
//! - Breaches LATCH (CUSUM and recon alike). Un-halting is an operator edit of
//!   the state file, never code. That is what "pre-registered kill" means.
//! - Scalar promotions (θ, v*, κ) are OPERATOR config edits. Jobs report and
//!   registry-log; they never mutate strategy.toml or production `_state`.

pub mod asof;
pub mod cfg;
pub mod cusum;
pub mod dsr;
pub mod error;
pub mod fits;
pub mod ic;
pub mod kappa;
pub mod pnl;
pub mod recon_gate;
pub mod registry;
pub mod splits;
pub mod stats;

pub use asof::{materialize_asof_state, AsofReport};
pub use cfg::{load_validate, ValidateCfg};
pub use cusum::{startup_gate, CusumOutcome, CusumParams, CusumState};
pub use dsr::{deflated_sharpe, expected_max_sharpe};
pub use error::ValidateError;
pub use kappa::{fit_kappa, kappa_panel, KappaFit};
pub use recon_gate::ReconState;
pub use registry::TrialsRegistry;
pub use splits::{purged_walk_forward, Split};
```

```rust
// crates/hkq-validate/src/recon_gate.rs
//! The recon latch (M9): persistent breach state written by the hkq-recon
//! binary and observed by hkq-live's startup gate — the kill switch's THIRD
//! producer (operator console, CUSUM, and now recon), giving
//! `HaltReason::ReconBreach` the producer it has lacked since M3.
//!
//! Why a latch and not just a log: quarantining a DailyBars partition leaves a
//! HOLE the factor panels silently bridge — per-code `shift(1)` and rolling
//! windows simply span the missing date, so r_on becomes a two-day return and
//! every derived moment inherits the lie. Trading must not resume until the
//! operator re-ingests (`hkq-nightly --force <date>`) and clears
//! `_state/recon.json` by hand. Un-latching is an operator edit, never code —
//! the CUSUM precedent, verbatim.
//!
//! One deliberate asymmetry vs the CUSUM gate: a MISSING state file is the
//! documented clean cold start, but an UNREADABLE one halts — a corrupt latch
//! could be hiding a breach, and a clean state must be provable.
use crate::error::ValidateError;
use hkq_data::lake::Lake;
use hkq_risk::{HaltReason, KillSwitch};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReconState {
    pub breached: bool,
    pub breach_date: Option<String>,
    pub detail: Option<String>,
    /// Most recent date that reconciled clean — audit trail only; a clean run
    /// NEVER un-latches an earlier breach.
    pub last_clean_date: Option<String>,
}

pub fn state_path(lake_root: &Path) -> PathBuf {
    lake_root.join("_state").join("recon.json")
}

/// Missing file ⇒ fresh state (cold start: nothing has ever reconciled).
/// CORRUPT file ⇒ hard error — silently resetting could erase a latched breach.
pub fn load_state(path: &Path) -> Result<ReconState, ValidateError> {
    match std::fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| {
            ValidateError::State(format!("corrupt recon state {}: {e}", path.display()))
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(ReconState::default()),
        Err(e) => Err(e.into()),
    }
}

pub fn save_state(path: &Path, st: &ReconState) -> Result<(), ValidateError> {
    let dir = path.parent().ok_or(ValidateError::Contract("recon state path has no parent"))?;
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(".recon.tmp");
    std::fs::write(&tmp, serde_json::to_vec_pretty(st)?)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

/// Latch a breach. First reason wins; an existing latch is never overwritten
/// or downgraded (the KillSwitch semantics, mirrored on disk).
pub fn latch_breach(
    lake_root: &Path, date: chrono::NaiveDate, detail: &str,
) -> Result<ReconState, ValidateError> {
    let path = state_path(lake_root);
    let mut st = load_state(&path)?;
    if !st.breached {
        st.breached = true;
        st.breach_date = Some(date.to_string());
        st.detail = Some(detail.to_string());
        tracing::error!(%date, detail, "RECON BREACH — latched");
    } else {
        tracing::error!(%date, prior = ?st.breach_date,
            "recon breach while already latched (first reason kept)");
    }
    save_state(&path, &st)?;
    Ok(st)
}

/// Record a clean reconciliation for `date`. NEVER un-latches; never moves
/// `last_clean_date` backwards (ISO dates compare lexicographically).
pub fn record_clean(
    lake_root: &Path, date: chrono::NaiveDate,
) -> Result<ReconState, ValidateError> {
    let path = state_path(lake_root);
    let mut st = load_state(&path)?;
    let d = date.to_string();
    if st.last_clean_date.as_deref().map_or(true, |prev| d.as_str() > prev) {
        st.last_clean_date = Some(d);
    }
    save_state(&path, &st)?;
    Ok(st)
}

/// hkq-live's startup gate. A latched breach halts before any order intent; an
/// unreadable latch ALSO halts (conservative: clean must be provable), and the
/// error propagates so the operator sees why.
pub fn startup_gate(lake: &Lake, ks: &KillSwitch) -> Result<ReconState, ValidateError> {
    match load_state(&state_path(lake.root())) {
        Ok(st) => {
            if st.breached {
                ks.halt(HaltReason::ReconBreach);
            }
            Ok(st)
        }
        Err(e) => {
            ks.halt(HaltReason::ReconBreach);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> PathBuf {
        std::env::temp_dir().join(format!(
            "hkq_recon_gate_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    #[test]
    fn latch_is_first_wins_and_clean_never_unlatches() {
        let root = tmp();
        let d1 = chrono::NaiveDate::from_ymd_opt(2026, 7, 2).unwrap();
        let d2 = chrono::NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        assert!(!load_state(&state_path(&root)).unwrap().breached); // missing ⇒ fresh
        let st = latch_breach(&root, d1, "2 breaches, worst 941").unwrap();
        assert!(st.breached);
        let st = latch_breach(&root, d2, "later breach").unwrap();
        assert_eq!(st.breach_date.as_deref(), Some("2026-07-02")); // first wins
        let st = record_clean(&root, d2).unwrap();
        assert!(st.breached); // clean NEVER un-latches
        assert_eq!(st.last_clean_date.as_deref(), Some("2026-07-03"));
        // Backdated clean run cannot move the watermark backwards.
        let st = record_clean(&root, d1).unwrap();
        assert_eq!(st.last_clean_date.as_deref(), Some("2026-07-03"));
        std::fs::remove_dir_all(root).ok();
    }

    #[test]
    fn startup_gate_halts_on_latch_and_on_corruption() {
        let root = tmp();
        let lake = Lake::new(&root);
        let (ks, _rx) = KillSwitch::new();
        assert!(!startup_gate(&lake, &ks).unwrap().breached);
        assert!(!ks.current().halted()); // clean cold start: no halt

        latch_breach(&root, chrono::NaiveDate::from_ymd_opt(2026, 7, 3).unwrap(), "x").unwrap();
        let (ks2, _rx2) = KillSwitch::new();
        let st = startup_gate(&lake, &ks2).unwrap();
        assert!(st.breached && ks2.current().halted());

        // Corrupt latch: unreadable ⇒ HALT (a clean state must be provable).
        std::fs::write(state_path(&root), b"{ not json").unwrap();
        let (ks3, _rx3) = KillSwitch::new();
        assert!(startup_gate(&lake, &ks3).is_err());
        assert!(ks3.current().halted());
        std::fs::remove_dir_all(root).ok();
    }
}
```

## `hkq-recon` — the blueprint's fourth binary

```toml
# crates/hkq-recon/Cargo.toml
[package]
name = "hkq-recon"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-data = { path = "../hkq-data" }
hkq-validate = { path = "../hkq-validate" }
tokio.workspace = true
anyhow.workspace = true
chrono.workspace = true
chrono-tz.workspace = true
serde_json.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
```

```rust
// crates/hkq-recon/src/main.rs
//! The tripwire (blueprint bin: hkq-recon; report §5): nightly reconciliation
//! of our stored DailyBars closes against an INDEPENDENT official-close source.
//! Tick-aware, in Decimal, via hkq-data's M1 recon math:
//!   |Δ| > 1 tick ⇒ QUARANTINE the partition, LATCH `_state/recon.json`
//!   (hkq-live halts at next startup with HaltReason::ReconBreach), exit
//!   non-zero so cron pages the operator.
//!
//! Failure taxonomy, deliberate: price CONTRADICTIONS quarantine + latch;
//! ingest HOLES (official priced a universe name our lake lacks) page via
//! non-zero exit but neither quarantine (nothing is wrong — something is
//! absent) nor latch; names WE store that the official source skipped are
//! informational (vendors drop suspended names).
//!
//! Run order: T 18:00 hkq-nightly → T 18:30 hkq-recon. The EastMoney source is
//! a live-quote endpoint: after the CAS it serves the just-ended session's
//! official close, and it REFUSES historical dates — historical recon needs an
//! EOD-file provider behind the same trait (HKEX OMD-C in production, §5).
//!
//! Recovery after a breach: investigate, re-ingest with
//! `hkq-nightly <cfg> <date> --force`, re-run `hkq-recon`, then clear the
//! latch by hand. Un-latching is an operator edit, never code.
//!
//! Usage: hkq-recon <strategy.toml> [YYYY-MM-DD]
use anyhow::Context;
use chrono::{NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg, ids::StockCode,
               session::{DayKind, TradingCalendar}};
use hkq_data::{cfg::load_sources, eastmoney::{load_ah_map, EastMoneyClient},
               lake::{Dataset, Lake}, provider::OfficialCloseProvider,
               recon::reconcile_closes};
use hkq_validate::recon_gate;

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
    let date: Option<NaiveDate> = args.next()
        .map(|s| s.parse())
        .transpose()
        .context("date must be YYYY-MM-DD")?;

    let cfg = StrategyCfg::load(&cfg_path)?;
    let sources = load_sources(&cfg_path)?;
    let calendar = FileCalendar::load(&cfg.ops.calendar_path)?;
    let date = date.unwrap_or_else(|| Utc::now().with_timezone(&Hong_Kong).date_naive());

    if calendar.day_kind(date) == DayKind::Closed {
        tracing::info!(%date, "market closed; nothing to reconcile");
        return Ok(());
    }

    let lake = Lake::new(&cfg.ops.lake_root);
    anyhow::ensure!(
        lake.exists(Dataset::DailyBars, date),
        "no daily_bars partition for {date} — run hkq-nightly first"
    );

    let codes = load_universe_codes(&cfg.ops.universe_codes_path)?;
    let em_cfg = sources.eastmoney
        .context("[sources.eastmoney] (with hk_close_url_template) is required by hkq-recon")?;
    // AH map is irrelevant to recon; the dummy singleton is the hkq-nightly
    // precedent for constructing the client without one.
    let ah_map = match &cfg.ops.ah_map_path {
        Some(p) => load_ah_map(p)?,
        None => [(StockCode(0), String::new())].into_iter().collect(),
    };
    let em = EastMoneyClient::new(em_cfg, ah_map);

    let official = em.official_closes(&codes, date).await
        .context("official close fetch failed (independent recon source)")?;
    tracing::info!(rows = official.height(), %date, "official closes fetched");

    let report = reconcile_closes(&lake, &official, date)?;
    let worst = report.breaches.iter().max_by_key(|b| b.abs_diff);
    let summary = serde_json::json!({
        "date": date.to_string(),
        "checked": report.checked,
        "breaches": report.breaches.len(),
        "worst_breach": worst.map(|b| serde_json::json!({
            "code": b.code, "ours": b.ours, "official": b.official,
            "abs_diff": b.abs_diff.to_string(), "tick": b.tick.to_string(),
        })),
        "missing_ours": report.missing_ours.len(),
        "missing_official": report.missing_official.len(),
        "clean": report.clean(),
    });
    println!("{}", serde_json::to_string_pretty(&summary)?);

    if !report.breaches.is_empty() {
        let q = lake.quarantine(Dataset::DailyBars, date)?;
        let w = worst.expect("non-empty breaches have a maximum");
        let detail = format!(
            "{} close breach(es); worst code {} |Δ|={} (tick {}); daily_bars quarantined to {}",
            report.breaches.len(), w.code, w.abs_diff, w.tick, q.display()
        );
        recon_gate::latch_breach(lake.root(), date, &detail)?;
        anyhow::bail!("RECON BREACH — {detail}");
    }

    recon_gate::record_clean(lake.root(), date)?;

    if !report.missing_ours.is_empty() {
        // Ingest hole, not a price contradiction: nothing to quarantine, no
        // latch — but cron must page, and the operator must re-ingest.
        anyhow::bail!(
            "recon: {} universe name(s) priced officially but absent from our lake \
             (e.g. {:?}) — ingest hole; re-run `hkq-nightly {} --force`",
            report.missing_ours.len(),
            &report.missing_ours[..report.missing_ours.len().min(5)],
            date
        );
    }
    if !report.missing_official.is_empty() {
        tracing::warn!(n = report.missing_official.len(),
            "names we store that the official source did not price (suspensions?) — informational");
    }
    tracing::info!(checked = report.checked, %date, "recon CLEAN — tripwire recorded");
    Ok(())
}
```

Config addition (two lines in the existing `[sources.eastmoney]` table; both VERIFY items are data, not code):

```toml
[sources.eastmoney]
# … M1 fields unchanged …
hk_close_url_template = "https://push2.eastmoney.com/api/qt/ulist.np/get?fltt=2&fields=f2,f12&secids={secids}"
# hk_secid_prefix = "116"   # EastMoney HKEX market prefix ⇒ "116.00700" — VERIFY
```

## Honest gaps and hand-off to Milestone 10

Six items, each now a named fact. First, the official source is a same-day quote endpoint: it certifies the session that just ended and refuses history by type, so *retroactive* recon over the recorded backlog waits on an EOD-file provider (HKEX OMD-C daily quotes, the report's canonical source) behind the same `OfficialCloseProvider` trait — a substitution, not a redesign. Second, the tripwire covers `DailyBars` closes only: `Bars1m`, `Flows`, and `Auction` partitions keep the manifest tripwire but no cross-vendor price check; extending recon to southbound flows against HKEX's official Connect statistics is the natural next arm, same pattern. Third, corporate-action verification remains open — today's comparison is raw-close vs raw-close (apples to apples, since `adj_close == close` until `kline_adjust` is verified), but once an adjusted history exists the official quote source has nothing to reconcile it against; that needs a corporate-actions dataset, a data acquisition with a data-layer owner. Fourth, the recon latch reset is deliberately manual — no auto-unlatch job will ever exist, per the CUSUM precedent that un-halting is an operator edit. Fifth, the previously named remainders are unchanged with unchanged owners: the Hansen SPA benchmark family (HSI open→close and sector-ETF series that nothing ingests), venue-tagged Fills (schema v2) to unstarve κ calibration, and the §3.7 ML layer with CPCV and the self-hosted walk-forward. Sixth, one operational note the runbook should carry: after any quarantine, PreMarket's warmup panels will span the hole silently — which is precisely why the latch halts trading until re-ingest; the latch *is* the safety property, not the log line. With this milestone the blueprint's topology is complete — every crate and all four binaries exist — and the machine that already had facts, opinions, decisions, a clock, a memory, a reflex, a hand, a mirror, and judgment now has the last thing §5 demanded of it: a tripwire under its own facts, wired to the same switch that stops everything else.
