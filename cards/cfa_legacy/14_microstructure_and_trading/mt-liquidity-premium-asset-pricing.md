---
schema_version: "cacg.v0"
id: "mt-liquidity-premium-asset-pricing"
title: "Liquidity and Asset Prices: The Illiquidity Premium and the Clientele Effect"
reading_id: "14_microstructure_and_trading"
summary: "Investors capitalize future trading costs, so illiquid assets sell at a discount and earn a higher gross return (illiquidity premium s/h); heterogeneous holding periods produce a concave return-spread locus via the clientele effect."
tags: ["microstructure", "liquidity", "illiquidity-premium", "asset-pricing", "clientele-effect", "bid-ask-spread"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p284:0432"
    chunk_hash: "77caa67000f33de92271186d2c9481ed7289be7d46c09592659aaa0a1108386b"
    page_range: [284, 284]
    quote: "asset returns contain an illiquidity premium in addition to a risk premium, as section 9.2 explains."
    edge_type: "defines"
---
# Liquidity and Asset Prices: The Illiquidity Premium and the Clientele Effect

## Intuition
Trading costs act like a recurring tax on capital gains: an investor who must cross the
bid-ask spread to buy now and again to sell later keeps less of the asset's gross return.
Rational investors therefore refuse to pay full frictionless value for an asset that is
costly to trade. They bid its price down until the *gross* return it offers is high enough
to cover both the usual risk compensation and the expected trading costs. The extra
return demanded purely to offset illiquidity is the **illiquidity premium**. The canonical
empirical anchor is the U.S. Treasury market: notes and bills of identical residual
maturity and default risk nonetheless trade at different prices, because notes carry a
spread several times wider than bills — so notes trade at a discount and yield more.

The premium is not a fixed property of the asset; it depends on *how often* the holder
pays the cost. An investor who holds for `h` periods pays the round-trip spread `s` only
once over those `h` periods, so the per-period drag is roughly `s/h`. Long-horizon
holders amortize the cost over more periods and so demand a smaller per-period premium
than short-horizon traders for the same spread.

```
 gross
 return R
   ^
   |                                  . slope = 1/h  (steeper for short horizon)
   |                              .
   |                          .
   |                      .
   | r + risk prem  .________________________  (intercept = net required return E(r_j))
   |              .
   +--------------------------------------------> bid-ask spread s
                  0
   R = (net required return)  +  illiquidity premium s/h
```

**Source:** Foucault, Pagano & Roell (2013) §9.2.1 pp.308-311.

## Definition
Let the midquote equal fundamental value `m_t = mu_t`, with proportional spread `s_t`
giving ask `a_t = m_t(1 + s_t/2)` and bid `b_t = m_t(1 - s_t/2)`. An investor buys at the
ask at date `t`, holds `h` (dividend-free) periods, and sells at the bid at `t+h`; the
asset is priced by discounting the future bid at the risk-adjusted required *net* return.
The **illiquidity premium** is the component of the required *gross* return that
compensates the holder for these transaction costs.

For a risky asset `j` with CAPM beta `beta_j`, the required gross return decomposes as

    E(R_j) = r + s_j/h + beta_j [ E(r_M) - r ],

i.e. risk-free rate + illiquidity premium `s_j/h` + systematic risk premium. The **clientele
effect** (Amihud-Mendelson) is the equilibrium sorting in which investors with longer
holding periods self-select into less liquid (higher-spread) assets, making the
equilibrium gross-return-vs-spread relation concave rather than linear.

**Source:** Foucault, Pagano & Roell (2013) §9.2.1-9.2.2 pp.309-313.

## Mathematical Reasoning
**Pricing and the premium.** No-arbitrage discounting of the future bid by the future ask,
`a_t = b_{t+h}/(1+r)^h`, rearranges to express current value as discounted future value
times an illiquidity adjustment `(1 - s_{t+h}/2)/(1 + s_t/2)`, a term decreasing in both
the current and future spread — higher spreads lower the asset's value. Converting to
returns, the per-period gross return solves
`(1+R)^h = (1+r)^h (1 + s_t/2)/(1 - s_{t+h}/2)`. Because the last factor exceeds 1, the
gross return `R` strictly exceeds the net required return `r`; the wedge is the trading-cost
compensation. With a constant spread `s_t = s`, a first-order approximation for small `r`,
`s` gives `R ≈ r + s/h`: the premium rises in the spread at rate `1/h`, inversely
proportional to the holding period, because the round-trip cost is borne only once per
`h` periods.

**Clientele concavity.** With two assets (spreads `s_1 < s_2`) and two clienteles
(horizons `h_1 < h_2`), an equilibrium in which clientele 1 holds the liquid asset and
clientele 2 the illiquid one requires each group to weakly prefer its own habitat:
`R_1 - s_1/h_1 >= R_2 - s_2/h_1` and `R_2 - s_2/h_2 >= R_1 - s_1/h_2`. Together these
bound the return-spread slope between the two clienteles' indifference-curve slopes:
`1/h_2 <= (R_2 - R_1)/(s_2 - s_1) <= 1/h_1`. Each clientele's indifference curve in
(spread, gross-return) space has slope `1/h_i`; the long-horizon group has the flatter
curve and is willing to accept a higher spread for return. Extending to many assets ranked
by liquidity, the equilibrium gross returns trace a **weakly concave** locus: gross return
increases with the spread but at a *decreasing* rate, with each clientele congregating where
its indifference slope is tangent.

**Source:** Foucault, Pagano & Roell (2013) §9.2.1-9.2.2 pp.310-314 (eqs. 9.3-9.10).

## Boundary Notes
- The base model assumes the **future spread is known** at purchase. If liquidity is
  uncertain but its risk is *idiosyncratic* (uncorrelated with market returns), nothing
  changes except replacing `s_{t+h}` with `E(s_{t+h})` — investors diversify it away and
  demand no liquidity-risk premium. When liquidity instead worsens precisely when market
  returns are low and volatility high, liquidity *risk* raises systematic risk (effective
  beta) and commands a separate premium (the liquidity-adjusted CAPM, a distinct card).
- The premium is a property of expected *future* trading costs over the holding horizon,
  not just the spread at purchase; intermediate **cash flows (coupons/dividends)** reduce
  the premium because return is realized without liquidating.
- The CAPM relation `E(r_j) = r + beta_j[E(r_M)-r]` holds for *net* returns (what investors
  consume); the `s_j/h` term appears only when the model is estimated on *gross* (midquote-
  based) returns. Conflating gross and net returns misattributes the premium.
- The slope `1/h` is the *representative* investor's horizon under a single clientele; with
  heterogeneous horizons the single-slope linear reading breaks and the locus is concave.

**Source:** Foucault, Pagano & Roell (2013) §9.2.1-9.2.3 pp.311-315.

## See Also
- [`mt-liquidity-adjusted-capm`](./mt-liquidity-adjusted-capm.md) -- extends the static premium to priced *liquidity risk* (time-varying spreads) via an augmented beta.
- [`mt-three-dimensions-liquidity`](./mt-three-dimensions-liquidity.md) -- the spread `s` here is one of the tightness/depth/resiliency dimensions being capitalized.
- [`mt-price-impact-measures-amihud`](./mt-price-impact-measures-amihud.md) -- supplies the broader trading-cost measure (price impact) that `s` proxies.
- [`mt-liquidity-nature-provision-return`](./mt-liquidity-nature-provision-return.md) -- why liquidity provision is compensated, the supply side of this premium.

## Escalate to Raw When
The source proves the full pricing identity (eq. 9.4) and the exact return-spread relation
(eqs. 9.5-9.9) plus the clientele inequality chain (eq. 9.10) and figures 9.1-9.3; this card
only sketches the `R ≈ r + s/h` approximation and the concavity argument. Re-read pp.309-314
for the exact algebra, the small-`r,s` approximation conditions, and the cross-sectional
regression specification (eq. 9.11) used by Amihud-Mendelson (1986) to estimate it.
