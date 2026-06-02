---
schema_version: "cacg.v0"
id: "rm-exposure-profile-shapes"
title: "Exposure-Profile Shapes: Square-Root-of-Time and (T−t)√t Rules"
reading_id: "11_risk_management"
summary: "Canonical credit-exposure profile shapes from diffusion plus cash-flow roll-off: monotone √t for single-cash-flow forwards, peaked (T−t)√t for swaps (max near T/3), and jump profiles for credit derivatives, per Gregory Ch.11.2."
tags: ["risk-management", "counterparty-risk", "exposure"]
citations:
  - source_id: "rm_gregory_2020_xva_challenge"
    chunk_id: "rm_gregory_2020_xva_challenge:p305:0461"
    chunk_hash: "28ce890c69823d5ee5fe473de9d4314f522261b4d367e975ee744ccd2b47de1f"
    page_range: [306, 306]
    quote: "the exposure of such a profile will follow a ‘square-root-of-time’ rule, meaning that the exposure will be proportional to the square root of the time"
    edge_type: "defines"
card_hash: "1f70b0eb741c6c1cdc418234fe7c6303a5d0ce4d934515ef00c97e35dc9c012a"
---
# Exposure-Profile Shapes: Square-Root-of-Time and (T−t)√t Rules

## Intuition
Counterparty credit exposure on a derivative is not a fixed notional; it is the
positive part of an uncertain future value, so it evolves over the life of the trade.
Two forces fight: **diffusion** (the longer you wait, the wider the value distribution,
pushing exposure up) and **cash-flow roll-off** (every payment exchanged retires some
of the remaining risk, pulling exposure down). The characteristic *shape* of the
exposure profile tells you which force dominates and when peak risk occurs. A single
final cash flow (FX forward) is pure diffusion — exposure climbs monotonically. A swap
exchanges many cash flows along the way — exposure rises then falls, peaking partway
through. A credit derivative carries a discrete default payoff — exposure can jump.

```
  Exposure                       Exposure
  ^                              ^
  |             __ √t           |        ___
  |        __--               |      /     \   (T-t)·√t
  |    _--      forward         |    /         \   swap
  |  -                          |  /             \_
  +-------------------> t       +----|-----------> t
  0                   T         0   ~T/3          T
   monotone increasing          peaked, max near T/3
```

**Source:** Gregory (2020) Ch.11.2.1–11.2.2 printed pp.293–294 (PDF pp.306–307).

## Definition
- **Forward-type (single final cash flow).** With i.i.d. underlying returns the
  exposure obeys a **square-root-of-time rule**, proportional to √t for t ≤ T. Maturity
  T does not enter except that exposure is zero after it. Vanilla options with an
  upfront premium share this shape.
- **Swap-type (periodic cash flows).** The competition between growing uncertainty and
  roll-off of fixed-against-floating payments yields a **peaked** profile,
  approximately proportional to (T − t)·√t for t ≤ T, where T is the maturity.
- **Credit derivatives.** Profiles are hard to characterise because of discrete
  payoffs; a single-name CDS shows a swap-like expected exposure but a potential-future-exposure
  **jump** tied to the reference entity's default.

These are *proportionality* (shape) statements; the percentage magnitudes shown in the
book's figures are illustrative of particular volatility/distribution assumptions only.

**Source:** Gregory (2020) Ch.11.2.1–11.2.7 printed pp.293–302 (PDF pp.306–315).

## Mathematical Reasoning
For a single cash flow whose value diffuses with i.i.d. increments, the standard
deviation of the future value scales as σ√t, and since exposure = E[max(value, 0)] is
proportional to that dispersion when the mean is near zero,

    Exposure(t) ∝ √t,        t ≤ T.

For a swap, model the remaining value at time t as the diffusion term times the
remaining "amount" of swap still live. Roll-off shrinks the remaining notional roughly
linearly in (T − t), while diffusion still contributes √t, giving

    Exposure(t) ∝ (T − t)·√t,        t ≤ T.

Maximising f(t) = (T − t)√t: set f′(t) = 0 ⇒ −√t + (T − t)/(2√t) = 0 ⇒ T − t = 2t ⇒
t* = T/3. So the **peak exposure occurs at one-third of the maturity** — a purely
structural consequence of the two competing powers of t, independent of the volatility
level. A longer-maturity swap carries more risk on both counts (longer life and more
payments to exchange).

**Source:** Gregory (2020) Ch.11.2.2 printed pp.293–294 (PDF pp.306–307).

## See Also
- [rm-portfolio-xva-aggregation](./rm-portfolio-xva-aggregation.md) — how these single-trade profiles aggregate and net into portfolio exposure.
- [rm-sa-ccr-counterparty-capital](./rm-sa-ccr-counterparty-capital.md) — how regulators turn exposure profiles into EAD via EEPE and add-ons.
- [rm-wrong-way-risk-taxonomy](./rm-wrong-way-risk-taxonomy.md) — what happens when exposure correlates with counterparty default.
- [rm-credit-risk-metrics-restatement](./rm-credit-risk-metrics-restatement.md) — the EE/EPE/PFE metrics these profiles are measured in.

## Escalate to Raw When
You need the worked percentage exposure figures (the FX-forward and swap profiles at
their assumed volatilities, or the CDS PFE jump at default given a recovery assumption),
the cross-currency and moneyness numeric profiles, or the Appendix 11A/11B closed-form
formulas for a normal distribution — those numeric recipes live in the raw text (Rule 1).

**Source:** Gregory (2020) Ch.11.2 printed pp.292–305 (PDF pp.305–318).
