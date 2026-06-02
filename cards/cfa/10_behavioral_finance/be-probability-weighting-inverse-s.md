---
schema_version: "cacg.v0"
id: "be-probability-weighting-inverse-s"
title: "Probability Weighting: The Inverse-S Function"
reading_id: "10_behavioral_finance"
summary: "The probability weighting function w(p) is commonly inverse-S: small probabilities overweighted and moderate-to-large underweighted, decomposing into a cognitive likelihood-insensitivity component and a motivational pessimism (elevation) component."
tags: ["behavioral-finance", "probability-weighting", "likelihood-insensitivity", "pessimism"]
citations:
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p220:0294"
    chunk_hash: "c81e7eb233151384c04a5b096c740453bb494c9e2190412e744bb58dcc69203e"
    page_range: [220, 220]
    quote: "the overweighting of small probabilities, and Tversky & Kahneman (1992) incorporated this phenomenon."
    edge_type: "supports"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p241:0323"
    chunk_hash: "535fc9a6b3206a0c9d63352c9421e5002f0ef2dc7223deac2702ca138a2781d8"
    page_range: [241, 241]
    quote: "Best-rank overweighting has sometimes been called subadditivity, or lower"
    edge_type: "defines"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p225:0299"
    chunk_hash: "5b5e831f6645fac7c39aa2d85740b33c6f3619838977172734cd60ac3f882fb0"
    page_range: [225, 225]
    quote: "6 Thus a is an anti-index of likelihood insensitivity."
    edge_type: "defines"
card_hash: "e0b11ab026fbf41a38dfa703f764e5364fdbd90e32bf0463de4a5abc95ca65f1"
---
# Probability Weighting: The Inverse-S Function

## Intuition

The probability weighting function `w(p)` maps the cumulative ranked probability into a decision weight. Empirically it is not the identity: the prevailing shape is inverse-S, steep near the endpoints `p=0` and `p=1` and shallow in the middle. This means small probabilities are overweighted (the possibility effect, hope, lottery-buying) and moderate-to-large probabilities are underweighted (the certainty effect, the gap between "very likely" and "sure"). People pay too much attention to extreme and exceptional outcomes and too little to changes among the moderate middle.
**Source:** Wakker (2010) §7.1 pp.204-205.

Crucially, the inverse-S decomposes into two psychologically orthogonal components. *Likelihood insensitivity* is cognitive: a regressive failure to discriminate among intermediate probabilities, weighting them all toward a "don't know" middle. *Pessimism* (its complement, elevation/optimism) is motivational: a uniform tilt of the curve that overweights bad ranks. The common finding mixes both — a steep, regressive curve sitting below the diagonal in its upper part.
**Source:** Wakker (2010) §7.1 pp.204-205.

## Definition

**Likelihood insensitivity** holds when `w` is shallow (insensitive) in a middle region with decision weights at the extremes dominating those in the middle; formally `w(p) >= w(p+r)-w(r)` for ranks in the best-rank region (best-rank overweighting / lower subadditivity) and the symmetric condition in the worst-rank region (worst-rank overweighting / upper subadditivity).
**Source:** Wakker (2010) §7.7 pp.223, §7.7 pp.225.

**Pessimism** holds when worsening the rank raises the decision weight; equivalently `w` is convex.
**Source:** Wakker (2010) §6.3 pp.174-175.

**Subproportionality** is the Prelec property making the weighting family well-suited to very small and very large probabilities (Prelec's compound-invariance family `exp(-(-ln p)^a)^b`).
**Source:** Wakker (2010) §7.2 pp.206-207.

## Mathematical Reasoning

The neo-additive family makes the decomposition explicit:

```
w(0)=0; w(1)=1; w(p) = b + a*p for 0 < p < 1, with a >= 0, b >= 0, a + b <= 1.
```

Here `a` is an index of likelihood sensitivity ("curvature" / inverse-S), and `(2b+a)/2` is an index of optimism (elevation). A low `a` means a flat middle (high insensitivity); the intercept `b` and the jumps at the endpoints carry the elevation. As `a -> 0` the curve becomes the extreme three-degrees-of-belief step ("sure," "don't know," "sure not").
**Source:** Wakker (2010) §7.2 pp.209, §7.10 pp.229.

Tversky & Kahneman's (1992) one-parameter family is

```
w(p) = p^c / ( p^c + (1-p)^c )^{1/c},   c = 0.61 best fits data.
```

For `c < 1` this generates the inverse-S; decreasing `c` deepens both the insensitivity and the pessimism simultaneously, which is why a single-parameter family conflates the two components. Best-rank overweighting (lower subadditivity) and worst-rank overweighting (upper subadditivity) are the two ends of the inverse-S; with `b_rb` usually taken at 0, worst-rank overweighting dominates so the curve crosses the diagonal below `1/2` and `w(1/2) <= 1/2`.
**Source:** Wakker (2010) §7.2 pp.206, §7.9 pp.227, §7.7 pp.225.

```
w(p)
 1 |                               .--*
   |                          .--''
   |                     _.-''   <- underweighting of large p
1/2|.................*'..............  (curve below diagonal)
   |          .-'                      45-degree line
   |      _.''  <- overweighting of small p
 0 *-''________________________________ p
   0            1/2                    1
```

## See Also

- [be-rank-dependent-utility-via-ranks](./be-rank-dependent-utility-via-ranks.md#intuition) — `w` is applied to the cumulative ranks defined there.
- [be-cumulative-prospect-theory-risk](./be-cumulative-prospect-theory-risk.md#intuition) — uses separate `w+` and `w-` for gains and losses.
- [be-ambiguity-sources-ellsberg](./be-ambiguity-sources-ellsberg.md#intuition) — source functions reuse this weighting apparatus for ambiguity.
- [be-myopic-loss-aversion-equity-premium](./be-myopic-loss-aversion-equity-premium.md#intuition) — downstream equity-premium pricing application.

## Escalate to Raw When

- You need the full formal definition of likelihood insensitivity with the insensitivity region `[b_rb, w_rb]` and the boundary restrictions in Eqs. (7.7.1)-(7.7.6).
**Source:** Wakker (2010) §7.7 pp.223-225.
- You need the parametric weighting families (Prelec, Goldstein-Einhorn, neo-additive) with their fitted parameters and behavioral foundations.
**Source:** Wakker (2010) §7.2 pp.206-209.
