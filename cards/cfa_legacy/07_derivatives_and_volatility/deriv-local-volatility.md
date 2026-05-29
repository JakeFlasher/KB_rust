---
schema_version: "cacg.v0"
id: "deriv-local-volatility"
title: "Local Volatility"
reading_id: "07_derivatives_and_volatility"
summary: "Local volatility σ_loc(S, t) is the deterministic function such that dS/S = r·dt + σ_loc(S, t)·dW^Q reproduces the entire observed implied-vol surface. Dupire's equation extracts σ_loc² from market call prices: σ_loc² = (∂C/∂T + r·K·∂C/∂K) / ((1/2)·K²·∂²C/∂K²). LSV models supply the joint-path richness exotics need."
tags: ["derivatives", "local-volatility"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p649:0965"
    chunk_hash: "187019bdfbd75c1ab9639ec7386a155ba6b084925df90a99b8b88358e5a56dc2"
    page_range: [649, 650]
    quote: "The volatility s1S, t2 is a function of both S and t and is chosen so that the model prices all European options consistently"
    edge_type: "defines"
card_hash: "2ba79909dec9369a77f0d14c074c63a1c3ad144413111121d6a3f73a64702eea"
---
# Local Volatility

## Intuition

Local volatility is a deterministic function `σ_loc(S, t)` that
makes the resulting one-factor diffusion model
`dS / S = r · dt + σ_loc(S, t) · dW^Q`
reproduce the entire observed implied-vol surface. The surface
is fixed at the calibration date; the local-vol surface is the
single state-and-time-dependent diffusion coefficient that is
consistent with all European-option market prices simultaneously.
Dupire's equation is the formula that extracts `σ_loc(S, t)`
from the surface; once calibrated, the model can price exotic
payoffs that are consistent with the European market.
**Source:** Hull §27 pp.626-635.

```
local-vol model

   underlying SDE:    dS/S = r · dt + sigma_loc(S, t) · dW^Q
                                       ^
                                       |
                          deterministic function of (spot, time)
                          calibrated to the implied-vol surface

   - reproduces ALL market European prices by construction
   - generates a non-flat smile / skew without stochastic vol
   - smile dynamics: surface flattens as spot moves
     (sticky-strike behavior; not always realistic)
```

## Definition

**Local volatility** `σ_loc(S, t)` is the deterministic
function such that the one-factor diffusion
`dS / S = r · dt + σ_loc(S, t) · dW^Q`
under the risk-neutral measure produces European call prices
matching the entire observed surface. **Dupire's equation**
extracts the local-vol surface from market call prices:
`σ_loc^2(K, T) = (∂C / ∂T + r · K · ∂C / ∂K) / ((1/2) · K^2 · ∂²C / ∂K^2)`,
where `C(K, T)` is the market call price as a function of strike
and expiry. The equation is a one-shot non-parametric
calibration: every cell of the surface gives one local-vol value.
**Source:** Hull §27 pp.626-640.

The model contrasts with stochastic-vol (where vol is itself a
random process) and with constant-vol BSM (where vol is a
single number). Local vol is **deterministic in `(S, t)`**, so
it does not introduce additional sources of randomness; the
randomness comes entirely from the Brownian motion of `S`. This
is why the local-vol model has a unique implied-vol surface by
construction: there is no second factor to leave the surface
mis-fit. **Source:** Hull §27 pp.626-640.

## Mathematical Reasoning

Dupire's equation derives from the Fokker-Planck equation for
the underlying's risk-neutral density `p^Q(S, T)`. The European
call price as a function of strike and expiry satisfies a
forward equation that mirrors the FP equation but in
calibration variables `(K, T)` rather than state variables
`(S, t)`. Inverting the algebra to solve for `σ_loc^2` yields
the Dupire formula. The derivation requires that the market
surface be smooth, arbitrage-free (no calendar arbitrage in the
`T` direction, no butterfly arbitrage in the `K` direction),
and twice-differentiable in `K`; arbitrage-free smile-fitting
upstream is therefore part of the calibration pipeline.
**Source:** Hull §27 pp.626-640.

By construction the implied-volatility-function (IVF) /
local-vol model fits every European one-time-payoff market
price exactly. The model is therefore calibration-correct for
European options, but Hull §27 cautions that it does not
necessarily produce the correct joint path distribution for
exotic / path-dependent payoffs — getting all European prices
right is a weaker condition than getting the underlying's full
risk-neutral law right, so exotic prices computed under local
vol can still differ from prices under a richer
underlying-and-vol joint dynamic. **Source:** Hull §27
pp.626-640.

The relationship to stochastic vol is one of complementarity.
Stochastic-vol models introduce a second randomness source
whose dynamics can be tuned to richer joint behaviour than
local vol's deterministic `σ_loc(S, t)`, but they generally do
NOT calibrate exactly to the European surface (smile-fitting
error remains). The practitioner solution is **local-stochastic
volatility (LSV)** models that combine a local-vol component
with a stochastic-vol component; the local component absorbs
the static European fit and the stochastic component supplies
the joint-path richness for exotics. The boundary into LSV is
past this card; it requires the SABR / Heston framing of
[`deriv-stochastic-vol-models.md`](./deriv-stochastic-vol-models.md#definition).
**Source:** Hull §27 pp.640-655.

## See Also

- [`deriv-vol-surface-anatomy.md`](deriv-vol-surface-anatomy.md) — implied-vol surface that local vol calibrates to
- [`deriv-stochastic-vol-models.md`](deriv-stochastic-vol-models.md) — stochastic-vol complement that supplies the smile dynamics local vol lacks

## Escalate to Raw When

Open Hull chapter 27 directly when any of the criteria below
applies. **Source:** Hull §27 pp.626-660.

- Numerical Dupire-extraction stability matters: the
  butterfly-arbitrage-free constraint is delicate near the
  surface boundaries and at illiquid strikes; smile-fitting
  upstream is critical. **Source:** Hull §27 pp.626-640.
- Local-stochastic-volatility (LSV) blending with SABR or
  Heston is needed; this is the practitioner default for
  exotic equity options. **Source:** Hull §27 pp.640-660.
- The card needs the implied-volatility-formula approximation
  for SABR; that is a smile parameterization separate from the
  local-vol diffusion. **Source:** Hull §27 pp.640-660.
