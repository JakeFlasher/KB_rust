---
schema_version: "cacg.v0"
id: "qm-projection-and-dimensionality-reduction"
title: "Principal Components / Dimensionality Reduction"
reading_id: "reading_01_qm"
summary: "framing principal components analysis as a dimensionality-reduction technique — rotating the original feature matrix onto orthogonal principal-component axes, ordering them by explained variance, and stopping at the cumulative-variance threshold the analyst chooses"
tags: ["definition", "dimensionality-reduction"]
card_edges:
  - target: "qm-structured-data-ml"
    edge_type: "extends"
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p016:0015"
    chunk_hash: "e56bcc63ab3914c54f5ef14bd84e318cdbe2637830d04f75190a6f853a42bd18"
    page_range: [16, 17]
    quote: "Root node: IOG > 10% Decision node: No Yes Terminal node: Free cash FCFG > 10% FCFG > 20% flow No Yes No Yes invest"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p017:0016"
    chunk_hash: "485e6c172c0a395fa5fbd6bfdd549ba202593eaefd78ae0e64e506c8542d70bb"
    page_range: [17, 18]
    quote: "For any new observation, we let all the classifier trees undertake classification by majority vote"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p018:0017"
    chunk_hash: "b35680f8ee4354892a1361bae4946561cd72e3842085086b02db5cb312490d18"
    page_range: [18, 19]
    quote: "Dendrogram f Complex non-linear data? Problem No Yes Regression Penalized Classification and regression/LASSO"
    edge_type: "supports"
card_hash: "0cdaf1897ed54db47d389e2d874129ec183c87ccaa503d356186cce98e0f582d"
---
framing principal components analysis as a dimensionality-reduction technique — rotating the original feature matrix onto orthogonal principal-component axes, ordering them by explained variance, and stopping at the cumulative-variance threshold the analyst chooses

## Original Card (preserved verbatim)

## Intuition

When the predictor count `k` is large and many predictors are
highly correlated with each other, the analyst has a redundancy
problem: most of the meaningful variation in the predictor space
lives in a much smaller number of effective directions than the
nominal `k` suggests. PCA rotates the original feature matrix onto
a new orthogonal coordinate system whose axes — the principal
components — are aligned with the directions of maximum variance in
the data. The first principal component captures the largest share
of total variance; the second captures the next-largest share and
is orthogonal to the first; and so on. **Source:**
notes/CFA_note_2.ocr.pdf pp.17-18.

Because the principal components are uncorrelated with each other
by construction, the analyst can keep only the first few — the ones
that cumulatively explain "enough" of the total variance for the
task at hand — and discard the rest. This reduces the dimensionality
of the feature space from `k` down to some smaller `m < k` without
losing much of the explanatory signal in the original predictors.
The chosen cutoff is the cumulative-explained-variance threshold:
keep enough components to cover the analyst-chosen cumulative-
variance share `τ` and stop once the running sum exceeds `τ`. The
specific value of `τ` depends on the application and is the
analyst's judgement call. **Source:**
notes/CFA_note_2.ocr.pdf pp.17-18.

```
<!-- primitive: regression-scatter-and-fit source: _diagram_primitives.md -->
   y
    ^                                       .
    |                                   .  ŷ = b̂_0 + b̂_1·x
    |                              .   /
    |                         .       /  .
    |                    .           / .
    |               .       .       /
    |          .                .  /
    |     .            .          /  .
    |          .          .      /
    |  .             .          /     .
    |       .                  /  .
    | b̂_0  ___________________/
    |                        /
    +-----------------------+----------------------> x
```

## Definition

The notes' PCA procedure has three steps. **Source:**
notes/CFA_note_2.ocr.pdf pp.17-18.

- **Step 1 — Standardise**: re-centre and re-scale each original
  feature so it has mean zero and unit variance. This step matters
  because PCA's variance-maximisation criterion depends on the
  scale of each feature; without standardisation, features measured
  in larger units would dominate the first principal component
  artificially. **Source:** notes/CFA_note_2.ocr.pdf pp.17-18.

- **Step 2 — First principal component**: find the linear
  combination of the standardised features that has maximum
  variance. This combination defines the first principal-component
  axis, and the projected values along this axis are the first
  principal-component scores. **Source:**
  notes/CFA_note_2.ocr.pdf pp.17-18.

- **Step 3 — Subsequent principal components**: find the next linear
  combination that is orthogonal (at right angles) to the first
  principal component and that has the next-largest variance. Repeat
  for further components, each orthogonal to all previous, until
  the cumulative explained variance exceeds the chosen threshold.
  At that point the analyst stops and uses the retained components
  as the reduced-dimensionality feature set. **Source:**
  notes/CFA_note_2.ocr.pdf pp.17-18.

## Mathematical Reasoning

The standardise-rotate-stop procedure (source DECOMPOSES) is the
notes' three-step decomposition of PCA. The notes present each step
as a named stage in the procedure; the underlying eigendecomposition
algorithm that operationalises the variance-maximisation criterion
is outside the notes' span and belongs to a raw statistical-learning
reference. **Source:** notes/CFA_note_2.ocr.pdf pp.17-18.

The variance-maximisation criterion (source ASSERTS) is the notes'
choice of objective for picking each principal component: the first
PC maximises variance over all linear combinations of the features,
and each subsequent PC maximises variance subject to being
orthogonal to all previously selected PCs. The notes assert this
criterion at the descriptive level; the formal Lagrangian /
eigenvalue argument that produces the principal-component axes as
the eigenvectors of the sample covariance matrix is outside the
notes' span and belongs to a raw statistical-learning reference.
**Source:** notes/CFA_note_2.ocr.pdf pp.17-18.

The orthogonality / uncorrelatedness property (source ASSERTS) is
the notes' assertion that distinct principal components are at
right angles to each other in the feature space. The notes assert
this without deriving the link between orthogonal eigenvectors and
zero sample correlation; the derivation is outside the notes' span
and belongs to a raw statistical-learning reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.17-18.

The cumulative-variance stopping rule (source ASSERTS) is the
notes' analyst-choice mechanism for deciding how many principal
components to keep. The notes assert the rule at the descriptive
level (stop when cumulative explained variance exceeds the chosen
threshold) without specifying a canonical threshold; threshold
choice depends on the application and is the analyst's judgement
call. **Source:** notes/CFA_note_2.ocr.pdf pp.17-18.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — provides the OLS estimator whose `(X^T X)^{-1}` matrix becomes
  ill-conditioned under near-collinearity in the predictor matrix
  `X`; PCA is a remedy that replaces the original `X` with a
  reduced-dimension orthogonal-axis feature matrix that has no
  near-collinearity by construction

## Escalate to Raw When

Open the underlying source or a more rigorous statistical-learning
reference when any of the criteria below applies. **Source:**
notes/CFA_note_2.ocr.pdf pp.17-18.

- The analyst needs the eigendecomposition algorithm to compute the
  principal components — the notes on
  `notes/CFA_note_2.ocr.pdf pp.17-18` describe the procedure at the
  algorithmic-stages level; the eigendecomposition itself
  (covariance-matrix eigenvalues, singular-value decomposition)
  is outside the notes' span and belongs to a raw statistical-
  learning reference. **Source:** notes/CFA_note_2.ocr.pdf
  pp.17-18.
- The analyst wants a probabilistic-factor-model formulation (factor
  analysis) rather than PCA's deterministic rotation — factor
  analysis is a related but distinct framework outside the notes'
  span and belongs to a raw statistical-learning reference.
  **Source:** notes/CFA_note_2.ocr.pdf pp.17-18.
- The dimensionality-reduction problem requires nonlinear methods
  (kernel PCA, t-SNE, UMAP, autoencoders) — the notes on
  `notes/CFA_note_2.ocr.pdf pp.17-18` describe linear PCA only;
  nonlinear methods are outside the notes' span and belong to a
  raw statistical-learning reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.17-18.
- The analyst needs inference / hypothesis tests on the
  principal-component loadings — the notes on
  `notes/CFA_note_2.ocr.pdf pp.17-18` describe the procedure
  without addressing inferential machinery for the rotation
  matrix; loading-inference belongs to a raw statistical-learning
  reference. **Source:** notes/CFA_note_2.ocr.pdf pp.17-18.
