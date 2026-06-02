---
schema_version: "cacg.v0"
id: "be-moderate-vs-adapt-bias-allocation"
title: "Moderate vs Adapt to Investor Biases"
reading_id: "10_behavioral_finance"
summary: "Pompian's two-principle decision rule for biases in asset allocation: moderate (correct) cognitive biases and adapt to emotional biases (Principle II), and moderate biases in less-wealthy clients but adapt to them in wealthier ones (Principle I)."
tags: ["behavioral-finance", "moderate-vs-adapt", "asset-allocation", "wealth-level", "bias-treatment"]
citations:
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p059:0060"
    chunk_hash: "6dd332f5d737a510d525aa9460b7ec207398700a75054eafef4b7346c3d729eb"
    page_range: [60, 60]
    quote: "When should advisors attempt to moderate, or counteract, biased client reasoning"
    edge_type: "defines"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p063:0065"
    chunk_hash: "69c68f9e75c801a08a5ad3b4b177a813fdce58fe2fce6a98ae1efa6f33b89510"
    page_range: [64, 64]
    quote: "way of life at risk, moderating the bias is the best response."
    edge_type: "supports"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p063:0065"
    chunk_hash: "69c68f9e75c801a08a5ad3b4b177a813fdce58fe2fce6a98ae1efa6f33b89510"
    page_range: [64, 64]
    quote: "Conversely, because emotional biases originate from impulse or intuition rather than conscious calculations, they are difficult to rectify."
    edge_type: "supports"
card_hash: "66b25cea4fda23e8f3a6ee7c3c5c6709a1b11d0d739eed7b4a4a510794e8a6bb"
---
# Moderate vs Adapt to Investor Biases

## Intuition

Once a client's biases are identified, the advisor faces one decision: moderate or adapt? To *moderate* is to counteract biased reasoning and steer the client back toward a predetermined (rational, mean-variance) allocation. To *adapt* is to adjust the recommendation so the biased client feels more comfortable with the portfolio. Pompian resolves "when to moderate, when to adapt?" with two principles that condition the answer on the bias *type* and the client's *wealth level*.
**Source:** Pompian (2006) Ch.3 pp.40, pp.43-44.

**Principle II (bias type):** moderate cognitive biases, adapt to emotional biases. Cognitive biases stem from faulty reasoning, so better information and advice can often correct them — moderate. Emotional biases originate from impulse or intuition rather than conscious calculation, so they are difficult to rectify — adapt. **Principle I (wealth level):** moderate biases in less-wealthy clients, adapt in wealthier ones. A client outliving his assets is a far graver failure than a client failing to amass the greatest possible fortune; if a biased allocation could put a less-wealthy client's way of life at risk, moderate. The most financially secure clients would, even after suboptimal bias-driven returns, still reside in the top percentile — so the cost of adapting is small, and adapting is appropriate.
**Source:** Pompian (2006) Ch.3 pp.44.

## Definition

**Moderate** is to attempt to counteract or correct biased client reasoning so the client conforms to a predetermined (rational) asset allocation; appropriate for cognitive biases and for less-wealthy clients.
**Source:** Pompian (2006) Ch.3 pp.40, pp.44.

**Adapt** is to adjust the asset-allocation recommendation to accommodate the client's biases so the client feels more comfortable with the portfolio; appropriate for emotional biases and for wealthier clients.
**Source:** Pompian (2006) Ch.3 pp.40, pp.44.

**Principle I** (wealth conditioning) is: moderate biases in less-wealthy clients; adapt to biases in wealthier clients, because outliving one's assets is a graver failure than under-accumulating.
**Source:** Pompian (2006) Ch.3 pp.44.

**Principle II** (type conditioning) is: moderate cognitive biases (correctable by information); adapt to emotional biases (resistant to correction).
**Source:** Pompian (2006) Ch.3 pp.44.

## Mathematical Reasoning

The two principles form a 2x2 decision grid over the axes (bias type: cognitive vs emotional) and (wealth: low vs high). The four cells are: low-wealth + cognitive -> **moderate**; high-wealth + emotional -> **adapt**; and the two mixed cells (low-wealth + emotional, high-wealth + cognitive) -> **moderate & adapt**, because Principles I and II point in opposite directions and the recommendation blends. A single client carrying several biases is processed bias-by-bias, so two clients with the *same* biases can be advised differently if their wealth differs.
**Source:** Pompian (2006) Ch.3 pp.44-45.

```
                 HIGH WEALTH (adapt)
                        |
    +-------------------+-------------------+
    |  Moderate & Adapt |      Adapt        |
COGNITIVE --------------+----------------- EMOTIONAL
(moderate)|             |                  | (adapt)
    |     Moderate      |  Moderate & Adapt |
    +-------------------+-------------------+
                        |
                 LOW WEALTH (moderate)
       (Pompian Figure 3.1: Principles I and II)
```

The grid is a qualitative disposition rule, not a numeric optimizer; it tells the advisor *whether* to deviate from the rational allocation and in which direction, but not by how much. The magnitude of any adaptation is bounded separately by the quantitative deviation method (see be-bias-aware-asset-allocation-workflow), which caps the behaviorally adjusted allocation at roughly 20% from the mean-variance output. (The source presents the grid graphically and verbally; the cell logic is the composition of the two stated principles.)
**Source:** Pompian (2006) Ch.3 pp.45.

## See Also

- [be-cognitive-vs-emotional-bias-taxonomy](./be-cognitive-vs-emotional-bias-taxonomy.md#intuition) — parent: the cognitive/emotional axis that Principle II reads.
- [be-bias-aware-asset-allocation-workflow](./be-bias-aware-asset-allocation-workflow.md#intuition) — the end-to-end workflow that applies this rule and bounds the deviation.
- [be-belief-perseverance-biases](./be-belief-perseverance-biases.md#intuition) — cognitive biases, default to moderate.
- [be-regret-aversion-status-quo-endowment](./be-regret-aversion-status-quo-endowment.md#intuition) — emotional biases, default to adapt.

## Escalate to Raw When

- You need Pompian's exact Principle I wording on the 99.9th-percentile secure client and the "outliving assets is graver" justification.
**Source:** Pompian (2006) Ch.3 pp.44.
- You need Figure 3.1's four-quadrant diagram or the Adirondack/Boulder/Catskill case-study clients that illustrate same-bias-different-advice.
**Source:** Pompian (2006) Ch.3 pp.44-45.
