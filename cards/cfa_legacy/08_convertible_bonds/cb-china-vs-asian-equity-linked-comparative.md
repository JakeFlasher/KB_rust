---
schema_version: "cacg.v0"
id: "cb-china-vs-asian-equity-linked-comparative"
title: "China vs Asian Equity-Linked Products Comparative"
reading_id: "08_convertible_bonds"
summary: "The Clifford Chance Asia-Pacific equity-linked-products guide (2nd ed Feb 2025) frames how Chinese onshore CBs compare to Asian peer products: Japan's 新株予約権付社債 (bonds with stock acquisition rights, named under 会社法), Hong Kong's HKEX-listed convertible-debt under Ch.28, and the broader regional offshore-USD-CB market. Differences span: holder-option-vs-mandatory conversion, conversion-share-sour..."
tags: ["convertible-bonds", "china-asian"]
citations:
  - source_id: "china_cb_clifford_chance_apac_equity_linked_2025"
    chunk_id: "china_cb_clifford_chance_apac_equity_linked_2025:p004:0003"
    chunk_hash: "16573fe5dcb83eaf85056e4d11443f2241a9ada92f9b18f85806c47849ddd71f"
    page_range: [4, 5]
    quote: "Conversion is usually at the holder’s option, although occasionally conversion may be mandatory on a specified future date."
    edge_type: "defines"
  - source_id: "china_cb_hkex_ch28_convertible_debt"
    chunk_id: "china_cb_hkex_ch28_convertible_debt:p001:0000"
    chunk_hash: "3b3a1ed50b0d93d93e76a3e010dd975f9a58eb0bd4a7dff66b12f3a8c469d548"
    page_range: [1, 1]
    quote: "All convertible debt securities must, prior to the issue thereof, be approved by the Exchange and the Exchange should be consulted at the earliest opportunity as to the requirements which will apply."
    edge_type: "supports"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p041:0045"
    chunk_hash: "53fc3079dc48462a65dff49f335124d01c4ec34bcb4e2029a3a7eadf661488ad"
    page_range: [41, 42]
    quote: "2.2 ANATOMY OF A CONVERTIBLE BOND"
    edge_type: "supports"
card_hash: "b4be125ffac5b01df10d907d22585c7942d7d7027ba07694a26026896c4217a6"
---
# China vs Asian Equity-Linked Products Comparative

## Intuition

The Chinese onshore convertible bond is one of four structurally-
related Asia-Pacific equity-linked product families that Clifford
Chance's 2025 practitioner guide classifies. All four jurisdictions
— Japan, Hong Kong, Singapore, and China onshore — share the
embedded-option convertible-bond structure (a fixed-income bond
plus a non-detachable conversion option on the issuer's underlying
equity). The Japanese 新株予約権付社債 ("bonds with stock
acquisition rights") is structurally a convertible bond named per
the 会社法 Companies Act convention that frames the conversion
right as a "stock acquisition right" (新株予約権) attached to the
host bond; the English-language phrase "convertible bond" applied
to Japanese instruments is therefore appropriate at the structural
level (the legal-framework naming is the cross-jurisdictional
distinction, not the embedded-vs-detachable structural axis).
A separate Japanese product line, the 分離型新株予約権付社債
(detachable variant), allows the warrant to be detached and
traded independently of the host bond, but this is a distinct
product class and is OUT OF SCOPE for this card per the
out-of-scope frontmatter. The Hong Kong convertible-debt
instrument under HKEX Main Board Ch.28 and the Singapore
convertible securities under SGX-ST follow the same embedded-
option structural pattern. The Chinese onshore CB under
SSE/SZSE rules carries TWO additional China-specific structural
features absent in the other three jurisdictions: the mandatory
issuer-side downward-reset (下修) on the conversion price, and
the strong-call (强赎) trigger that converts a soft-call into a
mandatory call when parity exceeds the trigger threshold for the
rule-specified number of trading days. The Clifford Chance guide
treats all four as members of the "equity-linked products"
superset and documents the legal-framework + Chinese-specific
structural differences in issuance mechanics, regulatory
framework, and secondary-market mechanics. **Source:** Clifford
Chance APAC 2e (Feb 2025) §1-§4 pp.1-30; DeSpiegeleer (2014)
§2 pp.21-30; Zubulake (1991) §1-§3 pp.5-50.

```
   Asian equity-linked products taxonomy
   -------------------------------------
   (all four use embedded-option CB structure;
    legal-framework + Chinese-specific features distinguish them)

   Japan         新株予約権付社債  =  bond  ⊕  embedded conversion option
                                  (named per 会社法 Companies Act
                                   convention; standard product line.
                                   分離型 detachable variant: OUT OF
                                   SCOPE)

   Hong Kong     Convertible bond  =  bond  ⊕  embedded conversion option
                                  (under HKEX Ch.28 listing)

   Singapore     Convertible       =  bond  ⊕  embedded conversion option
                 securities         (under SGX-ST listing)

   China onshore Convertible bond  =  bond  ⊕  embedded conversion option
                                       ⊕  mandatory 下修 reset
                                       ⊕  strong-call (强赎) trigger
                                  (under SSE/SZSE rules — most
                                   issuer-friendly of the four)
```

## Definition

The four jurisdictions covered in this card. **Source:** Clifford
Chance APAC 2e (Feb 2025) §1-§4 pp.1-30.

- **Japan 新株予約権付社債 (bond with stock acquisition rights)**:
  structurally a convertible bond — a fixed-income host bond
  with an embedded (non-detachable) stock-acquisition right
  (新株予約権) that the holder may exercise to acquire newly-
  issued shares from the issuer. The name reflects the 会社法
  Companies Act naming convention that frames the conversion
  feature as a "stock acquisition right" attached to the bond,
  rather than a substantive structural difference from the
  embedded-option CBs of the other three jurisdictions in this
  card. The English-language phrase "convertible bond" applied
  to the standard Japanese instrument is therefore appropriate
  at the structural level. The Japanese Companies Act (会社法)
  governs the issuance mechanics, and the Financial Instruments
  and Exchange Act (金商法) governs the secondary-market
  disclosure. A separate Japanese product line — the 分離型
  新株予約権付社債 (detachable-warrant variant) — is structurally
  distinct because the warrant is independently transferable;
  this variant is out of scope for this card (cross-Asian
  standard-CB comparison only). **Source:** Clifford Chance APAC
  2e (Feb 2025) §2 pp.5-15; DeSpiegeleer (2014) §2 pp.21-30.

- **Hong Kong convertible debt (HKEX Main Board Ch.28)**: bond
  plus embedded (non-detachable) conversion option. The HKEX
  Listing Rules Ch.28 codifies the listing requirements,
  including disclosure, conversion-price adjustment, and
  redemption mechanics. The structural shape mirrors the
  Anglo-American CB tradition (per
  [`cb-china-hkex-offshore-cb-comparison`](cb-china-hkex-offshore-cb-comparison.md#definition)).
  **Source:** Clifford Chance APAC 2e (Feb 2025) §3 pp.15-22;
  HKEX Main Board Ch.28 pp.1-2.

- **Singapore convertible securities (SGX-ST listing)**: same
  structural shape as HKEX (bond plus embedded option), governed
  by SGX-ST listing rules. Some smaller-cap variants use a
  warrant-detachable structure that approaches the Japanese
  pattern but the dominant convention is embedded-option.
  **Source:** Clifford Chance APAC 2e (Feb 2025) §4 pp.22-30.

- **China onshore CB (SSE/SZSE)**: bond plus embedded conversion
  option PLUS two China-specific structural features absent in
  the other three jurisdictions: mandatory downward-conversion
  reset (下修) on the conversion price under issuer-board +
  shareholder-vote control, and the strong-call (强赎) trigger
  that converts a soft-call into a mandatory call when parity
  exceeds the trigger threshold for the rule-specified number of
  trading days. These two features make Chinese CBs structurally
  the most issuer-friendly of the four jurisdictions. **Source:**
  Clifford Chance APAC 2e (Feb 2025) §1 pp.1-5; DeSpiegeleer
  (2014) §2 pp.21-30 (international classification framework).

## Mathematical Reasoning

The shared embedded-option structure of all four jurisdictions'
standard CBs is captured by the value-decomposition identity
common to Japan, Hong Kong, Singapore, and China onshore.
**Source:** Clifford Chance APAC 2e (Feb 2025) §1-§4 pp.1-30;
DeSpiegeleer (2014) §2 pp.21-30.

```
embedded-option CB (all four jurisdictions — standard product):
  V_total = V_bond + V_embedded_conversion_option

  V_embedded cannot be detached; the bondholder receives the
  joint payoff at maturity (or earlier conversion event).

  Jurisdiction-specific naming:
    Japan       : 新株予約権付社債 (Companies Act naming;
                  conversion right named "stock acquisition right")
    Hong Kong   : convertible debt (HKEX Ch.28 listing)
    Singapore   : convertible securities (SGX-ST listing)
    China onshore: 可转债 (SSE/SZSE rules)

separate Japanese product line — OUT OF SCOPE for this card:
  分離型新株予約権付社債 (detachable variant):
    V_total = V_bond + V_warrant_detachable
    (warrant independently transferable after detachment)
```

**Source:** Clifford Chance APAC 2e (Feb 2025) §2 pp.5-15;
DeSpiegeleer (2014) §2 pp.21-30.

The mathematical consequence is that all four jurisdictions'
standard CBs admit the same single-combined-market-price
decomposition that practitioners apply using the bond-plus-call
identity covered in
[`cb-payoff-decomposition-bond-plus-call`](cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning).
The Japanese 分離型 detachable variant (separate product line, out
of scope) would generalize to a bond-plus-tradable-warrant two-
asset model after detachment, but the standard 新株予約権付社債
treated in this card reduces to the single-price embedded-option
decomposition. **Source:** Clifford Chance APAC 2e (Feb 2025) §2
pp.10-15.

The China-specific 下修 and 强赎 features add two structural
modifications to the embedded-option payoff. **Source:** Clifford
Chance APAC 2e (Feb 2025) §1 pp.1-5; DeSpiegeleer (2014) §4
pp.110-180.

```
Chinese CB embedded-option payoff modifications:

  base embedded-option payoff (per HK / Singapore baseline):
    V_HK = V_bond + V_embedded_call(S, K_c, σ, r, q, τ)
  
  China-specific reset (下修):
    K_c → K_c'(t_reset) where K_c'(t_reset) is set by
                            issuer-board proposal + shareholder
                            vote, floored at recent-price floor
  
  China-specific strong-call (强赎):
    if Σ_{i=t-29}^{t} 1{S(i) ≥ 1.30 · K_c} ≥ 15:
      issuer may invoke strong-call,
      converting V_embedded_call to mandatory
      redemption-or-conversion choice for the holder
```

**Source:** Clifford Chance APAC 2e (Feb 2025) §1 pp.1-5;
DeSpiegeleer (2014) §4 pp.110-180 (international taxonomy framework
that places these Chinese features alongside Western soft-call /
reset variants).

The cross-jurisdictional consequence: Chinese onshore CB
valuation requires explicit modeling of both the 下修 reset and
the 强赎 strong-call trigger as additional state variables in
the embedded-option pricing, whereas Japan (standard 新株予約
権付社債) / HK / Singapore CBs reduce to the baseline bond-plus-
embedded-call decomposition. The Japanese standard product is
structurally on par with the HK and Singapore products in
valuation terms — the legal-framework naming convention does
not create a valuation-modeling difference at the standard-CB
level. (The 分離型 detachable variant, a separate Japanese
product line out of scope for this card, would require two-asset
modeling once detachment is exercised.) **Source:** Clifford
Chance APAC 2e (Feb 2025) §1-§2 pp.1-15.

## See Also

- [`cb-china-hkex-offshore-cb-comparison.md`](cb-china-hkex-offshore-cb-comparison.md) — China-onshore vs HK-offshore deeper comparison; this card extends to the full Asia-Pacific four-jurisdiction comparative
- [`cb-bond-anatomy-and-cashflows.md`](cb-bond-anatomy-and-cashflows.md) — baseline CB anatomy that the cross-jurisdictional comparison varies upon
- [`cb-mandatory-and-exotic-structures.md`](cb-mandatory-and-exotic-structures.md) — Western mandatory + exotic CB structures (PEPS / DECS / ACES / ELKS / CoCos) that contrast with the Asian taxonomy
- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — bond-plus-embedded-call identity that all four standard CBs in this card satisfy (Japan 新株予約権付社債, HK Ch.28, Singapore SGX-ST, China onshore); the Japanese 分離型 detachable variant generalizes to bond + tradable warrant but is out of scope for this card
- [`cb-china-downward-conversion.md`](cb-china-downward-conversion.md) — China-specific 下修 reset mechanism documented in the China-only column of the comparative table

## Escalate to Raw When

Open Clifford Chance Guide to Equity-Linked Products in Asia
Pacific 2e (Feb 2025) §1-§4 pp.1-56 directly for the cross-
jurisdictional practitioner taxonomy covering China onshore (§1),
Japan (§2), Hong Kong (§3), and Singapore (§4) issuance mechanics,
regulatory framework, disclosure conventions, and secondary-
market structure. **Source:** Clifford Chance APAC 2e (Feb 2025)
§1-§4 pp.1-56.

Open DeSpiegeleer (2014) Handbook of Hybrid Securities §2 pp.21-30
for the international classification framework that places
Japanese / HK / Singapore / Chinese instruments alongside Western
convertible variants (PEPS, DECS, CoCos), grounding the
cross-jurisdictional taxonomy in the European-American
practitioner literature. **Source:** DeSpiegeleer (2014) §2
pp.21-30.

Open Zubulake (1991) Complete Guide to Convertible Securities
Worldwide §1-§3 pp.5-50 for the historical cross-jurisdictional
overview that documents the pre-2010 Asian CB markets and the
evolution into the current four-jurisdiction structure documented
in Clifford Chance 2e. **Source:** Zubulake (1991) §1-§3
pp.5-50.
