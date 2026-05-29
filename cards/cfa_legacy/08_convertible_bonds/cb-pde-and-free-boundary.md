---
schema_version: "cacg.v0"
id: "cb-pde-and-free-boundary"
title: "Convertible PDE and Free Boundaries"
reading_id: "08_convertible_bonds"
summary: "Convertible PDE and Free Boundaries — placeholder summary                       "
tags: ["convertible-bonds", "pde-free"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p164:0217"
    chunk_hash: "6ae864fbf03e08ba391f5180490627a4d3f971f74a7ff444967de2a7464bdf8a"
    page_range: [164, 165]
    quote: "This is where lattice techniques such as binomial or trinomial trees enter the stage. These approaches can handle a fair amount of instrument complexity but remain very educational."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p164:0217"
    chunk_hash: "6ae864fbf03e08ba391f5180490627a4d3f971f74a7ff444967de2a7464bdf8a"
    page_range: [164, 165]
    quote: "Other possibilities to solve for the price of a hybrid instrument after having chosen a particular stochastic process are the finite difference and the Monte Carlo method."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p651:0968"
    chunk_hash: "0c1ac86a333b1329e33bc49bdf424202104f13df481a1858657a6c47ba2dcea5"
    page_range: [651, 652]
    quote: "We then roll back through the tree. At nodes where the terms of the instrument allow conversion we test whether conversion is optimal."
    edge_type: "supports"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p102:0122"
    chunk_hash: "60ff35e0a1b878067742233d3b9a780d017c970cd6f19d549f62a1df576d872a"
    page_range: [102, 103]
    quote: "The hazard function is of particular interest in default modeling because of its link to conditional default probabilities, which is similar to the link we saw in the discrete-time case."
    edge_type: "supports"
card_hash: "8119a499dbfb76f1cb9120667e52a457f804bb0c46a0bda37446c3b7d4d5bc08"
---
# Convertible PDE and Free Boundaries

## Intuition

The credit-aware binomial tree from the
[binomial-tree card](./cb-binomial-tree-valuation.md#mathematical-reasoning)
converges as `Δt → 0` to a continuum partial differential equation
(PDE) with three free boundaries: one for holder-driven American
conversion, one for issuer-driven call, and one for holder-driven
put. The PDE is a credit-augmented Black-Scholes-Merton equation in
which the share-price diffusion is shocked by an instantaneous
default event at intensity `h(t)`. The free boundaries partition the
state space `(S, t)` into a continuation region and three
exercise regions, exactly mirroring the tree's node-by-node
decisions.
**Source:** DeSpiegeleer et al. (2014) §3.6-§3.8 pp.95-145.

```
state-space partition (S, t):

   S
   ^                  forced-conversion region
   |        ____________________________________
   |       /    (S ≥ S*_conv(t)): holder converts
   |      /
   |     /  continuation region
   |    /     V satisfies the PDE
   |   /
   |  /     ____________________________________
   | /     (V ≤ P_put(t)): holder puts
   |/      ____________________________________
   +-----------------------------> t

   issuer's call region: V_continue ≥ K(t) ⇒ V = K(t)
   (typically active in upper-right region)
```

## Definition

Let `V(S, t)` denote the convertible's price as a function of the
share price `S` and calendar time `t ∈ [0, T]`. Under the standard
GBM share dynamics with constant volatility `σ`, dividend yield `δ`,
hazard rate `h`, and risk-free rate `r`, the **convertible PDE** in
the continuation region is given below. **Source:** DeSpiegeleer et
al. (2014) §3.6 pp.95-130.

```
PDE in the continuation region:

  ∂V/∂t  +  (r − δ) · S · ∂V/∂S
        +  ½ · σ^2 · S^2 · ∂^2V/∂S^2
        −  (r + h) · V
        +  h · R · F
        +  c · F
        =  0
```

The terms have the standard interpretations: `(r − δ) · S · ∂V/∂S` is
the drift of the underlying share under the risk-neutral measure
adjusted for the dividend yield; `½ · σ^2 · S^2 · ∂^2V/∂S^2` is the
diffusion term (BSM-style); `−(r + h) · V` combines the riskless rate
and the hazard rate (the hazard "kills" the bond at rate `h`);
`h · R · F` is the instantaneous default-recovery accrual rate; and
`c · F` is the continuous coupon accrual per unit of face.
**Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-130; Hull (recent
ed.) §27.4 pp.650-653.

The **terminal condition** at maturity `T` is the optional CB payoff.
**Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-130.

```
V(S, T) = max( q · S, F )
```

The **boundary conditions** at the three free boundaries are defined
below. **Source:** DeSpiegeleer et al. (2014) §3.6-§3.8 pp.95-145;
Hull (recent ed.) §27.4 pp.650-653.

- **Holder conversion boundary** `S^*_conv(t)`: at the conversion
  boundary `V = q · S` and `∂V/∂S = q` (smooth-pasting / value-
  matching). For `S > S^*_conv(t)`, the holder converts immediately.
  **Source:** DeSpiegeleer et al. (2014) §3.7 pp.130-145.
- **Issuer call boundary** (when call-eligible): the issuer caps the
  convertible at the call price `K(t)`. If `V_continue > K(t)`, the
  issuer calls; the holder optimally converts (under the screw-
  clause from the
  [call-and-put card](./cb-call-and-put-protection.md#mathematical-reasoning))
  to capture the parity excess. **Source:** Hull (recent ed.) §27.4
  pp.650-653.
- **Holder put boundary** (when put-eligible): the holder floors the
  convertible at the put price `P_put(t)`. If `V_continue < P_put(t)`,
  the holder puts. **Source:** DeSpiegeleer et al. (2014) §2.5
  pp.65-78.

## Mathematical Reasoning

The PDE arises from no-arbitrage in a delta-hedged portfolio under a
defaultable share: the hedger holds one convertible long and `Δ_S`
shares short; the instantaneous P&L contains a Wiener term that
vanishes by the BSM hedge plus a default-jump term that is set to
zero by the recovery accrual rate `h · R · F`; setting the drift of
the hedged portfolio equal to the riskless rate `r · V` and
rearranging yields the PDE above. **Source:** DeSpiegeleer et al.
(2014) §3.6 pp.95-130; Lando (2004) §3-§4 pp.60-130.

The **convergence of the credit-aware tree** to this PDE proceeds in
the standard manner: as `Δt → 0` and the tree depth `N → ∞`, the
discrete backward-induction operator converges to the differential
operator above, so the tree is the canonical numerical scheme for the
PDE (alongside finite-difference methods). **Source:** Hull (recent
ed.) §27.4 pp.650-653; DeSpiegeleer et al. (2014) §3.6 pp.95-130.

The **smooth-pasting condition** at `S^*_conv(t)` enforces continuity
of value AND first derivative. **Source:** DeSpiegeleer et al.
(2014) §3.7 pp.130-145.

```
Smooth-pasting at the conversion boundary S^*_conv(t):

  V(S^*_conv(t), t)         =  q · S^*_conv(t)        (value match)
  ∂V/∂S |_{S = S^*_conv(t)} =  q                       (slope match)
```

The **early-conversion threshold's location** is determined by the
free-boundary condition itself: `S^*_conv(t)` is the smallest `S`
at which `q · S(t) ≥ V_continue(S, t)`, with `V_continue` solving
the PDE in the continuation region under the call/put/credit
constraints. **Source:** DeSpiegeleer et al. (2014) §3.7
pp.130-145. As in the
[conversion-mechanics card's](./cb-conversion-feature-mechanics.md#mathematical-reasoning)
analysis, value matching plus smooth pasting are the joint
conditions that pin the boundary; issuer calls, holder puts,
credit spreads, volatility, and the dividend-vs-coupon trade-off
all shift `S^*_conv(t)`. In the simplified no-call / no-put /
stable-credit / European-style setting, the dividend-yield-vs-
coupon-yield comparison is the leading-order intuition: early
conversion looks attractive when the dividend yield on the
converted shares exceeds the coupon yield being given up, but
that comparison is a heuristic, not the boundary condition.
**Source:** DeSpiegeleer et al. (2014) §3.7 pp.130-145.

Asymptotic limits (cases below). **Source:** DeSpiegeleer et al.
(2014) §3.6 pp.95-130; Hull (recent ed.) §27.4 pp.650-653.

- `h → 0` (riskless): the PDE reduces to the standard BSM equation
  for an American option plus a straight-bond term; solvable with
  the bond-plus-call identity from the
  [payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning).
  **Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-130.
- `Δt → 0` (continuum tree): the credit-aware CRR tree's
  backward-induction recursion converges at rate `O(Δt)` for European
  payoffs; smooth-pasting introduces an `O(1/√N)` correction near the
  conversion boundary. **Source:** Hull (recent ed.) §27.4 pp.650-653.
- `r + h → 0`: the "infinite-life" limit; the PDE coefficient on
  `V` vanishes; the convertible behaves like a perpetual; rare in
  practice. **Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-130.

## See Also

- [`cb-binomial-tree-valuation.md`](cb-binomial-tree-valuation.md) — the discrete numerical scheme
- [`cb-credit-spread-machinery.md`](cb-credit-spread-machinery.md) — the hazard `h(t)` consumed by the PDE
- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — the early-conversion free boundary
- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — the issuer-call and holder-put boundaries

## Escalate to Raw When

Open DeSpiegeleer §3.6-§3.8 pp.95-145 directly for the formal PDE
derivation, the smooth-pasting analysis, and the connection to
finite-difference numerical schemes. **Source:** DeSpiegeleer et al.
(2014) §3.6-§3.8 pp.95-145.

Open Hull §27.4 pp.650-653 for the credit-aware-tree-to-PDE
correspondence and the canonical convergence-rate analysis.
**Source:** Hull (recent ed.) §27.4 pp.650-653.

Open Lando §3-§4 pp.60-130 for the formal treatment of the hazard-
augmented diffusion and the alternative recovery conventions that
modify the `h · R · F` term in the PDE. **Source:** Lando (2004)
§3-§4 pp.60-130.
