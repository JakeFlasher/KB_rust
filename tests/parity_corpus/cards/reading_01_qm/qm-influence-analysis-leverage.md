---
schema_version: "cacg.v0"
id: "qm-influence-analysis-leverage"
title: "Influence Analysis: Leverage, Studentised Residuals, Cook's Distance"
reading_id: "reading_01_qm"
summary: "framing per-observation influence and leverage diagnostics on a fitted regression — the leverage `h_ii` rule-of-thumb threshold, the studentised residual that flags outliers, and Cook's distance that combines leverage and residual magnitude to identify observations whose deletion would materially move the fitted coefficients"
tags: ["definition", "regression-diagnostics"]
card_edges:
  - target: "qm-multiple-linear-regression-foundations"
    edge_type: "extends"
  - target: "qm-regression-assumption-violations"
    edge_type: "extends"
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p009:0007"
    chunk_hash: "1a085a0ed614503ab4705f52128bf79000c5e957519c0fd68d822234b3ac758e"
    page_range: [9, 10]
    quote: "T-tests indicate no significance highly correlatedt-statistic?, when F-test indicates overall Multicollinearity Type II"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p010:0008"
    chunk_hash: "d5032da0221c72c91dd5dbc858a2d4d875fd2f6c170f91c4f4d83143d35631e1"
    page_range: [10, 11]
    quote: "B - Probabilioy 3i3l vs Probabilioy chreshol Dummy variables can also be used as dependent variables in Fit of logit"
    edge_type: "supports"
card_hash: "e5345359e7e67a7d1870e830268725c45cf82997778849a2af8744a52678088f"
---
framing per-observation influence and leverage diagnostics on a fitted regression — the leverage `h_ii` rule-of-thumb threshold, the studentised residual that flags outliers, and Cook's distance that combines leverage and residual magnitude to identify observations whose deletion would materially move the fitted coefficients

## Original Card (preserved verbatim)

## Intuition

Aggregate-residual diagnostics (the heteroskedasticity / serial-correlation
/ multicollinearity tests in the sibling
[`qm-regression-assumption-violations`](qm-regression-assumption-violations.md)
card) tell the analyst whether the regression's classical-assumption
set is intact in the aggregate. They do NOT identify individual
observations whose presence in the sample materially shifts the
fitted coefficient vector. The influence-analysis toolkit fills that
gap: it scores each observation on how unusual its predictor profile
is (leverage), how unusual its residual is (studentised residual),
and the combined deletion-impact on the fit (Cook's distance).
**Source:** notes/CFA_note_2.ocr.pdf pp.10.

The three statistics are complementary, not redundant. A high-leverage
observation can sit on the fitted line (with a small residual) and
have little influence; conversely, a low-leverage observation with a
large studentised residual is an aggregate outlier but does not
unduly bend the fitted hyperplane. Cook's distance is the combined
measure that catches the dangerous overlap — high leverage AND large
studentised residual — where a single observation can dominate the
estimated coefficients. **Source:** notes/CFA_note_2.ocr.pdf pp.10.

```
<!-- primitive: influence-leverage-plot source: _diagram_primitives.md -->
   studentized residual e*_i
       ^
   +2  + ----- studentized-outlier threshold ------
       |       .          .                  .
       |  .         .            .   .
    0  +-----.-----+------+-------+-------------+--
       |        .              .       .
       |   .         .   .        .
   -2  + ----- studentized-outlier threshold ------
       |                                   .  <-- influential
       |                                       (high lev × |e*|)
       +------------|-------------+------+--------> leverage h_ii
                3(k+1)/n
              (leverage threshold)
```

## Definition

For an OLS fit on `n` observations with `k + 1` columns (intercept
plus `k` slope predictors), the notes' per-observation diagnostics
are listed below. **Source:** notes/CFA_note_2.ocr.pdf pp.10.

- **Leverage** `h_{ii}` is the notes' measure of how far the
  predictor profile of observation `i` is from the average predictor
  profile across the sample. The notes assert the rule-of-thumb
  threshold `h_{ii} > 3 · (k + 1) / n` for flagging observation `i`
  as a potential high-leverage point. **Source:**
  notes/CFA_note_2.ocr.pdf pp.10.

- **Studentised residual** `e*_i` is the notes' rescaling of the
  raw residual `ε̂_i` by its own standard error computed with
  observation `i` excluded. The notes assert the conventional
  `|e*_i| > 2` outlier flag. **Source:**
  notes/CFA_note_2.ocr.pdf pp.10.

- **Cook's distance** `D_i` is the notes' combined-influence
  diagnostic that pairs leverage with studentised residual to
  quantify the deletion-impact of observation `i` on the fitted
  coefficients. The notes assert that a large `D_i` indicates
  observation `i` materially shifts the fit if removed; the specific
  algebraic combination of leverage and studentised residual into
  `D_i` is outside the notes' span and belongs to a raw econometric
  reference. **Source:** notes/CFA_note_2.ocr.pdf pp.10.

## Mathematical Reasoning

The leverage threshold (source ASSERTS) `h_{ii} > 3 · (k + 1) / n`
is the notes' rule-of-thumb flag for a high-leverage observation.
The notes assert the threshold form as a multiple of the average
leverage `(k + 1) / n` without deriving the underlying trace
identity for the hat matrix or the connection to the design-matrix
column-space dimension. **Source:** notes/CFA_note_2.ocr.pdf pp.10.

The studentised-residual rule (source ASSERTS) flags observation
`i` as an outlier when the leave-one-out studentised residual `e*_i`
exceeds the conventional two-sigma threshold `|e*_i| > 2`. The
notes assert the leave-one-out construction and the two-sigma rule;
the formal Student-t distributional argument for the leave-one-out
residual is outside the notes' scope. **Source:**
notes/CFA_note_2.ocr.pdf pp.10.

The Cook's-distance combined-influence rule (source ASSERTS) is
that `D_i` flags observations where BOTH a sizeable studentised
residual AND non-negligible leverage coincide; a high-leverage
observation on the fitted line (small `e*_i`) or a low-leverage
aggregate outlier (`h_{ii}` near `0`) alone produces small `D_i`,
matching the notes' guidance that neither factor alone is dangerous.
The notes assert the combined-influence intuition without deriving
a specific algebraic form for `D_i`; the formula and its influence-
function derivation are outside the notes' span and belong to a raw
econometric reference. **Source:** notes/CFA_note_2.ocr.pdf pp.10.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — establishes the OLS fit whose per-observation leverage,
  residual, and deletion-impact this card scores
- [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md)
  — the aggregate-residual diagnostic counterpart; both cards
  together describe the diagnostic stack the analyst runs after
  fitting OLS to decide whether the inferential machinery is
  reliable

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
notes/CFA_note_2.ocr.pdf pp.10.

- The analyst needs a robust-regression estimator that down-weights
  high-influence observations rather than merely flagging them — the
  notes on `notes/CFA_note_2.ocr.pdf pp.10` cover the diagnostic
  flags only; any estimator that replaces or augments OLS is outside
  the notes' scope and belongs to a raw econometric reference.
  **Source:** notes/CFA_note_2.ocr.pdf pp.10.
- The deletion-impact is wanted on a per-coefficient basis rather
  than on the full coefficient vector — the notes' Cook's distance
  on `notes/CFA_note_2.ocr.pdf pp.10` aggregates across all
  coefficients; per-coefficient deletion-impact diagnostics are
  outside the notes' scope and belong to a raw econometric
  reference. **Source:** notes/CFA_note_2.ocr.pdf pp.10.
- The data are time-series or panel — the notes' influence diagnostics
  on `notes/CFA_note_2.ocr.pdf pp.10` treat observations as exchangeable
  cross-section rows; time-series and panel structure are outside the
  notes' scope and belong to a raw econometric reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.10.
- The fitted model is nonlinear in the parameters — the notes'
  influence formulas on `notes/CFA_note_2.ocr.pdf pp.10` use the
  linear-OLS hat matrix; the nonlinear-regression analogue is
  outside the notes' scope and belongs to a raw econometric
  reference. **Source:** notes/CFA_note_2.ocr.pdf pp.10.
