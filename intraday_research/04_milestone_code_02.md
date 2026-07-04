
The three documents are attached in full, so I worked directly from them — no fetching needed. Reading `03_milestone_code_1.md`'s hand-off section, the next milestone is unambiguous: M1 ends by naming exactly three seams built for a single consumer — `Lake::scan`/`scan_date`, the `cols` schema contract, and `LiveMux`'s event channel — and that consumer is the factor engine.

# Milestone 2 — The Factor Engine (`hkq-factors`)

**Why this is next.** Every remaining crate is downstream of factor output: `hkq-signal` combines factor z-scores it cannot compute, `hkq-risk` sizes candidates it cannot rank, `hkq-validate` monitors ICs that don't exist yet, and the engine's 09:29:30 `ScoreFreeze` phase is, mathematically, one call into this crate. Building it now also has a structural property the live-wiring milestones don't: it is *pure computation* — panel in, panel out — so every formula in report §3.0–§3.4 gets locked under deterministic unit tests before any of it is exposed to vendor I/O. That ordering (prove the math, then wire the clock) is what keeps M3/M4 from debugging statistics and sockets simultaneously.

**In scope:** the complete `hkq-factors` crate — cross-sectional math (Acklam Φ⁻¹, rank-inverse-normal, winsorize, sequential orthogonalization), the §3.0 nightly moments panel (ON/ID decomposition, EWMA vols, Amihud, LAV), realized measures from 1-minute bars (RV, RS±, RSJ, BV, jump, seasonal volume profiles, IVU + terciles), the lead–lag network (Schäfer–Strimmer shrink + Fisher-z + BH FDR), sector aggregation with capped float-cap weights and S3, Stage-1 S1–S6 with the Σ_min cash-day gate, Stage-2 X1–X7 with the IVU regime gate and the fixed §3.4 pipeline, ICIR weights, and the `PanelBuilder` seam onto the M1 lake. **Deferred:** confirmation χ and meta-labeling (`hkq-signal`, M3), sizing/stops (`hkq-risk`, M3), all live orchestration (M4), and the §4 validation crate.

Engineering decisions beyond the blueprint sketch, briefly: `FactorWeights` lives here, not in `hkq-signal` — the blueprint references it from `hkq-factors` as `crate::icir` while also sketching it in `hkq-signal`; both stages consume it, so the factor crate owns it and M3 re-exports. Intraday binning is pure integer math on M1's `ts_ms` column (HKT = UTC+8, no DST, so a constant offset is *correct*) instead of `dt().truncate` — no datetime dtype inference, consistent with M1's determinism stance. The blueprint's orthogonalization solved SVD against the normal-equations RHS, which is dimensionally wrong for non-square systems; fixed to a direct SVD least-squares solve. Missing data has one policy everywhere: non-finite → null at factor birth, nulls survive winsorize/rank untouched (they simply leave the cross-section), and nulls become 0 (the cross-sectional neutral) only at the final weighted combine — this single policy *is* the X2-disabled mode, the S2/X1 unconfirmed fallbacks, and the S6 partial-feed degradation, rather than per-factor special cases. Returns use an adjusted open (`open × adj_close/close`) so `r_on`/`r_id` are computed on one corporate-action-consistent series. S4's "HSI-hedged" residuals ship as per-date cross-sectional demeaning with the β-hedge swap point documented (the β estimation job belongs to `hkq-validate`). `capped_weights` handles the infeasible case the blueprint ignores (n·cap ≤ 1 ⇒ equal weights). Sector aggregation is eager Rust per (date, sector) partition — water-filling is not expressible as a polars expression, and the blueprint's own maxim applies: at ~10²–10³ rows per day, clarity beats vectorized cleverness. Workspace polars features extend additively (`ewma`, `rank`, `rolling_window`, `clip`, `sign`, `log`, `abs`, …) exactly as M1's pin note anticipated.

```text
hkq/
├── Cargo.toml                       (updated: member, polars features, nalgebra)
└── crates/hkq-factors/
    ├── Cargo.toml
    └── src/{lib,error,cols,xsec,moments,realized,leadlag,sector,icir,stage1,stage2,panel}.rs
```

## Workspace

```toml
# Cargo.toml
[workspace]
resolver = "2"
members = ["crates/hkq-core", "crates/hkq-data", "crates/hkq-factors", "crates/hkq-nightly"]

[workspace.package]
edition = "2021"
rust-version = "1.83"

[workspace.dependencies]
tokio        = { version = "1.38", features = ["full"] }
tokio-stream = "0.1"
reqwest      = { version = "0.12", default-features = false, features = ["json", "gzip", "rustls-tls"] }
serde        = { version = "1", features = ["derive"] }
serde_json   = "1"
# M1 shipped the parquet substrate; M2 extends the feature set additively for the
# factor engine, per the M1 pin note. Everything still targets polars 0.46.
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

Pin note (unchanged discipline): all expression code targets polars 0.46. The three version-sensitive call sites — `Expr::map`'s `Column` closure, `rolling_quantile`'s argument order, and `ChunkedArray::apply` — are each marked `// PIN:` where they occur; if you float the pin, treat renames as mechanical.

## `hkq-factors`

```toml
# crates/hkq-factors/Cargo.toml
[package]
name = "hkq-factors"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-data = { path = "../hkq-data" }
polars.workspace = true
nalgebra.workspace = true
chrono.workspace = true
serde.workspace = true
thiserror.workspace = true
tracing.workspace = true
```

```rust
// crates/hkq-factors/src/lib.rs
#![forbid(unsafe_code)]
//! The factor engine (report §3): pure panel transforms from the M1 lake schema to
//! Stage-1 sector scores and Stage-2 stock scores.
//!
//! Design invariants:
//! - Statistical math is f64 (polars/nalgebra). No Decimal here by design — the
//!   accounting boundary is hkq-risk (M3).
//! - Every public function is pure w.r.t. its inputs: frames in, frames out.
//!   The ONLY I/O in this crate is `panel::PanelBuilder`, the documented seam onto
//!   `hkq_data::lake::Lake` (M1 hand-off contract).
//! - Missing data policy, uniform everywhere: non-finite → null at factor birth;
//!   nulls pass through winsorize/rank (excluded from the cross-section); nulls
//!   become 0.0 (the cross-sectional neutral) only at the final weighted combine.
//!   This single policy implements the report's X2-disabled mode and the S2/X1
//!   unconfirmed-gap fallbacks (§5) without per-factor special cases.

pub mod cols;
pub mod error;
pub mod icir;
pub mod leadlag;
pub mod moments;
pub mod panel;
pub mod realized;
pub mod sector;
pub mod stage1;
pub mod stage2;
pub mod xsec;

pub use error::FactorError;
pub use icir::FactorWeights;
pub use leadlag::LeadLagGraph;
pub use stage1::OpenContext;
pub use stage2::RegimeGate;
```

```rust
// crates/hkq-factors/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FactorError {
    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),
    #[error(transparent)]
    Data(#[from] hkq_data::error::DataError),
    #[error("input contract violated: {0}")]
    Contract(&'static str),
    #[error("insufficient data: {0}")]
    Insufficient(&'static str),
}
```

```rust
// crates/hkq-factors/src/cols.rs
//! Factor-layer schema contract, extending the M1 ingestion contract
//! (`hkq_data::model::cols`). M3+ crates import THESE constants — never literals.

pub use hkq_data::model::cols as base;

// M1 bars_1m per-bar price/volume column names (see TigerClient::backfill_bars_1m).
pub const O1M: &str = "o";
pub const H1M: &str = "h";
pub const L1M: &str = "l";
pub const C1M: &str = "c";

// Join keys / static reference.
pub const SECTOR: &str = "sector"; // UInt32 in every frame; SectorId(u16) at the boundary
pub const FLOAT_CAP: &str = "float_cap";

// §3.0 return decomposition + daily moments (per stock).
pub const R_ON: &str = "r_on";
pub const R_ID: &str = "r_id";
pub const R_CC: &str = "r_cc";
pub const SIGMA_ON: &str = "sigma_on";
pub const SIGMA_ID: &str = "sigma_id";
pub const SIGMA_CC: &str = "sigma_cc";
pub const ILLIQ: &str = "illiq";
pub const ILLIQ_MED_CS: &str = "illiq_med_cs";
pub const ADV_SHARES: &str = "adv_shares";
pub const RV: &str = "rv";
pub const RS_POS: &str = "rs_pos";
pub const RS_NEG: &str = "rs_neg";
pub const BV: &str = "bv";
pub const JUMP: &str = "jump";
pub const RSJ: &str = "rsj";
pub const N5: &str = "n5";
pub const RV_5D: &str = "rv_5d";
pub const LAV: &str = "lav";

// Intraday bins / seasonality.
pub const BIN5: &str = "bin5"; // absolute 5-minute slot (within-day grouping key)
pub const BOD5: &str = "bod5"; // HKT time-of-day 5-minute slot (seasonality key)
pub const V_BIN: &str = "v_bin";
pub const S_BIN: &str = "s_bin";
pub const S_BAR: &str = "s_bar";
pub const V_BIN_MED: &str = "v_bin_med";
pub const IVU: &str = "ivu";
pub const IVU_TERCILE: &str = "ivu_tercile"; // UInt32 ∈ {0,1,2}; cold start ⇒ 1

// Sector panel (keyed by SECTOR, DATE).
pub const R_ON_1: &str = "r_on_1";
pub const R_ID_1: &str = "r_id_1";
pub const R_LATE: &str = "r_late";
pub const V_LATE: &str = "v_late";
pub const R_LATE_SIGMA: &str = "r_late_sigma_60d";
pub const V_LATE_BAR: &str = "v_late_bar";
pub const N_MEMBERS: &str = "n_members";

// Stage-1 factors & composite.
pub const S1: &str = "s1";
pub const S2: &str = "s2";
pub const S3: &str = "s3";
pub const S4: &str = "s4";
pub const S5: &str = "s5";
pub const S6: &str = "s6";
pub const SIGMA_SCORE: &str = "sigma_score";

// Morning auction aggregates (engine-built at 09:29:30).
pub const GAP_Z: &str = "gap_z";
pub const GAP_Z_SECTOR: &str = "gap_z_sector";
pub const VS_AUCT: &str = "vs_auct";
pub const IEP: &str = "iep";
pub const IEV: &str = "iev";
pub const PREV_CLOSE: &str = "prev_close";
pub const IEV_BAR20: &str = "iev_bar20";

// Linked-market (S6) subcomponents.
pub const AH_DELTA: &str = "ah_delta";
pub const A50_BETA_RET: &str = "a50_beta_ret";
pub const ADR_RESID: &str = "adr_resid_agg";

// Stage-2 factors.
pub const BETA_SECTOR: &str = "beta_sector";
pub const VS_AUCT_I: &str = "vs_auct_i";
pub const IEP_0910: &str = "iep_0910";
pub const IEP_0920: &str = "iep_0920";
pub const VOL_TAU0: &str = "vol_tau0";
pub const VOL_TAU0_MED20: &str = "vol_tau0_med20";
pub const RSJ_1: &str = "rsj_1";
pub const JUMP_1: &str = "jump_1";
pub const EPS_GAP: &str = "eps_gap";
pub const X1: &str = "x1";
pub const X2: &str = "x2";
pub const X3: &str = "x3";
pub const X5: &str = "x5";
pub const X6: &str = "x6";
pub const X6_SPILLOVER: &str = "x6_spillover";
pub const X7: &str = "x7";
pub const SB_Z: &str = "sb_z";
pub const CONNECT_ELIG: &str = "connect_elig";
pub const SCORE: &str = "score";

/// `{factor}_z` — the rank-inverse-normal transform of `factor`.
pub fn z(name: &str) -> String {
    format!("{name}_z")
}
```

```rust
// crates/hkq-factors/src/xsec.rs
//! Cross-sectional math (§3.4): Acklam Φ⁻¹, rank-inverse-normal, winsorization,
//! sequential orthogonalization. All f64; nulls pass through untouched.
use crate::error::FactorError;
use nalgebra::{DMatrix, DVector};
use polars::prelude::*;

/// Acklam's rational approximation to the inverse normal CDF, |ε| < 1.15e−9 —
/// ample for rank arguments (rk − 0.5)/n which are bounded away from {0, 1}.
pub fn norm_ppf(p: f64) -> f64 {
    debug_assert!(p > 0.0 && p < 1.0);
    const A: [f64; 6] = [-3.969683028665376e1,  2.209460984245205e2, -2.759285104469687e2,
                          1.383577518672690e2, -3.066479806614716e1,  2.506628277459239e0];
    const B: [f64; 5] = [-5.447609879822406e1,  1.615858368580409e2, -1.556989798598866e2,
                          6.680131188771972e1, -1.328068155288572e1];
    const C: [f64; 6] = [-7.784894002430293e-3, -3.223964580411365e-1, -2.400758277161838e0,
                         -2.549732539343734e0,   4.374664141464968e0,   2.938163982698783e0];
    const D: [f64; 4] = [ 7.784695709041462e-3,  3.224671290700398e-1,  2.445134137142996e0,
                          3.754408661907416e0];
    let (pl, ph) = (0.02425, 1.0 - 0.02425);
    if p < pl {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= ph {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    }
}

/// Non-finite → null, feature-free: for finite x, x·0 == 0; for ±inf/NaN, x·0 is
/// NaN and the comparison is false; nulls stay null. One policy for all factors.
pub fn finite_or_null(lf: LazyFrame, cols_: &[&str]) -> LazyFrame {
    let exprs: Vec<Expr> = cols_.iter().map(|c| {
        when((col(*c) * lit(0.0)).eq(lit(0.0)))
            .then(col(*c))
            .otherwise(lit(NULL))
            .alias(*c)
    }).collect();
    lf.with_columns(exprs)
}

/// Cross-sectional rank-inverse-normal within `by` (§3.2/§3.4):
/// z̃ = Φ⁻¹((rk − 0.5)/n), written to `{col}_z`. Nulls stay null.
pub fn rank_inv_normal(lf: LazyFrame, cols_: &[&str], by: &str) -> LazyFrame {
    const TMP_U: &str = "__u";
    let mut lf = lf;
    for c in cols_ {
        let rk = col(*c)
            .rank(RankOptions { method: RankMethod::Average, descending: false }, None)
            .cast(DataType::Float64)
            .over([col(by)]);
        let n = col(*c).count().cast(DataType::Float64).over([col(by)]);
        let z_name = crate::cols::z(c);
        lf = lf
            .with_column(((rk - lit(0.5)) / n).alias(TMP_U))
            .with_column(
                // PIN(polars 0.46): Expr::map closure takes/returns Column;
                // ChunkedArray::apply maps Option→Option. Renames are mechanical.
                col(TMP_U)
                    .map(
                        |c| {
                            let ca = c.as_materialized_series().f64()?
                                .apply(|o| o.map(norm_ppf));
                            Ok(Some(ca.into_column()))
                        },
                        GetOutput::from_type(DataType::Float64),
                    )
                    .alias(z_name.as_str()),
            )
            .drop([TMP_U]);
    }
    lf
}

/// Winsorize at the cross-sectional p/(1−p) quantiles within `by` (§3.4, p = 0.01).
/// All-null columns keep their nulls (quantile bounds null ⇒ column untouched).
pub fn winsorize(lf: LazyFrame, cols_: &[&str], by: &str, p: f64) -> LazyFrame {
    let mut lf = lf;
    for c in cols_ {
        let lo = col(*c).quantile(lit(p), QuantileMethod::Linear).over([col(by)]);
        let hi = col(*c).quantile(lit(1.0 - p), QuantileMethod::Linear).over([col(by)]);
        lf = lf.with_column(
            when(lo.clone().is_not_null().and(hi.clone().is_not_null()))
                .then(col(*c).clip(lo, hi))
                .otherwise(col(*c))
                .alias(*c),
        );
    }
    lf
}

/// Sequential residualization in the report's FIXED order (§3.4), per `by` group:
/// each factor keeps only the component orthogonal to an intercept plus all
/// previously admitted factors. Nulls read as 0.0 — the cross-sectional neutral —
/// which is exactly the X2-disabled semantics.
///
/// The cross-section is ~10²–10³ rows once per day; small dense least squares via
/// SVD is the clear choice. (Blueprint fix: solve ‖Xβ − y‖₂ directly — the sketch
/// passed the normal-equations RHS to an SVD of X, which is dimensionally wrong
/// for non-square systems.)
pub fn orthogonalize_daily(
    df: DataFrame, order: &[&str], by: &str,
) -> Result<DataFrame, FactorError> {
    let parts = df.partition_by([by], true)?;
    let mut out: Vec<DataFrame> = Vec::with_capacity(parts.len());
    for mut part in parts {
        let n = part.height();
        if n == 0 {
            out.push(part);
            continue;
        }
        for j in 1..order.len() {
            let yca = part.column(order[j])?.as_materialized_series().f64()?.clone();
            let y: Vec<f64> = (0..n).map(|r| yca.get(r).unwrap_or(0.0)).collect();
            let k = j + 1; // intercept + previously admitted factors
            let mut x = DMatrix::<f64>::zeros(n, k);
            for r in 0..n {
                x[(r, 0)] = 1.0;
            }
            for (cidx, prev) in order[..j].iter().enumerate() {
                let v = part.column(prev)?.as_materialized_series().f64()?.clone();
                for r in 0..n {
                    x[(r, cidx + 1)] = v.get(r).unwrap_or(0.0);
                }
            }
            let yv = DVector::from_vec(y.clone());
            let beta = x.clone().svd(true, true).solve(&yv, 1e-12)
                .unwrap_or_else(|_| DVector::zeros(k));
            let resid: Vec<f64> =
                (0..n).map(|r| y[r] - x.row(r).transpose().dot(&beta)).collect();
            part.replace(order[j], Series::new(order[j].into(), resid))?;
        }
        out.push(part);
    }
    let mut it = out.into_iter();
    let first = it.next().ok_or(FactorError::Insufficient("empty frame in orthogonalize"))?;
    Ok(it.try_fold(first, |acc, d| acc.vstack(&d))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    #[test]
    fn ppf_known_quantiles() {
        assert!(norm_ppf(0.5).abs() < 1e-12);
        assert!((norm_ppf(0.975) - 1.959_963_98).abs() < 1e-6);
        assert!((norm_ppf(0.025) + 1.959_963_98).abs() < 1e-6);
        assert!((norm_ppf(0.999) - 3.090_232_3).abs() < 1e-5);
        assert!((norm_ppf(0.1) + norm_ppf(0.9)).abs() < 1e-9); // symmetry
    }

    #[test]
    fn rank_z_symmetric() {
        let lf = df!("g" => vec![1i32; 5], "f" => vec![10.0, 20.0, 30.0, 40.0, 50.0])
            .unwrap().lazy();
        let out = rank_inv_normal(lf, &["f"], "g").collect().unwrap();
        let z = out.column("f_z").unwrap().as_materialized_series().f64().unwrap().clone();
        assert!((z.get(2).unwrap()).abs() < 1e-12);                       // median → 0
        assert!((z.get(0).unwrap() + z.get(4).unwrap()).abs() < 1e-9);    // symmetry
        assert!((z.get(0).unwrap() - norm_ppf(0.1)).abs() < 1e-9);        // u = (1−.5)/5
    }

    #[test]
    fn winsorize_clips_tails() {
        let lf = df!("g" => vec![1i32; 5], "f" => vec![0.0, 1.0, 2.0, 3.0, 100.0])
            .unwrap().lazy();
        let out = winsorize(lf, &["f"], "g", 0.2).collect().unwrap();
        let f = out.column("f").unwrap().as_materialized_series().f64().unwrap().clone();
        assert!((f.get(4).unwrap() - 22.4).abs() < 1e-9); // q80 linear = 3 + .2·97
        assert!((f.get(0).unwrap() - 0.8).abs() < 1e-9);  // q20 linear
    }

    #[test]
    fn orthogonalization_removes_projection() {
        let a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let b: Vec<f64> = a.iter().map(|x| 0.5 * x + 0.01 * (x * 1.7).sin()).collect();
        let df = df!("__cs" => vec![1i32; 8], "a" => a.clone(), "b" => b).unwrap();
        let out = orthogonalize_daily(df, &["a", "b"], "__cs").unwrap();
        let resid = out.column("b").unwrap().as_materialized_series().f64().unwrap().clone();
        let sum: f64 = resid.into_no_null_iter().sum();
        assert!(sum.abs() < 1e-8); // intercept absorbed
        let resid = out.column("b").unwrap().as_materialized_series().f64().unwrap().clone();
        let dot: f64 = resid.into_no_null_iter().zip(a.iter()).map(|(r, x)| r * x).sum();
        assert!(dot.abs() < 1e-7); // orthogonal to admitted factor
    }
}
```

```rust
// crates/hkq-factors/src/moments.rs
//! The §3.0 nightly moments panel. Input is the M1 daily-bars lake schema joined
//! (left) with `realized::realized_daily` output; output adds the return
//! decomposition, EWMA vols (information through t−1 ONLY), Amihud, ADV, rv_5d, LAV.
//!
//! Consumption convention (document once, rely on everywhere): the morning of
//! trading day t reads ROW t−1 of this panel — r_id_1/rsj_1/etc. are row t−1's
//! values, and σ columns at row t−1 use returns through t−2. Non-anticipative by
//! construction; the engine (M4) slices, it never recomputes.
use crate::cols::{self, base};
use hkq_core::config::FactorCfg;
use polars::prelude::*;

const E: f64 = std::f64::consts::E;

pub(crate) fn ewm_var_opts(cfg: &FactorCfg) -> EWMOptions {
    EWMOptions {
        alpha: 1.0 - 2f64.powf(-1.0 / cfg.ewma_halflife_days),
        adjust: false,
        ignore_nulls: true,
        min_periods: cfg.ewma_min_obs,
        ..Default::default()
    }
}

/// σ_t = √(EWMA of r² through t−1): ewm runs through t, then shifts one row
/// inside the same partition — λσ²_{t−1} + (1−λ)r²_{t−1} exactly (§3.0).
pub fn ewm_sigma_over(cfg: &FactorCfg, key: &str, src: &str, dst: &str) -> Expr {
    col(src).pow(2.0)
        .ewm_mean(ewm_var_opts(cfg))
        .shift(lit(1))
        .sqrt()
        .over([col(key)])
        .alias(dst)
}

pub(crate) fn roll(w: usize) -> RollingOptionsFixedWindow {
    RollingOptionsFixedWindow { window_size: w, min_periods: (w * 2) / 3, ..Default::default() }
}

/// Input contract: [code, date, open, close, adj_close, volume, turnover, rv]
/// (`rv` may be all-null when 1m bars are unavailable — LAV degrades to null).
pub fn enrich_daily_panel(daily: LazyFrame, cfg: &FactorCfg) -> LazyFrame {
    daily
        .sort_by_exprs([col(base::CODE), col(base::DATE)], Default::default())
        // Corporate-action-consistent decomposition: scale the open by the day's
        // adjustment ratio so r_on/r_id live on ONE adjusted series.
        .with_column((col(base::ADJ_CLOSE) / col(base::CLOSE)).alias("__adj_ratio"))
        .with_column((col(base::OPEN) * col("__adj_ratio")).alias("__adj_open"))
        .with_columns([
            (col("__adj_open").log(E)
                - col(base::ADJ_CLOSE).log(E).shift(lit(1)).over([col(base::CODE)]))
            .alias(cols::R_ON),
            (col(base::ADJ_CLOSE).log(E) - col("__adj_open").log(E)).alias(cols::R_ID),
        ])
        .with_column((col(cols::R_ON) + col(cols::R_ID)).alias(cols::R_CC))
        .with_columns([
            ewm_sigma_over(cfg, base::CODE, cols::R_ON, cols::SIGMA_ON),
            ewm_sigma_over(cfg, base::CODE, cols::R_ID, cols::SIGMA_ID),
            ewm_sigma_over(cfg, base::CODE, cols::R_CC, cols::SIGMA_CC),
        ])
        .with_columns([
            when(col(base::TURNOVER).gt(lit(0.0)))
                .then(col(cols::R_CC).abs() / col(base::TURNOVER))
                .otherwise(lit(NULL))
                .rolling_mean(roll(cfg.amihud_window))
                .over([col(base::CODE)])
                .alias(cols::ILLIQ),
            col(base::VOLUME)
                .rolling_mean(roll(cfg.amihud_window))
                .over([col(base::CODE)])
                .alias(cols::ADV_SHARES),
            col(cols::RV)
                .rolling_mean(roll(cfg.rv_days))
                .over([col(base::CODE)])
                .alias(cols::RV_5D),
        ])
        .with_column(
            col(cols::ILLIQ).median().over([col(base::DATE)]).alias(cols::ILLIQ_MED_CS),
        )
        .with_column(
            (col(cols::RV_5D).sqrt()
                * (col(cols::ILLIQ) / col(cols::ILLIQ_MED_CS)).pow(lit(cfg.lav_gamma)))
            .alias(cols::LAV),
        )
        .drop(["__adj_ratio", "__adj_open"])
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    fn cfg() -> FactorCfg {
        FactorCfg {
            ewma_halflife_days: 2.0, ewma_min_obs: 2, amihud_window: 3,
            rv_days: 2, lav_gamma: 0.3, seasonal_vol_days: 3, ivu_tercile_window: 4,
        }
    }

    fn frame(last_close_bump: f64) -> LazyFrame {
        // open_t == prev adj close ⇒ r_on = 0, r_cc = r_id = ln(c_t/c_{t−1}).
        let c1 = 100.0;
        let c2 = c1 * (0.01f64).exp();
        let c3 = c2 * (-0.02f64).exp();
        let c4 = c3 * (0.03f64 + last_close_bump).exp();
        let closes = vec![c1, c2, c3, c4];
        let opens = vec![c1, c1, c2, c3];
        df!(
            "code" => vec![700u32; 4],
            "date" => vec!["2026-06-29", "2026-06-30", "2026-07-01", "2026-07-02"],
            "open" => opens,
            "close" => closes.clone(),
            "adj_close" => closes,
            "volume" => vec![1.0e6; 4],
            "turnover" => vec![1.0e8; 4],
            "rv" => vec![None::<f64>; 4],
        ).unwrap().lazy()
    }

    #[test]
    fn decomposition_identity_and_sigma_math() {
        let out = enrich_daily_panel(frame(0.0), &cfg()).collect().unwrap();
        let r_on = out.column("r_on").unwrap().as_materialized_series().f64().unwrap().clone();
        let r_id = out.column("r_id").unwrap().as_materialized_series().f64().unwrap().clone();
        let r_cc = out.column("r_cc").unwrap().as_materialized_series().f64().unwrap().clone();
        for i in 1..4 {
            assert!(r_on.get(i).unwrap().abs() < 1e-12);
            assert!((r_on.get(i).unwrap() + r_id.get(i).unwrap() - r_cc.get(i).unwrap())
                .abs() < 1e-12);
        }
        // σ_cc at row 3 = √(λ·r₁² + (1−λ)·r₂²), λ = 2^(−1/2); r₁=.01, r₂=−.02.
        let lambda = 2f64.powf(-0.5);
        let expect = (lambda * 0.0001 + (1.0 - lambda) * 0.0004).sqrt();
        let sig = out.column("sigma_cc").unwrap().as_materialized_series().f64().unwrap().clone();
        assert!((sig.get(3).unwrap() - expect).abs() < 1e-9);
    }

    #[test]
    fn sigma_never_sees_today() {
        let a = enrich_daily_panel(frame(0.0), &cfg()).collect().unwrap();
        let b = enrich_daily_panel(frame(0.10), &cfg()).collect().unwrap(); // bump LAST return
        let sa = a.column("sigma_cc").unwrap().as_materialized_series().f64().unwrap().clone();
        let sb = b.column("sigma_cc").unwrap().as_materialized_series().f64().unwrap().clone();
        assert_eq!(sa.get(3), sb.get(3)); // today's σ unchanged by today's return
    }
}
```

```rust
// crates/hkq-factors/src/realized.rs
//! Realized measures from M1 1-minute bars (§3.0) and the seasonal-volume /
//! IVU machinery (§3.3 X4). Binning is PURE INTEGER math on `ts_ms`:
//! HKT = UTC+8 with no DST, so a constant offset is correct — no datetime dtype,
//! no timezone inference, consistent with the M1 determinism decision.
use crate::cols::{self, base};
use hkq_core::config::FactorCfg;
use polars::prelude::*;

const E: f64 = std::f64::consts::E;
pub const MS_PER_DAY: i64 = 86_400_000;
pub const MS_PER_5M: i64 = 300_000;
pub const HKT_UTC_OFFSET_MS: i64 = 8 * 3_600_000;

/// Absolute 5-minute slot (grouping key within a day). The cast truncates toward
/// zero; epoch timestamps are positive ⇒ floor, whether `/` is int or float div.
pub fn bin5_expr() -> Expr {
    (col(base::TS_MS) / lit(MS_PER_5M)).cast(DataType::Int64).alias(cols::BIN5)
}

/// HKT time-of-day 5-minute slot ∈ 0..288 (seasonality key across days).
pub fn bod5_expr() -> Expr {
    (((col(base::TS_MS) + lit(HKT_UTC_OFFSET_MS)) % lit(MS_PER_DAY)) / lit(MS_PER_5M))
        .cast(DataType::Int64)
        .alias(cols::BOD5)
}

/// Per-(code, date): RV, RS±, RSJ, BV, jump from 5-minute log returns (§3.0).
pub fn realized_daily(bars_1m: LazyFrame) -> LazyFrame {
    let five = bars_1m
        .sort_by_exprs([col(base::CODE), col(base::TS_MS)], Default::default())
        .with_column(bin5_expr())
        .group_by([col(base::CODE), col(base::DATE), col(cols::BIN5)])
        .agg([
            col(cols::C1M).last().alias("c5"),
            col(base::VOLUME).sum().alias("v5"),
        ])
        .sort_by_exprs([col(base::CODE), col(base::DATE), col(cols::BIN5)], Default::default())
        .with_column(
            (col("c5").log(E) - col("c5").log(E).shift(lit(1)))
                .over([col(base::CODE), col(base::DATE)])
                .alias("r5"),
        );

    five.group_by([col(base::CODE), col(base::DATE)])
        .agg([
            col("r5").pow(2.0).sum().alias(cols::RV),
            col("r5").pow(2.0).filter(col("r5").gt(lit(0.0))).sum().alias(cols::RS_POS),
            col("r5").pow(2.0).filter(col("r5").lt(lit(0.0))).sum().alias(cols::RS_NEG),
            (col("r5").abs() * col("r5").abs().shift(lit(1))).sum().alias("bv_raw"),
            col("r5").count().alias(cols::N5),
        ])
        .with_column(
            when(col(cols::N5).gt(lit(1)))
                .then(
                    lit(std::f64::consts::FRAC_PI_2)
                        * (col(cols::N5).cast(DataType::Float64)
                            / (col(cols::N5).cast(DataType::Float64) - lit(1.0)))
                        * col("bv_raw"),
                )
                .otherwise(lit(NULL))
                .alias(cols::BV),
        )
        .with_columns([
            (col(cols::RV) - col(cols::BV)).clip_min(lit(0.0)).alias(cols::JUMP),
            when(col(cols::RV).gt(lit(0.0)))
                .then((col(cols::RS_POS) - col(cols::RS_NEG)) / col(cols::RV))
                .otherwise(lit(NULL))
                .alias(cols::RSJ),
        ])
}

/// Trailing same-bin MEDIAN volume profile (X3's denominator; S3's V̄), keyed by
/// HKT time-of-day slot, shifted one day so today never sees itself.
pub fn seasonal_bin_volume(bars_1m: LazyFrame, days: usize) -> LazyFrame {
    bars_1m
        .with_column(bod5_expr())
        .group_by([col(base::CODE), col(base::DATE), col(cols::BOD5)])
        .agg([col(base::VOLUME).sum().alias(cols::V_BIN)])
        .sort_by_exprs([col(base::CODE), col(cols::BOD5), col(base::DATE)], Default::default())
        .with_column(
            col(cols::V_BIN)
                .rolling_median(RollingOptionsFixedWindow {
                    window_size: days,
                    min_periods: (days / 2).max(1),
                    ..Default::default()
                })
                .shift(lit(1))
                .over([col(base::CODE), col(cols::BOD5)])
                .alias(cols::V_BIN_MED),
        )
}

/// IVU (§3.3 X4): realized dispersion of 5-minute volume SHARES against the
/// name's trailing seasonal profile. One row per (code, date).
pub fn ivu_daily(bars_1m: LazyFrame, cfg: &FactorCfg) -> LazyFrame {
    let shares = bars_1m
        .with_column(bod5_expr())
        .group_by([col(base::CODE), col(base::DATE), col(cols::BOD5)])
        .agg([col(base::VOLUME).sum().alias(cols::V_BIN)])
        .with_column(
            (col(cols::V_BIN) / col(cols::V_BIN).sum().over([col(base::CODE), col(base::DATE)]))
                .alias(cols::S_BIN),
        );
    shares
        .sort_by_exprs([col(base::CODE), col(cols::BOD5), col(base::DATE)], Default::default())
        .with_column(
            col(cols::S_BIN)
                .rolling_mean(RollingOptionsFixedWindow {
                    window_size: cfg.seasonal_vol_days,
                    min_periods: (cfg.seasonal_vol_days / 2).max(1),
                    ..Default::default()
                })
                .shift(lit(1))
                .over([col(base::CODE), col(cols::BOD5)])
                .alias(cols::S_BAR),
        )
        .with_column((col(cols::S_BIN) - col(cols::S_BAR)).pow(2.0).alias("__dev2"))
        .group_by([col(base::CODE), col(base::DATE)])
        .agg([col("__dev2").mean().sqrt().alias(cols::IVU)])
}

/// Per-stock trailing-window tercile of IVU: 0 = low, 1 = mid, 2 = high.
/// Ranked against the PRIOR `window` days (shifted — no self-inclusion).
/// Cold start (null quantile bounds) ⇒ tercile 1, the neutral regime.
pub fn ivu_terciles(ivu: LazyFrame, window: usize) -> LazyFrame {
    let ro = RollingOptionsFixedWindow {
        window_size: window,
        min_periods: (window / 3).max(2),
        ..Default::default()
    };
    ivu.sort_by_exprs([col(base::CODE), col(base::DATE)], Default::default())
        .with_columns([
            // PIN(polars 0.46): rolling_quantile(interpol, quantile, options).
            col(cols::IVU)
                .rolling_quantile(QuantileMethod::Linear, 1.0 / 3.0, ro.clone())
                .shift(lit(1)).over([col(base::CODE)]).alias("__q1"),
            col(cols::IVU)
                .rolling_quantile(QuantileMethod::Linear, 2.0 / 3.0, ro)
                .shift(lit(1)).over([col(base::CODE)]).alias("__q2"),
        ])
        .with_column(
            when(col(cols::IVU).lt_eq(col("__q1"))).then(lit(0u32))
                .when(col(cols::IVU).gt(col("__q2"))).then(lit(2u32))
                .otherwise(lit(1u32))
                .alias(cols::IVU_TERCILE),
        )
        .drop(["__q1", "__q2"])
}

/// Per-(code, date) 14:00–15:30 window return and volume — S3's raw material
/// (§3.2). Slots [168, 186) in HKT time-of-day 5-minute units.
pub fn late_window_member(bars_1m: LazyFrame) -> LazyFrame {
    let lo = (14 * 60) / 5; // 168
    let hi = (15 * 60 + 30) / 5; // 186, exclusive
    bars_1m
        .sort_by_exprs([col(base::CODE), col(base::TS_MS)], Default::default())
        .with_column(bod5_expr())
        .filter(col(cols::BOD5).gt_eq(lit(lo as i64)).and(col(cols::BOD5).lt(lit(hi as i64))))
        .group_by([col(base::CODE), col(base::DATE)])
        .agg([
            (col(cols::C1M).last().log(E) - col(cols::C1M).first().log(E)).alias(cols::R_LATE),
            col(base::VOLUME).sum().alias(cols::V_LATE),
        ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    use hkq_core::session::hk;
    use polars::df;

    fn ts(h: u32, m: u32) -> i64 {
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        hk(d, NaiveTime::from_hms_opt(h, m, 0).unwrap()).timestamp_millis()
    }

    #[test]
    fn bod5_slots_match_hkt_clock() {
        let lf = df!(
            "code" => vec![700u32; 3],
            "date" => vec!["2026-07-03"; 3],
            "ts_ms" => vec![ts(9, 30), ts(13, 0), ts(15, 55)],
        ).unwrap().lazy();
        let out = lf.with_column(bod5_expr()).collect().unwrap();
        let b = out.column("bod5").unwrap().as_materialized_series().i64().unwrap().clone();
        assert_eq!(b.get(0), Some(114)); // 09:30
        assert_eq!(b.get(1), Some(156)); // 13:00
        assert_eq!(b.get(2), Some(191)); // 15:55
    }

    #[test]
    fn realized_measures_hand_computed() {
        // One bar per 5m slot ⇒ c5 = close. r5 = [.01, −.02, .03] on 4 closes.
        let c1 = 100.0;
        let c2 = c1 * (0.01f64).exp();
        let c3 = c2 * (-0.02f64).exp();
        let c4 = c3 * (0.03f64).exp();
        let lf = df!(
            "code" => vec![700u32; 4],
            "date" => vec!["2026-07-03"; 4],
            "ts_ms" => vec![ts(9, 30), ts(9, 35), ts(9, 40), ts(9, 45)],
            "c" => vec![c1, c2, c3, c4],
            "volume" => vec![10.0, 10.0, 10.0, 10.0],
        ).unwrap().lazy();
        let out = realized_daily(lf).collect().unwrap();
        assert_eq!(out.height(), 1);
        let g = |n: &str| out.column(n).unwrap().as_materialized_series()
            .f64().unwrap().get(0).unwrap();
        let rv = 0.0001 + 0.0004 + 0.0009;
        assert!((g("rv") - rv).abs() < 1e-12);
        assert!((g("rs_pos") - 0.0010).abs() < 1e-12);
        assert!((g("rs_neg") - 0.0004).abs() < 1e-12);
        assert!((g("rsj") - (0.0010 - 0.0004) / rv).abs() < 1e-12);
        let bv = std::f64::consts::FRAC_PI_2 * (3.0 / 2.0)
            * (0.01 * 0.02 + 0.02 * 0.03);
        assert!((g("bv") - bv).abs() < 1e-12);
        assert!((g("jump") - (rv - bv).max(0.0)).abs() < 1e-12);
    }

    #[test]
    fn ivu_zero_when_profile_repeats() {
        // Identical intraday shape 4 days running ⇒ day-4 deviation from the
        // trailing mean profile is exactly zero.
        let days = ["2026-06-30", "2026-07-01", "2026-07-02", "2026-07-03"];
        let (mut code, mut date, mut tsv, mut vol) = (vec![], vec![], vec![], vec![]);
        for (i, d) in days.iter().enumerate() {
            for (slot, v) in [(9u32, 30u32, 60.0), (9, 35, 30.0), (9, 40, 10.0)]
                .map(|(h, m, v)| ((h, m), v))
            {
                code.push(700u32);
                date.push(*d);
                tsv.push(ts(slot.0, slot.1) + (i as i64) * MS_PER_DAY);
                vol.push(v);
            }
        }
        let lf = df!("code" => code, "date" => date, "ts_ms" => tsv, "volume" => vol)
            .unwrap().lazy();
        let cfg = FactorCfg {
            ewma_halflife_days: 21.0, ewma_min_obs: 2, amihud_window: 3, rv_days: 2,
            lav_gamma: 0.3, seasonal_vol_days: 2, ivu_tercile_window: 4,
        };
        let out = ivu_daily(lf, &cfg).collect().unwrap()
            .sort(["date"], Default::default()).unwrap();
        let ivu = out.column("ivu").unwrap().as_materialized_series().f64().unwrap().clone();
        assert!(ivu.get(3).unwrap().abs() < 1e-15);
    }

    #[test]
    fn ivu_tercile_ranks_against_prior_window() {
        let n = 10;
        let lf = df!(
            "code" => vec![700u32; n],
            "date" => (0..n).map(|i| format!("2026-06-{:02}", i + 1)).collect::<Vec<_>>(),
            "ivu" => (0..n).map(|i| (i + 1) as f64).collect::<Vec<f64>>(),
        ).unwrap().lazy();
        let out = ivu_terciles(lf, 4).collect().unwrap();
        let t = out.column("ivu_tercile").unwrap().as_materialized_series()
            .u32().unwrap().clone();
        assert_eq!(t.get(0), Some(1));       // cold start ⇒ neutral regime
        assert_eq!(t.get(n - 1), Some(2));   // monotone series ⇒ top tercile
    }
}
```

```rust
// crates/hkq-factors/src/leadlag.rs
//! §3.2 S4 / §3.3 X6: lagged cross-correlation network on residual returns —
//! Schäfer–Strimmer shrink toward zero, Fisher-z p-values, Benjamini–Hochberg FDR.
//! Trusted at the DAILY horizon only (report §2.4: tick lead–lag is arbitraged).
//!
//! Residualization note: `demean_rows` removes the per-date cross-sectional mean —
//! the equal-weight market component. The report's β-hedged (HSI) residual is a
//! strict upgrade whose β estimation job belongs to hkq-validate; swap it in here
//! when that lands. The graph API is unchanged either way.
use crate::error::FactorError;
use nalgebra::DMatrix;
use std::collections::{BTreeMap, BTreeSet, HashSet};

pub struct LeadLagGraph {
    pub k: usize,
    /// a[(lead, lag)] = shrunk, FDR-surviving signed correlation; 0 otherwise.
    pub a: DMatrix<f64>,
}

impl LeadLagGraph {
    pub fn zero(k: usize) -> Self {
        Self { k, a: DMatrix::zeros(k, k) }
    }

    /// out[b] = Σ_a A[(a,b)]·x[a] over admissible sources (S4: all sectors;
    /// X6: top-quintile float-cap leaders only).
    pub fn propagate_masked(&self, x: &[f64], source_mask: &[bool]) -> Vec<f64> {
        let mut out = vec![0.0; self.k];
        for a in 0..self.k {
            if !source_mask.get(a).copied().unwrap_or(false) {
                continue;
            }
            let xa = x.get(a).copied().unwrap_or(0.0);
            if xa == 0.0 || !xa.is_finite() {
                continue;
            }
            for b in 0..self.k {
                if a != b {
                    out[b] += self.a[(a, b)] * xa;
                }
            }
        }
        out
    }

    pub fn propagate(&self, x: &[f64]) -> Vec<f64> {
        self.propagate_masked(x, &vec![true; self.k])
    }
}

/// resid: T×K matrix, columns aligned to the caller's label vector.
/// Λ_ab = Corr(x_a[..T−1], x_b[1..]) — column a LEADS column b.
pub fn lagged_corr_fdr(resid: &DMatrix<f64>, q: f64) -> LeadLagGraph {
    let (t, k) = resid.shape();
    if k < 2 || t < 12 {
        return LeadLagGraph::zero(k);
    }
    let n = t - 1;
    let std = |v: &[f64]| -> Vec<f64> {
        let m = v.iter().sum::<f64>() / v.len() as f64;
        let s = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() - 1) as f64).sqrt();
        let s = if s.is_finite() && s > 1e-12 { s } else { 1.0 };
        v.iter().map(|x| (x - m) / s).collect()
    };
    let cols_: Vec<Vec<f64>> = (0..k).map(|j| resid.column(j).iter().copied().collect()).collect();
    let lead: Vec<Vec<f64>> = cols_.iter().map(|c| std(&c[..n])).collect();
    let lagg: Vec<Vec<f64>> = cols_.iter().map(|c| std(&c[1..])).collect();

    let mut lambda = DMatrix::<f64>::zeros(k, k);
    for a in 0..k {
        for b in 0..k {
            if a != b {
                lambda[(a, b)] = lead[a].iter().zip(&lagg[b]).map(|(x, y)| x * y).sum::<f64>()
                    / (n as f64 - 1.0);
            }
        }
    }
    // Schäfer–Strimmer shrink toward zero: λ̂* = Σ V̂ar(r_ab) / Σ r_ab², clamped.
    let var_r = 1.0 / n as f64;
    let sum_r2: f64 = lambda.iter().map(|r| r * r).sum();
    let shrink = ((k * (k - 1)) as f64 * var_r / sum_r2.max(1e-12)).clamp(0.0, 1.0);
    lambda *= 1.0 - shrink;

    // Fisher-z p-values + BH at level q.
    let mut ps: Vec<(usize, usize, f64)> = Vec::with_capacity(k * (k - 1));
    for a in 0..k {
        for b in 0..k {
            if a != b {
                let r = lambda[(a, b)].clamp(-0.999_999, 0.999_999);
                let z = 0.5 * ((1.0 + r) / (1.0 - r)).ln() * (n as f64 - 3.0).sqrt();
                let p = 2.0 * (1.0 - phi(z.abs()));
                ps.push((a, b, p));
            }
        }
    }
    ps.sort_by(|x, y| x.2.total_cmp(&y.2));
    let m = ps.len() as f64;
    let mut cutoff = 0usize;
    for (i, (_, _, p)) in ps.iter().enumerate() {
        if *p <= q * (i as f64 + 1.0) / m {
            cutoff = i + 1;
        }
    }
    let keep: HashSet<(usize, usize)> = ps[..cutoff].iter().map(|(a, b, _)| (*a, *b)).collect();
    for a in 0..k {
        for b in 0..k {
            if a != b && !keep.contains(&(a, b)) {
                lambda[(a, b)] = 0.0;
            }
        }
    }
    LeadLagGraph { k, a: lambda }
}

/// Long panel [date, key, value] → (dates asc, keys asc, T×K matrix). Missing
/// (date, key) cells fill with 0.0 — the cross-sectional neutral.
pub fn panel_to_matrix(
    df: &polars::prelude::DataFrame, date_col: &str, key_col: &str, val_col: &str,
) -> Result<(Vec<String>, Vec<u32>, DMatrix<f64>), FactorError> {
    use polars::prelude::*;
    let dates_ca = df.column(date_col)?.as_materialized_series().str()?.clone();
    let keys_ca = df.column(key_col)?.as_materialized_series().u32()?.clone();
    let vals_ca = df.column(val_col)?.as_materialized_series().f64()?.clone();

    let mut dset: BTreeSet<String> = BTreeSet::new();
    let mut kset: BTreeSet<u32> = BTreeSet::new();
    for i in 0..df.height() {
        if let Some(d) = dates_ca.get(i) {
            dset.insert(d.to_string());
        }
        if let Some(k) = keys_ca.get(i) {
            kset.insert(k);
        }
    }
    let dates: Vec<String> = dset.into_iter().collect();
    let keys: Vec<u32> = kset.into_iter().collect();
    let didx: BTreeMap<&str, usize> =
        dates.iter().enumerate().map(|(i, d)| (d.as_str(), i)).collect();
    let kidx: BTreeMap<u32, usize> = keys.iter().enumerate().map(|(i, k)| (*k, i)).collect();

    let mut m = DMatrix::<f64>::zeros(dates.len(), keys.len());
    for i in 0..df.height() {
        let (Some(d), Some(k), Some(v)) = (dates_ca.get(i), keys_ca.get(i), vals_ca.get(i))
        else { continue };
        if v.is_finite() {
            m[(didx[d], kidx[&k])] = v;
        }
    }
    Ok((dates, keys, m))
}

/// Remove the per-date cross-sectional mean (row-wise demean).
pub fn demean_rows(m: &mut DMatrix<f64>) {
    let k = m.ncols();
    if k == 0 {
        return;
    }
    for r in 0..m.nrows() {
        let mean = m.row(r).iter().sum::<f64>() / k as f64;
        for c in 0..k {
            m[(r, c)] -= mean;
        }
    }
}

fn phi(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Abramowitz–Stegun 7.1.26, |ε| < 1.5e−7 — ample for FDR gating.
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

    fn xorshift(s: &mut u64) -> f64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        ((*s >> 11) as f64 / (1u64 << 53) as f64) - 0.5
    }

    #[test]
    fn detects_directed_edge_and_prunes_reverse() {
        let t = 400;
        let mut s = 0x9E3779B97F4A7C15u64;
        let x0: Vec<f64> = (0..t).map(|_| xorshift(&mut s)).collect();
        let mut x1 = vec![0.0; t];
        for i in 1..t {
            x1[i] = 0.9 * x0[i - 1] + 0.5 * xorshift(&mut s);
        }
        let x2: Vec<f64> = (0..t).map(|_| xorshift(&mut s)).collect();
        let mut m = DMatrix::<f64>::zeros(t, 3);
        for i in 0..t {
            m[(i, 0)] = x0[i];
            m[(i, 1)] = x1[i];
            m[(i, 2)] = x2[i];
        }
        let g = lagged_corr_fdr(&m, 0.10);
        assert!(g.a[(0, 1)] > 0.2, "true lead edge must survive: {}", g.a[(0, 1)]);
        assert_eq!(g.a[(1, 0)], 0.0, "reverse edge must be pruned");
        let s4 = g.propagate(&[1.0, 0.0, 0.0]);
        assert!(s4[1] > 0.0 && s4[0] == 0.0);
    }

    #[test]
    fn degenerate_inputs_yield_zero_graph() {
        let g = lagged_corr_fdr(&DMatrix::zeros(5, 3), 0.10);
        assert!(g.a.iter().all(|v| *v == 0.0));
    }
}
```

```rust
// crates/hkq-factors/src/sector.rs
//! Member → sector aggregation (§3.2): capped float-cap weights (15% water-filling
//! fixpoint), weighted panel aggregation, sector σ's, S3 late-flow, S5 flow z.
//! The water-filling step is not expressible as a polars expression; the daily
//! cross-section is tiny, so this is eager Rust by choice — clarity over cleverness.
use crate::cols::{self, base};
use crate::error::FactorError;
use crate::moments;
use hkq_core::config::FactorCfg;
use polars::prelude::*;

const E: f64 = std::f64::consts::E;

/// Float-cap weights, capped and renormalized (§3.2). If the cap is infeasible
/// (n·cap ≤ 1, e.g. a sector with < 7 members at 15%) the least-concentrated
/// admissible answer is equal weights — the blueprint sketch ignored this edge.
pub fn capped_weights(float_cap: &[f64], cap: f64) -> Vec<f64> {
    let n = float_cap.len();
    if n == 0 {
        return vec![];
    }
    let equal = vec![1.0 / n as f64; n];
    let s: f64 = float_cap.iter().map(|x| x.max(0.0)).sum();
    if s <= 0.0 || (n as f64) * cap <= 1.0 {
        return equal;
    }
    let mut w: Vec<f64> = float_cap.iter().map(|x| x.max(0.0) / s).collect();
    for _ in 0..32 {
        let over: f64 = w.iter().filter(|x| **x > cap).map(|x| x - cap).sum();
        if over < 1e-12 {
            break;
        }
        let under_sum: f64 = w.iter().filter(|x| **x < cap).copied().sum();
        if under_sum <= 0.0 {
            return equal;
        }
        w = w.iter()
            .map(|x| if *x > cap { cap } else { x + over * (x / under_sum) })
            .collect();
    }
    w
}

/// Capped-weight aggregation of `value_cols` per (date, sector): weighted mean
/// with weights renormalized over the non-null members of each column.
/// Input: [date, sector, float_cap, value_cols…]. Output: [date, sector,
/// n_members, value_cols…] (same names).
pub fn weighted_sector_agg(
    members: DataFrame, value_cols: &[&str], cap: f64,
) -> Result<DataFrame, FactorError> {
    // Defensive dtype normalization at the boundary.
    let mut casts: Vec<Expr> = vec![
        col(cols::SECTOR).cast(DataType::UInt32),
        col(cols::FLOAT_CAP).cast(DataType::Float64),
    ];
    casts.extend(value_cols.iter().map(|c| col(*c).cast(DataType::Float64)));
    let members = members.lazy().with_columns(casts).collect()?;

    let parts = members.partition_by([base::DATE, cols::SECTOR], true)?;
    let mut dates: Vec<String> = Vec::with_capacity(parts.len());
    let mut sectors: Vec<u32> = Vec::with_capacity(parts.len());
    let mut counts: Vec<u32> = Vec::with_capacity(parts.len());
    let mut agg: Vec<Vec<Option<f64>>> = vec![Vec::with_capacity(parts.len()); value_cols.len()];

    for part in parts {
        let h = part.height();
        if h == 0 {
            continue;
        }
        let date = part.column(base::DATE)?.as_materialized_series().str()?
            .get(0).unwrap_or_default().to_string();
        let sector = part.column(cols::SECTOR)?.as_materialized_series().u32()?
            .get(0).ok_or(FactorError::Contract("null sector id"))?;
        let fc_ca = part.column(cols::FLOAT_CAP)?.as_materialized_series().f64()?.clone();
        let fc: Vec<f64> = (0..h).map(|i| fc_ca.get(i).unwrap_or(0.0)).collect();
        let w = capped_weights(&fc, cap);

        dates.push(date);
        sectors.push(sector);
        counts.push(h as u32);
        for (ci, vc) in value_cols.iter().enumerate() {
            let v = part.column(vc)?.as_materialized_series().f64()?.clone();
            let (mut num, mut den) = (0.0f64, 0.0f64);
            for i in 0..h {
                if let Some(x) = v.get(i) {
                    if x.is_finite() {
                        num += w[i] * x;
                        den += w[i];
                    }
                }
            }
            agg[ci].push((den > 0.0).then_some(num / den));
        }
    }

    let mut out: Vec<Column> = vec![
        Series::new(base::DATE.into(), dates).into_column(),
        Series::new(cols::SECTOR.into(), sectors).into_column(),
        Series::new(cols::N_MEMBERS.into(), counts).into_column(),
    ];
    for (name, vals) in value_cols.iter().zip(agg) {
        out.push(Series::new((*name).into(), vals).into_column());
    }
    Ok(DataFrame::new(out)?)
}

/// Sector daily return panel: capped-weight aggregate of member r_on/r_id.
/// Input: enriched member panel joined with [code → sector, float_cap].
pub fn aggregate_sector_returns(
    members: DataFrame, cap: f64,
) -> Result<DataFrame, FactorError> {
    weighted_sector_agg(members, &[cols::R_ON, cols::R_ID], cap)
}

/// Sector nightly block for Stage 1: EWMA sector σ's + S3 late-flow, one row per
/// (sector, date). The engine slices row t−1 via `nightly_snapshot_for`.
pub fn sector_nightly_panel(
    sector_daily: LazyFrame, // [date, sector, r_on, r_id]
    sector_late: LazyFrame,  // [date, sector, r_late, v_late]
    cfg: &FactorCfg,
) -> LazyFrame {
    sector_daily
        .join(
            sector_late,
            [col(cols::SECTOR), col(base::DATE)],
            [col(cols::SECTOR), col(base::DATE)],
            JoinArgs::new(JoinType::Left),
        )
        .sort_by_exprs([col(cols::SECTOR), col(base::DATE)], Default::default())
        .with_columns([
            moments::ewm_sigma_over(cfg, cols::SECTOR, cols::R_ON, cols::SIGMA_ON),
            moments::ewm_sigma_over(cfg, cols::SECTOR, cols::R_ID, cols::SIGMA_ID),
        ])
        .with_columns([
            col(cols::R_LATE)
                .rolling_std(moments::roll(60))
                .shift(lit(1))
                .over([col(cols::SECTOR)])
                .alias(cols::R_LATE_SIGMA),
            col(cols::V_LATE)
                .rolling_mean(moments::roll(20))
                .shift(lit(1))
                .over([col(cols::SECTOR)])
                .alias(cols::V_LATE_BAR),
        ])
        .with_column(
            // S3 (§3.2): standardized late return × (1 + ln V/V̄)₊ — thin late
            // sessions mute it; the whole multiplier is floored at zero.
            when(col(cols::R_LATE_SIGMA).gt(lit(0.0)).and(col(cols::V_LATE_BAR).gt(lit(0.0))))
                .then(
                    (col(cols::R_LATE) / col(cols::R_LATE_SIGMA))
                        * (lit(1.0) + (col(cols::V_LATE) / col(cols::V_LATE_BAR)).log(E))
                            .clip_min(lit(0.0)),
                )
                .otherwise(lit(NULL))
                .alias(cols::S3),
        )
}

/// Slice the sector-nightly row for `prev_day` and rename to the OpenContext
/// contract: r_id_1 / r_on_1 are t−1's returns (§3.2 S1).
pub fn nightly_snapshot_for(panel: LazyFrame, prev_day: chrono::NaiveDate) -> LazyFrame {
    panel
        .filter(col(base::DATE).eq(lit(prev_day.to_string())))
        .with_columns([
            col(cols::R_ID).alias(cols::R_ID_1),
            col(cols::R_ON).alias(cols::R_ON_1),
        ])
}

/// S5 (§3.2): per-stock southbound net-buy z over `window` days (shifted — the
/// z is of t−1's print against its OWN trailing history), sector-aggregated.
/// flows: [date, code, net_buy]; member_map: [code, sector, float_cap].
pub fn s5_sector_flow(
    flows: LazyFrame, member_map: DataFrame, window: usize, cap: f64,
) -> Result<DataFrame, FactorError> {
    let ro = moments::roll(window);
    let mu = col(base::NET_BUY).rolling_mean(ro.clone()).shift(lit(1)).over([col(base::CODE)]);
    let sd = col(base::NET_BUY).rolling_std(ro).shift(lit(1)).over([col(base::CODE)]);
    let stock_z = flows
        .sort_by_exprs([col(base::CODE), col(base::DATE)], Default::default())
        .with_column(
            when(sd.clone().gt(lit(0.0)))
                .then((col(base::NET_BUY) - mu) / sd)
                .otherwise(lit(NULL))
                .alias(cols::S5),
        )
        .join(
            member_map.lazy(),
            [col(base::CODE)],
            [col(base::CODE)],
            JoinArgs::new(JoinType::Inner),
        )
        .collect()?;
    weighted_sector_agg(stock_z, &[cols::S5], cap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    #[test]
    fn capped_weights_waterfill_and_edges() {
        let w = capped_weights(&[90.0, 10.0], 0.5);
        assert!((w[0] - 0.5).abs() < 1e-9 && (w[1] - 0.5).abs() < 1e-9);
        let w = capped_weights(&[50.0, 30.0, 20.0], 0.4);
        assert!((w.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert!(w.iter().all(|x| *x <= 0.4 + 1e-9));
        // Infeasible cap (n·cap ≤ 1) ⇒ equal weights.
        let w = capped_weights(&[90.0, 10.0], 0.15);
        assert_eq!(w, vec![0.5, 0.5]);
    }

    #[test]
    fn weighted_agg_renormalizes_over_missing() {
        let members = df!(
            "date" => vec!["2026-07-03"; 3],
            "sector" => vec![7u32; 3],
            "float_cap" => vec![50.0, 30.0, 20.0],
            "r_id" => vec![Some(0.01), Some(0.03), None],
        ).unwrap();
        let out = weighted_sector_agg(members, &["r_id"], 0.9).unwrap();
        assert_eq!(out.height(), 1);
        let v = out.column("r_id").unwrap().as_materialized_series()
            .f64().unwrap().get(0).unwrap();
        // weights .5/.3 renormalized over the two non-null members
        let expect = (0.5 * 0.01 + 0.3 * 0.03) / 0.8;
        assert!((v - expect).abs() < 1e-12);
    }

    #[test]
    fn s3_muted_on_thin_late_volume() {
        let daily = df!(
            "date" => (1..=64).map(|i| format!("2026-{:02}-{:02}", 1 + i / 28, 1 + i % 28))
                .collect::<Vec<_>>(),
            "sector" => vec![1u32; 64],
            "r_on" => vec![0.0f64; 64],
            "r_id" => vec![0.001f64; 64],
        ).unwrap().lazy();
        let mut r_late = vec![0.001f64; 64];
        r_late[63] = 0.01; // strong late move on the final day…
        let mut v_late = vec![100.0f64; 64];
        v_late[63] = 1.0; // …on very thin volume: multiplier (1+ln(v/v̄))₊ → 0
        let late = df!(
            "date" => (1..=64).map(|i| format!("2026-{:02}-{:02}", 1 + i / 28, 1 + i % 28))
                .collect::<Vec<_>>(),
            "sector" => vec![1u32; 64],
            "r_late" => r_late,
            "v_late" => v_late,
        ).unwrap().lazy();
        let cfg = FactorCfg {
            ewma_halflife_days: 21.0, ewma_min_obs: 5, amihud_window: 60, rv_days: 5,
            lav_gamma: 0.3, seasonal_vol_days: 20, ivu_tercile_window: 60,
        };
        let out = sector_nightly_panel(daily, late, &cfg).collect().unwrap()
            .sort(["date"], Default::default()).unwrap();
        let s3 = out.column("s3").unwrap().as_materialized_series().f64().unwrap().clone();
        assert_eq!(s3.get(63), Some(0.0)); // floored at zero, not negative
    }
}
```

```rust
// crates/hkq-factors/src/icir.rs
//! ICIR factor weights (§3.2/§3.4): ω_f ∝ max(ICIR_f, 0) + δ, renormalized —
//! plus the daily Spearman rank-IC primitive the nightly attribution job appends.
//! NOTE (blueprint deviation, deliberate): the blueprint sketches this type in
//! hkq-signal while referencing it from hkq-factors as `crate::icir`. Both stages'
//! combiners consume it, so the factor crate OWNS it; hkq-signal (M3) re-exports.
use crate::error::FactorError;
use polars::prelude::*;
use std::collections::BTreeMap;

const MIN_OBS: usize = 20;

#[derive(Debug, Clone)]
pub struct FactorWeights(BTreeMap<String, f64>);

impl FactorWeights {
    pub fn get(&self, f: &str) -> f64 {
        *self.0.get(f).unwrap_or(&0.0)
    }

    /// Equal weights — the documented cold-start prior before an IC history exists.
    pub fn equal(factors: &[&str]) -> Self {
        let w = 1.0 / factors.len().max(1) as f64;
        Self(factors.iter().map(|f| (f.to_string(), w)).collect())
    }

    /// Fixed weights from raw non-negative loadings (config-pinned or test use);
    /// normalized, negatives clamped to zero. Zero-sum falls back to equal.
    pub fn from_raw<I: IntoIterator<Item = (String, f64)>>(pairs: I) -> Self {
        let m: BTreeMap<String, f64> =
            pairs.into_iter().map(|(k, v)| (k, v.max(0.0))).collect();
        let z: f64 = m.values().sum();
        if z <= 0.0 {
            let keys: Vec<&str> = m.keys().map(String::as_str).collect();
            return Self::equal(&keys);
        }
        Self(m.into_iter().map(|(k, v)| (k, v / z)).collect())
    }

    /// ω_f ∝ max(ICIR_f, 0) + δ over the trailing `window` of daily rank ICs.
    /// `ic_panel` carries one column per factor named `ic_{f}` (nightly append).
    /// Fewer than MIN_OBS observations ⇒ that factor's ICIR reads 0 (δ floor only).
    pub fn from_ic_panel(
        ic_panel: &DataFrame, factors: &[&str], window: usize, delta: f64,
    ) -> Result<Self, FactorError> {
        let mut raw = BTreeMap::new();
        for f in factors {
            let name = format!("ic_{f}");
            let s = ic_panel.column(&name)?.as_materialized_series().f64()?.clone();
            let n = s.len();
            let tail: Vec<f64> = (n.saturating_sub(window)..n)
                .filter_map(|i| s.get(i))
                .filter(|v| v.is_finite())
                .collect();
            let icir = if tail.len() >= MIN_OBS {
                let (m, sd) = mean_sd(&tail);
                (m / sd.max(1e-9)).max(0.0)
            } else {
                0.0
            };
            raw.insert(f.to_string(), icir + delta);
        }
        let z: f64 = raw.values().sum();
        if z <= 0.0 {
            return Ok(Self::equal(factors));
        }
        Ok(Self(raw.into_iter().map(|(k, v)| (k, v / z)).collect()))
    }
}

/// Daily rank IC: Spearman correlation of predicted vs realized, pairwise over
/// finite pairs. None if fewer than 3 valid pairs or a degenerate marginal.
pub fn spearman_ic(pred: &[f64], realized: &[f64]) -> Option<f64> {
    let pairs: Vec<(f64, f64)> = pred.iter().zip(realized)
        .filter(|(p, r)| p.is_finite() && r.is_finite())
        .map(|(p, r)| (*p, *r))
        .collect();
    if pairs.len() < 3 {
        return None;
    }
    let (xs, ys): (Vec<f64>, Vec<f64>) = pairs.into_iter().unzip();
    pearson(&average_ranks(&xs), &average_ranks(&ys))
}

fn average_ranks(v: &[f64]) -> Vec<f64> {
    let n = v.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| v[a].total_cmp(&v[b]));
    let mut ranks = vec![0.0; n];
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j + 1 < n && v[idx[j + 1]] == v[idx[i]] {
            j += 1;
        }
        let avg = (i + j) as f64 / 2.0 + 1.0;
        for &r in &idx[i..=j] {
            ranks[r] = avg;
        }
        i = j + 1;
    }
    ranks
}

fn pearson(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let (mut sxx, mut syy, mut sxy) = (0.0, 0.0, 0.0);
    for (a, b) in x.iter().zip(y) {
        sxx += (a - mx) * (a - mx);
        syy += (b - my) * (b - my);
        sxy += (a - mx) * (b - my);
    }
    (sxx > 0.0 && syy > 0.0).then(|| sxy / (sxx * syy).sqrt())
}

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let m = v.iter().sum::<f64>() / v.len().max(1) as f64;
    let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>()
        / (v.len().saturating_sub(1)).max(1) as f64)
        .sqrt();
    (m, sd)
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    #[test]
    fn spearman_basics() {
        assert!((spearman_ic(&[1.0, 2.0, 3.0], &[10.0, 20.0, 30.0]).unwrap() - 1.0).abs() < 1e-12);
        assert!((spearman_ic(&[1.0, 2.0, 3.0], &[3.0, 2.0, 1.0]).unwrap() + 1.0).abs() < 1e-12);
        assert!(spearman_ic(&[1.0, f64::NAN], &[1.0, 2.0]).is_none()); // < 3 valid pairs
    }

    #[test]
    fn dead_factor_gets_delta_floor_only() {
        let n = 30;
        let ic_a: Vec<f64> = (0..n).map(|i| 0.05 + 0.001 * ((i % 3) as f64)).collect();
        let ic_b: Vec<f64> = (0..n).map(|i| -0.05 - 0.001 * ((i % 3) as f64)).collect();
        let panel = df!("ic_a" => ic_a, "ic_b" => ic_b).unwrap();
        let w = FactorWeights::from_ic_panel(&panel, &["a", "b"], 30, 0.1).unwrap();
        assert!((w.get("a") + w.get("b") - 1.0).abs() < 1e-12);
        assert!(w.get("a") > 5.0 * w.get("b")); // negative-IC factor → δ share only
    }

    #[test]
    fn cold_start_equal() {
        let w = FactorWeights::equal(&["s1", "s2"]);
        assert!((w.get("s1") - 0.5).abs() < 1e-12);
        assert_eq!(w.get("unknown"), 0.0);
    }
}
```

```rust
// crates/hkq-factors/src/stage1.rs
//! Stage 1 (§3.2): the 09:29:30 sector composite and top-K selection with the
//! Σ_min cash-day gate. Building `OpenContext` is the ENGINE's job (M4) — this
//! module is pure functions of the frozen snapshot.
use crate::cols::{self, base as _base};
use crate::error::FactorError;
use crate::icir::FactorWeights;
use crate::xsec;
use hkq_core::config::Stage1Cfg;
use hkq_core::ids::SectorId;
use polars::prelude::*;

/// Frozen 09:29:30 snapshot (§6 runbook). Schemas (all keyed by `sector`, UInt32):
/// - sector_nightly: [sector, r_id_1, r_on_1, sigma_id, sigma_on, s3, s4, s5]
///   (s3/s4/s5 may be null — degraded feeds are neutral, never blocking)
/// - sector_auction: [sector, gap_z, vs_auct]  (may be EMPTY: no POS feed)
/// - sector_linked:  [sector, ah_delta, a50_beta_ret, adr_resid_agg] (nullable)
pub struct OpenContext {
    pub date: chrono::NaiveDate,
    pub sector_nightly: DataFrame,
    pub sector_auction: DataFrame,
    pub sector_linked: DataFrame,
}

/// S1 (§3.2): continuation on t−1 TRADED returns, fade on overnight-propagated
/// moves — the spillover-asymmetry factor (report consequence №3).
pub fn s1(cfg: &Stage1Cfg) -> Expr {
    (lit(cfg.theta1) * col(cols::R_ID_1) / col(cols::SIGMA_ID))
        - (lit(cfg.theta2) * col(cols::R_ON_1) / col(cols::SIGMA_ON))
}

/// S2 (§3.2): gap on CONFIRMED auction volume is information; the unconditional
/// −η tilt fades every gap. A null VS (no auction feed) zeroes the confirmation
/// term only — S2 degrades to the pure fade, the documented §5 fallback.
pub fn s2(cfg: &Stage1Cfg) -> Expr {
    let confirmed = col(cols::GAP_Z)
        * col(cols::VS_AUCT).sign().cast(DataType::Float64)
        * col(cols::VS_AUCT).abs().gt(lit(cfg.vs_threshold)).cast(DataType::Float64);
    confirmed.fill_null(lit(0.0)) - lit(cfg.eta) * col(cols::GAP_Z)
}

/// S6 (§3.2): mean of the AVAILABLE standardized linked-market subcomponents —
/// a missing feed shrinks the denominator instead of nulling the factor.
fn s6() -> Expr {
    let cnt = col(cols::AH_DELTA).is_not_null().cast(DataType::Float64)
        + col(cols::A50_BETA_RET).is_not_null().cast(DataType::Float64)
        + col(cols::ADR_RESID).is_not_null().cast(DataType::Float64);
    let sum = col(cols::AH_DELTA).fill_null(lit(0.0))
        + col(cols::A50_BETA_RET).fill_null(lit(0.0))
        + col(cols::ADR_RESID).fill_null(lit(0.0));
    when(cnt.clone().gt(lit(0.0))).then(sum / cnt).otherwise(lit(NULL))
}

/// Composite: rank-inverse-normal each factor across sectors, ICIR-shrunk weights,
/// missing factors neutral at combine. Returns the scored frame sorted descending.
pub fn sector_composite(
    ctx: &OpenContext, w: &FactorWeights, cfg: &Stage1Cfg,
) -> Result<DataFrame, FactorError> {
    const ALL: &str = "__all";
    let key = || [col(cols::SECTOR)];
    let lf = ctx.sector_nightly.clone().lazy()
        .with_column(col(cols::SECTOR).cast(DataType::UInt32))
        .join(
            ctx.sector_auction.clone().lazy()
                .with_column(col(cols::SECTOR).cast(DataType::UInt32)),
            key(), key(), JoinArgs::new(JoinType::Left),
        )
        .join(
            ctx.sector_linked.clone().lazy()
                .with_column(col(cols::SECTOR).cast(DataType::UInt32)),
            key(), key(), JoinArgs::new(JoinType::Left),
        )
        .with_columns([
            s1(cfg).alias(cols::S1),
            s2(cfg).alias(cols::S2),
            s6().alias(cols::S6),
        ])
        .with_column(lit(1i32).alias(ALL));

    let fs = [cols::S1, cols::S2, cols::S3, cols::S4, cols::S5, cols::S6];
    let lf = xsec::finite_or_null(lf, &fs);
    let lf = xsec::rank_inv_normal(lf, &fs, ALL);
    let score = fs.iter()
        .map(|f| {
            let zc = cols::z(f);
            lit(w.get(f)) * col(zc.as_str()).fill_null(lit(0.0))
        })
        .reduce(|a, b| a + b)
        .expect("factor list non-empty");

    Ok(lf.with_column(score.alias(cols::SIGMA_SCORE))
        .sort_by_exprs(
            [col(cols::SIGMA_SCORE)],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .collect()?)
}

/// Top-K selection with the absolute-quality gate: Σ_(K) > Σ_min or CASH (§3.2).
pub fn select_sectors(
    scored: &DataFrame, cfg: &Stage1Cfg,
) -> Result<Vec<SectorId>, FactorError> {
    if scored.height() == 0 {
        return Ok(vec![]);
    }
    let s = scored.column(cols::SIGMA_SCORE)?.as_materialized_series().f64()?.clone();
    let ids = scored.column(cols::SECTOR)?.as_materialized_series().u32()?.clone();
    let mut out = Vec::new();
    for i in 0..scored.height().min(cfg.top_k_sectors) {
        match (s.get(i), ids.get(i)) {
            (Some(v), Some(id)) if v.is_finite() && v > cfg.sigma_min_gate => {
                let id = u16::try_from(id)
                    .map_err(|_| FactorError::Contract("sector id exceeds u16"))?;
                out.push(SectorId(id));
            }
            _ => break, // sorted descending ⇒ nothing below clears the gate
        }
    }
    Ok(out)
}

// Silence unused-import lint for the re-exported base path used only in docs.
#[allow(unused_imports)]
use _base as _;

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    fn cfg(gate: f64) -> Stage1Cfg {
        Stage1Cfg {
            theta1: 1.0, theta2: 1.0, eta: 0.25, vs_threshold: 0.5,
            leadlag_window: 250, fdr_q: 0.10, icir_window: 250, icir_shrink_delta: 0.10,
            top_k_sectors: 2, sigma_min_gate: gate, member_weight_cap: 0.15,
        }
    }

    fn ctx() -> OpenContext {
        OpenContext {
            date: chrono::NaiveDate::from_ymd_opt(2026, 7, 3).unwrap(),
            sector_nightly: df!(
                "sector" => vec![1u32, 2, 3],
                "r_id_1" => vec![0.02, -0.01, 0.0],
                "r_on_1" => vec![-0.01, 0.02, 0.0],
                "sigma_id" => vec![0.01, 0.01, 0.01],
                "sigma_on" => vec![0.01, 0.01, 0.01],
                "s3" => vec![0.5, -0.5, 0.0],
                "s4" => vec![0.3, -0.3, 0.0],
                "s5" => vec![1.0, -1.0, 0.0],
            ).unwrap(),
            sector_auction: df!(
                "sector" => vec![1u32, 2, 3],
                "gap_z" => vec![1.0, -1.0, 0.0],
                "vs_auct" => vec![1.0, 0.1, 0.0], // sector 2 unconfirmed (|VS| < v*)
            ).unwrap(),
            sector_linked: df!(
                "sector" => vec![1u32, 2, 3],
                "ah_delta" => vec![Some(0.5), None, None],
                "a50_beta_ret" => vec![None::<f64>, None, None],
                "adr_resid_agg" => vec![None::<f64>, None, None],
            ).unwrap(),
        }
    }

    #[test]
    fn composite_orders_and_gates() {
        let w = FactorWeights::equal(&["s1", "s2", "s3", "s4", "s5", "s6"]);
        let scored = sector_composite(&ctx(), &w, &cfg(0.1)).unwrap();
        let top = scored.column("sector").unwrap().as_materialized_series()
            .u32().unwrap().get(0).unwrap();
        assert_eq!(top, 1); // strong traded-continuation + confirmed gap wins
        let sel = select_sectors(&scored, &cfg(0.1)).unwrap();
        assert!(!sel.is_empty());
        assert_eq!(sel[0], hkq_core::ids::SectorId(1));
    }

    #[test]
    fn cash_day_when_nothing_clears_gate() {
        let w = FactorWeights::equal(&["s1", "s2", "s3", "s4", "s5", "s6"]);
        let scored = sector_composite(&ctx(), &w, &cfg(999.0)).unwrap();
        assert!(select_sectors(&scored, &cfg(999.0)).unwrap().is_empty());
    }

    #[test]
    fn missing_auction_feed_degrades_not_blocks() {
        let mut c = ctx();
        c.sector_auction = df!(
            "sector" => Vec::<u32>::new(),
            "gap_z" => Vec::<f64>::new(),
            "vs_auct" => Vec::<f64>::new(),
        ).unwrap();
        let w = FactorWeights::equal(&["s1", "s2", "s3", "s4", "s5", "s6"]);
        let scored = sector_composite(&c, &w, &cfg(0.1)).unwrap();
        assert_eq!(scored.height(), 3); // S2 null → neutral; composite still ranks
    }
}
```

```rust
// crates/hkq-factors/src/stage2.rs
//! Stage 2 (§3.3–§3.4): stock factors within selected sectors, the IVU regime
//! gate, and the FIXED pipeline — winsorize → rank-inverse-normal → sequential
//! orthogonalization → ICIR combine. X2's null column (X2-disabled mode, §5)
//! flows through the standard missing-data policy and zeroes itself at combine.
use crate::cols::{self};
use crate::error::FactorError;
use crate::icir::FactorWeights;
use crate::xsec;
use hkq_core::config::Stage2Cfg;
use polars::prelude::*;

const E: f64 = std::f64::consts::E;
const XS: [&str; 6] = [cols::X1, cols::X2, cols::X3, cols::X5, cols::X6, cols::X7];

/// X4 regime gate (§3.3): multiplicative [ivu_tercile][x1|x2|x3] matrix from the
/// quarterly threshold-regression job (hkq-validate). Identity until first fit.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RegimeGate {
    pub g: [[f64; 3]; 3],
}

impl Default for RegimeGate {
    fn default() -> Self {
        Self { g: [[1.0; 3]; 3] }
    }
}

/// Input contract (one row per candidate stock; engine-built at freeze):
/// [code, sector, gap_z, gap_z_sector, beta_sector, vs_auct_i,
///  iep_0910, iep_0920, sigma_on, vol_tau0, vol_tau0_med20,
///  rsj_1, jump_1, rv_5d, lav, x6_spillover, sb_z, connect_elig, ivu_tercile].
/// Nullable everywhere; nulls follow the crate-wide neutral policy.
pub fn stage2_scores(
    stocks: DataFrame, gate: &RegimeGate, w: &FactorWeights, cfg: &Stage2Cfg,
) -> Result<DataFrame, FactorError> {
    const CS: &str = "__cs";
    for o in &cfg.ortho_order {
        if !XS.contains(&o.as_str()) {
            return Err(FactorError::Contract("ortho_order contains unknown factor name"));
        }
    }

    let lf = stocks.lazy()
        .with_column(col(cols::SECTOR).cast(DataType::UInt32))
        // X1 (§3.3): idiosyncratic gap, fade unless auction-volume confirmed.
        // Null VS ⇒ indicator 0 ⇒ pure fade — the documented IEV-less fallback.
        .with_column(
            (col(cols::GAP_Z) - col(cols::BETA_SECTOR) * col(cols::GAP_Z_SECTOR))
                .alias(cols::EPS_GAP),
        )
        .with_column(
            (lit(-1.0) * col(cols::EPS_GAP)
                + lit(cfg.phi)
                    * col(cols::EPS_GAP)
                    * col(cols::VS_AUCT_I).gt(lit(cfg.vs_threshold_stock))
                        .cast(DataType::Float64)
                        .fill_null(lit(0.0)))
            .alias(cols::X1),
        )
        // X2: late-auction IEP drift after the 09:15 no-cancel regime. Null in
        // X2-disabled mode; weight auto-zeroes at combine.
        .with_column(
            when(
                col(cols::IEP_0910).gt(lit(0.0))
                    .and(col(cols::IEP_0920).gt(lit(0.0)))
                    .and(col(cols::SIGMA_ON).gt(lit(0.0))),
            )
            .then((col(cols::IEP_0920) / col(cols::IEP_0910)).log(E) / col(cols::SIGMA_ON))
            .otherwise(lit(NULL))
            .alias(cols::X2),
        )
        // X3: opening volume surprise vs the 20d same-bin median (two-phase input:
        // auction-only at freeze, refreshed with first-5m volume at 09:35).
        .with_column(
            when(col(cols::VOL_TAU0).gt(lit(0.0)).and(col(cols::VOL_TAU0_MED20).gt(lit(0.0))))
                .then((col(cols::VOL_TAU0) / col(cols::VOL_TAU0_MED20)).log(E))
                .otherwise(lit(NULL))
                .alias(cols::X3),
        )
        // X5: −RSJ_{t−1}·1{𝒥>0} + ζ·√RV⁽⁵ᵈ⁾/LAV — either term may be absent;
        // the present one carries the factor.
        .with_column({
            let t1 = lit(-1.0) * col(cols::RSJ_1)
                * col(cols::JUMP_1).gt(lit(0.0)).cast(DataType::Float64);
            let t2 = when(col(cols::LAV).gt(lit(0.0)))
                .then(lit(cfg.zeta) * col(cols::RV_5D).sqrt() / col(cols::LAV))
                .otherwise(lit(NULL));
            when(t1.clone().is_not_null().or(t2.clone().is_not_null()))
                .then(t1.fill_null(lit(0.0)) + t2.fill_null(lit(0.0)))
                .otherwise(lit(NULL))
                .alias(cols::X5)
        })
        .with_column(col(cols::X6_SPILLOVER).alias(cols::X6))
        // X7: southbound z, hard zero for non-Connect names (clientele dummy is
        // an ML-layer concern, §3.3).
        .with_column(
            (col(cols::SB_Z) * col(cols::CONNECT_ELIG).cast(DataType::Float64))
                .alias(cols::X7),
        );

    // X4 gates X1–X3 multiplicatively per IVU tercile; null tercile ⇒ middle row.
    let lf = lf.with_columns([
        gated(gate, 0).alias(cols::X1),
        gated(gate, 1).alias(cols::X2),
        gated(gate, 2).alias(cols::X3),
    ]);

    // §3.4 pipeline over the selected-sector union as ONE cross-section.
    let lf = lf.with_column(lit(1i32).alias(CS));
    let lf = xsec::finite_or_null(lf, &XS);
    let lf = xsec::winsorize(lf, &XS, CS, cfg.winsor_pct);
    let lf = xsec::rank_inv_normal(lf, &XS, CS);
    let df = lf.collect()?;

    let order: Vec<String> = cfg.ortho_order.iter().map(|s| cols::z(s)).collect();
    let order_ref: Vec<&str> = order.iter().map(String::as_str).collect();
    let df = xsec::orthogonalize_daily(df, &order_ref, CS)?;

    let score = XS.iter()
        .map(|f| {
            let zc = cols::z(f);
            lit(w.get(f)) * col(zc.as_str()).fill_null(lit(0.0))
        })
        .reduce(|a, b| a + b)
        .expect("factor list non-empty");

    Ok(df.lazy()
        .with_column(score.alias(cols::SCORE))
        .sort_by_exprs(
            [col(cols::SCORE)],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .collect()?)
}

fn gated(gate: &RegimeGate, which: usize) -> Expr {
    const XCOLS: [&str; 3] = [cols::X1, cols::X2, cols::X3];
    let c = XCOLS[which];
    when(col(cols::IVU_TERCILE).eq(lit(0u32)))
        .then(col(c) * lit(gate.g[0][which]))
        .when(col(cols::IVU_TERCILE).eq(lit(2u32)))
        .then(col(c) * lit(gate.g[2][which]))
        .otherwise(col(c) * lit(gate.g[1][which])) // tercile 1 AND cold-start null
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    fn cfg() -> Stage2Cfg {
        Stage2Cfg {
            phi: 2.0, zeta: 0.5, vs_threshold_stock: 0.5, beta_window: 250,
            winsor_pct: 0.01,
            ortho_order: vec!["x5".into(), "x3".into(), "x2".into(), "x1".into(),
                              "x6".into(), "x7".into()],
            names_per_sector: 2,
        }
    }

    fn frame(x2_disabled: bool) -> DataFrame {
        let n = 6;
        let iep10 = if x2_disabled { vec![None::<f64>; n] }
                    else { vec![Some(10.0); n] };
        let iep20 = if x2_disabled { vec![None::<f64>; n] }
                    else { vec![Some(10.05), Some(10.0), Some(9.95), Some(10.0),
                                Some(10.0), Some(10.0)] };
        df!(
            "code" => vec![1u32, 2, 3, 4, 5, 6],
            "sector" => vec![1u32; n],
            // A (code 1): big gap UNCONFIRMED. B (code 2): same gap CONFIRMED.
            "gap_z" => vec![2.0, 2.0, 0.1, -0.1, 0.0, 0.05],
            "gap_z_sector" => vec![0.0; n],
            "beta_sector" => vec![1.0; n],
            "vs_auct_i" => vec![0.1, 2.0, 0.0, 0.0, 0.0, 0.0],
            "iep_0910" => iep10,
            "iep_0920" => iep20,
            "sigma_on" => vec![0.01; n],
            "vol_tau0" => vec![100.0; n],
            "vol_tau0_med20" => vec![100.0; n],
            "rsj_1" => vec![0.0; n],
            "jump_1" => vec![0.0; n],
            "rv_5d" => vec![0.0001; n],
            "lav" => vec![0.1; n],
            "x6_spillover" => vec![0.0; n],
            "sb_z" => vec![0.0; n],
            "connect_elig" => vec![1.0; n],
            "ivu_tercile" => vec![1u32; n],
        ).unwrap()
    }

    #[test]
    fn confirmation_flips_gap_sign() {
        let w = FactorWeights::from_raw([("x1".to_string(), 1.0)]);
        let out = stage2_scores(frame(false), &RegimeGate::default(), &w, &cfg()).unwrap();
        let top = out.column("code").unwrap().as_materialized_series()
            .u32().unwrap().get(0).unwrap();
        assert_eq!(top, 2); // confirmed gap → +φ·ε dominates; unconfirmed → fade
        let codes = out.column("code").unwrap().as_materialized_series().u32().unwrap().clone();
        let last = codes.get(out.height() - 1).unwrap();
        assert_eq!(last, 1); // the unconfirmed big gap is the worst-ranked name
    }

    #[test]
    fn x2_disabled_mode_scores_stay_finite() {
        let w = FactorWeights::equal(&XS);
        let out = stage2_scores(frame(true), &RegimeGate::default(), &w, &cfg()).unwrap();
        let s = out.column("score").unwrap().as_materialized_series().f64().unwrap().clone();
        assert_eq!(s.null_count(), 0);
        assert!(s.into_no_null_iter().all(|v| v.is_finite()));
        // and the x2 rank-z column is fully null (excluded, not fabricated)
        let x2z = out.column("x2_z").unwrap();
        assert_eq!(x2z.null_count(), out.height());
    }

    #[test]
    fn regime_gate_can_silence_a_factor() {
        let mut gate = RegimeGate::default();
        gate.g[1] = [0.0, 1.0, 1.0]; // tercile-1 rows: X1 multiplied to zero
        let w = FactorWeights::from_raw([("x1".to_string(), 1.0)]);
        let out = stage2_scores(frame(false), &gate, &w, &cfg()).unwrap();
        let s = out.column("score").unwrap().as_materialized_series().f64().unwrap().clone();
        // X1 became a constant zero column ⇒ ranks tie ⇒ all z = 0 ⇒ all scores 0.
        assert!(s.into_no_null_iter().all(|v| v.abs() < 1e-12));
    }
}
```

```rust
// crates/hkq-factors/src/panel.rs
//! The ONE seam onto the M1 lake (per the milestone-1 hand-off: `Lake::scan` /
//! `scan_date` + the `cols` contract). Everything returned is lazy; date pruning
//! uses ISO date-string comparison, which is lexicographically correct.
use crate::cols::base;
use crate::error::FactorError;
use crate::{moments, realized};
use chrono::NaiveDate;
use hkq_core::config::FactorCfg;
use hkq_data::lake::{Dataset, Lake};
use polars::prelude::*;

pub struct PanelBuilder<'a> {
    pub lake: &'a Lake,
    pub cfg: &'a FactorCfg,
}

fn date_range(lf: LazyFrame, from: NaiveDate, to: NaiveDate) -> LazyFrame {
    lf.filter(
        col(base::DATE)
            .gt_eq(lit(from.to_string()))
            .and(col(base::DATE).lt_eq(lit(to.to_string()))),
    )
}

impl<'a> PanelBuilder<'a> {
    pub fn new(lake: &'a Lake, cfg: &'a FactorCfg) -> Self {
        Self { lake, cfg }
    }

    pub fn daily_bars(&self, from: NaiveDate, to: NaiveDate) -> Result<LazyFrame, FactorError> {
        Ok(date_range(self.lake.scan(Dataset::DailyBars)?, from, to))
    }

    pub fn bars_1m(&self, from: NaiveDate, to: NaiveDate) -> Result<LazyFrame, FactorError> {
        Ok(date_range(self.lake.scan(Dataset::Bars1m)?, from, to))
    }

    pub fn flows(&self, from: NaiveDate, to: NaiveDate) -> Result<LazyFrame, FactorError> {
        Ok(date_range(self.lake.scan(Dataset::Flows)?, from, to))
    }

    pub fn mainland_prints(&self, from: NaiveDate, to: NaiveDate) -> Result<LazyFrame, FactorError> {
        Ok(date_range(self.lake.scan(Dataset::MainlandPrints)?, from, to))
    }

    /// §3.0 enriched daily panel. `from` must already include the caller's warmup
    /// (EWMA min_obs 63 + Amihud 60 ⇒ ~130 trading days before the first date you
    /// intend to consume). If the 1-minute dataset is missing entirely, degrades
    /// LOUDLY: `rv` is injected as null, so rv_5d/lav are null and X5 leans on its
    /// RSJ term — never a silent fabrication.
    pub fn enriched_daily(
        &self, from: NaiveDate, to: NaiveDate, sector_map: Option<DataFrame>,
    ) -> Result<LazyFrame, FactorError> {
        let daily = self.daily_bars(from, to)?;
        let joined = match self.bars_1m(from, to) {
            Ok(bars) => daily.join(
                realized::realized_daily(bars),
                [col(base::CODE), col(base::DATE)],
                [col(base::CODE), col(base::DATE)],
                JoinArgs::new(JoinType::Left),
            ),
            Err(e) => {
                tracing::warn!(error = %e,
                    "bars_1m dataset unavailable; rv/rv_5d/lav degrade to null");
                daily.with_column(lit(NULL).cast(DataType::Float64).alias(crate::cols::RV))
            }
        };
        let enriched = moments::enrich_daily_panel(joined, self.cfg);
        Ok(match sector_map {
            Some(m) => enriched.join(
                m.lazy(),
                [col(base::CODE)],
                [col(base::CODE)],
                JoinArgs::new(JoinType::Left),
            ),
            None => enriched,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hkq_data::lake::Lake;
    use polars::df;

    #[test]
    fn lake_roundtrip_through_enrichment() {
        let root = std::env::temp_dir().join(format!(
            "hkq_factors_panel_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        let lake = Lake::new(&root);
        let d1 = chrono::NaiveDate::from_ymd_opt(2026, 7, 2).unwrap();
        let d2 = chrono::NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        for d in [d1, d2] {
            let mut df = df!(
                "code" => vec![700u32, 5u32],
                "date" => vec![d.to_string(); 2],
                "open" => vec![100.0, 60.0],
                "high" => vec![101.0, 61.0],
                "low" => vec![99.0, 59.0],
                "close" => vec![100.5, 60.5],
                "adj_close" => vec![100.5, 60.5],
                "volume" => vec![1.0e6, 2.0e6],
                "turnover" => vec![1.0e8, 1.2e8],
            ).unwrap();
            lake.write_partition(Dataset::DailyBars, d, &mut df, "test", 1).unwrap();
        }
        let cfg = FactorCfg {
            ewma_halflife_days: 21.0, ewma_min_obs: 2, amihud_window: 2, rv_days: 2,
            lav_gamma: 0.3, seasonal_vol_days: 20, ivu_tercile_window: 60,
        };
        let pb = PanelBuilder::new(&lake, &cfg);
        // No bars_1m dataset written ⇒ degraded path: rv column injected as null.
        let out = pb.enriched_daily(d1, d2, None).unwrap().collect().unwrap();
        assert_eq!(out.height(), 4);
        for c in ["r_on", "r_id", "r_cc", "sigma_cc", "illiq", "lav"] {
            assert!(out.get_column_names().iter().any(|n| n.as_str() == c), "missing {c}");
        }
        std::fs::remove_dir_all(root).ok();
    }
}
```

## Honest gaps and hand-off to Milestone 3

Four gaps, all inherited data-plane facts rather than factor-math facts, now concentrated where M3 can close them: (1) nothing *produces* `Bars1m` partitions yet — the provider method exists since M1 but the nightly binary doesn't call `backfill_bars_1m`; until M3 wires that step, `PanelBuilder::enriched_daily` runs the documented null-`rv` degradation. (2) `vs_auct`/`iev_bar20` need auction *history*, which only accumulates once the live POS poller persists `Auction` partitions — until then S2 self-degrades to the pure −η fade and X1 to the pure fade, both by the null policy, not by special-case code. (3) Lead–lag residuals are cross-sectionally demeaned, not HSI-β-hedged; the β estimation job belongs with `hkq-validate`'s quarterly fits and swaps in behind `panel_to_matrix` without touching the graph API. (4) The IC panel that feeds `FactorWeights::from_ic_panel` is appended by the post-close attribution job (runbook §6), which is M3 scope alongside `hkq-signal`'s confirmation/meta-label layer and `hkq-risk`'s `size_book` — the sector map (`universe_static.parquet`) also becomes a config-owned path then, which is why every function here takes it as a parameter instead of freezing a premature `OpsCfg` field. Milestone 3 therefore consumes this crate through exactly three seams: `stage1::sector_composite`/`select_sectors` and `stage2::stage2_scores` for the decision path, `FactorWeights` + `spearman_ic` for the nightly weight refresh, and `PanelBuilder` for every panel it regresses on.
