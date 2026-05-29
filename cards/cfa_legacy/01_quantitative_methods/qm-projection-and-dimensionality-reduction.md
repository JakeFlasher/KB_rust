---
schema_version: "cacg.v0"
id: "qm-projection-and-dimensionality-reduction"
title: "Principal Components / Dimensionality Reduction"
reading_id: "01_quantitative_methods"
summary: "Principal components analysis (PCA) rotates a standardised feature matrix onto orthogonal axes ranked by explained variance; the analyst keeps enough leading components to cover a chosen cumulative-variance threshold and discards the rest. PCA is not in CFA L1 2022 Quantitative Methods; R7 only covers simple linear regression with its scalar predictor."
tags: ["quantitative-methods", "projection-dimensionality"]
citations:
  - source_id: "qm_eslii_2009_2ed"
    chunk_id: "qm_eslii_2009_2ed:p552:0693"
    chunk_hash: "adbcaf2487d3629533f56a2906d3564683a9295cdfecf94f0252e2924086aa39"
    page_range: [552, 553]
    quote: "Principal components are a sequence of projections of the data, mutually uncorrelated and ordered in variance."
    edge_type: "defines"
  - source_id: "qm_eslii_2009_2ed"
    chunk_id: "qm_eslii_2009_2ed:p555:0696"
    chunk_hash: "1ff00b8b03dbb07611c8d88f8dba2561874283a1c62f67ab953bfe7c508e108f"
    page_range: [555, 556]
    quote: "Xv2 has the highest variance among all linear combinations satisfying v2 orthogonal to v1, and so on."
    edge_type: "supports"
card_hash: "de5f3c9c8b2970eda674df13a2cdd6d5cf38bae513f0a720b132ca6b5c968925"
---
# Principal Components / Dimensionality Reduction

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
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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

The source' PCA procedure has three steps. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Step 1 — Standardise**: re-centre and re-scale each original
  feature so it has mean zero and unit variance. This step matters
  because PCA's variance-maximisation criterion depends on the
  scale of each feature; without standardisation, features measured
  in larger units would dominate the first principal component
  artificially. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Step 2 — First principal component**: find the linear
  combination of the standardised features that has maximum
  variance. This combination defines the first principal-component
  axis, and the projected values along this axis are the first
  principal-component scores. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Step 3 — Subsequent principal components**: find the next linear
  combination that is orthogonal (at right angles) to the first
  principal component and that has the next-largest variance. Repeat
  for further components, each orthogonal to all previous, until
  the cumulative explained variance exceeds the chosen threshold.
  At that point the analyst stops and uses the retained components
  as the reduced-dimensionality feature set. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The standardise-rotate-stop procedure (source DECOMPOSES) is the
notes' three-step decomposition of PCA. The source presents each step
as a named stage in the procedure; the underlying eigendecomposition
algorithm that operationalises the variance-maximisation criterion
is outside the source' span and belongs to a raw statistical-learning
reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The variance-maximisation criterion (source ASSERTS) is the source'
choice of objective for picking each principal component: the first
PC maximises variance over all linear combinations of the features,
and each subsequent PC maximises variance subject to being
orthogonal to all previously selected PCs. The source asserts this
criterion at the descriptive level; the formal Lagrangian /
eigenvalue argument that produces the principal-component axes as
the eigenvectors of the sample covariance matrix is outside the
notes' span and belongs to a raw statistical-learning reference.
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The orthogonality / uncorrelatedness property (source ASSERTS) is
the source' assertion that distinct principal components are at
right angles to each other in the feature space. The source asserts
this without deriving the link between orthogonal eigenvectors and
zero sample correlation; the derivation is outside the source' span
and belongs to a raw statistical-learning reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The cumulative-variance stopping rule (source ASSERTS) is the
notes' analyst-choice mechanism for deciding how many principal
components to keep. The source asserts the rule at the descriptive
level (stop when cumulative explained variance exceeds the chosen
threshold) without specifying a canonical threshold; threshold
choice depends on the application and is the analyst's judgement
call. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The analyst needs the eigendecomposition algorithm to compute the
  principal components — the source on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` describe the procedure at the
  algorithmic-stages level; the eigendecomposition itself
  (covariance-matrix eigenvalues, singular-value decomposition)
  is outside the source' span and belongs to a raw statistical-
  learning reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst wants a probabilistic-factor-model formulation (factor
  analysis) rather than PCA's deterministic rotation — factor
  analysis is a related but distinct framework outside the source'
  span and belongs to a raw statistical-learning reference.
  **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The dimensionality-reduction problem requires nonlinear methods
  (kernel PCA, t-SNE, UMAP, autoencoders) — the source on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` describe linear PCA only;
  nonlinear methods are outside the source' span and belong to a
  raw statistical-learning reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst needs inference / hypothesis tests on the
  principal-component loadings — the source on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` describe the procedure
  without addressing inferential machinery for the rotation
  matrix; loading-inference belongs to a raw statistical-learning
  reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
