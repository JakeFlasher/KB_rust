---
schema_version: "cacg.v0"
id: "cb-payoff-decomposition-bond-plus-call"
title: "Payoff Decomposition: Bond Plus Call"
reading_id: "08_convertible_bonds"
summary: "Payoff Decomposition: Bond Plus Call — placeholder summary                      "
tags: ["convertible-bonds", "payoff-decomposition"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p242:0371"
    chunk_hash: "834ae1b123738b60e5d1e1c1a09c41767bff9374213356771e8430f1eef484b0"
    page_range: [242, 242]
    quote: "They are therefore bonds with an embedded call option on the company’s stock."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p651:0967"
    chunk_hash: "c04abce6411df0c614d161917476d374cfabf4393d43ac4e6e20224be5a3e12f"
    page_range: [651, 651]
    quote: "Credit risk plays an important role in the valuation of convertibles. If credit risk is ignored, poor prices are obtained because the coupons and principal payments on the bond are overvalued."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p042:0046"
    chunk_hash: "151fe27cccb5c61ac865511bb7dd5222c5758b749045f100eb8e4c94bb4a2c6e"
    page_range: [42, 43]
    quote: "Splitting a convertible bond into a bond and a European option is only possible if the conversion right is restricted to the maturity date of the convertible bond."
    edge_type: "supports"
  - source_id: "cb_thorp_kassouf_1967_beat_the_market"
    chunk_id: "cb_thorp_kassouf_1967_beat_the_market:p050:0047"
    chunk_hash: "f47169e1593bf641ce481b99cbf00a9d4b83af796885b88b84f4c8f61530b39d"
    page_range: [50, 51]
    quote: "Our contribution has been to scientifically analyze warrants, particularly the warrant hedge, and to extend our methods to the vast area of all convertibles and their associated common stock, with a market value of perhaps $50 billion."
    edge_type: "supports"
card_hash: "476e8f31ea5c03e77c91037a568a12420425c5ea2ccce0a06fa753883e4cc714"
---
# Payoff Decomposition: Bond Plus Call

## Intuition

A convertible bond can be viewed as the sum of a straight (credit-risky) bond
and a long position in `N` call options on the issuer's share, struck at the
conversion price `K_c`. As an **identity** this is exact only for a narrowly-
specified non-callable, non-puttable, European-conversion, no-default
convertible; for real-world callable / credit-risky / American-conversion
issues it is an approximation that requires tree or PDE machinery to make
quantitative.
**Source:** Hull (recent ed.) §10.11 pp.241-242 ("convertibles are bonds with
an embedded call option"); Hull (recent ed.) §27.4 pp.650-653.

```
   CB(S,t)        =      B(t)        +    N · c(S, K_c, σ, r, δ, T-t)
                    (straight bond)        (embedded calls)
       ^                  ^                          ^
       |                  |                          |
       |          (flat in S; sensitive       slopes upward in S
       |           to spread, rates)          zero at S = 0,
       |                                      linear at S → ∞
       |
       +----------------------------------------------------> S
```

## Definition

For a **non-callable, non-puttable, European-conversion, no-default**
convertible bond with face `F`, conversion ratio `q`, conversion price
`K_c = F/q`, maturity `T`, riskless rate `r`, and dividend yield `δ`, the
bond-plus-call identity states the decomposition below.
**Source:** Hull (recent ed.) §10.11 pp.241-242.

    V(S, t)  =  B(t)  +  q · c( S(t), K_c, σ, r, δ, T-t )

Here `B(t)` is the (riskless-discounted) straight-bond floor and
`c(S, K, σ, r, δ, τ)` is the Black-Scholes-Merton European call price with
continuous dividend yield `δ` treated as a separate input — **not** absorbed
into a shifted rate. **Source:** Hull (recent ed.) §10.11 pp.241-242.

For real-world convertibles the identity becomes an approximation, with
specific failure modes documented case-by-case in Hull §27.4 pp.650-653.
**Source:** Hull (recent ed.) §27.4 pp.650-653.

- **American-conversion** convertibles need an American-call value, typically
  obtained via a binomial / trinomial tree or a finite-difference scheme;
  early-conversion premium is non-zero on dividend-paying underlyings.
  **Source:** Hull (recent ed.) §27.4 pp.650-653.
- **Callable / puttable** convertibles add an issuer short-call and a holder
  long-put; the residual decomposition is `V ≈ B + q · c_holder − N_call ·
  c_issuer + N_put · p_holder`, with all options computed under the
  same model and credit-risky discount. **Source:** Hull (recent ed.) §27.4
  pp.650-653.
- **Credit-risky** issuers couple the discount factor in `B(t)` with the
  embedded call's discounting; standard practice is a credit-aware tree
  (Hull §27.4 pp.650-653) or a coupled PDE. **Source:** Hull (recent ed.)
  §27.4 pp.650-653.

**Source:** Hull (recent ed.) §27.4 pp.650-653; DeSpiegeleer et al. (2014)
§3.2 pp.65-78.

## Mathematical Reasoning

The European-conversion identity follows from the no-arbitrage replication
argument: a portfolio long the convertible and short `q` shares (financed by
borrowing the strike `K_c` at the riskless rate) replicates the conversion
exchange exactly when no default occurs and the holder has only a single
exercise opportunity at maturity. **Source:** Hull (recent ed.) §10.11
pp.241-242.

Under the European-no-default special case, the dual-floor inequality
`V(S,t) ≥ max(B(t), q · S(t))` follows from `c ≥ 0` (calls are non-negative)
and the standard Black-Scholes lower bound for a call on a dividend-paying
share, `c(S, K, σ, r, δ, τ) ≥ max(0, S · e^(-δτ) − K · e^(-rτ))`. The
parity-floor `V ≥ q · S` is recovered when the bond floor's embedded "buy
the strike at maturity" component plus the call's intrinsic value reproduce
`q · S(t)` exactly. **Source:** Hull (recent ed.) §10.11 pp.241-242;
DeSpiegeleer et al. (2014) §3.2 pp.65-78.

Under the same special case, Greeks of the convertible inherit cleanly from
the decomposition: delta and gamma scale by `q`, vega scales by `q`, and
theta / rho split additively across the bond and call legs. The mapping
from decomposition Greeks to traded-CB Greeks is itself only approximate for
callable / credit-risky issues. **Source:** DeSpiegeleer et al. (2014) §3.4
pp.78-95.

Asymptotics of the special-case identity match those of the
[parity card](./cb-parity-and-conversion-value.md#mathematical-reasoning):

- `S → 0`: `c → 0` and `V → B(t)`. The CB collapses to the riskless-
  discounted bond floor; in the credit-risky general case, the same limit
  is reached but `B(t)` itself can collapse toward a recovery payment (see
  the [bond-floor card](./cb-bond-floor-investment-value.md#mathematical-reasoning)).
  **Source:** DeSpiegeleer et al. (2014) §3.4 pp.78-95.
- `S → ∞`: `c → S · e^(-δτ) − K_c · e^(-rτ)` (deep-ITM European call), so
  `V/S(t) → q · e^(-δτ)`; in the equity-dominant regime the convertible
  behaves like a long-equity holding with dividend-yield drag. **Source:**
  Hull (recent ed.) §10.11 pp.241-242.
- `S = K_c` (at-the-money on a face basis): `c` has its maximum gamma and
  vega; `V` exhibits maximum convexity, matching the "balanced CB" regime
  where conversion premium peaks. **Source:** DeSpiegeleer et al. (2014)
  §3.4 pp.78-95.

The decomposition predates Black-Scholes: Thorp & Kassouf (1967) used a
warrant-pricing framework anchored on the same long-bond + long-warrant
view to systematize convertible arbitrage in the 1960s before the
theoretical machinery existed. **Source:** Thorp & Kassouf (1967) §3
pp.50-90.

### Embedded-Call Payoff Diagram

The intrinsic payoff `q · max(S(T) − K_c, 0)` of the embedded call leg
at expiry plots as a long-call hockey-stick, scaled by the conversion
ratio `q`. **Source:** Hull §10.11 pp.241-242; Hull §27.4 pp.650-653.

```
<!-- primitive: call-payoff source: _diagram_primitives.md -->
payoff
   ^
   |             /
   |            /
   |           /
   |          /
   |---------+--------------> S
              K
   intrinsic = max(S - K, 0)
```

## See Also

- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — `B(t)` leg
- [`cb-parity-and-conversion-value.md`](cb-parity-and-conversion-value.md) — `q · S(t)` asymptote
- [`cb-conversion-premium.md`](cb-conversion-premium.md) — `V - q · S(t)` interpreted as call time-value
- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — mechanics of `q` and `K_c`

## Escalate to Raw When

Open Hull §10.11 pp.241-242 directly when the special-case identity needs to
be re-derived with different dividend conventions or different rate models.
**Source:** Hull (recent ed.) §10.11 pp.241-242.

Open Hull §27.4 pp.650-653 for the credit-aware convertible tree.
**Source:** Hull (recent ed.) §27.4 pp.650-653.

- The bond is callable / puttable — an issuer call adds a short call to the
  holder; a holder put adds a long put; a credit-aware tree resolves
  the joint exercise boundary. **Source:** Hull (recent ed.) §27.4
  pp.650-653.
- The conversion is American-style and the early-exercise premium is
  material on a dividend-paying underlying. **Source:** Hull (recent ed.)
  §27.4 pp.650-653.
- Dividend protection or anti-dilution clauses break the homogeneity used
  in the special-case decomposition. **Source:** DeSpiegeleer et al. (2014)
  §3.2 pp.65-78.

Open DeSpiegeleer §3.2-§3.4 pp.65-95 when calibrating the embedded-call
volatility against listed convertible quotes (the "implied volatility" of a
CB is the σ that reconciles the decomposition with the market price).
**Source:** DeSpiegeleer et al. (2014) §3.2-§3.4 pp.65-95.

Open Thorp & Kassouf (1967) for the historical lineage and the practitioner
arbitrage strategies that first exploited mispricings in the
bond-plus-warrant decomposition. **Source:** Thorp & Kassouf (1967) §3
pp.50-90.
