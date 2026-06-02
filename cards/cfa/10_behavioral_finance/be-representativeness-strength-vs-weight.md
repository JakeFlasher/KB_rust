---
schema_version: "cacg.v0"
id: "be-representativeness-strength-vs-weight"
title: "Strength-versus-Weight Theory of Biased Updating"
reading_id: "10_behavioral_finance"
summary: "Griffin-Tversky: belief updating forms an impression from evidence strength (extremity, ~ sample proportion) then under-adjusts for weight (reliability, ~ sample size); explains overreaction to sample proportion and underreaction to sample size."
tags: ["behavioral-finance", "representativeness", "belief-updating", "over-underreaction"]
citations:
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p155:0250"
    chunk_hash: "f6dc5630a12eb2dfbf23abba026c482d7a7b277835f8c265de729d87f86a65a2"
    page_range: [155, 155]
    quote: "of the evidence, and then they adjust this impression based on the"
    edge_type: "defines"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p155:0250"
    chunk_hash: "f6dc5630a12eb2dfbf23abba026c482d7a7b277835f8c265de729d87f86a65a2"
    page_range: [155, 155]
    quote: "(4.14), Griffin and Tversky found that the coefficient on sample proportion is greater than the coefficient on sample size."
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p156:0252"
    chunk_hash: "bec73e7718897f52f0d65b1e7f2992f9a66e732c5f20c33428937de75e72268c"
    page_range: [156, 156]
    quote: "The higher likelihood is identified with the strength of the evidence and the lower likelihood with the weight."
    edge_type: "supports"
card_hash: "ccee3efd69762abed72777423dc6fe4d193bba3dedc7c6f98a063508687f3117"
---
# Strength-versus-Weight Theory of Biased Updating

## Intuition
Griffin and Tversky proposed a single framework to unify a range of updating biases. Their key idea is that the psychological process of belief updating has two stages: people first form an impression from the *strength* of the evidence — how extreme or salient it is — and then adjust that impression for the *weight* of the evidence — how reliable or informative it is. The adjustment for weight is insufficient, so judgments are excessively driven by the representativeness-related features (strength) and not enough by the statistical credence (weight).
**Source:** Griffin & Tversky, in Benjamin (2019) Ch.2 §7.2 p.155.

This two-stage structure explains seemingly opposite phenomena at once. When the strength of evidence is high but its weight is low (a striking result from a tiny sample), people overreact. When strength is moderate but weight is high (an unremarkable result from a large sample), people underreact. The same machinery thus reconciles over-inference and under-inference within one theory.
**Source:** Griffin & Tversky, in Benjamin (2019) Ch.2 §7.2 pp.155-156.

## Definition
**Strength** of evidence is its extremeness or salience — for binary signals, the sample proportion `(N_a − N_b)/N` — determined by representativeness.
**Source:** Griffin & Tversky, in Benjamin (2019) Ch.2 §7.2 p.155.

**Weight** of evidence is its credence or reliability — the features that matter for normatively correct updating, e.g. the sample size `N`.
**Source:** Griffin & Tversky, in Benjamin (2019) Ch.2 §7.2 p.155.

**Biased updating (strength-vs-weight)** is the pattern in which the initial impression tracks strength and the subsequent adjustment for weight is too small, so inferences are over-sensitive to sample proportion and under-sensitive to sample size.
**Source:** Griffin & Tversky, in Benjamin (2019) Ch.2 §7.2 pp.155-156.

## Mathematical Reasoning
Following Kahneman and Tversky, in inferring from a sample of binary signals the sample proportion `(N_a − N_b)/N` is *strength* and the sample size `N` is *weight*. In bookbag-and-poker-chip updating problems where the estimated coefficient on sample proportion exceeds the coefficient on sample size, the theory is confirmed: inferences are too sensitive to proportion and insufficiently sensitive to size. Griffin and Tversky found over-inference from small sample sizes (3 and 5) and under-inference from larger ones (9, 17, 33), consistent with the relatively small coefficient on size.
**Source:** Griffin & Tversky, in Benjamin (2019) Ch.2 §7.2 pp.155-156.

The framework also explains base-rate neglect by identifying the *likelihood* information with strength and the *prior probabilities* with weight, since prior probabilities do not enter judgments of representativeness; posteriors then come out insufficiently sensitive to the priors. A third application: when evaluating the likelihood ratio `p(S|A)/p(S|B)`, people focus on how well evidence fits the *given* hypothesis `p(S|A)` (the higher, strength) and underweight the *alternative* `p(S|B)` (the lower, weight). When rates are close `(θ_A, θ_B) = (0.6, 0.5)` people over-infer; when rates are far apart `(0.6, 0.25)` they dramatically under-infer — "participants overweighted the numerator and underweighted the denominator."
**Source:** Griffin & Tversky, in Benjamin (2019) Ch.2 §7.2 p.156.

```
   evidence -> [ STRENGTH: extremity, ~ sample proportion ] -> initial impression
            -> [ WEIGHT:  reliability, ~ sample size N    ] -> adjustment (TOO SMALL)
   high strength + low weight  -> OVERREACTION
   low strength  + high weight -> UNDERREACTION
```
**Source:** Griffin & Tversky, in Benjamin (2019) Ch.2 §7.2 pp.155-156.

## See Also
- [be-gamblers-fallacy-law-of-small-numbers](./be-gamblers-fallacy-law-of-small-numbers.md#intuition) — small-sample representativeness as the source of over-inference.
- [be-diagnostic-expectations](./be-diagnostic-expectations.md#intuition) — representativeness-as-diagnosticity formalized for asset pricing.
- [be-representativeness-conjunction-base-rate](./be-representativeness-conjunction-base-rate.md#intuition) — representativeness driving conjunction fallacy and base-rate neglect.

## Escalate to Raw When
- You need the Antoniou et al. replication that controlled for risk preferences and found under-inference at all sample sizes. **Source:** Benjamin (2019) Ch.2 §7.2 p.156.
- You need how strength-vs-weight maps onto the chapter's reduced-form `c`-parameter updating model and the broader representativeness-vs-specific-biases debate. **Source:** Benjamin (2019) Ch.2 §7.3-7.4 pp.156-162.
