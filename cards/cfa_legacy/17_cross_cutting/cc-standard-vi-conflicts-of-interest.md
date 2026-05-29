---
schema_version: "cacg.v0"
id: "cc-standard-vi-conflicts-of-interest"
title: "Standard VI(A) Disclosure of Conflicts + VI(B) Priority of Transactions + VI(C) Referral Fees"
reading_id: "17_cross_cutting"
summary: "Standard VI.A REQUIRES full and fair disclosure of all matters that could impair independence/objectivity; VI.B REQUIRES client and employer transactions to take priority over Member's beneficial-owner trades; VI.C REQUIRES disclosure of referral-fee arrangements to clients, prospective clients, and employer."
tags: ["cfa-ethics", "standard-vi"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3712:5589"
    chunk_hash: "60546bfc84b2223a498003e073ebf8008f0b2cf478dd0120ad9f9c71b1ed5685"
    page_range: [3712, 3713]
    quote: "Members and candidates have the responsibility of determining how often, in what manner, and in what particular circumstances the disclosure of conflicts must be made."
    edge_type: "defines"
card_hash: "8d234c4fa20e9595815dfebdad54922fbb361583b32e34dbeb962a84dc679e06"
---
# Standard VI(A) Disclosure of Conflicts + VI(B) Priority of Transactions + VI(C) Referral Fees

## Intuition

Standard VI.A REQUIRES Members and Candidates to make full and fair
disclosure of all matters that could reasonably be expected to
impair their independence and objectivity OR interfere with
respective duties to their clients, prospective clients, and
employer. Disclosure must be prominent, in plain language, and
sufficient to enable clients to evaluate the conflict. **Source:**
CFA Institute (2022) L1 Vol.6/pp.443-450.

Standard VI.B REQUIRES investment transactions for clients and
employers to have priority over investment transactions in which a
Member or Candidate is the beneficial owner. The rule applies to
personal trades that intersect with client or employer activity:
the Member may not front-run client orders, may not trade ahead
of recommendation publication, and may not allocate hot IPOs to
personal accounts before clients. **Source:** CFA Institute (2022)
L1 Vol.6/pp.451-456.

Standard VI.C REQUIRES Members and Candidates to disclose to their
employer, clients, and prospective clients, as appropriate, any
compensation, consideration, or benefit received from or paid to
others for the recommendation of products or services. The
disclosure permits clients to understand whether the
recommendation is influenced by referral compensation. **Source:**
CFA Institute (2022) L1 Vol.6/pp.457-460.

```
<!-- primitive: ethics-conflict-flow source: _diagram_primitives.md -->
            +-----------------------------+
            | Identify potential conflict |
            | (employment, ownership,     |
            |  compensation, referral)    |
            +--------------+--------------+
                           |
                           v
            +-----------------------------+
            | VI.A: Disclose conflict to  |
            | clients and employer in     |
            | plain language?             |
            +--------+----------+---------+
                     |          |
                    yes         no  ----> VIOLATION VI.A
                     |
                     v
            +-----------------------------+
            | VI.B: Will the conflict     |
            | give priority to            |
            | personal/firm transactions? |
            +--------+----------+---------+
                     |          |
                     no         yes ----> RECUSE / restructure
                     |
                     v
            +-----------------------------+
            | VI.C: Is the conflict a     |
            | referral relationship?      |
            +--------+----------+---------+
                     |          |
                     no         yes ----> disclose fee terms
                     |
                     v
                  Conflict managed
```

## Definition

Standard VI.A REQUIRES Members and Candidates to: (1) make full and
fair disclosure of all matters that could reasonably be expected to
impair their independence and objectivity or interfere with their
respective duties to clients, prospective clients, and employer; (2)
ensure that such disclosures are prominent, in plain language, and
communicate the relevant information effectively. The conflict
inventory includes personal ownership of securities recommended to
clients, family relationships with covered issuers, board service
at issuers, employment by issuer-related entities, soft-dollar and
commission arrangements, and any compensation tied to
recommendations or sales. **Source:** CFA Institute (2022) L1
Vol.6/pp.443-450.

Standard VI.B REQUIRES Members and Candidates to give priority to
investment transactions for clients and employers over personal
transactions. The Recommended Procedure (RECOMMENDS) firm policies
that include: (a) restricted trading lists (no personal trades in
securities on the firm's covered list); (b) blackout windows
(personal trades barred for a period before and after firm
recommendations or client trades); (c) pre-clearance for personal
trades; (d) periodic compliance review of personal trading
records. **Source:** CFA Institute (2022) L1 Vol.6/pp.451-456.

Standard VI.C REQUIRES disclosure of any consideration paid or
received for referrals. The disclosure should reach the client (or
prospective client) and the employer; the amount and form of
consideration should be disclosed in enough detail for the client
to evaluate the potential influence on the recommendation.
**Source:** CFA Institute (2022) L1 Vol.6/pp.457-460.

## Mathematical Reasoning

The VI.A full-and-fair-disclosure standard (source REQUIRES) sets
a substantive disclosure floor: disclosure in fine print, buried
in a 30-page document, or behind a click-through is insufficient
even if technically present. The "prominent" requirement REQUIRES
the disclosure to be where a reasonable client would notice it
(e.g., the first page of the account-opening document, a dedicated
section in the periodic statement); "plain language" REQUIRES the
disclosure to be understandable by a non-specialist; "sufficient
detail" REQUIRES enough information for the client to evaluate the
conflict's likely effect on the recommendation. **Source:** CFA
Institute (2022) L1 Vol.6/pp.443-450.

The VI.B priority hierarchy (source REQUIRES) establishes a
three-tier ordering — clients first, employer second, personal
last — that applies whenever the Member's personal trade could
intersect with client or employer activity. The Standard does NOT
prohibit personal trading; it REQUIRES the priority ordering. The
Recommended Procedures' restricted-list and blackout-window
mechanisms operationalize the priority by ensuring that the
Member's personal trades cannot precede or follow client trades
in time. The pre-clearance requirement is the most exam-tested
Recommended Procedure: a Member who trades a covered security
without firm pre-clearance may be in violation of firm policy
even if not in violation of VI.B itself; combined with a personal
trade that intersects with a client trade, the Member is in
violation of both. **Source:** CFA Institute (2022) L1
Vol.6/pp.451-456.

The VI.C referral-fee disclosure obligation (source REQUIRES)
addresses a structural conflict the VI.A general disclosure misses:
referral arrangements are sufficiently common and sufficiently
material that the Standards single them out for explicit treatment.
The disclosure must reach both the client and the employer; a
referral arrangement disclosed to one but not the other still
violates VI.C. The Recommended Procedures suggest a written
referral-arrangement-policy document that lists all current
referral arrangements, the consideration paid or received, and the
client / employer notification protocol. **Source:** CFA Institute
(2022) L1 Vol.6/pp.457-460.

## See Also

- [`cc-standard-iv-b-c-additional-comp-and-responsibilities`](cc-standard-iv-b-c-additional-comp-and-responsibilities.md)
  — IV.B's written-consent requirement for additional compensation
  overlaps with VI.A's disclosure obligation; IV.B requires written
  consent of all parties, VI.A requires disclosure sufficient for
  the parties to evaluate
- [`cc-standard-vii-responsibilities-as-cfa-institute-member-or-candidate`](cc-standard-vii-responsibilities-as-cfa-institute-member-or-candidate.md)
  — VII.A's program-integrity obligation can overlap with VI's
  conflict-disclosure obligations when the conflict involves the
  Member's CFA candidacy or membership

## Escalate to Raw When

Open CFA L1 Vol.6 Reading 58 Standard VI.A (pp.443-450), VI.B
(pp.451-456), and VI.C (pp.457-460) sections directly for the full
Application examples, particularly the personal-trading-window
mechanics under VI.B and the referral-fee disclosure
fact patterns under VI.C that the card omits. **Source:** CFA
Institute (2022) L1 Vol.6/pp.443-460.

- The reader needs the personal-trading-window Application detail
  (how long a blackout period is "reasonable" before and after a
  firm recommendation? what records demonstrate compliance?).
  **Source:** CFA Institute (2022) L1 Vol.6/pp.453-456.
- The reader needs the VI.A disclosure-prominence Application detail
  (what placement satisfies "prominent"? what level of
  client-acknowledgment is required?). **Source:** CFA Institute
  (2022) L1 Vol.6/pp.446-450.
- The reader needs the VI.C referral-fee Application detail
  (cross-border referral arrangements; non-cash consideration like
  reciprocal business; multi-tier referral chains). **Source:** CFA
  Institute (2022) L1 Vol.6/pp.457-460.
