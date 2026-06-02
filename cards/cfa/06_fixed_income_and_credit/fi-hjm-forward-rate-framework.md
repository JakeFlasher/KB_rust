---
schema_version: "cacg.v0"
id: "fi-hjm-forward-rate-framework"
title: "HJM Forward-Rate Framework"
reading_id: "06_fixed_income_and_credit"
summary: "HJM Forward-Rate Framework — auto-generated placeholder summary; revise in fix-pass if needed; full audit notes available in audit_notes."
tags: ["fixed-income", "hjm-forward"]
citations:
  - source_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed"
    chunk_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed:p232:0301"
    chunk_hash: "61b13947189302dbb81be3016ca79aac47e1f382328098d80eb8092aa5c0c048"
    page_range: [232, 233]
    quote: "The Heath-Jarrow-Morton (HJM) Framework and the instantaneous forward rate f(t, T)= −∂ ln P(t, T) ∂T = −σ2 2 (T − t) 2 + θ(T − t)+ rt."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p737:1097"
    chunk_hash: "776c945f2ba7bb081dba2f85f9e2d4d10c195c1eff959d4e94cd5237255f4aab"
    page_range: [737, 738]
    quote: "In the Hull–White two-factor model when parameters are chosen appropriately, the volatility of the 3-month forward rate has a “humped” look."
    edge_type: "supports"
card_hash: "d170965de2b681d0b302e52e3e46a1fd491e22021736c848c7d6cb1a6ba28158"
---
# HJM Forward-Rate Framework

## Intuition

The Heath-Jarrow-Morton (HJM) framework treats the entire
forward-rate curve `f(t, T)` as the state, not the
single short rate. The drift of each forward-rate process
is fixed by the no-arbitrage condition; the modeler only
chooses the volatility structure. HJM is the natural
home for caps / floors / swaptions because their payoff
references a forward rate at a future date. **Source:**
Brigo+Mercurio (2006) Ch.5 pp.155-190.

```
f(t, T)
   ^
   |        T = 30y forward             T = 10y forward
   |        *  *  *      *  *  *  *  *
   |     *      .   *  .  *      *
   |   *      .       *      .
   |   o    .      T = 1y forward
   |     o-o-o-o-o-o-o-o-o-o-o-o-o-o-o
   |
   +-------------------------------------> t
   the entire forward curve is stochastic;
   each tenor has its own volatility profile.
```

## Definition

The HJM specification is
`df(t, T) = α(t, T) · dt + σ(t, T) · dW(t)` for each
fixed maturity `T`. The no-arbitrage condition pins the
drift to
`α(t, T) = σ(t, T) · ∫_t^T σ(t, u) · du` (under the
risk-neutral measure), so the modeler chooses only the
volatility function `σ(t, T)`. **Source:** Brigo+Mercurio
(2006) Ch.5 pp.155-190.

The short-rate models from
[`fi-short-rate-models.md`](./fi-short-rate-models.md#definition)
are special cases of HJM with specific choices of
`σ(t, T)`. Vasicek corresponds to
`σ(t, T) = σ · exp(-κ · (T - t))`; CIR corresponds to a
square-root variant. **Source:** Brigo+Mercurio (2006)
Ch.5 pp.155-190.

The Brace-Gatarek-Musiela (BGM) / LIBOR market model is
HJM applied to discrete LIBOR forward rates rather than
instantaneous forwards; it is the practitioner's tool for
cap / swaption pricing. **Source:** Hull §28
pp.720-740.

## Mathematical Reasoning

The HJM no-arbitrage drift condition follows from
requiring discounted bond prices to be martingales under
the risk-neutral measure. Once volatility is chosen, the
drift is forced. This collapses the curve-pricing degrees
of freedom: every forward rate's drift is determined by
the choice of `σ(t, T)`. **Source:** Brigo+Mercurio
(2006) Ch.5 pp.155-190.

The forward-rate algebra of
[`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#mathematical-reasoning)
states the no-arbitrage relationships at a single time `t`;
HJM is the dynamic extension that specifies how those
forward rates evolve. The static no-arbitrage at each `t`
is preserved by the HJM drift condition. **Source:**
Brigo+Mercurio (2006) Ch.5 pp.155-190.

The general HJM model is non-Markovian: the future
distribution of `f(t+s, T)` depends on the entire history
of `f(·, ·)`, not just `f(t, ·)`. Short-rate models are
the special case where one Markovian state suffices.
This is why short-rate models retain closed-form bond
prices and HJM in general does not. **Source:** Brigo+
Mercurio (2006) Ch.5 pp.155-190; Hull §28 pp.720-740.

For pricing instruments whose payoff depends on the
forward curve at a future date (caps, swaptions), HJM /
BGM is the natural framework: the reset / payment dates
align with the modeled forward-rate tenors. Short-rate
models can price these but require additional machinery
to extract the forward-rate distribution implied by the
short-rate process. **Source:** Hull §28 pp.720-740.

## See Also

- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — static no-arbitrage forward-rate algebra
- [`fi-short-rate-models.md`](fi-short-rate-models.md) — Markovian-state special cases of HJM

## Escalate to Raw When

Open Brigo+Mercurio Chapter 5 or Hull Chapter 28 directly
when any of the criteria below applies. **Source:** Brigo+
Mercurio (2006) Ch.5 pp.155-190;
Hull §28 pp.720-740.

- BGM / LIBOR-market-model calibration to a swaption
  surface is required; this card frames HJM but not
  practitioner BGM mechanics. **Source:** Hull §28
  pp.720-740.
- Multi-factor or stochastic-volatility HJM extensions
  are in scope; the single-factor case has known
  limitations for fitting the entire smile / skew.
  **Source:** Brigo+Mercurio (2006) Ch.5 pp.155-190.
- The risk-neutral / forward-measure / spot-measure
  change-of-numeraire algebra is needed for a specific
  payoff. **Source:** Brigo+Mercurio (2006) Ch.5
  pp.155-190.
