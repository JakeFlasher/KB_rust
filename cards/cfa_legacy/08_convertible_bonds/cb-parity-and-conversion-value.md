---
schema_version: "cacg.v0"
id: "cb-parity-and-conversion-value"
title: "Parity and Conversion Value"
reading_id: "08_convertible_bonds"
summary: "Parity and Conversion Value — placeholder summary                               "
tags: ["convertible-bonds", "parity-conversion"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p054:0062"
    chunk_hash: "4c551662f76244110c7d8287e2f7bd711460f9ddb1742a40d380042c6339e11c"
    page_range: [54, 55]
    quote: "The conversion value or parity depends on the level of the share price"
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p062:0070"
    chunk_hash: "3976cb6629efe93e473fdc974d68319b8dfdc08ae1762cedba01fb21283c4c80"
    page_range: [62, 63]
    quote: "For high share prices, the convertible’s price converges to parity and the convertible bond adopts equity-like behavior."
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p032:0030"
    chunk_hash: "fbcfebc44077d76bbbbe02e2ddb1a1cc9380cef91bb44e0d4132bebdca18f4dd"
    page_range: [32, 32]
    quote: "The conversion value is also known as parity value in the convertible marketplace."
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p242:0371"
    chunk_hash: "834ae1b123738b60e5d1e1c1a09c41767bff9374213356771e8430f1eef484b0"
    page_range: [242, 242]
    quote: "They are therefore bonds with an embedded call option on the company’s stock."
    edge_type: "supports"
card_hash: "38ed71e10ba7b1f63af96782540df1537f99d1b78b8fd3f5733de5db7bebc422"
---
# Parity and Conversion Value

## Intuition

If a holder converted the bond right now, the equity received would be worth
`q · S(t)`. That number — variously called **parity**, **conversion value**, or
**equity-leg value** — is the second of the convertible's two natural lower
bounds at conversion-eligible dates (the first being the credit-risky bond
floor, see the
[bond-floor card](./cb-bond-floor-investment-value.md#definition)). The
convertible price `V(t)` rides above the upper envelope of the two.
**Source:** DeSpiegeleer et al. (2014) §2.3 pp.32-50; Calamos (2003) §4
pp.40-65.

```
value
  ^                                       parity P(t) = q · S(t)
  |                                            /
  |                                           /
  |                                          /
  |                              .---------./
  |                          .--/         /
  |   ___________________.--/            /
  |  /                  /                /
  | /          B(t) bond floor          /
  | /                  /                /
  +/__________________/_________________> share price S
        out-of-the-money   at par   in-the-money
```

## Definition

The parity (conversion value) is `P(t) := q · S(t)`, where `q` is the
conversion ratio (shares per face `F`) and `S(t)` is the issuer's share price.
**Source:** DeSpiegeleer et al. (2014) §2.3 pp.32-38.

Two equivalent quoting conventions appear in practice: cash parity per bond
`P(t) = q · S(t)` in currency units, and percentage parity per face
`P(t) / F = (q / F) · S(t) = S(t) / K_c` where `K_c = F / q` is the
conversion price. **Source:** DeSpiegeleer et al. (2014) §2.3 pp.32-38.

The parity floor `P(t) = q · S(t)` and the credit-risky bond floor
`B(t)` bound the convertible's value `V(t)` from below; their relative
order partitions the OTM / ATM / ITM regimes traced by the canonical
parity-and-floor overlay. **Source:** Calamos (2003) §4 pp.40-65.

```
<!-- primitive: parity-and-floor source: _diagram_primitives.md -->
value
  ^                             parity P(t) = q · S(t)
  |                                /
  |                               /
  |                              /
  |                  ____________/
  |                 /           /
  |   _____________/           /
  |  /                        /
  | /        bond floor B(t) /
  |/_________________________________> share price S
       OTM      ATM      ITM
```

## Mathematical Reasoning

At any conversion-eligible date `t ∈ T_conv`, no-arbitrage gives
`V(t) ≥ P(t) = q · S(t)`: the holder can convert immediately and receive
`q · S(t)`, so the convertible price cannot trade strictly below this without
permitting risk-free arbitrage. **Source:** Hull (recent ed.) §10.11
pp.241-242 ("convertibles are bonds with an embedded call option").

For non-callable, non-puttable issues with continuous conversion eligibility,
the dual-floor inequality `V(t) ≥ max(B(t), q · S(t))` holds at every
`t ∈ T_conv`; the inequality is in general strict because the unexercised
conversion option still has time value. **Source:** Hull (recent ed.) §27.4
pp.650-653; DeSpiegeleer et al. (2014) §2.3 pp.32-50.

    V(t) ≥ max( B(t), q · S(t) )

Asymptotics are stated as ratio/delta convergence below, since absolute
time-value decay is regime-dependent. **Source:** DeSpiegeleer et al. (2014)
§3 pp.55-78.

- `S(t) → ∞`: `P(t) → ∞` and the conversion value dominates the bond floor.
  The CB's effective delta `∂V / ∂S → q` and the conversion premium ratio
  `V(t) / P(t) → 1` (delta-ratio convergence); the bond is "equity-like".
  **Source:** DeSpiegeleer et al. (2014) §3 pp.55-78.
- `S(t) → 0`: `P(t) → 0` and the credit-risky floor `B(t)` dominates. The
  CB's effective delta tends toward zero in calm-spread regimes, but credit-
  equity coupling can break this asymptotic — see the
  [bond-floor card](./cb-bond-floor-investment-value.md#mathematical-reasoning).
  **Source:** Calamos (2003) §4 pp.50-65.
- `S(t) ≈ K_c` (at-the-money on a face basis): both lower bounds matter
  simultaneously; CB convexity is largest and the conversion premium peaks.
  See the
  [premium card](./cb-conversion-premium.md#mathematical-reasoning).
  **Source:** Calamos (2003) §4 pp.50-65.

## See Also

- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — `q` and `K_c` come from
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — the other lower bound `B(t)`
- [`cb-conversion-premium.md`](cb-conversion-premium.md) — the gap `V(t) - P(t)`
- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — embedded-call view

## Escalate to Raw When

Open Hull §27.4 pp.650-653 when the no-arbitrage justification needs to
extend to dividend-paying underlyings or credit-risky issuers in a tree
model. **Source:** Hull (recent ed.) §27.4 pp.650-653.

Open DeSpiegeleer §3 pp.55-78 when the asymptotic regimes need to be matched
against an actual market price chart (the parity-line vs CB-price overlay is
a standard practitioner diagnostic). **Source:** DeSpiegeleer et al. (2014)
§3 pp.55-78.
