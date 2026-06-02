---
schema_version: "cacg.v0"
id: "rm-threshold-credit-models"
title: "Threshold (Latent-Variable) Credit Models and Their Copula Structure"
reading_id: "11_risk_management"
summary: "In a threshold credit model default occurs when a latent critical variable falls below a threshold; joint default depends only on the copula of the critical variables, so CreditMetrics and KMV/Merton are exactly Gaussian threshold models with a Gauss copula, per McNeil et al. (2015) Ch.11 §11.1."
tags: ["risk-management", "credit-risk", "copulas"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p449:0650"
    chunk_hash: "223b5f8b5bdfaa82686e2bee61d8eeb90eb455204871da36b4fa890dfe42a2d3"
    page_range: [450, 450]
    quote: "result clarifies the central role of copulas in threshold models"
    edge_type: "defines"
card_hash: "4d4578bff1a91b47b9521e536a249c7c264bd9226f3229833b59d544f2b1fc4c"
---
# Threshold (Latent-Variable) Credit Models and Their Copula Structure

## Intuition
A threshold credit model says a firm defaults when some unobserved "critical
variable" — read as a latent asset value or creditworthiness index — drops below a
critical level. This is the structural picture behind the most widely used industry
models: Merton's firm-value default, KMV, and CreditMetrics all reduce to one firm
crossing one threshold. The deep point of this card is that *the marginal default
probabilities and the dependence between defaults factor cleanly apart*: once you fix
each firm's default probability, the entire joint-default behaviour is governed by
the **copula** of the critical variables, nothing else. That is why these models are
"the same model" — they differ only in their copula, and the standard ones all use
the Gauss copula.

```
   critical variable X_i (latent asset value)
        |
   X_i  o-----o-----o------------> value
            ^  threshold d_i
        default  iff  X_i <= d_i      ( P(default) = F_{X_i}(d_i) )

   joint default of i and j  <===  copula C of (X_i, X_j)  only
```

**Source:** McNeil et al. (2015) Ch.11 §11.1.2 printed pp.428–429 (PDF pp.449–450).

## Definition
**Threshold model (Def. 11.1).** Let X = (X_1,…,X_m) be critical variables and D a
matrix of deterministic, increasing thresholds. The state is R_i = j iff
d_{ij} < X_i ≤ d_{i(j+1)}. Default is the event R_i = 0, i.e. X_i ≤ d_{i1}, so the
default probability is p_i = F_{X_i}(d_{i1}). A default-only model is written (X, d).

**Equivalence via copula (Lemma 11.2).** Two threshold models are equivalent if
(i) the marginal state distributions coincide and (ii) X and X̃ share the same copula
C. So a threshold model is fully pinned down by marginal default/migration
probabilities plus the copula of the critical variables.

- **Multivariate Merton / KMV:** firm values V_T are multivariate normal; default is
  V_{i,T} below a debt barrier — a Gaussian threshold model.
- **CreditMetrics:** standardized critical variables X ∼ N_m(0, P) with thresholds
  set from rating-migration probabilities — also a Gauss-copula threshold model.

**Source:** McNeil et al. (2015) Ch.11 §11.1.2–11.1.3 printed pp.428–431 (PDF pp.449–451).

## Mathematical Reasoning
Default correlation is E(Y_iY_j) = P(X_i ≤ d_{i1}, X_j ≤ d_{j1}), which depends on
the *joint* law of (X_i, X_j). Apply Sklar's theorem: the joint exceedance
probability equals C(F_{X_1}(d_{1j_1}),…), so two models with identical marginal
state distributions and the same copula C produce identical state vectors. Hence the
copula of the critical variables determines the link between marginal default
probabilities and joint defaults.

A subtle consequence: for multivariate-normal X, the *correlation* of critical
variables fixes the (Gauss) copula and therefore the default correlation — but
**outside the normal class, asset correlation does not fully determine default
correlation**, and two models matched on correlation can have very different joint
tails of the default count M = Σ Y_i. Because the default event is invariant under
strictly increasing transforms of critical variables and thresholds, the latent scale
is irrelevant; only the copula survives. Replacing the Gauss copula with a t copula
(same correlations) injects tail dependence and fattens the loss tail.

**Source:** McNeil et al. (2015) Ch.11 §11.1.2 printed pp.428–429 (PDF pp.449–450).

## See Also
- [rm-copulas-sklar-dependence](./rm-copulas-sklar-dependence.md) — Sklar's theorem, the engine behind the equivalence.
- [rm-tail-dependence-coefficients](./rm-tail-dependence-coefficients.md) — why a t copula fattens the default-count tail.
- [rm-bernoulli-poisson-mixture-credit](./rm-bernoulli-poisson-mixture-credit.md) — the mixture-model dual of threshold models.
- [rm-credit-var-portfolio](./rm-credit-var-portfolio.md) — the portfolio credit-VaR these models feed.

## Escalate to Raw When
You need the calibrated equicorrelation/asset-correlation parameters, the Clayton-θ
that targets a given joint default probability, or the simulated loss-distribution
tables comparing Gauss vs t critical variables — those worked calibrations and
numeric experiments live in the raw text (Rule 1).

**Source:** McNeil et al. (2015) Ch.11 §11.1 printed pp.428–442 (PDF pp.449–463).
