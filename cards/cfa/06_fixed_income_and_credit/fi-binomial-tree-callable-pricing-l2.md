---
schema_version: "cacg.v0"
id: "fi-binomial-tree-callable-pricing-l2"
title: "Binomial-Tree Callable Bond Pricing (L2)"
reading_id: "06_fixed_income_and_credit"
summary: "Arbitrage-free binomial-tree machinery for callable/putable/OAS bond pricing: short-rate dynamics on a recombining tree, backward induction with optionality at each node, and the L2 binomial alternative to closed-form continuous-time short-rate models."
tags: ["fixed-income", "binomial-tree"]
citations:
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p230:0288"
    chunk_hash: "924e115441311d91eb5a05e9b48c59d12090e8a08f8af32c99ab7fc96e3c8a0b"
    page_range: [230, 231]
    quote: "Trees in which the up-down and down-up states have the same value are called recombining trees."
    edge_type: "defines"
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p506:0651"
    chunk_hash: "abe60557ea45b385b9bf005625c58906cd1e5d8fb0f31f3ca9e478b5f33c19e9"
    page_range: [506, 507]
    quote: "This section has described swaptions as if they are physically settled, meaning that, at expiration, the counterparties enter into a swap at the appropriate rate and maturity."
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p689:1027"
    chunk_hash: "7b14e25547c278f0a93d14c98b5a052fa1d0fb3f34179fb7016116195d52cc4c"
    page_range: [689, 690]
    quote: "European Bond Options Many over-the-counter bond options and some embedded bond options are European."
    edge_type: "supports"
card_hash: "853e9c0b7277f8dc65931057865658fe17f0fa557b87137e7f1a2c722b6e844e"
---
# Binomial-Tree Callable Bond Pricing (L2)

## Intuition

A callable bond gives the issuer the right to redeem the bond before maturity at a pre-specified call price. The bond's value to the holder is therefore the value of an otherwise-equivalent non-callable bond minus the value of the issuer's call option. Closed-form pricing of this call option requires modeling the short-rate dynamics; for practitioner depth at the CFA L2 level, the **arbitrage-free binomial tree** is the canonical framework: a discrete-time, recombining lattice on which the short rate evolves under risk-neutral probabilities, and the bond price is computed by backward induction with the call decision applied at each callable node. **Source:** Tuckman & Serrat 3e (2011) Ch.7-9 pp.207-273.

```
recombining binomial tree (3 periods shown)
   t_0                  t_1                  t_2                  t_3
                                                                  r_uuu
                                            r_uu
                                                                  r_uud
                       r_u
                                            r_ud
                                                                  r_udd
   r_0
                                            r_du (= r_ud, by recombination)
                                                                  r_ddd
                       r_d
                                            r_dd

   risk-neutral up-prob q at each node (typically q = 1/2 in the
   Black-Derman-Toy and Ho-Lee branches; q varies by node in more
   general one-factor models).

   backward induction for a callable bond with call schedule K_t:
       at maturity T: V_T = face + final coupon
       at each interior node and time step:
           NoCallValue_t = coupon_t
              + D · [q · V_{t+1}(up) + (1−q) · V_{t+1}(down)]
           V_t(node) = min{ K_t, NoCallValue_t }
              for t in the call schedule (issuer calls when
              NoCallValue_t > K_t; outside the call schedule the
              constraint is inactive and V_t = NoCallValue_t).
   the tree's calibration: short-rate volatilities chosen so the
   risk-neutral price of every zero-coupon bond at each maturity
   matches the observed spot-rate curve (no-arbitrage condition).
```

## Definition

A **recombining binomial tree** is a lattice on which an up move followed by a down move arrives at the same node as a down move followed by an up move. Recombination keeps the tree's node count linear (rather than exponential) in the number of time steps and is the standard discretization in Tuckman's L2 framework. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

The **risk-neutral probability** `q` at each node is the up-move probability under which the tree's discount-and-expectation reproduces the observed prices of zero-coupon bonds at each maturity. In the simplest models (Ho-Lee, Black-Derman-Toy under standard parametrizations) `q = 1/2` and the tree's calibration is achieved entirely through the short-rate path. **Source:** Tuckman & Serrat 3e (2011) Ch.8 pp.229-249.

The **backward-induction algorithm** computes the bond price recursively: at maturity, the node value is the final cash flow; at each interior node, the value is the coupon plus the discounted risk-neutral expectation of the next-step values; at callable nodes, the issuer's optimal exercise is applied (the node value is the min of the no-call value and the call price `K_t`). The price at the root node is the bond's no-arbitrage value. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

The **arbitrage-free condition** is that the tree, calibrated to the observed term structure, prices every traded zero-coupon bond at its observed market price. This is what distinguishes Tuckman's L2 framework from a simple Black-Scholes-style binomial tree on rates: the tree is calibrated to the cross-section of zero-coupon bond prices rather than fit to a single bond's volatility. **Source:** Tuckman & Serrat 3e (2011) Ch.9 pp.251-273.

The **option-adjusted spread (OAS)** is the constant spread added to the short-rate tree at every node that equates the tree-priced value of the bond (including optionality) to the observed market price. OAS is the practitioner's measure of credit-and-liquidity premium net of the embedded option's value. **Source:** Tuckman & Serrat 3e (2011) Ch.18 pp.483-525.

## Mathematical Reasoning

The bond's payoff at a callable node is `min{ K_t, NoCallValue_t }` where `NoCallValue_t = coupon_t + D · E_q[V_{t+1}]`. The min arises because the issuer optimally calls (pays `K_t` to redeem) when the no-call value exceeds the call price. Equivalently, the value of the embedded call option from the holder's perspective is the present value of the holder's foregone upside above the call price. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

The bond's price decomposes as `CallableBondPrice = NonCallableBondPrice − CallOptionValue` where the call option value is computed implicitly via the difference of the two backward inductions (one with call decisions, one without). For a putable bond, the decomposition is `PutableBondPrice = NonPutableBondPrice + PutOptionValue` where the holder optimally puts at a put-protection node when `V_t < P_t`. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

The tree's calibration to the term structure is a one-factor analogue of the broader arbitrage-free framework developed in [`fi-arbitrage-free-valuation-l2.md`](./fi-arbitrage-free-valuation-l2.md#mathematical-reasoning). The tree replicates the observed zero-coupon bond prices through a sequence of period-by-period volatility specifications; this is the discrete analogue of the continuous-time short-rate models in [`fi-short-rate-models.md`](./fi-short-rate-models.md#mathematical-reasoning) (Vasicek, CIR, Hull-White). **Source:** Tuckman & Serrat 3e (2011) Ch.9 pp.251-273.

The connection to the L1 callable / putable intuition from [`fi-callable-and-putable-bonds.md`](./fi-callable-and-putable-bonds.md#mathematical-reasoning) is direct: the L1 framework asserts that a callable bond's yield exceeds an otherwise-equivalent non-callable bond's yield by the value of the embedded call option per unit duration. The L2 binomial-tree framework computes the option value quantitatively rather than asserting it; this is the depth shift Tuckman Ch.7-10 adds on top of the L1 intuition. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

The OAS framework from [`fi-oas-and-effective-duration.md`](./fi-oas-and-effective-duration.md#mathematical-reasoning) uses the binomial-tree machinery as its computational core: an OAS-implied spread is the residual that, after stripping the option value via the tree, explains the bond's market price. The L2 binomial-tree depth therefore strengthens OAS's interpretation by replacing the L1's qualitative "OAS is the model-stripped credit / liquidity spread" with a quantitative tree-based stripping. **Source:** Tuckman & Serrat 3e (2011) Ch.18 pp.483-525.

The continuous-time equivalent from Hull §29 (binomial-tree-on-rates extending the equity-options binomial) develops the same machinery in a continuous limit; Tuckman's discrete-time framework is the practitioner-friendly alternative that converges to the continuous-time limit as the tree's time step shrinks. **Source:** Hull §29 pp.690-720.

## See Also

- [`fi-callable-and-putable-bonds.md`](fi-callable-and-putable-bonds.md) — L1 callable / putable intuition the binomial-tree depth quantifies
- [`fi-oas-and-effective-duration.md`](fi-oas-and-effective-duration.md) — OAS as the tree-stripped credit-and-liquidity residual
- [`fi-short-rate-models.md`](fi-short-rate-models.md) — continuous-time analogue of the binomial-tree calibration
- [`fi-arbitrage-free-valuation-l2.md`](fi-arbitrage-free-valuation-l2.md) — broader L2 arbitrage-free valuation framework the binomial tree implements

## Escalate to Raw When

Open Tuckman & Serrat 3e Ch.7-10 (Term Structure Models) and Ch.18
(Fixed Income Options) directly when any of the criteria below
applies. **Source:** Tuckman & Serrat 3e (2011)
Ch.7-10 pp.207-285; Ch.18 pp.483-525.

- The card user needs the explicit tree-construction recipe for
  a specific one-factor short-rate model (Ho-Lee, Black-Derman-
  Toy, Hull-White) including its calibration to a dated term
  structure.
  **Source:** Tuckman & Serrat 3e (2011) Ch.9-10 pp.251-285.
- A specific callable-bond pricing exercise with the dated call
  schedule and a calibrated tree is required at desk-level
  numerical precision.
  **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.
- Multi-factor binomial / trinomial trees on rates (two-factor
  HJM-style discretizations) are required — out of this card's
  one-factor framing; route to a future specialty plan.
  **Source:** Hull §29 pp.690-720.
- The card user needs the Bermudan-callable variant (call
  exercisable on multiple discrete dates, not the full
  continuous-call schedule) — Tuckman provides the framework
  but the dated implementation requires the chapter's worked
  example.
  **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.
