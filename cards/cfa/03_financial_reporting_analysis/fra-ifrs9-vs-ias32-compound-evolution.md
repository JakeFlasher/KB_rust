---
schema_version: "cacg.v0"
id: "fra-ifrs9-vs-ias32-compound-evolution"
title: "IAS-32-vs-IFRS-9 Classification Boundary"
reading_id: "03_financial_reporting_analysis"
summary: "Framing the issuer-side classification boundary between IAS 32 compound-instrument split (mandatory liability/equity at issuance) and IFRS 9 embedded-derivative bifurcation (host + FVTPL embedded derivative). The fixed-for-fixed criterion gates which path applies; the consequence is low-volatility amortized-cost reporting vs high-volatility FVTPL."
tags: ["financial-reporting", "ifrs9-ias32"]
citations:
  - source_id: "fra_hkicpa_hkas_32_2022"
    chunk_id: "fra_hkicpa_hkas_32_2022:p018:0027"
    chunk_hash: "6f45f9c881d8b4e5623777feed11ef1735a29759e47e207e005d3800dc97eb1d"
    page_range: [18, 18]
    quote: "The sum of the carrying amounts assigned to the liability and equity components on initial recognition is always equal to the fair value that would be ascribed to the instrument as a whole."
    edge_type: "defines"
  - source_id: "fra_hkicpa_hkas_32_2022"
    chunk_id: "fra_hkicpa_hkas_32_2022:p033:0056"
    chunk_hash: "381e984511295061e9db5d95b57914774f2c183100765c30cd797fca727d06c2"
    page_range: [33, 33]
    quote: "Compound financial instruments (paragraphs 28-32) AG30 Paragraph 28 applies only to issuers of non-derivative compound financial instruments."
    edge_type: "supports"
  - source_id: "fra_hkicpa_hkfrs_9_2024"
    chunk_id: "fra_hkicpa_hkfrs_9_2024:p020:0031"
    chunk_hash: "bd3813438110278317a013355ae92a8f3bd2be9fb58626219d91b69d031d158e"
    page_range: [20, 21]
    quote: "an embedded derivative shall be separated from the host and accounted for as a derivative under this Standard"
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1641:2376"
    chunk_hash: "f36e61b35cfc0da04c3fbe4148a8fee0dd897a9c629dde85f44ba84a5799cbb8"
    page_range: [1641, 1641]
    quote: "Convertible debt gives the debt holder the option to exchange the debt for equity."
    edge_type: "supports"
card_hash: "ec12989083170b9a3d2c125ef911656ea1754d2a80ce0804b00da3002344500e"
---
# IAS-32-vs-IFRS-9 Classification Boundary

## Intuition

A convertible bond's accounting fate depends on a single
classification question: does the conversion right qualify as
*equity* under IAS 32, or must it be carved out as an *embedded
derivative* under IFRS 9? The answer determines whether the
issuer reports a quiet amortized-cost liability with a constant
equity component (the IAS 32 path covered upstream in
[`fra-issuer-side-compound-instrument-split`](./fra-issuer-side-compound-instrument-split.md)),
or a host debt instrument bundled with a fair-value-through-
profit-or-loss embedded derivative that bounces with every
shareprice and rate move (the IFRS 9 path).
**Source:** HKICPA HKAS 32 (2022) pp.17-19; HKICPA HKFRS 9 (2024) pp.20-23.

The IAS 32 equity-classification gate is narrow by design.
HKAS 32 paragraph 28 requires the issuer to evaluate the terms
of a non-derivative instrument to determine whether it contains
both a liability and an equity component; paragraph 29 makes
clear that the equity component must "grant an option to the
holder to convert it into a fixed number of ordinary shares of
the entity." If the conversion feature is anything OTHER than a
fixed-number-of-shares-for-a-fixed-amount-of-cash exchange — for
example, a variable conversion ratio, a contingent reset that
depends on future share price, or a settlement option that lets
the issuer pay cash instead of shares — the conversion right
ceases to meet the equity definition, the IAS 32 path is closed,
and HKFRS 9 §4.3 takes over with embedded-derivative bifurcation.
**Source:** HKICPA HKAS 32 (2022) pp.17-18.

```
+----------------------------------------------------------+
|  Classification gate: IAS 32 equity vs IFRS 9 derivative |
+----------------------------------------------------------+
|                                                          |
|       Convertible debt instrument at issuance            |
|                       |                                  |
|             does the conversion right                    |
|             exchange a FIXED number of                   |
|             shares for a FIXED amount                    |
|             of cash?                                     |
|                       |                                  |
|         +---YES-------+-------NO----+                    |
|         |                           |                    |
|    IAS 32 path                IFRS 9 path                |
|    (compound instrument)      (host + embedded deriv.)   |
|         |                           |                    |
|    liability + equity         host bond at amortized     |
|    split at issuance;         cost; embedded derivative  |
|    equity component           bifurcated and measured    |
|    is RESIDUAL (P - L_0)      at FVTPL                   |
|    and stays constant         each reporting period      |
|                                                          |
|  Reporting-quality consequence:                          |
|    IAS 32 = low-volatility income statement              |
|    IFRS 9 = high-volatility income statement             |
+----------------------------------------------------------+
```

**Source:** HKICPA HKAS 32 (2022) pp.17-18; HKICPA HKFRS 9 (2024) pp.20-21.

## Definition

The IAS 32 equity-classification test for the conversion right
is the "fixed-for-fixed" criterion (the term commonly used in
practice for the HKAS 32 paragraph 29 wording). A convertible
bond's embedded conversion option qualifies as an equity
instrument if and only if the conversion will exchange a FIXED
number of the issuer's ordinary shares for a FIXED cash amount
(or, equivalently, for a fixed amount of another financial
asset). If either side of that exchange is variable, the option
fails the test and is reclassified. **Source:** HKICPA HKAS 32
(2022) pp.17-18.

Common patterns that BREAK the fixed-for-fixed criterion:
variable conversion ratios that flex with market share price;
contingent-adjustment provisions that reset the conversion price
on antidilution or down-round triggers; settlement features that
let the issuer or holder elect cash settlement equal to the
share value; and any contractual link of the conversion amount
to a price index, a foreign-currency rate, or another variable
that is not specific to a party to the contract. **Source:**
HKICPA HKAS 32 (2022) pp.17-18.

HKFRS 9 paragraph 4.3.3 then prescribes the bifurcation rule
when the IAS 32 equity test fails. An embedded derivative shall
be separated from the host and accounted for as a derivative if
and only if three conditions all hold: (a) the economic
characteristics and risks of the embedded derivative are NOT
closely related to those of the host; (b) a separate instrument
with the same terms as the embedded derivative would meet the
definition of a derivative; and (c) the hybrid contract is not
itself measured at fair value through profit or loss. For a
conversion feature on a debt host, (a) typically holds (the
equity-linked payoff is not closely related to the debt-service
host) and (b) typically holds (the standalone conversion option
would be a derivative), so the bifurcation triggers unless the
issuer elects fair-value-option treatment for the entire hybrid
contract under HKFRS 9 §4.3.5. **Source:** HKICPA HKFRS 9 (2024)
pp.20-22.

```
+--------------------------------------------------------+
|  HKFRS 9 §4.3.3 bifurcation gate                       |
+--------------------------------------------------------+
|                                                        |
|  Three cumulative conditions for separation:           |
|                                                        |
|  (a) economic characteristics + risks of embedded      |
|      derivative NOT closely related to host            |
|  (b) standalone instrument with same terms WOULD       |
|      meet the definition of a derivative               |
|  (c) hybrid contract is NOT measured at FVTPL          |
|                                                        |
|  All three must hold ⇒ bifurcate; otherwise stay      |
|  unified (host accounted for under appropriate         |
|  standard; embedded derivative not separated).         |
|                                                        |
|  Override (§4.3.5): entity may DESIGNATE entire        |
|  hybrid contract as at FVTPL, eliminating bifurcation  |
|  (subject to two exceptions: derivative does not       |
|  significantly modify cash flows, or separation is     |
|  clearly prohibited).                                  |
+--------------------------------------------------------+
```

**Source:** HKICPA HKFRS 9 (2024) pp.20-22.

The classification-vs-measurement layering is essential to the
analyst's reading. IAS 32 (Presentation) decides the
CLASSIFICATION of each component: which goes to liability, which
to equity, and whether bifurcation is required. HKFRS 9
(Financial Instruments) decides the MEASUREMENT of each
component once classified: amortized cost vs FVTPL for the
liability host, fair value for any separated embedded
derivative, and equity components stay at original carrying value
through life under IAS 32. HKAS 32 paragraph 31 cross-references
this division explicitly: "HKFRS 9 deals with the measurement of
financial assets and financial liabilities." **Source:** HKICPA
HKAS 32 (2022) pp.18.

## Mathematical Reasoning

The income-statement consequence of misclassification is the
real analytical bite. Under the IAS 32 path (fixed-for-fixed
passes), the issuer reports each period a stable reported
interest expense `Interest_t = r_market · L_{t-1}` from the
amortized-cost liability (see
[`fra-effective-interest-amortization-bond-side`](./fra-effective-interest-amortization-bond-side.md))
plus zero P&L from the equity component, which sits at constant
carrying amount `E_0 = P − L_0` through life. Total annual P&L
attributable to the convertible: `Interest_t` only. **Source:**
HKICPA HKAS 32 (2022) pp.18.

Under the IFRS 9 path (fixed-for-fixed fails), the same issuer
reports each period `Interest_t = r_market · L_{t-1,host}` from
the host debt liability PLUS a fair-value-change line item on the
separately-recognized embedded derivative. Because the embedded
derivative on the issuer's side is a derivative LIABILITY (HKAS 32
paragraph 27 explicitly classifies a derivative that the issuer
can settle by exchanging shares for cash as a financial liability
unless all settlement alternatives result in equity classification),
the P&L sign is inverted relative to a derivative asset: a
share-price rally pushes the derivative-liability fair value UP
(the conversion right is more valuable to the holder, so the
issuer's obligation is more onerous) and recognizes a LOSS to the
issuer; a share-price drop or volatility collapse pushes the
derivative-liability fair value DOWN and recognizes a GAIN. The
loss side is large (the derivative-liability fair value can grow
without an explicit ceiling as the share price rises); the gain
side is bounded by the carrying value of the derivative liability
at the start of the period (the liability cannot fall below zero,
so the maximum reportable gain in any period equals the
prior-period derivative-liability balance). The recognition
mechanic is the HKFRS 9 §4.3.3 separation rule combined with the
§4.2.1(a) subsequent-measurement requirement that financial
liabilities at fair value through profit or loss, including
derivatives that are liabilities, are subsequently measured at
fair value. Total annual P&L attributable to the convertible:
`Interest_t,host − ΔFV_t,deriv,liability` (loss when liability
fair value rises; gain when it falls), which can swing materially
even when no transaction occurs. **Source:** HKICPA HKAS 32 (2022)
pp.17 (paragraph 27, derivative-liability classification of
issuer-settled-cash-or-shares contracts); HKICPA HKFRS 9 (2024)
pp.19 (paragraph 4.2.1(a), FVTPL subsequent-measurement of
derivative liabilities) and pp.20-22 (paragraph 4.3.3, embedded-
derivative separation rule).

```
+--------------------------------------------------------+
|  Annual P&L decomposition by classification regime     |
+--------------------------------------------------------+
|                                                        |
|  IAS 32 path (fixed-for-fixed passes):                 |
|     P&L_t = Interest_t                                 |
|           = r_market · L_{t-1}                         |
|     (deterministic given r_market and L_{t-1};         |
|      smooth amortization trajectory)                   |
|                                                        |
|  IFRS 9 path (fixed-for-fixed fails):                  |
|     P&L_t = Interest_t,host − ΔFV_t,deriv,liability    |
|           = r_market,host · L_{t-1,host}               |
|             − (FV_t,deriv − FV_{t-1},deriv)            |
|     (the −ΔFV term reflects derivative-LIABILITY sign: |
|      share-price up ⇒ liability FV up ⇒ LOSS;          |
|      share-price down ⇒ liability FV down ⇒ GAIN;      |
|      driven by share price + implied volatility +      |
|      risk-free rate movements between reporting        |
|      dates; fair-value recognition every period via    |
|      HKFRS 9 §4.3.3 separation)                        |
|                                                        |
|  Comparative volatility:                               |
|     σ(P&L_IAS32)  ≪  σ(P&L_IFRS9)                      |
|     because the ΔFV term has stochastic mark-to-market |
|     content absent from the IAS 32 amortized-cost path |
+--------------------------------------------------------+
```

**Source:** HKICPA HKAS 32 (2022) pp.18; HKICPA HKFRS 9 (2024)
pp.20-22.

The settlement-event accounting also differs. Under IAS 32 (per
HKAS 32 AG32 + AG33), conversion at maturity derecognizes the
liability component and reclassifies the original equity
component within equity, with NO P&L impact; early extinguishment
allocates the consideration paid between liability and equity
using the original split mechanic, recognizing any gain or loss
on the liability leg in profit or loss while the equity-leg
adjustment stays within equity. Under IFRS 9, when the embedded
derivative is separated, the host debt follows ordinary
amortized-cost-derecognition mechanics on settlement and the
embedded derivative settles at its fair value; conversion of the
underlying hybrid is treated as the simultaneous derecognition
of the host plus the embedded derivative, with the cumulative
fair-value-change history already recognized period-by-period in
P&L. **Source:** HKICPA HKAS 32 (2022) pp.33-34.

The carve-out of AG30 is also worth holding: "Paragraph 28
applies only to issuers of non-derivative compound financial
instruments. Paragraph 28 does not deal with compound financial
instruments from the perspective of holders." The IAS 32 split
is asymmetric — issuer-side only — and the holder's accounting
runs through HKFRS 9's classification framework (amortized cost
vs FVOCI vs FVTPL based on business model and SPPI test).
**Source:** HKICPA HKAS 32 (2022) pp.33.

## See Also

- [`fra-issuer-side-compound-instrument-split`](./fra-issuer-side-compound-instrument-split.md) — the upstream IAS 32 split that this card's IFRS 9 alternative replaces when the fixed-for-fixed criterion fails
- [`fra-effective-interest-amortization-bond-side`](./fra-effective-interest-amortization-bond-side.md) — the amortized-cost mechanics that apply to the liability host under either classification regime
- [`fra-conversion-extinguishment-accounting`](./fra-conversion-extinguishment-accounting.md) — the IAS 32 settlement-path mechanics referenced for the comparative IFRS 9 settlement treatment
- [`fra-ifrs-vs-us-gaap-framework`](./fra-ifrs-vs-us-gaap-framework.md) — the broader IFRS-vs-US-GAAP context (ASC 470-20 historically did NOT bifurcate convertibles; the boundary discipline differs across regimes)
- [`fra-non-current-liabilities`](./fra-non-current-liabilities.md) — the general non-current-liability framework that applies to the host debt instrument after bifurcation
- [`cb-credit-vs-equity-decomposition`](../08_convertible_bonds/cb-credit-vs-equity-decomposition.md) — the investor-side decomposition perspective on the same classification distinction

## Escalate to Raw When

Open HKICPA HKAS 32 (pp.17-19 + pp.33-34) and HKICPA HKFRS 9 (pp.20-23)
directly when any of the criteria below applies. **Source:** HKICPA
HKAS 32 (2022) pp.17-19; HKICPA HKFRS 9 (2024) pp.20-23.

- the convertible's conversion ratio explicitly varies with the
  share price (e.g., a reset feature, a contingent
  conversion-price adjustment, or a market-based ratchet), and
  the analyst must determine whether the fixed-for-fixed test
  fails outright or whether a safe-harbor anti-dilution clause
  preserves equity classification. **Source:** HKICPA HKAS 32
  (2022) pp.17-18.
- the issuer has elected the HKFRS 9 §4.3.5 fair-value-option
  override on the entire hybrid contract and the analyst must
  reconstruct the carrying-value trajectory from the FVTPL
  movements. **Source:** HKICPA HKFRS 9 (2024) pp.21-22.
- the convertible has multiple interdependent embedded
  derivatives (e.g., a conversion option PLUS an issuer call
  PLUS a put), where HKFRS 9 §4.3.7 may apply because individual
  fair values cannot be reliably measured. **Source:** HKICPA
  HKFRS 9 (2024) pp.22-23.
- the issuer's instrument has a contingent-settlement provision
  in the conversion mechanic (e.g., cash settlement on certain
  outcomes), and the analyst must apply HKAS 32 paragraph 25 to
  determine whether the financial-liability classification
  applies. **Source:** HKICPA HKAS 32 (2022) pp.17.
