*Thinking...*

> The user is asking for a detailed technical research blueprint on intraday sector rotation and stock selection strategies in the Hong Kong market, including recent academic foundations, precise mathematical formulations for algorithms, and specific data sources like TigerOpenAPI and Xueqiu. This is a legitimate quantitative finance research project focused on algorithmic design.
> 
> I should search for recent 2024-2025 arXiv papers on intraday momentum, sector rotation, lead-lag effects, and Hong Kong market microstructure to ground the response in actual literature, then synthesize a comprehensive technical document with proper citations and mathematical specifications.
> 
> I'm refining my search strategy to find more specific resources on production-grade quantitative frameworks and recent deep learning approaches to stock ranking, including sector rotation techniques.
> 
> I'm pulling together research on intraday momentum patterns, noting that while the Gao et al. study shows the first half-hour predicting the last half-hour with volatility effects, the Hong Kong ETF data reveals no intraday momentum—a crucial constraint that means the strategy needs to focus on cross-sectional effects instead of time-series index momentum. I'm also tracking volume-based information uncertainty metrics from the Chinese market showing strong predictive accuracy.
> 
> Beyond that, I'm seeing several complementary patterns: cross-sectional reversals in emerging markets where first-half returns negatively predict the rest of the day, end-of-day reversals, and lead-lag effects through industry information diffusion and index futures tick dynamics. The momentum spillover research is particularly useful—peer overnight versus intraday decomposition gives me a direct signal for constructing the open: high peer overnight returns followed by full intraday reversal, or high peer intraday returns sustaining into the focal stock's intraday period. I'm also noting daily momentum exists in China, and I have references to Qlib's factor libraries, transformer-based ranking models with IC metrics, overnight-daytime tug-of-war dynamics, and gap analysis work to explore.
> 
> For Hong Kong specifics, the market structure matters significantly: continuous trading 9:30–12:00 and 13:00–16:00 with a lunch break, pre-opening auction 9:00–9:30, closing auction 16:00–16:10. The user's "09:30 AM open" aligns with the continuous session start. Stamp duty is critical for cost modeling—currently 0.1% on both sides (reduced from 0.13% in November 2023), making round-trip costs roughly 13–26 basis points, which is substantial for intraday strategies. Even though they want signal generation without leverage, I should factor cost-awareness into the objective to ensure statistical robustness. Stock Connect southbound flows and HSICS industry classification are also relevant for the Hong Kong context.
> 
> Now I'm organizing the research blueprint, starting with the HKEX market microstructure—trading sessions, auction mechanics, stamp duty costs, and the market volatility control mechanisms. Then I'll synthesize the theoretical foundations, particularly around intraday momentum patterns, noting that while prior research shows momentum effects in some markets, the Hong Kong index itself doesn't exhibit intraday momentum, which means I should focus on cross-sectional dynamics instead.
> 
> I'm also incorporating cross-sectional reversal patterns in emerging markets, volatility and volume conditioning effects, lead-lag dynamics between stocks and their peers (especially how overnight moves in peer stocks predict intraday continuation in focal stocks), southbound flow predictability, daily momentum effects, and end-of-day reversals. The key insight is distinguishing between overnight-driven gaps that reverse intraday versus intraday peer momentum that continues.
> 
> For the algorithmic specification, I'm designing a multi-stage approach: first filtering the universe by liquidity and eligibility criteria, then aggregating stocks into sectors, and at market open scoring sectors based on overnight gap decomposition, cross-sector lead-lag relationships estimated with shrinkage methods, and prior-day late-session momentum.
> 
> I'm also recognizing a critical timing edge with A-share markets: the mainland call auction closes at 9:25 HKT, five minutes before Hong Kong continuous trading begins at 9:30, so the A-share opening price is observable before HK opens. Similarly, overnight US ADR returns for dual-listed stocks predict HK opens, and the gap between ADR-implied prices and HK pre-open indicative equilibrium prices provides a signal. I'm defining an information uncertainty measure based on opening volume percentiles and intraday dispersion.
> 
> At the stock level, I'm scoring based on idiosyncratic gaps (stock gap minus beta-adjusted sector gap), relative strength within the sector over the first few minutes, liquidity-adjusted volatility using Amihud scaling and semivariance decomposition, order imbalance signals from the pre-open session, and peer spillover effects. I'm combining these signals through cross-sectional rank normalization, z-score standardization, winsorization, and orthogonalization, then weighting by information coefficient or using machine learning ranking with pairwise loss functions.
> 
> For portfolio construction, I'm selecting top sectors and stocks within them, weighting inversely by volatility, respecting participation constraints relative to average daily volume, and executing entries after a confirmation window around 9:45 with exits before close or at the close auction using VWAP slicing.
> 
> Finally, I'm validating robustness through purged walk-forward cross-validation with embargo periods, deflated Sharpe ratios, White's reality check, and information coefficient metrics. Now I'm laying out the precise formulas: overnight gap as log return ratio, standardized by EWMA volatility; first and last half-hour returns for the HK session structure (11 half-hours total across the morning and afternoon sessions); realized volatility with bipower variation to isolate jump components; semivariance split by sign to capture directional jumps; Amihud illiquidity ratio; liquidity-adjusted volatility scaling realized vol by illiquidity percentile; and volume surprise as log deviation from the median open volume over a lookback window.
> 
> I'm also defining intraday volume uncertainty as the coefficient of variation of volume shares relative to seasonal patterns, adapting the MDPI paper's framework operationally; lead-lag dynamics using shrunk cross-correlation matrices with the Hayashi-Yoshida estimator for asynchronous data; sector momentum spillover from peer intraday returns; Southbound flow z-scores over a 60-day trailing window; and for dual-listed stocks, the A-H premium change and A-share opening signal as predictors of HK open drift.
> 
> Finally, I'm standardizing all factors cross-sectionally via rank-z transformation for the composite signal. The transaction costs break down to roughly 25–45 basis points per round trip when accounting for stamp duty, trading fees, levies, settlement charges, and bid-ask spreads, which means I need to target conditional alpha above 30 basis points to justify trades—this naturally pushes the strategy toward high-volatility names and lower turnover. I should also note that half-day trading sessions ending at noon matter for the data pipeline, and HKEX's volatility control mechanism (±10% in 5 minutes triggers a cooling-off period) plus opening price limits (±15% in pre-auction) are execution constraints worth tracking. For the report format, I'll structure this as prose with headers rather than bullet lists to keep it readable as technical documentation. I'm laying out the theoretical pillars for the strategy: lead-lag dynamics within the Hong Kong market and across linked pairs (mainland A-shares, US ADRs), the role of southbound Stock Connect flows as a predictive signal, daily momentum patterns with end-of-day reversals, and evidence that naive gap-fade approaches can fail. Then I'm moving into the algorithmic core—defining the sector-level feature set with six components: gap significance, a decomposed momentum measure that treats intraday continuation separately from overnight reversal, late-day informed flow, a lead-lag network score using correlation-based ranking, southbound flow intensity, and linked-market signals from A-share auctions and mainland indices at the open.
> 
> Now I'm specifying the stock-level features within selected sectors: idiosyncratic gap residuals, auction-phase order imbalance metrics, opening volume surprises, volatility regime indicators, liquidity-adjusted risk measures, peer spillover effects, stock-level southbound flows, and execution costs. I'm combining these through rank normalization and orthogonalization, then weighting by information coefficient and information ratio with optional machine learning via gradient boosting with ranking losses and cross-validation. The entry logic hinges on a confirmation window from 9:30 to 9:45, with actual entry at 9:45 based on interactions between gap magnitude and volume surprise conditional on volatility regime, followed by portfolio construction using inverse-volatility weighting with position limits and hard stops at market close, plus a meta-labeling filter that compares expected alpha against transaction costs.
> 
> For validation, I'm applying rank information coefficient and ratio metrics with Newey-West adjustments, purged cross-validation with embargo periods, Sharpe ratio diagnostics, stress tests across regimes, and multiple-testing corrections, while ensuring label design avoids look-ahead bias and includes a live shadow trading period. The data pipeline requires end-of-day OHLCV with sector classifications and southbound flow data, plus intraday one-minute bars with order book snapshots and tick-level trades for volatility estimation.
> 
> I'm sourcing from Tiger API for real-time market depth and auction data, supplemented by YFinance for historical context (though with caveats on survivorship and delay), and Xueqiu for mainland flows, while recommending HKEX's OMD-C feed for production use. The operational schedule runs from pre-market factor computation through a structured intraday window—auction ingestion at 09:00–09:22, mainland open at 09:25, confirmation phase 09:30–09:45, entry at 09:45, then exit 15:45–16:00—with pre-computed seasonal adjustments and point-in-time sector mappings to support execution.
> 
> Now I'm compiling the citation map for the core findings: lead-lag effects decay to millisecond scales (suggesting pairwise arbitrage has already compressed them), so I'm focusing on daily and coarser horizons; southbound flows show strong weekly predictability at 25.84% annualized long-short returns driven by demand shocks, though daily net buying lost predictive power after 2017; industry lead-lag and spillover effects operate through unified analyst coverage and news co-mention; overnight-intraday asymmetries follow a tug-of-war dynamic; end-of-day reversals and gap patterns provide additional signals; and for the technical stack, I'm referencing Qlib's Alpha158/360 expression engine and ranking loss functions for transformer-based stock selection, with position sizing mechanics anchored to a 15% volatility band and value-at-risk constraints, plus the 0.1% stamp duty on both sides effective November 2023.
> 
> For the write-up itself, I'm targeting a dense technical prose style around 3,500+ words with display math for the overnight-intraday decomposition (open-to-close and close-to-open log returns), gap z-scores computed via EWMA with a 0.94 decay factor, and sector-level gap aggregation using value weighting. I'll keep everything in prose with headings rather than bullet points, and I need to note that Hong Kong's T+2 settlement with broker-allowed reuse of unsettled proceeds affects capital recycling constraints differently than US rules, plus trading hours are 9:30–12 and 13–16.
> 
> For the lead-lag analysis, I'm setting up a rolling 250-day correlation matrix with Ledoit-Wolf shrinkage to identify which sectors lead others, then constructing a signal as a weighted sum of lagged residual returns from leader sectors. For intraday asynchronous tick data, I'll use the Hayashi-Yoshida cross-covariance estimator to handle non-synchronous trading, compute the lead-lag ratio as an asymmetry statistic, and cite the relevant result. Volume seasonality gets captured by comparing current interval volume to a 20-day median baseline, with the surprise metric as the log ratio. I'm also defining an operational IVU measure based on realized dispersion of relative volume shares in the opening period.
> 
> Now I'm incorporating liquidity-adjusted volatility using the Amihud illiquidity ratio scaled by a power parameter, then computing semivariance from 5-minute returns to capture downside risk and signed jump volatility. For auction imbalance, I'll extract the order imbalance ratio from the final snapshot if available, or fall back to the intraday price drift between 9:10 and 9:20. Finally, I'm rank-transforming all signals to normality and applying sequential orthogonalization to decorrelate them before weighting.
> 
> Weights are proportional to the information coefficient information ratio with shrinkage toward equal weighting, and I'll use Spearman rank IC annualized by √252. For risk-adjusted returns, I'm computing the probabilistic Sharpe ratio with a deflated benchmark that accounts for multiple testing and skewness-kurtosis adjustments. On costs, Hong Kong stamp duty runs 20 basis points round-trip plus exchange and regulatory fees totaling around 22–23 basis points, so I need conditional alpha of at least 50 basis points to break even—though ETF implementations could cut that to roughly 2 basis points if sector ETF liquidity permits.
> 
> For the sector leg, I'm considering Hong Kong sector ETFs like the HSCEI or HS TECH tracker, which sidestep stamp duty entirely. On settlement, T+2 cash accounts in Hong Kong typically allow immediate reuse of proceeds, though this is broker-dependent. Volatility circuit breakers kick in at ±10% over 5 minutes for HSI constituents, triggering a cooling-off period, and the position band sits at 15%. The Hong Kong session runs 9:30–12:00 and 13:00–16:00 with a closing auction window, giving me 11 intraday half-hour bins to work with. Mainland A-shares open via call auction from 9:15–9:25 with continuous trading starting at 9:30, and CSI300 futures also open at 9:30, so there's a 5-minute lead before Hong Kong's continuous session begins. SGX A50 futures trade nearly around the clock and provide overnight price discovery before the 9:30 open.
> 
> For Hong Kong-listed names with US ADR listings like Alibaba or JD.com, the overnight US close creates an implied gap I can calculate by adjusting the ADR price for the FX rate relative to the Hong Kong close. This is standard practice. I'm planning to keep citations concentrated in the literature and cost sections, while sections with my own derivations won't need them. I'll minimize direct quotes and paraphrase everything with proper attribution. For the data specification, I'll use structured prose with inline enumerations rather than tables to keep the format clean and readable.
> 
> Now I'm drafting the full report with a clear structure: starting with market constraints and their mathematical implications, then theoretical foundations covering intraday momentum, cross-sectional signals, volume uncertainty, lead-lag dynamics, and southbound flows, followed by the algorithmic specification with notation, universe construction, and signal definitions. The document will be dense and rigorous, around 4000 words, with H2/H3 headers and display equations where needed. Since this is institutional research, I can note data sourcing risks like web scraping terms of service as an engineering consideration without needing retail disclaimers.

# Intraday Sector Rotation and Stock Selection on HKEX — Mathematical Specification (v1.0)

**Scope.** Long-only, cash-settled, zero-overnight-inventory strategy on Hong Kong equities. Decision epoch: the 09:30 continuous-session open; forced flat by the close. This document specifies (i) the theoretical priors the signal stack rests on, (ii) exact factor mathematics for a two-stage sector→stock selection cascade, and (iii) the data contract for the Rust implementation. All timestamps are HKT (UTC+8, no DST).

---

## 1. Market-Structure Constraints and Their Mathematical Consequences

The design space is dictated by HKEX microstructure, so we fix these facts first.

**Session grid.** HKEX continuous trading runs 09:30–12:00 (morning session) and 13:00–16:00 (afternoon session)[[2]](https://longbridge.com/en/academy/hk-stocks/blog/hkex-trading-hours-morning-and-afternoon-sessions-100104), giving 5.5 hours of continuous trading partitioned in this spec into eleven 30-minute bins \(\tau \in \{1,\dots,11\}\) with a lunch discontinuity between \(\tau=5\) and \(\tau=6\). A Pre-Opening Session (POS) runs 09:00–09:30 as a single-price auction in which submitted orders are matched at one price[[2]](https://longbridge.com/en/academy/hk-stocks/blog/hkex-trading-hours-morning-and-afternoon-sessions-100104); matching occurs at the final Indicative Equilibrium Price (IEP) during a random matching period from 09:20 to 09:22[[6]](https://pic.bankofchina.com/bocappd/macau/202010/P020201019358880833694.pdf), and the IEP — the price maximizing executable volume — is recalculated on every new order until the end of the auction[[6]](https://pic.bankofchina.com/bocappd/macau/202010/P020201019358880833694.pdf). The POS therefore emits an observable state trajectory \(\{IEP_t, IEV_t\}_{t \in [09{:}00, 09{:}22]}\) that is a legitimate, non-anticipative feature source for the 09:30 decision. Two guardrails must be encoded as hard constraints: the opening auction price is constrained to a ±15% fluctuation band[[7]](https://www.futunn.com/en/learn/detail-before-entering-the-market-understand-the-trading-rules-of-the-hong-kong-stock-market-83831-230556033), and a Volatility Control Mechanism applies to Hang Seng Index and HSCEI constituents, triggering a 5-minute cooling-off period if price moves more than 10% within 5 minutes of continuous trading[[7]](https://www.futunn.com/en/learn/detail-before-entering-the-market-understand-the-trading-rules-of-the-hong-kong-stock-market-83831-230556033).

**The cost floor is the binding constraint.** Effective 17 November 2023, stamp duty on Hong Kong stock is 0.1% per contract note, charged to buyer and seller individually[[8]](https://www.china-briefing.com/doing-business-guide/hong-kong/taxation-and-accounting/stamp-duty-hong-kong). An intraday round trip (we pay buyer duty on entry and seller duty on exit) therefore incurs a deterministic 20 bps of duty, plus exchange/levy fees (~1.5–2.5 bps round trip) plus twice the effective half-spread plus impact. Define the per-name cost floor

$$
c_i = 20\,\text{bps} + c^{fees} + s_i + \kappa \,\sigma_i \sqrt{\frac{q_i}{ADV_i}},
$$

where \(s_i\) is the trailing-median quoted spread, and the last term is a square-root impact model with participation \(q_i/ADV_i\). Realistically \(c_i \in [25, 50]\) bps. **Every downstream design choice — concentration, conditional trading, confirmation windows — exists to clear this hurdle**; the strategy must be built to trade *conditionally and concentratedly*, not diversified-and-always-on. Note that ETFs are stamp-duty-exempt, so a sector-ETF expression of the Stage-1 signal (e.g., via the HS TECH tracker) has a cost floor roughly an order of magnitude lower and should be retained as a benchmark implementation against which single-name alpha must justify itself. Settlement is T+2 for Hong Kong equity trades[[8]](https://www.bitget.com/wiki/when-does-the-hong-kong-stock-exchange-open); same-day reuse of sale proceeds in a cash account is broker-dependent and must be a config flag in the execution layer.

---

## 2. Theoretical Foundations

### 2.1 Time-series intraday momentum — and its documented failure at the HK index level

The canonical result is Gao–Han–Li–Zhou: morning session returns significantly predict afternoon and closing session price directions ("market intraday momentum")[[3]](https://www.mdpi.com/2227-7072/14/2/47), with two robust conditioning facts: predictability is increasing in volatility[[6]](https://assets.super.so/e46b77e7-ee08-445e-b43f-4ffd88ae0a0e/files/ee7dac49-530b-4950-b5d0-e0b5eee08f2e.pdf) and is far stronger in recessions/stress regimes, where the R² rises more than six-fold relative to expansions[[6]](https://assets.super.so/e46b77e7-ee08-445e-b43f-4ffd88ae0a0e/files/ee7dac49-530b-4950-b5d0-e0b5eee08f2e.pdf). Critically for us, the APAC replication shows that in ETFs across China, Hong Kong, Japan, Singapore and South Korea, intraday momentum is evident mainly in China and Japan, while Hong Kong and Singapore show no intraday momentum[[1]](https://www.sciencedirect.com/science/article/pii/S0927538X2300152X).

**Design consequence №1:** in Hong Kong, the alpha is *not* in timing the index intraday. The problem must be posed **cross-sectionally** — ranking sectors and names against each other at the open — with time-series signals used only as conditioning variables. This is the mathematical justification for the two-stage relative-ranking architecture below.

### 2.2 Cross-sectional opening-signal predictability

In emerging markets adjacent to ours, first half-hour returns *negatively* predict rest-of-day returns in the cross-section[[2]](https://www.sciencedirect.com/science/article/pii/S2214845023000029) — i.e., cross-sectional *reversal* — while time-series continuation coexists at the index level in China. The opening interval is special because many announcements accumulate between the prior close and the new open, and new information not captured by overnight returns takes roughly 30 minutes to digest[[2]](https://www.sciencedirect.com/science/article/pii/S2214845023000029). Related decomposition work finds that the predictive content of the first half-hour comes primarily from the 09:30–10:00 traded session, with overnight returns losing predictability[[5]](https://www.sciencedirect.com/science/article/abs/pii/S0927539823000245). There is also an exploitable end-of-day structure: returns from the prior close up to one hour before today's close negatively predict the last half-hour, an end-of-day reversal that is especially strong[[1]](http://www.efmaefm.org/0EFMAMEETINGS/EFMA%20ANNUAL%20MEETINGS/2024-Lisbon/papers/EndofDayReversal_withnames.pdf) — relevant to *exit timing*, since our forced liquidation lands exactly in that window.

**Design consequence №2:** the *sign* attached to an opening gap must be conditional, not fixed. Traded (post-09:30) price moves carry continuation information; pure overnight gap components lean toward reversal. The signal stack must decompose \(r^{ON}\) vs. \(r^{ID}\) explicitly everywhere.

### 2.3 Volume-based information uncertainty as the regime switch

Recent Chinese-market evidence formalizes the volume conditioning: threshold regression identifies a statistically significant volume-uncertainty critical value separating regimes, and the predictive power of opening returns for the final half-hour direction is most potent under the joint condition of high opening volume and high uncertainty, reaching 63.04% directional accuracy[[3]](https://www.mdpi.com/2227-7072/14/2/47), with tree-based models (XGBoost) confirming volume-uncertainty features among the most important predictors at 71.43% out-of-sample accuracy in the high-uncertainty regime[[3]](https://www.mdpi.com/2227-7072/14/2/47). Combined with the volatility-regime results in §2.1, the correct reading is that **opening-signal alpha is a conditional expectation that is near zero unconditionally and materially positive only in identifiable (high-volume-surprise, high-vol, high-uncertainty) states**.

### 2.4 Lead–lag structure and momentum spillover

Three tiers of lead–lag are relevant, operating at different horizons. At the *slow* (daily) horizon, Hou's classic result: slow diffusion of industry information drives the lead–lag effect; big-firm→small-firm predictability is predominantly an intra-industry phenomenon, driven by sluggish adjustment to negative information, and is stronger in small, less competitive, neglected industries[[3]](https://academic.oup.com/rfs/article-abstract/20/4/1113/1615954). The modern unification: when firms are economically connected, news about one is relevant for the other, and underreaction produces cross-firm return predictability ("momentum spillover")[[1]](https://www.sciencedirect.com/science/article/abs/pii/S0304405X19302533); in China specifically, news co-mention linkages extracted from business-news corpora subsume other linkage definitions and recover more cross-industry links than alternatives[[4]](https://www.sciencedirect.com/science/article/abs/pii/S037842662400270X). At the *fast* (tick) horizon, be skeptical: Hayashi–Yoshida estimates of pairwise lead–lag times among large US stocks have collapsed from a few seconds in 2000–2005 to under 10 milliseconds by 2021–22[[2]](https://www.researchgate.net/publication/5217119_Industry_Information_Diffusion_and_the_Lead-Lag_Effect_in_Stock_Returns) — pairwise tick lead-lag is an HFT-arbitraged resource we should not budget alpha to at our decision frequency, though lead–lag *spread* mean-reversion signals remain profitable out-of-sample in Chinese index futures, particularly in high-volatility periods[[7]](https://arxiv.org/html/2501.03171v1).

The decisive refinement for an open-of-day strategy is the overnight/intraday decomposition of spillover. High *peer overnight* returns are followed by elevated focal-stock overnight returns that **fully reverse intraday**, whereas high *peer intraday* returns are followed by high focal intraday returns with minor overnight reaction — consistent with individuals' trading on salient information distorting opening prices while slow-moving arbitrage corrects mispricing[[7]](https://jfqa.org/2025/10/16/decoding-momentum-spillover-effects/). This is the sharpest available prior for 09:30 signal signing:

**Design consequence №3:** propagate *yesterday's traded (intraday) peer/sector returns* as positive-sign continuation features; treat *overnight-gap-propagated* sector moves as fade candidates. This asymmetry is the core of the Stage-1 scoring below. It is consistent with the broader tug-of-war literature documenting strong cross-period reversal between overnight and intraday expected returns[[6]](https://personal.lse.ac.uk/polk/research/TugOfWar.pdf) and momentum profits earned entirely overnight with no daytime momentum effect[[4]](https://www.sciencedirect.com/science/article/abs/pii/S0927538X23002226).

### 2.5 Flow-based predictors: southbound Stock Connect

Hong Kong has a structural, observable demand-shock channel unavailable in most markets. Southbound cross-border capital flows significantly predict short-term returns on Hong Kong stocks[[1]](https://papers.ssrn.com/sol3/Delivery.cfm/92478f9c-a81c-4be4-a4ce-c93cec58bb98-MECA.pdf?abstractid=5128472&mirid=1); a weekly-rebalanced long-short portfolio on this signal achieves up to 25.84% annualized after five-factor adjustment, with mechanism tests attributing predictability primarily to demand shocks[[1]](https://papers.ssrn.com/sol3/Delivery.cfm/92478f9c-a81c-4be4-a4ce-c93cec58bb98-MECA.pdf?abstractid=5128472&mirid=1). At higher frequency, daily southbound net buying exhibited significant predictive power for stock returns (in the 2015–2017 sample) even where weekly-frequency predictability was absent[[2]](https://www.sciencedirect.com/science/article/abs/pii/S0927538X25002240), and southbound net purchases positively predict returns of connected Hong Kong stocks[[3]](https://www.sciencedirect.com/science/article/abs/pii/S1042443123000239). This motivates a stock- and sector-level flow factor with daily refresh.

### 2.6 Negative results the design must respect

A recent falsification study of OHLCV-only intraday signals in index futures is a useful discipline: the gap-fill fade failed at every tested entry time; gaps did not consistently fill, with the market as likely to continue as to reverse[[9]](https://arxiv.org/pdf/2605.04004). The implication is not that gap features are useless, but that **unconditional univariate gap rules are dead**; gaps enter only interacted with volume surprise, uncertainty regime, and the overnight/intraday decomposition. Similarly, the long history of published HK intraday rules decaying post-publication (once-profitable short-term Hang Seng trading rules became defunct after entering the public information set[[3]](http://weblib.cpce-polyu.edu.hk/apps/wps/assets/pdf/w20160505.pdf)) mandates the multiple-testing and decay-monitoring protocol of §4.

---

## 3. Algorithmic Specification

### 3.0 Notation

Indices: stocks \(i \in \mathcal{U}_t\), sectors \(k \in \mathcal{K}\) (Hang Seng Industry Classification at level 2, or data-driven clusters — see §3.2), trading days \(t\), intraday bins \(\tau\). Prices: \(O, H, L, C, V\) with adjusted series for corporate actions. Log returns \(r = \Delta \ln P\). Core decomposition:

$$
r^{ON}_{i,t} = \ln O_{i,t} - \ln C_{i,t-1}, \qquad r^{ID}_{i,t} = \ln C_{i,t} - \ln O_{i,t}, \qquad r^{CC}_{i,t} = r^{ON}_{i,t} + r^{ID}_{i,t}.
$$

EWMA volatility with half-life \(h\) (default \(h=21\)d for daily moments): \(\sigma^2_{t} = \lambda \sigma^2_{t-1} + (1-\lambda) r_{t-1}^2\), \(\lambda = 2^{-1/h}\), maintained separately for \(r^{ON}\), \(r^{ID}\), and \(r^{CC}\). Intraday realized measures from 5-minute returns \(r_{j}\):

$$
RV = \sum_j r_j^2, \quad RS^{+} = \sum_j r_j^2 \mathbf{1}_{\{r_j>0\}}, \quad RS^{-} = \sum_j r_j^2 \mathbf{1}_{\{r_j<0\}}, \quad RSJ = \frac{RS^{+}-RS^{-}}{RV},
$$

with bipower variation \(BV = \frac{\pi}{2}\frac{J}{J-1}\sum_{j\ge2} |r_j||r_{j-1}|\) and jump component \(\mathcal{J} = \max(RV - BV, 0)\). Amihud illiquidity \(ILLIQ_i = \mathbb{E}_{60d}\!\left[\,|r^{CC}_{i,d}| / DV_{i,d}\right]\) with \(DV\) dollar volume — retained because intraday predictability is stronger in high-Amihud (less liquid) names, where it is harder to arbitrage away[[6]](https://assets.super.so/e46b77e7-ee08-445e-b43f-4ffd88ae0a0e/files/ee7dac49-530b-4950-b5d0-e0b5eee08f2e.pdf). Liquidity-adjusted volatility, used for sizing and tie-breaking:

$$
LAV_i = \sqrt{RV^{(5d)}_i}\cdot\left(\frac{ILLIQ_i}{\mathrm{med}_j\, ILLIQ_j}\right)^{\gamma}, \qquad \gamma = 0.3.
$$

### 3.1 Universe construction (nightly, T−1)

\(\mathcal{U}_t\) = all Main Board common shares satisfying: 60-day median daily turnover ≥ HK$30M; price ≥ HK$1 (excludes penny-name tick-grid distortion); listed ≥ 60 days; not flagged for halts/extreme regulatory events; quoted spread median ≤ 35 bps. Maintain the sector map point-in-time. Target cardinality ≈ 300–500 names. All trailing statistics are computed on this survivorship-free, point-in-time universe.

### 3.2 Stage 1 — Sector scoring, computable by 09:29:30

Sector returns are float-cap-weighted aggregates of members: \(r_{k} = \sum_{i \in k} w_i r_i\), \(w_i \propto \text{floatcap}_i\) (capped at 15% per name). Optionally replace/validate the HSIC taxonomy with correlation-network communities (Louvain on the shrunk correlation matrix of residual returns), consistent with evidence that community detection on return-correlation networks recovers groupings consistent with standard industry classifications[[1]](https://arxiv.org/html/2312.10084v1). Six sector-level factors:

**S1 — Decomposed prior-day momentum (the spillover-asymmetry factor).** Following §2.4, yesterday's *traded* sector return is a continuation signal and the *overnight* component a reversal signal:

$$
S1_k = \theta_1 \,\frac{r^{ID}_{k,t-1}}{\sigma^{ID}_k} \;-\; \theta_2\, \frac{r^{ON}_{k,t-1}}{\sigma^{ON}_k}, \qquad \theta_1, \theta_2 > 0
$$

with \((\theta_1, \theta_2)\) re-estimated quarterly by regressing realized sector open-to-close returns on the two standardized components over a trailing 2-year window (expected magnitudes are of the same order; start at \(\theta_1=\theta_2=1\)).

**S2 — Today's gap significance, penalized.** \(G_k = r^{ON}_{k,t}\) using the 09:20–09:22 auction print (or last IEP if unmatched): \(z^{gap}_k = G_k / \sigma^{ON}_k\). Per §2.2/§2.6, the *raw* gap enters the composite with a small negative prior weight (fade), but its **interaction** with auction volume confirmation enters positively:

$$
S2_k = z^{gap}_k \cdot \mathrm{sgn}\!\left(VS^{auct}_k\right)\cdot \mathbf{1}\{|VS^{auct}_k| > v^*\} - \eta\, z^{gap}_k,
$$

where \(VS^{auct}_k = \ln\!\big(IEV_k / \overline{IEV}_k^{(20d)}\big)\) is auction-volume surprise (value-weighted across members) and \(\eta \approx 0.25\). A gap on heavy auction volume is information; a gap on thin volume is noise-plus-retail-salience and leans reversal.

**S3 — Prior-day late-session informed flow.** Institutions concentrate informed execution late; combined with the end-of-day-reversal caveat for the *very last* half-hour, use 14:00–15:30 of \(t-1\):

$$
S3_k = \frac{r_{k,t-1}^{[14:00,15:30]}}{\sigma\!\left(r_{k}^{[14:00,15:30]}\right)} \cdot \left(1 + \ln\frac{V^{[14:00,15:30]}_{k,t-1}}{\bar V^{[14:00,15:30]}_{k}}\right)_{+}.
$$

**S4 — Cross-sector lead–lag score (daily horizon).** Estimate the lagged cross-correlation matrix on residual (HSI-hedged) sector returns \( \tilde r_k \) over rolling 250 days with Ledoit–Wolf shrinkage: \(\Lambda_{ab} = \mathrm{Corr}(\tilde r_{a,t-1}, \tilde r_{b,t})\). Threshold by FDR (Benjamini–Hochberg, \(q=0.1\)) into adjacency \(A\); the score propagates yesterday's leader moves:

$$
S4_b = \sum_a A_{ab}\, \frac{\tilde r^{ID}_{a,t-1}}{\sigma^{ID}_a}.
$$

Only the *intraday* component of leaders propagates (consequence №3). Given §2.4's evidence on tick-level lead-lag decay, \(A\) is only trusted at the daily horizon; do not attempt sub-second network alpha.

**S5 — Southbound flow pressure.** With \(NB_{i,t-1}\) the stock-level southbound net buy (shares × price, from Stock Connect data) and 60-day moments:

$$
S5_k = \sum_{i\in k} w_i \frac{NB_{i,t-1} - \mu^{NB}_i}{\sigma^{NB}_i}.
$$

If intraday Connect turnover snapshots are available, refresh at 09:30 with the prior-day final print; the factor is justified by §2.5.

**S6 — Linked-market open information.** Both mainland exchanges conclude their opening call auction at 09:25 HKT, five minutes before HK continuous trading; A50/CSI300 futures and, for dual-listed names, the A-share opening print are strictly prior information. For the AH subset, define the A-share-implied sector open surprise \( \Delta^{AH}_k = \sum_{i \in k \cap AH} w_i \big( r^{A,ON}_{i,t} - \rho_i \hat r^{H,ON}_{i,t} \big)\) (A-share 09:25 auction return minus the HK gap that the POS is already pricing, \(\rho_i\) the trailing AH-return beta), plus the 09:00–09:29 return of A50 futures for the sector's mainland-exposure beta. Overnight US ADR returns for cross-listed constituents enter identically as \(r^{ADR}_{i} = \ln(P^{US,close}_i \cdot FX / C^{HK}_{i,t-1})\) mapped through ADR ratios; the residual \(r^{ADR}_i - z^{gap}_i \sigma^{ON}_i\) measures how much overnight information the POS has *failed* to impound.

**Sector composite.** Each \(S1..S6\) is cross-sectionally rank-inverse-normal transformed across \(k\): \( \tilde z = \Phi^{-1}\big((\mathrm{rk}-0.5)/|\mathcal{K}|\big)\), then combined \(\Sigma_k = \sum_f \omega_f \tilde z_{f,k}\) with \(\omega_f \propto \max(\widehat{ICIR}_f, 0) + \delta\) (trailing 250-day ICIR with additive shrinkage \(\delta = 0.1\), renormalized). Select the top \(K=2\text{–}3\) sectors, subject to an absolute-quality gate \(\Sigma_{(K)} > \Sigma^{min}\) — on days when no sector clears the gate, **the correct position is 100% cash** (the conditional-alpha logic of §2.3).

### 3.3 Stage 2 — Stock scoring within selected sectors

**X1 — Idiosyncratic gap.** \( \epsilon^{gap}_i = z^{gap}_i - \hat\beta_i z^{gap}_{k(i)} \) (beta from 250-day daily regression). Prior weight negative (idiosyncratic overnight moves reverse) *unless* confirmed by X2/X3, mirroring S2's interaction:

$$
X1_i = -\epsilon^{gap}_i + \phi\,\epsilon^{gap}_i \cdot \mathbf{1}\{VS^{auct}_i > v^*\}, \quad \phi \approx 2.
$$

**X2 — Auction order-flow drift.** From the POS state trajectory: \(X2_i = \ln\big(IEP_{i,09:20} / IEP_{i,09:10}\big) / \sigma^{ON}_i\), optionally augmented with the imbalance ratio \((Q^{buy}-Q^{sell})/(Q^{buy}+Q^{sell})\) at the final snapshot where depth data exists. Late-auction IEP drift reflects institutional order arrival after the 09:15 no-cancellation regime begins.

**X3 — Opening volume surprise.** \(X3_i = \ln\big(V_{i,\tau_0} / \bar V_{i,\tau_0}^{(20d,med)}\big)\), where \(\tau_0\) is the auction plus first 5 minutes, and \(\bar V\) is the same-bin trailing median (intraday seasonality profile).

**X4 — Information-uncertainty regime (IVU-style).** Operationalized (adapted from the construction whose empirical regime-dependence is documented in §2.3) as dispersion of realized 5-minute volume shares against the name's seasonal profile over the prior day: \(IVU_i = \big(\frac{1}{B}\sum_b (s_{i,b} - \bar s_{i,b})^2\big)^{1/2}\), binned into terciles over the trailing 60 days. IVU does not enter the score additively; it **gates the sign/weight of X1–X3** per regime, re-estimated by threshold regression quarterly — the HK analogue of the documented threshold-regression regime split on volume uncertainty[[3]](https://www.mdpi.com/2227-7072/14/2/47).

**X5 — Volatility quality.** \(X5_i = -RSJ_{i,t-1} \cdot \mathbf{1}\{\mathcal{J}_{i,t-1} > 0\} + \zeta \sqrt{RV^{(5d)}_i}/LAV_i\): prefer high-vol names (predictability rises with vol, §2.1) whose recent variance is *not* dominated by upside jumps already exploited, with the \(LAV\) ratio penalizing volatility that is expensive to access.

**X6 — Within-industry peer spillover.** Leaders = top-quintile floatcap names in sector \(k\): \(X6_i = \sum_{j \in \text{leaders}(k), j\neq i} \tilde A_{ji}\, \tilde r^{ID}_{j,t-1}/\sigma^{ID}_j\) — big→small intra-industry diffusion per Hou, restricted again to the traded component.

**X7 — Stock-level southbound z-score,** as S5 but un-aggregated; zero for non-Connect-eligible names (keep an eligibility dummy so the ML layer can learn the clientele difference).

**X8 — Execution feasibility (not alpha).** \(X8_i = -\big(s_i + \kappa\sigma_i\sqrt{q_i/ADV_i}\big)\) enters only the final net-score and the trade filter.

### 3.4 Normalization, orthogonalization, combination

Winsorize raw factors at the cross-sectional 1st/99th percentiles → rank-inverse-normal within the selected sectors' union → sequential residualization in the fixed order (X5, X3, X2, X1, X6, X7): \( \tilde f_j \leftarrow f_j - X_{1:j-1}(X_{1:j-1}^{\top}X_{1:j-1})^{-1}X_{1:j-1}^{\top} f_j\), so each factor is paid only for orthogonal information. Combine with shrunk-ICIR weights as in Stage 1. The evaluation statistic throughout is the daily rank IC — the Spearman rank correlation between predicted and realized returns across the universe at each time step, measuring whether the model correctly orders names from best to worst on the day[[1]](https://arxiv.org/html/2603.05917) — and its IR.

### 3.5 Confirmation window and the trade decision

Given §2.2 (≈30-minute digestion) and the cost floor, do **not** trade the 09:30 print. Observe \([09{:}30, 09{:}45)\); compute the confirmation statistic \( \chi_i = \mathrm{sgn}(\text{score}_i)\cdot r_{i}^{[09:30,09:45)} / \sigma^{15m}_i \). Enter at 09:45 only names with \(\text{score}_i\) in the top-\(m\) of selected sectors **and** \(\chi_i > 0\) **and** expected net edge positive:

$$
\widehat{\alpha}_i = \widehat{\mathbb{E}}\big[r_i^{[09:45,\,15:45]} \,\big|\, \text{score}_i, \text{regime}\big] \;>\; c_i + m^* ,
$$

with \(\widehat{\mathbb{E}}[\cdot]\) the trailing regression of realized 09:45→15:45 returns on the composite score within regime buckets, and margin \(m^* = 10\) bps. This is meta-labeling in the López de Prado sense: the ranking model proposes, a calibrated conditional-expectation model disposes.

### 3.6 Portfolio construction and exit

\(K \in \{2,3\}\) sectors, \(m \in \{2,4\}\) names per sector (concentration is forced by the 20 bps duty; a 30-name book cannot clear the hurdle). Weights inverse-\(LAV\), normalized to \(\sum w_i = 1\) (cash, no leverage), participation cap \(q_i \le 2\%\) of interval-projected volume, VCM-aware order slicing. Exit: begin liquidation 15:30–15:45 via VWAP slices. Rationale: our forced sell coincides with the documented end-of-day reversal window (§2.2); for positions that rallied intraday, the expected last-half-hour drift is adverse, so **early exit into strength dominates exiting at the close auction**; route only residual (illiquid tail) shares to the CAS. A per-name intraday stop at \(-2.5 \sigma^{15m}\) from entry closes the left tail; the lunch gap (12:00–13:00) is treated as a no-decision hold with orders cancelled at 11:58.

### 3.7 Optional ML ranking layer

Once the linear composite is live and measured, the same feature matrix supports a learned ranker. Frameworks: Microsoft Qlib is the reference open-source stack — it covers the full ML pipeline of data processing, model training and backtesting across alpha seeking, risk modeling, portfolio optimization and order execution[[1]](https://github.com/microsoft/qlib), ships the Alpha158 library (158 technical factors over multiple windows) and Alpha360 (360 price/volume-normalized factors)[[8]](https://arxiv.org/html/2505.15155v2), and its expression engine lets researchers implement factors as expressions rather than code[[5]](https://ar5iv.labs.arxiv.org/html/2009.11189) — useful as the *prototyping* environment whose validated factor expressions the Rust team then ports. For the model itself, the current literature favors cross-sectional transformers trained with ranking objectives: MASTER-style architectures benchmarked across pointwise, pairwise, listwise, and weighted ranking losses for daily stock ranking[[3]](https://arxiv.org/pdf/2510.14156) — use LightGBM-LambdaRank as the production baseline (deterministic, cheap, portable) with label \(y_i = r_i^{[09:45,15:45]} - r_{HSI}^{[09:45,15:45]}\), and treat deep rankers as research candidates only after the linear stack's capacity is exhausted.

---

## 4. Statistical Robustness Protocol

**Cross-validation.** Purged walk-forward with a 5-day embargo between train and test (features use trailing 250-day windows; purging removes label-overlap leakage). Model selection via combinatorial purged CV; report the distribution of OOS Sharpe across paths, not the best path.

**Significance.** Daily rank IC series → Newey–West \(t\)-statistics (lag 5). Portfolio-level: block bootstrap (block length 10d) for Sharpe CIs. Against data-mining: compute the Deflated Sharpe Ratio

$$
DSR = \Phi\!\left(\frac{(\widehat{SR} - SR^{*})\sqrt{T-1}}{\sqrt{1 - \hat\gamma_3 \widehat{SR} + \frac{\hat\gamma_4 - 1}{4}\widehat{SR}^2}}\right),
$$

where \(SR^{*}\) is the expected maximum Sharpe under the number of effective trials \(N\) (all factor variants, thresholds, and window choices ever evaluated must be logged to a trials registry to make \(N\) honest); require \(DSR > 0.95\). Complement with the Hansen SPA test against the null family {always-cash, HSI open-close, sector-ETF version of Stage 1}.

**Regime and decay monitoring.** All ICs reported unconditionally *and* stratified by VHSI tercile, volume-surprise tercile, and IVU regime — the theory (§2.1, §2.3) predicts concentration of alpha in high-vol/high-uncertainty cells; if OOS alpha appears in the low-vol cells instead, the model is misspecified, not lucky. Given the documented post-publication decay of HK rules (§2.6), production runs a CUSUM monitor on the 60-day rolling IC with a pre-registered kill threshold.

**Cost realism.** Backtests must charge the full §1 cost stack per fill, model the 15% POS band and VCM halts as executable-price constraints, and mark unfilled auction orders per the rule that outstanding at-auction orders after the pre-opening session are cancelled before continuous trading[[5]](https://www.dbsvickers.com/vickers/resources/mca-faq/market-information-and-order-types.page).

---

## 5. Data Requirements and Source Mapping

**End-of-day (nightly ingestion, T−1).** Adjusted OHLCV and raw close for all Main Board equities (10y history); corporate actions and adjustment factors; board-lot sizes; free float and shares outstanding; point-in-time Hang Seng industry classification; index membership flags (HSI/HSCEI/HSTECH) for VCM eligibility; daily southbound Stock Connect net buying per stock and CCASS shareholding snapshots (HKEX publishes daily Connect turnover and shareholding data — the flow factor's raw material); AH-pair mapping with ratios; ADR mapping with ratios and US closes plus USDHKD/CNHHKD; VHSI level; trading calendar including half-days (the exchange publishes half-day schedules, with trading commonly ending at midday[[8]](https://www.bitget.com/wiki/when-does-the-hong-kong-stock-exchange-open) — the bin grid collapses to \(\tau \le 5\) and the strategy must either skip or use a compressed exit at 11:45).

**Intraday (streaming).** One-minute OHLCV bars for all universe names and sector indices from 09:00 (or finest available granularity through the POS); POS \(IEP/IEV\) snapshot series at ≤30s cadence for X2 (this is the one feed that is *mandatory* from a proper vendor — Yahoo-class sources do not carry it); level-1 quotes for spread estimation \(s_i\); trade prints (for 5-minute realized measures and, where tick data exists, Hayashi–Yoshida diagnostics); A50 futures quotes 09:00–09:30; SSE/SZSE 09:25 auction prints for the AH subset.

**Source mapping and their honest limits.** TigerOpenAPI: primary candidate for HK real-time quotes, minute bars, depth, and order routing in one stack; verify per-symbol subscription caps and whether its quote feed exposes pre-open IEP/IEV fields for HK (it exposes HK market status and auction-phase quotes; the Rust client should treat auction fields as optional-with-fallback to X2-disabled mode). YFinance: acceptable *only* for EOD prototyping — `.HK` suffixed symbols, but 1-minute history is limited to a trailing window of days, auction data is absent, delisted names drop out (survivorship), and adjustment quality must be cross-checked; never a production dependency. Xueqiu: useful for A-share/AH quotes and southbound flow aggregates; it is an unofficial interface with rate-limiting and ToS risk — for production, the canonical sources are HKEX OMD-C (market data), HKEX's Stock Connect statistics pages (flows/CCASS), and a licensed A-share feed, with Tiger as the execution venue. All ingestion must be idempotent, timestamped in exchange time, and reconciled nightly against EOD official closes (the official closing price is the closing-auction print[[1]](https://www.bitget.com/wiki/what-time-does-the-hong-kong-stock-exchange-open)).

---

## 6. Daily Runbook

T−1 18:00 — ingest EOD, flows, CCASS; rebuild universe, seasonal volume profiles, EWMA moments, lead–lag adjacency (weekly), factor weights (monthly). T 08:45 — ingest US ADR closes, FX; compute S6 ADR block. 09:00–09:22 — stream POS; update \(IEP/IEV\) trajectories; 09:25 — ingest mainland auction prints; finalize S1–S6, select sectors, compute stock scores, freeze candidate list. 09:30–09:45 — confirmation window; compute \(\chi_i\); meta-label filter against \(c_i\). 09:45 — entries (sliced). 11:58 — cancel resting orders into lunch. 15:30–15:45 — exit program; 16:00–16:08 — CAS residuals only. Post-close — attribution: realized rank IC per factor, cost slippage vs. model, CUSUM update, trials-registry append.

---

**Summary of the load-bearing claims.** The HK-specific evidence says index-level intraday timing is empty but cross-sectional structure is not; opening signals pay only conditionally on volume/uncertainty/volatility regimes; yesterday's *traded* returns propagate forward through sector and peer links while *overnight* components reverse; southbound flow is a real, observable demand-shock predictor; and a 20 bps statutory duty floor forces concentration, conditional participation, and a confirmation-window entry. Every formula above is a direct mechanization of one of those five statements, and the validation protocol in §4 is what keeps the Rust implementation from shipping a multiple-testing artifact.

---
Learn more:
1. [Intraday Momentum and Return Predictability](https://www.diva-portal.org/smash/get/diva2:1878991/FULLTEXT01.pdf)
2. [A Decadal Analysis of the Lead-Lag Effect in the NYSE - arXiv](https://arxiv.org/html/2312.10084v1)
3. [End-of-Day Reversal Guido Baltussen1,3, Zhi Da2, and Amar Soebhag1,4](http://www.efmaefm.org/0EFMAMEETINGS/EFMA%20ANNUAL%20MEETINGS/2024-Lisbon/papers/EndofDayReversal_withnames.pdf)
4. [Market intraday momentum: APAC evidence - ScienceDirect](https://www.sciencedirect.com/science/article/pii/S0927538X2300152X)
5. [GitHub - microsoft/qlib: Qlib is an AI-oriented Quant investment platform that aims to use AI tech to empower Quant Research, from exploring ideas to implementing productions. Qlib supports diverse ML modeling paradigms, including supervised learning, market dynamics modeling, and RL, and is now equipped with https://github.com/microsoft/RD-Agent to automate R&D process. · GitHub](https://github.com/microsoft/qlib)
6. [Accepted for publication in IEEE Access. DOI: 10.1109/ACCESS.2026.3691980. © 2026 IEEE. Stock Market Prediction Using Node Transformer Architecture Integrated with BERT Sentiment Analysis](https://arxiv.org/html/2603.05917)
7. [Shared analyst coverage: Unifying momentum spillover effects - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0304405X19302533)
8. [Southbound Capital Flows and Stock Return Predictability by Tian Ding, Wenjing Song, Jiangze Bian, Ge Zhang :: SSRN](https://papers.ssrn.com/sol3/Delivery.cfm/92478f9c-a81c-4be4-a4ce-c93cec58bb98-MECA.pdf?abstractid=5128472&mirid=1)
9. [what time does the hong kong stock exchange open](https://www.bitget.com/wiki/what-time-does-the-hong-kong-stock-exchange-open)
10. [GovHK: Stamp Duty Rates](https://www.gov.hk/en/residents/taxes/stamp/stamp_duty_rates.htm)
11. [Intraday momentum and return predictability: Evidence from the crude oil market](https://ideas.repec.org/a/eee/ecmode/v95y2021icp374-384.html)
12. [Industry Information Diffusion and the Lead-Lag Effect in Stock Returns | Request PDF](https://www.researchgate.net/publication/5217119_Industry_Information_Diffusion_and_the_Lead-Lag_Effect_in_Stock_Returns)
13. [Cross-sectional reversal of intraday returns and investor heterogeneity in an emerging market - ScienceDirect](https://www.sciencedirect.com/science/article/pii/S2214845023000029)
14. [Hong Kong Stock Exchange Today: Hang Seng Index Jumps 1.57% to 23,416 as Gold-Related Shares Lead Afternoon Rally](https://www.bbntimes.com/financial/hong-kong-stock-exchange-today-hang-seng-index-jumps-1-57-to-23-416-as-gold-related-shares-lead-afternoon-rally)
15. [Qlib: An Open-Source AI Platform by Microsoft for Quant Investment | by Tattva Tarang | Coding Nexus | Medium](https://medium.com/coding-nexus/qlib-an-open-source-ai-platform-by-microsoft-for-quant-investment-6479f8d11447)
16. [This work has been submitted to IEEE Access for possible publication. Stock Market Prediction Using Node Transformer Architecture Integrated with BERT Sentiment Analysis](https://arxiv.org/html/2603.05917v2)
17. [Production Complementarity and Momentum Spillover Across Industries | Request PDF](https://www.researchgate.net/publication/362814819_Production_Complementarity_and_Momentum_Spillover_Across_Industries)
18. [Southbound capital flows and stock return predictability - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0927538X25002240)
19. [HKEX Trading Hours: Morning and Afternoon Sessions Guide](https://longbridge.com/en/academy/hk-stocks/blog/hkex-trading-hours-morning-and-afternoon-sessions-100104)
20. [Hong Kong's Stock Stamp Duty: Current Landscape | TAX.hk](https://www.tax.hk/en/articles/hong-kong-stamp-duty-on-stocks-key-exemptions-every-investor)
21. [Enhancing Intraday Momentum Prediction: The Role of Volume-Based Information Uncertainty in the Chinese Stock Market](https://www.mdpi.com/2227-7072/14/2/47)
22. [Industry Information Diffusion and the Lead-lag Effect in Stock Returns | The Review of Financial Studies | Oxford Academic](https://academic.oup.com/rfs/article-abstract/20/4/1113/1615954)
23. [Overnight returns, daytime reversals, and future stock returns - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0304405X21004116)
24. [A review on high frequency intraday trading in Hong Kong ...](http://weblib.cpce-polyu.edu.hk/apps/wps/assets/pdf/w20160505.pdf)
25. [Qlib: An AI-oriented Quantitative Investment Platform. - Microsoft Research](https://www.microsoft.com/en-us/research/publication/qlib-an-ai-oriented-quantitative-investment-platform/)
26. [ON EVALUATING LOSS FUNCTIONS FOR STOCK RANKING:](https://arxiv.org/pdf/2510.14156)
27. [Modeling Momentum Spillover with Economic Links Discovered from Financial Documents](https://www.researchgate.net/publication/375921728_Modeling_Momentum_Spillover_with_Economic_Links_Discovered_from_Financial_Documents)
28. [Cross-border equity flows and information transmission: Evidence from Chinese stock markets - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S1042443123000239)
29. [HKEx to introduce pre-opening session and index basket order trading mechanism on 25 March](https://www.hkex.com.hk/News/News-Release/2002/020319news?sc_lang=en)
30. [Share Transfer Stamp Duty in Hong Kong: Rates, Calculation & Process (2026 Guide)](https://sleek.com/hk/resources/shares-transfer-stamp-duty/)
31. [Intraday Return Predictability in the Cryptocurrency Markets: Momentum, Reversal, or Both by Zhuzhu Wen, Elie Bouri, Yahua Xu, Yang Zhao :: SSRN](https://papers.ssrn.com/sol3/Delivery.cfm/SSRN_ID4135239_code2537556.pdf?abstractid=4080253&mirid=1)
32. [Quant 4.0: Engineering Quantitative Investment with Automated,   Explainable and Knowledge-driven Artificial Intelligence](https://arxiv.org/pdf/2301.04020)
33. [Momentum investing and a tale of intraday and overnight returns: Evidence from Taiwan - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0927538X23002226)
34. [Hong Kong market progress check: 2026 outlook on track | IG International](https://www.ig.com/en/news-and-trade-ideas/hong-kong-equities-progress-review-260204)
35. [Demystifying Qlib: Your Guide to Microsoft’s AI-Driven Quantitative Investment Platform | by Grepix | Medium](https://grepix.medium.com/demystifying-qlib-your-guide-to-microsofts-ai-driven-quantitative-investment-platform-c530fd632995)
36. [ACT: Anti-Crosstalk Learning for Cross-Sectional Stock Ranking via Temporal Disentanglement and Structural Purification](https://arxiv.org/html/2604.20204v1)
37. [Diamond cuts diamond: News co-mention momentum spillover prevails in China - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S037842662400270X)
38. [Hong Kong Market Indicators - 360MiQ.com](https://360miq.com/market?data=HKEX)
39. [Hong Kong Stock Market Timetable - Securities](https://www.sdicsi.com.hk/en/financial-services/securities-trading/hong-kong-stock-trading-operation)
40. [IRD : Stamp Duty](https://www.ird.gov.hk/eng/tax/sdu.htm)
41. [Intraday return predictability in the cryptocurrency markets: Momentum, reversal, or both](https://ideas.repec.org/a/eee/ecofin/v62y2022ics1062940822000833.html)
42. [(PDF) Detecting the lead–lag effect in stock markets: definition, patterns, and investment strategies](https://www.researchgate.net/publication/360743560_Detecting_the_lead-lag_effect_in_stock_markets_definition_patterns_and_investment_strategies)
43. [Time series momentum and reversal: Intraday information from realized semivariance - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0927539823000245)
44. [Daily Momentum and New Investors in Emerging Stock Markets∗](https://wxiong.mycpanel.princeton.edu/papers/DailyMomentum.pdf)
45. [\[2009.11189\] Qlib : An AI-oriented Quantitative Investment Platform](https://ar5iv.labs.arxiv.org/html/2009.11189)
46. [Asset Pricing in Pre-trained Transformer](https://arxiv.org/pdf/2505.01575)
47. [Shared analyst coverage: Unifying momentum spillover effects | Request PDF](https://www.researchgate.net/publication/336395011_Shared_analyst_coverage_Unifying_momentum_spillover_effects)
48. [The value of information in China’s connected market - ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0927539824000616)
49. [Market Information & Order Types Frequently Asked Questions | DBS Vickers Online Trading](https://www.dbsvickers.com/vickers/resources/mca-faq/market-information-and-order-types.page)
50. [Hong Kong SAR - Corporate - Other taxes](https://taxsummaries.pwc.com/hong-kong-sar/corporate/other-taxes)
51. [Market Intraday Momentum](https://assets.super.so/e46b77e7-ee08-445e-b43f-4ffd88ae0a0e/files/ee7dac49-530b-4950-b5d0-e0b5eee08f2e.pdf)
52. [High-frequency lead-lag relationships in the Chinese stock index](https://arxiv.org/pdf/2501.03171)
53. [A tug of war: Overnight versus intraday expected returns](https://personal.lse.ac.uk/polk/research/TugOfWar.pdf)
54. [Hong Kong 50 forecast – Market Outlook & Future Trends | Capital.com UK](https://capital.com/en-gb/analysis/hong-kong-50-forecast)
55. [Comprehensive Guide to Microsoft Qlib: The AI-Oriented Quantitative Investment Platform](https://www.quantlabsnet.com/post/comprehensive-guide-to-microsoft-qlib-the-ai-oriented-quantitative-investment-platform)
56. [On Evaluating Loss Functions for Stock Ranking: An Empirical Analysis With Transformer Model](https://arxiv.org/html/2510.14156v1)
57. [Decoding Momentum Spillover Effects by Huaixin Wang :: SSRN](https://papers.ssrn.com/sol3/Delivery.cfm/4179413.pdf?abstractid=4179413)
58. [Hong Kong - Southbound Fund Inflows vs. HSI | Hong Kong Stock Market | Collection | MacroMicro](https://en.macromicro.me/collections/1658/hk-stock-relative/15374/hk-southward-funds-and-hsi)
59. [the service hours for 'Enhanced Limit Order' are 9:00 a.m. ...](https://pic.bankofchina.com/bocappd/macau/202010/P020201019358880833694.pdf)
60. [Hong Kong Stock Stamp Duty Rate Hike: Impact and Reversal](https://www.bitget.com/en-CA/wiki/hong-kong-stock-stamp-duty-rate-hike)
61. [Intraday return predictability: Evidence from commodity ETFs and their related volatility indices - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC7480318/)
62. [High-frequency lead-lag relationships in the Chinese stock index futures market: tick-by-tick dynamics of calendar spreads](https://arxiv.org/html/2501.03171v1)
63. [Intraday time series momentum: global evidence and links to market](https://centaur.reading.ac.uk/95566/1/Accepted-Version.pdf)
64. [The first 20 minutes in the Hong Kong stock market](https://arxiv.org/pdf/cond-mat/0006145)
65. [Microsoft Qlib: AI-Driven Investment Platform | PDF | Machine Learning | Artificial Intelligence](https://www.scribd.com/document/498701514/2009-11189v1)
66. [ACT: Anti-Crosstalk Learning for Cross-Sectional Stock Ranking via Temporal Disentanglement and Structural Purification](https://arxiv.org/pdf/2604.20204)
67. [Decoding Momentum Spillover Effects – JFQA](https://jfqa.org/2025/10/16/decoding-momentum-spillover-effects/)
68. [Hong Kong - Southbound Fund Inflows vs. HSI | MacroMicro](https://en.macromicro.me/charts/15374/hk-southward-funds-and-hsi)
69. [Before entering the market: understanding the trading rules of the Hong Kong stock market.](https://www.futunn.com/en/learn/detail-before-entering-the-market-understand-the-trading-rules-of-the-hong-kong-stock-market-83831-230556033)
70. [Exchange Rates for Stamp Duty Calculation](https://www.hkex.com.hk/eng/market/sec_tradinfo/stampfx/stampfx.asp)
71. [Does Overnight News Explain Overnight Returns?](https://arxiv.org/pdf/2507.04481)
72. [Hong Kong Stock Market — Live Quotes, News, Ideas — TradingView](https://www.tradingview.com/markets/stocks-hong-kong/)
73. [R&D-Agent-Quant: A Multi-Agent Framework for Data-Centric Factors and Model Joint Optimization](https://arxiv.org/html/2505.15155v2)
74. [Pre-training Time Series Models with Stock Data Customization](https://arxiv.org/pdf/2506.16746)
75. [Shared Analyst Coverage: Unifying Momentum Spillover ...](https://www.nber.org/system/files/working_papers/w25201/w25201.pdf)
76. [Southbound Flow | SH-HK Stock Connect - Citigroup](https://www.citiwarrants.com/en/market/sn-bound)
77. [when does the hong kong stock exchange open — HKT hours](https://www.bitget.com/wiki/when-does-the-hong-kong-stock-exchange-open)
78. [Stamp Duty in Hong Kong - Hong Kong Guide | Doing Business in Hong Kong](https://www.china-briefing.com/doing-business-guide/hong-kong/taxation-and-accounting/stamp-duty-hong-kong)
79. [Structural Limits of OHLCV-Based Intraday Signals in MNQ Futures: A Systematic Falsification Study](https://arxiv.org/pdf/2605.04004)
80. [qlib/examples/benchmarks/TRA/README.md at main · microsoft/qlib](https://github.com/microsoft/qlib/blob/main/examples/benchmarks/TRA/README.md)
81. [\[2510.14156\] On Evaluating Loss Functions for Stock Ranking: An Empirical Analysis With Transformer Model](https://arxiv.org/abs/2510.14156)
82. [Modeling Momentum Spillover with Economic Links Discovered from Financial Documents](https://dl.acm.org/doi/fullHtml/10.1145/3604237.3626862)
83. [A General Framework For Constructing Locally Self-Normalized   Multiple-Change-Point Tests](https://arxiv.org/pdf/2205.00304)
84. [Pre-opening Session (POS)](https://www.hkex.com.hk/Global/Exchange/FAQ/Securities-Market/Trading/Pre_opening-Session?sc_lang=en)
85. [Rates of Stamp Duty – Transfer of Hong Kong Stock From 01 ...](https://www.ird.gov.hk/eng/pdf/sd_stock_rates.pdf)
86. [Decoding Momentum Spillover Effects | Journal of Financial and Quantitative Analysis | Cambridge Core](https://www.cambridge.org/core/journals/journal-of-financial-and-quantitative-analysis/article/decoding-momentum-spillover-effects/EB6BE5A096753108881E1514E54035DF)
87. [Hong Kong Stamp Duty 2026: Property & Stock Rates | Air Corporate](https://air-corporate.com/hk/blog/stamp-duty-hong-kong)
