---
schema_version: "cacg.v0"
id: "cc-standard-ii-a-material-nonpublic"
title: "Standard II(A) Material Nonpublic Information"
reading_id: "17_cross_cutting"
summary: "Standard II.A prohibits Members/Candidates who possess material nonpublic information from acting on or causing others to act on it; the gate requires both materiality and non-public status; the mosaic-theory carve-out permits analyst-derived material conclusions from non-material non-public plus public inputs; disseminate-or-refrain governs unsolicited possession."
tags: ["cfa-ethics", "standard-ii"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3611:5425"
    chunk_hash: "b0ce9988e05bb0c6bf2e31d5e4c11588b7b5a9c674e0d8e552d0f6b1102abdef"
    page_range: [3612, 3612]
    quote: "Members and Candidates who possess material nonpublic information that could affect the value of an investment must not act or cause others to act on the information."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3614:5429"
    chunk_hash: "899856bc6e25d75eac22182aa129fff294f0d5ad3704ccc087721ad402cdc708"
    page_range: [3614, 3614]
    quote: "Under the “mosaic theory,” financial analysts are free to act on this collection, or mosaic, of information without risking violation."
    edge_type: "defines"
card_hash: "8ebb7d2bd7508be5da91dc1c244c4b7c003707e8193158aac67f44442b78eeda"
---
# Standard II(A) Material Nonpublic Information

## Intuition

Standard II.A PROHIBITS Members and Candidates who possess material
nonpublic information that could affect the value of an investment
from acting on or causing others to act on the information. The
prohibition has two conjunctive predicates: information must be
both MATERIAL (a reasonable investor would consider it important to
an investment decision) AND NON-PUBLIC (not yet disseminated through
public channels). Either predicate alone is insufficient; the
violation gate fires only when both hold. **Source:** CFA Institute
(2022) L1 Vol.6/pp.343-347.

The mosaic-theory carve-out PERMITS the analyst's distinctive value
contribution: a skilled analyst who combines individually
non-material public and non-public observations into a material
mosaic-level conclusion has NOT violated II.A — the resulting
conclusion is analyst-derived and material, but the inputs were
either non-material or non-public-but-non-material. The carve-out
draws the line at material-AND-non-public inputs; mosaic of
non-material non-public inputs plus public inputs is permitted.
**Source:** CFA Institute (2022) L1 Vol.6/pp.347-351.

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

Standard II.A PROHIBITS Members and Candidates who possess material
nonpublic information that could affect the value of an investment
from acting on the information or causing others to act on it. The
Standard does not require intent to defraud or even direct trading;
"causing others to act" reaches tippee, tipper, and front-running
fact patterns. **Source:** CFA Institute (2022) L1 Vol.6/pp.343-347.

Material information is information whose disclosure would
reasonably be expected to affect the price of a security or
information that reasonable investors would want to know before
making an investment decision. The Guidance identifies three
categories: hard-numeric information (earnings, dividends,
mergers); soft information (management changes, regulatory
investigations, customer or supplier disruptions); analyst
recommendations themselves (a forthcoming analyst rating change
is material to short-term price). **Source:** CFA Institute (2022)
L1 Vol.6/pp.344-347.

Nonpublic information is information that has NOT been disseminated
through public channels (press release, regulatory filing, news
wire) for sufficient time that the market has absorbed it. The
Standards do not impose a strict time threshold; the
reasonable-absorption test depends on the issuer's market
visibility and the dissemination channel's reach. **Source:** CFA
Institute (2022) L1 Vol.6/pp.344-347.

The mosaic-theory PERMITS analysts to combine public and
non-material non-public information into a material conclusion
through skilled analysis. The carve-out is what gives sell-side
research its raison d'être: an analyst who interviews 10 industry
sources, none of whom disclose material non-public information,
and concludes that the issuer's next quarter will miss consensus
estimates by 10%, has produced material analyst-derived information
that the Standards PERMIT the analyst to act on and recommend.
**Source:** CFA Institute (2022) L1 Vol.6/pp.347-351.

## Mathematical Reasoning

The two-predicate gate (source PROHIBITS) is set-theoretic: the
forbidden set is the intersection (material) ∩ (non-public). The
Standard does NOT prohibit acting on material information that is
public (public material information is the substance of legitimate
investment analysis); it does NOT prohibit acting on non-public
information that is non-material (non-material non-public
information is the substance of legitimate competitive industry
research). The conjunctive structure is what makes the mosaic
carve-out possible: an analyst's input set may include non-material
non-public information without triggering the prohibition.
**Source:** CFA Institute (2022) L1 Vol.6/pp.343-351.

The disseminate-or-refrain framework (source REQUIRES) is the
mechanism for resolving the case where a Member comes into
possession of material non-public information unsolicited (e.g., a
client's CEO mentions an undisclosed earnings warning at a private
dinner). The Member's obligation is either to cause the issuer to
disseminate the information through public channels (the preferred
resolution) OR to refrain from trading and from making investment
recommendations based on the information until the information
becomes public. The Standard PROHIBITS the alternative of trading
on the information; passive non-trading combined with proximity to
ongoing client recommendations is itself a violation if the
information silently informs the recommendations. **Source:** CFA
Institute (2022) L1 Vol.6/pp.347-351.

The tipper-tippee chain (source PROHIBITS) extends II.A's reach
to third parties: a Member who tells a friend about a forthcoming
merger has caused the friend (the tippee) to act on the
information; the Member (the tipper) violates II.A by causing the
tippee to act, and the tippee violates II.A by acting. The chain
extends to remote tippees who know or should have known that the
information originated from a non-public source. The Recommended
Procedures (RECOMMENDS) for firms include firewalls between
research/banking divisions, restricted lists for issuers with
pending M&A activity, watch lists for issuers with elevated
volatility, and trading monitoring to flag unusual personal
trading by employees with access to potentially material
information. **Source:** CFA Institute (2022) L1 Vol.6/pp.347-354.

## See Also

- [`cc-standard-i-d-misconduct`](cc-standard-i-d-misconduct.md) —
  insider trading is often double-charged under I.D's
  professional-conduct prong as a fraud-on-the-market case
- [`cc-material-info-and-dissemination-delay`](cc-material-info-and-dissemination-delay.md)
  — the Ghosh fact pattern applying II.A to a specific
  dissemination-delay scenario

## Escalate to Raw When

Open CFA L1 Vol.6 Reading 58 Standard II.A section directly for the
full Application examples, particularly the tippee/tipper case
studies, the regulator/government-information cases, and the
industry-expert-network fact patterns. **Source:** CFA Institute
(2022) L1 Vol.6/pp.343-354.

- The reader needs the full tipper/tippee application case studies
  (the Ghosh case is one application; the card covers it in the
  sibling cc-material-info-and-dissemination-delay file). **Source:**
  CFA Institute (2022) L1 Vol.6/pp.347-354.
- The reader needs the regulator/government-information detail
  (when does a Member's possession of pre-release regulatory
  information violate II.A?); the card states the general rule but
  does not work specific government-leak fact patterns. **Source:**
  CFA Institute (2022) L1 Vol.6/pp.347-354.
- The reader needs the expert-network application fact patterns
  (channel-check calls, industry consultants, paid expert
  networks); these are recent additions to CFA L1 ethics
  material that the card brackets as out-of-scope at this
  revision. **Source:** CFA Institute (2022) L1 Vol.6/pp.347-354.
