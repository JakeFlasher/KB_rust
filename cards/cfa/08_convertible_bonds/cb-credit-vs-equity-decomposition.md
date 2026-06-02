---
schema_version: "cacg.v0"
id: "cb-credit-vs-equity-decomposition"
title: "Credit vs Equity Decomposition"
reading_id: "08_convertible_bonds"
summary: "Credit vs Equity Decomposition — placeholder summary                            "
tags: ["convertible-bonds", "credit-equity"]
citations:
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p031:0029"
    chunk_hash: "90a96317aa678f45db3f88768e5f61b2e2cf6259b9b9ab60a06673180992db70"
    page_range: [31, 32]
    quote: "The fixed-income value (investment value) will rise or fall in accordance to changes in either interest rates or credit-quality ratings."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p062:0070"
    chunk_hash: "3976cb6629efe93e473fdc974d68319b8dfdc08ae1762cedba01fb21283c4c80"
    page_range: [62, 63]
    quote: "For high share prices, the convertible’s price converges to parity and the convertible bond adopts equity-like behavior."
    edge_type: "supports"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p102:0122"
    chunk_hash: "60ff35e0a1b878067742233d3b9a780d017c970cd6f19d549f62a1df576d872a"
    page_range: [102, 103]
    quote: "The hazard function is of particular interest in default modeling because of its link to conditional default probabilities, which is similar to the link we saw in the discrete-time case."
    edge_type: "supports"
card_hash: "3419f89668d9076fce2446d030716585bf65ac6516177337beb2adb2cb39cb0d"
---
# Credit vs Equity Decomposition

## Intuition

A convertible bond's market price can be split into two complementary legs:
the **credit-risky bond leg** that captures the present value of the
straight-bond cashflows (and their default-discounted recovery payment),
and the **equity-option leg** that captures the time-value plus intrinsic
value of the embedded call on the issuer's shares. The practitioner ratio
"credit / equity %" measures how much of the convertible's price is
explained by which leg in the current regime.
**Source:** Calamos (2003) §7 pp.130-170; DeSpiegeleer et al. (2014) §3.2
pp.65-78.

```
CB price V(t)
       ^
       |  equity-leg dominant ─────────── 100%
       |        :  :  :  :  :  :
       |       /                  ___ V(t)
       |      /            ______/
       |     /        ____/
       |    /    ____/   credit-leg dominant
       | __/  __/
       |/    /             0%
       +----------------------------> share price S
```

## Definition

For a non-callable, non-puttable, European-conversion convertible the
canonical bond-plus-call identity (see the
[payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#definition))
gives

    V(S, t)  =  B(t)  +  q · c(S, K_c, σ, r, δ, T-t)

The two practitioner ratios are defined below; `credit_pct + equity_pct =
1` by construction. **Source:** Calamos (2003) §7 pp.130-145.

    credit_pct(t)  :=  B(t) / V(S, t)
    equity_pct(t)  :=  q · c(S, K_c, σ, r, δ, T-t) / V(S, t)

For real-world callable / credit-risky issues the identity is approximate,
so the ratios are defined against a **model price** `V_model(S, t)` from a
credit-aware tree or PDE: `credit_pct = B_model(t) / V_model(t)` and
`equity_pct = (V_model(t) − B_model(t)) / V_model(t)`. The tree separately
tracks default-survival and default-event branches, so `B_model(t)`
incorporates both the survival cashflow PV and the recovery PV (see the
[bond-floor card](./cb-bond-floor-investment-value.md#definition)).
**Source:** Lando (2004) §3-§4 pp.60-90; DeSpiegeleer et al. (2014) §3.6
pp.95-110.

## Mathematical Reasoning

The split is well-defined whenever the embedded option leg is non-negative
(`q · c ≥ 0`) and the bond leg is bounded below by recovery PV
(`B(t) ≥ R · F · D_rf(t, τ_avg)`); together these guarantee `0 ≤ credit_pct
≤ 1`. **Source:** Calamos (2003) §7 pp.130-150.

Asymptotic regimes match the parity asymptotics in delta-ratio
terms; see
[`cb-parity-and-conversion-value.md`](./cb-parity-and-conversion-value.md#mathematical-reasoning).

- `S → ∞`: `c → S − K_c · e^(-(r-δ)τ)` (deep-ITM call); the option leg
  grows without bound while `B(t)` stays bounded by riskless-discounted
  nominal. Therefore `credit_pct → 0` and `equity_pct → 1` in the
  delta-ratio limit. **Source:** DeSpiegeleer et al. (2014) §3.4 pp.78-95.
- `S → 0` with stable credit: `c → 0` and `B(t)` dominates, so
  `credit_pct → 1` and `equity_pct → 0`. With **stressed credit**
  (the double-signed-gamma regime; see the
  [bond-floor card](./cb-bond-floor-investment-value.md#mathematical-reasoning)),
  `B(t)` itself collapses toward recovery PV; the split becomes
  ill-conditioned because both legs shrink simultaneously. **Source:**
  DeSpiegeleer et al. (2014) §2.4 pp.45-58; Lando (2004) §4 pp.75-90.
- `S ≈ K_c` (balanced regime): typical `equity_pct` lies near one-half;
  this is the calmest part of the vol surface and the "balanced CB"
  classification practitioners care about. **Source:** Calamos (2003) §7
  pp.150-170.

The ratio `credit_pct` is **not** the issuer's credit-spread weight in a
pricing tree; it is a derived statistic. Specifically, `credit_pct` falls
when the issuer's spread tightens (because `B(t)` rises) but it also falls
when `S` rises (because the equity leg grows). Practitioner reports often
also quote a **delta-adjusted credit / equity %** that strips out the
delta-driven shift. **Source:** Calamos (2003) §7 pp.155-170.

## See Also

- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — the underlying identity
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — `B(t)` definition and stress regime
- [`cb-parity-and-conversion-value.md`](cb-parity-and-conversion-value.md) — asymptotic context

## Escalate to Raw When

Open Calamos §7 pp.130-170 directly for the practitioner's delta-adjusted
credit-equity statistics and the trader-quoted regime classification.
**Source:** Calamos (2003) §7 pp.130-170.

Open Lando §3-§4 pp.60-90 when the credit-risky `B_model(t)` requires a
specific hazard-rate model (homogeneous Poisson, doubly-stochastic, or
structural). **Source:** Lando (2004) §3-§4 pp.60-90.

Open DeSpiegeleer §3.2-§3.4 pp.65-95 when calibrating the tree-implied
credit / equity split against a listed convertible's market quote.
**Source:** DeSpiegeleer et al. (2014) §3.2-§3.4 pp.65-95.
