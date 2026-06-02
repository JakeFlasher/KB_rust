---
schema_version: "cacg.v0"
id: "cb-conversion-premium"
title: "Conversion Premium"
reading_id: "08_convertible_bonds"
summary: "Conversion Premium — placeholder summary                                        "
tags: ["convertible-bonds", "conversion-premium"]
citations:
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p032:0030"
    chunk_hash: "fbcfebc44077d76bbbbe02e2ddb1a1cc9380cef91bb44e0d4132bebdca18f4dd"
    page_range: [32, 32]
    quote: "The premium above conversion value represents the percentage premium that the convertible is trading above its equity value component. The higher the conversion premium, the lower the equity sensitivity, and the lower the conversion premium, the more equity sensitive the issue."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p062:0070"
    chunk_hash: "3976cb6629efe93e473fdc974d68319b8dfdc08ae1762cedba01fb21283c4c80"
    page_range: [62, 63]
    quote: "For high share prices, the convertible’s price converges to parity and the convertible bond adopts equity-like behavior."
    edge_type: "supports"
card_hash: "c6128b8489e2f360aacc0faf7eafa1c0408deb1adce58718efa4a392f61affea"
---
# Conversion Premium

## Intuition

The conversion premium is the percentage by which the convertible's market
price exceeds its parity (conversion value). It is positive while the
embedded conversion option still has time value; it shrinks toward zero in
the deep-equity regime and is mathematically large in the deep-bond regime
where parity itself is small.
**Source:** Calamos (2003) §4 pp.50-80; DeSpiegeleer et al. (2014) §3.1
pp.55-65.

```
                     CB price V(t)
                          ^           ____  V(t) (smooth curve)
                          |         /
                          |       /
                          |     /
                          |    .         <-- conversion premium = V/P - 1
                          |   .
                          |  .         /  parity P(t) = q·S(t)
                          | .       /
                          |.    /
                          |  /            (premium ratio → 1 in deep equity)
   bond floor B(t)        |/
   (premium ratio large)  /
                          +----------------------> S(t)
```

## Definition

The **conversion premium** at time `t` is the relative excess of the CB price
over parity, an unitless ratio. **Source:** Calamos (2003) §4 pp.55-70;
DeSpiegeleer et al. (2014) §3.1 pp.55-65.

    π(t) := V(t) / P(t)  -  1   =   ( V(t) - q · S(t) ) / ( q · S(t) )

This is the convention named by both Calamos (2003) §4 pp.55-70 and
DeSpiegeleer et al. (2014) §3.1 pp.55-65 when they say "conversion premium"
without a qualifier. **Source:** Calamos (2003) §4 pp.55-70.

A distinct quantity sometimes confused with the conversion premium is the
share's **in-the-money percentage** `S(t) / K_c - 1`, which compares the
market share price to the conversion price `K_c = F/q` rather than the bond
price to parity. The two coincide only at par (`V(t) = F` and the bond is
quoted at 100). **Source:** Calamos (2003) §4 pp.55-70.

The complementary practitioner quantity is the **investment premium**
`π_B(t) := V(t) / B(t) - 1`, which measures the percentage by which the
convertible exceeds its credit-risky bond floor — small when the bond is
trading near `B(t)`, large in the equity-dominant regime. **Source:**
Calamos (2003) §4 pp.55-70.

## Mathematical Reasoning

By the dual-floor inequality `V(t) ≥ max(B(t), q · S(t))` (see
[parity card](./cb-parity-and-conversion-value.md#mathematical-reasoning)),
the conversion premium is bounded below by zero whenever parity dominates the
bond floor; it can be strictly positive even at parity dominance because the
unexercised conversion option still carries time value. **Source:**
DeSpiegeleer et al. (2014) §3.1 pp.55-65.

Under the special-case decomposition for a non-callable, non-puttable,
European-conversion, no-default convertible (see the
[payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#definition)),
the premium maps to the Black-Scholes embedded-call **time value** in the
equity-dominant regime and to the bond-floor cushion in the bond-dominant
regime. The mapping is approximate for real callable / credit-risky issues
because the decomposition itself is approximate there. **Source:**
DeSpiegeleer et al. (2014) §3.4 pp.65-78; Calamos (2003) §5 pp.70-90.

Asymptotic behavior of the premium ratio (cases below).
**Source:** DeSpiegeleer et al. (2014) §3.4 pp.65-78; Calamos
(2003) §5 pp.70-90.

- `S(t) → ∞`: the CB's delta `∂V/∂S → q` and `V(t)/P(t) → 1`, so
  `π(t) → 0` (delta-ratio sense). The convertible is equity-like; the
  premium vanishes in the ratio sense even when an absolute time-value
  residual remains. **Source:** DeSpiegeleer et al. (2014) §3.4 pp.65-78.
- `S(t) → 0`: parity `P(t) → 0`, so `π(t) = V(t)/P(t) - 1` is
  mathematically large — the more useful quantity in this regime is the
  investment premium `π_B(t) → 0` (the bond is trading near its
  credit-risky floor). **Source:** Calamos (2003) §5 pp.70-80.
- `S(t) ≈ K_c` (at-the-money on a face basis): maximum convexity in the
  CB price; the absolute conversion premium tends to its empirical peak in
  calm markets — practitioners watch this as the "balanced" CB regime.
  **Source:** DeSpiegeleer et al. (2014) §3.4 pp.65-78.

## See Also

- [`cb-parity-and-conversion-value.md`](cb-parity-and-conversion-value.md) — `P(t)` is the denominator
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — `B(t)` for the investment-premium analog
- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — premium ↔ embedded-call time value in the special-case decomposition

## Escalate to Raw When

Open Calamos §4-§5 pp.50-90 when the practitioner-quoting conventions
matter: distinguishing premium-over-parity, premium-over-investment-value,
and the dollar-equivalent premium quoted by dealers. **Source:** Calamos
(2003) §4-§5 pp.50-90.

Open DeSpiegeleer §3.1-§3.4 pp.55-78 when calibrating the embedded-call
volatility against listed convertible quotes (the "implied volatility" of a
CB is the σ that reconciles the decomposition with the market price).
**Source:** DeSpiegeleer et al. (2014) §3.1-§3.4 pp.55-78.

Open Philips §3 pp.30-55 for the historical evolution of premium norms before
the 2000s arbitrage-driven compression. **Source:** Philips (1997) §3
pp.30-55.
