---
schema_version: "cacg.v0"
id: "qm-influence-analysis-leverage"
title: "Influence Analysis: Leverage, Studentised Residuals, Cook's Distance"
reading_id: "01_quantitative_methods"
summary: "Per-observation regression diagnostics: leverage h_ii flags unusual predictor profiles, studentised residuals flag aggregate outliers, and Cook's distance combines both to identify observations whose deletion would materially move the fitted coefficients. These quantitative diagnostics are not in CFA L1 2022; R7 only mentions outliers descriptively."
tags: ["quantitative-methods", "influence-analysis"]
citations:
  - source_id: "qm_greene_2019_econometric_analysis_8ed"
    chunk_id: "qm_greene_2019_econometric_analysis_8ed:p146:0209"
    chunk_hash: "1660dc3d50cf276387b2252ed6de4fddb834ccf4ebb7eaf0dfc2e0f40281449a"
    page_range: [146, 147]
    quote: "The influence measure, hii = xi = (X(i) = X(i))-1 xi = 1 n + a K - 1 j = 1 a K - 1 k = 1 (xi,j - xn, j )(xi,k - xk)(Z(i) = M0 Z(i)) jk, (4-68) has been used to flag influential observations."
    edge_type: "defines"
  - source_id: "qm_greene_2019_econometric_analysis_8ed"
    chunk_id: "qm_greene_2019_econometric_analysis_8ed:p147:0210"
    chunk_hash: "2392bb75515b481f7fe4359af299d0e167f3d3bf4a83c2cde56ad4a86d283a42"
    page_range: [147, 147]
    quote: "Studentized residuals are constructed with this in mind by computing the regression coefficients and the residual variance without observation i for each observation in the sample and then standardizing the modified residuals."
    edge_type: "supports"
card_hash: "090adaa4be1ecd8c297b6e304f3225f8710cb0f77bb9b53394c27c6b8eb1dd0b"
---
# Influence Analysis: Leverage, Studentised Residuals, Cook's Distance

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
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The three statistics are complementary, not redundant. A high-leverage
observation can sit on the fitted line (with a small residual) and
have little influence; conversely, a low-leverage observation with a
large studentised residual is an aggregate outlier but does not
unduly bend the fitted hyperplane. Cook's distance is the combined
measure that catches the dangerous overlap — high leverage AND large
studentised residual — where a single observation can dominate the
estimated coefficients. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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
plus `k` slope predictors), the source' per-observation diagnostics
are listed below. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Leverage** `h_{ii}` is the source' measure of how far the
  predictor profile of observation `i` is from the average predictor
  profile across the sample. The source asserts the rule-of-thumb
  threshold `h_{ii} > 3 · (k + 1) / n` for flagging observation `i`
  as a potential high-leverage point. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Studentised residual** `e*_i` is the source' rescaling of the
  raw residual `ε̂_i` by its own standard error computed with
  observation `i` excluded. The source asserts the conventional
  `|e*_i| > 2` outlier flag. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Cook's distance** `D_i` is the source' combined-influence
  diagnostic that pairs leverage with studentised residual to
  quantify the deletion-impact of observation `i` on the fitted
  coefficients. The source asserts that a large `D_i` indicates
  observation `i` materially shifts the fit if removed; the specific
  algebraic combination of leverage and studentised residual into
  `D_i` is outside the source' span and belongs to a raw econometric
  reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The leverage threshold (source ASSERTS) `h_{ii} > 3 · (k + 1) / n`
is the source' rule-of-thumb flag for a high-leverage observation.
The source asserts the threshold form as a multiple of the average
leverage `(k + 1) / n` without deriving the underlying trace
identity for the hat matrix or the connection to the design-matrix
column-space dimension. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The studentised-residual rule (source ASSERTS) flags observation
`i` as an outlier when the leave-one-out studentised residual `e*_i`
exceeds the conventional two-sigma threshold `|e*_i| > 2`. The
notes assert the leave-one-out construction and the two-sigma rule;
the formal Student-t distributional argument for the leave-one-out
residual is outside the source' scope. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The Cook's-distance combined-influence rule (source ASSERTS) is
that `D_i` flags observations where BOTH a sizeable studentised
residual AND non-negligible leverage coincide; a high-leverage
observation on the fitted line (small `e*_i`) or a low-leverage
aggregate outlier (`h_{ii}` near `0`) alone produces small `D_i`,
matching the source' guidance that neither factor alone is dangerous.
The source asserts the combined-influence intuition without deriving
a specific algebraic form for `D_i`; the formula and its influence-
function derivation are outside the source' span and belong to a raw
econometric reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The analyst needs a robust-regression estimator that down-weights
  high-influence observations rather than merely flagging them — the
  notes on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` cover the diagnostic
  flags only; any estimator that replaces or augments OLS is outside
  the source' scope and belongs to a raw econometric reference.
  **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The deletion-impact is wanted on a per-coefficient basis rather
  than on the full coefficient vector — the source' Cook's distance
  on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` aggregates across all
  coefficients; per-coefficient deletion-impact diagnostics are
  outside the source' scope and belong to a raw econometric
  reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The data are time-series or panel — the source' influence diagnostics
  on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` treat observations as exchangeable
  cross-section rows; time-series and panel structure are outside the
  notes' scope and belong to a raw econometric reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The fitted model is nonlinear in the parameters — the source'
  influence formulas on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` use the
  linear-OLS hat matrix; the nonlinear-regression analogue is
  outside the source' scope and belongs to a raw econometric
  reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
