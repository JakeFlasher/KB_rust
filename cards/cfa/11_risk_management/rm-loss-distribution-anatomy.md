---
schema_version: "cacg.v0"
id: "rm-loss-distribution-anatomy"
title: "Loss-Distribution Anatomy — P&L Convention, Horizons, and Mapping"
reading_id: "11_risk_management"
summary: "Loss-distribution anatomy at McNeil's Ch.2 entry-point depth: the loss convention L_{t+1} = −ΔV, the factor mapping V(t,Z_t), the choice of horizon Δt, and the acceptance-set framing that connects the loss distribution to required capital via translation-invariance."
tags: ["risk-management", "loss-distribution"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p079:0110"
    chunk_hash: "dc598d5343cf28a8152e1d0a325b674c4d9d667d3bc869fc8ede354527ff4180"
    page_range: [79, 79]
    quote: "In an analytical method we attempt to choose a model for Xt+1 and a mapping function f in such a way that the distribution of Lt+1 can be determined analytically."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p296:0424"
    chunk_hash: "763c816c0294ae7f1b894657552fb8969e0f5112e1c079ff5de5eca34bdc662b"
    page_range: [296, 297]
    quote: "Consider a probability space (Ω, F ,P) and a linear space M ⊂ L0(Ω, F ,P), where L0(Ω, F ,P) denotes the set of all random variables on (Ω, F ,P) that are almost surely (a.s.) finite."
    edge_type: "supports"
  - source_id: "rm_hull_2023_rmfi"
    chunk_id: "rm_hull_2023_rmfi:p631:0874"
    chunk_hash: "cddcc51bff72a09eea6e9eab085bfb3e17e09dfb03624fe28a535a37242ecf1d"
    page_range: [632, 632]
    quote: "The loss probability distributions for market, credit, and operational risk are very"
    edge_type: "supports"
  - source_id: "rm_embrechts_kluppelberg_mikosch_1997_modelling_extremal_events"
    chunk_id: "rm_embrechts_kluppelberg_mikosch_1997_modelling_extremal_events:p061:0057"
    chunk_hash: "e247008845b1033eae828f8984bf6b4fa7e18a560e8cfe8e6752ccd773d637a5"
    page_range: [61, 61]
    quote: "We have already encountered members of the following three"
    edge_type: "supports"
  - source_id: "rm_bouchaud_potters_2003_theory_financial_risk"
    chunk_id: "rm_bouchaud_potters_2003_theory_financial_risk:p028:0029"
    chunk_hash: "3f9fd2cce34c0ac31c079446bff23802fcc779b59229bdca640bdf6db7e86bf3"
    page_range: [29, 29]
    quote: "the cumulants simply add when one sums independent random variables"
    edge_type: "supports"
  - source_id: "rm_sornette_2017_why_stock_markets_crash"
    chunk_id: "rm_sornette_2017_why_stock_markets_crash:p077:0083"
    chunk_hash: "1d0fd7ff8f1b198167b079e38cf7bc3ed1734d7035d6a8d68e3095f507f19819"
    page_range: [77, 77]
    quote: "is defined as a persistent decrease in the price over consecutive"
    edge_type: "supports"
card_hash: "917e99f0212445b5827512604bd91242442bae984afefe395a4eb3e481b2de67"
---
# Loss-Distribution Anatomy — P&L Convention, Horizons, and Mapping

## Intuition

Quantitative risk management speaks in the language of **losses**, not profits. The convention is `L = −ΔV = −(V_1 − V_0)`, where `V_t` is the portfolio value at time `t`. A loss is a non-negative random variable whenever the portfolio loses money; a gain shows up as a negative loss. This sign convention is the small but load-bearing translation that makes the rest of the apparatus — risk measures, capital, exceedances — consistent. **Source:** McNeil et al. (2015) Ch.2 pp.58-60.

The **loss distribution** `F_L(l) = P(L ≤ l)` is the central object. Every downstream risk number is a functional of `F_L`: `VaR_α` is a quantile of `F_L`, `ES_α` is a tail-conditional expectation of `F_L`, scenario stress reads from the upper tail of `F_L` under a stressed factor measure. **The choice of horizon, the factor mapping, and the conditioning information together determine `F_L`** — different choices give different risk numbers for the same portfolio at the same instant, so being explicit about all three is part of the discipline. **Source:** McNeil et al. (2015) Ch.2 pp.58-61.

Three structural decisions shape any loss distribution: **horizon `Δt`** (intraday / daily / multi-day for market-risk capital, annual for credit / operational capital), **factor mapping** `V = V(X)` (which risk factors `X` enter, see `[[rm-sensitivity-versus-simulation]]`), and **conditioning** (unconditional `F_L` vs conditional `F_L(· | F_t)` given current information). Changing the level `α` alone re-specs only the quantile read off `F_L`; changing the horizon `Δt`, the factor mapping, or the conditioning information re-specs the loss distribution itself. **Source:** McNeil et al. (2015) Ch.2 pp.58-61.

```
<!-- primitive: var-tail-and-es source: _diagram_primitives.md -->
   density f_L(l)
   ^
   |   * * *
   |  *       *
   | *          *
   |*             *
   |*               *
   |*                 *
   |*                   *
   |*                      *           VaR_α = q_α(L)
   |*                          *       (α-quantile of loss L)
   |*                              *
   |*                                  ES_α = E[L | L >= q_α]
   |*                                     *
   |*           body of f_L                * tail (mass = 1 − α)
   |*                                          *
   |*                                                 *  *
   +*------------------------------------*------------------> L
                                       q_α (VaR)
       <-------- α probability mass -------->
       <------- 1 − α tail mass: ES averages L here ------>
```

## Definition

Let `V_t` denote the value of a portfolio at time `t`, and let `Δt` be the chosen risk horizon. The **one-period loss** is: **Source:** McNeil et al. (2015) Ch.2 pp.58-59.

```
L_{t+Δt}  =  − ( V_{t+Δt}  −  V_t )
```

Under the **factor-mapping convention** `V_t = V(t, X_t)` for some risk-factor vector `X_t ∈ R^d`, the loss is: **Source:** McNeil et al. (2015) Ch.2 pp.59-60.

```
L_{t+Δt}  =  −[ V(t + Δt, X_t + ΔX) − V(t, X_t) ]      where ΔX = X_{t+Δt} − X_t
```

The **linearised loss** (first-order Taylor expansion) is `L^Δ = −[ V_t · Δt + ∇_X V_t · ΔX ]`, useful when the factor mapping is approximately linear over the horizon. See `[[rm-sensitivity-versus-simulation]]` for the full Taylor calculus. **Source:** McNeil et al. (2015) Ch.2 pp.59-61.

The **loss distribution** at horizon `Δt` is the law of `L_{t+Δt}` under the chosen probability measure: **Source:** McNeil et al. (2015) Ch.2 pp.60-61.

```
F_{L,Δt}(l)  =  P( L_{t+Δt} ≤ l )
```

The **conditional loss distribution** `F_{L,Δt}(l | F_t) = P( L_{t+Δt} ≤ l | F_t )` conditions on the information available at `t` (typically: current factor levels and a chosen filtration). The unconditional distribution `F_{L,Δt}(l)` averages over `F_t`; the conditional distribution adapts to the current state. **Source:** McNeil et al. (2015) Ch.2 pp.60-61.

A position with **acceptance set** `A` is acceptable if `L ∈ A`. The natural acceptance set generated by a risk measure `ρ` is `A_ρ = { L : ρ(L) ≤ 0 }`. Equivalently, with capital `C` held against the position, acceptability requires `ρ(L − C) ≤ 0`, which by translation invariance reduces to `C ≥ ρ(L)`. **Source:** McNeil et al. (2015) Ch.2 pp.61 + Ch.8 pp.275-280.

## Mathematical Reasoning

The sign convention `L = −ΔV` makes the **right tail of `F_L`** the bad-state region — high losses correspond to high `L`. This aligns the loss distribution with the standard quantile convention `q_α(L) = inf{l : F_L(l) ≥ α}` so that `VaR_α = q_α(L)` is a high quantile of `L`, not a low quantile of `ΔV`. Without the sign flip, every risk-measure formula would carry a confusing minus sign; with it, the apparatus inherits clean quantile / expectation conventions from probability theory. **Source:** McNeil et al. (2015) Ch.2 pp.58-60.

The **horizon choice `Δt`** has a non-trivial scaling property. For a Gaussian factor model with i.i.d. log-returns, loss volatility scales as `σ_{Δt} = σ_1 · √Δt` (the "square-root-of-time" rule), so `VaR_α(Δt) ≈ √Δt · VaR_α(1)` for purely diffusive risk. The rule **breaks** under fat tails, autocorrelation, gap risk, or liquidity overlays — extrapolating a short-horizon VaR up to a longer horizon by the `√Δt` factor is known to under-estimate the genuine multi-day tail in stressed regimes. McNeil sets out the diffusive-vs-non-diffusive distinction at the entry-point depth. **Source:** McNeil et al. (2015) Ch.2 pp.59-61.

The **factor mapping `V(X)`** is the genuine modelling choice — every other ingredient (`Δt`, `α`, `ρ`) is essentially regulatory. Two competing mappings on the same portfolio give two different `F_L`s: an equity book mapped through individual stocks captures idiosyncratic dispersion; mapped through factor models (market / size / value / momentum) captures systematic risk only and underestimates concentration. The vertical's `[[rm-sensitivity-versus-simulation]]` card formalises the trade-off between low-dimensional sensitivity mappings (cheap, blind to non-linearities) and high-dimensional simulation mappings (expensive, captures non-linearities). **Source:** McNeil et al. (2015) Ch.2 pp.61-64.

The **conditional vs unconditional** distinction matters most under volatility clustering (GARCH-like dynamics). A conditional `F_L(· | F_t)` from a volatility-targeting model **rises** during a stressed period — its right tail thickens with the current vol estimate, raising VaR / ES exactly when the firm is most exposed. An unconditional `F_L` averaged over decades has a fatter right tail but does not respond to current-state information, so it under-reacts during a crisis and over-reacts during calm. Risk-on-risk-off practice typically uses conditional measures for limit enforcement and unconditional measures for capital-floor calibration. **Source:** McNeil et al. (2015) Ch.2 pp.60-61.

The **acceptance-set framing** provides the formal bridge between the loss distribution and the capital decision. The set `A_ρ = {L : ρ(L) ≤ 0}` collects all loss positions the firm can hold without additional capital. For a coherent measure `ρ`, `A_ρ` is a **convex cone** — the convexity inherits from subadditivity (`L_1, L_2 ∈ A` ⇒ `L_1 + L_2 ∈ A`) and positive-homogeneity (`L ∈ A`, `λ ≥ 0` ⇒ `λ L ∈ A`). The Artzner-Delbaen-Eber-Heath dual-representation theorem characterises every coherent `ρ` as a supremum of expected losses over a set of "generalised scenarios" — see `[[rm-risk-measure-axioms]]` for the surface treatment and McNeil Ch.8 §8.1.2 for the proof. **Source:** McNeil et al. (2015) Ch.2 pp.61 + Ch.8 pp.275-280.

## See Also

- [rm-risk-measure-axioms](./rm-risk-measure-axioms.md) — the four coherence axioms applied to functionals of `F_L`.
- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — `VaR_α` and `ES_α` as the two canonical functionals read off `F_L`.
- [rm-sensitivity-versus-simulation](./rm-sensitivity-versus-simulation.md) — the two ways to construct `F_L` from the factor mapping `V(X)`.
- `rm-loss-distribution-shapes-by-risk-type` (Hull (2023) RMFI, pp.632) — deepening that extends this card.
- `rm-heavy-tail-class-hierarchy` (Embrechts-Klueppelberg-Mikosch (1997) Modelling Extremal Events, pp.61) — deepening that extends this card.
- `rm-cumulant-expansion-nongaussian-correction` (Bouchaud-Potters (2003) Theory of Financial Risk, pp.29) — deepening that extends this card.
- `rm-crashes-as-outliers-drawdowns` (Sornette (2017) Why Stock Markets Crash, pp.77) — deepening that extends this card.

## Escalate to Raw When

The conceptual depth in this card stops at the loss convention, the horizon / mapping / conditioning decision triple, and the acceptance-set framing. When the operator needs the full empirical loss-distribution estimation machinery (extreme-value theory POT fits, generalised Pareto distribution for the tail, time-series volatility modelling via GARCH / EGARCH / FIGARCH, or formal Daniell-type dual-representation results on general probability spaces), open McNeil Ch.5 pp.151-180 (EVT), Ch.4 pp.119-150 (time series), and Ch.8 §8.1.2-§8.1.3 pp.280-285 (dual representation) directly. **Source:** McNeil et al. (2015) Ch.4-5 pp.119-180 + Ch.8 pp.280-285.
