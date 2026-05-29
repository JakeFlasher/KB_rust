---
schema_version: "cacg.v0"
id: "ec-utility-and-choice-under-uncertainty"
title: "Utility and Choice Under Uncertainty"
reading_id: "02_economics"
summary: "vNM expected-utility theorem: a preference relation on lotteries satisfying continuity and the independence axiom is representable by a utility function with the expected-utility form U(L) = sum_i p_i u(x_i); the Bernoulli utility u is unique up to positive affine transformation, with independence the substantive structural axiom."
tags: ["economics", "utility-choice"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p190:0310"
    chunk_hash: "233fde42c2e2a37f45eae827f573014f49035a4d6db8a43b6d0fcd0a0d5f6f7f"
    page_range: [190, 191]
    quote: "if we mix each of two lotteries with a third one, then the preference ordering of the two resulting mixtures does not depend on (is independent of) the particular third lottery used."
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p194:0316"
    chunk_hash: "c060eba0ff5723566cd103dcfe0bc448bdb691e1504f48e6724e7a248b22f3ce"
    page_range: [194, 194]
    quote: "his preferences are representable by a utility function with the expected utility form. It is the most important result in the theory of choice under uncertainty"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3364:5066"
    chunk_hash: "67759a7504764dd87acb5cfd6dac954b9b7863c03ae052a5fb341650c39099c7"
    page_range: [3364, 3365]
    quote: "In traditional finance theory, the individual focuses on maximizing unobservable utility, whereas the business maximizes a generally observable value"
    edge_type: "supports"
card_hash: "5e8930a39c332638ee1fba340ad9995506bf2cbe4071da0bff13a445fad023a8"
---
# Utility and Choice Under Uncertainty

## Intuition

When outcomes are uncertain, the consumer chooses among lotteries — probability distributions over outcomes — rather than among certain bundles. The von Neumann-Morgenstern (vNM) expected-utility theorem says that under three axioms (transitivity + continuity + independence) on the preference relation over lotteries, preferences are represented by a function that takes the *expected value* of a single Bernoulli utility `u: R → R`. The agent's choice over lotteries `L = (p_1, ..., p_n)` and `L' = (p_1', ..., p_n')` over outcomes `x_1, ..., x_n` reduces to comparing `Σ p_i · u(x_i)` against `Σ p_i' · u(x_i)`. **Source:** Mas-Colell et al. (1995) Ch.6 pp.167-175.

This is a bridge card per resolved DEC-4: the vNM expected-utility framework is the microeconomic foundation that downstream asset-pricing models (CAPM in 09; SDF representation; consumption-based pricing) build on. The CAPM derivation itself stays in 09 per AC-11 BOUNDARY-DISCIPLINE; this card only establishes that asset returns can be ranked by expected utility of payoffs without duplicating the portfolio-optimization derivation. **Source:** Mas-Colell et al. (1995) Ch.6 pp.167-175.

```
   lottery L = (p_1, p_2, ..., p_N) over outcomes {x_1, ..., x_N}

         p_1   p_2   ...   p_N
          │     │           │
          v     v           v
         x_1   x_2   ...   x_N        outcomes

   expected utility:  EU(L) = Σ_i p_i · u(x_i)
                                       └── Bernoulli utility on outcomes ──┘

   vNM theorem (under {transitivity, continuity, independence}):
       L ≿ L'  ⇔  EU(L) ≥ EU(L')
   u is unique up to positive affine transformation a·u + b (a > 0)
```

## Definition

A **lottery** `L` is a list of probabilities `(p_1, ..., p_N)` over `N` outcomes, with `Σ p_i = 1` and `p_i ≥ 0`. The set of all lotteries on a fixed outcome set `C = {x_1, ..., x_N}` is the `(N-1)`-simplex. **Source:** Mas-Colell et al. (1995) pp.167-182.

A preference relation `≿` on the simplex satisfies the **expected-utility axioms** if: **Source:** Mas-Colell et al. (1995) pp.167-182.

```
Continuity:     for any L, L', L'',  the sets { α ∈ [0,1] : αL + (1-α)L' ≿ L'' }
                and { α ∈ [0,1] : L'' ≿ αL + (1-α)L' } are closed in [0,1]

Independence:   for any L, L', L'' and α ∈ (0,1),
                L ≿ L'  iff  αL + (1-α)L'' ≿ αL' + (1-α)L''
```

(plus Transitivity from the rational-preference axioms.) **Source:** Mas-Colell et al. (1995) Ch.6 pp.168-172.

The **vNM expected-utility theorem** (Proposition 6.B.2 in MWG): a preference relation on the lottery space satisfies the three axioms above if and only if there exists a Bernoulli utility function `u: C → R` such that for any two lotteries `L, L'`,. **Source:** Mas-Colell et al. (1995) pp.167-182.

```
L ≿ L'  ⇔  Σ_i p_i · u(x_i)  ≥  Σ_i p_i' · u(x_i)
```

The Bernoulli utility `u` is unique up to positive affine transformation: `v(x) = a · u(x) + b` for `a > 0, b ∈ R` represents the same preference. **Source:** Mas-Colell et al. (1995) Ch.6 pp.173-175.

## Mathematical Reasoning

The Independence Axiom is the substantive force of the theorem. It says: if the consumer prefers `L` to `L'`, then mixing each with a third lottery `L''` at the same weight `α` preserves the preference. Without Independence the Allais paradox-style preference reversals are admissible; with Independence the preference relation over lotteries reduces algebraically to expected utility over outcomes. The Continuity Axiom (analog of the consumer-choice continuity axiom) is the topological condition needed for utility-representability on the lottery simplex. **Source:** Mas-Colell et al. (1995) Ch.6 pp.169-173.

The expected-utility representation is **separable across outcomes**: utility is a probability-weighted average of single-outcome utilities, with each outcome's utility evaluated by the same `u`. This separability is what allows asset-pricing models to write expected utility of wealth as `E[u(W)]` and to derive marginal-utility-weighted expected return conditions. The marginal utility `u'(x)` is the local rate at which the agent values an extra unit of wealth at outcome `x` — concavity of `u` (decreasing `u'`) is the substantive concept of risk aversion (developed in the sibling `ec-risk-aversion-utility-derivation` card). **Source:** Mas-Colell et al. (1995) Ch.6 pp.175-182.

The vNM framework is silent on the source of probabilities — they are taken as objective inputs to the lottery. Subjective expected utility (Savage's axioms) extends the framework to handle preferences over uncertain acts when probabilities are not given exogenously; that extension is out of scope here (and lives in MWG Ch.6 §6.F pp.197-211). The CAPM and SDF derivations in subcorpus 09 use the objective-probability vNM framework throughout, so this card establishes the only foundation needed for the 09 cross-link. **Source:** Mas-Colell et al. (1995) Ch.6 pp.175-182.

### Compound Lotteries and the Reduction Axiom

A **compound lottery** is a lottery over lotteries: e.g., with probability `α` the consumer faces lottery `L_1`, with probability `1−α` she faces lottery `L_2`. The **Reduction-of-Compound-Lotteries axiom** (implicit in MWG's framing) states that the consumer is indifferent between any compound lottery and the simple lottery obtained by computing the resulting marginal probabilities over outcomes. This axiom is a consequence of the vNM expected-utility theorem; it lets all multi-stage lotteries be reduced to single-stage lotteries for the expected-utility comparison. The independence axiom together with reduction is the substantive content that asset-pricing applications inherit when computing expected utility of a stochastic payoff stream as `E[Σ β^t u(c_t)]`. **Source:** Mas-Colell et al. (1995) Ch.6 pp.168-173.

## Boundary Notes

The expected-utility framework is a **theorem about preferences**, not about probabilities. The Independence Axiom states the substantive content: pairwise lottery preferences must be preserved under mixing with a third common lottery at fixed mixing weight. When Independence fails (Allais paradox: experimental subjects systematically violate it under certain payoff scales), expected-utility representation fails — preferences may still be rational and continuous, but they cannot be written as `Σ p_i · u(x_i)` for any `u`. Non-expected-utility extensions (Quiggin rank-dependent utility, Machina's local-expected-utility analysis, prospect theory) relax Independence; those extensions live in MWG Ch.6 pp.179-182 and future-10 Behavioral Finance per the v10 BOUNDARY-DISCIPLINE. **Source:** Mas-Colell et al. (1995) Ch.6 pp.179-182.

The vNM Bernoulli utility `u` is **defined on outcomes**, not on lotteries; this distinction matters when constructing the asset-pricing pricing kernel in 09. The kernel `m = β · u'(C_{t+1}) / u'(C_t)` (in the consumption-based CCAPM framing) evaluates marginal utility on the realized consumption outcome at each future state, then takes the expectation across states weighted by the lottery probabilities. The 09 `pm-stochastic-discount-factor-intuition.md` card uses this construction; this 02 card supplies the foundational link between preferences and the Bernoulli `u` that the SDF derivation requires. **Source:** Mas-Colell et al. (1995) Ch.6 pp.175-182.

## See Also

- [`ec-consumer-preference-and-choice`](./ec-consumer-preference-and-choice.md) — the underlying rationality axioms (transitivity inherits here)
- [`ec-risk-aversion-utility-derivation`](./ec-risk-aversion-utility-derivation.md) — Arrow-Pratt risk-aversion measures built on the vNM Bernoulli `u`
- [`pm-stochastic-discount-factor-intuition`](../09_portfolio_management_and_asset_pricing/pm-stochastic-discount-factor-intuition.md) — SDF asset-pricing equation; uses the vNM expected-utility framework as its foundation
- [`pm-capm-and-sml`](../09_portfolio_management_and_asset_pricing/pm-capm-and-sml.md) — CAPM equation derivation; assumes vNM expected utility plus mean-variance preferences

## Escalate to Raw When

The full proof of the vNM expected-utility theorem (sufficiency direction) constructs the Bernoulli `u` by solving a fixed-point equation on the lottery simplex — re-open MWG Ch.6 pp.173-175 for the construction. The Allais and Machina paradoxes (preference reversals violating Independence) sit in MWG pp.179-182. For the subjective-probability extension (Savage axioms; state-contingent preferences) see MWG §6.F pp.197-211. The asset-pricing cross-references in 09 build directly on the vNM framework without using Savage; the boundary discipline in `_style_guide.md` requires that this card stay at the preference-foundation level and not derive CAPM equations. **Source:** Mas-Colell et al. (1995) pp.167-182.
