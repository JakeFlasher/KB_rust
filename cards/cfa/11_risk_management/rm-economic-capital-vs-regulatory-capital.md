---
schema_version: "cacg.v0"
id: "rm-economic-capital-vs-regulatory-capital"
title: "Economic Capital: Unexpected Loss and the Rating-Linked Confidence Level"
reading_id: "11_risk_management"
summary: "Economic capital is a bank's own one-year unexpected-loss buffer — the gap between the confidence-level quantile of the loss distribution and expected loss — with the confidence level pinned to a target rating and built bottom-up or top-down via Merton, per Hull Ch.28."
tags: ["risk-management", "economic-capital", "regulatory-capital"]
citations:
  - source_id: "rm_hull_2023_rmfi"
    chunk_id: "rm_hull_2023_rmfi:p627:0869"
    chunk_hash: "3c464f8fa0faf09eec00825e3a465ef3d3e2e618d4e099df882328936fbef9bc"
    page_range: [628, 628]
    quote: "Economic capital is usually defined as the amount of capital a financial institution needs in order to absorb losses over one year with a certain confidence level."
    edge_type: "defines"
card_hash: "035ca8df54d69b9db0a2ea546da4411e7b3ca8e1dbd0491aa3fe981643b8e5f4"
---
# Economic Capital: Unexpected Loss and the Rating-Linked Confidence Level

## Intuition
Regulatory capital is a one-size-fits-all rule imposed from outside. **Economic
capital** (or risk capital) is the bank's *own internal* estimate of the capital it
needs for the risks it actually runs — a "currency" for risk-taking, allocated to
business units so their profitability can be judged against the capital they consume.
The key move is to separate *expected* loss, which is priced into products and covered
by margins, from *unexpected* loss, which is what capital must absorb. The bank then
chooses how solvent it wants to be by pinning a confidence level to a target credit
rating: a AA bank wants its one-year default probability as small as a AA corporate's,
so it holds capital out to that quantile of its loss distribution.

```
   loss density
   ^
   |     ___
   |   /     \
   |__/        \________> L
   +----|----------|----
     E[loss]    X-percentile
        |<--------|  economic capital = unexpected loss
```

**Source:** Hull (2023) Ch.28 §28.1 printed pp.599–600 (PDF pp.627–628).

## Definition
- **Economic capital.** The capital needed to absorb losses over one year with a
  chosen confidence level X% — i.e. the probability the bank's losses stay within its
  capital over one year is X%.
- **Unexpected loss.** Capital covers unexpected loss = (actual loss − expected loss).
  Expected loss is absorbed in product pricing; only unexpected loss requires capital.
- **Rating → confidence mapping.** The confidence level is set to the target rating's
  one-year default probability: a AA target (≈0.02% one-year PD) implies a confidence
  level ≈99.98%; a BBB target (≈0.2% PD) implies ≈99.8%.
- **Two measurement routes.** *Top-down* estimates asset volatility and the chance
  assets fall below liabilities (Merton's structural model is the framework).
  *Bottom-up* (the common choice) estimates loss distributions per risk type and
  business unit, then aggregates to a firm-wide loss distribution.
- **Scope vs regulatory capital.** Economic capital may be assessed for *business
  risk* (strategic + reputational), for which no regulatory capital is required.

**Source:** Hull (2023) Ch.28 §28.1–28.1.1 printed pp.600–601 (PDF pp.628–629).

## Mathematical Reasoning
Let L be the one-year loss with distribution function F and let q_X = F⁻¹(X) be the
X-percentile. Economic capital is the unexpected-loss buffer

    EconCap = q_X − E[L],

the horizontal gap in the loss diagram between the confidence-level quantile and the
mean. Two design choices fix it: the *confidence level* X (set by the target rating's
default probability, so 1 − X equals the tolerated one-year default probability) and
the *shape of F* (heavier tails push q_X out and enlarge the buffer for the same X).

The rating link is an inequality on tolerated insolvency: choosing X = 99.98% means
P(L > EconCap + E[L]) = 1 − X = 0.02%, matching a AA corporate's one-year PD. Raising
the target rating lowers the tolerated default probability, raises X, and (for any
fixed loss law) enlarges economic capital monotonically.

Bottom-up construction aggregates per-risk economic capital amounts E_i; because the
risks are less than perfectly correlated, the firm-wide total is below their sum,

    EconCap_total ≤ Σ_i E_i,

with the gap being the diversification benefit (the hybrid aggregation uses a
correlation-weighted quadratic form, exact under normality, approximate otherwise).

**Source:** Hull (2023) Ch.28 §28.1–28.1.1 printed pp.600–601 (PDF pp.628–629).

## See Also
- [rm-basel-capital-accord-evolution](./rm-basel-capital-accord-evolution.md) — the regulatory-capital regime economic capital contrasts with.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — the expected/unexpected-loss anatomy this buffer reads off.
- [rm-raroc-risk-adjusted-return-on-capital](./rm-raroc-risk-adjusted-return-on-capital.md) — performance measured against this economic capital.
- [rm-integrated-firm-wide-risk-aggregation](./rm-integrated-firm-wide-risk-aggregation.md) — the firm-wide aggregation of the per-risk buffers.

## Escalate to Raw When
You need the worked Example 28.1/28.2 plug-and-chug (the per-unit economic-capital
figure from the rating-implied VaR net of expected loss, or the one-year VaR
time-scaling computation), or the Deutsche Bank economic-vs-regulatory capital tables —
those calculated buffers live in the raw text (Rule 1).

**Source:** Hull (2023) Ch.28 §28.1–28.2 printed pp.599–603 (PDF pp.627–631).
