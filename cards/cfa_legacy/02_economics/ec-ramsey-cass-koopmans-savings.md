---
schema_version: "cacg.v0"
id: "ec-ramsey-cass-koopmans-savings"
title: "Ramsey-Cass-Koopmans Optimal Savings"
reading_id: "02_economics"
summary: "Ramsey-Cass-Koopmans replaces Solow's exogenous saving rate with an infinitely-lived household maximizing discounted CRRA utility; the consumption Euler equation c-dot/c = (r-rho)/theta says consumption rises if the real return exceeds the discount rate, and the steady-state interest rate is pinned by the modified golden rule r* = rho + theta*g."
tags: ["economics", "ramsey-cass"]
citations:
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p068:0085"
    chunk_hash: "81cf13dfacd59322cb0ec9d27720958e354df9d24f7157584f646af82ccabf5a"
    page_range: [68, 69]
    quote: "This model, which was developed by Ramsey (1928), Cass (1965), and Koopmans (1965), avoids all market imperfections and all issues raised by heteroge"
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p076:0096"
    chunk_hash: "8599d9d0b012124a07b11073bcb3b1c14b2f863d206230d53d2f7303eb423145"
    page_range: [76, 77]
    quote: "This condition states that consumption per worker is rising if the real return exceeds the rate at which the house"
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p204:0334"
    chunk_hash: "403ed71dcf99fc1214395365f16e6306d8f16bf4d81b5746b98b598fed869071"
    page_range: [204, 205]
    quote: "A decision maker is a risk averter (or exhibits risk aversion) if for any lottery F( ·),the degenerate lottery that yields the amount J x dF(x) with certainty is at least as good as the lottery F"
    edge_type: "supports"
card_hash: "e7ca8495a25ce7722a72ae9733aa510084a2e2254e0dfb37ca9fce582f2b7f44"
---
# Ramsey-Cass-Koopmans Optimal Savings

## Intuition

The Ramsey-Cass-Koopmans model replaces Solow's exogenous saving rate `s` with an infinitely-lived representative household that optimizes intertemporal consumption. The household values utility flow `u(c_t)` over an infinite horizon, discounted at the rate of time preference `ρ`. Optimal consumption follows the **Euler equation**: the percentage growth of consumption `c̊/c` equals the difference between the return on saving (`r`) and the discount rate (`ρ`), scaled by the inverse intertemporal elasticity of substitution `1/θ`. The steady-state interest rate `r*` is pinned down by the **modified golden rule** `r* = ρ + θ · g`, where `g` is the technological-progress rate. **Source:** Romer (2019) Ch.2 pp.50-75.

```
   consumption path c(t) under different (r, ρ) regimes

   c
   ^
   |
   |                                    .   r > ρ: c grows (saver)
   |                                .       Euler: c̊/c = (r-ρ)/θ > 0
   |                            .
   |                        .
   |                    .
   |   r = ρ (Ramsey BGP)
   |   ------- c flat ----------                horizontal: r = ρ
   |
   |       .   r < ρ: c declines
   |           (impatient consumer dissaves)
   |
   +-----------------------------------------------> t

   steady state: r* = ρ + θ · g  (modified golden rule)
       g = exogenous tech-progress rate
       θ = inverse intertemporal elasticity of substitution
```

The Ramsey model **avoids dynamic inefficiency** that the Solow model can exhibit: in Solow, exogenous `s > s_GR = α` leads to over-saving, but in Ramsey the household optimally chooses `s` to satisfy the modified golden rule, ensuring efficiency. The Ramsey steady state has a lower capital level than Solow's golden-rule level (because optimal savings discount future utility at rate `ρ > 0`); this is the standard Ramsey-Solow distinction. **Source:** Romer (2019) Ch.2 pp.65-75.

## Definition

The Ramsey-Cass-Koopmans planner solves the infinite-horizon optimization problem: **Source:** Romer (2019) pp.50-75.

```
max  ∫_0^∞  e^(−ρt) · u(c_t) · L_t  dt
s.t.  k̃̇ = f(k̃) − c̃ − (n + g + δ) · k̃        [capital accumulation]
      k̃(0) given;  k̃ ≥ 0, lim_{t→∞} k̃(t) ≥ 0    [feasibility]
```

where `u(c) = c^(1−θ) / (1−θ)` is the CRRA Bernoulli utility per worker (with `θ > 0` the inverse intertemporal elasticity of substitution; `θ = 1` gives `u = log c`), `ρ > 0` is the rate of time preference, and the variables are in per-effective-worker units. **Source:** Romer (2019) Ch.2 pp.55-65.

The **first-order necessary condition** (the consumption Euler equation in per-capita form, with technology progress `g`): **Source:** Romer (2019) pp.50-75.

```
c̃̇ / c̃  =  (1/θ) · (r − ρ − θ · g)        where r = f'(k̃) − δ
```

Equivalently in per-capita terms (no normalization for `A`): **Source:** Romer (2019) pp.50-75.

```
c̊ / c  =  (1/θ) · (r − ρ)        (without growth rescaling)
```

The **modified golden rule** characterizes the steady state where consumption growth equals technology growth (`c̃̇ = 0` in per-effective-worker terms, hence `c̊/c = g` in per-capita terms): **Source:** Romer (2019) pp.50-75.

```
r*  =  ρ + θ · g
f'(k̃*) − δ  =  ρ + θ · g        (steady-state k chosen so MPK − δ matches)
```

**Source:** Romer (2019) Ch.2 pp.65-75.

The **transversality condition** rules out the household accumulating capital forever without consuming it: **Source:** Romer (2019) pp.50-75.

```
lim_{t→∞}  e^(−ρ t) · u'(c_t) · k_t  =  0
```

**Source:** Romer (2019) Ch.2 pp.65-70.

## Mathematical Reasoning

The Euler equation derives from the household's first-order condition for intertemporal consumption smoothing. The Lagrangian for the dynamic optimization (with `λ(t)` the costate for capital accumulation) gives the FOC `u'(c) = λ` and the costate equation `λ̇ / λ = ρ − r`. Differentiating the FOC w.r.t. time gives `u''(c) · c̊ = λ̇`, hence `c̊ / c = (−u'(c) / (c · u''(c))) · (r − ρ) = (1/θ) · (r − ρ)` for CRRA utility (where `θ = −c · u''(c) / u'(c)` is the inverse IES). The Euler equation says: when `r > ρ`, the household saves (consumption grows over time); when `r < ρ`, the household dissaves (consumption falls). The IES `1/θ` measures the household's willingness to substitute consumption across periods. **Source:** Romer (2019) Ch.2 pp.65-75.

The modified golden rule follows from setting `c̃̇ = 0` at the steady state: `(1/θ)(r* − ρ − θg) = 0`, hence `r* = ρ + θg`. With CRS Cobb-Douglas production `f(k̃) = k̃^α`, the steady-state capital is `k̃* = (α / (ρ + θg + δ))^(1/(1−α))`, which is smaller than the Solow golden-rule level `k̃_GR = (α / (n + g + δ))^(1/(1−α))` whenever `ρ + θg > n + g + δ − δ = n + g`, i.e., `ρ > n − (1 − θ) g`. Under standard parameter magnitudes (positive time preference, modest population and technology growth, IES below one), this inequality holds and Ramsey steady-state capital is below Solow's golden rule — the household's impatience makes optimal savings less than what would maximize long-run consumption. **Source:** Romer (2019) Ch.2 pp.65-75.

**Saddle-path stability**: the Ramsey model in `(k̃, c̃)` phase space exhibits saddle-path dynamics. The steady state `(k̃*, c̃*)` has one stable arm (the saddle path) and one unstable arm. The household chooses initial consumption `c̃(0)` to land on the saddle path; any other choice leads to either capital depletion (`k̃ → 0`) or violation of the transversality condition. The saddle-path solution is unique given the initial capital `k̃(0)` and the transversality condition. This contrasts with Solow, which has a globally stable steady state without saddle-path indeterminacy. **Source:** Romer (2019) Ch.2 pp.65-75.

The Ramsey framework is the canonical foundation for modern macro: Real Business Cycle models (sibling `ec-real-business-cycle-theory`), New Keynesian DSGE models (sibling `ec-nominal-rigidity-and-NK-monetary`), and consumption-based asset pricing (09 `pm-stochastic-discount-factor-intuition`) all build on the Ramsey household's Euler equation as the core intertemporal optimality condition. The Euler equation also generalizes to stochastic settings (replace `r` with `E[r]` and add risk-adjustment terms from the covariance between marginal utility and returns) — this is the bridge to the SDF asset-pricing framework in 09. **Source:** Romer (2019) Ch.2 pp.65-75.

## See Also

- [`ec-solow-growth-model`](./ec-solow-growth-model.md) — exogenous-savings baseline that Ramsey replaces with optimal savings
- [`ec-utility-and-choice-under-uncertainty`](./ec-utility-and-choice-under-uncertainty.md) — vNM expected-utility framework that the Euler equation generalizes to stochastic settings
- [`ec-permanent-income-consumption`](./ec-permanent-income-consumption.md) — Romer Ch.8 extends the Ramsey framework to consumption smoothing under income uncertainty
- [`ec-real-business-cycle-theory`](./ec-real-business-cycle-theory.md) — Romer Ch.5 uses Ramsey-style household + stochastic-productivity firms

## Escalate to Raw When

The full saddle-path-dynamics analysis (phase diagram, stable manifold characterization, transitional dynamics) is in Romer Ch.2 Part A pp.65-75. The overlapping-generations (OLG) variant where the household lives finitely and dynastic structure is broken (Diamond model) is in Romer Ch.2 Part B pp.76-98; this card covers Part A only (infinite-horizon Ramsey). The integration of Ramsey with uncertainty (stochastic Euler equation, asset-pricing implications) is in Romer Ch.8 pp.368-419 (treated in `ec-permanent-income-consumption`) and the consumption-CAPM literature (09 cross-link). **Source:** Romer (2019) pp.50-75.
