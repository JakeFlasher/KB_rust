*Thinking...*

> Looking at the documents provided, I need to understand what's been built across the first seven milestones and determine what Milestone 8 should logically address. The progression shows a complete data pipeline from ingestion through factors and decision-making, so I'm considering what architectural or operational capability would come next to make the system production-ready. The cash arm is trivial to generate, but the other two benchmarks need external data acquisitions for HSI and sector ETFs. The trials wiring is intentionally one-way — backtests append results and reports consume them, but the DSR > 0.95 promotion gate isn't enforced yet, leaving that decision to the operator. The hkq-recon binary still can't be built without the independent official-close source from M1. M8 should focus on "The Judgment" — turning replays into the full promotion verdict by implementing as-of state snapshotting for walk-forward purity, κ calibration from realized vs model fill divergence, and the benchmark family that completes the picture.
> 
> Actually, I'm realizing the as-of reconstruction is more tractable than I initially thought. The alpha map can be refit from the Scores history bounded to any date, the regime gate and AH-β come from quarterly lake fits with date-bounded windows, and the weights already support as_of parameters. So point-in-time state reconstruction is computable from the lake itself — all the inputs are dated partitions, which means I can rebuild the state as it existed on any historical date without needing to snapshot it forward.
> 
> For M8, the promotion protocol becomes: implement the as-of state reconstruction functions that rebuild the alpha map, gate, and AH-β by re-running the fits with data strictly before a target date, then wire this into the replay driver. The tricky part is that `NightlyState::load` currently reads the static `_state` files from the lake root — the alpha map, regime gate, and AH-β parquet. I need to either parameterize `run_replay` to inject reconstructed state or point it to a sandbox state directory instead of the production one.
> 
> Looking at the current replay structure, `run_replay` takes both the production lake and the strategy config, and it passes the production config's lake root to `NightlyState::load`. The solution is to clone the config and redirect its lake root to a sandbox directory where the as-of fitted state files get written before the replay runs. The walk-forward driver would reconstruct as-of state (alpha, gate, AH-β) from production data up to each date, write it into the sandbox state directory, then run replay with either a modified signature accepting an explicit state root or a cfg clone pointing there. The signature change is tricky since M7's `run_replay` is frozen, so the house style would be to add a new function like `run_replay_with_state` rather than break the existing binary. I'm weighing whether to extend the signature or delegate through a new wrapper, keeping in mind this ties into the broader M7 hand-off scope around as-of snapshotting, κ calibration, and benchmark family. The walk-forward driver belongs in hkq-backtest since it needs to compose engine::replay with validate::asof, respecting the dependency graph. I'll add an `--asof` flag to hkq-backtest for this mode. The engine's `run_replay` needs a surgical patch to accept an optional state_root parameter instead of always reading from the lake, though this breaks the M7 test's call site — but that's being updated anyway in this milestone.
> 
> Actually, there's a cleaner path: I can clone the cfg and point its `ops.lake_root` at the sandbox root before calling `run_replay`, since the function already takes cfg as a parameter and the state loaders read from `cfg.ops.lake_root/_state`. Let me trace through what `run_replay` actually uses from cfg — it loads state from the configured lake root, writes to the output root, and pulls cost constants like `impact_kappa` and spread limits, but nothing else depends on the lake_root after the initial state load.
> 
> The walk-forward driver would just write the as-of state into the sandbox's `_state` directory, then call `run_replay` with a modified cfg where `ops.lake_root` points to the sandbox. This keeps panels coming from production while state comes from the sandbox — no engine changes needed. There's a binary-level invariant that `out_root` must differ from the production lake root, but that's enforced by the caller, and here the sandbox root is genuinely separate from production, so it's safe. Though there's a potential issue: during PostClose, the replay's own refit logic writes alpha state back to `out_root/_state`, which would overwrite the as-of alpha we just seeded.
> 
> The solution is to materialize the as-of state fresh before each day's replay — that way any refit that happens during the replay gets overwritten before the next day starts, keeping everything deterministic. But this raises a deeper design question: should the as-of alpha be fit from production scores (what the live shadow has recorded), or from the walk-forward's own sandbox scores accumulated so far? There's a tradeoff between statistical purity — learning only from the walk-forward's own history — versus anchoring to what the production system actually had available at that point in time.
> 
> The self-contained approach is purer: the entire learning loop runs within the walk-forward, mirroring how the real system cold-starts. The alpha map warms up gradually as enough scored days accumulate, which matches the conservative stance during the shadow period. The regime gate and ICIR weights also accumulate from the replayed sandbox data, so everything stays self-referential and doesn't depend on historical production scores that might not exist for older dates.
> 
> But there's a wrinkle: `load_weights` reads production attribution from the `lake` param, creating an asymmetry where weights come from production while alpha and gate come from sandbox state files. For true self-containment, we'd want attribution from sandbox too, but that would require changing how weights are loaded. The simpler, more defensible approach might be the shadow-anchored flavor instead: fit alpha, gate, and ah_beta from production scores and realized labels as of each date D, with ICIR weights already date-bounded from production attribution. This answers what the machine would have done on day D given only the information available up to that point.
> 
> M7's original critique was that the mirror uses current-state artifacts (alpha, gate, ah_beta) rather than as-of snapshots per date, which is what belongs in the promotion protocol. Shadow-anchored reconstruction from the lake is exactly that as-of snapshotting done computationally—better than literal snapshots because it works retroactively for dates before snapshotting existed, since the alpha map is a pure function of historical Scores and Bars1m data.
> 
> Let me verify the reproducibility: AlphaMap::fit(panel bounded < D) should match what the machine's PostClose refit produced at D-1 evening. The M4 refit_alpha fits on the full scores history up to and including D-1, so as-of D morning, alpha = fit(scores < D)—exactly reproducible. The gate is trickier since the actual historical gate depends on when the operator ran fit-quarterly, but the protocol-pure choice is fit(data < D) with the same floors, which represents the "if the fit had been run every night" counterfactual and is the statistically conservative choice for walk-forward evaluation. Similarly, ah_beta uses a fit window ending at D-1.
> 
> Now I'm looking at the κ calibration logic, where the regression M7 identified is that sandbox fills carry model prices while the shadow tier's fills carry real ones, and their divergence per participation bucket is what transforms κ from prior to estimated.
> 
> Working through the math: the SimExec fill price follows a formula with the reference VWAP, and the real fill's adverse move in basis points can be regressed against the participation-scaled volatility term to extract κ̂. The key question is which VWAP to use as the reference for real fills—I have Bars1m and Fills with timestamps, so for each real fill I can compute the VWAP of the minute bar containing it. The SimExec convention uses the latest completed bar's VWAP, but for consistency with how fills are actually executed, I'll use the bar containing the fill itself as the reference, which is a modeling choice I'll document.
> 
> For the participation ratio q/ADV, I need the fill shares and the average daily volume as of the fill date. The ADV comes from the enriched panel or can be derived from DailyBars as a rolling mean. Rather than joining the full enriched panel for each fill, I can follow the approach of bucketing fills by participation level and running the regression within each bucket—that's cleaner and aligns with the M7 methodology.
> 
> Now I'm setting up the actual job: load the real-tier fills from the lake with columns for code, date, side, shares, price, reference VWAP, static spread, and the volatility and ADV metrics pulled from the enriched panel as of the prior day. The tricky part is that paper fills execute at the limit price (which is just the mark at sizing time), so they don't carry real impact information—only tiger fills do. The pipeline can be built and tested with synthetic data, but it should reject the calibration if actual tiger fills are missing. Paper fills would show adverse selection as mark-versus-mark noise rather than true market impact. The Fills schema doesn't have a venue tag to distinguish paper from tiger fills, so the operator needs to know which tier wrote to the lake.
> 
> For each fill, I'm computing the reference VWAP from the prior completed bar in the 1-minute bars table, dropping fills where the prior bar is missing. Then I calculate adverse selection in basis points as the signed difference between execution price and reference, scaled appropriately. The impact regressor x combines the cross-sectional volatility and ADV ratio in a way that matches the model's scale, so the regression slope directly gives κ. For the volatility and ADV values, I need to pull from the enriched daily panel as of the trading day before each fill—non-anticipative, matching what the live sizing engine saw. Building the full enriched panel is expensive but this is a batch job, so I'll reuse the existing PanelBuilder over a window that includes enough warmup (about 130 trading days before the earliest fill date). The sigma_cc values are already shifted to represent what was known at the start of each day, so for a fill on date D I grab the row from D−1.
> 
> To handle the shift cleanly in the join, I'll compute the enriched panel per code and date, then shift the sigma_cc and ADV columns by one row within each code partition (sorted by date), so the shifted row at date D holds the previous trading day's values. Then I subtract half the spread from the adverse selection—pulling the spread per name from the static universe file when available, otherwise falling back to the config default. Finally, I regress the spread-adjusted adverse selection on the impact regressor, both pooled across all fills and separately within each participation bucket, fitting through the origin to recover κ̂.
> 
> I'll report the through-origin κ̂ as the recommendation (matching the model form), plus diagnostics including an intercept-fit version to check if the half-spread adjustment is accurate, a per-bucket table to verify the √-scaling assumption holds, and the sample size. The registry logs κ̂, its standard error, fill count, and intercept bias. Since κ promotion is an operator edit to the config, I'll report and log the result but never auto-write it. For the standard error, I'm computing SE = sqrt(Σr²/(n−1)/Σx²) and the t-statistic—straightforward math that I can validate with synthetic fills where the true κ is known.
> 
> On the SPA front, the benchmark family is still partially blocked: the always-cash arm is trivial to generate (zero returns), but the HSI and sector-ETF series are still pending data acquisition. I'm weighing whether to ship the SPA machinery now with just the always-cash benchmark (which reduces to a studentized bootstrap test of mean PnL > 0) or defer SPA entirely and keep DSR plus bootstrap CIs as the promotion gates. Given that M8 is already substantial with the as-of snapshotting, walk-forward validation, and κ calibration, and since SPA's real value comes from the max-over-benchmarks correction—which is meaningless with a single degenerate benchmark—I'm leaning toward deferring SPA again and sticking with the existing gates until the data arrives.
> 
> For the promotion verdict assembly, I can point the existing report job at the walk-forward sandbox now that the daily PnL series is available.
> 
> M8 needs to add a metric marker to the walk-forward binary so the registry can distinguish pure trials from current-state counterfactuals, keeping the DSR gate evaluation in the report job. For the as-of reconstruction, I'm building functions to materialize the AlphaMap, RegimeGate, and AH-β state files atomically for a given date, plus a date-bounded variant of the scores panel that filters by date using polars — reusing the existing panel and applying a date filter rather than re-scanning.
> 
> For the alpha fitting, I need to mirror the panel construction from the live binary: joining scores with realized window returns and adding a null VHSI column, then filtering by date and fitting the AlphaMap. Since validate can't depend on engine, I'm duplicating the ALPHA_MIN_OBS constant locally with a comment explaining the design (following the precedent of duplicating gate_state_path between the two modules). I'll also add alpha_state_path to validate's module, mirroring engine's version.
> 
> Now I'm setting up the kappa module to compute fill-versus-model divergence and run through-origin regression with bucket diagnostics.
> 
> Adding CLI jobs and walk-forward mode... For the first date in as-of mode, I need to ensure materialize writes all three state files with sensible defaults when data is insufficient. If the alpha panel has too few observations, I'll write a conservative empty-coefficient alpha map that predicts zero. For an insufficient gate, I'll write an identity gate. For ah-beta, I'll write an empty parquet with the correct schema so ah_beta_frame reads zero rows and the prior holds. Writing all three files deterministically prevents stale state from leaking forward. But I'm realizing there's a subtlety with how NightlyState::load is called — in as-of mode it uses the asof_cfg, but run_replay internally calls it with the mutated cfg clone.
> 
> The workspace stays unchanged — asof.rs only needs hkq-signal (already a validate dep) and kappa reuses existing polars and PanelBuilder. Now I'm mapping out the file inventory: lib.rs gets the new module declarations, asof.rs and kappa.rs are entirely new with full implementations and tests, main.rs in both validate and backtest get updated with the new job types, and crucially, no surgical patches to frozen crates — the seams from earlier milestones are sufficient. Let me verify that refit_alpha can actually rebuild the alpha panel without engine access by checking what data it needs to scan.
> 
> Checking dependencies...
> 
> I'll use a named constant for the minimum observations threshold to match the existing pattern, then implement the three reconstruction functions plus the materialization function in asof.rs with atomic writes for the alpha state. For the κ panel, I need to pull fill details like code, date, timestamp, side, shares, and price, then cross-reference against the one-minute bars to get VWAP for each fill.
> 
> The reference bar for a fill is the most recently completed bar before that fill's timestamp — I'll compute this by dividing the fill's timestamp by 60,000 milliseconds, casting to integer, then subtracting one bar period to get the prior bar's open time. Then I'll join the fills against the bars table on code, date, and that computed bar open time to pull in the VWAP. I also need to grab sigma_cc and adv_shares from the enriched daily panel using a shift operation to get the prior-row values. Now I'm defining the fit function to estimate κ using least squares regression on the e and x values, requiring a minimum number of fills to ensure statistical reliability. I'll compute the standard error and t-statistic, plus an intercept-slope diagnostic, and organize results by participation quartiles to check if κ varies across bucket sizes. For testing, I'm building a synthetic scenario with known κ to verify recovery, and a small end-to-end test using minimal fixture data—just one code with a few days of bars and a couple of fills—to validate the panel construction and hand-check the computed e and x values.
> 
> For the panel test itself, I'm setting up a specific scenario where a fill at 09:45:30 references the bar at 09:44, with a known vwap to compute adverse selection; when the purchase price equals the reference exactly, the adverse should be exactly −s/2 regardless of volatility, giving me a clean assertion. I'll also verify that fills missing their reference bar get properly dropped and counted.
> 
> Adding the walk-forward binary changes... I'm wrestling with whether the walk-forward should bootstrap alpha from production Scores (the only historical record available) or from sandbox Scores generated during the run itself. Using production Scores creates a mixing concern—they came from a different config—but it's the only ground truth we have; using sandbox Scores keeps the simulation self-consistent but risks contamination if the output root carries prior runs. The honest framing is that a promotion-grade walk-forward starts cold on alpha and warms as it progresses, which is the defensible default for shadow discipline.
> 
> Now I'm circling back to the practical question: sandbox-sourced alpha means pulling Scores from the sandbox lake but Bars1m from production (since the sandbox never writes bars), which requires a two-lake join in the materialization and gate logic—doable but adds complexity. Meanwhile, AH-β inputs like DailyBars and MainlandPrints are production-only anyway, so that piece is necessarily prod-sourced regardless.
> 
> The real sticking point is attribution weights: the engine loads them from whatever lake is passed to run_replay, so going fully self-hosted would require patching the engine to source weights from the sandbox lake instead. That's a seam surgery I want to avoid.
> 
> Weighing the options—partial self-hosting (sandbox alpha/gate, production weights/AH-β) is inconsistent and messy; full self-hosting needs an engine patch; but shadow-anchored requires zero patches and stays internally coherent: every learned artifact comes from production history strictly before day D, which is exactly what the live machine would have had that morning. That's the move—it fixes the stated defect without surgical changes, and the true self-hosted variant becomes a named gap for later work once the weights-source seam is properly addressed.
> 
> So M8 is "The Verdict": I'm building the point-in-time state reconstruction module, adding walk-forward mode to the backtest tool, creating the κ calibration job with its CLI, and adding an audit CLI for single-date materialization—all without touching the frozen crates or workspace structure. Self-hosted walk-forward and the full data reconciliation are deferred since they depend on the weights seam and have different consumers downstream.
> 
> The binding constraint here is that the DSR gate currently uses a counterfactual the protocol itself marks as dishonest for promotion, so the as-of reconstruction is what unlocks the pre-registered gate; κ calibration is equally critical since every replay PnL inherits an uncalibrated cost prior, and now that the inputs exist in the lake, calibration becomes the only path from prior to real estimate. This milestone is almost entirely math over the data lake with zero vendor surface, so it can land while the M6 verification work continues operationally.
> 
> Writing the asof.rs module... Writing Rust code... I'll add a Signal variant to the ValidateError enum in error.rs since that's cleaner than mapping errors inline, and it's fine to update the validate crate's own files in this milestone. For the asof_gate function, I'm handling the case where the scores_realized_panel returns Insufficient by defaulting to an empty RegimeGate, while propagating other errors — and I'm simplifying the doc comment to just note that nightly refresh is the only reconstruction that's a pure function of the lake data.
> 
> Now I'm implementing asof_ah_beta, which computes per-name AH betas as of a given date using the M5 quarterly regression over a trailing window ending the day before that date.  If the panel is empty or the fit fails, it returns an empty dataframe matching the schema so the engine's default prior holds. Then I'm moving into materialize, which writes out the full as-of state snapshot—alpha_map.json, regime_gate.json, and ah_beta.parquet—atomically to ensure no stale artifacts leak between replayed days.
> 
> Now I'm implementing the materialization function itself: it fetches the three learned components (alpha, gate, and ah_beta), writes alpha as JSON with a temp-file-then-rename pattern for atomicity, saves the gate and ah_beta dataframes via the fits module, and returns a report with the date and some metadata about what was materialized. I'm also verifying that the RegimeGate comparison works—the `g` field is public and arrays of f64 support PartialEq, so checking if the gate equals the default is valid.
> 
> For testing, I need to build a fixture lake with Scores and Bars1m data across multiple dates, then verify that asof_alpha respects the ALPHA_MIN_OBS floor of 60 rows and only uses data before the target date. I'll create two dates with 40 codes each before the target, then one date after, and construct the post-date scores with opposite relationships to confirm the asof function correctly excludes them.
> 
> For the ah_beta test, I need to generate 70 daily partitions with MainlandPrints and DailyBars, setting up the data so that the overnight return r_on (derived from open/close prices) correlates with the mainland print values at a known slope around 1.3. I'll control this by setting the open price relative to the previous close using the print value, keeping intraday movement flat, and ensuring at least 60 matched rows with finite values to meet the AH_MIN_OBS threshold.
> 
> For the materialize test, I'm simplifying the approach: the main integration test will verify that a cold lake produces conservative output (empty alpha map, identity gate, zero-row ah panel), and I'll separately test that materializing over a stale warm alpha file overwrites it with the conservative version. The bounding check is straightforward—just verify that including a print row at or after the cutoff date doesn't change the recovered β and that the observation count stays within bounds.
> 
> Now I'm moving into the kappa.rs module, which handles κ calibration from fills. The cost model prices simulated fills using a reference (the latest completed 1-minute bar's VWAP) adjusted by a spread formula involving κ, volatility, and order size relative to ADV.
> 
> Defining the kappa panel builder... Handling NaN and null comparisons in the turnover calculation — NaN comparisons evaluate to false in Polars following IEEE semantics, and null values in the when-condition fall through to the otherwise branch, so the logic mirrors the intent correctly. I'm planning to left-join fills with bars on code/date/__ref_open, then count null references to track dropped fills and filter them out. For sigma and ADV, I'll collect the fills eagerly first to extract the date range, then use PanelBuilder to fetch enriched daily factors with the appropriate warmup period, and join those onto the panel by code and date with a one-day shift to get the as-of-morning values.
> 
> For the spread column, I'm handling both cases — loading from a static parquet file if provided, otherwise using a constant — with a local const to define the column name as a contract with the statics schema. Then I'm computing the execution slippage, volatility impact, and participation metrics using the side sign, reference price, and ADV values, with guards to ensure valid inputs like positive reference prices and finite positive sigma values.
> 
> For filtering, I'm dropping rows where sigma is zero or ADV is zero since they carry no information about execution impact — those unknowns would default to zero impact in the simulation anyway. The output includes date, code, side, shares, participation ratio, impact volatility, and adverse slippage, with standardized column names.
> 
> Defining the fit structures... For insufficient data, I'll treat it as a non-fatal warning and return Ok rather than bailing, matching the report's behavior. On the CLI side, I need to add the `asof-state` command with an optional date parameter (defaulting to today's HKT date), but critically, the materialized state must go to a separate audit directory like `<lake_root>/_asof/<date>/` rather than the production `_state` folder—otherwise it would overwrite the live state the system loads. For the backtest updates, I'm adding an `--asof` flag that clones and modifies the config to point the lake root to the output directory when that mode is active.
> 
> In the main loop, for each trading day I'll call materialize_asof_state if in asof mode, track failures, then run the replay with the modified config so it loads the just-materialized alpha file. I'm hoisting the vcfg load to the top of run() since it's needed early, updating the summary JSON to mark the mode as either "asof_walk_forward" or "current_state_counterfactual", and noting in the trials metadata that as-of trials are promotion-grade. The usage string gets the `[--asof]` flag, and I need to be careful that the replay's NightlyState load uses the modified config to read from the output root, while the alpha materialization each day still reads from the production Scores.
> 
> For the library updates, I'm adding the asof and kappa modules to lib.rs with their public exports and updating the header docs, then adding a Signal variant to error.rs. On the Polars side, I'm deciding between using Series::n_unique() directly or reusing the existing date_groups function from fits — the latter is heavier but consistent with how the codebase counts dates elsewhere, and since the panel already has the required columns for date_groups, I'll go with that approach for consistency.
> 
> For the timestamp bucketing, I'm mirroring M2's pattern exactly: cast the millisecond column to Int64 after dividing by 60,000, then multiply back and subtract 60,000 to get the floor bucket. The join uses three keys (CODE, DATE, and __ref_open), fills come in as u64 from the parquet and need casting to Float64 for calculations, and I'm sorting by code and date before applying the shift operation.
> 
> For statics, I'll use the same ScanArgsParquet approach with hive disabled like the sector panel does. The enriched_daily function signature takes optional sector_map which I'll pass as None. I need to extract the date range from the panel by collecting the fills frame and iterating through the DATE column to find min/max values rather than trying to compute it lazily.
> 
> Writing test cases... The key issue is that DailyBars for the fill date D needs to exist so the shift can carry forward the previous day's values; in production this happens naturally after the nightly job writes D's partition, but in tests I need to explicitly write it. I'll document that fills on dates without a daily-bar row get dropped, and stick with the shift-then-join approach rather than a complex asof-join. For the test config, I'll write a small StrategyCfg literal with tight FactorCfg windows, then assert the panel has both fills with e ≈ −17.5, participation computed from shares divided by the constant adv, and x > 0 finite.
> 
> Now checking the timestamp alignment: the fill at 09:45:30 has a ref_open that floors to 09:44:00, which matches the bar's exact minute boundary timestamp. The join on (CODE, DATE, __ref_open) works because both fill and bar rows are same-day D, and fills at 09:45 reference bars at 09:44 or later (early fills are rare, and CAS fills at 16:00+ reference the 15:59 bar which exists). The fills schema uses "date" as a string, bars use the TURNOVER column, and everything aligns.
> 
> Setting up the asof test... I'm verifying the bar selection logic where entry at 09:45 and exit at 15:45 exclusive correctly captures both the opening bar (117) and closing bar (188) within the range, then computing forward returns as log ratios. For the materialization test, I'm checking that a cold lake produces empty alpha buckets and zero basis points as expected, and after writing and reading back the JSON, the AlphaMap coefficients remain empty. For the beta recovery test, I'm generating 70 trading days with alternating alpha patterns, constructing open prices from the previous close scaled by the alpha factor, and setting close equal to open with adjusted close matching. The test data partitions by date with minimal columns—code, date, and the alpha open return—matching the mainland schema requirements.
> 
> Now I'm setting up the beta estimation test where fitting across 550 days captures rows 2 through 70 (69 observations, exceeding the 60-day minimum), recovering a beta near 1.3 and clamped within [0, 2]. I'm also injecting a poisoned print at day 71 with an extreme alpha value to confirm the estimate remains stable, then writing absurd data at that point to verify the bounded behavior holds. The response structure follows the established series format: opening context, a titled milestone section, justification from the prior milestone, scope and deferral notes, engineering decisions, file tree, workspace note, and full code reprints for validation and new modules, finishing with honest gaps and a hand-off to the next milestone. Defining error types and module structure... Writing module imports... Filtering the scores panel to dates before the target date, then joining with realized window returns based on code and date, adding a null VHSI tercile column, and fitting the alpha map with the minimum observation threshold. I'm also verifying that the let-else pattern with a tuple of Results works correctly in Rust 1.83.
> 
> Now moving to the gate function, which retrieves the regime gate as of a given date by building a panel from scores and realized returns, applying the same observation floors as the quarterly job, and returning a default gate if there's insufficient data.
> 
> Then there's the beta function that computes per-name AH betas as of a date over a rolling window, returning an empty frame if history is missing or too short so the engine falls back to its default prior.
> 
> Now I'm defining the report structure that captures the as-of state snapshot—alpha bucket count, whether the gate is identity, and how many names got fitted betas—then the materialize function that rebuilds and writes the complete state files atomically to the sandbox. Writing test helpers... I'm testing the alpha calculation at different dates to verify it's bounded correctly before the regime break, then checking that the fitted slope changes when the flipped day enters the historical window, and finally confirming the cold start returns zero alpha conservatively. Finishing the test function...
> 
> The asof_gate function depends on scores_realized_panel which pulls specific columns like EPS_GAP, so a warm-gate test would require building those columns in the fixture. Rather than write out a large fixture with 45 dates and multiple codes just to test the date-bounding logic (which is identical to alpha's filter and already covered by M5 tests), I'll stick with the identity-floor test plus rely on the shared bounding rationale across alpha and gate. The gate's filter logic is one line and identical to alpha's, so skipping the warm variant keeps tests focused without losing coverage. Writing test code... Defining the kappa calibration module... Writing the join and validation logic... Writing assembly logic... Defining the fit structures... Calculating the kappa coefficient and standard error, then computing a free-intercept diagnostic with means and covariances. Now I'm partitioning the data into equal-count participation buckets by sorting on participation values and dividing into segments. Writing test cases... Building the test data... Continuing with the test assertions...
> 
> I'm checking that the adverse impact value equals negative half the config fallback (−17.5) when price matches the reference, and verifying the participation ratio matches the expected ADV calculation. The impact metric should also be finite and positive. But I'm realizing a problem: if the prior day has constant closes, the volatility will be zero, which gets filtered out and leaves the panel empty. I need to vary the daily closes so the returns aren't all zero and volatility actually gets calculated.
> 
> I'll set the closing prices to alternate by about 1% so the returns vary and the EWMA variance becomes positive after the minimum observation threshold. With 10 warmup days, I'll have enough history for the sigma calculation at the test date. The ADV from the rolling mean of constant volume stays exact at 1e6 regardless, and I need to make sure the adjusted close matches the regular close. The shift operation pulls sigma and ADV from the previous partition row, which should be the last warmup day before the test date. I also need to verify that the enriched daily panel gets the realized volatility join from the 1-minute bars correctly—those only exist at the test date, so other dates will have null realized volatility, but that doesn't affect sigma or ADV calculations.
> 
> Now I'm setting up the FactorCfg with the test parameters from M7: EWMA halflife of 2, minimum observations of 2, Amihud window of 3, realized volatility days of 2, LAV gamma of 0.3, seasonal window of 2, and IVU window of 4. I also need the full StrategyCfg struct with all required fields—universe, factors, stage1, stage2, trade, costs, and ops—copying M7's configuration but without the static path.
> 
> For the main.rs validation, I'm adding two new job dispatches: one for asof-state and one for fit-kappa. The asof-state job reconstructs the materialized state for a given date and writes it to a separate audit directory under `_asof/<date>/_state/` rather than touching the production state directory.
> 
> Finishing the job_fit_kappa function... Writing the backtest main function... Adding the asof metric to the registry entry, hoisting the config validation to the top level to avoid reloading it, updating the usage docs, and importing the materialize function. I'm also verifying the state materialization order in as-of mode — the initial state gets written before the first replay so alpha/gate/ah can read those files, and during each day's PostClose the refit_alpha call will harmlessly warn and skip when it can't find Bars1m in the sandbox lake.
> 
> Checking the Scores flow: replay writes partitions into the output root that accumulate across walk-forward days, while asof_alpha reads from the production lake unaffected. For the NightlyState load step, the production lake is passed correctly for bounded weight loading, and the PreMarket inputs like iev_bar20_frame already have point-in-time filtering built in through their windowed panel logic — only the three state files needed explicit as-of handling. The Cargo.toml stays unchanged since hkq-validate is already a dependency.
> 
> Now I'm framing the response around Milestone 8 — calling it "The Judgment" to echo M7's closing about what the mirror lacks. The core story is that the DSR gate currently starves the pre-registered promotion gate of promotion-grade trials, the κ prior contaminates replay PnL, and both inputs now exist; the milestone proves the seam design by consuming it without any patches to frozen crates. The engineering approach centers on reconstruction over snapshots for determinism, shadow-anchored semantics tying all artifacts to production history, the state-root parameter split avoiding engine surgery, always writing all three artifacts to prevent stale leaks, and using the κ reference convention from SimExec verbatim with prior-row σ/ADV from the machine's own morning numbers.
> 
> For diagnostics I'm adding through-origin and intercept checks plus a flat-bucket shape test, with governance handled through report and registry where operators edit config; the audit path lives in _asof/<date> for the CLI job while production _state stays untouchable, and duplicated constants follow the M5 precedent. The file tree stays unchanged from M6, with code split across error.rs, lib.rs, asof.rs, kappa.rs, and two main.rs files for validation and backtesting. The honest gaps deferred to M9 include self-hosted walk-forward with the weights-source seam and two-lake panels, the SPA benchmark family still blocked on data ingestion for HSI and sector-ETF, a gate cadence refinement where the registry already records quarterly_fit metrics so a cadence-faithful reconstruction is possible once wired in, and κ needing real tiger fills instead of paper fills since the fill schema carries no venue tag.
> 
> The system now has facts, opinions, decisions, a clock, memory, reflex, a hand, and a mirror—and as of this milestone, judgment—but what remains is the ML layer consuming CPCV, data acquisitions, and recon. I'm extending the validate main.rs from M5 carefully, keeping cusum and fit-quarterly and report intact while adding asof and kappa imports and two new dispatch arms; the backtest main gets reprinted with the same registry append logic for asof metrics. Let me verify kappa's implementation details...
> 
> Checking the panel construction: the joined frame has code, date, ts_ms, side, shares, px, and ref columns, then known joins on (code, date) to add sigma and adv columns, statics adds spread_bps, and the final select projects DATE, CODE, side, shares, PARTICIPATION, X_IMPACT, and E_ADVERSE. The cast to Float64 preserves the column name, the when-then-otherwise expression works inside select with an alias, and the enriched panel from PanelBuilder sorts by code before shifting to handle the time-series ordering correctly.
> 
> For the test fixture, I'm setting up warmup days where close alternates by ±1% and open equals close so returns vary, then the sigma_cc calculation uses exponential weighted mean over returns with min_periods=2 followed by a shift(1), which means the first non-null sigma appears at row 3 (plenty of warmup rows available). The adverse calculation needs constant volume of 1e6 across all days, and the shift ensures row j gets sigma from rows up to j−1, so the warmup is sufficient before the actual test date.
> 
> For the alpha tests, I'm verifying that AlphaMap::fit works with 80 rows against the global min_obs threshold of 60, and the OLS regression on exact-linear data should be precise to within the tolerance of 1.0 since the return is computed via ln(last/first). The later fit has 120 rows all above the threshold. I'm also checking the bod5 window calculation where 09:45 maps to minute 117 and 15:44 maps to 188, both within the [117,189) range. The Scores schema includes the date column as required, and asof_gate uses ValidateCfg::default() which floors fit_min_obs to 120. The materialize writes log the gate and handle the mutable dataframe for beta, and I need to verify that VHSI_TERCILE is exported as a public constant and update lib.rs with the new modules.
> 
> Now checking that AlphaMap and RegimeGate both derive Default, and confirming all the public exports: date_groups, GATE_MIN_DATES, scores_realized_panel, fit_regime_gate, ah_panel, fit_ah_beta, save_gate, and save_ah_beta. The chrono Duration operations work on NaiveDate in fits, and the asof tests import Dataset and Lake from hkq_data::lake. I need to add the Datelike import for the weekday() call in the kappa test warmup loop. For the response, I'll mirror the M7 opener about having all documents attached and quote their own hand-off about what's missing—the as-of snapshotting, κ calibration, and benchmark family that turn replays into the full promotion verdict. Finally, I should verify that hkq-validate's Cargo.toml has all the dependencies needed.
> 
> All the required crates are already in the manifest—hkq-signal, hkq-factors, chrono, serde_json, polars, hkq-core, hkq-data, tracing, anyhow, toml, and sha1—so no manifest changes are needed. The hkq-backtest Cargo.toml stays unchanged. Now I'm laying out the validate main.rs structure: updating the usage strings in three context messages, adding the match arms, inserting the new job functions in order (job_cusum, median, job_fit_quarterly, job_report, job_asof_state, job_fit_kappa), and merging the imports to include asof, cusum, dsr, fits, kappa, pnl, and stats. The NaiveDate import is already there. The core argument is that M7's hand-off identifies what's missing—three items blocked on external data (SPA benchmarks, recon, POS/VCM realism), one governance piece (gate enforcement), and two pure math problems whose inputs the system started writing at M4–M7: point-in-time state and κ calibration. The DSR angle ties back to M5's pre-registered DSR>0.95 as the promotion gate.
> 
> The binding constraint now shifts from trial existence (M7 solved that) to trial honesty—every trial the mirror generates is stamped as a mechanical counterfactual with today's learned state, so the gate is fed but with evidence the protocol itself disqualifies for promotion. Every replay PnL and sr in the registry flows through κ, which has been a hand-set prior since M1 while the report claims "calibrated from fills"; real fills began existing at M6, so M8 upgrades both the state the mirror replays with and the prices it settles at. The κ̂ feeds config rather than auto-calibrating, but turning the cost model from assumption into estimate closes the §4 "cost realism" clause. The structural elegance matters too: zero patches to M1–M7 code, pure composition over existing seams, which proves the seam architecture was sound. Shadow-anchoring (all state from production history) rather than self-hosting reproduces exactly what the live machine had—alpha, weights, window fits all bit-exact—so the walk-forward answers whether the machine as operated would have earned its SR. Self-hosting would require a weights-source seam in the engine, which is deferred with a named owner, and partial self-hosting would mix provenances, breaking internal consistency.
> 
> The key insight is that NightlyState::load's parameter split already handles most of the problem: panels and windowed inputs are point-in-time via the lake param and date windows, with only the three _state files being "today-state." Since they're keyed off cfg.ops.lake_root, pointing that at the sandbox root whose _state we control makes the whole assembly as-of with zero engine edits. I'm also hoisting the validate config load to the top of run() so it's used for both materialization and registry, removing the inner load call.
> 
> Updating backtest main...
> 
> Adding command-line parsing and metrics...
> 
> Now I'm adding documentation about the dangers of reusing out_root across experiments in as-of mode—state gets rewritten per day, and mixing modes in one root poisons the sandbox PnL, so I'm recommending a fresh root per experiment in the doc comment. For the response format, I'll use headers and prose paragraphs flowing naturally, keeping engineering decisions dense like the prior milestones, and for the gaps section I'll list them as named facts. Counting the M9 hand-off items: self-hosted walk-forward seam, cadence-faithful gate replay from registry records, and κ venue-tag / tiger-fills requirement since the fills schema lacks a venue column.
> 
> Now I'm verifying the implementation details—checking that asof_gate's use of date_groups works on the scores_realized_panel which guarantees the columns, confirming that fits::scores_realized_panel properly handles missing Scores or Bars1m data and empty joins, and validating the let-else pattern in asof_alpha where both scan results are dropped on error. In kappa_panel, n_all counts fills rows post-join matching the fills count, with dropped calculated as the difference. I'm also reconsidering the KAPPA buy/sell edge case and the sign handling for the sell formula.
> 
> Now I need to finalize the answer by explicitly listing the deferred scope items—SPA data, self-hosted workflow, recon data, VHSI/A50/quote-replay, promotion automation, and CPCV consumer—then write out the complete files. I'll keep the Cargo.toml as a one-liner noting it's unchanged, extend lib.rs with the new module documentation and re-exports for asof and kappa, write tight module docs for both, add all tests, and verify the backtest imports and function signatures work correctly with the new materialize_asof_state call.

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

## Honest gaps and hand-off to Milestone 9

Six items, each now a named fact. First, the walk-forward is **shadow-anchored, not self-hosted**: every as-of artifact is reconstructed from *production-recorded* history strictly before D — which makes it a faithful counterfactual of the machine as operated (alpha and weights bit-exact, window fits window-exact) — but a walk-forward that learns from its *own* sandbox scores (evaluating a divergent config from a cold start) needs one more seam: `load_weights` reads attribution from the `lake` parameter, so a self-hosted run requires a weights-source seam in PreMarket plus two-lake panel variants; its genuine consumer is config-divergent experimentation under CPCV and the §3.7 ML layer, and it lands with them. Second, the gate reconstruction is the nightly-cadence counterfactual; a **cadence-faithful** replay is now possible in principle because every real `quarterly_fit` since M5 logged its matrix (g00…g22) to the hash-chained registry — wiring registry-recorded gates into `materialize_asof_state` is a small, named refinement. Third, κ calibration is **built but starved**: the Fills schema carries no venue tag, paper fills measure mark noise rather than impact, and the job's honest output therefore waits on accumulated M6 tiger fills (or a schema-v2 venue tag on Fills — a data-layer decision with a data-layer owner). Fourth, the Hansen SPA benchmark family is unchanged: always-cash is trivial, but HSI open→close and the sector-ETF Stage-1 arm still need ingested benchmark series nothing writes — the same data-milestone fact M5 through M7 named. Fifth, promotion **enforcement** remains operator governance by design: the `report` job states the DSR verdict, `--asof` trials are now the evidence it should weigh, and nothing automates the config edit — per the M5 precedent, unchanged. Sixth, `hkq-recon` remains the only blueprint binary not built, still blocked on the independent official-close source, still logged loudly every night. The system now has facts, opinions, decisions, a clock, a memory, a reflex, a hand, a mirror — and judgment: any recorded day replays under the state the machine actually had that morning, the resulting trials enter the registry as promotion-grade evidence, and the cost model that prices them is calibratable from the hand's own fills. What it still lacks is breadth of evidence — the benchmark family and the reconciliation binary that are data acquisitions, not code — and the learned layer (§3.7, with CPCV and the self-hosted walk-forward) that would finally consume the machinery this milestone completed.
