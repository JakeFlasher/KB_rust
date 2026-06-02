---
schema_version: "cacg.v0"
id: "be-cognitive-vs-emotional-bias-taxonomy"
title: "Cognitive vs Emotional Bias Taxonomy"
reading_id: "10_behavioral_finance"
summary: "Pompian's master partition of investor biases into cognitive errors (faulty reasoning, correctable with information) versus emotional biases (impulse/intuition, hard to rectify); the organizing taxonomy that conditions how an advisor treats each bias."
tags: ["behavioral-finance", "bias-taxonomy", "cognitive-bias", "emotional-bias"]
citations:
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p063:0065"
    chunk_hash: "69c68f9e75c801a08a5ad3b4b177a813fdce58fe2fce6a98ae1efa6f33b89510"
    page_range: [64, 64]
    quote: "Behavioral biases fall into two broad categories, cognitive and emotional, with both varieties yielding irrational judgments."
    edge_type: "defines"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p063:0065"
    chunk_hash: "69c68f9e75c801a08a5ad3b4b177a813fdce58fe2fce6a98ae1efa6f33b89510"
    page_range: [64, 64]
    quote: "Because cognitive biases stem from faulty reasoning, better information and advice can often"
    edge_type: "supports"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p068:0069"
    chunk_hash: "285d84b24dbaadfc2f31c6e26572af9e57e6520219cf80163d962971ddf7a1bc"
    page_range: [69, 69]
    quote: "This sort of bias taxonomy is helpful—an underlying theory about why people operate under bias has not been produced."
    edge_type: "supports"
card_hash: "ca9e329cb9d538dbaef8e43809172e4b570c77dda5574bc54264c42378c65aab"
---
# Cognitive vs Emotional Bias Taxonomy

## Intuition

Behavioral finance catalogs more than fifty systematic investor errors, and an advisor cannot treat them as an undifferentiated list. Pompian organizes them along a single fault line: whether a bias arises from *how a person reasons* or from *how a person feels*. Cognitive biases are defects of the reasoning machinery — bad heuristics, misweighted information, faulty probability intuitions — and because they are reasoning errors, better information and advice can often correct them. Emotional biases originate in impulse or intuition rather than conscious calculation, so they resist rectification: you cannot argue someone out of a feeling.
**Source:** Pompian (2006) Ch.3 pp.44.

The taxonomy is not merely descriptive bookkeeping. It is the lever that drives Pompian's whole practitioner program: the cognitive-versus-emotional classification becomes pertinent in the investor case studies, where it helps determine whether an asset allocation should undergo behavioral modification. A cognitive bias is a candidate for *moderation* (you correct it); an emotional bias is a candidate for *adaptation* (you build the portfolio around it). The book deliberately declines to theorize about *why* people are biased — no universal theory of investor behavior exists — and instead gauges only the presence or absence of each named bias.
**Source:** Pompian (2006) Pt.Two pp.49.

EDITION NOTE: this on-disk source is the 2006 1st edition, whose Part Two lists 20 biases tagged simply "Cognitive" or "Emotional" — it does NOT subdivide cognitive biases into a *belief-perseverance* family and an *information-processing* family. That finer two-sub-family packaging is the later 2012 2nd-edition (and CFA Level 3) framing; cards that impose it (be-belief-perseverance-biases, be-information-processing-biases) flag the sub-family labels as external 2e packaging, not as text present in this 2006 PDF.
**Source:** Pompian (2006) Pt.Two pp.49.

## Definition

**Cognitive bias** is a bias that stems from faulty reasoning; because the defect is in reasoning, better information and advice can often correct it. In Pompian's 2006 list the cognitive biases include the heuristic family (anchoring and adjustment, availability, representativeness) plus ambiguity aversion, self-attribution, and conservatism.
**Source:** Pompian (2006) Ch.3 pp.44.

**Emotional bias** is a bias that originates from impulse or intuition rather than conscious calculations, and is therefore difficult to rectify. Pompian's 2006 emotional biases include endowment, loss aversion, and self-control.
**Source:** Pompian (2006) Ch.3 pp.44.

**Bias taxonomy** is the act of classifying biases (as heuristics, beliefs, judgments, preferences, or along cognitive/emotional lines) into a meaningful framework; Pompian treats the cognitive/emotional split as the helpful one but warns no underlying theory of *why* people are biased has been produced.
**Source:** Pompian (2006) Pt.Two pp.49.

## Mathematical Reasoning

The source is a practitioner taxonomy, not a formal model; it offers a classification map rather than an operator. The partition is a binary type label `t(bias) in {cognitive, emotional}` that feeds a downstream treatment rule. The treatment map can be stated as a disposition function: `treat(bias) = moderate` if `t = cognitive`, `treat(bias) = adapt` if `t = emotional` — refined later by wealth level in be-moderate-vs-adapt-bias-allocation. (The source states the classification and the moderate/adapt linkage in prose without formalizing them.)
**Source:** Pompian (2006) Ch.3 pp.44.

```
                 INVESTOR BIASES
                       |
          +------------+------------+
          |                         |
   COGNITIVE (faulty           EMOTIONAL (impulse/
   reasoning -> correctable    intuition -> hard to
   with information)           rectify -> adapt)
          |                         |
   (2e packaging splits        e.g. endowment,
   cognitive into belief-      loss aversion,
   perseverance + info-        self-control
   processing sub-families)
```

The diagram's lower-level split (belief-perseverance vs information-processing) is drawn with a dashed conceptual status: it is external 2e packaging layered onto the 2006 cognitive node, not a partition the 2006 text itself draws.
**Source:** Pompian (2006) Pt.Two pp.49.

## See Also

- [be-belief-perseverance-biases](./be-belief-perseverance-biases.md#intuition) — 2e sub-family of cognitive biases (conservatism, confirmation, representativeness, illusion of control, hindsight).
- [be-information-processing-biases](./be-information-processing-biases.md#intuition) — 2e sub-family of cognitive biases (anchoring, mental accounting, framing, availability, self-attribution, recency).
- [be-overconfidence-bias](./be-overconfidence-bias.md#intuition) — a flagship cognitive bias treated in its own chapter.
- [be-regret-aversion-status-quo-endowment](./be-regret-aversion-status-quo-endowment.md#intuition) — the emotional "stickiness" cluster.
- [be-self-control-mental-accounting](./be-self-control-mental-accounting.md#intuition) — emotional self-control and its mental-accounting partner.
- [be-moderate-vs-adapt-bias-allocation](./be-moderate-vs-adapt-bias-allocation.md#intuition) — the treatment rule the taxonomy feeds.

## Escalate to Raw When

- You need the exact roster of which of the 20 named biases Pompian tags cognitive versus emotional, chapter by chapter — open each chapter's "Bias Type:" line in Part Two.
**Source:** Pompian (2006) Pt.Two pp.49.
- You need Pompian's caution that the study of behavioral finance is still nascent and no overarching theory of investor behavior should be expected.
**Source:** Pompian (2006) Pt.Two pp.49.
