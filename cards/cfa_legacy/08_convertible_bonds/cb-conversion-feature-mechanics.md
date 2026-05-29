---
schema_version: "cacg.v0"
id: "cb-conversion-feature-mechanics"
title: "Conversion Feature Mechanics"
reading_id: "08_convertible_bonds"
summary: "Conversion Feature Mechanics — placeholder summary                              "
tags: ["convertible-bonds", "conversion-feature"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p053:0060"
    chunk_hash: "9ec10ac25ac4f579e25eaecbbce66f1ca31e67388046f9f92539cc87b2a4cbdd"
    page_range: [53, 54]
    quote: "The holder of a convertible bond has the option to end the bond’s existence prematurely by converting it into shares. This right is the optional conversion."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p242:0371"
    chunk_hash: "834ae1b123738b60e5d1e1c1a09c41767bff9374213356771e8430f1eef484b0"
    page_range: [242, 242]
    quote: "They are therefore bonds with an embedded call option on the company’s stock."
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p651:0967"
    chunk_hash: "c04abce6411df0c614d161917476d374cfabf4393d43ac4e6e20224be5a3e12f"
    page_range: [651, 651]
    quote: "Credit risk plays an important role in the valuation of convertibles. If credit risk is ignored, poor prices are obtained because the coupons and principal payments on the bond are overvalued."
    edge_type: "supports"
card_hash: "966ce590ca3087bf72c4f769d5dea47f675312797c4434154a29d184e68a6333"
---
# Conversion Feature Mechanics

## Intuition

The conversion feature is an embedded American-style call option held by the
bondholder, struck at the conversion price `K_c = F/q`, on the issuer's own
shares. Exercising it terminates the bond's cash-flow stream and delivers `q`
shares per face `F` of bond converted.
**Source:** Hull (recent ed.) §10.11 pp.241-242 ("convertibles are bonds with
an embedded call option"); DeSpiegeleer et al. (2014) §2.3 pp.30-44.

```
holder's decision at conversion-eligible date t
(with bond price V(t), share price S(t), conversion value q·S(t)):

         q · S(t)  >  V(t) ?
              |              \
              | yes            no
              v                 v
       convert NOW          hold (collect coupon, retain optionality)
       receive q · S(t)
       lose remaining cashflows
       lose optionality
```

## Definition

The **conversion ratio** `q` is the number of shares the holder receives per
face `F` of bond converted; the **conversion price** is `K_c := F / q`, the
break-even share price at which the conversion exchange is at par with face
value. **Source:** DeSpiegeleer et al. (2014) §2.3 pp.32-38.

The **conversion value** (parity) of the bond at time `t` is `P(t) := q · S(t)`
— the value the holder would receive on immediate conversion (see the
[parity card](./cb-parity-and-conversion-value.md#definition) for the
floor-inequality treatment). **Source:** DeSpiegeleer et al. (2014) §2.3
pp.32-38.

When the holder elects conversion, the issuer creates `q` new shares per face
`F` converted (or delivers treasury shares); the resulting share count is
`N_post = N_pre + q · n_converted` where `n_converted` is the count of bonds
converted, diluting existing shareholders. **Source:** DeSpiegeleer et al.
(2014) §2.3 pp.38-44; Calamos (2003) §2 pp.20-30.

The dilution is the issuer's implicit cost of the embedded call: in
no-arbitrage Black-Scholes-Merton notation the issuer is short `q` calls per
face on its own equity, so an issuer call premium discount is the standard
practitioner explanation for sub-straight coupon `c` (see the
[anatomy card](./cb-bond-anatomy-and-cashflows.md#mathematical-reasoning)).
**Source:** Hull (recent ed.) §10.11 pp.241-242.

## Mathematical Reasoning

By no-arbitrage at any conversion-eligible date `t`, the convertible price
satisfies `V(t) ≥ q · S(t)`: otherwise an arbitrageur buys the bond, converts,
and sells the shares for risk-free profit. **Source:** Hull (recent ed.)
§27.4 pp.650-653.

Voluntary conversion is a **continuation-value** decision, not a one-line
yield-comparison rule: the holder converts at `t` only if the conversion
value `q · S(t)` exceeds the conditional expected continuation value
`E[V(t+) | F_t]`. The optimal exercise boundary therefore depends on
remaining coupon stream, dividend yield `δ`, issuer-call schedule, holder-put
schedule, credit-spread dynamics, and volatility. **Source:** Hull (recent
ed.) §27.4 pp.650-653 (convertible-bond tree with credit/default mechanics);
DeSpiegeleer et al. (2014) §3.5 pp.78-95.

A useful first-order intuition (no calls, no puts, deterministic dividends)
is that voluntary early conversion becomes attractive when the dividend yield
on `S` rises so the equity income exceeds the bond income — but issuer-call
provisions, soft-call triggers, and dynamic credit can shift the boundary
materially in either direction. **Source:** DeSpiegeleer et al. (2014) §3.5
pp.78-95.

In the limit `S(t) → ∞`, the holder's optimal action approaches conversion
and the CB's effective delta `∂V/∂S` converges toward `q`; the conversion
premium (see the
[premium card](./cb-conversion-premium.md#mathematical-reasoning)) shrinks
toward zero in delta-ratio terms even when an absolute time-value tail
persists. **Source:** DeSpiegeleer et al. (2014) §3.4 pp.65-78.

In the limit `S(t) → 0`, voluntary conversion is deeply out-of-the-money and
`V(t)` approaches the credit-risky bond floor `B(t)`; the floor itself can
collapse if the issuer's spread widens with the equity stress (see the
[bond-floor card](./cb-bond-floor-investment-value.md#mathematical-reasoning)
on credit-equity coupling). **Source:** DeSpiegeleer et al. (2014) §3.4
pp.65-78.

### Holder Conversion Decision Flow

At each conversion-eligible date `t`, the holder compares parity
`q · S(t)` against continuation value `V(t)` and converts when parity
strictly exceeds continuation value. **Source:** Hull §10.11 pp.241-242;
DeSpiegeleer et al. (2014) §2.3 pp.30-44.

```
<!-- primitive: conversion-decision source: _diagram_primitives.md -->
holder at conversion-eligible date t
(bond price V(t), share price S(t)):

      q · S(t)  >  V(t) ?
           |                \
           | yes              no
           v                   v
      convert              hold (collect coupon,
      receive q · S(t)      retain optionality)
```

## See Also

- [`cb-bond-anatomy-and-cashflows.md`](cb-bond-anatomy-and-cashflows.md) — the bond fields `(F, q, K_c)` come from
- [`cb-parity-and-conversion-value.md`](cb-parity-and-conversion-value.md) — `P(t) = q · S(t)`
- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — the embedded call view
- [`cb-china-csrc-disclosure-timing.md`](cb-china-csrc-disclosure-timing.md) — China-onshore prospectus rule that mandates disclosure of these mechanics for CB issuers (CSRC Standard No. 60 Article 17)

## Escalate to Raw When

Open DeSpiegeleer §2.3 pp.30-44 or Hull §27.4 pp.650-653 directly when any
criterion below applies. **Source:** DeSpiegeleer et al. (2014) §2.3
pp.30-44; Hull (recent ed.) §27.4 pp.650-653.

- The bond has a **mandatory** conversion (the holder does not choose); the
  embedded right is then held by the issuer instead and the decomposition
  flips sign. **Source:** DeSpiegeleer et al. (2014) §2.3 pp.30-44.
- A **soft call** with conversion-price trigger is active — the holder's
  rational decision shifts because non-conversion risks forced redemption at
  par. **Source:** Hull (recent ed.) §27.4 pp.650-653.
- **Dividend protection** clauses or a **conversion-price reset** (e.g.
  China downward conversion) materially shift `q` over time.
  **Source:** DeSpiegeleer et al. (2014) §2.3 pp.30-44.
- **Net share settlement** or cash-or-shares optionality at conversion
  changes the issuer's dilution exposure. **Source:** DeSpiegeleer et al.
  (2014) §2.3 pp.30-44.
