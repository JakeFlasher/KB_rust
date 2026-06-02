---
schema_version: "cacg.v0"
id: "rm-bernoulli-poisson-mixture-credit"
title: "Bernoulli and Poisson Mixture Credit Models (incl. CreditRisk+)"
reading_id: "11_risk_management"
summary: "Mixture credit models make defaults conditionally independent given common factors Psi with conditional PDs p_i(Psi); one-factor Bernoulli mixtures generate default-rate contagion, the Poisson-mixture limit yields CreditRisk+, and most threshold models (those with a conditional-independence structure) can be written as Bernoulli mixtures, per McNeil et al. (2015) Ch.11 §11.2."
tags: ["risk-management", "credit-risk", "mixture-models"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p457:0662"
    chunk_hash: "d102587c39bcbdb9783a468d5bf2ea61d6cec35ee4211567044ba27c74e9e82a"
    page_range: [457, 457]
    quote: "Dependence between defaults stems from the dependence of individual default probabilities on the set of common factors"
    edge_type: "defines"
card_hash: "1b1b60ff30c83e3a7e146773e33a387a3b80ca65d0ddf98e0dbba01ffdb87518"
---
# Bernoulli and Poisson Mixture Credit Models (incl. CreditRisk+)

## Intuition
Mixture credit models take a complementary vantage point to threshold models; for a
broad class (made precise in Mathematical Reasoning) the two views coincide. The story:
there is a common economic factor Ψ
(a macro state of the world); *conditional on Ψ*, every obligor defaults
independently, each with its own conditional probability p_i(Ψ). All the
co-movement of defaults — the clustering that produces fat portfolio-loss tails — is
manufactured by the *shared* dependence on Ψ. When the economy is bad (Ψ adverse),
every conditional default probability rises together, so defaults arrive in waves.
This "conditional-independence given a common factor" structure is exactly what makes
the models computationally tractable and is the basis of CreditRisk+.

```
              common factor  Ψ  (macro state)
                  /     |      \
                 v      v       v
       p_1(Ψ)   p_2(Ψ)  ...  p_m(Ψ)     conditional default probs
         |        |            |
        Y_1      Y_2   ...    Y_m        independent GIVEN Ψ
                                         (dependence is all in Ψ)
```

**Source:** McNeil et al. (2015) Ch.11 §11.2 printed pp.436 (PDF p.457).

## Definition
**Bernoulli mixture model (Def. 11.5).** With a p-dimensional factor vector Ψ and
functions p_i : R^p → [0,1], the default indicators Y = (Y_1,…,Y_m) follow a
Bernoulli mixture if, conditional on Ψ = ψ, the Y_i are *independent* Bernoulli with
P(Y_i = 1 | Ψ = ψ) = p_i(ψ). The unconditional law is obtained by integrating over
Ψ; the marginal default probability is p_i = E(p_i(Ψ)).

**One-factor / exchangeable** version: identical p_i ≡ p give an exchangeable model
where the random mixing variable Q = p(Ψ) is the stochastic default rate; var(Q) > 0
is precisely what creates positive default correlation (default-rate "contagion").

**Poisson mixture model (Def. 11.14).** Conditional on Ψ, the counts Ỹ_i are
independent Poisson with rate λ_i(ψ). Setting Y_i = 1{Ỹ_i ≥ 1} recovers a Bernoulli
mixture with p_i = 1 − e^{−λ_i}. **CreditRisk+** is the Poisson mixture with
independent gamma-distributed factors.

**Source:** McNeil et al. (2015) Ch.11 §11.2.1, §11.2.5 printed pp.436, 444 (PDF pp.457, 465).

## Mathematical Reasoning
Because defaults are conditionally independent, the conditional joint law factorizes,
P(Y = y | Ψ = ψ) = Π_i p_i(ψ)^{y_i}(1 − p_i(ψ))^{1−y_i}, and unconditional
quantities follow by mixing over Ψ. Default correlation is therefore generated *only*
through the spread of the conditional PDs across factor states.

The two families connect in one direction: by Lemma 11.10, any **threshold model whose
critical variables have a conditional-independence structure with conditioning variable Ψ
produces default indicators that follow a Bernoulli mixture** with
p_i(ψ) = P(X_i ≤ d_i | Ψ = ψ). McNeil notes that the majority of useful threshold models —
including all of his examples — admit such a representation, so a Gaussian one-factor
threshold model can be written as a Bernoulli mixture. (McNeil establishes this
direction; he does not assert the converse on these pages, so the two are not claimed to
be fully interchangeable.)

The Poisson step is an asymptotic/rare-event approximation: when p_i(ψ) is small,
P(Ỹ_i = 0 | ψ) = e^{−p_i(ψ)} ≈ 1 − p_i(ψ), so Bernoulli indicators are well
approximated by conditionally independent Poisson counts, at the cost of allowing a
(negligible-probability) "multiple default". Choosing gamma factors yields the
closed-form CreditRisk+ loss distribution.

**Source:** McNeil et al. (2015) Ch.11 §11.2.4–11.2.5 printed pp.442–445 (PDF pp.463–465).

## See Also
- [rm-threshold-credit-models](./rm-threshold-credit-models.md) — the dual representation via latent variables and copulas.
- [rm-credit-var-portfolio](./rm-credit-var-portfolio.md) — the portfolio loss distribution these models parameterize.
- [rm-copulas-sklar-dependence](./rm-copulas-sklar-dependence.md) — the copula view the threshold↔mixture equivalence rests on.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — anatomy of the loss tail mixture variance drives.

## Escalate to Raw When
You need the gamma-factor CreditRisk+ recursion, the simulated default-count
quantiles (q_0.95(M), q_0.99(M)) tabulated against asset correlation, or the
probit/logit-normal mixing-distribution calibrations — those worked recursions and
numeric tables live in the raw text (Rule 1).

**Source:** McNeil et al. (2015) Ch.11 §11.2 printed pp.436–448 (PDF pp.457–469).
