---
schema_version: "cacg.v0"
id: "be-confirmation-prior-biased-inference"
title: "Confirmation Bias and Prior-Biased Inference"
reading_id: "10_behavioral_finance"
summary: "Prior-biased inference: agents update asymmetrically, weighting signals that confirm current beliefs more than disconfirming ones (c_conf >= 0 >= c_disconf); Rabin-Schrag misperception model; belief polarization and finance implications."
tags: ["behavioral-finance", "confirmation-bias", "belief-updating", "biased-inference"]
citations:
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p162:0262"
    chunk_hash: "4e4e5959760c811bdb29d94d03b8ace68aa45a28c2b287a541a4b348c1ca23bd"
    page_range: [162, 162]
    quote: "drawing inferences in a manner that is biased in favor of current beliefs"
    edge_type: "defines"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p163:0263"
    chunk_hash: "7dacc58e0c79166c3435f7759fe60c722198d1b332190631f1bf6b2456d836c1"
    page_range: [163, 163]
    quote: "Prior-biased inference is the possibility that c may depend on whether a newly observed signal reinforces or weakens current priors"
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p168:0271"
    chunk_hash: "9bb13dda5622b677a72ae51a45febfc82b3e6a501f8e852988b7239584d3b01a"
    page_range: [168, 168]
    quote: "is that the agent sometimes misperceives disconfirming signals as confirming."
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p169:0273"
    chunk_hash: "6aefeadfc9c0ceeff14a6379329fcbefeb5697eb5f38d10167a711f289eb4d2d"
    page_range: [169, 169]
    quote: "momentum arises and bubbles occur because once biased traders are optimistic, they underreact to negative signals"
    edge_type: "supports"
card_hash: "366e48f25b3031a5bb201c042b9b9efa770c27f3db3f7e41b2fcd0801b8ea33d"
---
# Confirmation Bias and Prior-Biased Inference

## Intuition
People interpret new evidence asymmetrically, reading it as more supportive of what they already believe. Benjamin adopts the narrower term *prior-biased inference* for the slice of confirmation bias concerned with updating from signals that have actually been observed (as opposed to selectively seeking or recalling information). The classic demonstration is belief polarization: when proponents and opponents of a position read the *same* mixed evidence, their views move *further apart*, each side crediting the supportive study and discrediting the contrary one.
**Source:** Benjamin (2019) Ch.2 §8, §8.2 pp.162, 164.

The bias is conceptually distinct from base-rate neglect and can coexist with it; the two often push in opposite directions in a given updating problem. Prior-biased inference reinforces current beliefs, while base-rate neglect tends to move beliefs away from certainty. The signature of prior-biased inference is specifically the *asymmetric* response to confirming versus disconfirming signals.
**Source:** Benjamin (2019) Ch.2 §8.1 pp.163-164.

## Definition
**Prior-biased inference** is drawing inferences from observed signals in a manner biased in favor of current beliefs — updating more strongly on signals that confirm than on signals that disconfirm the currently favored hypothesis.
**Source:** Benjamin (2019) Ch.2 §8 pp.162-163.

**Confirmation bias (Rabin-Schrag "confirmatory bias")** is modeled as the agent sometimes *misperceiving* a disconfirming signal as a confirming one, then updating with Bayes' rule on the perceived (not true) signals.
**Source:** Benjamin (2019) Ch.2 §8.2 p.168.

**Belief polarization** is the empirical pattern where individuals with different priors who observe the same mixed evidence end up with more extreme beliefs in opposite directions.
**Source:** Benjamin (2019) Ch.2 §8.2 p.164.

## Mathematical Reasoning
Building on the reduced-form log-odds updating model, prior-biased inference is parameterized as a discrete difference in the amount of updating depending on whether a signal is confirming or disconfirming. Writing the posterior log-odds ratio with confirming exponent `c_0 + c_conf` and disconfirming exponent `c_0 + c_disconf`, the prior-biased-inference hypothesis is `c_conf ≥ 0 ≥ c_disconf`, with at least one inequality strict. The base-rate-neglect parameter `d` enters separately, so the two biases are independently identified. Charness and Dave's sequential bookbag-and-poker-chip experiment estimates the regression coefficients `β̂_1, β̂_2 < 1` (under-inference / base-rate neglect) together with `β̂_3 > 0` and `β̂_4 < 0`, confirming the asymmetric confirming/disconfirming response.
**Source:** Benjamin (2019) Ch.2 §8.1, §8.2 pp.163-164, 167-168.

Rabin and Schrag's formal model: the agent begins with equal priors on states `A` and `B`, observes i.i.d. signals `s_t ∈ {a,b}` that match the state with probability `θ > 1/2`. If a signal matches the currently favored state, *perceived signal* equals the true signal; but if the signal disconfirms, with probability `q > 0` she misperceives it as confirming. She updates by Bayes' rule on perceived signals, unaware of the misperception. Implications: (i) relative to a Bayesian, she ends up *overconfident*; (ii) if the bias is severe or signals uninformative (`θ` near 1/2), there is positive probability she converges to certainty on the *wrong* state. Applied to finance (Pouget et al.), confirmatory-biased traders generate excess volume, excess volatility, and momentum/bubbles — "once biased traders are optimistic, they underreact to negative signals."
**Source:** Benjamin (2019) Ch.2 §8.2-9 pp.168-169.

## See Also
- [be-representativeness-strength-vs-weight](./be-representativeness-strength-vs-weight.md#mathematical-reasoning) — the companion reduced-form `c`/`d` updating framework.
- [be-belief-perseverance-biases](./be-belief-perseverance-biases.md#intuition) — confirmation bias within the CFA belief-perseverance family.
- [be-overconfidence-self-attribution-prices](./be-overconfidence-self-attribution-prices.md#intuition) — biased self-attribution and overconfidence in asset pricing.

## Escalate to Raw When
- You need the full belief-polarization evidence and the Baliga/Benoit-Dubra "ancillary matter" Bayesian-rationalization arguments. **Source:** Benjamin (2019) Ch.2 §8.2 pp.164-165.
- You need the preference-biased inference (asymmetric updating / good-news-bad-news) extension distinct from prior-biased inference. **Source:** Benjamin (2019) Ch.2 §9 pp.169-170.
