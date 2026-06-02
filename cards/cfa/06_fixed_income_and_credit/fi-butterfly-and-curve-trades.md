---
schema_version: "cacg.v0"
id: "fi-butterfly-and-curve-trades"
title: "Butterfly and Curve Trades"
reading_id: "06_fixed_income_and_credit"
summary: "Barbell-bullet butterflies and curve trades as practitioner relative-value positions: duration-neutral by construction, exposed to specific term-structure shape changes (curvature, slope, twist), decomposable in the level/slope/curvature factor basis."
tags: ["fixed-income", "butterfly-curve"]
citations:
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p176:0217"
    chunk_hash: "b1e51e70a1dcd3ff083b6c4a62ac8b1d544e4ee7fa3bb067a2d0e49513591bf1"
    page_range: [176, 177]
    quote: "By definition, then, that bond’s key-rate ’01 with respect to that key rate would equal its yield-based DV01 while its key-rate ’01 with respect to all other key rates would be zero."
    edge_type: "defines"
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p201:0250"
    chunk_hash: "7bf19d52a81570ee1d122e5314376c8ba32d989e6a80410d360fb94f4ed2aa02"
    page_range: [201, 201]
    quote: "Because the yield on the x-bond is 5% today, the level equation (6.25) predicts that the yield on the y-bond will be 5% today, despite its being 1% yesterday."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2666:3987"
    chunk_hash: "bf98a49cbef0814bdc8ad5787ed26134c5a558286759abf6aa21b22e53a5acde"
    page_range: [2666, 2667]
    quote: "Solution to 2: Despite the significant differences in times-to-maturity (10, 20, and 30 years), the approximate modified durations on the three bonds are fairly similar (4.768, 5.169, and 5.063)."
    edge_type: "supports"
card_hash: "8ad78ae01b6f9faca278bf11a20cc5dd07e393f616f19797fbf941c5d85bb44f"
---
# Butterfly and Curve Trades

## Intuition

A practitioner sees the Treasury curve at multiple tenors and may hold a view on how the curve's shape will change without taking a directional view on the level of rates. The **barbell-bullet butterfly** is the canonical curve-trade structure: long the 2y and 30y (the "wings") + short the 10y (the "body"), or the mirror. The trade is constructed to be duration-neutral so that a parallel shift in the curve has zero first-order PnL; the PnL comes from how the curve's curvature changes (the 10y rises or falls relative to the average of the 2y and 30y). **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

```
butterfly trade structure (long the wings, short the body)
   yield curve at observation
       ^
       |
     y |               y_body
       |              *
       |        y_wing1            y_wing2
       |     *                  *
       |  *
       +----+-------+--------+-------+--> tenor
         t_wing1    t_body            t_wing2

   trade legs (notionals chosen to make portfolio duration neutral):
       long N_w1 of bond at t_wing1 (duration D_w1)
       short N_b of bond at t_body  (duration D_b)
       long N_w2 of bond at t_wing2 (duration D_w2)
       constraint: N_w1 · D_w1 + N_w2 · D_w2 = N_b · D_b
   PnL drivers (after parallel shift cancels):
       (i) curvature change: body up vs wings = profit if short body
       (ii) slope change differential between wings
       (iii) carry differential across the three legs
   payoff vs change in body-yield (parallel shift hedged):
       PnL
        ^
        |     long butterfly profits when body
        |     yields rise relative to wings
        |          /
        |         /
        +--------*-------------> body-yield change relative to wings
                /
               /
        long butterfly loses when body falls relative to wings
```

## Definition

A **butterfly** is a three-tenor portfolio combining a long wing position, a short (or long) body position, and a second long wing position at three distinct curve tenors. The "long butterfly" convention buys the wings and sells the body; the "short butterfly" reverses. The trade's name comes from the visual that the body yield deviates from the line connecting the two wing yields. **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

A **curve-steepener trade** is long the short-tenor bond + short the long-tenor bond (or the mirror "curve-flattener" trade). The steepener profits if the curve slope rises (long tenor rises relative to short tenor); the flattener profits if the slope falls. **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

The **duration-neutral construction** of a curve trade chooses leg notionals so that the portfolio's aggregate dollar duration (sum of `Notional · ModifiedDuration · Price / 100` across legs) is zero. A duration-neutral portfolio has zero first-order PnL under a parallel curve shift. **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

The **dollar duration (DV01)** of a bond is the dollar PnL per one-basis-point parallel shift in its yield. Tuckman uses DV01 as the basic unit for sizing curve trades; the analogue in CFA L1 is modified duration scaled to dollars. **Source:** CFA L1 Curriculum (2022) Vol.5/pp.275-310.

The **level / slope / curvature factor basis** (Tuckman Ch.6) is the empirical PCA-derived three-factor decomposition of curve moves: the first factor (level) explains the bulk of variance and is parallel-shift-like; the second factor (slope) tilts short vs long ends in opposite directions; the third factor (curvature) bends the middle relative to the ends. A duration-neutral butterfly is by construction insulated from the level factor and captures the slope + curvature factors. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

## Mathematical Reasoning

For a butterfly with leg notionals `N_w1, N_b, N_w2` and dollar-durations `D_w1, D_b, D_w2`, the duration-neutral constraint is `N_w1 · D_w1 + N_w2 · D_w2 = N_b · D_b`. This single linear constraint leaves a 2-parameter family of trades; practitioners typically fix one wing's notional and an "ear-balance" ratio (e.g. `N_w1 · D_w1 = N_w2 · D_w2` for a balanced butterfly) to pin down the remaining notionals. **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

The PnL of a duration-neutral curve trade under a non-parallel shift is approximately the inner product of the leg-level dollar durations with the per-tenor yield change vector. For a butterfly with parallel-shift sensitivity hedged, the leading PnL term is proportional to the curvature change `Δy_b − (Δy_w1 + Δy_w2) / 2` weighted by the body-leg DV01. **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

The connection to key-rate / partial DV01 from [`fi-key-rate-and-partial-duration.md`](./fi-key-rate-and-partial-duration.md#mathematical-reasoning) is direct: a butterfly trade has a specific key-rate-DV01 profile concentrated at the three tenors involved; the partial-DV01 sum (across tenors) equals the parallel-shift DV01 hedged to zero by construction. Curve-trade construction is therefore a sub-problem of more general key-rate hedging. **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

The L1 duration / convexity framework from [`fi-duration-and-convexity.md`](./fi-duration-and-convexity.md#mathematical-reasoning) handles parallel shifts only; curve trades require the multi-factor framework Tuckman develops at L2 depth. The CFA L1 single-factor framework captures the level-factor PnL of a curve trade as approximately zero (by construction) and silently absorbs the slope-and-curvature PnL into "model error" — Tuckman's multi-factor treatment makes the slope and curvature contributions explicit. **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169; CFA L1 Curriculum (2022) Vol.5/pp.275-310.

The convexity dimension of a butterfly is worth noting: the wings (long-duration legs) typically carry more convexity than the short-duration body, so a long butterfly is also long convexity. This means even when the curvature factor moves orthogonally to the slope factor, the trade can still gain from large parallel moves via the convexity differential — Tuckman calls this the "free option" of a long butterfly and warns that it is paid for via lower carry. **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

The empirical structure of curve moves — that the first PCA factor (level) holds the dominant share of variance, the second (slope) holds a large minority share, and the third (curvature) holds a small minority share — is developed in [`fi-term-structure-factor-models.md`](./fi-term-structure-factor-models.md#mathematical-reasoning) at the PCA-estimation depth Tuckman provides. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

## See Also

- [`fi-key-rate-and-partial-duration.md`](fi-key-rate-and-partial-duration.md) — partial DV01 framework that generalizes butterfly construction to arbitrary tenor sets
- [`fi-duration-and-convexity.md`](fi-duration-and-convexity.md) — L1 single-factor framework that butterfly trades extend
- [`fi-term-structure-factor-models.md`](fi-term-structure-factor-models.md) — PCA decomposition of curve moves underlying the slope and curvature factors
- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — spot-rate curve from which the per-tenor yields and durations are computed

## Escalate to Raw When

Open Tuckman & Serrat 3e Ch.5 (Multi-Factor Risk Metrics and Hedges)
directly when any of the criteria below applies.
**Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

- Specific notional-sizing recipes for a particular butterfly
  trade (e.g. a 2-10-30 butterfly at a specific date) require
  the dated dollar-durations and the chosen ear-balance ratio
  Tuckman illustrates.
  **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.
- A dated historical curve-trade scenario (e.g. a flattener
  during a hiking cycle) requires the empirical yield-curve
  series and per-leg PnL attribution Tuckman provides.
  **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.
- The convexity-differential carry analysis on a long butterfly
  versus carry-on-the-curve is required at trade-desk detail —
  out of CFA L1 and L2 scope.
  **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.
- Cross-currency curve trades (a 2-10-30 butterfly in USD
  hedged against the equivalent in EUR) are in scope — out
  of this card's single-currency framing.
  **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.
