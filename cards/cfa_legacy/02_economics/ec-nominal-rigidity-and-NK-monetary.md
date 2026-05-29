---
schema_version: "cacg.v0"
id: "ec-nominal-rigidity-and-NK-monetary"
title: "Nominal Rigidity and New Keynesian Monetary Framework"
reading_id: "02_economics"
summary: "NK extends RBC with Calvo sticky prices so monetary shocks have short-run real effects; the three-equation DSGE workhorse pairs the dynamic IS (Euler equation), NK Phillips curve pi_t = beta E_t pi_{t+1} + kappa y-tilde_t derived from forward-looking Calvo pricing, and Taylor rule, yielding analytic impulse responses subject to the Taylor principle for determinacy."
tags: ["economics", "nominal-rigidity"]
citations:
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p259:0363"
    chunk_hash: "6a32ea2bc9039dd07b333e4357e7f749e02e4e04deff43d716b80403fc19feac"
    page_range: [259, 260]
    quote: "Equation (6.8) is known as the new Keynesian IS curve."
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p280:0390"
    chunk_hash: "a1c189d39fdd50c04ddf3efc096e9dc9dbeab8232ccb7f2f6ce98374087b34a6"
    page_range: [280, 281]
    quote: "There we will encounter specifications for inflation behavior firmly grounded in microeconomic assumptions, including a purely forward"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p870:1218"
    chunk_hash: "d3673f9437ea75688d6440281e99c0c4164bebc09e5eece04045e212b54ba7e8"
    page_range: [870, 871]
    quote: "This is the process whereby a central bank’s interest rate gets transmitted through the economy and ultimately affects the rate of increase of prices"
    edge_type: "supports"
card_hash: "0b31060dde78d39754f3b8c972c9f9c18dbbae8ed629d441be753e7cd59c81ab"
---
# Nominal Rigidity and New Keynesian Monetary Framework

## Intuition

The New Keynesian (NK) framework extends the flexible-price RBC baseline with **sticky prices**: a fraction of firms cannot reset their prices in any given period. With staggered price setting (Calvo pricing — each firm has a constant probability `1 − θ` of resetting its price each period, independent of how long ago it last reset), nominal demand shocks (e.g., from monetary policy) translate into real output movements in the short run, but eventually all prices reset and the long-run output level is determined by real fundamentals exactly as in RBC. NK is the modern micro-foundation for the Keynesian intuition that monetary policy has short-run real effects. **Source:** Romer (2019) Ch.6-7 pp.238-367.

```
   sticky-price response to a monetary expansion (NK)
   compared to flexible-price (RBC) response

   M ↑ at initial period (one-time level shift)

   RBC:                              NK:
   real output                       real output
   ^                                 ^
   |  Y* (unchanged — money         |          ___
   |   neutral in flex-price)       |         /   \
   +------------------>             |        /     \____ gradual return as
        t                           |       /            prices reset (Calvo)
                                    |------/   peak at ~quarter 4-8
                                    +------+------------> t

   inflation π                       inflation π
   ^   1-shot jump                  ^             ___
   |   to absorb M ↑                |            /   \____ gradual rise (sticky)
   |   then back to π̅               |   π̅ baseline
   +------------------>             +-----------+--------> t
```

The **three-equation NK model** is the modern macro workhorse: (i) a dynamic IS curve linking output gap to expected real interest rate, derived from the household's Euler equation; (ii) an NK Phillips curve linking inflation to marginal cost (proxied by the output gap), derived from Calvo pricing; (iii) a monetary-policy rule (Taylor rule) linking the nominal rate to inflation and the output gap. These three equations form a small linear DSGE system that admits closed-form solutions for the impulse response of inflation and output to demand and supply shocks. **Source:** Romer (2019) Ch.7 pp.290-367.

## Definition

The **Calvo pricing model** (the canonical micro-foundation for the NK Phillips curve) assumes each firm has a constant probability `1 − θ` of being able to reset its price in any period; otherwise the price stays nominal-fixed. The optimal reset price `P_t*` is the markup over a weighted-average of current and expected future nominal marginal costs: **Source:** Romer (2019) pp.238-367.

```
P_t*  =  μ · ( (1 − β θ) ·  ∑_{j=0}^∞  (β θ)^j · E_t [ MC_{t+j} ] )  (log-lin)
P_t   =  θ · P_{t-1}  +  (1 − θ) · P_t*                              (aggregate)
```

where `μ` is the desired markup (>1 under monopolistic competition), `β` is the household's discount factor, and `θ ∈ (0, 1)` is the Calvo non-reset probability per period (a higher `θ` means stickier prices). **Source:** Romer (2019) Ch.7 pp.290-330.

The **NK Phillips curve** is the log-linearized aggregate dynamic for inflation: **Source:** Romer (2019) pp.238-367.

```
π_t  =  β · E_t[π_{t+1}]  +  κ · ỹ_t        [NK Phillips curve]
        where  κ = (1 − θ)(1 − β θ) / θ · slope
              ỹ_t = output gap (Y_t − Y_t^flex) / Y_t^flex
```

The slope `κ` is increasing in `(1 − θ)` (less sticky → steeper Phillips curve) and depends on the elasticity of marginal cost to the output gap. The crucial NK feature is that inflation is **forward-looking** (depends on expected future inflation `E_t π_{t+1}`), in contrast to the backward-looking (lagged-inflation) accelerationist Phillips curve. **Source:** Romer (2019) Ch.7 pp.330-355.

The **dynamic IS curve** is the linearized Euler equation in log-deviation form: **Source:** Romer (2019) pp.238-367.

```
ỹ_t  =  E_t[ỹ_{t+1}]  −  (1/σ) · ( i_t − E_t[π_{t+1}] − r_t^n )    [IS]
        where σ = inverse intertemporal elasticity of substitution
              r_t^n = natural real rate
```

The output gap today depends on the expected output gap tomorrow plus the gap between the real interest rate `i_t − E_t π_{t+1}` and the natural real rate `r_t^n`. Combined with a Taylor monetary-policy rule `i_t = ρ + φ_π · π_t + φ_y · ỹ_t`, these three equations form the standard NK DSGE system. The Taylor-rule details sit in sibling [`ec-monetary-policy-and-inflation`](./ec-monetary-policy-and-inflation.md). **Source:** Romer (2019) Ch.7 pp.355-367.

## Mathematical Reasoning

The NK Phillips curve derives from log-linearizing the Calvo aggregate-price-index identity around the steady state. The Calvo identity is `P_t^(1−ε) = θ · P_{t-1}^(1−ε) + (1 − θ) · P_t*^(1−ε)` (constant-elasticity-of-substitution aggregator with elasticity `ε`); log-linearizing gives `p_t = θ · p_{t-1} + (1 − θ) · p_t*` (in log deviations). Combining with the optimal reset condition `p_t* = (1 − βθ) · ∑ (βθ)^j · E_t mc_{t+j}` and rearranging yields the forward-looking inflation dynamic `π_t = β · E_t π_{t+1} + κ · mc_t`. Substituting marginal cost as a function of the output gap (`mc_t ∝ ỹ_t` under standard assumptions on labor markets) gives the NK Phillips curve in output-gap form. **Source:** Romer (2019) Ch.7 pp.330-355.

The **solution to the three-equation NK system** under a Taylor rule `i_t = ρ + φ_π · π_t + φ_y · ỹ_t` (with `φ_π > 1` — the Taylor principle — ensuring uniqueness of the rational-expectations equilibrium) is a state-space DSGE that produces analytic impulse responses to monetary shocks, productivity shocks, and demand shocks. Under a monetary tightening (transitory rise in `i_t`), the dynamic IS curve says the output gap falls, the Phillips curve says inflation falls, and the policy rule then validates the initial rate rise. The persistence of the response depends on `θ` (price stickiness) and the Taylor-rule coefficients. **Source:** Romer (2019) Ch.7 pp.355-367.

The **monetary-policy implications** of NK are dramatic: under flexible prices, monetary policy is neutral (changing the money supply changes nominal variables proportionally but leaves real variables unchanged). Under Calvo sticky prices, monetary policy has temporary real effects — a monetary expansion raises real output above its natural level for several quarters, until enough firms have reset their prices to absorb the higher nominal demand into inflation rather than output. The cumulative output response to a permanent money-supply shock eventually returns to zero (long-run neutrality), but the transitional response can be substantial. This is the modern micro-foundation for "monetary policy matters in the short run, prices are flexible in the long run" — the Friedman-monetarist intuition rewritten in DSGE form. **Source:** Romer (2019) Ch.7 pp.355-367.

## See Also

- [`ec-real-business-cycle-theory`](./ec-real-business-cycle-theory.md) — RBC flexible-price baseline that NK extends with sticky prices
- [`ec-monetary-policy-and-inflation`](./ec-monetary-policy-and-inflation.md) — concrete Taylor rule + ZLB + inflation-targeting institutional framework
- [`ec-monetary-fiscal-policy-mechanics-l1`](./ec-monetary-fiscal-policy-mechanics-l1.md) — CFA L1 mechanical policy-transmission framework

## Escalate to Raw When

The full derivation of the Calvo Phillips curve including the cross-sectional price-dispersion welfare cost is in Romer Ch.7 pp.290-330. Alternative sticky-price mechanisms (menu costs, sticky information à la Mankiw-Reis, Taylor staggered-contract pricing) are in Romer Ch.6 pp.238-290. The treatment of zero-lower-bound monetary policy, forward guidance, quantitative easing, and the Eggertsson-Krugman liquidity-trap analysis is in Romer Ch.12 (treated in sibling `ec-monetary-policy-and-inflation`). Advanced topics — heterogeneous-agent NK (HANK), financial frictions, NK with capital accumulation — are graduate research frontiers out of v10 scope. **Source:** Romer (2019) pp.238-367.
