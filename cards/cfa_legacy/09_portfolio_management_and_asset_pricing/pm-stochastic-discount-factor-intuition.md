---
schema_version: "cacg.v0"
id: "pm-stochastic-discount-factor-intuition"
title: "Stochastic Discount Factor — Intuition"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Stochastic Discount Factor — Intuition: framing the stochastic discount factor (SDF) as the unifying object of asset pricing — what role it plays, how it produces every linear factor pricing model as a special case, and why its existence follows from the law of one price"
tags: ["portfolio-management", "stochastic-discount-factor", "asset-pricing"]
citations:
  - source_id: "pm_cochrane_2005_asset_pricing_revised"
    chunk_id: "pm_cochrane_2005_asset_pricing_revised:p026:0030"
    chunk_hash: "a1d4dc94a457f2c7bda430e8b42d7ace91978bab74c27717d6ac90fdcc61e053"
    page_range: [26, 27]
    quote: "All asset pricing models amount to alternative ways of connecting the stochastic discount factor to data."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3821:5793"
    chunk_hash: "1a3b2797bfa0ffc817eec8a05bfcfe57b6c1f913c466dc9aaa7bd90b43e5a295"
    page_range: [3821, 3822]
    quote: "The condition in a financial market in which two equivalent financial instruments or combinations of financial instruments can sell for only one price."
    edge_type: "supports"
card_hash: "1e21fb04ea9dd5f1c248a370f601345484a2071132c29277b77d2ca9bcb217a5"
---
# Stochastic Discount Factor — Intuition

## Intuition

The stochastic discount factor `m` is the single object from which
every asset price is derived under no-arbitrage. The relationship
is uniform across asset classes: equity, debt, derivatives, exotic
payoffs all satisfy the same pricing equation `price = E[m ·
payoff]` evaluated under the physical probability measure. Different
asset-pricing theories — CAPM, multifactor models, consumption-based
models — differ only in what they assume about `m`. The SDF
framework is the unifying lens; each theory becomes a specification
of `m` rather than a separate pricing structure. **Source:**
Cochrane (2005) pp.3-31.

```
        every asset prices via the SDF:
        ===============================

        price_i  =  E[ m · payoff_i ]    under physical measure

        for an excess return r_i_excess:
        0  =  E[ m · r_i_excess ]

        substituting m specifications:
        +-----------------------------+----------------------------+
        | theory                       | what m equals              |
        +-----------------------------+----------------------------+
        | consumption-based            | beta · u'(c_(t+1)) / u'(c_t)|
        | CAPM                         | a + b · R_M                 |
        | multifactor (linear)         | a + b' · f                  |
        | risk-neutral (Q-measure)     | exp(-r · dt) (deterministic)|
        +-----------------------------+----------------------------+
```

The SDF interpretation Cochrane emphasizes is that `m` measures the
investor's marginal value of payoff in each future state of the
world. A high `m` in a given state means that an extra dollar of
payoff in that state is especially valuable — typically a
consumption-poor or recession state where utility from additional
income is high. Assets that pay off in high-`m` states are valuable
(they hedge bad states); assets that pay off in low-`m` states are
cheap (they pay when payoffs are least needed). The expected-return
premium of an asset reflects the negative covariance of its payoff
with `m`. **Source:** Cochrane (2005) pp.3-31.

## Definition

The basic pricing equation expresses the price of asset `i` today
as the expected product of the SDF `m` and the asset's payoff at
the next period. **Source:** Cochrane (2005) pp.3-31.

```
p_i(time)  =  E[ m(time+1) · x_i(time+1) | I(time) ]
```

For a return — payoff per dollar invested — the equation reads as
the unconditional moment condition. **Source:** Cochrane (2005)
pp.3-31.

```
1  =  E[ m · R_i ]      for a gross return R_i
0  =  E[ m · r_i ]      for an excess return r_i = R_i - Rf
```

The consumption-based motivation produces an explicit form for
`m` from the investor's first-order condition: at the optimum,
the marginal cost of an extra unit of investment today equals the
expected discounted marginal benefit at the next period. **Source:**
Cochrane (2005) pp.3-31.

```
m(time+1)  =  beta · u'(c(time+1)) / u'(c(time))
```

Here `beta` is the time-preference discount factor, `u(c)` is the
investor's utility-of-consumption function, and the fraction is
the marginal rate of substitution (MRS) between consumption tomorrow
and consumption today. The SDF is the MRS scaled by the time-
preference factor. Different asset-pricing theories specify
different proxies for this MRS. **Source:** Cochrane (2005)
pp.3-31.

## Mathematical Reasoning

The expected-return implication of the basic pricing equation is
the covariance decomposition. For an excess return `r_i_excess`,
the condition `E[m · r_i_excess] = 0` rearranges. **Source:**
Cochrane (2005) pp.3-31.

```
E[r_i_excess]  =  - cov(m, r_i_excess) / E[m]
              =  - (Rf) · cov(m, r_i_excess)             when E[m] = 1/Rf
```

Assets whose excess returns covary negatively with `m` (positive
in good states where `m` is low, negative in bad states where `m`
is high) earn positive expected excess return — the investor must
be paid to hold risk concentrated in bad states. Assets whose
excess returns covary positively with `m` (hedging assets) earn
negative expected excess return — the investor pays to hold a
state-claim that delivers in bad times. The expected-return
premium is the negative covariance with the SDF, scaled. **Source:**
Cochrane (2005) pp.3-31.

The existence of an SDF that prices all assets follows from the
law of one price under positive prices. Cochrane Ch.4 establishes
the existence theorem: if any portfolio strategy that has zero
payoff in every state must have zero price (the law of one price),
then there exists a strictly positive `m` such that every asset's
price equals the expected `m`-discounted payoff. **Source:**
Cochrane (2005) pp.61-75.

```
law of one price + no arbitrage  ==>  there exists m > 0 such that
                                       p_i = E[m · x_i]  for every asset i
```

The existence theorem is constructive in finite-state markets:
`m` can be computed as the projection of any candidate discount
factor onto the payoff space, and the resulting projection prices
all traded assets. In incomplete markets the existence is unique
only up to non-traded directions. The L1 reader does not need
the proof; the qualitative claim that "an SDF exists under
no-arbitrage" is the carry-out. **Source:** Cochrane (2005)
pp.61-75.

The CAPM as a special case follows by substituting a linear `m =
a + b · R_M` into the basic pricing equation and solving for the
expected-return-beta form. The substitution produces the standard
CAPM expression with the market portfolio as the single factor.
**Source:** Cochrane (2005) pp.3-31.

```
m = a + b · R_M
0 = E[m · r_i_excess]
   = a · E[r_i_excess] + b · E[R_M · r_i_excess]
   = a · E[r_i_excess] + b · cov(R_M, r_i_excess)
                       (since E[R_M · Rf-component] cancels)

  ==>  E[r_i_excess]  proportional to  cov(R_M, r_i_excess)
                      = beta_i · var(R_M)

  ==>  E[r_i_excess] = beta_i · (E[R_M] - Rf)
```

The same substitution machinery produces APT, ICAPM, and Fama-
French models when `m` is set to the appropriate linear function
of those factors. The SDF representation absorbs all of these
under the same pricing equation. **Source:** Cochrane (2005)
pp.61-75.

A specific implication for the L1 framing: CAPM-violating anomalies
are equivalent to a misspecified `m`. If `m_CAPM = a + b · R_M`
fails to price small-cap value stocks correctly, an alternative
`m_FF3 = a' + b1 · R_M + b2 · F_size + b3 · F_value` may price
them. The SDF lens lets the analyst diagnose anomalies as
"missing factors in the SDF specification" rather than as
unmodelable mysteries. **Source:** Cochrane (2005) pp.3-31.

## See Also

- [`pm-multifactor-asset-pricing-intuition.md`](pm-multifactor-asset-pricing-intuition.md) — beta-representation form that is dual to the SDF representation under linear-`m` specifications
- [`pm-capm-and-sml.md`](pm-capm-and-sml.md) — CAPM as the linear-`m` special case under the consumption-equals-market-wealth assumption
- [`pm-factor-models-intuition.md`](pm-factor-models-intuition.md) — L1-core multifactor card; the SDF framework justifies why factor exposures price assets

## Escalate to Raw When

Open Cochrane (2005) Ch.1 / Ch.4 directly when any of the
criteria below applies. **Source:** Cochrane (2005) pp.3-75.

- Continuous-time SDF and Itô-process pricing — Cochrane §1.5 and
  §4.3 develop the continuous-time analog of the basic pricing
  equation. **Source:** Cochrane (2005) pp.25-31, 72-75.
- The risk-neutral probability change-of-measure (Q-measure) and
  its connection to the physical-measure SDF — Cochrane Ch.3
  develops it in finite-state markets; this content overlaps
  derivative pricing in subcorpus 07. **Source:** Cochrane (2005)
  pp.49-58.
- The Hansen-Jagannathan bound on `m`'s standard deviation —
  Cochrane Ch.5 §5.6 develops the implied lower bound on
  `var(m)` and its empirical interpretation as the equity-premium
  puzzle. **Source:** Cochrane (2005) pp.92-97.
