---
schema_version: "cacg.v0"
id: "pa-m2-risk-adjusted-return-transform"
title: "M-Squared: The Risk-Adjusted Return Transform"
reading_id: "15_performance_and_attribution"
summary: "Modigliani M2 rescales a portfolio's return along its Sharpe line to the benchmark's risk, producing a return metric (not a ratio) that preserves Sharpe rank while making risk-adjusted outperformance directly comparable; it generalises to MS2, adjusted M2, GH1/GH2, and differential return."
tags: ["risk-adjusted-return", "m-squared", "sharpe-line"]
citations:
  - source_id: "pa_bacon_2022_rapm"
    chunk_id: "pa_bacon_2022_rapm:p228:0198"
    chunk_hash: "c74c43b23d98397c1a7d1f39a849e2436e1d02eff73800fa4a1145b89db80168"
    page_range: [229, 229]
    quote: "a genuinely risk-adjusted return, extremely useful for comparing portfolios with different levels of risk."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p110:0109"
    chunk_hash: "b51038e2c672032e687d475aa369a8af3f9afb574738306a3c1956c73d6b5bcd"
    page_range: [110, 111]
    quote: "Therefore, MM is simply the portfolio return adjusted upward or downward to match the benchmark’s standard deviation."
    edge_type: "supports"
card_hash: "ae5da804484cee621f80fe6add0fe909d7a4fbf54e6a20dbfc66173910c2e575"
---
# M-Squared: The Risk-Adjusted Return Transform

## Intuition

The Sharpe ratio ranks portfolios but cannot tell you *how much* better one is than another: it is a ratio, not a return, so its units (excess return per unit of risk) do not translate into a quantity an investor "fully understands." M2 fixes this by sliding a portfolio along its own Sharpe line until it sits at the benchmark's risk, and reading off the return there. Because every portfolio is re-expressed at one common risk level — the benchmark's — the resulting numbers are returns you can subtract and compare directly. A portfolio that earned a high raw return only by taking more risk than the benchmark gets adjusted *down*; a lower-risk portfolio that still beat the line gets adjusted *up*. This is why a portfolio with a *lower* absolute return can have a *higher* M2.

**Source:** Bacon (2022) §11 (Risk-Adjusted Return / M2) pp.207-208

## Definition

M2 (the Modigliani–Modigliani measure) is the return a portfolio would have earned had it carried the **same risk as the benchmark**, holding its Sharpe ratio fixed. Geometrically (Bacon Figure 11.1): draw the portfolio's Sharpe line from the risk-free rate through the portfolio point; erect a vertical line at the benchmark's variability sigma_b; the intercept is M2. It is named for Leah Modigliani and Franco Modigliani (1997), *not* because anything is squared.

The transform is a pure re-scaling along the Sharpe line, so it **preserves the Sharpe-ratio ordering** of portfolios — M2 and Sharpe always rank the same set identically. Christopherson, Cariño & Ferson (2009) give the same construction as a leverage of the portfolio with cash: the reference portfolio mixes the portfolio return with the risk-free asset in the proportion sigma_b/sigma_p so that its volatility matches the benchmark, and its expected return *is* M2.

**Source:** Bacon (2022) §11 (M2) pp.207-208
**Source:** Christopherson, Cariño & Ferson (2009) §10 (Modigliani-Modigliani measure) pp.110-111

## Mathematical Reasoning

Two algebraically equivalent forms (Bacon Eq. 11.1 and 11.2):

```
   M2 = r + SR * (sigma_b - sigma)              (penalty/reward form)

   M2 = (r - r_F) * (sigma_b / sigma) + r_F      (rescaled-excess form)

   where  SR = (r - r_F)/sigma   (the portfolio Sharpe ratio)
          sigma_b = benchmark variability,  sigma = portfolio variability
```

The two forms are identical: substitute SR = (r - r_F)/sigma into the first and expand. The first form makes the geometry explicit — when sigma > sigma_b the term SR*(sigma_b - sigma) is a **penalty** (return adjusted down); when sigma < sigma_b it is a **reward** — provided SR > 0. Christopherson, Cariño & Ferson derive the same result as the expected return of a cash-levered reference portfolio whose volatility is forced to sigma_b via the leverage factor sigma_b/sigma_p (Eq. 10.5: MM_p = SR_p * sigma_b + E(R_F)).

**Rank preservation.** Because both portfolios are evaluated at the *common* risk sigma_b, M2 = r_F + SR*sigma_b + (the same r_F-baseline), the M2 difference between any two portfolios is sigma_b*(SR_1 - SR_2). The ordering by M2 therefore reproduces the ordering by Sharpe ratio exactly — M2 adds an interpretable scale, not new information about rank.

**Why a return, not a ratio.** Subtracting the benchmark gives an *M2 excess return*, geometric or arithmetic (Bacon Eq. 11.3/11.4):

```
   M2 excess return = (1 + M2)/(1 + b) - 1       (geometric)
   M2 excess return = M2 - b                       (arithmetic)
```

```
        Return                              The Sharpe line slid to sigma_b
          ^                          B'
          |                       .--o   <- M2 for B (intercept at sigma_b)
          |        A           .-'  :
          |        o........-'      :        A: high sigma, high raw return
          |     .-'        :        :           -> adjusted DOWN
          |  .-'           :        :        B: low sigma, lower raw return
       r_F o               :        :           -> adjusted UP, ends ABOVE A
          |                :        :
          +----------------+--------+------------> Risk (variability)
                       sigma_A   sigma_b
```

**Source:** Bacon (2022) §11 (M2, M2 Excess Return) pp.208-209
**Source:** Christopherson, Cariño & Ferson (2009) §10 (Modigliani-Modigliani measure, Eq. 10.5) pp.110-111

**Generalisations (same construction, different denominator).** Bacon shows the horizontal axis can be relabelled with *any* risk measure, simply re-reading the intercept:

- **Differential return** — adjusts the *benchmark* to the portfolio's risk instead of vice-versa (Eq. 11.6); diverges from M2 excess return because the two Sharpe lines diverge over time. Less useful for many-portfolio comparison since each portfolio needs its own adjusted benchmark.
- **GH1 / GH2** (Graham–Harvey) — replace the straight Sharpe line with an efficient frontier of benchmark-plus-cash (GH1) or portfolio-plus-cash (GH2) combinations, then read the gap at common risk.
- **MS2** — substitute downside risk and the Sortino ratio: MS2 = r + Sortino*(sigma_Db - sigma_D) (Eq. 11.7).
- **Adjusted M2 / skew-adjusted M2** — swap in the adjusted (or skew-adjusted) Sharpe ratio as the line's gradient, incorporating skewness and kurtosis; a less desirable third/fourth moment shallows the gradient and lowers the intercept (Eq. 11.8/11.9).

Bacon asserts (without formal proof) that M2 is "a demonstrably better measure than either Sharpe ratio from which it is derived or differential return"; this card states the claim as asserted and does not supply a proof.

**Source:** Bacon (2022) §11 (Differential Return, GH1, GH2, MS2, Adjusted M2) pp.209-215

## Boundary Notes

M2 and the Sharpe ratio both use **total** risk (standard deviation) as the denominator. Christopherson, Cariño & Ferson note that total-risk measures are appropriate for evaluating *entire* portfolios, not individual components within a diversified whole — for component or sub-portfolio appraisal, systematic-risk measures (Treynor, Jensen alpha) are the right tool. The penalty/reward interpretation of M2 also holds only "assuming of course the Sharpe ratio is positive" (Bacon).

**Source:** Christopherson, Cariño & Ferson (2009) §10 (Modigliani-Modigliani measure) pp.114-115
**Source:** Bacon (2022) §11 (M2) pp.208

## See Also

- [`pa-variability-ratios-sharpe-information.md`](pa-variability-ratios-sharpe-information.md) — the Sharpe ratio M2 is built from and whose rank it preserves.
- [`pa-regression-appraisal-jensen-treynor.md`](pa-regression-appraisal-jensen-treynor.md) — systematic-risk appraisal measures, the right tool when M2's total-risk denominator is inappropriate.
- [`pa-partial-moment-ratios-sortino-omega-kappa.md`](pa-partial-moment-ratios-sortino-omega-kappa.md) — the Sortino ratio underlying MS2, the downside-risk M2 variant.
- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — the geometric-vs-arithmetic choice that also governs M2 excess return.

Cross-vertical: this connects to portfolio-management Sharpe/Treynor/information-ratio appraisal (pm-*) and to risk-measure choice (rm-*); GIPS-compliant performance presentation (17 ethics GIPS) governs how such risk-adjusted statistics are disclosed.

## Escalate to Raw When

- You need a **worked numerical example** computing M2, M2 excess return, differential return, MS2, adjusted M2, or skew-adjusted M2 for specific portfolios — see Bacon Tables 11.1-11.3 and Exhibits 11.1-11.2 (pp.209-216) and Christopherson Tables 10.1-10.3 (pp.108-114), deliberately omitted here per the no-worked-arithmetic rule.
- You need the exact **M3 correlation correction** (Muralidhar) or the **omega excess return** downside-style-beta formula (Eq. 11.10-11.12) — Bacon pp.211, 215-216.
- You need the precise leverage-factor derivation of the cash-blended reference portfolio (sigma_b/sigma_p) — Christopherson, Cariño & Ferson Eq. 10.5 pp.110-111.
- You need to confirm whether arithmetic or continuously-compounded returns should drive the M2 excess-return definition for a given mandate — Bacon pp.209.
