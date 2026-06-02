---
schema_version: "cacg.v0"
id: "ec-investment-cost-of-capital"
title: "Investment with Adjustment Costs and Tobin's q"
reading_id: "02_economics"
summary: "Romer Ch.9 q-theory of investment: static firm-profit-max extended to dynamic neoclassical investment with quadratic adjustment costs; Tobin's q = 1 at steady state; user cost of capital P_K(r + δ − π̇_K/P_K) as the threshold rental rate."
tags: ["economics", "investment-cost"]
citations:
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p449:0641"
    chunk_hash: "39981ba557efb96e15a9e8ad5f51921e368e0caecddf8b29d001b2e13265c413"
    page_range: [449, 449]
    quote: "Our analysis implies that what is relevant to investment is marginal q the ratio of the market value of a marginal unit of capital to its replacement cost."
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p439:0627"
    chunk_hash: "aaa27b1d2ff3e514aef2d38a0d415851c2f0c3e2bc62eadfcc0f91c3ae10fdc2"
    page_range: [439, 440]
    quote: "In Section 9.8, we will consider adjustment costs that take more complicated forms than the smooth adjustment costs of q theory."
    edge_type: "defines"
card_hash: "a0d5082b5699e79e7d0dd298c9f46d2cf13580bfe3d92b2c2737b41b426fee34"
---
# Investment with Adjustment Costs and Tobin's q

## Intuition

The static firm-profit-maximization condition `r = MPK − δ` (sibling `ec-rental-rate-of-capital-microeconomic-foundation`) assumes capital is frictionlessly adjustable to its desired level each period. The dynamic neoclassical investment theory (Romer Ch.9) introduces **adjustment costs** — the firm pays a quadratic cost to install new capital quickly, so it cannot jump immediately to the desired level. The optimal investment rate equates the marginal value of an additional unit of installed capital (the shadow price `q`) to the marginal cost of installing it (1 unit of consumption plus the marginal adjustment cost). The condition `q = 1` at the long-run equilibrium pins down the steady-state capital stock. **Source:** Romer (2019) Ch.9 pp.420-440.

```
   Tobin's q theory of investment

   q (marginal value of installed
   capital per unit replacement cost)
   ^
   |    q > 1 → invest (build new capital)
   |       firm pays adjustment cost
   |       to install K faster
   |
   | --- q = 1 (long-run steady state) ---
   |       investment rate matches
   |       depreciation; K constant
   |
   |    q < 1 → disinvest (let K depreciate)
   |       firm cannot get back the
   |       installation cost
   +---------------------------------------> time

   short-run: q jumps with news about future profits
   long-run: q → 1; K adjusts gradually due to adjustment costs
```

The **Tobin's q** framework is the modern foundation for empirical investment regressions: the firm's investment-to-capital ratio should depend on `q` (the ratio of market value of installed capital to its replacement cost), not on current cash flow. The empirical literature finds that `q` alone explains a modest share of investment variation, while cash flow has substantial residual explanatory power — interpreted variously as financing constraints (firms with poor capital-market access invest only when internally funded), measurement error in `q`, or behavioral biases. **Source:** Romer (2019) Ch.9 pp.440-459.

## Definition

The neoclassical firm with adjustment costs maximizes the present value of profits net of investment and adjustment costs: **Source:** Romer (2019) pp.420-459.

```
max  ∫_0^∞  e^(−r·t) · [ π(K_t, ·) − I_t − C(I_t, K_t) ]  dt
s.t.  K̇_t  =  I_t  −  δ · K_t                  [capital accumulation]
      K_0 given
```

where `π(K, ·)` is the operating profit per unit time, `I` is gross investment per unit time, `δ` is depreciation, and `C(I, K)` is the adjustment-cost function (typically `C(I, K) = (φ/2) · (I/K)^2 · K` — quadratic in the investment-to-capital ratio). **Source:** Romer (2019) Ch.9 pp.420-440.

The **first-order condition for investment** (the Hamiltonian costate condition) gives: **Source:** Romer (2019) pp.420-459.

```
1  +  ∂C/∂I  =  q                                  [MB = MC]
q̇  =  (r + δ) · q  −  ∂π/∂K  −  ∂C/∂K              [Bellman dynamic for q]
```

where `q` is the shadow value of installed capital (the costate variable for capital in the Hamiltonian). The first equation says the firm invests until the marginal value of an installed unit (`q`) equals the marginal cost (one unit of foregone consumption plus the marginal adjustment cost). The dynamic equation for `q` says the shadow price grows at the discount rate `(r + δ)` minus the marginal profit from holding capital plus marginal adjustment-cost savings. **Source:** Romer (2019) Ch.9 pp.440-459.

The **user cost of capital** is the steady-state rental rate at which firms hold capital: **Source:** Romer (2019) pp.420-459.

```
user_cost  =  P_K · (r + δ − π̇_K / P_K)
```

where `P_K` is the price of capital goods, `r` is the real interest rate, `δ` is depreciation, and `π̇_K / P_K` is the rate of capital-price appreciation (the capital gain). The firm holds capital up to the point where `MPK = user_cost / P_Y` (real user cost). Tax-corrected user cost (adding investment tax credits, depreciation allowances, corporate-income-tax wedge) is the standard input to public-finance analyses of investment incentives. **Source:** Romer (2019) Ch.9 pp.440-459.

## Mathematical Reasoning

The **average-vs-marginal q distinction** is the bridge from theory to empirics. **Marginal q** (the FOC variable) is the shadow value of an additional unit of installed capital. **Average q** (Tobin's q) is the ratio of the firm's market value to its replacement-cost book value: **Source:** Romer (2019) pp.420-459.

```
average_q  =  market_value / replacement_cost_of_capital
```

Under constant returns to scale and perfect competition, Hayashi's (1982) theorem proves `average q = marginal q`, so the empirically-observable Tobin's q is informative about the FOC variable. Under decreasing returns or market power, average q exceeds marginal q (the firm's market value reflects rents that are not marginal investment incentives). This is why empirical Tobin's-q investment regressions sometimes underperform: average q is observed, but marginal q is what the FOC requires. **Source:** Romer (2019) Ch.9 pp.440-459.

The **adjustment-cost dynamic** explains why investment is gradual rather than discontinuous. With `C(I, K) = (φ/2) · (I/K)^2 · K`, the FOC `1 + φ · (I/K) = q` solves for `I/K = (q − 1)/φ`. The investment rate is linear in `q`, with slope `1/φ`. A high `φ` (large adjustment costs) means investment responds slowly to `q` deviations from one; a low `φ` (small adjustment costs) means investment is highly responsive and the model collapses to the static `MPK = r + δ` condition in the limit `φ → 0`. The empirical magnitudes of `φ` imply that adjustment is moderately gradual — investment-rate impulse responses to demand shocks show hump-shaped dynamics over multiple quarters. **Source:** Romer (2019) Ch.9 pp.440-459.

The **cost-of-capital mapping to corporate finance**: the user-cost-of-capital framework is the production-side foundation for the WACC concept in 05 Equity (`eq-equity-cost-of-capital-estimation`, `eq-implied-cost-of-capital-foundations`). The 02 framing emphasizes the production-theoretic interpretation (rental rate of capital from the firm's intertemporal optimization); the 05 framing emphasizes the financing-side composition (weighted-average of debt and equity costs). The two frameworks are duals — the rental rate `r + δ − π̇_K` equals the required return on capital from the investor side via the no-arbitrage condition. Per the v10 plan's resolved DEC-4 BOUNDARY-DISCIPLINE, this card stays on the production side and cross-links via `Repo touchpoints:` without re-deriving WACC. **Source:** Romer (2019) Ch.9 pp.440-459.

## See Also

- [`ec-firm-profit-maximization`](./ec-firm-profit-maximization.md) — static firm profit-max that this dynamic framework extends with adjustment costs
- [`ec-rental-rate-of-capital-microeconomic-foundation`](./ec-rental-rate-of-capital-microeconomic-foundation.md) — production-theory anchor for the rental rate `r = MPK − δ` (no-adjustment-costs baseline)
- [`ec-ramsey-cass-koopmans-savings`](./ec-ramsey-cass-koopmans-savings.md) — household savings framework that supplies the discount rate `r` used in the firm's intertemporal optimization

## Escalate to Raw When

The full derivation of the Hamiltonian costate equation and the saddle-path-stability analysis of the `(K, q)` dynamic in phase-space is in Romer Ch.9 pp.440-459. The Hayashi (1982) theorem on average-vs-marginal q equivalence under CRS+perfect-competition is in Romer Ch.9 §9.5 pp.450-459. The integration of investment with financing frictions (the Bernanke-Gertler financial accelerator, the Kiyotaki-Moore credit-cycle model) sits in Romer Ch.10 (Financial Markets and Investment) and broader graduate-research literature out of both this card's primary span (Ch.9 only) and v10 scope. The closest v10 cross-link for the financing-side perspective is the WACC framework in 05 (`eq-equity-cost-of-capital-estimation`). **Source:** Romer (2019) pp.420-459.
