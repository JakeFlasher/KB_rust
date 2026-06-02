---
schema_version: "cacg.v0"
id: "pa-active-risk-tracking-error-ex-ante-vs-ex-post"
title: "Active Risk: Ex-Ante vs Ex-Post Tracking Error"
reading_id: "15_performance_and_attribution"
summary: "Tracking error (relative/active risk) is the standard deviation of excess return. It splits into ex-ante (predicted by a factor model on today's positions) and ex-post (realised from a return history); the two differ in meaning and are the denominator of the Information Ratio."
tags: ["tracking-error", "active-risk", "information-ratio"]
citations:
  - source_id: "pa_bacon_2022_rapm"
    chunk_id: "pa_bacon_2022_rapm:p082:0073"
    chunk_hash: "c35a0b08f62e46ed35a3c8d0aca8bd7b2f8ebea9fc2a428cdbfb9d41710bc93e"
    page_range: [83, 83]
    quote: "The variability of excess return calculated using standard deviation is called tracking error, tracking risk, relative risk or active risk."
    edge_type: "defines"
  - source_id: "pa_connor_goldberg_korajczyk_2010"
    chunk_id: "pa_connor_goldberg_korajczyk_2010:p043:0041"
    chunk_hash: "4697ec94ddd23880a0131f63d528ae21dc234150aa627d7af8ae63f57078ef01"
    page_range: [44, 44]
    quote: "Each manager can restate the portfolio management problem from his perspective as active return–active risk optimization of his subportfolio."
    edge_type: "supports"
card_hash: "bcde7320898e72da4a5bd8d4df29811447e758b2c769b366a501db5d118fc01d"
---
# Active Risk: Ex-Ante vs Ex-Post Tracking Error

## Intuition

An active manager is paid to differ from the benchmark, so the relevant risk is
not how much the portfolio bounces around in absolute terms but how much it
*drifts away from the benchmark*. Tracking error captures exactly that: the
variability of the excess return (portfolio minus benchmark). Two managers can
both beat the benchmark by 2% a year, yet one does it smoothly month after month
and the other lurches through wins and losses — the smooth one has the lower
tracking error and is delivering that excess return more reliably.

Crucially the same word names two different numbers. *Ex-ante* tracking error is a
forecast: feed today's portfolio positions into a factor risk model and ask how
volatile the excess return *should* be going forward. *Ex-post* tracking error is
a fact: take the realised history of excess returns and compute its standard
deviation. One looks forward from a snapshot; the other looks backward from a
track record. They rarely agree, and confusing them silently corrupts every ratio
built on top of them.

**Source:** Bacon (2022) §"Relative Risk"/"Tracking Error" printed pp.61-62 (PDF pp.83-84)

## Definition

**Tracking error** (also called tracking risk, relative risk, or active risk) is
the standard deviation of the excess return of the portfolio against its
benchmark. It is a *relative* risk measure: unlike absolute measures that compute
portfolio and benchmark risk separately and then compare, tracking error operates
directly on the excess-return series.

It comes in two forms that must always be labelled:

- **Ex-post (realised) tracking error** — the standard deviation of a *historical*
  series of excess returns (arithmetic excess `a_i = r_i - b_i`, or geometric excess
  `g_i = (1+r_i)/(1+b_i) - 1`). It is a property of what already happened.
- **Ex-ante (predicted) tracking error** — a *forecast* produced by a factor risk
  model applied to the portfolio's current active positions. Because the
  calculation methods and meaning are quite different from the ex-post figure,
  Bacon stresses it is essential to label which one is in use.

In the factor-model view, the active portfolio is the difference between the
managed weights and the benchmark weights (`w_A = w - w_B`), and active risk is the
uncertainty of that active (demeaned) return. Predicted active variance therefore
decomposes into a **common-factor** component (active factor exposures run through
the factor covariance matrix) plus a **specific (idiosyncratic)** component — the
same factor-plus-specific split that governs total risk, applied to active weights.

**Sources:** Bacon (2022) §"Relative Risk"/"Tracking Error" printed pp.61-62 (PDF
pp.83-84); Connor, Goldberg & Korajczyk (2010) §1.3.5-1.3.6 printed pp.19-23 (PDF
pp.44-48)

## Mathematical Reasoning

Let `a_i = r_i - b_i` be the arithmetic excess return in period `i`, with mean `a_bar`.
Ex-post tracking error is the dispersion of that excess-return series:

```
                ____________________
               /  i=n
  sigma_A =    / sum  (a_i - a_bar)^2
            \ /  i=1
             V  -------------------
                     n
```

(the geometric form replaces `a_i` with `g_i = (1+r_i)/(1+b_i) - 1`). This is purely
descriptive of a realised history — no model, no forecast.

The **Information Ratio** is the reward-per-unit-of-active-risk gradient with this
sigma_A in the denominator:

```
          annualised excess return        a_tilde
   IR  =  --------------------------  =  -----------
          annualised tracking error      sigma_tilde_A
```

so the *quality* of the IR is only as good as the sigma_tilde_A you feed it. Substituting an
ex-ante sigma_tilde_A for an ex-post one changes what the ratio means, which is why Bacon
warns against mixing them.

**Ex-ante decomposition (factor + specific).** Writing active weights `w_A`,
predicted active variance under a `k`-factor model with factor-beta matrix `B`,
factor covariance `C_f`, and diagonal specific-variance matrix `D` is the sum of a
common-factor term and a specific term:

```
   sigma^2_A(ex-ante)  =  (B' w_A)' C_f (B' w_A)   +   w_A' D w_A
                          +------ common factor ------+   +- specific -+
```

The ex-ante / ex-post divergence as a decision grid:

```
                    |  built from ...      |  answers ...
   -----------------+----------------------+---------------------------
   EX-ANTE  sigma_A |  today's positions   |  how risky is the active
   (predicted)      |  + factor model      |  bet I hold right now?
   -----------------+----------------------+---------------------------
   EX-POST  sigma_A |  realised excess-    |  how variable was the
   (realised)       |  return history      |  active return I delivered?
```

A structural caveat Bacon flags: because ex-ante tracking error reads only a
*current snapshot*, a manager can "window-dress" — cut active bets at the
measurement date — to shrink the forecast sigma_tilde_A and so inflate the apparent IR. The
ex-post figure, drawn from the whole history, is not gameable this way. Bacon
*asserts* this gaming channel rather than proving a formal bound; the card asserts
it at the same level and labels the gap.

One more sign caveat from the IR template: low tracking error is not unconditionally
good. In isolation it measures *consistency*, which can equally mean consistent
underperformance — a tight, persistent shortfall is generally judged worse than an
erratic one.

**Sources:** Bacon (2022) §3 Eq.(3.15)-(3.16), Eq.(3.21) printed pp.62-65 (PDF
pp.84-87); Connor, Goldberg & Korajczyk (2010) §1.3.5-1.3.6 printed pp.19-23 (PDF
pp.44-48)

## Boundary Notes

This card owns the *active-risk denominator* itself — its definition, its ex-ante
vs ex-post forms, and the factor-plus-specific decomposition of predicted active
risk. The full reward/variability ratio template (Sharpe, Information Ratio,
gradient-on-a-plane reading) lives in the variability-ratios card; the
*numerator* fork — arithmetic vs geometric excess return — is its own definitional
card. The mechanics of estimating the factor covariance matrix that powers the
ex-ante forecast belong to the factor-model card, not here.

**Source:** Bacon (2022) §3 printed pp.61-65 (PDF pp.83-87)

## See Also

- [`pa-variability-ratios-sharpe-information.md`](pa-variability-ratios-sharpe-information.md) — the Information Ratio is the Sharpe template with this tracking error as its denominator; that card owns the ratio, this one owns the denominator.
- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — the excess-return numerator that feeds the tracking-error series; arithmetic vs geometric is a definitional fork.
- [`pa-factor-model-types-and-covariance-decomposition.md`](pa-factor-model-types-and-covariance-decomposition.md) — the factor covariance machinery that produces the ex-ante (predicted) tracking error and its factor/specific split.
- [`pa-multifactor-alpha-timing-conditional.md`](pa-multifactor-alpha-timing-conditional.md) — pairs predicted active risk with predicted active return when judging factor bets. Active risk and the Information Ratio also anchor the active-management framing in pm-* portfolio-management cards and the risk-budgeting view in rm-* risk cards.

## Escalate to Raw When

- You need the worked tracking-error / Information-Ratio numeric examples (Bacon
  Exhibit 3.4 and surrounding tables, pp.65-66) — worked arithmetic is out of
  scope here per the no-worked-calculations rule.
- You must reconcile a *specific* commercial factor risk model's ex-ante tracking
  error against your realised ex-post figure (model horizon, half-life,
  estimation-universe mismatches) — Connor, Goldberg & Korajczyk (2010) Ch. 3-6.
- You need the exact marginal-contribution-to-active-risk algebra (MCR, the
  value-additivity of marginal contributions) for risk budgeting — Connor,
  Goldberg & Korajczyk (2010) §1.3.6 pp.20-23.
- You need the annualisation and disclosure conventions (data frequency, time
  period, population vs sample, simple vs continuously compounded) required before
  publishing a live tracking-error or Information-Ratio number — Bacon (2022) p.64.
