---
schema_version: "cacg.v0"
id: "cb-china-hkex-offshore-cb-comparison"
title: "HKEX Offshore Convertible-Securities Listing Regime"
reading_id: "08_convertible_bonds"
summary: "HKEX Main Board Listing Rules carry two convertible-securities chapters: Ch.28 (convertible debt securities) and Ch.16 (convertible equity securities). The two share three core gates — Exchange-approval is required before issue, underlying shares must be listed (or admitted to a recognised market), and any post-issue alteration of terms requires Exchange approval unless taking effect under exis..."
tags: ["convertible-bonds", "china-hkex"]
citations:
  - source_id: "china_cb_hkex_ch28_convertible_debt"
    chunk_id: "china_cb_hkex_ch28_convertible_debt:p001:0000"
    chunk_hash: "3b3a1ed50b0d93d93e76a3e010dd975f9a58eb0bd4a7dff66b12f3a8c469d548"
    page_range: [1, 1]
    quote: "All convertible debt securities must, prior to the issue thereof, be approved by the Exchange and the Exchange should be consulted at the earliest opportunity as to the requirements which will apply."
    edge_type: "defines"
  - source_id: "china_cb_hkex_ch16_convertible_equity"
    chunk_id: "china_cb_hkex_ch16_convertible_equity:p001:0000"
    chunk_hash: "a624b5a772ad59909e7c712b3850820c7c6af31b619bc16c8ce120e9f6f88d1a"
    page_range: [1, 1]
    quote: "All convertible equity securities which are convertible into new securities or outstanding securities of the issuer or a company in the same group as the issuer must, prior to the issue thereof, be approved by the Exchange and the Exchange should be consulted at the earliest opportunity as to the requirements which will apply."
    edge_type: "supports"
  - source_id: "china_cb_clifford_chance_apac_equity_linked_2025"
    chunk_id: "china_cb_clifford_chance_apac_equity_linked_2025:p004:0003"
    chunk_hash: "16573fe5dcb83eaf85056e4d11443f2241a9ada92f9b18f85806c47849ddd71f"
    page_range: [4, 5]
    quote: "Conversion is usually at the holder’s option, although occasionally conversion may be mandatory on a specified future date."
    edge_type: "supports"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p041:0045"
    chunk_hash: "53fc3079dc48462a65dff49f335124d01c4ec34bcb4e2029a3a7eadf661488ad"
    page_range: [41, 42]
    quote: "2.2 ANATOMY OF A CONVERTIBLE BOND"
    edge_type: "supports"
card_hash: "08699137befeeaf64e16d0a3691d8f447ad3266d52537b1c7af97a7d64db4c0b"
---
# HKEX Offshore Convertible-Securities Listing Regime

## Intuition

The HKEX Main Board Listing Rules carry two convertible-securities
chapters: **Chapter 28 Convertible Debt Securities** (governs the
listing of convertible debt securities) and **Chapter 16 Convertible
Equity Securities** (governs the listing of convertible equity
securities). The two chapters apply by the classification of the
instrument: Ch.28 covers convertible debt securities (debt-form
convertibles); Ch.16 covers convertible equity securities (equity-
form convertibles). Both chapters share a common structural
template: (a) **Exchange approval gate** — all convertible securities
must be approved by the HKEX before issue, and the Exchange should
be consulted at the earliest opportunity; (b) **underlying-shares-
must-be-listed-or-recognised constraint** — the convertibles may be
listed only if the underlying shares are (or will become at the same
time) a listed class or are listed/dealt on another regulated stock
market recognised by the Exchange; (c) **alteration-of-terms
approval** — any alterations after issue must be approved by the
Exchange except where the alterations take effect automatically
under the existing terms. **Source:** HKEX Ch.28 §28.01-§28.06 pp.1-2;
HKEX Ch.16 §16.01-§16.04 pp.1-1.

```
   HKEX offshore-CB listing regime vs China-onshore CSRC regime
   ------------------------------------------------------------

   +-----------------------------------+  +-----------------------------------+
   |  HKEX Main Board (THIS CARD)      |  |  China onshore (SSE/SZSE/CSRC)    |
   |                                   |  |                                   |
   |  - Exchange-approval gate per     |  |  - CSRC approval + SSE/SZSE       |
   |    issue (Ch.28 §28.01 /          |  |    listing (see cb-china-csrc-    |
   |    Ch.16 §16.01)                  |  |    disclosure-timing.md)          |
   |                                   |  |                                   |
   |  - Underlying-shares must be      |  |  - Underlying-shares must be      |
   |    listed or recognised foreign-  |  |    listed on SSE/SZSE (no         |
   |    market-listed (Ch.28 §28.03 /  |  |    foreign-market recognition     |
   |    Ch.16 §16.02 open Exchange-    |  |    path for onshore CB)           |
   |    discretion clause)             |  |                                   |
   |                                   |  |                                   |
   |  - Alteration-of-terms requires   |  |  - Alteration-of-terms requires   |
   |    Exchange approval (Ch.28       |  |    shareholder + CSRC approval    |
   |    §28.05 / Ch.16 §16.03)         |  |    per prospectus + 下修 voting   |
   |                                   |  |                                   |
   |  - Convertible-debt requirements  |  |  - Trading rules + suitability /  |
   |    comply with both debt and      |  |    risk-disclosure / algo-trading |
   |    underlying-equity rules        |  |    framework (see cb-china-       |
   |    (Ch.28 §28.02 conflict rule:   |  |    exchange-price-limit-          |
   |    equity rules prevail)          |  |    suitability-rules.md)          |
   +-----------------------------------+  +-----------------------------------+
```

**Source:** HKEX Ch.28 §28.01-§28.06 pp.1-2; HKEX Ch.16 §16.01-§16.04
pp.1-1.

## Definition

The HKEX offshore-CB listing regime decomposes into three
rule-layer components shared between Ch.28 and Ch.16. First, the
**Exchange-approval gate**: every convertible-securities issue
must be approved by the HKEX prior to issue, with Ch.28 §28.01
carving out an exception for "convertible debt issues to
professional investors only" (those issues are not subject to
Ch.28) and Ch.16 §16.01 carrying no such carve-out; the Exchange
should be consulted at the earliest opportunity. Second, the
**underlying-shares-listed-or-recognised constraint**: convertible
securities may be listed only if the underlying shares are (or
will become at the same time) (1) a class of listed shares or
(2) a class of shares listed or dealt in on another regulated,
regularly operating, open stock market recognised by the Exchange;
the Exchange retains discretion in other circumstances; Ch.28 §28.03
adds a Hong-Kong-specific carve-out for "State" or "Supranational"
issuers. Third, the **alteration-of-terms approval requirement**:
any alterations in the terms after issue must be approved by the
Exchange except where the alterations take effect automatically
under the existing terms (this contrasts with the China-onshore
下修 mechanism in [cb-china-downward-conversion](./cb-china-downward-conversion.md)
which is governed by shareholder vote per the prospectus rather
than by exchange approval).
**Source:** HKEX Ch.28 §28.01 + §28.03 + §28.05 pp.1-2; HKEX Ch.16
§16.01 + §16.02 + §16.03 pp.1.

The Ch.28-specific **debt-vs-equity conflict rule** is the
structural feature that distinguishes convertible-debt-securities
treatment from convertible-equity-securities treatment. Ch.28
§28.02 requires convertible-debt-securities to comply BOTH with
the debt-securities-listing requirements AND with the
underlying-equity-securities requirements; in the event of
conflict or inconsistency between the two, the equity-securities
requirements prevail. This conflict-resolution rule does NOT
appear in Ch.16 (which treats convertibles uniformly within the
equity-securities regime). **Source:** HKEX Ch.28 §28.02 pp.1.

Each chapter references additional listing-document content rules
in separate appendices: Ch.16 §16.04 names "paragraph 19 of
Appendix D1A and paragraph 21 of Appendix D1B"; Ch.28 §28.06 names
"Paragraphs 19 to 31 of Appendix D1C". The appendix content itself
is NOT cited by this card directly because the appendices live
in separate HKEX rulebook PDFs; the listing-document content
rules are an Out-of-Scope item for this card. **Source:** HKEX
Ch.16 §16.04 pp.1; HKEX Ch.28 §28.06 pp.2.

## Mathematical Reasoning

The HKEX offshore-CB listing regime does not introduce new pricing
or valuation math. Its content is structural rule-layer
specification of the listing-approval and listing-document
process. The structural distinction between Ch.16 (convertible
equity securities) and Ch.28 (convertible debt securities) maps to
the cross-jurisdictional convertible-securities classification
treated in the international literature. **Source:** HKEX Ch.28
§28.01-§28.06 pp.1-2; HKEX Ch.16 §16.01-§16.04 pp.1-1;
DeSpiegeleer et al. (2014) §1 pp.21-30 (international
convertible-securities classification framework).

```
   HKEX convertible-securities classification gate (Ch.16 vs Ch.28)

   Issuer brings a convertible-securities proposal
                       |
                       v
        +--------------+--------------+
        |                             |
        v                             v
   Is the instrument a              Is the instrument a
   class of equity securities       debt security with an
   convertible into other           embedded equity-conversion
   equity securities?               option?
        |                             |
        v                             v
   Ch.16 Convertible                Ch.28 Convertible
   Equity Securities                Debt Securities
   §16.01-§16.04                    §28.01-§28.06
        |                             |
        v                             v
   Equity-securities                Comply with BOTH debt-securities
   regime applies                   AND underlying-equity-securities
                                    requirements; equity-securities
                                    requirements prevail in conflict
                                    (§28.02 conflict rule)
```

**Source:** HKEX Ch.28 §28.02 pp.1 (conflict rule); HKEX Ch.16
§16.01 pp.1 (Ch.16 scope statement); HKEX Ch.28 §28.01 pp.1
(Ch.28 scope + professional-investor carve-out).

The **comparison with China-onshore CSRC regime** distinguishes
HKEX's exchange-approval framework from the CSRC + SSE/SZSE +
shareholder-vote framework that governs onshore CB issuance and
alteration. The onshore regime requires CSRC pre-issuance approval
(see [cb-china-csrc-disclosure-timing](./cb-china-csrc-disclosure-timing.md))
plus the shareholder-vote-supermajority requirement for prospectus
clause alterations (see [cb-china-downward-conversion](./cb-china-downward-conversion.md));
the HKEX regime relies on Exchange-approval at issuance and for
alterations, with no equivalent shareholder-vote-supermajority
requirement at the rule-layer level. Under HKEX Ch.28, the
Exchange-approval mechanism serves as the alteration-control
device for any convertible-debt securities listed under that
chapter. **Source:** HKEX Ch.28 §28.01 + §28.05 pp.1-2; HKEX Ch.16
§16.01 + §16.03 pp.1.

The **international convertible-securities classification context**
(Supporting only — NOT an HKEX-specific claim): DeSpiegeleer et al.
(2014) §1 pp.21-30 treats convertible securities as a single
asset class with two structural subclassifications by issuance
form — debt-form convertibles and equity-form convertibles.
Zubulake §1-§3 pp.5-50 surveys cross-jurisdictional issuance
patterns at a worldwide level. The HKEX classification gate
between Ch.28 (debt-form) and Ch.16 (equity-form) mirrors this
debt-vs-equity structural split. **Source:** DeSpiegeleer et al.
(2014) §1 pp.21-30; Zubulake §1-§3 pp.5-50.

Asymptotic / regime behaviour of the HKEX listing framework
follows four patterns. Standard-listing regime: the convertible-
securities issue goes through the Ch.16 / Ch.28 approval process;
the underlying shares are either listed on HKEX Main Board OR
on a recognised foreign market; alterations require Exchange
approval. Professional-investor-only regime (Ch.28 §28.01
carve-out): the convertible-debt issue is not subject to Ch.28;
alternate professional-investor listing rules apply instead
(out of scope for this card). Exchange-discretion regime: the
underlying shares are not listed on HKEX or a recognised foreign
market, but the Exchange exercises its discretion to list the
convertibles on the basis that holders have sufficient information
to value the underlying. State / Supranational regime (Ch.28
§28.03 + §28.04 carve-outs): sovereign or supranational issuers
are exempted from the underlying-listed-or-recognised constraint.
**Source:** HKEX Ch.28 §28.01-§28.06 pp.1-2; HKEX Ch.16
§16.01-§16.04 pp.1-1.

## See Also

- [`cb-china-trading-mechanics.md`](cb-china-trading-mechanics.md) — base China-onshore CB trading mechanics that contrasts against the HKEX offshore listing regime treated in this card
- [`cb-china-csrc-disclosure-timing.md`](cb-china-csrc-disclosure-timing.md) — China-onshore CSRC disclosure / listing-approval regime that contrasts against the HKEX Exchange-approval gate
- [`cb-china-exchange-price-limit-suitability-rules.md`](cb-china-exchange-price-limit-suitability-rules.md) — China-onshore SSE/SZSE exchange-rulebook layer that contrasts against the HKEX listing regime

## Escalate to Raw When

Open HKEX Main Board Listing Rules Ch.28 Convertible Debt
Securities pp.1-2 directly for the HKEX-side convertible-debt
listing-approval and alteration-of-terms framework: §28.01
Exchange-approval gate + professional-investor carve-out; §28.02
debt-vs-equity conflict rule; §28.03 underlying-equity-securities-
listed-or-recognised constraint with State / Supranational carve-
out; §28.04 non-equity-property listing discretion; §28.05
alteration-of-terms approval. Open HKEX Ch.16 Convertible Equity
Securities pp.1-1 for the parallel equity-securities-regime
treatment: §16.01-§16.04 Exchange-approval gate + underlying-
shares constraint + alteration-of-terms approval + listing-
document content cross-references (Appendix D1A paragraph 19 +
Appendix D1B paragraph 21). Open DeSpiegeleer et al. (2014) §1
pp.21-30 for the international convertible-securities
classification framework (debt-form vs equity-form). Open
Zubulake §1-§3 pp.5-50 for the broader cross-jurisdictional
convertible-securities issuance survey. Open Clifford Chance
Asia-Pacific Equity-Linked Products 2e (Feb 2025) §1-§3
pp.1-30 for the pan-Asian comparative legal framework that
places HKEX alongside Japanese 新株予約権付社債 and Singapore
convertible securities as the three principal Asia-Pacific
embedded-option equity-linked issuance regimes (the parallel
cross-Asian treatment is in the sibling card
`cb-china-vs-asian-equity-linked-comparative`). **Source:**
HKEX Ch.28 pp.1-2; HKEX Ch.16 pp.1-1; DeSpiegeleer et al.
(2014) §1 pp.21-30; Zubulake §1-§3 pp.5-50; Clifford Chance
APAC 2e (Feb 2025) §1-§3 pp.1-30.
