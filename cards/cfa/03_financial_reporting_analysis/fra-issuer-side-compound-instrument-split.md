---
schema_version: "cacg.v0"
id: "fra-issuer-side-compound-instrument-split"
title: "Issuer-Side Compound-Instrument Split (IAS 32)"
reading_id: "03_financial_reporting_analysis"
summary: "Framing IAS 32's mandatory split of convertible-bond issue proceeds into a liability component (PV of contractual cash flows at market rate on comparable non-convertible debt) and an equity component (residual). The split uses the with-and-without method; the equity component carries no subsequent fair-value remeasurement."
tags: ["financial-reporting", "issuer-side"]
citations:
  - source_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed"
    chunk_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed:p1253:1368"
    chunk_hash: "a57f2f81659bde5d685c8c21ef57137759a88120612588fd2611cf65ca85dfcc"
    page_range: [1253, 1254]
    quote: "ILLUSTRATION 16.1 Convertible Debt Components As indicated, the equity component is the residual amount after subtracting the liability component."
    edge_type: "defines"
  - source_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed"
    chunk_id: "fra_kieso_weygandt_warfield_2020_ifrs_4ed:p1253:1368"
    chunk_hash: "a57f2f81659bde5d685c8c21ef57137759a88120612588fd2611cf65ca85dfcc"
    page_range: [1253, 1254]
    quote: "First, determine the total fair value of the convertible debt with both the liability and equity component."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1622:2347"
    chunk_hash: "a5aec3f7c68a323a28f681fb628fe1a34a3b1bd9370b1f699f708d8c61f55e85"
    page_range: [1622, 1623]
    quote: "Periodic interest payments are made based on the interest rate promised in the bond contract applied to the bonds’ face value."
    edge_type: "supports"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p319:0526"
    chunk_hash: "8a28bca47536dfca2143ab777a12de6daa7c7ad49c2473c3e53108f6152aaca6"
    page_range: [319, 320]
    quote: "REFORMULATION OF THE BALANCE SHEET The typical balance sheet usually divides assets and liabilities into current and noncurrent (long-term) categories."
    edge_type: "supports"
card_hash: "88eca1c63aa88dc0452b54738e39c1975b6bfbb32917633be1b39d9f3a6034ad"
---
# Issuer-Side Compound-Instrument Split (IAS 32)

## Intuition

A convertible bond issued by a corporation is two things at once:
a contractual obligation to pay coupons and principal in cash
(the debt leg) AND the holder's right to convert into a fixed
number of the issuer's ordinary shares (the equity leg). IAS 32
treats this dual nature as definitional: a convertible bond is
a compound financial instrument, and the issuer must split the
issue proceeds into a liability component and an equity
component on day one. The split is mandatory, not elective.
**Source:** Kieso (2020) Ch.16 pp.1253-1255.

The intuition behind the split is that the cash received at
issuance bundles two distinct rights the issuer has sold: a
debt-service obligation (which would, alone, command some
market price) and a conversion option (which would, alone,
command a separate market price). Accounting rules require
both rights to be visible on the balance sheet so that
downstream analyses — leverage ratios, interest-coverage
measurement, share-dilution exposure — can read each leg
separately. **Source:** Kieso (2020) Ch.16 pp.1253-1255.

```
+--------------------------------------------------+
|  Single instrument carries dual nature           |
+--------------------------------------------------+
|                                                  |
|  Convertible Bond  =  Debt obligation  +         |
|                       Conversion right           |
|                                                  |
|  IAS 32 mandate:                                 |
|     Issue proceeds P split between               |
|       Liability (Bonds Payable) AND              |
|       Equity (Share Premium — Conversion Equity) |
|                                                  |
|  The split is mandatory; entire-debt or          |
|  entire-equity classification is prohibited.     |
+--------------------------------------------------+
```

**Source:** Kieso (2020) Ch.16 pp.1253-1255.

## Definition

A compound financial instrument under IAS 32 is a single
non-derivative instrument that contains both a liability
component and an equity component from the issuer's
perspective. A convertible bond is the canonical example:
the contractual cash-flow obligation (coupons + principal)
is the liability component; the holder's right to convert
into a fixed number of ordinary shares is the equity
component. IFRS requires issuers to separate the two and
record them in distinct accounts. **Source:** Kieso (2020)
Ch.16 pp.1253-1255.

The procedure is the "with-and-without" method. IFRS
prescribes valuing the liability first and deriving the
equity as the residual; the reverse is prohibited because
equity is itself defined as a residual interest (assets
minus liabilities). **Source:** Kieso (2020) Ch.16 pp.1253-1255.

Step 1 — identify total fair value at issuance: this is
the proceeds `P`. **Source:** Kieso (2020) Ch.16 pp.1254.

Step 2 — compute the liability component `L_0` as the PV
of all contractual cash flows discounted at the market
rate `r_market` for a comparable non-convertible bond.
**Source:** Kieso (2020) Ch.16 pp.1254.

Step 3 — compute the equity component `E_0` as the
residual `E_0 = P − L_0`. **Source:** Kieso (2020) Ch.16 pp.1254.

The journal entry on the issuance date debits Cash for the
proceeds and credits two distinct accounts: Bonds Payable
for the liability component, and Share Premium — Conversion
Equity for the residual equity component. **Source:** Kieso
(2020) Ch.16 pp.1254-1255.

```
<!-- primitive: cb-split-t-account source: _diagram_primitives.md -->

       Convertible Bond Issuance (issuer-side, IAS 32 split)

          +--------------------------+
   Cash   |  Issue Proceeds  =  P    |    (single inflow)
   (Dr.)  +--------------------------+
                       |
            split mandated by IAS 32.31
                       |
              +--------+--------+
              v                 v
   +----------------------+  +-------------------------+
   |  LIABILITY (Cr.)     |  |  EQUITY (Cr.)           |
   |  L_0 = PV(coupons +  |  |  E_0 = P - L_0          |
   |          face;       |  |  (residual; no          |
   |          r_market)   |  |   subsequent FV re-     |
   |  r_market = yield    |  |   measurement under     |
   |  on comparable non-  |  |   IAS 32.32)            |
   |  convertible bond    |  |                         |
   +----------------------+  +-------------------------+
              |                       (parked in Share
              |                        Premium - Conversion Equity)
              v
       subsequent measurement at amortized cost using
       effective-interest method (extends the t-account
       primitive's general amortized-cost convention)

  Identity (issuance): P = L_0 + E_0
  Asymmetry: L_0 is re-measured (amortization to face);
             E_0 stays at carrying value through life.
```

**Source:** Kieso (2020) Ch.16 pp.1254-1255.

## Mathematical Reasoning

The IAS 32 split produces a small set of identities the
analyst should keep in head when reading a convertible
issuer's balance sheet. **Source:** Kieso (2020) Ch.16 pp.1254.

```
+--------------------------------------------------------+
|  IAS 32 compound-instrument identities                 |
+--------------------------------------------------------+
|                                                        |
|  At the issuance date:                                 |
|     P     =  L_naught  +  E_naught                     |
|     L_naught  =  PV( coupons + face ; r_market )       |
|     E_naught  =  P  minus  L_naught                    |
|                                                        |
|  Through life:                                         |
|     L_t   amortizes toward Face by effective-interest  |
|              (climbs each period in discount case;     |
|               falls each period in premium case)       |
|     E_t   =  E_naught  (constant; no FV remeasurement) |
|                                                        |
|  At settlement (at-maturity paths 1 + 2):              |
|     Liability is derecognized at L_T = Face            |
|     (amortization has fully converged L_0 -> Face)     |
|  At settlement (pre-maturity paths 3 + 4 + induced):   |
|     Liability is derecognized at current L_t           |
|     (unamortized gap Face - L_t simply ceases; gap is  |
|      a discount when L_t < Face, a premium when L_t    |
|      > Face)                                           |
|  Equity component is reclassified within equity        |
|  (no FV remeasurement at any settlement event)         |
+--------------------------------------------------------+
```

**Source:** Kieso (2020) Ch.16 pp.1253-1257.

The settlement mechanics that consume the running `L_t`
input are documented in
[`fra-conversion-extinguishment-accounting`](./fra-conversion-extinguishment-accounting.md).
**Source:** Kieso (2020) Ch.16 pp.1255-1258.

The dynamic structure — `L_t` moves toward Face (climbing
in the common discount-issuance case, falling in the mirror
premium-issuance case), `E_t` stays constant — is what
distinguishes the IAS 32 treatment from a naive
"split-it-once-and-leave-everything-alone" approach AND
from a derivatives-pricing "mark-everything-to-market"
approach. The analyst's leverage ratio at any point should
use the current `L_t`, not face value (overstates leverage)
and not the original `L_0` (understates leverage mid-life).
**Source:** Kieso (2020) Ch.16 pp.1253-1256.

The income-statement consequence over the bond's full life:
total reported interest expense EXCEEDS total cash coupon
by exactly the amortized discount `Face − L_0` (which equals
the equity component `E_0` only at par issuance P = Face;
under general issuance E_0 = P − L_0 is distinct from
Face − L_0). The accounting captures the economic cost of
accreting the bond-side carrying amount from L_0 to Face,
spreading it across the bond's life via the amortization
trajectory rather than recognizing it on day one. **Source:**
Kieso (2020) Ch.16 pp.1254-1256.

## See Also

- [`fra-non-current-liabilities`](./fra-non-current-liabilities.md) — general bond-issuance recognition and effective-interest amortization context; the compound-instrument liability leg inherits this amortization mechanic
- [`fra-effective-interest-amortization-bond-side`](./fra-effective-interest-amortization-bond-side.md) — specialized companion card on the liability-component's amortization trajectory and effective-interest formula in the compound-instrument context
- [`fra-conversion-extinguishment-accounting`](./fra-conversion-extinguishment-accounting.md) — derecognition mechanics on the four settlement paths plus induced conversions
- [`fra-ifrs-vs-us-gaap-framework`](./fra-ifrs-vs-us-gaap-framework.md) — broader IFRS-vs-US-GAAP context; US GAAP ASC 470-20 historically did NOT bifurcate convertibles at issuance
- [`cb-bond-anatomy-and-cashflows`](../08_convertible_bonds/cb-bond-anatomy-and-cashflows.md) — investor / pricing perspective on the same instrument
- [`cb-credit-vs-equity-decomposition`](../08_convertible_bonds/cb-credit-vs-equity-decomposition.md) — investor-side decomposition mirroring the issuer-side IAS 32 split
- [`cb-issuer-motives`](../08_convertible_bonds/cb-issuer-motives.md) — issuer-motivation perspective on why corporations issue convertibles

## Escalate to Raw When

Open Kieso Ch.16 (pp.1253-1255) directly when any of the
criteria below applies. **Source:** Kieso (2020) Ch.16 pp.1253-1256.

- the analyst needs the worked Roche Group numerical
  example for cross-check of the with-and-without method
  arithmetic. **Source:** Kieso (2020) Ch.16 pp.1254-1255.
- the issuer has a multi-currency or convertible-with-
  embedded-FX-option structure that the current scope
  defers — Kieso Ch.16 introduces the basic IAS 32
  treatment, more specialized currency-translation
  interactions sit outside this card. **Source:** Kieso
  (2020) Ch.16 pp.1253-1256.
- the issuer's transaction-cost allocation across liability
  and equity components materially affects reported
  amortization (the with-and-without allocation
  proportion applies). **Source:** Kieso (2020) Ch.16 pp.1253-1256; cross-reference IAS 32 paragraph 28.
