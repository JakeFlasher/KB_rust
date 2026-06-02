---
schema_version: "cacg.v0"
id: "rm-basel-capital-accord-evolution"
title: "Basel Capital Accords I → II → II.5 → III: The Regulatory-Capital Workflow"
reading_id: "11_risk_management"
summary: "The Basel lineage builds bank capital off risk-weighted assets: the Cooke ratio and RWA in Basel I, three pillars plus IRB in Basel II, and the CET1/AT1/Tier-2 stack with conservation/countercyclical buffers, a 3% leverage ratio, and LCR/NSFR in Basel III, per Hull Ch.25–26."
tags: ["risk-management", "basel-accords", "regulatory-capital"]
citations:
  - source_id: "rm_hull_2023_rmfi"
    chunk_id: "rm_hull_2023_rmfi:p573:0791"
    chunk_hash: "2f3ea8af54276cc648f5e73bd0e2f86e53ed26b5e4f1f1966a71d06a9158fbd7"
    page_range: [573, 573]
    quote: "The general requirement in Basel I that banks hold a total capital equal to 8% of risk-weighted assets (RWA) remains unchanged."
    edge_type: "defines"
card_hash: "b0529336cc5f32eb3acd05b095c221f4a767666025079661f3d5f1e146cb8bbb"
---
# Basel Capital Accords I → II → II.5 → III: The Regulatory-Capital Workflow

## Intuition
Bank regulation since 1988 is evolutionary: each accord modifies the last while
preserving its scaffolding, so the current regime only makes sense as a sequence. The
through-line is one idea — make a bank hold capital *proportional to the risk it
takes*, measured by risk-weighted assets (RWA), not raw balance-sheet size (which
ignored derivatives' future exposure). Basel I priced credit risk crudely; Basel II
refined the risk weights and added supervisory review, disclosure, and an operational
charge; Basel II.5 patched the trading book after 2008; Basel III tightened the
*definition* of capital, layered on buffers, and — for the first time — added a
non-risk-weighted leverage backstop and liquidity ratios.

```
  1988 Basel I        1996 Amend.     2007 Basel II      2009 Basel II.5    2010+ Basel III
  Cooke ratio /  ──►  market-risk ──► 3 pillars +   ──►  stressed VaR,  ──► CET1/AT1/Tier2,
  RWA, 8% min         capital         IRB, op-risk        CRM (trading)      buffers, 3% LR, LCR/NSFR
```

**Source:** Hull (2023) Ch.25 §25.1–25.3 printed pp.533–536 (PDF pp.561–564).

## Definition
- **Cooke ratio / RWA (Basel I).** Credit exposures split into three categories —
  on-balance-sheet assets, off-balance-sheet items, and OTC derivatives. Each
  on-balance asset gets a regulator-set risk weight (three weight tiers: 0% for
  cash/OECD-government claims, 20%/50% for banks/mortgages, 100% for corporates).
- **Current-exposure method (derivatives).** The credit-equivalent amount of a
  derivative is max(V, 0) + aL — current exposure plus an add-on for possible future
  exposure increase.
- **Three pillars (Basel II).** (1) Minimum capital requirements (credit + market +
  operational); (2) supervisory review; (3) market discipline (disclosure). Credit
  capital via the standardized, Foundation IRB, or Advanced IRB approaches.
- **Capital stack (Basel III).** Tier 1 equity (CET1 — share capital + retained
  earnings, ex-goodwill), Additional Tier 1 (AT1, e.g. non-cumulative preferred), and
  Tier 2 (subordinated debt). CET1 is "going-concern", Tier 2 "gone-concern" capital.
- **Buffers + backstops (Basel III).** A capital conservation buffer (extra CET1) and
  a discretionary countercyclical buffer sit above the minimums; a non-risk-weighted
  minimum leverage ratio of 3% (Tier 1 / total exposure); and two liquidity ratios —
  the LCR (30-day stress) and NSFR (one-year stable funding).

**Source:** Hull (2023) Ch.25 §25.3–25.8 printed pp.536–557 (PDF pp.564–585); Ch.26 §26.2 printed pp.567–573 (PDF pp.595–601).

## Mathematical Reasoning
**RWA construction.** Total on-balance-sheet RWA = Σ_i w_i L_i, where L_i is the
principal of item i and w_i its risk weight. Off-balance items enter via conversion
factors; OTC derivatives enter via the credit-equivalent amount

    CEA = max(V, 0) + aL,

where max(V, 0) is current exposure (if the counterparty defaults and V > 0 the bank
loses V; if V ≤ 0 there is neither gain nor loss) and aL allows for future exposure
growth. The capital constraint is then

    Total Capital ≥ 0.08 × (credit-risk RWA + market-risk RWA + operational-risk RWA),

with any directly computed risk charge multiplied by 12.5 to convert it to an
RWA-equivalent so the 8% relation always holds.

**Capital tiering as loss-absorption ordering.** The tiers are an absorption
waterfall: while the bank is a going concern (positive equity) CET1 absorbs losses
first; once equity is gone, Tier 2 (which ranks below depositors in liquidation)
absorbs the rest. The Basel III minimums layer as inequalities on the same RWA
denominator: CET1 ≥ 4.5%, Total Tier 1 ≥ 6%, Total capital ≥ 8% — with the
conservation buffer pushing the *normal-times* CET1 requirement to 7%, and dividend
distribution constrained as CET1 falls into the buffer band.

```
  losses ─►  CET1 (going-concern)  ─►  AT1  ─►  Tier 2 (gone-concern, > depositors)
             4.5% floor                            8% total floor on RWA
             +2.5% conservation buffer  ─►  7% normal-times CET1
```

**Source:** Hull (2023) Ch.25 §25.3.1 printed pp.536–538 (PDF pp.564–566); Ch.26 §26.2.1–26.2.5 printed pp.568–572 (PDF pp.596–600).

## See Also
- [rm-frtb-stressed-es-market-risk-capital](./rm-frtb-stressed-es-market-risk-capital.md) — FRTB, the later market-risk-capital overhaul on this lineage.
- [rm-economic-capital-vs-regulatory-capital](./rm-economic-capital-vs-regulatory-capital.md) — the internal counterpart to these one-size-fits-all rules.
- [rm-credit-var-portfolio](./rm-credit-var-portfolio.md) — the credit-VaR machinery behind the IRB risk weights.
- [rm-operational-risk-basics](./rm-operational-risk-basics.md) — the operational-risk charge Basel II added to the stack.

## Escalate to Raw When
You need the worked Example 25.1 RWA sum (corporate + government + mortgage at their
weights), the Table 25.4 standardized risk-weight grid by rating, the Table 26.2/26.3
dividend-restriction bands, or the specific buffer/ratio percentages as computed rows
— those numeric tables and worked sums live in the raw text (Rule 1).

**Source:** Hull (2023) Ch.25 §25.3–25.8 printed pp.533–557 (PDF pp.561–585); Ch.26 §26.2 printed pp.567–573 (PDF pp.595–601).
