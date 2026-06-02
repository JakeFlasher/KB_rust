---
schema_version: "cacg.v0"
id: "qm-decision-trees-and-roots"
title: "Classification and Regression Trees (CART)"
reading_id: "01_quantitative_methods"
summary: "CART is a non-parametric supervised learning algorithm that partitions the feature space by recursive binary splits at decision nodes; terminal nodes predict the majority class for classification problems or the terminal-node mean for regression problems. CART is not in CFA L1 2022 Quantitative Methods; the parametric counterpart in R7 is simple linear regression."
tags: ["quantitative-methods", "decision-trees"]
citations:
  - source_id: "qm_eslii_2009_2ed"
    chunk_id: "qm_eslii_2009_2ed:p324:0408"
    chunk_hash: "2bed9a519a346acccdeed17875c45f2886e6273f4ff30cd806df7af03aa201a6"
    page_range: [324, 324]
    quote: "To simplify matters, we restrict attention to recursive binary partitions like that in the top right panel of Figure 9.2. We first split the space into two regions, and model the response by the mean of Y in each region. We choose the variable and split-point to achieve the best fit."
    edge_type: "defines"
  - source_id: "qm_eslii_2009_2ed"
    chunk_id: "qm_eslii_2009_2ed:p327:0413"
    chunk_hash: "528098361f5c483657edf748f2a5f20563750faed1b0880d465bab6c8098830e"
    page_range: [327, 328]
    quote: "If the target is a classification outcome taking values 1, 2, . . . , K, the only changes needed in the tree algorithm pertain to the criteria for splitting nodes and pruning the tree."
    edge_type: "supports"
card_hash: "d9f184c58407666786c30dcc30105b7f125914c33e115e4ca1cf562c5ebe0469"
---
# Classification and Regression Trees (CART)

## Intuition

CART is a non-parametric supervised-learning algorithm that asks a
fundamentally different question from linear regression: instead of
fitting one global function over the entire feature space, it
partitions the feature space into a tree of disjoint regions and
fits a simple constant prediction in each region. The partitioning
proceeds by recursive binary splits: at each level of the tree, the
algorithm chooses a feature and a split-point that best separates
the labelled data, then recurses into each child node and repeats.
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The tree has three node types: the **root node** at the top
(containing all training observations), **decision nodes** in the
middle (where a feature-and-split-point pair routes each observation
to a left or right child), and **terminal nodes** at the leaves
(where the algorithm stops splitting and assigns a prediction). The
recursion stops at a terminal node when further splits no longer
materially reduce the chosen error metric. Classification terminals
predict the majority class among the observations that land there;
regression terminals predict the mean of the labels at that
terminal. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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

CART is the source' supervised-learning algorithm with three named
node types. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Root node**: the top of the tree, containing the full training
  sample. Every observation starts here before any split is applied.
  **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Decision node**: an internal node where a feature and a
  split-point on that feature route each observation to one of two
  child nodes based on whether the feature value falls above or
  below the split-point. The source' criterion for choosing the
  feature-and-split-point at each decision node is to generate the
  widest separation of the labelled data — the split that minimises
  classification error in classification trees, or that minimises
  some equivalent error metric in regression trees. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Terminal node** (leaf): a node where the recursion stops. The
  notes' stopping rule is that at any level, when a further split no
  longer materially reduces the chosen error metric for the tree's
  prediction mode (classification error for classification trees, an
  equivalent regression loss such as SSE / MSE for regression trees),
  the current node becomes terminal. The terminal's **prediction** is
  the majority class in classification problems and the mean of the
  labels in regression problems. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The recursive-binary-splits structure (source DECOMPOSES) is the
notes' decomposition of the tree-building process into three
node-type roles: root → decision-node recursion → terminal-node
prediction. The source asserts the tripartite structure and the
binary-split nature of each non-terminal node; the formal
optimisation argument for choosing the feature-and-split-point at
each decision node is outside the source' span and belongs to a raw
statistical-learning reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The split-selection criterion (source ASSERTS) is the source'
"widest separation of the labelled data, minimising classification
error" rule. The source asserts this guidance at the descriptive level
without specifying the specific impurity metric (e.g., Gini, entropy,
information-gain) that operationalises the rule; concrete impurity
metrics are outside the source' span and belong to a raw
statistical-learning reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The stopping rule (source ASSERTS) is the source' guidance that the
recursion terminates when further splits no longer materially reduce
the chosen error metric (classification error for classification
trees; the analogous regression loss such as SSE / MSE for regression
trees). The source asserts this stopping condition at the descriptive
level without specifying the specific threshold or pruning algorithm;
threshold choice and post-fit pruning (e.g., cost-complexity
pruning) are outside the source' span and belong to a raw
statistical-learning reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The terminal-prediction rule (source ASSERTS) is the source'
dual-mode rule: classification terminals predict the majority class
of the observations at that terminal, and regression terminals
predict the mean of the labels at that terminal. The source asserts
both modes; the more nuanced terminal-prediction variants
(probabilistic terminals, posterior-mean terminals) are outside the
notes' span and belong to a raw statistical-learning reference.
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The analyst wants tree-ensemble methods (random forests, gradient
  boosting) — the source on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` cover
  the single-tree algorithm only; multi-tree ensemble methods are
  outside the source' span and belong to a raw statistical-learning
  reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst needs the specific impurity metric for split selection
  — the source on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` describe the
  split rule at the descriptive level; concrete Gini / entropy /
  information-gain metrics are outside the source' span and belong
  to a raw statistical-learning reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst needs a pruning algorithm — the source on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` describe the recursion-stops-
  when-error-no-longer-falls heuristic; post-fit cost-complexity
  pruning and related algorithms are outside the source' span and
  belong to a raw statistical-learning reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The dependent variable is continuous and the analyst wants
  per-region uncertainty quantification — CART's terminal-mean
  prediction is a point estimate; uncertainty quantification under
  CART belongs to a raw statistical-learning reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
