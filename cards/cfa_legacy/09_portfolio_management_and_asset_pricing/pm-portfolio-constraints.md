---
schema_version: "cacg.v0"
id: "pm-portfolio-constraints"
title: "Portfolio Constraints — The LLTTU Families"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Portfolio Constraints — The LLTTU Families: enumerating the five constraint families recorded in an IPS — liquidity, time horizon, taxes, legal / regulatory, unique circumstances (LLTTU) — and showing how each family bounds the feasible portfolio in a different dimension"
tags: ["portfolio-management", "ips-constraints", "llttu"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3284:4947"
    chunk_hash: "c2d77dcd1f1ba1ac25cf7547cff780ef2d1f6672fc480f9e24253432943a9405"
    page_range: [3284, 3285]
    quote: "we analyze five major types of constraints on portfolio selection: liquidity, time horizon, tax concerns, legal and regulatory factors, and unique circumstances."
    edge_type: "defines"
card_hash: "ecb0c10bd975efda2da0401e67cd9ee768ed02b645c5bb6fec9720a40d103b7e"
---
# Portfolio Constraints — The LLTTU Families

## Intuition

A portfolio is feasible only inside the intersection of five
constraint families. Each family bounds the allocation in a
different dimension: liquidity bounds how much may be illiquid,
time horizon bounds how short-term the duration profile may be,
taxes shape the after-tax-return objective, legal / regulatory
bounds restrict permitted holdings or maximum exposures, and
unique circumstances cover everything else specific to the
investor (ESG preferences, family-business concentration, religious
prohibitions). The five together carve the universe of conceivable
portfolios down to the IPS-feasible set `W_IPS` inside which the
allocation step operates. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.10-35.

```
       universe of holdings
       ====================
       +------------------------------------------+
       |                                          |
       |       liquidity-feasible region          |
       |       +----------------------------+     |
       |       |                            |     |
       |       |   horizon-feasible         |     |
       |       |   +------------------+     |     |
       |       |   |                  |     |     |
       |       |   |  tax-feasible    |     |     |
       |       |   |  +------------+  |     |     |
       |       |   |  |            |  |     |     |
       |       |   |  |  legal-OK  |  |     |     |
       |       |   |  |  +------+  |  |     |     |
       |       |   |  |  | UNIQ |  |  |     |     |
       |       |   |  |  +------+  |  |     |     |
       |       |   |  +------------+  |     |     |
       |       |   +------------------+     |     |
       |       +----------------------------+     |
       |                                          |
       +------------------------------------------+

       W_IPS = the innermost intersection
```

The order of the families is not strict; the constraints typically
do not commute and a tight liquidity constraint may not bind a
permitted-holdings list at all. The investor and manager elicit each
family separately and document them in the IPS so the allocation
step has unambiguous boundaries to operate within. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.10-35.

## Definition

The liquidity constraint specifies the minimum fraction of the
portfolio that must be readily convertible to cash without material
loss in value. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.10-35.

```
liquidity:  fraction_liquid(w) >= L_min
```

`L_min` is set by the investor's spending needs (anticipated cash
withdrawals over the planning horizon) plus a contingency buffer.
Higher `L_min` excludes illiquid asset classes (private equity,
direct real estate, hedge funds with lockups) from `W_IPS`.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.10-35.

The time horizon constraint specifies the planning period over
which the portfolio is held before withdrawal or repurposing.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.10-35.

```
horizon:  planning_period >= H_min  (single-stage)
       or planning_period =  [H_1, H_2, ...]  (multi-stage)
```

A long horizon supports return-seeking allocations because realized
drawdowns have time to recover; a short horizon constrains the
allocation toward duration-matched fixed income or cash. Multi-
stage horizons (e.g. retirement preceded by a savings phase)
require a glide path connecting the stages. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.10-35.

The tax constraint reframes the return objective into after-tax
terms and constrains the trading style. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.10-35.

```
tax:  return_objective_post_tax = (1 - tau) · return_objective_pre_tax
      tax_drag(turnover) increases with realized capital gains rate
```

`tau` is the marginal tax rate applied to portfolio income. The
constraint affects asset-class preference (municipal bonds preferred
in high-tax-bracket accounts; tax-exempt institutions are
indifferent) and trading-style preference (low-turnover strategies
preferred in taxable accounts; tax-loss harvesting becomes a
secondary objective). **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.10-35.

The legal / regulatory constraint specifies binding restrictions
on permitted holdings, maximum exposures, and operational
requirements. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.10-35.

```
legal:  permitted_holdings(w) subset of L_legal
        max_exposure_per_class(w) <= E_legal
        ERISA / UCITS / state-blue-sky / other applicable rules
```

For institutional portfolios (pensions, endowments, insurance
general accounts), the legal / regulatory family is typically the
tightest binding constraint. For retail investors it is usually
loose, but estate / trust-specific rules can bind. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.10-35.

The unique circumstances constraint covers everything specific to
the investor that does not fit the prior four families. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.10-35.

```
unique:  ESG preference; concentrated-holding restrictions;
         religious prohibitions; family-legacy concentration;
         tobacco / firearms / fossil-fuel exclusions; etc.
```

This family is heterogeneous by construction — it absorbs whatever
is specific enough that no general rule applies. The IPS records
these explicitly so the allocation step can implement them.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.10-35.

## Mathematical Reasoning

The five families combine multiplicatively into the feasible set
`W_IPS`. Symbolically, `W_IPS` is the intersection of the per-family
feasible sets. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.10-35.

```
W_IPS = W_liquidity intersect W_horizon intersect W_tax
        intersect W_legal intersect W_unique
```

When the constraint families are mutually consistent, `W_IPS` is
non-empty and the allocation step operates within it. When the
families conflict — a high return objective requiring illiquid
high-return assets, with a high `L_min` excluding them — `W_IPS`
may be empty and the IPS itself must be revised to make the
constraints jointly satisfiable. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.10-35.

The interaction between constraints and risk tolerance is not
additive. A short time horizon both lowers ability to bear risk
(less time to recover from drawdown) and constrains the feasible
allocation directly (by excluding illiquid or long-duration
assets). The two effects compound rather than substitute, and a
short-horizon high-risk-objective combination is typically rejected
on consistency grounds. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.10-35.

A specific implication for IPS construction: each constraint
family is documented with quantitative bounds where possible (an
explicit `L_min`, a stated `H_min`, a recorded marginal tax rate,
a per-class exposure cap, and a list of unique restrictions). Soft
or aspirational language ("prefer to keep some cash on hand")
fails the IPS-as-policy test because the manager cannot
operationalize it. The IPS-revision discipline requires turning
soft preferences into hard constraints before the policy is
binding. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.10-35.

The boundary between a binding constraint and a stated objective
is sometimes blurry. ESG preferences may live in the unique
constraints family (as exclusions) or as a sub-objective shaping
within-class selection. The L1 framing prefers the explicit-
constraint approach: anything important enough to act on goes into
`W_IPS` so the allocation step respects it deterministically.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.10-35.

## See Also

- [`pm-investment-policy-statement.md`](pm-investment-policy-statement.md) — the IPS that records the LLTTU constraints
- [`pm-risk-tolerance-and-objectives.md`](pm-risk-tolerance-and-objectives.md) — the return / risk objectives that operate inside `W_IPS`
- [`pm-allocation-process.md`](pm-allocation-process.md) — the allocation step that searches inside `W_IPS` for the strategic optimum

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R51 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.10-35.

- Worked tax-efficient asset-location strategies (which classes
  belong in tax-advantaged vs taxable accounts) — Vol.6 R51
  introduces; deeper development belongs in future-13.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.10-35.
- ERISA / UCITS / pension-specific regulatory frameworks for
  institutional investors — Vol.6 R51 mentions; deeper coverage
  belongs in future-13. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.10-35.
- Multi-stage glide-path construction connecting savings and
  spending phases — Vol.6 R51 introduces; the full glide-path
  discipline belongs in future-13. **Source:** CFA L1 Curriculum
  (2022) Vol.6/pp.10-35.
