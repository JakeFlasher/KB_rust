---
schema_version: "cacg.v0"
id: "ec-risk-aversion-utility-derivation"
title: "Risk Aversion and Utility Derivation"
reading_id: "02_economics"
summary: "Risk aversion is equivalent to concavity of the vNM Bernoulli utility u (via Jensen's inequality); the Arrow-Pratt coefficient of absolute risk aversion r_A(x) = -u''(x)/u'(x) is the local curvature measure such that for small fair gambles the risk premium pi approx (1/2)*sigma^2*r_A(x); relative risk aversion R(w) = w*A(w) underpins CRRA tractability used downstream in 09 SDF asset pricing."
tags: ["economics", "risk-aversion"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p204:0334"
    chunk_hash: "403ed71dcf99fc1214395365f16e6306d8f16bf4d81b5746b98b598fed869071"
    page_range: [204, 205]
    quote: "A decision maker is a risk averter (or exhibits risk aversion) if for any lottery F( ·),the degenerate lottery that yields the amount J x dF(x) with certainty is at least as good as the lottery F"
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p206:0337"
    chunk_hash: "cf36e591c0e1c5d5e108bee9b6844f27d81b7d5d810fb3c1cb2693e55fcd52cc"
    page_range: [206, 206]
    quote: "Suppose a decision maker is an expected uti l ity maximizer with a Bernoulli uti l ity function u( ·) on amounts of money."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3364:5066"
    chunk_hash: "67759a7504764dd87acb5cfd6dac954b9b7863c03ae052a5fb341650c39099c7"
    page_range: [3364, 3365]
    quote: "In traditional finance theory, the individual focuses on maximizing unobservable utility, whereas the business maximizes a generally observable value"
    edge_type: "supports"
card_hash: "1c7bf89e4b4de1c156c14645fe5d715473f3d2a55860c68474b5a7f6f984fcd9"
---
# Risk Aversion and Utility Derivation

## Intuition

An expected-utility agent with strictly concave Bernoulli utility `u` is risk averse: she prefers the certain expected value of any gamble to the gamble itself. The strength of risk aversion at wealth level `w` is captured by the local curvature of `u` — the more negative `u''(w)` relative to the slope `u'(w)`, the larger the risk premium the agent demands to accept a fair gamble. The Arrow-Pratt coefficient of absolute risk aversion `A(w) = − u''(w) / u'(w)` is the standard local measure; multiplying by `w` gives the coefficient of relative risk aversion `R(w) = w · A(w)`. **Source:** Mas-Colell et al. (1995) Ch.6 pp.183-194.

This is a bridge card per resolved DEC-4: the risk-aversion machinery is the microeconomic input that downstream asset-pricing models (SDF in 09; CAPM in 09; pricing kernels) consume to translate expected utility of wealth into pricing equations. The pricing equations themselves stay in 09 per AC-11 BOUNDARY-DISCIPLINE; this card stops at deriving the Arrow-Pratt coefficients and the small-gamble risk-premium approximation. **Source:** Mas-Colell et al. (1995) Ch.6 pp.183-194.

```
   u(w)
   ^
   |                              .  .  .  .  u(E[W])   <-- u at expected wealth
   |                           .              ────────
   |                         /                E[u(W)]   <-- expected utility
   |                        /
   |                      /     Jensen gap = π · u'(w)
   |                    /       (risk premium)
   |                  /
   |                /
   |              /
   |            /          u concave  ⇒  E[u(W)] ≤ u(E[W])
   |          /            agent is risk averse
   |        /
   +------+--------+---------------+----------------> wealth W
          c(F,u)    E[W]
          (cert. eq.)
   π = E[W] − c(F,u);  for small gambles: π ≈ (1/2) · σ² · A(w)
```

## Definition

An expected-utility agent with Bernoulli utility `u: R → R` is **risk averse at wealth `w`** if for any random variable `Z` with `E[Z] = 0`,. **Source:** Mas-Colell et al. (1995) pp.183-202.

```
u(w) ≥ E[u(w + Z)]
```

Equivalently (by Jensen's inequality), the agent is risk averse iff `u` is concave. Strict risk aversion corresponds to strict concavity. **Source:** Mas-Colell et al. (1995) Ch.6 pp.183-187.

The **certainty equivalent** `c(F, u)` of a lottery with distribution `F` for an agent with utility `u` is the certain amount that makes the agent indifferent. **Source:** Mas-Colell et al. (1995) pp.183-202.

```
u(c(F, u)) = E_F[u(W)]
```

The **risk premium** `π(F, u)` is the excess of expected wealth over the certainty equivalent. **Source:** Mas-Colell et al. (1995) pp.183-202.

```
π(F, u) = E_F[W] − c(F, u)
```

The **Arrow-Pratt coefficients** at wealth `w` are: **Source:** Mas-Colell et al. (1995) pp.183-202.

```
absolute risk aversion:  A(w) = − u''(w) / u'(w)
relative risk aversion:  R(w) = − w · u''(w) / u'(w) = w · A(w)
```

For a small gamble `Z` with `E[Z] = 0` and `var(Z) = σ²`, the second-order Taylor expansion of the certainty-equivalent equation gives the **risk-premium approximation**: **Source:** Mas-Colell et al. (1995) pp.183-202.

```
π ≈ (1/2) · σ² · A(w)
```

**Source:** Mas-Colell et al. (1995) Ch.6 pp.187-194.

## Mathematical Reasoning

Concavity of `u` implies risk aversion via Jensen's inequality: for any concave `u` and any random variable `W`, `u(E[W]) ≥ E[u(W)]`. The reverse direction holds because if `u` is not concave anywhere, then there is a small fair gamble around some `w_0` that the agent strictly prefers — contradicting risk aversion. The equivalence makes concavity the structural characterization of risk-averse vNM preferences. **Source:** Mas-Colell et al. (1995) Ch.6 pp.183-188.

The Arrow-Pratt risk-premium approximation derives from a second-order expansion of both sides of `u(w − π) = E[u(w + Z)]` around `w`. Left side: `u(w) − π · u'(w) + O(π²)`. Right side: `u(w) + E[Z] · u'(w) + (1/2) · E[Z²] · u''(w) + O(E[|Z|³])`. With `E[Z] = 0` and `E[Z²] = σ²`, equating and dropping higher-order terms gives `π · u'(w) ≈ −(1/2) · σ² · u''(w)`, hence `π ≈ (1/2) · σ² · (−u''(w) / u'(w)) = (1/2) · σ² · A(w)`. The Arrow-Pratt coefficient `A(w)` is exactly the rate at which risk-premium scales with variance for small gambles. **Source:** Mas-Colell et al. (1995) Ch.6 pp.188-192.

Two canonical Bernoulli utilities exhibit constant-coefficient behavior. **CARA (constant absolute risk aversion)**: `u(w) = −exp(−α w)` gives `A(w) = α` for all `w`; equivalently the certainty equivalent of a gamble `w + Z` is `w + c(Z)` for some constant shift `c(Z)` depending only on the distribution of `Z`, not on `w`. **CRRA (constant relative risk aversion)**: `u(w) = w^(1−γ) / (1−γ)` for `γ ≠ 1` (and `u(w) = log(w)` for `γ = 1`) gives `R(w) = γ` for all `w`; equivalently the certainty-equivalent ratio `c(F, u) / E_F[W]` depends only on the scale-free distribution of `W`. Asset-pricing models routinely assume CRRA because it makes the SDF equation independent of the wealth level — a critical tractability property exploited in 09 `pm-stochastic-discount-factor-intuition.md`. **Source:** Mas-Colell et al. (1995) Ch.6 pp.193-200.

## See Also

- [`ec-utility-and-choice-under-uncertainty`](./ec-utility-and-choice-under-uncertainty.md) — vNM expected-utility theorem; this card requires its Bernoulli `u`
- [`ec-consumer-preference-and-choice`](./ec-consumer-preference-and-choice.md) — rationality axioms underwriting `u`'s existence
- [`pm-stochastic-discount-factor-intuition`](../09_portfolio_management_and_asset_pricing/pm-stochastic-discount-factor-intuition.md) — SDF asset-pricing equation; uses CRRA Bernoulli utility as its canonical input
- [`pm-capm-and-sml`](../09_portfolio_management_and_asset_pricing/pm-capm-and-sml.md) — CAPM derivation; assumes mean-variance preferences (a special case of expected-utility with quadratic `u` or normal returns)

## Escalate to Raw When

The full comparative-statics theorems for absolute and relative risk aversion (Pratt's theorem on the equivalence of larger Arrow-Pratt coefficients with higher risk premia at every wealth level) are in MWG Ch.6 pp.190-194; re-open if a question requires the proof of equivalence rather than the local approximation alone. The Rothschild-Stiglitz framework for partial-ordering of risky distributions (mean-preserving spreads, second-order stochastic dominance) is in MWG Ch.6 pp.194-202. Empirical estimates of the Arrow-Pratt coefficients from asset-market data, and the equity-premium-puzzle critique of CRRA preferences, are out of scope for this card and live in 09 `pm-stochastic-discount-factor-intuition.md` plus Cochrane Pt.IV. **Source:** Mas-Colell et al. (1995) pp.183-202.
