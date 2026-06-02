---
schema_version: "cacg.v0"
id: "fra-cash-vs-accrual-accounting"
title: "Cash vs Accrual Accounting"
reading_id: "03_financial_reporting_analysis"
summary: "Explains why net income (accrual measure) and operating cash flow (cash measure) disagree by construction; frames accrual accounting as the system that records revenues when earned and expenses when incurred regardless of cash timing; gives the analyst guidance on which measure to weight under which question."
tags: ["financial-reporting", "cash-accrual"]
citations:
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p137:0228"
    chunk_hash: "e34c12870566a37189a2dc95b562a7db37aab0af2ce1d6b46fe6a02cad18d9f4"
    page_range: [137, 138]
    quote: "The cash flow statement tracks operating and investment activities with cash accounting"
    edge_type: "defines"
  - source_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed"
    chunk_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed:p259:0237"
    chunk_hash: "5b11f722bc287465067583efae3a5972983c81c96721f243e3ee041f20666d35"
    page_range: [259, 260]
    quote: "Most companies use accrual-basis accounting: They recognize revenue when the performance obligation is satisfied and expenses in the period are incurred, without regard to the time of receipt or payment of cash."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1085:1551"
    chunk_hash: "4e8e1e7cfcf7e760f55c4d68aa51498fa5d09a9f2bc01caae4fd33c26dbaafe8"
    page_range: [1085, 1086]
    quote: "The relationship among the three parts of the balance sheet (assets, liabilities, and owners’ equity) may be shown in equation form as follows: Assets = Liabilities + Owners’ equity"
    edge_type: "supports"
card_hash: "643b21d1092dc93c07a3adcc6fd736ed481cf3abd0aef4ce948a81bc8421b0a1"
---
# Cash vs Accrual Accounting

## Intuition

A firm that sells a product on 30-day credit has earned revenue
today but will collect cash 30 days later. A firm that buys
inventory today and sells it 60 days later has paid cash today but
incurs the cost-of-goods-sold expense 60 days from now. Cash
movements and economic activity are temporally separated by working
capital. Accrual accounting is the system that records the
economic activity when it happens; cash accounting is the system
that records only the cash leg. The two systems give different
period-by-period numbers, and the difference between them is
informative, not noise. **Source:** Penman (2013) Ch.4 pp.110-139.

The valuation use case explains why both systems matter. Cash
flows pay the analyst's eventual claim; accruals describe the
economic activity that produces those future cash flows. A firm
whose accrual earnings keep climbing while cash flow stays flat is
saying that the activity is real but the working-capital build is
deferring the cash. Whether that is value-creating reinvestment or
deteriorating collectability is a question the analyst answers by
reading the accrual structure. **Source:** Penman (2013) Ch.4
pp.110-139.

```
<!-- primitive: accrual-to-cash-bridge source: _diagram_primitives.md -->
       Net Income (accrual)
              |
              |  + non-cash D&A
              |  + non-cash SBC
              |  + non-cash impairments
              v
       NI + non-cash adjustments
              |
              |  +- working-capital changes
              |    (- Δ AR  - Δ Inv  + Δ AP  + Δ Acc Liab)
              v
       Operating Cash Flow (cash)
```

The bridge above shows the canonical accrual-to-cash reconciliation
that appears (in expanded form) on every published indirect-method
cash flow statement. Net income is adjusted for non-cash items
(depreciation, amortization, stock-based compensation, impairments)
and for working-capital changes (accounts receivable, inventory,
accounts payable, accrued liabilities) to recover operating cash
flow. The decomposition is the analyst's primary lens on
earnings-cash divergence. **Source:** Penman (2013) Ch.4 pp.110-139.

## Definition

Accrual accounting records revenues when earned (the firm has
substantially performed and the customer is obligated to pay) and
expenses when incurred (the firm has consumed the resource that
generated the revenue) regardless of when the related cash is
received or paid. Cash-basis accounting records revenues when cash
is received and expenses when cash is paid. The published income
statement under IFRS and US GAAP is accrual-basis; the cash flow
statement reconciles the accrual income statement back to cash
movements. **Source:** Penman (2013) Ch.4 pp.110-139.

The conceptual driver behind accrual accounting is the matching
principle: expenses are recognized in the period whose revenue
they helped produce. A factory that produces inventory in period 1
and sells it in period 2 incurs the related cost (cost of goods
sold) in period 2, not period 1. Matching produces a more faithful
period-by-period picture of the firm's profitability than cash
movements alone. **Source:** Penman (2013) Ch.4 pp.110-139.

The accruals are the building blocks of the difference between net
income and operating cash flow. They divide naturally into two
families. **Source:** Penman (2013) Ch.4 pp.110-139.

- Working-capital accruals — changes in accounts receivable,
  inventory, accounts payable, and accrued liabilities. These are
  the timing differences between when the firm records revenue or
  expense and when cash actually moves. **Source:** Penman (2013)
  Ch.4 pp.110-139.
- Long-term accruals — depreciation, amortization, deferred taxes,
  pension accruals, and stock-based compensation. These spread
  multi-period cash transactions (capital expenditure, deferred
  tax payments, equity grants) across the accrual periods that
  benefit from them. **Source:** Penman (2013) Ch.4 pp.110-139.

The stock-based compensation accrual deserves a closer look because
its cash mechanics differ from the other long-term accruals. The
recognition concept is the grant-date fair value: the firm measures
the expense on the date it grants the equity instrument to the
employee, then allocates that fixed expense over the service
(vesting) period. Subsequent share-price movement after grant date
does not change the recognized expense; option-pricing
re-measurement and modification-event accounting are
standards-detail mechanics outside the L1-depth recognition
concept. The cash itself does not move at recognition — the
employee receives an equity claim, not cash — so the indirect-
method CFO add-back returns the recognized expense to operating
cash flow. The matching exercise-related cash flow appears later
and is a financing-activity inflow rather than an operating-
activity inflow. **Source:** Penman (2013) Ch.9 pp.258-291.

## Mathematical Reasoning

The aggregate identity tying accrual earnings to cash from operations
is `CFO = NI + non-cash items - ΔWC` where `non-cash items` collects
depreciation, amortization, stock-based compensation, and
impairments, and `ΔWC` is the period change in net non-cash working
capital (Δ AR + Δ Inv − Δ AP − Δ Acc Liab in the canonical sign
convention). The identity is structural, not empirical: it holds in
every period for every firm because it is the algebra of the
indirect-method bridge. **Source:** Penman (2013) Ch.4 pp.110-139.

The implication for valuation is that net income and operating cash
flow each carry distinct information. Net income reflects the
period's economic activity smoothed by accruals; operating cash
flow reflects the period's cash extraction. Over a long horizon
the two converge — accrual reversals net to zero across the firm's
life — so the long-run sum of net income equals the long-run sum
of operating cash flow plus capital-structure-related differences.
Period-by-period differences are the working-capital and long-term
accruals net of their reversals. **Source:** Penman (2013) Ch.4
pp.110-139.

This convergence has a direct valuation consequence. Discounted
cash flow valuation uses cash flows, which are unambiguous and
auditable but volatile period-by-period. Accrual-based valuation
(residual earnings, accrual-anomaly scoring) uses accruals to
smooth the volatility and capture the longer-run economic
activity. Both approaches must agree at infinite horizon by the
identity above; period-by-period they differ by how much of the
firm's economic activity has been extracted as cash by that date.
**Source:** Penman (2013) Ch.4 pp.110-139.

The accrual-anomaly literature documents that high-accrual firms
underperform low-accrual firms in subsequent periods on a
risk-adjusted basis, suggesting accruals are a value-relevant signal
that the market underweights. The analyst can read accruals as a
diagnostic — the larger the accrual share of earnings, the more
the analyst should question persistence. The valuation-vs-discovery
distinction (whether to use cash flows for valuation OR signal
strength for portfolio selection) is treated in the later FRA
accrual-anomaly card. **Source:** Penman (2013) Ch.4 pp.110-139.

The CFA L1 framing of accrual vs cash is consistent with Penman's
account: the curriculum emphasizes the matching principle, the
revenue-recognition criteria, and the indirect-method bridge as the
primary surfaces on which accrual-cash divergence is observed. The
vocabulary differs slightly but the structural identity is the
same. **Source:** CFA L1 Curriculum (2022) Vol.2/pp.475-514.

## See Also

- [`fra-financial-statement-objectives`](./fra-financial-statement-objectives.md) — accrual accounting is the framework that the four primary statements implement
- [`fra-articulation-of-financial-statements`](./fra-articulation-of-financial-statements.md) — the accrual-to-cash bridge is one of the core articulation identities

## Escalate to Raw When

Open Penman Ch.4 directly when any of the criteria below applies.
**Source:** Penman (2013) Ch.4 pp.110-139.

- the analyst needs to evaluate the persistence of an accrual surge
  (revenue recognized far ahead of cash collection) — Penman's
  treatment of working-capital accruals supplies the diagnostic
  patterns. **Source:** Penman (2013) Ch.4 pp.110-139.
- the firm has unusual long-term accrual structure (heavy
  depreciation, large pension accruals, deferred-tax assets) and
  the analyst needs the textbook's framework for separating
  cash-flow-relevant accruals from purely-accounting accruals.
  **Source:** Penman (2013) Ch.4 pp.110-139.
- the question is whether to value the firm on cash flows or on
  accrual earnings — Penman's discussion of the identity that ties
  the two together over the long run is the basis for the
  trade-off. **Source:** Penman (2013) Ch.4 pp.110-139.
