# T37 -- Stress-test cacg verifier against a real CFA_reading card

## Target

Use `/home/jakeshea/CFA_reading/.claude/knowledge/11_risk_management/rm-value-at-risk-notes.md`.

Reasoning: its primary source is a single quotable PDF span,
`notes/CFA_note_2.ocr.pdf pp.81-83`, with the canonical file at
`/home/jakeshea/CFA_reading/notes/CFA_note_2.ocr.pdf`. The card also has a
McNeil supporting source, but the smoke fixture should intentionally translate
only four notes-backed claims. This gives a small `cacg.v0` companion while
stress-testing real OCR prose, formulas, and paraphrase boundaries from one
three-page PDF slice.

## Fixture Construction Steps

Do not modify `/home/jakeshea/CFA_reading/`. In cacg, create
`tests/fixtures/cfa_smoke/` and copy, or symlink if fixture conventions allow,
`/home/jakeshea/CFA_reading/notes/CFA_note_2.ocr.pdf` into it as
`CFA_note_2.ocr.pdf`.

Run `kb ingest tests/fixtures/cfa_smoke/CFA_note_2.ocr.pdf --out tests/fixtures/cfa_smoke/out --source-id cfa_note_2_ocr`
to produce `chunks_manifest.json`. Then create a hand-translated
`cacg.v0` card in the same fixture directory. Populate each citation from the
manifest: `source_id: cfa_note_2_ocr`, the exact `chunk_id` and `chunk_hash`
for the chunk covering pages 81-83, `page_range`, and an actual `quote`
copied from the chunk text. The CFA card's inline `**Source:**` strings are
provenance labels only; they are not literal quoted material and should not be
used as `quote` values.

Recommended four citations:

1. `page_range: [81, 81]`; quote the notes sentence beginning "The VaR time
   period should relate to the nature of the situation". This maps cleanly.
2. `page_range: [81, 81]`; quote "The left side of a traditional probability
   distribution displays the low or negative returns..." This maps cleanly if
   the chunk keeps the line intact.
3. `page_range: [82, 82]`; quote "The average loss that would be incurred if
   the VaR cutoff is exceeded". This maps cleanly.
4. `page_range: [82, 82]`; quote "The difference in VaR between the \"before\"
   and \"after\" VaR if a position size is changed..." This maps cleanly.

Claims that require `--fuzzy`: "Value at risk is an estimate of the maximum
expected loss..." because the OCR text around "loss", "level", "probability",
and "time element" is visibly corrupted; "Parametric (variance-covariance)
Method to estimate VaR" because the OCR has variants like `covatiance` and
`vak`.

Claims that should fail under any heuristic: the formal quantile definition
`inf { l in R : P(L <= l) >= alpha }`, the empirical inf-quantile consistency
discussion, and VaR's subadditivity counterexample. Those are card-added or
McNeil-supported synthesis, not literal notes text on pp.81-83.

## Predicted Verifier Outcomes

Citation 1: VERIFIED under normalized exact substring.

Citation 2: VERIFIED if pypdfium2 extraction preserves the sentence as one
normalized span; otherwise requires-fuzzy only for minor line/OCR drift.

Citation 3: VERIFIED under normalized exact substring.

Citation 4: VERIFIED under normalized exact substring.

Negative controls: the corrupted VaR definition and parametric-method label are
`requires-fuzzy` at best; the formal quantile, subadditivity, and estimator
consistency claims are unverifiable against this one-PDF notes fixture.

Likely failure modes: OCR substitutions (`loss`/`lo`, `probability`/
`probaubility`, `variance-covariance`/`variance -covatiance`), hyphenated line
breaks in prose, mathematical notation collapsed into question marks, and page
drift if someone swaps in the combined CFA curriculum PDF instead of
`notes/CFA_note_2.ocr.pdf`. This also re-confirms
`BL-20260518-no-prior-pair-atomic`: the fixture must leave no half-published
`sources_manifest.json` without its paired `chunks_manifest.json` if ingest
publication fails.

## Scope Cap

Attempt four citations. A fifth can be the corrupted VaR-definition fuzzy case
as an explicit negative/optional-fuzzy control. Beyond that, returns diminish:
the card repeats the same pages and mostly tests paraphrase policy rather than
new verifier mechanics.

## Docs Drop-in

To bring an external KB card into cacg, copy only the cited PDF slice into a
fixture, ingest it, then write a `cacg.v0` companion whose citations pin
manifest `chunk_id`/`chunk_hash` values and literal chunk quotes.

Treat legacy inline source labels as provenance hints, not quotes; every
`cacg.v0` quote must be copied from normalized chunk text or it should be
marked fuzzy/unsupported.
