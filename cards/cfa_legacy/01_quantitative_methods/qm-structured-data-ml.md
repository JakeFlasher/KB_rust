---
schema_version: "cacg.v0"
id: "qm-structured-data-ml"
title: "Supervised ML Pipeline and Bias-Variance"
reading_id: "01_quantitative_methods"
summary: "CFA L1 2022 only touches supervised-ML at the introductory level in Vol.6 Reading 55 \"Fintech in Investment Management\": train / validation / test split, over- and under-fitting, the supervised / unsupervised / deep-learning typology. K-fold CV, formal bias-variance decomposition, LASSO/CART tuning, and PCA as the legacy card asserts are CFA L2 / raw-statistical-learning content, not L1."
tags: ["quantitative-methods", "structured-data"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3500:5260"
    chunk_hash: "902f6226d880561e69d2011effdaa1844083835662c335fc00970bae4302873a"
    page_range: [3500, 3501]
    quote: "ML involves splitting the dataset into three distinct subsets: a training dataset, a validation dataset, and a test dataset."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3500:5260"
    chunk_hash: "902f6226d880561e69d2011effdaa1844083835662c335fc00970bae4302873a"
    page_range: [3500, 3501]
    quote: "Overfitting occurs when the ML model learns the input and target dataset too precisely."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3501:5261"
    chunk_hash: "f1bcea068f71dae4350a761b52fe7e06dd73694b37e8854f20477d93682209db"
    page_range: [3501, 3501]
    quote: "In supervised learning, computers learn to model relationships based on labeled training data."
    edge_type: "supports"
card_hash: "3a5566c9d58419980d9182dede7956014a754b33f8c425f1830a98aa93898839"
---
# Supervised ML Pipeline and Bias-Variance

## Intuition

Supervised machine learning maps a set of input features `X` to a
target variable `Y` using labelled training data — pairs `(X_i, Y_i)`
where the analyst knows both the inputs and the correct output for
each observation. The fitted model is a function `f̂(X) ≈ Y` whose
quality depends on how well it generalises from the training data
to new, previously-unseen inputs. The supervised-ML pipeline
formalises this by partitioning the labelled data into three
disjoint samples that play distinct roles. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

When the same model is evaluated on different samples, two error
patterns emerge. In-sample (training) error reflects how well the
model fits the data it was trained on; out-of-sample (held-out)
error reflects how well it generalises. The gap between them is
controlled by the model-complexity choice: simple models tend to
under-fit (high in-sample AND out-of-sample error — the bias
contribution dominates), while complex models tend to over-fit
(low in-sample error but high out-of-sample error — the variance
contribution dominates). The pipeline's job is to find a
model-complexity setting that balances the two. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

```
<!-- primitive: model-selection-penalty-curve source: _diagram_primitives.md -->
   penalty term
       ^
       |                                   BIC: ln(n)·(k+1)
       |                                  /  (steeper for large n)
       |                                 /
       |                                /
       |                               /     AIC: 2(k+1)
       |                              /     /
       |                             /     /
       |                            /     /
       |                           /     /
       |                          /     /
       |                         /     /
       |                        /     /
       |                       /     /
       |                      /     /
       |                     /     /
       +--------------------+-----+----------> complexity k
       0                   k*_BIC  k*_AIC
                          (BIC prefers fewer regressors than AIC)
```

## Definition

The source' supervised-ML pipeline has three named pieces.
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Training / validation / test partition**: the labelled data is
  split into three disjoint samples. The **training sample** is used
  to fit the model parameters; the **validation sample** is used to
  tune the model's hyperparameters (model complexity, regularisation
  strength, and similar tuning knobs whose specific identity depends
  on the model class); the **test sample** is held out entirely
  until the final model is chosen and is used only once to estimate
  the chosen model's out-of-sample error. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Bias-variance distinction**: holding model complexity constant,
  in-sample (training) error reflects bias contribution and
  out-of-sample (held-out) error reflects variance contribution. The
  notes' guidance is that the analyst should diagnose which
  contribution dominates from the relative size of the two error
  measurements and adjust model complexity accordingly. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **K-fold cross-validation**: an alternative to the single
  train / validation split that uses the data more efficiently.
  The labelled sample (excluding the held-out test sample) is
  partitioned into `K` equal-sized folds. The model is trained on
  `K − 1` folds and validated on the remaining fold; the procedure
  is repeated `K` times so each fold serves once as the validation
  fold. The `K` validation errors are averaged to produce the
  cross-validated error estimate. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The train / validation / test pipeline (source ASSERTS) is the
notes' three-sample partition discipline for supervised learning.
The source asserts the three roles (fitting / tuning / final-error-
estimation) and the disjointness of the samples. The formal
optimal-split-fraction guidance (e.g., 60% / 20% / 20%) is outside
the source' span; the partition discipline at the role-and-disjointness
level is what the source asserts. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The bias-variance distinction (source ASSERTS) is the source' guidance
that, at fixed model complexity, in-sample error tracks bias and
out-of-sample error tracks variance. The source asserts this
distinction at the diagnostic level; the formal bias-variance
decomposition algebra (squared-bias + variance + irreducible-noise
identity) is outside the source' span and belongs to a raw
statistical-learning reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The K-fold cross-validation procedure (source ASSERTS) is the
notes' answer to the single-split holdout problem: a single
validation fold gives a noisy estimate of out-of-sample error
because the result depends on which observations happen to land in
the validation sample. K-fold rotates each fold through the
validation role exactly once and averages the `K` validation
errors. The source asserts the procedure at the algorithmic-stages
level; the formal arguments for how K-fold's average error compares
to true out-of-sample loss (consistency, variance reduction at
finite `n`) are outside the source' span and belong to a raw
statistical-learning reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — provides the OLS estimator that is the simplest supervised-
  learning model; the same train / validation / test partition and
  the bias-variance distinction apply, though linear regression's
  closed-form sampling theory makes the variance-contribution
  analysis exact in the classical-assumption setting
- [`qm-penalized-regression-lasso`](qm-penalized-regression-lasso.md)
  — LASSO's `λ` hyperparameter is the canonical example of a
  validation-tuned parameter in the supervised-ML pipeline; the
  validation sample chooses `λ`, and the test sample provides the
  final out-of-sample error estimate of the LASSO-selected model
- [`qm-decision-trees-and-roots`](qm-decision-trees-and-roots.md)
  — CART's tree-depth (or equivalently the pruning strength) is
  another canonical validation-tuned hyperparameter that the
  supervised-ML pipeline's bias-variance trade-off informs
- [`qm-projection-and-dimensionality-reduction`](qm-projection-and-dimensionality-reduction.md)
  — PCA's retained-component count is yet another canonical
  validation-tuned hyperparameter; the supervised-ML pipeline
  treats the principal-component count the same way it treats LASSO
  `λ` or CART tree-depth

## Escalate to Raw When

Open the underlying source or a more rigorous statistical-learning
reference when any of the criteria below applies. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The analyst needs the formal bias-variance decomposition algebra
  — the source on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` describe the
  distinction at the diagnostic level; the squared-bias + variance
  + irreducible-noise identity is outside the source' span and
  belongs to a raw statistical-learning reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst wants to compare ML-pipeline variants beyond the
  single train / validation / test partition or K-fold — bootstrap
  validation, nested cross-validation, leave-one-out cross-validation,
  and time-series-aware splits (rolling-window, expanding-window)
  are outside the source' span and belong to a raw statistical-
  learning reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst needs to handle imbalanced classes or heavy-tailed
  targets — the source on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` do not
  cover sampling adjustments for imbalanced or heavy-tailed data;
  these adjustments belong to a raw statistical-learning reference.
  **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst is working with unstructured data (images, text,
  audio) — the source on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` cover
  the structured-data pipeline; representation learning for
  unstructured data belongs to a raw deep-learning reference.
  **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
