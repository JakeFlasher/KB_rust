---
schema_version: "cacg.v0"
id: "rm-historical-simulation-var"
title: "Historical-Simulation VaR — McNeil Ch.9 §9.2.3-§9.2.4"
reading_id: "11_risk_management"
summary: "Historical-simulation VaR estimates the loss distribution non-parametrically from the empirical distribution of past risk-factor changes applied to the current portfolio, reading the alpha-quantile directly with no explicit distributional assumption; McNeil Ch.9 §9.2.3–§9.2.4."
tags: ["risk-management", "historical-simulation"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p079:0111"
    chunk_hash: "84360624107881a0619d3082e68079dbb39c9e8498a49b128230f9408e5b1685"
    page_range: [79, 80]
    quote: "the historical-simulation method can be thought of as estimating the distribution of the loss using the empirical distribution of past risk-factor changes"
    edge_type: "defines"
card_hash: "d0083ac9890a92c426eb087e850d78daf34526143a1b50997cf4be6f83e61ef8"
---
# Historical-Simulation VaR — McNeil Ch.9 §9.2.3-§9.2.4

## Intuition

**Historical-simulation (HS) VaR** is the assumption-free VaR estimator: take the historical sample of past portfolio losses `{L_t}` over a chosen lookback window, sort them, and read off the empirical α-quantile as `VaR_α`. No distributional family is assumed; no parameters are estimated. The empirical loss CDF `F̂_L(l) = (1/T) · Σ_t 1{L_t ≤ l}` converges to the true CDF by Glivenko-Cantelli, so under the (strong) assumption that the lookback period's loss distribution matches the next period's, HS VaR is consistent. **Source:** McNeil et al. (2015) Ch.9 pp.342-343.

The HS route's central trade-off: **assumption-free comes at a data-hungry cost**. The estimator's variance at high `α` levels is driven by exceedance counts in the tail; with a lookback window of `T` observations and a tail mass of `1 − α`, the expected number of tail exceedances is `(1 − α) · T`. For deep-tail quantiles (`α` close to 1), the expected count drops fast and the empirical quantile becomes dominated by a handful of historical worst losses — small `T` yields a noisy estimate, large `T` reaches back to potentially-irrelevant regimes. Practice typically uses moderate `T` and supplements with parametric or EVT-based tail estimators for the deepest quantiles. **Source:** McNeil et al. (2015) Ch.9 pp.343-344.

The static HS estimator assumes **stationarity** of the loss distribution — a poor assumption during volatility regime shifts. The **dynamic historical simulation** (DHS) extension addresses this by **rescaling** each historical loss by the ratio of current-period volatility to the volatility at the historical period: `L_t^{rescaled} = L_t · (σ̂_{current} / σ̂_t)`. The rescaled losses are then sorted and read off as in vanilla HS. DHS adapts to the current regime without abandoning the assumption-free framing, at the cost of a volatility-estimation overlay (typically EWMA / GARCH). **Source:** McNeil et al. (2015) Ch.9 pp.344-345.

```
   Historical-simulation VaR pipeline
   ──────────────────────────────────

   +---------------------+      +---------------------+
   | Lookback window of  |      | (optional) DHS:     |
   | past portfolio      |      | volatility rescaling|
   | losses {L_1..L_T}   |      | L_t' = L_t·(σ_c/σ_t)|
   +-----------+---------+      +----------+----------+
               |                            |
               +-------------+--------------+
                             |
                             v
                  +-----------------------+
                  | Empirical loss CDF    |
                  | F̂_L(l) = (1/T)·       |
                  |          Σ_t 1{L_t≤l} |
                  +-----------+-----------+
                              |
                              v
                  +-----------------------+
                  | HS VaR:               |
                  | empirical α-quantile  |
                  | of sorted {L_t}       |
                  | = L_(⌈α·T⌉)            |
                  +-----------+-----------+
                              |
                              v
                  caveats:    tail-data hungry (deep α needs many obs);
                              regime shifts (DHS partially fixes);
                              never sees losses worse than historical max
```

## Definition

Let `{L_1, L_2, …, L_T}` be a sample of past one-period portfolio losses over a chosen lookback window. The **empirical loss CDF** is: **Source:** McNeil et al. (2015) Ch.9 pp.342-343.

```
F̂_L(l)  =  (1/T) · Σ_t 1{L_t ≤ l}            (summed over the lookback window)
```

The **historical-simulation VaR** at level `α` is the empirical inf-quantile of the sample: **Source:** McNeil et al. (2015) Ch.9 pp.343.

```
VaR_α^{HS}  =  inf{ l ∈ R : F̂_L(l) ≥ α }  =  L_(⌈α·T⌉)
```

where `L_(1) ≤ L_(2) ≤ … ≤ L_(T)` are the order statistics of the sample and `⌈·⌉` denotes the ceiling. **Source:** McNeil et al. (2015) Ch.9 pp.343.

The **dynamic historical simulation** rescales each historical loss by the ratio of current-period volatility `σ̂_{current}` to the volatility at the historical period `σ̂_t`: **Source:** McNeil et al. (2015) Ch.9 pp.344-345.

```
L_t^{rescaled}  =  L_t · ( σ̂_{current} / σ̂_t )

VaR_α^{DHS}  =  empirical α-quantile of {L_t^{rescaled}}
```

The volatility series `{σ̂_t}` is typically estimated via **EWMA** (exponentially-weighted moving average) or a **GARCH** filter. McNeil treats this overlay at the conceptual level; full GARCH parameter estimation defers to future-01 econometrics. **Source:** McNeil et al. (2015) Ch.9 pp.344-345 + Ch.4 pp.119-150.

## Mathematical Reasoning

The HS estimator's **consistency** rests on Glivenko-Cantelli: as `T → ∞`, the empirical CDF `F̂_L` converges uniformly to the true CDF `F_L` (under stationarity and ergodicity of the loss series). The inf-quantile functional is continuous wherever `F_L` is continuous, so `VaR_α^{HS} → VaR_α` (the true population VaR) almost surely. The estimator's **asymptotic variance** is governed by the Bahadur-Kiefer representation: at smooth `F_L`, `√T · (VaR_α^{HS} − VaR_α) →_d N(0, α(1−α) / f_L(q_α)²)`, where `f_L` is the loss density at the quantile. The variance is **inversely proportional** to the squared density at the quantile — a peaky-mode tail keeps the variance low, while a flat tail (deep `α`, fat-tailed loss) makes it explode. **Source:** McNeil et al. (2015) Ch.9 pp.343 + Ch.5 pp.135-172.

The **data-hungry tail problem** is concrete: at `α` close to 1, the expected exceedance count is `(1−α)·T`. The empirical quantile is determined by the top few observations, and small sample sizes give unstable quantiles. A handful of extreme losses from a single crisis period dominates the estimator — and once those observations roll off the lookback window, the VaR estimate can drop abruptly. McNeil flags this as the **window-edge effect** and recommends complementary parametric / EVT tail estimation for deep quantiles. **Source:** McNeil et al. (2015) Ch.9 pp.343-344.

The **stationarity assumption** is the load-bearing one. Vanilla HS treats every historical observation as equally relevant to the next period's tail — true only if the joint factor distribution is constant over the lookback window. Real factor distributions exhibit **volatility clustering** (calm regimes alternate with crisis regimes) and **structural breaks** (new instruments, regulatory changes, market regime shifts). Vanilla HS under-reacts to a current vol spike (averages over calm history) and over-reacts after a crisis rolls off (loses the worst tail). DHS partially fixes this by rescaling historical losses to current volatility, but the rescaling assumes the **shape** of the loss distribution (skewness, kurtosis, tail thickness) is invariant to the volatility level — a less heroic assumption than full stationarity but still imperfect. **Source:** McNeil et al. (2015) Ch.9 pp.344-345.

A subtle structural property: HS **never produces a VaR worse than the historical maximum loss** in the window (modulo the DHS rescaling). This is by construction — the empirical quantile cannot exceed `L_(T)`, the sample max. For deep-tail levels where the true tail extends past the worst historical observation, HS systematically **under-estimates** the tail. Combining HS with EVT-based POT estimators (peaks-over-threshold) at the deep tail addresses this; the EVT machinery is McNeil Ch.5 and defers to future-01. **Source:** McNeil et al. (2015) Ch.9 pp.343 + Ch.5 pp.155-172.

The HS route is the **most robust** to distributional mis-specification (no distribution assumed) and the **most intuitive** to communicate (the VaR number is "literally the historical α-worst loss"). It is **least efficient** when its stationarity assumption holds (a correctly-specified parametric estimator would have lower variance) and **least informative** in the deep tail (where data is scarce). Risk-management practice reports HS alongside parametric and Monte Carlo VaR (see `[[rm-parametric-var]]` and `[[rm-monte-carlo-var]]`); the three estimators' agreement is a diagnostic for model risk, and their disagreement is a diagnostic for structural assumptions. **Source:** McNeil et al. (2015) Ch.9 pp.340-347.

## See Also

- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — Batch-0 card with VaR / ES definitions.
- [rm-value-at-risk-notes](./rm-value-at-risk-notes.md) — Batch-1 L1-notes framing of the 3-route estimator taxonomy.
- [rm-parametric-var](./rm-parametric-var.md) — Batch-2 sibling card on the parametric variance-covariance route.
- [rm-monte-carlo-var](./rm-monte-carlo-var.md) — Batch-2 sibling card on the model-implied Monte Carlo route.

## Escalate to Raw When

The conceptual depth in this card stops at vanilla HS + DHS volatility rescaling + the data-hungry tail caveats. When the operator needs the full filtered-historical-simulation machinery (Hull-White filtering, bootstrap variants), empirical-quantile bias corrections, EVT-based POT tail estimators for the deep tail (Pickands-Balkema-de Haan theorem, GPD parameter estimation), or formal asymptotic-distribution theory for the empirical quantile under non-stationarity, open McNeil Ch.5 pp.135-172 + Ch.9 §9.2.3-§9.2.4 pp.342-347 directly. **Source:** McNeil et al. (2015) Ch.5 + Ch.9 pp.135-347.
