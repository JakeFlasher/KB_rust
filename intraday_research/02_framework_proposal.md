
The full text of `01_report.md` is already attached in your message, so I worked directly from that content rather than re-fetching the URL. Below is the implementation blueprint, built strictly against the report's specification (section references like §3.2 refer to the report).

Before the code, the five report constraints that drive every architectural decision: the 20 bps stamp-duty floor forces a *conditional, concentrated* system (cash is a first-class position; the engine must be able to do nothing); alpha is cross-sectional, so the factor engine is organized around per-date panel transforms, not per-symbol time series; the overnight/intraday decomposition (\(r^{ON}\) vs \(r^{ID}\)) must exist as separate, typed columns everywhere; every opening signal is regime-gated, so regime state is part of the context object, not an afterthought; and the validation protocol (purged CV, DSR, trials registry, CUSUM kill switch) is a crate, not a notebook.

# HKQ — Workspace Topology

```text
hkq/
├── Cargo.toml                      # workspace + pinned shared deps
├── config/
│   ├── strategy.toml               # every constant from the report, no magic numbers in code
│   ├── calendar_hk.json            # trading days, half-days (τ ≤ 5 collapse)
│   └── universe_static.parquet     # board lots, AH/ADR maps, Connect eligibility (point-in-time)
├── crates/
│   ├── hkq-core                    # ids, money (Decimal), sessions, calendar, config, errors
│   ├── hkq-data                    # async ingestion: providers, rate limits, parquet lake, recon
│   ├── hkq-factors                 # polars factor engine: moments, S1–S6, X1–X8, transforms
│   ├── hkq-signal                  # ICIR combination, gates, confirmation χ, meta-label α̂
│   ├── hkq-risk                    # cost floor c_i, Decimal sizing, board lots, stops, kill switch
│   ├── hkq-exec                    # order slicing, participation pacing, VCM awareness, Tiger routing
│   ├── hkq-validate                # purged CV, NW t-stats, DSR, CUSUM, trials registry
│   └── hkq-engine                  # runbook state machine, actor wiring, backtest replay
└── bins: hkq-nightly / hkq-live / hkq-backtest / hkq-recon
```

Dataflow (live day):

```text
 T−1 18:00 nightly job                     09:00 →                16:10
┌───────────────────┐      ┌────────────────────────────────────────────┐
│ hkq-data ingest   │      │ tiger POS poller ─┐                        │
│  EOD/flows/CCASS  │      │ eastmoney A-shr ──┤  mpsc<MarketEvent>     │
│        ↓          │      │ a50 futures ──────┼──────► engine actor    │
│ hkq-factors       │      │ 1m bars/quotes ───┘        (single writer) │
│  moments panel    │      │                                │           │
│  seasonal profiles│      │            OrderIntent ────────▼           │
│  lead–lag graph   │      │            hkq-risk gate → hkq-exec actor  │
│  ICIR weights     │      │                     fills ◄──────┘         │
│  AlphaMap (a,b)   │      │ watch<RiskState> (CUSUM kill) ─── all      │
└───────────────────┘      └────────────────────────────────────────────┘
```

A deliberate boundary on numerics, since you asked for `rust_decimal`: all *statistical* math (factors, ranks, ICs, regressions) runs in `f64` inside polars/nalgebra — rank-inverse-normal transforms in fixed-point are pointless and slow. All *accounting* math (cash, prices, quantities, fees, stamp duty) is `rust_decimal::Decimal` behind newtypes, and `f64` never touches an order. The conversion happens exactly once, in `hkq-risk::size_book`, with an explicit rounding policy. This is the idiomatic split; pretending factor z-scores need decimal precision would be cargo-cult safety.

# Workspace `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.dependencies]
tokio       = { version = "1.38", features = ["full"] }
reqwest     = { version = "0.12", features = ["json", "gzip", "rustls-tls"] }
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
polars      = { version = "0.46", features = [
  "lazy", "parquet", "dtype-datetime", "dtype-date", "dynamic_group_by",
  "ewma", "rank", "log", "abs", "rolling_window", "temporal", "is_in",
  "round_series", "pct_change", "cum_agg", "partition_by", "semi_anti_join",
] }
rust_decimal        = { version = "1.35", features = ["serde-str"] }
rust_decimal_macros = "1.35"
chrono      = { version = "0.4", features = ["serde"] }
chrono-tz   = "0.9"
nalgebra    = "0.33"
thiserror   = "1"
anyhow      = "1"
async-trait = "0.1"
futures     = "0.3"
governor    = "0.6"
tracing     = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
rsa         = "0.9"      # Tiger request signing
sha1        = "0.10"
base64      = "0.22"
toml        = "0.8"
```

Pin note: the polars Rust expression API drifts between minor versions (`shift(lit(1))` vs `shift(1)`, `RollingOptionsFixedWindow` naming, `map` on `Series` vs `Column`). Everything below targets 0.46; treat renames as mechanical.

# `hkq-core` — Domain Types, Sessions, Calendar

```rust
// crates/hkq-core/src/ids.rs
use serde::{Deserialize, Serialize};

/// HKEX numeric stock code (e.g. 700 → "00700"). Copy, hashable, order-stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StockCode(pub u32);

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
use rust_decimal::{Decimal, RoundingStrategy, prelude::*};
use serde::{Deserialize, Serialize};
use crate::error::CoreError;

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
    pub fn as_f64(self) -> f64 { self.0.to_f64().unwrap() } // signal side only

    /// HKEX Securities spread table (Part A): snap DOWN to valid tick (for stops/limits).
    pub fn snap_down_to_tick(self) -> Px {
        let t = hk_tick_size(self.0);
        Px((self.0 / t).floor() * t)
    }
}

/// HKEX price-dependent minimum spread table.
pub fn hk_tick_size(px: Decimal) -> Decimal {
    use rust_decimal_macros::dec;
    const _: () = (); // (upper bound, tick)
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

/// Board lot size for a listing (HK stocks trade in lots; odd lots are a separate board).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardLot(pub u32);

/// Share quantity PROVEN to be a whole number of board lots. The only quantity type
/// `hkq-exec` accepts — odd-lot orders are unrepresentable.
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
    pub fn cap_shares(self, max_shares: u64) -> Option<LotQty> {
        let n_lots = (max_shares / self.lot as u64).min(self.shares / self.lot as u64);
        (n_lots > 0).then_some(LotQty { shares: n_lots * self.lot as u64, lot: self.lot })
    }
    pub fn notional(self, px: Px) -> Cash { Cash(px.get() * Decimal::from(self.shares)) }
}
```

```rust
// crates/hkq-core/src/session.rs
use chrono::{NaiveDate, NaiveTime, DateTime};
use chrono_tz::Asia::Hong_Kong;
use chrono_tz::Tz;

/// The report's clock (§1, §6). HKT, no DST. All engine phase changes key off these.
pub mod t {
    use chrono::NaiveTime;
    pub const POS_START: NaiveTime      = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
    pub const POS_NO_CANCEL: NaiveTime  = NaiveTime::from_hms_opt(9, 15, 0).unwrap();
    pub const POS_MATCH_FROM: NaiveTime = NaiveTime::from_hms_opt(9, 20, 0).unwrap();
    pub const POS_MATCH_TO: NaiveTime   = NaiveTime::from_hms_opt(9, 22, 0).unwrap();
    pub const MAINLAND_PRINT: NaiveTime = NaiveTime::from_hms_opt(9, 25, 0).unwrap();
    pub const SCORE_FREEZE: NaiveTime   = NaiveTime::from_hms_opt(9, 29, 30).unwrap();
    pub const OPEN: NaiveTime           = NaiveTime::from_hms_opt(9, 30, 0).unwrap();
    pub const X3_REFRESH: NaiveTime     = NaiveTime::from_hms_opt(9, 35, 0).unwrap();
    pub const ENTRY: NaiveTime          = NaiveTime::from_hms_opt(9, 45, 0).unwrap();
    pub const LUNCH_CANCEL: NaiveTime   = NaiveTime::from_hms_opt(11, 58, 0).unwrap();
    pub const AM_CLOSE: NaiveTime       = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
    pub const PM_OPEN: NaiveTime        = NaiveTime::from_hms_opt(13, 0, 0).unwrap();
    pub const EXIT_START: NaiveTime     = NaiveTime::from_hms_opt(15, 30, 0).unwrap();
    pub const EXIT_END: NaiveTime       = NaiveTime::from_hms_opt(15, 45, 0).unwrap();
    pub const CONT_CLOSE: NaiveTime     = NaiveTime::from_hms_opt(16, 0, 0).unwrap();
    pub const CAS_HARD_END: NaiveTime   = NaiveTime::from_hms_opt(16, 10, 0).unwrap();
    /// Compressed-exit start on half-days (config half_day_mode = "compressed").
    pub const HALF_DAY_EXIT: NaiveTime  = NaiveTime::from_hms_opt(11, 45, 0).unwrap();
}

pub fn hk(date: NaiveDate, time: NaiveTime) -> DateTime<Tz> {
    date.and_time(time).and_local_timezone(Hong_Kong).unwrap()
}

/// Map an HKT timestamp to bin τ ∈ 1..=11 (lunch discontinuity between τ=5 and τ=6).
pub fn bin_of(ts_hkt: NaiveTime) -> Option<crate::ids::Bin> {
    let mins = |t: NaiveTime| t.signed_duration_since(t::OPEN).num_minutes();
    let m = mins(ts_hkt);
    let tau = if (0..150).contains(&m) { 1 + (m / 30) as u8 }              // 09:30–12:00 → τ 1..=5
        else {
            let pm = ts_hkt.signed_duration_since(t::PM_OPEN).num_minutes();
            if (0..180).contains(&pm) { 6 + (pm / 30) as u8 } else { return None } // 13:00–16:00 → τ 6..=11
        };
    crate::ids::Bin::new(tau)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayKind { Full, HalfDay, Closed }

pub trait TradingCalendar: Send + Sync {
    fn day_kind(&self, d: NaiveDate) -> DayKind;
    fn prev_trading_day(&self, d: NaiveDate) -> NaiveDate;
}
```

```rust
// crates/hkq-core/src/config.rs — every constant in the report, named and sourced.
use serde::Deserialize;

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

#[derive(Debug, Clone, Deserialize)]
pub struct UniverseCfg {           // §3.1
    pub min_median_turnover_hkd: f64,   // 30_000_000
    pub min_price_hkd: f64,             // 1.0
    pub min_listed_days: u32,           // 60
    pub max_median_spread_bps: f64,     // 35.0
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactorCfg {             // §3.0
    pub ewma_halflife_days: f64,        // 21
    pub ewma_min_obs: usize,            // 63
    pub amihud_window: usize,           // 60
    pub rv_days: usize,                 // 5
    pub lav_gamma: f64,                 // 0.3
    pub seasonal_vol_days: usize,       // 20
    pub ivu_tercile_window: usize,      // 60
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stage1Cfg {             // §3.2
    pub theta1: f64, pub theta2: f64,   // 1.0, 1.0 (quarterly re-fit, 2y window)
    pub eta: f64,                       // 0.25
    pub vs_threshold: f64,              // v* — estimated, see hkq-validate::thresholds
    pub leadlag_window: usize,          // 250
    pub fdr_q: f64,                     // 0.10
    pub icir_window: usize,             // 250
    pub icir_shrink_delta: f64,         // 0.10
    pub top_k_sectors: usize,           // 2..=3
    pub sigma_min_gate: f64,            // Σ_min — absolute quality gate; below ⇒ cash day
    pub member_weight_cap: f64,         // 0.15
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stage2Cfg {             // §3.3–§3.4
    pub phi: f64,                       // 2.0 (X1)
    pub zeta: f64,                      // X5 second-term weight
    pub beta_window: usize,             // 250
    pub winsor_pct: f64,                // 0.01
    pub ortho_order: Vec<String>,       // ["x5","x3","x2","x1","x6","x7"] — fixed (§3.4)
    pub names_per_sector: usize,        // m ∈ {2,4}
}

#[derive(Debug, Clone, Deserialize)]
pub struct TradeCfg {              // §3.5–§3.6
    pub margin_bps: f64,                // m* = 10
    pub stop_sigma15m_mult: f64,        // 2.5
    pub participation_cap: f64,         // 0.02 of projected interval volume
    pub half_day_mode: HalfDayMode,     // Skip | Compressed
    pub reuse_unsettled_proceeds: bool, // broker-dependent T+2 flag (§1)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HalfDayMode { Skip, Compressed }

#[derive(Debug, Clone, Deserialize)]
pub struct CostCfg {               // §1
    pub stamp_bps_per_side: f64,        // 10.0
    pub fees_bps_roundtrip: f64,        // 2.2
    pub impact_kappa: f64,              // κ, calibrated from fills
}
```

# `hkq-data` — Async Ingestion, Lake, Reconciliation

Provider capabilities are separate traits so each vendor implements only what it truly has — the report is explicit that Yahoo-class sources must never back the POS feed, and that Tiger's auction fields are optional-with-fallback (X2-disabled mode).

```rust
// crates/hkq-data/src/model.rs
use chrono::{DateTime, NaiveDate};
use chrono_tz::Tz;
use hkq_core::ids::StockCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar1m {
    pub code: StockCode,
    pub ts: DateTime<Tz>,     // bar OPEN time, exchange clock (HKT) — non-negotiable (§5)
    pub o: f64, pub h: f64, pub l: f64, pub c: f64,
    pub volume: f64, pub turnover: f64,
}

/// POS state snapshot (§1): {IEP_t, IEV_t} trajectory, ≤30 s cadence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionSnap {
    pub code: StockCode,
    pub ts: DateTime<Tz>,
    pub iep: Option<f64>,
    pub iev: Option<f64>,
    pub bid_qty: Option<f64>,  // for the X2 imbalance augmentation, where depth exists
    pub ask_qty: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum MarketEvent {
    Auction(AuctionSnap),
    Bar(Bar1m),
    Quote { code: StockCode, ts: DateTime<Tz>, bid: f64, ask: f64 },
    MainlandAuctionPrint { code: StockCode, a_open_ret: f64 },  // 09:25 A-share auction (S6)
    A50 { ts: DateTime<Tz>, px: f64 },
    Vcm { code: StockCode, in_cooling_off: bool },              // §1 VCM constraint
    Halt { code: StockCode },
}
```

```rust
// crates/hkq-data/src/provider.rs
use async_trait::async_trait;
use futures::stream::BoxStream;
use polars::prelude::DataFrame;
use chrono::NaiveDate;
use hkq_core::ids::StockCode;
use crate::model::*;
use crate::error::DataError;

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
    /// Per-stock southbound net buy for date d (S5/X7 raw material).
    async fn southbound_net_buy(&self, d: NaiveDate) -> Result<DataFrame, DataError>;
}

#[async_trait]
pub trait LinkedMarketFeed: Send + Sync {
    /// SSE/SZSE 09:25 auction prints for the AH subset + A50 quotes 09:00–09:30 (S6).
    async fn mainland_open_prints(&self, ah_codes: &[StockCode])
        -> Result<DataFrame, DataError>;
    async fn subscribe_a50(&self) -> Result<BoxStream<'static, MarketEvent>, DataError>;
}
```

The shared HTTP substrate — one place for rate limiting, retry with jittered backoff, and tracing:

```rust
// crates/hkq-data/src/http.rs
use governor::{Quota, RateLimiter, state::{NotKeyed, InMemoryState}, clock::DefaultClock};
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
                .build().expect("client"),
            limiter: Arc::new(RateLimiter::direct(
                Quota::per_second(NonZeroU32::new(rps).unwrap()))),
            max_retries,
        }
    }

    /// GET+JSON with token-bucket admission and exponential backoff (100ms·2^k + jitter).
    pub async fn get_json<T: serde::de::DeserializeOwned>(
        &self, url: &str, headers: &[(&str, &str)],
    ) -> Result<T, crate::error::DataError> {
        let mut attempt = 0u32;
        loop {
            self.limiter.until_ready().await;
            let mut req = self.http.get(url);
            for (k, v) in headers { req = req.header(*k, *v); }
            match req.send().await {
                Ok(r) if r.status().is_success() => return Ok(r.json::<T>().await?),
                Ok(r) if r.status().as_u16() == 429 || r.status().is_server_error() => { /* retry */ }
                Ok(r) => return Err(crate::error::DataError::Status(r.status().as_u16())),
                Err(e) if attempt < self.max_retries => { tracing::warn!(%e, "transient"); }
                Err(e) => return Err(e.into()),
            }
            attempt += 1;
            if attempt > self.max_retries { return Err(crate::error::DataError::RetriesExhausted); }
            let base = Duration::from_millis(100 * (1 << attempt.min(6)));
            tokio::time::sleep(base + Duration::from_millis(fastrand_ms(250))).await;
        }
    }
}

fn fastrand_ms(cap: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64 % cap
}
```

The Tiger adapter. Tiger OpenAPI is a signed-POST gateway (RSA private-key signature over canonicalized params); there is no official Rust SDK, so we wrap the HTTP protocol. During the POS we *poll* the quote endpoint on a 15 s cadence, which satisfies the report's ≤30 s IEP/IEV requirement without needing their push socket on day one:

```rust
// crates/hkq-data/src/tiger.rs
use async_trait::async_trait;
use futures::StreamExt;
use crate::{http::RatedClient, model::*, provider::*, error::DataError};
use hkq_core::ids::StockCode;

pub struct TigerCfg {
    pub gateway: String,          // e.g. "https://openapi.tigersecurities.com/gateway"
    pub tiger_id: String,
    pub private_key_pem: String,  // RSA key registered with Tiger
}

pub struct TigerClient {
    cfg: TigerCfg,
    http: RatedClient,
}

impl TigerClient {
    pub fn new(cfg: TigerCfg) -> Self {
        Self { cfg, http: RatedClient::new(8, 4) } // verify per-symbol subscription caps (§5)
    }

    /// Tiger request envelope: method name + params + RSA-SHA1 signature.
    /// VERIFY field names against your account's API version before go-live.
    async fn call(&self, method: &str, biz: serde_json::Value)
        -> Result<serde_json::Value, DataError>
    {
        let params = serde_json::json!({
            "tiger_id": self.cfg.tiger_id,
            "method": method,
            "charset": "UTF-8",
            "version": "2.0",
            "timestamp": chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            "biz_content": biz.to_string(),
        });
        let sign = self.sign(&params)?;      // RSA over sorted "k=v&..." — see sign()
        // POST { params + sign } to gateway; parse { code, data } envelope …
        todo!("wire POST via RatedClient; deserialize Tiger envelope")
    }

    fn sign(&self, _params: &serde_json::Value) -> Result<String, DataError> {
        // rsa + sha1 + base64 over the canonical param string, per Tiger's spec.
        todo!("RSA-SHA1 canonical signing")
    }
}

#[async_trait]
impl AuctionFeed for TigerClient {
    async fn subscribe_pos(&self, codes: &[StockCode])
        -> Result<futures::stream::BoxStream<'static, AuctionSnap>, DataError>
    {
        // 15s polling loop over quote-depth during 09:00–09:22. If the payload lacks
        // auction fields for HK, return Err(DataError::Unsupported) — the engine then
        // sets X2 = disabled and S2/X1 fall back to IEV-less variants (§5).
        let (tx, rx) = tokio::sync::mpsc::channel::<AuctionSnap>(1024);
        let _codes = codes.to_vec();
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(std::time::Duration::from_secs(15));
            loop {
                tick.tick().await;
                // for each batch of codes: self.call("quote_real_time", …)
                // extract IEP/IEV-equivalent auction fields → tx.send(AuctionSnap{..})
                let _ = &tx; todo!("poll + parse + send");
            }
        });
        Ok(tokio_stream::wrappers::ReceiverStream::new(rx).boxed())
    }
}
```

Yahoo (research/EOD prototyping only, feature-gated so it cannot be linked into the live binary), EastMoney and Xueqiu:

```rust
// crates/hkq-data/src/yahoo.rs   #[cfg(feature = "research")]
// Chart API: https://query1.finance.yahoo.com/v8/finance/chart/{code}.HK
// Report constraints honored in code: EOD prototyping ONLY — 1m history is a trailing
// window, no auction data, delisted names vanish (survivorship). The adapter refuses
// to implement AuctionFeed/IntradayFeed by design.

// crates/hkq-data/src/eastmoney.rs
// push2 public JSON endpoints: A-share 09:25 auction prints for the AH subset,
// index/futures quotes for the A50 leg, and daily HSGT (southbound) per-stock net buy.
// Endpoints are undocumented and can move — every URL lives in config, every parse
// is versioned, and nightly recon (below) catches silent schema drift.

// crates/hkq-data/src/xueqiu.rs
// Session-cookie authenticated JSON. Rate-limited (governor), ToS risk flagged (§5):
// acceptable for research; production sources are HKEX OMD-C + HKEX Connect pages.
```

The lake — hive-partitioned parquet, atomic writes, idempotent by construction, exchange-time timestamps, nightly reconciliation against official closes (§5):

```rust
// crates/hkq-data/src/lake.rs
use polars::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum Dataset { DailyBars, Bars1m, Auction, Flows, Ccass, MainlandPrints, Fx, Vhsi,
                   Fills, Attribution }

impl Dataset {
    fn dir(self) -> &'static str {
        match self {
            Dataset::DailyBars => "daily_bars", Dataset::Bars1m => "bars_1m",
            Dataset::Auction => "auction",      Dataset::Flows => "flows",
            Dataset::Ccass => "ccass",          Dataset::MainlandPrints => "mainland",
            Dataset::Fx => "fx",                Dataset::Vhsi => "vhsi",
            Dataset::Fills => "fills",          Dataset::Attribution => "attribution",
        }
    }
}

pub struct Lake { root: PathBuf }

impl Lake {
    pub fn new(root: impl AsRef<Path>) -> Self { Self { root: root.as_ref().into() } }

    /// Idempotent: write tmp, fsync, atomic rename over date partition. Re-running a
    /// nightly job is always safe (§5 "ingestion must be idempotent").
    pub fn write_partition(&self, ds: Dataset, date: chrono::NaiveDate, df: &mut DataFrame)
        -> PolarsResult<()>
    {
        let dir = self.root.join(ds.dir()).join(format!("date={date}"));
        std::fs::create_dir_all(&dir)?;
        let tmp = dir.join(".part.tmp");
        let fin = dir.join("part.parquet");
        let f = std::fs::File::create(&tmp)?;
        ParquetWriter::new(f).with_compression(ParquetCompression::Zstd(None)).finish(df)?;
        std::fs::rename(tmp, fin)?;
        Ok(())
    }

    pub fn scan(&self, ds: Dataset) -> PolarsResult<LazyFrame> {
        LazyFrame::scan_parquet(
            self.root.join(ds.dir()).join("**/*.parquet").to_string_lossy().as_ref(),
            ScanArgsParquet { hive_options: Default::default(), ..Default::default() },
        )
    }
}

/// Nightly reconciliation (§5): our stored close vs the official closing-auction print.
/// Any |Δ| > 1 tick ⇒ quarantine the partition and page the operator.
pub fn reconcile_closes(lake: &Lake, official: DataFrame, date: chrono::NaiveDate)
    -> PolarsResult<DataFrame>
{
    let ours = lake.scan(Dataset::DailyBars)?
        .filter(col("date").eq(lit(date)))
        .select([col("code"), col("close").alias("close_ours")]);
    ours.collect()?
        .lazy().join(official.lazy(), [col("code")], [col("code")], JoinArgs::new(JoinType::Inner))
        .with_column((col("close_ours") - col("close_official")).abs().alias("abs_diff"))
        .filter(col("abs_diff").gt(lit(0.0)))
        .collect()
}
```

# `hkq-factors` — The Factor Engine (polars)

First the shared math helpers, including the inverse normal CDF that the rank-z transform needs (Acklam's rational approximation, \(|\varepsilon| < 1.15\times10^{-9}\), ample for ranks):

```rust
// crates/hkq-factors/src/xsec.rs
use polars::prelude::*;

pub fn norm_ppf(p: f64) -> f64 {
    debug_assert!((0.0..1.0).contains(&p) && p > 0.0);
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
        (((((C[0]*q + C[1])*q + C[2])*q + C[3])*q + C[4])*q + C[5])
            / ((((D[0]*q + D[1])*q + D[2])*q + D[3])*q + 1.0)
    } else if p <= ph {
        let q = p - 0.5; let r = q * q;
        (((((A[0]*r + A[1])*r + A[2])*r + A[3])*r + A[4])*r + A[5]) * q
            / (((((B[0]*r + B[1])*r + B[2])*r + B[3])*r + B[4])*r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0]*q + C[1])*q + C[2])*q + C[3])*q + C[4])*q + C[5])
            / ((((D[0]*q + D[1])*q + D[2])*q + D[3])*q + 1.0)
    }
}

/// Cross-sectional rank-inverse-normal (§3.2/§3.4): z̃ = Φ⁻¹((rk − 0.5)/n) within `by`.
pub fn rank_inv_normal(lf: LazyFrame, cols: &[&str], by: &str) -> LazyFrame {
    let mut lf = lf;
    for c in cols {
        let rk = col(*c)
            .rank(RankOptions { method: RankMethod::Average, descending: false }, None)
            .cast(DataType::Float64);
        let n = col(*c).count().over([col(by)]).cast(DataType::Float64);
        let u = ((rk.over([col(by)]) - lit(0.5)) / n).alias("__u");
        lf = lf.with_column(u).with_column(
            col("__u").map(
                |s| Ok(Some(s.f64()?.apply(|o| o.map(norm_ppf)).into_series())),
                GetOutput::from_type(DataType::Float64),
            ).alias(&format!("{c}_z")),
        ).drop(["__u"]);
    }
    lf
}

/// Winsorize at cross-sectional p/(1−p) quantiles within `by` (§3.4, p = 0.01).
pub fn winsorize(lf: LazyFrame, cols: &[&str], by: &str, p: f64) -> LazyFrame {
    let mut lf = lf;
    for c in cols {
        let lo = col(*c).quantile(lit(p), QuantileMethod::Linear).over([col(by)]);
        let hi = col(*c).quantile(lit(1.0 - p), QuantileMethod::Linear).over([col(by)]);
        lf = lf.with_column(col(*c).clip(lo, hi).alias(*c));
    }
    lf
}

/// Sequential residualization in the report's FIXED order (§3.4): each factor keeps
/// only the component orthogonal to previously admitted factors, per date. The
/// cross-section is ~10²–10³ rows once per day — clarity beats vectorized cleverness,
/// so we solve small OLS systems with nalgebra per date-partition.
pub fn orthogonalize_daily(df: DataFrame, order: &[&str], date_col: &str)
    -> PolarsResult<DataFrame>
{
    use nalgebra::{DMatrix, DVector};
    let parts = df.partition_by([date_col], true)?;
    let mut out: Vec<DataFrame> = Vec::with_capacity(parts.len());
    for mut part in parts {
        for j in 1..order.len() {
            let y: Vec<f64> = part.column(order[j])?.f64()?.into_no_null_iter().collect();
            let n = y.len();
            let k = j + 1; // intercept + previously admitted factors
            let mut x = DMatrix::<f64>::zeros(n, k);
            for r in 0..n { x[(r, 0)] = 1.0; }
            for (cidx, prev) in order[..j].iter().enumerate() {
                let v = part.column(prev)?.f64()?;
                for r in 0..n { x[(r, cidx + 1)] = v.get(r).unwrap_or(0.0); }
            }
            let yv = DVector::from_vec(y.clone());
            let beta = x.clone().svd(true, true).solve(&(x.transpose() * &yv), 1e-12)
                .unwrap_or_else(|_| DVector::zeros(k));
            // NB: solve on normal equations XᵀXβ = Xᵀy via SVD of X — swap for QR if preferred.
            let resid: Vec<f64> = (0..n)
                .map(|r| y[r] - x.row(r).transpose().dot(&beta)).collect();
            part.replace(order[j], Series::new(order[j].into(), resid))?;
        }
        out.push(part);
    }
    let mut it = out.into_iter();
    let first = it.next().unwrap();
    it.try_fold(first, |acc, d| acc.vstack(&d))
}
```

The nightly moments panel — a direct mechanization of §3.0:

```rust
// crates/hkq-factors/src/moments.rs
use polars::prelude::*;
use hkq_core::config::FactorCfg;

const E: f64 = std::f64::consts::E;
fn lg(e: Expr) -> Expr { e.log(E) }

/// λ = 2^{−1/h}; polars ewm_mean(adjust=false, α = 1−λ) of r², shifted 1 day, is exactly
/// σ²_t = λσ²_{t−1} + (1−λ)r²_{t−1} — variance uses information through t−1 ONLY.
fn ewm_var_opts(cfg: &FactorCfg) -> EWMOptions {
    EWMOptions {
        alpha: 1.0 - 2f64.powf(-1.0 / cfg.ewma_halflife_days),
        adjust: false, ignore_nulls: true, min_periods: cfg.ewma_min_obs,
        ..Default::default()
    }
}

/// Input: daily panel [date, code, sector, open, close, volume, dollar_vol, float_cap,
/// spread_med, rv (joined), listed_days …] sorted by (code, date).
pub fn enrich_daily_panel(daily: LazyFrame, cfg: &FactorCfg) -> LazyFrame {
    let ewm = ewm_var_opts(cfg);
    let roll = |w: usize| RollingOptionsFixedWindow {
        window_size: w, min_periods: (w * 2) / 3, ..Default::default()
    };
    daily
        // ── §3.0 return decomposition ────────────────────────────────────────────
        .with_columns([
            (lg(col("open")) - lg(col("close")).shift(lit(1)).over([col("code")]))
                .alias("r_on"),
            (lg(col("close")) - lg(col("open"))).alias("r_id"),
        ])
        .with_column((col("r_on") + col("r_id")).alias("r_cc"))
        // ── EWMA vols, maintained separately for ON / ID / CC ──────────────────
        .with_columns([
            col("r_on").pow(2.0).ewm_mean(ewm.clone()).shift(lit(1)).sqrt()
                .over([col("code")]).alias("sigma_on"),
            col("r_id").pow(2.0).ewm_mean(ewm.clone()).shift(lit(1)).sqrt()
                .over([col("code")]).alias("sigma_id"),
            col("r_cc").pow(2.0).ewm_mean(ewm).shift(lit(1)).sqrt()
                .over([col("code")]).alias("sigma_cc"),
        ])
        // ── Amihud 60d, ADV, 5d mean RV ─────────────────────────────────────────
        .with_columns([
            (col("r_cc").abs() / col("dollar_vol"))
                .rolling_mean(roll(cfg.amihud_window)).over([col("code")]).alias("illiq"),
            col("volume").rolling_mean(roll(cfg.amihud_window)).over([col("code")])
                .alias("adv_shares"),
            col("rv").rolling_mean(roll(cfg.rv_days)).over([col("code")]).alias("rv_5d"),
        ])
        // ── LAV = √RV⁽⁵ᵈ⁾ · (ILLIQ / med_cs ILLIQ)^γ,  γ = 0.3 ─────────────────
        .with_column(col("illiq").median().over([col("date")]).alias("illiq_med_cs"))
        .with_column(
            (col("rv_5d").sqrt()
                * (col("illiq") / col("illiq_med_cs")).pow(lit(cfg.lav_gamma)))
            .alias("lav"))
}

/// Realized measures from 1-minute bars → 5-minute grid (§3.0): RV, RS±, RSJ, BV, jump.
pub fn realized_measures(bars_1m: LazyFrame) -> LazyFrame {
    let five = bars_1m
        .with_column(col("ts").dt().truncate(lit("5m")).alias("bin5"))
        .group_by([col("code"), col("date"), col("bin5")])
        .agg([col("c").last().alias("c5"), col("volume").sum().alias("v5")])
        .sort_by_exprs([col("code"), col("date"), col("bin5")], Default::default())
        .with_column(
            (lg(col("c5")) - lg(col("c5")).shift(lit(1)))
                .over([col("code"), col("date")]).alias("r5"));

    five.group_by([col("code"), col("date")])
        .agg([
            col("r5").pow(2.0).sum().alias("rv"),
            col("r5").pow(2.0).filter(col("r5").gt(lit(0.0))).sum().alias("rs_pos"),
            col("r5").pow(2.0).filter(col("r5").lt(lit(0.0))).sum().alias("rs_neg"),
            (col("r5").abs() * col("r5").abs().shift(lit(1))).sum().alias("bv_raw"),
            col("r5").count().alias("n5"),
        ])
        .with_column(
            (lit(std::f64::consts::FRAC_PI_2)
                * (col("n5").cast(DataType::Float64)
                    / (col("n5").cast(DataType::Float64) - lit(1.0)))
                * col("bv_raw")).alias("bv"))
        .with_columns([
            (col("rv") - col("bv")).clip_min(lit(0.0)).alias("jump"),          // 𝒥 = max(RV−BV,0)
            ((col("rs_pos") - col("rs_neg")) / col("rv")).alias("rsj"),        // RSJ
        ])
}

/// IVU (§3.3 X4): realized dispersion of 5-min volume shares vs the 20d seasonal
/// profile, terciled per stock over a trailing 60d window.
pub fn ivu(bars_1m: LazyFrame, cfg: &FactorCfg) -> LazyFrame {
    let shares = bars_1m
        .with_column(col("ts").dt().truncate(lit("5m")).alias("bin5"))
        .group_by([col("code"), col("date"), col("bin5")])
        .agg([col("volume").sum().alias("v_bin")])
        .with_column(
            (col("v_bin") / col("v_bin").sum().over([col("code"), col("date")]))
                .alias("s_bin"));
    let seasonal = shares.clone()
        .sort_by_exprs([col("code"), col("bin5"), col("date")], Default::default())
        .with_column(
            col("s_bin").rolling_mean(RollingOptionsFixedWindow {
                window_size: cfg.seasonal_vol_days, min_periods: 10, ..Default::default()
            }).shift(lit(1)).over([col("code"), col("bin5")]).alias("s_bar"));
    seasonal
        .with_column((col("s_bin") - col("s_bar")).pow(2.0).alias("dev2"))
        .group_by([col("code"), col("date")])
        .agg([col("dev2").mean().sqrt().alias("ivu")])
    // tercile assignment vs trailing 60d per-stock distribution happens in stage2.rs
}
```

The lead–lag network (§3.2 S4, reused within-sector for X6). Ledoit–Wolf–style shrink-to-zero on a *lagged* correlation matrix (Schäfer–Strimmer form, since LW proper targets symmetric covariance), Fisher-z p-values, Benjamini–Hochberg at \(q=0.1\):

```rust
// crates/hkq-factors/src/leadlag.rs
use nalgebra::DMatrix;

pub struct LeadLagGraph {
    /// a[(lead, lag)] = shrunk, FDR-surviving signed correlation; 0 otherwise.
    pub a: DMatrix<f64>,
}

/// resid: T×K matrix of residual (index-hedged) returns, columns aligned to `labels`.
pub fn lagged_corr_fdr(resid: &DMatrix<f64>, q: f64) -> LeadLagGraph {
    let (t, k) = resid.shape();
    let n = t - 1;
    let std = |v: &[f64]| {
        let m = v.iter().sum::<f64>() / v.len() as f64;
        let s = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() - 1) as f64).sqrt();
        v.iter().map(|x| (x - m) / s.max(1e-12)).collect::<Vec<_>>()
    };
    // Λ_ab = Corr(x_a[..T−1], x_b[1..])  — column a leads column b.
    let cols: Vec<Vec<f64>> = (0..k).map(|j| resid.column(j).iter().copied().collect()).collect();
    let lead: Vec<Vec<f64>> = cols.iter().map(|c| std(&c[..n])).collect();
    let lagg: Vec<Vec<f64>> = cols.iter().map(|c| std(&c[1..])).collect();
    let mut lambda = DMatrix::<f64>::zeros(k, k);
    for a in 0..k { for b in 0..k { if a != b {
        lambda[(a, b)] = lead[a].iter().zip(&lagg[b]).map(|(x, y)| x * y).sum::<f64>()
            / (n as f64 - 1.0);
    }}}
    // Schäfer–Strimmer shrink toward zero: λ̂* = Σ V̂ar(r_ab) / Σ r_ab²  (clamped to [0,1]).
    let var_r = 1.0 / n as f64; // asymptotic var of a correlation under H0, plug-in
    let sum_r2: f64 = lambda.iter().map(|r| r * r).sum();
    let shrink = ((k * (k - 1)) as f64 * var_r / sum_r2.max(1e-12)).clamp(0.0, 1.0);
    lambda *= 1.0 - shrink;
    // Fisher z p-values + BH at level q.
    let mut ps: Vec<(usize, usize, f64)> = Vec::with_capacity(k * (k - 1));
    for a in 0..k { for b in 0..k { if a != b {
        let r = lambda[(a, b)].clamp(-0.999999, 0.999999);
        let z = 0.5 * ((1.0 + r) / (1.0 - r)).ln() * ((n as f64 - 3.0).sqrt());
        let p = 2.0 * (1.0 - phi(z.abs()));
        ps.push((a, b, p));
    }}}
    ps.sort_by(|x, y| x.2.total_cmp(&y.2));
    let m = ps.len() as f64;
    let mut cutoff = 0usize;
    for (i, (_, _, p)) in ps.iter().enumerate() {
        if *p <= q * (i as f64 + 1.0) / m { cutoff = i + 1; }
    }
    let keep: std::collections::HashSet<(usize, usize)> =
        ps[..cutoff].iter().map(|(a, b, _)| (*a, *b)).collect();
    for a in 0..k { for b in 0..k {
        if a != b && !keep.contains(&(a, b)) { lambda[(a, b)] = 0.0; }
    }}
    LeadLagGraph { a: lambda }
}

fn phi(x: f64) -> f64 { 0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2)) }
fn erf(x: f64) -> f64 { // Abramowitz–Stegun 7.1.26, |ε| < 1.5e−7 — fine for FDR gating
    let s = x.signum(); let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let y = 1.0 - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t
        - 0.284496736) * t + 0.254829592) * t * (-x * x).exp();
    s * y
}
```

Stage 1 — the six sector factors, assembled at 09:29:30 from the frozen `OpenContext`. Everything that can be precomputed nightly (S1, S3, S4, S5, sector σ's, IEV baselines) is; only S2 and S6 need morning inputs:

```rust
// crates/hkq-factors/src/stage1.rs
use polars::prelude::*;
use hkq_core::config::Stage1Cfg;
use crate::xsec;

/// Frozen 09:29:30 snapshot (§6 runbook). Building this struct is the ONLY place
/// morning I/O meets factor math — factors themselves are pure functions of it.
pub struct OpenContext {
    pub date: chrono::NaiveDate,
    /// sector panel t−1 (nightly): [sector, r_id_1, r_on_1, sigma_id, sigma_on,
    ///   s3_late_flow, s4_leadlag, s5_flow_z]  ← S1/S3/S4/S5 precomputed
    pub sector_nightly: DataFrame,
    /// member auction aggregation (morning): [sector, gap_z, vs_auct]
    pub sector_auction: DataFrame,
    /// linked markets (morning): [sector, ah_delta, a50_beta_ret, adr_resid_agg]
    pub sector_linked: DataFrame,
}

/// S1 (§3.2): θ₁·r_ID/σ_ID − θ₂·r_ON/σ_ON of t−1 — continuation on traded returns,
/// fade on overnight-propagated moves (report consequence №3).
pub fn s1(cfg: &Stage1Cfg) -> Expr {
    (lit(cfg.theta1) * col("r_id_1") / col("sigma_id"))
        - (lit(cfg.theta2) * col("r_on_1") / col("sigma_on"))
}

/// S2 (§3.2): z_gap·sgn(VS)·1{|VS|>v*} − η·z_gap. Gap on confirmed auction volume is
/// information; gap on thin volume leans reversal via the unconditional −η tilt.
pub fn s2(cfg: &Stage1Cfg) -> Expr {
    let confirmed = col("gap_z")
        * col("vs_auct").sign()
        * col("vs_auct").abs().gt(lit(cfg.vs_threshold)).cast(DataType::Float64);
    confirmed - lit(cfg.eta) * col("gap_z")
}

/// Composite: rank-inv-normal each factor across sectors, ICIR-shrunk weights,
/// top-K with the absolute gate Σ_min — below the gate the day is CASH (§3.2).
pub fn sector_composite(
    ctx: &OpenContext, weights: &crate::icir::FactorWeights, cfg: &Stage1Cfg,
) -> PolarsResult<DataFrame> {
    let lf = ctx.sector_nightly.clone().lazy()
        .join(ctx.sector_auction.clone().lazy(), [col("sector")], [col("sector")],
              JoinArgs::new(JoinType::Left))
        .join(ctx.sector_linked.clone().lazy(), [col("sector")], [col("sector")],
              JoinArgs::new(JoinType::Left))
        .with_columns([
            s1(cfg).alias("s1"),
            s2(cfg).alias("s2"),
            col("s3_late_flow").alias("s3"),
            col("s4_leadlag").alias("s4"),
            col("s5_flow_z").alias("s5"),
            // S6: equal-weighted sum of standardized linked-market subcomponents
            ((col("ah_delta") + col("a50_beta_ret") + col("adr_resid_agg")) / lit(3.0))
                .alias("s6"),
        ])
        .with_column(lit(1i32).alias("__all")); // single cross-section ⇒ rank over const
    let lf = xsec::rank_inv_normal(lf, &["s1","s2","s3","s4","s5","s6"], "__all");
    let w = weights; // ω_f ∝ max(ICIR_f, 0) + δ, renormalized (computed nightly)
    lf.with_column(
        (lit(w.get("s1")) * col("s1_z") + lit(w.get("s2")) * col("s2_z")
       + lit(w.get("s3")) * col("s3_z") + lit(w.get("s4")) * col("s4_z")
       + lit(w.get("s5")) * col("s5_z") + lit(w.get("s6")) * col("s6_z"))
        .alias("sigma_score"))
      .sort_by_exprs([col("sigma_score")], SortMultipleOptions::default().with_order_descending(true))
      .collect()
}

/// Selection with the cash-day gate.
pub fn select_sectors(scored: &DataFrame, cfg: &Stage1Cfg)
    -> PolarsResult<Vec<hkq_core::ids::SectorId>>
{
    let s = scored.column("sigma_score")?.f64()?;
    let ids = scored.column("sector")?.u16()?;
    let mut out = Vec::new();
    for i in 0..scored.height().min(cfg.top_k_sectors) {
        match (s.get(i), ids.get(i)) {
            // Σ_(K) > Σ_min or the position is 100% cash — §3.2, the conditional-alpha logic.
            (Some(v), Some(id)) if v > cfg.sigma_min_gate =>
                out.push(hkq_core::ids::SectorId(id)),
            _ => break,
        }
    }
    Ok(out)
}
```

Two nightly Stage-1 precomputations worth showing — S3's late-session flow and the value-weight cap used everywhere sectors aggregate members:

```rust
// crates/hkq-factors/src/sector_agg.rs

/// Float-cap weights, capped at 15% per name and renormalized (§3.2). The cap is a
/// water-filling fixpoint; 8 iterations is overkill for convergence at K≲50 members.
pub fn capped_weights(float_cap: &[f64], cap: f64) -> Vec<f64> {
    let mut w: Vec<f64> = {
        let s: f64 = float_cap.iter().sum();
        float_cap.iter().map(|x| x / s).collect()
    };
    for _ in 0..8 {
        let over: f64 = w.iter().filter(|x| **x > cap).map(|x| x - cap).sum();
        if over < 1e-12 { break; }
        let under_sum: f64 = w.iter().filter(|x| **x < cap).sum();
        w = w.iter()
            .map(|x| if *x > cap { cap } else { x + over * (x / under_sum) })
            .collect();
    }
    w
}

/// S3 (§3.2): standardized 14:00–15:30 sector return of t−1, scaled by the positive
/// part of (1 + ln V/V̄). Window return from 1m bars; σ from trailing 60d of the same
/// window series; the whole multiplier is floored at zero — thin late sessions mute it.
pub fn s3_late_flow(sector_win: LazyFrame) -> LazyFrame {
    sector_win.with_column(
        ((col("r_late") / col("r_late_sigma_60d"))
            * (lit(1.0) + (col("v_late") / col("v_late_bar")).log(std::f64::consts::E))
                .clip_min(lit(0.0)))
        .alias("s3_late_flow"))
}
```

Stage 2 — stock factors within selected sectors, the regime gate, and the transform pipeline in the report's exact order:

```rust
// crates/hkq-factors/src/stage2.rs
use polars::prelude::*;
use hkq_core::config::Stage2Cfg;
use crate::xsec;

/// IVU tercile gate (§3.3 X4): multipliers on X1–X3 per regime. Loaded from the
/// quarterly threshold-regression job in hkq-validate; identity until first fit.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct RegimeGate { pub g: [[f64; 3]; 3] } // [ivu_tercile][x1|x2|x3]

impl Default for RegimeGate { fn default() -> Self { Self { g: [[1.0; 3]; 3] } } }

/// Stock frame at freeze: [code, sector, gap_z, beta_sector, vs_auct_i, iep_0910,
/// iep_0920, sigma_on, vol_tau0, vol_tau0_med20, rsj_1, jump_1, rv_5d, lav,
/// x6_spillover, sb_z, connect_elig, ivu_tercile]
pub fn stage2_scores(
    stocks: DataFrame, gate: &RegimeGate, w: &crate::icir::FactorWeights, cfg: &Stage2Cfg,
) -> PolarsResult<DataFrame> {
    let e = std::f64::consts::E;
    let lf = stocks.lazy()
        // X1 (§3.3): ε = z_gap,i − β̂·z_gap,sector ; X1 = −ε + φ·ε·1{VS_i > v*}
        .with_column((col("gap_z") - col("beta_sector") * col("gap_z_sector")).alias("eps_gap"))
        .with_column(
            (lit(-1.0) * col("eps_gap")
             + lit(cfg.phi) * col("eps_gap")
               * col("vs_auct_i").gt(lit(cfg.vs_threshold_stock)).cast(DataType::Float64))
            .alias("x1"))
        // X2: ln(IEP_09:20 / IEP_09:10)/σ_ON — late-auction drift after the 09:15
        // no-cancel regime. Column is null in X2-disabled mode; weight auto-zeroes.
        .with_column(
            ((col("iep_0920") / col("iep_0910")).log(e) / col("sigma_on")).alias("x2"))
        // X3: opening volume surprise vs 20d same-bin median (two-phase: pre/full)
        .with_column(((col("vol_tau0") / col("vol_tau0_med20")).log(e)).alias("x3"))
        // X5: −RSJ_{t−1}·1{𝒥>0} + ζ·√RV⁽⁵ᵈ⁾/LAV
        .with_column(
            (lit(-1.0) * col("rsj_1")
                * col("jump_1").gt(lit(0.0)).cast(DataType::Float64)
             + lit(cfg.zeta) * col("rv_5d").sqrt() / col("lav"))
            .alias("x5"))
        // X6 precomputed nightly (within-sector leader graph); X7 = sb z or 0.
        .with_column(col("x6_spillover").alias("x6"))
        .with_column(
            (col("sb_z") * col("connect_elig").cast(DataType::Float64)).alias("x7"));

    // X4: gate multiplies X1–X3 per IVU tercile — regime gates, not an additive factor.
    let g = gate.clone();
    let lf = lf.with_columns([
        gated(&g, 0).alias("x1"), gated(&g, 1).alias("x2"), gated(&g, 2).alias("x3"),
    ]);

    // §3.4 pipeline: winsorize 1/99 → rank-inv-normal (within selected-sector union,
    // one cross-section) → sequential orthogonalization in FIXED order → ICIR combine.
    let lf = lf.with_column(lit(1i32).alias("__cs"));
    let lf = xsec::winsorize(lf, &["x1","x2","x3","x5","x6","x7"], "__cs", cfg.winsor_pct);
    let lf = xsec::rank_inv_normal(lf, &["x1","x2","x3","x5","x6","x7"], "__cs");
    let df = lf.collect()?;
    let order: Vec<String> = cfg.ortho_order.iter().map(|s| format!("{s}_z")).collect();
    let order_ref: Vec<&str> = order.iter().map(String::as_str).collect();
    let df = xsec::orthogonalize_daily(df, &order_ref, "__cs")?;

    df.lazy()
        .with_column(
            (lit(w.get("x5")) * col("x5_z") + lit(w.get("x3")) * col("x3_z")
           + lit(w.get("x2")) * col("x2_z").fill_null(lit(0.0)) // X2-disabled mode
           + lit(w.get("x1")) * col("x1_z") + lit(w.get("x6")) * col("x6_z")
           + lit(w.get("x7")) * col("x7_z"))
            .alias("score"))
        .sort_by_exprs([col("sector"), col("score")],
            SortMultipleOptions::default().with_order_descending(true))
        .collect()
}

fn gated(gate: &RegimeGate, which: usize) -> Expr {
    // multiplier chosen by the row's ivu_tercile ∈ {0,1,2}
    when(col("ivu_tercile").eq(lit(0))).then(col(X[which]) * lit(gate.g[0][which]))
        .when(col("ivu_tercile").eq(lit(1))).then(col(X[which]) * lit(gate.g[1][which]))
        .otherwise(col(X[which]) * lit(gate.g[2][which]))
}
const X: [&str; 3] = ["x1", "x2", "x3"];
```

One reconciliation note, since it's the only place the report under-specifies against its own runbook: §3.3 defines X3's window \(\tau_0\) as auction *plus first five minutes*, but the runbook freezes scores at 09:29:30. The implementation therefore runs X3 two-phase: at freeze it uses the auction-only surprise (candidate list selection), and at 09:35 the engine refreshes X3 with the first-5-minute volume before the 09:45 entry filter. Both variants share the same expression with different `vol_tau0` inputs, so nothing is duplicated.

# `hkq-signal` — ICIR Weights, Confirmation, Meta-Label

```rust
// crates/hkq-signal/src/icir.rs
use polars::prelude::*;
use std::collections::BTreeMap;

pub struct FactorWeights(BTreeMap<String, f64>);

impl FactorWeights {
    pub fn get(&self, f: &str) -> f64 { *self.0.get(f).unwrap_or(&0.0) }

    /// ω_f ∝ max(ICIR_f, 0) + δ (δ = 0.1), renormalized (§3.2/§3.4). `ic_panel` is the
    /// nightly-appended daily rank-IC per factor (Spearman of factor vs realized
    /// forward open→close return across the cross-section).
    pub fn from_ic_panel(ic_panel: &DataFrame, factors: &[&str], window: usize, delta: f64)
        -> PolarsResult<Self>
    {
        let mut raw = BTreeMap::new();
        for f in factors {
            let s = ic_panel.column(&format!("ic_{f}"))?.f64()?;
            let n = s.len();
            let tail: Vec<f64> = (n.saturating_sub(window)..n)
                .filter_map(|i| s.get(i)).collect();
            let (m, sd) = mean_sd(&tail);
            raw.insert(f.to_string(), (m / sd.max(1e-9)).max(0.0) + delta);
        }
        let z: f64 = raw.values().sum();
        Ok(Self(raw.into_iter().map(|(k, v)| (k, v / z)).collect()))
    }
}

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let m = v.iter().sum::<f64>() / v.len().max(1) as f64;
    let sd = (v.iter().map(|x| (x - m).powi(2)).sum::<f64>()
        / (v.len().saturating_sub(1)).max(1) as f64).sqrt();
    (m, sd)
}
```

```rust
// crates/hkq-signal/src/decision.rs
use hkq_core::{config::TradeCfg, ids::StockCode};

/// Meta-label map (§3.5): nightly OLS of realized r[09:45→15:45] on the composite
/// score within regime buckets (VHSI tercile × IVU tercile). α̂ = a + b·score.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AlphaMap { pub coef: std::collections::BTreeMap<String, (f64, f64)> }

impl AlphaMap {
    pub fn expected_alpha_bps(&self, vhsi_t: u8, ivu_t: u8, score: f64) -> f64 {
        let (a, b) = self.coef.get(&format!("{vhsi_t}-{ivu_t}")).copied().unwrap_or((0.0, 0.0));
        (a + b * score) * 1e4
    }
}

pub struct Candidate {
    pub code: StockCode,
    pub sector: hkq_core::ids::SectorId,
    pub score: f64,
    pub sigma_15m: f64,          // trailing EWMA std of 15-minute returns
    pub lav: f64,
    pub cost_floor_bps: f64,     // c_i from hkq-risk, at tentative size q_i
    pub r_0930_0945: f64,        // filled during the confirmation window
    pub vhsi_tercile: u8,
    pub ivu_tercile: u8,
}

/// §3.5 — the three-fold entry filter at 09:45. The ranking model proposes,
/// the calibrated conditional-expectation model disposes.
pub fn entry_filter(cands: &[Candidate], alpha: &AlphaMap, m_per_sector: usize,
                    cfg: &TradeCfg) -> Vec<&Candidate> {
    let mut by_sector: std::collections::BTreeMap<u16, Vec<&Candidate>> = Default::default();
    for c in cands { by_sector.entry(c.sector.0).or_default().push(c); }
    let mut out = Vec::new();
    for (_k, mut v) in by_sector {
        v.sort_by(|a, b| b.score.total_cmp(&a.score));
        out.extend(v.into_iter().take(m_per_sector).filter(|c| {
            // (i) confirmation: χ = sgn(score)·r[09:30,09:45)/σ15m > 0  (long-only ⇒ score>0 too)
            let chi = c.score.signum() * c.r_0930_0945 / c.sigma_15m.max(1e-9);
            // (ii) meta-label: α̂ > c_i + m*  (m* = 10 bps)
            let alpha_bps = alpha.expected_alpha_bps(c.vhsi_tercile, c.ivu_tercile, c.score);
            c.score > 0.0 && chi > 0.0 && alpha_bps > c.cost_floor_bps + cfg.margin_bps
        }));
    }
    out
}
```

# `hkq-risk` — Cost Floor, Decimal Sizing, Stops

```rust
// crates/hkq-risk/src/cost.rs
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use hkq_core::money::Cash;

pub struct CostModel {
    pub stamp_per_side: Decimal,   // dec!(0.001)  — 0.1% per contract note (§1)
    pub fees_rt: Decimal,          // dec!(0.00022) — exchange + levies, round trip
    pub kappa: f64,
}

impl CostModel {
    /// Signal-side twin (f64): c_i = 20bps + fees + s_i + κ·σ_i·√(q_i/ADV_i)  (§1).
    /// Used ONLY for the α̂ > c_i + m* comparison; never for accounting.
    pub fn floor_bps(&self, spread_bps: f64, sigma_cc: f64, q_shares: f64, adv: f64) -> f64 {
        20.0 + 2.2 + spread_bps
            + self.kappa * sigma_cc * (q_shares / adv.max(1.0)).sqrt() * 1e4
    }

    /// Accounting side (Decimal): exact statutory charge for attribution (§4 cost realism).
    /// HK stamp duty rounds UP to the dollar per contract note.
    pub fn stamp_duty(&self, notional: Cash) -> Cash {
        Cash((notional.0 * self.stamp_per_side).ceil())
    }
}
```

```rust
// crates/hkq-risk/src/sizing.rs
use rust_decimal::{Decimal, prelude::*};
use hkq_core::money::{Cash, Px, BoardLot, LotQty};
use hkq_core::ids::StockCode;

pub struct SizingInput {
    pub code: StockCode,
    pub ref_px: Px,                  // 09:45 marketable reference
    pub lot: BoardLot,
    pub lav: f64,                    // §3.6 inverse-LAV weights
    pub sigma_15m: f64,
    pub projected_interval_vol: f64, // seasonal profile × today's surprise multiplier
}

pub struct SizedOrder {
    pub code: StockCode,
    pub qty: LotQty,
    pub limit: Px,
    pub stop: Px,                    // −2.5·σ15m from entry, snapped down to tick (§3.6)
    pub target_cash: Cash,
}

/// §3.6: w_i ∝ 1/LAV_i, Σw = 1 (cash account, no leverage), participation ≤ 2% of
/// projected interval volume, board-lot floors. The no-leverage invariant is enforced
/// in Decimal and returns the residual cash explicitly — cash is a position.
pub fn size_book(equity: Cash, inputs: &[SizingInput], participation_cap: f64,
                 stop_mult: f64) -> (Vec<SizedOrder>, Cash) {
    // 1) weights in f64 (statistics), normalized
    let inv: Vec<f64> = inputs.iter().map(|i| 1.0 / i.lav.max(1e-9)).collect();
    let z: f64 = inv.iter().sum();
    let mut used = Decimal::ZERO;
    let mut orders = Vec::with_capacity(inputs.len());

    for (i, inp) in inputs.iter().enumerate() {
        // 2) f64 → Decimal exactly once; round DOWN to the cent (never size up).
        let w = Decimal::from_f64(inv[i] / z).unwrap_or(Decimal::ZERO);
        let target = Cash((equity.0 * w).round_dp_with_strategy(
            2, rust_decimal::RoundingStrategy::ToZero));

        // 3) board-lot floor, then participation cap q_i ≤ 2%·projected volume
        let Some(qty) = LotQty::floor_from_cash(target, inp.ref_px, inp.lot) else { continue };
        let cap_shares = (inp.projected_interval_vol * participation_cap).floor() as u64;
        let Some(qty) = qty.cap_shares(cap_shares) else { continue };

        let notional = qty.notional(inp.ref_px);
        // 4) hard invariant: Σ notional ≤ equity — checked in Decimal, no float ever.
        if used + notional.0 > equity.0 { continue; }
        used += notional.0;

        // stop: entry·(1 − 2.5σ15m), snapped down to a valid tick
        let stop_raw = inp.ref_px.get()
            * (Decimal::ONE - Decimal::from_f64(stop_mult * inp.sigma_15m).unwrap());
        let stop = Px::new(stop_raw).map(Px::snap_down_to_tick)
            .unwrap_or(inp.ref_px); // degenerate σ ⇒ effectively immediate-exit guard

        orders.push(SizedOrder { code: inp.code, qty, limit: inp.ref_px, stop,
                                 target_cash: notional });
    }
    (orders, Cash(equity.0 - used))
}
```

The kill switch is a `tokio::sync::watch<RiskState>` written by exactly two producers — the CUSUM monitor (§4) and the operator console — and read by every actor; any transition to `Halted` causes the engine to cancel resting orders and, if past 09:45, run the exit program immediately.

# `hkq-engine` — The Runbook as a State Machine

The daily runbook (§6) is encoded as an explicit phase enum with compile-time-checked transitions; phases advance on wall-clock timers (chrono-tz boundaries → `sleep_until`) or terminal market events, never on data availability alone. The strategy actor is the *single writer* of book state — no `Mutex` on the hot path, all cross-actor communication over channels.

```rust
// crates/hkq-engine/src/phases.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    PreMarket,        // 08:45 ADR/FX ingest → S6 ADR block
    PosStream,        // 09:00–09:22 IEP/IEV trajectories
    MainlandPrints,   // 09:25 A-share auction, A50
    ScoreFreeze,      // 09:29:30 S1–S6 → sectors → stock scores → candidate freeze
    Confirmation,     // 09:30–09:45 χ accumulation; X3 refresh at 09:35
    Entry,            // 09:45 meta-label filter → size_book → sliced entries
    MorningHold,      // stops live; 11:58 cancel resting
    Lunch,            // 12:00–13:00 no-decision hold (§3.6)
    AfternoonHold,    // stops live
    ExitProgram,      // 15:30–15:45 VWAP slices into strength (§3.6)
    CasResidual,      // 16:00–16:08 residual illiquid tail only
    PostClose,        // attribution, IC append, CUSUM update, trials registry
}
```

```rust
// crates/hkq-engine/src/live.rs (core loop, abridged to the control flow that matters)
pub async fn run_day(mut self) -> anyhow::Result<()> {
    use Phase::*;
    // Half-day handling (§1/§5): grid collapses to τ ≤ 5.
    match self.calendar.day_kind(self.date) {
        DayKind::Closed => return Ok(()),
        DayKind::HalfDay if self.cfg.trade.half_day_mode == HalfDayMode::Skip => {
            tracing::info!("half-day: skipping (config)"); return Ok(());
        }
        _ => {}
    }
    let mut phase = PreMarket;
    let mut timer = Box::pin(tokio::time::sleep_until(self.next_boundary(phase)));

    loop {
        tokio::select! {
            biased;
            // Kill switch pre-empts everything.
            _ = self.kill_rx.changed() => {
                if self.kill_rx.borrow().halted() { self.flatten_now().await?; break; }
            }
            _ = &mut timer => {
                phase = self.advance(phase).await?;   // executes the phase's entry action
                if phase == PostClose { self.post_close().await?; break; }
                timer.as_mut().reset(self.next_boundary(phase));
            }
            Some(ev) = self.md_rx.recv() => match (phase, ev) {
                (PosStream, MarketEvent::Auction(s))      => self.ctx.absorb_auction(s),
                (MainlandPrints, MarketEvent::MainlandAuctionPrint{code, a_open_ret})
                                                          => self.ctx.absorb_ah(code, a_open_ret),
                (Confirmation | MorningHold | AfternoonHold, MarketEvent::Bar(b)) => {
                    self.book.update_marks(&b);
                    self.check_stops(&b).await?;          // −2.5σ15m hard stop (§3.6)
                }
                (_, MarketEvent::Vcm { code, in_cooling_off }) =>
                    self.exec_tx.send(ExecCmd::PauseSymbol(code, in_cooling_off)).await?,
                _ => {}
            },
            Some(fill) = self.fill_rx.recv() => self.book.apply_fill(fill),
        }
    }
    // Terminal invariant (§ scope): zero overnight inventory. Loud, not silent.
    anyhow::ensure!(self.book.is_flat(), "book not flat after CAS — manual intervention");
    Ok(())
}
```

Phase entry actions map one-to-one onto the report's runbook: `ScoreFreeze` builds `OpenContext` (any missing feed degrades the corresponding factor to its documented fallback rather than blocking), calls `sector_composite → select_sectors → stage2_scores`, and freezes candidates; `Entry` runs `entry_filter → size_book` and hands `SizedOrder`s to the exec actor, which slices children paced against realized interval volume so cumulative participation stays under the 2% cap, pausing on VCM cooling-off events; `ExitProgram` liquidates 15:30–15:45 in VWAP slices with only the illiquid residue routed to the closing auction; `MorningHold`'s boundary action at 11:58 cancels all resting orders into lunch.

The backtester reuses this exact engine with two swaps behind traits: `SimClock` (event-time from the 1-minute parquet partitions) and `SimExec`, which fills at bar VWAP adjusted by half-spread plus the κ-impact term, charges the full §1 cost stack per fill including up-rounded stamp duty, honors the ±15% POS band, and simulates VCM cooling-off as unfillable windows — the report's §4 cost-realism requirements are properties of the fill model, not post-hoc haircuts.

# `hkq-validate` — The §4 Protocol

```rust
// crates/hkq-validate/src/lib.rs (essentials)

/// Purged walk-forward splits with embargo (§4): label horizon 1d, embargo 5d.
pub fn purged_walk_forward(n_days: usize, n_folds: usize, embargo: usize, horizon: usize)
    -> Vec<(std::ops::Range<usize>, std::ops::Range<usize>)>
{
    let fold = n_days / n_folds;
    (1..n_folds).map(|k| {
        let test = k * fold..((k + 1) * fold).min(n_days);
        let train_end = test.start.saturating_sub(embargo + horizon); // purge + embargo
        (0..train_end, test)
    }).collect()
}

/// Newey–West t-stat of the mean daily rank IC (lag 5, §4).
pub fn newey_west_t(x: &[f64], lags: usize) -> f64 {
    let n = x.len() as f64;
    let m = x.iter().sum::<f64>() / n;
    let e: Vec<f64> = x.iter().map(|v| v - m).collect();
    let mut s = e.iter().map(|v| v * v).sum::<f64>() / n;
    for l in 1..=lags {
        let w = 1.0 - l as f64 / (lags as f64 + 1.0);
        let g = e.iter().zip(&e[l..]).map(|(a, b)| a * b).sum::<f64>() / n;
        s += 2.0 * w * g;
    }
    m / (s / n).sqrt()
}

/// Deflated Sharpe Ratio (§4): PSR against SR* = E[max SR] under N effective trials.
/// `n_trials` MUST come from the trials registry — that is the whole point.
pub fn deflated_sharpe(sr: f64, t: usize, skew: f64, kurt: f64,
                       sr_var_across_trials: f64, n_trials: usize) -> f64 {
    const EULER: f64 = 0.5772156649015329;
    let n = n_trials.max(2) as f64;
    let sr_star = sr_var_across_trials.sqrt()
        * ((1.0 - EULER) * qnorm(1.0 - 1.0 / n) + EULER * qnorm(1.0 - 1.0 / (n * std::f64::consts::E)));
    let num = (sr - sr_star) * ((t as f64 - 1.0).sqrt());
    let den = (1.0 - skew * sr + (kurt - 1.0) / 4.0 * sr * sr).sqrt();
    phi(num / den)  // require > 0.95 before promotion (§4)
}

/// CUSUM on the 60d rolling IC with pre-registered kill threshold (§4). Crossing
/// flips the watch<RiskState> to Halted — decay is an operational event, not a memo.
pub struct CusumIc { pub mu0: f64, pub k: f64, pub h: f64, s: f64 }
impl CusumIc {
    pub fn update(&mut self, ic: f64) -> bool {
        self.s = (self.s + (self.mu0 - ic) - self.k).max(0.0);
        self.s > self.h
    }
}

/// Append-only trials registry: every evaluated variant (config hash + IC + SR) is
/// recorded so N in deflated_sharpe is honest. JSONL, hash-chained.
pub struct TrialsRegistry { path: std::path::PathBuf }
```

The nightly attribution job appends per-factor realized rank ICs (which feed `FactorWeights`), refreshes the `AlphaMap` regressions per regime bucket, re-fits \((\theta_1,\theta_2)\) and the threshold-regression regime gates quarterly, and reports ICs stratified by VHSI tercile, volume-surprise tercile, and IVU regime — with the report's misspecification test wired in as an alert: alpha concentrating in *low*-vol cells is a bug signal, not a pleasant surprise.

# Honest Gaps Before Go-Live

Three items are stubbed where vendor verification is unavoidable: the Tiger request-signing canonicalization and the exact auction-field names in its HK quote payload must be validated against your account's API version (the engine already runs in X2-disabled fallback if IEP/IEV is absent, per §5); EastMoney/Xueqiu endpoints are unofficial, so they live behind config URLs with schema-versioned parsers and the nightly close reconciliation as the tripwire — production should substitute HKEX OMD-C and the official Connect statistics as the report specifies; and \(v^*\), \(\Sigma_{min}\), \(\zeta\), \(\kappa\), and the regime-gate matrix are estimation outputs, not constants, so `strategy.toml` ships with the report's priors and the `hkq-validate` jobs own their refresh. Polars expression names may need mechanical renames if you deviate from the 0.46 pin. Finally, the report's own discipline applies to the software: the backtester and live engine share one factor crate and one cost model by construction, the trials registry makes the DSR's \(N\) honest, and the CUSUM kill switch means the first production deployment should still be a paper-trading shadow period whose fills are reconciled daily before capital touches it.
