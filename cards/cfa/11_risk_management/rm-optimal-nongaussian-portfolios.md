---
schema_version: "cacg.v0"
id: "rm-optimal-nongaussian-portfolios"
title: "Optimal Non-Gaussian / Minimum-VaR Portfolios"
reading_id: "11_risk_management"
summary: "Bouchaud & Potters' minimum-loss-probability portfolio under power-law tails: tail amplitudes add as A_p^µ = Σ p_i^µ A_i^µ, so minimizing loss probability minimizes A_p^µ independently of the loss level, with the Markowitz Σ⁻¹ weights and the CAPM β-relation surviving only as the Gaussian µ=2 special case, per Bouchaud & Potters (2003) Ch.12 §12.1-12.2."
tags: ["risk-management", "portfolio-optimization", "fat-tails"]
citations:
  - source_id: "rm_bouchaud_potters_2003_theory_financial_risk"
    chunk_id: "rm_bouchaud_potters_2003_theory_financial_risk:p229:0300"
    chunk_hash: "37593fd278c66d67cc59add9452ceea52dbade5b944932a56e4fbdc6d1e40217"
    page_range: [229, 229]
    quote: "(that generalize the variance) simply add to describe the far-tail of the distribution of the"
    edge_type: "defines"
card_hash: "b9846543e60958fe5feac386b0b182e60c45b26c84da75ceebb0f3f7137291c0"
---
# Optimal Non-Gaussian / Minimum-VaR Portfolios

## Intuition
Markowitz says: diversify by minimizing variance, and the optimal weights fall out of
the inverse covariance matrix. But variance is the *wrong* objective when tails are
genuinely fat — minimizing variance can actually *increase* the value-at-risk.
Bouchaud & Potters ask the sharper question directly: which weights minimize the
*probability of a large loss*? When each asset's loss tail is a power law, the answer
is governed not by the variance but by the asset's **tail amplitude** A_i — the scale
of its extreme losses. Tail amplitudes add under summation the way variances do in the
Gaussian world, so there is a clean "tail-Markowitz" optimization. The remarkable
payoff: the minimum-loss-probability portfolio is the *same* whatever loss level you
care about, and the familiar Markowitz/CAPM machinery re-emerges only as the special
case µ = 2 (the Gaussian), breaking for any truly fat-tailed asset.

```
   return
   ^                        Gaussian frontier: minimize variance σ_p^2
   |                .-''''   (Markowitz; µ=2 only)
   |            .-''
   |        .-''   .-''''    fat-tail frontier: minimize tail amplitude A_p^µ
   |     .-'' .-''            (loss-probability P* ∝ (m_p - m_0)^µ)
   |  .-''.-''
   +-----------------------------> risk  (σ_p   vs   A_p)
```

**Source:** Bouchaud & Potters (2003) Ch.12 §12.1.2 printed pp.206–207 (PDF pp.228–229).

## Definition
Assume each asset's far-loss tail is a power law,
P_T(η_i) ≃ µA_i^µ / |η_i|^{1+µ} as η_i → −∞, with tail index µ > 1 (so the mean
return exists) and **tail amplitude** A_i fixing the order of magnitude of asset i's
extreme losses.

- **Additivity of tail amplitudes.** Because power-law tails are stable under
  addition, p_i X_i has tail amplitude p_i^µ A_i^µ, and the portfolio's tail amplitude
  is A_p^µ = Σ_i p_i^µ A_i^µ — the direct generalization of the variance-additivity
  Σ_i p_i² σ_i².
- **Loss-probability objective.** The probability that the loss exceeds level Λ is
  P = A_p^µ / Λ^µ. Hence, **independently of Λ**, minimizing the loss probability is
  identical to minimizing A_p^µ, and the optimal portfolio does not depend on Λ.
- **Efficient border.** In the return / loss-probability plane the optimal set traces
  a curve P* ∝ (m_p − m_0)^µ, the fat-tail analogue of the Markowitz parabola.

**Source:** Bouchaud & Potters (2003) Ch.12 §12.1.2 printed pp.206–207 (PDF pp.228–229).

## Mathematical Reasoning
Minimizing A_p^µ = Σ_i p_i^µ A_i^µ at fixed mean return m_p with a Lagrange multiplier
ζ gives the first-order condition (valid for µ > 1):

  µ p_i^{*µ−1} A_i^µ = ζ (m_i − m_0),

i.e. the optimal weight of each asset scales with its excess return and inversely with
its tail amplitude, p_i^* ∝ [(m_i − m_0)/A_i^µ]^{1/(µ−1)} in the unconstrained case.
A diversification dividend follows: if all assets share a comparable tail amplitude,
the optimal portfolio's loss probability is smaller than any single asset's by a
factor M^{µ−1} in the number of assets M — but only when µ > 1; for µ < 1 a single
catastrophic asset dominates and "diversification" *raises* risk.

**Gaussian recovery (µ = 2).** Setting µ = 2 collapses the additive tail-amplitude
objective onto the variance, A_p² → Σ p_i p_j C_ij, and the first-order condition
becomes 2 Σ_j C_ij p_j* = ζ(m_i − m_0), inverted as the **Markowitz solution**
p_i* = (ζ/2) Σ_j C_ij^{−1}(m_j − m_0). The efficient border P* ∝ (m_p − m_0)^µ
reduces to the familiar parabola. The **CAPM β-relation** m_i − m_0 = β_i (m_p − m_0)
likewise emerges only because, at µ = 2, all optimal portfolios are proportional to
one another, so a single market portfolio prices every asset. For µ ≠ 2 the optimal
weights generally depend on the chosen risk level Λ, optimal portfolios are *not*
mutually proportional, and the CAPM relation fails.

```
   objective         optimal weights              regime
   ----------------  --------------------------   ------------------
   min A_p^µ         p_i* ∝ [(m_i-m_0)/A_i^µ]^{1/(µ-1)}   fat tails, any µ>1
   min σ_p^2  (µ=2)  p_i* = (ζ/2) Σ_j C_ij^{-1}(m_j-m_0)  Gaussian (Markowitz)
                     m_i - m_0 = β_i (m_p - m_0)           CAPM (µ=2 only)
```

**Source:** Bouchaud & Potters (2003) Ch.12 §12.1.2, §12.2.1 printed pp.207–214 (PDF pp.229–236).

## See Also
- [rm-levy-stable-paretian-tails](./rm-levy-stable-paretian-tails.md) — the power-law tail amplitudes A_i this objective minimizes.
- [rm-parametric-var](./rm-parametric-var.md) — the Gaussian-variance VaR this generalizes away from.
- [rm-value-at-risk-notes](./rm-value-at-risk-notes.md) — the loss-probability / VaR object being minimized here.
- [rm-credit-var-portfolio](./rm-credit-var-portfolio.md) — portfolio-level tail-risk aggregation in a related setting.

## Escalate to Raw When
You need the worked Monte-Carlo vs. approximation VaR comparison table (the linear /
quadratic portfolio numbers), the explicit cleaned-correlation-matrix efficient-frontier
figures, the exponential-asset α-weighting recipe, or the kurtosis-correction weight
formula with its plugged values — those worked numbers and tables live in the raw text
(Rule 1).

**Source:** Bouchaud & Potters (2003) Ch.12 §12.1–12.2 printed pp.206–218 (PDF pp.228–240).
