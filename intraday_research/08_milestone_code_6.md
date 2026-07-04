 

All seven documents are attached in full, so I worked directly from them — no fetching needed. M5's hand-off is unambiguous about what comes next: "the re-sequencing debt comes due: M6 is the signed Tiger venue behind `hkq-exec::Venue` plus partial-fill accounting in the pacer (which still treats submission as sent) — and it now lands inside a system that can refuse to trade, which was the point of doing validate first." It also bundles one engine landing point with this milestone by name: `_state/ah_beta.parquet` exists but `freeze.rs` still applies the ρ=1 prior, "deliberately bundled with the next engine-touching milestone."

# Milestone 6 — The Hand That Touches the Market (signed Tiger venue + partial-fill truth)

**Why this is next.** M1–M5 built a machine that produces facts, opinions, decisions, a clock, and — as of M5 — a memory and a reflex. Both kill-switch producers are now real (operator console + CUSUM startup gate), the shadow tier's output is monitored, and the trials registry makes promotion honest. What the system still cannot do is touch the market: `hkq-exec::Venue` has exactly one implementation, and `PaperVenue`'s fiction — every child fills fully, instantly, at its limit — is load-bearing in a place it must not be: the pacer counts a child as *sent* the moment it is emitted. A real venue rejects, expires, cancels, and partially fills; if the pacer keeps treating submission as execution, every one of those events silently destroys shares from the parent's budget and the participation ledger lies. So M6 delivers two inseparable things: the signed Tiger order route, and the accounting honesty that makes its fills survivable — venue-confirmed fill/return events flowing back into the pacer so unfilled shares reopen the routing budget instead of vanishing. This is deliberately the *only* milestone that is almost pure vendor-verification surface, and it lands inside a system that already halts itself: the CUSUM gate, the operator console, and the §4 shadow discipline were all built first precisely so that the first real order enters a guarded machine.

**In scope:** the `Venue` trait's honest lifecycle shape (`Submission::{Immediate,Routed}`, `poll_updates`, `cancel_children` — with defaults that keep `PaperVenue` semantically byte-equivalent to M4/M5); partial-fill accounting in the `Pacer` (slots retire on *venue-confirmed* fills, returned shares re-pace, `ChildId` tracking); the `TigerVenue` (signed order placement through hkq-data's existing RSA-SHA1 gateway, status polling with incremental-fill reconstruction from cumulative averages, cancel-as-request with poll-confirmed terminals); the crate-owned `[exec.tiger]` config table with M1-style alias tables for every VERIFY surface; the exec actor wired for lifecycle updates and venue-side cancels on halt/VCM; the marked engine landing point activated (PreMarket joins `_state/ah_beta.parquet`, freeze consumes β̂ᵢ in place of the ρ=1 prior); and `hkq-live` venue selection as an explicit CLI decision, defaulting to paper. **Deferred:** backtest replay (`SimClock`/`SimExec` — M7, and `Submission::Immediate` is exactly its seam), quote-stream spread estimation (still dropped on the floor deliberately), VHSI ingestion and the A50 dataset (unchanged typed degradation), Hansen SPA (still needs benchmark series that aren't data yet).

Engineering decisions beyond the blueprint sketch, briefly. M4 promised the venue would plug in "without touching the actor" while *also* demanding partial-fill accounting in the pacer — those are contradictory, and the honest resolution is the one shipped here: the trait grows two defaulted methods and a two-variant `Submission`, the actor gains a poll arm that routes lifecycle events into the pacer, and `Fill`/`ParentOrder`/`ExecCmd`/`spawn_exec` stay byte-identical so hkq-engine and its literal test constructions are untouched (the type contract is load-bearing; `ChildOrder` gains its `id` because it never crosses the crate boundary). Slot retirement moves from emission-time to fill-confirmation-time — the venue's word is truth, the pacer is a rate governor and the `Book` remains the only ledger; returns reopen the budget under the *same* volume allowance, so a cancelled remainder re-paces without violating the participation cap. There is exactly one RSA-SHA1 signing implementation in this codebase and it stays in hkq-data: a three-line `call_signed` append exposes it, and the dependency arrow exec→data joins the existing live-binary→{data,exec} arrows without a cycle. Everything vendor-shaped — method names, the id field, payload aliases, status vocabularies — is config in a crate-owned `[exec.tiger]` table per the `[sources]`/`[validate]` precedent, because the exact wire shape is what must be verified per account, and config is where VERIFY lives. Cancels never mutate the venue book: we request, the poll confirms, and only a confirmed terminal returns shares — synthesizing terminals from our own intent is how double-sends happen. Incremental fill prices are reconstructed exactly from cumulative (qty, avg-px) pairs; the wire limit price is emitted via `Px::as_f64` with the documented exactness argument (tick-valid HK prices ≤4 dp round-trip digit-for-digit through f64's shortest decimal representation, and nothing on the accounting side ever reads it back). VCM pause now also cancels resting venue children for the name — a cooling-off with live resting orders would be a fill you can't refuse. The AH-β landing is a one-line join at PreMarket plus a defensive read in freeze (fixtures without the column keep the prior), with the direction note made explicit: validate's fit regresses HK r_ON on the A-print, so the implementable form is β̂ᵢ·r^A − priced, identical to M4 at β=1. Venue selection is `--venue paper|tiger` on the command line, never a config-file surprise.

```text
hkq/
├── Cargo.toml                        (unchanged — M6 adds no workspace surface)
└── crates/
    ├── hkq-data/src/tiger.rs         (append: TigerClient::call_signed)
    ├── hkq-exec/
    │   ├── Cargo.toml                (updated: hkq-data, serde, serde_json, toml)
    │   └── src/{lib,model,cfg,pacing,venue,tiger,actor}.rs
    ├── hkq-engine/
    │   └── src/{cols,premarket,freeze}.rs   (surgical patches: AH-β landing)
    └── hkq-live/src/main.rs          (updated: explicit venue selection)
```

## Workspace

`Cargo.toml` is byte-identical to M5: no new members, no new workspace dependencies, no new polars features. That is itself a milestone fact — the entire deliverable lives behind seams that already exist.

## Surgical patch to a frozen crate

```rust
// (append inside crates/hkq-data/src/tiger.rs `impl TigerClient`)
    /// Signed-gateway pass-through for sibling crates (M6: hkq-exec's order
    /// route). The RSA-SHA1 canonicalization has exactly ONE implementation —
    /// this one — and order placement reuses it rather than duplicating it.
    /// Same envelope, same retry/rate substrate, same VERIFY surface.
    pub async fn call_signed(
        &self, method: &str, biz: serde_json::Value,
    ) -> Result<serde_json::Value, DataError> {
        self.inner.call(method, biz).await
    }
```

## `hkq-exec`

```toml
# crates/hkq-exec/Cargo.toml
[package]
name = "hkq-exec"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-data = { path = "../hkq-data" }
hkq-risk = { path = "../hkq-risk" }
tokio.workspace = true
async-trait.workspace = true
serde.workspace = true
serde_json.workspace = true
toml.workspace = true
thiserror.workspace = true
tracing.workspace = true
chrono.workspace = true
```

```rust
// crates/hkq-exec/src/lib.rs
#![forbid(unsafe_code)]
//! Execution seam (report §3.6, blueprint dataflow): parent orders in, paced
//! lot-multiple children out, fills back. As of M6 the seam carries the FULL
//! order lifecycle: a venue may fill a child immediately (paper tier), or
//! accept it for routing and report incremental fills / terminal states through
//! `poll_updates` (the signed Tiger route). The pacer's budget is reconciled
//! against venue-CONFIRMED events — submission is no longer treated as sent.
//!
//! Halt semantics (unchanged, deliberate asymmetry): a `Halted` risk state
//! cancels resting BUY parents — now both pacer-side and venue-side — while
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
pub mod tiger;
pub mod venue;

pub use actor::spawn_exec;
pub use cfg::{load_exec, ExecCfg, OrderAliases, TigerExecCfg};
pub use model::{
    ChildId, ChildOrder, ExecCmd, ExecError, Fill, Pacing, ParentOrder, Side, TerminalState,
    VenueUpdate,
};
pub use pacing::Pacer;
pub use tiger::TigerVenue;
pub use venue::{CancelScope, PaperVenue, Submission, Venue};
```

```rust
// crates/hkq-exec/src/model.rs
use hkq_core::ids::StockCode;
use hkq_core::money::Px;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side { Buy, Sell }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pacing {
    /// Children released against realized bar volume: cum_routed ≤ cap · cum_vol.
    Paced,
    /// Whole quantity submitted at once (protective stops, kill path, CAS flush).
    Immediate,
}

/// Pacer-assigned child identity. Internal to this crate's lifecycle accounting:
/// `Fill` (the engine-facing type) deliberately does NOT carry it — the Book
/// accounts by (code, side, shares) and the engine stays byte-identical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChildId(pub u64);

/// A parent instruction. `shares % lot == 0` is enforced at construction —
/// exec mirrors `LotQty`'s invariant without owning cash-target semantics
/// (exits are born from accumulated fills, not from cash).
#[derive(Debug, Clone)]
pub struct ParentOrder {
    pub code: StockCode,
    pub side: Side,
    pub shares: u64,
    pub lot: u32,
    pub limit: Px,
}

impl ParentOrder {
    pub fn new(code: StockCode, side: Side, shares: u64, lot: u32, limit: Px) -> Option<Self> {
        (lot > 0 && shares > 0 && shares % lot as u64 == 0)
            .then_some(Self { code, side, shares, lot, limit })
    }
}

#[derive(Debug, Clone)]
pub struct ChildOrder {
    pub id: ChildId,
    pub code: StockCode,
    pub side: Side,
    pub shares: u64,
    pub lot: u32,
    pub limit: Px,
}

#[derive(Debug, Clone)]
pub struct Fill {
    pub code: StockCode,
    pub side: Side,
    pub shares: u64,
    pub lot: u32,
    pub px: Px,
    pub ts_ms: i64,
}

/// Terminal disposition of a routed child, as CONFIRMED by the venue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalState { Filled, Cancelled, Rejected, Expired }

/// Lifecycle events a routed venue reports back through `Venue::poll_updates`.
#[derive(Debug, Clone)]
pub enum VenueUpdate {
    /// Incremental execution: `fill.shares` is the INCREMENT, priced exactly
    /// (reconstructed from cumulative qty × average price at the venue).
    Filled { id: ChildId, fill: Fill },
    /// The child reached a terminal state; `unfilled` shares never executed
    /// and must reopen the pacer's routing budget.
    Terminal { id: ChildId, state: TerminalState, unfilled: u64 },
}

#[derive(Debug)]
pub enum ExecCmd {
    Place { order: ParentOrder, pacing: Pacing },
    /// Engine forwards every 1m bar: pacing fuel + paper mark.
    OnBar { code: StockCode, ts_ms: i64, close: f64, volume: f64 },
    CancelSymbol(StockCode),
    CancelAll,
    /// VCM cooling-off (§1): pause/resume child release for one name.
    Pause { code: StockCode, on: bool },
    /// Submit every remainder immediately (CAS window / kill path).
    Flush,
}

#[derive(Debug, Error)]
pub enum ExecError {
    #[error("order rejected: {0}")]
    Rejected(&'static str),
    #[error(transparent)]
    Data(#[from] hkq_data::error::DataError),
    #[error("venue payload: {0}")]
    Payload(String),
    #[error("config: {0}")]
    Config(String),
}
```

```rust
// crates/hkq-exec/src/cfg.rs
//! `[exec]` table of strategy.toml — crate-owned config per the hkq-data
//! `[sources]` and hkq-validate `[validate]` precedents. Everything that must
//! be VERIFIED against a live Tiger account — method names, the id field,
//! payload field names, status vocabularies — is DATA here, never a hardcoded
//! assumption (the M1 discipline, extended to the order route).
use crate::model::ExecError;
use serde::Deserialize;
use std::path::Path;

fn d_place() -> String { "place_order".into() }
fn d_cancel() -> String { "cancel_order".into() }
fn d_status() -> String { "order_status".into() }
fn d_id_field() -> String { "id".into() }
fn d_filled() -> Vec<String> { vec!["FILLED".into()] }
fn d_cancelled() -> Vec<String> { vec!["CANCELLED".into(), "CANCELED".into()] }
fn d_rejected() -> Vec<String> { vec!["REJECTED".into(), "INVALID".into()] }
fn d_expired() -> Vec<String> { vec!["EXPIRED".into(), "LAPSED".into()] }

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ExecCfg {
    #[serde(default)]
    pub tiger: Option<TigerExecCfg>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TigerExecCfg {
    /// Trading account id for order routing. VERIFY.
    pub account: String,
    #[serde(default = "d_place")]
    pub place_order_method: String,     // VERIFY against your account API version
    #[serde(default = "d_cancel")]
    pub cancel_order_method: String,    // VERIFY
    #[serde(default = "d_status")]
    pub order_status_method: String,    // VERIFY
    /// biz field carrying the venue order id in status/cancel calls.
    #[serde(default = "d_id_field")]
    pub id_field: String,
    /// Payload field aliases (M1 AuctionAliases pattern): config-extendable
    /// because the order payload's field names are exactly what must be
    /// verified per account.
    #[serde(default)]
    pub aliases: Option<OrderAliases>,
    /// Status vocabularies. Anything not listed is LIVE (order still working).
    #[serde(default = "d_filled")]
    pub status_filled: Vec<String>,
    #[serde(default = "d_cancelled")]
    pub status_cancelled: Vec<String>,
    #[serde(default = "d_rejected")]
    pub status_rejected: Vec<String>,
    #[serde(default = "d_expired")]
    pub status_expired: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderAliases {
    pub order_id: Vec<String>,
    pub status: Vec<String>,
    pub filled: Vec<String>,
    pub avg_px: Vec<String>,
    pub ts: Vec<String>,
}

impl Default for OrderAliases {
    fn default() -> Self {
        Self {
            order_id: vec!["id".into(), "order_id".into(), "orderId".into()],
            status: vec!["status".into(), "order_status".into(), "state".into()],
            filled: vec!["filled".into(), "filled_quantity".into(), "filledQuantity".into()],
            avg_px: vec!["avg_fill_price".into(), "avg_price".into(), "avgFillPrice".into()],
            ts: vec!["update_time".into(), "timestamp".into(), "modify_time".into()],
        }
    }
}

#[derive(Debug, Deserialize)]
struct ExecFile {
    exec: Option<ExecCfg>,
}

/// Absent `[exec]` table ⇒ paper-only config remains valid (tiger: None).
pub fn load_exec(path: impl AsRef<Path>) -> Result<ExecCfg, ExecError> {
    let raw = std::fs::read_to_string(path.as_ref())
        .map_err(|e| ExecError::Config(format!("{}: {e}", path.as_ref().display())))?;
    let f: ExecFile = toml::from_str(&raw)
        .map_err(|e| ExecError::Config(format!("{}: {e}", path.as_ref().display())))?;
    Ok(f.exec.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_table_means_paper_only() {
        let f: ExecFile = toml::from_str("[ops]\nx = 1\n").unwrap();
        assert!(f.exec.unwrap_or_default().tiger.is_none());
    }

    #[test]
    fn tiger_table_defaults_fill_in() {
        let f: ExecFile = toml::from_str("[exec.tiger]\naccount = \"U123\"\n").unwrap();
        let t = f.exec.unwrap().tiger.unwrap();
        assert_eq!(t.account, "U123");
        assert_eq!(t.place_order_method, "place_order");
        assert_eq!(t.id_field, "id");
        assert!(t.status_cancelled.iter().any(|s| s == "CANCELED"));
    }
}
```

```rust
// crates/hkq-exec/src/pacing.rs
//! Pure participation pacing (§3.6) with M6 partial-fill truth: cumulative
//! (ROUTED − RETURNED) ≤ cap · cumulative realized volume, floored to whole
//! lots — and a slot retires only when the venue has CONFIRMED the parent's
//! full size. M4 treated submission as sent; a live venue rejects, expires,
//! cancels, and partially fills, so returned shares must REOPEN the budget.
//! No clocks, no channels — the actor feeds bars/fills/terminals in, children
//! come out. The pacer is a rate governor; the Book is the ledger.
use crate::model::{ChildId, ChildOrder, Pacing, ParentOrder, Side};
use hkq_core::ids::StockCode;
use std::collections::HashMap;

#[derive(Debug)]
struct Slot {
    key: u64,
    p: ParentOrder,
    pacing: Pacing,
    /// Cumulative shares emitted as children (may exceed p.shares across re-routes).
    routed: u64,
    /// Cumulative shares venue-confirmed as NOT executed (cancel/reject/expiry).
    returned: u64,
    /// Cumulative shares venue-confirmed as executed.
    filled: u64,
    cum_vol: f64,
    paused: bool,
}

impl Slot {
    /// Shares still eligible for routing — returns reopen this budget.
    fn outstanding(&self) -> u64 {
        self.p.shares.saturating_sub(self.routed - self.returned)
    }
    fn done(&self) -> bool {
        self.filled >= self.p.shares
    }
}

#[derive(Debug)]
pub struct Pacer {
    cap: f64,
    slots: Vec<Slot>,
    child_slot: HashMap<ChildId, u64>,
    next_child: u64,
    next_slot: u64,
}

impl Pacer {
    pub fn new(participation_cap: f64) -> Self {
        Self {
            cap: participation_cap.max(0.0),
            slots: Vec::new(),
            child_slot: HashMap::new(),
            next_child: 0,
            next_slot: 0,
        }
    }

    fn mint(&mut self, p: &ParentOrder, slot_key: u64, shares: u64) -> ChildOrder {
        let id = ChildId(self.next_child);
        self.next_child += 1;
        self.child_slot.insert(id, slot_key);
        ChildOrder { id, code: p.code, side: p.side, shares, lot: p.lot, limit: p.limit }
    }

    /// Register a parent. Immediate pacing emits the full child at once; EVERY
    /// placement now owns a slot, so a rejected stop/kill child re-emits on the
    /// next bar instead of silently vanishing (M4 retained nothing for
    /// Immediate — a venue rejection would have orphaned the shares).
    pub fn place(&mut self, p: ParentOrder, pacing: Pacing) -> Vec<ChildOrder> {
        let key = self.next_slot;
        self.next_slot += 1;
        let mut slot = Slot {
            key, p, pacing, routed: 0, returned: 0, filled: 0, cum_vol: 0.0, paused: false,
        };
        let out = match pacing {
            Pacing::Immediate => {
                let shares = slot.p.shares;
                slot.routed = shares;
                let parent = slot.p.clone();
                vec![self.mint(&parent, key, shares)]
            }
            Pacing::Paced => vec![],
        };
        self.slots.push(slot);
        out
    }

    /// Advance pacing on one bar of `code`. Paced: allowed = ⌊cap·cum_vol⌋,
    /// next child = lot-floored (allowed − (routed − returned)), capped at the
    /// parent. Immediate: any outstanding (i.e. returned) shares re-emit at
    /// once — urgency is the point of Immediate. Paused slots do neither and
    /// accrue no volume (VCM cooling-off, M4 semantics preserved).
    pub fn on_bar(&mut self, code: StockCode, volume: f64) -> Vec<ChildOrder> {
        let cap = self.cap;
        let mut mints: Vec<(u64, ParentOrder, u64)> = Vec::new();
        for s in self.slots.iter_mut() {
            if s.p.code != code || s.done() || s.paused {
                continue;
            }
            match s.pacing {
                Pacing::Immediate => {
                    let inc = s.outstanding();
                    if inc > 0 {
                        s.routed += inc;
                        mints.push((s.key, s.p.clone(), inc));
                    }
                }
                Pacing::Paced => {
                    if volume.is_finite() && volume > 0.0 {
                        s.cum_vol += volume;
                    }
                    let allowed = (cap * s.cum_vol).floor() as u64;
                    let target = allowed.min(s.p.shares);
                    let net = s.routed - s.returned;
                    let inc = target.saturating_sub(net) / s.p.lot as u64 * s.p.lot as u64;
                    if inc > 0 {
                        s.routed += inc;
                        mints.push((s.key, s.p.clone(), inc));
                    }
                }
            }
        }
        let out = mints.into_iter().map(|(k, p, sh)| self.mint(&p, k, sh)).collect();
        self.retire_done();
        out
    }

    /// Venue-confirmed execution of `shares` for child `id`. Returns true iff
    /// the parent slot is now fully filled (and retired). Fills beyond the
    /// routed budget are clamped LOUDLY — the Book remains the ledger.
    pub fn on_fill(&mut self, id: ChildId, shares: u64) -> bool {
        let Some(key) = self.child_slot.get(&id).copied() else {
            tracing::debug!(?id, shares, "pacer: fill for untracked child (post-flush/cancelled parent)");
            return false;
        };
        let Some(s) = self.slots.iter_mut().find(|s| s.key == key) else { return false };
        let max_fill = (s.routed - s.returned).min(s.p.shares);
        let f = (s.filled + shares).min(max_fill);
        if f < s.filled + shares {
            tracing::error!(code = %s.p.code, claimed = shares,
                "pacer: venue fill exceeds routed budget — clamped (Book is the ledger)");
        }
        s.filled = f;
        let done = s.done();
        if done {
            self.retire_done();
        }
        done
    }

    /// Venue-confirmed NON-execution: `shares` of child `id` came back
    /// (cancel/reject/expiry, or a partial fill's terminal remainder). The
    /// routing budget reopens; subsequent bars re-pace the remainder. Terminal
    /// events also retire the child's tracking entry.
    pub fn on_returned(&mut self, id: ChildId, shares: u64) {
        let Some(key) = self.child_slot.remove(&id) else {
            if shares > 0 {
                tracing::warn!(?id, shares,
                    "pacer: returned shares for untracked child — dropped (post-flush; Book is the ledger)");
            }
            return;
        };
        let Some(s) = self.slots.iter_mut().find(|s| s.key == key) else { return };
        let max_ret = s.routed.saturating_sub(s.filled + s.returned);
        let r = shares.min(max_ret);
        if r < shares {
            tracing::error!(code = %s.p.code, claimed = shares, accepted = r,
                "pacer: return exceeds in-flight — clamped");
        }
        s.returned += r;
    }

    /// Everything still eligible, submitted now (CAS flush / kill path). Slots
    /// and child tracking are cleared: post-flush lifecycle events are logged
    /// and dropped — end-of-day truth lives in the Book. In-flight children
    /// (routed, unconfirmed) are NOT re-emitted: they are resting at the venue,
    /// and duplicating them would double-sell.
    pub fn flush(&mut self) -> Vec<ChildOrder> {
        let mints: Vec<(u64, ParentOrder, u64)> = self.slots.iter()
            .filter(|s| !s.done() && s.outstanding() > 0)
            .map(|s| (s.key, s.p.clone(), s.outstanding()))
            .collect();
        let out = mints.into_iter().map(|(k, p, sh)| self.mint(&p, k, sh)).collect();
        self.slots.clear();
        self.child_slot.clear();
        out
    }

    pub fn pause(&mut self, code: StockCode, on: bool) {
        for s in self.slots.iter_mut() {
            if s.p.code == code {
                s.paused = on;
            }
        }
    }

    pub fn cancel_symbol(&mut self, code: StockCode) -> usize {
        self.remove_where(|s| s.p.code == code)
    }

    pub fn cancel_all(&mut self) -> usize {
        self.remove_where(|_| true)
    }

    pub fn cancel_buys(&mut self) -> usize {
        self.remove_where(|s| s.p.side == Side::Buy)
    }

    /// Live parent slots: routing budget open OR venue confirmations pending.
    pub fn resting(&self) -> usize {
        self.slots.len()
    }

    fn retire_done(&mut self) {
        let dead: Vec<u64> = self.slots.iter().filter(|s| s.done()).map(|s| s.key).collect();
        if dead.is_empty() {
            return;
        }
        self.slots.retain(|s| !s.done());
        self.child_slot.retain(|_, k| !dead.contains(k));
    }

    fn remove_where(&mut self, f: impl Fn(&Slot) -> bool) -> usize {
        let dead: Vec<u64> = self.slots.iter().filter(|s| f(s)).map(|s| s.key).collect();
        self.slots.retain(|s| !f(s));
        self.child_slot.retain(|_, k| !dead.contains(k));
        dead.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hkq_core::money::Px;

    fn parent(shares: u64) -> ParentOrder {
        ParentOrder::new(StockCode(700), Side::Buy, shares, 500,
                         Px::from_f64_quote(10.0).unwrap()).unwrap()
    }

    #[test]
    fn lot_invariant_at_construction() {
        assert!(ParentOrder::new(StockCode(1), Side::Buy, 999, 500,
                                 Px::from_f64_quote(1.0).unwrap()).is_none());
        assert!(parent(10_000).shares == 10_000);
    }

    #[test]
    fn paced_release_and_fill_confirmed_retirement() {
        let mut p = Pacer::new(0.02);
        assert!(p.place(parent(10_000), Pacing::Paced).is_empty());
        // 100k volume ⇒ allowed 2000 ⇒ 4 lots.
        let c1 = p.on_bar(StockCode(700), 100_000.0).remove(0);
        assert_eq!(c1.shares, 2_000);
        // +40k ⇒ allowed 2800 ⇒ inc 800 → lot-floor 500.
        let c2 = p.on_bar(StockCode(700), 40_000.0).remove(0);
        assert_eq!(c2.shares, 500);
        // Huge volume ⇒ remainder only.
        let c3 = p.on_bar(StockCode(700), 10_000_000.0).remove(0);
        assert_eq!(c3.shares, 7_500);
        // M6 semantics: fully ROUTED is not fully DONE — the venue must confirm.
        assert_eq!(p.resting(), 1);
        assert!(p.on_bar(StockCode(700), 1e9).is_empty()); // budget exhausted
        for c in [&c1, &c2, &c3] {
            p.on_fill(c.id, c.shares);
        }
        assert_eq!(p.resting(), 0); // venue-confirmed ⇒ retired
        assert!(p.on_bar(StockCode(700), 1e9).is_empty());
    }

    #[test]
    fn returns_reopen_the_budget_and_repace() {
        let mut p = Pacer::new(0.02);
        p.place(parent(10_000), Pacing::Paced);
        let c0 = p.on_bar(StockCode(700), 100_000.0).remove(0); // 2000
        p.on_fill(c0.id, 800);
        p.on_returned(c0.id, 1_200); // venue cancelled the remainder
        // Same cumulative allowance (2000) now re-admits the returned shares:
        // net routed = 800 ⇒ inc 1200 → lot-floored 1000.
        let c1 = p.on_bar(StockCode(700), 0.0).remove(0);
        assert_eq!(c1.shares, 1_000);
        // Immediate placements re-emit returns urgently, no pacing.
        let mut q = Pacer::new(0.02);
        let c = q.place(parent(1_000), Pacing::Immediate).remove(0);
        assert_eq!(c.shares, 1_000);
        q.on_returned(c.id, 1_000); // venue rejected outright
        let r = q.on_bar(StockCode(700), 1.0).remove(0);
        assert_eq!(r.shares, 1_000);
    }

    #[test]
    fn flush_emits_outstanding_and_drops_tracking() {
        let mut p = Pacer::new(0.02);
        p.place(parent(10_000), Pacing::Paced);
        let c = p.on_bar(StockCode(700), 100_000.0).remove(0);
        p.on_fill(c.id, 2_000);
        let f = p.flush();
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].shares, 8_000);
        assert_eq!(p.resting(), 0);
        // Post-flush lifecycle events are dropped loudly, never panic.
        p.on_returned(f[0].id, 8_000);
        assert_eq!(p.resting(), 0);
    }

    #[test]
    fn pause_cancel_and_clamp_semantics() {
        let mut p = Pacer::new(0.02);
        p.place(parent(10_000), Pacing::Paced);
        p.pause(StockCode(700), true);
        assert!(p.on_bar(StockCode(700), 1e6).is_empty()); // VCM: no emission, no accrual
        p.pause(StockCode(700), false);
        let c = p.on_bar(StockCode(700), 100_000.0).remove(0);
        assert_eq!(c.shares, 2_000);
        // Venue nonsense (fill > routed) clamps loudly instead of corrupting.
        p.on_fill(c.id, 5_000);
        assert_eq!(p.resting(), 1); // filled clamped to 2000 < 10000
        // Halt semantics: buys die, sells live.
        let mut sell = parent(500);
        sell.side = Side::Sell;
        p.place(sell, Pacing::Paced);
        assert_eq!(p.cancel_buys(), 1);
        assert_eq!(p.resting(), 1);
        assert_eq!(p.cancel_all(), 1);
    }
}
```

```rust
// crates/hkq-exec/src/venue.rs
//! The routing seam, M6 shape. Two execution styles behind ONE trait:
//! `Submission::Immediate` — the child is fully filled at submission (paper
//! tier now; the backtest SimExec in M7 rides the same arm); and
//! `Submission::Routed` — the venue accepted the child and reports lifecycle
//! through `poll_updates` (the signed Tiger route). The defaulted methods keep
//! `PaperVenue` semantically identical to M4/M5: full fill at limit, no
//! lifecycle, nothing to cancel.
use crate::model::{ChildOrder, ExecError, Fill, VenueUpdate};
use async_trait::async_trait;
use hkq_core::ids::StockCode;

#[derive(Debug)]
pub enum Submission {
    /// Filled in full at submission (deterministic shadow tier).
    Immediate(Fill),
    /// Accepted for routing; lifecycle arrives via `poll_updates`.
    Routed,
}

#[derive(Debug, Clone, Copy)]
pub enum CancelScope {
    All,
    Code(StockCode),
    Buys,
}

#[async_trait]
pub trait Venue: Send + Sync {
    /// Route one child. Must NOT block awaiting fills — routed venues ack and
    /// report through `poll_updates`.
    async fn submit(&self, child: &ChildOrder, ts_ms: i64) -> Result<Submission, ExecError>;

    /// Reconcile routed children against the venue. Immediate venues have no
    /// lifecycle: the default returns nothing.
    async fn poll_updates(&self) -> Result<Vec<VenueUpdate>, ExecError> {
        Ok(vec![])
    }

    /// Best-effort cancel of resting venue children in `scope`. Returns the
    /// number of cancel REQUESTS issued — confirmation arrives via
    /// `poll_updates`, never synthesized from our own intent.
    async fn cancel_children(&self, scope: CancelScope) -> Result<usize, ExecError> {
        let _ = scope;
        Ok(0)
    }
}

/// Deterministic full fill at the child's limit — the shadow/paper tier.
pub struct PaperVenue;

#[async_trait]
impl Venue for PaperVenue {
    async fn submit(&self, child: &ChildOrder, ts_ms: i64) -> Result<Submission, ExecError> {
        Ok(Submission::Immediate(Fill {
            code: child.code,
            side: child.side,
            shares: child.shares,
            lot: child.lot,
            px: child.limit,
            ts_ms,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChildId, Side};
    use hkq_core::{ids::StockCode, money::Px};

    #[tokio::test]
    async fn paper_fills_at_limit_and_has_no_lifecycle() {
        let c = ChildOrder {
            id: ChildId(1), code: StockCode(700), side: Side::Buy, shares: 500, lot: 500,
            limit: Px::from_f64_quote(10.0).unwrap(),
        };
        match PaperVenue.submit(&c, 123).await.unwrap() {
            Submission::Immediate(f) => {
                assert_eq!(f.shares, 500);
                assert_eq!(f.px, c.limit);
                assert_eq!(f.ts_ms, 123);
            }
            Submission::Routed => panic!("paper is the immediate tier"),
        }
        assert!(PaperVenue.poll_updates().await.unwrap().is_empty());
        assert_eq!(PaperVenue.cancel_children(CancelScope::All).await.unwrap(), 0);
    }
}
```

```rust
// crates/hkq-exec/src/tiger.rs
//! The signed Tiger order route — the first Venue that touches the real market.
//! Transport is hkq-data's signed gateway (`TigerClient::call_signed` — ONE
//! RSA-SHA1 implementation, reused); everything vendor-shaped is `[exec.tiger]`
//! config: method names, id field, payload aliases, status vocabularies.
//!
//! Lifecycle model: `submit` places a DAY limit order and stores the venue's
//! order id; `poll_updates` reconciles every open child against the status
//! endpoint, reconstructing INCREMENTAL fills exactly from cumulative
//! (filled qty, average price) pairs; `cancel_children` issues cancel REQUESTS
//! and mutates nothing — the venue's word (a polled terminal state) is the only
//! thing that returns shares to the pacer. Wire price note: tick-valid HK
//! prices (≤ 4 dp) round-trip digit-for-digit through f64's shortest decimal
//! representation, so the JSON number reprints the Decimal exactly; nothing on
//! the accounting side ever reads it back.
use crate::cfg::{OrderAliases, TigerExecCfg};
use crate::model::{ChildId, ChildOrder, ExecError, Fill, Side, TerminalState, VenueUpdate};
use crate::venue::{CancelScope, Submission, Venue};
use async_trait::async_trait;
use hkq_core::ids::StockCode;
use hkq_core::money::Px;
use hkq_data::tiger::TigerClient;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub(crate) struct OpenChild {
    pub venue_ref: String,
    pub code: StockCode,
    pub side: Side,
    pub lot: u32,
    pub shares: u64,
    pub cum_filled: u64,
    pub cum_notional: f64,
    pub limit: Px,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Lifecycle {
    Live,
    Terminal(TerminalState),
}

#[derive(Debug, Clone)]
pub(crate) struct StatusSnap {
    pub state_raw: String,
    pub cum_filled: u64,
    pub avg_px: Option<f64>,
    pub ts_ms: Option<i64>,
}

/// Pure venue-side order book: insert on ack, reconcile on status snapshots.
/// Fully unit-testable without HTTP — the transport wrapper below is thin.
#[derive(Debug, Default)]
pub(crate) struct VenueBook {
    pub open: HashMap<ChildId, OpenChild>,
}

impl VenueBook {
    /// Apply one status snapshot. Emits the incremental fill (if any) and the
    /// terminal event (if the venue says so), removing terminal children.
    pub fn apply_status(
        &mut self, id: ChildId, snap: &StatusSnap, life: Lifecycle, now_ms: i64,
    ) -> Vec<VenueUpdate> {
        let Some(oc) = self.open.get_mut(&id) else { return vec![] };
        let mut out = Vec::new();
        let cum = snap.cum_filled.min(oc.shares);
        if cum > oc.cum_filled {
            let inc = cum - oc.cum_filled;
            // Incremental price from cumulative averages:
            // (avg·cum − prev_notional) / inc — exact when avg is reported.
            let px_f = match snap.avg_px.filter(|p| p.is_finite() && *p > 0.0) {
                Some(avg) => {
                    let p = (avg * cum as f64 - oc.cum_notional) / inc as f64;
                    if p.is_finite() && p > 0.0 { p } else { oc.limit.as_f64() }
                }
                None => oc.limit.as_f64(),
            };
            oc.cum_notional += px_f * inc as f64;
            oc.cum_filled = cum;
            let px = Px::from_f64_quote(px_f).unwrap_or(oc.limit);
            out.push(VenueUpdate::Filled {
                id,
                fill: Fill {
                    code: oc.code, side: oc.side, shares: inc, lot: oc.lot, px,
                    ts_ms: snap.ts_ms.unwrap_or(now_ms),
                },
            });
        }
        if let Lifecycle::Terminal(state) = life {
            let unfilled = oc.shares - oc.cum_filled;
            out.push(VenueUpdate::Terminal { id, state, unfilled });
            self.open.remove(&id);
        }
        out
    }
}

pub(crate) fn classify(cfg: &TigerExecCfg, raw: &str) -> Lifecycle {
    let hit = |set: &[String]| set.iter().any(|s| s.eq_ignore_ascii_case(raw.trim()));
    if hit(&cfg.status_filled) {
        Lifecycle::Terminal(TerminalState::Filled)
    } else if hit(&cfg.status_cancelled) {
        Lifecycle::Terminal(TerminalState::Cancelled)
    } else if hit(&cfg.status_rejected) {
        Lifecycle::Terminal(TerminalState::Rejected)
    } else if hit(&cfg.status_expired) {
        Lifecycle::Terminal(TerminalState::Expired)
    } else {
        Lifecycle::Live
    }
}

/// Normalize `data` into the per-order object across payload shapes
/// (object, {items:[…]}, or array — the M1 quote_items convention).
fn normalize_item(data: &Value) -> Option<&Value> {
    match data {
        Value::Object(o) => match o.get("items").and_then(Value::as_array) {
            Some(items) => items.first(),
            None => Some(data),
        },
        Value::Array(a) => a.first(),
        _ => None,
    }
}

pub(crate) fn parse_order_ref(data: &Value, aliases: &OrderAliases) -> Option<String> {
    let item = normalize_item(data)?;
    for k in &aliases.order_id {
        match item.get(k) {
            Some(Value::Number(n)) => return Some(n.to_string()),
            Some(Value::String(s)) if !s.is_empty() => return Some(s.clone()),
            _ => {}
        }
    }
    None
}

pub(crate) fn parse_status_snap(data: &Value, aliases: &OrderAliases) -> Option<StatusSnap> {
    let item = normalize_item(data)?;
    let state_raw = aliases.status.iter()
        .find_map(|k| item.get(k).and_then(Value::as_str))
        .map(str::to_string)?;
    let cum_filled = aliases.filled.iter().find_map(|k| {
        item.get(k).and_then(|v| v.as_u64().or_else(|| v.as_f64().map(|f| f.max(0.0) as u64)))
    })?;
    let avg_px = aliases.avg_px.iter().find_map(|k| item.get(k).and_then(Value::as_f64));
    let ts_ms = aliases.ts.iter().find_map(|k| item.get(k).and_then(Value::as_i64));
    Some(StatusSnap { state_raw, cum_filled, avg_px, ts_ms })
}

fn id_value(venue_ref: &str) -> Value {
    venue_ref.parse::<i64>().map(Value::from)
        .unwrap_or_else(|_| Value::String(venue_ref.to_string()))
}

pub struct TigerVenue {
    client: TigerClient,
    cfg: TigerExecCfg,
    aliases: OrderAliases,
    book: Mutex<VenueBook>,
}

impl TigerVenue {
    pub fn new(client: TigerClient, cfg: TigerExecCfg) -> Self {
        let aliases = cfg.aliases.clone().unwrap_or_default();
        Self { client, cfg, aliases, book: Mutex::new(VenueBook::default()) }
    }
}

#[async_trait]
impl Venue for TigerVenue {
    async fn submit(&self, child: &ChildOrder, ts_ms: i64) -> Result<Submission, ExecError> {
        let action = match child.side { Side::Buy => "BUY", Side::Sell => "SELL" };
        let biz = json!({
            "account": self.cfg.account,
            "symbol": child.code.to_string(),      // "00700" — 5-digit HK line
            "sec_type": "STK",
            "market": "HK",
            "currency": "HKD",
            "action": action,
            "order_type": "LMT",
            "limit_price": child.limit.as_f64(),   // exactness note in module docs
            "total_quantity": child.shares,
            "time_in_force": "DAY",
        });
        let data = self.client.call_signed(&self.cfg.place_order_method, biz).await?;
        let venue_ref = parse_order_ref(&data, &self.aliases).ok_or_else(|| {
            ExecError::Payload(format!("no order id in place response (schema drift?): {data}"))
        })?;
        tracing::info!(code = %child.code, ?action, shares = child.shares,
            venue_ref = %venue_ref, ts_ms, "tiger: child routed");
        let mut book = self.book.lock().expect("venue book poisoned");
        book.open.insert(child.id, OpenChild {
            venue_ref,
            code: child.code,
            side: child.side,
            lot: child.lot,
            shares: child.shares,
            cum_filled: 0,
            cum_notional: 0.0,
            limit: child.limit,
        });
        Ok(Submission::Routed)
    }

    async fn poll_updates(&self) -> Result<Vec<VenueUpdate>, ExecError> {
        let snapshot: Vec<(ChildId, String)> = {
            let book = self.book.lock().expect("venue book poisoned");
            book.open.iter().map(|(id, o)| (*id, o.venue_ref.clone())).collect()
        };
        if snapshot.is_empty() {
            return Ok(vec![]); // nothing routed ⇒ zero HTTP
        }
        let now = chrono::Utc::now().timestamp_millis();
        let mut out = Vec::new();
        for (id, venue_ref) in snapshot {
            let biz = json!({
                "account": self.cfg.account,
                &self.cfg.id_field: id_value(&venue_ref),
            });
            match self.client.call_signed(&self.cfg.order_status_method, biz).await {
                Ok(data) => {
                    let Some(snap) = parse_status_snap(&data, &self.aliases) else {
                        tracing::warn!(%venue_ref, "tiger: status payload unparseable (schema drift?)");
                        continue;
                    };
                    let life = classify(&self.cfg, &snap.state_raw);
                    let mut book = self.book.lock().expect("venue book poisoned");
                    out.extend(book.apply_status(id, &snap, life, now));
                }
                Err(e) => tracing::warn!(%venue_ref, error = %e, "tiger: status poll failed; next cycle"),
            }
        }
        Ok(out)
    }

    async fn cancel_children(&self, scope: CancelScope) -> Result<usize, ExecError> {
        let targets: Vec<(ChildId, String)> = {
            let book = self.book.lock().expect("venue book poisoned");
            book.open.iter()
                .filter(|(_, o)| match scope {
                    CancelScope::All => true,
                    CancelScope::Code(c) => o.code == c,
                    CancelScope::Buys => o.side == Side::Buy,
                })
                .map(|(id, o)| (*id, o.venue_ref.clone()))
                .collect()
        };
        let mut n = 0usize;
        for (_id, venue_ref) in targets {
            let biz = json!({
                "account": self.cfg.account,
                &self.cfg.id_field: id_value(&venue_ref),
            });
            match self.client.call_signed(&self.cfg.cancel_order_method, biz).await {
                // Request only — the book mutates when a poll CONFIRMS the terminal.
                Ok(_) => n += 1,
                Err(e) => tracing::warn!(%venue_ref, error = %e, "tiger: cancel request failed"),
            }
        }
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> TigerExecCfg {
        toml::from_str("account = \"U123\"").unwrap()
    }

    fn open_child(shares: u64) -> OpenChild {
        OpenChild {
            venue_ref: "42".into(),
            code: StockCode(700),
            side: Side::Buy,
            lot: 100,
            shares,
            cum_filled: 0,
            cum_notional: 0.0,
            limit: Px::from_f64_quote(10.0).unwrap(),
        }
    }

    #[test]
    fn incremental_fill_math_from_cumulative_averages() {
        let mut book = VenueBook::default();
        book.open.insert(ChildId(1), open_child(2_000));
        // Snapshot 1: cum 800 @ avg 10.00 ⇒ incremental 800 @ 10.00.
        let s1 = StatusSnap { state_raw: "WORKING".into(), cum_filled: 800,
                              avg_px: Some(10.00), ts_ms: Some(5) };
        let u = book.apply_status(ChildId(1), &s1, Lifecycle::Live, 99);
        assert_eq!(u.len(), 1);
        let VenueUpdate::Filled { fill, .. } = &u[0] else { panic!() };
        assert_eq!(fill.shares, 800);
        assert!((fill.px.as_f64() - 10.00).abs() < 1e-9);
        assert_eq!(fill.ts_ms, 5);
        // Duplicate snapshot: idempotent, no phantom fill.
        assert!(book.apply_status(ChildId(1), &s1, Lifecycle::Live, 99).is_empty());
        // Snapshot 2: cum 2000 @ avg 10.06 ⇒ inc 1200 @ (10.06·2000 − 8000)/1200 = 10.10.
        let s2 = StatusSnap { state_raw: "FILLED".into(), cum_filled: 2_000,
                              avg_px: Some(10.06), ts_ms: None };
        let u = book.apply_status(ChildId(1), &s2,
                                  Lifecycle::Terminal(TerminalState::Filled), 77);
        assert_eq!(u.len(), 2);
        let VenueUpdate::Filled { fill, .. } = &u[0] else { panic!() };
        assert_eq!(fill.shares, 1_200);
        assert!((fill.px.as_f64() - 10.10).abs() < 1e-6);
        assert_eq!(fill.ts_ms, 77); // vendor ts absent ⇒ now
        let VenueUpdate::Terminal { state, unfilled, .. } = &u[1] else { panic!() };
        assert_eq!(*state, TerminalState::Filled);
        assert_eq!(*unfilled, 0);
        assert!(book.open.is_empty()); // terminal ⇒ removed
    }

    #[test]
    fn cancelled_partial_returns_the_remainder() {
        let mut book = VenueBook::default();
        book.open.insert(ChildId(2), open_child(2_000));
        let s = StatusSnap { state_raw: "Cancelled".into(), cum_filled: 800,
                             avg_px: Some(10.0), ts_ms: Some(1) };
        let u = book.apply_status(ChildId(2), &s,
                                  classify(&cfg(), &s.state_raw), 9);
        assert_eq!(u.len(), 2); // the 800 fill AND the terminal
        let VenueUpdate::Terminal { state, unfilled, .. } = &u[1] else { panic!() };
        assert_eq!(*state, TerminalState::Cancelled);
        assert_eq!(*unfilled, 1_200);
    }

    #[test]
    fn classification_and_parsers_use_config_vocabulary() {
        let c = cfg();
        assert_eq!(classify(&c, "FILLED"), Lifecycle::Terminal(TerminalState::Filled));
        assert_eq!(classify(&c, "canceled"), Lifecycle::Terminal(TerminalState::Cancelled));
        assert_eq!(classify(&c, "REJECTED"), Lifecycle::Terminal(TerminalState::Rejected));
        assert_eq!(classify(&c, "EXPIRED"), Lifecycle::Terminal(TerminalState::Expired));
        assert_eq!(classify(&c, "PARTIALLY_FILLED"), Lifecycle::Live);

        let a = OrderAliases::default();
        let ack = serde_json::json!({ "id": 1234567 });
        assert_eq!(parse_order_ref(&ack, &a).as_deref(), Some("1234567"));
        let ack = serde_json::json!({ "items": [{ "order_id": "ABC-1" }] });
        assert_eq!(parse_order_ref(&ack, &a).as_deref(), Some("ABC-1"));

        let st = serde_json::json!({
            "status": "WORKING", "filled": 300.0, "avg_fill_price": 9.98, "update_time": 111
        });
        let s = parse_status_snap(&st, &a).unwrap();
        assert_eq!(s.cum_filled, 300);
        assert_eq!(s.ts_ms, Some(111));
        assert!((s.avg_px.unwrap() - 9.98).abs() < 1e-12);
        // Missing filled field ⇒ None (schema drift is loud upstream, never 0).
        assert!(parse_status_snap(&serde_json::json!({ "status": "X" }), &a).is_none());
    }
}
```

```rust
// crates/hkq-exec/src/actor.rs
//! The exec actor: single consumer of `ExecCmd`, single producer of `Fill`.
//! Policy lives in `Pacer`; this loop is transport + the halt rule + (M6) the
//! lifecycle pump that keeps the pacer's budget venue-truthful.
use crate::model::{ChildOrder, ExecCmd, Fill, Side, VenueUpdate};
use crate::pacing::Pacer;
use crate::venue::{CancelScope, Submission, Venue};
use hkq_risk::RiskState;
use std::time::Duration;
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;

/// Venue lifecycle poll cadence. Vendor rate limits live in hkq-data's
/// RatedClient; strategy constants live in config; this is neither — it is the
/// actor's bookkeeping heartbeat, and 1 s is far inside every constraint that
/// matters (children are minutes apart by construction; an empty venue book
/// short-circuits to zero HTTP).
const VENUE_POLL: Duration = Duration::from_millis(1_000);

pub fn spawn_exec<V: Venue + 'static>(
    venue: V,
    participation_cap: f64,
    fill_tx: mpsc::Sender<Fill>,
    mut risk: watch::Receiver<RiskState>,
) -> (mpsc::Sender<ExecCmd>, JoinHandle<()>) {
    let (tx, mut rx) = mpsc::channel::<ExecCmd>(1024);
    let handle = tokio::spawn(async move {
        let mut pacer = Pacer::new(participation_cap);
        let mut poll = tokio::time::interval(VENUE_POLL);
        poll.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tokio::select! {
                changed = risk.changed() => {
                    if changed.is_err() { break; }
                    if risk.borrow().halted() {
                        let n = pacer.cancel_buys();
                        match venue.cancel_children(CancelScope::Buys).await {
                            Ok(k) => tracing::error!(cancelled_buy_parents = n, venue_cancel_requests = k,
                                "exec: HALTED — buys cancelled (pacer + venue), sells continue"),
                            Err(e) => tracing::error!(cancelled_buy_parents = n, error = %e,
                                "exec: HALTED — venue cancel failed; poll reconciles survivors"),
                        }
                    }
                }
                _ = poll.tick() => {
                    match venue.poll_updates().await {
                        Ok(updates) => {
                            for u in updates {
                                apply_update(&mut pacer, &fill_tx, u).await;
                            }
                        }
                        Err(e) => tracing::warn!(error = %e, "exec: venue poll failed; next cycle"),
                    }
                }
                cmd = rx.recv() => {
                    let Some(cmd) = cmd else { break };
                    match cmd {
                        ExecCmd::Place { order, pacing } => {
                            if risk.borrow().halted() && order.side == Side::Buy {
                                tracing::warn!(code = %order.code, "exec: buy rejected while halted");
                                continue;
                            }
                            let ts = chrono::Utc::now().timestamp_millis();
                            for c in pacer.place(order, pacing) {
                                route(&venue, &fill_tx, &mut pacer, &c, ts).await;
                            }
                        }
                        ExecCmd::OnBar { code, ts_ms, volume, .. } => {
                            for c in pacer.on_bar(code, volume) {
                                route(&venue, &fill_tx, &mut pacer, &c, ts_ms).await;
                            }
                        }
                        ExecCmd::CancelSymbol(code) => {
                            pacer.cancel_symbol(code);
                            if let Err(e) = venue.cancel_children(CancelScope::Code(code)).await {
                                tracing::warn!(%code, error = %e, "exec: venue cancel-symbol failed");
                            }
                        }
                        ExecCmd::CancelAll => {
                            let n = pacer.cancel_all();
                            match venue.cancel_children(CancelScope::All).await {
                                Ok(k) => tracing::info!(cancelled_parents = n,
                                    venue_cancel_requests = k, "exec: cancel all resting"),
                                Err(e) => tracing::warn!(error = %e, "exec: venue cancel-all failed"),
                            }
                        }
                        ExecCmd::Pause { code, on } => {
                            tracing::warn!(%code, on, "exec: VCM pause toggle");
                            pacer.pause(code, on);
                            // Cooling-off with live resting children would be a fill
                            // we can't refuse: cancel venue-side too. Confirmed
                            // terminals return the shares; the paused slot holds
                            // them until the VCM lifts.
                            if on {
                                if let Err(e) = venue.cancel_children(CancelScope::Code(code)).await {
                                    tracing::warn!(%code, error = %e, "exec: VCM venue cancel failed");
                                }
                            }
                        }
                        ExecCmd::Flush => {
                            let ts = chrono::Utc::now().timestamp_millis();
                            for c in pacer.flush() {
                                route(&venue, &fill_tx, &mut pacer, &c, ts).await;
                            }
                        }
                    }
                }
            }
        }
        tracing::info!("exec actor finished");
    });
    (tx, handle)
}

/// Submit one child and reconcile the pacer with the venue's answer.
async fn route<V: Venue>(
    venue: &V, fill_tx: &mpsc::Sender<Fill>, pacer: &mut Pacer, child: &ChildOrder, ts_ms: i64,
) {
    match venue.submit(child, ts_ms).await {
        Ok(Submission::Immediate(fill)) => {
            pacer.on_fill(child.id, fill.shares);
            if fill.shares < child.shares {
                pacer.on_returned(child.id, child.shares - fill.shares);
            }
            if fill_tx.send(fill).await.is_err() {
                tracing::warn!("exec: fill channel closed");
            }
        }
        Ok(Submission::Routed) => {} // lifecycle arrives via poll_updates
        Err(e) => {
            tracing::error!(code = %child.code, error = %e, "exec: child rejected at submit");
            pacer.on_returned(child.id, child.shares); // budget reopens, re-paces
        }
    }
}

async fn apply_update(pacer: &mut Pacer, fill_tx: &mpsc::Sender<Fill>, u: VenueUpdate) {
    match u {
        VenueUpdate::Filled { id, fill } => {
            pacer.on_fill(id, fill.shares);
            if fill_tx.send(fill).await.is_err() {
                tracing::warn!("exec: fill channel closed");
            }
        }
        VenueUpdate::Terminal { id, state, unfilled } => {
            if unfilled > 0 {
                tracing::warn!(?id, ?state, unfilled,
                    "exec: child terminal with unfilled shares — routing budget reopened");
            }
            pacer.on_returned(id, unfilled);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChildId, Pacing, ParentOrder, TerminalState};
    use crate::venue::PaperVenue;
    use hkq_core::{ids::StockCode, money::Px};
    use hkq_risk::{HaltReason, KillSwitch};
    use std::sync::{Arc, Mutex};

    fn parent(side: Side, shares: u64) -> ParentOrder {
        ParentOrder::new(StockCode(700), side, shares, 500,
                         Px::from_f64_quote(10.0).unwrap()).unwrap()
    }

    #[tokio::test]
    async fn paper_roundtrip_and_halt_asymmetry_preserved() {
        let (ks, rx) = KillSwitch::new();
        let (fill_tx, mut fill_rx) = mpsc::channel(64);
        let (cmd_tx, _h) = spawn_exec(PaperVenue, 0.5, fill_tx, rx);

        cmd_tx.send(ExecCmd::Place { order: parent(Side::Buy, 1_000), pacing: Pacing::Paced })
            .await.unwrap();
        cmd_tx.send(ExecCmd::OnBar { code: StockCode(700), ts_ms: 1, close: 10.0, volume: 2_000.0 })
            .await.unwrap();
        let f = fill_rx.recv().await.unwrap();
        assert_eq!(f.shares, 1_000); // 0.5·2000 covers the whole parent
        assert_eq!(f.side, Side::Buy);

        ks.halt(HaltReason::Operator);
        cmd_tx.send(ExecCmd::Place { order: parent(Side::Buy, 1_000), pacing: Pacing::Immediate })
            .await.unwrap();
        cmd_tx.send(ExecCmd::Place { order: parent(Side::Sell, 1_000), pacing: Pacing::Immediate })
            .await.unwrap();
        let f = fill_rx.recv().await.unwrap(); // only the SELL fills
        assert_eq!(f.side, Side::Sell);
        assert!(tokio::time::timeout(std::time::Duration::from_millis(50), fill_rx.recv())
            .await.is_err());
    }

    // ── routed-venue fake: submit acks, poll scripts one partial + cancel ──
    #[derive(Clone)]
    struct FakeRouted {
        inner: Arc<FakeInner>,
    }
    struct FakeInner {
        submitted: Mutex<Vec<ChildOrder>>,
        stage: Mutex<u8>,
        cancels: Mutex<Vec<&'static str>>,
    }
    impl FakeRouted {
        fn new() -> Self {
            Self { inner: Arc::new(FakeInner {
                submitted: Mutex::new(vec![]), stage: Mutex::new(0), cancels: Mutex::new(vec![]),
            }) }
        }
    }
    #[async_trait::async_trait]
    impl Venue for FakeRouted {
        async fn submit(&self, child: &ChildOrder, _ts: i64) -> Result<Submission, crate::model::ExecError> {
            self.inner.submitted.lock().unwrap().push(child.clone());
            Ok(Submission::Routed)
        }
        async fn poll_updates(&self) -> Result<Vec<VenueUpdate>, crate::model::ExecError> {
            let subs = self.inner.submitted.lock().unwrap();
            let mut stage = self.inner.stage.lock().unwrap();
            match (*stage, subs.first()) {
                (0, Some(c0)) => {
                    *stage = 1;
                    Ok(vec![
                        VenueUpdate::Filled { id: c0.id, fill: Fill {
                            code: c0.code, side: c0.side, shares: 800, lot: c0.lot,
                            px: c0.limit, ts_ms: 1,
                        }},
                        VenueUpdate::Terminal {
                            id: c0.id, state: TerminalState::Cancelled,
                            unfilled: c0.shares - 800,
                        },
                    ])
                }
                _ => Ok(vec![]),
            }
        }
        async fn cancel_children(&self, scope: CancelScope) -> Result<usize, crate::model::ExecError> {
            self.inner.cancels.lock().unwrap().push(match scope {
                CancelScope::All => "all",
                CancelScope::Buys => "buys",
                CancelScope::Code(_) => "code",
            });
            Ok(0)
        }
    }

    #[tokio::test(start_paused = true)]
    async fn routed_partial_fill_reopens_budget_and_halt_cancels_at_venue() {
        let (ks, rx) = KillSwitch::new();
        let (fill_tx, mut fill_rx) = mpsc::channel(64);
        let fake = FakeRouted::new();
        let (cmd_tx, _h) = spawn_exec(fake.clone(), 0.02, fill_tx, rx);

        cmd_tx.send(ExecCmd::Place { order: parent(Side::Buy, 10_000), pacing: Pacing::Paced })
            .await.unwrap();
        cmd_tx.send(ExecCmd::OnBar { code: StockCode(700), ts_ms: 1, close: 10.0, volume: 100_000.0 })
            .await.unwrap();

        // Venue reports 800 filled, then a cancel returning 1200.
        let f = fill_rx.recv().await.unwrap();
        assert_eq!(f.shares, 800);
        assert_eq!(f.side, Side::Buy);

        // Next bar: the returned 1200 re-pace inside the SAME 2000-share
        // allowance — net routed 800 ⇒ inc 1200 → lot-floored 1000.
        tokio::time::sleep(Duration::from_millis(50)).await;
        cmd_tx.send(ExecCmd::OnBar { code: StockCode(700), ts_ms: 2, close: 10.0, volume: 0.0 })
            .await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        {
            let subs = fake.inner.submitted.lock().unwrap();
            assert_eq!(subs.len(), 2);
            assert_eq!(subs[0].shares, 2_000);
            assert_eq!(subs[1].shares, 1_000);
            assert_ne!(subs[0].id, ChildId(subs[1].id.0)); // fresh child identity
        }

        ks.halt(HaltReason::Operator);
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(fake.inner.cancels.lock().unwrap().contains(&"buys"));
    }
}
```

## `hkq-engine` — the AH-β landing point, activated

```rust
// (append inside crates/hkq-engine/src/cols.rs)
/// M6: per-name AH β column, joined at PreMarket from `_state/ah_beta.parquet`
/// (hkq-validate's quarterly fit). Null / absent ⇒ the M2 ρ = 1 prior stands.
pub const AH_BETA: &str = "ah_beta";
```

```rust
// (append inside crates/hkq-engine/src/premarket.rs, alongside the other loaders)
/// M6: per-name AH β artifact from hkq-validate's quarterly fit. Missing or
/// unreadable ⇒ EMPTY frame ⇒ left-join nulls ⇒ freeze's ρ = 1 prior stands —
/// the fit's absence is a documented cold start, never an error here.
fn ah_beta_frame(lake_root: &Path) -> DataFrame {
    let path = lake_root.join("_state").join("ah_beta.parquet");
    let empty = || df!(base::CODE => Vec::<u32>::new(), ecols::AH_BETA => Vec::<f64>::new())
        .expect("static empty frame");
    if !path.exists() {
        tracing::info!("no ah_beta state: ρ = 1 prior (cold start)");
        return empty();
    }
    let scan = LazyFrame::scan_parquet(
        path.to_string_lossy().as_ref(),
        ScanArgsParquet {
            hive_options: HiveOptions { enabled: Some(false), ..Default::default() },
            ..Default::default()
        },
    );
    match scan.and_then(|lf| lf.select([col(base::CODE), col(ecols::AH_BETA)]).collect()) {
        Ok(df) => {
            tracing::info!(names = df.height(), "ah betas loaded (ρ = 1 prior replaced per name)");
            df
        }
        Err(e) => {
            tracing::error!(error = %e, "ah_beta state unreadable; ρ = 1 prior stands");
            empty()
        }
    }
}
```

And in `NightlyState::load`, replace the single statement

```rust
        let stock = on(on(on(on(on(on(on(stock_prev, beta), s15), vmed), iev20), ivu), x6), sbz)
            .collect()?;
```

with

```rust
        let ahb = ah_beta_frame(&cfg.ops.lake_root);
        let stock = on(on(on(on(on(on(on(on(stock_prev, beta), s15), vmed), iev20), ivu), x6), sbz), ahb)
            .collect()?;
```

(`Path`, `df!`, `LazyFrame::scan_parquet`, `ScanArgsParquet`, and `HiveOptions` are already in scope in `premarket.rs`; the existing PreMarket test passes unchanged — no state file in its temp root means the empty-join/null-column path.) Append one unit test inside `premarket.rs`'s `mod tests`:

```rust
    // (append inside crates/hkq-engine/src/premarket.rs `mod tests`)
    #[test]
    fn ah_beta_frame_cold_and_warm() {
        let root = std::env::temp_dir().join(format!(
            "hkq_ahb_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(root.join("_state")).unwrap();
        assert_eq!(ah_beta_frame(&root).height(), 0); // cold: prior stands
        let mut df = df!(
            "code" => vec![700u32, 941],
            "ah_beta" => vec![1.3f64, 0.7],
            "n_obs" => vec![120u32, 90],
        ).unwrap();
        let f = std::fs::File::create(root.join("_state").join("ah_beta.parquet")).unwrap();
        ParquetWriter::new(f).finish(&mut df).unwrap();
        let out = ah_beta_frame(&root);
        assert_eq!(out.height(), 2);
        assert!(out.get_column_names().iter().any(|c| c.as_str() == ecols::AH_BETA));
        std::fs::remove_dir_all(root).ok();
    }
```

In `crates/hkq-engine/src/freeze.rs`, inside `sector_linked_frame`, insert directly after the `sig` map is built:

```rust
    // M6: per-name AH β from `_state/ah_beta.parquet` (joined at PreMarket).
    // Defensive read: fixture/legacy frames without the column keep ρ = 1 —
    // the M2 cold-start prior, now an ESTIMATED quantity when the fit exists.
    let mut betas: std::collections::HashMap<u32, f64> = std::collections::HashMap::new();
    if let Ok(bcol) = stock.column(ecols::AH_BETA) {
        if let Ok(b) = bcol.as_materialized_series().f64() {
            for i in 0..stock.height() {
                if let (Some(c), Some(v)) = (scode.get(i), b.get(i)) {
                    if v.is_finite() {
                        betas.insert(c, v);
                    }
                }
            }
        }
    }
```

and replace the single line `let delta = a_ret - priced;` with

```rust
        // β̂_i·r^A − priced: hkq-validate's fit regresses HK r_ON on the A-share
        // 09:25 print, so the estimated direction maps A→HK — scale the A print
        // into HK-overnight units, then net what the POS already priced.
        // Identical to the shipped behavior at ρ = 1 (cold start / unfitted name).
        let rho = betas.get(&c).copied().unwrap_or(1.0);
        let delta = rho * a_ret - priced;
```

Append one test inside `freeze.rs`'s `mod tests`:

```rust
    // (append inside crates/hkq-engine/src/freeze.rs `mod tests`)
    #[test]
    fn ah_beta_scales_the_mainland_delta() {
        let mut st = state();
        let mut bd = board();
        bd.absorb_mainland(StockCode(700), 0.02);
        let d = st.date;
        let members = member_auction_frame(&st, &bd, d).unwrap();
        let l1 = sector_linked_frame(&st, &bd, &members, 0.9, d).unwrap(); // ρ = 1 prior
        st.stock.with_column(Series::new(ecols::AH_BETA.into(), vec![2.0f64; 4])).unwrap();
        let l2 = sector_linked_frame(&st, &bd, &members, 0.9, d).unwrap(); // β̂ = 2
        let ah = |df: &DataFrame| {
            let sec = df.column(cols::SECTOR).unwrap().as_materialized_series()
                .u32().unwrap().clone();
            let v = df.column(cols::AH_DELTA).unwrap().as_materialized_series()
                .f64().unwrap().clone();
            (0..df.height()).find(|i| sec.get(*i) == Some(1)).and_then(|i| v.get(i)).unwrap()
        };
        // δ(β) = β·a − priced ⇒ δ(2) − δ(1) = a = 0.02 exactly.
        assert!((ah(&l2) - ah(&l1) - 0.02).abs() < 1e-9);
    }
```

## `hkq-live` — explicit venue selection

`Cargo.toml` is unchanged. The binary gains one CLI decision and reorders the Tiger client ahead of the exec spawn:

```rust
// crates/hkq-live/src/main.rs
//! Paper/shadow live binary (report §4: shadow period before capital) — and, as
//! of M6, optionally the REAL one.
//! Usage: hkq-live <strategy.toml> --equity <HKD> [--venue paper|tiger] [YYYY-MM-DD]
//!
//! Venue selection is an EXPLICIT CLI decision, default paper: routing real
//! orders must never be a config-file surprise. Both tiers run behind the same
//! guards — the operator console (`halt` + Enter) and hkq-validate's CUSUM
//! startup gate, which latches a breach BEFORE any order intent exists.
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
use hkq_exec::{cfg::load_exec, spawn_exec, PaperVenue, TigerVenue};
use hkq_risk::{HaltReason, KillSwitch};
use hkq_validate::{cfg::load_validate, cusum};
use rust_decimal::Decimal;
use std::str::FromStr;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VenueChoice { Paper, Tiger }

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
    let mut venue_choice = VenueChoice::Paper;
    let mut rest = args.peekable();
    while let Some(a) = rest.next() {
        match a.as_str() {
            "--equity" => {
                let v = rest.next().context("--equity needs a value")?;
                equity = Some(Decimal::from_str(&v).context("equity must be a decimal HKD amount")?);
            }
            "--venue" => {
                let v = rest.next().context("--venue needs a value: paper | tiger")?;
                venue_choice = match v.as_str() {
                    "paper" => VenueChoice::Paper,
                    "tiger" => VenueChoice::Tiger,
                    other => anyhow::bail!("unknown venue '{other}' (expected paper | tiger)"),
                };
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

    // Tiger client first (M6): the order route and the market data share ONE
    // signed gateway client (and one rate limiter).
    let tiger = TigerClient::new(sources.tiger.context("[sources.tiger] is required")?)?;

    // Exec actor: venue is an explicit CLI decision, PAPER by default.
    let (fill_tx, fill_rx) = tokio::sync::mpsc::channel(4096);
    let (exec_tx, _exec_handle) = match venue_choice {
        VenueChoice::Paper => {
            tracing::info!("venue: PAPER (deterministic full fills at limit)");
            spawn_exec(PaperVenue, cfg.trade.participation_cap, fill_tx, kill_rx.clone())
        }
        VenueChoice::Tiger => {
            let ecfg = load_exec(&cfg_path)?
                .tiger
                .context("[exec.tiger] table is required for --venue tiger")?;
            tracing::warn!(account = %ecfg.account,
                "venue: TIGER — REAL ORDERS WILL BE ROUTED. Operator kill: 'halt' + Enter.");
            spawn_exec(TigerVenue::new(tiger.clone(), ecfg),
                       cfg.trade.participation_cap, fill_tx, kill_rx.clone())
        }
    };

    // Market data fan-in.
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

Config addition (one crate-owned table; every VERIFY item is data, not code):

```toml
[exec.tiger]
account = "YOUR_TIGER_ACCOUNT"          # VERIFY: trading account id for order routing
# place_order_method  = "place_order"     (default — VERIFY against your API version)
# cancel_order_method = "cancel_order"    (default — VERIFY)
# order_status_method = "order_status"    (default — VERIFY)
# id_field            = "id"              (biz field carrying the venue order id)
# status_filled    = ["FILLED"]
# status_cancelled = ["CANCELLED", "CANCELED"]
# status_rejected  = ["REJECTED", "INVALID"]
# status_expired   = ["EXPIRED", "LAPSED"]
# [exec.tiger.aliases]                    (payload field names, M1 alias-table pattern)
# order_id = ["id", "order_id", "orderId"]
# status   = ["status", "order_status", "state"]
# filled   = ["filled", "filled_quantity", "filledQuantity"]
# avg_px   = ["avg_fill_price", "avg_price", "avgFillPrice"]
# ts       = ["update_time", "timestamp", "modify_time"]
```

## Honest gaps and hand-off to Milestone 7

Six items, each now a named fact. First, the backtest tier is the only tier that doesn't exist: `SimClock` replacing `instant_for`'s wall-clock arithmetic and `SimExec` behind this exact trait — and the M6 trait shape makes SimExec's job precise: it is a `Submission::Immediate` venue whose fill price applies the §4 cost-realism model (half-spread + κ-impact), or a scripted `Routed` venue when partial-fill realism is under test; the seam needs no further surgery. Second, participation truth is refined but not perfect: the pacer caps *routed-net* shares against realized volume, not *printed* participation — the telemetry to audit the difference (filled vs routed per parent) now exists, and wiring it into validate's attribution is a reporting change, not an accounting one. Third, the entire VERIFY surface for go-live is concentrated in `[exec.tiger]` and `[sources.tiger]`: method names, the id field, payload aliases, status vocabularies, and whether your account wants the order id as a number or a string (`id_field` + the numeric-or-string `id_value` handles both) — one paper-vs-tiger reconciliation day settles all of it, and the M5 shadow machinery is exactly the harness for that day. Fourth, the quote stream is still routed and dropped, so live spread estimation remains static-file; it lands when someone consumes it, not before. Fifth, rejected sell children now re-emit through the Immediate slot's return path, but a name that is *unsellable all day* (venue down, halted into the close) still ends in the M4 terminal invariant — a loud "book not flat" error demanding the operator, which is the correct failure mode for a system this size. Sixth, VHSI, the A50 dataset, Hansen SPA benchmarks, and CPCV are unchanged deferrals with unchanged owners. The system now has facts, opinions, decisions, a clock, a memory, a reflex — and a hand. What it lacks is a mirror: the replay tier that proves the hand would have done the same thing yesterday, and that is exactly two trait implementations away.
