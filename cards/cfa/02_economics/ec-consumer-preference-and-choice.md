---
schema_version: "cacg.v0"
id: "ec-consumer-preference-and-choice"
title: "Consumer Preference and Choice"
reading_id: "02_economics"
summary: "MWG Ch.1 rationality axioms (completeness, transitivity) underwriting the preference relation; the choice rule via maximization; the Weak Axiom of Revealed Preference (WARP) as the consistency condition linking observed choice to underlying preference."
tags: ["economics", "consumer-preference"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p025:0027"
    chunk_hash: "04d63093e7c41974a528c1ab00eca47eacb37a0a9fa0b5de8e5b9d84973994d0"
    page_range: [25, 25]
    quote: "The completeness axiom says that this task has taken place: our decision makers make only meditated choices."
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p029:0036"
    chunk_hash: "cbe9a5df1de2955010f06224da98bbfd68b724abd716542e8c2e94e970cf872f"
    page_range: [29, 30]
    quote: "In words, the weak axiom says that if xis ever chosen when y is available, then there can be no budget set containing both alternatives for which y is chosen and x is not."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p580:0797"
    chunk_hash: "f61c9cfb1f0d1ae4d81e960d30a367488b2af0cb22e72018ffe30eec7c76facd"
    page_range: [580, 582]
    quote: "Microeconomics classifies private economic units into two groups: consumers (or households) and firms."
    edge_type: "supports"
card_hash: "360317735fcde2852dffc5c4d167044cac08f65c211e472fde9e486e4c338c77"
---
# Consumer Preference and Choice

## Intuition

A consumer's choice is the visible behavior; preference is the unseen ordering that generates it. Microeconomics builds demand by first axiomatizing preference (completeness + transitivity make it rational), then showing that any rational preference can be represented by a utility function so we can take derivatives. The Weak Axiom of Revealed Preference (WARP) is the consistency requirement that connects observed choice back to an underlying preference: if you chose `x` when `y` was available, you should not later choose `y` when `x` is available at the same prices. **Source:** Mas-Colell et al. (1995) Ch.1 pp.5-16.

The preference-based and choice-based approaches are two views of the same agent: preference-based starts with a binary relation `≿` over alternatives and derives the choice function; choice-based starts with the choice function `C(B)` for each budget `B` and derives the preference relation via revealed-preference. WARP is what makes the two equivalent under standard regularity conditions. **Source:** Mas-Colell et al. (1995) Ch.1 pp.5-16.

```
   preference-based                      choice-based
   ─────────────────                     ────────────
        ≿                                   C(B)
        │                                    │
        │ maximize over B                    │ collect across all B
        ▼                                    ▼
       C(B)  ────── WARP equivalence ──────  ≿*
       (choice from B)                       (revealed preference)

   WARP: if x ∈ C(B), y ∈ B, and x ∈ B',  then x ∈ B' implies y ∉ C(B')
         (no pairwise choice reversal at fixed prices)
```

## Definition

A preference relation `≿` on a set of alternatives `X` is **rational** if it satisfies two axioms. **Source:** Mas-Colell et al. (1995) pp.5-16.

```
Completeness:  for all x, y in X, either x ≿ y or y ≿ x (or both)
Transitivity:  for all x, y, z in X, if x ≿ y and y ≿ z, then x ≿ z
```

Strict preference `≻` and indifference `~` are derived from `≿`: `x ≻ y` iff `x ≿ y` and not `y ≿ x`; `x ~ y` iff `x ≿ y` and `y ≿ x`. **Source:** Mas-Colell et al. (1995) Ch.1 pp.5-12.

A **utility function** `u: X → R` represents `≿` if `x ≿ y ⇔ u(x) ≥ u(y)` for all `x, y`. For finite or countable `X`, rationality alone guarantees such a `u` exists; on continuous spaces an additional continuity axiom is required. Utility is **ordinal**: any strictly increasing transformation `v = f(u)` represents the same preferences. **Source:** Mas-Colell et al. (1995) Ch.1 pp.9-14.

A **choice structure** is a pair `(B, C(·))` where `B` is a family of nonempty subsets of `X` (the budget sets) and `C: B → 2^X` selects nonempty subsets for each `B ∈ B`. The Weak Axiom of Revealed Preference is the consistency condition. **Source:** Mas-Colell et al. (1995) pp.5-16.

```
WARP:  if x, y ∈ B and x ∈ C(B), and there is another B' with x, y ∈ B'
       and y ∈ C(B'),  then x ∈ C(B') too
```

**Source:** Mas-Colell et al. (1995) Ch.1 pp.10-13.

## Mathematical Reasoning

Rationality of `≿` is the structural assumption that makes maximization well-defined as a choice procedure. Without completeness the agent could face two alternatives and have no opinion; without transitivity the agent could be cyclic (`x ≻ y ≻ z ≻ x`) and any choice rule built from `≿` would be ambiguous on triplets. With both axioms, the maximization rule `C(B) = { x ∈ B : x ≿ y for all y ∈ B }` always returns a nonempty set on finite `B` and gives a coherent ordering on infinite `B` under standard continuity. **Source:** Mas-Colell et al. (1995) Ch.1 pp.5-11.

The link between preference and choice runs in two directions. Forward direction (preference → choice): given rational `≿`, the maximization rule generates a choice structure that satisfies WARP. Reverse direction (choice → preference): given a choice structure satisfying WARP, the revealed-preference relation `x ≿* y ⇔ ∃ B ∈ B with x, y ∈ B and x ∈ C(B)` is rational on the set of alternatives that ever appear in some `B`. The two-way equivalence is Proposition 1.D.2 in MWG: rational `≿` and WARP-satisfying choice are interchangeable representations of the same agent. **Source:** Mas-Colell et al. (1995) Ch.1 pp.12-16.

WARP is the contract between observed market behavior and the unobserved preference relation. Empirical demand analysis assumes WARP holds because otherwise no preference-based prediction would be testable. The Strong Axiom of Revealed Preference (SARP) is the cyclic-consistency extension required for full integrability of demand to a utility function in `R^L` — SARP rules out cycles of any finite length, not just pairwise reversals. SARP equivalence to utility-representability under continuity is a separate result, not required for the basic preference-choice equivalence. **Source:** Mas-Colell et al. (1995) Ch.1 pp.12-16.

## See Also

- [`ec-consumer-utility-and-demand`](./ec-consumer-utility-and-demand.md) — utility maximization, Walrasian demand, indirect utility, expenditure function
- [`ec-utility-and-choice-under-uncertainty`](./ec-utility-and-choice-under-uncertainty.md) — vNM expected utility under lotteries
- [`ec-risk-aversion-utility-derivation`](./ec-risk-aversion-utility-derivation.md) — Arrow-Pratt risk aversion

### Cardinal vs Ordinal Utility

Utility-representability under rationality alone is **ordinal** — only the ranking of bundles matters, not the numerical magnitudes assigned. Any strictly increasing transformation `v = f(u)` represents the same preferences and is equally valid as a utility function. This ordinality property has substantive consequences: the consumer's optimal choice depends only on the ranking, so cross-consumer utility comparisons are not meaningful within the ordinal framework. The **cardinal utility** required for interpersonal welfare comparisons (e.g., "is consumer A's utility loss from a tax larger than consumer B's gain?") needs additional structure: either the vNM expected-utility framework (where utility is unique up to positive affine transformation, narrower than ordinal but still not interpersonally comparable) or a normative welfare-weighting choice imposed by the policymaker. The two welfare theorems (FFWT, SFWT) sidestep cardinal-utility issues by working only with Pareto efficiency, an ordinal criterion. **Source:** Mas-Colell et al. (1995) Ch.1 pp.14-16.

## Boundary Notes

The rational-preference axioms here are necessary conditions for the entire static consumer-choice machinery: without completeness and transitivity, the sibling `ec-consumer-utility-and-demand` card's utility-maximization framing has no derivable utility function. The card stays at the foundational level — no demand curves, Slutsky decomposition, or Walrasian / Hicksian demand machinery (those live in `ec-consumer-utility-and-demand`). The lottery / vNM extensions live in `ec-utility-and-choice-under-uncertainty`; this card's axioms are the certainty-world prerequisite that vNM extends. **Source:** Mas-Colell et al. (1995) Ch.1 pp.14-16.

The behavioral-deviation literature (Allais paradox, intransitive preference cycles in experimental settings, framing effects) is deferred to future-10 Behavioral Finance per the v10 BOUNDARY-DISCIPLINE. This card's rationality axioms are the **descriptive-vs-normative bridge**: the axioms describe how a "consistent" agent must behave; the behavioral literature documents when humans deviate from this normative benchmark. The deviation literature does not invalidate the rationality framework — it documents the gap between the framework and observed behavior, and modern behavioral models extend or relax specific axioms rather than abandoning the rationality program. **Source:** Mas-Colell et al. (1995) Ch.1 pp.14-16.

### Continuity and Utility-Representation

For countable alternative sets, rationality (completeness + transitivity) alone suffices for a utility-function representation. For uncountable sets (e.g., consumption bundles in `R^L_+`), an additional **continuity** axiom is required: for any sequence `x_n → x` and `y_n → y` with `x_n ≿ y_n` for all `n`, the limit preference satisfies `x ≿ y`. Continuity rules out discontinuous "jumps" in preferences (e.g., lexicographic preferences that strictly favor any positive amount of good 1 over any amount of good 2) that have no continuous utility representation. The Debreu representation theorem (MWG Proposition 3.C.1) gives the formal continuity-implies-representation result. **Source:** Mas-Colell et al. (1995) Ch.3 pp.46-47.

## Escalate to Raw When

The proof of Proposition 1.D.2 (preference ↔ WARP-satisfying choice
equivalence) requires the regularity conditions for the universe `B` of
budget sets — re-open MWG pp.13-15 if a question requires the precise
condition on `B` (must include all two-element and three-element subsets
of `X`). **Source:** Mas-Colell et al. (1995) pp.5-16.

The continuity axiom required for utility-representation on uncountable
`X` is in MWG pp.46-47 (Ch.3 prerequisite). For the WARP-vs-SARP
distinction and the equivalence of WARP-satisfying demand with
utility-maximization in `R^L`, re-open MWG Ch.2 pp.27-36. **Source:** Mas-Colell et al. (1995) pp.5-16.

The lexicographic-preference counterexample (preferences that are
rational but lack continuous utility representation) is in MWG pp.46-47;
this is the canonical example showing why continuity is needed beyond
rationality alone for the Debreu representation theorem. **Source:**
Mas-Colell et al. (1995) Ch.3 pp.46-47.
