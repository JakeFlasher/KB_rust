---
schema_version: "cacg.v0"
id: "fi-callable-and-putable-bonds"
title: "Callable and Putable Bonds"
reading_id: "06_fixed_income_and_credit"
summary: "Extends the vanilla bond to bonds with embedded issuer-call or holder-put rights: the option-side cashflow asymmetry, its effect on duration and convexity, and the call/put-protection schedule semantics."
tags: ["fixed-income", "callable-putable"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2385:3495"
    chunk_hash: "ddb2386c7a1bdfbcb60922696a99e4bfc03f8a4602094176bb727fcaa838ba19"
    page_range: [2385, 2385]
    quote: "A callable bond gives the issuer the right to redeem all or part of the bond before the specified maturity date."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p688:1025"
    chunk_hash: "f83673541d826f29ffcf607ab361bbb992d83b9b0428a7e79e76739719b8737b"
    page_range: [688, 689]
    quote: "Embedded Bond Options One example of a bond with an embedded bond option is a callable bond."
    edge_type: "supports"
card_hash: "4d5416875af8d659f957c7bdfa4e4a0f6c85b52bdc9367a98d6f186268a498a4"
---
# Callable and Putable Bonds

## Intuition

A callable bond grants the issuer the right to redeem the
bond before maturity at a pre-specified call price; a
putable bond grants the holder the right to sell the bond
back at a pre-specified put price. The embedded option
truncates the price-yield curve asymmetrically: the
callable's upside is capped (when yields fall, the issuer
calls), and the putable's downside is bounded (when yields
rise, the holder puts). **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.290-310.

```
price
   ^   *
   |    *  vanilla (convex everywhere)
   |     *
   |      *
   |       *  callable: capped above K_c
   |        +--*-----+ <-- issuer calls when price > K_c
   |             *      .
   |              *       . putable: floored below K_p
   |               *        +-------+ <-- holder puts when price < K_p
   |                *       .
   +----------------------------------> yield
   declining yield (left) drives callability;
   rising yield (right) drives putability.
```

## Definition

A callable bond extends the vanilla bond with a schedule of
issuer-call dates `T_c` and call prices `K_c(t)`. At each
call date the issuer compares the call price to the
prevailing market price; if calling is cheaper, the issuer
exercises and the bond ceases. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.290-310;
Hull §10 pp.241-260.

A putable bond extends the vanilla bond with a schedule of
holder-put dates `T_p` and put prices `K_p(t)`. At each put
date the holder compares the put price to the prevailing
market price; if putting is more valuable, the holder
exercises. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.290-310.

The bond's price decomposes as the value of the underlying
straight bond plus or minus the embedded option:
`P_callable = P_straight - call_value`,
`P_putable = P_straight + put_value`. The option values
are non-negative; both reduce or augment the bond's price
relative to the option-free counterpart. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.290-310; Hull §10 pp.241-260.

## Mathematical Reasoning

The callable's price-yield curve flattens above the call
price, producing the negative-convexity signature near at-
the-money: as yields fall the holder bears the cost of
truncated upside, so the convexity correction reverses
sign relative to the vanilla curve from
[`fi-duration-and-convexity.md`](./fi-duration-and-convexity.md#mathematical-reasoning).
Effective duration via shock is the only reliable measure
because the closed-form Macaulay derivation does not
accommodate the call boundary. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.290-310.

The putable's price-yield curve floors below the put
price, producing positive convexity reinforcement near at-
the-money: as yields rise the holder caps the loss at the
put price, so the price falls more slowly than the vanilla
curve. Effective duration shrinks toward the put-truncated
horizon as the put approaches at-the-money. **Source:** CFA
L1 Curriculum (2022) Vol.5/pp.290-310.

Hull's interest-rate-tree machinery (binomial / trinomial
short-rate trees calibrated to the term structure) prices
both option types by backward induction. The valuation
inputs are the term structure, volatility, and the call /
put schedule; the option's exercise boundary emerges
endogenously. **Source:** Hull §10 pp.241-260.

The convertible-bond literature uses the same option-
truncation machinery in
[`../08_convertible_bonds/cb-call-and-put-protection.md`](../08_convertible_bonds/cb-call-and-put-protection.md#definition)
where the embedded options interact with the conversion
right. **Source:** CFA L1 Curriculum (2022) Vol.5/pp.290-310.

## See Also

- [`fi-bond-anatomy-and-cashflows.md`](fi-bond-anatomy-and-cashflows.md) — vanilla bond as the option-free baseline
- [`fi-duration-and-convexity.md`](fi-duration-and-convexity.md) — option-embedded duration via shock; negative convexity
- [`../08_convertible_bonds/cb-call-and-put-protection.md`](../08_convertible_bonds/cb-call-and-put-protection.md) — call / put interaction with convertible bond conversion right

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 45 or Hull Chapter 10
directly when any of the criteria below applies. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.290-310;
Hull §10 pp.241-260.

- The call schedule is American (continuous) rather than
  Bermudan (discrete dates) and a continuous-exercise
  boundary must be characterized. **Source:** Hull §10
  pp.241-260.
- Soft-call provisions (parity-trigger / lookback /
  conditional triggers) are in scope; the trigger
  mechanics fall outside this card's discrete-call view.
  **Source:** CFA L1 Curriculum (2022) Vol.5/pp.290-310.
- Multiple embedded options interact (call + put + sinking
  fund + conversion); the bundle's exercise boundary may
  shift relative to the single-option case. **Source:**
  Hull §10 pp.241-260.
