---
schema_version: "cacg.v0"
id: "fra-effective-interest-amortization-bond-side"
title: "Effective-Interest Amortization — Compound-Instrument Liability Leg"
reading_id: "03_financial_reporting_analysis"
summary: "Walks through the issuer's effective-interest amortization schedule for bonds payable issued at a discount or premium — how the interest-expense and discount-amortization columns are computed each period, and how the carrying amount converges to face value at maturity."
tags: ["financial-reporting", "effective-interest"]
citations:
  - source_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed"
    chunk_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed:p1125:1197"
    chunk_hash: "762dfe682df87b9ac7f9cd0f898d288215560f5776e8542e3cc0ea327c03f99a"
    page_range: [1125, 1126]
    quote: "by paying more or less at issuance, investors earn a rate different than the coupon rate on the bond"
    edge_type: "defines"
  - source_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed"
    chunk_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed:p1126:1198"
    chunk_hash: "f0044397ef0c433c3cd7f8ba1a4408d88dd332b116f5c73a7f92245b91e18248"
    page_range: [1126, 1127]
    quote: "Because the investors required an effective-interest rate of 10 percent, they paid €92,278 for the €100,000 of bonds, creating a €7,722 discount"
    edge_type: "defines"
card_hash: "891cb5afca12173e5069dbcce2a45ad5f0dc80f203257260b2eee5d32d572405"
---
# Effective-Interest Amortization — Compound-Instrument Liability Leg

## Intuition

Once a convertible bond is issued and split per IAS 32, the
liability component `L_0` typically sits BELOW face value in
the common discount-issuance case (where the market discount
rate `r_market` exceeds the bond's stated coupon rate, so the
PV of contractual cash flows discounted at `r_market` falls
short of face). The difference `Face − L_0` is the discount;
over the bond's life, this discount is amortized into interest
expense so that the carrying amount `L_t` rises from `L_0` at
issuance toward `Face` at maturity. In the premium-issuance
mirror case (coupon rate exceeds `r_market`, so `L_0` exceeds
face), the same effective-interest recursion runs with reversed
sign — premium amortizes DOWN into reduced interest expense and
`L_t` falls from `L_0` toward `Face` at maturity. Either way the
mechanism is the effective-interest method; convention in this
card uses the discount case as the running example. **Source:**
Kieso (2020) Ch.16 pp.1254-1256.

The substantive reading: a convertible bond's REPORTED
interest expense exceeds its CASH coupon, and the gap widens
each reporting period because the carrying amount keeps
climbing. An analyst comparing the convertible issuer's
interest-coverage ratio against a straight-debt peer's must
use the reported interest expense (not the cash coupon) to
see the apples-to-apples economic cost of the borrowing.
The conversion-option value `E_0` may have looked free at
issuance (residual; not a cash outflow), but the issuer
pays for it indirectly via elevated reported interest
expense over the bond's life. **Source:** Kieso (2020)
Ch.16 pp.1254-1256.

```
+------------------------------------------------------+
|  Carrying-amount trajectory (rising-discount case)   |
+------------------------------------------------------+
|                                                      |
|       Face   ......................... L_T          |
|                                       /              |
|                                     /                |
|                                   /                  |
|                                 /                    |
|                               /                      |
|       L_0   _________________/                       |
|             |                                        |
|       issuance                          maturity     |
|                                                      |
|  Amortized discount = Face − L_0                     |
|     (equals E_0 only at par issuance P = Face;       |
|      general case: E_0 = P − L_0, distinct from      |
|      Face − L_0 when P ≠ Face)                       |
|  Amortizes into reported interest expense            |
+------------------------------------------------------+
```

**Source:** Kieso (2020) Ch.16 pp.1254-1256.

## Definition

The effective-interest method is a one-period recursion that
takes the prior carrying amount and produces the current one
via three identities. The cash coupon is constant; the
interest expense varies (it rises each period as the
carrying amount rises); the amortization is the difference.
**Source:** Kieso (2020) Ch.16 pp.1254-1256.

Constants across the bond's life: `r_market` is the market
rate at issuance; `c` is the stated coupon rate; `Face` is
the par redemption amount; `Coupon = c · Face` is the cash
paid annually. **Source:** Kieso (2020) Ch.16 pp.1254.

Recursion: `Interest_t = r_market · L_{t-1}`,
`Discount_amort_t = Interest_t − Coupon`,
`L_t = L_{t-1} + Discount_amort_t`. **Source:** Kieso
(2020) Ch.16 pp.1254-1256.

Boundary conditions: `L_0 = PV(coupons + face; r_market)`
comes from the IAS 32 split documented in
[`fra-issuer-side-compound-instrument-split`](./fra-issuer-side-compound-instrument-split.md);
`L_T = Face` is the convergence at maturity. **Source:**
Kieso (2020) Ch.16 pp.1254-1256.

```
+--------------------------------------------------------+
|  Effective-interest method (one-period recursion)      |
+--------------------------------------------------------+
|                                                        |
|  Constants:                                            |
|     r_market   =  market rate at issuance              |
|     c          =  stated (coupon) rate                 |
|     Face       =  par redemption amount                |
|     Coupon     =  c · Face         (cash paid annually)|
|                                                        |
|  Carrying-amount recursion:                            |
|     Interest_t        =  r_market · L_{t-1}            |
|     Discount_amort_t  =  Interest_t − Coupon           |
|     L_t               =  L_{t-1} + Discount_amort_t    |
|                                                        |
|  Boundary conditions:                                  |
|     L_0  =  PV( coupons + face ; r_market )            |
|     L_T  =  Face                                       |
|                                                        |
|  Monotonicity (rising-discount case, r_market > c):    |
|     L_0 < L_1 < L_2 < ... < L_T  =  Face               |
|     Interest_t > Coupon for every t                    |
+--------------------------------------------------------+
```

**Source:** Kieso (2020) Ch.16 pp.1254-1256.

## Mathematical Reasoning

For a non-convertible bond, the effective-interest
amortization ENDS the story: at maturity, the carrying
amount equals face, the issuer repays, and the journal
entry is `Dr Bonds Payable / Cr Cash` at face. For a
convertible bond, the amortization STILL converges to face
but the amortization trajectory becomes the running input
to the four settlement paths covered in
[`fra-conversion-extinguishment-accounting`](./fra-conversion-extinguishment-accounting.md).
**Source:** Kieso (2020) Ch.16 pp.1254-1258.

If the holder converts at maturity, the carrying amount is
exactly `Face`, and the conversion entry transfers `Face`
from Bonds Payable to ordinary share accounts (no gain/loss
because `L_T = Face` is already there). **Source:** Kieso
(2020) Ch.16 pp.1255.

If the holder converts BEFORE maturity at some time `t < T`,
the carrying amount is `L_t` (below `Face` in the common
discount-issuance case, above `Face` in the mirror premium
case); the conversion entry transfers `L_t` (not Face) from
Bonds Payable to ordinary share accounts. The mid-life
amortization trajectory determines the equity-account credit
on early conversion. **Source:** Kieso (2020) Ch.16 pp.1256.

If the issuer repurchases at maturity for cash, the entry
is `Dr Bonds Payable Face / Cr Cash Face` (carrying matches
face; no gain/loss). **Source:** Kieso (2020) Ch.16 pp.1255.

If the issuer repurchases BEFORE maturity, gain/loss
recognition uses the SAME bifurcation logic as issuance:
the liability component of the repurchase price is computed
at the prevailing market rate for a shortened-maturity
non-convertible bond, and the difference from carrying
`L_t` is the gain/loss on the liability leg. **Source:**
Kieso (2020) Ch.16 pp.1257.

The income-statement consequence (signed, sign-dependent on
the issuance regime): REPORTED interest expense on a convertible
bond differs from the cash coupon each period by exactly
`Discount_amort_t = L_t · r_market − c · Face`, which is
**positive in the discount-issuance case** (`c < r_market`,
`L_0 < Face` → reported interest EXCEEDS cash coupon) and
**negative in the mirror premium-issuance case** (`c > r_market`,
`L_0 > Face` → reported interest is BELOW cash coupon). Over
the bond's full life, total reported interest differs from
total cash coupon by exactly the signed amount `Face − L_naught`
(positive on discount issuance, negative on premium issuance;
equals the original equity component `E_naught` only at par
issuance P = Face; under general issuance E_naught = P − L_naught
is distinct from Face − L_naught). The accounting captures the
economic cost of accreting (or amortizing) the bond-side
carrying amount from L_naught toward Face across the bond's
life. **Source:** Kieso (2020) Ch.16 pp.1254-1256.

The cash-flow statement treats the cash coupon as an
operating cash outflow (the IFRS default, optionally
financing per IAS 7); the NON-CASH amortization is
reconciled in the indirect-method CFO bridge from net
income as a signed adjustment of exactly the period's
`Discount_amort_t` — an **add-back** in the discount-
issuance case (`Discount_amort_t > 0`, alongside other
non-cash expense items) and a **subtraction** in the
mirror premium-issuance case (`Discount_amort_t < 0`,
alongside other non-cash income items). **Source:** Kieso
(2020) Ch.16 pp.1254-1256.

At any reporting date `t`, the balance sheet shows Bonds
Payable at `L_t` (not face), Share Premium — Conversion
Equity at `E_0` (constant), and total claims of convertible
holders at `L_t + E_0`. **Source:** Kieso (2020) Ch.16 pp.1254-1256.

## See Also

- [`fra-issuer-side-compound-instrument-split`](./fra-issuer-side-compound-instrument-split.md) — the upstream IAS 32 split that produces `L_0` and `E_0` as the boundary conditions for this card's amortization recursion
- [`fra-non-current-liabilities`](./fra-non-current-liabilities.md) — the general effective-interest amortization framework; this card's recursion is the specialization for compound-instrument liability legs
- [`fra-conversion-extinguishment-accounting`](./fra-conversion-extinguishment-accounting.md) — the downstream settlement-path mechanics that consume `L_t` as the running carrying-amount input
- [`fra-cash-flow-statement-mechanics`](./fra-cash-flow-statement-mechanics.md) — the CFO indirect-method bridge framework that absorbs the non-cash `Discount_amort_t` add-back
- [`cb-bond-anatomy-and-cashflows`](../08_convertible_bonds/cb-bond-anatomy-and-cashflows.md) — investor-side cash-flow view
- [`cb-bond-floor-investment-value`](../08_convertible_bonds/cb-bond-floor-investment-value.md) — investor-side floor analogous to the issuer-side liability component
- [`cb-payoff-decomposition-bond-plus-call`](../08_convertible_bonds/cb-payoff-decomposition-bond-plus-call.md) — holder-perspective decomposition mirroring the issuer-side split

## Escalate to Raw When

Open Kieso Ch.16 (pp.1254-1257) directly when any of the
criteria below applies. **Source:** Kieso (2020) Ch.16 pp.1254-1257.

- the analyst needs the worked Convertible Bond
  Amortization Schedule (Illustration 16.5) for cross-check
  of dollar-by-dollar amortization arithmetic that this
  card does NOT reproduce per RULE-03-PROHIBITED-PATTERNS.
  **Source:** Kieso (2020) Ch.16 pp.1255.
- the convertible has a stepped or variable coupon
  structure (escalating coupon, payment-in-kind, etc.) that
  complicates the constant-`r_market` assumption in the
  recursion. **Source:** Kieso (2020) Ch.16 pp.1254-1257.
- the issuer's prior-period amortization is restated due
  to a discount-rate measurement error or a transaction-
  cost reallocation. **Source:** Kieso (2020) Ch.16 pp.1254-1257.
- the convertible carries non-standard amortization features
  (catch-up amortization on prepayment, soft-call-triggered
  acceleration) that the current scope defers. **Source:**
  Kieso (2020) Ch.16 pp.1254-1257.
