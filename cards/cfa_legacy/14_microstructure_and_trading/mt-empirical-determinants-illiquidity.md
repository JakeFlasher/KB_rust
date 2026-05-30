---
schema_version: "cacg.v0"
id: "mt-empirical-determinants-illiquidity"
title: "Estimating the Determinants of Market Illiquidity: Structural vs Reduced-Form"
reading_id: "14_microstructure_and_trading"
summary: "Illiquidity is explained two ways: structural estimation that separates adverse-selection from inventory/processing costs, and reduced-form regressions tying spreads to volume, volatility, and firm size via the information environment."
tags: ["microstructure", "illiquidity", "adverse-selection", "price-impact", "pin", "empirical"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p166:0259"
    chunk_hash: "0135c4c7613418dd95c9cafb27930b5178fadd3fff6a061b7691d9114679236f"
    page_range: [166, 167]
    quote: "positively correlated in the cross-section with volatility (0.239) and the bid-ask spread"
    edge_type: "supports"
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p041:0049"
    chunk_hash: "8eb3ff936603f7433fac991b26ae965c6ce97604e37320a917ffa1ce1293412e"
    page_range: [42, 42]
    quote: "the present one adopts a more data-oriented statistical"
    edge_type: "supports"
---
# Estimating the Determinants of Market Illiquidity: Structural vs Reduced-Form

## Intuition
Why is one stock harder to trade than another, and why does the same stock's spread widen and narrow over time? Empirical microstructure answers this on two levels. The **reduced-form** level just correlates an illiquidity proxy (the relative bid-ask spread, or the Amihud price-impact ratio) with observable firm characteristics — trading volume, return volatility, market capitalization, price level — and lets the regression speak. The **structural** level instead writes down a price-formation model (a dealer who faces adverse selection and carries inventory) and estimates its deep parameters, so that the spread is decomposed into economically labeled pieces rather than left as a black-box number.

Both levels point at the same economic driver: the **information environment**. Stocks about which the market is poorly informed (small caps, no analyst coverage, high volatility) expose liquidity suppliers to more informed-trading risk, so they quote wider. This is why a single empirical regularity — small, volatile, thinly covered stocks are illiquid — shows up whether you measure it crudely (a cross-sectional spread regression) or finely (a structurally estimated probability of informed trading, PIN).

```
        ILLIQUIDITY PROXY (spread, Amihud ratio)
                       |
        +--------------+---------------+
        |                              |
  REDUCED-FORM                    STRUCTURAL
  spread ~ a + b*log(volume)      Glosten-Harris / Hasbrouck VAR / PIN-MLE
        + c*volatility + ...      => separate lambda (adverse seln)
  (signs, fit, "what")               from beta (inventory), gamma (processing)
        \____________  same driver  ____________/
                INFORMATION ENVIRONMENT
        (size down, vol up, coverage down => illiquid)
```

**Source:** Foucault, Pagano & Röell (2013) ch.5 (Estimating the Determinants of Market Illiquidity) pp.165-167.

## Definition
Let `dt` be the trade-direction indicator (+1 buy, -1 sell), `qt` the signed trade size, and `pt` the transaction price. The **reduced-form** illiquidity object is a regression whose dependent variable is a liquidity proxy (relative spread `bas` or the Amihud ratio) and whose regressors are firm/market characteristics (log volume, volatility, log market cap, turnover, log price); coefficients and explanatory power summarize *what* covaries with illiquidity without claiming a mechanism.

The **structural** object is a price-impact equation derived from a dealer model. In the general Glosten-Harris form the efficient value updates as `μt = μt-1 + λ0·dt + λ1·qt + εt`, and the observed price change satisfies `Δpt = λ0·dt + λ1·qt + γ0·Δdt + γ1·Δqt + εt`, where `λ` parameters carry **adverse selection** (permanent impact) and `γ` parameters carry **order-processing cost** (transitory impact). The **PIN** structural object instead posits Poisson order arrivals (informed intensity `εi` on information days arriving with probability `α`, uninformed buy/sell intensities `εb`, `εs`) and defines `PIN = α·εi / (εb + εs + α·εi)` as the fraction of order flow that is informed, estimated by maximum likelihood from daily buy/sell counts.

**Source:** Foucault, Pagano & Röell (2013) §5.2, §5.4, eqs.(5.6)-(5.7), (5.27) pp.168-182.

## Mathematical Reasoning
The key reason structure is needed — not just a reduced-form regression of `Δpt` on `qt` — is that **adverse selection and inventory cost are observationally identical in the short run**. Adding inventory holding cost `β` to the value equation `μt = μt-1 + λ·qt + εt` gives the dealer pre-trade valuation `mt = μt-1 + εt - β·zt`. Using market clearing `Δzt = -qt-1`, consecutive midquote changes become `Δmt = (λ + β)·qt-1 + εt`, so a naive regression of price change on order flow recovers the **sum** `λ + β` and therefore *overestimates* the informativeness `λ` of order flow.

Identification comes from a behavioral asymmetry. Only the **unexpected** part of the trade conveys information, because the predictable part is already priced: writing `μt = μt-1 + λ·[qt − E(qt|Ωt-1)] + εt`, the adverse-selection term loads on the trade *innovation* `qt − E(qt|Ωt-1)`, whereas the inventory term `β·qt` loads on the *full* trade size (inventory risk depends on the whole position change). Modeling the order flow as serially correlated (e.g. AR(1): `qt = φ·qt-1 + ηt`, so `E(qt|Ωt-1) = φ·qt-1`) makes the innovation `qt − φ·qt-1` move differently from the level `qt`, so the two coefficients separate. This is the mechanism that lets one estimate `λ` and `β` distinctly.

Comparative statics tie the structural estimate back to the reduced-form signs. In the opening-trade special case the spread satisfies `a1 − b1 = 2·PIN·(vH − vL)`, so illiquidity is **increasing in PIN**. Cross-sectionally PIN is negatively correlated with firm size (correlation ≈ −0.58: larger firms have relatively less informed trading) and positively correlated with volatility and the bid-ask spread. The volatility link is doubly signed in the structure: volatile stocks have more frequent information events (higher `α`) and larger informed payoffs (higher `εi`), both of which raise PIN and hence the spread.

**Source:** Foucault, Pagano & Röell (2013) §5.2.2-§5.4, eqs.(5.9)-(5.20), (5.27) pp.170-183.

## Boundary Notes
- **Reduced-form caveats.** Spread regressions are descriptive, not causal: volume, volatility, and size are mutually correlated, so coefficient signs depend on specification (which is why the source asks the same regression in six forms). A reduced-form fit cannot decompose the spread into economic components.
- **Structural caveats.** Price-impact regressions face real econometric problems: trade direction is often unobserved and must be inferred (Lee-Ready), inducing measurement error; discrete price grids inject rounding error that creates negative residual autocorrelation; and intraday news arrival makes errors heteroskedastic — handled by joint MLE (Glosten-Harris), Newey-West, or GMM (Huang-Stoll). The simple `Δpt = (λ+β)·qt + ...` form is **misspecified** when inventory costs are present and silently conflates `λ` with `β`.
- **PIN limits.** PIN is empirically very stable over time and, notably, **not** correlated with trading volume — possibly because informed traders adjust to noise-trader intensity to keep PIN invariant. PIN may also pick up cross-sectional variation driven by order fragmentation rather than pure information asymmetry, so it is one structural lens, not the truth.
- **Scope.** This card is about *estimating and explaining* illiquidity (post-2010 empirical evidence on cross-sectional and time variation). The *mechanics* of how the spread is decomposed, the PIN model internals, and the Amihud measure construction live in the sibling cards.

**Source:** Foucault, Pagano & Röell (2013) §5.2.1, §5.4 pp.168-184; Hasbrouck (2007) ch.4 (structural vs reduced-form framing) p.42.

## See Also
- [`mt-spread-decomposition-components`](./mt-spread-decomposition-components.md) -- supplies the adverse-selection / inventory / processing pieces this card estimates
- [`mt-pin-probability-informed-trading`](./mt-pin-probability-informed-trading.md) -- the structural MLE object whose cross-sectional correlations drive these determinants
- [`mt-price-impact-measures-amihud`](./mt-price-impact-measures-amihud.md) -- the Amihud ratio used as the reduced-form illiquidity dependent variable

## Escalate to Raw When
- You need the full price-impact derivation including the `λ0`, `λ1`, `γ0`, `γ1` interpretation and the Glosten-Harris restriction tests (FPR §5.2, eqs.(5.6)-(5.20)).
- You need the complete PIN likelihood (eqs.(5.28)-(5.31)) and the Easley-Hvidkjaer-O'Hara estimation results / exact correlation magnitudes.
- You need the econometric treatment of trade-direction inference, discreteness, and heteroskedasticity, or Hasbrouck's VAR / Wold reduced-form machinery — re-read FPR pp.168-184 and Hasbrouck (2007) ch.3-4.
