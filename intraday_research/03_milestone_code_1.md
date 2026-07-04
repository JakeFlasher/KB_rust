 
The documents are already attached in full in this conversation (and I don't have a separate URL-fetch tool — only search, which isn't needed here), so I worked directly from the attached `01_report.md` and `02_framework_proposal.md`, exactly as the blueprint itself did.

# Milestone 1 — Core Data Layer & Ingestion Pipeline

**In scope:** the `hkq-core` domain substrate (ids, money newtypes, session clock, calendar, config, errors), the complete `hkq-data` crate (models, capability traits, rate-limited HTTP substrate, Tiger/EastMoney/Xueqiu/Yahoo adapters, parquet lake with atomic idempotent writes, close reconciliation, nightly orchestrator, live event multiplexer), and a thin `hkq-nightly` binary. **Deferred:** `hkq-factors`/`-signal`/`-risk`/`-exec`/`-validate`/`-engine`, all polars factor transforms, and all sizing *logic* (the `LotQty` domain type ships now because it's part of the core type contract; `size_book` does not).

Engineering decisions that go beyond the blueprint sketch, briefly: session times use a `LazyLock` struct rather than `const` items (avoids gambling on `const Option::unwrap`/const-chrono across toolchains); parquet files store instants as `i64` epoch-milliseconds plus an ISO `date` string column with hive inference *disabled* — deterministic schemas beat partition-pruning magic at this scale, and `scan_date` gives the fast path; `TigerClient` holds an `Arc` inner so polling tasks are spawnable, the RSA-SHA1 signing is fully implemented, and `subscribe_pos` probes the payload once and returns `DataError::Unsupported` if no auction fields exist — the engine's documented X2-disabled fallback (§5); undocumented EastMoney endpoints live behind config URL templates with schema-versioned parsers so drift fails loudly; the 1-minute bar poller emits only *completed* bars via a per-symbol watermark; lake writes are tmp-file + fsync + atomic-rename with a `_manifest.json` tripwire, and reconciliation is tick-aware per the report's "|Δ| > 1 tick ⇒ quarantine" rule.

```text
hkq/
├── Cargo.toml
├── config/  (strategy.toml, calendar_hk.json, universe_codes.txt, ah_map.csv)
└── crates/
    ├── hkq-core/src/{lib,error,ids,money,session,calendar,config}.rs
    ├── hkq-data/src/{lib,error,cfg,model,provider,http,tiger,eastmoney,xueqiu,yahoo,lake,recon,ingest}.rs
    └── hkq-nightly/src/main.rs
```

## Workspace

```toml
# Cargo.toml
[workspace]
resolver = "2"
members = ["crates/hkq-core", "crates/hkq-data", "crates/hkq-nightly"]

[workspace.package]
edition = "2021"
rust-version = "1.83"

[workspace.dependencies]
tokio        = { version = "1.38", features = ["full"] }
tokio-stream = "0.1"
reqwest      = { version = "0.12", default-features = false, features = ["json", "gzip", "rustls-tls"] }
serde        = { version = "1", features = ["derive"] }
serde_json   = "1"
# Milestone-1 features only; the factor engine (M2) extends this set additively.
polars       = { version = "0.46", features = ["lazy", "parquet", "dtype-date", "dtype-datetime"] }
rust_decimal        = { version = "1.35", features = ["serde-str"] }
rust_decimal_macros = "1.35"
chrono       = { version = "0.4", features = ["serde"] }
chrono-tz    = "0.9"
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

Pin note (per blueprint): everything targets polars 0.46; if you float the pin, treat `ScanArgsParquet`/`Column` accessor renames as mechanical.

## `hkq-core`

```toml
# crates/hkq-core/Cargo.toml
[package]
name = "hkq-core"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
toml.workspace = true
chrono.workspace = true
chrono-tz.workspace = true
rust_decimal.workspace = true
rust_decimal_macros.workspace = true
thiserror.workspace = true
```

```rust
// crates/hkq-core/src/lib.rs
#![forbid(unsafe_code)]
//! Domain substrate shared by every HKQ crate: identifiers, money newtypes,
//! the HKT session clock (report §1/§6), trading calendar, config, errors.

pub mod calendar;
pub mod config;
pub mod error;
pub mod ids;
pub mod money;
pub mod session;
```

```rust
// crates/hkq-core/src/error.rs
use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("non-positive price: {0}")]
    NonPositivePrice(Decimal),
    #[error("float not representable as Decimal: {0}")]
    BadFloat(f64),
    #[error("invalid intraday bin: {0}")]
    BadBin(u8),
    #[error("config error: {0}")]
    Config(String),
    #[error("calendar error: {0}")]
    Calendar(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

```rust
// crates/hkq-core/src/ids.rs
use serde::{Deserialize, Serialize};

/// HKEX numeric stock code (e.g. 700 → "00700"). Copy, hashable, order-stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StockCode(pub u32);

impl StockCode {
    /// Parse "700", "00700", or "00700.HK" (universe files, vendor payloads).
    pub fn parse(s: &str) -> Option<Self> {
        let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() { return None; }
        digits.parse::<u32>().ok().map(StockCode)
    }
}

impl std::fmt::Display for StockCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:05}", self.0)
    }
}

/// Sector id: HSIC level-2 code, or a Louvain community id in cluster mode (§3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SectorId(pub u16);

/// Intraday 30-minute bin τ ∈ 1..=11; half-days collapse to τ ≤ 5 (§1, §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bin(u8);

impl Bin {
    pub fn new(tau: u8) -> Option<Self> { (1..=11).contains(&tau).then_some(Bin(tau)) }
    pub fn tau(self) -> u8 { self.0 }
}
```

```rust
// crates/hkq-core/src/money.rs
//! Accounting math is `Decimal` behind newtypes; `f64` never touches an order.
//! Statistical math stays f64 — the boundary conversion is explicit and singular.
use crate::error::CoreError;
use rust_decimal::{prelude::*, Decimal, RoundingStrategy};
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

/// Price in HKD. Positive by construction. Tick validity enforced at order build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Px(Decimal);

impl Px {
    pub fn new(v: Decimal) -> Result<Self, CoreError> {
        (v > Decimal::ZERO).then_some(Px(v)).ok_or(CoreError::NonPositivePrice(v))
    }

    /// Vendor floats are quantized to 4 dp (finer than any HK tick) at the boundary,
    /// banker's rounding. This is the ONLY place a float becomes a price.
    pub fn from_f64_quote(v: f64) -> Result<Self, CoreError> {
        let d = Decimal::from_f64(v).ok_or(CoreError::BadFloat(v))?;
        Self::new(d.round_dp_with_strategy(4, RoundingStrategy::MidpointNearestEven))
    }

    pub fn get(self) -> Decimal { self.0 }
    /// Signal side only — never for accounting.
    pub fn as_f64(self) -> f64 { self.0.to_f64().expect("Decimal→f64 is total") }

    /// HKEX securities spread table: snap DOWN to a valid tick (stops/limits).
    pub fn snap_down_to_tick(self) -> Px {
        let t = hk_tick_size(self.0);
        Px((self.0 / t).floor() * t)
    }
}

/// HKEX price-dependent minimum spread table (Part A, equities).
pub fn hk_tick_size(px: Decimal) -> Decimal {
    let table: [(Decimal, Decimal); 11] = [
        (dec!(0.25), dec!(0.001)), (dec!(0.50), dec!(0.005)), (dec!(10.00), dec!(0.010)),
        (dec!(20.00), dec!(0.020)), (dec!(100.00), dec!(0.050)), (dec!(200.00), dec!(0.100)),
        (dec!(500.00), dec!(0.200)), (dec!(1000.00), dec!(0.500)), (dec!(2000.00), dec!(1.000)),
        (dec!(5000.00), dec!(2.000)), (dec!(9995.00), dec!(5.000)),
    ];
    table.iter().find(|(ub, _)| px <= *ub).map(|(_, t)| *t).unwrap_or(dec!(5.000))
}

/// Signed HKD cash. All portfolio accounting flows through this type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct Cash(pub Decimal);

/// Board lot size for a listing (odd lots trade on a separate board).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardLot(pub u32);

/// Share quantity PROVEN to be a whole number of board lots — the only quantity
/// type the (future) exec crate accepts; odd-lot orders are unrepresentable.
/// Sizing *logic* (`size_book`) is Milestone 3; the invariant type ships now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LotQty { shares: u64, lot: u32 }

impl LotQty {
    /// floor(target_cash / (px · lot)) lots, all in Decimal. None if < 1 lot.
    pub fn floor_from_cash(target: Cash, px: Px, lot: BoardLot) -> Option<LotQty> {
        let lot_notional = px.get() * Decimal::from(lot.0);
        if lot_notional <= Decimal::ZERO { return None; }
        let n_lots = (target.0 / lot_notional).floor().to_u64()?;
        (n_lots > 0).then_some(LotQty { shares: n_lots * lot.0 as u64, lot: lot.0 })
    }
    pub fn shares(self) -> u64 { self.shares }
    pub fn lot(self) -> u32 { self.lot }
    pub fn cap_shares(self, max_shares: u64) -> Option<LotQty> {
        let n_lots = (max_shares / self.lot as u64).min(self.shares / self.lot as u64);
        (n_lots > 0).then_some(LotQty { shares: n_lots * self.lot as u64, lot: self.lot })
    }
    pub fn notional(self, px: Px) -> Cash { Cash(px.get() * Decimal::from(self.shares)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_table_bands() {
        assert_eq!(hk_tick_size(dec!(0.24)), dec!(0.001));
        assert_eq!(hk_tick_size(dec!(9.99)), dec!(0.010));
        assert_eq!(hk_tick_size(dec!(150.0)), dec!(0.100));
        assert_eq!(hk_tick_size(dec!(12000.0)), dec!(5.000));
    }

    #[test]
    fn lot_floor_and_cap() {
        let px = Px::from_f64_quote(10.0).unwrap();
        let q = LotQty::floor_from_cash(Cash(dec!(12000)), px, BoardLot(500)).unwrap();
        assert_eq!(q.shares(), 1000); // 2 lots of 500
        assert_eq!(q.cap_shares(700).unwrap().shares(), 500);
        assert!(q.cap_shares(499).is_none());
    }

    #[test]
    fn snap_down() {
        let p = Px::from_f64_quote(9.994).unwrap().snap_down_to_tick();
        assert_eq!(p.get(), dec!(9.99));
    }
}
```

```rust
// crates/hkq-core/src/session.rs
//! The report's clock (§1, §6). HKT, no DST. All engine phase changes key off these.
use crate::ids::Bin;
use chrono::{DateTime, NaiveDate, NaiveTime};
use chrono_tz::{Asia::Hong_Kong, Tz};
use std::sync::LazyLock;

/// Named runbook boundaries. A `LazyLock` struct instead of `const` items: chrono
/// const-constructors + const `unwrap` are toolchain gambles; this is not.
#[derive(Debug, Clone, Copy)]
pub struct SessionTimes {
    pub pos_start: NaiveTime,      // 09:00 POS begins
    pub pos_no_cancel: NaiveTime,  // 09:15 no-cancellation regime
    pub pos_match_from: NaiveTime, // 09:20 random matching window opens
    pub pos_match_to: NaiveTime,   // 09:22 matching complete
    pub mainland_print: NaiveTime, // 09:25 SSE/SZSE auction print (S6)
    pub score_freeze: NaiveTime,   // 09:29:30 OpenContext freeze
    pub open: NaiveTime,           // 09:30 continuous session
    pub x3_refresh: NaiveTime,     // 09:35 X3 two-phase refresh
    pub entry: NaiveTime,          // 09:45 entry decision
    pub lunch_cancel: NaiveTime,   // 11:58 cancel resting orders
    pub am_close: NaiveTime,       // 12:00
    pub pm_open: NaiveTime,        // 13:00
    pub exit_start: NaiveTime,     // 15:30 exit program begins
    pub exit_end: NaiveTime,       // 15:45
    pub cont_close: NaiveTime,     // 16:00
    pub cas_hard_end: NaiveTime,   // 16:10 CAS residual deadline
    pub half_day_exit: NaiveTime,  // 11:45 compressed-exit start on half-days
}

static TIMES: LazyLock<SessionTimes> = LazyLock::new(|| {
    let t = |h, m, s| NaiveTime::from_hms_opt(h, m, s).expect("valid HKT time");
    SessionTimes {
        pos_start: t(9, 0, 0),      pos_no_cancel: t(9, 15, 0),
        pos_match_from: t(9, 20, 0), pos_match_to: t(9, 22, 0),
        mainland_print: t(9, 25, 0), score_freeze: t(9, 29, 30),
        open: t(9, 30, 0),           x3_refresh: t(9, 35, 0),
        entry: t(9, 45, 0),          lunch_cancel: t(11, 58, 0),
        am_close: t(12, 0, 0),       pm_open: t(13, 0, 0),
        exit_start: t(15, 30, 0),    exit_end: t(15, 45, 0),
        cont_close: t(16, 0, 0),     cas_hard_end: t(16, 10, 0),
        half_day_exit: t(11, 45, 0),
    }
});

impl SessionTimes {
    pub fn get() -> &'static SessionTimes { &TIMES }
}

/// Localize a (date, time) to the exchange clock. HKT has no DST ⇒ always unique.
pub fn hk(date: NaiveDate, time: NaiveTime) -> DateTime<Tz> {
    date.and_time(time)
        .and_local_timezone(Hong_Kong)
        .single()
        .expect("HKT has no DST gaps")
}

/// Map an HKT clock time to bin τ ∈ 1..=11 (lunch discontinuity between τ=5 and τ=6).
pub fn bin_of(ts_hkt: NaiveTime) -> Option<Bin> {
    let s = SessionTimes::get();
    let m = ts_hkt.signed_duration_since(s.open).num_minutes();
    let tau = if (0..150).contains(&m) {
        1 + (m / 30) as u8 // 09:30–12:00 → τ 1..=5
    } else {
        let pm = ts_hkt.signed_duration_since(s.pm_open).num_minutes();
        if (0..180).contains(&pm) { 6 + (pm / 30) as u8 } else { return None } // 13:00–16:00 → τ 6..=11
    };
    Bin::new(tau)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayKind { Full, HalfDay, Closed }

pub trait TradingCalendar: Send + Sync {
    fn day_kind(&self, d: NaiveDate) -> DayKind;
    fn prev_trading_day(&self, d: NaiveDate) -> NaiveDate;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn t(h: u32, m: u32) -> NaiveTime { NaiveTime::from_hms_opt(h, m, 0).unwrap() }

    #[test]
    fn bin_grid() {
        assert_eq!(bin_of(t(9, 30)).unwrap().tau(), 1);
        assert_eq!(bin_of(t(11, 59)).unwrap().tau(), 5);
        assert!(bin_of(t(12, 0)).is_none());   // lunch
        assert!(bin_of(t(12, 30)).is_none());
        assert_eq!(bin_of(t(13, 0)).unwrap().tau(), 6);
        assert_eq!(bin_of(t(15, 59)).unwrap().tau(), 11);
        assert!(bin_of(t(16, 0)).is_none());   // continuous close
        assert!(bin_of(t(9, 29)).is_none());   // pre-open
    }
}
```

```rust
// crates/hkq-core/src/calendar.rs
//! JSON-backed trading calendar (config/calendar_hk.json): explicit holidays and
//! half-days; weekends are structurally closed. The file must cover the full
//! backtest + live range — `prev_trading_day` panics loudly if walked off it.
use crate::error::CoreError;
use crate::session::{DayKind, TradingCalendar};
use chrono::{Datelike, NaiveDate, Weekday};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CalendarFile {
    holidays: Vec<NaiveDate>,
    half_days: Vec<NaiveDate>,
}

#[derive(Debug)]
pub struct FileCalendar {
    holidays: BTreeSet<NaiveDate>,
    half_days: BTreeSet<NaiveDate>,
}

impl FileCalendar {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, CoreError> {
        let raw = std::fs::read_to_string(path.as_ref())?;
        let f: CalendarFile = serde_json::from_str(&raw)
            .map_err(|e| CoreError::Calendar(format!("{}: {e}", path.as_ref().display())))?;
        Ok(Self {
            holidays: f.holidays.into_iter().collect(),
            half_days: f.half_days.into_iter().collect(),
        })
    }
}

impl TradingCalendar for FileCalendar {
    fn day_kind(&self, d: NaiveDate) -> DayKind {
        if matches!(d.weekday(), Weekday::Sat | Weekday::Sun) || self.holidays.contains(&d) {
            DayKind::Closed
        } else if self.half_days.contains(&d) {
            DayKind::HalfDay
        } else {
            DayKind::Full
        }
    }

    fn prev_trading_day(&self, d: NaiveDate) -> NaiveDate {
        let mut cur = d;
        for _ in 0..400 {
            cur = cur.pred_opt().expect("date underflow");
            if self.day_kind(cur) != DayKind::Closed { return cur; }
        }
        panic!("calendar exhausted walking back from {d}: extend calendar_hk.json");
    }
}
```

```rust
// crates/hkq-core/src/config.rs
//! Every constant in the report, named and sourced — no magic numbers in code.
//! Vendor/source config is owned by hkq-data (same TOML file, `[sources]` tables).
use crate::error::CoreError;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct StrategyCfg {
    pub universe: UniverseCfg,
    pub factors: FactorCfg,
    pub stage1: Stage1Cfg,
    pub stage2: Stage2Cfg,
    pub trade: TradeCfg,
    pub costs: CostCfg,
    pub ops: OpsCfg,
}

impl StrategyCfg {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, CoreError> {
        let raw = std::fs::read_to_string(path.as_ref())?;
        toml::from_str(&raw)
            .map_err(|e| CoreError::Config(format!("{}: {e}", path.as_ref().display())))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniverseCfg {                    // §3.1
    pub min_median_turnover_hkd: f64,       // 30_000_000
    pub min_price_hkd: f64,                 // 1.0
    pub min_listed_days: u32,               // 60
    pub max_median_spread_bps: f64,         // 35.0
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactorCfg {                      // §3.0
    pub ewma_halflife_days: f64,            // 21
    pub ewma_min_obs: usize,                // 63
    pub amihud_window: usize,               // 60
    pub rv_days: usize,                     // 5
    pub lav_gamma: f64,                     // 0.3
    pub seasonal_vol_days: usize,           // 20
    pub ivu_tercile_window: usize,          // 60
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stage1Cfg {                      // §3.2
    pub theta1: f64,
    pub theta2: f64,
    pub eta: f64,                           // 0.25
    pub vs_threshold: f64,                  // v* (estimated; hkq-validate owns refresh)
    pub leadlag_window: usize,              // 250
    pub fdr_q: f64,                         // 0.10
    pub icir_window: usize,                 // 250
    pub icir_shrink_delta: f64,             // 0.10
    pub top_k_sectors: usize,               // 2..=3
    pub sigma_min_gate: f64,                // Σ_min — below ⇒ cash day
    pub member_weight_cap: f64,             // 0.15
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stage2Cfg {                      // §3.3–§3.4
    pub phi: f64,                           // 2.0
    pub zeta: f64,
    pub vs_threshold_stock: f64,
    pub beta_window: usize,                 // 250
    pub winsor_pct: f64,                    // 0.01
    pub ortho_order: Vec<String>,           // ["x5","x3","x2","x1","x6","x7"] — FIXED
    pub names_per_sector: usize,            // m ∈ {2,4}
}

#[derive(Debug, Clone, Deserialize)]
pub struct TradeCfg {                       // §3.5–§3.6
    pub margin_bps: f64,                    // m* = 10
    pub stop_sigma15m_mult: f64,            // 2.5
    pub participation_cap: f64,             // 0.02
    pub half_day_mode: HalfDayMode,
    pub reuse_unsettled_proceeds: bool,     // broker-dependent T+2 flag (§1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HalfDayMode { Skip, Compressed }

#[derive(Debug, Clone, Deserialize)]
pub struct CostCfg {                        // §1
    pub stamp_bps_per_side: f64,            // 10.0
    pub fees_bps_roundtrip: f64,            // 2.2
    pub impact_kappa: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpsCfg {
    pub lake_root: PathBuf,
    pub calendar_path: PathBuf,
    pub universe_codes_path: PathBuf,       // one code per line; M2 replaces with §3.1 job
    pub ah_map_path: Option<PathBuf>,       // csv: hk_code,em_secid
    #[serde(default)]
    pub log_json: bool,
}
```

## `hkq-data`

```toml
# crates/hkq-data/Cargo.toml
[package]
name = "hkq-data"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[features]
# Research-only providers (Yahoo). MUST NOT be enabled by live binaries.
research = []

[dependencies]
hkq-core = { path = "../hkq-core" }
tokio.workspace = true
tokio-stream.workspace = true
reqwest.workspace = true
serde.workspace = true
serde_json.workspace = true
polars.workspace = true
chrono.workspace = true
chrono-tz.workspace = true
thiserror.workspace = true
async-trait.workspace = true
futures.workspace = true
governor.workspace = true
tracing.workspace = true
rsa.workspace = true
sha1.workspace = true
base64.workspace = true
rust_decimal.workspace = true
toml.workspace = true
```

```rust
// crates/hkq-data/src/lib.rs
#![forbid(unsafe_code)]
//! Async ingestion layer: capability-scoped providers, rate-limited HTTP substrate,
//! hive-partitioned parquet lake with atomic idempotent writes, nightly close
//! reconciliation, and the live MarketEvent multiplexer (report §5).

pub mod cfg;
pub mod eastmoney;
pub mod error;
pub mod http;
pub mod ingest;
pub mod lake;
pub mod model;
pub mod provider;
pub mod recon;
pub mod tiger;
pub mod xueqiu;

#[cfg(feature = "research")]
pub mod yahoo;
```

```rust
// crates/hkq-data/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("http transport: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("http status {0}")]
    Status(u16),
    #[error("retries exhausted")]
    RetriesExhausted,
    #[error("vendor error code={code}: {msg}")]
    Vendor { code: i64, msg: String },
    #[error("parse [{context}]: {msg}")]
    Parse { context: &'static str, msg: String },
    #[error("schema drift: {0} (bump sources schema_version / fix parser)")]
    SchemaDrift(&'static str),
    #[error("capability unsupported: {0}")]
    Unsupported(&'static str),
    #[error("request signing: {0}")]
    Signing(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),
    #[error("config: {0}")]
    Config(String),
}
```

```rust
// crates/hkq-data/src/model.rs
//! Wire-facing domain models. Every provider normalizes vendor payloads into these
//! structs at the boundary — timestamps become exchange-clock instants here, never
//! later. Parquet persistence uses `ts_ms` (epoch milliseconds, an absolute instant)
//! plus an ISO `date` string column: deterministic schema, zero dtype inference.
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use chrono_tz::{Asia::Hong_Kong, Tz};
use hkq_core::ids::StockCode;
use serde::{Deserialize, Serialize};

/// Canonical column names shared by writers and (M2) readers.
pub mod cols {
    pub const CODE: &str = "code";
    pub const DATE: &str = "date";
    pub const TS_MS: &str = "ts_ms";
    pub const OPEN: &str = "open";
    pub const HIGH: &str = "high";
    pub const LOW: &str = "low";
    pub const CLOSE: &str = "close";
    pub const ADJ_CLOSE: &str = "adj_close";
    pub const VOLUME: &str = "volume";
    pub const TURNOVER: &str = "turnover";
    pub const NET_BUY: &str = "net_buy";
    pub const A_OPEN_RET: &str = "a_open_ret";
}

pub fn ms_to_hkt(ms: i64) -> Option<DateTime<Tz>> {
    Utc.timestamp_millis_opt(ms).single().map(|u| u.with_timezone(&Hong_Kong))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar1m {
    pub code: StockCode,
    /// Bar OPEN time on the exchange clock (HKT) — non-negotiable (§5).
    pub ts: DateTime<Tz>,
    pub o: f64, pub h: f64, pub l: f64, pub c: f64,
    pub volume: f64,
    pub turnover: f64,
}

/// POS state snapshot (§1): {IEP_t, IEV_t} trajectory, ≤30 s cadence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionSnap {
    pub code: StockCode,
    pub ts: DateTime<Tz>,
    pub iep: Option<f64>,
    pub iev: Option<f64>,
    pub bid_qty: Option<f64>, // X2 imbalance augmentation, where depth exists
    pub ask_qty: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum MarketEvent {
    Auction(AuctionSnap),
    Bar(Bar1m),
    Quote { code: StockCode, ts: DateTime<Tz>, bid: f64, ask: f64 },
    /// 09:25 SSE/SZSE auction print mapped to the HK line (S6).
    MainlandAuctionPrint { code: StockCode, a_open_ret: f64 },
    A50 { ts: DateTime<Tz>, px: f64 },
    /// §1 VCM constraint — producer arrives with the exec milestone.
    Vcm { code: StockCode, in_cooling_off: bool },
    Halt { code: StockCode },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyBar {
    pub code: StockCode,
    pub date: NaiveDate,
    pub open: f64, pub high: f64, pub low: f64, pub close: f64,
    /// Equals `close` when the vendor adjust-mode is unverified; see TigerCfg.
    pub adj_close: f64,
    pub volume: f64,
    pub turnover: f64,
}
```

```rust
// crates/hkq-data/src/provider.rs
//! Capability traits: each vendor implements ONLY what it truly has. The report is
//! explicit that Yahoo-class sources must never back the POS feed, and that auction
//! fields are optional-with-fallback (X2-disabled mode) — hence per-capability traits.
use crate::{error::DataError, model::*};
use async_trait::async_trait;
use chrono::NaiveDate;
use futures::stream::BoxStream;
use hkq_core::ids::StockCode;
use polars::prelude::DataFrame;

#[async_trait]
pub trait EodProvider: Send + Sync {
    /// Adjusted OHLCV + raw close + turnover; MUST be point-in-time (no survivorship).
    async fn daily_bars(&self, codes: &[StockCode], from: NaiveDate, to: NaiveDate)
        -> Result<DataFrame, DataError>;
}

#[async_trait]
pub trait AuctionFeed: Send + Sync {
    /// Live IEP/IEV during POS. Err(Unsupported) ⇒ engine runs in X2-disabled mode (§5).
    async fn subscribe_pos(&self, codes: &[StockCode])
        -> Result<BoxStream<'static, AuctionSnap>, DataError>;
}

#[async_trait]
pub trait IntradayFeed: Send + Sync {
    async fn subscribe_bars_1m(&self, codes: &[StockCode])
        -> Result<BoxStream<'static, Bar1m>, DataError>;
    async fn backfill_bars_1m(&self, code: StockCode, date: NaiveDate)
        -> Result<DataFrame, DataError>;
}

#[async_trait]
pub trait FlowProvider: Send + Sync {
    /// Per-stock southbound net buy for date d (S5/X7 raw material). Units are
    /// vendor-native; column `net_buy` + manifest `source` tag record provenance.
    async fn southbound_net_buy(&self, d: NaiveDate) -> Result<DataFrame, DataError>;
}

#[async_trait]
pub trait LinkedMarketFeed: Send + Sync {
    /// SSE/SZSE 09:25 auction prints for the AH subset (S6).
    async fn mainland_open_prints(&self, ah_codes: &[StockCode])
        -> Result<DataFrame, DataError>;
    async fn subscribe_a50(&self) -> Result<BoxStream<'static, MarketEvent>, DataError>;
}
```

```rust
// crates/hkq-data/src/http.rs
//! Shared HTTP substrate: token-bucket admission (governor), bounded retry with
//! exponential backoff + jitter, tracing. Every provider goes through this.
use crate::error::DataError;
use governor::{clock::DefaultClock, state::{InMemoryState, NotKeyed}, Quota, RateLimiter};
use serde::de::DeserializeOwned;
use std::{num::NonZeroU32, sync::Arc, time::Duration};

pub struct RatedClient {
    http: reqwest::Client,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    max_retries: u32,
}

impl RatedClient {
    pub fn new(rps: u32, max_retries: u32) -> Self {
        Self {
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .pool_idle_timeout(Duration::from_secs(60))
                .user_agent("hkq/0.1")
                .build()
                .expect("reqwest client"),
            limiter: Arc::new(RateLimiter::direct(Quota::per_second(
                NonZeroU32::new(rps.max(1)).expect("nonzero rps"),
            ))),
            max_retries,
        }
    }

    pub async fn get_json<T: DeserializeOwned>(
        &self, url: &str, headers: &[(&str, &str)],
    ) -> Result<T, DataError> {
        self.exec(|| {
            let mut r = self.http.get(url);
            for (k, v) in headers { r = r.header(*k, *v); }
            r
        }).await
    }

    pub async fn post_json<T: DeserializeOwned>(
        &self, url: &str, body: &serde_json::Value, headers: &[(&str, &str)],
    ) -> Result<T, DataError> {
        self.exec(|| {
            let mut r = self.http.post(url).json(body);
            for (k, v) in headers { r = r.header(*k, *v); }
            r
        }).await
    }

    /// Retry policy: 429 and 5xx are transient; other non-2xx are terminal.
    /// Backoff: 100ms·2^k capped at ~6.4s, plus 0–250ms jitter.
    async fn exec<T: DeserializeOwned>(
        &self, build: impl Fn() -> reqwest::RequestBuilder,
    ) -> Result<T, DataError> {
        let mut attempt = 0u32;
        loop {
            self.limiter.until_ready().await;
            match build().send().await {
                Ok(r) if r.status().is_success() => return Ok(r.json::<T>().await?),
                Ok(r) if r.status().as_u16() == 429 || r.status().is_server_error() => {
                    tracing::warn!(status = %r.status(), "transient http status");
                }
                Ok(r) => return Err(DataError::Status(r.status().as_u16())),
                Err(e) if attempt < self.max_retries => {
                    tracing::warn!(error = %e, attempt, "transport error, retrying");
                }
                Err(e) => return Err(e.into()),
            }
            attempt += 1;
            if attempt > self.max_retries {
                return Err(DataError::RetriesExhausted);
            }
            let base = Duration::from_millis(100 * (1 << attempt.min(6)));
            tokio::time::sleep(base + Duration::from_millis(jitter_ms(250))).await;
        }
    }
}

fn jitter_ms(cap: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.subsec_nanos() as u64 % cap).unwrap_or(0)
}
```

```rust
// crates/hkq-data/src/cfg.rs
//! `[sources.*]` tables of strategy.toml. Kept out of hkq-core so the domain crate
//! stays vendor-agnostic. Undocumented endpoints live HERE, never in code (§5).
use crate::error::DataError;
use crate::{eastmoney::EastMoneyCfg, tiger::TigerCfg, xueqiu::XueqiuCfg};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct SourcesCfg {
    pub tiger: Option<TigerCfg>,
    pub eastmoney: Option<EastMoneyCfg>,
    pub xueqiu: Option<XueqiuCfg>,
}

#[derive(Debug, Deserialize)]
struct SourcesFile { sources: SourcesCfg }

pub fn load_sources(path: impl AsRef<Path>) -> Result<SourcesCfg, DataError> {
    let raw = std::fs::read_to_string(path.as_ref())?;
    let f: SourcesFile = toml::from_str(&raw)
        .map_err(|e| DataError::Config(format!("{}: {e}", path.as_ref().display())))?;
    Ok(f.sources)
}
```

```rust
// crates/hkq-data/src/tiger.rs
//! Tiger OpenAPI adapter. Signed-POST gateway (RSA-SHA1 over canonicalized params);
//! no official Rust SDK, so we wrap the HTTP protocol. During the POS we POLL quotes
//! on a 15s cadence, satisfying the report's ≤30s IEP/IEV requirement without the
//! push socket on day one.
//!
//! VERIFY-before-go-live (blueprint "Honest Gaps"): method names, biz_content field
//! names, timestamp timezone, and HK auction field names against YOUR account's API
//! version. All are config-driven below; none are hardcoded assumptions.
use crate::{error::DataError, http::RatedClient, model::*, provider::*};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use chrono::{NaiveDate, NaiveTime, Utc};
use chrono_tz::Asia::Hong_Kong;
use futures::{stream::BoxStream, StreamExt};
use hkq_core::ids::StockCode;
use hkq_core::session::{hk, SessionTimes};
use polars::{df, prelude::*};
use rsa::{
    pkcs1::DecodeRsaPrivateKey, pkcs1v15::SigningKey, pkcs8::DecodePrivateKey,
    signature::{SignatureEncoding, Signer},
    RsaPrivateKey,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sha1::Sha1;
use std::{collections::BTreeMap, collections::HashMap, sync::Arc, time::Duration};
use tokio_stream::wrappers::ReceiverStream;

fn default_rps() -> u32 { 8 }
fn default_retries() -> u32 { 4 }
fn default_pos_poll() -> u64 { 15 }
fn default_bar_poll() -> u64 { 10 }
fn default_batch() -> usize { 50 }
fn default_version() -> String { "2.0".into() }
fn default_quote_method() -> String { "quote_real_time".into() }
fn default_kline_method() -> String { "kline".into() }

#[derive(Debug, Clone, Deserialize)]
pub struct TigerCfg {
    pub gateway: String,            // e.g. "https://openapi.tigersecurities.com/gateway"
    pub tiger_id: String,
    pub private_key_path: String,   // PKCS#8 or PKCS#1 PEM registered with Tiger
    #[serde(default = "default_version")]
    pub api_version: String,
    #[serde(default = "default_quote_method")]
    pub quote_method: String,       // VERIFY against account API version
    #[serde(default = "default_kline_method")]
    pub kline_method: String,       // VERIFY
    /// Adjust mode for daily bars (e.g. "forward"). None ⇒ raw; adj_close = close.
    #[serde(default)]
    pub kline_adjust: Option<String>,
    #[serde(default = "default_rps")]
    pub rps: u32,
    #[serde(default = "default_retries")]
    pub max_retries: u32,
    #[serde(default = "default_pos_poll")]
    pub pos_poll_secs: u64,
    #[serde(default = "default_bar_poll")]
    pub bar_poll_secs: u64,
    #[serde(default = "default_batch")]
    pub batch_size: usize,
    /// Skip the auction-field probe (e.g. when subscribing outside 09:00–09:22 in
    /// integration tests). Production default: probe.
    #[serde(default)]
    pub assume_auction_fields: bool,
    /// Enforce the POS window on the polling loop (disable only for replay tests).
    #[serde(default = "crate::tiger::default_true")]
    pub pos_window_enforced: bool,
    #[serde(default)]
    pub auction_aliases: Option<AuctionAliases>,
}
pub(crate) fn default_true() -> bool { true }

/// Candidate vendor field names for auction data. Config-extendable because the HK
/// quote payload's auction fields are exactly what must be verified per account.
#[derive(Debug, Clone, Deserialize)]
pub struct AuctionAliases {
    pub iep: Vec<String>,
    pub iev: Vec<String>,
    pub bid_qty: Vec<String>,
    pub ask_qty: Vec<String>,
    pub ts: Vec<String>,
}

impl Default for AuctionAliases {
    fn default() -> Self {
        Self {
            iep: vec!["auction_price".into(), "iep".into(), "matched_price".into()],
            iev: vec!["auction_volume".into(), "iev".into(), "matched_volume".into()],
            bid_qty: vec!["bid_size".into(), "bid_volume".into()],
            ask_qty: vec!["ask_size".into(), "ask_volume".into()],
            ts: vec!["timestamp".into(), "latest_time".into()],
        }
    }
}

struct Inner {
    cfg: TigerCfg,
    http: RatedClient,
    key: SigningKey<Sha1>,
    aliases: AuctionAliases,
}

#[derive(Clone)]
pub struct TigerClient {
    inner: Arc<Inner>,
}

impl TigerClient {
    pub fn new(cfg: TigerCfg) -> Result<Self, DataError> {
        let pem = std::fs::read_to_string(&cfg.private_key_path)?;
        let sk = RsaPrivateKey::from_pkcs8_pem(&pem)
            .or_else(|_| RsaPrivateKey::from_pkcs1_pem(&pem))
            .map_err(|e| DataError::Signing(format!("private key parse: {e}")))?;
        let key = SigningKey::<Sha1>::new(sk);
        let http = RatedClient::new(cfg.rps, cfg.max_retries);
        let aliases = cfg.auction_aliases.clone().unwrap_or_default();
        Ok(Self { inner: Arc::new(Inner { cfg, http, key, aliases }) })
    }
}

impl Inner {
    /// Canonical param string: keys ascending, "k=v" joined by '&', `sign` excluded.
    /// This mirrors Tiger's Alipay-style gateway convention. VERIFY once at go-live.
    fn canonical(params: &BTreeMap<&str, String>) -> String {
        params.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&")
    }

    fn sign(&self, canonical: &str) -> String {
        let sig = self.key.sign(canonical.as_bytes()); // PKCS#1 v1.5, deterministic
        B64.encode(sig.to_bytes())
    }

    /// One signed request. Envelope: { code, message, data } where `data` is either
    /// a JSON object or a string-encoded JSON document (both occur in the wild).
    async fn call(&self, method: &str, biz: Value) -> Result<Value, DataError> {
        // VERIFY: timestamp timezone. CN-gateway convention is GMT+8 local time.
        let ts = Utc::now().with_timezone(&Hong_Kong).format("%Y-%m-%d %H:%M:%S").to_string();
        let mut p: BTreeMap<&str, String> = BTreeMap::new();
        p.insert("tiger_id", self.cfg.tiger_id.clone());
        p.insert("method", method.to_string());
        p.insert("charset", "UTF-8".to_string());
        p.insert("sign_type", "RSA".to_string());
        p.insert("version", self.cfg.api_version.clone());
        p.insert("timestamp", ts);
        p.insert("biz_content", biz.to_string());

        let sign = self.sign(&Self::canonical(&p));
        let mut body = serde_json::Map::new();
        for (k, v) in &p { body.insert((*k).to_string(), Value::String(v.clone())); }
        body.insert("sign".into(), Value::String(sign));

        let resp: Value = self.http.post_json(&self.cfg.gateway, &Value::Object(body), &[]).await?;
        let code = resp.get("code").and_then(Value::as_i64)
            .ok_or(DataError::SchemaDrift("tiger envelope missing `code`"))?;
        if code != 0 {
            let msg = resp.get("message").and_then(Value::as_str).unwrap_or("").to_string();
            return Err(DataError::Vendor { code, msg });
        }
        Ok(match resp.get("data") {
            Some(Value::String(s)) => serde_json::from_str(s).unwrap_or(Value::String(s.clone())),
            Some(other) => other.clone(),
            None => Value::Null,
        })
    }
}

fn code_from_symbol(sym: &str) -> Option<StockCode> {
    StockCode::parse(sym.trim_start_matches(|c: char| !c.is_ascii_digit()))
}

fn pick_f64(obj: &Value, aliases: &[String]) -> Option<f64> {
    aliases.iter().find_map(|k| obj.get(k).and_then(Value::as_f64))
}

fn pick_i64(obj: &Value, aliases: &[String]) -> Option<i64> {
    aliases.iter().find_map(|k| obj.get(k).and_then(Value::as_i64))
}

/// Normalize `data` into a list of per-symbol objects across payload shapes.
fn quote_items(data: &Value) -> Vec<Value> {
    match data {
        Value::Array(a) => a.clone(),
        Value::Object(o) => o.get("items").and_then(Value::as_array).cloned()
            .unwrap_or_else(|| vec![data.clone()]),
        _ => vec![],
    }
}

fn parse_auction_snaps(data: &Value, aliases: &AuctionAliases) -> Vec<AuctionSnap> {
    let now = Utc::now().with_timezone(&Hong_Kong);
    quote_items(data).iter().filter_map(|item| {
        let code = item.get("symbol").and_then(Value::as_str).and_then(code_from_symbol)?;
        let iep = pick_f64(item, &aliases.iep);
        let iev = pick_f64(item, &aliases.iev);
        if iep.is_none() && iev.is_none() { return None; }
        let ts = pick_i64(item, &aliases.ts).and_then(ms_to_hkt).unwrap_or(now);
        Some(AuctionSnap {
            code, ts, iep, iev,
            bid_qty: pick_f64(item, &aliases.bid_qty),
            ask_qty: pick_f64(item, &aliases.ask_qty),
        })
    }).collect()
}

/// Emit only completed bars, exactly once, in order — pure for testability.
pub(crate) fn accept_bar(
    watermark: &mut HashMap<StockCode, i64>, code: StockCode, ts_ms: i64, now_ms: i64,
) -> bool {
    if ts_ms + 60_000 > now_ms { return false; } // bar still forming
    match watermark.get(&code) {
        Some(&w) if ts_ms <= w => false,
        _ => { watermark.insert(code, ts_ms); true }
    }
}

fn parse_kline_bars(data: &Value) -> Vec<(StockCode, i64, f64, f64, f64, f64, f64, f64)> {
    // (code, ts_ms, o, h, l, c, volume, turnover)
    let series = match data {
        Value::Array(a) => a.clone(),
        v @ Value::Object(_) => vec![v.clone()],
        _ => vec![],
    };
    let mut out = Vec::new();
    for s in &series {
        let Some(code) = s.get("symbol").and_then(Value::as_str).and_then(code_from_symbol)
            else { continue };
        let Some(items) = s.get("items").and_then(Value::as_array) else { continue };
        for it in items {
            let g = |k: &str| it.get(k).and_then(Value::as_f64);
            let (Some(t), Some(o), Some(h), Some(l), Some(c)) =
                (it.get("time").and_then(Value::as_i64), g("open"), g("high"), g("low"), g("close"))
                else { continue };
            out.push((code, t, o, h, l, c, g("volume").unwrap_or(0.0),
                      g("amount").unwrap_or(f64::NAN)));
        }
    }
    out
}

#[async_trait]
impl AuctionFeed for TigerClient {
    async fn subscribe_pos(&self, codes: &[StockCode])
        -> Result<BoxStream<'static, AuctionSnap>, DataError>
    {
        let inner = self.inner.clone();
        let codes: Vec<StockCode> = codes.to_vec();

        // Probe once: if the HK payload carries no auction fields, fail FAST so the
        // engine sets X2 = disabled (§5) instead of silently starving the factor.
        if !inner.cfg.assume_auction_fields {
            let probe = codes.iter().take(inner.cfg.batch_size)
                .map(ToString::to_string).collect::<Vec<_>>();
            let data = inner.call(&inner.cfg.quote_method,
                json!({ "symbols": probe, "market": "HK" })).await?;
            if parse_auction_snaps(&data, &inner.aliases).is_empty() {
                return Err(DataError::Unsupported(
                    "tiger HK quote payload exposes no IEP/IEV fields (X2-disabled mode)"));
            }
        }

        let (tx, rx) = tokio::sync::mpsc::channel::<AuctionSnap>(1024);
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(Duration::from_secs(inner.cfg.pos_poll_secs));
            tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            let cutoff: NaiveTime = SessionTimes::get().pos_match_to;
            loop {
                tick.tick().await;
                if tx.is_closed() { break; }
                let now_hkt = Utc::now().with_timezone(&Hong_Kong).time();
                if inner.cfg.pos_window_enforced && now_hkt > cutoff {
                    tracing::info!("POS window closed; auction poller stopping");
                    break;
                }
                for chunk in codes.chunks(inner.cfg.batch_size) {
                    let symbols: Vec<String> = chunk.iter().map(ToString::to_string).collect();
                    match inner.call(&inner.cfg.quote_method,
                        json!({ "symbols": symbols, "market": "HK" })).await
                    {
                        Ok(data) => {
                            for snap in parse_auction_snaps(&data, &inner.aliases) {
                                if tx.send(snap).await.is_err() { return; }
                            }
                        }
                        Err(e) => tracing::warn!(error = %e, "POS poll failed; next tick"),
                    }
                }
            }
        });
        Ok(ReceiverStream::new(rx).boxed())
    }
}

#[async_trait]
impl IntradayFeed for TigerClient {
    async fn subscribe_bars_1m(&self, codes: &[StockCode])
        -> Result<BoxStream<'static, Bar1m>, DataError>
    {
        let inner = self.inner.clone();
        let codes: Vec<StockCode> = codes.to_vec();
        let (tx, rx) = tokio::sync::mpsc::channel::<Bar1m>(4096);

        tokio::spawn(async move {
            let mut wm: HashMap<StockCode, i64> = HashMap::new();
            let mut tick = tokio::time::interval(Duration::from_secs(inner.cfg.bar_poll_secs));
            tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            loop {
                tick.tick().await;
                if tx.is_closed() { break; }
                let now_ms = Utc::now().timestamp_millis();
                for chunk in codes.chunks(inner.cfg.batch_size) {
                    let symbols: Vec<String> = chunk.iter().map(ToString::to_string).collect();
                    // VERIFY: period/limit field names for your account's API version.
                    let biz = json!({ "symbols": symbols, "period": "min1", "limit": 3 });
                    match inner.call(&inner.cfg.kline_method, biz).await {
                        Ok(data) => {
                            let mut bars = parse_kline_bars(&data);
                            bars.sort_by_key(|b| (b.0, b.1)); // (code, ts) order
                            for (code, t, o, h, l, c, v, amt) in bars {
                                if !accept_bar(&mut wm, code, t, now_ms) { continue; }
                                let Some(ts) = ms_to_hkt(t) else { continue };
                                let bar = Bar1m { code, ts, o, h, l, c, volume: v, turnover: amt };
                                if tx.send(bar).await.is_err() { return; }
                            }
                        }
                        Err(e) => tracing::warn!(error = %e, "bar poll failed; next tick"),
                    }
                }
            }
        });
        Ok(ReceiverStream::new(rx).boxed())
    }

    async fn backfill_bars_1m(&self, code: StockCode, date: NaiveDate)
        -> Result<DataFrame, DataError>
    {
        let s = SessionTimes::get();
        let begin = hk(date, s.pos_start).timestamp_millis();
        let end = hk(date, s.cas_hard_end).timestamp_millis();
        let biz = json!({
            "symbols": [code.to_string()], "period": "min1",
            "begin_time": begin, "end_time": end, "limit": 1000
        });
        let data = self.inner.call(&self.inner.cfg.kline_method, biz).await?;
        let mut rows = parse_kline_bars(&data);
        rows.sort_by_key(|b| b.1);
        let (mut codes, mut ts, mut o, mut h, mut l, mut c, mut v, mut amt) =
            (vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]);
        for (cd, t, ro, rh, rl, rc, rv, ra) in rows {
            codes.push(cd.0); ts.push(t); o.push(ro); h.push(rh);
            l.push(rl); c.push(rc); v.push(rv); amt.push(ra);
        }
        let df = df!(
            cols::CODE => codes, cols::TS_MS => ts,
            "o" => o, "h" => h, "l" => l, "c" => c,
            cols::VOLUME => v, cols::TURNOVER => amt,
            cols::DATE => vec![date.to_string(); ts.len()],
        )?;
        Ok(df)
    }
}

#[async_trait]
impl EodProvider for TigerClient {
    async fn daily_bars(&self, codes: &[StockCode], from: NaiveDate, to: NaiveDate)
        -> Result<DataFrame, DataError>
    {
        if self.inner.cfg.kline_adjust.is_none() {
            tracing::warn!("tiger kline adjust-mode unset: adj_close = close (raw). \
                            VERIFY adjust param before relying on adjusted history.");
        }
        let begin = hk(from, NaiveTime::MIN).timestamp_millis();
        let end = hk(to.succ_opt().unwrap_or(to), NaiveTime::MIN).timestamp_millis();

        let (mut c_, mut d_, mut o_, mut h_, mut l_, mut cl, mut ad, mut v_, mut t_) =
            (vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]);
        for chunk in codes.chunks(self.inner.cfg.batch_size) {
            let symbols: Vec<String> = chunk.iter().map(ToString::to_string).collect();
            let mut biz = json!({
                "symbols": symbols, "period": "day",
                "begin_time": begin, "end_time": end, "limit": 1000
            });
            if let Some(adj) = &self.inner.cfg.kline_adjust {
                biz["right"] = json!(adj); // VERIFY: param name for adjust mode
            }
            let data = self.inner.call(&self.inner.cfg.kline_method, biz).await?;
            for (code, tms, o, h, l, c, v, amt) in parse_kline_bars(&data) {
                let Some(hkt) = ms_to_hkt(tms) else { continue };
                c_.push(code.0);
                d_.push(hkt.date_naive().to_string());
                o_.push(o); h_.push(h); l_.push(l); cl.push(c);
                ad.push(c); // raw mode: adjusted series arrives with verified param
                v_.push(v); t_.push(amt);
            }
        }
        let df = df!(
            cols::CODE => c_, cols::DATE => d_,
            cols::OPEN => o_, cols::HIGH => h_, cols::LOW => l_,
            cols::CLOSE => cl, cols::ADJ_CLOSE => ad,
            cols::VOLUME => v_, cols::TURNOVER => t_,
        )?;
        Ok(df)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_sorts_keys() {
        let mut p: BTreeMap<&str, String> = BTreeMap::new();
        p.insert("timestamp", "T".into());
        p.insert("biz_content", "B".into());
        p.insert("method", "M".into());
        assert_eq!(Inner::canonical(&p), "biz_content=B&method=M&timestamp=T");
    }

    #[test]
    fn watermark_emits_completed_once_in_order() {
        let mut wm = HashMap::new();
        let c = StockCode(700);
        let now = 1_000_000_000;
        assert!(!accept_bar(&mut wm, c, now - 30_000, now)); // still forming
        assert!(accept_bar(&mut wm, c, now - 120_000, now)); // complete
        assert!(!accept_bar(&mut wm, c, now - 120_000, now)); // duplicate
        assert!(!accept_bar(&mut wm, c, now - 180_000, now)); // out-of-order stale
        assert!(accept_bar(&mut wm, c, now - 60_000, now));  // next bar
    }

    #[test]
    fn auction_parse_uses_aliases_and_skips_fieldless() {
        let aliases = AuctionAliases::default();
        let data = serde_json::json!({ "items": [
            { "symbol": "00700", "auction_price": 321.4, "auction_volume": 1.2e6 },
            { "symbol": "00005", "latest_price": 60.0 }  // no auction fields ⇒ skipped
        ]});
        let snaps = parse_auction_snaps(&data, &aliases);
        assert_eq!(snaps.len(), 1);
        assert_eq!(snaps[0].code, StockCode(700));
        assert_eq!(snaps[0].iep, Some(321.4));
    }
}
```

```rust
// crates/hkq-data/src/eastmoney.rs
//! EastMoney public JSON endpoints: A-share 09:25 auction prints for the AH subset,
//! A50-proxy quotes for the S6 leg, and daily southbound (HSGT) per-stock net buy.
//! Endpoints are UNDOCUMENTED and can move — every URL is a config template, every
//! parser is schema-versioned, and nightly close recon is the tripwire (§5).
//! Production substitutes HKEX OMD-C + official Connect statistics (report §5).
use crate::{error::DataError, http::RatedClient, model::*, provider::*};
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use futures::{stream::BoxStream, StreamExt};
use hkq_core::ids::StockCode;
use hkq_core::session::SessionTimes;
use polars::{df, prelude::*};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::BTreeMap, path::Path, sync::Arc, time::Duration};
use tokio_stream::wrappers::ReceiverStream;

fn default_rps() -> u32 { 4 }
fn default_retries() -> u32 { 3 }
fn default_poll() -> u64 { 5 }
fn default_schema() -> u32 { 1 }

#[derive(Debug, Clone, Deserialize)]
pub struct EastMoneyCfg {
    /// Batch quote URL template; `{secids}` substituted. Must request float
    /// formatting (e.g. fltt=2) and fields f12 (code), f17 (open), f18 (prev close).
    pub quote_url_template: String,
    /// Single-quote URL template for the A50 proxy; `{secid}` substituted (f43 last).
    pub a50_url_template: String,
    /// EM secid of the A50/mainland-index proxy instrument. VERIFY choice.
    pub a50_secid: String,
    /// Southbound per-stock net-buy URL template; `{date}` (YYYY-MM-DD) substituted.
    pub southbound_url_template: String,
    /// Currency/unit of the southbound net-buy figures as served (provenance tag).
    pub southbound_unit: String,
    #[serde(default = "default_schema")]
    pub schema_version: u32,
    #[serde(default = "default_rps")]
    pub rps: u32,
    #[serde(default = "default_retries")]
    pub max_retries: u32,
    #[serde(default = "default_poll")]
    pub a50_poll_secs: u64,
    #[serde(default = "crate::tiger::default_true")]
    pub a50_window_enforced: bool,
}

/// HK code → EastMoney secid ("1.600941" SSE / "0.000001" SZSE) for the AH subset.
pub type AhMap = BTreeMap<StockCode, String>;

/// Load config/ah_map.csv — lines of `hk_code,em_secid` (header optional).
pub fn load_ah_map(path: impl AsRef<Path>) -> Result<AhMap, DataError> {
    let raw = std::fs::read_to_string(path.as_ref())?;
    let mut map = AhMap::new();
    for line in raw.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let mut parts = line.splitn(2, ',');
        let (Some(code), Some(secid)) = (parts.next(), parts.next()) else { continue };
        if let Some(c) = StockCode::parse(code.trim()) {
            map.insert(c, secid.trim().to_string());
        }
    }
    if map.is_empty() {
        return Err(DataError::Config(format!("empty AH map: {}", path.as_ref().display())));
    }
    Ok(map)
}

struct Inner {
    cfg: EastMoneyCfg,
    http: RatedClient,
    ah_map: AhMap,
    em_code_to_hk: BTreeMap<String, StockCode>, // "600941" → 00941's HK line
}

#[derive(Clone)]
pub struct EastMoneyClient { inner: Arc<Inner> }

impl EastMoneyClient {
    pub fn new(cfg: EastMoneyCfg, ah_map: AhMap) -> Self {
        let em_code_to_hk = ah_map.iter()
            .filter_map(|(hk, secid)| {
                secid.split('.').nth(1).map(|em| (em.to_string(), *hk))
            })
            .collect();
        let http = RatedClient::new(cfg.rps, cfg.max_retries);
        Self { inner: Arc::new(Inner { cfg, http, ah_map, em_code_to_hk }) }
    }
}

/// push2 ulist parser, schema v1: { data: { diff: [ { f12, f17, f18, ... } ] } }.
/// Requires fltt=2 in the URL so prices arrive as floats.
fn parse_ulist_v1(v: &Value) -> Result<Vec<(String, f64, f64)>, DataError> {
    let diff = v.pointer("/data/diff").and_then(Value::as_array)
        .ok_or(DataError::SchemaDrift("eastmoney ulist: /data/diff missing"))?;
    let mut out = Vec::with_capacity(diff.len());
    for item in diff {
        let (Some(code), Some(open), Some(prev)) = (
            item.get("f12").and_then(Value::as_str).map(str::to_string),
            item.get("f17").and_then(Value::as_f64),
            item.get("f18").and_then(Value::as_f64),
        ) else { continue }; // suspended / not-yet-printed names drop out
        out.push((code, open, prev));
    }
    Ok(out)
}

#[async_trait]
impl LinkedMarketFeed for EastMoneyClient {
    async fn mainland_open_prints(&self, ah_codes: &[StockCode])
        -> Result<DataFrame, DataError>
    {
        let inner = &self.inner;
        let secids: Vec<&str> = ah_codes.iter()
            .filter_map(|c| inner.ah_map.get(c).map(String::as_str))
            .collect();
        if secids.is_empty() {
            return Err(DataError::Config("no requested codes present in AH map".into()));
        }
        let url = inner.cfg.quote_url_template.replace("{secids}", &secids.join(","));
        let v: Value = inner.http.get_json(&url, &[]).await?;
        let rows = match inner.cfg.schema_version {
            1 => parse_ulist_v1(&v)?,
            _ => return Err(DataError::SchemaDrift("eastmoney ulist: unknown schema_version")),
        };
        let (mut hk_codes, mut rets, mut a_codes) = (vec![], vec![], vec![]);
        for (em_code, open, prev) in rows {
            let Some(hk) = inner.em_code_to_hk.get(&em_code) else { continue };
            if open > 0.0 && prev > 0.0 {
                hk_codes.push(hk.0);
                rets.push((open / prev).ln());
                a_codes.push(em_code);
            }
        }
        Ok(df!(cols::CODE => hk_codes, cols::A_OPEN_RET => rets, "a_code" => a_codes)?)
    }

    async fn subscribe_a50(&self) -> Result<BoxStream<'static, MarketEvent>, DataError> {
        let inner = self.inner.clone();
        let (tx, rx) = tokio::sync::mpsc::channel::<MarketEvent>(256);
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(Duration::from_secs(inner.cfg.a50_poll_secs));
            tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            let url = inner.cfg.a50_url_template.replace("{secid}", &inner.cfg.a50_secid);
            let s = SessionTimes::get();
            loop {
                tick.tick().await;
                if tx.is_closed() { break; }
                let now = Utc::now().with_timezone(&Hong_Kong);
                if inner.cfg.a50_window_enforced
                    && (now.time() < s.pos_start || now.time() > s.open)
                {
                    if now.time() > s.open { break; } // S6 window over for today
                    continue;
                }
                match inner.http.get_json::<Value>(&url, &[]).await {
                    // v1: { data: { f43: <last px, float with fltt=2> } }
                    Ok(v) => {
                        if let Some(px) = v.pointer("/data/f43").and_then(Value::as_f64) {
                            if tx.send(MarketEvent::A50 { ts: now, px }).await.is_err() { return; }
                        } else {
                            tracing::warn!("a50 payload missing /data/f43 (schema drift?)");
                        }
                    }
                    Err(e) => tracing::warn!(error = %e, "a50 poll failed; next tick"),
                }
            }
        });
        Ok(ReceiverStream::new(rx).boxed())
    }
}

#[async_trait]
impl FlowProvider for EastMoneyClient {
    async fn southbound_net_buy(&self, d: NaiveDate) -> Result<DataFrame, DataError> {
        let inner = &self.inner;
        let url = inner.cfg.southbound_url_template.replace("{date}", &d.to_string());
        let v: Value = inner.http.get_json(&url, &[]).await?;
        let rows = match inner.cfg.schema_version {
            // v1: { result: { data: [ { SECURITY_CODE, NET_BUY_AMT } ] } } — VERIFY.
            1 => v.pointer("/result/data").and_then(Value::as_array).cloned()
                .ok_or(DataError::SchemaDrift("eastmoney southbound: /result/data missing"))?,
            _ => return Err(DataError::SchemaDrift("eastmoney southbound: unknown schema_version")),
        };
        let (mut codes, mut net) = (vec![], vec![]);
        for r in &rows {
            let (Some(code), Some(amt)) = (
                r.get("SECURITY_CODE").and_then(Value::as_str).and_then(StockCode::parse),
                r.get("NET_BUY_AMT").and_then(Value::as_f64),
            ) else { continue };
            codes.push(code.0);
            net.push(amt);
        }
        if codes.is_empty() {
            return Err(DataError::SchemaDrift("eastmoney southbound: zero rows parsed"));
        }
        let n = codes.len();
        Ok(df!(
            cols::CODE => codes, cols::NET_BUY => net,
            "unit" => vec![inner.cfg.southbound_unit.clone(); n],
            cols::DATE => vec![d.to_string(); n],
        )?)
    }
}
```

```rust
// crates/hkq-data/src/xueqiu.rs
//! Xueqiu: session-cookie authenticated JSON. Research tier ONLY — rate-limited and
//! ToS-flagged per report §5; used for AH cross-checks, never as a production feed.
//! Deliberately implements NO core provider trait: it cannot back POS, bars, or flows.
use crate::{error::DataError, http::RatedClient};
use serde::Deserialize;
use serde_json::Value;

fn default_rps() -> u32 { 2 }

#[derive(Debug, Clone, Deserialize)]
pub struct XueqiuCfg {
    /// e.g. "https://stock.xueqiu.com/v5/stock/realtime/quotec.json?symbol={symbols}"
    pub quote_url_template: String,
    /// Full Cookie header value for an authenticated session (config/secret store).
    pub cookie: String,
    #[serde(default = "default_rps")]
    pub rps: u32,
}

pub struct XueqiuClient {
    cfg: XueqiuCfg,
    http: RatedClient,
}

impl XueqiuClient {
    pub fn new(cfg: XueqiuCfg) -> Self {
        let http = RatedClient::new(cfg.rps, 2);
        Self { cfg, http }
    }

    /// Raw batch quotes (e.g. ["SH600941","00941"]). Caller owns interpretation;
    /// this stays Value-typed precisely because the schema is not a contract.
    pub async fn batch_quotes(&self, symbols: &[&str]) -> Result<Value, DataError> {
        let url = self.cfg.quote_url_template.replace("{symbols}", &symbols.join(","));
        self.http.get_json(&url, &[("Cookie", self.cfg.cookie.as_str())]).await
    }
}
```

```rust
// crates/hkq-data/src/yahoo.rs
//! Yahoo Finance chart API — RESEARCH/EOD prototyping only, feature-gated so it
//! cannot be linked into live binaries. Report constraints honored in code: EOD
//! only; 1m history is a trailing window; no auction data; survivorship bias.
//! This adapter REFUSES to implement AuctionFeed/IntradayFeed by design.
#![cfg(feature = "research")]

use crate::{error::DataError, http::RatedClient, model::cols, provider::EodProvider};
use async_trait::async_trait;
use chrono::NaiveDate;
use hkq_core::ids::StockCode;
use polars::{df, prelude::*};
use serde_json::Value;

pub struct YahooClient {
    http: RatedClient,
    base: String, // "https://query1.finance.yahoo.com/v8/finance/chart"
}

impl YahooClient {
    pub fn new(base: impl Into<String>) -> Self {
        Self { http: RatedClient::new(2, 3), base: base.into() }
    }
}

/// Yahoo HK symbols are 4-digit for codes ≤ 9999 ("0700.HK"), else bare digits.
fn yahoo_symbol(code: StockCode) -> String {
    if code.0 <= 9999 { format!("{:04}.HK", code.0) } else { format!("{}.HK", code.0) }
}

#[async_trait]
impl EodProvider for YahooClient {
    async fn daily_bars(&self, codes: &[StockCode], from: NaiveDate, to: NaiveDate)
        -> Result<DataFrame, DataError>
    {
        let p1 = hkq_core::session::hk(from, chrono::NaiveTime::MIN).timestamp();
        let p2 = hkq_core::session::hk(
            to.succ_opt().unwrap_or(to), chrono::NaiveTime::MIN).timestamp();
        let (mut c_, mut d_, mut o_, mut h_, mut l_, mut cl, mut ad, mut v_, mut t_) =
            (vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]);

        for &code in codes {
            let url = format!(
                "{}/{}?period1={p1}&period2={p2}&interval=1d&events=div%2Csplit",
                self.base, yahoo_symbol(code));
            let v: Value = self.http.get_json(&url, &[]).await?;
            let r = v.pointer("/chart/result/0")
                .ok_or(DataError::SchemaDrift("yahoo chart: /chart/result/0"))?;
            let ts = r.pointer("/timestamp").and_then(Value::as_array).cloned()
                .unwrap_or_default();
            let q = r.pointer("/indicators/quote/0")
                .ok_or(DataError::SchemaDrift("yahoo chart: quote block"))?;
            let adj = r.pointer("/indicators/adjclose/0/adjclose").and_then(Value::as_array);
            let arr = |k: &str| q.get(k).and_then(Value::as_array).cloned().unwrap_or_default();
            let (o, h, l, c, vol) = (arr("open"), arr("high"), arr("low"), arr("close"),
                                     arr("volume"));
            for i in 0..ts.len() {
                let (Some(t), Some(oo), Some(hh), Some(ll), Some(cc)) = (
                    ts[i].as_i64(),
                    o.get(i).and_then(Value::as_f64), h.get(i).and_then(Value::as_f64),
                    l.get(i).and_then(Value::as_f64), c.get(i).and_then(Value::as_f64),
                ) else { continue };
                let Some(hkt) = crate::model::ms_to_hkt(t * 1000) else { continue };
                c_.push(code.0);
                d_.push(hkt.date_naive().to_string());
                o_.push(oo); h_.push(hh); l_.push(ll); cl.push(cc);
                ad.push(adj.and_then(|a| a.get(i)).and_then(Value::as_f64).unwrap_or(cc));
                v_.push(vol.get(i).and_then(Value::as_f64).unwrap_or(0.0));
                t_.push(f64::NAN); // Yahoo has no HKD turnover — recorded as absent
            }
        }
        Ok(df!(
            cols::CODE => c_, cols::DATE => d_,
            cols::OPEN => o_, cols::HIGH => h_, cols::LOW => l_,
            cols::CLOSE => cl, cols::ADJ_CLOSE => ad,
            cols::VOLUME => v_, cols::TURNOVER => t_,
        )?)
    }
}
```

```rust
// crates/hkq-data/src/lake.rs
//! Hive-partitioned parquet lake. Idempotent by construction (§5): tmp file, fsync,
//! atomic rename. A `_manifest.json` per partition (rows, columns, source, parser
//! schema version) is the cheap tripwire for silent vendor schema drift.
//!
//! Determinism choice: files carry an ISO `date` string column and scans DISABLE
//! hive inference — no dtype guessing. `scan_date` is the partition-pruned fast path.
use crate::error::DataError;
use polars::prelude::*;
use serde::Serialize;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dataset {
    DailyBars, Bars1m, Auction, Flows, Ccass, MainlandPrints, Fx, Vhsi, Fills, Attribution,
}

impl Dataset {
    pub fn dir(self) -> &'static str {
        match self {
            Dataset::DailyBars => "daily_bars",  Dataset::Bars1m => "bars_1m",
            Dataset::Auction => "auction",       Dataset::Flows => "flows",
            Dataset::Ccass => "ccass",           Dataset::MainlandPrints => "mainland",
            Dataset::Fx => "fx",                 Dataset::Vhsi => "vhsi",
            Dataset::Fills => "fills",           Dataset::Attribution => "attribution",
        }
    }
}

#[derive(Serialize)]
struct Manifest<'a> {
    dataset: &'a str,
    date: String,
    rows: usize,
    columns: Vec<String>,
    source: &'a str,
    schema_version: u32,
    written_at_utc: String,
}

pub struct Lake { root: PathBuf }

impl Lake {
    pub fn new(root: impl AsRef<Path>) -> Self { Self { root: root.as_ref().into() } }

    fn partition_dir(&self, ds: Dataset, date: chrono::NaiveDate) -> PathBuf {
        self.root.join(ds.dir()).join(format!("date={date}"))
    }

    pub fn exists(&self, ds: Dataset, date: chrono::NaiveDate) -> bool {
        self.partition_dir(ds, date).join("part.parquet").exists()
    }

    /// Idempotent atomic write: tmp → fsync(file) → rename → fsync(dir, unix).
    /// Re-running a nightly job is always safe (§5 "ingestion must be idempotent").
    pub fn write_partition(
        &self, ds: Dataset, date: chrono::NaiveDate, df: &mut DataFrame,
        source: &str, schema_version: u32,
    ) -> Result<(), DataError> {
        let dir = self.partition_dir(ds, date);
        std::fs::create_dir_all(&dir)?;

        let tmp = dir.join(".part.tmp");
        let fin = dir.join("part.parquet");
        {
            let f = File::create(&tmp)?;
            ParquetWriter::new(&f)
                .with_compression(ParquetCompression::Zstd(None))
                .finish(df)?;
            f.sync_all()?;
        }
        std::fs::rename(&tmp, &fin)?;

        let manifest = Manifest {
            dataset: ds.dir(),
            date: date.to_string(),
            rows: df.height(),
            columns: df.get_column_names().iter().map(|s| s.to_string()).collect(),
            source,
            schema_version,
            written_at_utc: chrono::Utc::now().to_rfc3339(),
        };
        let mtmp = dir.join(".manifest.tmp");
        std::fs::write(&mtmp, serde_json::to_vec_pretty(&manifest)
            .map_err(|e| DataError::Parse { context: "manifest", msg: e.to_string() })?)?;
        std::fs::rename(&mtmp, dir.join("_manifest.json"))?;

        #[cfg(unix)]
        if let Ok(d) = File::open(&dir) { let _ = d.sync_all(); }

        tracing::info!(dataset = ds.dir(), %date, rows = df.height(), source, "partition written");
        Ok(())
    }

    /// Full-dataset scan (hive inference OFF; `date` is an in-file string column).
    pub fn scan(&self, ds: Dataset) -> Result<LazyFrame, DataError> {
        let glob = self.root.join(ds.dir()).join("date=*").join("part.parquet");
        Ok(LazyFrame::scan_parquet(
            glob.to_string_lossy().as_ref(),
            ScanArgsParquet {
                hive_options: HiveOptions { enabled: Some(false), ..Default::default() },
                ..Default::default()
            },
        )?)
    }

    /// Partition-pruned fast path for a single date.
    pub fn scan_date(&self, ds: Dataset, date: chrono::NaiveDate) -> Result<LazyFrame, DataError> {
        let path = self.partition_dir(ds, date).join("part.parquet");
        if !path.exists() {
            return Err(DataError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("missing partition {}", path.display()),
            )));
        }
        Ok(LazyFrame::scan_parquet(
            path.to_string_lossy().as_ref(),
            ScanArgsParquet {
                hive_options: HiveOptions { enabled: Some(false), ..Default::default() },
                ..Default::default()
            },
        )?)
    }

    /// Move a bad partition aside (recon breach ⇒ quarantine + page the operator).
    pub fn quarantine(&self, ds: Dataset, date: chrono::NaiveDate) -> Result<PathBuf, DataError> {
        let src = self.partition_dir(ds, date);
        let dst_dir = self.root.join("_quarantine").join(ds.dir());
        std::fs::create_dir_all(&dst_dir)?;
        let dst = dst_dir.join(format!("date={date}"));
        if dst.exists() { std::fs::remove_dir_all(&dst)?; }
        std::fs::rename(&src, &dst)?;
        tracing::error!(dataset = ds.dir(), %date, dst = %dst.display(), "partition QUARANTINED");
        Ok(dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    #[test]
    fn write_scan_roundtrip_and_idempotency() {
        let root = std::env::temp_dir().join(format!(
            "hkq_lake_test_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let lake = Lake::new(&root);
        let date = chrono::NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();

        let mut df0 = df!(
            "code" => vec![700u32, 5u32],
            "close" => vec![321.4f64, 60.05],
            "date" => vec![date.to_string(), date.to_string()],
        ).unwrap();

        lake.write_partition(Dataset::DailyBars, date, &mut df0, "test", 1).unwrap();
        assert!(lake.exists(Dataset::DailyBars, date));
        // Idempotent re-write over the same partition.
        lake.write_partition(Dataset::DailyBars, date, &mut df0, "test", 1).unwrap();

        let out = lake.scan_date(Dataset::DailyBars, date).unwrap().collect().unwrap();
        assert_eq!(out.height(), 2);
        assert!(out.get_column_names().iter().any(|c| c.as_str() == "close"));

        std::fs::remove_dir_all(root).ok();
    }
}
```

```rust
// crates/hkq-data/src/recon.rs
//! Nightly reconciliation (§5): our stored close vs the official closing-auction
//! print. Any |Δ| > 1 tick ⇒ caller quarantines the partition and pages the operator.
//! Comparison is tick-aware via hkq-core's HKEX spread table, in Decimal.
use crate::error::DataError;
use crate::lake::{Dataset, Lake};
use hkq_core::money::hk_tick_size;
use polars::prelude::*;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CloseBreach {
    pub code: u32,
    pub ours: f64,
    pub official: f64,
    pub abs_diff: Decimal,
    pub tick: Decimal,
}

#[derive(Debug, Default)]
pub struct ReconReport {
    pub checked: usize,
    pub breaches: Vec<CloseBreach>,
    pub missing_ours: Vec<u32>,     // official has it, our lake doesn't
    pub missing_official: Vec<u32>, // we stored it, official source doesn't
}

impl ReconReport {
    pub fn clean(&self) -> bool {
        self.breaches.is_empty() && self.missing_ours.is_empty()
    }
}

/// `official` schema: [code: u32, close_official: f64].
pub fn reconcile_closes(
    lake: &Lake, official: &DataFrame, date: chrono::NaiveDate,
) -> Result<ReconReport, DataError> {
    let ours_df = lake.scan_date(Dataset::DailyBars, date)?
        .select([col("code"), col("close")])
        .collect()?;

    let codes = ours_df.column("code")?.as_materialized_series().u32()?;
    let closes = ours_df.column("close")?.as_materialized_series().f64()?;
    let mut ours: HashMap<u32, f64> = HashMap::with_capacity(ours_df.height());
    for (c, px) in codes.into_no_null_iter().zip(closes.into_no_null_iter()) {
        ours.insert(c, px);
    }

    let ocodes = official.column("code")?.as_materialized_series().u32()?;
    let ocloses = official.column("close_official")?.as_materialized_series().f64()?;

    let mut report = ReconReport::default();
    let mut seen: HashMap<u32, ()> = HashMap::new();

    for (c, off) in ocodes.into_no_null_iter().zip(ocloses.into_no_null_iter()) {
        seen.insert(c, ());
        let Some(&mine) = ours.get(&c) else {
            report.missing_ours.push(c);
            continue;
        };
        report.checked += 1;
        let d_off = Decimal::from_f64(off).unwrap_or_default().round_dp(4);
        let d_mine = Decimal::from_f64(mine).unwrap_or_default().round_dp(4);
        let tick = hk_tick_size(d_off);
        let abs_diff = (d_mine - d_off).abs();
        if abs_diff > tick {
            report.breaches.push(CloseBreach { code: c, ours: mine, official: off, abs_diff, tick });
        }
    }
    for c in ours.keys() {
        if !seen.contains_key(c) { report.missing_official.push(*c); }
    }
    Ok(report)
}
```

```rust
// crates/hkq-data/src/ingest.rs
//! Orchestration. Nightly: pull EOD / flows / mainland prints and write partitions,
//! idempotently, with per-step failure isolation. Live: fan provider streams into
//! one bounded mpsc<MarketEvent> — the engine actor (M4) is the single consumer.
use crate::error::DataError;
use crate::lake::{Dataset, Lake};
use crate::model::{AuctionSnap, Bar1m, MarketEvent};
use crate::provider::{EodProvider, FlowProvider, LinkedMarketFeed};
use chrono::NaiveDate;
use futures::{stream::BoxStream, StreamExt};
use hkq_core::ids::StockCode;
use tokio::sync::mpsc;
use tokio::task::JoinSet;

#[derive(Debug)]
pub enum StepOutcome {
    Written { rows: usize },
    SkippedExisting,
    SkippedNoProvider,
    Failed(String),
}

#[derive(Debug)]
pub struct StepReport {
    pub name: &'static str,
    pub outcome: StepOutcome,
}

#[derive(Debug)]
pub struct IngestReport {
    pub date: NaiveDate,
    pub steps: Vec<StepReport>,
}

impl IngestReport {
    pub fn all_ok(&self) -> bool {
        !self.steps.iter().any(|s| matches!(s.outcome, StepOutcome::Failed(_)))
    }
}

pub struct NightlyIngest<'a> {
    pub lake: &'a Lake,
    pub eod: &'a dyn EodProvider,
    pub flows: Option<&'a dyn FlowProvider>,
    pub linked: Option<&'a dyn LinkedMarketFeed>,
}

impl<'a> NightlyIngest<'a> {
    /// Run all nightly steps for `date`. `force` re-writes existing partitions.
    /// Failures are isolated per step: one broken vendor never blocks the others.
    pub async fn run(
        &self, date: NaiveDate, codes: &[StockCode], ah_codes: &[StockCode], force: bool,
    ) -> IngestReport {
        let mut steps = Vec::new();

        steps.push(self.step_daily_bars(date, codes, force).await);
        steps.push(self.step_flows(date, force).await);
        steps.push(self.step_mainland(date, ah_codes, force).await);

        let report = IngestReport { date, steps };
        for s in &report.steps {
            match &s.outcome {
                StepOutcome::Failed(e) =>
                    tracing::error!(step = s.name, error = %e, "nightly step FAILED"),
                o => tracing::info!(step = s.name, outcome = ?o, "nightly step done"),
            }
        }
        report
    }

    async fn step_daily_bars(
        &self, date: NaiveDate, codes: &[StockCode], force: bool,
    ) -> StepReport {
        let name = "daily_bars";
        if !force && self.lake.exists(Dataset::DailyBars, date) {
            return StepReport { name, outcome: StepOutcome::SkippedExisting };
        }
        let outcome = match self.eod.daily_bars(codes, date, date).await {
            Ok(mut df) => match self.lake
                .write_partition(Dataset::DailyBars, date, &mut df, "tiger:kline", 1)
            {
                Ok(()) => StepOutcome::Written { rows: df.height() },
                Err(e) => StepOutcome::Failed(e.to_string()),
            },
            Err(e) => StepOutcome::Failed(e.to_string()),
        };
        StepReport { name, outcome }
    }

    async fn step_flows(&self, date: NaiveDate, force: bool) -> StepReport {
        let name = "southbound_flows";
        let Some(flows) = self.flows else {
            return StepReport { name, outcome: StepOutcome::SkippedNoProvider };
        };
        if !force && self.lake.exists(Dataset::Flows, date) {
            return StepReport { name, outcome: StepOutcome::SkippedExisting };
        }
        let outcome = match flows.southbound_net_buy(date).await {
            Ok(mut df) => match self.lake
                .write_partition(Dataset::Flows, date, &mut df, "eastmoney:hsgt", 1)
            {
                Ok(()) => StepOutcome::Written { rows: df.height() },
                Err(e) => StepOutcome::Failed(e.to_string()),
            },
            Err(e) => StepOutcome::Failed(e.to_string()),
        };
        StepReport { name, outcome }
    }

    async fn step_mainland(
        &self, date: NaiveDate, ah_codes: &[StockCode], force: bool,
    ) -> StepReport {
        let name = "mainland_prints";
        let Some(linked) = self.linked else {
            return StepReport { name, outcome: StepOutcome::SkippedNoProvider };
        };
        if ah_codes.is_empty() {
            return StepReport { name, outcome: StepOutcome::SkippedNoProvider };
        }
        if !force && self.lake.exists(Dataset::MainlandPrints, date) {
            return StepReport { name, outcome: StepOutcome::SkippedExisting };
        }
        let outcome = match linked.mainland_open_prints(ah_codes).await {
            Ok(mut df) => match self.lake
                .write_partition(Dataset::MainlandPrints, date, &mut df, "eastmoney:ulist", 1)
            {
                Ok(()) => StepOutcome::Written { rows: df.height() },
                Err(e) => StepOutcome::Failed(e.to_string()),
            },
            Err(e) => StepOutcome::Failed(e.to_string()),
        };
        StepReport { name, outcome }
    }
}

/// Live-day fan-in: N provider streams → one bounded channel (backpressure by
/// construction), each pump a supervised task. The engine actor is the sole reader.
pub struct LiveMux {
    tx: mpsc::Sender<MarketEvent>,
    tasks: JoinSet<()>,
}

impl LiveMux {
    pub fn new(capacity: usize) -> (Self, mpsc::Receiver<MarketEvent>) {
        let (tx, rx) = mpsc::channel(capacity);
        (Self { tx, tasks: JoinSet::new() }, rx)
    }

    pub fn pump_auction(&mut self, mut s: BoxStream<'static, AuctionSnap>) {
        let tx = self.tx.clone();
        self.tasks.spawn(async move {
            while let Some(snap) = s.next().await {
                if tx.send(MarketEvent::Auction(snap)).await.is_err() { break; }
            }
            tracing::info!("auction pump finished");
        });
    }

    pub fn pump_bars(&mut self, mut s: BoxStream<'static, Bar1m>) {
        let tx = self.tx.clone();
        self.tasks.spawn(async move {
            while let Some(bar) = s.next().await {
                if tx.send(MarketEvent::Bar(bar)).await.is_err() { break; }
            }
            tracing::info!("bar pump finished");
        });
    }

    pub fn pump_events(&mut self, mut s: BoxStream<'static, MarketEvent>) {
        let tx = self.tx.clone();
        self.tasks.spawn(async move {
            while let Some(ev) = s.next().await {
                if tx.send(ev).await.is_err() { break; }
            }
            tracing::info!("event pump finished");
        });
    }

    /// Abort all pumps and drain join handles (used by the kill path).
    pub async fn shutdown(mut self) {
        self.tasks.abort_all();
        while self.tasks.join_next().await.is_some() {}
    }
}
```

## `hkq-nightly` binary

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
tracing.workspace = true
tracing-subscriber.workspace = true
```

```rust
// crates/hkq-nightly/src/main.rs
//! T−1 18:00 nightly job: EOD + flows + mainland prints → lake, then reconciliation.
//! Usage: hkq-nightly <strategy.toml> [YYYY-MM-DD] [--force]
use anyhow::Context;
use chrono::{NaiveDate, Utc};
use chrono_tz::Asia::Hong_Kong;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg, ids::StockCode,
               session::{DayKind, TradingCalendar}};
use hkq_data::{cfg::load_sources, eastmoney::{load_ah_map, EastMoneyClient},
               ingest::NightlyIngest, lake::Lake,
               provider::{FlowProvider, LinkedMarketFeed}, tiger::TigerClient};

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

    let report = ingest.run(date, &codes, &ah_codes, force).await;

    // Close reconciliation needs an INDEPENDENT official-close source (HKEX daily
    // quotes / OMD-C EOD, report §5). Reconciling Tiger against Tiger is circular,
    // so until that source is wired we log the gap rather than fake the check.
    tracing::warn!("recon skipped: independent official-close source not configured (M1 gap)");

    anyhow::ensure!(report.all_ok(), "one or more nightly steps failed: {report:?}");
    tracing::info!(%date, "nightly ingest complete");
    Ok(())
}
```

## Example configuration

```toml
# config/strategy.toml (M1-relevant excerpts; §3 constants included for contract completeness)
[universe]
min_median_turnover_hkd = 30_000_000.0
min_price_hkd = 1.0
min_listed_days = 60
max_median_spread_bps = 35.0

[factors]
ewma_halflife_days = 21.0
ewma_min_obs = 63
amihud_window = 60
rv_days = 5
lav_gamma = 0.3
seasonal_vol_days = 20
ivu_tercile_window = 60

[stage1]
theta1 = 1.0
theta2 = 1.0
eta = 0.25
vs_threshold = 0.5        # v* prior — hkq-validate owns the refresh
leadlag_window = 250
fdr_q = 0.10
icir_window = 250
icir_shrink_delta = 0.10
top_k_sectors = 2
sigma_min_gate = 0.5      # Σ_min prior
member_weight_cap = 0.15

[stage2]
phi = 2.0
zeta = 0.5
vs_threshold_stock = 0.5
beta_window = 250
winsor_pct = 0.01
ortho_order = ["x5", "x3", "x2", "x1", "x6", "x7"]
names_per_sector = 2

[trade]
margin_bps = 10.0
stop_sigma15m_mult = 2.5
participation_cap = 0.02
half_day_mode = "skip"
reuse_unsettled_proceeds = false

[costs]
stamp_bps_per_side = 10.0
fees_bps_roundtrip = 2.2
impact_kappa = 0.1

[ops]
lake_root = "data/lake"
calendar_path = "config/calendar_hk.json"
universe_codes_path = "config/universe_codes.txt"
ah_map_path = "config/ah_map.csv"
log_json = false

[sources.tiger]
gateway = "https://openapi.tigersecurities.com/gateway"
tiger_id = "YOUR_TIGER_ID"
private_key_path = "secrets/tiger_rsa.pem"
# quote_method / kline_method / kline_adjust: VERIFY against your account API version

[sources.eastmoney]
quote_url_template = "https://push2.eastmoney.com/api/qt/ulist.np/get?fltt=2&fields=f1,f12,f17,f18&secids={secids}"
a50_url_template   = "https://push2.eastmoney.com/api/qt/stock/get?fltt=2&fields=f43&secid={secid}"
a50_secid          = "VERIFY_A50_SECID"
southbound_url_template = "https://datacenter-web.eastmoney.com/api/data/v1/get?VERIFIED_SOUTHBOUND_QUERY&date={date}"
southbound_unit    = "CNY_wan_unverified"
schema_version     = 1
```

```json
// config/calendar_hk.json (maintained from the HKEX calendar; excerpt)
{
  "holidays":  ["2026-07-01", "2026-10-01"],
  "half_days": ["2026-12-24", "2026-12-31"]
}
```

## Honest gaps and hand-off to Milestone 2

Everything flagged `VERIFY` is the blueprint's acknowledged vendor-verification surface, now concentrated in config rather than code: Tiger's method names, biz-field names, timestamp timezone, adjust-mode parameter, and HK auction field names (the alias table + subscribe-time probe make absence non-fatal — `Unsupported` triggers the documented X2-disabled mode); EastMoney's southbound query and A50 secid are config templates whose parsers fail loudly with `SchemaDrift`. The one structural gap is reconciliation's independent official-close source — the recon function and quarantine path are built and tested, but the nightly binary deliberately logs the gap instead of reconciling Tiger against itself. Milestone 2 (the `hkq-factors` crate) consumes this layer through exactly three seams: `Lake::scan`/`scan_date` for the nightly panels, the `cols` schema contract, and `LiveMux`'s `mpsc<MarketEvent>` for the 09:29:30 `OpenContext` freeze.
