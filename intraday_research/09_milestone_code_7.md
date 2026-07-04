 
All eight documents are attached in full, so I worked directly from them — no fetching needed (I have search only, and it isn't required here). M6's hand-off is unambiguous about what comes next: "the backtest tier is the only tier that doesn't exist: `SimClock` replacing `instant_for`'s wall-clock arithmetic and `SimExec` behind this exact trait — and the M6 trait shape makes SimExec's job precise: it is a `Submission::Immediate` venue whose fill price applies the §4 cost-realism model (half-spread + κ-impact)… the seam needs no further surgery." Its closing line names the milestone: "What it lacks is a mirror: the replay tier that proves the hand would have done the same thing yesterday, and that is exactly two trait implementations away."

# Milestone 7 — The Mirror (`SimClock` + `SimExec`: the backtest replay tier)

**Why this is next.** M1–M6 built a machine with facts, opinions, decisions, a clock, a memory, a reflex, and a hand — but no way to re-run yesterday. That absence is now the binding constraint on three §4 obligations at once. First, the promotion gates are starving: the trials registry has no `sr`-carrying trials, so `sr_dispersion()` returns `None` and the DSR — the pre-registered promotion gate — is permanently incomputable; only a replay tier can generate honest trials. Second, cost realism is unmeasurable: κ ships as a config prior "calibrated from fills," and calibrating it requires replaying the same day under different impact assumptions against the shadow record. Third, the M6 hand-off's "one paper-vs-tiger reconciliation day" needs a deterministic reference run to reconcile against. The alternatives are all blocked by data facts, not code facts: `hkq-recon` needs the independent official-close source (the standing M1 gap), Hansen SPA needs benchmark series nothing ingests, per-sector A50 betas need an A50 dataset that isn't persisted, and CPCV's first genuine consumer is the §3.7 ML layer. The mirror is the only milestone whose inputs already exist in full — and per the blueprint's own crate table ("hkq-engine — runbook state machine, actor wiring, **backtest replay**"), it lands inside the engine, where the `pub(crate)` seams stay private.

**In scope:** the `EngineClock` seam in `hkq-engine` (`WallClock` = M4's arithmetic behind the trait, `SimClock` = a UTC↔tokio-Instant anchor for virtual time); the `SimExec` venue in `hkq-exec` (full `Submission::Immediate` fills at the latest completed bar's VWAP, adversely adjusted by half-spread + κ-impact — price realism only, because the Book already owns statutory costs); the replay driver `hkq-engine::replay` (historical `Bars1m`/`Auction`/`MainlandPrints` partitions → one merged, sorted event pump on the virtual timeline, the *unchanged* `run_day` select loop, the *unchanged* exec actor, write-isolated into a sandbox lake root); the `hkq-backtest` binary (the blueprint's third bin: paused-time current-thread runtime, date-range loop, §4 PnL summary through `hkq-validate`'s existing `pnl`/`stats` path, and an honest `sr` trial appended to the production registry); one honesty patch to PreMarket (`load_weights` bounded to dates strictly before the trading day — byte-identical live behavior, correct under replay); and the workspace gaining tokio's `test-util` feature, which the paused-time machinery requires (and which M6's `start_paused` actor tests already assumed). **Deferred:** as-of state snapshotting for point-in-time reconstruction (the promotion protocol's job, with the trials registry — M8), scripted-`Routed` partial-fill realism as a reusable venue (the M6 `FakeRouted` pattern covers tests; a config-driven venue waits for a consumer), POS-band/VCM/quote replay (those events are not persisted — a data-milestone fact, unchanged owners), Hansen SPA benchmarks, VHSI, A50, CPCV, and `hkq-recon` (all unchanged deferrals with unchanged owners).

Engineering decisions beyond the blueprint sketch, briefly. The virtual clock is tokio's own paused time, not a bespoke scheduler: the whole system — schedule timers, the exec actor's poll interval, halt deadlines, channel wakeups — already runs on tokio time, so anchoring one `SimClock` origin and running a current-thread `start_paused` runtime replays a full day at memory speed while every interleaving resolves in strict virtual-timestamp order. This is what maximizes the mirror's fidelity: `run_day` is not reimplemented, the exec actor is not bypassed, and the same biased select that runs live runs in replay. Bars are delivered at open + 60 s + 1 s — the completion instant plus one poll tick of feed latency — so boundary races resolve *exactly as live*: the 09:44 bar arrives after the 09:45 Entry action, the 09:34 bar after the 09:35 X3 refresh, because that is what the live poller does; auction snaps replay at their recorded arrival timestamps and mainland prints at 09:25:05, mirroring `hkq-live`'s one-shot. One pump task with a pre-merged, stably-sorted event list keeps ordering deterministic (SimExec has no lifecycle events, so the actor's unbiased poll arm is behaviorally inert). SimExec owns *price* realism only — VWAP (turnover/volume, close fallback) adversely adjusted by s/2 + κ·σ·√(q/ADV) — never statutory costs, because M4 put stamp-ceiling and fees inside `Book::apply_fill` precisely so both tiers share one accounting path; fills are full (queue dynamics are out of scope by design, as M6 assigned partial-fill realism to scripted Routed venues), marks are fed by the driver *before* each bar dispatches so a child minted from bar t fills against bar t's VWAP — the standard trade-at-bar convention, documented as such. Write isolation is structural, not procedural: PostClose artifacts (Scores/Fills/Auction/Attribution, the alpha refit path) land in a sandbox lake via a sandbox config clone, so a replay *cannot* contaminate the shadow record, the learned state, or the CUSUM's input series — and the `hkq-validate report` job pointed at the sandbox root computes SR/CI/DSR over replayed history with zero new code, which is the §4 pipeline composing for free. The one honesty patch: `load_weights` now filters attribution to dates strictly before the trading day — in live this is a no-op (today's row cannot exist at 08:45), in replay it prevents ICIR weights from seeing the future. Everything the replay cannot time-travel is stated loudly on the API: current alpha map, gate, AH-β, and config constants make a replay a *mechanical counterfactual with today's learned state*, not a point-in-time reconstruction — as-of snapshotting belongs to the promotion protocol.

```text
hkq/
├── Cargo.toml                        (updated: member, tokio test-util)
└── crates/
    ├── hkq-exec/
    │   └── src/{lib,sim}.rs          (lib updated; sim.rs NEW — the §4 fill model)
    ├── hkq-engine/
    │   └── src/{lib,clock,replay,day,premarket}.rs
    │                                 (clock.rs, replay.rs NEW; day/premarket surgical patches)
    └── hkq-backtest/
        ├── Cargo.toml                (NEW)
        └── src/main.rs               (NEW — the blueprint's third binary)
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
  "crates/hkq-live", "crates/hkq-backtest",
]

[workspace.package]
edition = "2021"
rust-version = "1.83"

[workspace.dependencies]
# test-util: tokio's virtual clock. The replay tier runs production replays on a
# start_paused current-thread runtime (hkq-backtest owns it), and the M6 exec
# actor tests already assume paused time. Live binaries never pause the clock.
tokio        = { version = "1.38", features = ["full", "test-util"] }
tokio-stream = "0.1"
reqwest      = { version = "0.12", default-features = false, features = ["json", "gzip", "rustls-tls"] }
serde        = { version = "1", features = ["derive"] }
serde_json   = "1"
# Feature set unchanged since M2 — M7 adds no new expression surface.
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

Three append-blocks and four line-swaps, in the M5/M6 style — everything else in M1–M6 stays byte-identical, and `hkq-live` is untouched (the clock is a defaulted field, not a signature change).

```rust
// (append to the imports in crates/hkq-engine/src/day.rs)
use crate::clock::{EngineClock, WallClock};
use std::sync::Arc;
```

```rust
// (append inside crates/hkq-engine/src/day.rs `pub struct TradingDay`, after `post_close_done`)
    /// M7: the clock seam. WallClock in every constructor; the replay driver
    /// (crate::replay) swaps in a SimClock through this pub(crate) field.
    pub(crate) clock: Arc<dyn EngineClock>,
```

In `TradingDay::new`, append one line to the struct literal (after `post_close_done: false,`):

```rust
            clock: Arc::new(WallClock),
```

In `run_day`, replace

```rust
        let mut timer = Box::pin(tokio::time::sleep_until(instant_for(self.date, sched[0].0)));
```

with

```rust
        let mut timer = Box::pin(tokio::time::sleep_until(self.clock.instant_for(self.date, sched[0].0)));
```

and replace

```rust
                    timer.as_mut().reset(instant_for(self.date, sched[idx].0));
```

with

```rust
                    timer.as_mut().reset(self.clock.instant_for(self.date, sched[idx].0));
```

Both former call sites of the free function are gone, so it becomes the shared wall-time math behind `WallClock` — replace its signature line

```rust
fn instant_for(date: chrono::NaiveDate, t: chrono::NaiveTime) -> tokio::time::Instant {
```

with

```rust
/// M7: ONE copy of the wall-time arithmetic — `clock::WallClock` delegates here.
pub(crate) fn wall_instant_for(date: chrono::NaiveDate, t: chrono::NaiveTime) -> tokio::time::Instant {
```

And the PreMarket honesty patch — in `crates/hkq-engine/src/premarket.rs`, replace the whole `load_weights` function with the date-bounded version (live behavior is byte-identical: at 08:45 no attribution partition ≥ `date` can exist; under replay this stops ICIR weights from reading the future):

```rust
/// ICIR weights from the attribution panel, bounded to dates STRICTLY BEFORE
/// the trading day (M7): in live this filter is vacuous — today's row is only
/// written at PostClose — but a replayed historical day must not let its
/// weights see attribution rows from its own future.
fn load_weights(lake: &Lake, cfg: &StrategyCfg, as_of: NaiveDate) -> (FactorWeights, FactorWeights) {
    let equal = || (FactorWeights::equal(&S_FACTORS), FactorWeights::equal(&X_FACTORS));
    let Ok(lf) = lake.scan(Dataset::Attribution) else {
        tracing::info!("no attribution history: ICIR weights start EQUAL (cold start)");
        return equal();
    };
    let Ok(panel) = lf
        .filter(col(base::DATE).lt(lit(as_of.to_string())))
        .sort_by_exprs([col(base::DATE)], Default::default())
        .collect()
    else {
        return equal();
    };
    if panel.height() == 0 {
        return equal();
    }
    let w = cfg.stage1.icir_window;
    let d = cfg.stage1.icir_shrink_delta;
    let ws = FactorWeights::from_ic_panel(&panel, &S_FACTORS, w, d)
        .unwrap_or_else(|e| { tracing::warn!(error = %e, "s-weights fallback equal"); FactorWeights::equal(&S_FACTORS) });
    let wx = FactorWeights::from_ic_panel(&panel, &X_FACTORS, w, d)
        .unwrap_or_else(|e| { tracing::warn!(error = %e, "x-weights fallback equal"); FactorWeights::equal(&X_FACTORS) });
    (ws, wx)
}
```

with its single call site in `NightlyState::load` updated from

```rust
        let (weights_s, weights_x) = load_weights(lake, cfg);
```

to

```rust
        let (weights_s, weights_x) = load_weights(lake, cfg, date);
```

## `hkq-exec` — the SimExec venue

```rust
// crates/hkq-exec/src/lib.rs
#![forbid(unsafe_code)]
//! Execution seam (report §3.6, blueprint dataflow): parent orders in, paced
//! lot-multiple children out, fills back. As of M6 the seam carries the FULL
//! order lifecycle: a venue may fill a child immediately (paper tier), or
//! accept it for routing and report incremental fills / terminal states through
//! `poll_updates` (the signed Tiger route). The pacer's budget is reconciled
//! against venue-CONFIRMED events — submission is no longer treated as sent.
//! M7 adds the third tier: `SimExec`, the backtest venue — Immediate fills at
//! the latest completed bar's VWAP, adversely adjusted per §4 cost realism.
//!
//! Halt semantics (unchanged, deliberate asymmetry): a `Halted` risk state
//! cancels resting BUY parents — both pacer-side and venue-side — while
//! SELL flow continues untouched. A kill switch that blocked exits would be a
//! capital trap, not a safety mechanism.
//!
//! Ledger discipline: the pacer is a rate governor, not the ledger. The Book
//! (hkq-engine) is the single accounting truth; every clamp in this crate that
//! drops venue nonsense does so LOUDLY and defers to the Book.

pub mod actor;
pub mod cfg;
pub mod model;
pub mod pacing;
pub mod sim;
pub mod tiger;
pub mod venue;

pub use actor::spawn_exec;
pub use cfg::{load_exec, ExecCfg, OrderAliases, TigerExecCfg};
pub use model::{
    ChildId, ChildOrder, ExecCmd, ExecError, Fill, Pacing, ParentOrder, Side, TerminalState,
    VenueUpdate,
};
pub use pacing::Pacer;
pub use sim::{SimExec, SimName};
pub use tiger::TigerVenue;
pub use venue::{CancelScope, PaperVenue, Submission, Venue};
```

```rust
// crates/hkq-exec/src/sim.rs
//! SimExec — the backtest venue (M7). The M6 hand-off fixed its shape exactly:
//! a `Submission::Immediate` venue whose fill price applies the §4 cost-realism
//! model (half-spread + κ-impact) to the latest COMPLETED bar's reference
//! price. Fills are full — queue and resting-order dynamics are out of scope by
//! design; partial-fill realism is exercised through scripted `Routed` venues
//! in tests (M6's FakeRouted pattern) — but every fill price is ADVERSE:
//!
//!   px = ref · (1 ± (s/2 + κ·σ_cc·√(q/ADV)) / 10⁴)      (+ buys, − sells)
//!
//! where ref is the bar's VWAP (turnover / volume; close when degenerate) and
//! (s, σ_cc, ADV) are the §1 per-name cost inputs from the nightly panel.
//!
//! Statutory costs (stamp ceil-to-dollar, fees) are NOT charged here: the Book
//! charges them inside `apply_fill` (M4), so backtest and live share ONE
//! accounting path by construction. This venue owns PRICE realism only.
//!
//! Feed contract: the replay driver calls `on_bar` BEFORE dispatching the same
//! bar to the engine, so a child minted from bar t fills against bar t's VWAP —
//! the standard trade-at-bar convention, documented as such. The handle is
//! Clone-shared (Arc inner): the driver feeds marks, the exec actor fills.
use crate::model::{ChildOrder, ExecError, Fill, Side};
use crate::venue::{Submission, Venue};
use async_trait::async_trait;
use hkq_core::ids::StockCode;
use hkq_core::money::Px;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// §1 cost inputs for one name, sourced from the nightly panel at replay start.
#[derive(Debug, Clone, Copy)]
pub struct SimName {
    /// Trailing-median quoted spread s_i, in bps.
    pub spread_bps: f64,
    /// Close-to-close vol σ_i for the impact term.
    pub sigma_cc: f64,
    /// Average daily volume ADV_i, in shares.
    pub adv_shares: f64,
}

#[derive(Debug, Default)]
struct SimState {
    names: HashMap<StockCode, SimName>,
    /// Latest completed bar's reference price (VWAP; close fallback).
    marks: HashMap<StockCode, f64>,
}

#[derive(Debug, Clone)]
pub struct SimExec {
    kappa: f64,
    default_spread_bps: f64,
    state: Arc<Mutex<SimState>>,
}

impl SimExec {
    /// `kappa` from `[costs].impact_kappa`; `default_spread_bps` for names with
    /// no static spread — callers pass `[universe].max_median_spread_bps`, the
    /// same conservative fallback the candidate cost floor uses. No magic
    /// numbers in code (hkq-core's rule).
    pub fn new(kappa: f64, default_spread_bps: f64) -> Self {
        Self {
            kappa: kappa.max(0.0),
            default_spread_bps: default_spread_bps.max(0.0),
            state: Arc::new(Mutex::new(SimState::default())),
        }
    }

    /// Register the §1 inputs for one name (from `NightlyState.nums`).
    pub fn set_name(&self, code: StockCode, name: SimName) {
        self.state.lock().expect("sim state").names.insert(code, name);
    }

    /// Driver feed: one completed bar. Reference = VWAP = turnover/volume when
    /// both are sane, else the close; degenerate prices leave the mark alone.
    pub fn on_bar(&self, code: StockCode, close: f64, volume: f64, turnover: f64) {
        let vwap = if turnover.is_finite() && volume.is_finite() && volume > 0.0 && turnover > 0.0
        {
            turnover / volume
        } else {
            close
        };
        if vwap.is_finite() && vwap > 0.0 {
            self.state.lock().expect("sim state").marks.insert(code, vwap);
        }
    }
}

#[async_trait]
impl Venue for SimExec {
    async fn submit(&self, child: &ChildOrder, ts_ms: i64) -> Result<Submission, ExecError> {
        let (reference, adverse_bps) = {
            let st = self.state.lock().expect("sim state");
            // No mark yet (name never printed a bar) ⇒ the child's limit is the
            // only price we have — the engine prices every limit at a real mark,
            // so this degrades to the paper fiction for that name, loudly typed.
            let reference = st
                .marks
                .get(&child.code)
                .copied()
                .unwrap_or_else(|| child.limit.as_f64());
            let (s, sig, adv) = st.names.get(&child.code).map_or(
                (self.default_spread_bps, 0.0, 1.0),
                |n| (n.spread_bps.max(0.0), n.sigma_cc.abs(), n.adv_shares.max(1.0)),
            );
            let impact_bps = self.kappa * sig * (child.shares as f64 / adv).sqrt() * 1e4;
            (reference, s / 2.0 + impact_bps)
        };
        let dir = match child.side {
            Side::Buy => 1.0,
            Side::Sell => -1.0,
        };
        let raw = reference * (1.0 + dir * adverse_bps / 1e4);
        // 4-dp quantization at the same boundary every vendor float crosses
        // (Px::from_f64_quote). A degenerate adjusted price falls back to the
        // tick-valid limit rather than fabricating one.
        let px = Px::from_f64_quote(raw).unwrap_or(child.limit);
        Ok(Submission::Immediate(Fill {
            code: child.code,
            side: child.side,
            shares: child.shares,
            lot: child.lot,
            px,
            ts_ms,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ChildId;
    use crate::venue::CancelScope;

    fn child(side: Side, shares: u64, limit: f64) -> ChildOrder {
        ChildOrder {
            id: ChildId(1),
            code: StockCode(700),
            side,
            shares,
            lot: 100,
            limit: Px::from_f64_quote(limit).unwrap(),
        }
    }

    #[tokio::test]
    async fn adverse_fill_math_hand_computed() {
        let sim = SimExec::new(0.1, 35.0);
        sim.set_name(StockCode(700), SimName {
            spread_bps: 10.0,
            sigma_cc: 0.02,
            adv_shares: 1_000_000.0,
        });
        // VWAP is turnover/volume = 10.0 — NOT the (deliberately different) close.
        sim.on_bar(StockCode(700), 9.9, 1_000.0, 10_000.0);
        // impact = 0.1·0.02·√(10_000/10⁶)·10⁴ = 20 bps; half-spread 5 ⇒ 25 bps.
        let Submission::Immediate(buy) =
            sim.submit(&child(Side::Buy, 10_000, 10.0), 7).await.unwrap()
        else {
            panic!("sim is the immediate tier")
        };
        assert!((buy.px.as_f64() - 10.0 * 1.0025).abs() < 1e-9);
        assert_eq!(buy.shares, 10_000);
        assert_eq!(buy.ts_ms, 7);
        let Submission::Immediate(sell) =
            sim.submit(&child(Side::Sell, 10_000, 10.0), 8).await.unwrap()
        else {
            panic!()
        };
        assert!((sell.px.as_f64() - 10.0 * 0.9975).abs() < 1e-9);
        // Adverse on BOTH sides, symmetric around the reference.
        assert!((buy.px.as_f64() + sell.px.as_f64() - 20.0).abs() < 1e-9);
    }

    #[tokio::test]
    async fn fallbacks_are_typed_not_fabricated() {
        let sim = SimExec::new(0.1, 35.0);
        // No mark, no name: reference = limit, adverse = half the default spread,
        // impact zero (σ unknown ⇒ 0 — never invented).
        let Submission::Immediate(f) =
            sim.submit(&child(Side::Buy, 100, 10.0), 1).await.unwrap()
        else {
            panic!()
        };
        assert!((f.px.as_f64() - 10.0 * (1.0 + 17.5 / 1e4)).abs() < 1e-9);
        // Degenerate turnover ⇒ close is the reference.
        sim.on_bar(StockCode(700), 12.0, 500.0, f64::NAN);
        let Submission::Immediate(f) =
            sim.submit(&child(Side::Sell, 100, 10.0), 2).await.unwrap()
        else {
            panic!()
        };
        assert!((f.px.as_f64() - 12.0 * (1.0 - 17.5 / 1e4)).abs() < 1e-9);
        // Immediate tier: no lifecycle, nothing to cancel (trait defaults).
        assert!(sim.poll_updates().await.unwrap().is_empty());
        assert_eq!(sim.cancel_children(CancelScope::All).await.unwrap(), 0);
    }
}
```

## `hkq-engine` — the clock seam and the replay driver

```rust
// crates/hkq-engine/src/lib.rs
#![forbid(unsafe_code)]
//! The runbook as a state machine (report §6, blueprint dataflow). This crate is
//! deliberately thin: every statistical decision lives in hkq-factors/-signal,
//! every accounting decision in hkq-risk, every routing decision in hkq-exec.
//! What remains — and what lives here — is TIME and STATE:
//!
//! - `schedule`: the daily runbook as sorted data, half-day aware.
//! - `premarket`: rebuild all derived factor state from the M1 lake at 08:45.
//! - `morning`: absorb POS/mainland/A50 into the pre-freeze board.
//! - `freeze`: 09:29:30 OpenContext assembly → Stage 1 → Stage 2; 09:35 X3
//!   column swap; candidate construction for the 09:45 gate.
//! - `book`: single-writer Decimal book; statutory costs charged per fill.
//! - `day`: the `tokio::select!` loop — a clock around tested functions.
//! - `clock` (M7): the clock seam — WallClock live, SimClock in replay.
//! - `replay` (M7): the mirror — the SAME TradingDay and exec actor re-run
//!   against historical partitions on a virtual timeline, write-isolated.

pub mod book;
pub mod clock;
pub mod cols;
pub mod day;
pub mod error;
pub mod freeze;
pub mod morning;
pub mod premarket;
pub mod replay;
pub mod schedule;

pub use clock::{EngineClock, SimClock, WallClock};
pub use day::{Channels, RunCfg, TradingDay};
pub use error::EngineError;
pub use premarket::NightlyState;
pub use replay::{run_replay, ReplayReport};
pub use schedule::{build_schedule, Action, Phase};
```

```rust
// crates/hkq-engine/src/clock.rs
//! The clock seam (M7). The ONLY thing the trading day asks about time is
//! "give me the tokio Instant for (date, HKT time)" — so the seam is exactly
//! that one question behind a trait. `WallClock` is M4's arithmetic; `SimClock`
//! anchors an arbitrary UTC origin onto the runtime's timeline, so that under a
//! START-PAUSED runtime a historical day replays in virtual time: tokio
//! auto-advances to the next deadline whenever the system is idle, and every
//! timer/event interleaving resolves in strict virtual-timestamp order.
//! Everything downstream — schedule timers, the exec actor's poll interval,
//! halt deadlines — already runs on tokio time and needs no changes.
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use std::time::Duration;

pub trait EngineClock: Send + Sync {
    /// The tokio Instant corresponding to HKT wall time `t` on `date`.
    fn instant_for(&self, date: NaiveDate, t: NaiveTime) -> tokio::time::Instant;
}

/// Live tier: target − real now, clamped at zero (M4's `instant_for`, shared).
#[derive(Debug, Default, Clone, Copy)]
pub struct WallClock;

impl EngineClock for WallClock {
    fn instant_for(&self, date: NaiveDate, t: NaiveTime) -> tokio::time::Instant {
        crate::day::wall_instant_for(date, t)
    }
}

/// Replay tier: a fixed (origin_utc ↔ origin_instant) anchor. Targets before
/// the anchor clamp to it — they fire immediately, in registration order.
///
/// Construct INSIDE the runtime (the anchor reads `tokio::time::Instant::now()`,
/// which is virtual under a paused runtime). On a live clock this type still
/// works — the replay would simply run at real-time speed — so the paused
/// runtime is a property the calling binary owns, not this type.
#[derive(Debug, Clone, Copy)]
pub struct SimClock {
    origin: tokio::time::Instant,
    origin_utc_ms: i64,
}

impl SimClock {
    pub fn anchored_at(origin_utc: DateTime<Utc>) -> Self {
        Self {
            origin: tokio::time::Instant::now(),
            origin_utc_ms: origin_utc.timestamp_millis(),
        }
    }

    /// Virtual Instant for an absolute UTC epoch-millisecond timestamp.
    pub fn instant_for_ms(&self, utc_ms: i64) -> tokio::time::Instant {
        let delta = (utc_ms - self.origin_utc_ms).max(0) as u64;
        self.origin + Duration::from_millis(delta)
    }
}

impl EngineClock for SimClock {
    fn instant_for(&self, date: NaiveDate, t: NaiveTime) -> tokio::time::Instant {
        self.instant_for_ms(
            hkq_core::session::hk(date, t).with_timezone(&Utc).timestamp_millis(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[tokio::test(start_paused = true)]
    async fn sim_clock_maps_utc_deltas_onto_the_virtual_timeline() {
        let origin = Utc.with_ymd_and_hms(2026, 7, 3, 0, 30, 0).unwrap(); // 08:30 HKT
        let clock = SimClock::anchored_at(origin);
        let now = tokio::time::Instant::now();
        let t = clock.instant_for_ms(origin.timestamp_millis() + 5_000);
        assert_eq!(t.duration_since(now), Duration::from_secs(5));
        // Pre-anchor targets clamp to the origin (fire immediately).
        let t = clock.instant_for_ms(origin.timestamp_millis() - 60_000);
        assert_eq!(t.duration_since(now), Duration::ZERO);
        // HKT mapping: 09:00 HKT = 01:00 UTC ⇒ 30 virtual minutes after anchor.
        let d = chrono::NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        let nine = chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap();
        let t = EngineClock::instant_for(&clock, d, nine);
        assert_eq!(t.duration_since(now), Duration::from_secs(30 * 60));
    }

    #[tokio::test(start_paused = true)]
    async fn paused_time_auto_advances_through_sim_deadlines() {
        // One virtual hour elapses in ~zero real time — the replay mechanism.
        let origin = Utc.with_ymd_and_hms(2026, 7, 3, 1, 0, 0).unwrap();
        let clock = SimClock::anchored_at(origin);
        let start = tokio::time::Instant::now();
        tokio::time::sleep_until(clock.instant_for_ms(origin.timestamp_millis() + 3_600_000))
            .await;
        assert_eq!(
            tokio::time::Instant::now().duration_since(start),
            Duration::from_secs(3600)
        );
    }
}
```

```rust
// crates/hkq-engine/src/replay.rs
//! The mirror (M7; blueprint: "backtest replay" is hkq-engine scope). The SAME
//! TradingDay, the SAME schedule, the SAME exec actor — re-run against
//! historical partitions on a virtual timeline. Exactly two swaps, both behind
//! traits: SimClock (EngineClock) and SimExec (hkq-exec::Venue). Everything
//! else is the identical code path; that is the entire point of a mirror.
//!
//! Time model: run inside a CURRENT-THREAD, START-PAUSED tokio runtime (the
//! hkq-backtest binary owns that). Under paused time, sleeps auto-advance when
//! the runtime is idle, so a day replays at memory speed while every
//! timer/event interleaving resolves in strict virtual-timestamp order —
//! including the same boundary races live has: a bar completing at 09:45:00
//! arrives AFTER the 09:45 Entry action, exactly as it does through the live
//! poller. Bars are delivered at open + 60 s + 1 s (completion plus one poll
//! tick of feed latency); auction snaps replay at their recorded arrival
//! timestamps; mainland prints at 09:25:05, mirroring hkq-live's one-shot.
//!
//! Write isolation: PostClose artifacts (Scores/Fills/Auction/Attribution and
//! the alpha-refit path) land in a SANDBOX lake root via a sandbox config
//! clone. A replay CANNOT touch the production lake, the shadow record, or the
//! CUSUM's input series. Inputs (PreMarket panels, learned state) read from
//! the production root.
//!
//! Honesty caveat, stated once and loudly: a replay consumes the CURRENT
//! learned state (alpha map, regime gate, AH-β; ICIR weights are date-bounded)
//! and current strategy.toml constants. It is a mechanical counterfactual for
//! validating the machine and studying costs — NOT a point-in-time
//! reconstruction. As-of state snapshotting is the promotion protocol's job
//! (hkq-validate, with the trials registry).
use crate::clock::SimClock;
use crate::day::{Channels, RunCfg, TradingDay};
use crate::error::EngineError;
use crate::premarket::NightlyState;
use chrono::{NaiveDate, NaiveTime, Utc};
use hkq_core::config::StrategyCfg;
use hkq_core::ids::StockCode;
use hkq_core::money::Cash;
use hkq_core::session::{hk, SessionTimes, TradingCalendar};
use hkq_data::lake::{Dataset, Lake};
use hkq_data::model::{ms_to_hkt, AuctionSnap, Bar1m, MarketEvent};
use hkq_exec::{spawn_exec, SimExec, SimName};
use hkq_factors::cols::{self, base};
use hkq_risk::KillSwitch;
use polars::prelude::*;
use std::path::Path;
use std::sync::Arc;

/// Feed latency added to a bar's completion instant (open + 60 s): completed
/// bars arrive on the next poll tick, never at the exact boundary, so schedule
/// ties resolve exactly as they do live (timers first).
const BAR_LATENCY_MS: i64 = 1_000;

#[derive(Debug, Clone)]
pub struct ReplayReport {
    pub date: NaiveDate,
    pub events: usize,
    pub bars: usize,
    pub auction_snaps: usize,
    pub mainland_prints: usize,
    /// Fill rows persisted to the sandbox (0 ⇒ nothing traded).
    pub fills: usize,
    /// Stage-2 rows persisted (0 ⇒ cash day: Σ_min gate or entry gates).
    pub scored_names: usize,
}

fn f64col(df: &DataFrame, name: &str) -> Result<Float64Chunked, EngineError> {
    Ok(df.column(name)?.as_materialized_series().f64()?.clone())
}

/// Bars1m partition → Bar events at completion + latency. Hard requirement.
fn bar_events(lake: &Lake, date: NaiveDate) -> Result<Vec<(i64, MarketEvent)>, EngineError> {
    let df = lake
        .scan_date(Dataset::Bars1m, date)?
        .sort_by_exprs([col(base::CODE), col(base::TS_MS)], Default::default())
        .collect()?;
    let code = df.column(base::CODE)?.as_materialized_series().u32()?.clone();
    let ts = df.column(base::TS_MS)?.as_materialized_series().i64()?.clone();
    let (o, h, l, c) = (
        f64col(&df, cols::O1M)?, f64col(&df, cols::H1M)?,
        f64col(&df, cols::L1M)?, f64col(&df, cols::C1M)?,
    );
    let (v, t) = (f64col(&df, base::VOLUME)?, f64col(&df, base::TURNOVER)?);
    let mut out = Vec::with_capacity(df.height());
    for i in 0..df.height() {
        let (Some(cd), Some(ms)) = (code.get(i), ts.get(i)) else { continue };
        let Some(hkt) = ms_to_hkt(ms) else { continue };
        out.push((
            ms + 60_000 + BAR_LATENCY_MS,
            MarketEvent::Bar(Bar1m {
                code: StockCode(cd),
                ts: hkt,
                o: o.get(i).unwrap_or(f64::NAN),
                h: h.get(i).unwrap_or(f64::NAN),
                l: l.get(i).unwrap_or(f64::NAN),
                c: c.get(i).unwrap_or(f64::NAN),
                volume: v.get(i).unwrap_or(0.0),
                turnover: t.get(i).unwrap_or(f64::NAN),
            }),
        ));
    }
    Ok(out)
}

/// Auction partition (persisted by a live/shadow run) → AuctionSnap events at
/// their RECORDED arrival timestamps. Absent ⇒ the null-S2/X1 path, exactly as
/// a live day without a POS feed (§5 X2-disabled degradation).
fn auction_events(lake: &Lake, date: NaiveDate) -> Vec<(i64, MarketEvent)> {
    let Ok(lf) = lake.scan_date(Dataset::Auction, date) else {
        tracing::info!(%date, "no auction partition — replay runs the null-S2/X1 path");
        return vec![];
    };
    let inner = || -> Result<Vec<(i64, MarketEvent)>, EngineError> {
        let df = lf.collect()?;
        let code = df.column(base::CODE)?.as_materialized_series().u32()?.clone();
        let ts = df.column(base::TS_MS)?.as_materialized_series().i64()?.clone();
        let (iep, iev) = (f64col(&df, cols::IEP)?, f64col(&df, cols::IEV)?);
        let (bq, aq) = (f64col(&df, "bid_qty")?, f64col(&df, "ask_qty")?);
        let mut out = Vec::with_capacity(df.height());
        for i in 0..df.height() {
            let (Some(cd), Some(ms)) = (code.get(i), ts.get(i)) else { continue };
            let Some(hkt) = ms_to_hkt(ms) else { continue };
            out.push((
                ms,
                MarketEvent::Auction(AuctionSnap {
                    code: StockCode(cd),
                    ts: hkt,
                    iep: iep.get(i),
                    iev: iev.get(i),
                    bid_qty: bq.get(i),
                    ask_qty: aq.get(i),
                }),
            ));
        }
        Ok(out)
    };
    match inner() {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(error = %e, "auction partition unreadable; replayed without it");
            vec![]
        }
    }
}

/// MainlandPrints partition → one-shot prints at 09:25:05, mirroring the live
/// binary's mainland task (mainland_print + 5 s). Absent ⇒ S6 degrades.
fn mainland_events(lake: &Lake, date: NaiveDate) -> Vec<(i64, MarketEvent)> {
    let Ok(lf) = lake.scan_date(Dataset::MainlandPrints, date) else {
        tracing::info!(%date, "no mainland partition — S6 runs degraded");
        return vec![];
    };
    let at_ms = hk(date, SessionTimes::get().mainland_print)
        .with_timezone(&Utc)
        .timestamp_millis()
        + 5_000;
    let inner = || -> Result<Vec<(i64, MarketEvent)>, EngineError> {
        let df = lf.collect()?;
        let code = df.column(base::CODE)?.as_materialized_series().u32()?.clone();
        let ret = f64col(&df, base::A_OPEN_RET)?;
        let mut out = Vec::with_capacity(df.height());
        for i in 0..df.height() {
            let (Some(cd), Some(r)) = (code.get(i), ret.get(i)) else { continue };
            out.push((
                at_ms,
                MarketEvent::MainlandAuctionPrint { code: StockCode(cd), a_open_ret: r },
            ));
        }
        Ok(out)
    };
    match inner() {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(error = %e, "mainland partition unreadable; replayed without it");
            vec![]
        }
    }
}

fn partition_height(lake: &Lake, ds: Dataset, date: NaiveDate) -> usize {
    lake.scan_date(ds, date)
        .and_then(|lf| lf.collect().map_err(Into::into))
        .map(|df| df.height())
        .unwrap_or(0)
}

/// Replay one historical trading day through the unchanged TradingDay.
///
/// MUST run inside a current-thread, start-paused tokio runtime (hkq-backtest
/// owns that); on a live clock this function still works, but replays in real
/// time. `out_root` MUST differ from the production lake root — the caller
/// enforces it, and this function never writes anywhere else.
pub async fn run_replay(
    prod_lake: &Lake,
    cfg: &StrategyCfg,
    out_root: &Path,
    date: NaiveDate,
    equity: Cash,
    cal: &dyn TradingCalendar,
) -> Result<ReplayReport, EngineError> {
    // 1) PreMarket from the PRODUCTION lake — inputs, exactly as hkq-live.
    let state = NightlyState::load(prod_lake, cfg, date, cal)?;

    // 2) Sandbox config: identical constants, every write path redirected
    //    (PostClose partitions AND the alpha-refit state file).
    let mut sandbox_cfg = cfg.clone();
    sandbox_cfg.ops.lake_root = out_root.to_path_buf();

    // 3) Historical events, merged and stably sorted on the virtual timeline.
    let mut events = auction_events(prod_lake, date);
    let n_auction = events.len();
    let mainland = mainland_events(prod_lake, date);
    let n_mainland = mainland.len();
    events.extend(mainland);
    let bars = bar_events(prod_lake, date)?;
    let n_bars = bars.len();
    if n_bars == 0 {
        return Err(EngineError::State(format!(
            "no 1m bars for {date} — nothing to replay (run hkq-nightly backfill)"
        )));
    }
    events.extend(bars);
    events.sort_by_key(|(ms, _)| *ms);
    let n_events = events.len();

    // 4) SimExec with the §1 cost inputs from the nightly panel.
    let sim = SimExec::new(cfg.costs.impact_kappa, cfg.universe.max_median_spread_bps);
    for (code, nums) in &state.nums {
        sim.set_name(*code, SimName {
            spread_bps: nums.spread_bps.unwrap_or(cfg.universe.max_median_spread_bps),
            sigma_cc: nums.sigma_cc.unwrap_or(0.0),
            adv_shares: nums.adv_shares.unwrap_or(1.0),
        });
    }

    // 5) Virtual clock anchored before the first schedule boundary (09:00).
    let anchor = NaiveTime::from_hms_opt(8, 30, 0).expect("valid HKT time");
    let clock = SimClock::anchored_at(hk(date, anchor).with_timezone(&Utc));

    // 6) The SAME actors as live: exec actor, kill switch, channels. `_ks`
    //    stays bound — dropping the switch would close the watch channel.
    let (_ks, kill_rx) = KillSwitch::new();
    let (fill_tx, fill_rx) = tokio::sync::mpsc::channel(4096);
    let (exec_tx, _exec_handle) =
        spawn_exec(sim.clone(), cfg.trade.participation_cap, fill_tx, kill_rx.clone());
    let (md_tx, md_rx) = tokio::sync::mpsc::channel::<MarketEvent>(8192);

    // 7) ONE pump, marks-before-delivery: the venue's reference price is the
    //    bar that just completed, never a future one. The sender is held open
    //    after the last event so run_day's md arm stays quiet through PostClose
    //    (a closed channel would spin the select loop and stall auto-advance).
    let pump_sim = sim.clone();
    let pump = tokio::spawn(async move {
        for (ms, ev) in events {
            tokio::time::sleep_until(clock.instant_for_ms(ms)).await;
            if let MarketEvent::Bar(b) = &ev {
                pump_sim.on_bar(b.code, b.c, b.volume, b.turnover);
            }
            if md_tx.send(ev).await.is_err() {
                return;
            }
        }
        std::future::pending::<()>().await;
    });

    // 8) The SAME TradingDay, on the virtual clock, writing to the sandbox.
    let mut day = TradingDay::new(
        sandbox_cfg,
        RunCfg { equity },
        date,
        Lake::new(out_root),
        state,
        exec_tx,
    )?;
    day.clock = Arc::new(clock);
    let result = day.run_day(cal, Channels { md_rx, fill_rx, kill_rx }).await;
    pump.abort();
    result?;

    // 9) Cheap read-back facts for the report — all from the sandbox.
    let out_lake = Lake::new(out_root);
    Ok(ReplayReport {
        date,
        events: n_events,
        bars: n_bars,
        auction_snaps: n_auction,
        mainland_prints: n_mainland,
        fills: partition_height(&out_lake, Dataset::Fills, date),
        scored_names: partition_height(&out_lake, Dataset::Scores, date),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    use hkq_core::config::*;
    use hkq_core::session::DayKind;
    use polars::df;
    use std::path::PathBuf;

    struct FixedCal;
    impl TradingCalendar for FixedCal {
        fn day_kind(&self, d: NaiveDate) -> DayKind {
            use chrono::Datelike;
            if matches!(d.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun) {
                DayKind::Closed
            } else {
                DayKind::Full
            }
        }
        fn prev_trading_day(&self, d: NaiveDate) -> NaiveDate {
            let mut c = d;
            loop {
                c = c.pred_opt().unwrap();
                if self.day_kind(c) != DayKind::Closed {
                    return c;
                }
            }
        }
    }

    fn ts(d: NaiveDate, h: u32, m: u32) -> i64 {
        hk(d, NaiveTime::from_hms_opt(h, m, 0).unwrap()).timestamp_millis()
    }

    fn cfg_for(root: &Path, static_path: &Path) -> StrategyCfg {
        StrategyCfg {
            universe: UniverseCfg { min_median_turnover_hkd: 0.0, min_price_hkd: 0.0,
                min_listed_days: 0, max_median_spread_bps: 35.0 },
            factors: FactorCfg { ewma_halflife_days: 2.0, ewma_min_obs: 2, amihud_window: 3,
                rv_days: 2, lav_gamma: 0.3, seasonal_vol_days: 2, ivu_tercile_window: 4 },
            stage1: Stage1Cfg { theta1: 1.0, theta2: 1.0, eta: 0.25, vs_threshold: 0.5,
                leadlag_window: 250, fdr_q: 0.10, icir_window: 250, icir_shrink_delta: 0.10,
                top_k_sectors: 2, sigma_min_gate: -1.0e9, member_weight_cap: 0.9 },
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
                universe_static_path: Some(static_path.into()), log_json: false },
        }
    }

    /// M4-premarket-style fixture: statics, 10 warmup weekdays (daily + sparse
    /// 1m bars), a full replay day of rising minute bars (low == close so no
    /// stop noise), and a flat 100 bps alpha prior (without it the shadow
    /// stance trades nothing — the M3 cold-start rule, working as designed).
    fn build_prod_lake(root: &Path, date: NaiveDate) -> PathBuf {
        std::fs::create_dir_all(root).unwrap();
        let lake = Lake::new(root);
        let cal = FixedCal;

        let static_path = root.join("universe_static.parquet");
        let mut st = df!(
            "code" => vec![700u32, 5u32],
            "sector" => vec![1u32, 2u32],
            "float_cap" => vec![100.0, 80.0],
            "board_lot" => vec![100u32, 400u32],
            "connect_elig" => vec![1u32, 1u32],
            "spread_bps" => vec![Some(5.0), Some(8.0)],
        ).unwrap();
        let f = std::fs::File::create(&static_path).unwrap();
        ParquetWriter::new(f).finish(&mut st).unwrap();

        let mut d = date - chrono::Duration::days(20);
        let mut days = vec![];
        while days.len() < 10 {
            if cal.day_kind(d) == DayKind::Full && d < date {
                days.push(d);
            }
            d = d.succ_opt().unwrap();
        }
        for (i, day) in days.iter().enumerate() {
            let px = 100.0 + i as f64;
            let mut daily = df!(
                "code" => vec![700u32, 5u32],
                "date" => vec![day.to_string(); 2],
                "open" => vec![px, 60.0],
                "high" => vec![px + 1.0, 61.0],
                "low" => vec![px - 1.0, 59.0],
                "close" => vec![px + 0.5, 60.5],
                "adj_close" => vec![px + 0.5, 60.5],
                "volume" => vec![1.0e6, 2.0e6],
                "turnover" => vec![1.0e8, 1.2e8],
            ).unwrap();
            lake.write_partition(Dataset::DailyBars, *day, &mut daily, "test", 1).unwrap();
            let mk = |cd: u32, p: f64| df!(
                "code" => vec![cd; 4],
                "date" => vec![day.to_string(); 4],
                "ts_ms" => vec![ts(*day, 9, 30), ts(*day, 9, 35), ts(*day, 9, 40), ts(*day, 14, 30)],
                "o" => vec![p; 4], "h" => vec![p * 1.01; 4], "l" => vec![p * 0.99; 4],
                "c" => vec![p, p * 1.001, p * 0.999, p * 1.002],
                "volume" => vec![1000.0; 4],
                "turnover" => vec![p * 1000.0; 4],
            ).unwrap();
            let mut bars = mk(700, px);
            bars.vstack_mut(&mk(5, 60.0)).unwrap();
            lake.write_partition(Dataset::Bars1m, *day, &mut bars, "test", 1).unwrap();
        }

        // Replay day: full session, gently rising, h == l == c (no stop noise).
        let (mut code, mut ds, mut tsv, mut o, mut h, mut l, mut c, mut v, mut t) =
            (vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]);
        {
            let mut push = |cd: u32, hh: u32, mm: u32, p: f64| {
                code.push(cd); ds.push(date.to_string()); tsv.push(ts(date, hh, mm));
                o.push(p); h.push(p); l.push(p); c.push(p);
                v.push(1.0e5); t.push(p * 1.0e5);
            };
            let mut minutes = vec![];
            for m in 0..150u32 { minutes.push((9 + (30 + m) / 60, (30 + m) % 60)); }
            for m in 0..180u32 { minutes.push((13 + m / 60, m % 60)); }
            for (k, (hh, mm)) in minutes.into_iter().enumerate() {
                let g = k as f64;
                push(700, hh, mm, 110.0 * (1.0 + 0.0004 * g));
                push(5, hh, mm, 60.0 * (1.0 + 0.0002 * g));
            }
        }
        let mut bars = df!(
            "code" => code, "date" => ds, "ts_ms" => tsv,
            "o" => o, "h" => h, "l" => l, "c" => c,
            "volume" => v, "turnover" => t,
        ).unwrap();
        lake.write_partition(Dataset::Bars1m, date, &mut bars, "test", 1).unwrap();

        let alpha = hkq_signal::AlphaMap::flat(100.0);
        let ap = crate::premarket::alpha_state_path(root);
        std::fs::create_dir_all(ap.parent().unwrap()).unwrap();
        std::fs::write(&ap, serde_json::to_vec_pretty(&alpha).unwrap()).unwrap();

        static_path
    }

    #[test]
    fn bar_events_deliver_at_completion_plus_poll_latency() {
        let root = std::env::temp_dir().join(format!(
            "hkq_replay_bars_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let lake = Lake::new(&root);
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        let mut df = df!(
            "code" => vec![700u32, 700],
            "date" => vec![d.to_string(); 2],
            "ts_ms" => vec![ts(d, 9, 31), ts(d, 9, 30)], // unsorted on purpose
            "o" => vec![10.0, 10.0], "h" => vec![10.0, 10.0],
            "l" => vec![10.0, 10.0], "c" => vec![10.1, 10.0],
            "volume" => vec![5.0, 5.0], "turnover" => vec![50.5, 50.0],
        ).unwrap();
        lake.write_partition(Dataset::Bars1m, d, &mut df, "test", 1).unwrap();
        let ev = bar_events(&lake, d).unwrap();
        assert_eq!(ev.len(), 2);
        assert_eq!(ev[0].0, ts(d, 9, 30) + 61_000); // sorted, completion + 1 s
        assert_eq!(ev[1].0, ts(d, 9, 31) + 61_000);
        let MarketEvent::Bar(b) = &ev[1].1 else { panic!() };
        assert_eq!(b.code, StockCode(700));
        assert!((b.c - 10.1).abs() < 1e-12);
        std::fs::remove_dir_all(root).ok();
    }

    #[test]
    fn optional_partitions_default_to_empty_not_errors() {
        let root = std::env::temp_dir().join(format!(
            "hkq_replay_opt_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let lake = Lake::new(&root);
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        assert!(auction_events(&lake, d).is_empty());
        assert!(mainland_events(&lake, d).is_empty());
        std::fs::remove_dir_all(root).ok();
    }

    /// The milestone's proof: a full day replays through the UNCHANGED
    /// TradingDay + exec actor on the virtual clock, actually trades, ends
    /// flat (run_day's terminal invariant), and every write lands in the
    /// sandbox — the production lake stays byte-untouched.
    #[tokio::test(start_paused = true)]
    async fn full_day_mirror_trades_and_stays_isolated() {
        let root = std::env::temp_dir().join(format!(
            "hkq_replay_e2e_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let date = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap(); // Friday
        let static_path = build_prod_lake(&root, date);
        let cfg = cfg_for(&root, &static_path);
        let prod = Lake::new(&root);
        let out_root = root.join("_backtest");
        let equity = Cash(rust_decimal::Decimal::from(1_000_000));

        let rep = run_replay(&prod, &cfg, &out_root, date, equity, &FixedCal)
            .await
            .expect("replay completes and the book ends flat");

        assert_eq!(rep.bars, 660);
        assert_eq!(rep.events, 660); // no auction/mainland partitions in fixture
        assert!(rep.scored_names > 0, "expected a non-cash day");
        assert!(rep.fills > 0, "the mirror must actually trade");

        // Write isolation: PostClose landed in the sandbox, not production.
        let out = Lake::new(&out_root);
        assert!(out.exists(Dataset::Fills, date));
        assert!(out.exists(Dataset::Scores, date));
        assert!(!prod.exists(Dataset::Fills, date));
        assert!(!prod.exists(Dataset::Scores, date));
        assert!(!prod.exists(Dataset::Attribution, date));

        // Determinism of the virtual timeline: replaying the same day again
        // (idempotent partition overwrite) reproduces the same fill count.
        let rep2 = run_replay(&prod, &cfg, &out_root, date, equity, &FixedCal)
            .await
            .unwrap();
        assert_eq!(rep2.fills, rep.fills);
        assert_eq!(rep2.scored_names, rep.scored_names);

        std::fs::remove_dir_all(root).ok();
    }
}
```

## `hkq-backtest` — the third binary

```toml
# crates/hkq-backtest/Cargo.toml
[package]
name = "hkq-backtest"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-data = { path = "../hkq-data" }
hkq-engine = { path = "../hkq-engine" }
hkq-validate = { path = "../hkq-validate" }
tokio.workspace = true
anyhow.workspace = true
chrono.workspace = true
rust_decimal.workspace = true
serde_json.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
```

```rust
// crates/hkq-backtest/src/main.rs
//! The mirror's binary (blueprint bin: hkq-backtest).
//! Usage: hkq-backtest <strategy.toml> --equity <HKD> --from YYYY-MM-DD
//!        [--to YYYY-MM-DD] [--out <dir>]
//!
//! Each trading day runs through the UNCHANGED TradingDay + exec actor on a
//! virtual clock (current-thread runtime, paused time — replays finish at
//! memory speed), with SimExec providing §4 cost-realistic fills. All outputs
//! land in the sandbox root (default <lake_root>/_backtest), never the
//! production lake. Two protocol hooks close the loop: the sandbox is a valid
//! Fills history, so `hkq-validate report` pointed at it computes CIs and DSR
//! unchanged; and every run appends an `sr`-carrying trial to the PRODUCTION
//! trials registry — a backtest is a trial, and DSR's N only ever grows.
use anyhow::Context;
use chrono::NaiveDate;
use hkq_core::{calendar::FileCalendar, config::StrategyCfg, money::Cash,
               session::{DayKind, TradingCalendar}};
use hkq_data::lake::Lake;
use hkq_engine::replay::run_replay;
use hkq_validate::cfg::load_validate;
use hkq_validate::registry::{sha1_hex_of_file, TrialsRegistry};
use hkq_validate::{pnl, stats};
use rust_decimal::Decimal;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;

const USAGE: &str =
    "usage: hkq-backtest <strategy.toml> --equity <HKD> --from YYYY-MM-DD [--to YYYY-MM-DD] [--out <dir>]";

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
    runtime.block_on(run(cfg_path, from, to, out, equity))
}

async fn run(
    cfg_path: String, from: NaiveDate, to: NaiveDate, out: Option<PathBuf>, equity: Decimal,
) -> anyhow::Result<()> {
    let cfg = StrategyCfg::load(&cfg_path)?;
    let calendar = FileCalendar::load(&cfg.ops.calendar_path)?;
    let prod_lake = Lake::new(&cfg.ops.lake_root);
    let out_root = out.unwrap_or_else(|| cfg.ops.lake_root.join("_backtest"));
    anyhow::ensure!(
        out_root != cfg.ops.lake_root,
        "--out must differ from ops.lake_root: a replay must never write into the production lake"
    );

    let (mut ok_days, mut failed_days) = (0usize, 0usize);
    let mut d = from;
    while d <= to {
        if calendar.day_kind(d) == DayKind::Closed {
            d = d.succ_opt().context("date overflow")?;
            continue;
        }
        match run_replay(&prod_lake, &cfg, &out_root, d, Cash(equity), &calendar).await {
            Ok(r) => {
                ok_days += 1;
                tracing::info!(date = %r.date, events = r.events, bars = r.bars,
                    auction_snaps = r.auction_snaps, mainland_prints = r.mainland_prints,
                    scored_names = r.scored_names, fills = r.fills,
                    cash_day = (r.scored_names == 0), "replay day complete");
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
    // --out per experiment when that distinction matters.
    let out_lake = Lake::new(&out_root);
    match pnl::daily_pnl(&out_lake) {
        Ok(series) => {
            let x: Vec<f64> = series.iter().map(|(_, v)| *v).collect();
            let total: f64 = x.iter().sum();
            let sr = stats::sharpe(&x);
            let summary = serde_json::json!({
                "days_replayed_this_run": ok_days,
                "days_failed_this_run": failed_days,
                "sandbox_root": out_root.display().to_string(),
                "sandbox_days_with_fills": x.len(),
                "sandbox_total_pnl_hkd": total,
                "sharpe_daily": sr,
                "sharpe_annualized": sr.map(|s| s * 252f64.sqrt()),
                "note": "point hkq-validate `report` at a strategy.toml whose ops.lake_root is the sandbox for bootstrap CIs and the DSR promotion gate",
            });
            println!("{}", serde_json::to_string_pretty(&summary)?);

            // §4 honesty: a backtest is a TRIAL. Record it in the PRODUCTION
            // registry so the DSR's N can only grow — never quietly shrink.
            if let Some(sr) = sr {
                let vcfg = load_validate(&cfg_path)?;
                let reg = TrialsRegistry::open(vcfg.registry_path(&cfg.ops.lake_root));
                let mut m = BTreeMap::new();
                m.insert("sr".to_string(), sr);
                m.insert("days".to_string(), x.len() as f64);
                m.insert("total_pnl_hkd".to_string(), total);
                match reg.append("backtest", &sha1_hex_of_file(&cfg_path)?, &m,
                                 &format!("replay {from}..={to} → {}", out_root.display())) {
                    Ok(t) => tracing::info!(seq = t.seq, "trial recorded — DSR's N grows honestly"),
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

## Honest gaps and hand-off to Milestone 8

Six items, each now a named fact. First, the mirror is a *current-state counterfactual*: it consumes today's alpha map, regime gate, and AH-β artifacts (only the ICIR weights are date-bounded, via this milestone's PreMarket patch), so a replayed 2025 day answers "what would today's machine have done" — as-of state snapshotting per date is the missing half of a statistically pure walk-forward, and it belongs to the promotion protocol in hkq-validate, where the trials registry and the `_state` directory are the obvious landing points. Second, execution realism has a documented ceiling: trade-at-completed-bar-VWAP with adverse half-spread plus κ-impact, full fills, no queue dynamics, no POS-band or VCM windows — the last two because nothing persists quote or VCM events yet (the same data-milestone facts M5 named for A50 and VHSI), and partial-fill stress remains the scripted-`Routed` test pattern from M6 rather than a shipped venue. Third, κ itself is now *calibratable* — the sandbox fills carry model prices while the shadow tier's fills carry real ones, and their divergence per participation bucket is exactly the regression that turns κ from prior into estimate — but that calibration job is validate scope, deliberately not smuggled in here. Fourth, the benchmark family for Hansen SPA (always-cash, HSI open→close, sector-ETF Stage-1) still needs ingested benchmark series; the mirror can now *generate* the always-cash arm trivially, but the other two remain data acquisitions. Fifth, the trials wiring is one-way by design: the backtest bin appends `sr` trials, and the `report` job deflates against them, but nothing yet *enforces* the DSR > 0.95 gate as a promotion action — governance stays with the operator, per the M5 precedent that scalar promotions are config edits, not automation. Sixth, `hkq-recon` remains the only blueprint binary not built, blocked on the independent official-close source — the M1 gap, still logged loudly every night. The system now has facts, opinions, decisions, a clock, a memory, a reflex, a hand — and a mirror that replays any recorded day through the identical code path at memory speed, into a sandbox the production lake never feels. What it lacks is judgment about what the mirror shows: the as-of snapshotting, κ calibration, and benchmark family that turn replays into the report's full §4 promotion verdict — and every one of those now has its input data being written daily.
