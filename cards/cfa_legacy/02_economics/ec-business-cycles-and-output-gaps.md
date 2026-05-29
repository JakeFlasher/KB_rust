---
schema_version: "cacg.v0"
id: "ec-business-cycles-and-output-gaps"
title: "Business Cycles and Output Gaps (CFA L1 R11)"
reading_id: "02_economics"
summary: "CFA L1 R11 business-cycle phases (expansion/peak/contraction/trough), leading/coincident/lagging indicator taxonomy, and the output gap (Y − Y*) / Y* diagnostic. Output gap estimation is model-dependent because Y* is unobservable."
tags: ["economics", "business-cycles"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p777:1080"
    chunk_hash: "862113bb998c4b1abe20953d428033ddc0df96fbe9449cf463fad76fad376ef3"
    page_range: [777, 778]
    quote: "Finally, cycles are recurrent; they happen again and again over time but not in a periodic way; they do not all have the exact same intensity and duration."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p782:1086"
    chunk_hash: "c3653e1572293c40194fec4434d0b83a73572d571a0138dd4cf4a03e9560922a"
    page_range: [782, 783]
    quote: "Indeed, as we will see later, the equity market is classified as a leading indicator of the economy."
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p207:0284"
    chunk_hash: "b00324a240d63fe6616fe048833b74d8bdcfc67ff0652ac238385c572c3f9480"
    page_range: [207, 207]
    quote: "Understanding the causes of aggregate fluctuations is a central goal of macroeconomics."
    edge_type: "supports"
card_hash: "02f7fd0ed8b1a307b876422f5458e757d46a578158bfd743139290564c3841d7"
---
# Business Cycles and Output Gaps (CFA L1 R11)

## Intuition

The **business cycle** is the recurring pattern of expansions and contractions in real GDP around the long-run potential-output trend. The CFA L1 R11 curriculum identifies four canonical phases — **expansion** (GDP rising), **peak** (GDP at local maximum), **contraction / recession** (GDP falling), **trough** (GDP at local minimum) — and trains students to recognize which macro variables lead, coincide with, or lag the cycle. The cycle is not a deterministic clock but a probabilistic rhythm: cycles vary in duration (post-WWII US cycles have ranged roughly from one to ten years peak-to-peak) and amplitude (modest mid-cycle contractions vs deep recessions), and the dating of phase transitions is judgmental rather than algorithmic. **Source:** CFA Institute (2022) Vol.2 pp.199-241.

```
   business-cycle phases + indicator timing

   real GDP
   ^
   |                  PEAK                   PEAK
   |                  /\                     /\
   |          EXP    /  \    CONTRACTION    /  \
   |         /      /    \                 /    \
   |        /                \              /
   |       /                  \            /
   |      / TROUGH             \          / TROUGH
   |     /                       \________/
   |    /
   +-----------------------------------------> t

   leading indicators turn  coincident indicators turn   lagging indicators turn
   BEFORE the cycle peak/   WITH the peak/trough         AFTER the peak/trough
   trough (predictive)      (real-time diagnostic)       (confirmatory)

   examples (CFA L1):
   leading:    new orders, building permits, stock returns, term spread
   coincident: industrial production, employment, real personal income
   lagging:    unemployment rate, CPI inflation, prime rate
```

The **output gap** `(Y − Y*) / Y*` is the single-number diagnostic that summarizes where the economy sits in the cycle: positive gap means above potential (expansionary, often associated with rising inflation); negative gap means below potential (recessionary, often associated with falling inflation and rising unemployment). Output-gap estimation is operationally hard because `Y*` is unobservable (it is constructed from production-function decompositions, statistical filters like HP, or judgmental estimates from central-bank staff), so output-gap diagnostics are model-dependent and revised heavily as more data arrives. **Source:** CFA Institute (2022) Vol.2 pp.199-241.

## Definition

The **business-cycle phases**, in CFA L1 vocabulary. **Source:** CFA Institute (2022) Vol.2 pp.199-241.

```
expansion:    Y growing; unemployment falling; inflation often rising;
              consumer and business confidence rising
peak:         Y at local maximum; growth rate transitioning from + to −
contraction:  Y falling for at least two consecutive quarters (US convention);
              unemployment rising; inflation often easing
trough:       Y at local minimum; growth rate transitioning from − to +
```

The **recession dating** convention in the US is the NBER's: a recession is a "significant decline in economic activity spread across the economy, lasting more than a few months, normally visible in real GDP, real income, employment, industrial production, and wholesale-retail sales." NBER uses judgment, not a mechanical rule like "two quarters of GDP decline" (which is the rule-of-thumb often taught). **Source:** CFA Institute (2022) Vol.2 pp.199-241.

The **output gap** is computed as. **Source:** CFA Institute (2022) Vol.2 pp.199-241.

```
output_gap  =  (Y − Y*) / Y*                                   [%]
            >  0     :  economy above potential (overheating)
            =  0     :  economy at potential (full employment / NAIRU)
            <  0     :  economy below potential (slack; recession or stagnation)
```

The CFA L1 curriculum also teaches the related **unemployment-gap** diagnostic via **Okun's law**: the relationship between the output gap and the unemployment-rate deviation from NAIRU, often stated as a stylized fact rather than a structural equation. **Source:** CFA Institute (2022) Vol.2 pp.199-241.

The **leading / coincident / lagging indicator taxonomy** (CFA L1 R11 emphasis). **Source:** CFA Institute (2022) Vol.2 pp.199-241.

```
leading:    new orders for capital goods, building permits, stock returns,
            term spread (10-year minus 3-month treasury), consumer sentiment
coincident: industrial production index, employment, real personal income
            excluding transfers, manufacturing and trade sales
lagging:    unemployment rate, CPI inflation rate, prime rate, average
            duration of unemployment, ratio of consumer credit to income
```

**Source:** CFA Institute (2022) Vol.2 pp.199-241.

## Mathematical Reasoning

The **output-gap estimation challenge** is the core analytical issue. `Y*` is unobservable; three standard approaches: (a) **production-function** decomposition uses an aggregate production function `Y = A · F(K, L)` with `Y*` evaluated at full-employment labor input `L*` and trend total-factor-productivity `A*`; (b) **statistical filters** (Hodrick-Prescott, Baxter-King band-pass, Beveridge-Nelson) extract a smooth trend `Y*_t` from observed `Y_t` and define the gap as the cyclical residual; (c) **judgmental** estimates from central-bank staff combine production-function and filter methods with policy judgment about NAIRU and capacity utilization. All three approaches share the issue that `Y*` estimates are heavily revised as new data arrive — the real-time output gap can differ substantially from the ex-post estimate. **Source:** CFA Institute (2022) Vol.2 pp.199-241.

The **Okun's-law relationship** is a stylized fact taught at L1: a one-percentage-point decline in the unemployment-rate-vs-NAIRU gap is associated with an output-gap increase on the order of two-to-three percentage points. The L1 exam treats Okun's law qualitatively (direction + approximate magnitude); the structural micro-foundation — why labor input changes are amplified into output changes by hours-worked adjustments, capacity utilization, and labor-hoarding — sits in the Romer-anchored RBC framework (sibling [`ec-real-business-cycle-theory`](./ec-real-business-cycle-theory.md)). The Romer treatment derives the relationship from the firm's labor-demand FOC under intertemporal substitution; the L1 treatment states the empirical regularity without derivation. **Source:** CFA Institute (2022) Vol.2 pp.199-241; supporting Romer (2019) Ch.5 pp.188-237.

The **practitioner's leading-indicator framework** combines multiple individual leading indicators into a single composite leading-economic-index (LEI). The composite's construction weights each component by historical timing-and-amplitude correlations with NBER-dated turning points; the L1 exam treats the LEI methodology at the qualitative level (knowing the canonical components and their cycle-timing properties), not the weighting algorithm. Practitioners building real-time recession-probability indicators (Chauvet-Piger, Sahm rule) use the same indicators in econometric specifications; the formal model class is graduate-econometrics out of v10 scope. **Source:** CFA Institute (2022) Vol.2 pp.199-241.

## See Also

- [`ec-aggregate-supply-demand-mechanics`](./ec-aggregate-supply-demand-mechanics.md) — AS-AD framework that explains the inflation-output co-movements across cycle phases
- [`ec-real-business-cycle-theory`](./ec-real-business-cycle-theory.md) — Romer-anchored RBC DSGE micro-foundation of business-cycle dynamics
- [`ec-monetary-policy-and-inflation`](./ec-monetary-policy-and-inflation.md) — Taylor-rule monetary-policy response to output-gap deviations
- [`ec-commodity-price-forecasting`](./ec-commodity-price-forecasting.md) — commodity-cycle relationship to broader business cycle

## Escalate to Raw When

The DSGE structural decomposition of cyclical dynamics into productivity shocks, monetary shocks, and demand shocks (Smets-Wouters DSGE estimation, Christiano-Eichenbaum-Evans VAR identification, Bayesian DSGE) is in the Romer-anchored framework (sibling [`ec-real-business-cycle-theory`](./ec-real-business-cycle-theory.md)) and graduate-research literature out of v10 scope. The output-gap estimation literature (Edge-Rudd Bayesian-output-gap models, Laubach-Williams natural-rate estimation, Stock-Watson factor-model output gap) is mainstream macroeconometrics out of v10 scope. The recession-probability nowcasting literature (state-space recession-probability models, Chauvet-Piger smoothed-recession probability) is also out of scope. **Source:** Romer (2019) Ch.5 pp.188-237; CFA Institute (2022) Vol.2 pp.199-241.
