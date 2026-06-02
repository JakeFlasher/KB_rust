---
schema_version: "cacg.v0"
id: "cc-standard-i-c-misrepresentation"
title: "Standard I(C) Misrepresentation"
reading_id: "17_cross_cutting"
summary: "Standard I.C prohibits Members/Candidates from knowingly making any misrepresentation in investment analysis, recommendations, actions, or other professional activities; the prohibition reaches plagiarism, performance-claim misstatement (overstated returns, cherry-picked composites), and material-omission misrepresentation where omitted context makes an otherwise-true statement misleading."
tags: ["cfa-ethics", "standard-i"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3597:5401"
    chunk_hash: "cad1d390b04abd01110506a863095cce525c17aa274406cf015bb005ca5455fd"
    page_range: [3597, 3598]
    quote: "Members and Candidates must not knowingly make any misrepresentations relating to investment analysis, recommendations, actions, or other professional activities."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3597:5401"
    chunk_hash: "cad1d390b04abd01110506a863095cce525c17aa274406cf015bb005ca5455fd"
    page_range: [3597, 3598]
    quote: "Trust is the foundation of the investment profession. Investors must be able to rely on the statements and information provided to them"
    edge_type: "defines"
card_hash: "cf5358023231ee78ba5fc46711748f084fa42acc55925e4339c889cabac6dbe6"
---
# Standard I(C) Misrepresentation

## Intuition

Standard I.C PROHIBITS knowingly making any misrepresentation
relating to investment analysis, recommendations, actions, or other
professional activities. The prohibition reaches three structurally
distinct conduct classes that exam questions repeatedly test:
plagiarism (presenting another's work as one's own), performance-
claim misstatement (overstated returns, omitted losses, cherry-
picked composites), and material-omission misrepresentation
(disclosure gaps that render an otherwise true statement
misleading). **Source:** CFA Institute (2022) L1 Vol.6/pp.328-335.

The "knowingly" predicate is the violation gate: I.C PROHIBITS
intentional or reckless misrepresentation but does NOT capture
honest mistakes that diligent professionals would still make.
However, the Standard's Guidance notes that diligent professionals
have an obligation to verify the accuracy of any representation they
make; a "should have known" finding by the PCP is sufficient to
establish knowing misrepresentation even absent direct evidence of
intent. **Source:** CFA Institute (2022) L1 Vol.6/pp.328-332.

```
<!-- primitive: ethics-applicability-gate source: _diagram_primitives.md -->
                 +-------------------------+
                 | Does the fact pattern   |
                 | involve a Member or     |
                 | Candidate?              |
                 +-----------+-------------+
                             |
                  +----------+----------+
                  |                     |
                 yes                    no
                  |                     |
                  v                     v
        +-------------------+   +-----------------+
        | Is the conduct in |   | Code/Standards  |
        | professional      |   | do not apply    |
        | activities?       |   | (out of scope)  |
        +---------+---------+   +-----------------+
                  |
        +---------+---------+
        |                   |
       yes                  no
        |                   |
        v                   v
  +---------------+    +---------------------------+
  | Map to        |    | Personal-conduct fallback:|
  | controlling   |    | I.D Misconduct may still  |
  | Standard      |    | apply if conduct reflects |
  | (I.A..VII.B)  |    | adversely on integrity    |
  +---------------+    +---------------------------+
```

## Definition

Standard I.C PROHIBITS Members and Candidates from knowingly making
any misrepresentation relating to (a) investment analysis,
(b) recommendations, (c) actions, or (d) other professional
activities. A "misrepresentation" is any untrue statement or any
statement that is otherwise false or misleading. The "knowingly"
predicate covers both direct intent and reckless disregard for
truth; the Standards' Guidance imputes constructive knowledge to
professionals who failed to verify a representation they made.
**Source:** CFA Institute (2022) L1 Vol.6/pp.328-332.

Plagiarism (one of three I.C conduct classes) PROHIBITS copying or
using in substantially the same form materials prepared by another
without acknowledgment. Verbatim and paraphrased material both
trigger I.C; recycling one's own prior work (e.g., reusing a prior
employer's research) without permission and disclosure also
violates I.C. **Source:** CFA Institute (2022) L1 Vol.6/pp.332-335.

Performance-claim misstatement (second conduct class) PROHIBITS
overstated returns, cherry-picked composites, hypothetical or
backtested results presented without explicit disclosure of their
nature, and inflated AUM figures. The Standard's link to III.D
(Performance Presentation) is procedural: III.D REQUIRES fair,
accurate, and complete performance presentation; I.C PROHIBITS
knowingly misrepresenting performance. A single misstatement can
violate both Standards simultaneously. **Source:** CFA Institute
(2022) L1 Vol.6/pp.335-339.

Material-omission misrepresentation (third conduct class) PROHIBITS
otherwise-true statements that omit material context the reasonable
investor would consider important. Examples include omitting
concentration risk, omitting fee structure, and omitting the
hypothetical-vs-actual nature of a return series. **Source:** CFA
Institute (2022) L1 Vol.6/pp.336-339.

## Mathematical Reasoning

The plagiarism prohibition (source PROHIBITS) operationalizes the
trust principle by REQUIRING analysts to credit the originator of
any analysis they present, on the theory that the cited source's
reputational stake in the analysis is part of the analysis's
credibility. The Standards' Guidance carves out exceptions for
factual market data (e.g., closing prices from a public source need
not be attributed) and for recognized industry conventions (e.g.,
the standard option-pricing formula need not credit Black and
Scholes in every footnote). The carve-out applies to
factual / conventional content; original analysis or argument
ALWAYS requires attribution. **Source:** CFA Institute (2022) L1
Vol.6/pp.332-335.

The performance-claim misstatement prohibition (source PROHIBITS)
operationalizes the duty to present fairly: a misrepresentation by
selective period reporting (showing only the last 3 strong years
of a 10-year track record) violates I.C even if the reported figures
are arithmetically correct because the selection is misleading. The
violation predicate is the reasonable-investor materiality
standard: if a reasonable investor would consider the omitted period
material to evaluation, the selection is a misrepresentation.
**Source:** CFA Institute (2022) L1 Vol.6/pp.335-338.

The material-omission prohibition (source PROHIBITS) is the most
subtle of the three: a true statement can violate I.C when context
omitted makes the true statement misleading. The classic example
PROHIBITS a manager stating "our small-cap fund returned 30%
last year" without disclosing that the return came from a single
concentrated position now closed; the statement is true but
omits the concentration risk and the position-specific nature of
the return. The Standard REQUIRES disclosure of any context the
reasonable investor would consider material to the truthful
interpretation. **Source:** CFA Institute (2022) L1 Vol.6/pp.336-339.

## See Also

- [`cc-standard-i-a-b-knowledge-of-law-and-independence`](cc-standard-i-a-b-knowledge-of-law-and-independence.md)
  — I.A and I.B are the prior two Professionalism sub-standards; the
  prerequisite reading
- [`cc-standard-i-d-misconduct`](cc-standard-i-d-misconduct.md) —
  I.D is the fourth Professionalism sub-standard; misconduct can
  overlap with I.C when the misrepresentation is fraudulent

## Escalate to Raw When

Open CFA L1 Vol.6 Reading 58 Standard I.C section directly for the
full Application examples — particularly the electronic-media /
social-media plagiarism cases, third-party-research attribution
cases, and cross-employer recycled-research cases — that the card
omits. **Source:** CFA Institute (2022) L1 Vol.6/pp.328-339.

- The reader needs the full plagiarism-application case studies
  (verbatim vs. paraphrased; with and without attribution; the
  factual-data carve-out). **Source:** CFA Institute (2022) L1
  Vol.6/pp.332-335.
- The reader needs the electronic-media and social-media
  plagiarism Application detail (when does a forwarded post become
  a misattribution? what attribution-line carve-outs apply?); the
  card states the general prohibition but does not work the
  channel-specific cases. **Source:** CFA Institute (2022) L1
  Vol.6/pp.332-335.
- The reader needs the III.D Performance Presentation interaction
  detail (when does an I.C misstatement also violate III.D? when is
  III.D the primary controlling Standard?); the sibling
  cc-standard-iii-c-d-e card covers III.D's text. **Source:** CFA
  Institute (2022) L1 Vol.6/pp.336-339.
