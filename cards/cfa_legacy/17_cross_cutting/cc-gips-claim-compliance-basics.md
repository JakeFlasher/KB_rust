---
schema_version: "cacg.v0"
id: "cc-gips-claim-compliance-basics"
title: "GIPS Claim-Compliance Basics (Reading 59 only)"
reading_id: "17_cross_cutting"
summary: "A \"claim of GIPS compliance\" requires the firm to include all actual, discretionary, fee-paying portfolios in at least one composite; the claim applies firm-wide, partial-firm claims are prohibited, and GIPS compliance is a Recommended Procedure for Standard III.D rather than an independent Standard."
tags: ["cfa-ethics", "gips-claim"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3761:5667"
    chunk_hash: "489150d6b52c55f959af4e59affbf2872fcf2be32bd1a7eaa7739cb43b160c19"
    page_range: [3761, 3762]
    quote: "The GIPS standards are a practitioner-driven set of ethical principles that establish a standardized, industry-wide approach for investment firms to follow"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3762:5668"
    chunk_hash: "40f5ef943a4c97f2612a819c3dd0db3cc818d71c9cd3d10e96ea6b4d3775b2d6"
    page_range: [3762, 3762]
    quote: "Compliance is a firm-wide process that cannot be achieved on a single product or composite."
    edge_type: "defines"
card_hash: "1ce1632e74a749b2c1c720a7376256f84743562fd83e45da6d51e514e2f43470"
---
# GIPS Claim-Compliance Basics (Reading 59 only)

## Intuition

The Global Investment Performance Standards (GIPS) are voluntary
ethical principles for fair, full, and consistent investment
performance presentation. A firm may "claim compliance with GIPS"
only when it satisfies the firm-wide scope requirement: ALL of the
firm's actual, fee-paying, discretionary portfolios must be included
in at least one composite. Partial-firm claims (claiming compliance
for one strategy or one division while excluding others) are
PROHIBITED. **Source:** CFA Institute (2022) L1 Vol.6/pp.491-495.

GIPS is distinct from Standard III.D (Performance Presentation):
III.D REQUIRES fair, accurate, and complete performance
presentation regardless of GIPS; GIPS is a Recommended Procedure
RECOMMENDS for III.D compliance but is NOT itself a Standard. A
firm can satisfy III.D without claiming GIPS compliance, and a
firm claiming GIPS compliance can still violate III.D if the
presentation omits material context. **Source:** CFA Institute
(2022) L1 Vol.6/pp.491-495.

```
<!-- primitive: ethics-gips-compliance-flow source: _diagram_primitives.md -->
        +--------------------------------------+
        | Firm decides to claim GIPS           |
        | compliance                           |
        +-----------------+--------------------+
                          |
                          v
        +--------------------------------------+
        | All discretionary fee-paying         |
        | portfolios included in at least one  |
        | composite?                           |
        +----+-----------------------+---------+
             |                       |
             yes                     no -----> CANNOT claim
             |                                 GIPS compliance
             v
        +--------------------------------------+
        | Composite construction follows       |
        | GIPS standards firm-wide?            |
        +----+-----------------------+---------+
             |                       |
             yes                     no -----> CANNOT claim
             |                                 GIPS compliance
             v
        +--------------------------------------+
        | Claim "<Firm> claims compliance      |
        | with GIPS" — applies to ENTIRE FIRM. |
        | Partial-firm claims are PROHIBITED.  |
        +--------------------------------------+
        | OUT OF SCOPE for 17 (deferred to     |
        | future-15): attribution methodology, |
        | composite construction depth, return |
        | computation, performance presentation|
        +--------------------------------------+
```

## Definition

A "claim of compliance with GIPS" REQUIRES the firm to have
implemented GIPS-compliant policies firm-wide: every actual,
discretionary, fee-paying portfolio managed by the firm must be
included in at least one composite, with the composites
constructed and reported according to GIPS standards. The claim
applies to the ENTIRE FIRM as defined by the firm; the firm
definition must be reasonable (a true investment-management
organization, not an arbitrarily narrow slice). **Source:** CFA
Institute (2022) L1 Vol.6/pp.491-495.

Partial-firm claims are PROHIBITED: a firm cannot claim
"compliance" for one strategy or one office while excluding
others. The all-or-nothing rule prevents the most common GIPS
abuse — claiming compliance for the top-performing strategies
while leaving weaker strategies outside the GIPS framework. The
firm must define itself as the scope of the compliance claim and
include all discretionary fee-paying portfolios within that
definition. **Source:** CFA Institute (2022) L1 Vol.6/pp.493-495.

The III.D / GIPS distinction is important: Standard III.D
REQUIRES fair, accurate, and complete performance presentation
regardless of whether GIPS is claimed; GIPS is one path to III.D
compliance (the Recommended Procedure RECOMMENDS it) but it is not
the only path. A firm without sufficient resources to implement
GIPS firm-wide can still satisfy III.D through a different
fair-accurate-complete presentation discipline; a firm claiming
GIPS compliance must still ensure the presentation satisfies III.D
on top of the GIPS technical requirements. **Source:** CFA
Institute (2022) L1 Vol.6/pp.493-498.

## Mathematical Reasoning

The firm-wide-scope requirement (source REQUIRES) is the structural
anti-cherry-picking mechanism: by REQUIRING the firm to include
ALL discretionary fee-paying portfolios in composites, GIPS removes
the discretion to selectively report only strong-performing
portfolios. The Standard's definition of "firm" must be reasonable
— an arbitrarily narrow firm definition (e.g., "the equity team in
the New York office") would defeat the anti-cherry-picking purpose;
the firm must correspond to a recognizable
investment-management organization. **Source:** CFA Institute (2022)
L1 Vol.6/pp.491-495.

The partial-firm-claim prohibition (source PROHIBITS) operates on
the same logic at the claim level: the firm cannot claim "GIPS
compliance for the equity strategy only" because that would allow
the firm to hide the fixed-income strategy's weaker performance
behind the all-firm GIPS branding. The all-or-nothing claim
discipline ensures that the GIPS signal is informative about
firm-wide quality rather than selective product-level marketing.
**Source:** CFA Institute (2022) L1 Vol.6/pp.493-495.

The boundary with III.D and with future-15 (source ASSERTS) is
explicit: this card scopes strictly to Reading 59 claim-compliance
basics — what a GIPS claim means and the firm-wide-scope rule. The
deeper machinery — composite construction details, return
calculation methods (time-weighted vs. money-weighted), benchmark
selection, hierarchical-composite design, fee disclosure within
composites, and the broader performance-attribution-methodology
framework — is explicitly OUT OF SCOPE and deferred to future-15
(Performance & Attribution) per DEC-4. The card's `Out of scope:`
field names this deferral so future-15 has a clear handoff target.
**Source:** CFA Institute (2022) L1 Vol.6/pp.491-498.

## See Also

- [`cc-standard-iii-c-d-e-suitability-performance-confidentiality`](cc-standard-iii-c-d-e-suitability-performance-confidentiality.md)
  — Standard III.D (Performance Presentation) is the binding
  Standard; GIPS is a Recommended Procedure for III.D compliance
  but not a Standard itself

## Escalate to Raw When

Open CFA L1 Vol.6 Reading 59 directly for the full GIPS-claim
context — the GIPS history, the GIPS Executive Committee structure,
and the detailed firm-definition guidance — that the card omits.
Open the GIPS Standards Handbook for the full composite
construction, return-calculation, and attribution methodology when
future-15 is authored. **Source:** CFA Institute (2022) L1
Vol.6/pp.491-498.

- The reader needs the GIPS history (why were the standards
  created, who can claim compliance, who benefits) that R59 covers
  but the card brackets as background. **Source:** CFA Institute
  (2022) L1 Vol.6/pp.491-495.
- The reader needs the firm-definition Application detail (how
  does an integrated multi-strategy firm define itself? what
  about a holding-company structure with separately branded
  subsidiaries?). **Source:** CFA Institute (2022) L1
  Vol.6/pp.493-495.
- The reader needs the composite construction / return computation
  / attribution depth that this card explicitly defers to
  future-15; the GIPS Standards Handbook is the canonical source
  when future-15 is authored. **Source:** CFA Institute (2022) L1
  Vol.6/pp.491-498.
