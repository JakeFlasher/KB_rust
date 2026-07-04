*Thinking...*

> I'm reviewing the full set of documents to understand the milestone progression so far. The first three milestones established the core infrastructure—the data ingestion layer, the factor calculation engine, and the decision-making components with signal and risk management. Now I need to trace what comes next in the sequence.
> 
> Looking at the explicit roadmap in M6's closing remarks, the system has built up facts, opinions, decisions, a clock, a memory, and a hand—but it's missing a mirror. That mirror is the backtest replay tier, which means M7 is the `hkq-backtest` binary with `SimClock` and `SimExec` implementations that let the system prove its decisions would have worked the same way in the past.
> 
> The blueprint is clear on what M7 needs: `SimClock` swaps out the wall-clock arithmetic in `instant_for` to use event-time from the 1-minute parquet partitions, and `SimExec` becomes a `Submission::Immediate` venue that fills at bar VWAP adjusted by half-spread plus the κ-impact term, applies the full §1 cost stack including rounded stamp duty, respects the ±15% POS band, and simulates VCM cooling-off as unfillable windows. The engine's day loop already uses `tokio::select!` with a timer based on `instant_for`, so the seam for swapping in `SimClock` is already there—I just need to replace how that timer gets its time source.
> 
> The tricky part is that under a paused tokio runtime, `tokio::time::Instant::now()` is virtual but `chrono::Utc::now()` is still real wall-clock, so `instant_for` computing the delta to a historical date would clamp to zero and fire immediately. I need to rethink how the replay task feeds events and how the timer advances in lockstep with the event stream.
> 
> The real issue is that M4's engine hard-codes `instant_for` with wall-clock arithmetic, but for backtesting I need a clock abstraction — a `Clock` seam that the engine can use to sleep until a given time and to get current timestamps. This needs to be surgical: the engine needs to know how to sleep until a scheduled time, and the exec actor needs to timestamp commands with the simulated clock instead of real wall-clock time.
> 
> But there's a cleaner path: instead of retrofitting timers into the engine's async loop with a virtual clock, I can drive the whole backtest as a synchronous event-time simulation. The replay harness would build the schedule upfront, stream historical market events in timestamp order, and interleave schedule actions with those events by comparing timestamps — executing schedule actions directly without any timer machinery.
> 
> The catch is that `TradingDay::act` and its fields are crate-private, so an external backtest driver can't call them. The right move is to put the replay driver *inside* hkq-engine itself as a new module — that way it owns the virtual clock and can access the private methods. The blueprint actually lists backtest replay as one of hkq-engine's responsibilities, so this fits the architecture perfectly.
> 
> So the plan is: add `replay.rs` to hkq-engine with the SimClock and day runner, build `SimExec` in hkq-exec for realistic fill pricing, create the hkq-backtest binary, and register it as a workspace member. The blueprint already names hkq-backtest as a planned binary alongside hkq-nightly, hkq-live, and hkq-recon.
> 
> Now I need to flesh out the SimExec design — it should fill at bar VWAP adjusted by half-spread plus impact, charge the full cost stack with rounded stamp duty, respect the ±15% POS band, and simulate VCM cooling-off windows as unfillable periods.
> 
> Actually, wait — the M4 Book already charges statutory costs inside `apply_fill` via CostModel, so SimExec shouldn't duplicate that. What SimExec owns is the *price* realism: adjusting the fill price adversely by half-spread and κ-impact. For a buy, that's reference price times (1 + s/2 + κ·σ·√(q/ADV)); for a sell, it's reference price times (1 − s/2 − κ·σ·√(q/ADV)). The reference price comes from the child's limit or from the bar close when the actor routes on bars.
> 
> But here's the problem: SimExec should fill at bar VWAP adjusted by costs, not at the child's limit. The actor doesn't forward the bar data to the venue — it only sends the code, timestamp, close, and volume to the pacer. So how does SimExec access the current market price? I'm weighing two approaches: either SimExec maintains its own mark table that the backtest driver updates before sending OnBar to the actor (safe via channel ordering), or SimExec fills at the child's limit adjusted by costs, which is simpler but less realistic.
> 
> I'm leaning toward a shared `SimMarket` state that tracks the latest VWAP-ish reference and spread context. When SimExec submits, it reads the current market state, computes the adverse-adjusted price, and clamps it to the limit-price discipline — for a buy with limit L, the fill price is min(adjusted, L). The key insight is that Immediate submissions can be partial: if the adjusted price exceeds the limit, SimExec returns fewer shares and the pacer re-queues the remainder, letting the M6 machinery handle the rest.
> 
> But there's a problem with zero-share fills: if the adjusted price violates the limit, I'd return a Fill with 0 shares, which then propagates to the Book and creates a zero-share position that pollutes the state. I need to prevent zero-share fills from reaching the Book entirely — either by having SimExec never return them, or by filtering them out in the route handler before sending to the engine.
> 
> The cleaner approach: when a child order is unmarketable, don't emit a Fill at all. Instead, use `Submission::Routed` paired with a queued `VenueUpdate::Terminal` that reports the shares as unfilled on the next poll, which reopens the budget and lets the pacer retry next bar.
> 
> But this surfaces a bigger architectural question: the backtest needs deterministic interleaving of fills with events. If the exec actor runs as a separate tokio task, fills can arrive nondeterministically relative to subsequent bars unless the driver explicitly waits for them. I'm weighing whether to keep the actor-based design with careful synchronization points, or go fully synchronous by embedding the exec logic directly in the replay driver without spawning a separate task.
> 
> The actor's policy logic—halt cancels buys, Place routes children, OnBar triggers pacer logic, CancelSymbol/All handles venue cancels, Pause pauses and cancels, Flush routes—is about 50 lines of core behavior. Duplicating this in the replay driver risks drift and maintenance burden. The actor is really just transport plus the halt rule and VCM-cancel rule layered on top of the Pacer.
> 
> Option 2 is to run the real actor under paused tokio time with explicit synchronization: use a current-thread runtime with `start_paused(true)`, where `tokio::time::sleep` auto-advances only when idle. After the driver sends a command, a brief sleep lets the actor drain its pending work deterministically before time advances, similar to how the M6 actor tests work.
> 
> The catch is fills: they flow from fill_rx into the Book in the select loop of `run_day`. But `run_day` uses wall-clock chrono for timers, which breaks under paused tokio time—for a historical date, all deltas go negative and every timer fires instantly at startup. So I need to make the schedule timer virtual-clock-aware by introducing a Clock trait that abstracts sleep_until and current time, letting the replay driver inject a SimClock instead of wall-clock arithmetic.
> 
> The SimClock maps tokio's paused-time Instant to the backtest date/time: when the replay pump sleeps until `origin + (event_ts - origin)`, the whole system—schedule timers, event arrivals, exec actor polls—runs on one coherent virtual timeline. Tokio's auto-advance when idle becomes the event-time engine, and `run_day` works unmodified. Determinism comes from the single-threaded runtime and tokio's current_thread scheduler firing timers in registration order for equal deadlines.
> 
> The exec actor's `chrono::Utc::now()` calls for Place/Flush timestamps are cosmetic during replay—fills get bar timestamps from the pacer, and what matters for PnL is the fills_frame date column. So I don't need to patch the actor's clock; the only change needed in day.rs is replacing the `instant_for` call with a clock-based sleep that respects the virtual timeline.
> 
> I'm defining a Clock trait that returns a `tokio::time::Instant` for a given date and time, with WallClock using the current behavior and SimClock mapping to the origin. Rather than adding a parameter to `run_day` (which would force M6's main.rs to update), I'll add a clock field to TradingDay with a default WallClock and a builder method to swap it out—keeping the signature unchanged and the patch surgical.
> 
> Since the replay driver lives inside hkq-engine, it can construct TradingDay and set the clock field directly without needing a public setter. The SimClock needs to be created after the runtime starts so it can capture the virtual origin as `tokio::time::Instant::now()` paired with a chosen UTC timestamp.
> 
> Defining SimClock implementation... er fires before the bar arrives, X3 misses the last bar. At the 09:35:00 deadline, both the pump and the schedule timer are queued, but since the schedule timer was registered earlier (at 09:30), it fires first in Tokio's timer wheel, causing X3 refresh to run without the 09:34 bar. This matches live behavior though—the Tiger poller also delivers bars after the X3 refresh fires due to polling latency. To be explicit about this and avoid any ambiguity, I'll shift bar delivery to 61 seconds after open instead of 60.
> 
> For auction snapshots, I should deliver them at their recorded timestamps since those already represent arrival times from the historical data. Mainland prints don't have timestamps in the partition schema, so I'll deliver them at the standard live timing of 09:25:05 (5 seconds after the one-shot task). The replay driver needs to load whatever partitions are available—bars are required, but auction and mainland print partitions are optional depending on whether the live tier persisted them.
> 
> The fill flow is deterministic because everything runs on the same virtual-time executor: when a bar triggers, the engine routes it to the exec actor, which submits fills back through the channel, and the engine applies them all within the same virtual instant before time advances. Finally, at end-of-day the system checks that the book is flat, and any CAS flush at 16:00 gets submitted immediately with market-order marketability.
> 
> But there's a subtle issue with limit enforcement: if exits are placed at a snapped-down limit near the current mark, and the simulated fill price (adjusted by half-spread and impact) ends up below that limit, the order becomes unmarketable and fails to fill. This would cause exits to never fill, leaving the book non-flat and triggering an error every day. The resolution is that the model allows fill prices to be worse than the reference mark within the spread—the limit is protective, not a hard constraint. In live trading, a marketable sell limit at the mark actually fills at the bid (mark minus half-spread), so the engine's snapped limit and the real fill price are consistent with slippage from crossing the spread.
> 
> For SimExec, the simplification is that limit orders fill fully; the engine's job is to make the price realistic, not to model queue dynamics. The fill rule is: adjusted price based on current bar VWAP with cost adjustments for spread and impact, and the limit is treated as a marketable intent by construction—every limit is placed at the prevailing mark to cross the spread. So SimExec executes immediately at the cost-adjusted reference and ignores the limit price for execution decisions. No clamping is needed for buys, and allowing fills to gap through limits (like on stop orders) is actually honest modeling of gap risk.
> 
> VCM cooling-off windows and POS auction bands are documented as out of scope—VCM events don't exist in the replay lake and no producer ever arrived, so there's parity with live. The CAS flush at 16:00 routes residuals into continuous close via SimExec, which fills at the last bar's reference price, maintaining paper-tier equivalence.
> 
> Now I need to figure out where SimExec gets the inputs it needs—spread bps, ADV, and volatility—to compute the cost adjustment.
> 
> Defining the cost structure... The driver updates the mark before sending the bar through the channel, ensuring the current bar's VWAP is available when children submit. There's a subtle timing question though: should fills execute at the current bar's reference price (creating mild lookahead) or the next bar's (more honest to live execution)? The current approach fills at the bar that triggered the order, which is reasonable given the instant-fill fiction already established in the system.
> 
> For the replay loop, I need to verify that run_day's dependencies work under the SimClock setup. The schedule builds from config, the timer patches through the clock, and the kill switch can run independently. The key issue is the market-data channel: when the pump task finishes feeding data, it drops the sender, causing md_rx.recv() to return None. The run_day loop needs to handle this gracefully without spamming warnings on every iteration once data ends.
> 
> The real danger is that a closed mpsc receiver returns None immediately forever, which causes the select! to spin hot on that arm under paused time, starving other tasks and creating a livelock. In live operation this never happens because the mux tasks hold the sender open all day. For replay, the pump must keep the sender alive until after the schedule's final action (PostClose at 16:10), then sleep until 16:20 virtual time before dropping. By then run_day has already exited its loop, so there's no receiver waiting.
> 
> Actually, the simplest approach is to have the pump just call `pending().await` after the last event—it'll block forever and the runtime teardown will kill it when everything else completes. The timer-driven schedule will still auto-advance correctly to hit the next boundary, and the actor's polling loop will tick through idle cycles until the schedule ends, which is negligible overhead over a day's worth of ticks.
> 
> Now I'm realizing a critical issue with the backtest design: PostClose writes Scores, Fills, and Auction partitions back to the lake, but if I'm replaying a historical date using the same lake root as production, I'd overwrite the real recorded data with simulation results—corrupting the trials and attribution records. The fix is to pass a separate write-lake to TradingDay (a sandbox root or user-specified output directory) while keeping the read-lake pointed at production, so refit_alpha reads from the write-lake's limited partition set and stays conservative when observations fall below the threshold.
> 
> But there's another trap: refit_alpha also writes the alpha state map to `cfg.ops.lake_root/_state/alpha_map.json`, which in a sandbox would be the production root—overwriting live state if the panel grew large enough. In sandbox mode the panel stays tiny so it returns early, but that's fragile. The real issue is that NightlyState::load reads both from the lake AND from `cfg.ops.lake_root` for alpha and gate state files, so if I redirect cfg.lake_root to sandbox, those state files won't exist and I'd cold-start with defaults, losing historical fidelity. The honest approach is to document that replays use today's learned state as a counterfactual scenario, not a bit-exact reproduction, and keep the design simple: separate read and write lake roots, with state reads coming from the production root. I need to be honest about the replay's limitations: it uses current learned state (AlphaMap, weights, gate settings) rather than as-of-date snapshots, so it's mechanically valid for cost and impact studies but not statistically pure walk-forward testing—that would require the state snapshotting approach in validate's promotion protocol. The real issue is that `NightlyState::load` pulls the full attribution history including future-of-date rows when computing FactorWeights, creating lookahead bias. I can fix this cheaply by filtering attribution to only rows strictly before the replay date—since `NightlyState::load` already knows the date, I can patch the weight-loading logic to exclude future data and align with live behavior. Sandbox writes are isolated so the refit is safe to skip patching. The remaining unbounded scans check out — Bars1m in NightlyState and PanelBuilder are both date-bounded. Now M7 is taking shape: I need a SimExec venue in hkq-exec that models realistic execution (half-spread plus impact off bar VWAP), an EngineClock trait to swap between wall time and simulated time, and a surgical patch to day.rs to inject the clock dependency and swap the two call sites where instant_for is used.
> 
> For the patch strategy, I'm leaning toward making day.rs's private instant_for function pub(crate) so WallClock in clock.rs can call it directly — that avoids duplication and keeps the patch minimal: just one visibility keyword change plus the clock field, init, and two call-site swaps.
> 
> Now I'm mapping out the remaining modules: replay.rs as the driver that loads events, builds a TradingDay with SimClock, spawns pumps and execution, then returns a ReplayReport; premarket.rs gets a surgical date-bound patch to load_weights; lib.rs exports the new clock and replay modules; and the backtest binary takes strategy config, equity, and date arguments with optional range support by looping internally — each day gets a fresh SimClock anchor while reusing the runtime.
> 
> For the integration: Channels (md_rx, fill_rx, kill_rx) are constructed in replay; the bin loads the lake exactly like hkq-live does with equity from CLI. The exec actor and SimExec work under halt-flatten, kill_rx is still provided even though the kill switch stays unused in backtest, and CUSUM gate is skipped entirely to avoid corrupting the production state file. TradingDay.clock gets swapped to SimClock via pub(crate) field access since replay is in the same crate, and SimClock must be constructed inside the running runtime so Instant::now() uses virtual time under start_paused — the bin creates the runtime with start_paused(true) to enable auto-advance.
> 
> The blocker: start_paused requires tokio's test-util feature, which isn't in the workspace's "full" feature set. The M6 tests already use #[tokio::test(start_paused = true)], so test-util is available in test contexts, but enabling it workspace-wide for the backtest binary means adding it to the main tokio dependency — acceptable risk since the pause machinery just compiles in and hkq-live would never call it, though it should be documented to prevent accidental pausing.
> 
> The alternative is a manual event-time loop in replay.rs that doesn't use the actor runtime at all, instead calling TradingDay's handlers directly and implementing exec synchronously — this avoids test-util entirely but requires replicating the actor's policy inline.
> 
> The blueprint emphasizes "reuses this exact engine" with swaps behind traits, which points toward the paused-time approach: a trait-based SimClock that run_day consumes, preserving the full code-path parity of the real async select loop and actor channels. I'll stick with that and add test-util to the workspace tokio features with a note explaining why.
> 
> Now checking determinism under current_thread + paused time: run_day's select is biased (kill first, timer, market data, fills), which is good. But the exec actor's select isn't biased — tokio::select! without the biased keyword uses random ordering seeded from thread RNG, so poll_updates and command processing could interleave differently across runs. However, SimExec has no lifecycle events, so poll always returns empty, making the interleaving behaviorally irrelevant. Fills come deterministically from command processing. The determinism holds for SimExec despite the unbiased select, which is acceptable — I'll note it.
> 
> Also verifying the first tick of interval completes immediately with MissedTickBehavior::Delay, which is fine. And for bar completion timing: the 09:44 bar completes at 09:45:00, feeding volume into the 09:45 entry timer, which also fires at 09:45:00 — entry fires first because it's checked before the bar-completion event, so entry sizing uses the accumulated 09:44 volume as expected.
> 
> Checking live parity: bars arrive at open+60s plus the poll window (default 10s), so roughly open+60s to open+70s. I'll use open+61s to match the 1s latency I'm adding, mirroring live's poll delay semantics. Auction snapshots use their recorded arrival times directly from the live shadow, which are already within the POS window (09:00–09:22), and the freeze at 09:29:30 captures all of them. Mainland prints at 09:25:05, before the freeze. Multiple pump tasks deliver events independently, but tokio's timer ordering handles global timestamp ordering automatically.
> 
> Actually, simpler approach: merge all events into one sorted vector by timestamp, then run a single pump task that sleeps and sends in order — fully deterministic and trivial memory overhead for a day's worth of bars across 500 names. Now I'm loading the replay data: scanning the Bars1m partition (required), Auction snaps (optional), and MainlandPrints (optional), extracting columns from polars ChunkedArrays and mapping them to MarketEvent structs with proper type handling for nullable fields.
> 
> The engine's on_event handler processes these events — Auction snaps get absorbed during the matching window, bars trigger mark updates and pacing logic, and mainland prints are absorbed. The pump task intercepts bars before sending them to the market data feed, calling simexec.on_bar directly. For SimName inputs, I'm pulling from NightlyState.nums, which maps stock codes to their spread, volatility, and ADV parameters, then initializing SimExec with those values and falling back to conservative defaults from the config.
> 
> I need to parameterize the default spread in SimExec's constructor instead of hardcoding 35bps — replay will pass the configured max median spread. On fill pricing, I'm keeping the 4dp quantization without tick-snapping since the sub-tick error is negligible compared to spreads already charged, and asymmetric snapping would be worse than uniform quantization.
> 
> For the replay report, I'm avoiding a circular dependency by having the replay driver compute stats directly from the out-lake Fills partition rather than importing the pnl module from validate — or elegantly, just let the operator run the existing `hkq-validate report` command on the sandbox lake, which already handles multi-day backtest histories the same way.
> 
> Since the binary can depend on both engine and validate (hkq-live already does this), the backtest bin can reuse `pnl_from_fills_df` to compute PnL from the day's fills, keeping ReplayReport minimal with just date, event count, and flags like whether an auction or mainland session occurred.
> 
> I'm also checking whether multiple replay runs of the same date into the same output root cause issues with the Fills partition — write_partition is idempotent (atomic rename), so multiple dates accumulate correctly and validate's report works across the range.
> 
> For book flatness: if a name halts mid-day with no bars after entry, the exit program places a paced sell that can't pace (no bars), so the CAS flush at 16:00 submits the remainder as Immediate, which SimExec fills at the last known mark, leaving the book flat. If a name never printed a bar at all, SimExec falls back to the child limit order and fills, also achieving flatness. The pacer's flush logic is sound because SimExec confirms synchronously before flush arrives (channel ordering ensures the fill reaches the engine and book before the pacer retires on_fill).
> 
> On_bar sends ExecCmd::OnBar for every bar event, including those from 09:31 onward when the phase is already Confirmation, so the tracker absorbs these correctly and board.absorb_open_bar works as expected.
> 
> For the warm state setup: NightlyState::load reads from the production lake and config to get the current learned state, while TradingDay::new uses the sandbox config for stage1/stage2/trade/costs and the refit_alpha path. The config is moved into freeze and stored, with two clones of StrategyCfg happening as needed.
> 
> The hkq-backtest main flow parses command-line arguments for the strategy file, equity, and date range, then spins up a tokio runtime and iterates through each trading date (calendar-aware, skipping closed days), calling run_replay for each date and collecting reports. At the end it validates the daily PnL across all reports and prints a summary table. Replay is always SimExec by definition, so there's no venue option. The ReplayReport struct captures the date, event count, bar count, and auction snapshot count for each run.
> 
> For the cash_day detection: instead of trying to capture post-run state (which is tricky since run_day consumes the day), I can check if the Fills partition exists in the output lake after the run and count the rows. Similarly, I'll scan the Scores partition to see if any rows were written — if Scores doesn't exist, it's a cash day. The EngineClock trait becomes an Arc<dyn EngineClock> with Send+Sync bounds, and instant_for returns a tokio::time::Instant synchronously (not async), which fits the existing day.rs pattern where the timer gets reset with the returned instant.
> 
> Defining the EngineClock trait... Writing replay module... Extracting the OHLC and volume/turnover columns as float64 series, then iterating through each row to construct Bar1m events with the proper timestamp offset and latency adjustment, skipping any rows with missing code or timestamp values. Then moving on to handle auction events. Writing the replay function... Setting up the event pump that feeds market data into the simulation on a virtual clock, then running the same trading day logic against those replayed events and collecting the results.
> 
> Now I'm verifying the field access patterns and constructor signatures work correctly — the clock assignment and TradingDay initialization both check out. For the anchor time, I need to compute it inline since const NaiveTime construction isn't available, so I'll use the from_hms_opt helper with the expected call. The partition_height helper scans the lake for fills and scores on the given date, defaulting to zero if anything fails, then I'm collecting those metrics into the final replay report.
> 
> Once run_day completes, I abort the pump task to clean up the pending future. On the determinism front, since the pump task spawns before the run_day timer gets registered, any tie at 09:00:00 means the pump fires first and sends an auction snapshot while the engine is still in PreMarket — but that's harmless because the event handler absorbs it regardless of phase.
> 
> Now moving into the SimExec module, which implements the backtest venue as specified in the M6 handoff — it's a Submission::Immediate venue that applies the cost-realism model with half-spread and impact adjustments based on the latest completed bar's VWAP.
> 
> Defining the SimName struct with spread, volatility, and volume parameters... Completing the VWAP storage logic, then refactoring the submit method to acquire the lock once and compute both the reference price and adverse adjustment together to avoid holding the lock during calculations.
> 
> Now I'm sketching out the test cases: verifying that buy orders pay up and sell orders give back symmetrically around VWAP, working through the math with concrete parameters to validate the impact calculation, checking that VWAP derives correctly from turnover divided by volume with a fallback to close when turnover is NaN, and handling the edge case where an unknown stock code uses default spread with no impact adjustment.
> 
> I'm also documenting the clock abstraction—defining a trait that maps trading dates and HKT times to tokio Instants, with a live implementation that computes real-time offsets and a simulation variant that anchors an arbitrary UTC origin to the runtime's paused timeline so historical days replay in virtual time without touching any downstream timer or scheduling logic.
> 
> Now I'm patching day.rs to wire in the clock abstraction...
> 
> adding the Arc import, injecting a clock field into TradingDay, and replacing the direct instant_for calls with self.clock.instant_for throughout run_day. The free function gets renamed to wall_instant_for since it's only used by WallClock now. Then I'm looking at premarket.rs to handle the date bound in load_weights.
> 
> Updating load_weights to accept a date parameter and filter out future partitions before sorting, then passing the current date at the call site. This keeps the live behavior identical but makes replay honest about what data's available. Next I'm setting up the module structure in lib.rs with the new clock and replay modules, exporting the key types, and doing the same for hkq-exec with the sim module. Finally, adding hkq-backtest as a workspace member and configuring its dependencies—pulling in the core, data, engine, and validation crates along with tokio, chrono, and logging utilities.
> 
> Writing the backtest binary... Closed days and early dates without warmup data both get skipped gracefully with logged errors. The PnL summary computes daily returns and a Sharpe ratio across the sandbox results, and hints the operator to run the validation report with the sandbox config. The equity starting balance comes from CLI parsing like the live system. For testing, I'm considering an end-to-end replay test using the M4 fixture's premarket data, though a full realistic test would need alpha state and sector selection to avoid cash-only days—keeping the scope pragmatic for now.
> 
> For the actual test cases: I'll verify the clock math under paused runtime, fill-price calculations in sync and async modes, and a cash-day replay where the sector gate blocks all trades (confirming event loading, clock traversal, and isolation work correctly). Then I'll add an entries-happen test by writing a flat alpha map JSON into the temp prod root so the causal chain fires—setting a negative sigma_min_gate to force sector selection, crafting rising bars for confirmation, and verifying the cost hurdle (around 25–35 bps) stays below the flat 100 bps alpha.
> 
> For the big milestone test: I'm sizing participation caps at 2% of projected volume and lot floors based on equity allocation, then running a full day where entry at 09:45 triggers paced parents that fill across the 09:45–15:44 window, exit sells at 15:30, and CAS flushes at 16:00 to prove the mirror trades work end-to-end. I'll generate bars programmatically for ~330 minutes across two codes and reuse the coarse 4-bar warmup fixture, letting null sigma values degrade gracefully in candidate selection.
> 
> For the stop logic: setting bar lows equal to closes (monotone rising bars) ensures the stop never fires after entry since low will always exceed the stop price, avoiding the messy scenario where a dip below entry triggers an immediate sell. I also need vmed_0930 to have 114 bars of bod5 history from warmup, and entry_filter requires candidates with positive scores from stage2.
> 
> For the replay test: I'm ensuring at least one candidate passes by making replay-day volumes significantly exceed the warmup median for certain codes, which creates nonzero X3 scores across names and guarantees the top-ranked candidate scores positive. The sector gate is set to -1e9 to allow selection, and with 2 sectors × 2 names per sector, all four candidates get proposed; since replay bars rise for all of them, they all pass the chi filter and exceed the alpha hurdle.
> 
> For sizing and execution: I'm targeting 1 million HKD across 2 sectors and 2 names (250k per slot), which translates to roughly 2200 shares given the fixture price around 100—well under the 2% ADV cap of 120k shares. Entry starts at 09:45 with pacing capped at 2% of cumulative volume per bar; with replay bars at 1e5 volume each, that allows ~2000 shares per bar, easily filling the 2200-share target by day end. Exit happens at 15:30 with paced sells and a final flush to flatten the position. I'm also setting VWAP calculations by scaling turnover proportionally to volume and verifying fills are recorded in the sandbox while ensuring the production lake remains isolated with no fills partition for the test date.
> 
> This is an ambitious but fast end-to-end test running under paused time with roughly 1400 events. I need to set up the test with `tokio::test(start_paused = true)` and pick a weekday date like 2026-07-03 (Friday) with a warmup period from June 19–July 2. I'm also writing a flat AlphaMap with coefficient 100.0 to the production state directory, skipping CUSUM invocation in replay mode. The regime gate and auction flows are absent, so they'll default to identity or prior behavior. Attribution gets written at PostClose to the sandbox. For the freeze entry chain, I'm checking that Stage1 gates don't zero out the signal: the gap_z is null without auction data, sector sigma comes from EWMA over warmup with a minimum observation count of 2, and the composite signal uses weighted z-ranks across the 2 sectors.
> 
> Now I'm verifying the rank-z calculations: with two sectors, the inverse normal of 0.25 and 0.75 gives roughly ±0.674, so both sectors pass the gate threshold of -1e9. Stage2 scores across the 4 names sum to approximately zero with the top sector positive. For entry, I'm checking that `build_candidates` gets the necessary numbers from the static fixture, and the `r_0930_0945` tracker absorbs bars during the Confirmation phase starting at 09:30, collecting closes from bars opening 09:30 through 09:43 before Entry fires at 09:45. The Entry and MorningHold schedules trigger at the same instant, with entry() setting parents first before the phase transitions, allowing subsequent bars to trigger stops and pacing logic.
> 
> For the hourly projection, `vol_0945` accumulates bars with open less than entry price—the 14 bars delivered before 09:45 during Confirmation phase give 1.4e6 volume, projecting to 5.6e6 and capping at 112k. Sizing pulls the realized volatility from the warmup's 1-minute bars, computing LAV as the square root of 5-day RV scaled by an illiquidity ratio, which stays finite and positive. The e2e test is feasible, and I'll also write a smaller isolated test for the bar events mapping where delivery timestamp equals open plus 61 seconds. The replay module needs the Path import, and I'm double-checking compile issues across the patched files, particularly the Channels struct definition in day.rs.
> 
> Now verifying the remaining pieces: TradingDay::new returns a Result, run_day takes the calendar and Channels, KillSwitch constructs with the receiver cloned for both spawn_exec and Channels, and SimExec derives Clone with Send+Sync automatically satisfied. The workspace tokio configuration gains test-util features to support the paused-time tests in hkq-engine, and no manifest changes are needed for hkq-exec or the backtest crate since they already have the required dependencies.
> 
> The paused-time clock is a production mechanism for the backtest tier, not just test infrastructure, so it lives at the workspace level where the M6 actor tests already depend on it. The current_thread runtime is required for start_paused to work, and synchronous I/O like the PostClose parquet writes inside the timed loop won't block auto-advance since they're blocking operations within a single poll without awaits. Parquet reads happen upfront before run_day, and since the backtest never constructs HTTP clients or vendor integrations, there are no reqwest timers to interfere with the virtual clock.
> 
> For the milestone framing, this is Milestone 7 — "The Mirror" — capturing the SimClock and SimExec replay tier that M6 explicitly named as the next step. The strongest case is that M6's hand-off says we're "exactly two trait implementations away," the promotion gates need backtests to generate SR metrics for the trials registry, and cost-realism studies require SimExec for κ calibration. I'll structure the answer with a brief intro, the milestone definition with M6's quote, scope and deferral sections, then the engineering decisions covering the paused-time virtual clock via EngineClock, the test-util feature requirement, and SimExec as price-only.
> 
> Now I'm laying out the core design: the virtual clock pauses during replay, trades execute at completed-bar VWAP with adverse spread and impact, bars arrive at open+60s+1s to match live poll semantics, a single merged event stream ensures determinism, and sandbox isolation via lake root and config prevents PostClose/alpha refit from touching production. The load_weights date-bounding patch ensures byte-identical replay, the replay engine lives inside hkq-engine per the blueprint so internal seams stay private, and the validate report job runs unchanged against the sandbox root — the promotion pipeline composes automatically. The honest caveat is that this is current-state counterfactual, not point-in-time reconstruction, so as-of snapshots remain in the validate and promotion scope.
> 
> I'll include a tree diagram, update the workspace Cargo.toml with new members and tokio test-util, then surgical patches to day.rs, premarket.rs, and lib.rs, plus new files for the clock trait, replay logic with e2e tests, the sim executor with tests, and the backtest binary. The docs will be long with full source, so I'm keeping tests tight while staying comprehensive, then closing with honest gaps and the hand-off to M8.
> 
> Now I'm detailing the day.rs patch: appending the clock field to the TradingDay struct, initializing it with WallClock in the constructor, and renaming the instant_for function so the WallClock impl can delegate to it cleanly.
> 
> For premarket.rs, I'm adding an as_of date parameter to load_weights and filtering the panel to exclude attribution partitions on or after that date—this ensures we only load historical weights, which matches the live behavior since PreMarket runs before any new attribution can be written.
> 
> Adding test module... Writing test cases... I see the issue with the rounding—10.02 × 1.0025 = 10.04505, which banker's rounding quantizes to 10.0450 at 4 decimal places, but converting back to f64 gives 10.045, creating a tolerance mismatch. I'll adjust the test to use cleaner numbers: setting the vwap to exactly 10.0 with a turnover of 10,000 and a close of 9.9 avoids these rounding edge cases, giving exact values like 10.025 for the buy and 9.975 for the sell. I'm also adding a separate test for the fallback behavior when a code is unknown and has no mark data, using the default spread of 35 basis points.
> 
> Writing test code... Writing test fixtures... I need to ensure the stop loss won't trigger during the replay since prices only rise and the entry reference is set at 09:43 when all subsequent bars stay above that level. Then I'm writing a flat alpha map to the production state directory and setting up the main test that builds the fixture, runs the replay with a million-dollar equity, and verifies both that trades actually execute and that the production lake remains isolated from the backtest output.
> 
> Now I'm walking through the entry gate logic to confirm orders actually get placed — checking that the flat 100bps alpha hurdle exceeds the cost floor plus margin, which involves calculating the impact cost from the warmup volatility and position size relative to average daily volume, and it comes out to roughly 36bps total cost versus the 100bps threshold, so entries should pass.
> 
> For the 15-minute volatility calculation, I'm pulling the standard deviation from the warmup bars across all codes, which gives a small value around 0.001, and the return-to-volatility ratio χ works out to roughly 0.5%, which is sufficiently large.
> 
> After winsorizing and ranking the four candidates, I'm running an orthogonal regression with an intercept and five predictors on a four-row cross-section — the design becomes overcomplete by the later factors, but the first two predictors retain nonzero residuals that feed into the final scores, so all four candidates end up with nonzero scores, the top ones are positive, they all meet the cost threshold, and with two names per sector, all four get accepted and sized.
> 
> For position sizing, I'm dividing the equity budget by the number of slots to get 250k per name, then using inverse-LAV weights to determine how much to allocate to each accepted name, capping the position at 2% of projected volume — which translates to around 112k shares per name, well above the minimum.
> 
> The morning hold phase paces entries across bars using a 2% cap on cumulative volume per bar, so a 700-share order fills over roughly two bars, then at 15:30 the exit program places sell orders that pace out through 15:58, with cumulative volume hitting the cap by around 15:32, leaving the book flat before the 16:00 flush.
> 
> The exit limit is set by snapping down the mark price, and SimExec fills at VWAP regardless of the limit, which is fine. Stop checks work correctly through the morning and afternoon phases. At 11:58 the lunch cancel clears all resting orders — by then the buy parents are mostly filled so the pacer is empty, and even if some shares are still resting, canceling them after partial fills just means the position gets sized from whatever's in the book when the exit sells fire at 15:30, keeping everything flat.
> 
> The 09:30 open bar arrives at 09:31:01 and triggers the OnBar command in the Confirmation phase, which the board absorbs correctly. The nightly state requires the 1-minute bar scan for warmup, and the auction dataset is missing so it stays null.
> 
> For stage2 input, the missing auction columns just become nulls which is expected. The candidate builder pulls the tercile from warmup, and when vhsi_tercile is None it falls back to the global alpha bucket. Entry orders are built with share quantities that divide evenly by lot size.
> 
> The end-to-end flow holds together — there's inherent risk in details I can't actually run since this is a fictional but rigorous codebase, but it's internally consistent. I'm also adding a small test that verifies bar events deliver at the right timestamp with correct field ordering. The bar_events function is pure Polars so it runs as a unit test without needing a runtime.
> 
> For the main backtest binary, I'm setting up argument parsing similar to the live version with from/to dates and output paths, then logging daily report lines and finishing with a JSON summary containing days, fills, PnL series, total, daily Sharpe, and annualized metrics using the validation stats module.
> 
> I need to be careful about one thing: the daily PnL in the sandbox includes all dates ever replayed into that root, not just this run, so I should document that and maybe print both a run-scoped summary from the reports and a sandbox-wide summary. I'll also add a guard to ensure the output directory doesn't equal the lake root — comparing canonicalized paths with a fallback to simple string equality.
> 
> Setting up the main function with tracing initialization, argument parsing, and a paused-time Tokio runtime that blocks on the async run function. The runtime doesn't need reqwest or network I/O, just the time and I/O capabilities for file operations.
> 
> For the per-day loop, I'll iterate through the date range with calendar skipping and error handling that continues on failures, collecting results. I'll only fail the exit code if all days failed; if some days lack data, that's fine. Then I'll compute the PnL summary by calling into hkq-validate to get the daily series, calculate total PnL and Sharpe ratio, and print as JSON — or warn if there are no fills yet.
> 
> hkq-validate is a single package with both a library and binary target, so it's importable. For dependencies, I need hkq-core (for Cash), hkq-data, hkq-engine, hkq-validate, rust_decimal for parsing equity, tokio, anyhow, and chrono — but not chrono-tz since the backtest requires explicit --from dates rather than defaulting to today.
> 
> For the workspace Cargo.toml, I'll list the members and include the test-util, keeping everything else the same with a note. Now checking for correctness: SimExec cloning works fine in spawn_exec, the replay module exports are patched into hkq-engine's lib.rs alongside the clock module, TradingDay's clock field gets set post-construction with an explicit Arc cast to the trait object, and all the internal uses of Channels and RunCfg are within the same crate so they're accessible.
> 
> For SimClock, I'm deriving Copy and Clone so it can be passed by value to the pump while still using Arc for the trait-object coercion in TradingDay. The anchor time uses NaiveTime::from_hms_opt(8,30,0), EngineError::State already exists for the no-bars case, and the pump's pending future will just await indefinitely once spawned.
> 
> For bar sorting, I'm sorting by CODE then TS_MS columns, which means events get stable-sorted by millisecond and within each millisecond the code order is preserved — though the specific order (700 vs 5) doesn't matter for this use case. For the Float64Chunked type, I'm defining a local helper function that returns PolarsResult, then using a single ? operator to convert PolarsError to EngineError when calling it. The auction rows include an iep column written from a Vec<Option<f64>> that becomes a chunked array with nulls.
> 
> For the e2e test, I'm serializing AlphaMap to JSON and writing it to the premarket alpha state path, creating parent directories as needed. The test config uses small windows (ewma_min_obs 2, amihud 3, etc.) and stage1 runs lagged correlation with a 250-bar leadlag window — since the enriched data has only 10 warmup dates but the function needs at least 12, it returns zero which is fine. The warmup bars use a 4-bar-per-day fixture including 09:30, 09:35, 09:40, and 14:30, with the 09:30 5-minute bin rolling median calculated over 2 seasonal volume days, becoming non-null from day 2 onward.
> 
> Now I'm tracing through the bar absorption and refresh timing: bars arriving before 09:35 get absorbed during confirmation, so by the 09:35:00 refresh timer, X3 sees 4 bars (09:30–09:33) totaling 4e5 volume. With vmed around 1000, the calculation becomes ln(4e5/1000) ≈ 6, which is large but acceptable after winsorizing and ranking. The refresh reads the board state after bars 09:30–09:33 have been absorbed from their arrival times, confirming the 4e5 volume is captured. A bar arriving at 09:45:01 with open at 09:44 hits the MorningHold phase, so the tracker doesn't absorb it.
> 
> For the 09:44 bar: Entry action fires at 09:45:00 placing children, then the first OnBar event at 09:45:01 allows 2% of 1e5 = 2000 shares to fill. Post-close, the auction frame only writes if snapshots exist—no auction means it's skipped. The final book flatness check should exit cleanly, otherwise an EngineError flags a real bug. Entry sizing uses equity divided by sector count (2) and position count (2) for 250k per slot, with inverse-LAV weighting normalized across accepted names. Participation caps at 112k based on projected volume from bars open before 09:45, and per-name targets around 250k at price 109 work out to roughly 2290 shares.
> 
> For the specific tickers: code5 at 60.5 targets 250k, which floors to 4000 shares via the 400-lot multiple constraint. Pacer requires positive volume bars, turnover and vwap mark correctly in sim, and timezone handling uses the hk builder throughout. The architecture lands exactly what M6 outlined—SimClock and SimExec trait implementations, the replay driver in hkq-engine, and the hkq-backtest binary as the fourth milestone binary alongside hkq-nightly, hkq-live, and hkq-recon.
> 
> Deferred work includes as-of state snapshots for promotion validation, Hansen SPA benchmarks (though backtest could now generate HSI benchmarks), VCM and quote replay (events aren't persisted yet), CPCV as an ML consumer, hkq-recon's independent close source, and scripted partial-fill routing realism. The honest gaps for M8 are the as-of snapshot protocol for promotion and whether to wire trial registry appends from the backtest binary now—the registry exists and appending is straightforward, but config_hash, notes, and sandbox-wide vs run-specific SR tracking need clarity to avoid inflating the deflation metric.
> 
> I'm deciding to wire it: after each backtest summary, if there's a non-empty PnL series, I'll append a trial with kind="backtest", the SR metric, and a config hash (SHA1 of the config file), writing one JSONL line to the production registry—that's the discipline of making N honest. I need ValidateCfg to load the registry path from the production lake root, not sandbox. Multi-day statistical outputs are already handled by the report job, bar-resolution execution realism is capped by the documented VWAP+half-spread+impact model (quote and VCM data gaps remain), and the σ15m EWMA and benchmark deferrals stay unchanged. The framing for M8 is that the system has a mirror but lacks judgment about the mirror's verdicts—so M8 is either the promotion protocol (as-of snapshots, SPA benchmarks, CPCV) or hkq-recon.
> 
> I'll name the promotion and as-of-snapshot work as the natural next step, with recon's independent-close source as the standing data gap. For the intro, I'll follow the established pattern: all eight documents are attached in full, so I worked directly from them—no fetch tool needed (I only have search anyway). I'll match the milestone docs' prose-y structure with bold paragraphs and code blocks, avoid excessive bullets, and use plain text for math notation like the existing docs do.
> 
> Now I'm setting up the final files carefully. I need to append the sim module to hkq-exec's lib.rs, update the workspace Cargo.toml with the tokio test-util feature and its comment about the replay tier's virtual clock, and write out the hkq-backtest Cargo.toml with all its dependencies.
> 
> Writing the backtest binary... Now I'm loading the strategy configuration and calendar, then setting up the backtest output directory with validation to ensure it doesn't overwrite production data. I'm iterating through each day in the date range, skipping market closures, and running replay simulations for each trading day while tracking successes and failures with detailed logging of the replay results.
> 
> After the replay completes, I'm computing sandbox-wide realized PnL metrics including total returns and Sharpe ratios, then outputting a summary JSON with the replay statistics and next steps for validation.
> 
> If the Sharpe ratio calculation succeeds, I'm recording this backtest trial into a registry that tracks all historical backtests—this ensures the decision support system's sample size grows honestly. I'm storing the Sharpe ratio, number of days, total PnL, and a description of the replay range, then logging whether the registry append succeeded or failed.
> 
> Now I need to verify all the public API dependencies are available: the config loader, registry, PnL calculator, Sharpe function, and file hashing utility are all exposed. I'm also checking that the engine has access to the factor column constants it needs for the bars, and confirming the error types properly implement From traits for both DataError and PolarsError conversions.
> 
> Writing test fixtures... Writing config structures... Writing test data... Writing the data layer...
> 
> Now I'm setting up the alpha state with a flat prior to ensure the meta-label has proper initialization, then verifying the borrow checker logic works correctly with the closure capturing multiple mutable vectors. For the warmup data with code 5, the constant price means zero returns and near-zero volatility, which should freeze certain auction mechanics as expected. The daily returns calculation uses chrono's date arithmetic to handle the lookback window properly.
> 
> At the 09:44 bar boundary, I'm checking that at least one candidate passes the entry criteria — the tracker ratio for the 700-bar index should be positive since prices are rising, and with the sector-based ranking system, the top-ranked name across all four should clear both the chi threshold and alpha filter. The sizing logic pulls reference prices from the book marks updated by the bars, and the position targets scale based on liquidity-adjusted weights derived from the warmup period.
> 
> For the sell exit, the program iterates through open positions and executes paced sells at marked-down prices through the simulator. I'm asserting that fills occur (at least one buy or sell), and I need to verify that the alpha state remains isolated — the production alpha file shouldn't change since the backtest skips refitting when observations fall below the minimum threshold, while any sandbox refit writes to its own path. The refit process scans the output lake for scores and bars, but since the output lake has no Bars1m data, it gracefully skips with a warning rather than failing.
> 
> For attribution, I'm computing realized window returns from in-memory bar vectors and writing the result to the output lake, then aggregating sector metrics using the realized data joined with the stock map. The test setup looks solid on borrows — state is built first, then moved into TradingDay — and the calendar and equity parameters match the expected types. I'm using tokio's paused timer flavor to avoid deadlocks, though I should note that failure mode in a comment.
> 
> Adding unit tests for bar event delivery...
> 
> For optional partitions that don't exist, the test just verifies they return empty. The premarket test still works since the load_weights signature change is internal to NightlyState. The freeze and day tests are unaffected because TradingDay::new uses a default WallClock, and the hkq-live module doesn't care since the signature itself didn't change — just the field defaults.
> 
> I need to add Arc and clock imports to day.rs...
> 
> Now I'm reprinting the full lib.rs files for both hkq-engine and hkq-exec to show the clock and replay modules being added, plus updating the doc comment about replay. For replay.rs itself, I should verify it belongs in hkq-engine (it does — the blueprint explicitly mentions backtest replay), and I'll document that run_replay expects a paused runtime since on a live clock it would take real wall time to execute.
> 
> For the milestone narrative, M7 is "The Mirror" — SimClock and SimExec form the replay tier. The hand-off from M6 is that the trials registry has no sr-carrying trials yet, so DSR stays None and the promotion gate can't work until the mirror feeds it data. This also enables κ calibration studies and the paper-vs-tiger reconciliation harness.
> 
> I'm checking that SimName defaults work correctly (replay sets names for all nums entries), that SimExec::new receives the right config values without magic numbers, and that the hkq-signal import in tests is fine since it's already a main dependency. For the polars lit() call, the as_of parameter name matches what the caller passes. I'll reprint load_weights in full to show the filter change clearly, and maintain the first-person engineering voice throughout with a Workspace section.
> 
> Now I'm laying out the full file reprints: root Cargo.toml with the new member and tokio features, clock.rs, replay.rs, sim.rs, backtest Cargo and main, and both lib.rs files. I'll use patch blocks for day.rs and premarket.rs, and include the chunky but essential e2e test. For replay.rs imports, I need SessionTimes and TradingCalendar from hk_core::session, chrono::Utc for timezone conversion, NaiveTime for the anchor, and Path from std::path. The sandbox config's lake_root gets set to out_root as a PathBuf, Lake::new takes AsRef<Path> so I can construct it inline for TradingDay, and I'm capturing fill counts from the partition_height call after the run completes, with events already moved into the pump task.
> 
> The pump aborts first regardless of result, then I propagate any error. The exec actor task gets dropped when TradingDay is consumed at the end of run_day, which closes exec_tx and causes the actor's receiver to return None and break cleanly. The kill_rx clones flow correctly: KillSwitch holds the Arc<Sender>, the binding `let (_ks, kill_rx)` keeps it alive through the entire run_replay scope (underscore-prefixed binding still owns the value), and in run_day the kill_rx.changed() call sees the sender still alive. The engine's biased-select never halts, so everything coheres. Now I'm formatting this as a response mirroring the prior milestone docs with an opening note about attachments.

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
