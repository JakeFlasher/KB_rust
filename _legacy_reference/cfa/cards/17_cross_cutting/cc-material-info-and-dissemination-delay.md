---
Use when: applying Standard II.A (Material Nonpublic Information) to a dissemination-delay fact pattern; illustrates how a Member's silent possession of material non-public information during a delay between regulator/issuer awareness and public dissemination triggers II.A's "act on or cause others to act on" prohibition; positioned as a case-application card anchored on CFA L1 Vol.6 R57/R58
Primary raw source: CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.6/pp.343-354
Supporting sources:
  - CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.6/pp.343-354
Repo touchpoints:
  - .claude/knowledge/17_cross_cutting/cc-standard-ii-a-material-nonpublic.md
Out of scope: Standards I-VII general text (sibling cards); II.A general framework (sibling card); securities-law specifics on insider trading; other ledger-mapped case cards (pension-trustee advisory, personal-employer obligations — deferred to follow-on plan to honor 16-card cap)
CFA Relevance: core
Source Stance: primary-cfa
Node type: Example
edges:
  - applies_to:.claude/knowledge/17_cross_cutting/cc-standard-ii-a-material-nonpublic.md
deliverable-ready: true
---

# Application Case: Material Information and Dissemination Delay (Ghosh Fact Pattern)

## Intuition

The Ghosh fact pattern illustrates a structural question II.A raises
but does not resolve on the abstract Standard text alone: what is
the Member's obligation during the gap between a regulator's
awareness of an issuer's wrongdoing and the public dissemination
of that information? **Source:** CFA Institute (2022) L1 Vol.6/pp.343-354.

The fact pattern: an analyst becomes aware, through legitimate
professional channels, that a regulator is investigating an issuer
for accounting fraud. The investigation is non-public (the
regulator has not yet announced it); the regulator's findings, if
disclosed, would be material to the issuer's stock price. The
analyst's question is whether the analyst may continue covering
the stock — and whether the analyst's existing buy recommendation
must be withdrawn — during the dissemination-delay window.
**Source:** CFA Institute (2022) L1 Vol.6/pp.343-354.

```
<!-- primitive: ethics-material-info-gate source: _diagram_primitives.md -->
              +-------------------------------+
              | Information about issuer      |
              +---------------+---------------+
                              |
              +---------------+---------------+
              |                               |
              v                               v
     +-----------------+              +-------------------+
     | MATERIAL?       |              | NON-PUBLIC?       |
     | (reasonable     |              | (not disseminated |
     |  investor would |              |  through public   |
     |  consider it    |              |  channels)        |
     |  important)     |              +---------+---------+
     +--------+--------+                        |
              |                                 |
        +-----+-----+                +----------+----------+
        |           |                |                     |
       yes          no              yes                   no
        |           |                |                     |
        |   (no II.A trigger)        |    (no II.A trigger)
        |                            |
        +-------------+--------------+
                      |
                      v
        +-------------------------------+
        | Both predicates true →        |
        | II.A PROHIBITS acting OR      |
        | causing others to act on the  |
        | information.                  |
        |                               |
        | Mosaic-theory CARVE-OUT:      |
        | analyst-derived non-material  |
        | + public mosaic remains       |
        | PERMITTED.                    |
        +-------------------------------+
```

## Definition

The Ghosh fact-pattern application APPLIES Standard II.A to the
dissemination-delay window between stage-b (analyst learns of
investigation) and stage-e (regulator publicly discloses). During
this window, the analyst is in possession of material non-public
information and is therefore subject to II.A's
"act on or cause others to act on" prohibition. **Source:**
CFA Institute (2022) L1 Vol.6/pp.343-354 (governing Standard).

The dissemination-or-refrain framework REQUIRES the analyst to
either encourage the issuer or regulator to disseminate the
information publicly (the preferred resolution) OR to refrain from
trading and from making investment recommendations based on the
information until the information becomes public. The analyst's
existing buy recommendation — predating the analyst's awareness of
the investigation — should be withdrawn through a compliance
process that does NOT itself signal the non-public information to
the market. **Source:** CFA Institute (2022) L1 Vol.6/pp.347-351
(disseminate-or-refrain Standard text).

The reason this card is the controlling-Standard II.A and NOT
adjacent Standards (RULE-17-CASE-ANSWER-DISCIPLINE): I.A
(Knowledge of the Law) is implicated because regulatory
investigations have securities-law dimensions, but I.A is the
meta-obligation to follow the law generally; II.A is the
conduct-specific prohibition on acting on material non-public
information. I.D (Misconduct) might be charged as a residual
fraud prong, but II.A is the primary controlling Standard
because the conduct (acting on or causing others to act on the
information) is what II.A specifically prohibits. III.A
(Loyalty / Prudence / Care) governs the fiduciary duty but
does not address the dissemination-delay window's specific
prohibition. **Source:** CFA Institute (2022) L1 Vol.6/pp.343-360
(II.A vs. adjacent Standards).

## Mathematical Reasoning

The Ghosh fact pattern APPLIES the II.A two-predicate gate
(material AND non-public) to the regulatory-investigation context:
the regulator's investigation IS material (a reasonable investor
would consider the existence of a fraud investigation important to
the stock's valuation); the investigation IS non-public (the
regulator has not yet announced it); both predicates true; II.A
PROHIBITS acting on or causing others to act on the information.
**Source:** CFA Institute (2022) L1 Vol.6/pp.343-351.

The disseminate-or-refrain mechanism (REQUIRES) operates as
follows in the Ghosh fact pattern: the analyst should encourage
the issuer or regulator to disseminate the investigation publicly;
absent such disclosure, the analyst must refrain from trading and
from making investment recommendations on the information until
the information becomes public. The existing buy recommendation
poses a special problem because passive maintenance of an existing
recommendation while in possession of material non-public negative
information is itself an "action" in II.A's sense (clients
continue to act on the unchanged recommendation; the analyst is
causing them to do so by not updating). The Standard therefore
REQUIRES the analyst to withdraw the buy recommendation, but the
withdrawal must be effected through a compliance process that does
NOT itself signal the non-public investigation to the market — for
example, the firm may suspend coverage citing "pending review" or
"resource constraints" rather than the specific reason. **Source:**
CFA Institute (2022) L1 Vol.6/pp.347-351.

The card's stance is `primary-cfa` anchored on Vol.6 R57/R58 which
supplies both the controlling Standard text and the case-application
framework. The dissemination-delay scenarios this card frames
(intra-day or multi-day delays between regulator/issuer awareness
and full public dissemination) apply Standard II.A directly without
requiring case-specific annotations beyond the Vol.6 span. **Source:**
CFA Institute (2022) L1 Vol.6/pp.343-354.

## See Also

- [`cc-standard-ii-a-material-nonpublic`](cc-standard-ii-a-material-nonpublic.md)
  — the controlling Standard governing this fact pattern; the
  abstract two-predicate framework that this card APPLIES to a
  specific scenario

## Escalate to Raw When

Open CFA L1 Vol.6 Reading 58 Standard II.A section directly for
the full Application examples (the Standards-Application materials
include multiple dissemination-delay fact patterns the card
brackets as out-of-scope at this revision). The original Ghosh-style
case annotation context derives from the Vol.6 R58 Standard II.A
section. **Source:** CFA Institute (2022) L1 Vol.6/pp.343-354.

> Note: a prior revision of this card cited a user-volatile notes
> annotation [reference scrubbed per Critical Rule 9 — no
> user-volatile source citations may appear in cacg.v0 cards].

- The reader needs the full Application-case taxonomy (Vol.6
  R58's Standard II.A subsection works multiple
  dissemination-delay fact patterns; the card states the
  Ghosh-style pattern abstractly). **Source:** CFA Institute (2022)
  L1 Vol.6/pp.347-354.
- The reader needs the firm-level escalation detail (how does a
  Member escalate a Ghosh-style fact pattern to compliance? what
  documentation does the Recommended Procedure REQUIRE?). **Source:**
  CFA Institute (2022) L1 Vol.6/pp.347-354.
- The reader needs the buy-recommendation-withdrawal mechanism
  detail (specifically how to withdraw without signaling the
  non-public information; the card states the general principle
  but does not work the practical mechanics). **Source:** CFA
  Institute (2022) L1 Vol.6/pp.347-354.
