---
schema_version: "cacg.v0"
id: "be-belief-perseverance-biases"
title: "Belief-Perseverance Biases"
reading_id: "10_behavioral_finance"
summary: "The belief-perseverance family of cognitive biases — conservatism, confirmation, representativeness, illusion of control, hindsight — in which investors cling to or over-defend prior beliefs and mis-update in the face of new evidence."
tags: ["behavioral-finance", "belief-perseverance", "cognitive-bias", "conservatism", "representativeness"]
citations:
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p139:0143"
    chunk_hash: "3207d0d282f11fc3da8ce0b22e6415aa68e26624b31188fecd9dcd85ef3c9b5b"
    page_range: [139, 139]
    quote: "Conservatism bias is a mental process in which people cling to their prior views or forecasts at the expense of"
    edge_type: "defines"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p206:0219"
    chunk_hash: "32de5f72859ac62474b94861d3af40e02142e92f82db43d3674a8f3e188d6c4c"
    page_range: [207, 207]
    quote: "that emphasizes ideas that confirm our beliefs, while devaluing whatever contradicts our beliefs."
    edge_type: "supports"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p218:0232"
    chunk_hash: "026c0e785667aae3381754b841b56ca16bc9e806aa9208089156a518249d1c6b"
    page_range: [219, 219]
    quote: "hindsight bias is the impulse that insists:"
    edge_type: "supports"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p130:0134"
    chunk_hash: "4af84c7fe2da640b3cc784e294ce9aa95a0c7e41df819c9d408c0bb83aab6d9e"
    page_range: [131, 131]
    quote: "The illusion of control bias describes the tendency of human beings to believe that they can control or at least influence"
    edge_type: "supports"
card_hash: "5cf445cdb0be9db68d81e93a0678fe223812d2aad9c51204069c541f6dacfe6b"
---
# Belief-Perseverance Biases

## Intuition

Belief perseverance is the cognitive habit of protecting a belief once it is formed — under-updating when evidence cuts against it, over-collecting evidence that confirms it, and rewriting memory so the past seems to have agreed with the present view. Pompian's 2006 edition documents the constituent biases in separate chapters but the common thread is the same: the mind treats an existing belief as a possession to defend rather than a hypothesis to revise. Conservatism is the purest case — people cling to their prior views or forecasts at the expense of acknowledging new information, underreacting to data that should move the estimate.
**Source:** Pompian (2006) Ch.11 pp.119.

The family members differ in *how* the belief is defended. Confirmation bias is selective perception that emphasizes ideas confirming the belief while devaluing contradictory evidence (the classic four-card Wason task: people seek confirmation, not falsification). Representativeness mis-classifies a new phenomenon by forcing it into a familiar category on a rough best-fit basis. Illusion of control is an unwarranted belief that one can influence chance outcomes. Hindsight is the "I knew it all along" reconstruction that exaggerates the predictability of past events and the quality of one's own foresight.
**Source:** Pompian (2006) Ch.17 pp.187, Ch.10 pp.111, Ch.18 pp.199.

EDITION NOTE: the *grouping label* "belief-perseverance biases" is external 2012 2nd-edition (and CFA Level 3) packaging. The 2006 PDF's Part Two introduction states it will make "no attempt ... to distinguish elaborately among types of biases, except to note whether a bias is cognitive or emotional," and each of the 20 bias chapters carries only a "Bias Type: Cognitive" header (e.g., conservatism p.119, confirmation p.187, illusion of control p.111) — so the 2006 edition treats conservatism, confirmation, representativeness, illusion of control, and hindsight as standalone cognitive-bias chapters and never names a "belief-perseverance" super-family. This card adopts the 2e umbrella for cross-referencing only; every member definition below is sourced to its own 2006 chapter.
**Source:** Pompian (2006) Pt.Two pp.49-50.

## Definition

**Belief-perseverance biases** (2e umbrella) are the cognitive biases in which an investor holds, defends, or mis-updates a prior belief: conservatism, confirmation, representativeness, illusion of control, and hindsight.
**Source:** Pompian (2006) Pt.Two pp.49-50.

**Conservatism** is the mental process in which people cling to prior views or forecasts at the expense of acknowledging new information, causing them to *underreact* to that information (the inverse direction from representativeness overreaction).
**Source:** Pompian (2006) Ch.11 pp.119.

**Confirmation bias** is selective perception that overvalues and actively seeks evidence supporting existing beliefs while ignoring or devaluing contradictory evidence — a form of selection bias in evidence-gathering.
**Source:** Pompian (2006) Ch.17 pp.187-188.

**Representativeness bias** is the classification reflex: forcing a new, inconsistent phenomenon into a preconstructed category on a best-fit approximation, producing an incorrect understanding when the analogue is in fact drastically different.
**Source:** Pompian (2006) Ch.5 pp.62.

**Illusion of control** is the tendency to believe one can control or at least influence outcomes that are in fact beyond one's control (Langer: "an expectancy of a personal success probability inappropriately higher than the objective probability would warrant").
**Source:** Pompian (2006) Ch.10 pp.111-112.

**Hindsight bias** is the post-hoc belief that an elapsed event was predictable ("I knew it all along"), causing people to overestimate the quality of their own past predictions and underrate the uncertainty that preceded the event.
**Source:** Pompian (2006) Ch.18 pp.199-200.

## Mathematical Reasoning

Conservatism is sharpest in Bayesian terms. In Ward Edwards' two-urn experiment, after observing a draw of 8 reds and 4 blues, the posterior probability that the draw came from the red-majority urn is about 0.97, yet subjects estimate around 0.7: they overweight the base rate `P(urn)=0.5` and underweight the likelihood carried by the new sample `P(data|urn)`. In Bayes-update language `P(urn|data) = P(data|urn) P(urn) / P(data)`, conservatism shrinks the update toward the prior, i.e. the realized posterior moves less than the normative posterior.
**Source:** Pompian (2006) Ch.11 pp.120.

The family pairs with representativeness as opposite update errors: representativeness *overreacts* (people overweight a representative-looking sample and underweight base rates), conservatism *underreacts* (people overweight base rates and underweight new sample evidence). People can exhibit both: if new data appears representative of an underlying model they overweight it; if no representative relationship is evident, conservatism dominates and new data is underemphasized. (The source presents these as competing directional distortions of the Bayesian update without a unified parametric model.)
**Source:** Pompian (2006) Ch.11 pp.119-120.

## See Also

- [be-cognitive-vs-emotional-bias-taxonomy](./be-cognitive-vs-emotional-bias-taxonomy.md#intuition) — parent: these are all cognitive biases, candidates for moderation.
- [be-information-processing-biases](./be-information-processing-biases.md#intuition) — sibling 2e sub-family of cognitive biases.
- [be-representativeness-conjunction-base-rate](./be-representativeness-conjunction-base-rate.md#intuition) — the formal base-rate-neglect treatment of representativeness.
- [be-confirmation-prior-biased-inference](./be-confirmation-prior-biased-inference.md#intuition) — the formal prior-biased-inference model of confirmation.
- [be-overconfidence-bias](./be-overconfidence-bias.md#intuition) — hindsight feeds overconfidence in one's foresight.

## Escalate to Raw When

- You need the full Ward Edwards urn numbers or Hirshleifer's "costly processing of new information" explanation of why conservatism and base-rate underweighting coexist.
**Source:** Pompian (2006) Ch.11 pp.120.
- You need the four-card Wason selection task details or the IBM/OS-2 confirmation case study for client-facing illustration.
**Source:** Pompian (2006) Ch.17 pp.188-189.
- You need the Breinholt-Dalrymple "Red & Black" descending-outcome experiment that operationalizes illusion of control.
**Source:** Pompian (2006) Ch.10 pp.112-113.
