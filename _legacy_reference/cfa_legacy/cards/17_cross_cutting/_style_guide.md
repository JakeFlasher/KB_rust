# Ethics & Cross-Cutting Card Style Guide

Extends: ../_style_guide_common.md

This guide is the per-topic authoring standard for
`.claude/knowledge/17_cross_cutting/*.md`. Cross-cutting rules (rigor
density, ASCII diagram complexity, citation density, prohibited
patterns) live in the shared guide referenced above. This file carries
17-specific additions for Ethics & Professional Standards cards:

- the STANDARDS-VS-GUIDANCE-VS-RECOMMENDED-PROCEDURE distinction
  enforced on every card body;
- the CASE-ANSWER DISCIPLINE for application-flavored cards (state the
  violated Standard, why the conduct triggers it, why adjacent
  Standards are not the controlling issue);
- the OCR/CHINESE-FRAGMENT PROVENANCE rule (handwritten study notes
  may anchor application case cards but never normative Standards
  doctrine);
- the ETHICS SOURCE-MARKER convention (REQUIRES / RECOMMENDS / PERMITS
  / PROHIBITS / APPLIES / ASSERTS, replacing the math-flavored
  PROVES / ASSERTS / APPLIES set used in 01 per Codex Round-1
  correction);
- the FULL-PATH BODY-CITATION convention for both Vol.6 Reading spans
  and [notes-citation removed per Critical Rule 9] spans;
- the PROHIBITED-PATTERNS list specific to ethics (worked GIPS return
  computations, performance-presentation attribution drills,
  hypothetical exam-style p-value or risk-metric numerics);
- the BOUNDARY-DISCIPLINE specifying which content belongs in 17 vs
  in adjacent existing or future subcorpora (existing-09 fiduciary /
  IPS / performance-ratio cross-links; future-15 attribution depth).

## Topic-specific additions

### STANDARDS-VS-GUIDANCE-VS-RECOMMENDED-PROCEDURE distinction

CFA L1 Vol.6 Reading 58 organizes each Standard into three distinct
subsections that must NOT be blurred in a single card paragraph:

- **Standard text** — the normative obligation itself (e.g., "Members
  and Candidates must use reasonable care and judgment to achieve and
  maintain independence and objectivity"). A Standards violation is
  defined relative to this text. Source-marker = `REQUIRES` or
  `PROHIBITS`.
- **Guidance** — CFA Institute interpretive commentary expanding the
  Standard's scope, illustrating common violations, and naming defenses
  (e.g., the "mosaic theory" carve-out under II.A). Guidance is binding
  in spirit but not Standard text; presenting it as a Standard
  obligation overstates the source. Source-marker = `REQUIRES` only
  when guidance traces back to explicit Standard text; otherwise
  `ASSERTS`.
- **Recommended Procedures** — practitioner-level best practices that
  reduce risk of violation (e.g., trade-allocation policies for III.B,
  firewall procedures for II.A). These are diligence evidence, NOT
  Standards. Presenting a Recommended Procedure as a mandatory rule is
  a CR6 violation (paraphrase beyond source rigor). Source-marker =
  `RECOMMENDS`.

Each card body MUST visually distinguish the three subsections — either
by separate H3 headings (`### Standard text`, `### Guidance`,
`### Recommended Procedures`), a distinguishing ASCII primitive (the
`ethics-standards-hierarchy` decision tree), or pipe-table columns. A
single prose paragraph that conflates the three is rejected at Codex
calibration.

Rule ID: RULE-17-STANDARDS-VS-GUIDANCE-DISTINCTION

### CASE-ANSWER DISCIPLINE

Application-flavored cards (currently just
`cc-material-info-and-dissemination-delay`; future cross-cutting cases
may be added) MUST follow the three-step case-answer pattern:

1. **State the violated Standard** with exact citation (e.g.,
   "Standard II.A — Material Nonpublic Information, Vol.6/pp.343-354").
2. **Explain why the conduct triggers it** (which fact-pattern elements
   establish materiality, non-public status, and the conduct that
   violates II.A's prohibition).
3. **Explain why adjacent Standards are not the controlling issue**
   (e.g., I.A Knowledge of the Law is implicated by the same conduct,
   but II.A is the controlling Standard because II.A is the
   conduct-specific prohibition; I.A is a meta-obligation).

A case card that names only one Standard without distinguishing it
from adjacent candidate Standards is rejected at Codex calibration.

Rule ID: RULE-17-CASE-ANSWER-DISCIPLINE

### OCR/CHINESE-FRAGMENT PROVENANCE

[notes-citation removed per Critical Rule 9] pp.115-117 contains three ethics-mapped
study notes (`cc-personal-employer-obligations`,
`cc-material-info-and-dissemination-delay`,
`cc-pension-trustee-advisory-ethics`) per the chapter ledger. Within
v8's 16-card cap, only ONE of these (the Ghosh material-info case) is
authored in Batch 2; the other two are deferred to a follow-on plan.

OCR fragments from the notes PDF are LOW-AUTHORITY relative to Vol.6:

- Notes may anchor a card's PRIMARY content only when the card is
  explicitly an APPLICATION case card (Source Stance:
  `primary-cfa`) AND the controlling Vol.6 span appears in
  `Supporting sources:` as the binding authority.
- Notes may NOT anchor a Standards-doctrine card (e.g., a card stating
  "II.A prohibits X" with notes as Primary source). The Standard text
  governs; notes only illustrate.
- OCR fragments in Chinese (rare in the ethics span; pp.115-117 are
  mostly English) MUST be translated and provenance-noted in the card
  body before being quoted: each Chinese fragment carries an inline
  English gloss plus a reference to the OCR JSON page from
  `[notes-ocr-json removed per Critical Rule 9]`.

Rule ID: RULE-17-OCR-PROVENANCE

### ETHICS SOURCE-MARKER convention

Each formula block, Standards-claim block, or Recommended-Procedure
block MUST carry an explicit marker declaring what the source does
with the claim:

- `REQUIRES` — Vol.6 Standard text mandates the conduct as a binding
  obligation. Most-frequent marker for Standards I.A, I.B, III, IV, V,
  VI, VII.
- `PROHIBITS` — Vol.6 Standard text forbids the conduct as a
  violation. Used for I.C (misrepresentation), I.D (misconduct), II.A
  (material nonpublic), II.B (market manipulation).
- `RECOMMENDS` — Vol.6 "Recommended Procedures" subsection prescribes
  best practice; not a Standard violation by itself.
- `PERMITS` — Vol.6 carves out an allowed conduct (e.g., II.A
  mosaic-theory carve-out for analyst-derived non-material
  information).
- `APPLIES` — Vol.6 R60 "Application of the Standard" or a notes-
  anchored case card APPLIES a Standard to a fact pattern.
- `ASSERTS` — non-normative background (e.g., R56 trust-foundation
  claims, R57 Code historical evolution). Used sparingly; never for
  Standards-doctrine claims.

The marker convention replaces the math-flavored
PROVES/ASSERTS/APPLIES set used in 01 QM per Codex Round-1 directive.
Math markers imply proof-bearing claims; ethics obligations are not
proven, they are mandated.

Rule ID: RULE-17-ETHICS-SOURCE-MARKERS

### FULL-PATH BODY-CITATION convention

Every `**Source:**` marker in a card body MUST use:

- For Vol.6 readings:
  `**Source:** CFA Institute (2022) L1 Vol.6/pp.<P-Q>`

[Prior revision admitted a notes-PDF alias-citation form here; the
admission was removed per Critical Rule 9 because the notes source is
user-volatile and non-quotable under the cacg.v0 source-matrix.]

Bare `pp.<P-Q>` against the CFA L1 combined PDF is REJECTED by the
linter (AC-2.1; combined-PDF disambiguation).

Rule ID: RULE-17-FULL-PATH-CITATIONS

### PROHIBITED-PATTERNS (17-specific)

The following patterns are forbidden in 17 card bodies; the shared
guide's general prohibitions (CR1, EPUB-cite, image-diagrams) apply
too:

- Worked GIPS return computations (e.g., "compute the time-weighted
  return given monthly cash flows of $X, $Y, $Z"). Violates CR1; the
  R59 GIPS-basics card scopes to claim-compliance basics only.
- Performance-presentation attribution drills (e.g., "decompose the
  excess return into allocation, selection, and interaction effects").
  Out of scope per DEC-4 (future-15 owns).
- Hypothetical Standards p-value or risk-metric numerics (e.g.,
  "Standard III.A requires the manager to keep VaR below 5%"). VaR
  thresholds are policy, not Standard text.
- Application-case cards naming a Standard without distinguishing it
  from adjacent candidates (violates RULE-17-CASE-ANSWER-DISCIPLINE).
- Standards-doctrine cards anchored on `primary-cfa`
  (violates RULE-17-OCR-PROVENANCE).
- The math-flavored `PROVES` / `DECOMPOSES` markers from 01's style
  guide. Ethics obligations are not proven.

Rule ID: RULE-17-PROHIBITED-PATTERNS

### BOUNDARY-DISCIPLINE (existing and future subcorpora)

Boundaries with adjacent verticals:

- **existing-09 (Portfolio Management):** ethics cards may CITE
  `pm-investment-policy-statement.md`, `pm-portfolio-constraints.md`,
  `pm-performance-ratios-definitions.md` as `Repo touchpoints:` for
  Standard III.A loyalty / III.C suitability / III.D performance.
  Ethics cards do NOT re-state PM machinery (e.g., do not re-derive
  the Sharpe ratio in `cc-standard-iii-d-...` — cite the PM card).
- **existing-05 (Equity):** ethics cards may CITE
  `eq-equity-cost-of-capital-estimation.md`,
  `eq-share-count-and-per-share-effects.md` as `Repo touchpoints:` for
  Standard V.A diligence in valuation analysis and IV.B
  share-based-compensation conflicts.
- **future-15 (Performance & Attribution):** GIPS attribution methods,
  composite construction, return-computation methodology, and
  performance-presentation drills are DEFERRED to future-15. The 17
  GIPS card scopes only to claim-compliance basics.
- **future-13 (Wealth & Institutional):** institutional governance,
  fiduciary-duty case law for pension trustees, and advisory-board
  ethics are DEFERRED to future-13. The 17 ethics cards stay at the
  Standards level.

Rule ID: RULE-17-BOUNDARY-DISCIPLINE

### DIAGRAM-INTERPRETATION rules

The 6 ethics primitives in `_diagram_primitives.md` are
prose-supporting decision diagrams, not quantitative figures:

- `ethics-standards-hierarchy` shows Code → Standard → Guidance →
  Recommended Procedures as nested layers; instantiating cards must
  preserve the four-layer structure even when the card body only
  discusses two.
- `ethics-applicability-gate` shows a yes/no decision flow
  for whether a Standard applies to a given fact pattern; nodes are
  predicate questions, leaves are Standard names.
- `ethics-conflict-flow` shows the disclose-or-recuse
  decision flow for VI.A/B/C; the diagram does not quantify conflict
  magnitudes.
- `ethics-material-info-gate` shows the two-predicate gate
  (material? AND non-public?) for II.A; the mosaic-theory carve-out
  appears as an explicit branch.
- `ethics-gips-compliance-flow` shows the firm-wide-claim-compliance
  flow for R59; no return-attribution methodology, no composite
  rebuilding.
- `ethics-action-response-tree` shows the supervisor's
  detect/prevent/report decision flow for IV.C and the
  whistleblower's escalation path for IV.A.

Diagrams that quantify (e.g., a "VaR threshold" tied to a Standard) or
that smuggle in worked numerics are rejected. The diagrams clarify
qualitative structure, not magnitudes.

Rule ID: RULE-17-DIAGRAM-INTERPRETATION

## Overrides

The 17 guide does not override any shared rule from the common guide.
All shared rules (RULE-RIGOR-DENSITY, RULE-CARD-LENGTH, RULE-MATH-RIGOR,
RULE-DIAGRAM-COMPLEXITY, RULE-CITATION-DENSITY,
RULE-PROHIBITED-PATTERNS) apply unchanged. The 17 additions above
SUPPLEMENT the shared rules rather than override them.
