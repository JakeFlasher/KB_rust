---
schema_version: "cacg.v0"
id: "be-reference-dependent-preferences-foundations"
title: "Foundations of Reference-Dependent Preferences"
reading_id: "10_behavioral_finance"
summary: "O'Donoghue-Sprenger's foundations: reference-dependent preferences derive gain-loss utility from how realized outcomes compare to a reference point, departing from Expected Utility over final wealth; the value function has zero value at the reference point, diminishing sensitivity, and loss aversion, with modeling choices over coding and reference-point selection."
tags: ["behavioral-finance", "reference-dependence", "prospect-theory", "loss-aversion", "gain-loss-utility"]
citations:
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p010:0010"
    chunk_hash: "a6bf6f4fbbb0588c0cf08039183ba4a8ea4a19bdd1e0fcffbc4833d87d2c17a4"
    page_range: [11, 11]
    quote: "outcomes are not experienced on an absolute scale, but rather are experienced relative to some point of reference. Moreover, losses relative to the reference point are felt more severely than commensurate gains."
    edge_type: "defines"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p020:0025"
    chunk_hash: "324d8171f25df0bffab230707acdc79409adde75ca6eed85815a8504b8e5a3f7"
    page_range: [21, 21]
    quote: "let r be a reference point around which a person defines gains and"
    edge_type: "defines"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p020:0025"
    chunk_hash: "324d8171f25df0bffab230707acdc79409adde75ca6eed85815a8504b8e5a3f7"
    page_range: [21, 21]
    quote: "Zero value of reference point: v(0) = 0."
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p021:0026"
    chunk_hash: "084910a26b8e6304aca9798604570cab55e8ee81e63c2be3418194c16fd01c5c"
    page_range: [22, 22]
    quote: "losses loom larger than commensurate gains"
    edge_type: "supports"
card_hash: "fdf5809eeab1c0de0328399df5104a8516666e79737da8915f984f18960f52e6"
---
# Foundations of Reference-Dependent Preferences

## Intuition

Reference dependence captures a central intuition: outcomes are not experienced on an absolute scale but relative to some point of reference, and losses relative to that reference are felt more severely than commensurate gains. This "loss aversion" rationalizes prominent deviations from the canonical Expected-Utility-over-final-wealth model and interprets a broad swathe of economic behavior. The idea is analogous to the human perceptual system, which is more sensitive to *changes* in brightness, loudness, or temperature than to their absolute levels.
**Source:** O'Donoghue & Sprenger (2018) §1 pp.11.

The departure from Expected Utility is structural. Standard Expected Utility, under integration, evaluates a prospect by applying a concave utility to final wealth `w + x_n`. Reference-dependent preferences instead define gains and losses around a reference point `r` and apply a value function `v(.)` to the *gain or loss* `x_n - r`. Whether something is a gain or a loss -- not just its absolute size -- now drives behavior. This is what lets the model accommodate Allais-type independence violations and the small-stakes risk aversion that Rabin's calibration theorem shows is impossible under Expected Utility over wealth.
**Source:** O'Donoghue & Sprenger (2018) §3.1-3.2 pp.20.

Two modeling choices are central and contested. First, *coding*: which gains and losses the decision-maker brackets together (a single grand bet, or many separate bets) -- tests of reference dependence are really joint tests of the value function and the correct bracket. Second, *reference-point selection*: the choice of `r` is a powerful degree of freedom. Kahneman-Tversky proposed the status quo / current wealth, but recognized that gains and losses are sometimes coded relative to an expectation or aspiration level that differs from the status quo -- the motivation for later expectations-based models.
**Source:** O'Donoghue & Sprenger (2018) §3.2, §4.5 pp.20.

## Definition

**Reference-dependent preferences** are models with gain-loss utility derived from how realized outcomes compare to a reference point; the chapter explicitly restricts the term to gain-loss formulations (excluding habit formation, inequity aversion, salience, anticipated regret).
**Source:** O'Donoghue & Sprenger (2018) §1 pp.11.

**Reference point `r`** is the point around which a person defines gains and losses: an outcome `x_n > r` is a gain, `x_n < r` is a loss.
**Source:** O'Donoghue & Sprenger (2018) §3.2 pp.20.

**Value function `v(.)`** has three Kahneman-Tversky features: (1) zero value at the reference point, `v(0) = 0`; (2) diminishing sensitivity, `v''(x) < 0` for `x > 0` but `v''(x) > 0` for `x < 0`; (3) loss aversion, `v(x) < -v(-x)` and `v'(x) < v'(-x)` for `x > 0`.
**Source:** O'Donoghue & Sprenger (2018) §3.2 pp.20.

**Loss aversion** is the feature that losses loom larger than commensurate gains -- the value function is steeper over losses than over gains.
**Source:** O'Donoghue & Sprenger (2018) §3.2 pp.21.

## Mathematical Reasoning

For prospects `L = (x_1, p_1; ...; x_N, p_N)` and reference point `r`, the reference-dependent value is

```
  V(L | r) = sum_{n=1}^{N} p_n * v(x_n - r).
```

With `r = 0` (initial wealth, as Kahneman-Tversky suggest for simple gambles), `x_n` is itself the gain/loss. The structure of `V(L|r)` is identical in form to Expected Utility, so the shape of `v(.)` governs local risk attitudes: diminishing sensitivity (concavity) over gains makes the person locally risk-averse over gains, while convexity over losses makes her locally risk-seeking over losses.
**Source:** O'Donoghue & Sprenger (2018) §3.2 pp.20.

Two candidate functional forms are used. The Tversky-Kahneman (1992) form and a two-part-linear form:

```
  Tversky-Kahneman:                Two-part linear:
    v(x) = x^alpha    if x >= 0       v(x) = x         if x >= 0
         = -lambda(-x)^beta if x<0          = lambda*x  if x <= 0
    alpha, beta in (0,1], lambda >= 1     lambda >= 1
```

Here `alpha, beta` capture diminishing sensitivity in the gain and loss domains (= 1 means none), and `lambda` captures loss aversion (= 1 means none). The two-part-linear form drops diminishing sensitivity to isolate the implications of loss aversion alone -- the tradition most applications follow.
**Source:** O'Donoghue & Sprenger (2018) §3.2 pp.21, 22.

Crucially, loss aversion is *irrelevant* to the within-domain risk attitudes -- it only matters, creating an additional source of risk aversion, when a choice has both possible gains and possible losses. In binary `(x_1, x_2)` space with exogenous `r`, the indifference curve is kinked at `(r, r)`, so a loss-averse individual need not be risk-neutral over infinitesimally small stakes (unlike under Expected Utility), with three regions of slopes around `r`.
**Source:** O'Donoghue & Sprenger (2018) §3.2 pp.23.

## See Also

- [be-loss-aversion-reference-dependence](./be-loss-aversion-reference-dependence.md#intuition) -- loss aversion as the core gain-loss asymmetry.
- [be-expectations-based-reference-points](./be-expectations-based-reference-points.md#intuition) -- the Koszegi-Rabin alternative where `r` is rational expectations.
- [be-cumulative-prospect-theory-risk](./be-cumulative-prospect-theory-risk.md#intuition) -- the full prospect-theory model adding probability weighting.
- [be-prospect-theory-ingredient-decomposition](./be-prospect-theory-ingredient-decomposition.md#intuition) -- separating the value-function ingredients.
- [be-regret-aversion-status-quo-endowment](./be-regret-aversion-status-quo-endowment.md#intuition) -- reference dependence applied to riskless choice (endowment effect).

## Escalate to Raw When

- You need the riskless-choice multi-good formulation `U(x|r) = u(x) + v(x|r)` with the Tversky-Kahneman (1991) functional form for `v^n(x^n|r^n)` (pp.15, 24).
- You need the editing/mental-accounting and coding-and-bracketing discussion in applications (Sections 3.4, 4.5, pp.16, 30).
- You need the Rabin calibration theorem and Allais/Samuelson motivation for departing from Expected Utility (Section 2.3, pp.7-10).
