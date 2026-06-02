---
schema_version: "cacg.v0"
id: "rm-monte-carlo-var"
title: "Monte Carlo VaR — McNeil Ch.9 §9.2.5-§9.2.7"
reading_id: "11_risk_management"
summary: "Monte Carlo VaR is the general simulation route: draw many risk-factor scenarios from an explicit calibrated parametric model, full-revalue the portfolio per scenario, and read the alpha-quantile from the simulated loss sample; McNeil Ch.9 §9.2.5 emphasises the cost of per-scenario revaluation."
tags: ["risk-management", "monte-carlo"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p367:0523"
    chunk_hash: "7be9a5b5d7c55435f7515af053e85714f8ab30dc1a9d3ba98d5aefd9bf605ca5"
    page_range: [367, 367]
    quote: "that involves the simulation of an explicit parametric model for risk-factor changes"
    edge_type: "defines"
card_hash: "a41742a86f986ecd711961893cf98ce33bfd0b2060add7719332ce933a41fbcf"
---
# Monte Carlo VaR — McNeil Ch.9 §9.2.5-§9.2.7

## Intuition

**Monte Carlo (MC) VaR** sits between parametric and historical-simulation routes. Like parametric, it commits to a joint factor distribution; unlike parametric, it does not require closed-form quantile evaluation. Like historical simulation, it reads risk off an empirical quantile; unlike HS, the sample is generated synthetically from a model rather than read from history. The pipeline: (a) calibrate a joint factor-model `X ~ F_X(·; θ)` from history; (b) simulate `N` scenarios `X^{(k)}`; (c) full-revalue the portfolio at each scenario to get loss samples `L^{(k)} = −(V(X^{(k)}) − V(X_0))`; (d) read `VaR_α` as the empirical α-quantile of `{L^{(k)}}`. **Source:** McNeil et al. (2015) Ch.9 pp.346-347.

The MC route's strength is **freedom from closed-form constraints**: any factor distribution can be sampled (heavy-tailed, asymmetric, regime-switching), any portfolio non-linearity is captured by full revaluation, and any path-dependent payoff (American options, barrier products, callable bonds) is priced exactly per scenario. The price is **computational cost**: full revaluation of a complex book over a large scenario count `N` costs `N · cost(V)`, which for instrument books with American features can be prohibitive. The **delta-gamma Monte Carlo hybrid** trades exactness for speed by revaluing the second-order Taylor expansion `Δ·ΔX + 1/2·ΔX^T·H·ΔX` per scenario instead of full `V` — this recovers most of the simulation's distributional sensitivity at a fraction of the cost (see `[[rm-sensitivity-versus-simulation]]`). **Source:** McNeil et al. (2015) Ch.9 pp.346-349.

A structural design choice is the **factor model**: the dimensionality `d` of `X`, the family of `F_X` (multivariate normal, Student-t, copula-coupled marginals), the calibration window, and the conditioning mechanism (unconditional draws vs conditional on current factor levels). McNeil recommends **dimension reduction** via principal-components for high-dimensional factor sets; the leading PCs capture the bulk of variance and the simulation runs on a low-dimensional surrogate. The trade-off: PC-truncation loses tail-relevant information in the dropped dimensions. **Source:** McNeil et al. (2015) Ch.9 pp.347-348.

```
   Monte Carlo VaR pipeline
   ────────────────────────

   +-------------------+      +-------------------+      +-------------------+
   | Joint factor      |      | Number of         |      | Portfolio value   |
   | model F_X(·; θ)   |      | scenarios N       |      | function V(X)     |
   | (calibrated from  |      | (variance-cost    |      | (full-revaluation |
   |  history)         |      |  trade-off)       |      |  or delta-gamma)  |
   +---------+---------+      +---------+---------+      +---------+---------+
             |                          |                          |
             +-----------+--------------+--------------+-----------+
                         |                             |
                         v                             v
              +----------------------+      +----------------------+
              | Simulate scenarios   |      | Per-scenario         |
              | X^(k) ~ F_X(·; θ)    | ---> | revaluation:         |
              | for k = 1..N         |      | L^(k) = −(V(X^(k))   |
              |                      |      |          − V(X_0))   |
              +----------+-----------+      +----------+-----------+
                                                       |
                                                       v
                                          +----------------------+
                                          | MC VaR:              |
                                          | empirical α-quantile |
                                          | of {L^(k)}           |
                                          +----------+-----------+
                                                     |
                                                     v
              caveats:  factor-model risk (assumed F_X may be wrong);
                        full-revaluation cost (delta-gamma MC if cost binds);
                        variance ~ 1/N (convergence O(1/√N))
```

## Definition

Let `X ∈ R^d` be the risk-factor vector and `F_X(·; θ)` a joint factor-distribution model calibrated to historical data. Let `V(X)` be the portfolio value function and `X_0` the current factor vector. **Monte Carlo VaR** at level `α` from `N` simulated scenarios is: **Source:** McNeil et al. (2015) Ch.9 pp.346.

```
Step 1:  draw N samples X^(k) ~ F_X(·; θ)            for k = 1, …, N
Step 2:  revalue:  L^(k)  =  −( V(X^(k)) − V(X_0) )
Step 3:  sort:     L_(1)  ≤  L_(2)  ≤  …  ≤  L_(N)
Step 4:  read:     VaR_α^{MC}  =  L_(⌈α·N⌉)
```

The **delta-gamma Monte Carlo hybrid** replaces step 2's full revaluation with the second-order Taylor expansion: **Source:** McNeil et al. (2015) Ch.9 pp.346-347.

```
L^(k)  ≈  −( a^T ΔX^(k)  +  1/2 · (ΔX^(k))^T · H · ΔX^(k) )

where ΔX^(k) = X^(k) − X_0,  a = ∇V(X_0),  H = Hessian of V at X_0
```

The hybrid costs `N · cost(quadratic form)` instead of `N · cost(V)`, dropping the per-scenario cost by the cost ratio between a quadratic-form evaluation and a full instrument repricer (the ratio is implementation-dependent but typically large for non-trivial option books). The penalty is loss of payoff non-linearity beyond second order. **Source:** McNeil et al. (2015) Ch.9 pp.347-349.

The **convergence rate** of the empirical-quantile estimator under MC sampling is `O(1/√N)` by the Bahadur-Kiefer representation: **Source:** McNeil et al. (2015) Ch.9 pp.346-347.

```
√N · (VaR_α^{MC} − VaR_α)  →_d  N(0, α(1−α) / f_L(q_α)²)        (as N → ∞)
```

so increasing the scenario count `N` shrinks the standard error in proportion to `1/√N`. **Variance-reduction techniques** (importance sampling, control variates, stratified sampling) can accelerate convergence considerably for the deep-tail quantile case. **Source:** McNeil et al. (2015) Ch.9 pp.347-348.

## Mathematical Reasoning

The MC estimator's **consistency** holds under the same Glivenko-Cantelli argument as the HS estimator, with the simulated empirical CDF converging to the true model-implied CDF as `N → ∞`. The key distinction: HS's empirical CDF converges to the **historical** loss CDF (subject to stationarity); MC's empirical CDF converges to the **model-implied** loss CDF (subject to correct factor-model specification). MC trades historical-window-edge risk for factor-model risk. **Source:** McNeil et al. (2015) Ch.9 pp.346-347.

The **factor-model risk** is the dominant error source. Choosing `F_X` requires committing to a distributional family (jointly normal, elliptical Student-t, copula-coupled marginals, mixture models), and the wrong choice biases every simulated tail. Common practice combines multiple `F_X` candidates and reports MC VaR under each as a **model-risk band**; persistent disagreement across families is a diagnostic for model risk. The factor-model decision is upstream of the simulation and is the load-bearing call. **Source:** McNeil et al. (2015) Ch.9 pp.346-347.

The **variance-reduction trade-off** matters most at deep tail levels. Plain MC at `α` close to 1 wastes most scenarios on the body of the distribution where they provide no information about the quantile — the expected number of tail observations is `(1 − α) · N`, which is small even for large `N`. **Importance sampling** corrects this by drawing scenarios from a tilted distribution that emphasises the tail, then re-weighting per scenario to recover the original-distribution quantile. **Control variates** use a correlated random variable with known mean to subtract systematic noise. **Stratified sampling** divides the factor space into strata and allocates more scenarios to high-loss strata. McNeil treats importance sampling at the conceptual level in Ch.9 §9.2.6 and recommends it for deep-tail MC; full implementation depth defers to Ch.8 plus future-01 quantitative methods. **Source:** McNeil et al. (2015) Ch.9 pp.347-348.

The **multi-period scaling** of MC VaR is a structural advantage over the static estimators. For a `Δt`-horizon factor model, MC simulates entire path-realisations `{X(t)}_{0 ≤ t ≤ Δt}` and prices `V` at the horizon endpoint, capturing path-dependent payoffs (barrier knock-outs, American exercise, callable bonds). Vanilla HS and parametric methods scale single-period loss by `√Δt` (a Gaussian-only assumption) and miss the path-dependence entirely. The cost: simulating intermediate path points raises per-scenario cost from `cost(V)` to `cost(V) × path-length`. **Source:** McNeil et al. (2015) Ch.9 pp.348-349.

The boundary with 07 Derivatives is precise: **07 owns instrument-pricing Monte Carlo** (single-name option pricing, exotic derivatives, MC Greeks via likelihood-ratio / pathwise methods). **11 owns portfolio-risk Monte Carlo** (joint-factor scenarios → portfolio loss distribution → VaR/ES quantile). The same underlying simulation engine is reused; the framing differs by what's being estimated (instrument value vs portfolio risk measure). **Source:** McNeil et al. (2015) Ch.9 pp.346-349.

## See Also

- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — Batch-0 card with VaR / ES definitions.
- [rm-value-at-risk-notes](./rm-value-at-risk-notes.md) — Batch-1 L1-notes framing of the 3-route estimator taxonomy.
- [rm-parametric-var](./rm-parametric-var.md) — Batch-2 sibling card on the closed-form variance-covariance route.
- [rm-historical-simulation-var](./rm-historical-simulation-var.md) — Batch-2 sibling card on the empirical-quantile-from-history route.
- [rm-sensitivity-versus-simulation](./rm-sensitivity-versus-simulation.md) — Batch-0 card framing the sensitivity-vs-simulation dichotomy that the delta-gamma MC hybrid sits between.
- [../07_derivatives_and_volatility/deriv-monte-carlo-pricing](../07_derivatives_and_volatility/deriv-monte-carlo-pricing.md) — 07 vertical's instrument-pricing Monte Carlo (07 territory).

## Escalate to Raw When

The conceptual depth in this card stops at the basic MC pipeline + delta-gamma MC hybrid + the convergence-rate / variance-reduction overview. When the operator needs the full simulation machinery (random-number generation theory, low-discrepancy / quasi-MC sequences, importance-sampling tilting choice, control-variate construction, copula sampling for non-elliptical joint factor models, or MC Greek estimation via likelihood-ratio / pathwise methods), open McNeil Ch.9 §9.2.5-§9.2.7 pp.346-350 directly. Instrument-level Monte Carlo pricing belongs to 07. **Source:** McNeil et al. (2015) Ch.9 pp.346-350.
