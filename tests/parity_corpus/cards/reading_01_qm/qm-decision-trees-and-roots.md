---
schema_version: "cacg.v0"
id: "qm-decision-trees-and-roots"
title: "Classification and Regression Trees (CART)"
reading_id: "reading_01_qm"
summary: "framing the classification-and-regression-tree (CART) model — a non-parametric supervised-learning algorithm that partitions the feature space by recursive binary splits, with terminal-node predictions chosen by majority class for classification problems or by terminal-node mean for regression problems"
tags: ["decision-trees", "definition"]
card_edges:
  - target: "qm-multiple-linear-regression-foundations"
    edge_type: "extends"
  - target: "qm-structured-data-ml"
    edge_type: "extends"
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p015:0014"
    chunk_hash: "c9db4c36421e7e9d9e642399ee10f44783f7a04a7c7d7fa986421ec30d57847d"
    page_range: [15, 16]
    quote: "Repeat this process k times. The average of the k validation errors is then taken as a reasonable estimate of the"
    edge_type: "supports"
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
card_hash: "2e98cfd5bc9b6bce34a2be7cf9dc0e45a2efab1f687f889c5198fc22a00aa224"
---
framing the classification-and-regression-tree (CART) model — a non-parametric supervised-learning algorithm that partitions the feature space by recursive binary splits, with terminal-node predictions chosen by majority class for classification problems or by terminal-node mean for regression problems

## Original Card (preserved verbatim)

## Intuition

CART is a non-parametric supervised-learning algorithm that asks a
fundamentally different question from linear regression: instead of
fitting one global function over the entire feature space, it
partitions the feature space into a tree of disjoint regions and
fits a simple constant prediction in each region. The partitioning
proceeds by recursive binary splits: at each level of the tree, the
algorithm chooses a feature and a split-point that best separates
the labelled data, then recurses into each child node and repeats.
**Source:** notes/CFA_note_2.ocr.pdf pp.16-17.

The tree has three node types: the **root node** at the top
(containing all training observations), **decision nodes** in the
middle (where a feature-and-split-point pair routes each observation
to a left or right child), and **terminal nodes** at the leaves
(where the algorithm stops splitting and assigns a prediction). The
recursion stops at a terminal node when further splits no longer
materially reduce the chosen error metric. Classification terminals
predict the majority class among the observations that land there;
regression terminals predict the mean of the labels at that
terminal. **Source:** notes/CFA_note_2.ocr.pdf pp.16-17.

```
                       ┌──────────────────┐
                       │    root node     │
                       │  (training data) │
                       └────────┬─────────┘
                                │ split on x_j ≤ s_1 ?
                  ┌─────────────┴─────────────┐
                  ▼                           ▼
         ┌─────────────────┐         ┌─────────────────┐
         │  decision node  │         │  decision node  │
         │  (x_j ≤ s_1)    │         │  (x_j > s_1)    │
         └────────┬────────┘         └────────┬────────┘
                  │ split on x_m ≤ s_2 ?      │ split on x_p ≤ s_3 ?
            ┌─────┴─────┐                ┌────┴─────┐
            ▼           ▼                ▼          ▼
       ┌─────────┐  ┌─────────┐     ┌─────────┐ ┌─────────┐
       │terminal │  │terminal │     │terminal │ │terminal │
       │predict A│  │predict B│     │predict A│ │predict B│
       │(class / │  │(class / │     │(class / │ │(class / │
       │ mean ȳ) │  │ mean ȳ) │     │ mean ȳ) │ │ mean ȳ) │
       └─────────┘  └─────────┘     └─────────┘ └─────────┘
```

## Definition

CART is the notes' supervised-learning algorithm with three named
node types. **Source:** notes/CFA_note_2.ocr.pdf pp.16-17.

- **Root node**: the top of the tree, containing the full training
  sample. Every observation starts here before any split is applied.
  **Source:** notes/CFA_note_2.ocr.pdf pp.16-17.

- **Decision node**: an internal node where a feature and a
  split-point on that feature route each observation to one of two
  child nodes based on whether the feature value falls above or
  below the split-point. The notes' criterion for choosing the
  feature-and-split-point at each decision node is to generate the
  widest separation of the labelled data — the split that minimises
  classification error in classification trees, or that minimises
  some equivalent error metric in regression trees. **Source:**
  notes/CFA_note_2.ocr.pdf pp.16-17.

- **Terminal node** (leaf): a node where the recursion stops. The
  notes' stopping rule is that at any level, when a further split no
  longer materially reduces the chosen error metric for the tree's
  prediction mode (classification error for classification trees, an
  equivalent regression loss such as SSE / MSE for regression trees),
  the current node becomes terminal. The terminal's **prediction** is
  the majority class in classification problems and the mean of the
  labels in regression problems. **Source:**
  notes/CFA_note_2.ocr.pdf pp.16-17.

## Mathematical Reasoning

The recursive-binary-splits structure (source DECOMPOSES) is the
notes' decomposition of the tree-building process into three
node-type roles: root → decision-node recursion → terminal-node
prediction. The notes assert the tripartite structure and the
binary-split nature of each non-terminal node; the formal
optimisation argument for choosing the feature-and-split-point at
each decision node is outside the notes' span and belongs to a raw
statistical-learning reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.16-17.

The split-selection criterion (source ASSERTS) is the notes'
"widest separation of the labelled data, minimising classification
error" rule. The notes assert this guidance at the descriptive level
without specifying the specific impurity metric (e.g., Gini, entropy,
information-gain) that operationalises the rule; concrete impurity
metrics are outside the notes' span and belong to a raw
statistical-learning reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.16-17.

The stopping rule (source ASSERTS) is the notes' guidance that the
recursion terminates when further splits no longer materially reduce
the chosen error metric (classification error for classification
trees; the analogous regression loss such as SSE / MSE for regression
trees). The notes assert this stopping condition at the descriptive
level without specifying the specific threshold or pruning algorithm;
threshold choice and post-fit pruning (e.g., cost-complexity
pruning) are outside the notes' span and belong to a raw
statistical-learning reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.16-17.

The terminal-prediction rule (source ASSERTS) is the notes'
dual-mode rule: classification terminals predict the majority class
of the observations at that terminal, and regression terminals
predict the mean of the labels at that terminal. The notes assert
both modes; the more nuanced terminal-prediction variants
(probabilistic terminals, posterior-mean terminals) are outside the
notes' span and belong to a raw statistical-learning reference.
**Source:** notes/CFA_note_2.ocr.pdf pp.16-17.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — the parametric counterpart to CART: linear regression fits one
  global linear function over the entire feature space, whereas
  CART partitions the feature space and fits a different prediction
  in each region. The two are complementary; CART captures
  interactions and non-monotonicities naturally, but loses the
  closed-form sampling theory that makes OLS inference precise

## Escalate to Raw When

Open the underlying source or a more rigorous statistical-learning
reference when any of the criteria below applies. **Source:**
notes/CFA_note_2.ocr.pdf pp.16-17.

- The analyst wants tree-ensemble methods (random forests, gradient
  boosting) — the notes on `notes/CFA_note_2.ocr.pdf pp.16-17` cover
  the single-tree algorithm only; multi-tree ensemble methods are
  outside the notes' span and belong to a raw statistical-learning
  reference. **Source:** notes/CFA_note_2.ocr.pdf pp.16-17.
- The analyst needs the specific impurity metric for split selection
  — the notes on `notes/CFA_note_2.ocr.pdf pp.16-17` describe the
  split rule at the descriptive level; concrete Gini / entropy /
  information-gain metrics are outside the notes' span and belong
  to a raw statistical-learning reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.16-17.
- The analyst needs a pruning algorithm — the notes on
  `notes/CFA_note_2.ocr.pdf pp.16-17` describe the recursion-stops-
  when-error-no-longer-falls heuristic; post-fit cost-complexity
  pruning and related algorithms are outside the notes' span and
  belong to a raw statistical-learning reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.16-17.
- The dependent variable is continuous and the analyst wants
  per-region uncertainty quantification — CART's terminal-mean
  prediction is a point estimate; uncertainty quantification under
  CART belongs to a raw statistical-learning reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.16-17.
