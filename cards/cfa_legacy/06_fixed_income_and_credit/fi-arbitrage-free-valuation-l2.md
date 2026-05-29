---
schema_version: "cacg.v0"
id: "fi-arbitrage-free-valuation-l2"
title: "Arbitrage-Free Valuation Framework (L2)"
reading_id: "06_fixed_income_and_credit"
summary: "Frames the arbitrage-free valuation framework for fixed-income securities at L2 practitioner depth: no-arbitrage conditions on the term-structure model, replication portfolios as the price-discovery mechanism, the discount curve as a system of constraints, and the distinction between deterministic and stochastic discounting."
tags: ["fixed-income", "arbitrage-free"]
citations:
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p231:0289"
    chunk_hash: "1ef01c19df2aa82628077a67b5c6fcb8885907b1dce7682e286496e6135001ad"
    page_range: [231, 232]
    quote: "The previous section showed that the risk-neutral probability of an up move on date 0 is .8024."
    edge_type: "defines"
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p480:0617"
    chunk_hash: "ed9cc6011f1308ea792e8ba0aebb9db76c711b280e844cdf92f2fa86f469bec2"
    page_range: [480, 481]
    quote: "SWAP TRADING WITH FINANCING The cash flows of receiving fixed on a swap are qualitatively the same as those from a financed bond purchase, with one key difference."
    edge_type: "supports"
  - source_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed"
    chunk_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed:p064:0100"
    chunk_hash: "0a6c8ca1c3630596246e2ff4cba71c2259ee1829f498b2b50be5fad3b4bdb6b3"
    page_range: [64, 65]
    quote: "In particular, we will see later on that F(t; T,S) is the expectation of L(T,S) at time t under a suitable probability measure."
    edge_type: "supports"
card_hash: "fc46203e32e6ebf123553d2ae046ec75c498880fadb284a9047db0699f721ff5"
---
# Arbitrage-Free Valuation Framework (L2)

## Intuition

The arbitrage-free valuation framework asserts that any fixed-income security must be priceable as a portfolio of zero-coupon bonds at the prevailing spot-rate curve, with no opportunity for risk-free profit. For bonds with deterministic cash flows (no embedded options, no credit risk) this reduces to the discount-and-sum recipe developed in [`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#mathematical-reasoning). For bonds with stochastic cash flows (callable, putable, MBS prepayments, floating coupons) the framework requires stochastic discounting under a risk-neutral measure such that every traded zero-coupon bond is correctly priced. The L2 practitioner depth Tuckman develops is the construction of one-factor and multi-factor short-rate models whose risk-neutral discount expectations replicate the observed spot-rate curve. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

```
arbitrage-free valuation: the constraint stack
      observed market quotes (inputs):
          spot rates s_0, s_1, ..., s_N (across tenors)
          OR equivalently, zero-coupon bond prices
              B(0, T_0), B(0, T_1), ..., B(0, T_N)
          OR equivalently, par yields y_0, y_1, ..., y_N
      arbitrage-free constraint:
          for any candidate term-structure model M with parameters θ,
          the model-implied zero-coupon bond prices must match the
          observed prices at every traded tenor:
              B_M(0, T_k; θ) = B(0, T_k)    for k = 0, ..., N
      this is a system of N+1 equations in the model's free
      parameters; the system pins down θ up to identification
      assumptions (volatility, mean-reversion speed, market-price-
      of-risk).
      pricing of contingent claims (e.g. callable bonds, MBS):
          take the calibrated model M(θ), simulate or solve the
          risk-neutral expectation of the future cash flow stream,
          discount under the model's stochastic discount factor.
          arbitrage-free because the calibrated M(θ) prices every
          traded zero-coupon bond correctly by construction.
   the framework is general; the binomial-tree implementation in
   the companion card is one concrete computational technique;
   continuous-time short-rate or HJM models are alternative
   implementations of the same arbitrage-free constraint.
```

## Definition

The **arbitrage-free condition** for a term-structure model is that the model, calibrated to the observed yield curve, prices every traded zero-coupon bond exactly. Formally: the model-implied price `B_M(0, T_k; θ)` equals the market-observed price `B(0, T_k)` at every tenor `T_k` for which a market quote exists. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

A **replication portfolio** is a portfolio of traded instruments (typically zero-coupon bonds at a discrete set of tenors) whose cash flows reproduce a target security's cash flows. The no-arbitrage condition implies the target security's price equals the replication portfolio's price. For deterministic-cash-flow bonds this is the foundational discount-and-sum identity. For stochastic-cash-flow bonds the replication is dynamic (rebalanced over time) and the no-arbitrage price is the cost of the self-financing replication strategy. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

The **risk-neutral measure** `Q` is the probability measure under which every traded security's discounted price is a martingale (the discount factor uses the short rate). Pricing under `Q` reduces the valuation problem to computing the expectation of the discounted payoff under `Q`. **Source:** Brigo+Mercurio (2006) Ch.1-2 pp.10-35.

The **stochastic discount factor (SDF)** `M_t` is the random discount applied to the future payoff such that `Price_0 = E[M_T · Payoff_T]` under the physical (real-world) measure. Equivalently `M_T = e^(−∫_0^T r_s ds) · (dQ/dP)_T` factors into the deterministic discount times the change-of-measure Radon-Nikodym derivative. **Source:** Brigo+Mercurio (2006) Ch.1-2 pp.10-35.

The **deterministic-vs-stochastic-discounting distinction**: for a bond whose cash flows do not depend on future short rates (a vanilla fixed-coupon non-callable Treasury), the discount-and-sum recipe with the observed spot rates is exact. For a bond whose cash flows depend on future short rates (callable, putable, FRN, MBS), the discount must be applied path-by-path under the risk-neutral measure; the arbitrage-free condition is the joint constraint on the path distribution and the discount weights. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

## Mathematical Reasoning

For a non-contingent bond paying cash flows `CF_1, CF_2, ..., CF_N` at known dates `t_1, ..., t_N`, the no-arbitrage price is `P = Σ_k B(0, t_k) · CF_k` where `B(0, t_k)` is the observed zero-coupon bond price at tenor `t_k`. This is the foundational arbitrage-free identity and underlies the spot-rate curve framework of [`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#mathematical-reasoning). **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

For a contingent claim with payoff `f(r_t, t)` (dependent on the short-rate path), the no-arbitrage price under the risk-neutral measure `Q` is `P = E_Q[ exp(−∫_0^T r_s ds) · f(r_T, T) ]`. The risk-neutral measure is constructed via Girsanov's theorem so that the discounted prices of traded securities are martingales. The calibration of the term-structure model to the observed spot curve pins down the model's drift function (after the volatility is specified). **Source:** Brigo+Mercurio (2006) Ch.1-2 pp.10-35.

The **two-curve discounting framework** developed in [`fi-swap-spreads-and-libor-curve.md`](./fi-swap-spreads-and-libor-curve.md#mathematical-reasoning) (Tuckman Ch.17) refines the arbitrage-free condition for the post-2008 world where collateralization changes the relevant discount rate: collateralized derivatives discount at the OIS rate (the cost of the cash collateral) while floating-coupon-projection uses the LIBOR / SOFR forwards (the rate at which the floating leg resets). The arbitrage-free condition becomes a joint constraint on both curves. **Source:** Tuckman & Serrat 3e (2011) Ch.17 pp.457-481.

The connection to the binomial-tree implementation from [`fi-binomial-tree-callable-pricing-l2.md`](./fi-binomial-tree-callable-pricing-l2.md#mathematical-reasoning) is the concrete computational expression of the arbitrage-free constraint on a one-factor lattice: the tree's calibration is precisely the discrete-time solution to `B_M(0, T_k; θ) = B(0, T_k)` at every tenor. The continuous-time analogues (Hull-White, BDT, BK) solve the same constraint in continuous time. **Source:** Tuckman & Serrat 3e (2011) Ch.7 pp.207-227.

The connection to short-rate models from [`fi-short-rate-models.md`](./fi-short-rate-models.md#mathematical-reasoning) is the model-class specification: a short-rate model (Vasicek, CIR, Hull-White) provides the parametric form for `r_t`; the arbitrage-free framework provides the calibration constraint. Together they yield a usable pricing recipe for contingent claims. **Source:** Tuckman & Serrat 3e (2011) Ch.8 pp.229-249.

A subtle but critical point: the arbitrage-free framework does NOT pin down the volatility of the short-rate process. The framework only constrains the drift (after volatility is specified) so that the term structure is matched. The volatility specification is therefore an exogenous modeling choice; different volatility specifications produce different prices for contingent claims even when both are arbitrage-free with respect to the observed term structure. This is the source of the "model risk" in fixed-income contingent-claim pricing. **Source:** Brigo+Mercurio (2006) Ch.1-2 pp.10-35.

## See Also

- [`fi-binomial-tree-callable-pricing-l2.md`](fi-binomial-tree-callable-pricing-l2.md) — discrete-time binomial-tree implementation of the arbitrage-free framework
- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — deterministic-cash-flow special case of the arbitrage-free identity
- [`fi-swap-spreads-and-libor-curve.md`](fi-swap-spreads-and-libor-curve.md) — two-curve refinement post-2008
- [`fi-short-rate-models.md`](fi-short-rate-models.md) — continuous-time short-rate model class the framework calibrates

## Escalate to Raw When

Open Tuckman & Serrat 3e Ch.7 (The Science of Term Structure
Models) and Ch.17 (Arbitrage with Financing and Two-Curve
Discounting) directly when any of the criteria below applies.
**Source:** Tuckman & Serrat 3e (2011)
Ch.7 pp.207-227; Ch.17 pp.457-481.

- The card user needs the explicit risk-neutral-measure
  construction via Girsanov's theorem for a particular
  short-rate model — escalate to Brigo+Mercurio Ch.1-2 for the
  measure-theoretic framework.
  **Source:** Brigo+Mercurio (2006) Ch.1-2 pp.10-35.
- The card user needs the calibration of a continuous-time
  short-rate model to a dated term structure at numerical
  precision — Tuckman provides the conceptual framework and
  the binomial-tree implementation; the continuous-time
  numerical calibration belongs to specialized practitioner
  libraries.
  **Source:** Tuckman & Serrat 3e (2011) Ch.9 pp.251-273.
- The card user needs the cross-currency multi-curve
  framework for collateralized derivatives — out of this
  card's single-currency framing; route to a future specialty
  plan.
  **Source:** Tuckman & Serrat 3e (2011) Ch.17 pp.457-481.
- The card user needs the model-risk decomposition for a
  specific contingent-claim trade (e.g. how a callable bond's
  price varies across short-rate model specifications at the
  same calibrated term structure) — Tuckman provides the
  conceptual framework; the empirical decomposition requires
  dated model-runs and is out of CFA L1 and L2 scope.
  **Source:** Brigo+Mercurio (2006) Ch.1-2 pp.10-35.
