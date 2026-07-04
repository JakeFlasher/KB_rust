*Thinking...*

> Looking at the project documents in order: the research specification for the Hong Kong intraday sector rotation strategy, followed by the Rust workspace architecture blueprint that outlines eight core crates and four binary targets for backtesting, live trading, nightly runs, and reconciliation.
> 
> The three completed milestones have built out the data layer (M1), the factor computation engine (M2), and the signal generation plus risk management layer (M3). The M4 hand-off notes that the execution engine is mostly orchestration at this point—its phases wire together the sector scoring, confirmation tracking, and entry filtering, with each actor watching the risk state and the PostClose phase handling attribution and score persistence.
> 
> There are four remaining gaps to address: the attribution pipeline needs to persist the live-frozen stage-2 scores that only the engine holds, VHSI ingestion is deferred, the kill switch has both producer handles but only one active producer (the operator), and there's an incomplete note about candidate costs.
> 
> M4 is clearly the engine itself—the runbook state machine, actor wiring, and the live loop. The question is whether to bundle in order execution too. The M3 document defers both `hkq-exec` and the engine state machine to M4, which is a substantial scope. The engine needs the phase enum with state transitions, assembly of the OpenContext from live feeds, and the main actor loop with tokio's select macro to drive the day's execution.
> 
> The full scope includes book state tracking, the PostClose attribution pipeline, scores persistence, and the execution layer with order slicing and participation pacing. That's enormous—previous milestones ran 1500–2500 lines each. The natural boundary is whether to include a minimal ExecutionVenue trait in the engine crate with a paper implementation for testing, deferring the full Tiger execution to M5, or to bundle both the engine and hkq-exec together in M4. Since the engine can't be tested end-to-end without some execution component, a trait-based approach with a sim implementation seems like the right call.
> 
> Looking at what M3 explicitly hands off: the phase pipeline (sector composite → confirmation → entry), the bar handler, every actor holding a RiskState watch, the PostClose attribution logic, the scores-partition dataset decision, cost_floor_bps estimation, and OpsCfg gaining universe_static_path. So M4 should be the engine crate plus the execution seam—defining the ExecutionVenue trait with Fill and OrderId types, then a SimExec implementation for backtesting.
> 
> Defining the core structures: Phase enum with transitions and boundary times, Book for positions and marks, OpenContextBuilder to assemble the frozen frames at 09:29:30 from auction and mainland data, and the main engine actor loop with tokio::select! and a kill switch. The PostClose pipeline handles attribution and scores persistence, but that requires touching hkq-data to add the Scores variant to the Dataset enum—a small change M3 deferred.
> 
> For M4, I need to add Dataset::Scores to lake.rs and also add universe_static_path to OpsCfg in hkq-core/config.rs so the engine can load the universe blueprint at PreMarket. Both are file modifications that warrant reprinting in full per the milestone convention. I need to reprint config.rs and lake.rs in full with the new additions marked. Now I'm thinking through the engine's run_day phases and realizing the nightly job needs to orchestrate the full factor computation pipeline—universe rebuild, seasonal profiles, moments, lead-lag adjacency, and weight calculation—but that orchestration logic isn't wired anywhere yet in the current setup.
> 
> I'm weighing two approaches: either have the engine compute all derived panels deterministically at PreMarket from the lake (which is fast for 500 names and keeps nightly simple, though it deviates from the report's T−1 18:00 placement), or extend hkq-nightly to persist these panels as new Datasets (which adds more schema contracts but aligns with the runbook). Option A is cleaner architecturally since panels are pure functions of the lake with no staleness risk, and the M2/M3 crates were designed exactly for this kind of in-process computation.
> 
> But actually, the engine needs score and IC history from prior PostClose writes to fit AlphaMap and compute FactorWeights, so the flow is: load ic_panel from Dataset::Attribution and scores history from Dataset::Scores at PreMarket, use those to calibrate the alpha map and weights, then compute sector residuals and per-stock features via pure functions, and finally build the lead-lag graph. This keeps M4 self-contained without introducing new persisted panel schemas beyond what M3 already decided on.
> 
> For PostClose, M4 writes three datasets: Scores (the frozen stage2 frame with candidate metadata), Attribution (one IC row per day, partitioned by date), and Fills (the book's fills for attribution and cost analysis). The engine should also persist Auction snapshots to Dataset::Auction at the end of the PosStream phase, which closes a gap from M2 where auction history was needed but unavailable.
> 
> Now the M4 scope is becoming clearer: the execution seam needs an ExecutionVenue trait or actor model to handle order placement and cancellation, plus a pure Slicer that decides child order sizes based on participation caps and realized volume, accounting for VCM pauses.
> 
> For the paper venue in shadow mode, fills should be deterministic — a marketable order fills at the limit price immediately with fees accounted in the book. The exec actor manages child slicing over time and emits fills (or routes to Tiger later). To keep this shippable, I'm defining the core models: ChildOrder, Fill, ExecCmd (Place, CancelAll, CancelSymbol, PauseSymbol for VCM, FlattenAll to exit), and ExecReport. The ParticipationPacer is pure logic that tracks realized volume per code and computes allowed cumulative shares, deciding the next child order size floored to lots — this is testable and handles VCM pauses per code. Then PaperVenue implements the venue trait.
> 
> Now I'm thinking through the exec actor loop: it consumes ExecCmd and Bar events for volume pacing, plus a tick, and sends fills. It watches RiskState; when halted, it cancels resting orders and stops placing new ones. The engine day flow is the real constraint — I need to map out what's truly required end-to-end: PreMarket loads state and builds DayState, PosStream absorbs auction snapshots into OpenContextBuilder for IEP/IEV trajectories, MainlandPrints feeds in opening returns and A50 events, ScoreFreeze at 09:29:30 builds OpenContext and freezes candidates, then Confirmation feeds 1m bars starting at 09:30.
> 
> At 09:35 there's an X3 refresh — the engine recomputes stage2 scores with updated vol_tau0 from the first five minutes of volume before the 09:45 entry filter kicks in. I can implement this by keeping the stage2 input frame, updating the vol_tau0 column with accumulated bar volume, re-running stage2_scores, and re-freezing candidate scores without re-selecting sectors. Then at 09:45 Entry builds Candidates with score, sigma_15m, lav, cost_floor_bps, realized returns, and terciles, runs the entry filter, sizes accepted candidates, arms stops, and sends Place commands to exec. During MorningHold, bars feed into book marks and StopBook checks for breaches, then flattens breached names — though I need to think through how sell orders work since SizedOrder is a buy.
> 
> The exec model needs a Side enum so I can map SizedOrder to buy children and stops/exits to sell children of held quantity. At 11:58 there's a lunch cancel for resting entry children, then the book holds through lunch. AfternoonHold from 13:00–15:30 keeps stops live, and ExitProgram from 15:30–15:45 liquidates all positions via paced sell slices. CasResidual at 16:00–16:08 handles any remaining illiquid tail — in paper mode the book should be flat by end of day. PostClose writes Fills, Scores, and Auction partitions, computes realized window returns from today's streamed 1m bars (kept in memory for attribution), appends a daily_ic_row to the Attribution dataset, updates CUSUM, and logs a summary. The backtester reuses the exact engine with SimClock and SimExec swaps.
> 
> For testability without live feeds, the engine consumes market events from a receiver and I need a Clock abstraction to drive time deterministically. Using tokio's paused time mode with a Clock trait that provides HKT timestamps and sleep_until methods lets me feed synthetic events and control time in tests — the live implementation uses chrono and tokio, while a test SimClock can step through time manually.
> 
> Rather than building a complex auto-advancing SimClock, I'll keep unit tests focused on the pure phase-entry actions (freeze_scores, run_entry, handle_bar, post_close) as synchronous methods on DayState, and test the phase-transition logic separately. For integration testing, I'll run a full day by calling these phase handlers in sequence with synthetic events, skipping real timers entirely — this matches the established pattern from M1-M3 where async loops stay thin and pure logic gets heavy coverage.
> 
> Now I'm thinking through what data the engine needs to assemble for OpenContext, starting with the nightly-derived values computed at PreMarket from the lake. When auction volume history isn't available, I fall back to the 09:30–09:35 bin median and treat nulls as neutral per the degradation policy. I'm pulling sector membership, float cap, and other static attributes from the universe file, then building the sector daily panel by aggregating enriched member returns and computing sector-level metrics like residual returns and flows. For the beta_sector feature that M2 stage2 needs, I'm computing a 250-day rolling regression of each stock's gap returns against its sector's gap returns using polars group operations and expressions.
> 
> Now I'm working through the morning auction snapshot — capturing the earliest IEP at or after 09:10, the latest by 09:20, and the final IEP/IEV to compute the gap input (log return normalized by volatility). For mainland prints, I'm calculating the after-hours delta as the sector-weighted sum of each stock's after-hours return minus its gap contribution, using a default AH beta of 1 as a prior. The raw component gets standardized later in the composite via rank inverse normal, so I'll keep it as the value-weighted mean of (a_open_ret − gap) for now.
> 
> Since we lack per-sector mainland-exposure betas and ADR data isn't ingested, I'm setting a50_beta_ret and adr_resid_agg to null with documentation notes, letting S6 degrade gracefully to just the available components (ah_delta and any others). The stage-2 stock frame then includes the core gap metrics, volatility measures, jump indicators, and sector-level aggregates like gap_z_sector computed as the float-cap-weighted mean within each sector.
> 
> For x6_spillover, I'm precomputing the within-sector leader graph propagation at PreMarket since it only depends on prior-day data — identifying top-quintile members by float cap, building the lagged correlation structure per sector, then computing each stock's spillover as a weighted sum of leader residual returns. This avoids latency at freeze time. For sb_z, I need to extract stock-level southbound flow z-scores from the rolling 60-day shifted data, which means replicating the stock_z calculation at PreMarket rather than relying on the aggregated sector flow output.
> 
> Now I'm designing the core execution and book-keeping layer — defining the order types, fill records, and execution commands that the engine will handle, plus the position book that tracks holdings with entry prices, stops, cash balance, and realized costs through a cost model that accounts for duties and fees.
> 
> Defining the execution command types...
> 
> The pacer runs per parent order, checking on each bar how many shares it's allowed to send based on its participation cap and cumulative volume since activation, then emits child orders up to that limit. The exec actor needs to receive volume ticks from the engine so it can pace correctly — I'll have the engine forward bar volume events to exec as part of the command stream, then exec advances pacing on each bar, submits children to the venue, collects fills, and forwards them back to the engine's fill channel. The venue trait is simple: submit a child order and get back a fill immediately (paper trading semantics), with the PaperVenue implementation filling fully at the limit price.
> 
> This keeps the exec actor as a thin boundary around pure pacing logic, which satisfies the blueprint's architecture. The engine's main loop uses select to handle kill signals, phase timers, market data, and fills, advancing through a sequence of market phases from pre-market through post-close with specific action windows at each boundary.
> 
> I'm defining the schedule as a list of (time, Action) pairs rather than pure phase boundaries, so the engine can target the next action time and handle both phase transitions and intra-phase events like X3 refresh at 09:35 within Confirmation. This keeps the Phase enum from the blueprint unchanged while driving the timing logic through the schedule.
> 
> Defining the action enum and schedule for a full trading day...
> 
> For the afternoon, positions flatten by 15:45, then CasResidual runs from 16:00–16:08 to route residuals (though in paper mode there are none), followed by PostClose. Half-day mode compresses the schedule entirely—either skipping the day or running a shortened version that exits at 11:45 instead of the full afternoon. I'll make schedule building a pure function that takes day kind and mode, returning the ordered action list, which makes it easy to test and verify.
> 
> Now I'm moving to the DayState and handlers module, where I'll define a TradingDay struct that holds the config, date, static universe maps, nightly frames, previous close data, and intraday bar maps.
> 
> For the handlers, I need to absorb auction snapshots by tracking IEP values at specific times—09:10 and 09:20—to compute the X2 factor. With 15-second polling, I'll keep the last snapshot before each mark. I also need handlers to absorb mainland prints and A50 prices, capturing the first and last values in the 09:00–09:29 window, then freeze the candidate frame and run stage-one scoring with sector composites.
> 
> On cash days when no sectors are selected, there are no stage-two scores, so I'll skip the attribution and IC row append with a log note. Once stage-two scoring completes on the selected sectors' member frame, I'll build a frozen DataFrame and extract per-code stage-two fields like score and sigma_15m for later use in candidate sizing.
> 
> Sigma_15m is the trailing EWMA standard deviation of 15-minute returns, which M3 needs for sizing decisions but isn't computed in M2. I'll need to calculate it at PreMarket by binning the 1-minute bars into 15-minute returns and computing the EWMA standard deviation over a trailing window like 20 days.
> 
> I'm writing a helper function that groups 1-minute bars into 15-minute closes per code and date, computes log returns within each day, then calculates the standard deviation over the trailing window. I'll use a simple trailing standard deviation over the seasonal volatility days (20 days) rather than a full EWMA to keep it straightforward, with a note documenting the choice.
> 
> For entry actions, I'll compute SizingInput for each accepted candidate using the last 1-minute close as the reference price, the static lot size, nightly liquidity available, sigma_15m, and a projected interval volatility based on the seasonal profile and surprise factor.
> 
> For projected_interval_vol, I'm taking the sum of v_bin_med across the next hour's slots from the seasonal machinery, scaled by today's surprise ratio (realized 09:30–09:45 volume divided by the median of those slots), with a fallback to realized volume times 4 if seasonal data is unavailable.
> 
> For cost_floor_bps, I need the spread in basis points — I'll add a spread_bps column to the universe_static contract (maintained nightly) and use that median spread per name, degrading to the configured max_median_spread_bps when missing.
> 
> During execution, the stop book tracks breached stops and triggers market-ish sell orders at the last close snapped down to tick; fills are applied with embedded costs and logged for persistence, then at day-end I'm building the realized returns window and attribution row with factor scores for the dataset.
> 
> For PreMarket equity initialization, I need to decide whether to add an engine config section or use a CLI argument.  Adding a field to TradeCfg would break existing test constructors across sibling crates, so I'll go with a required `--equity` CLI arg instead — keeps the config stable and lets bins own their runtime params. RegimeGate defaults to identity and optionally loads from a JSON path if provided; AlphaMap loads from a sibling state file next to the lake, and if missing the engine trades conservatively in shadow mode until calibrated. The learning loop closes at PostClose when I refit AlphaMap from accumulated Scores and realized labels, then write the updated JSON. FactorWeights derive from an IC panel scan at PreMarket, defaulting to equal weights on cold start if the attribution dataset has no partitions yet.
> 
> For sector factors, I'm considering whether to persist sector-level scores separately or compute sector ICs directly into the Attribution partition alongside stock-factor ICs — I can aggregate realized returns by sector using the sector map, then build one attribution row with both stock and sector IC columns from the stage1 frame in memory at PostClose.
> 
> The daily_ic_row function keys on CODE, so I'd rename the sector column to "code" for the call (treating sector IDs as the cross-section key) with a clear comment explaining the reuse, avoiding changes to M3. This lets FactorWeights::from_ic_panel work for both stages at PreMarket.
> 
> For the remaining gaps: VHSI tercile is None (no feed) and goes to AlphaMap global bucket as documented; ConfirmationTracker absorbs bars during the Confirmation phase; equity sizing uses book.cash from the CLI; and I'm accumulating auction snapshots into a dataset that tracks IEV history, which PreMarket then loads to compute the 20-day mean IEV per code.
> 
> The hkq-live binary parses config and optional flags, constructs the Tiger and EastMoney clients, runs the LiveMux pump with a KillSwitch and operator console (stdin watcher for halt commands), then builds and runs the Engine for the day.
> 
> Now I'm planning the response structure to match the previous milestone format—intro, scope, engineering decisions, file tree, workspace and crate configs, full source with tests, and honest gaps for M5 handoff. I'm laying out the files needed: workspace Cargo.toml updates, config.rs with universe_static_path, lake.rs with Dataset::Scores, and the hkq-exec and hkq-live crates with their modules.
> 
> Defining the file list...
> 
> I need to be disciplined about code size — this will likely exceed 2000 lines across ~19 files, which matches the scale of previous milestones. I can keep the exec layer compact by organizing it as four focused files: model, pacing, venue (trait + paper implementation), and actor. The real weight comes from premarket with its polars assembly and freeze with frame building, so I'll write those carefully. For testing, I'll include focused unit tests per module — schedule ordering, book accounting, pacer math, paper venue mechanics, auction book marks, and candidate assembly from synthetic frames. I'll also do one roundtrip integration test for premarket that writes minimal daily bars and static parquet, then verifies the build produces the expected columns. For the APIs I'm reusing from hkq_core, I need to remember: StockCode is a u32 wrapper with 5-digit display, SectorId is u16, Px requires Decimal construction with error handling, and there's a hk_tick_size function for snapping prices.
> 
> Now I'm reviewing the data layer APIs — Lake handles partition writes with schema versioning, scan operations, and quarantine logic. I'm adding a Scores dataset variant pointing to a "scores" directory. The model columns include the standard OHLC bars plus NET_BUY and A_OPEN_RET, with Bar1m and AuctionSnap structs, and MarketEvent as an enum covering Auction, Bar, Quote, and MainlandAuctionPrint variants.
> 
> For data ingestion, I'm defining provider traits — Tiger handles auction feeds and intraday bars, EastMoney provides mainland prints and A50 data. LiveMux manages the event pump with capacity-bounded channels and shutdown coordination. The factors module extends the base columns with computed fields like sector, float cap, returns, volatility measures, and various technical indicators including gap z-scores, auction imbalance metrics, and tercile rankings.
> 
> Now I'm adding more specialized columns for close-to-close returns, after-hours deltas, beta residuals, and cross-sectional transformations like normalization, winsorization, and orthogonalization. The moments submodule enriches daily panels with realized volatility and exponential moving averages. For realized measures, I'm computing intraday bins, body-to-open ratios, and implied volatility from bars. The leadlag module builds correlation graphs and transforms panel data into matrices for analysis, while the sector module aggregates weighted metrics across stock groups with position caps.
> 
> Building sector-level aggregations... Adding workspace members and updating the config structure to include a universe static path for M4, then defining the execution model with order types, fills, and command enums for the exec module.
> 
> Now I'm thinking through the Flush semantics for the kill path — when flattening immediately, the engine sends sell parents and a Flush command to submit any remainder at once without waiting for pacing. For stops, I want immediacy too since they close tail risk, so I'm adding a Pacing enum to ParentOrder to distinguish between Paced entries and Immediate stops. For child limit prices during paced execution, I'm keeping them deterministic by using the parent limit rather than refreshing from bar closes, though that's noted as a future refinement for realism.
> 
> Defining the participation pacer structure... ent.qty.cap_shares(inc) will give me the child quantity floored to the lot size, since the increment is always less than or equal to remaining shares. I'll update the pacer's sent counter at submission, and for the paper venue fills happen instantly while live venues would handle partial fills asynchronously. Now I'm sketching out the test cases: cap math with a 2% cap on 100k volume yielding 2000 shares (4 lots of 500), pause blocking orders, cancel clearing state, flush emitting the remainder, and immediate orders going out in full. Then I'm defining the venue trait with an async submit method and a paper implementation that fills child orders at their limit price completely, plus the execution actor that coordinates between the venue, pacer, command receiver, fill sender, and risk state watcher.
> 
> The actor's main loop selects on incoming commands and risk state changes—when halted it cancels all resting buys but allows sells to continue (exits must flow during a halt). I'm adding a cancel_buys method to enforce this policy: reject new buy orders while halted, accept sells, and transition to halt by canceling the buy side. Then I'll write an end-to-end test with the paper venue: place a paced buy, feed bar events, verify the fill arrives, trigger a halt and confirm the buy cancels while sells still execute. Now I'm moving into the engine layer with error types wrapping Polars, data, factor, signal, and core errors, plus a schedule module that defines the market phases from pre-market through post-close and the corresponding actions.
> 
> Defining the schedule builder... Defining the Book struct... Defining the nightly state structures... Computing beta coefficients...
> 
> Now I'm calculating sigma_15m by binning the 1-minute bars into 15-minute intervals, computing returns within each date and code, then aggregating the standard deviation. For the volume baseline, I'm pulling the median volume from the 09:30 time slot across the last 20 days as a reference point. The intraday event volume comes from scanning the auction data—taking the mean of final IEV over the trailing 20 days per stock, with nulls as a fallback if the data isn't available.
> 
> For spillover effects, I'm constructing a within-sector residual matrix of standardized returns, applying lagged correlation analysis with FDR correction to identify leader stocks by top quintile market cap, then propagating those signals back to each member. The sentiment-based z-score pulls from the flows data over a 90-day window with a 60-day rolling shift, taking the previous date's value per stock.
> 
> Finally, assembling the nightly stock frame by joining all these components together. Building the residual matrix from sector data by normalizing returns, demeaning rows, applying lagged correlation with FDR correction, then propagating lagged volatility to compute S4 per sector. Then computing S5 from sector flows with a 60-day window, collecting the nightly snapshot with renamed columns, and joining S4/S5 to produce the final frame with all required fields. For weights, I'm loading the attribution scan sorted by date, then initializing S1 and S2 factors from the IC panel with cold-start fallback to equal weights, and loading the alpha map from state or using defaults.
> 
> Now I'm decomposing the premarket function into smaller helpers that each return focused dataframes with clear documentation, being careful to use only proven Polars patterns like joins, group-by aggregations, window functions, and rolling operations. For weighted aggregations I'm using sum-over-partition divided by sum-over-partition to compute weighted means. Then defining the frozen struct to hold scored sectors, selected sector IDs, and stage2 data, with a builder function that computes member-level gap_z from log returns normalized by sigma, sector-level aggregation of those gaps and auction volumes, and a linked sector function.
> 
> For stage2 input I'm filtering to selected sectors and pulling gap_z and auction metrics from the board, then at refresh time I'm recomputing volatility baselines by filling nulls from the auction phase with intraday volumes and medians. The key insight is handling null semantics carefully: if auction volatility is missing but intraday median exists at refresh, use just the intraday piece; if both are null at freeze, leave it neutral so stage2 scoring propagates that through null-aware expressions.
> 
> Now I'm structuring the freeze pipeline: open context gets built from nightly state and sector data, then sector composites are scored and filtered to select viable sectors—if nothing qualifies it's a cash day. For candidates, I'm taking the equity budget, dividing by position count and market impact factors to get a tentative share quantity, then feeding all stage2 rows as candidate seeds and letting the entry filter enforce the top-m quota per sector. Tests validate that freeze produces the right selected sectors and scores, and candidate assembly correctly maps the fields.
> 
> The TradingDay orchestrator holds all runtime state: configuration, the lake of historical data, nightly state, morning board, confirmation tracker, frozen sectors, candidate list, order book, stop book, and channels for execution commands, fills, and market events. Event handlers dispatch on phase—ScoreFreeze triggers the freeze run, Entry processes new positions, ExitProgram starts the exit sequence, CasResidual flushes remaining cash, PostClose wraps the day. Bar processing always updates marks and appends to the daily bar history, then forwards to phase-specific logic: during Confirmation the tracker and board absorb the open bar; in holds phase the stop book checks for triggers and sends sell orders at close price; volume circuit breaker pauses execution; halts cancel the symbol and pause trading.
> 
> The main run_day loop selects on a kill signal, scheduled actions, market data, and fills. When halted it cancels all orders, flattens positions immediately, and jumps to post-close. The schedule drives the day's phases with a timer that resets after each batch of simultaneous actions. The instant_for helper converts HKT datetimes to tokio Instants using the current UTC time, guarding against past times. For testing I'll expose all handlers and write a paper_day test that drives actions and events directly without timers, verifying the full flow from freeze through candidates, entry fills, stop exits, program exit, and flat positions with partitions written.
> 
> For the test setup I'm using a minimal premarket fixture with a couple of codes across ~70 days, small EWMA and Amihud windows, and a test FactorCfg constructed directly. I'll skip the full lake-reading test and instead do a premarket smoke test plus a day-sequence test using hand-built NightlyState, writing to a temp lake only in post-close.
> 
> In post_close I'm writing the auction snapshots, fills, and scores datasets, then computing attribution by taking today's bars, calculating realized returns over the session window, deriving daily IC rows for both stocks and sectors, and stacking them together before writing the attribution partition. After that comes the alpha refit logic scanning the scores history.
> 
> For the alpha refit I'm building a panel from the scores dataset joined with realized returns, including the ivu_tercile and injecting a null vhsi_tercile column, then fitting the AlphaMap with a minimum observation threshold. The persisted scores need to carry the full stage2 frame columns including the z-normalized factors so they're available for both IC calculation and future re-analysis. I'm adding a DATE column before writing to parquet, and the FactorWeights computation at premarket will scan the attribution data to build the IC panel, handling cold starts where no prior dataset exists by defaulting to equal weights.
> 
> For the execution wiring, each bar triggers an OnBar command that places entry orders with pacing, sets immediate stop-loss sells, and handles lunch cancellation by clearing all resting buy orders. The exit program places paced sell orders at marked-down limits for open positions, then the residual flush submits any remaining orders immediately to flatten the portfolio. In the live trading setup I'm loading the config and building the market data sources, then wiring up position and bar subscriptions through the LiveMux, with a special handler spawned around 09:25 to fetch mainland auction prints and inject them as market events into the event stream.
> 
> For the kill switch I'm creating an operator stdin task that listens for a "halt" command and triggers the halt semantics. The ExecActor spawns with either PaperVenue (default) or rejects live mode if no live venue is configured yet. Then TradingDay runs the full day cycle.
> 
> For the response structure I'm organizing it as: brief intro acknowledging the docs, then the Milestone 4 section with why/scope/deferred, followed by the engineering decisions prose covering schedule-as-data folding, instantaneous action phases, single-writer book, exec halt semantics, PaperVenue with the Venue trait seam, cost_floor sizing, X3 two-phase column swap, and the ρ=1 AH prior with null a50 beta validation.
> 
> Now I'm working through the remaining technical details: the M2 gap handling with attribution rows carrying both sector and cross-section ICs, alpha refit at PostClose writing JSON state, premarket recompute from the lake for determinism, sigma_15m simplification, equity as a CLI argument to avoid breaking M3 test constructors, and universe_static_path as an Option so nightly stays unaffected. Then I'll lay out the file tree, workspace config, and full code for the core modules, followed by honest gaps and M5 hand-offs around live Tiger venue routing, SimExec backtest replay, CUSUM producer with quarterly fits, and partial-fill handling.
> 
> Writing helper functions... I need to verify that the enriched dataset includes the realized columns like rsj and jump, which come from joining realized_daily output. If bars data is missing, those columns won't exist and the select will fail — so I should add a check that bars_1m is available, otherwise raise an error saying the engine requires that dataset. Now I'm computing beta by calculating the member z-score, then the sector-weighted mean z-score, and finally beta per code.
> 
> Calculating sigma_15m... For the refresh path, I'm handling the volume median calculation by combining the intraday auction baseline (iev_bar20) with the morning bin median (vmed_0930), using fill_null(0) to handle missing values gracefully — if both are absent the result stays null, which the X3 expression guards against anyway. Then I'm computing iev_bar20 itself by scanning the auction dataset, filtering to the last 30 days, extracting the final IEV snapshot per day per code, and averaging those daily values across all codes.
> 
> For the left join onto the stock frame, missing values from cold start remain as nulls. Moving to x6, I'm building a sector-by-sector analysis where I partition the enriched data, convert each sector's panel into a matrix of z-scores (normalized returns), demean within sector, compute lagged correlations with FDR control, extract the x vector from the previous date's row, identify the top 20% of codes by float cap as leaders, and mask the spillover effect accordingly. The panel_to_matrix function fills gaps with zeros, and the output is a dataframe mapping each code to its x6 spillover value.
> 
> For sb_z, I'm computing a 60-day rolling standardized net buy flow per code, filtering to the previous date, and handling missing flows gracefully. The ivu_tercile step applies daily IVU calculations and groups them into terciles, then filters to the previous date. For s4, I'm repeating the sector-level panel approach: converting sector daily z-scores into a matrix, demeaning, computing lagged correlations, extracting the previous date's row, and propagating the correlation structure to get sector spillover. The sector z dataframe comes from collecting sector-level returns and volatilities, guarding against division by zero. Finally, s5 aggregates sector-level flows over a 60-day window using a member map derived from the sector and code mappings, filters to the previous date, and left joins handle any missing flow data by filling with nulls.
> 
> Now I'm assembling the final sector_nightly snapshot by joining the base panel with s4 and s5 on sector, selecting the relevant columns while avoiding name collisions. For weights loading, I'm checking if the Attribution table exists and has data, then attempting to build factor weights from the IC panel; if that fails or the table is empty, I fall back to equal weights as a safe default.
> 
> The factor lists are split into S_FACTORS (s1 through s6) and X_FACTORS (x1, x2, x3, x5, x6, x7, skipping x4 which serves as a gate), and daily_ic_row generates the corresponding IC columns. For the alpha state path, I'm using the lake root directory to construct a _state subdirectory for storing the alpha map JSON without needing to expose the root through the Lake API.
> 
> Building the member auction frame by reading stock data columns, iterating through rows to look up auction info from the board, then computing gap_z as the log return normalized by sigma and vs as the log ratio of intraday to 20-bar average. The sector aggregation weights these values by market cap and selects just the sector and computed metrics for the stage1 join.
> 
> When the POS feed is empty, gap values are null, which propagates through the weighted aggregation and degrades stage2 — this is expected behavior. For sector-linked calculations, if gap data is missing I'll default the gap component to zero with a doc comment explaining the fallback, then compute delta as the difference between actual open return and the gap component. After aggregating by sector, I'm left-joining onto the full sector list from sector_nightly to preserve all sectors, filling missing values with nulls for neutral handling downstream.
> 
> Building the stage2 input by filtering stock data to only selected sectors using a boolean mask constructed in Rust rather than relying on the is_in feature.
> 
> Adding morning columns per code through row iteration and lookups — gap_z, vs_auct_i, intraday entry prices, and volatility measures — then joining gap_z_sector from the sector_auction frame. The nightly frame already carries all the other required fields like beta_sector, sigma_on, and various risk metrics. Once all columns are in place, I'm validating against the documented stage2_scores input schema.
> 
> For X3 refresh, I'm replacing the volatility columns on the saved input frame and re-running stage2_scores to update the frozen results, since candidates are rebuilt at entry time anyway when they read the refreshed frame.
> 
> Iterating through rows to construct candidates... Defining order execution logic... Writing day.rs utilities... Writing the refit pipeline... Subscribing to the A50 index stream from the mainland client. The dataframe rows contain a stock code as a u32 column and an opening return as an f64 column.
> 
> For the stdin operator console, I'm spawning a blocking thread that reads lines in a loop and halts the engine via the kill switch when needed. The live venue only supports paper trading mode; anything else errors out.
> 
> Args parsing is minimal — just the config path followed by required `--equity` and optional `--date` flags, with equity parsed as a Decimal. The engine constructor bundles all its dependencies into a struct: config, run settings, lake, calendar, channels, and the execution/fill/market data/kill receivers.
> 
> I need to verify the exec actor's fill channel and engine's market data channel won't deadlock — the engine forwards OnBar events to exec on a bounded channel of 1024, exec submits orders and sends fills back on another bounded channel, and the engine drains fills in the same select loop, so that's safe.
> 
> For testing, I'm planning concrete compile-safe tests: one for exec pacing that places a parent order of 10,000 shares with 500-share lots and 0.02 cap, then sends OnBar with 100,000 volume to trigger 2,000-share children repeatedly, testing pause and flush behavior. Another test spawns the exec actor with PaperVenue, places a buy order, sends OnBar, expects a 2,000-share fill at limit, then halts via kill switch.
> 
> Now I'm working through the schedule test — verifying monotone non-decreasing timestamps with an Enter action at 09:00, then X3Refresh at 09:35 and LunchCancel at 11:58, with the compressed schedule ending by 12:10 and lacking AfternoonHold. The book test buys 1,000 shares at 10, calculating cash impact as 10,000 principal plus 10 in stamp duty plus 1.10 in fees, leaving 989,988.90 cash. Selling at the same price nets 9,977.80 after costs, for a round-trip cost of 22.20 that matches the M3 config. The morning test snaps prices at 09:05, 09:09:50, 09:15, and 09:21, setting the 09:20 snapshot to 102 from the 09:15 snap since that's the last one before the 09:20 boundary.
> 
> For the freeze test, I'm hand-building a NightlyState-like setup with a stock dataframe containing 4 codes across 2 sectors plus all required columns, then running the freeze logic to select candidates and compute scores. The premarket test writes DailyBars for 2 codes over 80 days with small factor windows (ewma_min_obs 2, amihud 3, rv 2, seasonal 2, ivu 4), then partitions 1-minute bars for the last 5 days with snapshots at 09:30, 09:35, 09:40, and 09:45 to feed into 15-minute bins.
> 
> I'm storing the universe as a plain parquet file and loading it via LazyFrame::scan_parquet, then validating that NightlyState loads correctly with the expected columns and row counts. For the day handler test, I'll skip a full-day simulation since freeze and premarket tests already cover most paths, but I'll add a lightweight test that constructs a TradingDay with minimal synthetic state, injects a position into the book with armed stops, feeds a bar that triggers below the stop level, and verifies the ExecCmd::Place sell command fires—this validates the exit correctness path. I'll make TradingDay fields pub(crate) and add a test helper to construct it inline.
> 
> Now I'm defining the error type as an enum wrapping Core, Data, Factor, Signal, Polars, Io, and serde_json variants, plus Contract for static strings and State for dynamic errors. For hkq-exec dependencies, I need hkq-core, hkq-risk, tokio, async-trait, thiserror, and tracing—no rust_decimal since Px comes from core and there's no decimal math needed. For hkq-engine, I'm pulling in hkq-core, hkq-data, hkq-factors, hkq-signal, hkq-risk, hkq-exec, polars, tokio, chrono, chrono-tz, serde, serde_json, thiserror, tracing, and rust_decimal_macros for tests. For hkq-live, the dependencies are hkq-core, hkq-data, hkq-risk, hkq-exec, hkq-engine, tokio, tokio-stream, futures, anyhow, chrono, and chrono-tz.
> 
> Defining the exec pacing module... Continuing the actor implementation with the message loop that handles risk state changes and incoming commands, dispatching orders through the pacer and submitting them to the venue with timestamps. Now moving into the book structure that tracks positions, cash, and fill history.
> 
> For the book's apply_fill method, I'm calculating the notional value and associated costs like stamp duty and fees, then updating the cash and position based on whether it's a buy or sell, and logging each fill to the history. The equity calculation sums up the current cash plus the marked-to-market value of all positions.
> 
> For the fills dataframe, I'm casting shares to u32 for most cases but keeping u64 as the underlying type for safety since Polars supports it. Now I'm writing the day.rs assembly with the act method that routes different actions—entering phases like ScoreFreeze or Entry, handling refreshes and lunch cancellations—and the on_event handler that processes market events like auctions depending on the current phase.
> 
> In on_bar, I'm marking prices, pushing bars to the daily history, and executing callbacks; during holds I check if stops are breached and if so, trigger immediate sells while tracking which stocks are already exiting to avoid double-selling. At ExitProgram start I disarm all stops to prevent re-triggering after exit sells are placed. The freeze method calls the freeze runner with current state and board data. I'm running into a borrow checker issue with the select! macro — holding multiple mutable references to self while polling different futures won't work. The cleanest solution is to extract the channel receivers into a separate Channels struct that gets passed into run_day, keeping only the sender in self so I can use it inside the handlers without conflicts. This way the receivers move out of self and live locally in the loop, avoiding the simultaneous borrow problem.
> 
> The select! arms handle the kill watch signal (checking for halts and draining fills with a timeout), the scheduled timer (resetting it via pin and advancing through the schedule until it's exhausted), market events, and fills. Once the schedule runs out or a halt triggers an early break, post_close executes as the final schedule entry and the loop exits.
> 
> For the freeze.rs side, I'm extracting columns from the stock DataFrame — code, sector, float cap, previous close, sigma, and the bar20 volatility metric — then building member vectors with Options. The weighted sector aggregation needs the date repeated as a string column alongside sector and float cap values, and for stage2 input I'm iterating through filtered stock rows to construct the morning feature series.
> 
> The key is that extra columns like the 9:30 volume median persist through the polars pipeline since with_columns preserves them, so the stage2_input frame stored in Frozen carries everything needed for refresh. On refresh, I rebuild the volatility and median columns by pulling the raw iev and volume data from the board, joining them back, then re-running stage2_scores to get the updated frame.
> 
> For candidate selection, I read code, sector, score, and tercile from the frozen stage2 frame, then map in the risk metrics. The tentative cash allocation divides equity by the product of selected candidates and names per sector, giving me share quantities based on previous close, and finally I apply the cost floor check using spread basis points, correlation-adjusted sigma, quantity, and average daily volume.
> 
> I'm also finalizing the StockNums map creation in premarket by iterating the stock frame rows after collection and building the map with sector, lot, connectivity, float cap, previous close, and the optional volatility and liquidity fields. The statics parquet schema is locked to code, sector, float cap, board lot, connectivity flag, and nullable spread basis points — I'll document this contract in the premarket docstring and config comments.
> 
> I think the design is complete now, though the response will be quite long. I'm keeping the polars patterns to what's proven in the attached code — the dataframe macro, Series construction, lazy evaluation, joins with explicit join types, grouping and aggregation, rolling windows, exponential weighting helpers, and the standard filtering and column operations.
> 
> For reading the statics parquet, I'm using the scan pattern with hive options disabled. When I need to filter by sector, I'll use a semi join instead of building a boolean mask directly — that keeps the left rows that match the selected sectors cleanly. I'm also being careful with dtype features, making sure u64 support is available when needed.
> 
> For the time handling, I'm using chrono's NaiveTime from the bar timestamp and the Hong Kong timezone helper. The Px type already implements Ord for stop comparisons, and LotQty sizing happens inside size_book. For the equity CLI, I'm parsing Decimal values from strings, and AlphaMap serializes to JSON via serde. The FactorWeights attribution logic scans and collects the daily IC rows, making sure the sector frame has both the return column and CODE for cross-section calculations.
> 
> When joining scores with realized returns on CODE, both frames might have a date column which causes a suffix collision—that's fine since the duplicate goes unused. I'm keeping the in-memory stage2 frame date-free for the IC calculation, then adding the date column only when persisting to the lake. For the alpha refit join, I'm explicitly joining on both code and date since both frames have those columns. The stage2 scores output includes the `__cs` column from the pipeline, so I'll drop it before persisting using a safe pattern that ignores errors if the column doesn't exist.
> 
> For the sector auction case with all-None gaps, the weighted aggregation returns rows with null values rather than an empty frame, which the composite handles fine. The `select_sectors` function returns a vector of sector IDs, and I'm verifying that stage1 config fields like member_weight_cap, fdr_q, icir_window, and icir_shrink_delta are all being used correctly. When passing the sector auction frame to the composite, extra columns like n_members and date pass through harmlessly since the join is on sector only. The OpenContext construction needs owned DataFrame clones, and the nightly snapshot function filters and renames the lazy frame before collecting and joining. For premarket, I need today's date for range calculations, with the previous day coming from the calendar's trading day lookup.
> 
> For the bars and flows, I'm pulling 1-minute bars up through the previous day for sigma and volatility calculations, while flows for s5 should gracefully fall back to a null-filled frame if the dataset is absent. I'm also defining engine column constants like sigma_15m, v_bin_med_0930, and internal columns for intermediate values, plus building out the EngineError type with conversions from Polars, data, factor, signal, core, IO, and JSON errors, along with channel and contract variants. Channel send failures map to EngineError::Channel.
> 
> Now I'm laying out the session schedule logic: half-day entry at 09:45, exit at 11:45, then CasResidual at 11:58 (reusing lunch_cancel), and PostClose at 12:10 (computed as am_close + 10 minutes). Full-day PostClose sits at cas_hard_end (16:10). I'm using NaiveTime's Add impl with chrono::Duration to compute these times, and I'll keep the module structure compact while preserving the actor test details. Defining the premarket load function... Reusing the enriched dataframe with a cheap clone for efficiency across slices and beta calculations. For the x6 frame, I'm partitioning by sector after selecting the key columns, then computing float cap per code within each partition to identify leaders by market cap. The lagged correlation with FDR filtering is guarded to handle edge cases like zero graphs. I'm also setting up the core data structures and function signatures for the freeze module, including the Frozen struct and run_freeze function.
> 
> Now defining the public API for refresh_x3 and build_candidates, along with test helpers for member auction frames and stage2 inputs. For build_candidates, I'm converting equity from Cash to f64 using Decimal's ToPrimitive trait, computing the cost floor in basis points, and constructing Candidate objects with sector IDs, scores, and optional premarket metrics. The test will verify that freeze correctly selects the expected sector and generates candidates with proper filtering and sizing.
> 
> For the day test, I'm building a minimal TradingDay via a test helper with an armed stop order, then injecting a bar below the stop level to verify the execution channel receives a sell order. I'll also need to cover book, morning, schedule, pacing, and venue tests for comprehensive coverage. The live main will assemble the full pipeline and subscribe to bars across the universe via tiger polling. I'm estimating the final code spans roughly 120 lines for config, 150 for lake, 380 for exec, 1100 for engine, 130 for live, 120 for tomls, and 450 words of prose—substantial but consistent with the series scope.
> 
> A few implementation details: the engine's premarket uses PanelBuilder which requires FactorCfg from the config, and I need to add rust_decimal_macros as a dev dependency since the book and exec tests use the dec! macro. The actor's risk watcher needs to call changed() on the Receiver, and PaperVenue's submit method takes a timestamp parameter and returns either a Fill or an ExecError. When submit fails, the actor logs a warning and continues. I'll keep the intro concise—docs attached, M3 hands off to M4 explicitly—then lay out the plan, reproducing config.rs and lake.rs in full per convention while describing the rest.

All five documents are attached in full, so I worked directly from them — no fetching needed. M3's hand-off makes the next milestone explicit and nearly writes its spec: "Milestone 4 — the engine — is now almost entirely glue: its `ScoreFreeze → Confirmation → Entry` phases are `sector_composite → select_sectors → stage2_scores → ConfirmationTracker → entry_filter → size_book`, its bar handler is `StopBook::breached`, its every actor holds a `watch<RiskState>` receiver, and its `PostClose` is the attribution pipeline." M3 also deferred four wiring facts *to* M4 by name: the `Dataset::Scores` variant, `OpsCfg.universe_static_path`, the tentative-size cost floor from `equity/(K·m)`, and the PostClose scheduling of `realized_window_returns → daily_ic_row → append_ic_history`.

# Milestone 4 — The Engine & Execution Seam (`hkq-engine` + `hkq-exec` + `hkq-live`)

**Why this is next.** M1 produces facts, M2 produces opinions, M3 converts opinions into decisions — but nothing yet owns *time*. Every M3 function is deliberately pure ("the engine owns WHEN; this crate owns WHETHER and WHY"), which means the entire remaining risk of the system is temporal glue: the runbook state machine, the single-writer book, the fan-in of morning feeds into the 09:29:30 freeze, and the two feedback loops that only close once something persists live-frozen scores and fills. M4 delivers exactly that glue and nothing else: every statistical and accounting decision was already locked under M1–M3 tests, so the engine can be a thin, auditable clock — the failure mode the whole milestone ordering exists to avoid (debugging statistics and sockets simultaneously) is now structurally impossible.

**In scope:** the `hkq-exec` crate (parent/child order model with the lot-multiple invariant, pure participation pacer with VCM pause and flush, `Venue` trait + deterministic `PaperVenue`, the exec actor with halt-aware semantics); the `hkq-engine` crate (schedule-as-data runbook, Decimal book with statutory costs per fill, morning absorption state, the PreMarket assembly that rebuilds all nightly factor state from the lake, freeze/X3-refresh/entry/exit phase actions, the `run_day` select loop, PostClose persistence + attribution + AlphaMap refit); the `hkq-live` paper binary; `Dataset::Scores` (the two-line variant M3 reserved); and `OpsCfg.universe_static_path`. **Deferred:** the live Tiger routing venue (M5, behind the `Venue` trait shipped here), `hkq-validate` (CUSUM producer, quarterly θ/gate/v*/β fits — M5), backtest replay (`SimClock`/`SimExec` swap behind the same seams, M6), and VHSI ingestion (still `None` → global bucket, unchanged degradation).

Engineering decisions beyond the blueprint sketch, briefly. The runbook is *data*, not control flow: `build_schedule` returns a sorted `Vec<(NaiveTime, Action)>` with same-instant actions folded in list order, so half-day compression is a different list, not a different loop — and the schedule is unit-testable without a clock. Phase entry actions are instantaneous methods on `TradingDay`; the async loop is ~40 lines and owns no logic. PreMarket *recomputes* all derived panels from the lake instead of persisting them nightly: panels are pure functions of M1 partitions, recomputation at 08:45 is seconds at this scale, and it avoids inventing schema contracts for intermediates — the only new persisted artifacts are the ones that *cannot* be recomputed (live-frozen `Scores`, `Fills`, `Auction` snapshots — the last closing M2's `iev_bar20` gap so S2/X1 confirmation self-heal as history accumulates). The attribution row carries both stock ICs (`ic_x*`) and sector ICs (`ic_s*`) by reusing `daily_ic_row` with the sector id aliased to the `code` join key — documented reuse, not a hack left implicit — so *both* stages' ICIR weights refresh from one dataset. The X3 two-phase refresh is a column swap (`vol_tau0`, `vol_tau0_med20`) followed by re-running `stage2_scores` on the frozen input frame: one expression, two inputs, exactly as the blueprint's reconciliation note demanded. Exec's halt semantics are asymmetric by design: a `Halted` state cancels and rejects *buys* while sells still flow — a kill switch that blocked exits would be a capital trap. `ParentOrder`/`ChildOrder` carry `shares + lot` with the lot-multiple invariant enforced at construction rather than reusing `LotQty`, because exits are born from accumulated fills, not from cash targets — `LotQty` remains the sizing-side proof and hkq-core stays frozen. Equity is a CLI argument, not a new `TradeCfg` field: adding a field would silently break M3's literal struct constructions in two sibling crates' tests — the type contract is load-bearing. AH deltas use the ρ=1 prior with the POS-priced gap subtracted where available (β refresh is validate's job); `a50_beta_ret` stays null until per-sector mainland betas exist, because a constant column would fabricate a cross-sectional factor. `sigma_15m` is the trailing std of 15-minute log returns over the seasonal window — the one simplification in this milestone, isolated in one helper with the EWMA refinement flagged for validate.

```text
hkq/
├── Cargo.toml                        (updated: members)
└── crates/
    ├── hkq-core/src/config.rs        (updated: OpsCfg.universe_static_path)
    ├── hkq-data/src/lake.rs          (updated: Dataset::Scores)
    ├── hkq-exec/
    │   ├── Cargo.toml
    │   └── src/{lib,model,pacing,venue,actor}.rs
    ├── hkq-engine/
    │   ├── Cargo.toml
    │   └── src/{lib,error,cols,schedule,book,morning,premarket,freeze,day}.rs
    └── hkq-live/
        ├── Cargo.toml
        └── src/main.rs
```

## Workspace

```toml
# Cargo.toml
[workspace]
resolver = "2"
members = [
  "crates/hkq-core", "crates/hkq-data", "crates/hkq-factors",
  "crates/hkq-signal", "crates/hkq-risk", "crates/hkq-exec",
  "crates/hkq-engine", "crates/hkq-nightly", "crates/hkq-live",
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
# Feature set unchanged from M2/M3 — M4 adds no new expression surface.
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

## `hkq-core` — one config field

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
    /// M4 (the field M3 reserved): static reference parquet with columns
    /// [code:u32, sector:u32, float_cap:f64, board_lot:u32, connect_elig:u32,
    ///  spread_bps:f64?]. Required by hkq-engine (loud error at PreMarket if
    /// absent); hkq-nightly ignores it — Option keeps M1/M3 configs valid.
    pub universe_static_path: Option<PathBuf>,
    #[serde(default)]
    pub log_json: bool,
}
```

## `hkq-data` — the reserved `Scores` variant

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
    /// M4: the live-frozen stage-2 frame persisted at PostClose — the raw material
    /// of the AlphaMap refit and the honest input of nightly attribution (M3 note).
    Scores,
}

impl Dataset {
    pub fn dir(self) -> &'static str {
        match self {
            Dataset::DailyBars => "daily_bars",  Dataset::Bars1m => "bars_1m",
            Dataset::Auction => "auction",       Dataset::Flows => "flows",
            Dataset::Ccass => "ccass",           Dataset::MainlandPrints => "mainland",
            Dataset::Fx => "fx",                 Dataset::Vhsi => "vhsi",
            Dataset::Fills => "fills",           Dataset::Attribution => "attribution",
            Dataset::Scores => "scores",
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
        lake.write_partition(Dataset::DailyBars, date, &mut df0, "test", 1).unwrap();

        let out = lake.scan_date(Dataset::DailyBars, date).unwrap().collect().unwrap();
        assert_eq!(out.height(), 2);
        assert!(out.get_column_names().iter().any(|c| c.as_str() == "close"));

        std::fs::remove_dir_all(root).ok();
    }

    #[test]
    fn scores_dataset_has_a_home() {
        assert_eq!(Dataset::Scores.dir(), "scores");
    }
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
hkq-risk = { path = "../hkq-risk" }
tokio.workspace = true
async-trait.workspace = true
thiserror.workspace = true
tracing.workspace = true
chrono.workspace = true
```

```rust
// crates/hkq-exec/src/lib.rs
#![forbid(unsafe_code)]
//! Execution seam (report §3.6, blueprint dataflow): parent orders in, paced
//! lot-multiple children out, fills back. The actor is a *policy* shell — all
//! pacing arithmetic is pure and synchronously tested. Venue routing is a trait:
//! `PaperVenue` ships now (shadow/paper tier); the signed Tiger order route is
//! M5 scope and plugs in behind the same trait without touching this crate's
//! semantics.
//!
//! Halt semantics (deliberate asymmetry): a `Halted` risk state cancels resting
//! BUY parents and rejects new ones; SELL flow continues untouched. A kill
//! switch that blocked exits would be a capital trap, not a safety mechanism.

pub mod actor;
pub mod model;
pub mod pacing;
pub mod venue;

pub use actor::spawn_exec;
pub use model::{ChildOrder, ExecCmd, ExecError, Fill, Pacing, ParentOrder, Side};
pub use pacing::Pacer;
pub use venue::{PaperVenue, Venue};
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
    /// Children released against realized bar volume: cum_sent ≤ cap · cum_vol.
    Paced,
    /// Whole quantity submitted at once (protective stops, kill path, CAS flush).
    Immediate,
}

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
}
```

```rust
// crates/hkq-exec/src/pacing.rs
//! Pure participation pacing (§3.6): cumulative children ≤ cap · cumulative
//! realized volume, floored to whole lots. No clocks, no channels — the actor
//! feeds bars in, children come out.
use crate::model::{ChildOrder, Pacing, ParentOrder, Side};
use hkq_core::ids::StockCode;

#[derive(Debug)]
struct Slot {
    p: ParentOrder,
    sent: u64,
    cum_vol: f64,
    paused: bool,
}

#[derive(Debug)]
pub struct Pacer {
    cap: f64,
    slots: Vec<Slot>,
}

impl Pacer {
    pub fn new(participation_cap: f64) -> Self {
        Self { cap: participation_cap.max(0.0), slots: Vec::new() }
    }

    /// Register a parent. Immediate pacing emits the full child at once and
    /// retains nothing; Paced parents wait for volume.
    pub fn place(&mut self, p: ParentOrder, pacing: Pacing) -> Vec<ChildOrder> {
        match pacing {
            Pacing::Immediate => vec![child(&p, p.shares)],
            Pacing::Paced => {
                self.slots.push(Slot { p, sent: 0, cum_vol: 0.0, paused: false });
                vec![]
            }
        }
    }

    /// Advance pacing on one bar of `code`. allowed = ⌊cap·cum_vol⌋ shares,
    /// next child = lot-floored (allowed − sent), capped at the remainder.
    pub fn on_bar(&mut self, code: StockCode, volume: f64) -> Vec<ChildOrder> {
        let mut out = Vec::new();
        for s in self.slots.iter_mut() {
            if s.p.code != code || s.paused {
                continue;
            }
            if volume.is_finite() && volume > 0.0 {
                s.cum_vol += volume;
            }
            let allowed = (self.cap * s.cum_vol).floor() as u64;
            let target = allowed.min(s.p.shares);
            let inc = target.saturating_sub(s.sent) / s.p.lot as u64 * s.p.lot as u64;
            if inc > 0 {
                s.sent += inc;
                out.push(child(&s.p, inc));
            }
        }
        self.slots.retain(|s| s.sent < s.p.shares);
        out
    }

    /// Everything still resting, submitted now (CAS flush / kill path).
    pub fn flush(&mut self) -> Vec<ChildOrder> {
        let out = self.slots.iter()
            .map(|s| child(&s.p, s.p.shares - s.sent))
            .collect();
        self.slots.clear();
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
        let n = self.slots.len();
        self.slots.retain(|s| s.p.code != code);
        n - self.slots.len()
    }

    pub fn cancel_all(&mut self) -> usize {
        std::mem::take(&mut self.slots).len()
    }

    pub fn cancel_buys(&mut self) -> usize {
        let n = self.slots.len();
        self.slots.retain(|s| s.p.side != Side::Buy);
        n - self.slots.len()
    }

    pub fn resting(&self) -> usize {
        self.slots.len()
    }
}

fn child(p: &ParentOrder, shares: u64) -> ChildOrder {
    ChildOrder { code: p.code, side: p.side, shares, lot: p.lot, limit: p.limit }
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
    fn paced_release_respects_cap_and_lots() {
        let mut p = Pacer::new(0.02);
        assert!(p.place(parent(10_000), Pacing::Paced).is_empty());
        // 100k volume ⇒ allowed 2000 ⇒ 4 lots.
        let c = p.on_bar(StockCode(700), 100_000.0);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].shares, 2_000);
        // +40k ⇒ allowed 2800 ⇒ inc 800 → lot-floor 500.
        let c = p.on_bar(StockCode(700), 40_000.0);
        assert_eq!(c[0].shares, 500);
        // Huge volume ⇒ remainder only, then slot retires.
        let c = p.on_bar(StockCode(700), 10_000_000.0);
        assert_eq!(c[0].shares, 7_500);
        assert_eq!(p.resting(), 0);
        assert!(p.on_bar(StockCode(700), 1e9).is_empty());
    }

    #[test]
    fn pause_flush_cancel_and_immediate() {
        let mut p = Pacer::new(0.02);
        p.place(parent(10_000), Pacing::Paced);
        p.pause(StockCode(700), true);
        assert!(p.on_bar(StockCode(700), 1e6).is_empty()); // VCM cooling-off
        p.pause(StockCode(700), false);
        assert_eq!(p.on_bar(StockCode(700), 100_000.0)[0].shares, 2_000);
        let f = p.flush();
        assert_eq!(f[0].shares, 8_000);
        assert_eq!(p.resting(), 0);

        let c = p.place(parent(1_000), Pacing::Immediate);
        assert_eq!(c[0].shares, 1_000); // stops/kill: no pacing
        p.place(parent(1_000), Pacing::Paced);
        let mut sell = parent(500);
        sell.side = Side::Sell;
        p.place(sell, Pacing::Paced);
        assert_eq!(p.cancel_buys(), 1); // halt semantics: buys die, sells live
        assert_eq!(p.resting(), 1);
        assert_eq!(p.cancel_all(), 1);
    }
}
```

```rust
// crates/hkq-exec/src/venue.rs
//! The routing seam. `PaperVenue` fills every child fully at its limit —
//! deterministic by design for the shadow tier; spread/impact realism is the
//! backtest SimExec's job (M6) and live routing is Tiger's (M5), both behind
//! this trait.
use crate::model::{ChildOrder, ExecError, Fill};
use async_trait::async_trait;

#[async_trait]
pub trait Venue: Send + Sync {
    async fn submit(&self, child: &ChildOrder, ts_ms: i64) -> Result<Fill, ExecError>;
}

pub struct PaperVenue;

#[async_trait]
impl Venue for PaperVenue {
    async fn submit(&self, child: &ChildOrder, ts_ms: i64) -> Result<Fill, ExecError> {
        Ok(Fill {
            code: child.code,
            side: child.side,
            shares: child.shares,
            lot: child.lot,
            px: child.limit,
            ts_ms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Side;
    use hkq_core::{ids::StockCode, money::Px};

    #[tokio::test]
    async fn paper_fills_at_limit() {
        let c = ChildOrder {
            code: StockCode(700), side: Side::Buy, shares: 500, lot: 500,
            limit: Px::from_f64_quote(10.0).unwrap(),
        };
        let f = PaperVenue.submit(&c, 123).await.unwrap();
        assert_eq!(f.shares, 500);
        assert_eq!(f.px, c.limit);
        assert_eq!(f.ts_ms, 123);
    }
}
```

```rust
// crates/hkq-exec/src/actor.rs
//! The exec actor: single consumer of `ExecCmd`, single producer of `Fill`.
//! Policy lives in `Pacer`; this loop is transport + the halt rule.
use crate::model::{ExecCmd, Fill, Pacing, Side};
use crate::pacing::Pacer;
use crate::venue::Venue;
use hkq_risk::RiskState;
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;

pub fn spawn_exec<V: Venue + 'static>(
    venue: V,
    participation_cap: f64,
    fill_tx: mpsc::Sender<Fill>,
    mut risk: watch::Receiver<RiskState>,
) -> (mpsc::Sender<ExecCmd>, JoinHandle<()>) {
    let (tx, mut rx) = mpsc::channel::<ExecCmd>(1024);
    let handle = tokio::spawn(async move {
        let mut pacer = Pacer::new(participation_cap);
        loop {
            tokio::select! {
                changed = risk.changed() => {
                    if changed.is_err() { break; }
                    if risk.borrow().halted() {
                        let n = pacer.cancel_buys();
                        tracing::error!(cancelled_buys = n, "exec: HALTED — buys cancelled, sells continue");
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
                                submit(&venue, &fill_tx, &c, ts).await;
                            }
                        }
                        ExecCmd::OnBar { code, ts_ms, volume, .. } => {
                            for c in pacer.on_bar(code, volume) {
                                submit(&venue, &fill_tx, &c, ts_ms).await;
                            }
                        }
                        ExecCmd::CancelSymbol(code) => { pacer.cancel_symbol(code); }
                        ExecCmd::CancelAll => {
                            let n = pacer.cancel_all();
                            tracing::info!(cancelled = n, "exec: cancel all resting");
                        }
                        ExecCmd::Pause { code, on } => {
                            tracing::warn!(%code, on, "exec: VCM pause toggle");
                            pacer.pause(code, on);
                        }
                        ExecCmd::Flush => {
                            let ts = chrono::Utc::now().timestamp_millis();
                            for c in pacer.flush() {
                                submit(&venue, &fill_tx, &c, ts).await;
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

async fn submit<V: Venue>(
    venue: &V, fill_tx: &mpsc::Sender<Fill>, child: &crate::model::ChildOrder, ts_ms: i64,
) {
    match venue.submit(child, ts_ms).await {
        Ok(fill) => {
            if fill_tx.send(fill).await.is_err() {
                tracing::warn!("exec: fill channel closed");
            }
        }
        Err(e) => tracing::error!(code = %child.code, error = %e, "exec: child rejected"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ParentOrder, Side};
    use crate::venue::PaperVenue;
    use hkq_core::{ids::StockCode, money::Px};
    use hkq_risk::{HaltReason, KillSwitch};

    fn parent(side: Side) -> ParentOrder {
        ParentOrder::new(StockCode(700), side, 1_000, 500,
                         Px::from_f64_quote(10.0).unwrap()).unwrap()
    }

    #[tokio::test]
    async fn paced_place_bar_fill_roundtrip_and_halt_asymmetry() {
        let (ks, rx) = KillSwitch::new();
        let (fill_tx, mut fill_rx) = mpsc::channel(64);
        let (cmd_tx, _h) = spawn_exec(PaperVenue, 0.5, fill_tx, rx);

        cmd_tx.send(ExecCmd::Place { order: parent(Side::Buy), pacing: Pacing::Paced })
            .await.unwrap();
        cmd_tx.send(ExecCmd::OnBar { code: StockCode(700), ts_ms: 1, close: 10.0, volume: 2_000.0 })
            .await.unwrap();
        let f = fill_rx.recv().await.unwrap();
        assert_eq!(f.shares, 1_000); // 0.5·2000 covers the whole parent
        assert_eq!(f.side, Side::Buy);

        ks.halt(HaltReason::Operator);
        cmd_tx.send(ExecCmd::Place { order: parent(Side::Buy), pacing: Pacing::Immediate })
            .await.unwrap();
        cmd_tx.send(ExecCmd::Place { order: parent(Side::Sell), pacing: Pacing::Immediate })
            .await.unwrap();
        let f = fill_rx.recv().await.unwrap(); // only the SELL fills
        assert_eq!(f.side, Side::Sell);
        assert!(tokio::time::timeout(std::time::Duration::from_millis(50), fill_rx.recv())
            .await.is_err());
    }
}
```

## `hkq-engine`

```toml
# crates/hkq-engine/Cargo.toml
[package]
name = "hkq-engine"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-data = { path = "../hkq-data" }
hkq-factors = { path = "../hkq-factors" }
hkq-signal = { path = "../hkq-signal" }
hkq-risk = { path = "../hkq-risk" }
hkq-exec = { path = "../hkq-exec" }
polars.workspace = true
tokio.workspace = true
chrono.workspace = true
chrono-tz.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
tracing.workspace = true
rust_decimal.workspace = true

[dev-dependencies]
rust_decimal_macros.workspace = true
```

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
//!   Panels are pure functions of partitions; recomputing beats persisting
//!   intermediates (no new schema contracts, no staleness).
//! - `morning`: absorb POS/mainland/A50 into the pre-freeze board.
//! - `freeze`: 09:29:30 OpenContext assembly → Stage 1 → Stage 2; 09:35 X3
//!   column swap; candidate construction for the 09:45 gate.
//! - `book`: single-writer Decimal book; statutory costs charged per fill.
//! - `day`: the `tokio::select!` loop — a clock around tested functions.

pub mod book;
pub mod cols;
pub mod day;
pub mod error;
pub mod freeze;
pub mod morning;
pub mod premarket;
pub mod schedule;

pub use day::{Channels, RunCfg, TradingDay};
pub use error::EngineError;
pub use premarket::NightlyState;
pub use schedule::{build_schedule, Action, Phase};
```

```rust
// crates/hkq-engine/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error(transparent)]
    Core(#[from] hkq_core::error::CoreError),
    #[error(transparent)]
    Data(#[from] hkq_data::error::DataError),
    #[error(transparent)]
    Factor(#[from] hkq_factors::FactorError),
    #[error(transparent)]
    Signal(#[from] hkq_signal::SignalError),
    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("state serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("channel closed: {0}")]
    Channel(&'static str),
    #[error("contract violated: {0}")]
    Contract(&'static str),
    #[error("engine state: {0}")]
    State(String),
}
```

```rust
// crates/hkq-engine/src/cols.rs
//! Engine-born column names (everything else imports hkq_factors::cols).
pub const SIGMA_15M: &str = "sigma_15m";
/// Trailing same-bin median of the 09:30 continuous 5m volume — the non-auction
/// half of X3's baseline (the auction half is IEV_BAR20).
pub const VMED_0930: &str = "v_bin_med_0930";
pub const SPREAD_BPS: &str = "spread_bps";
pub const BOARD_LOT: &str = "board_lot";
pub const IEV_RAW: &str = "__iev_raw";
pub const V0935: &str = "__v0935";
```

```rust
// crates/hkq-engine/src/schedule.rs
//! The §6 runbook as data. Same-instant actions fold in list order, so
//! "Entry then MorningHold at 09:45" is two rows, not special-cased control
//! flow — and half-day compression is a different list, not a different loop.
use hkq_core::config::HalfDayMode;
use hkq_core::session::{DayKind, SessionTimes};
use chrono::NaiveTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    PreMarket, PosStream, MainlandPrints, ScoreFreeze, Confirmation, Entry,
    MorningHold, Lunch, AfternoonHold, ExitProgram, CasResidual, PostClose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Enter(Phase),
    /// §3.3 two-phase X3: refresh vol_tau0 with the first-5m volume, re-score.
    X3Refresh,
    /// 11:58 — cancel resting orders into lunch (§3.6).
    LunchCancel,
    /// Submit residuals into the CAS window (paper: flush).
    CasFlush,
}

pub fn build_schedule(kind: DayKind, mode: HalfDayMode) -> Vec<(NaiveTime, Action)> {
    use Action::*;
    use Phase::*;
    let s = SessionTimes::get();
    match (kind, mode) {
        (DayKind::Closed, _) => vec![],
        (DayKind::HalfDay, HalfDayMode::Skip) => vec![],
        (DayKind::HalfDay, HalfDayMode::Compressed) => {
            let post = s.am_close.overflowing_add_signed(chrono::Duration::minutes(10)).0;
            vec![
                (s.pos_start, Enter(PosStream)),
                (s.pos_match_to, Enter(MainlandPrints)),
                (s.score_freeze, Enter(ScoreFreeze)),
                (s.open, Enter(Confirmation)),
                (s.x3_refresh, X3Refresh),
                (s.entry, Enter(Entry)),
                (s.entry, Enter(MorningHold)),
                (s.half_day_exit, Enter(ExitProgram)),   // 11:45 compressed exit (§5)
                (s.lunch_cancel, Enter(CasResidual)),
                (s.lunch_cancel, CasFlush),
                (post, Enter(PostClose)),
            ]
        }
        (DayKind::Full, _) => vec![
            (s.pos_start, Enter(PosStream)),
            (s.pos_match_to, Enter(MainlandPrints)),
            (s.score_freeze, Enter(ScoreFreeze)),
            (s.open, Enter(Confirmation)),
            (s.x3_refresh, X3Refresh),
            (s.entry, Enter(Entry)),
            (s.entry, Enter(MorningHold)),
            (s.lunch_cancel, LunchCancel),
            (s.am_close, Enter(Lunch)),
            (s.pm_open, Enter(AfternoonHold)),
            (s.exit_start, Enter(ExitProgram)),
            (s.exit_end, Enter(CasResidual)),
            (s.cont_close, CasFlush),
            (s.cas_hard_end, Enter(PostClose)),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn t(h: u32, m: u32) -> NaiveTime { NaiveTime::from_hms_opt(h, m, 0).unwrap() }

    #[test]
    fn full_day_is_sorted_and_complete() {
        let s = build_schedule(DayKind::Full, HalfDayMode::Skip);
        assert_eq!(s.len(), 14);
        assert!(s.windows(2).all(|w| w[0].0 <= w[1].0));
        assert_eq!(s[0], (t(9, 0), Action::Enter(Phase::PosStream)));
        assert!(s.contains(&(t(9, 35), Action::X3Refresh)));
        assert!(s.contains(&(t(11, 58), Action::LunchCancel)));
        assert_eq!(s.last().unwrap().1, Action::Enter(Phase::PostClose));
    }

    #[test]
    fn half_day_modes() {
        assert!(build_schedule(DayKind::HalfDay, HalfDayMode::Skip).is_empty());
        assert!(build_schedule(DayKind::Closed, HalfDayMode::Compressed).is_empty());
        let s = build_schedule(DayKind::HalfDay, HalfDayMode::Compressed);
        assert!(s.windows(2).all(|w| w[0].0 <= w[1].0));
        assert!(s.iter().all(|(tt, _)| *tt <= t(12, 10)));
        assert!(!s.iter().any(|(_, a)| *a == Action::Enter(Phase::AfternoonHold)));
        assert!(s.contains(&(t(11, 45), Action::Enter(Phase::ExitProgram))));
    }
}
```

```rust
// crates/hkq-engine/src/book.rs
//! Single-writer Decimal book. Statutory costs (stamp ceil-to-dollar, fees)
//! are charged INSIDE apply_fill via hkq-risk's CostModel — the backtest and
//! live tiers share this exact accounting by construction (§4 cost realism).
//! Marks are f64 (statistics side, reporting only); cash is Decimal and is the
//! ONLY equity number sizing ever sees.
use hkq_core::ids::StockCode;
use hkq_core::money::Cash;
use hkq_exec::{Fill, Side};
use hkq_risk::CostModel;
use polars::prelude::*;
use rust_decimal::Decimal;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub shares: u64,
    pub lot: u32,
}

#[derive(Debug)]
pub struct Book {
    cash: Cash,
    cost: CostModel,
    pos: BTreeMap<StockCode, Pos>,
    marks: HashMap<StockCode, f64>,
    // Fill log for the Fills partition (stats-side copies of Decimal charges).
    f_code: Vec<u32>, f_ts: Vec<i64>, f_side: Vec<&'static str>,
    f_shares: Vec<u64>, f_px: Vec<f64>, f_duty: Vec<f64>, f_fees: Vec<f64>,
}

impl Book {
    pub fn new(cash: Cash, cost: CostModel) -> Self {
        Self {
            cash, cost, pos: BTreeMap::new(), marks: HashMap::new(),
            f_code: vec![], f_ts: vec![], f_side: vec![],
            f_shares: vec![], f_px: vec![], f_duty: vec![], f_fees: vec![],
        }
    }

    pub fn cash(&self) -> Cash { self.cash }
    pub fn is_flat(&self) -> bool { self.pos.is_empty() }
    pub fn position(&self, code: StockCode) -> Option<Pos> { self.pos.get(&code).copied() }
    pub fn open_positions(&self) -> Vec<(StockCode, Pos)> {
        self.pos.iter().map(|(c, p)| (*c, *p)).collect()
    }
    pub fn mark(&self, code: StockCode) -> Option<f64> { self.marks.get(&code).copied() }
    pub fn update_mark(&mut self, code: StockCode, close: f64) {
        if close.is_finite() && close > 0.0 {
            self.marks.insert(code, close);
        }
    }
    pub fn n_fills(&self) -> usize { self.f_code.len() }

    pub fn apply_fill(&mut self, f: &Fill) {
        let notional = Cash(f.px.get() * Decimal::from(f.shares));
        let duty = self.cost.stamp_duty(notional);
        let fees = self.cost.fees_per_side(notional);
        match f.side {
            Side::Buy => {
                self.cash = Cash(self.cash.0 - notional.0 - duty.0 - fees.0);
                let e = self.pos.entry(f.code).or_insert(Pos { shares: 0, lot: f.lot });
                e.shares += f.shares;
            }
            Side::Sell => {
                let held = self.pos.get(&f.code).map(|p| p.shares).unwrap_or(0);
                let qty = f.shares.min(held);
                if qty < f.shares {
                    tracing::error!(code = %f.code, "sell fill exceeds position; clamped");
                }
                self.cash = Cash(self.cash.0 + notional.0 - duty.0 - fees.0);
                if let Some(p) = self.pos.get_mut(&f.code) {
                    p.shares -= qty;
                    if p.shares == 0 {
                        self.pos.remove(&f.code);
                    }
                }
            }
        }
        self.marks.insert(f.code, f.px.as_f64());
        self.f_code.push(f.code.0);
        self.f_ts.push(f.ts_ms);
        self.f_side.push(match f.side { Side::Buy => "buy", Side::Sell => "sell" });
        self.f_shares.push(f.shares);
        self.f_px.push(f.px.as_f64());
        self.f_duty.push(duty.0.to_string().parse().unwrap_or(f64::NAN));
        self.f_fees.push(fees.0.to_string().parse().unwrap_or(f64::NAN));
    }

    /// Marked equity — REPORTING ONLY (f64 marks). Sizing uses `cash()`.
    pub fn marked_equity(&self) -> f64 {
        let cash: f64 = self.cash.0.to_string().parse().unwrap_or(f64::NAN);
        cash + self.pos.iter()
            .map(|(c, p)| self.marks.get(c).copied().unwrap_or(0.0) * p.shares as f64)
            .sum::<f64>()
    }

    pub fn fills_frame(&self, date: chrono::NaiveDate) -> PolarsResult<DataFrame> {
        df!(
            "code" => self.f_code.clone(),
            "date" => vec![date.to_string(); self.f_code.len()],
            "ts_ms" => self.f_ts.clone(),
            "side" => self.f_side.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            "shares" => self.f_shares.clone(),
            "px" => self.f_px.clone(),
            "duty" => self.f_duty.clone(),
            "fees" => self.f_fees.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hkq_core::config::CostCfg;
    use hkq_core::money::Px;
    use rust_decimal_macros::dec;

    fn book() -> Book {
        let cost = CostModel::from_cfg(&CostCfg {
            stamp_bps_per_side: 10.0, fees_bps_roundtrip: 2.2, impact_kappa: 0.1,
        }).unwrap();
        Book::new(Cash(dec!(1000000)), cost)
    }

    fn fill(side: Side, shares: u64, px: f64) -> Fill {
        Fill { code: StockCode(700), side, shares, lot: 500,
               px: Px::from_f64_quote(px).unwrap(), ts_ms: 1 }
    }

    #[test]
    fn decimal_roundtrip_with_statutory_costs() {
        let mut b = book();
        b.apply_fill(&fill(Side::Buy, 1_000, 10.0));
        // 10_000 notional + duty ceil(10) + fees 1.10
        assert_eq!(b.cash().0, dec!(989988.90));
        assert!(!b.is_flat());
        assert_eq!(b.position(StockCode(700)).unwrap().shares, 1_000);

        b.apply_fill(&fill(Side::Sell, 1_000, 10.0));
        // +10_000 − 10 − 1.10 ⇒ total round-trip cost 22.20 (M3 cost test scaled)
        assert_eq!(b.cash().0, dec!(999977.80));
        assert!(b.is_flat());
        assert_eq!(b.n_fills(), 2);
        assert!((b.marked_equity() - 999_977.80).abs() < 1e-6);
    }

    #[test]
    fn oversell_clamps_loudly_and_marks_update() {
        let mut b = book();
        b.apply_fill(&fill(Side::Buy, 500, 10.0));
        b.update_mark(StockCode(700), 11.0);
        assert_eq!(b.mark(StockCode(700)), Some(11.0));
        b.apply_fill(&fill(Side::Sell, 1_000, 10.0)); // clamped to 500
        assert!(b.is_flat());
    }
}
```

```rust
// crates/hkq-engine/src/morning.rs
//! Pre-freeze absorption state: the POS {IEP, IEV} trajectory marks that X2/S2
//! need (09:10 / 09:20 / final), mainland 09:25 prints, the A50 window return,
//! and the opening-volume accumulators for the X3 refresh and volume projection.
//! Pure state machine — the engine feeds it, freeze reads it.
use hkq_core::ids::StockCode;
use hkq_core::session::SessionTimes;
use hkq_data::model::AuctionSnap;
use chrono::NaiveTime;
use polars::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default)]
pub struct AuctionMarks {
    pub iep_0910: Option<f64>,
    pub iep_0920: Option<f64>,
    pub iep: Option<f64>,
    pub iev: Option<f64>,
}

#[derive(Debug, Default)]
pub struct MorningBoard {
    pub auction: HashMap<StockCode, AuctionMarks>,
    snaps: Vec<AuctionSnap>,
    pub mainland: HashMap<StockCode, f64>,
    a50_first: Option<f64>,
    a50_last: Option<f64>,
    vol_0935: HashMap<StockCode, f64>,
    vol_0945: HashMap<StockCode, f64>,
}

impl MorningBoard {
    pub fn absorb_auction(&mut self, s: &AuctionSnap) {
        let sess = SessionTimes::get();
        let t = s.ts.time();
        if t > sess.pos_match_to {
            return; // post-auction quote noise
        }
        let m = self.auction.entry(s.code).or_default();
        if let Some(iep) = s.iep.filter(|v| v.is_finite() && *v > 0.0) {
            let t0910 = NaiveTime::from_hms_opt(9, 10, 0).expect("valid");
            if t <= t0910 { m.iep_0910 = Some(iep); }
            if t <= sess.pos_match_from { m.iep_0920 = Some(iep); }
            m.iep = Some(iep);
        }
        if let Some(iev) = s.iev.filter(|v| v.is_finite() && *v >= 0.0) {
            m.iev = Some(iev);
        }
        self.snaps.push(s.clone());
    }

    pub fn absorb_mainland(&mut self, code: StockCode, a_open_ret: f64) {
        if a_open_ret.is_finite() {
            self.mainland.insert(code, a_open_ret);
        }
    }

    pub fn absorb_a50(&mut self, px: f64) {
        if !(px.is_finite() && px > 0.0) {
            return;
        }
        if self.a50_first.is_none() {
            self.a50_first = Some(px);
        }
        self.a50_last = Some(px);
    }

    /// A50 09:00→09:29 window return — reserved for the per-sector mainland-β
    /// job (hkq-validate); exposed so the wiring is ready.
    pub fn a50_window_return(&self) -> Option<f64> {
        let (f, l) = (self.a50_first?, self.a50_last?);
        let r = (l / f).ln();
        r.is_finite().then_some(r)
    }

    /// Opening continuous volume, gated by bar OPEN time (M1 convention).
    pub fn absorb_open_bar(&mut self, code: StockCode, bar_open: NaiveTime, volume: f64) {
        if !(volume.is_finite() && volume >= 0.0) {
            return;
        }
        let s = SessionTimes::get();
        if bar_open < s.open {
            return;
        }
        if bar_open < s.x3_refresh {
            *self.vol_0935.entry(code).or_insert(0.0) += volume;
        }
        if bar_open < s.entry {
            *self.vol_0945.entry(code).or_insert(0.0) += volume;
        }
    }

    pub fn vol_0935(&self, code: StockCode) -> Option<f64> { self.vol_0935.get(&code).copied() }
    pub fn vol_0945(&self, code: StockCode) -> Option<f64> { self.vol_0945.get(&code).copied() }
    pub fn n_snaps(&self) -> usize { self.snaps.len() }

    /// Raw POS trajectory → Dataset::Auction partition. This is the history
    /// that matures IEV_BAR20 and unlocks the confirmed-gap terms (M2 gap #2).
    pub fn auction_frame(&self, date: chrono::NaiveDate) -> PolarsResult<DataFrame> {
        let n = self.snaps.len();
        let (mut code, mut ts, mut iep, mut iev, mut bq, mut aq) =
            (Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n),
             Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n));
        for s in &self.snaps {
            code.push(s.code.0);
            ts.push(s.ts.timestamp_millis());
            iep.push(s.iep);
            iev.push(s.iev);
            bq.push(s.bid_qty);
            aq.push(s.ask_qty);
        }
        df!(
            "code" => code, "date" => vec![date.to_string(); n], "ts_ms" => ts,
            "iep" => iep, "iev" => iev, "bid_qty" => bq, "ask_qty" => aq,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    use hkq_core::session::hk;

    fn snap(h: u32, m: u32, s: u32, iep: f64, iev: f64) -> AuctionSnap {
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        AuctionSnap {
            code: StockCode(700),
            ts: hk(d, NaiveTime::from_hms_opt(h, m, s).unwrap()),
            iep: Some(iep), iev: Some(iev), bid_qty: None, ask_qty: None,
        }
    }

    #[test]
    fn trajectory_marks_and_window_gates() {
        let mut b = MorningBoard::default();
        b.absorb_auction(&snap(9, 5, 0, 100.0, 1e5));
        b.absorb_auction(&snap(9, 9, 50, 101.0, 2e5));
        b.absorb_auction(&snap(9, 15, 0, 102.0, 3e5));
        b.absorb_auction(&snap(9, 21, 30, 103.0, 4e5));
        b.absorb_auction(&snap(9, 40, 0, 999.0, 9e9)); // post-auction: ignored
        let m = b.auction[&StockCode(700)];
        assert_eq!(m.iep_0910, Some(101.0));
        assert_eq!(m.iep_0920, Some(102.0));
        assert_eq!(m.iep, Some(103.0));
        assert_eq!(m.iev, Some(4e5));
        assert_eq!(b.n_snaps(), 4);

        let t = |h, m| NaiveTime::from_hms_opt(h, m, 0).unwrap();
        b.absorb_open_bar(StockCode(700), t(9, 31), 10.0);
        b.absorb_open_bar(StockCode(700), t(9, 36), 20.0);
        b.absorb_open_bar(StockCode(700), t(9, 44), 30.0);
        b.absorb_open_bar(StockCode(700), t(9, 45), 99.0); // ≥ entry: excluded
        assert_eq!(b.vol_0935(StockCode(700)), Some(10.0));
        assert_eq!(b.vol_0945(StockCode(700)), Some(60.0));
    }
}
```

```rust
// crates/hkq-engine/src/premarket.rs
//! 08:45 PreMarket assembly (runbook §6): rebuild every nightly-derived input
//! from the M1 lake through M2's pure functions. Nothing here is persisted —
//! derived panels are pure functions of partitions, and recomputation at this
//! scale (≤500 names) is seconds. The only state files are the ones that CANNOT
//! be recomputed: the AlphaMap JSON (fit from live-frozen scores at PostClose)
//! and the lake datasets themselves.
use crate::cols as ecols;
use crate::error::EngineError;
use chrono::{Duration, NaiveDate};
use hkq_core::config::StrategyCfg;
use hkq_core::ids::{SectorId, StockCode};
use hkq_core::money::BoardLot;
use hkq_core::session::TradingCalendar;
use hkq_data::lake::{Dataset, Lake};
use hkq_factors::cols::{self, base};
use hkq_factors::icir::FactorWeights;
use hkq_factors::leadlag::{demean_rows, lagged_corr_fdr, panel_to_matrix};
use hkq_factors::panel::PanelBuilder;
use hkq_factors::realized::bod5_expr;
use hkq_factors::sector;
use hkq_factors::stage2::RegimeGate;
use hkq_signal::AlphaMap;
use polars::prelude::*;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const E: f64 = std::f64::consts::E;
/// Calendar-day warmup covering max(β 250, EWMA 63 + Amihud 60) trading days.
const WARMUP_CAL_DAYS: i64 = 420;
pub const S_FACTORS: [&str; 6] = ["s1", "s2", "s3", "s4", "s5", "s6"];
pub const X_FACTORS: [&str; 6] = ["x1", "x2", "x3", "x5", "x6", "x7"];
/// AlphaMap refit floor — conservative until hkq-validate owns the schedule.
pub const ALPHA_MIN_OBS: usize = 60;

#[derive(Debug, Clone, Copy)]
pub struct StockNums {
    pub sector: SectorId,
    pub lot: BoardLot,
    pub connect: bool,
    pub prev_close: f64,
    pub sigma_on: Option<f64>,
    pub sigma_cc: Option<f64>,
    pub sigma_15m: Option<f64>,
    pub lav: Option<f64>,
    pub adv_shares: Option<f64>,
    pub spread_bps: Option<f64>,
}

pub struct NightlyState {
    pub date: NaiveDate,
    pub prev: NaiveDate,
    /// One row per universe name (t−1 information only). Columns documented in
    /// `load` — the freeze module's input contract.
    pub stock: DataFrame,
    /// [sector, r_id_1, r_on_1, sigma_id, sigma_on, s3, s4, s5] — Stage 1 input.
    pub sector_nightly: DataFrame,
    pub nums: BTreeMap<StockCode, StockNums>,
    pub weights_s: FactorWeights,
    pub weights_x: FactorWeights,
    pub alpha: AlphaMap,
    pub gate: RegimeGate,
}

pub fn alpha_state_path(lake_root: &Path) -> PathBuf {
    lake_root.join("_state").join("alpha_map.json")
}

impl NightlyState {
    /// Assemble everything the day needs. Loud on missing prerequisites:
    /// the static file and the Bars1m dataset are hard requirements (M3 wired
    /// the backfill precisely so this can be one).
    pub fn load(
        lake: &Lake, cfg: &StrategyCfg, date: NaiveDate, cal: &dyn TradingCalendar,
    ) -> Result<Self, EngineError> {
        let prev = cal.prev_trading_day(date);
        let from = date - Duration::days(WARMUP_CAL_DAYS);
        let static_path = cfg.ops.universe_static_path.as_ref().ok_or(
            EngineError::Contract("ops.universe_static_path is required by the engine"))?;
        let statics = load_statics(static_path)?;
        let sector_map = sector_map_frame(&statics)?;

        // Hard requirement: rv/rsj/jump come from 1m bars (M3's nightly step).
        lake.scan(Dataset::Bars1m).map_err(|_| EngineError::State(
            "bars_1m dataset missing — run hkq-nightly before the engine".into()))?;

        let pb = PanelBuilder::new(lake, &cfg.factors);
        let enriched = pb.enriched_daily(from, prev, Some(sector_map.clone()))?.collect()?;
        let bars = pb.bars_1m(prev - Duration::days(60), prev)?;

        // ── per-stock t−1 slice ───────────────────────────────────────────────
        let prev_s = prev.to_string();
        let stock_prev = enriched.clone().lazy()
            .filter(col(base::DATE).eq(lit(prev_s.clone())))
            .select([
                col(base::CODE), col(cols::SECTOR), col(cols::FLOAT_CAP),
                col(cols::CONNECT_ELIG), col(ecols::SPREAD_BPS),
                col(base::CLOSE).alias(cols::PREV_CLOSE),
                col(cols::SIGMA_ON), col(cols::SIGMA_ID), col(cols::SIGMA_CC),
                col(cols::RV_5D), col(cols::LAV), col(cols::ADV_SHARES),
                col(cols::RSJ).alias(cols::RSJ_1), col(cols::JUMP).alias(cols::JUMP_1),
                col(cols::R_ID).alias(cols::R_ID_1), col(cols::R_ON).alias(cols::R_ON_1),
            ]);

        let beta = beta_frame(&enriched)?;
        let s15 = sigma_15m_frame(bars.clone()).collect()?;
        let vmed = vmed_0930_frame(bars.clone(), cfg, &prev_s)?;
        let iev20 = iev_bar20_frame(lake, prev)?;
        let ivu = ivu_frame(bars.clone(), cfg, &prev_s)?;
        let x6 = x6_frame(&enriched, &prev_s, cfg.stage1.fdr_q)?;
        let sbz = sb_z_frame(lake, from, &prev_s)?;

        let on = |l: LazyFrame, r: DataFrame| {
            l.join(r.lazy(), [col(base::CODE)], [col(base::CODE)],
                   JoinArgs::new(JoinType::Left))
        };
        let stock = on(on(on(on(on(on(on(stock_prev, beta), s15), vmed), iev20), ivu), x6), sbz)
            .collect()?;

        // ── sector nightly block ─────────────────────────────────────────────
        let sector_nightly = sector_nightly_frame(&enriched, bars, lake, cfg, from, prev)?;

        // ── learning-loop state ──────────────────────────────────────────────
        let (weights_s, weights_x) = load_weights(lake, cfg);
        let alpha = load_alpha(&cfg.ops.lake_root);
        let nums = build_nums(&stock, &statics)?;

        tracing::info!(
            names = stock.height(), sectors = sector_nightly.height(), %prev,
            "premarket state assembled"
        );
        Ok(Self {
            date, prev, stock, sector_nightly, nums,
            weights_s, weights_x, alpha, gate: RegimeGate::default(),
        })
    }
}

struct StaticRow { sector: u32, float_cap: f64, lot: u32, connect: bool, spread: Option<f64> }

fn load_statics(path: &Path) -> Result<BTreeMap<StockCode, StaticRow>, EngineError> {
    let df = LazyFrame::scan_parquet(
        path.to_string_lossy().as_ref(),
        ScanArgsParquet {
            hive_options: HiveOptions { enabled: Some(false), ..Default::default() },
            ..Default::default()
        },
    )?.collect()?;
    let code = df.column(base::CODE)?.as_materialized_series().u32()?.clone();
    let sec = df.column(cols::SECTOR)?.as_materialized_series().u32()?.clone();
    let fc = df.column(cols::FLOAT_CAP)?.as_materialized_series().f64()?.clone();
    let lot = df.column(ecols::BOARD_LOT)?.as_materialized_series().u32()?.clone();
    let conn = df.column(cols::CONNECT_ELIG)?.as_materialized_series().u32()?.clone();
    let spr = df.column(ecols::SPREAD_BPS).ok().map(|c| {
        c.as_materialized_series().f64().map(|s| s.clone())
    }).transpose()?;
    let mut out = BTreeMap::new();
    for i in 0..df.height() {
        let (Some(c), Some(s), Some(f), Some(l)) =
            (code.get(i), sec.get(i), fc.get(i), lot.get(i)) else { continue };
        out.insert(StockCode(c), StaticRow {
            sector: s, float_cap: f, lot: l,
            connect: conn.get(i).unwrap_or(0) != 0,
            spread: spr.as_ref().and_then(|s| s.get(i)),
        });
    }
    if out.is_empty() {
        return Err(EngineError::Contract("universe_static parquet has no usable rows"));
    }
    Ok(out)
}

fn sector_map_frame(s: &BTreeMap<StockCode, StaticRow>) -> Result<DataFrame, EngineError> {
    let (mut code, mut sec, mut fc, mut conn, mut spr) =
        (vec![], vec![], vec![], vec![], vec![]);
    for (c, r) in s {
        code.push(c.0);
        sec.push(r.sector);
        fc.push(r.float_cap);
        conn.push(if r.connect { 1.0f64 } else { 0.0 });
        spr.push(r.spread);
    }
    Ok(df!(
        base::CODE => code, cols::SECTOR => sec, cols::FLOAT_CAP => fc,
        cols::CONNECT_ELIG => conn, ecols::SPREAD_BPS => spr,
    )?)
}

/// β̂_i for X1: regression of member z_gap on the float-cap-weighted sector
/// z_gap over the full warmup panel (β refresh cadence is hkq-validate's job).
fn beta_frame(enriched: &DataFrame) -> Result<DataFrame, EngineError> {
    let zon = when(col(cols::SIGMA_ON).gt(lit(0.0)))
        .then(col(cols::R_ON) / col(cols::SIGMA_ON))
        .otherwise(lit(NULL));
    let out = enriched.clone().lazy()
        .with_column(zon.alias("__zon"))
        .with_column(
            ((col(cols::FLOAT_CAP) * col("__zon")).sum()
                .over([col(base::DATE), col(cols::SECTOR)])
                / col(cols::FLOAT_CAP).sum().over([col(base::DATE), col(cols::SECTOR)]))
            .alias("__zsec"),
        )
        .group_by([col(base::CODE)])
        .agg([
            ((col("__zon") - col("__zon").mean())
                * (col("__zsec") - col("__zsec").mean())).sum().alias("__sxy"),
            (col("__zsec") - col("__zsec").mean()).pow(2.0).sum().alias("__sxx"),
        ])
        .with_column(
            when(col("__sxx").gt(lit(1e-12)))
                .then(col("__sxy") / col("__sxx"))
                .otherwise(lit(1.0)) // prior β = 1 when the sector series is degenerate
                .alias(cols::BETA_SECTOR),
        )
        .select([col(base::CODE), col(cols::BETA_SECTOR)])
        .collect()?;
    Ok(out)
}

/// σ15m: trailing std of 15-minute log returns over the seasonal window.
/// (Report: trailing EWMA std — the EWMA refinement rides with hkq-validate's
/// quarterly fits; the simplification is isolated here by design.)
pub fn sigma_15m_frame(bars: LazyFrame) -> LazyFrame {
    bars.sort_by_exprs([col(base::CODE), col(base::TS_MS)], Default::default())
        .with_column(bod5_expr())
        .with_column((col(cols::BOD5) / lit(3)).cast(DataType::Int64).alias("__b15"))
        .group_by([col(base::CODE), col(base::DATE), col("__b15")])
        .agg([col(cols::C1M).last().alias("__c15")])
        .sort_by_exprs([col(base::CODE), col(base::DATE), col("__b15")], Default::default())
        .with_column(
            (col("__c15").log(E) - col("__c15").log(E).shift(lit(1)))
                .over([col(base::CODE), col(base::DATE)])
                .alias("__r15"),
        )
        .group_by([col(base::CODE)])
        .agg([col("__r15").std(1).alias(ecols::SIGMA_15M)])
}

/// Trailing same-bin median volume of the 09:30 continuous 5m bin (bod5 = 114):
/// the non-auction half of X3's baseline.
fn vmed_0930_frame(
    bars: LazyFrame, cfg: &StrategyCfg, prev_s: &str,
) -> Result<DataFrame, EngineError> {
    Ok(hkq_factors::realized::seasonal_bin_volume(bars, cfg.factors.seasonal_vol_days)
        .filter(col(base::DATE).eq(lit(prev_s.to_string()))
            .and(col(cols::BOD5).eq(lit(114i64))))
        .select([col(base::CODE), col(cols::V_BIN_MED).alias(ecols::VMED_0930)])
        .collect()?)
}

/// 20d mean of the final auction IEV per name — null until the live engine has
/// accumulated Auction partitions (the documented S2/X1 cold-start degradation).
fn iev_bar20_frame(lake: &Lake, prev: NaiveDate) -> Result<DataFrame, EngineError> {
    let empty = || -> Result<DataFrame, EngineError> {
        Ok(df!(base::CODE => Vec::<u32>::new(), cols::IEV_BAR20 => Vec::<f64>::new())?)
    };
    let Ok(lf) = lake.scan(Dataset::Auction) else { return empty() };
    let from = (prev - Duration::days(40)).to_string();
    let out = lf
        .filter(col(base::DATE).gt_eq(lit(from)).and(col(base::DATE).lt_eq(lit(prev.to_string()))))
        .sort_by_exprs([col(base::CODE), col(base::DATE), col(base::TS_MS)], Default::default())
        .group_by([col(base::CODE), col(base::DATE)])
        .agg([col(cols::IEV).last().alias("__iev_d")])
        .group_by([col(base::CODE)])
        .agg([col("__iev_d").mean().alias(cols::IEV_BAR20)])
        .collect();
    match out {
        Ok(df) => Ok(df),
        Err(e) => {
            tracing::warn!(error = %e, "auction history unreadable; IEV_BAR20 degrades to null");
            empty()
        }
    }
}

fn ivu_frame(bars: LazyFrame, cfg: &StrategyCfg, prev_s: &str) -> Result<DataFrame, EngineError> {
    let ivu = hkq_factors::realized::ivu_daily(bars, &cfg.factors);
    Ok(hkq_factors::realized::ivu_terciles(ivu, cfg.factors.ivu_tercile_window)
        .filter(col(base::DATE).eq(lit(prev_s.to_string())))
        .select([col(base::CODE), col(cols::IVU_TERCILE)])
        .collect()?)
}

/// X6 (§3.3): within-sector leader graph, entirely t−1 information ⇒ built here.
/// Leaders = top-quintile float-cap; propagation on standardized r_id at t−1.
fn x6_frame(enriched: &DataFrame, prev_s: &str, fdr_q: f64) -> Result<DataFrame, EngineError> {
    let zid = when(col(cols::SIGMA_ID).gt(lit(0.0)))
        .then(col(cols::R_ID) / col(cols::SIGMA_ID))
        .otherwise(lit(NULL));
    let panel = enriched.clone().lazy()
        .select([col(base::DATE), col(base::CODE), col(cols::SECTOR),
                 col(cols::FLOAT_CAP), zid.alias("__zid")])
        .collect()?;
    let parts = panel.partition_by([cols::SECTOR], true)?;

    let (mut out_code, mut out_x6) = (Vec::new(), Vec::new());
    for part in parts {
        let (dates, keys, mut m) = panel_to_matrix(&part, base::DATE, base::CODE, "__zid")?;
        let k = keys.len();
        if k < 2 {
            for c in keys { out_code.push(c); out_x6.push(0.0); }
            continue;
        }
        demean_rows(&mut m);
        let g = lagged_corr_fdr(&m, fdr_q);
        let x: Vec<f64> = match dates.iter().position(|d| d == prev_s) {
            Some(i) => (0..k).map(|j| m[(i, j)]).collect(),
            None => vec![0.0; k],
        };
        // Leaders: top-quintile float cap (≥1 name).
        let fc_ca = part.column(cols::FLOAT_CAP)?.as_materialized_series().f64()?.clone();
        let code_ca = part.column(base::CODE)?.as_materialized_series().u32()?.clone();
        let mut fc_map: BTreeMap<u32, f64> = BTreeMap::new();
        for i in 0..part.height() {
            if let (Some(c), Some(f)) = (code_ca.get(i), fc_ca.get(i)) {
                fc_map.insert(c, f);
            }
        }
        let mut ranked: Vec<(u32, f64)> =
            keys.iter().map(|c| (*c, fc_map.get(c).copied().unwrap_or(0.0))).collect();
        ranked.sort_by(|a, b| b.1.total_cmp(&a.1));
        let n_lead = (k + 4) / 5;
        let leaders: std::collections::HashSet<u32> =
            ranked[..n_lead.min(k)].iter().map(|(c, _)| *c).collect();
        let mask: Vec<bool> = keys.iter().map(|c| leaders.contains(c)).collect();
        let s = g.propagate_masked(&x, &mask);
        for (j, c) in keys.iter().enumerate() {
            out_code.push(*c);
            out_x6.push(s[j]);
        }
    }
    Ok(df!(base::CODE => out_code, cols::X6_SPILLOVER => out_x6)?)
}

/// Stock-level southbound z (X7 raw material): 60d rolling z of net_buy,
/// shifted one day (t−1's print vs its OWN trailing history), sliced at t−1.
fn sb_z_frame(lake: &Lake, from: NaiveDate, prev_s: &str) -> Result<DataFrame, EngineError> {
    let empty = || -> Result<DataFrame, EngineError> {
        Ok(df!(base::CODE => Vec::<u32>::new(), cols::SB_Z => Vec::<f64>::new())?)
    };
    let Ok(lf) = lake.scan(Dataset::Flows) else { return empty() };
    let ro = hkq_factors::moments::roll(60);
    let mu = col(base::NET_BUY).rolling_mean(ro.clone()).shift(lit(1)).over([col(base::CODE)]);
    let sd = col(base::NET_BUY).rolling_std(ro).shift(lit(1)).over([col(base::CODE)]);
    let out = lf
        .filter(col(base::DATE).gt_eq(lit(from.to_string()))
            .and(col(base::DATE).lt_eq(lit(prev_s.to_string()))))
        .sort_by_exprs([col(base::CODE), col(base::DATE)], Default::default())
        .with_column(
            when(sd.clone().gt(lit(0.0)))
                .then((col(base::NET_BUY) - mu) / sd)
                .otherwise(lit(NULL))
                .alias(cols::SB_Z),
        )
        .filter(col(base::DATE).eq(lit(prev_s.to_string())))
        .select([col(base::CODE), col(cols::SB_Z)])
        .collect();
    match out {
        Ok(df) => Ok(df),
        Err(e) => {
            tracing::warn!(error = %e, "flows unreadable; sb_z degrades to null");
            empty()
        }
    }
}

fn sector_nightly_frame(
    enriched: &DataFrame, bars: LazyFrame, lake: &Lake, cfg: &StrategyCfg,
    from: NaiveDate, prev: NaiveDate,
) -> Result<DataFrame, EngineError> {
    let cap = cfg.stage1.member_weight_cap;
    let sector_daily = sector::aggregate_sector_returns(enriched.clone(), cap)?;

    // Late-window member returns → sector aggregate (weighted means; the S3
    // ratio V/V̄ is invariant to the mean-vs-sum choice given one convention).
    let late_members = hkq_factors::realized::late_window_member(bars)
        .join(
            enriched.clone().lazy().select([
                col(base::CODE), col(base::DATE), col(cols::SECTOR), col(cols::FLOAT_CAP),
            ]),
            [col(base::CODE), col(base::DATE)],
            [col(base::CODE), col(base::DATE)],
            JoinArgs::new(JoinType::Inner),
        )
        .collect()?;
    let sector_late = sector::weighted_sector_agg(
        late_members, &[cols::R_LATE, cols::V_LATE], cap)?
        .lazy()
        .select([col(base::DATE), col(cols::SECTOR), col(cols::R_LATE), col(cols::V_LATE)])
        .collect()?;

    let panel = sector::sector_nightly_panel(
        sector_daily.clone().lazy(), sector_late.lazy(), &cfg.factors);
    let snap = sector::nightly_snapshot_for(panel.clone(), prev).collect()?;

    // S4: cross-sector lead–lag on standardized, CS-demeaned r_id.
    let zpanel = panel
        .with_column(
            when(col(cols::SIGMA_ID).gt(lit(0.0)))
                .then(col(cols::R_ID) / col(cols::SIGMA_ID))
                .otherwise(lit(NULL))
                .alias("__zid"),
        )
        .select([col(base::DATE), col(cols::SECTOR), col("__zid")])
        .collect()?;
    let (dates, keys, mut m) = panel_to_matrix(&zpanel, base::DATE, cols::SECTOR, "__zid")?;
    demean_rows(&mut m);
    let g = lagged_corr_fdr(&m, cfg.stage1.fdr_q);
    let prev_s = prev.to_string();
    let s4 = match dates.iter().position(|d| *d == prev_s) {
        Some(i) => g.propagate(&(0..keys.len()).map(|j| m[(i, j)]).collect::<Vec<_>>()),
        None => vec![0.0; keys.len()],
    };
    let s4_df = df!(cols::SECTOR => keys.clone(), cols::S4 => s4)?;

    // S5: sector-aggregated southbound z at t−1 (null-degrades without flows).
    let member_map = enriched.clone().lazy()
        .filter(col(base::DATE).eq(lit(prev_s.clone())))
        .select([col(base::CODE), col(cols::SECTOR), col(cols::FLOAT_CAP)])
        .collect()?;
    let s5_df = match lake.scan(Dataset::Flows) {
        Ok(flows) => {
            let flows = flows.filter(
                col(base::DATE).gt_eq(lit(from.to_string()))
                    .and(col(base::DATE).lt_eq(lit(prev_s.clone()))));
            match sector::s5_sector_flow(flows, member_map, 60, cap) {
                Ok(df) => df.lazy()
                    .filter(col(base::DATE).eq(lit(prev_s.clone())))
                    .select([col(cols::SECTOR), col(cols::S5)])
                    .collect()?,
                Err(e) => {
                    tracing::warn!(error = %e, "s5 aggregation failed; degrades to null");
                    df!(cols::SECTOR => Vec::<u32>::new(), cols::S5 => Vec::<f64>::new())?
                }
            }
        }
        Err(_) => df!(cols::SECTOR => Vec::<u32>::new(), cols::S5 => Vec::<f64>::new())?,
    };

    Ok(snap.lazy()
        .join(s4_df.lazy(), [col(cols::SECTOR)], [col(cols::SECTOR)],
              JoinArgs::new(JoinType::Left))
        .join(s5_df.lazy(), [col(cols::SECTOR)], [col(cols::SECTOR)],
              JoinArgs::new(JoinType::Left))
        .select([
            col(cols::SECTOR), col(cols::R_ID_1), col(cols::R_ON_1),
            col(cols::SIGMA_ID), col(cols::SIGMA_ON),
            col(cols::S3), col(cols::S4), col(cols::S5),
        ])
        .collect()?)
}

fn load_weights(lake: &Lake, cfg: &StrategyCfg) -> (FactorWeights, FactorWeights) {
    let equal = || (FactorWeights::equal(&S_FACTORS), FactorWeights::equal(&X_FACTORS));
    let Ok(lf) = lake.scan(Dataset::Attribution) else {
        tracing::info!("no attribution history: ICIR weights start EQUAL (cold start)");
        return equal();
    };
    let Ok(panel) = lf.sort_by_exprs([col(base::DATE)], Default::default()).collect() else {
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

fn load_alpha(lake_root: &Path) -> AlphaMap {
    let path = alpha_state_path(lake_root);
    match std::fs::read(&path) {
        Ok(bytes) => match serde_json::from_slice::<AlphaMap>(&bytes) {
            Ok(a) => { tracing::info!(buckets = a.coef.len(), "alpha map loaded"); a }
            Err(e) => { tracing::error!(error = %e, "alpha map corrupt; CONSERVATIVE default"); AlphaMap::default() }
        },
        Err(_) => {
            tracing::warn!("no alpha map: default predicts 0 bps ⇒ engine trades NOTHING (shadow stance)");
            AlphaMap::default()
        }
    }
}

fn build_nums(
    stock: &DataFrame, statics: &BTreeMap<StockCode, StaticRow>,
) -> Result<BTreeMap<StockCode, StockNums>, EngineError> {
    let g = |n: &str| stock.column(n).and_then(|c| Ok(c.as_materialized_series().f64()?.clone()));
    let code = stock.column(base::CODE)?.as_materialized_series().u32()?.clone();
    let (pc, son, scc, s15, lav, adv, spr) = (
        g(cols::PREV_CLOSE)?, g(cols::SIGMA_ON)?, g(cols::SIGMA_CC)?,
        g(ecols::SIGMA_15M)?, g(cols::LAV)?, g(cols::ADV_SHARES)?, g(ecols::SPREAD_BPS)?,
    );
    let mut out = BTreeMap::new();
    for i in 0..stock.height() {
        let Some(c) = code.get(i) else { continue };
        let sc = StockCode(c);
        let Some(st) = statics.get(&sc) else { continue };
        let Some(prev_close) = pc.get(i).filter(|v| v.is_finite() && *v > 0.0) else { continue };
        let sector = u16::try_from(st.sector)
            .map_err(|_| EngineError::Contract("sector id exceeds u16"))?;
        out.insert(sc, StockNums {
            sector: SectorId(sector),
            lot: BoardLot(st.lot),
            connect: st.connect,
            prev_close,
            sigma_on: son.get(i),
            sigma_cc: scc.get(i),
            sigma_15m: s15.get(i),
            lav: lav.get(i),
            adv_shares: adv.get(i),
            spread_bps: st.spread,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hkq_core::config::*;
    use hkq_core::session::DayKind;

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
                if self.day_kind(c) != DayKind::Closed { return c; }
            }
        }
    }

    fn test_cfg(root: &Path, static_path: &Path) -> StrategyCfg {
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
                universe_static_path: Some(static_path.into()), log_json: false },
        }
    }

    #[test]
    fn premarket_assembles_from_a_minimal_lake() {
        use hkq_core::session::hk;
        use chrono::NaiveTime;
        let root = std::env::temp_dir().join(format!(
            "hkq_engine_pm_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&root).unwrap();
        let lake = Lake::new(&root);
        let cal = FixedCal;
        let date = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap(); // Friday

        // Static file: two names, two sectors.
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

        // 10 weekday partitions of daily bars + 1m bars.
        let mut d = NaiveDate::from_ymd_opt(2026, 6, 19).unwrap();
        let mut days = vec![];
        while days.len() < 10 {
            if cal.day_kind(d) == DayKind::Full { days.push(d); }
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

            let ts = |h: u32, m: u32| hk(*day, NaiveTime::from_hms_opt(h, m, 0).unwrap())
                .timestamp_millis();
            let mk = |c: u32, base_px: f64| -> DataFrame {
                df!(
                    "code" => vec![c; 4],
                    "date" => vec![day.to_string(); 4],
                    "ts_ms" => vec![ts(9, 30), ts(9, 35), ts(9, 40), ts(14, 30)],
                    "o" => vec![base_px; 4], "h" => vec![base_px * 1.01; 4],
                    "l" => vec![base_px * 0.99; 4],
                    "c" => vec![base_px, base_px * 1.001, base_px * 0.999, base_px * 1.002],
                    "volume" => vec![1000.0; 4],
                    "turnover" => vec![base_px * 1000.0; 4],
                ).unwrap()
            };
            let mut bars = mk(700, px);
            bars.vstack_mut(&mk(5, 60.0)).unwrap();
            lake.write_partition(Dataset::Bars1m, *day, &mut bars, "test", 1).unwrap();
        }

        let cfg = test_cfg(&root, &static_path);
        let state = NightlyState::load(&lake, &cfg, date, &cal).unwrap();
        assert_eq!(state.stock.height(), 2);
        for c in ["prev_close", "sigma_on", "beta_sector", "sigma_15m",
                  "x6_spillover", "ivu_tercile", "iev_bar20", "sb_z", "lav"] {
            assert!(state.stock.get_column_names().iter().any(|n| n.as_str() == c),
                    "missing stock col {c}");
        }
        assert_eq!(state.sector_nightly.height(), 2);
        for c in ["r_id_1", "r_on_1", "sigma_id", "sigma_on", "s3", "s4", "s5"] {
            assert!(state.sector_nightly.get_column_names().iter()
                .any(|n| n.as_str() == c), "missing sector col {c}");
        }
        let nums = state.nums.get(&StockCode(700)).unwrap();
        assert_eq!(nums.lot.0, 100);
        assert!(nums.prev_close > 0.0);
        // Cold-start learning state: equal weights, conservative alpha.
        assert!((state.weights_s.get("s1") - 1.0 / 6.0).abs() < 1e-9);
        assert_eq!(state.alpha.expected_alpha_bps(None, 1, 1.0), 0.0);
        std::fs::remove_dir_all(root).ok();
    }
}
```

```rust
// crates/hkq-engine/src/freeze.rs
//! 09:29:30 — the only place morning I/O meets factor math (report §6). Builds
//! the OpenContext frames from the MorningBoard + nightly state, runs Stage 1 →
//! Stage 2, and owns the 09:35 X3 refresh (a column swap on the frozen input,
//! not duplicated code) and 09:45 candidate construction.
use crate::cols as ecols;
use crate::error::EngineError;
use crate::morning::MorningBoard;
use crate::premarket::NightlyState;
use hkq_core::config::StrategyCfg;
use hkq_core::ids::{SectorId, StockCode};
use hkq_core::money::Cash;
use hkq_factors::cols::{self, base};
use hkq_factors::sector::weighted_sector_agg;
use hkq_factors::stage1::{sector_composite, select_sectors, OpenContext};
use hkq_factors::stage2::stage2_scores;
use hkq_risk::CostModel;
use hkq_signal::{Candidate, ConfirmationTracker};
use polars::prelude::*;
use rust_decimal::prelude::ToPrimitive;

pub struct Frozen {
    pub scored_sectors: DataFrame,
    pub selected: Vec<SectorId>,
    /// Stage-2 input, retained for the 09:35 X3 column swap.
    pub stage2_input: DataFrame,
    pub stage2: DataFrame,
}

/// Member-level auction frame → weighted_sector_agg → [sector, gap_z, vs_auct].
/// gap_z_i = ln(IEP/prev_close)/σ_ON; vs_i = ln(IEV/IEV̄₂₀). Absent feeds stay
/// null and degrade S2 to the pure fade (M2's tested path).
pub(crate) fn member_auction_frame(
    state: &NightlyState, board: &MorningBoard, date: chrono::NaiveDate,
) -> Result<DataFrame, EngineError> {
    let stock = &state.stock;
    let code = stock.column(base::CODE)?.as_materialized_series().u32()?.clone();
    let sec = stock.column(cols::SECTOR)?.as_materialized_series().u32()?.clone();
    let fc = stock.column(cols::FLOAT_CAP)?.as_materialized_series().f64()?.clone();
    let pc = stock.column(cols::PREV_CLOSE)?.as_materialized_series().f64()?.clone();
    let son = stock.column(cols::SIGMA_ON)?.as_materialized_series().f64()?.clone();
    let iev20 = stock.column(cols::IEV_BAR20)?.as_materialized_series().f64()?.clone();

    let n = stock.height();
    let (mut c_, mut s_, mut f_, mut gz, mut vs) =
        (Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n),
         Vec::with_capacity(n), Vec::with_capacity(n));
    for i in 0..n {
        let (Some(c), Some(s), Some(f)) = (code.get(i), sec.get(i), fc.get(i)) else { continue };
        let m = board.auction.get(&StockCode(c)).copied().unwrap_or_default();
        let gap = match (m.iep, pc.get(i), son.get(i)) {
            (Some(iep), Some(prev), Some(sig))
                if iep > 0.0 && prev > 0.0 && sig > 0.0 =>
            {
                let g = (iep / prev).ln() / sig;
                g.is_finite().then_some(g)
            }
            _ => None,
        };
        let v = match (m.iev, iev20.get(i)) {
            (Some(iev), Some(bar)) if iev > 0.0 && bar > 0.0 => {
                let v = (iev / bar).ln();
                v.is_finite().then_some(v)
            }
            _ => None,
        };
        c_.push(c);
        s_.push(s);
        f_.push(f);
        gz.push(gap);
        vs.push(v);
    }
    Ok(df!(
        base::CODE => c_, base::DATE => vec![date.to_string(); s_.len()],
        cols::SECTOR => s_, cols::FLOAT_CAP => f_,
        cols::GAP_Z => gz, cols::VS_AUCT => vs,
    )?)
}

fn sector_auction_frame(members: &DataFrame, cap: f64) -> Result<DataFrame, EngineError> {
    Ok(weighted_sector_agg(members.clone(), &[cols::GAP_Z, cols::VS_AUCT], cap)?
        .lazy()
        .select([col(cols::SECTOR), col(cols::GAP_Z), col(cols::VS_AUCT)])
        .collect()?)
}

/// S6 linked block: AH delta with the ρ = 1 prior (per-name AH betas are a
/// hkq-validate estimation job) minus whatever gap the POS already priced;
/// a50_beta_ret / adr_resid stay null until their estimators/feeds exist —
/// a constant column would FABRICATE a cross-sectional factor.
pub(crate) fn sector_linked_frame(
    state: &NightlyState, board: &MorningBoard, members: &DataFrame, cap: f64,
    date: chrono::NaiveDate,
) -> Result<DataFrame, EngineError> {
    let code = members.column(base::CODE)?.as_materialized_series().u32()?.clone();
    let sec = members.column(cols::SECTOR)?.as_materialized_series().u32()?.clone();
    let fc = members.column(cols::FLOAT_CAP)?.as_materialized_series().f64()?.clone();
    let gz = members.column(cols::GAP_Z)?.as_materialized_series().f64()?.clone();

    let stock = &state.stock;
    let scode = stock.column(base::CODE)?.as_materialized_series().u32()?.clone();
    let son = stock.column(cols::SIGMA_ON)?.as_materialized_series().f64()?.clone();
    let mut sig: std::collections::HashMap<u32, f64> = std::collections::HashMap::new();
    for i in 0..stock.height() {
        if let (Some(c), Some(s)) = (scode.get(i), son.get(i)) {
            sig.insert(c, s);
        }
    }

    let (mut s_, mut f_, mut d_) = (vec![], vec![], vec![]);
    for i in 0..members.height() {
        let (Some(c), Some(s), Some(f)) = (code.get(i), sec.get(i), fc.get(i)) else { continue };
        let Some(a_ret) = board.mainland.get(&StockCode(c)).copied() else { continue };
        // POS-priced gap in return units; 0 when the POS printed nothing.
        let priced = gz.get(i).zip(sig.get(&c).copied()).map(|(z, s)| z * s).unwrap_or(0.0);
        let delta = a_ret - priced;
        if delta.is_finite() {
            s_.push(s);
            f_.push(f);
            d_.push(delta);
        }
    }
    let ah = weighted_sector_agg(
        df!(
            base::CODE => vec![0u32; s_.len()], base::DATE => vec![date.to_string(); s_.len()],
            cols::SECTOR => s_, cols::FLOAT_CAP => f_, cols::AH_DELTA => d_,
        )?,
        &[cols::AH_DELTA],
        cap,
    )?;
    Ok(state.sector_nightly.clone().lazy()
        .select([col(cols::SECTOR)])
        .join(ah.lazy().select([col(cols::SECTOR), col(cols::AH_DELTA)]),
              [col(cols::SECTOR)], [col(cols::SECTOR)], JoinArgs::new(JoinType::Left))
        .with_columns([
            lit(NULL).cast(DataType::Float64).alias(cols::A50_BETA_RET),
            lit(NULL).cast(DataType::Float64).alias(cols::ADR_RESID),
        ])
        .collect()?)
}

fn stage2_input_frame(
    state: &NightlyState, board: &MorningBoard, sector_auction: &DataFrame,
    selected: &[SectorId],
) -> Result<DataFrame, EngineError> {
    let sel = df!(cols::SECTOR => selected.iter().map(|s| s.0 as u32).collect::<Vec<_>>())?;
    let picked = state.stock.clone().lazy()
        .join(sel.lazy(), [col(cols::SECTOR)], [col(cols::SECTOR)],
              JoinArgs::new(JoinType::Semi))
        .collect()?;

    // Morning columns, row-aligned by code.
    let code = picked.column(base::CODE)?.as_materialized_series().u32()?.clone();
    let pc = picked.column(cols::PREV_CLOSE)?.as_materialized_series().f64()?.clone();
    let son = picked.column(cols::SIGMA_ON)?.as_materialized_series().f64()?.clone();
    let n = picked.height();
    let (mut gz, mut vsi, mut i10, mut i20, mut ievr) =
        (Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n),
         Vec::with_capacity(n), Vec::with_capacity(n));
    let iev20 = picked.column(cols::IEV_BAR20)?.as_materialized_series().f64()?.clone();
    for i in 0..n {
        let m = code.get(i)
            .map(|c| board.auction.get(&StockCode(c)).copied().unwrap_or_default())
            .unwrap_or_default();
        let gap = match (m.iep, pc.get(i), son.get(i)) {
            (Some(iep), Some(prev), Some(sig)) if iep > 0.0 && prev > 0.0 && sig > 0.0 => {
                let g = (iep / prev).ln() / sig;
                g.is_finite().then_some(g)
            }
            _ => None,
        };
        gz.push(gap);
        vsi.push(match (m.iev, iev20.get(i)) {
            (Some(iev), Some(bar)) if iev > 0.0 && bar > 0.0 => Some((iev / bar).ln()),
            _ => None,
        });
        i10.push(m.iep_0910);
        i20.push(m.iep_0920);
        ievr.push(m.iev);
    }
    let mut out = picked;
    out.with_column(Series::new(cols::GAP_Z.into(), gz))?;
    out.with_column(Series::new(cols::VS_AUCT_I.into(), vsi))?;
    out.with_column(Series::new(cols::IEP_0910.into(), i10))?;
    out.with_column(Series::new(cols::IEP_0920.into(), i20))?;
    out.with_column(Series::new(ecols::IEV_RAW.into(), ievr))?;

    // Freeze-phase X3 inputs: auction-only surprise (raw IEV vs IEV̄₂₀).
    let out = out.lazy()
        .with_columns([
            col(ecols::IEV_RAW).alias(cols::VOL_TAU0),
            col(cols::IEV_BAR20).alias(cols::VOL_TAU0_MED20),
        ])
        .join(
            sector_auction.clone().lazy()
                .select([col(cols::SECTOR), col(cols::GAP_Z).alias(cols::GAP_Z_SECTOR)]),
            [col(cols::SECTOR)], [col(cols::SECTOR)],
            JoinArgs::new(JoinType::Left),
        )
        .collect()?;
    Ok(out)
}

/// The 09:29:30 action. `None` ⇒ no sector cleared Σ_min ⇒ CASH DAY (§3.2).
pub fn run_freeze(
    state: &NightlyState, board: &MorningBoard, date: chrono::NaiveDate, cfg: &StrategyCfg,
) -> Result<Option<Frozen>, EngineError> {
    let cap = cfg.stage1.member_weight_cap;
    let members = member_auction_frame(state, board, date)?;
    let sector_auction = sector_auction_frame(&members, cap)?;
    let sector_linked = sector_linked_frame(state, board, &members, cap, date)?;

    let ctx = OpenContext {
        date,
        sector_nightly: state.sector_nightly.clone(),
        sector_auction: sector_auction.clone(),
        sector_linked,
    };
    let scored = sector_composite(&ctx, &state.weights_s, &cfg.stage1)?;
    let selected = select_sectors(&scored, &cfg.stage1)?;
    if selected.is_empty() {
        return Ok(None);
    }
    let input = stage2_input_frame(state, board, &sector_auction, &selected)?;
    let stage2 = stage2_scores(input.clone(), &state.gate, &state.weights_x, &cfg.stage2)?;
    Ok(Some(Frozen { scored_sectors: scored, selected, stage2_input: input, stage2 }))
}

/// 09:35 X3 refresh (§3.3 two-phase): vol_tau0 ← IEV + V[09:30,09:35); baseline
/// ← IEV̄₂₀ + same-bin median. One expression, second input — then re-score the
/// FROZEN candidate set (sector selection does not reopen).
pub fn refresh_x3(
    froz: &mut Frozen, state: &NightlyState, board: &MorningBoard, cfg: &StrategyCfg,
) -> Result<(), EngineError> {
    let code = froz.stage2_input.column(base::CODE)?
        .as_materialized_series().u32()?.clone();
    let v0935: Vec<Option<f64>> = (0..froz.stage2_input.height())
        .map(|i| code.get(i).and_then(|c| board.vol_0935(StockCode(c))))
        .collect();
    let mut input = froz.stage2_input.clone();
    input.with_column(Series::new(ecols::V0935.into(), v0935))?;
    let input = input.lazy()
        .with_columns([
            (col(ecols::IEV_RAW).fill_null(lit(0.0)) + col(ecols::V0935).fill_null(lit(0.0)))
                .alias(cols::VOL_TAU0),
            (col(cols::IEV_BAR20).fill_null(lit(0.0)) + col(ecols::VMED_0930).fill_null(lit(0.0)))
                .alias(cols::VOL_TAU0_MED20),
        ])
        .collect()?;
    froz.stage2 = stage2_scores(input.clone(), &state.gate, &state.weights_x, &cfg.stage2)?;
    froz.stage2_input = input;
    Ok(())
}

/// 09:45 candidate assembly. The tentative size for the cost floor is the
/// M3-documented equity/(K·m) participation estimate; hkq-risk re-checks the
/// invariant after size_book.
pub fn build_candidates(
    froz: &Frozen, state: &NightlyState, tracker: &ConfirmationTracker,
    cost: &CostModel, equity: Cash, cfg: &StrategyCfg,
) -> Result<Vec<Candidate>, EngineError> {
    let df = &froz.stage2;
    let code = df.column(base::CODE)?.as_materialized_series().u32()?.clone();
    let score = df.column(cols::SCORE)?.as_materialized_series().f64()?.clone();
    let ivu = df.column(cols::IVU_TERCILE)?.as_materialized_series().u32()?.clone();

    let k = froz.selected.len().max(1);
    let m = cfg.stage2.names_per_sector.max(1);
    let slot_cash = equity.0.to_f64().unwrap_or(0.0) / (k * m) as f64;

    let mut out = Vec::with_capacity(df.height());
    for i in 0..df.height() {
        let (Some(c), Some(s)) = (code.get(i), score.get(i)) else { continue };
        let sc = StockCode(c);
        let Some(nums) = state.nums.get(&sc) else { continue };
        let q_shares = (slot_cash / nums.prev_close).max(0.0);
        let floor = cost.floor_bps(
            nums.spread_bps.unwrap_or(cfg.universe.max_median_spread_bps),
            nums.sigma_cc.unwrap_or(0.0),
            q_shares,
            nums.adv_shares.unwrap_or(1.0),
        );
        out.push(Candidate {
            code: sc,
            sector: nums.sector,
            score: s,
            sigma_15m: nums.sigma_15m.unwrap_or(f64::NAN),
            lav: nums.lav.unwrap_or(f64::NAN),
            cost_floor_bps: floor,
            r_0930_0945: tracker.window_return(sc),
            vhsi_tercile: None, // VHSI feed absent ⇒ AlphaMap global bucket (M3)
            ivu_tercile: ivu.get(i).unwrap_or(1).min(2) as u8,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morning::MorningBoard;
    use hkq_core::config::*;
    use hkq_core::money::BoardLot;
    use hkq_core::session::hk;
    use hkq_data::model::AuctionSnap;
    use hkq_factors::icir::FactorWeights;
    use hkq_factors::stage2::RegimeGate;
    use hkq_signal::AlphaMap;
    use chrono::{NaiveDate, NaiveTime};
    use std::collections::BTreeMap;

    fn cfg() -> StrategyCfg {
        StrategyCfg {
            universe: UniverseCfg { min_median_turnover_hkd: 0.0, min_price_hkd: 0.0,
                min_listed_days: 0, max_median_spread_bps: 35.0 },
            factors: FactorCfg { ewma_halflife_days: 21.0, ewma_min_obs: 2, amihud_window: 3,
                rv_days: 2, lav_gamma: 0.3, seasonal_vol_days: 2, ivu_tercile_window: 4 },
            stage1: Stage1Cfg { theta1: 1.0, theta2: 1.0, eta: 0.25, vs_threshold: 0.5,
                leadlag_window: 250, fdr_q: 0.10, icir_window: 250, icir_shrink_delta: 0.10,
                top_k_sectors: 1, sigma_min_gate: 0.05, member_weight_cap: 0.9 },
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
            ops: OpsCfg { lake_root: "/tmp".into(), calendar_path: "/tmp".into(),
                universe_codes_path: "/tmp".into(), ah_map_path: None,
                universe_static_path: None, log_json: false },
        }
    }

    fn state() -> NightlyState {
        let stock = df!(
            "code" => vec![700u32, 5, 941, 1299],
            "sector" => vec![1u32, 1, 2, 2],
            "float_cap" => vec![100.0, 50.0, 80.0, 60.0],
            "connect_elig" => vec![1.0f64, 1.0, 1.0, 1.0],
            "spread_bps" => vec![Some(5.0); 4],
            "prev_close" => vec![100.0, 60.0, 70.0, 50.0],
            "sigma_on" => vec![0.01f64; 4],
            "sigma_id" => vec![0.01f64; 4],
            "sigma_cc" => vec![0.015f64; 4],
            "rv_5d" => vec![0.0001f64; 4],
            "lav" => vec![0.1f64; 4],
            "adv_shares" => vec![1.0e6f64; 4],
            "rsj_1" => vec![0.0f64; 4],
            "jump_1" => vec![0.0f64; 4],
            "r_id_1" => vec![0.01f64, 0.0, -0.01, 0.0],
            "r_on_1" => vec![0.0f64; 4],
            "beta_sector" => vec![1.0f64; 4],
            "sigma_15m" => vec![0.01f64; 4],
            "v_bin_med_0930" => vec![Some(1000.0); 4],
            "iev_bar20" => vec![Some(1.0e5); 4],
            "ivu_tercile" => vec![1u32; 4],
            "x6_spillover" => vec![0.0f64; 4],
            "sb_z" => vec![0.0f64; 4],
        ).unwrap();
        let sector_nightly = df!(
            "sector" => vec![1u32, 2],
            "r_id_1" => vec![0.02, -0.02],
            "r_on_1" => vec![-0.005, 0.005],
            "sigma_id" => vec![0.01, 0.01],
            "sigma_on" => vec![0.01, 0.01],
            "s3" => vec![Some(0.5), Some(-0.5)],
            "s4" => vec![Some(0.2), Some(-0.2)],
            "s5" => vec![Some(0.5), Some(-0.5)],
        ).unwrap();
        let mut nums = BTreeMap::new();
        for (c, sec, pc) in [(700u32, 1u16, 100.0), (5, 1, 60.0), (941, 2, 70.0), (1299, 2, 50.0)] {
            nums.insert(StockCode(c), crate::premarket::StockNums {
                sector: SectorId(sec), lot: BoardLot(100), connect: true, prev_close: pc,
                sigma_on: Some(0.01), sigma_cc: Some(0.015), sigma_15m: Some(0.01),
                lav: Some(0.1), adv_shares: Some(1.0e6), spread_bps: Some(5.0),
            });
        }
        NightlyState {
            date: NaiveDate::from_ymd_opt(2026, 7, 3).unwrap(),
            prev: NaiveDate::from_ymd_opt(2026, 7, 2).unwrap(),
            stock, sector_nightly, nums,
            weights_s: FactorWeights::equal(&crate::premarket::S_FACTORS),
            weights_x: FactorWeights::equal(&crate::premarket::X_FACTORS),
            alpha: AlphaMap::flat(100.0),
            gate: RegimeGate::default(),
        }
    }

    fn board() -> MorningBoard {
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        let mut b = MorningBoard::default();
        for (c, iep, iev) in [(700u32, 101.0, 3.0e5), (5, 60.3, 2.5e5),
                              (941, 69.5, 1.0e5), (1299, 49.8, 9.0e4)] {
            b.absorb_auction(&AuctionSnap {
                code: StockCode(c),
                ts: hk(d, NaiveTime::from_hms_opt(9, 21, 0).unwrap()),
                iep: Some(iep), iev: Some(iev), bid_qty: None, ask_qty: None,
            });
        }
        b
    }

    #[test]
    fn freeze_selects_scores_and_builds_candidates() {
        let st = state();
        let bd = board();
        let cfg = cfg();
        let d = st.date;
        let froz = run_freeze(&st, &bd, d, &cfg).unwrap().expect("not a cash day");
        // Sector 1: traded continuation + confirmed positive gaps ⇒ must win.
        assert_eq!(froz.selected, vec![SectorId(1)]);
        assert_eq!(froz.stage2.height(), 2); // members of sector 1 only
        assert!(froz.stage2.column("score").is_ok());

        let cost = CostModel::from_cfg(&cfg.costs).unwrap();
        let mut tracker = ConfirmationTracker::new();
        tracker.absorb(StockCode(700), NaiveTime::from_hms_opt(9, 31, 0).unwrap(), 101.0);
        tracker.absorb(StockCode(700), NaiveTime::from_hms_opt(9, 44, 0).unwrap(), 102.0);
        let cands = build_candidates(
            &froz, &st, &tracker, &cost,
            Cash(rust_decimal::Decimal::from(1_000_000)), &cfg).unwrap();
        assert_eq!(cands.len(), 2);
        let c700 = cands.iter().find(|c| c.code == StockCode(700)).unwrap();
        assert!(c700.r_0930_0945.unwrap() > 0.0);
        assert!(c700.cost_floor_bps > 22.2); // 2·stamp + fees + spread at least
        let c5 = cands.iter().find(|c| c.code == StockCode(5)).unwrap();
        assert!(c5.r_0930_0945.is_none()); // no bars ⇒ no confirmation data
    }

    #[test]
    fn x3_refresh_swaps_volume_columns() {
        let st = state();
        let mut bd = board();
        let cfg = cfg();
        let mut froz = run_freeze(&st, &bd, st.date, &cfg).unwrap().unwrap();
        bd.absorb_open_bar(StockCode(700), NaiveTime::from_hms_opt(9, 31, 0).unwrap(), 5000.0);
        refresh_x3(&mut froz, &st, &bd, &cfg).unwrap();
        let vt = froz.stage2_input.column("vol_tau0").unwrap()
            .as_materialized_series().f64().unwrap().clone();
        let code = froz.stage2_input.column("code").unwrap()
            .as_materialized_series().u32().unwrap().clone();
        let i700 = (0..code.len()).find(|i| code.get(*i) == Some(700)).unwrap();
        assert!((vt.get(i700).unwrap() - (3.0e5 + 5000.0)).abs() < 1e-9);
    }

    #[test]
    fn cash_day_when_gate_unreachable() {
        let st = state();
        let bd = board();
        let mut c = cfg();
        c.stage1.sigma_min_gate = 999.0;
        assert!(run_freeze(&st, &bd, st.date, &c).unwrap().is_none());
    }
}
```

```rust
// crates/hkq-engine/src/day.rs
//! The trading-day actor: a clock and a socket around tested functions.
//! Single writer of book state; every decision delegated to M2/M3 crates.
use crate::book::Book;
use crate::error::EngineError;
use crate::freeze::{self, Frozen};
use crate::morning::MorningBoard;
use crate::premarket::{NightlyState, X_FACTORS, S_FACTORS, ALPHA_MIN_OBS, alpha_state_path};
use crate::schedule::{build_schedule, Action, Phase};
use hkq_core::config::{HalfDayMode, StrategyCfg};
use hkq_core::ids::StockCode;
use hkq_core::money::{Cash, Px};
use hkq_core::session::{DayKind, SessionTimes, TradingCalendar};
use hkq_data::lake::{Dataset, Lake};
use hkq_data::model::MarketEvent;
use hkq_exec::{ExecCmd, Fill, Pacing, ParentOrder, Side};
use hkq_factors::cols::{self, base};
use hkq_risk::{size_book, RiskState, SizingInput, StopBook};
use hkq_signal::attribution::{daily_ic_row, realized_window_returns, R_FWD};
use hkq_signal::{entry_filter, AlphaMap, ConfirmationTracker};
use polars::prelude::*;
use std::collections::HashSet;
use tokio::sync::{mpsc, watch};

#[derive(Debug, Clone)]
pub struct RunCfg {
    /// Starting cash — a CLI argument, deliberately NOT a TradeCfg field
    /// (adding one would break M3's literal constructions in sibling tests).
    pub equity: Cash,
}

pub struct Channels {
    pub md_rx: mpsc::Receiver<MarketEvent>,
    pub fill_rx: mpsc::Receiver<Fill>,
    pub kill_rx: watch::Receiver<RiskState>,
}

pub struct TradingDay {
    pub(crate) cfg: StrategyCfg,
    pub(crate) date: chrono::NaiveDate,
    pub(crate) lake: Lake,
    pub(crate) state: NightlyState,
    pub(crate) board: MorningBoard,
    pub(crate) tracker: ConfirmationTracker,
    pub(crate) frozen: Option<Frozen>,
    pub(crate) book: Book,
    pub(crate) stops: StopBook,
    pub(crate) exiting: HashSet<StockCode>,
    pub(crate) exec_tx: mpsc::Sender<ExecCmd>,
    pub(crate) phase: Phase,
    pub(crate) post_close_done: bool,
    // Today's 1m bars for PostClose attribution (the live copy of Bars1m).
    b_code: Vec<u32>, b_ts: Vec<i64>, b_c: Vec<f64>, b_v: Vec<f64>,
}

impl TradingDay {
    pub fn new(
        cfg: StrategyCfg, run: RunCfg, date: chrono::NaiveDate, lake: Lake,
        state: NightlyState, exec_tx: mpsc::Sender<ExecCmd>,
    ) -> Result<Self, EngineError> {
        let cost = hkq_risk::CostModel::from_cfg(&cfg.costs)?;
        Ok(Self {
            cfg, date, lake, state,
            board: MorningBoard::default(),
            tracker: ConfirmationTracker::new(),
            frozen: None,
            book: Book::new(run.equity, cost),
            stops: StopBook::default(),
            exiting: HashSet::new(),
            exec_tx,
            phase: Phase::PreMarket,
            post_close_done: false,
            b_code: vec![], b_ts: vec![], b_c: vec![], b_v: vec![],
        })
    }

    /// The blueprint loop: biased kill pre-emption, then the schedule timer,
    /// then market data, then fills.
    pub async fn run_day(
        mut self, cal: &dyn TradingCalendar, mut ch: Channels,
    ) -> Result<(), EngineError> {
        let kind = cal.day_kind(self.date);
        let sched = build_schedule(kind, self.cfg.trade.half_day_mode);
        if sched.is_empty() {
            tracing::info!(date = %self.date, ?kind, "no session today (closed or half-day skip)");
            return Ok(());
        }
        let mut idx = 0usize;
        let mut timer = Box::pin(tokio::time::sleep_until(instant_for(self.date, sched[0].0)));

        loop {
            tokio::select! {
                biased;
                changed = ch.kill_rx.changed() => {
                    if changed.is_ok() && ch.kill_rx.borrow().halted() {
                        tracing::error!("KILL SWITCH observed — cancel, flatten, post-close");
                        self.halt_flatten(&mut ch).await?;
                        break;
                    }
                }
                _ = &mut timer => {
                    let t = sched[idx].0;
                    while idx < sched.len() && sched[idx].0 == t {
                        let a = sched[idx].1;
                        self.act(a).await?;
                        idx += 1;
                    }
                    if idx >= sched.len() {
                        break;
                    }
                    timer.as_mut().reset(instant_for(self.date, sched[idx].0));
                }
                ev = ch.md_rx.recv() => {
                    match ev {
                        Some(ev) => self.on_event(ev).await?,
                        None => tracing::warn!("market-data channel closed"),
                    }
                }
                fill = ch.fill_rx.recv() => {
                    if let Some(f) = fill {
                        self.on_fill(&f);
                    }
                }
            }
        }
        if !self.post_close_done {
            self.post_close().await?;
        }
        // Terminal invariant (report scope): zero overnight inventory. Loud.
        if !self.book.is_flat() {
            return Err(EngineError::State(
                "book not flat after CAS — manual intervention required".into()));
        }
        tracing::info!(equity = self.book.marked_equity(), fills = self.book.n_fills(),
                       "day complete, book flat");
        Ok(())
    }

    pub(crate) async fn act(&mut self, a: Action) -> Result<(), EngineError> {
        match a {
            Action::Enter(p) => {
                tracing::info!(?p, "phase");
                self.phase = p;
                match p {
                    Phase::ScoreFreeze => self.freeze()?,
                    Phase::Entry => self.entry().await?,
                    Phase::ExitProgram => self.exit_program().await?,
                    Phase::PostClose => self.post_close().await?,
                    _ => {}
                }
            }
            Action::X3Refresh => {
                if let Some(froz) = self.frozen.as_mut() {
                    freeze::refresh_x3(froz, &self.state, &self.board, &self.cfg)?;
                    tracing::info!("X3 refreshed with first-5m volume (§3.3 two-phase)");
                }
            }
            Action::LunchCancel => {
                self.send_exec(ExecCmd::CancelAll).await?;
            }
            Action::CasFlush => {
                self.send_exec(ExecCmd::Flush).await?;
            }
        }
        Ok(())
    }

    pub(crate) async fn on_event(&mut self, ev: MarketEvent) -> Result<(), EngineError> {
        match ev {
            MarketEvent::Auction(s) => self.board.absorb_auction(&s),
            MarketEvent::MainlandAuctionPrint { code, a_open_ret } => {
                self.board.absorb_mainland(code, a_open_ret);
            }
            MarketEvent::A50 { px, .. } => self.board.absorb_a50(px),
            MarketEvent::Bar(b) => self.on_bar(&b).await?,
            MarketEvent::Vcm { code, in_cooling_off } => {
                self.send_exec(ExecCmd::Pause { code, on: in_cooling_off }).await?;
            }
            MarketEvent::Halt { code } => {
                tracing::warn!(%code, "trading halt event");
                self.send_exec(ExecCmd::Pause { code, on: true }).await?;
            }
            MarketEvent::Quote { .. } => {} // spread feed lands with the live venue (M5)
        }
        Ok(())
    }

    pub(crate) async fn on_bar(&mut self, b: &hkq_data::model::Bar1m) -> Result<(), EngineError> {
        self.book.update_mark(b.code, b.c);
        self.b_code.push(b.code.0);
        self.b_ts.push(b.ts.timestamp_millis());
        self.b_c.push(b.c);
        self.b_v.push(b.volume);
        // Pacing fuel for the exec actor.
        self.send_exec(ExecCmd::OnBar {
            code: b.code, ts_ms: b.ts.timestamp_millis(), close: b.c, volume: b.volume,
        }).await?;

        match self.phase {
            Phase::Confirmation => {
                self.tracker.absorb(b.code, b.ts.time(), b.c);
                self.board.absorb_open_bar(b.code, b.ts.time(), b.volume);
            }
            Phase::MorningHold | Phase::AfternoonHold => {
                if let Ok(low) = Px::from_f64_quote(b.l) {
                    if self.stops.breached(b.code, low) && !self.exiting.contains(&b.code) {
                        self.trigger_stop(b.code, b.c).await?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn on_fill(&mut self, f: &Fill) {
        self.book.apply_fill(f);
        if f.side == Side::Sell && self.book.position(f.code).is_none() {
            self.stops.disarm(f.code);
            self.exiting.remove(&f.code);
        }
    }

    fn freeze(&mut self) -> Result<(), EngineError> {
        match freeze::run_freeze(&self.state, &self.board, self.date, &self.cfg)? {
            Some(f) => {
                tracing::info!(selected = ?f.selected, candidates = f.stage2.height(),
                               "scores frozen");
                self.frozen = Some(f);
            }
            None => tracing::info!("Σ_min gate not cleared — CASH DAY (§3.2)"),
        }
        Ok(())
    }

    async fn entry(&mut self) -> Result<(), EngineError> {
        let Some(froz) = self.frozen.as_ref() else { return Ok(()) };
        let cost = hkq_risk::CostModel::from_cfg(&self.cfg.costs)?;
        let cands = freeze::build_candidates(
            froz, &self.state, &self.tracker, &cost, self.book.cash(), &self.cfg)?;
        let decision = entry_filter(
            &cands, &self.state.alpha, self.cfg.stage2.names_per_sector, &self.cfg.trade);
        for (c, r) in &decision.rejected {
            tracing::info!(code = %c.code, reason = ?r, "candidate rejected");
        }
        if decision.accepted.is_empty() {
            tracing::info!("no candidate cleared the three-fold gate — cash day at entry");
            return Ok(());
        }

        let mut inputs = Vec::with_capacity(decision.accepted.len());
        for c in &decision.accepted {
            let Some(nums) = self.state.nums.get(&c.code) else { continue };
            let ref_v = self.book.mark(c.code).unwrap_or(nums.prev_close);
            let Ok(ref_px) = Px::from_f64_quote(ref_v) else {
                tracing::warn!(code = %c.code, "invalid reference price; dropped");
                continue;
            };
            // Hour-ahead projection = 4 × realized confirmation-window volume;
            // the pacer enforces the true per-bar cap regardless (see M4 notes).
            let projected = self.board.vol_0945(c.code).unwrap_or(0.0) * 4.0;
            inputs.push(SizingInput {
                code: c.code, ref_px, lot: nums.lot,
                lav: c.lav, sigma_15m: c.sigma_15m,
                projected_interval_vol: projected,
            });
        }
        let (orders, residual) = size_book(self.book.cash(), &inputs, &self.cfg.trade);
        tracing::info!(orders = orders.len(), residual = %residual.0, "book sized");
        for o in &orders {
            self.stops.arm(o.code, o.stop);
            let lot = self.state.nums.get(&o.code).map(|n| n.lot.0).unwrap_or(1);
            let Some(parent) = ParentOrder::new(o.code, Side::Buy, o.qty.shares(), lot, o.limit)
            else { continue };
            self.send_exec(ExecCmd::Place { order: parent, pacing: Pacing::Paced }).await?;
        }
        Ok(())
    }

    async fn trigger_stop(&mut self, code: StockCode, last_close: f64) -> Result<(), EngineError> {
        let Some(pos) = self.book.position(code) else { return Ok(()) };
        tracing::warn!(%code, shares = pos.shares, "protective stop TRIGGERED");
        self.stops.disarm(code);
        self.exiting.insert(code);
        self.send_exec(ExecCmd::CancelSymbol(code)).await?;
        let limit = Px::from_f64_quote(last_close)
            .map(|p| p.snap_down_to_tick())
            .unwrap_or_else(|_| Px::from_f64_quote(0.01).expect("min px"));
        if let Some(parent) = ParentOrder::new(code, Side::Sell, pos.shares, pos.lot, limit) {
            self.send_exec(ExecCmd::Place { order: parent, pacing: Pacing::Immediate }).await?;
        }
        Ok(())
    }

    async fn exit_program(&mut self) -> Result<(), EngineError> {
        // Stops stand down: a stop firing mid-exit would double-sell.
        for (code, _) in self.book.open_positions() {
            self.stops.disarm(code);
        }
        for (code, pos) in self.book.open_positions() {
            if self.exiting.contains(&code) {
                continue;
            }
            self.exiting.insert(code);
            let mark = self.book.mark(code)
                .or_else(|| self.state.nums.get(&code).map(|n| n.prev_close))
                .unwrap_or(0.01);
            let limit = Px::from_f64_quote(mark)
                .map(|p| p.snap_down_to_tick())
                .unwrap_or_else(|_| Px::from_f64_quote(0.01).expect("min px"));
            if let Some(parent) = ParentOrder::new(code, Side::Sell, pos.shares, pos.lot, limit) {
                self.send_exec(ExecCmd::Place { order: parent, pacing: Pacing::Paced }).await?;
            }
        }
        Ok(())
    }

    async fn halt_flatten(&mut self, ch: &mut Channels) -> Result<(), EngineError> {
        self.send_exec(ExecCmd::CancelAll).await?;
        for (code, pos) in self.book.open_positions() {
            let mark = self.book.mark(code)
                .or_else(|| self.state.nums.get(&code).map(|n| n.prev_close))
                .unwrap_or(0.01);
            let limit = Px::from_f64_quote(mark)
                .map(|p| p.snap_down_to_tick())
                .unwrap_or_else(|_| Px::from_f64_quote(0.01).expect("min px"));
            if let Some(parent) = ParentOrder::new(code, Side::Sell, pos.shares, pos.lot, limit) {
                self.send_exec(ExecCmd::Place { order: parent, pacing: Pacing::Immediate }).await?;
            }
        }
        // Drain fills briefly; the paper venue answers immediately.
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
        while !self.book.is_flat() {
            match tokio::time::timeout_at(deadline, ch.fill_rx.recv()).await {
                Ok(Some(f)) => self.on_fill(&f),
                _ => break,
            }
        }
        self.post_close().await
    }

    /// PostClose (§6): persist what cannot be recomputed, then close both
    /// learning loops — the IC panel row (both stages) and the AlphaMap refit.
    pub(crate) async fn post_close(&mut self) -> Result<(), EngineError> {
        if self.post_close_done {
            return Ok(());
        }
        self.post_close_done = true;
        self.phase = Phase::PostClose;

        if self.board.n_snaps() > 0 {
            let mut df = self.board.auction_frame(self.date)?;
            self.lake.write_partition(Dataset::Auction, self.date, &mut df, "engine:pos", 1)?;
        }
        if self.book.n_fills() > 0 {
            let mut df = self.book.fills_frame(self.date)?;
            self.lake.write_partition(Dataset::Fills, self.date, &mut df, "engine:fills", 1)?;
        }

        let Some(froz) = self.frozen.as_ref() else {
            tracing::info!("cash day: no scores, no attribution row");
            return Ok(());
        };

        // Live-frozen stage-2 frame → Dataset::Scores (the AlphaMap's raw material).
        let mut scores = froz.stage2.clone();
        let _ = scores.drop_in_place("__cs"); // pipeline helper column, if present
        let n = scores.height();
        scores.with_column(Series::new(base::DATE.into(), vec![self.date.to_string(); n]))?;
        self.lake.write_partition(Dataset::Scores, self.date, &mut scores, "engine:freeze", 1)?;

        // Attribution: realized §3.5 label from today's streamed bars.
        if self.b_code.is_empty() {
            tracing::warn!("no bars streamed today; attribution skipped");
            return Ok(());
        }
        let s = SessionTimes::get();
        let bars = df!(
            base::CODE => self.b_code.clone(),
            base::DATE => vec![self.date.to_string(); self.b_code.len()],
            base::TS_MS => self.b_ts.clone(),
            cols::C1M => self.b_c.clone(),
            base::VOLUME => self.b_v.clone(),
        )?;
        let realized = realized_window_returns(bars.lazy(), s.entry, s.exit_end).collect()?;

        let mut row = daily_ic_row(self.date, &froz.stage2, &realized, &X_FACTORS)?;

        // Sector-stage ICs in the same row: the sector id plays the `code` join
        // key (documented reuse of daily_ic_row — a cross-section is a cross-
        // section), against float-cap-aggregated member realized returns.
        let sec_map = self.state.stock.clone().lazy()
            .select([col(base::CODE), col(cols::SECTOR), col(cols::FLOAT_CAP)]);
        let sec_realized_members = realized.clone().lazy()
            .join(sec_map, [col(base::CODE)], [col(base::CODE)], JoinArgs::new(JoinType::Inner))
            .collect()?;
        let sec_realized = hkq_factors::sector::weighted_sector_agg(
            sec_realized_members, &[R_FWD], self.cfg.stage1.member_weight_cap)?;
        let sec_realized = sec_realized.lazy()
            .select([col(cols::SECTOR).alias(base::CODE), col(R_FWD)])
            .collect()?;
        let mut sec_cols = vec![col(cols::SECTOR).alias(base::CODE)];
        sec_cols.extend(S_FACTORS.iter().map(|f| col(cols::z(f).as_str())));
        let sec_scores = froz.scored_sectors.clone().lazy().select(sec_cols).collect()?;
        match daily_ic_row(self.date, &sec_scores, &sec_realized, &S_FACTORS) {
            Ok(sec_row) => {
                let extra: Vec<Column> = sec_row.get_columns().iter()
                    .filter(|c| c.name().as_str() != base::DATE)
                    .cloned()
                    .collect();
                row = row.hstack(&extra)?;
            }
            Err(e) => tracing::warn!(error = %e, "sector IC row skipped"),
        }
        self.lake.write_partition(Dataset::Attribution, self.date, &mut row, "engine:attrib", 1)?;

        // AlphaMap refit from the full scores history + realized labels.
        if let Err(e) = self.refit_alpha() {
            tracing::warn!(error = %e, "alpha refit skipped");
        }
        tracing::info!("post-close complete: scores/fills/auction persisted, IC row appended");
        Ok(())
    }

    fn refit_alpha(&self) -> Result<(), EngineError> {
        let s = SessionTimes::get();
        let scores = self.lake.scan(Dataset::Scores)?
            .select([col(base::DATE), col(base::CODE), col(cols::SCORE), col(cols::IVU_TERCILE)]);
        let realized = realized_window_returns(
            self.lake.scan(Dataset::Bars1m)?, s.entry, s.exit_end);
        let panel = scores
            .join(realized, [col(base::CODE), col(base::DATE)],
                  [col(base::CODE), col(base::DATE)], JoinArgs::new(JoinType::Inner))
            .with_column(lit(NULL).cast(DataType::UInt32)
                .alias(hkq_signal::alpha::VHSI_TERCILE))
            .collect()?;
        let fitted = AlphaMap::fit(&panel, ALPHA_MIN_OBS)?;
        if fitted.coef.is_empty() {
            tracing::info!(rows = panel.height(),
                "alpha panel below min_obs — map stays conservative");
            return Ok(());
        }
        let path = alpha_state_path(&self.cfg.ops.lake_root);
        std::fs::create_dir_all(path.parent().expect("state dir"))?;
        std::fs::write(&path, serde_json::to_vec_pretty(&fitted)?)?;
        tracing::info!(buckets = fitted.coef.len(), rows = panel.height(), "alpha map refit");
        Ok(())
    }

    async fn send_exec(&self, cmd: ExecCmd) -> Result<(), EngineError> {
        self.exec_tx.send(cmd).await.map_err(|_| EngineError::Channel("exec"))
    }
}

fn instant_for(date: chrono::NaiveDate, t: chrono::NaiveTime) -> tokio::time::Instant {
    let target = hkq_core::session::hk(date, t).with_timezone(&chrono::Utc);
    let delta = (target - chrono::Utc::now()).to_std().unwrap_or(std::time::Duration::ZERO);
    tokio::time::Instant::now() + delta
}

#[cfg(test)]
mod tests {
    use super::*;
    use hkq_core::money::BoardLot;
    use hkq_core::session::hk;
    use chrono::{NaiveDate, NaiveTime};
    use rust_decimal_macros::dec;

    fn day_with_position() -> (TradingDay, mpsc::Receiver<ExecCmd>) {
        let (exec_tx, exec_rx) = mpsc::channel(64);
        let root = std::env::temp_dir().join(format!(
            "hkq_day_test_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let cfg = crate::freeze::tests::cfg_for_day();
        let date = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        let mut td = TradingDay::new(
            cfg, RunCfg { equity: Cash(dec!(1000000)) }, date, Lake::new(&root),
            crate::freeze::tests::state_for_day(), exec_tx,
        ).unwrap();
        // Inject a held position with an armed stop at 9.75.
        td.book.apply_fill(&Fill {
            code: StockCode(700), side: Side::Buy, shares: 1_000, lot: 100,
            px: Px::from_f64_quote(10.0).unwrap(), ts_ms: 1,
        });
        td.stops.arm(StockCode(700), Px::from_f64_quote(9.75).unwrap());
        td.phase = Phase::MorningHold;
        (td, exec_rx)
    }

    fn bar(l: f64, c: f64) -> hkq_data::model::Bar1m {
        let d = NaiveDate::from_ymd_opt(2026, 7, 3).unwrap();
        hkq_data::model::Bar1m {
            code: StockCode(700),
            ts: hk(d, NaiveTime::from_hms_opt(10, 15, 0).unwrap()),
            o: c, h: c, l, c, volume: 1000.0, turnover: c * 1000.0,
        }
    }

    #[tokio::test]
    async fn stop_breach_emits_immediate_sell_once() {
        let (mut td, mut rx) = day_with_position();
        td.on_bar(&bar(9.80, 9.82)).await.unwrap(); // above stop: nothing
        let c1 = rx.recv().await.unwrap();
        assert!(matches!(c1, ExecCmd::OnBar { .. }));

        td.on_bar(&bar(9.70, 9.72)).await.unwrap(); // pierce
        let mut saw_sell = false;
        for _ in 0..3 {
            match rx.try_recv() {
                Ok(ExecCmd::Place { order, pacing }) => {
                    assert_eq!(order.side, Side::Sell);
                    assert_eq!(order.shares, 1_000);
                    assert_eq!(pacing, Pacing::Immediate);
                    saw_sell = true;
                }
                Ok(_) => {}
                Err(_) => break,
            }
        }
        assert!(saw_sell);

        // Second breach while exiting: no duplicate sell.
        td.on_bar(&bar(9.60, 9.61)).await.unwrap();
        let mut dup = false;
        while let Ok(cmd) = rx.try_recv() {
            if matches!(cmd, ExecCmd::Place { .. }) { dup = true; }
        }
        assert!(!dup);

        // Sell fill closes the position and clears exit tracking.
        td.on_fill(&Fill {
            code: StockCode(700), side: Side::Sell, shares: 1_000, lot: 100,
            px: Px::from_f64_quote(9.70).unwrap(), ts_ms: 2,
        });
        assert!(td.book.is_flat());
        assert!(td.stops.is_empty());
    }
}

// Test-only re-exports so day tests can reuse the freeze fixtures.
#[cfg(test)]
pub(crate) mod fixtures {
    pub use crate::freeze::tests::{cfg_for_day, state_for_day};
}
```

One fixture hook is needed in `freeze.rs`'s test module for the day test above — append inside `mod tests`:

```rust
// (append inside crates/hkq-engine/src/freeze.rs `mod tests`)
    pub(crate) fn cfg_for_day() -> StrategyCfg { cfg() }
    pub(crate) fn state_for_day() -> NightlyState { state() }
```

## `hkq-live` — the paper/shadow binary

```toml
# crates/hkq-live/Cargo.toml
[package]
name = "hkq-live"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true

[dependencies]
hkq-core = { path = "../hkq-core" }
hkq-data = { path = "../hkq-data" }
hkq-risk = { path = "../hkq-risk" }
hkq-exec = { path = "../hkq-exec" }
hkq-engine = { path = "../hkq-engine" }
tokio.workspace = true
tokio-stream.workspace = true
futures.workspace = true
anyhow.workspace = true
chrono.workspace = true
chrono-tz.workspace = true
rust_decimal.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
```

```rust
// crates/hkq-live/src/main.rs
//! Paper/shadow live binary (report §4: shadow period before capital).
//! Usage: hkq-live <strategy.toml> --equity <HKD> [YYYY-MM-DD]
//!
//! Wiring per the blueprint dataflow: provider streams → LiveMux → engine actor
//! (single md consumer) → ExecCmd → exec actor (PaperVenue) → fills → engine.
//! Operator console: type `halt` + Enter to flip the kill switch.
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
use hkq_exec::{spawn_exec, PaperVenue};
use hkq_risk::{HaltReason, KillSwitch};
use rust_decimal::Decimal;
use std::str::FromStr;
use tokio_stream::wrappers::ReceiverStream;

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
    let mut rest = args.peekable();
    while let Some(a) = rest.next() {
        match a.as_str() {
            "--equity" => {
                let v = rest.next().context("--equity needs a value")?;
                equity = Some(Decimal::from_str(&v).context("equity must be a decimal HKD amount")?);
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

    // Kill switch + operator console (second producer is hkq-validate's CUSUM, M5).
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

    // Exec actor: paper venue only in M4 — the signed Tiger route is M5.
    let (fill_tx, fill_rx) = tokio::sync::mpsc::channel(4096);
    let (exec_tx, _exec_handle) =
        spawn_exec(PaperVenue, cfg.trade.participation_cap, fill_tx, kill_rx.clone());

    // Market data fan-in.
    let tiger = TigerClient::new(sources.tiger.context("[sources.tiger] is required")?)?;
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

One tiny addition `hkq-live` needs on `Lake` (it moves one instance into the engine and keeps none): add to `lake.rs`'s `impl Lake` (included in the full reprint above if you prefer; shown separately for review clarity):

```rust
// (append inside crates/hkq-data/src/lake.rs `impl Lake`)
    /// Cheap second handle onto the same root (Lake is just a path).
    pub fn new_from(other: &Lake) -> Self { Self { root: other.root.clone() } }
```

Config addition (one line in the `[ops]` table):

```toml
[ops]
universe_static_path = "config/universe_static.parquet"
# columns: code:u32, sector:u32, float_cap:f64, board_lot:u32, connect_elig:u32, spread_bps:f64?
```

## Honest gaps and hand-off to Milestone 5

Five gaps, all now wiring facts with named owners. First, the live venue: `hkq-exec::Venue` has exactly one implementation (`PaperVenue`); the signed Tiger order route — plus partial-fill accounting in the pacer, which currently treats submission as sent — is M5's first deliverable, and it plugs in behind the trait without touching the actor or the engine. Second, the kill switch still has one real producer (the operator console); `hkq-validate`'s CUSUM monitor takes the second `KillSwitch` clone and calls `halt(CusumIcBreach)` from the attribution stream this milestone started writing — the IC panel it monitors now actually exists and grows daily. Third, the estimation jobs the engine consumes as priors — θ₁/θ₂, the regime-gate matrix (identity today), v*, per-name AH betas (ρ=1 prior), per-sector A50 betas (null today), and the EWMA refinement of σ15m — are all `hkq-validate` quarterly fits; each has a marked landing point in `premarket.rs`/`freeze.rs`. Fourth, spread estimation is static-file only until the live venue's quote stream lands (the `Quote` event is already routed and dropped on the floor deliberately). Fifth, VHSI ingestion remains absent and `vhsi_tercile` remains `None` — unchanged, typed degradation. Milestone 6 (backtest) is now a two-trait swap by construction: `SimClock` replaces `instant_for`'s wall-clock arithmetic and `SimExec` replaces `PaperVenue` with the §4 cost-realism fill model, while `TradingDay`'s handlers — freeze, refresh, entry, stops, exit, post-close — are the very functions this milestone put under test, byte-identical in both tiers.
