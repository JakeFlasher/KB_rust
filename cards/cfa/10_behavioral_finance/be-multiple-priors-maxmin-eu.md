---
schema_version: "cacg.v0"
id: "be-multiple-priors-maxmin-eu"
title: "Multiple Priors and Maxmin Expected Utility"
reading_id: "10_behavioral_finance"
summary: "Maxmin expected utility (Gilboa-Schmeidler) evaluates a prospect as the minimum expected utility over a convex set of priors C; it equals Choquet EU when C is the CORE of a convex capacity, and the alpha-maxmin model blends the worst-case inf with the best-case sup."
tags: ["behavioral-finance", "multiple-priors", "maxmin-expected-utility", "ambiguity"]
citations:
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p341:0469"
    chunk_hash: "64449bb101caa62e1925d20b3c1ed18994da82edd9370b22a6f9e96707af0c20"
    page_range: [341, 341]
    quote: "A popular alternative to RDU for Case 1 is the maxmin expected utility (MEU) model, also sometimes referred to using the general name multiple priors"
    edge_type: "defines"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p340:0467"
    chunk_hash: "67fa26fbdfad7179318d0cb570cf84e9c9717c0b4422ef194a1a8af16f9d8bbd"
    page_range: [340, 340]
    quote: "If W is convex then its CORE is nonempty and W is the minimum over all of its CORE elements."
    edge_type: "supports"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p342:0470"
    chunk_hash: "07de4e9e2c7d89528006904feae25ca5796f27b06a9249d616905a8cbd7a6867"
    page_range: [342, 342]
    quote: "It became popular in decision theory when Gilboa & Schmeidler (1989) proved its soundness by providing a behavioral foundation."
    edge_type: "supports"
card_hash: "0f77335a182f40f0349799a9d734d56682ccd5e56b231debe94a984e33e0f83c"
---
# Multiple Priors and Maxmin Expected Utility

## Intuition

When probabilities are unknown, one natural model of ambiguity aversion is to entertain not a single prior but a whole *set* of priors `C` and to evaluate each prospect by its worst case: take the minimum expected utility over all priors in the set. This is maxmin expected utility (MEU), also called the multiple priors model (Gilboa & Schmeidler 1989). A cautious decision maker who does not know the true distribution hedges by valuing a prospect at the most pessimistic expectation consistent with the candidate priors.
**Source:** Wakker (2010) §11.5 pp.325, §11.5 pp.326.

Wakker presents MEU as the principal benchmark *alternative* to rank-dependent Choquet expected utility, and shows where the two coincide and where they diverge. When the prior set `C` is exactly the CORE of a convex capacity `W`, MEU and RDU give identical values — the Choquet integral against a convex `W` equals the minimum EU over the CORE. But for general convex sets `C` that are not the CORE of any capacity, the two models are genuinely distinct and neither nests the other. The `alpha`-maxmin model relaxes pure pessimism by mixing the worst-case and best-case expectations.
**Source:** Wakker (2010) §11.4 pp.324, §11.5 pp.325-326.

## Definition

**Maxmin expected utility (MEU) / multiple priors** evaluates `MEU(x) = inf_{P in C} EU_P(x)`, the lowest expected utility over a convex set of priors `C`.
**Source:** Wakker (2010) §11.5 pp.325.

**CORE** of a capacity `W` is the set of probability measures `P` with `P(E) >= W(E)` for all events `E`; if `W` is convex the CORE is nonempty and `W` is the minimum over its CORE elements.
**Source:** Wakker (2010) §11.4 pp.324.

**alpha-maxmin (expected utility)** model evaluates `x -> alpha * inf_{P in C} EU_P(x) + (1-alpha) * sup_{P in C} EU_P(x)` for some `0 <= alpha <= 1`, blending pessimism and optimism.
**Source:** Wakker (2010) §11.5 pp.325.

## Mathematical Reasoning

The link between the rank-dependent (Choquet) model and multiple priors runs through the CORE. Theorem 11.4.1: if `W` is convex then its CORE is nonempty, `W` is the minimum over all CORE elements, and for any utility `U`,

```
RDU(x) = min_{P in CORE} EU_P(x),
```

so RDU equals MEU when `C` = CORE. For a general convex prior set `C`, define the lower-envelope capacity `W(E) = inf{ P(E) : P in C }`; then `W(E) <= P(E)` for all `P in C`, which implies `RDU <= MEU` and the inequality can be strict. Hence there exist MEU models that are not rank-dependent in any sense, and the two model classes are distinct.
**Source:** Wakker (2010) §11.4 pp.324, §11.5 pp.325.

```
W convex  --CORE-->  C = CORE(W)
                       RDU(x) = min_{P in C} EU_P(x)   (models coincide)

general convex C  -->  W(E) = inf_{P in C} P(E)
                       RDU(x) <= MEU(x)                 (can be strict; distinct models)
```

The rank-dependent definitions for probability-interval sets `I_E` recover the pessimistic, optimistic, and intermediate cases:

```
W(E) = inf(I_E)                              (most pessimistic)
W(E) = sup(I_E)                              (most optimistic)
W(E) = alpha*inf(I_E) + (1-alpha)*sup(I_E)   (alpha-maxmin intermediate)
```

MEU was introduced by Wald (1950) and became popular when Gilboa & Schmeidler (1989) proved its soundness by giving a behavioral foundation, using the Anscombe-Aumann two-stage approach with a convex outcome set and linear utility. Artzner et al. (1999) characterized linear-utility MEU with the representing function (a "risk measure") as primitive, and Observation 11.5.1 shows `alpha`-maxmin agrees with binary RDU on binary prospects.
**Source:** Wakker (2010) §11.5 pp.326-327.

## See Also

- [be-comonotonicity-choquet-integration](./be-comonotonicity-choquet-integration.md#intuition) — Choquet EU against a convex `W` is the bridge to the CORE.
- [be-ambiguity-sources-ellsberg](./be-ambiguity-sources-ellsberg.md#intuition) — the source-based rank-dependent treatment Wakker prefers over multiple priors.
- [be-rank-dependent-utility-via-ranks](./be-rank-dependent-utility-via-ranks.md#intuition) — the rank-dependent model that coincides with MEU on the CORE.
- [be-ambiguity-aversion-asset-pricing](./be-ambiguity-aversion-asset-pricing.md#intuition) — downstream ambiguity-aversion asset-pricing application.

## Escalate to Raw When

- You need the full discussion of multiple-priors behavioral foundations (Gilboa-Schmeidler, Chateauneuf, Casadesus-Masanell et al.) and the violations of monotonicity for multiple priors.
**Source:** Wakker (2010) §11.5 pp.326-327, App.11.10 pp.338.
- You need the CORE and convex-capacity results (Theorem 11.4.1) with proofs.
**Source:** Wakker (2010) §11.4 pp.324.
