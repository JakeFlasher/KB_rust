---
schema_version: "cacg.v0"
id: "fa-liquidity-measurement-and-price-impact"
title: "Measuring Liquidity & Price Impact: Roll, Kyle, Amihud"
reading_id: "22_fund_level_arbitrage"
summary: "Three estimators turn observable trade data into liquidity numbers: Roll infers the effective spread from negative serial covariance of price changes, Kyle's lambda measures depth as the order flow that moves price by one unit, and Amihud proxies price impact by |return|/volume."
tags: ["roll-measure", "kyle-lambda", "amihud"]
citations:
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p095:0098"
    chunk_hash: "2fb85d25715169d1ec82a12caeafd611eb4be2136a2f7c1f7bb17bd6e54757cb"
    page_range: [96, 96]
    quote: "bid-ask spread directly from transaction prices.7 The Roll measure takes the size of the bid-ask spreads as a given."
    edge_type: "defines"
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p099:0102"
    chunk_hash: "5b621e777e0e7f51dd6186095eef8172d7f0d1a944c2f1232dde9dd112f4ee1e"
    page_range: [99, 99]
    quote: "to induce prices to rise or fall by one dollar. The measure is implemented using intradaily transaction data that may not always be available."
    edge_type: "defines"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p090:0108"
    chunk_hash: "23c89d44acd0ad9c7dbe5746e0c9864f71ae91e276f7d27967dff6d631a42356"
    page_range: [90, 90]
    quote: "size in the secondary market. Define the price impact cost as the deviation of the average execution price from the prevailing price at the time of the order"
    edge_type: "supports"
---
# Measuring Liquidity & Price Impact: Roll, Kyle, Amihud

## Intuition

Liquidity is not directly observable, but its fingerprints are all over the trade tape. Three estimators each read a different fingerprint. **Roll** notices that, even when fundamental value follows a random walk, transaction prices ricochet between the bid and the ask — the *bid-ask bounce* — and this injects *negative* serial correlation into trade-to-trade returns; the deeper the spread, the harder the bounce, so the spread can be backed out of return autocovariance without ever seeing a quote. **Kyle** reasons that a market maker who cannot tell informed from noise order flow must protect himself by moving price up against buys and down against sells; the steepness of that response is *depth*. **Amihud** simply asks how much a unit of trading volume nudges the price — a security is illiquid if a little volume moves it a lot. Roll measures the *spread* dimension; Kyle and Amihud measure the *depth/price-impact* dimension.

**Source:** van der Merwe (2015) pp.96-99.

```
 bid-ask bounce (Roll)            price impact (Kyle / Amihud)
 ask  o     o                     P
      |\   /|\                     ^        /  slope = lambda (Kyle)
 mid  | \ / | \                    |       /   depth = 1/lambda
      |  X  |  o   ...             |      /
 bid  o / \ o                      |     /   little volume, big move
      t t+1 t+2                    +--------------> signed volume q
 cov(dP_t, dP_{t-1}) < 0           Amihud ~ |return| / volume
```

## Definition

- **Roll implied spread.** Let value follow a random walk `V_t = V_{t-1} + e_t` with the observed price `P_t = V_t + (S/2) d_t`, where `d_t = +1` (buy-initiated) or `-1` (sell-initiated). Under balanced, serially uncorrelated, news-free order flow, the *effective* spread `S` is recovered from the serial covariance of price changes.
- **Kyle's lambda (depth).** The market maker sets price as an increasing function of net order-flow imbalance; `lambda` is the price move induced per unit of (signed) order flow. *Depth* is its reciprocal, `1/lambda` — the order flow needed to move price by one unit.
- **Amihud illiquidity ratio.** The per-period average of `|return| / traded-volume`; the price response associated with one currency unit of trading volume — a rough price-impact proxy using only daily data.

The Roll measure "infers an effective bid-ask spread directly from transaction prices," whereas Kyle's and Amihud's are *price-impact* (depth) measures.

**Source:** van der Merwe (2015) pp.96-99.

## Mathematical Reasoning

Roll's estimator. With `dP_t = (S/2) d_(d_t) + e_t`, the listed assumptions (order flow balanced, `E(d_t)=0`; no autocorrelation in orders; orders carry no news, `E(d_t e_t)=E(d_t e_{t+1})=0`; zero expected return) yield

    cov( dP_t , dP_{t-1} ) = - S^2 / 4    =>    S = 2 * sqrt( -cov( dP_t , dP_{t-1} ) ).

The estimator is only defined when the sample autocovariance is negative — exactly the sign the bid-ask bounce predicts; a non-negative sample covariance signals that one of Roll's assumptions (e.g. informed order flow correlated with `e_t`) is violated.

Kyle's depth. Modeling price impact linearly, `E[P_t] - m_t = lambda * s * q_t` (signed size `q_t`), so

    depth = dq / dP = 1 / (lambda * s),     i.e. depth is inversely proportional to lambda.

Crowding and capacity (qualitative). Because price impact rises with size, an arbitrageur scales each trade until marginal impact cost offsets the edge; as more managers chase the same signal, their combined demand on the same names raises the effective price-impact coefficient `lambda` and transaction cost, eroding net alpha and shrinking each participant's capacity. Madhavan makes this point qualitatively — more money chasing the same themes lowers the likelihood of successful alpha generation — without a closed-form capacity formula.

Amihud. `ILLIQ = avg_t ( |r_t| / Vol_t )`; comparative statics: `dILLIQ/dVol < 0` (more volume ⇒ lower impact) and `dILLIQ/d|r| > 0` (bigger moves per trade ⇒ less liquid). All three are *estimators of the same latent liquidity*, differing in data needs: Roll and Amihud use daily/transaction prices; Kyle's lambda needs intraday signed order flow that "may not always be available."

**Source:** van der Merwe (2015) pp.96-99; Madhavan (2016) §6 p.90 (price impact), §14.5 p.181 (crowding/capacity).

## See Also

- [`fa-market-liquidity-dimensions-and-no-arbitrage`](./fa-market-liquidity-dimensions-and-no-arbitrage.md) — these estimators quantify the tightness (Roll spread) and depth (Kyle/Amihud) dimensions defined there.
- [`fa-amihud-mendelson-and-priced-liquidity-risk`](./fa-amihud-mendelson-and-priced-liquidity-risk.md) — why the spread/illiquidity these tools measure is *priced* into expected returns.
- [`fa-liquidity-adjusted-var`](./fa-liquidity-adjusted-var.md) — feeds spread/impact estimates into a liquidity-augmented risk measure.
- [`fa-search-bargaining-liquidity-premium`](./fa-search-bargaining-liquidity-premium.md) — the OTC/dealer-market alternative when no order book exists for Roll/Kyle.
- `mt-roll-implicit-spread-estimator`, `mt-kyle-lambda-market-depth-price-impact`, and `mt-price-impact-measures-amihud` (reading 14) give the primary-source derivations of the Roll, Kyle, and Amihud estimators; this card collects them through the fund-arbitrage lens and adds Madhavan's crowding/capacity point.

Legacy cross-refs (other tree, prose only): the risk-management VaR notes treat the loss-distribution anatomy that a liquidity-adjusted VaR widens, and the portfolio-management tracking-error material relies on transaction-cost estimates of exactly the kind derived in `mt-roll-implicit-spread-estimator`, `mt-kyle-lambda-market-depth-price-impact`, and `mt-price-impact-measures-amihud` (reading 14).

## Escalate to Raw When

Go to the raw source when you need the full derivation chain `dP_t = (S/2) d_(d_t) + e_t` through to `cov(dP_t, dP_{t-1}) = -S^2/4`, including each of Roll's four order-arrival assumptions and the random-walk/white-noise setup. Also escalate for the empirical calibration: van der Merwe reports Stoll's worked NYSE-vs-NASDAQ Roll-measure cents estimates and the cross-exchange comparison, and Madhavan gives the linear price-impact cost `E[p_t] - m_t = lambda s q_t` plus the primary-vs-secondary-market trade-cost worked example — concrete figures and plugged numbers that belong in the raw text, not abstracted here.

**Source:** van der Merwe (2015) pp.96-99; Madhavan (2016) p.90.
