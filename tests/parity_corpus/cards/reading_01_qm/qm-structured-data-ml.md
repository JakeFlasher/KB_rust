---
schema_version: "cacg.v0"
id: "qm-structured-data-ml"
title: "Supervised ML Pipeline and Bias-Variance"
reading_id: "reading_01_qm"
summary: "framing the supervised-machine-learning workflow for structured (tabular) data — the train / validation / test sample partition, the bias-versus-variance distinction between in-sample and out-of-sample error, and the K-fold cross-validation procedure that uses the data more efficiently than a single train / validation split"
tags: ["definition", "supervised-learning"]
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p013:0012"
    chunk_hash: "611d81c11337c94f68e768154d35f4fd0bd7ce048d91d747cbb4b5521edd04ef"
    page_range: [13, 14]
    quote: "Step 2: test whether the error term from the regression has a unit root using a Dickey?Fuller test"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p014:0013"
    chunk_hash: "71dc136707b5532cfc5670333fa869d70c23e790a8df1a11454f4eb6e17a2b2b"
    page_range: [14, 15]
    quote: "Output Input #3 Evaluation of fitness targetvariable is continnows y (e.g"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p015:0014"
    chunk_hash: "c9db4c36421e7e9d9e642399ee10f44783f7a04a7c7d7fa986421ec30d57847d"
    page_range: [15, 16]
    quote: "Repeat this process k times. The average of the k validation errors is then taken as a reasonable estimate of the"
    edge_type: "supports"
card_hash: "35cb2868a727c2813fdd802c01e02fb4f5a09c619a24b8a152db581c979748ee"
---
framing the supervised-machine-learning workflow for structured (tabular) data — the train / validation / test sample partition, the bias-versus-variance distinction between in-sample and out-of-sample error, and the K-fold cross-validation procedure that uses the data more efficiently than a single train / validation split

## Original Card (preserved verbatim)

## Intuition

Supervised machine learning maps a set of input features `X` to a
target variable `Y` using labelled training data — pairs `(X_i, Y_i)`
where the analyst knows both the inputs and the correct output for
each observation. The fitted model is a function `f̂(X) ≈ Y` whose
quality depends on how well it generalises from the training data
to new, previously-unseen inputs. The supervised-ML pipeline
formalises this by partitioning the labelled data into three
disjoint samples that play distinct roles. **Source:**
notes/CFA_note_2.ocr.pdf pp.14-15.

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
notes/CFA_note_2.ocr.pdf pp.14-15.

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

The notes' supervised-ML pipeline has three named pieces.
**Source:** notes/CFA_note_2.ocr.pdf pp.14-15.

- **Training / validation / test partition**: the labelled data is
  split into three disjoint samples. The **training sample** is used
  to fit the model parameters; the **validation sample** is used to
  tune the model's hyperparameters (model complexity, regularisation
  strength, and similar tuning knobs whose specific identity depends
  on the model class); the **test sample** is held out entirely
  until the final model is chosen and is used only once to estimate
  the chosen model's out-of-sample error. **Source:**
  notes/CFA_note_2.ocr.pdf pp.14-15.

- **Bias-variance distinction**: holding model complexity constant,
  in-sample (training) error reflects bias contribution and
  out-of-sample (held-out) error reflects variance contribution. The
  notes' guidance is that the analyst should diagnose which
  contribution dominates from the relative size of the two error
  measurements and adjust model complexity accordingly. **Source:**
  notes/CFA_note_2.ocr.pdf pp.14-15.

- **K-fold cross-validation**: an alternative to the single
  train / validation split that uses the data more efficiently.
  The labelled sample (excluding the held-out test sample) is
  partitioned into `K` equal-sized folds. The model is trained on
  `K − 1` folds and validated on the remaining fold; the procedure
  is repeated `K` times so each fold serves once as the validation
  fold. The `K` validation errors are averaged to produce the
  cross-validated error estimate. **Source:**
  notes/CFA_note_2.ocr.pdf pp.14-15.

## Mathematical Reasoning

The train / validation / test pipeline (source ASSERTS) is the
notes' three-sample partition discipline for supervised learning.
The notes assert the three roles (fitting / tuning / final-error-
estimation) and the disjointness of the samples. The formal
optimal-split-fraction guidance (e.g., 60% / 20% / 20%) is outside
the notes' span; the partition discipline at the role-and-disjointness
level is what the notes assert. **Source:** notes/CFA_note_2.ocr.pdf
pp.14-15.

The bias-variance distinction (source ASSERTS) is the notes' guidance
that, at fixed model complexity, in-sample error tracks bias and
out-of-sample error tracks variance. The notes assert this
distinction at the diagnostic level; the formal bias-variance
decomposition algebra (squared-bias + variance + irreducible-noise
identity) is outside the notes' span and belongs to a raw
statistical-learning reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.14-15.

The K-fold cross-validation procedure (source ASSERTS) is the
notes' answer to the single-split holdout problem: a single
validation fold gives a noisy estimate of out-of-sample error
because the result depends on which observations happen to land in
the validation sample. K-fold rotates each fold through the
validation role exactly once and averages the `K` validation
errors. The notes assert the procedure at the algorithmic-stages
level; the formal arguments for how K-fold's average error compares
to true out-of-sample loss (consistency, variance reduction at
finite `n`) are outside the notes' span and belong to a raw
statistical-learning reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.14-15.

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
notes/CFA_note_2.ocr.pdf pp.14-15.

- The analyst needs the formal bias-variance decomposition algebra
  — the notes on `notes/CFA_note_2.ocr.pdf pp.14-15` describe the
  distinction at the diagnostic level; the squared-bias + variance
  + irreducible-noise identity is outside the notes' span and
  belongs to a raw statistical-learning reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.14-15.
- The analyst wants to compare ML-pipeline variants beyond the
  single train / validation / test partition or K-fold — bootstrap
  validation, nested cross-validation, leave-one-out cross-validation,
  and time-series-aware splits (rolling-window, expanding-window)
  are outside the notes' span and belong to a raw statistical-
  learning reference. **Source:** notes/CFA_note_2.ocr.pdf
  pp.14-15.
- The analyst needs to handle imbalanced classes or heavy-tailed
  targets — the notes on `notes/CFA_note_2.ocr.pdf pp.14-15` do not
  cover sampling adjustments for imbalanced or heavy-tailed data;
  these adjustments belong to a raw statistical-learning reference.
  **Source:** notes/CFA_note_2.ocr.pdf pp.14-15.
- The analyst is working with unstructured data (images, text,
  audio) — the notes on `notes/CFA_note_2.ocr.pdf pp.14-15` cover
  the structured-data pipeline; representation learning for
  unstructured data belongs to a raw deep-learning reference.
  **Source:** notes/CFA_note_2.ocr.pdf pp.14-15.
