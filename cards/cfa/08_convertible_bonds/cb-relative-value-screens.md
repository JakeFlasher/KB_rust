---
schema_version: "cacg.v0"
id: "cb-relative-value-screens"
title: "Convertible Relative-Value Screens"
reading_id: "08_convertible_bonds"
summary: "Convertible Relative-Value Screens — placeholder summary                        "
tags: ["convertible-bonds", "relative-value"]
citations:
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p032:0030"
    chunk_hash: "fbcfebc44077d76bbbbe02e2ddb1a1cc9380cef91bb44e0d4132bebdca18f4dd"
    page_range: [32, 32]
    quote: "The premium above conversion value represents the percentage premium that the convertible is trading above its equity value component. The higher the conversion premium, the lower the equity sensitivity, and the lower the conversion premium, the more equity sensitive the issue."
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p040:0040"
    chunk_hash: "253a241294eac963c3c70a3819667be19863fd64b7fa17a007ada6aed4a2dbb4"
    page_range: [40, 41]
    quote: "Under-valued convertible—Since the hedged convertible position is still “long-volatility,” the arbitrageur seeks issues that are undervalued or trading at implied volatility levels below the expected norm."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p062:0070"
    chunk_hash: "3976cb6629efe93e473fdc974d68319b8dfdc08ae1762cedba01fb21283c4c80"
    page_range: [62, 63]
    quote: "For high share prices, the convertible’s price converges to parity and the convertible bond adopts equity-like behavior."
    edge_type: "supports"
card_hash: "4c07071390ee25d4a34741ab5bfb0552874d91301483df4f7e91c33fa2d73219"
---
# Convertible Relative-Value Screens

## Intuition

Convertible-arbitrage funds source ideas by running cross-issuer
**relative-value screens** that rank convertibles by deviation
from a fundamental fair value. The four practitioner-quoted
screens are: **adjusted conversion premium** (how rich is the
embedded call relative to peers?), **credit/equity ratio
comparison** (is this CB classified into the same regime as its
peers, or is it an outlier?), **implied-vs-realized vol gap** (is
the market pricing more vol than the underlying historically
delivers?), and **parity-spread to bond-floor metric** (how
asymmetric is the upside-vs-downside exposure?). Screens are
**rankings** not absolute valuations — the goal is to find the
cheapest-vs-richest convertible within a peer group, not a
single-issue price target.
**Source:** Calamos (2003) §7-§11 pp.130-300.

```
relative-value-screen workflow (practitioner playbook):

   universe (all listed CBs in scope)
        |
        v
   filter: lint-pass, deliverable-ready, peer-group-classifier
        |
        v
   evaluate screens: adj-CP, credit/equity %, IV-RV gap, parity-spread
        |
        v
   rank cross-issuer
        |
        v
   shortlist: outliers (typically top + bottom decile per screen)
        |
        v
   case-by-case fundamental review (issuer credit, prospectus, flow)
```

## Definition

The four practitioner-quoted relative-value screens. **Source:**
Calamos (2003) §7-§11 pp.130-300; DeSpiegeleer et al. (2014)
§3.2-§3.4 pp.65-95.

- **Adjusted conversion premium**: the
  [conversion-premium card's](./cb-conversion-premium.md#definition)
  ratio `(V(t) − q · S(t)) / (q · S(t))` adjusted for the
  bond's dividend protection level, the call schedule, and the
  remaining time to maturity. Cross-issuer ranking flags the
  cheapest embedded call. **Source:** Calamos (2003) §7
  pp.150-170.
- **Credit/equity ratio comparison**: the
  [credit-equity-decomposition card's](./cb-credit-vs-equity-decomposition.md#definition)
  `credit_pct` / `equity_pct` ratio compared against a peer-group
  mean. Outliers are either misclassified (peer-group mismatch)
  or genuinely cheap/rich on a regime basis. **Source:**
  Calamos (2003) §7 pp.155-170; DeSpiegeleer et al. (2014) §3.4
  pp.78-95.
- **Implied-vs-realized vol gap**: the difference between the
  vol input used to mark the convertible (typically a slice of
  the listed-equity-option implied-vol surface; see the
  [volatility-surface card](./cb-volatility-surface.md#definition))
  and the underlying share's recent realized vol. A large
  positive gap suggests the convertible is rich on vega; a
  negative gap suggests the convertible is cheap.
  **Source:** Calamos (2003) §11 pp.260-300.
- **Parity-spread metric**: the gap between the convertible's
  market price and its parity `q · S(t)`, expressed in basis
  points of face. Combined with the bond floor `B(t)`, this
  metric is the practitioner's quick-look at upside-vs-downside
  asymmetry. **Source:** Philips (1997) §3 pp.90-130.

The screens are not orthogonal — adjusted conversion premium and
implied-vs-realized vol gap are highly correlated, and the
credit/equity ratio depends on parity, the bond floor, and the
embedded-call vega via the
[bond-floor card](./cb-bond-floor-investment-value.md#definition)
and the
[Greeks card](./cb-greeks-delta-gamma-vega.md#definition). The
practitioner's mental model is to use the four screens jointly,
not as independent indicators. **Source:** Calamos (2003) §7-§11
pp.130-300.

## Mathematical Reasoning

The **peer-group classifier** that precedes the screens is the
practitioner's key methodological choice: peer groups are defined by
credit rating, sector, geography, and convertible-structure family
(optional vs mandatory, callable vs non-callable), and a convertible
is then ranked **within** its peer group rather than across the
universe — this controls for issuer-quality and structure-class
effects that would otherwise swamp the screen signal. **Source:**
Calamos (2003) §7 pp.130-170.

The **z-score across the peer group** is the canonical
ranking. **Source:** Calamos (2003) §7-§11 pp.130-300.

```
For each screen X, peer group P:

  z_i := (X_i − μ_P(X)) / σ_P(X)

  μ_P(X) := mean of X across i in P
  σ_P(X) := standard deviation of X across i in P

Outliers: |z_i| > 1.5 (typical practitioner threshold)
```

The **adjusted conversion premium** correction terms are qualitative;
the practitioner adjusts the raw conversion premium by the three
corrections enumerated below. **Source:** Calamos (2003) §7
pp.150-170.

- a **dividend-protection** correction (a fully-protected
  convertible is intrinsically cheaper because the holder
  retains the dividend stream on the underlying). **Source:**
  Calamos (2003) §7 pp.150-170.
- a **time-to-maturity** correction (longer-dated convertibles
  carry higher conversion premium for the same intrinsic
  value, because the embedded call has more time value).
  **Source:** Calamos (2003) §7 pp.150-170.
- a **call-schedule** correction (callable convertibles
  trade with discounted conversion premium because the
  issuer's call caps the upside). **Source:** Calamos (2003)
  §7 pp.150-170.

These corrections are **comparable across issuers** in the same
peer group; the screen ranks the residual after correcting
for these structural differences. **Source:** Calamos (2003)
§7 pp.150-170.

The **implied-vs-realized vol gap** screen builds on the
volatility-surface fitting choice from the
[volatility-surface card](./cb-volatility-surface.md#mathematical-reasoning):
the practitioner fits the implied vol surface from listed equity
options, slices it at `(K_c, T_CB)`, and compares to the underlying's
trailing 30-day or 90-day realized vol. **Source:** Calamos (2003) §11
pp.260-300.

```
IV-RV gap (single CB):

  gap_i := σ_listed(K_c, T_CB) − σ_realized_90d

Cross-issuer ranking on z_i := (gap_i − μ_P(gap)) / σ_P(gap).

Positive z_i → convertible is rich on vega; negative z_i → cheap.
```

The **parity-spread to bond-floor metric** captures the dual-
floor dynamic from the
[bond-floor card](./cb-bond-floor-investment-value.md#mathematical-reasoning)
and the
[parity card](./cb-parity-and-conversion-value.md#mathematical-reasoning);
asymmetric upside vs downside is captured by the sign of
`parity − B(t)` — positive means the equity-side floor is binding,
negative means the bond-side floor is binding. **Source:** Philips
(1997) §3 pp.90-130.

Asymptotic regimes (cases below). **Source:** Calamos (2003) §7-§11
pp.140-300.

- **Crisis regime**: vol gaps widen because realized vol
  spikes; conversion premiums collapse as parity falls toward
  recovery PV; credit/equity ratios shift toward 100%
  credit-dominated. Cross-issuer screens become noisy. **Source:**
  Calamos (2003) §11 pp.290-300.
- **Stable bull regime**: convergence — adjusted conversion
  premiums and IV-RV gaps narrow across issuers; credit/equity
  ratios shift toward equity-dominated; parity-spread metrics
  shrink. Cross-issuer screen signals also narrow. **Source:**
  Calamos (2003) §11 pp.260-280.
- **Sector-specific dispersion**: peer-group classification
  matters because sector-specific events (oil, biotech) can
  produce screen z-scores that are not informative about
  relative value. **Source:** Calamos (2003) §7 pp.140-160.

## See Also

- [`cb-conversion-premium.md`](cb-conversion-premium.md) — the conversion premium ranked by the first screen
- [`cb-credit-vs-equity-decomposition.md`](cb-credit-vs-equity-decomposition.md) — the credit/equity ratio used by the second screen
- [`cb-volatility-surface.md`](cb-volatility-surface.md) — the implied-vol input for the third screen
- [`cb-arbitrage-strategy.md`](cb-arbitrage-strategy.md) — the strategy that consumes screen-sourced ideas

## Escalate to Raw When

Open Calamos §7-§11 pp.130-300 directly for the practitioner's
relative-value playbook, peer-group taxonomy, and the joint use
of the four screens. **Source:** Calamos (2003) §7-§11 pp.130-300.

Open DeSpiegeleer §3.2-§3.4 pp.65-95 for the formal credit/equity
decomposition and the regime classifier that the second screen
encodes. **Source:** DeSpiegeleer et al. (2014) §3.2-§3.4
pp.65-95.

Open Philips §3 pp.90-130 for the historical evolution of
relative-value screens across market regimes and the parity-
spread metric's methodological origins. **Source:** Philips (1997)
§3 pp.90-130.
