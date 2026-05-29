---
schema_version: "cacg.v0"
id: "fra-non-current-liabilities"
title: "Non-Current Liabilities"
reading_id: "03_financial_reporting_analysis"
summary: "Framing recognition and measurement of long-term debt and other non-current obligations: bonds at PV of contractual cash flows discounted at market rate, effective-interest amortization for premium/discount, lease liabilities on balance sheet under IFRS 16/ASC 842. Leverage and interest-coverage ratios fall out of the surface."
tags: ["financial-reporting", "non-current"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1622:2346"
    chunk_hash: "efff73cd3c00613fb26097c38b488e9e9444101d05f810f222f066d25bf7da16"
    page_range: [1622, 1622]
    quote: "In this reading, we use the terms bond and note interchangeably because the accounting treatments of bonds payable and long-term notes payable are similar."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1622:2347"
    chunk_hash: "a5aec3f7c68a323a28f681fb628fe1a34a3b1bd9370b1f699f708d8c61f55e85"
    page_range: [1622, 1623]
    quote: "Periodic interest payments are made based on the interest rate promised in the bond contract applied to the bonds’ face value."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1108:1585"
    chunk_hash: "a0d7c48d7912fd7856b93dd0144fa48dab97552df213064fa5cf5c6d8e6b6d65"
    page_range: [1108, 1109]
    quote: "Arguably, the most critical are the differences that exist between IFRS and US GAAP."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1762:2545"
    chunk_hash: "790e4d8557bce1740f346a31cb955a21d66ec69f5f4c9b7ad5feafce4d4abff4"
    page_range: [1762, 1762]
    quote: "Whatever the techniques adopted, the analytical focus of credit analysis is on debt-paying ability."
    edge_type: "supports"
card_hash: "087a5288ff2a5a6e2a85a198fb3c7878a2b6d6d19b2bacd97361c702d810f3bb"
---
# Non-Current Liabilities

## Intuition

Non-current liabilities are obligations the firm expects to settle
over a period longer than one year (or one operating cycle).
Long-term debt is the largest category for most firms — bonds
issued to public capital markets, term loans from banks, syndicated
loans — and it is the basis for the firm's leverage profile. Lease
liabilities, deferred tax liabilities, pension obligations, and
provisions for warranties or environmental restoration round out
the typical non-current-liability section. Each category has
specific measurement rules; the long-term debt category drives
most analyst attention because it links directly to interest
expense on the income statement and to financing cash flows on
the cash flow statement. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.435-488.

The accounting for long-term debt at issuance follows a simple
principle: a bond is recorded at the present value of its
contractual cash flows discounted at the market interest rate
prevailing at issuance. If the market rate equals the bond's coupon
rate, the bond is issued at par (face value). If the market rate
exceeds the coupon, the bond is issued at a discount (below face
value). If the market rate is below the coupon, the bond is issued
at a premium (above face value). The issuance proceeds equal the
present value, and the difference between proceeds and face value
is amortized over the bond's life as additional interest expense
(for discounts) or as offsetting interest expense (for premiums).
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.435-488.

```
+--------------------------------------------+
| Bond Issuance: Three Cases                 |
+--------------------------------------------+
| Issued at PAR (market rate = coupon)       |
|   Proceeds = Face Value                    |
|   Interest Exp_period = Coupon × Face      |
|                                            |
| Issued at DISCOUNT (market rate > coupon)  |
|   Proceeds < Face Value                    |
|   Interest Exp_period = market rate ×      |
|       (carrying amount)                    |
|     > Coupon × Face                        |
|   Carrying amount climbs toward Face       |
|                                            |
| Issued at PREMIUM (market rate < coupon)   |
|   Proceeds > Face Value                    |
|   Interest Exp_period = market rate ×      |
|       (carrying amount)                    |
|     < Coupon × Face                        |
|   Carrying amount declines toward Face     |
+--------------------------------------------+
```

The schematic illustrates the three issuance cases and the
direction in which the carrying amount converges to face value as
the bond approaches maturity. The effective-interest method
(market rate × beginning-period carrying amount = interest expense)
is the standard amortization technique under both IFRS and US
GAAP. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.435-488.

## Definition

A non-current liability is an obligation that is not expected to be
settled within one year or one operating cycle, whichever is
longer. The major non-current-liability categories are: long-term
debt (bonds, notes, term loans), lease liabilities (under IFRS 16 /
ASC 842), deferred tax liabilities, pension and other post-
employment-benefit obligations, provisions and contingent
liabilities, and deferred revenue (long-term portion). **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.435-488.

A contingent liability is a potential obligation arising from past
events whose existence will be confirmed only by the occurrence or
non-occurrence of one or more uncertain future events not wholly
within the firm's control. The L1-LOS treatment of contingent
liabilities turns on two questions: how likely is the outflow, and
can the amount be reliably measured? Under IFRS, a provision is
recognized on the balance sheet when an outflow is probable and
the amount can be reliably estimated; possible-but-not-probable
outflows are disclosed in the notes (no balance-sheet recognition);
remote outflows need not be disclosed. Under US GAAP, the
recognition threshold is "probable AND reasonably estimable" and
the disclosure framework parallels IFRS but uses different
likelihood vocabulary. The two frameworks share the structure: a
contingent obligation either becomes a recognized provision (on
balance sheet, with the corresponding expense in the income
statement), a disclosed contingency (in the notes only), or
neither. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.435-488.

Long-term debt is initially recorded at the present value of the
contractual cash flows (face-value redemption at maturity plus
periodic coupon payments) discounted at the market interest rate
prevailing at issuance. The proceeds received equal that present
value. After issuance, the carrying amount is updated each period
under the effective-interest method: the period's interest expense
equals the market rate at issuance multiplied by the beginning-
period carrying amount; the period's amortization of premium or
discount is the difference between the cash coupon paid and the
period's interest expense. The carrying amount converges to face
value at maturity. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.435-488.

Lease accounting underwent significant change with the adoption of
IFRS 16 (2019) and ASC 842 (2018-2019). Both frameworks now require
lessees to recognize most leases on the balance sheet as a
right-of-use asset (the lessee's right to use the leased asset over
the lease term) and a corresponding lease liability (the present
value of the future lease payments). The right-of-use asset is
amortized over the lease term; the lease liability accretes
interest under the effective-interest method, with each lease
payment partly reducing the liability and partly recognized as
interest expense. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.435-488.

The two frameworks differ in classification on the income statement
and cash flow statement. Under IFRS 16, all leases are treated as
finance leases for income-statement purposes — the lessee reports
amortization of the right-of-use asset and interest expense on the
lease liability separately. Under ASC 842, leases are classified as
either finance leases (same separate-line treatment) or operating
leases (single straight-line lease expense, no separate
amortization). The IFRS approach unifies the income-statement
treatment; the US GAAP dual-classification preserves the prior
operating-lease style for one branch. **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.435-488.

## Mathematical Reasoning

The effective-interest method's per-period algebra is: `Interest
Expense_period = Market Rate × Carrying Amount_beginning`.
The cash coupon paid is `Coupon Rate × Face Value` (constant each
period for a fixed-rate bond). The amortization is the difference:
`Amortization = Interest Expense − Cash Coupon`, and the carrying
amount updates by adding the amortization in one unified rule:
`Carrying Amount_end = Carrying Amount_beginning + (Interest
Expense − Cash Coupon)`. The sign of the amortization handles the
premium / discount distinction automatically: for a premium bond
the market rate is below the coupon rate, so interest expense is
below the cash coupon, the amortization is negative, and the
carrying amount declines toward face value across periods; for a
discount bond the market rate is above the coupon rate, so interest
expense exceeds the cash coupon, the amortization is positive, and
the carrying amount grows toward face value across periods. The
unified update preserves the right direction in both cases without
needing a per-issuance-case formula. **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.435-488.

The bond's present value at issuance is the sum of the present
values of each future coupon payment (the periodic cash coupon
discounted to issuance date) plus the present value of the face
redemption at maturity, all discounted at the market rate `r`
prevailing at issuance over the bond's `N`-period horizon. When
the market rate equals the coupon rate, this present value equals
face; when the market rate exceeds the coupon, the present value
falls below face (discount issuance); when the market rate is below
the coupon, the present value rises above face (premium issuance).
The issuance proceeds equal the present value by construction.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.435-488.

The income-statement and balance-sheet effects of the
effective-interest method ensure that the bond's total interest
expense over its life equals the total cash difference between
coupon payments plus face redemption and issuance proceeds. The
amortization of premium reduces interest expense below the cash
coupon; the amortization of discount raises interest expense above
the cash coupon. The cumulative amortization equals the original
premium or discount, eliminating the difference at maturity.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.435-488.

The leverage ratios that fall out of the non-current-liability
surface include the debt-to-equity ratio (`Total Debt / Total
Equity`), the debt-to-assets ratio (`Total Debt / Total Assets`),
and the interest-coverage ratio (`EBIT / Interest Expense`). The
debt definition for these ratios is sensitive to whether
short-term debt and lease liabilities are included; the analyst
should be explicit about the definition. The post-IFRS 16 / ASC 842
lease-on-balance-sheet treatment increased reported total debt and
the corresponding leverage ratios for firms with significant
operating-lease use (retail, airlines, hospitality); cross-period
comparison across the standards-change boundary requires
adjustment. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.435-488.

The non-current-liability surface interacts with the cash flow
statement in a specific pattern: bond issuance proceeds appear in
financing activities (CFF) as a cash inflow; bond redemption at
maturity appears in CFF as an outflow; interest payments appear in
CFO under US GAAP and in either CFO or CFF under IFRS (per the
firm's classification choice); lease payments appear partly in CFO
(interest portion) and partly in CFF (principal portion) under both
post-2019 frameworks. The analyst tracking total cash to debt
holders should aggregate across these classifications. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.435-488.

The analyst's treatment of contingent liabilities follows the
recognition layer's outcome. Recognized provisions (probable + reliably
estimable) appear directly on the balance sheet and contribute to
total liabilities for leverage and solvency ratios; the matching
expense reduces the period's reported income. Disclosed
non-recognized contingencies (possible but not probable, or
reasonably possible under US GAAP) do not appear on the balance
sheet but represent potential future cash outflows that the analyst
should incorporate into liquidity and solvency assessment when
material — a large pending litigation contingent claim, a contingent
acquisition consideration, a guarantee on a third-party obligation,
all sit outside the GAAP-recognized liability total but inside the
firm's downside risk envelope. The CFA L1 applications reading
treats this as a routine adjustment in stress-tested liquidity
analysis. **Sources:** CFA L1 Curriculum (2022) Vol.3/pp.435-488 +
Vol.3/pp.561-598.

## See Also

- [`fra-balance-sheet-foundations`](./fra-balance-sheet-foundations.md) — non-current liabilities are the major non-current liability category alongside deferred-tax and pension obligations
- [`fra-income-statement-foundations`](./fra-income-statement-foundations.md) — interest expense from long-term debt drives EBT-vs-EBIT margin and ETR-vs-statutory-tax-rate readings
- [`fra-cash-flow-statement-mechanics`](./fra-cash-flow-statement-mechanics.md) — bond issuance / redemption / interest classifications interact with the CFO / CFI / CFF partition

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R24 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.435-488.

- the firm has issued long-term debt at a significant premium or
  discount and the analyst needs to reconstruct the
  effective-interest amortization schedule for cross-period
  interest-expense comparison. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.435-488.
- the firm has significant lease activity and the analyst needs the
  curriculum's per-framework (IFRS 16 vs ASC 842) classification
  treatment for the income-statement and cash-flow effects.
  **Source:** CFA L1 Curriculum (2022) Vol.3/pp.435-488.
- the firm reports leverage ratios that materially exceed peers and
  the analyst needs the curriculum's per-component disclosure (debt
  covenants, callable provisions, conversion features, secured vs
  unsecured) for risk assessment. **Source:** CFA L1 Curriculum
  (2022) Vol.3/pp.435-488.
