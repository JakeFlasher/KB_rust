---
schema_version: "cacg.v0"
id: "be-diagnostic-expectations"
title: "Diagnostic Expectations Operator"
reading_id: "10_behavioral_finance"
summary: "The diagnostic-expectations operator: a representativeness-distorted density h^theta(x) proportional to h(x|news) times [h(x|news)/h(x|baseline)]^theta, renormalized; the diagnostic expectation E^theta over-weights states made representative by recent news, theta tunes the distortion, and theta=0 recovers rational expectations."
tags: ["behavioral-finance", "diagnostic-expectations", "representativeness", "overreaction"]
citations:
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p160:0152"
    chunk_hash: "6c37ce0669b4860d1ccd22b18d5a793a9e5afb22f2f62a15f36c254f1457c69e"
    page_range: [161, 161]
    quote: "probability judgments are formed using the representativeness-distorted density"
    edge_type: "defines"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p167:0159"
    chunk_hash: "456c8df2fe40a88d0ab390aa29c913fd0d3990c828acd65b868dd6c05effd6ad"
    page_range: [168, 168]
    quote: "Cash flow realizations that have become more likely in light of news are overweighted, while cash flow realizations that have become less likely are underweighted."
    edge_type: "supports"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p168:0160"
    chunk_hash: "ccf46e712b76b82973faf98404bdbcbcfb87282394232c259722da10c95ed414"
    page_range: [169, 169]
    quote: "When θ = 0, diagnostic beliefs coincide with rational expectations"
    edge_type: "supports"
card_hash: "8e14650473a8f8171a23c0f43c119d6478822f364c7da0b55acf36811c2fdb6f"
---
# Diagnostic Expectations Operator

## Intuition

Diagnostic expectations turn the representativeness heuristic into a single tunable operator on probability distributions. The idea: after observing news, an agent easily recalls the outcomes that the news has made much more likely than they were before, those representative outcomes come quickly to mind and are over-weighted in judgment, while outcomes that were also likely under prior conditions face memory interference and are under-weighted. Favorable housing news, for example, both makes high mortgage cash flows objectively more likely *and* makes them more representative -- and that second, psychological effect boosts beliefs even further, producing overreaction to news.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.147-148, 154.

The operator is forward-looking: beliefs respond to regime changes and news about the future, so unlike mechanical adaptive rules it is not vulnerable to the Lucas critique. It inherits the kernel-of-truth property -- distortions exaggerate real, news-driven differences -- and it nests rational expectations as the special case `theta = 0`. A single parameter `theta >= 0` indexes how far judgment departs from Bayes, which lets one read off neglect of tail risk, extrapolation, and overreaction as outputs of the same machine rather than as separate ad hoc biases.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.140-141, 148.

## Definition

**Diagnostic density** `h^theta(T = tau | G)` is the representativeness-distorted probability distribution agents actually use: it multiplies the true conditional density by a power of the representativeness ratio and renormalizes to integrate to 1.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.148.

**Diagnostic expectation** `E^theta_t[x]` is the expectation taken under the diagnostic density `h^theta` (equivalently `f^theta`) rather than under the true conditional density; it over-weights states made representative by recent news.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.154-155.

**Distortion parameter** `theta >= 0` measures the extent of probability distortion: larger `theta` inflates the probability of highly representative states and discounts unrepresentative ones; `theta = 0` gives undistorted (rational) beliefs.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.148, 155.

## Mathematical Reasoning

The core operator (BCGS 2016a, BGS 2018) forms judgments from the representativeness-distorted density

```
  h^theta(T = tau | G) = h(T = tau | G) * [ h(T = tau | G) / h(T = tau | -G) ]^theta * Z,
```

where the bracket is the representativeness ratio `R(tau, G)`, `theta >= 0` controls distortion, and `Z` is the normalizing constant ensuring `h^theta` integrates to 1. The decision maker inflates the probability of highly representative types (`R > 1`) and discounts unrepresentative ones (`R < 1`).
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.148.

In the finance setting the object of inference is a future cash flow `X`, the target group is the news set `G = I_0`, and the comparison group is the previous (baseline) period `-G = I_{-1}`. The distorted cash-flow density is

```
  f^theta(X | I_0) = f(X | I_0) * [ f(X | I_0) / f(X | I_{-1}) ]^theta * Z.
```

Realizations that have become more likely in light of news are overweighted; realizations that have become less likely are underweighted, even if they remain likely in absolute terms.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.154.

With lognormal cash flows the operator has a closed form (Proposition 5.1): if `ln X | I_0 ~ N(mu_0, sigma_0^2)` and `ln X | I_{-1} ~ N(mu_{-1}, sigma_{-1}^2)`, the diagnostic density is again lognormal with distorted mean and variance,

```
  mu_0(theta) = mu_0 + theta*sigma_0^2 / [ sigma_{-1}^2 + theta(sigma_{-1}^2 - sigma_0^2) ] * (mu_0 - mu_{-1}),
  sigma_0^2(theta) = sigma_0^2 * sigma_{-1}^2 / [ sigma_{-1}^2 + theta(sigma_{-1}^2 - sigma_0^2) ].
```

When `theta = 0` these collapse to `mu_0(0) = mu_0` and `sigma_0^2(0) = sigma_0^2` -- rational expectations. When `theta > 0`, the mean is inflated above the truth (`mu_0(theta) > mu_0`) if and only if there is genuine good news, `mu_0 > mu_{-1}`; the diagnostic distribution shifts right and develops a thinner left tail.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.155.

```
                  baseline        true after-news      diagnostic
   density        N(mu_{-1},.)    N(mu_0,.)            N(mu_0(theta),.)
       .              /\              /\                    /\
      / \            /  \            /  \                  /  \
   __/   \____  ____/    \____  ____/    \____  ______/      \____
        mu_{-1}          mu_0                   mu_0(theta)  --> ln X
   good news shifts the truth right; diagnostic beliefs over-shoot it.
```

**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.158 (Figure 5.1).

## See Also

- [be-representativeness-conjunction-base-rate](./be-representativeness-conjunction-base-rate.md#mathematical-reasoning) -- the heuristic and likelihood-ratio `R(tau,G)` the operator is built on.
- [be-kernel-of-truth](./be-kernel-of-truth.md#mathematical-reasoning) -- why `mu_0(theta) > mu_0 iff mu_0 > mu_{-1}` keeps the distortion anchored to real news.
- [be-rational-vs-diagnostic-expectations](./be-rational-vs-diagnostic-expectations.md#intuition) -- the `theta=0` nesting and predictable forecast errors.
- [be-two-model-mispricing](./be-two-model-mispricing.md#intuition) -- the representativeness/overreaction lineage (Barberis-Shleifer-Vishny) this operator descends from.
- [be-neglected-tail-risk](./be-neglected-tail-risk.md#intuition) -- the thin-left-tail consequence applied to safe-debt issuance.

## Escalate to Raw When

- You need the proof of Proposition 5.1 or the exact regularity condition `(1+theta)sigma_{-1}^2 - theta*sigma_0^2 > 0` (pp.155, Appendix).
- You need the dynamic AR(1) form of the operator, `E^theta_t(X_{t+1}) = rho*X_t + rho*theta*(X_t - rho*X_{t-1})` (Proposition 6.1, pp.172).
- You need the memory / fan-effect microfoundation linking `R(.)` to selective recall (pp.147-148).
