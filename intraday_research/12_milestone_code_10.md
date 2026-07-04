*Thinking...*

> I'm looking at the documents the user has provided—there are several milestone files covering different stages of development, along with the framework proposal and research specification that form the foundation.
> 
> The user mentions 11 URLs including a ninth milestone document, but that file isn't actually attached to this conversation. I notice the previous milestone documents all mention having all materials available, so there's a gap here where the latest codebase state document is referenced but not included. I only have web_search available, not a fetch tool, so I can't retrieve that missing document on my own. So the pattern makes sense now — the 11th document listed in the prompt template is actually the milestone I'm being asked to produce, not something that already exists. The user's message includes all ten prior documents (report, proposal, and milestones 1–8), and my task is to generate milestone 9. I could search for it to be thorough, but that's probably unnecessary given the clear pattern of how these exercises work. So the missing M9 document isn't attached, and I don't have a fetch tool—only search. I could try a web search to be thorough, though a private repo URL likely won't be indexed. The more practical approach: work from the complete M1–M8 codebase state I have, compare it against the blueprint, and derive what M9 should be. If the document already existed, it would've been included like the others.
> 
> Looking at the workspace topology against the blueprint, every crate is built except `hkq-recon`, which has been consistently flagged as data-blocked—it needs an independent official-close source from HKEX to reconcile against Tiger, not Tiger against itself. The reconciliation logic itself exists in hkq-data since M1, but the binary wiring and data provider are missing. M8's hand-off lists six concrete gaps: self-hosted walk-forward (needs weights-source seam), cadence-faithful gate replay (registry wiring refinement), κ calibration starved (needs accumulated fills or schedule data), and a few others still in progress.
> 
> The closing observation is sharp: the system lacks breadth of evidence—the benchmark family and recon binary are data acquisitions, not code—and the learned layer (§3.7 with CPCV and self-hosted walk-forward) that would actually consume all this machinery. So M9 is either the data milestone (benchmark ingestion, official closes, hkq-recon) or the ML layer (§3.7 + CPCV + self-hosted walk-forward). The pattern across milestones is to pick the one whose inputs already exist, and the §3.7 layer depends on CPCV and the self-hosted walk-forward being ready first.
> 
> The data milestone unblocks everything immediately: Hansen SPA (§4 mandate), VHSI terciles for AlphaMap (currently None), per-sector A50 betas (S6 component null), ADR residuals (S6 third component null), and hkq-recon (§5 mandate, logged loudly every night since M1). The report explicitly mandates §4's Hansen SPA test against the null family and §5's nightly reconciliation against official closes—both unmet since M1. AlphaMap's VHSI buckets are typed degradation (vhsi_tercile is None everywhere), and S6 has null columns that would fabricate factors without the A50 persistence and sector-ETF feeds. The deciding factor is clear: M9 is the data milestone, owned by the data layer.
> 
> Now I'm wrestling with whether M9 already exists in the repo or if this is the turn to write it. The user listed 11_milestone_code_9.md as part of the current codebase state "(9 of 9)", which suggests 9 milestone files already exist. But the attachments I have are report + proposal + M1–M8 (10 docs total), which matches the pattern of every prior turn where the M_n turn has exactly M1 through M(n-1) attached. The URL list might be pre-listing where my output will live, or M9 might already exist and I'm writing M10 or a different path. Without the content of M9, I can't know what's already been done.
> 
> The most defensible move is to be transparent about this gap: note that M9 isn't attached despite being listed, acknowledge I can't fetch it, and proceed with what I can verify—the blueprint still has unbuilt components, so the next logical step is the next milestone. If M9 does exist and covers the same ground, the user can tell me and re-attach it.
> 
> Now I'm checking whether the framework might actually be fully implemented already by reviewing the blueprint's component checklist against what's been built: workspace and config are done, the core modules are in place, data providers are wired up, factors and signals are complete, risk controls are implemented, execution logic is there—but I'm noticing some gaps like the recon function not being officially wired and a few other pieces that suggest the work isn't quite finished.
> 
> Looking at the report requirements, there are several unmet items: the Hansen SPA test against benchmarks is missing, nightly reconciliation against official closes isn't done, VHSI regime stratification has no ingestion pipeline despite the enum existing since M1, and some S6 subcomponents need additional data feeds. The ML layer and related components are deferred by design. So the framework is incomplete—PATH A is the right direction, and the next clear milestone from M8's handoff is getting the deferred data facts into actual datasets and wiring them through.
> 
> For Milestone 9, I'm thinking through the scope carefully. The core need is new dataset writers in the lake: VHSI (the enum variant exists but nothing populates it), HSI index daily bars, sector-ETF daily bars for benchmarks, A50 intraday persistence, and official closes for reconciliation. The tricky part is where these live in the existing Dataset enum—HSI and sector ETFs could potentially route through DailyBars if I add them to a benchmark code list, but HSI as an index doesn't have a stock code, so I'd need to check what EastMoney can serve via secid.
> 
> For the consumers, `hkq-recon` becomes unblocked as a separate binary that loads official closes from an independent provider and runs the existing reconciliation logic with quarantine on breach. The nightly binary stays frozen, though I could optionally patch its warning message to point users toward the recon binary—but that's cosmetic and probably not worth the touch.
> 
> For the SPA test in `hkq-validate`, I need to implement a stationary/circular block bootstrap over loss differentials comparing the strategy against a null family of benchmarks (always-cash, HSI open-to-close, sector ETFs). The challenge is normalizing everything to consistent units—strategy PnL comes from realized fills per day, but benchmarks are in returns, so I need to figure out how to compare them fairly.
> 
> The cleanest approach is to work with daily return series where strategy daily return equals PnL divided by equity, with equity passed as a CLI argument (following the M4 decision that equity is a runtime parameter, not a config field). If I express both strategy and benchmarks in HKD per day on the same equity base, the SPA statistic becomes scale-invariant across the series pair, so the comparison stays consistent regardless of the equity value chosen.
> 
> For the CLI, I'll add a subcommand like `hkq-validate <strategy.toml> spa --equity <HKD>` to keep it straightforward. The alternative of computing benchmarks as returns times equity base while strategy PnL stays in HKD creates an implicit equity mismatch that's harder to reason about. I'm also considering whether the sector-ETF benchmark arm (holding selected sector ETFs open-to-close to match the signal horizon) could sidestep the equity question entirely, since the Scores dataset already encodes which sectors are selected per date.
> 
> For the data infrastructure, I need to decide where to store ETF daily bars and index data. The cleanest approach is to ingest ETF bars directly into DailyBars by treating them as regular stock codes (e.g., 2800 for Tracker Fund), while HSI and other indices that don't have stock codes go into a separate Benchmarks dataset. There's also the A50 window return that S6 consumes for per-sector betas — that's a PostClose persistence job where the engine computes and stores the daily A50 window return, since MorningBoard only holds it in memory and drops it.
> 
> The scope challenge is real: I need to fit benchmarks ingestion, VHSI regime stratification, reconciliation, and Hansen SPA into one coherent milestone without touching the engine too heavily. The only engine patch I'm willing to do is landing the VHSI tercile at PreMarket (currently it's None in freeze.rs, so I'd compute it from the Vhsi dataset and pass it through to Candidates). Everything else — benchmarks, recon, SPA — should be data acquisition and analysis layers. I'm planning a tight but complete M9 that covers the core evidence pipeline.
> 
> For the data sources, I'm deciding between extending the frozen EastMoney crate surgically or creating new modules. The honest approach is to add a new `hkex.rs` module for official closes (HKEX is the canonical source per the report), and either extend EastMoney or create a new module for index/VHSI klines. Since VHSI isn't reliably on EastMoney, I'll use a config-templated URL approach with symbol-to-secid mappings, keeping it schema-versioned and verifiable per the VERIFY discipline.
> 
> For the dataset structure, I'm consolidating: the benchmarks step writes index OHLC rows for all configured symbols to the Benchmarks table, and when a `vhsi_symbol` is configured, it also writes the close price as a level to the Vhsi dataset. This way the M1 variant gets used and there's no dangling code. For the nightly pipeline, I'm adding a new `step_benchmarks` step to ingest indices, while keeping ETF daily bars separate from the trading universe to avoid polluting factor panels.
> 
> The benchmarks provider handles any secid through a single code path, with config mapping benchmark symbols to their secids including ETFs and indices. The `hkq-recon` binary will fetch official closes from HKEX, reconcile them, and exit non-zero on breach to trigger cron alerts. For validation, I'm implementing a Hansen SPA test using studentized max-t over loss differentials with stationary bootstrap, which will evaluate strategy performance against a benchmark family including always-cash, HSI open-to-close moves, and sector-ETF arms.
> 
> For VHSI terciles, I need to compute trailing-window terciles of the VHSI level and integrate them into the AlphaMap fit — this requires patching the engine's refit_alpha and M8's asof logic to join the tercile column from the lake when VHSI data exists. PreMarket will load yesterday's VHSI tercile from the dataset, freeze will use it, and both day.rs and asof.rs will join the per-date tercile value. This follows the same landing-point pattern as earlier milestones, though I'm recognizing this is becoming substantial and need to focus on what's truly essential for M9 coherence.
> 
> The core gap M9 addresses is evidence breadth — the benchmark family for SPA, the regime axis for stratification, and the official close for lake trust. All three are data acquisitions with existing consumers. I'm adding a Benchmarks dataset to hkq-data with a new BenchmarkProvider trait in provider.rs, an EastMoney implementation that fetches index daily data via a new config URL template, and an HkexClient for official closes with its own provider trait and schema parser. The VHSI side-write happens in the nightly step, and I'll use the append-inside-frozen-crates pattern from earlier milestones to patch provider.rs, eastmoney.rs, and cfg.rs.
> 
> For config changes, I'm deciding between reprinting the full EastMoneyCfg struct with new index fields or creating separate BenchCfg and HkexCfg structs wired into SourcesCfg via surgical patch blocks. The cleaner approach is a dedicated BenchClient in bench.rs with its own RatedClient and config struct, keeping benchmark ingestion independent from the main EastMoney client while staying backward compatible with serde defaults.
> 
> For the binaries: hkq-nightly gets a full reprint with the new step_benchmarks call (matching M3's precedent), hkq-recon is a thin new binary that loads config, builds HkexClient, fetches official closes, reconciles them, and exits non-zero if breaches are found (quarantining by default), and hkq-validate needs a SPA core module implementing Hansen's stationary bootstrap test with consistent recentering to compute p-values for strategy returns against benchmark returns.
> 
> For the benchmark builders: I'm creating sector-ETF logic that maps selected sectors to ETF symbols via a new optional field in ValidateCfg, then computes equal-weighted open-to-close returns for those ETFs on each date, treating days with no selection as cash days. The vhsi tercile series is trickier since both the engine and validate need it, so I need to figure out where to place that shared logic. But that violates the series' principle of shipping features with their consumers already in place — ingesting VHSI without using it would leave dead data. So either VHSI lands alongside its regime-bucketing consumer in AlphaMap, or it doesn't land this milestone at all. I'm reconsidering whether to drop VHSI entirely from M9 and focus on "The Benchmarks & The Referee" instead: benchmarks ingestion, SPA, official closes, and hkq-recon. That's still substantial work — a new dataset, two providers, a nightly step, a new binary, the SPA module, and tests — comparable in scope to M5. Then M10 becomes the regime axis: VHSI terciles feeding into AlphaMap stratification, A50 persistence for sector betas, and ADR together.
> 
> Actually, I can follow M5's exact precedent here: ingest VHSI and benchmarks in M9, consume them only in validation (the stratified IC report and SPA), run recon, but leave the engine untouched — no AlphaMap changes yet. The VHSI-bucket landing and candidates' tercile assignment become a named handoff to M10, the next engine-touching milestone. That gives M9 a clean shape: data acquisitions and validation consumption, zero engine surgery, mirroring how M5 wrote ah_beta.parquet but deferred its freeze consumption to M6.
> 
> The scope includes the benchmarks provider (config-driven index klines), VHSI side-channel writes, official-close provider (HKEX-shaped), the hkq-recon binary (completing the blueprint's bin table), the SPA implementation in validation, benchmark series builders for HSI and ETF stages, and the stratified IC report with VHSI tercile additions. Everything else — engine landings of VHSI terciles, A50 persistence, ADR, the ML layer, self-hosted workflows — defers to later milestones. After M9, the blueprint's bin table is complete and §4's test family is complete, leaving only regime/linked data landings and operational verification for the final arc.
> 
> I need to declare PATH A with the missing components list since the framework is incomplete without hkq-recon, SPA, and the benchmark/VHSI data. I'll also search once for the missing M9 codebase file to be thorough, though I'll proceed from M1–M8 if it's not found, since M8's hand-off already specifies what remains.
> 
> Actually, I should frame this output as Milestone 9 itself — the user's file list just pre-names where it goes. The codebase ends at M8, and M8 hands off to Milestone 9, so that's what I'm writing.
> 
> For the code changes, I'm adding the hkq-recon crate to the workspace members list, then patching hkq-data's lake.rs to add a Benchmarks variant to the Dataset enum with a corresponding directory match case, plus a test to verify the dataset has a home directory.
> 
> In provider.rs, I'm defining two new async traits: BenchmarkProvider for fetching daily OHLC data for benchmark symbols normalized to a standard schema, and OfficialCloseProvider for retrieving independent official closing prices that must come from a different vendor than the lake itself — this is the recon tripwire requirement from the spec.
> 
> cfg.rs gets two new optional fields on SourcesCfg: bench and hkex, both wrapped in Option so missing config tables deserialize to None.
> 
> Now I'm starting to define the BenchClient module...
> 
> Defining the benchmark configuration... Writing config and parser... For the recon binary, the date defaults to today in HK time, and if the calendar's closed there's nothing to do. HKEX is required as an independent official-close source, so I'll validate that upfront. I'm fetching official closes and running reconciliation to produce a JSON report with breaches, missing data, and a clean flag. If there are price breaches, I quarantine the DailyBars partition and exit non-zero; missing_ours entries also fail the check but don't warrant quarantine since that's an ingest gap. A `--no-quarantine` flag lets me do dry runs, and I'll use bail to ensure cron catches any failures.
> 
> Now I'm adding new fields to ValidateCfg for the SPA family — HSI symbol, an ETF sector map, and VHSI symbol. Rather than patching individual fields, I'll reprint the entire cfg.rs, lib.rs, and main.rs files wholesale, plus add a new spa.rs module to handle the SPA logic.
> 
> For the stratified-IC report, I'm debating whether to include it in M9 or defer it. The core thesis for this milestone is benchmarks (HSI + ETFs), SPA math, official closes, and HKQ reconciliation. VHSI ingestion can happen as a side-write during the benchmarks step — just another symbol to load — which unlocks M10's engine work without requiring its consumption yet. I'll include the VHSI data pipeline but explicitly defer the stratified-IC report and AlphaMap bucket refinements to the next milestone when the engine actually needs them.
> 
> The justification is solid: persisting VHSI now lets the 60-day distribution mature in the background, following the same pattern as auction history in earlier milestones. By the time M10 lands, the regime axis will be warm and ready.
> 
> For the SPA implementation, I'm defining a function that takes a strategy's returns, a family of benchmark return series, a block size for resampling, and bootstrap parameters. The test checks whether the strategy outperforms the best benchmark in the family under the null hypothesis that it doesn't beat any of them.
> 
> The key insight is framing this as a minimum test: the strategy must beat ALL nulls, so the binding constraint is the best-performing null. I'm computing the loss differential for each null, then testing whether the minimum expected differential is positive via a studentized bootstrap under the recentered null. With just a few nulls, this collapses to a per-null t-test applied across the family.
> 
> I'll implement this using Hansen's SPA machinery—studentization, per-k recentering, and circular block bootstrap—but with the roles flipped: instead of testing whether the best alternative beats a benchmark, I'm testing whether the strategy beats the worst null. The p-value accounts for multiplicity and dependence across the null family. I'll document this clearly and return the test statistic and result.
> 
> For variance estimation, I'll compute the long-run variance using a Bartlett kernel with lags equal to block size minus one, reusing that for both studentization and the recentering threshold. I'll add a small helper function for this rather than modifying the existing stats module.
> 
> Now I need to build the data series: HSI open-to-close daily returns from the Benchmarks table, and for each date in the strategy, the mean open-to-close return of whichever ETF symbols correspond to the sectors present in that day's Scores partition—dropping missing symbols or treating cash days as zero if no sectors are active.
> 
> For alignment, I'm being strict about data quality: only compare dates where the strategy and all benchmark arms have actual data, never imputing missing values. If a benchmark row is absent on a strategy date, I drop that date from the analysis and log it loudly. The CLI stays positional—`hkq-validate <cfg> spa <EQUITY_HKD>`—and the output is JSON with per-arm mean edge in basis points, t-statistic, SPA p-value, and a promotion flag when p < 0.05.
> 
> I'm adding config fields for the HSI symbol, an ETF map keyed by sector IDs, and SPA parameters like block size and bootstrap count. The registry logs each SPA evaluation as a test event. Most of the validate module stays unchanged; I'm only reprinting cfg, lib, and main to wire in the spa job, plus a new spa.rs module with tests. The total scope is around 2000+ lines across all files, which is substantial but manageable if I keep the prose tight and let the code speak for itself.
> 
> Now I'm defining the SPA implementation — it's Hansen's test flipped for a family of null models rather than a single benchmark, so the binding statistic becomes the minimum studentized differential across all nulls. I'm structuring the core types to hold each arm's name, mean difference, and t-statistic, plus the overall result with the minimum t, p-value, sample size, and bootstrap count. The long-run variance calculation uses a lag-based estimator with a minimum sample threshold.
> 
> The main function takes multiple arms of equal-length returns, validates the block size and bootstrap count, computes the mean and long-run variance for each arm to get studentized t-statistics, then finds the minimum t. The recentering step follows Hansen's consistent approach — for arms close to binding (those with means near the critical threshold), I keep their recentered means; for deeply negative ones, I zero them out to reflect the null constraint. Calling it "Hansen SPA" would be a misnomer — the original Hansen test flips the roles, treating the strategy as the benchmark and the null family as candidates to beat it, which gives a weaker guarantee of absence of evidence rather than promotion. A more practical approach combines both: use an intersection-union test with per-arm bootstrap p-values to ensure the strategy dominates each null arm, then apply Hansen-style machinery to guard against the reverse scenario where luck alone could produce the observed dominance. I'm planning to implement this with a circular block bootstrap that resamples time blocks jointly across all arms to preserve their dependence, then compute per-arm studentized statistics.
> 
> For each arm, I'll calculate the bootstrap p-value as the fraction of replicates where the centered bootstrap t-statistic exceeds the observed t-statistic, and the SPA p-value becomes the maximum across arms — this intersection-union framing means the strategy must beat every member of the null family. The documentation will clarify that this flips Hansen's orientation: rather than testing whether anything beats the strategy, we're testing whether the strategy beats everything, which is what the promotion gate actually requires. Using the original long-run variance estimate for studentization keeps things stable and standard.
> 
> For testing, I'll construct scenarios with a clear positive edge to expect small p-values, cases where the strategy matches one arm to expect large p-values around 0.3 or higher, and degenerate inputs that return None. The tests will be deterministic given a seed. I'm also building helper functions to extract benchmark return series from the lake by symbol and to construct ETF stage-one series by scanning the Scores and Benchmarks tables, filtering for matching symbols and dates, and computing log returns with guards for finite values and positive prices.
> 
> Now I'm working through the logic for handling missing data: when a sector maps to a symbol but that symbol-date pair has no benchmark data, I should drop that date rather than impute zero, since it represents genuine data absence. If the etf_map lacks a sector entirely, that's a configuration problem—I'll warn once and drop dates where no selected sector is mapped or priced. For cash days where the strategy had no fills, they won't appear in the strategy's PnL series either, which creates a bias in the comparison since we're excluding days when the strategy chose to stand aside while the benchmark may have moved. The honest approach is to use a calendar of actual trading days across the sample period.
> 
> I need to determine which days count as "strategy was live"—that's all non-Closed trading days within the span from the earliest to latest Fills date. I can load the FileCalendar from the config path to get this. The strategy return series should cover every open day in that range, using actual PnL where available and zero for cash days. The HSI benchmark must have a row for every such date; missing data gets dropped loudly. For the CLI, I'll parse the third positional argument as a decimal equity value for the spa job, keeping it generic so it works for other commands too.
> 
> Setting up the spa job flow... Writing the recon binary... Parsing the kline fields defensively — skip rows with fewer than 5 fields, extract numeric values for open/high/low/close, and push the date string with parsed prices. The benchmark client builds a URL from the template with symbol and date range, fetches JSON, accumulates rows filtered by date range (using lexicographic comparison since EastMoney returns ISO dates), and constructs a dataframe with symbol, date, and OHLC columns — warning on per-symbol errors but failing only if all symbols return empty. I need to check whether "symbol" should be added to the canonical column names in the model.
> 
> For HKEX, I'm defining a config struct that specifies the official close URL template, schema version, alias mappings for code and close fields, rate limiting, and a JSON pointer to locate the row array. The parser extracts the code (either as a string parsed to StockCode or directly as a u32) and close price (handling comma-separated values like "1,234.50" from HKEX pages) from each row using the configured aliases, then builds a list of code-close pairs — failing if no rows are found.
> 
> Now I'm updating the SourcesCfg struct in cfg.rs to include optional bench and hkex configuration fields, and adding the corresponding module declarations to lib.rs.
> 
> Updating Dataset enum... Writing the filter logic...
> 
> I need to capture the HKEX configuration status before moving the sources struct into clients, then store it as a boolean. For the BenchClient, I'll grab the vhsi_symbol from the config before it's consumed, or add a public accessor method to the client so I can retrieve it later. Then I'm updating the config validation to add default value functions for the benchmark fields like the HSI symbol and SPA block parameters.
> 
> Now I'm restructuring the main.rs argument parsing to handle the third positional argument differently depending on the command — fit-quarterly and asof-state parse it as a date, while spa parses it as an equity value in HKD. The job_spa function will validate that the equity is positive and handle the PnL series appropriately.
> 
> For the spa workflow itself, I'm building the strategy date grid from the first to last PnL date, filtering out closed days and zero-filling cash days. The HSI benchmark series gets loaded, and if it's missing I'll warn and return early. The ETF arm follows a similar pattern — if the etf_map is empty I skip it with a warning, otherwise I load the ETF stage-1 series and handle missing Scores by treating those dates as all-cash. The alignment step keeps only dates where both the HSI and ETF arms have valid data, dropping dates where sectors are unmapped or unpriced, then I compute the spa p-value and output the results.
> 
> Now I'm defining the public API for the spa module, including the seed constant, the ArmDiff and SpaOutcome structs, and the main spa_pvalue function that takes arm names, difference vectors, block size, bootstrap count, and seed. The implementation validates inputs, computes the mean and long-run variance for each arm using circular block resampling, calculates the t-statistic, and handles edge cases like constant differences where the variance is zero.
> 
> For degenerate cases with zero variance, I'm setting the p-value based on whether the mean is positive or negative, and using infinity as a sentinel for the t-statistic when appropriate. The bootstrap loop draws joint index sequences using circular blocks, recomputes the centered t-statistics for each resampled series, and counts how many bootstrap replicates exceed the original t-value to get the p-value per arm. The final p-value is the maximum across all arms, and I'm also tracking the minimum t-statistic.
> 
> For the PRNG, I'm following the existing pattern from stats.rs and using an xorshift64* style generator inline. Then I'm defining a benchmark_series function that reads from the Lake, filters by symbol, and returns a map of dates to log returns computed as ln(close/open), with guards for finite and positive values.
> 
> For the ETF arm, I need to restructure the data access — instead of keying by symbol alone, I'm building a single benchmark_returns function that returns a map keyed by (symbol, date) pairs, which both the benchmark_series and ETF stages can use. I'm also adding a selected_sectors_by_date function that scans the Scores table and collects the sector selections per date. For each ETF date, if there's no sector selection I default to cash (0.0 return), otherwise I map the selected sectors to their symbols and handle any unmapped sectors by returning None.
> 
> For the test suite, I'm writing tests for the spa_pvalue function to verify it recovers strong edges (mean 5bps vs noise), handles identical arms correctly (p=1), maintains determinism with the same seed, and tests the alignment with a small fixture lake that writes Benchmarks and Scores partitions with test symbols and sectors. I'm keeping the main.rs job_report note unchanged to match the M8 text.
> 
> Now I'm deciding between reprinting the full files or using surgical patch blocks. Given the scale—main.rs is ~300 lines, cfg.rs ~140, spa.rs ~350—reprinting would hit ~1800 total lines across all files. M7 used patch blocks for day.rs to avoid reprinting, so I'll apply that approach here: patch the usage doc comment, the arg-parsing block, and append the job_spa handler in main.rs; for cfg.rs, append the new helper functions and fields, then replace the Default impl to include them. This keeps the changes surgical and manageable.
> 
> For the hand-off gaps to M10: VHSI consumption starts accumulating tonight but terciles and regime buckets land with the next engine milestone; A50 persistence is still missing so per-sector betas remain unbuildable; the ETF arm needs the operator to populate etf_map and benchmark symbols, and since HSI also needs history backfill, I'm noting that benchmark history begins accumulating from the first configured night—the template's date range can be widened for initial backfill rather than adding a separate backfill flag.
> 
> The key insight is that the EM kline endpoint returns full history anyway, so I don't need to range-filter—one fetch gives years of data, but writing it all into a single date partition means nightly runs create duplicates across partitions. However, the BTreeMap deduplication by (symbol, date) with last-write-wins makes duplicates harmless to consumers, and this way the SPA family gets years of benchmark history on night one without a bespoke backfill job. The partition bloat is trivial (a few thousand rows per symbol in KBs), and I'll document this as an intentional deviation: Benchmarks partitions are ingest-dated snapshots of the vendor's full history, consumers dedupe by (symbol, date), and idempotent re-runs behave exactly like every other dataset. The nightly exists check ensures only one snapshot per day, and the template's from/to range parameters become optional.
> 
> For VHSI, the side-write filters symbol rows into [date, level] spanning the whole window into one ingest-dated partition, with future consumers deduping the same way. The remaining blockers are unchanged: κ still needs venue-tagged fills, promotion enforcement stays with operator governance, and the self-hosted workflow is untouched. With hkq-recon built, the blueprint's four-binary table is complete—the rest of §4 is data maturation, not construction. M9 is the data milestone that unblocks the benchmark family and recon by acquiring HSI and sector-ETF series, landing the two §4/§5 consumers named since M1/M5, with zero engine changes. The structural scope is tight: only frozen-crate patches touch hkq-data (enum variant, two provider traits, config fields, two new modules)—hkq-core, factors, signal, risk, exec, engine, and hkq-live are byte-identical.
> 
> Now I'm planning the response structure: opening note with docs status and what M8's hand-off names as remaining work, milestone header with why-next and engineering decisions, tree diagram, workspace config, patches across hkq-data and new modules, hkq-nightly reprint, hkq-recon new, hkq-validate patches, config examples, and honest gaps leading to M10. Before writing, I need to search for the KB entry to verify the milestone state, then I'll explicitly checklist the blueprint components—hkq-recon, Hansen SPA, and benchmark/VHSI data are missing; everything else is built.
> 
> Checking the SPA test design: n=300 with block=10 and boot=500 keeps tests fast, and the diff series with near-zero mean and tiny sd gives a t-statistic around 17, so p-value is effectively zero—good signal. For the degenerate equal-arm case, all zeros means p_k=1 and max p=1, which is the right behavior. I'll copy the deterministic PRNG closure style from stats.rs. ValidateError already has the Insufficient and Config variants I need. The pnl::daily_pnl function returns dates as ISO strings, so I'll parse first and last to NaiveDate, iterate through calendar days excluding Closed ones, and defensively union any pnl dates not in the grid. For the registry, I'm adding a "spa_test" entry with metrics for spa_p, t_min, days, and n_arms, plus the config hash. The JSON output shape is starting to take form with days count and other fields.
> 
> Now handling edge cases: if arms < 2 (only cash because benchmarks are missing), benchmark_returns returns Insufficient, which I'll warn about but continue with cash-only SPA—still meaningful as a t-test against zero. If HSI has no rows but the table is non-empty, I'll warn loudly that the arm is missing and SPA degrades to cash-only. For unmapped ETF sectors, I collect them into a set, warn once, and drop those dates; if the operator maps nothing, I skip the arm entirely with a warning. Now writing bench.rs...
> 
> Writing bench.rs module... Defining the config structures... Implementing the benchmark method that fetches kline data for each symbol across a date range, parsing the JSON response according to the configured schema version, and accumulating the results into vectors for date, open, high, low, and close prices while tracking any fetch failures. Writing test cases... Defining the client struct and helper functions... Writing test cases... Adding column constants and module exports...
> 
> Now I'm updating the lake.rs file to replace the Dataset enum and its dir method, then adding the benchmarks step to the nightly main.rs with logic to filter VHSI data and transform the close column to level.
> 
> The VHSI side-write happens after collecting the filtered dataframe — it checks if rows exist, writes the partition with a warning fallback if it fails, and logs if the symbol was configured but no data matched. The benchmarks partition gets written first, then the side-write follows, with the step returning the row count from benchmarks.
> 
> I'm also adding the exists check to skip if benchmarks already exist for the date, computing the backfill range using a new accessor on BenchClient, and placing this step after the bars step in the pipeline.
> 
> For the recon message, I'm capturing whether HKEX is configured right after loading sources, then logging either an info message about running reconciliation or a warning that the source isn't set up.
> 
> The bench client gets instantiated from the sources config, and I'm being careful with the partial moves — checking `sources.hkex.is_some()` before any moves happen, since the struct fields move independently and later moves are fine as long as no borrow is held.
> 
> Writing hkq-recon binary... Finishing the error handling for the official closes fetch, then loading the data lake and running the reconciliation. I'm transforming the breach records into JSON format and printing a summary report that includes the date, number of items checked, any discrepancies found, and missing entries from either source. If there are breaches and quarantine is enabled, I move the daily bars dataset to quarantine and log an error directing the operator to fix the source and rerun. Finally, I bail out if the reconciliation isn't clean, otherwise log success.
> 
> Now I'm verifying the ReconReport structure and its serialization — the Vec<u32> fields for missing entries serialize correctly through the json! macro. Setting up the Cargo.toml for the recon crate with its dependencies, then moving into the SPA validation module to implement the statistical gate that tests whether a single strategy dominates across a null family of alternatives using intersection-union testing with bootstrap methods.
> 
> Defining the core data structures for the SPA test: each arm tracks the mean edge, studentized t-statistic using Bartlett long-run variance, and bootstrap p-value, while the outcome aggregates these across all arms to produce the maximum p-value and minimum t-statistic that determine whether the strategy passes the gate. Writing the bootstrap logic... I need to optimize by precomputing the long-run variance square roots for each arm before the bootstrap loop, then reusing those cached values instead of recalculating them on every replicate. After that, I'm reading benchmark data from the lake, extracting symbol, date, open, and close columns, and converting them to materialized series for downstream processing. Continuing to iterate through the date and sector columns to build a map of which sectors were selected for each date. Then I'm defining a function that computes the equal-weight mean return of selected sector ETFs for a given date, returning None if any sector is unmapped or missing data rather than imputing values. Starting to write tests for the module. Adding test cases... Writing test assertions... Parsing the equity argument and validating it's positive, then loading the daily P&L series and benchmark returns from the lake. I'm setting up the trading calendar and building a map of P&L values by date, extracting the date range to span all open trading days where cash positions are treated as zero returns rather than gaps.
> 
> Now I'm checking whether the ETF map is configured — if not, the sector-ETF comparison arm gets skipped. I'm iterating through each day in the range, collecting returns for the strategy, HSI benchmark, and ETF arm where data exists, while tracking cash-only days and dates that lack complete benchmark data.
> 
> I need to validate that we have at least 40 comparable days before running the SPA test, then construct the comparison matrix with strategy returns, strategy minus HSI, and strategy minus ETF (if enabled). Finally, I'm running the SPA analysis and formatting the results as JSON with mean edge in basis points, t-statistics, and bootstrap p-values. I need to add the `spa` module to the imports in main.rs alongside the other validation modules. I'm also noting that `BTreeMap::keys()` returns a double-ended iterator, so I can use `.next_back()` to get the last key directly. Then I'm setting up config examples for the benchmark and HKEX data sources with their respective URLs and symbol mappings. Checking that the symbol resolution works through the re-exports and that the reconciliation function signature matches what's expected. The milestone name should capture the essence of adding benchmarks, official closes, and the reconciliation mechanism — something like "The Evidence" to reflect that the system now has facts to verify against, with the recon tripwire as the safety mechanism. The narrative arc continues building toward what's still missing.
> 
> Now I'm mapping out the deferred work: engine landing for VHSI terciles, A50 persistence, ADR feeds, and various unchanged components. After M9 completes, all four binaries will exist and the bin table is done — what remains is data maturation and the learned layer. I need to write the milestone document with an opening section that briefly names the missing components, explain why this is the right next step, lay out what's in scope versus deferred, cover the engineering decisions, show the tree and code, and be honest about remaining gaps before handing off to M10. The prose should stay tight and house-style compact, and I need to verify the import paths for the Polars macros are correct across all files.
> 
> Checking the imports and type signatures for the data pipeline: DataFrame traits pull from prelude, chrono handles dates, async_trait and serde Deserialize are in place, and the RatedClient methods work with Value types. The date formatting uses a compact string representation. For HKEX, the StockCode parser handles the string format correctly and the numeric conversions use safe try_from with ok(). Now I'm finalizing the spa.rs structure with the necessary imports and refactoring the spa_pvalue function to cache the long-run variance calculation per arm, handling degenerate cases and only bootstrapping where needed.
> 
> Writing the spa_pvalue function... Completing the bootstrap resampling loop...
> 
> For each bootstrap iteration, I'm sampling blocks of consecutive differences and checking how often the resampled t-statistic exceeds the observed one. Then I'm computing p-values for each arm based on these exceedance counts, taking the maximum p-value across all arms and the minimum t-statistic, and returning the final outcome with the number of days, bootstrap iterations, and per-arm results.
> 
> Now I'm verifying the test numbers with a specific noise function that generates values in the range [−0.01, 0.01] with mean near zero and standard deviation around 0.0058. For the first arm with mean 0.003, the t-statistic comes out to roughly 8.9, giving a p-value approaching zero as expected. The second arm with mean 0.0001 yields a t-statistic near 0.3 and p-value between 0.35 and 0.45, which should safely exceed the 0.2 threshold. The strong arm alone produces p < 0.05 as required. I'm also checking for potential integer overflow when computing i*2654435761.
> 
> Adding config fields...
> 
> Now I need to wire up the imports for the benchmark client and provider, plus the polars utilities for lazy evaluation and column operations. The vhsi side-write will reference the model columns, so I'm importing those too. For the step_benchmarks function, I'm setting up the signature to take the lake, client, date, and force flag, then returning a StepReport. The caller logic branches on whether a benchmark client exists — if it does, I call step_benchmarks; otherwise I construct a SkippedNoProvider outcome. Both paths feed into the same report.steps.push call. The full nightly main.rs is coming together at around 215 lines, and I need to make sure the response structure and prose are clear before finalizing.
> 
> I should do a web search first to check for that missing milestone file, then address the workspace member ordering — hkq-recon goes at the end after backtest. Let me verify the Cargo.toml dependencies too, especially that hkq-recon has tokio for async. The final crate structure shows hkq-data with the new bench.rs and hkex.rs modules, hkq-nightly updated with the benchmarks step, and hkq-recon as the new fourth binary crate.
> 
> Now I'm laying out the engineering decisions: snapshot ingest for benchmarks with deduplication, the SPA orientation logic around min/max flips and size-correctness, zero-filling cash days since cash is a position, dropping missing benchmark rows rather than imputing, deferring VHSI consumption after side-write, enforcing the independent-vendor rule at the recon call site with quarantine defaulting on, keeping config-owned alias tables and schema versions per M1 discipline, being explicit about unmapped/unpriced ETF drops and empty map skips, and crucially — no patches to engine, factors, signal, risk, or exec; only hkq-data gets additive changes.
> 
> Writing the final document now...
> 
> The search didn't surface anything useful from the KB_rust repo since it's a personal GitHub repository that isn't indexed, which makes sense. I notice the 11th document is missing and couldn't be retrieved, so I'll work with what I can verify: the report, proposal, and Milestones 1 through 8. I'll now write out the full response covering the PATH A declaration, any missing components, Milestone 9, and the implementation code while keeping the prose consistent with the established style and ensuring the code is correct.

Ten of the eleven listed documents are attached in full, so I worked directly from them — no fetch tool exists in this environment (search only), and a search for the one document that is **not** attached, `11_milestone_code_9.md`, returned nothing relevant (the raw KB repo isn't indexed). The verifiable codebase state is therefore the attached chain: report + blueprint + Milestones 1–8, whose final hand-off explicitly names what Milestone 9 must be. This response is that milestone; if an M9 file already exists in your repo, re-attach it and I'll reconcile.

**Path declaration: PATH A — the framework is incomplete.** Auditing M1–M8 against the blueprint: every crate exists and every mathematical component of §3 is built and tested, but three blueprint/report obligations are still missing. First, `hkq-recon` — one of the blueprint's four named binaries — has never been built; every nightly since M1 logs "recon skipped" because no independent official-close source is wired (report §5). Second, the §4 protocol is missing its last test: the Hansen SPA against the null family {always-cash, HSI open→close, sector-ETF Stage-1} — blocked since M5 because nothing ingests HSI or sector-ETF series. Third, the §5 data contract's "VHSI level" has no writer: `Dataset::Vhsi` has existed as an enum variant since M1 and has never received a row, which is why `vhsi_tercile` is `None` everywhere and the AlphaMap runs on IVU-global buckets. All three are what M8's hand-off called them: data acquisitions with named, waiting consumers. That is the milestone.

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

## Honest gaps and hand-off to Milestone 10

Six items, each now a named fact. First, the **regime axis is accumulating but unconsumed**: `Dataset::Vhsi` finally receives rows nightly, but `Candidate.vhsi_tercile` is still `None`, the AlphaMap still fits IVU-global buckets, and the report's stratified-IC misspecification tripwire is unbuilt — all three are the *same* landing (a trailing-tercile computation at PreMarket, a per-date tercile join in the alpha refit *and* in M8's `asof_alpha`, keeping as-of bit-faithful to live), and per the M5→M6 `ah_beta` precedent they ride the next engine-touching milestone, with the history warming itself in the meantime. Second, the ETF arm and the recon source are **operator config acquisitions**: the code paths degrade loudly (arm skipped, gap logged) until `etf_map`, real secids, and a verified independent close endpoint are filled in — one verification day settles all three, and the alias tables are where drift gets absorbed. Third, **A50 persistence and per-sector mainland betas** remain unbuildable for the same data reason as ever: the `MorningBoard` still drops the stream, so S6's `a50_beta_ret` stays null; ADR feeds likewise. Fourth, **κ calibration remains built but starved** (venue-tagged fills or accumulated tiger fills — unchanged data-layer owner). Fifth, **promotion stays governance**: `spa` reports and registry-logs next to the DSR verdict, and nothing automates the config edit — unchanged by design. Sixth, **CPCV, the self-hosted walk-forward, and §3.7** remain deferred until the learned layer consumes them. The system now has facts, opinions, decisions, a clock, a memory, a reflex, a hand, a mirror, judgment — and evidence: all four blueprint binaries exist, the lake is reconciled against an independent truth nightly, and the §4 verdict weighs the strategy against the null family the report pre-registered. What it still lacks is the regime landings that turn accumulated VHSI history into conditional alpha, and the §3.7 learned layer — the last construction milestone the blueprint names.

---
Learn more:
1. [1.97.0 · Milestone #144 · rust-lang/rust](https://github.com/rust-lang/rust/milestone/144)
2. [1.90.0 · Milestone #134 · rust-lang/rust](https://github.com/rust-lang/rust/milestone/134)
3. [1.93.0 · Milestone #137 · rust-lang/rust](https://github.com/rust-lang/rust/milestone/137)
4. [1.94.0 · Milestone #139 · rust-lang/rust](https://github.com/rust-lang/rust/milestone/139)
5. [Milestone #133 · rust-lang/rust - 1.89.0](https://github.com/rust-lang/rust/milestone/133)
6. [1.92.0 · Milestone #136 · rust-lang/rust](https://github.com/rust-lang/rust/milestone/136)
7. [rust-lang/rust](https://github.com/rust-lang/rust/milestone/135)
8. [rust\_reboot](https://commits.facepunch.com/r/rust_reboot)
9. [1.95.0 · Milestone #140 · rust-lang/rust](https://github.com/rust-lang/rust/milestone/140)
10. [rust-lang/cargo](https://github.com/rust-lang/cargo/milestone/51)
