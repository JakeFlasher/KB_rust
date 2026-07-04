
All ten documents are attached in full, so I worked directly from them — no fetching needed (I have search only, and it isn't required here). M8's closing line names the remaining territory exactly: "What it still lacks is breadth of evidence — the benchmark family and the reconciliation binary that are data acquisitions, not code — and the learned layer (§3.7, with CPCV and the self-hosted walk-forward) that would finally consume the machinery this milestone completed." The learned layer is explicitly gated behind its consumer ("deep rankers as research candidates only after the linear stack's capacity is exhausted", report §3.7) and behind a weights-source seam M8 deliberately deferred. The benchmark family and `hkq-recon`, by contrast, have been *unchanged, named deferrals since M5* — and both become pure composition today: the SPA math is a lake-only batch job the moment benchmark series exist, benchmark series are one nightly step through the **existing** `EodProvider` seam (HK-listed instruments — zero new vendor surface), and the recon binary was designed in M1 (`reconcile_closes` + `quarantine` are built and tested) minus only an independent close source, which is one config-templated fetch in the M1 EastMoney pattern. That is the milestone.

# Milestone 9 — The Referee (benchmark family + Hansen SPA + `hkq-recon`: breadth of evidence)

**Why this is next.** M8 completed the *promotion* arm of §4: honest walk-forward trials feed a DSR that deflates against a tamper-evident registry. But §4 prescribes two arms, and the second — "Complement with the Hansen SPA test against the null family {always-cash, HSI open-close, sector-ETF version of Stage 1}" — has been starved by the same data fact since M5: nothing ingests benchmark series. The DSR answers "is this alpha distinguishable from selection noise?"; the SPA answers the falsification question the DSR structurally cannot: "does a *trivial* alternative beat it?" A strategy can clear DSR > 0.95 while losing to the stamp-exempt sector-ETF twin of its own Stage-1 signal — exactly the embarrassment the report's cost-floor logic (§1) exists to prevent, and exactly what nothing currently measures. Meanwhile the *oldest* standing gap in the codebase — "recon skipped: independent official-close source not configured (M1 gap)" — has been logged loudly every single night since M1, and `hkq-recon` is the only blueprint binary not built. Both gaps close with the same shape of work: config-templated acquisition of small daily series plus pure math over the lake, no engine surgery, no frozen-crate rewrites. And there is a sequencing reason it must precede the §3.7 layer: the ML milestone will generate *many* trials fast; promoting any of them without the SPA referee and a reconciled price ledger would put a learned model behind gates that are half-built.

**In scope:** the `Dataset::Benchmarks` variant (two-line lake patch, M4 `Scores` precedent — a *separate* dataset so benchmark instruments can never leak into the §3.1 universe panels); the crate-owned `[benchmarks]` table in `hkq-data::cfg` (per the `[sources]` precedent: its first consumer is an ingestion step) defining the family {codes, hsi_code, sector→ETF map}; the nightly ingestion step in the `hkq-nightly` binary (M3 precedent: binaries wire steps, frozen crates stay frozen) reusing `EodProvider::daily_bars`; `hkq-validate::benchmarks` (the null-family return panel: always-cash, HSI-proxy open→close, and the sector-ETF Stage-1 arm reconstructed from *persisted live-frozen* `Scores` sectors — cash on cash days, dropped-never-imputed on data gaps); `hkq-validate::spa` (Hansen 2005 SPA: studentized max statistic, stationary bootstrap with one shared index path across arms, consistent recentering, kernel long-run variance tied to the same bootstrap parameter, deterministic xorshift PRNG per the M5 house style); the `spa` job in the validate binary with registry logging; and the `hkq-recon` binary — the blueprint's fourth bin — built with **zero patches** on public seams (`RatedClient`, `reconcile_closes`, `Lake::quarantine`, `TrialsRegistry`), fetching official closes from a config-templated independent source and quarantining breached partitions. **Deferred:** the §3.7 learned layer with CPCV and the self-hosted walk-forward (unchanged owner — its seam and its consumer arrive together); VHSI and A50 ingestion (index-quote vendor surface, unchanged owner; `vhsi_tercile` stays `None`); the Fills venue tag / schema v2 (κ-honesty stays operator discipline, data-layer owner unchanged); quote/VCM replay (not persisted, unchanged); promotion *enforcement* (operator governance, by design, unchanged).

Engineering decisions beyond the blueprint sketch, briefly. **The SPA direction is chosen deliberately**: Hansen's test takes one base model and a family of alternatives, so the base is *our strategy* and the alternatives are the nulls — d_k,t = r_bench,k,t − r_strat,t, H₀: no family member has superior predictive ability. A **small** p-value falsifies the strategy (some benchmark beats it); a large p-value is the absence-of-dominance the promotion protocol needs alongside DSR's positive evidence. SPA falsifies, DSR promotes — the report's "complement" made mechanical. The bootstrap shares one stationary index path per replicate across all arms because the max statistic is meaningless without cross-arm dependence; the recentering is Hansen's consistent (SPA_c) rule, so a benchmark the strategy crushes recenters to zero and cannot dilute the null distribution; ω̂² uses the stationary-bootstrap-implied kernel with the *same* q, and at q = 1 collapses exactly to γ̂₀ — which is the unit test. **The benchmark instruments are HK-listed codes** (2800 as the HSI open→close proxy, sector ETFs for the Stage-1 arm), which buys three things at once: zero new vendor surface (the existing Tiger kline path serves them), stamp-exemption realism (the report itself mandates the ETF expression as the benchmark the single-name alpha "must justify itself" against), and *implementability* — you cannot hold the index itself, so the tradeable proxy is the honest null. **The strategy return series is complete by construction**: within [first, last] recorded activity, a session without fills has *truly* zero realized PnL (Fills is the complete record; the book is flat daily — M4's terminal invariant), so cash days enter the comparison honestly instead of vanishing; sessions where a configured arm has no honest input are dropped and counted, never imputed, and strategy sessions missing benchmark bars are reported loudly as ingestion gaps. The ETF-Stage1 arm reconstructs each day's selection from the *persisted* Scores partition (distinct sectors = the frozen 09:29:30 selection — no recomputed morning factors, the M3/M4 honesty rule), and holds cash when the strategy held cash. **`hkq-recon` mutates nothing on a clean night and stands the system down structurally on a dirty one**: a breach quarantines the `daily_bars` partition, which makes the next PreMarket fail loudly on its own missing-input contract — the designed halt, with no new state file to rot; every run appends a `recon_ok`/`recon_breach` record to the hash-chained registry (the audit trail is the registry, not a memo — M5), and since those records carry no `sr` metric they cannot distort DSR's N. Independence is config-deep and stated: the default template points at a *different vendor* than the ingest path, the parser is schema-versioned and refuses zero-row responses, and reconciling Tiger against Tiger is structurally impossible because the template is required config. Equity for return normalization is a CLI argument (statistics-side f64), per the "runtime param, not config" precedent.

```text
hkq/
├── Cargo.toml                        (updated: member hkq-recon)
└── crates/
    ├── hkq-data/src/{lake,cfg}.rs    (surgical: Dataset::Benchmarks variant;
    │                                  [benchmarks] table + loader — appends only)
    ├── hkq-nightly/
    │   ├── Cargo.toml                (updated: dev-dependency async-trait)
    │   └── src/main.rs               (updated: benchmarks ingestion step)
    ├── hkq-validate/
    │   └── src/{lib,benchmarks,spa,main}.rs
    │                                 (benchmarks.rs, spa.rs NEW; lib/main updated;
    │                                  asof, kappa, cfg, splits, stats, dsr, cusum,
    │                                  ic, pnl, registry, fits stay byte-identical)
    └── hkq-recon/
        ├── Cargo.toml                (NEW)
        └── src/main.rs               (NEW — the blueprint's fourth binary)
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

Two append-blocks and one two-line enum change, in the M5/M6 style — everything else in M1–M8 stays byte-identical.

In `crates/hkq-data/src/lake.rs`, extend the `Dataset` enum (the M4 `Scores` precedent):

```rust
// (inside crates/hkq-data/src/lake.rs `enum Dataset`, after `Scores`)
    /// M9: benchmark-instrument daily bars (HSI proxy + sector ETFs) — the §4
    /// SPA null family's raw material. A SEPARATE dataset by design: benchmark
    /// instruments must never leak into the §3.1 universe panels.
    Benchmarks,
```

and the matching arm in `Dataset::dir`:

```rust
// (inside crates/hkq-data/src/lake.rs `impl Dataset::dir`, after the Scores arm)
            Dataset::Benchmarks => "benchmarks",
```

And the `[benchmarks]` table, owned by `hkq-data` per the `[sources]` precedent (its first consumer is an ingestion step; hkq-validate consumes the same table for the SPA family):

```rust
// (append inside crates/hkq-data/src/cfg.rs)

/// M9: the §4 benchmark-family definition — `[benchmarks]` table of
/// strategy.toml. Instruments are HK-LISTED codes fetched through the existing
/// `EodProvider` — ZERO new vendor surface. hkq-nightly ingests `codes` daily
/// into Dataset::Benchmarks; hkq-validate's SPA job reads `hsi_code` and the
/// sector→ETF map to build the null family {always-cash, HSI open→close,
/// sector-ETF Stage-1}.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct BenchmarksCfg {
    /// Every benchmark instrument to ingest daily. Must include `hsi_code` and
    /// every ETF referenced by the sector map — validate warns when an arm's
    /// instrument is missing from the data, but ingestion is driven by THIS list.
    pub codes: Vec<u32>,
    /// The HSI open→close proxy (e.g. 2800, the Tracker Fund): the §4 null #2.
    /// A tradeable, stamp-exempt instrument is the IMPLEMENTABLE index benchmark.
    #[serde(default)]
    pub hsi_code: Option<u32>,
    /// CSV `sector_id,etf_code` for the sector-ETF Stage-1 arm (§4 null #3).
    #[serde(default)]
    pub sector_etf_map_path: Option<std::path::PathBuf>,
}

#[derive(Debug, serde::Deserialize)]
struct BenchFile {
    benchmarks: Option<BenchmarksCfg>,
}

/// Absent table ⇒ Ok(None): the SPA family is explicitly opt-in (paper-only
/// configs stay valid). A present-but-inconsistent table is a LOUD error.
pub fn load_benchmarks(path: impl AsRef<Path>) -> Result<Option<BenchmarksCfg>, DataError> {
    let raw = std::fs::read_to_string(path.as_ref())?;
    let f: BenchFile = toml::from_str(&raw)
        .map_err(|e| DataError::Config(format!("{}: {e}", path.as_ref().display())))?;
    let Some(b) = f.benchmarks else { return Ok(None) };
    if b.codes.is_empty() {
        return Err(DataError::Config("[benchmarks] codes must be non-empty".into()));
    }
    if let Some(h) = b.hsi_code {
        if !b.codes.contains(&h) {
            return Err(DataError::Config(format!(
                "[benchmarks] hsi_code {h} must appear in codes — it is ingested from there"
            )));
        }
    }
    Ok(Some(b))
}

#[cfg(test)]
mod bench_cfg_tests {
    use super::*;

    fn tmp_with(content: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "hkq_benchcfg_{}.toml", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn absent_table_is_none() {
        let p = tmp_with("[ops]\nx = 1\n");
        assert!(load_benchmarks(&p).unwrap().is_none());
        std::fs::remove_file(p).ok();
    }

    #[test]
    fn hsi_membership_enforced_and_full_table_parses() {
        let p = tmp_with("[benchmarks]\ncodes = [3033]\nhsi_code = 2800\n");
        assert!(load_benchmarks(&p).is_err()); // hsi not in codes ⇒ loud
        std::fs::remove_file(p).ok();

        let p = tmp_with(
            "[benchmarks]\ncodes = [2800, 3033]\nhsi_code = 2800\n\
             sector_etf_map_path = \"config/sector_etf_map.csv\"\n");
        let b = load_benchmarks(&p).unwrap().unwrap();
        assert_eq!(b.codes, vec![2800, 3033]);
        assert_eq!(b.hsi_code, Some(2800));
        assert!(b.sector_etf_map_path.is_some());
        std::fs::remove_file(p).ok();
    }
}
```

## `hkq-nightly` — the benchmarks ingestion step

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

[dev-dependencies]
async-trait.workspace = true
```

```rust
// crates/hkq-nightly/src/main.rs
//! T−1 18:00 nightly job: EOD + flows + mainland prints + 1-minute bars +
//! (M9) benchmark daily bars → lake.
//! Usage: hkq-nightly <strategy.toml> [YYYY-MM-DD] [--force]
//!
//! M9 addition: the benchmarks step. The §4 SPA null family {always-cash, HSI
//! open→close, sector-ETF Stage-1} needs daily bars for HK-listed benchmark
//! instruments; they flow through the SAME EodProvider as the universe (zero
//! new vendor surface) into their OWN dataset — a separate directory by
//! design, so benchmark instruments can never leak into the §3.1 panels.
//! Wired in the BINARY, not hkq-data::ingest, per the M3 precedent: frozen
//! crates stay frozen, failures isolate, writes stay atomic and idempotent.
use anyhow::Context;
use chrono::{NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg, ids::StockCode,
               session::{DayKind, TradingCalendar}};
use hkq_data::{cfg::{load_benchmarks, load_sources, BenchmarksCfg},
               eastmoney::{load_ah_map, EastMoneyClient},
               ingest::{NightlyIngest, StepOutcome, StepReport}, lake::{Dataset, Lake},
               provider::{EodProvider, FlowProvider, IntradayFeed, LinkedMarketFeed},
               tiger::TigerClient};
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

/// M9: benchmark-instrument daily bars (HSI proxy + sector ETFs) → the
/// Benchmarks dataset — the §4 SPA null family's raw material. Zero rows from
/// the vendor is a FAILURE, not a skip: a configured family that silently
/// stops accruing history would starve the SPA test without anyone noticing.
async fn step_benchmarks(
    lake: &Lake, eod: &dyn EodProvider, bcfg: &BenchmarksCfg, date: NaiveDate, force: bool,
) -> StepReport {
    let name = "benchmarks";
    if !force && lake.exists(Dataset::Benchmarks, date) {
        return StepReport { name, outcome: StepOutcome::SkippedExisting };
    }
    let codes: Vec<StockCode> = bcfg.codes.iter().copied().map(StockCode).collect();
    let outcome = match eod.daily_bars(&codes, date, date).await {
        Ok(mut df) if df.height() > 0 => {
            match lake.write_partition(Dataset::Benchmarks, date, &mut df,
                                       "tiger:kline:benchmarks", 1) {
                Ok(()) => StepOutcome::Written { rows: df.height() },
                Err(e) => StepOutcome::Failed(e.to_string()),
            }
        }
        Ok(_) => StepOutcome::Failed(format!(
            "benchmarks configured but vendor returned zero rows for {} codes", codes.len())),
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

    // M3: 1-minute bars — rv/rv_5d/lav (§3.0), seasonal profiles, IVU (§3.3),
    // and the §3.5 meta-label window returns.
    let bars = step_bars_1m(&lake, &tiger, &codes, date, force).await;
    match &bars.outcome {
        StepOutcome::Failed(e) => tracing::error!(step = bars.name, error = %e, "nightly step FAILED"),
        o => tracing::info!(step = bars.name, outcome = ?o, "nightly step done"),
    }
    report.steps.push(bars);

    // M9: benchmark daily bars — the §4 SPA null family accrues from today.
    if let Some(bcfg) = load_benchmarks(&cfg_path)? {
        let st = step_benchmarks(&lake, &tiger, &bcfg, date, force).await;
        match &st.outcome {
            StepOutcome::Failed(e) => tracing::error!(step = st.name, error = %e, "nightly step FAILED"),
            o => tracing::info!(step = st.name, outcome = ?o, "nightly step done"),
        }
        report.steps.push(st);
    } else {
        tracing::info!("no [benchmarks] table: SPA family ingestion skipped (opt-in)");
    }

    // Close reconciliation now has its own binary (M9): run hkq-recon AFTER this
    // job with an independent [recon] source — the M1 circularity note stands.
    tracing::info!("close reconciliation is hkq-recon's job (independent source; run it after ingest)");

    anyhow::ensure!(report.all_ok(), "one or more nightly steps failed: {report:?}");
    tracing::info!(%date, "nightly ingest complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use hkq_data::error::DataError;
    use polars::df;

    struct FakeEod;

    #[async_trait]
    impl EodProvider for FakeEod {
        async fn daily_bars(
            &self, codes: &[StockCode], from: NaiveDate, _to: NaiveDate,
        ) -> Result<DataFrame, DataError> {
            Ok(df!(
                "code" => codes.iter().map(|c| c.0).collect::<Vec<_>>(),
                "date" => vec![from.to_string(); codes.len()],
                "open" => vec![100.0; codes.len()],
                "high" => vec![101.0; codes.len()],
                "low" => vec![99.0; codes.len()],
                "close" => vec![100.5; codes.len()],
                "adj_close" => vec![100.5; codes.len()],
                "volume" => vec![1.0e6; codes.len()],
                "turnover" => vec![1.0e8; codes.len()],
            ).unwrap())
        }
    }

    #[tokio::test]
    async fn benchmarks_step_writes_then_skips_idempotently() {
        let root = std::env::temp_dir().join(format!(
            "hkq_bench_step_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let lake = Lake::new(&root);
        let bcfg = BenchmarksCfg {
            codes: vec![2800, 3033], hsi_code: Some(2800), sector_etf_map_path: None,
        };
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        let st = step_benchmarks(&lake, &FakeEod, &bcfg, d, false).await;
        assert!(matches!(st.outcome, StepOutcome::Written { rows: 2 }));
        assert!(lake.exists(Dataset::Benchmarks, d));
        let st = step_benchmarks(&lake, &FakeEod, &bcfg, d, false).await;
        assert!(matches!(st.outcome, StepOutcome::SkippedExisting));
        std::fs::remove_dir_all(root).ok();
    }
}
```

## `hkq-validate` — the null-family panel

```rust
// crates/hkq-validate/src/benchmarks.rs
//! The §4 SPA null family, built from the lake (M9): per aligned session, the
//! strategy's daily return and each benchmark arm's daily return.
//!
//! Arms:
//! - always_cash: 0.0 every session — the "do nothing" null.
//! - hsi_open_close: ln(close/open) of the configured HSI proxy (e.g. 2800) —
//!   the implementable "just hold the index intraday" null (§2.1 says HK index
//!   intraday timing is empty; this arm is the test of that claim vs US).
//! - sector_etf_stage1: the stamp-exempt ETF twin of the strategy's OWN
//!   Stage-1 selection (§1: "a sector-ETF expression … should be retained as a
//!   benchmark implementation against which single-name alpha must justify
//!   itself"). Selection is reconstructed from PERSISTED live-frozen Scores
//!   partitions — the day's distinct sectors ARE the day's selection; nothing
//!   is recomputed (the M3/M4 honesty rule). Cash days ⇒ the arm holds cash.
//!
//! Alignment honesty:
//! - Within [first, last] recorded strategy activity, a session with no fills
//!   has TRULY zero realized PnL (Fills is the complete record; the book is
//!   flat daily by the M4 terminal invariant) — cash days enter as real zeros.
//! - A session where a configured arm has no honest input (missing HSI bar;
//!   selected sectors with no mapped ETF bar) is DROPPED and counted — never
//!   imputed.
//! - Strategy sessions missing benchmark bars entirely are counted and
//!   reported loudly: that is an ingestion gap, and silence would bias the
//!   comparison.
use crate::error::ValidateError;
use crate::pnl;
use hkq_data::cfg::BenchmarksCfg;
use hkq_data::lake::{Dataset, Lake};
use hkq_factors::cols::{self, base};
use polars::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub const ARM_CASH: &str = "always_cash";
pub const ARM_HSI: &str = "hsi_open_close";
pub const ARM_ETF: &str = "sector_etf_stage1";

/// CSV `sector_id,etf_code` (comments with '#', unparseable lines skipped —
/// the M1 load_ah_map convention). Empty ⇒ loud config error.
pub fn load_sector_etf_map(path: &Path) -> Result<BTreeMap<u32, u32>, ValidateError> {
    let raw = std::fs::read_to_string(path)?;
    let mut map = BTreeMap::new();
    for line in raw.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let mut parts = line.splitn(2, ',');
        let (Some(s), Some(e)) = (parts.next(), parts.next()) else { continue };
        if let (Ok(sec), Ok(etf)) = (s.trim().parse::<u32>(), e.trim().parse::<u32>()) {
            map.insert(sec, etf);
        }
    }
    if map.is_empty() {
        return Err(ValidateError::Config(format!(
            "empty sector→ETF map: {}", path.display())));
    }
    Ok(map)
}

#[derive(Debug, Clone)]
pub struct BenchmarkPanel {
    pub dates: Vec<String>,
    /// Strategy daily returns: realized PnL / equity, 0.0 on fill-less sessions.
    pub strategy: Vec<f64>,
    /// Aligned benchmark arms, ready for `spa::spa_test`.
    pub arms: Vec<(String, Vec<f64>)>,
    /// Sessions dropped because a configured arm had no honest input that day.
    pub dropped_days: usize,
    /// Strategy sessions (fills recorded) with NO benchmark partition — an
    /// ingestion gap, reported loudly; those sessions are excluded.
    pub missing_benchmark_days: usize,
    /// Panel sessions where the strategy recorded no fills (true zeros).
    pub flat_days: usize,
}

/// [date → {code → ln(close/open)}] from the Benchmarks dataset rows.
fn returns_by_date(bars: &DataFrame) -> Result<BTreeMap<String, BTreeMap<u32, f64>>, ValidateError> {
    let date = bars.column(base::DATE)?.as_materialized_series().str()?.clone();
    let code = bars.column(base::CODE)?.as_materialized_series().u32()?.clone();
    let open = bars.column(base::OPEN)?.as_materialized_series().f64()?.clone();
    let close = bars.column(base::CLOSE)?.as_materialized_series().f64()?.clone();
    let mut out: BTreeMap<String, BTreeMap<u32, f64>> = BTreeMap::new();
    for i in 0..bars.height() {
        let (Some(d), Some(c), Some(o), Some(cl)) =
            (date.get(i), code.get(i), open.get(i), close.get(i)) else { continue };
        if o.is_finite() && cl.is_finite() && o > 0.0 && cl > 0.0 {
            let r = (cl / o).ln();
            if r.is_finite() {
                out.entry(d.to_string()).or_default().insert(c, r);
            }
        }
    }
    Ok(out)
}

/// [date → selected sectors] from persisted Scores. Missing dataset ⇒ the
/// strategy never scored (all sessions are cash days for the ETF arm) — a TRUE
/// empty, not a degradation. A present-but-unreadable dataset stays LOUD.
fn scores_sectors(lake: &Lake) -> Result<BTreeMap<String, BTreeSet<u32>>, ValidateError> {
    let Ok(lf) = lake.scan(Dataset::Scores) else {
        tracing::info!("no scores history: ETF-Stage1 arm treats every session as a cash day");
        return Ok(BTreeMap::new());
    };
    let df = lf.select([col(base::DATE), col(cols::SECTOR)]).collect()?;
    let date = df.column(base::DATE)?.as_materialized_series().str()?.clone();
    let sec = df.column(cols::SECTOR)?.as_materialized_series().u32()?.clone();
    let mut out: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    for i in 0..df.height() {
        if let (Some(d), Some(s)) = (date.get(i), sec.get(i)) {
            out.entry(d.to_string()).or_default().insert(s);
        }
    }
    Ok(out)
}

pub fn build_panel(
    lake: &Lake, bcfg: &BenchmarksCfg, equity_hkd: f64,
) -> Result<BenchmarkPanel, ValidateError> {
    if !(equity_hkd.is_finite() && equity_hkd > 0.0) {
        return Err(ValidateError::Contract("equity must be positive and finite"));
    }
    // Strategy PnL: complete by construction (flat-daily book). No fills at all
    // is fine if scores exist — a shadow that scored but never traded.
    let pnl: BTreeMap<String, f64> = match pnl::daily_pnl(lake) {
        Ok(v) => v.into_iter().collect(),
        Err(ValidateError::Insufficient(_)) => BTreeMap::new(),
        Err(e) => return Err(e),
    };
    let sectors_by_date = scores_sectors(lake)?;

    let mut activity: BTreeSet<String> = pnl.keys().cloned().collect();
    activity.extend(sectors_by_date.keys().cloned());
    let (Some(d0), Some(d1)) = (activity.iter().next().cloned(), activity.iter().last().cloned())
    else {
        return Err(ValidateError::Insufficient("no strategy history (no fills, no scores)"));
    };

    let bars = lake.scan(Dataset::Benchmarks)
        .map_err(|_| ValidateError::Insufficient(
            "no benchmarks dataset — configure [benchmarks] and run hkq-nightly"))?
        .filter(col(base::DATE).gt_eq(lit(d0.clone())).and(col(base::DATE).lt_eq(lit(d1.clone()))))
        .select([col(base::DATE), col(base::CODE), col(base::OPEN), col(base::CLOSE)])
        .collect()?;
    if bars.height() == 0 {
        return Err(ValidateError::Insufficient("benchmarks dataset empty over the strategy window"));
    }
    let ret_by_date = returns_by_date(&bars)?;

    let missing_benchmark_days = pnl.keys()
        .filter(|d| d.as_str() >= d0.as_str() && d.as_str() <= d1.as_str()
            && !ret_by_date.contains_key(*d))
        .count();
    if missing_benchmark_days > 0 {
        tracing::warn!(missing_benchmark_days,
            "strategy sessions lack benchmark bars — ingestion gap; sessions excluded LOUDLY");
    }

    let etf_map = match &bcfg.sector_etf_map_path {
        Some(p) => Some(load_sector_etf_map(p)?),
        None => None,
    };
    let use_hsi = bcfg.hsi_code.is_some();

    let (mut dates, mut strat, mut cash, mut hsi, mut etf) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let (mut dropped, mut flat) = (0usize, 0usize);

    for (d, rets) in &ret_by_date {
        // HSI arm input for this session — absent bar ⇒ the whole day drops.
        let hsi_r = bcfg.hsi_code.and_then(|c| rets.get(&c).copied());
        if use_hsi && hsi_r.is_none() {
            dropped += 1;
            tracing::warn!(date = %d, "no HSI-proxy bar; session dropped from the SPA panel");
            continue;
        }
        // ETF-Stage1 arm: cash on cash days; mean of mapped ETFs on scored days;
        // scored day with zero mapped-ETF bars ⇒ drop, never impute.
        let etf_r = match (&etf_map, sectors_by_date.get(d)) {
            (Some(map), Some(secs)) => {
                let rs: Vec<f64> = secs.iter()
                    .filter_map(|s| map.get(s))
                    .filter_map(|e| rets.get(e))
                    .copied()
                    .collect();
                if rs.is_empty() {
                    dropped += 1;
                    tracing::warn!(date = %d,
                        "selected sectors have no mapped ETF bar; session dropped");
                    continue;
                }
                Some(rs.iter().sum::<f64>() / rs.len() as f64)
            }
            (Some(_), None) => Some(0.0), // strategy held cash ⇒ its ETF twin holds cash
            (None, _) => None,
        };
        let s_r = match pnl.get(d) {
            Some(v) => v / equity_hkd,
            None => { flat += 1; 0.0 } // true zero: fills are the complete record
        };
        dates.push(d.clone());
        strat.push(s_r);
        cash.push(0.0);
        if let Some(h) = hsi_r { hsi.push(h); }
        if let Some(e) = etf_r { etf.push(e); }
    }
    if dates.is_empty() {
        return Err(ValidateError::Insufficient("no aligned sessions between strategy and benchmarks"));
    }

    let mut arms: Vec<(String, Vec<f64>)> = vec![(ARM_CASH.to_string(), cash)];
    if use_hsi { arms.push((ARM_HSI.to_string(), hsi)); }
    if etf_map.is_some() { arms.push((ARM_ETF.to_string(), etf)); }

    Ok(BenchmarkPanel {
        dates, strategy: strat, arms,
        dropped_days: dropped, missing_benchmark_days, flat_days: flat,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use polars::df;
    use std::path::PathBuf;

    fn tmp(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hkq_benchpanel_{tag}_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    fn write_fills(lake: &Lake, d: NaiveDate) {
        let mut fills = df!(
            "code" => vec![700u32; 2],
            "date" => vec![d.to_string(); 2],
            "ts_ms" => vec![1i64, 2],
            "side" => vec!["buy".to_string(), "sell".to_string()],
            "shares" => vec![1_000u64, 1_000],
            "px" => vec![10.0, 11.0],
            "duty" => vec![10.0, 11.0],
            "fees" => vec![1.1, 1.21],
        ).unwrap();
        lake.write_partition(Dataset::Fills, d, &mut fills, "test", 1).unwrap();
    }

    fn write_scores(lake: &Lake, d: NaiveDate, sectors: &[u32]) {
        let mut scores = df!(
            "code" => (1..=sectors.len() as u32).collect::<Vec<_>>(),
            "date" => vec![d.to_string(); sectors.len()],
            "sector" => sectors.to_vec(),
            "score" => vec![1.0f64; sectors.len()],
        ).unwrap();
        lake.write_partition(Dataset::Scores, d, &mut scores, "test", 1).unwrap();
    }

    fn write_bench(lake: &Lake, d: NaiveDate, rows: &[(u32, f64, f64)]) {
        let mut df = df!(
            "code" => rows.iter().map(|r| r.0).collect::<Vec<_>>(),
            "date" => vec![d.to_string(); rows.len()],
            "open" => rows.iter().map(|r| r.1).collect::<Vec<_>>(),
            "high" => rows.iter().map(|r| r.1.max(r.2)).collect::<Vec<_>>(),
            "low" => rows.iter().map(|r| r.1.min(r.2)).collect::<Vec<_>>(),
            "close" => rows.iter().map(|r| r.2).collect::<Vec<_>>(),
            "adj_close" => rows.iter().map(|r| r.2).collect::<Vec<_>>(),
            "volume" => vec![1.0e6; rows.len()],
            "turnover" => vec![1.0e8; rows.len()],
        ).unwrap();
        lake.write_partition(Dataset::Benchmarks, d, &mut df, "test", 1).unwrap();
    }

    fn bcfg(map_path: Option<PathBuf>) -> BenchmarksCfg {
        BenchmarksCfg { codes: vec![2800, 3033, 3067], hsi_code: Some(2800),
                        sector_etf_map_path: map_path }
    }

    #[test]
    fn panel_aligns_cash_days_and_arm_values() {
        let root = tmp("align");
        let lake = Lake::new(&root);
        let d1 = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();
        let d2 = NaiveDate::from_ymd_opt(2026, 7, 2).unwrap();
        let d3 = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();

        write_fills(&lake, d1);
        write_fills(&lake, d3);
        write_scores(&lake, d1, &[1]);
        write_scores(&lake, d3, &[1, 2]);
        for d in [d1, d2, d3] {
            write_bench(&lake, d, &[(2800, 100.0, 101.0), (3033, 50.0, 51.0), (3067, 20.0, 19.8)]);
        }
        let map_path = root.join("sector_etf_map.csv");
        std::fs::write(&map_path, "# sector,etf\n1,3033\n2,3067\n").unwrap();

        let panel = build_panel(&lake, &bcfg(Some(map_path)), 1.0e6).unwrap();
        assert_eq!(panel.dates, vec![d1.to_string(), d2.to_string(), d3.to_string()]);
        // pnl = 11000 − 10000 − 21 − 2.31 = 976.69 per traded day (M5 hand math).
        assert!((panel.strategy[0] - 976.69 / 1.0e6).abs() < 1e-12);
        assert_eq!(panel.strategy[1], 0.0); // cash day: a TRUE zero
        assert!((panel.strategy[2] - 976.69 / 1.0e6).abs() < 1e-12);
        assert_eq!(panel.flat_days, 1);
        assert_eq!(panel.dropped_days, 0);
        assert_eq!(panel.missing_benchmark_days, 0);

        assert_eq!(panel.arms.len(), 3);
        let arm = |n: &str| &panel.arms.iter().find(|(k, _)| k == n).unwrap().1;
        assert!(arm(ARM_CASH).iter().all(|v| *v == 0.0));
        for v in arm(ARM_HSI) {
            assert!((v - (101.0f64 / 100.0).ln()).abs() < 1e-12);
        }
        let e = arm(ARM_ETF);
        assert!((e[0] - (51.0f64 / 50.0).ln()).abs() < 1e-12);          // sector 1 only
        assert_eq!(e[1], 0.0);                                          // cash day
        let mean = ((51.0f64 / 50.0).ln() + (19.8f64 / 20.0).ln()) / 2.0;
        assert!((e[2] - mean).abs() < 1e-12);                           // sectors {1,2}
        std::fs::remove_dir_all(root).ok();
    }

    #[test]
    fn missing_inputs_drop_loudly_never_impute() {
        let root = tmp("drop");
        let lake = Lake::new(&root);
        let d1 = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();
        let d2 = NaiveDate::from_ymd_opt(2026, 7, 2).unwrap();
        write_fills(&lake, d1);
        write_fills(&lake, d2);
        write_bench(&lake, d1, &[(2800, 100.0, 101.0)]);
        write_bench(&lake, d2, &[(3033, 50.0, 51.0)]); // no HSI bar ⇒ day drops
        let panel = build_panel(&lake, &bcfg(None), 1.0e6).unwrap();
        assert_eq!(panel.dates, vec![d1.to_string()]);
        assert_eq!(panel.dropped_days, 1);
        assert_eq!(panel.arms.len(), 2); // cash + hsi (no map configured)

        // Missing dataset entirely ⇒ typed Insufficient, never a default.
        let empty = Lake::new(tmp("none"));
        assert!(matches!(
            build_panel(&empty, &bcfg(None), 1.0e6),
            Err(ValidateError::Insufficient(_))
        ));
        std::fs::remove_dir_all(root).ok();
    }
}
```

## `hkq-validate` — the SPA test

```rust
// crates/hkq-validate/src/spa.rs
//! Hansen's SPA test (§4; M9). Base model = THE STRATEGY, alternatives = the
//! null family — d_{k,t} = r_bench,k,t − r_strat,t, H₀: no family member has
//! superior predictive ability over the strategy. A SMALL p-value FALSIFIES
//! the strategy (some trivial alternative beats it, jointly accounting for
//! the family's multiplicity); a large p-value is the absence-of-dominance
//! verdict that complements DSR's positive evidence. SPA falsifies, DSR
//! promotes — the two arms of the §4 promotion protocol.
//!
//! Mechanics (Hansen 2005):
//! - t_k = √n·d̄_k/ω̂_k with ω̂² the stationary-bootstrap-implied long-run
//!   variance: ω̂² = γ̂₀ + 2Σκ(i)γ̂_i, κ(i) = ((n−i)/n)(1−q)^i + (i/n)(1−q)^{n−i}.
//!   At q = 1 this collapses to γ̂₀ exactly — the anchor unit test.
//! - T = max_k (t_k)₊, the studentized max statistic.
//! - Stationary bootstrap (Politis–Romano), ONE index path per replicate
//!   shared across arms — the max statistic is meaningless without cross-arm
//!   dependence. Deterministic xorshift64* PRNG (the M5 house rule: no rand
//!   dependency, reproducible reports).
//! - Consistent recentering (SPA_c): an arm with t_k < −√(2·ln ln n) is
//!   "deeply inferior" and recenters to zero, so benchmarks the strategy
//!   crushes cannot dilute the null distribution.
//!
//! Degenerate arms (zero long-run variance — e.g. a benchmark identical to
//! the strategy) carry no information: dropped and counted, never fabricated.
use crate::error::ValidateError;

/// Below this many aligned sessions the test refuses: an SPA verdict from a
/// handful of days is a prior wearing a costume (the M8 κ rule, applied here).
pub const SPA_MIN_DAYS: usize = 40;
pub const SPA_MIN_BOOT: usize = 200;
const OMEGA2_FLOOR: f64 = 1e-18;

#[derive(Debug, Clone)]
pub struct ArmStat {
    pub name: String,
    /// Mean daily edge of the BENCHMARK over the strategy, in bps
    /// (positive ⇒ the benchmark did better on average).
    pub mean_edge_bps: f64,
    /// √n·d̄/ω̂ — the studentized mean differential.
    pub t_marginal: f64,
    /// Marginal one-sided bootstrap p for "this benchmark beats the strategy".
    pub p_marginal: f64,
}

#[derive(Debug, Clone)]
pub struct SpaFit {
    /// T = max_k (t_k)₊. Zero ⇔ no benchmark even LOOKS better in-sample.
    pub t_stat: f64,
    /// Bootstrap tail P(T* ≥ T) under the recentered null. SMALL ⇒ falsified.
    pub p_value: f64,
    pub n_days: usize,
    pub n_boot: usize,
    pub q: f64,
    pub arms: Vec<ArmStat>,
    pub dropped_arms: usize,
}

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn uniform01(s: &mut u64) -> f64 {
    (xorshift(s) >> 11) as f64 / (1u64 << 53) as f64
}

/// γ̂_lag with the 1/n convention (Hansen's).
fn autocov(d: &[f64], mean: f64, lag: usize) -> f64 {
    let n = d.len();
    (0..n - lag).map(|t| (d[t] - mean) * (d[t + lag] - mean)).sum::<f64>() / n as f64
}

/// The stationary-bootstrap-implied long-run variance ω̂² (Hansen 2005).
pub fn kernel_omega2(d: &[f64], q: f64) -> f64 {
    let n = d.len();
    let nf = n as f64;
    let mean = d.iter().sum::<f64>() / nf;
    let p = 1.0 - q;
    let mut w = autocov(d, mean, 0);
    for i in 1..n {
        let k = ((nf - i as f64) / nf) * p.powi(i as i32)
            + (i as f64 / nf) * p.powi((n - i) as i32);
        if k != 0.0 {
            w += 2.0 * k * autocov(d, mean, i);
        }
    }
    w
}

struct Arm {
    name: String,
    d: Vec<f64>,
    mean: f64,
    omega: f64,
    t: f64,
    /// Recentering target under SPA_c.
    g: f64,
}

/// Ok(None) below the day/bootstrap floors or when every arm is degenerate —
/// refusal, not fabrication. Err on contract violations (empty family, length
/// mismatch). q is shared by the kernel variance and the bootstrap by design.
pub fn spa_test(
    strategy: &[f64],
    benchmarks: &[(String, Vec<f64>)],
    q: f64,
    n_boot: usize,
    seed: u64,
) -> Result<Option<SpaFit>, ValidateError> {
    if benchmarks.is_empty() {
        return Err(ValidateError::Contract("SPA needs at least one benchmark arm"));
    }
    for (_, b) in benchmarks {
        if b.len() != strategy.len() {
            return Err(ValidateError::Contract("benchmark series length mismatch"));
        }
    }
    let q = q.clamp(1e-3, 1.0);
    // Jointly finite sessions only (the panel builder guarantees this; the
    // core defends anyway — house rule: total functions).
    let keep: Vec<usize> = (0..strategy.len())
        .filter(|&t| strategy[t].is_finite()
            && benchmarks.iter().all(|(_, b)| b[t].is_finite()))
        .collect();
    let n = keep.len();
    if n < SPA_MIN_DAYS || n_boot < SPA_MIN_BOOT {
        return Ok(None);
    }
    let nf = n as f64;
    let sqn = nf.sqrt();
    let lo = -(2.0 * nf.ln().ln()).sqrt(); // SPA_c "deeply inferior" threshold

    let mut arms: Vec<Arm> = Vec::with_capacity(benchmarks.len());
    let mut dropped_arms = 0usize;
    for (name, b) in benchmarks {
        let d: Vec<f64> = keep.iter().map(|&t| b[t] - strategy[t]).collect();
        let mean = d.iter().sum::<f64>() / nf;
        let om2 = kernel_omega2(&d, q);
        if !(om2.is_finite() && om2 > OMEGA2_FLOOR) {
            dropped_arms += 1;
            tracing::warn!(arm = %name,
                "SPA arm degenerate (zero long-run variance) — dropped, not fabricated");
            continue;
        }
        let omega = om2.sqrt();
        let t = sqn * mean / omega;
        let g = if t >= lo { mean } else { 0.0 };
        arms.push(Arm { name: name.clone(), d, mean, omega, t, g });
    }
    if arms.is_empty() {
        return Ok(None);
    }
    let t_stat = arms.iter().fold(0.0f64, |acc, a| acc.max(a.t.max(0.0)));

    let mut s = seed | 1;
    let mut idx = vec![0usize; n];
    let mut ge_joint = 0usize;
    let mut ge_marg = vec![0usize; arms.len()];
    for _ in 0..n_boot {
        for t in 0..n {
            idx[t] = if t == 0 || uniform01(&mut s) < q {
                (xorshift(&mut s) % n as u64) as usize
            } else {
                (idx[t - 1] + 1) % n
            };
        }
        let mut t_star = 0.0f64;
        for (k, a) in arms.iter().enumerate() {
            let m = idx.iter().map(|&i| a.d[i]).sum::<f64>() / nf;
            let z = sqn * (m - a.g) / a.omega;
            if z >= a.t {
                ge_marg[k] += 1;
            }
            if z > t_star {
                t_star = z;
            }
        }
        if t_star >= t_stat {
            ge_joint += 1;
        }
    }
    let bf = n_boot as f64;
    let arms_out = arms.iter().enumerate()
        .map(|(k, a)| ArmStat {
            name: a.name.clone(),
            mean_edge_bps: a.mean * 1e4,
            t_marginal: a.t,
            p_marginal: ge_marg[k] as f64 / bf,
        })
        .collect();
    Ok(Some(SpaFit {
        t_stat,
        p_value: ge_joint as f64 / bf,
        n_days: n,
        n_boot,
        q,
        arms: arms_out,
        dropped_arms,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xs(s: &mut u64) -> f64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        ((*s >> 11) as f64 / (1u64 << 53) as f64) - 0.5
    }

    #[test]
    fn kernel_at_q1_is_gamma0_exactly() {
        let x: Vec<f64> = (0..100).map(|i| ((i * 37) % 11) as f64 - 5.0).collect();
        let n = x.len() as f64;
        let m = x.iter().sum::<f64>() / n;
        let g0 = x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / n;
        assert!((kernel_omega2(&x, 1.0) - g0).abs() < 1e-12);
    }

    #[test]
    fn dominated_strategy_is_rejected_and_deterministic() {
        let mut s = 0x9E3779B97F4A7C15u64;
        let n = 300;
        let strat: Vec<f64> = (0..n).map(|_| 0.002 * xs(&mut s)).collect();
        let better: Vec<f64> = strat.iter().map(|v| v + 0.001 + 0.0002 * xs(&mut s)).collect();
        let worse: Vec<f64> = strat.iter().map(|v| v - 0.001 + 0.0002 * xs(&mut s)).collect();
        let arms = vec![("better".to_string(), better), ("worse".to_string(), worse)];

        let fit = spa_test(&strat, &arms, 0.1, 500, 7).unwrap().unwrap();
        assert!(fit.t_stat > 3.0, "t {}", fit.t_stat);
        assert!(fit.p_value < 0.02, "p {}", fit.p_value);
        let b = fit.arms.iter().find(|a| a.name == "better").unwrap();
        assert!(b.p_marginal < 0.02 && b.mean_edge_bps > 5.0);
        let w = fit.arms.iter().find(|a| a.name == "worse").unwrap();
        assert!(w.p_marginal > 0.5); // no evidence the WORSE benchmark wins

        let fit2 = spa_test(&strat, &arms, 0.1, 500, 7).unwrap().unwrap();
        assert_eq!(fit.p_value, fit2.p_value); // same seed ⇒ same verdict
    }

    #[test]
    fn dominant_strategy_yields_t_zero_p_one() {
        let mut s = 0xDEADBEEFCAFEBABEu64;
        let n = 200;
        let strat: Vec<f64> = (0..n).map(|_| 0.001 + 0.002 * xs(&mut s)).collect();
        let cash = vec![0.0; n];
        let fit = spa_test(&strat, &[("always_cash".into(), cash)], 0.1, 300, 42)
            .unwrap().unwrap();
        assert_eq!(fit.t_stat, 0.0);
        assert!((fit.p_value - 1.0).abs() < 1e-12); // nothing comes close
    }

    #[test]
    fn degenerate_arms_and_floors_refuse() {
        let strat: Vec<f64> = (0..100).map(|i| (i % 7) as f64 * 1e-4).collect();
        // Identical twin ⇒ d ≡ 0 ⇒ zero variance ⇒ dropped ⇒ None.
        assert!(spa_test(&strat, &[("twin".into(), strat.clone())], 0.1, 300, 1)
            .unwrap().is_none());
        // Day floor and bootstrap floor.
        let cash = vec![0.0; 100];
        assert!(spa_test(&strat[..10], &[("c".into(), cash[..10].to_vec())], 0.1, 300, 1)
            .unwrap().is_none());
        assert!(spa_test(&strat, &[("c".into(), cash.clone())], 0.1, 10, 1)
            .unwrap().is_none());
        // Contract violations are loud.
        assert!(spa_test(&strat, &[], 0.1, 300, 1).is_err());
        assert!(spa_test(&strat, &[("c".into(), vec![0.0; 5])], 0.1, 300, 1).is_err());
    }
}
```

## `hkq-validate` — updated shell files

```rust
// crates/hkq-validate/src/lib.rs
#![forbid(unsafe_code)]
//! The §4 protocol as a crate, not a notebook: purged splits, NW t-stats, DSR
//! against an honest trials registry, the CUSUM kill producer, the quarterly
//! estimation jobs, the promotion protocol proper (M8: as-of reconstruction +
//! κ calibration) — and, as of M9, the FALSIFICATION arm: the Hansen SPA test
//! against the report's null family {always-cash, HSI open→close, sector-ETF
//! Stage-1}, built from benchmark series the nightly job now ingests.
//!
//! Design invariants:
//! - Every statistic is a pure function of frames/slices; the ONLY I/O is the
//!   lake (read), the `_state` directory (fit artifacts, CUSUM state), and the
//!   hash-chained trials registry (append).
//! - Monitoring, reconstruction, and benchmarking consume the SAME persisted
//!   artifacts the learning loops train on: no recomputed morning factors, no
//!   fabricated ICs, no imputed fills or benchmark returns — the M3/M4
//!   honesty rule, everywhere.
//! - Degradation is typed: missing history ⇒ `Insufficient` (callers continue,
//!   loudly); schema drift ⇒ polars errors (nobody continues).
//! - The CUSUM breach LATCHES. Un-halting is an operator edit of the state
//!   file, never code. That is what "pre-registered kill threshold" means.
//! - Scalar promotions (θ, v*, κ) are OPERATOR config edits. Jobs report and
//!   registry-log; they never mutate strategy.toml or production `_state`.
//! - SPA falsifies, DSR promotes: d = benchmark − strategy, so a SMALL SPA p
//!   means a null-family member beats the strategy. Both verdicts are
//!   registry-logged; neither automates a promotion.

pub mod asof;
pub mod benchmarks;
pub mod cfg;
pub mod cusum;
pub mod dsr;
pub mod error;
pub mod fits;
pub mod ic;
pub mod kappa;
pub mod pnl;
pub mod registry;
pub mod spa;
pub mod splits;
pub mod stats;

pub use asof::{materialize_asof_state, AsofReport};
pub use benchmarks::{build_panel, load_sector_etf_map, BenchmarkPanel};
pub use cfg::{load_validate, ValidateCfg};
pub use cusum::{startup_gate, CusumOutcome, CusumParams, CusumState};
pub use dsr::{deflated_sharpe, expected_max_sharpe};
pub use error::ValidateError;
pub use kappa::{fit_kappa, kappa_panel, KappaFit};
pub use registry::TrialsRegistry;
pub use spa::{spa_test, ArmStat, SpaFit};
pub use splits::{purged_walk_forward, Split};
```

```rust
// crates/hkq-validate/src/main.rs
//! §4 protocol jobs. Deliberately synchronous — pure batch over the lake.
//!
//! Usage:
//!   hkq-validate <strategy.toml> cusum
//!   hkq-validate <strategy.toml> fit-quarterly [YYYY-MM-DD]
//!   hkq-validate <strategy.toml> report
//!   hkq-validate <strategy.toml> asof-state [YYYY-MM-DD]
//!   hkq-validate <strategy.toml> fit-kappa
//!   hkq-validate <strategy.toml> spa --equity <HKD> [--boot N] [--seed N] [--q F]   (M9)
//!
//! `cusum` exits non-zero on a latched breach so cron/alerting notices; the
//! authoritative runtime producer is hkq-live's startup gate, not this job.
//! `spa` reads Fills/Scores AND Benchmarks from the SAME lake root — for
//! sandbox evaluation, ensure benchmark partitions exist in that root too.
use anyhow::{bail, Context};
use chrono::{Duration, NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use hkq_core::config::StrategyCfg;
use hkq_data::cfg::load_benchmarks;
use hkq_data::lake::Lake;
use hkq_validate::cfg::{load_validate, ValidateCfg};
use hkq_validate::registry::{sha1_hex_of_file, TrialsRegistry};
use hkq_validate::splits::purged_walk_forward;
use hkq_validate::{asof, benchmarks, cusum, dsr, fits, kappa, pnl, spa, stats};
use serde_json::json;
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("info".parse()?))
        .init();

    let mut args = std::env::args().skip(1);
    let cfg_path = args.next().context(
        "usage: hkq-validate <strategy.toml> \
         <cusum|fit-quarterly|report|asof-state|fit-kappa|spa> [args]")?;
    let cmd = args.next().context(
        "missing subcommand: cusum | fit-quarterly | report | asof-state | fit-kappa | spa")?;
    let rest: Vec<String> = args.collect();

    let cfg = StrategyCfg::load(&cfg_path)?;
    let vcfg = load_validate(&cfg_path)?;
    let lake = Lake::new(&cfg.ops.lake_root);

    let date_arg = |rest: &[String]| -> anyhow::Result<Option<NaiveDate>> {
        rest.first()
            .map(|s| s.parse::<NaiveDate>())
            .transpose()
            .context("date must be YYYY-MM-DD")
    };

    match cmd.as_str() {
        "cusum" => job_cusum(&lake, &vcfg),
        "fit-quarterly" => job_fit_quarterly(&cfg, &cfg_path, &vcfg, &lake, date_arg(&rest)?),
        "report" => job_report(&vcfg, &lake),
        "asof-state" => job_asof_state(&vcfg, &lake, date_arg(&rest)?),
        "fit-kappa" => job_fit_kappa(&cfg, &cfg_path, &vcfg, &lake),
        "spa" => job_spa(&cfg_path, &vcfg, &lake, &rest),
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
        "note": "DSR is None until ≥2 registered trials carry an `sr` metric — record your trials. Promotion-grade trials are kind=walkforward (hkq-backtest --asof). Run the `spa` job for the falsification arm.",
    });
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

/// M8: audit materialization of one date's reconstructed state.
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

/// M9: the §4 falsification arm. d = benchmark − strategy, so a SMALL p means
/// some null-family member beats the strategy. Reports + registry-logs; the
/// record carries NO `sr` metric, so DSR's N is untouched by design.
fn job_spa(
    cfg_path: &str, vcfg: &ValidateCfg, lake: &Lake, rest: &[String],
) -> anyhow::Result<()> {
    let mut equity: Option<f64> = None;
    let mut n_boot = 2000usize;
    let mut seed = 20260704u64;
    let mut q = 0.1f64;
    let mut it = rest.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--equity" => equity = Some(it.next().context("--equity needs a value")?
                .parse().context("equity must be a number (HKD)")?),
            "--boot" => n_boot = it.next().context("--boot needs a value")?
                .parse().context("--boot must be an integer")?,
            "--seed" => seed = it.next().context("--seed needs a value")?
                .parse().context("--seed must be an integer")?,
            "--q" => q = it.next().context("--q needs a value")?
                .parse().context("--q must be a float in (0, 1]")?,
            other => bail!("unknown spa argument '{other}'"),
        }
    }
    let equity = equity.context(
        "--equity <HKD> is required (return normalization — statistics-side, like the report's SR)")?;
    anyhow::ensure!(equity.is_finite() && equity > 0.0, "equity must be positive");

    let bcfg = load_benchmarks(cfg_path)?
        .context("[benchmarks] table is required for the SPA job")?;
    let panel = benchmarks::build_panel(lake, &bcfg, equity)?;
    tracing::info!(days = panel.dates.len(), arms = panel.arms.len(),
        dropped = panel.dropped_days, missing_bench = panel.missing_benchmark_days,
        flat = panel.flat_days, "benchmark panel assembled");

    match spa::spa_test(&panel.strategy, &panel.arms, q, n_boot, seed)? {
        Some(fit) => {
            let out = json!({
                "t_stat": fit.t_stat,
                "p_value": fit.p_value,
                "n_days": fit.n_days,
                "n_boot": fit.n_boot,
                "q": fit.q,
                "dropped_arms": fit.dropped_arms,
                "dropped_days": panel.dropped_days,
                "missing_benchmark_days": panel.missing_benchmark_days,
                "flat_days": panel.flat_days,
                "arms": fit.arms.iter().map(|a| json!({
                    "name": a.name,
                    "mean_edge_bps": a.mean_edge_bps,
                    "t": a.t_marginal,
                    "p": a.p_marginal,
                })).collect::<Vec<_>>(),
                "spa_not_falsified_at_5pct": fit.p_value >= 0.05,
                "note": "SPA is the FALSIFICATION arm of §4: d = benchmark − strategy, so a SMALL p means a null-family member (always-cash / HSI proxy / sector-ETF Stage-1) beats the strategy. DSR remains the promotion arm. Run against a lake holding Fills/Scores AND Benchmarks.",
            });
            println!("{}", serde_json::to_string_pretty(&out)?);

            let reg = TrialsRegistry::open(vcfg.registry_path(lake.root()));
            let mut m = BTreeMap::new();
            m.insert("spa_t".to_string(), fit.t_stat);
            m.insert("spa_p".to_string(), fit.p_value);
            m.insert("n_days".to_string(), fit.n_days as f64);
            for a in &fit.arms {
                m.insert(format!("p_{}", a.name), a.p_marginal);
            }
            reg.append("spa_test", &sha1_hex_of_file(cfg_path)?, &m,
                       "Hansen SPA vs {always-cash, HSI open→close, sector-ETF Stage-1}")?;
        }
        None => tracing::warn!(days = panel.dates.len(), floor = spa::SPA_MIN_DAYS,
            "SPA refused: too few aligned sessions or every arm degenerate — keep accruing benchmark history"),
    }
    Ok(())
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
serde.workspace = true
serde_json.workspace = true
toml.workspace = true
polars.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
```

```rust
// crates/hkq-recon/src/main.rs
//! Nightly close reconciliation (blueprint bin #4; report §5: "reconciled
//! nightly against EOD official closes"). Closes the M1 gap that has been
//! logged loudly every night since: reconciling Tiger against Tiger is
//! circular, so this binary fetches closes from an INDEPENDENT config-templated
//! source and compares them tick-aware via hkq-data's reconcile_closes.
//! Usage: hkq-recon <strategy.toml> [YYYY-MM-DD]     (run AFTER hkq-nightly)
//!
//! Built with ZERO patches to frozen crates — everything rides public seams:
//! RatedClient (rate-limited HTTP), reconcile_closes + Lake::quarantine (M1,
//! built and tested), TrialsRegistry (M5, the audit trail).
//!
//! Failure semantics, by design:
//! - |Δ| > 1 tick on any name ⇒ the daily_bars partition is QUARANTINED and
//!   the process exits non-zero (cron pages the operator). Quarantine is the
//!   stand-down mechanism: the next PreMarket fails loudly on its own
//!   missing-input contract — no new halt state to rot.
//! - Zero parseable rows from the official source ⇒ refuse to reconcile
//!   (schema drift is never a clean bill of health).
//! - Every run appends recon_ok/recon_breach to the hash-chained registry.
//!   Those records carry no `sr` metric, so DSR's N is untouched.
//!
//! Independence is config-deep: the default template points at a DIFFERENT
//! vendor than the ingest path, and the required-config template makes
//! Tiger-vs-Tiger structurally impossible. When a licensed HKEX EOD file
//! becomes available, swap the template + schema_version — the seam holds.
use anyhow::{bail, Context};
use chrono::{NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg, ids::StockCode,
               session::{DayKind, TradingCalendar}};
use hkq_data::{http::RatedClient, lake::{Dataset, Lake}, recon::reconcile_closes};
use hkq_validate::cfg::load_validate;
use hkq_validate::registry::{sha1_hex_of_file, TrialsRegistry};
use polars::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

fn d_prefix() -> String { "116.".into() }   // VERIFY: EastMoney HK market prefix
fn d_code_field() -> String { "f12".into() }
fn d_close_field() -> String { "f2".into() }
fn d_schema() -> u32 { 1 }
fn d_rps() -> u32 { 2 }
fn d_retries() -> u32 { 3 }
fn d_batch() -> usize { 80 }

/// `[recon]` table of strategy.toml — owned by this binary (the M1 discipline:
/// undocumented endpoints live in config, parsers are schema-versioned, drift
/// fails loudly).
#[derive(Debug, Clone, Deserialize)]
struct ReconCfg {
    /// Batch quote URL template; `{secids}` substituted. Must request float
    /// formatting (e.g. fltt=2) and the code/close fields configured below.
    official_close_url_template: String,
    #[serde(default = "d_prefix")]
    hk_secid_prefix: String,
    #[serde(default = "d_code_field")]
    code_field: String,        // VERIFY per source
    #[serde(default = "d_close_field")]
    close_field: String,       // VERIFY per source
    #[serde(default = "d_schema")]
    schema_version: u32,
    #[serde(default = "d_rps")]
    rps: u32,
    #[serde(default = "d_retries")]
    max_retries: u32,
    #[serde(default = "d_batch")]
    batch_size: usize,
}

#[derive(Debug, Deserialize)]
struct ReconFile {
    recon: Option<ReconCfg>,
}

/// Schema v1: push2-ulist shape { data: { diff: [ { <code_field>, <close_field> } ] } }.
fn parse_official_v1(v: &Value, code_field: &str, close_field: &str) -> Vec<(u32, f64)> {
    let Some(diff) = v.pointer("/data/diff").and_then(Value::as_array) else {
        return vec![];
    };
    diff.iter()
        .filter_map(|item| {
            let code = item.get(code_field).and_then(Value::as_str).and_then(StockCode::parse)?;
            let close = item.get(close_field).and_then(Value::as_f64)
                .filter(|c| c.is_finite() && *c > 0.0)?;
            Some((code.0, close))
        })
        .collect()
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
        .map(|s| s.parse().context("date must be YYYY-MM-DD"))
        .transpose()?;

    let cfg = StrategyCfg::load(&cfg_path)?;
    let raw = std::fs::read_to_string(&cfg_path)
        .with_context(|| format!("reading {cfg_path}"))?;
    let rcfg = toml::from_str::<ReconFile>(&raw)
        .with_context(|| format!("parsing {cfg_path}"))?
        .recon
        .context("[recon] table is required: an INDEPENDENT official-close source (the M1 gap this binary closes)")?;

    let calendar = FileCalendar::load(&cfg.ops.calendar_path)?;
    let date = date.unwrap_or_else(|| Utc::now().with_timezone(&Hong_Kong).date_naive());
    if calendar.day_kind(date) == DayKind::Closed {
        tracing::info!(%date, "market closed; nothing to reconcile");
        return Ok(());
    }

    let lake = Lake::new(&cfg.ops.lake_root);
    // Reconcile exactly what we stored — the code list COMES FROM the partition
    // under test, so missing_ours is empty by construction.
    let ours = lake.scan_date(Dataset::DailyBars, date)
        .context("daily_bars partition missing — run hkq-nightly before hkq-recon")?
        .select([col("code")])
        .collect()?;
    let codes: Vec<u32> = ours.column("code")?
        .as_materialized_series().u32()?
        .into_no_null_iter()
        .collect();
    anyhow::ensure!(!codes.is_empty(), "daily_bars partition for {date} has no rows");

    let http = RatedClient::new(rcfg.rps, rcfg.max_retries);
    let mut official: BTreeMap<u32, f64> = BTreeMap::new();
    for chunk in codes.chunks(rcfg.batch_size.max(1)) {
        let secids: Vec<String> = chunk.iter()
            .map(|c| format!("{}{}", rcfg.hk_secid_prefix, StockCode(*c)))
            .collect();
        let url = rcfg.official_close_url_template.replace("{secids}", &secids.join(","));
        match rcfg.schema_version {
            1 => {
                let v: Value = http.get_json(&url, &[]).await?;
                for (c, px) in parse_official_v1(&v, &rcfg.code_field, &rcfg.close_field) {
                    official.insert(c, px);
                }
            }
            other => bail!("unknown [recon] schema_version {other} — bump the parser, not the hope"),
        }
    }
    if official.is_empty() {
        bail!("official-close source returned ZERO parseable rows (schema drift?) — refusing to reconcile");
    }
    tracing::info!(requested = codes.len(), received = official.len(), %date,
        "official closes fetched from the independent source");

    let (ocodes, ocloses): (Vec<u32>, Vec<f64>) = official.into_iter().unzip();
    let official_df = df!("code" => ocodes, "close_official" => ocloses)?;
    let report = reconcile_closes(&lake, &official_df, date)?;

    if !report.missing_ours.is_empty() {
        // Structurally impossible (we queried our own codes) — if it fires, the
        // source echoed codes we never asked for. Loud, not fatal.
        tracing::warn!(n = report.missing_ours.len(),
            "official source returned codes absent from our partition (echo drift?)");
    }
    if !report.missing_official.is_empty() {
        tracing::warn!(n = report.missing_official.len(),
            "names we stored but the official source did not return (suspensions / vendor gap) — unverified, not breached");
    }

    let vcfg = load_validate(&cfg_path)?;
    let reg = TrialsRegistry::open(vcfg.registry_path(lake.root()));
    let mut m = BTreeMap::new();
    m.insert("checked".to_string(), report.checked as f64);
    m.insert("breaches".to_string(), report.breaches.len() as f64);
    m.insert("missing_official".to_string(), report.missing_official.len() as f64);

    if report.breaches.is_empty() {
        if let Err(e) = reg.append("recon_ok", &sha1_hex_of_file(&cfg_path)?, &m,
                                   &format!("close reconciliation clean for {date}")) {
            tracing::warn!(error = %e, "registry append failed (recon still clean)");
        }
        tracing::info!(checked = report.checked, %date, "reconciliation CLEAN");
        Ok(())
    } else {
        for b in &report.breaches {
            tracing::error!(code = b.code, ours = b.ours, official = b.official,
                diff = %b.abs_diff, tick = %b.tick, "official close mismatch > 1 tick");
        }
        if let Err(e) = reg.append("recon_breach", &sha1_hex_of_file(&cfg_path)?, &m,
                                   &format!("close reconciliation BREACH for {date} — partition quarantined")) {
            tracing::warn!(error = %e, "registry append failed (breach still handled)");
        }
        let dst = lake.quarantine(Dataset::DailyBars, date)?;
        bail!(
            "recon breach: {} names off by > 1 tick; daily_bars/date={date} QUARANTINED to {} — \
             the next PreMarket will fail loudly on its missing-input contract until the operator re-ingests",
            report.breaches.len(), dst.display()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_parser_v1_uses_config_fields_and_skips_garbage() {
        let v: Value = serde_json::json!({ "data": { "diff": [
            { "f12": "00700", "f2": 321.4 },
            { "f12": "00005", "f2": -1.0 },          // non-positive ⇒ skipped
            { "f12": "junk!", "f2": 10.0 },          // unparseable code ⇒ skipped
            { "f12": "00941", "other": 1.0 }         // missing close ⇒ skipped
        ]}});
        let rows = parse_official_v1(&v, "f12", "f2");
        assert_eq!(rows, vec![(700, 321.4)]);
        // Absent /data/diff ⇒ empty, never a panic — the caller treats empty as drift.
        assert!(parse_official_v1(&serde_json::json!({}), "f12", "f2").is_empty());
    }
}
```

## Config additions

```toml
# strategy.toml — M9 tables (both opt-in; every VERIFY item is data, not code)

[benchmarks]
codes = [2800, 3033]                       # HK-listed instruments, ingested nightly
hsi_code = 2800                            # HSI open→close proxy (Tracker Fund) — §4 null #2
sector_etf_map_path = "config/sector_etf_map.csv"   # §4 null #3

[recon]
official_close_url_template = "https://push2.eastmoney.com/api/qt/ulist.np/get?fltt=2&fields=f2,f12&secids={secids}"
# hk_secid_prefix = "116."     (VERIFY: EastMoney HK market prefix)
# code_field  = "f12"          (VERIFY)
# close_field = "f2"           (VERIFY: latest == official close after hours)
# schema_version = 1
```

```csv
# config/sector_etf_map.csv — sector_id,etf_code (both HK-listed; list every code in [benchmarks].codes)
1,3033
```

## Honest gaps and hand-off to Milestone 10

Seven items, each now a named fact. First, the §3.7 learned layer — with CPCV and the self-hosted walk-forward's weights-source seam — is now genuinely the last major code milestone, and it inherits a *complete* evidence pipeline: as-of trials feed DSR, the SPA referee falsifies against the family, and every verdict is registry-chained; the ML layer's many trials will land in gates that already exist. Second, the ETF-Stage1 arm prices open→close while the strategy trades 09:45→15:45; a timing-faithful arm needs ETF minute bars nobody ingests — open→close is the *conservative* choice (it grants the benchmark the full session), documented here as a named refinement, not a bias swept under the rug. Third, benchmark history starts accruing *today*, which means the SPA job refuses (`SPA_MIN_DAYS = 40`) until roughly two months of sessions exist — the M4 auction-history pattern: write early so the consumer matures on schedule. Fourth, sandbox SPA needs Benchmarks partitions in the sandbox root (a copy, or a future two-root flag when a consumer demands it); the primary §4 target — the production shadow lake — works as-is. Fifth, recon independence is config-deep, not cryptographic: the default template is a second vendor with VERIFY markers, and the licensed HKEX EOD file remains the gold source — swapping it in is a template + `schema_version` edit behind the same parser seam. Sixth, VHSI and A50 ingestion, the Fills venue tag, and quote/VCM persistence are unchanged deferrals with unchanged owners; `vhsi_tercile` remains `None` by type. Seventh, promotion enforcement remains operator governance, by design — the `spa` job states its verdict and logs it; nothing edits config. The system now has facts, opinions, decisions, a clock, a memory, a reflex, a hand, a mirror, judgment — and a referee: every §4 statistic, both the promoting kind and the falsifying kind, is computable from data the machine writes daily, against a price ledger it finally reconciles independently every night. What it still lacks is the learned layer that would consume all of it — and that layer now arrives to find the field fully lined, the whistle already in hand.
