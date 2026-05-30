---
schema_version: "cacg.v0"
id: "fa-amihud-mendelson-and-priced-liquidity-risk"
title: "Amihud-Mendelson & Priced Liquidity Risk: Level Premium + the LCAPM"
reading_id: "22_fund_level_arbitrage"
summary: "The bid-ask spread is a priced add-on cost: less-liquid assets sort to longer-horizon investors, so the level premium scales with cost-over-horizon. The Acharya-Pedersen LCAPM adds liquidity risk via four betas, with the fire-sale beta dominating."
tags: ["amihud-mendelson", "lcapm", "four-betas"]
citations:
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p093:0096"
    chunk_hash: "48736a18e49748aec2f0be90041956418908f66cfcfb353231aae0557e3b91e7"
    page_range: [93, 93]
    quote: "These investors’ longer holding periods mitigate the compensation that they would have required for higher transaction costs on less liquid securities."
    edge_type: "defines"
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p117:0124"
    chunk_hash: "a6ffa4ed52bdebc5a2a36ed1e9ed5965b159fdd95076038af18021f3e5070bb1"
    page_range: [118, 118]
    quote: "expected level of liquidity plus four betas multiplied by the market risk premium"
    edge_type: "supports"
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p120:0128"
    chunk_hash: "d9fc3167f8113d13899b04d855d1f6f06e50ba7c8458c25dd15b22efeb19222a"
    page_range: [121, 121]
    quote: "is the least important component of the liquidity risk premium"
    edge_type: "supports"
---
# Amihud-Mendelson & Priced Liquidity Risk: Level Premium + the LCAPM

## Intuition
Trading is not free: every round trip pays away a bid-ask spread, so an illiquid
security must offer a higher gross return to leave the same net return as a liquid
one. But the spread is paid only when you trade, so an investor who holds for many
periods amortizes the cost over a long horizon — the per-period drag shrinks. In
equilibrium this sorts clienteles: patient, long-horizon investors are willing to
own the most illiquid assets because they barely feel the spread, while short-horizon
traders crowd into liquid names. The compensation for spread therefore rises with
the spread but at a *decreasing* rate (return is concave in the spread), because the
investors who hold the worst-liquidity assets are precisely those who trade least.

```
       gross return
          ^
          |                         . . . . . .   (concave: long-horizon
          |                . . '                    clientele caps the
          |          . '                            extra return demanded)
          |      . '
          |   . '
          | .'
          +-------------------------------------> bid-ask spread (illiquidity)

  short-horizon holders  ->  liquid (low-spread) end
  long-horizon holders   ->  illiquid (high-spread) end   (clientele sorting)
```

**Source:** van der Merwe (2015) §4 pp.93-94.

## Definition
The exogenous "add-on" cost of trading is the proportional bid-ask spread `s_t`,
splitting the midquote `m_t` (taken equal to fundamental value `mu_t`) into an ask
`a_t = m_t(1 + s_t/2)` and a bid `b_t = m_t(1 - s_t/2)`. The **liquidity level
premium** is the compensation for paying this spread, equal in the
Hagstromer-Hansson-Nilsson decomposition to the ratio of the expected liquidity cost
to the expected holding period. The **liquidity-adjusted CAPM (LCAPM)** of Acharya
and Pedersen extends the classic mean-variance/CAPM frictionless model by making the
per-security transaction cost `c_t` a stochastic process; net return is `r_j = R_j -
c_j` (gross return minus transaction cost), and the model reduces to ordinary CAPM
when trading costs are zero. The **total liquidity premium** = liquidity level
premium + liquidity risk premium.

**Source:** van der Merwe (2015) §4 pp.94-95, pp.116-117, p.121.

## Mathematical Reasoning
Discounting the future bid against today's ask gives the level relation
`a_t = b_{t+h}/(1+r)^h`. Substituting the bid/ask split yields a current value that
shrinks in both the current and the expected future spread:
`mu_t = mu_{t+h} * (1 - s_{t+h}/2) / [(1+r)^h (1 + s_t/2)]` — the bracketed factor is
an illiquidity discount that falls as `s_t` or `s_{t+h}` rises. Because the spread is
amortized over `h` periods, the per-period premium scales roughly like
`E[c]/E[h]` — the level premium — which is concave in `s`.

The LCAPM states the required excess gross return as the expected liquidity level
`E(c_j)` plus four betas times the market risk premium `lambda = E(R_M - c_M) - r_f`:

```
E(R_j) = r_f + E(c_j) + lambda*beta1 + lambda*(beta2 - beta3 - beta4)

  beta1 = cov(R_j, R_M)/var(r_M)      market beta (cost-adjusted CAPM term)
  beta2 = cov(c_j, c_M)/var(r_M)      commonality: illiquid WHEN market illiquid (+)
  beta3 = cov(R_j, c_M)/var(r_M)      return rises when market illiquid -> hedge (-)
  beta4 = cov(c_j, R_M)/var(r_M)      illiquid WHEN market falls -> fire-sale (-)
```

Collecting the liquidity terms, `beta_LIQ = beta2 - beta3 - beta4`, so
`E(R_j) = r_f + E(c_j) + lambda*beta1 + lambda*beta_LIQ`, cleanly separating market
risk (`beta1`) from priced liquidity risk (`beta_LIQ`). Comparative statics:
`beta2 > 0` and `beta3 < 0`, `beta4 < 0` raise the required return (you dislike an
asset that turns illiquid in a bad market, or whose liquidity dries up in a downturn).
Empirically the commonality beta `beta2` is the *least* important component while the
fire-sale channel `beta4` (asset illiquid exactly when the market is down, forcing
sales at a deep discount) tends to dominate the priced liquidity risk.

**Source:** van der Merwe (2015) §4 pp.95, pp.117-119, p.121.

## See Also
- [`fa-liquidity-measurement-and-price-impact`](./fa-liquidity-measurement-and-price-impact.md) — supplies the spread / Amihud / Kyle measures that feed the level premium and the LCAPM betas.
- [`fa-amihud-mendelson-and-priced-liquidity-risk`](./fa-amihud-mendelson-and-priced-liquidity-risk.md) sits beside [`fa-funding-spirals-and-fire-sales`](./fa-funding-spirals-and-fire-sales.md) — the `beta4` fire-sale channel is realized when investors hit funding constraints and liquidate into a distressed market.
- [`fa-illiquidity-discount-and-crisis-amplification`](./fa-illiquidity-discount-and-crisis-amplification.md) — the time-varying, crisis-peaking behavior of the liquidity risk premium.
- Legacy cross-refs (other tree, prose only): the LCAPM is the liquidity-augmented sibling of the plain CAPM and SML covered by pm-capm-and-sml, and it extends the basic compensation-for-risk logic in pm-return-and-risk-fundamentals.
- [`mt-liquidity-adjusted-capm`](../14_microstructure_and_trading/mt-liquidity-adjusted-capm.md) and [`mt-liquidity-premium-asset-pricing`](../14_microstructure_and_trading/mt-liquidity-premium-asset-pricing.md) (reading 14) derive the LCAPM four-beta model and the clientele level premium from primary sources; this card adds the level-vs-risk split and the fund-return-attribution frame.

## Escalate to Raw When
Go to the raw chapter when you need the worked numeric magnitudes — e.g. the
Hagstromer-Hansson-Nilsson NYSE estimates that split a total annual liquidity premium
into a level component and a risk component, the Figure 4.4 time series showing risk-
premium peaks at the 1930s Depression, WWII, 1987, LTCM 1998 and Lehman 2008, or the
calibrated bid-ask-vs-return curve of Figure 4.1. Also escalate for the full
derivation steps of equations (4.3)-(4.5) and the Foucault simplification of (4.24),
or for the LTCM and asset-fire-sale case studies van der Merwe uses to illustrate the
`beta3` and `beta4` channels concretely.

**Source:** van der Merwe (2015) §4 pp.93-94, pp.120-122.
