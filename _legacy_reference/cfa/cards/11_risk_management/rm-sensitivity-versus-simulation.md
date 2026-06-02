---
Use when: classifying risk-measure approaches as sensitivity-based (delta / gamma / vega) versus simulation-based (historical / Monte Carlo) with the portfolio-mapping vs computational-cost trade-off
Primary raw source: 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.61-64
Supporting sources:
  - 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329
Repo touchpoints:
  - ../07_derivatives_and_volatility/deriv-greeks-overview.md
  - .claude/knowledge/11_risk_management/rm-loss-distribution-anatomy.md
  - .claude/knowledge/11_risk_management/rm-delta-gamma-vega-pl-decomposition.md
Out of scope: instrument-level Greeks derivation (07 territory); per-instrument Monte Carlo pricing (07 territory)
CFA Relevance: extension
Source Stance: primary-mcneil
deliverable-ready: true
---

# Sensitivity-Based vs Simulation-Based Risk Measurement

## Intuition

Every risk-measure pipeline maps **risk factors** (yields, equity prices, FX rates, volatilities, credit spreads) to a portfolio **P&L distribution** from which `VaR`, `ES`, or a sensitivity number is read off. Two structurally different approaches dominate practice. **Sensitivity-based** methods linearise (or low-order-Taylor-expand) the portfolio value `V(X)` around current risk-factor levels `X_0` and read risk off the **derivatives** of `V`: `Δ = ∂V/∂S`, `Γ = ∂²V/∂S²`, `ν = ∂V/∂σ`, etc. **Simulation-based** methods do not linearise — they sample many risk-factor scenarios `X^{(k)}` (from history or from a Monte Carlo generator), full-revalue the portfolio `V(X^{(k)})` in each scenario, and read risk off the empirical loss distribution `{−ΔV^{(k)}}`. **Source:** McNeil et al. (2015) Ch.2 pp.61-64.

The trade-off is fundamental: **sensitivities are cheap but local**, while **simulation is expensive but global**. A delta-gamma-vega P&L approximation can be evaluated quickly once the Greeks are computed, but it is accurate only for small risk-factor moves — exactly the regime that VaR and ES are NOT trying to summarise. Full revaluation over a large scenario sample captures non-linear payoffs (digital options, callable bonds, gap risk on barrier products) but costs `O(N_scenarios × cost_of_one_repricing)`, which for instrument books with American/path-dependent features can blow up. **Source:** McNeil et al. (2015) Ch.2 pp.62-64.

In practice, the two are layered: sensitivity-based numbers (delta, gamma, vega) drive day-to-day P&L attribution and intraday limit checks; simulation-based numbers (historical VaR, Monte Carlo VaR, ES) drive end-of-day regulatory capital and stress reports. Cards under `[[rm-delta-gamma-vega-pl-decomposition]]` and the Batch 2 simulation-VaR cards develop each side. **Source:** McNeil et al. (2015) Ch.2 pp.63-64 + Ch.9 pp.327-329.

```
<!-- primitive: pl-distribution-decomposition source: _diagram_primitives.md -->
   density f_ΔV(v)
   ^
   |            * *  total P&L = ΔV
   |          *      *
   |        *           *
   |      *                *
   |    *                    *
   |  *                        *
   |*                            *
   +------------------------------------> ΔV

   ΔV   ≈   Δ · ΔS    +    1/2 · Γ · (ΔS)^2    +    ν · Δσ    +    ε
          (delta)         (gamma curvature)       (vega)        (residual)

   * delta contribution: linear in underlying move ΔS
   * gamma contribution: convexity term (non-negative for long-option books)
   * vega contribution:  sensitivity to volatility shift Δσ
   * residual ε:         theta, rho, higher-order cross-Greeks
```

## Definition

A **risk-factor mapping** is a function `V : R^d → R` taking a `d`-dimensional risk-factor vector `X = (X_1, …, X_d)` (rates, prices, vols, spreads) to the portfolio value. The one-period loss is `L = −(V(X_1) − V(X_0))` where `X_0` is the current factor vector and `X_1` is the next-period factor vector. **Source:** McNeil et al. (2015) Ch.2 pp.59-62.

A **sensitivity-based** approximation replaces `V(X_1)` with a Taylor expansion around `X_0`: **Source:** McNeil et al. (2015) Ch.2 pp.62-63.

```
V(X_1) − V(X_0)  ≈  ∇V(X_0)^T · ΔX  +  1/2 · ΔX^T · H(X_0) · ΔX
                    └────────────┘    └────────────────────────┘
                     first-order        second-order (gamma)
                     (delta / vega)
```

where `∇V` is the gradient (delta vector for spot factors, vega vector for vol factors, …) and `H` is the Hessian matrix of second derivatives. Cross-derivatives like `∂²V/∂S∂σ` (vanna) and `∂²V/∂σ²` (volga) enter `H`. **Source:** McNeil et al. (2015) Ch.2 pp.62-63.

A **simulation-based** estimate of any law-determined risk measure `ρ(L)` (in particular VaR, ES) draws scenarios `X_1^{(k)}` for `k = 1, …, N` (historical bootstrap or Monte Carlo from a calibrated joint factor model), full-revalues `V(X_1^{(k)})` per scenario, and reports the empirical statistic on the sample `{L^{(k)}} = {−(V(X_1^{(k)}) − V(X_0))}`. **Source:** McNeil et al. (2015) Ch.2 pp.63-64 + Ch.9 pp.327-329.

## Mathematical Reasoning

The sensitivity approach inherits its accuracy guarantee from Taylor's theorem: the **remainder is `O(‖ΔX‖³)`** if the Hessian is bounded and the third derivatives are bounded in a neighbourhood. For small factor moves this is excellent; for the high-confidence tail moves that drive VaR/ES, the cubic term can dominate — especially for option books where `∂³V/∂S³` (speed) and `∂³V/∂S²∂σ` (vomma) are non-trivial. The boundary at which delta-gamma-vega fails is not a constant; it depends on the portfolio's **moneyness profile** and **convexity scale**. **Source:** McNeil et al. (2015) Ch.2 pp.62-63 + Ch.9 pp.327-329.

A canonical pathology: a delta-hedged long-gamma book with positive vega has `Δ ≈ 0` and `Γ, ν > 0`. The delta term contributes nothing to the linear P&L; the gamma term contributes a positive quadratic in `ΔS`; the vega term contributes a positive linear term in `Δσ`. A delta-only sensitivity number reports near-zero risk, hiding the genuine gamma + vega exposure. Adding the second-order term recovers the gamma contribution but still misses the cross-derivative vanna when `S` and `σ` are correlated under the joint scenario set. Simulation has no such blind spot — it revalues directly. **Source:** McNeil et al. (2015) Ch.9 pp.327-329.

The simulation approach inherits its accuracy guarantee from the **Glivenko-Cantelli theorem** (the empirical CDF converges uniformly to the true CDF as `N → ∞`) and, for the specific case of quantile estimation, from the **Bahadur-Kiefer representation** that gives `√N`-asymptotic-normality of the empirical quantile with variance proportional to `α(1−α) / f_L(q_α)²`. The variance blows up when the loss density `f_L` is small at the quantile — a known weakness of historical VaR for deep tail levels (`α` close to 1) where exceedance count is tiny. Monte Carlo addresses sample-size scarcity by allowing arbitrary `N`, but transfers the modelling burden to the joint factor distribution generator. **Source:** McNeil et al. (2015) Ch.9 pp.327-329.

There is a **hybrid** approach widely used in practice: a **delta-gamma Monte Carlo VaR** revalues the second-order P&L approximation `Δ·ΔX + 1/2·ΔX^T·H·ΔX` over `N` scenarios instead of full-revaluing `V`. This recovers most of the simulation's distributional sensitivity at a fraction of the cost — full revaluation needs `N · cost(V)` whereas delta-gamma-MC needs `N · cost(quadratic-form)` which is cheap. The price is that any payoff non-linearity beyond second order (digital cliffs, barrier knock-outs, deep-out-of-the-money convexity) is missed. The Batch 2 Monte Carlo VaR card develops this hybrid; see `[[rm-monte-carlo-var]]`. **Source:** McNeil et al. (2015) Ch.9 pp.327-329.

The vertical's design treats sensitivity-vs-simulation as a **diagnostic dichotomy**, not a competitive choice: every well-instrumented risk function runs both. Sensitivities answer "what is moving the P&L right now?" (attribution + intraday limits); simulation answers "how bad can it get at the `α` tail?" (capital + stress). Disagreement between the two — for example, a quiet delta-gamma read but an elevated historical VaR — is a **diagnostic signal**, typically pointing to a non-linear payoff or a correlated-factor regime that the Greeks do not capture. **Source:** McNeil et al. (2015) Ch.9 pp.327-329.

## See Also

- [rm-delta-gamma-vega-pl-decomposition](./rm-delta-gamma-vega-pl-decomposition.md) — Batch-1 notes-anchored card developing the second-order P&L approximation with vanna and volga cross-terms.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — defines the loss convention `L = −ΔV` that both approaches read.
- [rm-monte-carlo-var](./rm-monte-carlo-var.md) — Batch-2 card developing the delta-gamma Monte Carlo hybrid.
- [../07_derivatives_and_volatility/deriv-greeks-overview](../07_derivatives_and_volatility/deriv-greeks-overview.md) — instrument-level Greeks derivations (out of scope here; the 11 vertical reuses Greeks as portfolio-aggregated risk factors).

## Escalate to Raw When

The conceptual depth in this card stops at the dichotomy + Taylor / Glivenko-Cantelli accuracy heuristics. When the operator needs the full risk-factor-mapping calculus for non-trivial portfolios (factor selection, factor-PCA, basis-risk handling in cross-currency books, mapping of structured-product payoffs to a tractable factor set, or numerical-stability practices for second-order Greek aggregation), open McNeil Ch.2 §2.3.1 pp.61-64 and Ch.9 §9.1.2 + §9.2 pp.327-345 directly. Instrument-level Greeks derivations remain 07 territory and are explicitly out of scope here. **Source:** McNeil et al. (2015) Ch.2 pp.61-64 + Ch.9 pp.327-345.
