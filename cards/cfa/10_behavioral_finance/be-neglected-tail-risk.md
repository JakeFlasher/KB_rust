---
schema_version: "cacg.v0"
id: "be-neglected-tail-risk"
title: "Neglected Tail Risk"
reading_id: "10_behavioral_finance"
summary: "Under good news with non-rising volatility, diagnostic investors thin the left tail, perceive cash flows as too safe, over-issue safe-looking (AAA) debt, and build hidden fragility -- so that when a bad tail state later becomes representative, fire sales and price collapses follow."
tags: ["behavioral-finance", "neglected-risk", "diagnostic-expectations", "financial-fragility"]
citations:
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p231:0220"
    chunk_hash: "d86c5bf0fe7555fa14c968097f1c7b72095d3191ba2ef5b9294d6952038f160d"
    page_range: [232, 232]
    quote: "Agents neglect downside risk in the sense of definition 3.1 if and only if cash flow volatility has not increased relative to the past"
    edge_type: "defines"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p173:0165"
    chunk_hash: "a4c0c5555b7e0dfc485d22be26d8ca3d0f9afaf8d848250629581d40c60b5669"
    page_range: [174, 174]
    quote: "When market participants perceive a higher mean and a lower variance, they are optimistic about the average return on mortgages and they neglect tail risk."
    edge_type: "supports"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p175:0167"
    chunk_hash: "e4cf91ce4da7daa388119861361fa130059387a841e9f9f6021b5cf1c1705fe2"
    page_range: [176, 176]
    quote: "the pooling of risks exposed the entire financial sector to the same neglected risk."
    edge_type: "supports"
card_hash: "5c3815091682b0dd60e97b7bbd816134dafd554161a7f938fa594b382d7379a9"
---
# Neglected Tail Risk

## Intuition

Neglected tail risk is the diagnostic mechanism behind financial fragility. When good news arrives and cash-flow volatility has not risen, the diagnostic distribution shifts right and develops a thinner left tail: extreme bad outcomes become unrepresentative relative to the recent past, so they are recalled poorly and under-weighted in beliefs. Investors then perceive cash flows as *safer* than they truly are. This perceived safety, reinforced by risk-pooling and securitization that genuinely diversify idiosyncratic risk, leads intermediaries to issue and investors to absorb excessive quantities of safe-looking (AAA-rated) debt.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.156-159, 161.

The fragility is hidden because the neglected risk is *systematic*: pooling diversifies idiosyncratic shocks but exposes the entire financial sector to the same aggregate left-tail event. When news later makes a bad tail state representative -- a black-swan event seen as not merely possible but representative -- diagnostic beliefs over-react in reverse: perceived variance jumps, even holders of AAA assets perceive large risk and liquidate, and fire sales and price dislocations are far larger than rational investors expected. Diagnostic beliefs thus accelerate both the boom-time over-expansion and the crisis-time collapse.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.162-163, 165.

## Definition

**Neglect of downside risk** (here, definition 3.1) holds when agents underestimate the frequency of left-tail events; under diagnostic lognormal beliefs it arises if and only if cash-flow volatility has not increased relative to the past, `sigma_0^2 <= sigma_{-1}^2`.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.156.

**Diagnostic neglect threshold** `X_underbar` is the cash-flow level below which diagnostic beliefs neglect downside risk; it rises with good news and with the distortion `theta`, pushing more of the left tail into the neglected region.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.156.

**Excess safe-debt issuance** is the equilibrium over-supply of AAA-type debt `N^theta_0` that obtains when the perceived mean rises and perceived variance falls, so investors willingly absorb more debt than fundamentals warrant.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.161.

## Mathematical Reasoning

Proposition 5.2 gives the neglect threshold under diagnostic lognormal beliefs,

```
  X_underbar = mu_0 + theta * phi(sigma_{-1}/sigma_0) * (mu_0 - mu_{-1}),
```

with `phi(.) > 0` decreasing in `sigma_{-1}/sigma_0` and diverging to `+infinity` as the ratio approaches 1. A *necessary* condition for neglect is that volatility has not risen, `sigma_0^2 <= sigma_{-1}^2`; a more volatile environment instead makes extreme realizations representative and over-weights the left tail. Abstracting from volatility (`sigma_0^2 = sigma_{-1}^2`), neglect arises if and only if good news raises the mean, `mu_0 > mu_{-1}` -- the same condition that drives excess optimism in Proposition 5.1.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.156-157.

The AAA constraint pins safe-debt issuance. With `z* < 0` the `delta*`-percentile of the standard normal (Proposition 5.3),

```
  ln N^theta_0 = mu_0(theta) + sigma_0(theta) * z*.
```

Because a higher perceived mean `mu_0(theta)` and a lower perceived variance `sigma_0(theta)` both raise the right-hand side, safe-debt issuance is excessive precisely when `mu_0 > mu_{-1}` and `sigma_0^2 < sigma_{-1}^2`. In a crisis, a jump in perceived variance `sigma_0(theta)` has drastic consequences for assets characterized by a very negative `z*` (deep AAA tranches), forcing liquidation.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.161-162.

```
   perceived left tail under good news (theta > 0):

         true density            diagnostic density
              /\                        /\
             /  \                      /  \
   ____ _ _ /    \____         ______/      \____
   ^         neglected            thin left tail
   |  bad tail outcomes pushed below X_underbar and under-weighted
   |  --> over-issue safe debt --> hidden systematic fragility
```

**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.159 (Figure 5.2).

## See Also

- [be-diagnostic-expectations](./be-diagnostic-expectations.md#mathematical-reasoning) -- the operator and lognormal mean/variance distortions that thin the tail.
- [be-limits-of-arbitrage](./be-limits-of-arbitrage.md#intuition) -- why mispriced, fragile positions are not arbitraged away before the bust.
- [be-financial-crisis-narrative](./be-financial-crisis-narrative.md#intuition) -- neglected mortgage tail risk realized in 2007-09.
- [be-kernel-of-truth](./be-kernel-of-truth.md#mathematical-reasoning) -- why neglect vs over-weighting of the tail depends on whether news is good and volatility is falling.

## Escalate to Raw When

- You need the proof of Propositions 5.2 / 5.3 or the exact form of `phi(.)` and its limit (pp.156, Appendix).
- You need the GSV (2012) reduced-form "all low-probability cash flows neglected" special case and footnote 3 (pp.160).
- You need the fire-sale / "diversification myth" mechanism connecting pooling to systematic collapse (pp.162-163).
