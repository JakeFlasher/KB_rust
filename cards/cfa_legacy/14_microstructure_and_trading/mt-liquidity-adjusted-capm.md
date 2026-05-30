---
schema_version: "cacg.v0"
id: "mt-liquidity-adjusted-capm"
title: "Liquidity-Adjusted CAPM: Net Returns and the Three Liquidity Betas"
reading_id: "14_microstructure_and_trading"
summary: "Acharya-Pedersen extend the CAPM to net (after-cost) returns, decomposing the gross-return risk premium into the expected trading-cost level plus three liquidity-risk betas, so assets that stay liquid in down markets earn lower expected returns."
tags: ["microstructure", "liquidity-risk", "capm", "asset-pricing", "commonality-in-liquidity"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p299:0453"
    chunk_hash: "897182670b18d9c3a9f6644874f881382df17f84445fcb45f546ad97029c1f5e"
    page_range: [299, 299]
    quote: "they tend to remain liquid when the market is down: investors value such stocks more highly"
    edge_type: "defines"
---
# Liquidity-Adjusted CAPM: Net Returns and the Three Liquidity Betas

## Intuition
The classic CAPM prices an asset off a single beta — the covariance of its return with the market. But the CAPM was derived for the returns investors actually *keep*, i.e. net returns after trading costs. Once you admit that trading costs are themselves random and correlated across assets and with the market, a single market beta is no longer enough: investors also care about *when* an asset becomes expensive to trade. The liquidity-adjusted CAPM is what you get when you take the ordinary CAPM relation, apply it to net returns, and then re-express it in the gross returns that researchers actually measure off midquotes.

The economic punchline is a hierarchy of "bad states." An asset is unattractive (and must offer a higher gross return) if it becomes illiquid precisely when the rest of the market is illiquid, or when the market return is low — because that is exactly when an investor would most want to sell cheaply and cannot. Symmetrically, an asset that *stays* liquid when the market is down is a hedge: you can exit it at low cost in an adverse phase, so investors bid its price up and accept a lower expected return.

```
        ordinary CAPM on NET returns r_j = R_j - s_j
                          |
          re-express in GROSS returns R_j (midquote-based)
                          v
  E(R_j) - r = beta1*lambda_M          <- classic market-return beta
             + E(s_j)                  <- level: expected trading cost
             + beta2*lambda_M          <- s_j co-moves with s_M  (+)
             - beta3*lambda_M          <- R_j co-moves with s_M  (hedge, -)
             - beta4*lambda_M          <- s_j co-moves with R_M  (hedge, -)
```

**Source:** Foucault, Pagano & Roell (2013) §9.3 (Liquidity and Asset Prices) pp.298-300.

## Definition
Normalize the holding period to h = 1 and write the gross return as R_j ≈ r_j + s_j, where r_j is the net (after-cost) return and s_j is the (random) bid-ask spread / illiquidity cost. Treating the approximation as exact gives r_j = R_j − s_j. Because investors care about net returns, the ordinary CAPM holds in net terms:

  E(r_j) = r + beta_j [E(r_M) − r].

Substituting r_j = R_j − s_j and r_M = R_M − s_M, where s_M is value-weighted market illiquidity and R_M is the gross market return, yields the liquidity-adjusted CAPM in gross-return form:

  E(R_j) − r = beta1_j·lambda_M + E(s_j) + beta2_j·lambda_M − beta3_j·lambda_M − beta4_j·lambda_M,

with lambda_M ≡ E(R_M − s_M) − r the net market risk premium and each beta defined as a covariance divided by var(r_M):

  beta1_j = cov(R_j, R_M)/var(r_M)   — standard market-return beta
  beta2_j = cov(s_j, s_M)/var(r_M)   — commonality in liquidity
  beta3_j = cov(R_j, s_M)/var(r_M)   — return-vs-market-illiquidity
  beta4_j = cov(s_j, R_M)/var(r_M)   — illiquidity-vs-market-return

**Source:** Foucault, Pagano & Roell (2013) §9.3 eqs.(9.16)-(9.18) pp.298-299.

## Mathematical Reasoning
The decomposition follows from a single algebraic identity. The net-return beta of the ordinary CAPM is

  beta_j = cov(r_j, r_M)/var(r_M) = cov(R_j − s_j, R_M − s_M)/var(r_M).

Expanding the bilinear covariance into its four cross terms,

  cov(R_j − s_j, R_M − s_M) = cov(R_j,R_M) + cov(s_j,s_M) − cov(R_j,s_M) − cov(s_j,R_M),

and dividing through by var(r_M) gives beta_j = beta1_j + beta2_j − beta3_j − beta4_j. Carrying this through E(r_j) = r + beta_j·lambda_M and moving the expected spread E(s_j) to the right-hand side produces equation (9.18). No new economic assumption is needed beyond "CAPM holds on net returns" plus the linear cost wedge R_j = r_j + s_j.

Comparative statics on the signs:

- beta2_j (commonality) enters with a +: a stock whose spread widens when market-wide spreads widen offers no liquidity hedge, so higher beta2 ⇒ higher required gross return.
- beta3_j (return co-moves with market illiquidity) enters with a −: a stock that pays off when market liquidity dries up hedges that adverse state, so higher beta3 ⇒ lower required return.
- beta4_j (own illiquidity co-moves with market return) enters with a −: high beta4 means the stock's spread is *low* when the market return is *low*, i.e. it stays cheap-to-trade in down markets — a valuable hedge — so higher beta4 ⇒ lower required return.

Empirically (Acharya-Pedersen on 1962-1999 CRSP NYSE/AMEX data, illiquidity proxied by the Amihud ratio), illiquid stocks carry higher beta2 and more negative beta3, beta4; the dispersion of returns is fit better than by the plain CAPM; and the bulk of the cross-sectional liquidity-risk premium loads on beta4 — the "stay liquid when the market is down" channel.

**Source:** Foucault, Pagano & Roell (2013) §9.3 pp.299-300.

## Boundary Notes
- The relation is stated for the *gross*-return form (midquote-based returns). For tests that already use net (transaction-price) returns, trading costs net out and only the standard market beta survives — the three liquidity betas matter only because researchers typically measure midquote returns.
- The approximation R_j ≈ r_j + s_j is treated as exact for exposition; the level term E(s_j) is the expected end-of-holding-period cost, not a contemporaneous one.
- Liquidity level and the liquidity-risk covariances are taken as *exogenous*: the model prices commonality in liquidity but does not explain why illiquidity co-moves across stocks or with returns. That endogenous mechanism (funding constraints, fire sales, flight to liquidity) is the subject of the limits-to-arbitrage / funding-liquidity material, not this card.
- Single-period, normalized holding horizon h = 1; the multi-period clientele/holding-horizon channel for the *level* premium is a separate result.

**Source:** Foucault, Pagano & Roell (2013) §9.3-§9.4 pp.300-301.

## See Also
- [`mt-liquidity-premium-asset-pricing`](./mt-liquidity-premium-asset-pricing.md) -- supplies the level-of-cost premium E(s_j) that this card decomposes alongside the risk betas.
- [`mt-three-dimensions-liquidity`](./mt-three-dimensions-liquidity.md) -- defines spread / depth / resiliency, the liquidity primitives whose co-movement generates beta2-beta4.
- [`mt-funding-liquidity-fire-sales`](./mt-funding-liquidity-fire-sales.md) -- endogenizes the commonality and return-illiquidity covariation that this card treats as exogenous.

## Escalate to Raw When
The source derives the four-beta decomposition and reports the Acharya-Pedersen empirical signs and the ~1.1% illiquid-minus-liquid annual return gap; this card only sketches those magnitudes. Re-read pp.298-300 (eqs. 9.16-9.18 and the three numbered beta interpretations) for the exact covariance algebra and the CRSP/Amihud estimation details, and §9.4 for why the liquidity covariances are not exogenous in practice.
