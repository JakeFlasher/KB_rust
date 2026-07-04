 
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


 
# Milestone 9 — The Evidence (benchmark & official-close ingestion + `hkq-recon` + the §4 SPA gate)

**Why this is next.** M8 completed judgment — any recorded day replays under honest as-of state, and the DSR gate finally has promotion-grade trials to weigh — but §4's verdict is still evidenced against nothing external: the report demands the SPA test against {always-cash, HSI open→close, sector-ETF Stage-1}, and two of those three arms require benchmark series that no dataset carries. Meanwhile the lake itself has been running on unverified trust for eight milestones: the recon function and quarantine path have existed since M1, tick-aware and tested, while the binary that should call them nightly was never built because reconciling Tiger against Tiger is circular. Both gaps — and the never-written `Vhsi` dataset — are pure data acquisitions whose consumers are already named in code comments and hand-offs. This milestone acquires the data and lands the two consumers that need zero engine surgery (SPA in `hkq-validate`, recon as the blueprint's fourth binary), and starts the VHSI history clock so the regime axis is warm when its engine consumer lands. After M9, the blueprint's binary table is complete for the first time.

**In scope:** `Dataset::Benchmarks` and its first writer (a config-templated, schema-versioned index/ETF kline provider in the M1 EastMoney discipline — every symbol a `(symbol, secid)` config pair); the VHSI side-write into the long-dormant `Dataset::Vhsi` (`[date, level]`, accumulating the trailing distribution its future tercile consumer needs); the `OfficialCloseProvider` trait and an HKEX-shaped adapter (config URL template, alias tables, loud `SchemaDrift`); the nightly `benchmarks` step; the `hkq-recon` binary (fetch official closes → `reconcile_closes` → quarantine on breach → non-zero exit, closing the M1 gap); and in `hkq-validate`, `spa.rs` — studentized mean differentials, Bartlett long-run variance, a joint circular block bootstrap, and the intersection-union orientation the promotion gate actually needs — plus the `spa` job that assembles the three arms and registry-logs the result. **Deferred:** the *engine* landings of VHSI (AlphaMap regime buckets, `Candidate.vhsi_tercile`, stratified IC reporting) — bundled with the next engine-touching milestone per the M5→M6 `ah_beta` precedent; A50 persistence and per-sector mainland betas (the `MorningBoard` still drops the stream); ADR feeds; κ calibration remains built-but-starved (unchanged owner: venue-tagged or accumulated tiger fills); CPCV, the self-hosted walk-forward, and §3.7 (unchanged); promotion enforcement stays operator governance (SPA *reports*; nothing edits config).

Engineering decisions beyond the blueprint sketch, briefly. **Snapshot ingest for benchmarks, deliberately:** each nightly run fetches the trailing `backfill_days` window per symbol and writes it as one ingest-dated partition — benchmarks are a handful of symbols × a few years of daily rows (kilobytes), and the SPA family needs *years* of history on night one, so a snapshot replaces a bespoke backfill job; consumers dedupe by `(symbol, date)` with last-write-wins, and idempotent re-runs plus the manifest tripwire behave exactly as for every other dataset. **SPA orientation, stated once and loudly:** Hansen (2005) tests whether the *best of many models* beats *one benchmark*; promotion needs the reverse — *one strategy* must dominate *every member* of a null family — which is an intersection-union test, so the statistic is the **minimum** studentized edge \(T = \min_k \sqrt{n}\,\bar d_k/\hat\omega_k\) and the p-value is the **maximum** per-arm bootstrap p (size-correct for IUT with no recentering gymnastics); the machinery — studentization, Bartlett long-run variance, one joint block-resampled time path per replicate so cross-arm dependence survives — is Hansen's, the inequality is reversed, and DSR remains the senior gate. **Cash days are zeros, not gaps:** `Fills`-derived PnL omits days the strategy stood aside, and skipping them would flatter the strategy against a rallying index — cash is a position (§3.2), so the strategy series is zero-filled over every open calendar day in its span. **Drop, never impute:** a date the benchmark family cannot honestly price (missing HSI row, unmapped selected sector, unpriced ETF) is dropped from the whole comparison and counted, loudly. **The independent-vendor rule is enforced at the call site:** `hkq-recon` refuses to run without `[sources.hkex]`, and its error message says why; breaches quarantine `daily_bars` by default (`--no-quarantine` for dry runs) and exit non-zero so cron pages. **The only frozen-crate surface touched is `hkq-data`, additively** — one enum variant, two capability traits, two config fields, two new modules; hkq-core, -factors, -signal, -risk, -exec, -engine, hkq-live, and hkq-backtest are byte-identical, and in hkq-validate every M8 module (`asof`, `kappa`, `cusum`, `ic`, `pnl`, `registry`, `fits`, `splits`, `stats`, `dsr`, `error`) stays byte-identical.

```text
hkq/
├── Cargo.toml                        (updated: member hkq-recon; no new deps, no new features)
└── crates/
    ├── hkq-data/
    │   └── src/{lib,model,provider,cfg,lake,bench,hkex}.rs
    │                                 (bench.rs, hkex.rs NEW; lib/model/provider/cfg/lake surgical
    │                                  patches; http, tiger, eastmoney, xueqiu, yahoo, recon,
    │                                  ingest stay byte-identical)
    ├── hkq-nightly/src/main.rs       (updated: benchmarks step + recon pointer)
    ├── hkq-recon/                    (NEW — the blueprint's fourth and final binary)
    │   ├── Cargo.toml
    │   └── src/main.rs
    └── hkq-validate/
        └── src/{lib,cfg,spa,main}.rs (spa.rs NEW; lib/cfg/main surgical patches; everything
                                       else byte-identical)
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
# test-util: tokio's virtual clock (M7 replay tier). Live binaries never pause the clock.
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

## Surgical patches to `hkq-data`

```rust
// (append inside crates/hkq-data/src/lib.rs, alphabetical with the other modules)
pub mod bench;
pub mod hkex;
```

```rust
// (append inside crates/hkq-data/src/model.rs `pub mod cols`)
    /// M9: Benchmarks dataset key — lake-facing benchmark symbol ("HSI", "2800", "VHSI").
    pub const SYMBOL: &str = "symbol";
    /// M9: Vhsi dataset value column — the index LEVEL, one row per date.
    pub const LEVEL: &str = "level";
```

```rust
// (replace the `Dataset` enum in crates/hkq-data/src/lake.rs)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dataset {
    DailyBars, Bars1m, Auction, Flows, Ccass, MainlandPrints, Fx, Vhsi, Fills, Attribution,
    /// M4: the live-frozen stage-2 frame persisted at PostClose — the raw material
    /// of the AlphaMap refit and the honest input of nightly attribution (M3 note).
    Scores,
    /// M9: daily benchmark/index rows [symbol, date, open, high, low, close] —
    /// the §4 SPA null family's raw material (HSI, sector ETFs). VHSI levels
    /// side-write into Dataset::Vhsi at ingest. Partitions are ingest-dated
    /// SNAPSHOTS (see hkq-data::bench); consumers dedupe by (symbol, date).
    Benchmarks,
}
```

```rust
// (replace `Dataset::dir` in crates/hkq-data/src/lake.rs)
    pub fn dir(self) -> &'static str {
        match self {
            Dataset::DailyBars => "daily_bars",  Dataset::Bars1m => "bars_1m",
            Dataset::Auction => "auction",       Dataset::Flows => "flows",
            Dataset::Ccass => "ccass",           Dataset::MainlandPrints => "mainland",
            Dataset::Fx => "fx",                 Dataset::Vhsi => "vhsi",
            Dataset::Fills => "fills",           Dataset::Attribution => "attribution",
            Dataset::Scores => "scores",         Dataset::Benchmarks => "benchmarks",
        }
    }
```

```rust
// (append inside crates/hkq-data/src/lake.rs `mod tests`)
    #[test]
    fn benchmarks_dataset_has_a_home() {
        assert_eq!(Dataset::Benchmarks.dir(), "benchmarks");
    }
```

```rust
// (append inside crates/hkq-data/src/provider.rs)
#[async_trait]
pub trait BenchmarkProvider: Send + Sync {
    /// Daily benchmark/index rows normalized to [symbol, date, open, high,
    /// low, close] — the §4 SPA family's raw material (M9). Window semantics
    /// are provider policy (snapshots allowed); consumers dedupe by
    /// (symbol, date) with last-write-wins.
    async fn benchmark_daily(&self, from: NaiveDate, to: NaiveDate)
        -> Result<DataFrame, DataError>;
}

#[async_trait]
pub trait OfficialCloseProvider: Send + Sync {
    /// INDEPENDENT official closing prices for `d`: [code, close_official].
    /// Report §5: this must never be the same vendor as the lake's EOD writer —
    /// reconciling a vendor against itself is circular.
    async fn official_closes(&self, d: NaiveDate) -> Result<DataFrame, DataError>;
}
```

```rust
// (append inside crates/hkq-data/src/cfg.rs `pub struct SourcesCfg`)
    /// M9: benchmark/index kline source (§4 SPA family + §5 VHSI level).
    #[serde(default)]
    pub bench: Option<crate::bench::BenchCfg>,
    /// M9: INDEPENDENT official-close source for hkq-recon (report §5).
    #[serde(default)]
    pub hkex: Option<crate::hkex::HkexCfg>,
```

## `hkq-data` — the benchmark provider

```rust
// crates/hkq-data/src/bench.rs
//! Benchmark/index ingestion (M9): the §4 SPA null family's raw material (HSI
//! open→close, sector-ETF Stage-1 arm) plus the §5 "VHSI level" data contract
//! that has had an empty Dataset::Vhsi variant waiting since M1.
//!
//! EastMoney-shaped push2his kline endpoint by default, but — the M1 rule —
//! the URL is a config template, every symbol is a config (symbol, secid)
//! pair, the parser is schema-versioned, and drift fails LOUDLY (SchemaDrift,
//! never an empty success).
//!
//! Snapshot semantics, deliberate and documented: each run fetches the
//! trailing `backfill_days` window per symbol and writes ONE ingest-dated
//! partition. Benchmarks are a handful of symbols × a few years of daily
//! rows — kilobytes — and the SPA family needs YEARS of history on night one,
//! so a snapshot ingest replaces a bespoke backfill job. Consumers dedupe by
//! (symbol, date) with last-write-wins; idempotent re-runs and the manifest
//! tripwire behave exactly as for every other dataset.
use crate::error::DataError;
use crate::http::RatedClient;
use crate::model::cols;
use crate::provider::BenchmarkProvider;
use async_trait::async_trait;
use chrono::NaiveDate;
use polars::{df, prelude::*};
use serde::Deserialize;
use serde_json::Value;

fn default_rps() -> u32 { 2 }
fn default_retries() -> u32 { 3 }
fn default_schema() -> u32 { 1 }
fn default_backfill() -> i64 { 750 }

#[derive(Debug, Clone, Deserialize)]
pub struct BenchSymbol {
    /// Lake-facing symbol ("HSI", "2800", "VHSI") — the SPA family's key.
    pub symbol: String,
    /// Vendor security id ("100.HSI", "116.02800", …). VERIFY per vendor.
    pub secid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BenchCfg {
    /// Kline URL template; `{secid}`, `{from}`, `{to}` substituted (YYYYMMDD).
    pub kline_url_template: String,
    pub symbols: Vec<BenchSymbol>,
    /// Rows for this symbol ALSO land in Dataset::Vhsi as [date, level] — the
    /// §5 VHSI contract. History accumulates NOW so the trailing tercile
    /// distribution is warm when its regime-axis consumer lands (the
    /// Auction/IEV_BAR20 self-healing precedent).
    #[serde(default)]
    pub vhsi_symbol: Option<String>,
    /// Trailing window fetched per run (snapshot semantics, module docs).
    #[serde(default = "default_backfill")]
    pub backfill_days: i64,
    #[serde(default = "default_schema")]
    pub schema_version: u32,
    #[serde(default = "default_rps")]
    pub rps: u32,
    #[serde(default = "default_retries")]
    pub max_retries: u32,
}

pub struct BenchClient {
    cfg: BenchCfg,
    http: RatedClient,
}

impl BenchClient {
    pub fn new(cfg: BenchCfg) -> Self {
        let http = RatedClient::new(cfg.rps, cfg.max_retries);
        Self { cfg, http }
    }

    pub fn vhsi_symbol(&self) -> Option<&str> {
        self.cfg.vhsi_symbol.as_deref()
    }

    pub fn backfill_days(&self) -> i64 {
        self.cfg.backfill_days.max(1)
    }
}

/// push2his kline, schema v1: { data: { klines: ["date,open,close,high,low,…"] } }
/// — the fields2 order in the URL template pins this layout. Returns
/// (date, open, high, low, close); short/garbled lines drop silently, a
/// missing array is DRIFT.
fn parse_klines_v1(v: &Value) -> Result<Vec<(String, f64, f64, f64, f64)>, DataError> {
    let arr = v.pointer("/data/klines").and_then(Value::as_array)
        .ok_or(DataError::SchemaDrift("bench kline: /data/klines missing"))?;
    let mut out = Vec::with_capacity(arr.len());
    for line in arr {
        let Some(s) = line.as_str() else { continue };
        let f: Vec<&str> = s.split(',').collect();
        if f.len() < 5 {
            continue;
        }
        let p = |i: usize| f[i].parse::<f64>().ok().filter(|x| x.is_finite() && *x > 0.0);
        let (Some(o), Some(c), Some(h), Some(l)) = (p(1), p(2), p(3), p(4)) else { continue };
        out.push((f[0].to_string(), o, h, l, c));
    }
    Ok(out)
}

#[async_trait]
impl BenchmarkProvider for BenchClient {
    async fn benchmark_daily(&self, from: NaiveDate, to: NaiveDate)
        -> Result<DataFrame, DataError>
    {
        let (lo, hi) = (from.to_string(), to.to_string());
        let (mut sym, mut d_, mut o_, mut h_, mut l_, mut c_) =
            (vec![], vec![], vec![], vec![], vec![], vec![]);
        let mut hard_failures = 0usize;
        for s in &self.cfg.symbols {
            let url = self.cfg.kline_url_template
                .replace("{secid}", &s.secid)
                .replace("{from}", &from.format("%Y%m%d").to_string())
                .replace("{to}", &to.format("%Y%m%d").to_string());
            let v: Value = match self.http.get_json(&url, &[]).await {
                Ok(v) => v,
                Err(e) => {
                    tracing::warn!(symbol = %s.symbol, error = %e,
                        "bench fetch failed; other symbols continue");
                    hard_failures += 1;
                    continue;
                }
            };
            let rows = match self.cfg.schema_version {
                1 => parse_klines_v1(&v)?,
                _ => return Err(DataError::SchemaDrift("bench kline: unknown schema_version")),
            };
            for (date, o, h, l, c) in rows {
                // Defensive range filter: templates may ignore beg/end params.
                if date.as_str() < lo.as_str() || date.as_str() > hi.as_str() {
                    continue;
                }
                sym.push(s.symbol.clone());
                d_.push(date);
                o_.push(o);
                h_.push(h);
                l_.push(l);
                c_.push(c);
            }
        }
        if sym.is_empty() {
            return Err(DataError::SchemaDrift(
                "bench: zero rows across all configured symbols (drift or dead endpoint?)"));
        }
        if hard_failures > 0 {
            tracing::warn!(hard_failures, "bench: some symbols failed this run");
        }
        Ok(df!(
            cols::SYMBOL => sym, cols::DATE => d_,
            cols::OPEN => o_, cols::HIGH => h_, cols::LOW => l_, cols::CLOSE => c_,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kline_v1_parses_and_skips_garbage() {
        let v = serde_json::json!({ "data": { "klines": [
            "2026-07-02,100.0,101.5,102.0,99.5,123,456",
            "garbage",
            "2026-07-03,101.5,100.9,101.9,100.1,124,457"
        ]}});
        let rows = parse_klines_v1(&v).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "2026-07-02");
        assert!((rows[0].1 - 100.0).abs() < 1e-12); // open
        assert!((rows[0].4 - 101.5).abs() < 1e-12); // close
        // A missing array is DRIFT, never an empty success.
        let bad = serde_json::json!({ "data": {} });
        assert!(parse_klines_v1(&bad).is_err());
    }
}
```

## `hkq-data` — the independent official-close source

```rust
// crates/hkq-data/src/hkex.rs
//! Independent official-close source (M9; report §5): the recon tripwire's
//! feed. The canonical publisher is HKEX's daily quotations; the exact wire
//! shape is unofficial, so — the M1 discipline — the URL is a config template,
//! the parser is schema-versioned, field names are alias tables, and zero
//! parsed rows is SchemaDrift, never an empty success.
//!
//! The one HARD rule lives at the call site (hkq-recon): this source must be a
//! DIFFERENT vendor from the lake's EOD writer. Reconciling Tiger against
//! Tiger is circular — that circularity IS the M1 gap this module closes.
use crate::error::DataError;
use crate::http::RatedClient;
use crate::model::cols;
use crate::provider::OfficialCloseProvider;
use async_trait::async_trait;
use chrono::NaiveDate;
use hkq_core::ids::StockCode;
use polars::{df, prelude::*};
use serde::Deserialize;
use serde_json::Value;

fn default_rps() -> u32 { 2 }
fn default_retries() -> u32 { 3 }
fn default_schema() -> u32 { 1 }
fn default_rows_ptr() -> String { "/data".into() }

/// Payload field aliases (M1 AuctionAliases pattern) — the exact names are
/// what must be VERIFIED against the chosen publisher.
#[derive(Debug, Clone, Deserialize)]
pub struct CloseAliases {
    pub code: Vec<String>,
    pub close: Vec<String>,
}

impl Default for CloseAliases {
    fn default() -> Self {
        Self {
            code: vec!["code".into(), "sym".into(), "symbol".into(), "stock_code".into()],
            close: vec!["close".into(), "close_official".into(), "nominal_price".into(),
                        "np".into(), "px".into()],
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HkexCfg {
    /// `{date}` (YYYY-MM-DD) and `{date_compact}` (YYYYMMDD) substituted.
    pub official_close_url_template: String,
    /// JSON pointer to the per-stock row array ("/data", "/result/rows", …).
    #[serde(default = "default_rows_ptr")]
    pub rows_pointer: String,
    #[serde(default)]
    pub aliases: Option<CloseAliases>,
    #[serde(default = "default_schema")]
    pub schema_version: u32,
    #[serde(default = "default_rps")]
    pub rps: u32,
    #[serde(default = "default_retries")]
    pub max_retries: u32,
}

pub struct HkexClient {
    cfg: HkexCfg,
    http: RatedClient,
    aliases: CloseAliases,
}

impl HkexClient {
    pub fn new(cfg: HkexCfg) -> Self {
        let http = RatedClient::new(cfg.rps, cfg.max_retries);
        let aliases = cfg.aliases.clone().unwrap_or_default();
        Self { cfg, http, aliases }
    }
}

/// Nominal prices arrive as numbers OR display strings ("1,234.50") — strip
/// separators before parsing; refuse non-positive prices.
fn close_of(v: &Value, aliases: &[String]) -> Option<f64> {
    for k in aliases {
        match v.get(k) {
            Some(Value::Number(n)) => {
                return n.as_f64().filter(|x| x.is_finite() && *x > 0.0);
            }
            Some(Value::String(s)) => {
                let cleaned: String = s.chars().filter(|c| *c != ',' && *c != ' ').collect();
                if let Ok(x) = cleaned.parse::<f64>() {
                    if x.is_finite() && x > 0.0 {
                        return Some(x);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn code_of(v: &Value, aliases: &[String]) -> Option<StockCode> {
    for k in aliases {
        match v.get(k) {
            Some(Value::Number(n)) => {
                if let Some(u) = n.as_u64() {
                    return u32::try_from(u).ok().map(StockCode);
                }
            }
            Some(Value::String(s)) => {
                if let Some(c) = StockCode::parse(s) {
                    return Some(c);
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_v1(v: &Value, ptr: &str, aliases: &CloseAliases) -> Result<Vec<(u32, f64)>, DataError> {
    let rows = v.pointer(ptr).and_then(Value::as_array)
        .ok_or(DataError::SchemaDrift("official closes: rows pointer missing"))?;
    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let (Some(code), Some(close)) = (code_of(r, &aliases.code), close_of(r, &aliases.close))
        else { continue };
        out.push((code.0, close));
    }
    if out.is_empty() {
        return Err(DataError::SchemaDrift("official closes: zero rows parsed"));
    }
    Ok(out)
}

#[async_trait]
impl OfficialCloseProvider for HkexClient {
    async fn official_closes(&self, d: NaiveDate) -> Result<DataFrame, DataError> {
        let url = self.cfg.official_close_url_template
            .replace("{date}", &d.to_string())
            .replace("{date_compact}", &d.format("%Y%m%d").to_string());
        let v: Value = self.http.get_json(&url, &[]).await?;
        let rows = match self.cfg.schema_version {
            1 => parse_v1(&v, &self.cfg.rows_pointer, &self.aliases)?,
            _ => return Err(DataError::SchemaDrift("official closes: unknown schema_version")),
        };
        let (codes, closes): (Vec<u32>, Vec<f64>) = rows.into_iter().unzip();
        // Column contract shared with hkq-data::recon since M1.
        Ok(df!(cols::CODE => codes, "close_official" => closes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_aliases_and_display_strings() {
        let a = CloseAliases::default();
        let v = serde_json::json!({ "data": [
            { "sym": "00700", "nominal_price": "1,234.50" },
            { "code": 5, "close": 60.05 },
            { "sym": "bad", "close": "n/a" }
        ]});
        let rows = parse_v1(&v, "/data", &a).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, 700);
        assert!((rows[0].1 - 1234.50).abs() < 1e-9);
        assert_eq!(rows[1].0, 5);
        // Zero parseable rows ⇒ drift, never an empty success.
        let junk = serde_json::json!({ "data": [ { "sym": "x", "close": "?" } ] });
        assert!(parse_v1(&junk, "/data", &a).is_err());
    }
}
```

## `hkq-nightly` — the benchmarks step wired in

```rust
// crates/hkq-nightly/src/main.rs
//! T−1 18:00 nightly job: EOD + flows + mainland prints + 1-minute bars +
//! benchmarks → lake. Usage: hkq-nightly <strategy.toml> [YYYY-MM-DD] [--force]
//!
//! M9 addition: the benchmarks step (Dataset::Benchmarks snapshot + the VHSI
//! side-write into the Dataset::Vhsi variant that has waited since M1), and
//! the recon log line finally points at a real binary — `hkq-recon` — instead
//! of naming a gap.
use anyhow::Context;
use chrono::{Duration, NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg, ids::StockCode,
               session::{DayKind, TradingCalendar}};
use hkq_data::{bench::BenchClient, cfg::load_sources,
               eastmoney::{load_ah_map, EastMoneyClient},
               ingest::{NightlyIngest, StepOutcome, StepReport},
               lake::{Dataset, Lake}, model::cols as dcols,
               provider::{BenchmarkProvider, FlowProvider, IntradayFeed, LinkedMarketFeed},
               tiger::TigerClient};
use polars::prelude::{col, lit, DataFrame, IntoLazy};

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
/// Bars1m partition (M3). Idempotent skip; per-code failure isolation.
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

/// M9: benchmark snapshot (trailing `backfill_days` per configured symbol) →
/// Dataset::Benchmarks, plus the VHSI side-write → Dataset::Vhsi. One
/// ingest-dated partition each; consumers dedupe by (symbol, date) /
/// (date) with last-write-wins — see hkq-data::bench module docs.
async fn step_benchmarks(
    lake: &Lake, client: &BenchClient, date: NaiveDate, force: bool,
) -> StepReport {
    let name = "benchmarks";
    if !force && lake.exists(Dataset::Benchmarks, date) {
        return StepReport { name, outcome: StepOutcome::SkippedExisting };
    }
    let from = date - Duration::days(client.backfill_days());
    let outcome = match client.benchmark_daily(from, date).await {
        Ok(mut df) if df.height() > 0 => {
            match lake.write_partition(Dataset::Benchmarks, date, &mut df, "bench:kline", 1) {
                Ok(()) => {
                    if let Some(vs) = client.vhsi_symbol() {
                        let lvl = df.clone().lazy()
                            .filter(col(dcols::SYMBOL).eq(lit(vs)))
                            .select([col(dcols::DATE),
                                     col(dcols::CLOSE).alias(dcols::LEVEL)])
                            .collect();
                        match lvl {
                            Ok(mut lvl) if lvl.height() > 0 => {
                                if let Err(e) = lake.write_partition(
                                    Dataset::Vhsi, date, &mut lvl, "bench:vhsi", 1)
                                {
                                    tracing::warn!(error = %e,
                                        "vhsi side-write failed (benchmarks partition still written)");
                                }
                            }
                            Ok(_) => tracing::warn!(symbol = vs,
                                "vhsi symbol configured but no rows fetched"),
                            Err(e) => tracing::warn!(error = %e, "vhsi filter failed"),
                        }
                    }
                    StepOutcome::Written { rows: df.height() }
                }
                Err(e) => StepOutcome::Failed(e.to_string()),
            }
        }
        Ok(_) => StepOutcome::Failed("benchmarks: zero rows".into()),
        Err(e) => StepOutcome::Failed(e.to_string()),
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

    let hkex_configured = sources.hkex.is_some();
    let codes = load_universe_codes(&cfg.ops.universe_codes_path)?;
    let tiger = TigerClient::new(sources.tiger.context("[sources.tiger] is required")?)?;
    let bench = sources.bench.map(BenchClient::new);

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

    // M3: 1-minute bars — rv/rv_5d/lav, seasonal profiles, the §3.5 label window.
    let bars = step_bars_1m(&lake, &tiger, &codes, date, force).await;
    match &bars.outcome {
        StepOutcome::Failed(e) => tracing::error!(step = bars.name, error = %e, "nightly step FAILED"),
        o => tracing::info!(step = bars.name, outcome = ?o, "nightly step done"),
    }
    report.steps.push(bars);

    // M9: benchmarks (SPA family) + VHSI levels (regime axis, accumulating).
    let bench_step = match &bench {
        Some(b) => step_benchmarks(&lake, b, date, force).await,
        None => {
            tracing::warn!("[sources.bench] absent — SPA family and VHSI stay unbuildable");
            StepReport { name: "benchmarks", outcome: StepOutcome::SkippedNoProvider }
        }
    };
    match &bench_step.outcome {
        StepOutcome::Failed(e) => tracing::error!(step = bench_step.name, error = %e, "nightly step FAILED"),
        o => tracing::info!(step = bench_step.name, outcome = ?o, "nightly step done"),
    }
    report.steps.push(bench_step);

    // M9: the recon line finally names a binary instead of a gap.
    if hkex_configured {
        tracing::info!("official-close source configured — run `hkq-recon` after this ingest (report §5)");
    } else {
        tracing::warn!("recon source not configured ([sources.hkex] absent) — the M1 gap persists");
    }

    anyhow::ensure!(report.all_ok(), "one or more nightly steps failed: {report:?}");
    tracing::info!(%date, "nightly ingest complete");
    Ok(())
}
```

## `hkq-recon` — the blueprint's fourth and final binary

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
//! Close reconciliation (M9 — the blueprint's fourth and final binary, closing
//! the M1 gap). Report §5: our stored close vs the official print; any
//! |Δ| > 1 tick ⇒ QUARANTINE the partition and page the operator (non-zero
//! exit so cron/alerting notices). The recon math and the quarantine path have
//! existed — tick-aware, in Decimal — since M1; what never existed was an
//! INDEPENDENT source to compare against, because reconciling the EOD vendor
//! against itself is circular. Run AFTER hkq-nightly for the same date.
//!
//! Usage: hkq-recon <strategy.toml> [YYYY-MM-DD] [--no-quarantine]
use anyhow::{bail, Context};
use chrono::{NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg,
               session::{DayKind, TradingCalendar}};
use hkq_data::{cfg::load_sources, hkex::HkexClient, lake::{Dataset, Lake},
               provider::OfficialCloseProvider, recon::reconcile_closes};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("info".parse()?))
        .init();

    let mut args = std::env::args().skip(1);
    let cfg_path = args.next().unwrap_or_else(|| "config/strategy.toml".into());
    let mut date: Option<NaiveDate> = None;
    let mut quarantine = true;
    for a in args {
        if a == "--no-quarantine" { quarantine = false; }
        else { date = Some(a.parse().context("date must be YYYY-MM-DD")?); }
    }

    let cfg = StrategyCfg::load(&cfg_path)?;
    let sources = load_sources(&cfg_path)?;
    let calendar = FileCalendar::load(&cfg.ops.calendar_path)?;
    let date = date.unwrap_or_else(|| Utc::now().with_timezone(&Hong_Kong).date_naive());
    if calendar.day_kind(date) == DayKind::Closed {
        tracing::info!(%date, "market closed; nothing to reconcile");
        return Ok(());
    }

    // The HARD rule: an INDEPENDENT vendor, or no recon at all.
    let hkex = sources.hkex.context(
        "[sources.hkex] is required: recon needs an INDEPENDENT official-close source — \
         reconciling the EOD vendor against itself is circular (report §5)")?;
    let client = HkexClient::new(hkex);
    let official = client.official_closes(date).await
        .context("official closes fetch/parse failed (schema drift is loud by design)")?;

    let lake = Lake::new(&cfg.ops.lake_root);
    let report = reconcile_closes(&lake, &official, date)
        .context("recon failed — has hkq-nightly written daily_bars for this date?")?;

    let breaches: Vec<_> = report.breaches.iter().map(|b| json!({
        "code": b.code,
        "ours": b.ours,
        "official": b.official,
        "abs_diff": b.abs_diff.to_string(),
        "tick": b.tick.to_string(),
    })).collect();
    println!("{}", serde_json::to_string_pretty(&json!({
        "date": date.to_string(),
        "checked": report.checked,
        "breaches": breaches,
        "missing_ours": report.missing_ours,
        "missing_official": report.missing_official,
        "clean": report.clean(),
    }))?);

    if !report.breaches.is_empty() && quarantine {
        let dst = lake.quarantine(Dataset::DailyBars, date)?;
        tracing::error!(dst = %dst.display(),
            "daily_bars QUARANTINED — fix the source, then `hkq-nightly {date} --force`");
    }
    if !report.clean() {
        bail!(
            "recon NOT clean for {date}: {} price breaches, {} names missing from our lake — operator attention",
            report.breaches.len(), report.missing_ours.len()
        );
    }
    tracing::info!(%date, checked = report.checked, "recon clean");
    Ok(())
}
```

## `hkq-validate` — the SPA gate

```rust
// (append inside crates/hkq-validate/src/lib.rs, alphabetical with the other modules)
pub mod spa;
```

```rust
// (append to the re-exports in crates/hkq-validate/src/lib.rs)
pub use spa::{spa_pvalue, SpaArm, SpaOutcome};
```

```rust
// (append to the imports in crates/hkq-validate/src/cfg.rs)
use std::collections::BTreeMap;
```

```rust
// (append alongside the default helpers in crates/hkq-validate/src/cfg.rs)
fn d_hsi() -> String { "HSI".into() }
fn d_spa_block() -> usize { 10 }    // §4: block length 10d — the report's bootstrap convention
fn d_spa_boot() -> usize { 2000 }
```

```rust
// (append inside crates/hkq-validate/src/cfg.rs `pub struct ValidateCfg`)
    /// M9 — §4 SPA family: the Benchmarks-dataset symbol carrying the HSI arm.
    #[serde(default = "d_hsi")]
    pub hsi_symbol: String,
    /// Sector id (as string) → Benchmarks symbol for the Stage-1 ETF arm.
    /// Empty ⇒ the arm is skipped, loudly (family degraded, still reported).
    #[serde(default)]
    pub etf_map: BTreeMap<String, String>,
    #[serde(default = "d_spa_block")]
    pub spa_block: usize,
    #[serde(default = "d_spa_boot")]
    pub spa_boot: usize,
```

```rust
// (replace `impl Default for ValidateCfg` in crates/hkq-validate/src/cfg.rs)
impl Default for ValidateCfg {
    fn default() -> Self {
        Self {
            cusum_mu0: d_mu0(), cusum_k: d_k(), cusum_h: d_h(),
            fit_window_days: d_window(), fit_min_obs: d_min_obs(),
            vstar_grid: d_grid(), cv_folds: d_folds(), cv_embargo: d_embargo(),
            trials_registry_path: None,
            hsi_symbol: d_hsi(), etf_map: BTreeMap::new(),
            spa_block: d_spa_block(), spa_boot: d_spa_boot(),
        }
    }
}
```

```rust
// crates/hkq-validate/src/spa.rs
//! The §4 SPA gate (M9): the strategy against the report's null family
//! {always-cash, HSI open→close, sector-ETF Stage-1}.
//!
//! Orientation note, stated once and loudly: Hansen (2005) tests whether the
//! BEST of many models beats ONE benchmark — a union-intersection problem.
//! Promotion needs the reverse: ONE strategy must dominate EVERY member of a
//! null family — an intersection-union test. Same machinery (studentized mean
//! differentials, Bartlett long-run variance, a JOINT circular block bootstrap
//! so cross-arm dependence survives resampling), reversed direction: the test
//! statistic is the MINIMUM studentized edge and the p-value is the MAXIMUM
//! per-arm bootstrap p — size-correct for IUT without recentering gymnastics,
//! and conservative exactly where §4 wants conservatism. The DSR (report job)
//! remains the SENIOR promotion gate; both must clear.
//!
//! Units: all series are per-day returns on ONE equity base (the job converts
//! Fills-PnL via the CLI equity), so differentials are scale-consistent and
//! the statistic is invariant to the base. Days the strategy stood aside are
//! ZEROS, not gaps — cash is a position (§3.2), and always-cash is a
//! competitor precisely because doing nothing is always available.
use crate::error::ValidateError;
use hkq_data::lake::{Dataset, Lake};
use hkq_factors::cols::{self, base};
use polars::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

/// Deterministic seed — the report job's bootstrap convention.
pub const SPA_SEED: u64 = 20260704;

#[derive(Debug, Clone)]
pub struct SpaArm {
    pub name: String,
    /// Mean daily edge, strategy − null (return units).
    pub mean: f64,
    /// √n · mean / σ_LR (Bartlett, lags = block).
    pub t: f64,
    /// One-sided bootstrap p for THIS arm.
    pub p: f64,
}

#[derive(Debug, Clone)]
pub struct SpaOutcome {
    /// max over arms (IUT): the whole family must be dominated.
    pub p_value: f64,
    /// The binding arm's studentized edge.
    pub t_min: f64,
    pub n_days: usize,
    pub n_boot: usize,
    pub arms: Vec<SpaArm>,
}

/// Bartlett long-run variance of the MEAN's numerator series. None on short
/// or degenerate (zero-variance) input — degeneracy is decided by sign, not
/// resampled.
fn long_run_var(x: &[f64], lags: usize) -> Option<f64> {
    let n = x.len();
    if n < 8 {
        return None;
    }
    let nf = n as f64;
    let m = x.iter().sum::<f64>() / nf;
    let d: Vec<f64> = x.iter().map(|v| v - m).collect();
    let mut s = d.iter().map(|v| v * v).sum::<f64>() / nf;
    for l in 1..=lags.min(n - 1) {
        let w = 1.0 - l as f64 / (lags as f64 + 1.0);
        let g = d.iter().zip(&d[l..]).map(|(a, b)| a * b).sum::<f64>() / nf;
        s += 2.0 * w * g;
    }
    (s.is_finite() && s > 0.0).then_some(s)
}

/// diffs[k][t] = strategy_t − null_k,t; every arm shares the time axis. Arms
/// with zero long-run variance are decided WITHOUT bootstrap: a constant
/// positive edge dominates (p = 0), a constant zero/negative one cannot
/// (p = 1) — you cannot out-trade a series you equal.
pub fn spa_pvalue(
    names: &[String], diffs: &[Vec<f64>], block: usize, n_boot: usize, seed: u64,
) -> Option<SpaOutcome> {
    let m = diffs.len();
    if m == 0 || names.len() != m || block == 0 || n_boot < 100 {
        return None;
    }
    let n = diffs[0].len();
    if n < 8 || n < 2 * block || diffs.iter().any(|d| d.len() != n) {
        return None;
    }
    if diffs.iter().flatten().any(|v| !v.is_finite()) {
        return None;
    }
    let nf = n as f64;

    let mut arms: Vec<SpaArm> = Vec::with_capacity(m);
    let mut lrv_sqrt: Vec<Option<f64>> = Vec::with_capacity(m);
    for (k, d) in diffs.iter().enumerate() {
        let mean = d.iter().sum::<f64>() / nf;
        match long_run_var(d, block) {
            Some(v) => {
                let sd = v.sqrt();
                arms.push(SpaArm { name: names[k].clone(), mean, t: nf.sqrt() * mean / sd, p: 0.0 });
                lrv_sqrt.push(Some(sd));
            }
            None => {
                let (t, p) = if mean > 0.0 {
                    (f64::INFINITY, 0.0)
                } else {
                    (f64::NEG_INFINITY, 1.0)
                };
                arms.push(SpaArm { name: names[k].clone(), mean, t, p });
                lrv_sqrt.push(None);
            }
        }
    }

    // Joint circular block bootstrap: ONE index path per replicate, applied to
    // every arm — cross-arm dependence survives resampling. Deterministic
    // xorshift PRNG, the stats.rs precedent (reproducible reports, no rand dep).
    let mut s = seed | 1;
    let mut next = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut exceed = vec![0usize; m];
    let mut idx: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n_boot {
        idx.clear();
        while idx.len() < n {
            let start = (next() % n as u64) as usize;
            for j in 0..block {
                if idx.len() == n {
                    break;
                }
                idx.push((start + j) % n);
            }
        }
        for k in 0..m {
            let Some(sd) = lrv_sqrt[k] else { continue };
            let d = &diffs[k];
            let mean_b = idx.iter().map(|&i| d[i]).sum::<f64>() / nf;
            // Centered at the sample mean — the exact H0 boundary for this arm.
            if nf.sqrt() * (mean_b - arms[k].mean) / sd >= arms[k].t {
                exceed[k] += 1;
            }
        }
    }
    for k in 0..m {
        if lrv_sqrt[k].is_some() {
            arms[k].p = exceed[k] as f64 / n_boot as f64;
        }
    }

    let p_value = arms.iter().map(|a| a.p).fold(0.0_f64, f64::max);
    let t_min = arms.iter().map(|a| a.t).fold(f64::INFINITY, f64::min);
    Some(SpaOutcome { p_value, t_min, n_days: n, n_boot, arms })
}

// ───────────────────────── benchmark-series builders ────────────────────────

/// Benchmarks → (symbol, date) → ln(close/open). Snapshot partitions overlap
/// by design; dedupe is last-write-wins by (symbol, date) — hkq-data::bench.
pub fn benchmark_returns(lake: &Lake) -> Result<BTreeMap<(String, String), f64>, ValidateError> {
    let df = lake.scan(Dataset::Benchmarks)
        .map_err(|_| ValidateError::Insufficient(
            "no benchmarks history — configure [sources.bench] and run hkq-nightly"))?
        .select([col(base::SYMBOL), col(base::DATE), col(base::OPEN), col(base::CLOSE)])
        .collect()?;
    let sym = df.column(base::SYMBOL)?.as_materialized_series().str()?.clone();
    let dt = df.column(base::DATE)?.as_materialized_series().str()?.clone();
    let o = df.column(base::OPEN)?.as_materialized_series().f64()?.clone();
    let c = df.column(base::CLOSE)?.as_materialized_series().f64()?.clone();
    let mut out = BTreeMap::new();
    for i in 0..df.height() {
        let (Some(s), Some(d), Some(oo), Some(cc)) = (sym.get(i), dt.get(i), o.get(i), c.get(i))
        else { continue };
        if oo > 0.0 && cc > 0.0 {
            let r = (cc / oo).ln();
            if r.is_finite() {
                out.insert((s.to_string(), d.to_string()), r);
            }
        }
    }
    if out.is_empty() {
        return Err(ValidateError::Insufficient("benchmarks dataset is empty"));
    }
    Ok(out)
}

/// Scores → date → distinct selected sectors: the Stage-1 verdict AS FROZEN
/// live (a date's Scores partition exists iff the day traded, and its sector
/// column is exactly the selection).
pub fn selected_sectors_by_date(
    lake: &Lake,
) -> Result<BTreeMap<String, BTreeSet<u32>>, ValidateError> {
    let df = lake.scan(Dataset::Scores)
        .map_err(|_| ValidateError::Insufficient("no scores history (shadow not started?)"))?
        .select([col(base::DATE), col(cols::SECTOR)])
        .collect()?;
    let dt = df.column(base::DATE)?.as_materialized_series().str()?.clone();
    let sec = df.column(cols::SECTOR)?.as_materialized_series().u32()?.clone();
    let mut out: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    for i in 0..df.height() {
        if let (Some(d), Some(s)) = (dt.get(i), sec.get(i)) {
            out.entry(d.to_string()).or_default().insert(s);
        }
    }
    Ok(out)
}

/// The sector-ETF Stage-1 arm for one date: equal-weight mean of the selected
/// sectors' ETF open→close returns. No selection ⇒ Some(0.0) — the cash day
/// is a POSITION, faithfully mirrored. None ⇒ the arm cannot honestly price
/// this date (unmapped sector or missing ETF row): the caller DROPS the date
/// from the whole comparison and counts it — imputing would flatter someone.
pub fn etf_arm_for_date(
    selected: Option<&BTreeSet<u32>>,
    etf_map: &BTreeMap<String, String>,
    bench: &BTreeMap<(String, String), f64>,
    date: &str,
) -> Option<f64> {
    let Some(sel) = selected.filter(|s| !s.is_empty()) else { return Some(0.0) };
    let mut acc = 0.0;
    for s in sel {
        let sym = etf_map.get(&s.to_string())?;
        let r = bench.get(&(sym.clone(), date.to_string()))?;
        acc += *r;
    }
    Some(acc / sel.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use polars::df;

    fn noise(i: u64) -> f64 {
        ((i.wrapping_mul(2654435761) % 97) as f64) / 4800.0 - 0.01 // ≈ ±1%, mean ≈ 0
    }

    #[test]
    fn dominant_strategy_clears_and_iut_takes_the_max() {
        let n = 300u64;
        // Arm A: strong 30 bps/day edge. Arm B: 1 bp — indistinguishable from luck.
        let a: Vec<f64> = (0..n).map(|i| 0.0030 + noise(i)).collect();
        let b: Vec<f64> = (0..n).map(|i| 0.0001 + noise(i + 13)).collect();
        let names = vec!["a".to_string(), "b".to_string()];
        let out = spa_pvalue(&names, &[a.clone(), b], 10, 400, 7).unwrap();
        assert!(out.arms[0].p < 0.05, "strong arm p {}", out.arms[0].p);
        assert!(out.p_value > 0.2, "family p must be bound by the weak arm: {}", out.p_value);
        assert!(out.p_value >= out.arms[0].p && out.p_value >= out.arms[1].p);
        assert!(out.t_min <= out.arms[0].t);
        // Alone, the strong arm clears the gate — and deterministically so.
        let solo = spa_pvalue(&names[..1].to_vec(), &[a.clone()], 10, 400, 7).unwrap();
        let again = spa_pvalue(&names[..1].to_vec(), &[a], 10, 400, 7).unwrap();
        assert!(solo.p_value < 0.05);
        assert_eq!(solo.p_value, again.p_value);
    }

    #[test]
    fn degenerate_arms_are_decided_by_sign() {
        let names = vec!["x".to_string()];
        // Identical to the null: cannot dominate what you equal.
        let out = spa_pvalue(&names, &[vec![0.0; 100]], 10, 200, 5).unwrap();
        assert_eq!(out.p_value, 1.0);
        // A constant positive edge is riskless dominance.
        let out = spa_pvalue(&names, &[vec![0.0005; 100]], 10, 200, 5).unwrap();
        assert_eq!(out.p_value, 0.0);
        // Contract guards: empty family, short series, ragged arms.
        assert!(spa_pvalue(&[], &[], 10, 200, 5).is_none());
        assert!(spa_pvalue(&names, &[vec![0.0; 5]], 10, 200, 5).is_none());
        assert!(spa_pvalue(
            &["p".into(), "q".into()],
            &[vec![0.0; 100], vec![0.0; 99]], 10, 200, 5
        ).is_none());
    }

    #[test]
    fn benchmark_readers_and_etf_arm() {
        let root = std::env::temp_dir().join(format!(
            "hkq_spa_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let lake = Lake::new(&root);
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();

        let mut bench_df = df!(
            "symbol" => vec!["HSI".to_string(), "E1".to_string()],
            "date" => vec![d.to_string(); 2],
            "open" => vec![24_000.0, 100.0],
            "high" => vec![24_300.0, 101.5],
            "low" => vec![23_900.0, 99.5],
            "close" => vec![24_240.0, 101.0],
        ).unwrap();
        lake.write_partition(Dataset::Benchmarks, d, &mut bench_df, "test", 1).unwrap();
        let mut scores = df!(
            "date" => vec![d.to_string()],
            "code" => vec![700u32],
            "sector" => vec![1u32],
            "score" => vec![1.0f64],
        ).unwrap();
        lake.write_partition(Dataset::Scores, d, &mut scores, "test", 1).unwrap();

        let bench = benchmark_returns(&lake).unwrap();
        let hsi = bench[&("HSI".to_string(), d.to_string())];
        assert!((hsi - (24_240f64 / 24_000.0).ln()).abs() < 1e-12);

        let sel = selected_sectors_by_date(&lake).unwrap();
        let mut map = BTreeMap::new();
        map.insert("1".to_string(), "E1".to_string());
        let v = etf_arm_for_date(sel.get(&d.to_string()), &map, &bench, &d.to_string()).unwrap();
        assert!((v - (101f64 / 100.0).ln()).abs() < 1e-12);
        // Unmapped sector ⇒ None: the date must be dropped, loudly — never imputed.
        assert!(etf_arm_for_date(sel.get(&d.to_string()), &BTreeMap::new(), &bench,
                                 &d.to_string()).is_none());
        // No selection ⇒ cash ⇒ exactly zero.
        assert_eq!(etf_arm_for_date(None, &map, &bench, "2026-07-06"), Some(0.0));
        std::fs::remove_dir_all(root).ok();
    }
}
```

## `hkq-validate` — the `spa` job

```rust
// (replace the usage doc comment at the top of crates/hkq-validate/src/main.rs)
//! §4 protocol jobs. Deliberately synchronous — pure batch over the lake.
//!
//! Usage:
//!   hkq-validate <strategy.toml> cusum
//!   hkq-validate <strategy.toml> fit-quarterly [YYYY-MM-DD]
//!   hkq-validate <strategy.toml> report
//!   hkq-validate <strategy.toml> asof-state [YYYY-MM-DD]     (M8: audit artifact)
//!   hkq-validate <strategy.toml> fit-kappa                   (M8: κ from fills)
//!   hkq-validate <strategy.toml> spa <EQUITY_HKD>            (M9: §4 SPA vs null family)
//!
//! `cusum` exits non-zero on a latched breach so cron/alerting notices; the
//! authoritative runtime producer is hkq-live's startup gate, not this job.
```

```rust
// (append to the imports in crates/hkq-validate/src/main.rs)
use hkq_core::calendar::FileCalendar;
use hkq_core::session::{DayKind, TradingCalendar};
use hkq_validate::spa;
```

```rust
// (replace the argument-parsing + dispatch block inside `fn main` in
//  crates/hkq-validate/src/main.rs — from `let mut args` through the `match`)
    let mut args = std::env::args().skip(1);
    let cfg_path = args.next().context(
        "usage: hkq-validate <strategy.toml> <cusum|fit-quarterly|report|asof-state|fit-kappa|spa> [ARG]")?;
    let cmd = args.next().context(
        "missing subcommand: cusum | fit-quarterly | report | asof-state | fit-kappa | spa")?;
    let extra: Option<String> = args.next();
    let date_arg: Option<NaiveDate> = match cmd.as_str() {
        "fit-quarterly" | "asof-state" => extra.as_deref().map(str::parse).transpose()
            .context("date must be YYYY-MM-DD")?,
        _ => None,
    };

    let cfg = StrategyCfg::load(&cfg_path)?;
    let vcfg = load_validate(&cfg_path)?;
    let lake = Lake::new(&cfg.ops.lake_root);

    match cmd.as_str() {
        "cusum" => job_cusum(&lake, &vcfg),
        "fit-quarterly" => job_fit_quarterly(&cfg, &cfg_path, &vcfg, &lake, date_arg),
        "report" => job_report(&vcfg, &lake),
        "asof-state" => job_asof_state(&vcfg, &lake, date_arg),
        "fit-kappa" => job_fit_kappa(&cfg, &cfg_path, &vcfg, &lake),
        "spa" => job_spa(&cfg, &cfg_path, &vcfg, &lake, extra),
        other => bail!("unknown subcommand: {other}"),
    }
```

```rust
// (append at the end of crates/hkq-validate/src/main.rs)
/// M9: the §4 SPA gate against {always-cash, HSI open→close, sector-ETF
/// Stage-1}. Reports + registry-logs; the DSR (report job) stays the SENIOR
/// promotion gate — both must clear, and neither automates a config edit.
fn job_spa(
    cfg: &StrategyCfg, cfg_path: &str, vcfg: &ValidateCfg, lake: &Lake,
    equity_arg: Option<String>,
) -> anyhow::Result<()> {
    let equity: f64 = equity_arg
        .context("usage: hkq-validate <strategy.toml> spa <EQUITY_HKD>")?
        .parse()
        .context("equity must be a number (HKD)")?;
    anyhow::ensure!(equity > 0.0, "equity must be positive");

    let series = match pnl::daily_pnl(lake) {
        Ok(s) if !s.is_empty() => s,
        _ => {
            tracing::warn!("no fills history — nothing to test");
            return Ok(());
        }
    };
    let bench = match spa::benchmark_returns(lake) {
        Ok(b) => b,
        Err(e) => {
            tracing::warn!(error = %e,
                "benchmarks unavailable — configure [sources.bench] and run hkq-nightly");
            return Ok(());
        }
    };

    // Strategy returns on EVERY open day in the span — cash days are zeros,
    // not gaps: skipping them would flatter the strategy against a rallying
    // index, and always-cash is a competitor precisely because standing aside
    // is always available (§3.2).
    let calendar = FileCalendar::load(&cfg.ops.calendar_path)?;
    let pnl_map: BTreeMap<String, f64> = series.into_iter().collect();
    let from: NaiveDate = pnl_map.keys().next().expect("non-empty").parse()?;
    let to: NaiveDate = pnl_map.keys().next_back().expect("non-empty").parse()?;
    let sectors = spa::selected_sectors_by_date(lake).unwrap_or_default();
    let use_etf = !vcfg.etf_map.is_empty();
    if !use_etf {
        tracing::warn!("[validate].etf_map empty — sector-ETF arm skipped (family degraded)");
    }

    let (mut strat, mut hsi, mut etf) = (Vec::new(), Vec::new(), Vec::new());
    let (mut cash_days, mut dropped) = (0usize, 0usize);
    let mut d = from;
    while d <= to {
        if calendar.day_kind(d) != DayKind::Closed {
            let ds = d.to_string();
            let r_h = bench.get(&(vcfg.hsi_symbol.clone(), ds.clone())).copied();
            let r_e = if use_etf {
                spa::etf_arm_for_date(sectors.get(&ds), &vcfg.etf_map, &bench, &ds)
            } else {
                Some(0.0) // placeholder; the arm is excluded from `diffs` below
            };
            match (r_h, r_e) {
                (Some(h), Some(e)) => {
                    let r_s = match pnl_map.get(&ds) {
                        Some(p) => p / equity,
                        None => {
                            cash_days += 1;
                            0.0
                        }
                    };
                    strat.push(r_s);
                    hsi.push(h);
                    etf.push(e);
                }
                // Missing benchmark pricing: drop the date LOUDLY, never impute.
                _ => dropped += 1,
            }
        }
        d = d.succ_opt().context("date overflow")?;
    }
    anyhow::ensure!(strat.len() >= 40,
        "only {} comparable days — accumulate benchmark/shadow history before SPA", strat.len());
    if dropped > 0 {
        tracing::warn!(dropped, "dates without full benchmark pricing were dropped");
    }

    let mut names = vec![
        "always_cash".to_string(),
        format!("hsi_open_close({})", vcfg.hsi_symbol),
    ];
    let mut diffs: Vec<Vec<f64>> = vec![
        strat.clone(),                                          // strategy − 0
        strat.iter().zip(&hsi).map(|(s, b)| s - b).collect(),
    ];
    if use_etf {
        names.push("sector_etf_stage1".to_string());
        diffs.push(strat.iter().zip(&etf).map(|(s, b)| s - b).collect());
    }
    let out = spa::spa_pvalue(&names, &diffs, vcfg.spa_block, vcfg.spa_boot, spa::SPA_SEED)
        .context("SPA degenerate (series too short for the block length?)")?;

    let arms: Vec<_> = out.arms.iter().map(|a| json!({
        "name": a.name,
        "mean_edge_bps_per_day": a.mean * 1e4,
        "t": if a.t.is_finite() { json!(a.t) } else { json!(a.t.to_string()) },
        "p_boot": a.p,
    })).collect();
    println!("{}", serde_json::to_string_pretty(&json!({
        "days": out.n_days,
        "cash_days_zero_filled": cash_days,
        "dates_dropped_missing_benchmark": dropped,
        "block": vcfg.spa_block,
        "n_boot": out.n_boot,
        "arms": arms,
        "t_min": if out.t_min.is_finite() { json!(out.t_min) } else { json!(out.t_min.to_string()) },
        "spa_p_value": out.p_value,
        "clears_spa_at_5pct": out.p_value < 0.05,
        "note": "IUT orientation: the strategy must dominate EVERY null; p = max per-arm bootstrap p. DSR (report job) remains the senior §4 gate — both must clear; promotion stays an operator decision.",
    }))?);

    let reg = TrialsRegistry::open(vcfg.registry_path(lake.root()));
    let mut m = BTreeMap::new();
    m.insert("spa_p".to_string(), out.p_value);
    m.insert("days".to_string(), out.n_days as f64);
    m.insert("n_arms".to_string(), out.arms.len() as f64);
    if out.t_min.is_finite() {
        m.insert("t_min".to_string(), out.t_min);
    }
    reg.append("spa_test", &sha1_hex_of_file(cfg_path)?, &m,
               &format!("SPA vs null family: {}", names.join(", ")))?;
    Ok(())
}
```

Config additions (every VERIFY item is data, per the M1/M6 discipline):

```toml
[sources.bench]
# fields2 order (f51..f55 = date,open,close,high,low) PINS parser schema v1.
kline_url_template = "https://push2his.eastmoney.com/api/qt/stock/kline/get?secid={secid}&klt=101&fqt=0&fields1=f1&fields2=f51,f52,f53,f54,f55&beg={from}&end={to}"
symbols = [
  { symbol = "HSI",  secid = "100.HSI" },          # VERIFY secid per vendor
  { symbol = "VHSI", secid = "VERIFY_VHSI_SECID" },
  { symbol = "2800", secid = "116.02800" },        # ETFs for the Stage-1 arm
]
vhsi_symbol = "VHSI"
# backfill_days = 750   (snapshot window per run)

[sources.hkex]
# MUST be a different vendor from the lake's EOD writer (report §5).
official_close_url_template = "https://VERIFIED_INDEPENDENT_SOURCE/daily_quotes?date={date_compact}"
# rows_pointer = "/data"          (JSON pointer to the row array)
# aliases: code/close field-name lists — defaults documented in hkq-data::hkex

[validate]                # M9 additions to the existing table
hsi_symbol = "HSI"
spa_block = 10            # §4 block length
spa_boot = 2000

[validate.etf_map]        # sector id → Benchmarks symbol; empty ⇒ arm skipped loudly
# "1" = "2800"
```
 
