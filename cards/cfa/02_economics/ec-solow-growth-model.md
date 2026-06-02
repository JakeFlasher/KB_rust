---
schema_version: "cacg.v0"
id: "ec-solow-growth-model"
title: "Solow Growth Model"
reading_id: "02_economics"
summary: "The Solow model embeds CRS production f(k) with exogenous saving s, depreciation delta, and labor/technology growth (n,g); per-effective-worker capital k-tilde evolves k-tilde-dot = s*f(k-tilde) - (n+g+delta)*k-tilde and converges to k* where saving equals break-even investment; long-run per-capita output growth is the exogenous technology rate g."
tags: ["economics", "solow-growth"]
citations:
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p027:0025"
    chunk_hash: "7d56dbc522bef8b4f8f904e7803346b982cab1c0b4fbd0ddeafece25123c55cf"
    page_range: [27, 28]
    quote: "This chapter focuses on a relatively simple, transparent model that is an excellent starting point for studying these issues, the Solow growth model."
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p034:0035"
    chunk_hash: "c40d10bce00b16be8a4aa5dc383a7e3a807cb30cabebf6ff4d37f9f7943ee7d5"
    page_range: [34, 35]
    quote: "Equation (1.19) is the key equation of the Solow model. It states that the rate of change of the capital stock per unit of effective labor is the difference between two terms."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p722:1004"
    chunk_hash: "9a7867565d6faa6d12258726469fc89f903fa11e8ab372fcd1ea4fff592e6a5c"
    page_range: [722, 723]
    quote: "Aggregate demand tells us only the relationship between the price level and the amount of output demanded at those prices."
    edge_type: "supports"
card_hash: "9ea7e39fbf0805d1be65344da5620c4d1cc9e3796804d6dc4bf1973ee618887c"
---
# Solow Growth Model

## Intuition

The Solow model is the simplest dynamic macro framework with capital accumulation. The economy saves a constant fraction `s` of output each period and invests it as new capital; the existing capital stock depreciates at rate `δ`; the labor force grows at rate `n`. Per worker, capital `k = K/L` evolves toward the steady state `k*` where saving (`s · f(k)`) just covers the break-even investment needed to replenish depreciation and equip new workers (`(n + δ) · k`). Below `k*`, capital accumulates net (`k̇ > 0`) and pushes the economy toward `k*`; above `k*`, capital depletes (`k̇ < 0`). The steady state is globally stable. **Source:** Romer (2019) Ch.1 pp.6-30.

```
<!--primitive: solow-accumulation-steady-state source: _diagram_primitives.md-->
   investment
   per worker
   ^
   |                                 (n+delta) k    <- break-even line
   |                                /
   |                               /
   |                              /
   |                             /     ___________  <- s * f(k)
   |                            /  ___/
   |                           / _/   (concave)
   |                          *_/
   |                          *   <- (k*, s f(k*))
   |                         /steady state where
   |                        / s f(k*) = (n+delta) k*
   |                       /
   |                      /
   +----------------------+--------------------------> k
                          k*

   below k*: s f(k) > (n+delta) k --> k accumulates --> moves right
   above k*: s f(k) < (n+delta) k --> k depletes    --> moves left
   k*: balanced-growth-path capital per worker
```

The Solow model's two big results are (i) **conditional convergence** — economies with similar parameters but different starting capital levels converge to the same steady state, with poorer economies growing faster on the transition path; and (ii) **the saving rate raises levels but not long-run growth** — a permanent increase in `s` raises `k*` and per-capita output level `y* = f(k*)`, but at the new higher level the long-run per-capita growth rate returns to zero (or to the exogenous technological-progress rate `g` if we extend the model to include labor-augmenting technology). **Source:** Romer (2019) Ch.1 pp.30-49.

## Definition

The Solow model with labor-augmenting technological progress is summarized by three equations on per-effective-worker variables (`k̃ = K / (A · L)`, `ỹ = Y / (A · L)`): **Source:** Romer (2019) pp.6-49.

```
production:        ỹ = f(k̃)                     [CRS, concave, f(0)=0]
capital dynamics:  k̃̇ = s · f(k̃) − (n + g + δ) · k̃
steady state:      s · f(k̃*) = (n + g + δ) · k̃*
```

where `s ∈ (0, 1)` is the saving rate, `n` is the population-growth rate, `g` is the technology-growth rate (labor-augmenting), and `δ ∈ (0, 1)` is the depreciation rate. **Source:** Romer (2019) Ch.1 pp.10-25.

For Cobb-Douglas `f(k̃) = k̃^α` (capital share `α ∈ (0, 1)`), the steady-state closed form is: **Source:** Romer (2019) pp.6-49.

```
k̃* = (s / (n + g + δ))^(1 / (1 − α))
ỹ* = (s / (n + g + δ))^(α / (1 − α))
```

The **golden-rule saving rate** `s_GR` maximizes steady-state per-capita consumption `c̃* = (1 − s) · ỹ*` and equals `α` in Cobb-Douglas: `s_GR = α`. Economies with `s < α` are **under-saving** relative to the golden rule (capital is below the golden-rule level; raising `s` raises long-run consumption); economies with `s > α` are **dynamically inefficient** (over-accumulating capital; reducing `s` raises long-run consumption — the canonical Phelps dynamic-inefficiency case). **Source:** Romer (2019) Ch.1 pp.25-35.

The **convergence speed** near the steady state is governed by the loglinearized capital dynamic `d log(k̃) / dt ≈ −λ · log(k̃ / k̃*)` with `λ = (1 − α) · (n + g + δ)`. Under standard parameter magnitudes (moderate capital share and small annual depreciation-plus-growth wedge), the implied convergence half-life to the steady state spans roughly two decades — a strikingly slow approach. **Source:** Romer (2019) Ch.1 pp.35-49.

## Mathematical Reasoning

The Solow dynamic `k̃̇ = s · f(k̃) − (n + g + δ) · k̃` is the difference between gross saving (`s · f(k̃)` — output per effective worker times saving rate, which equals investment per effective worker in closed economy) and the break-even investment needed to (i) replenish depreciated capital (`δ · k̃`), (ii) equip new workers (`n · k̃`), and (iii) maintain the same capital stock per effective worker despite technological progress that grows the effective workforce (`g · k̃`). At the steady state, gross saving exactly equals break-even investment, so `k̃` is constant in per-effective-worker terms — but the absolute capital stock `K(t) = k̃* · A(t) · L(t)` grows at rate `n + g` (population growth plus technological progress). **Source:** Romer (2019) Ch.1 pp.10-25.

**Stability**: linearizing around `k̃*`, the dynamic becomes `k̃̇ ≈ (f'(k̃*) · s − (n + g + δ)) · (k̃ − k̃*) = (s · f'(k̃*) − (n + g + δ)) · (k̃ − k̃*)`. Using the steady-state condition `s · f(k̃*) = (n + g + δ) · k̃*`, hence `s · f'(k̃*) < (n + g + δ)` whenever `f` is concave (the average product `f(k̃*)/k̃*` exceeds the marginal product `f'(k̃*)`). So the coefficient on the deviation `(k̃ − k̃*)` is negative — perturbations decay toward zero — and `k̃*` is globally asymptotically stable. **Source:** Romer (2019) Ch.1 pp.25-35.

**Steady-state policy comparative statics**: differentiating the steady-state condition `s · f(k̃*) = (n + g + δ) · k̃*` totally with respect to `s` gives `∂k̃* / ∂s > 0` (higher saving raises steady-state capital). Differentiating with respect to `n` (or `δ`) gives `∂k̃* / ∂n < 0` (higher population growth lowers steady-state capital per effective worker — the same gross investment is spread thinner across more workers). The long-run **per-capita output growth rate** equals `g` (the exogenous technology rate) regardless of `s` — this is the Solow model's central limitation that the endogenous-growth literature (`ec-endogenous-growth-and-convergence`) attempts to address. **Source:** Romer (2019) Ch.1 pp.35-49.

### Historical Contrast — Malthusian Trap

Per Codex Round-1 REQUIRED_CHANGE 2 (alias closure for the source-ledger `ec-classical-malthus-growth-model` candidate), this card carries a brief historical contrast. The **classical Malthusian model** (pre-industrial-revolution growth framework) assumed (a) constant returns to land-and-labor without capital, and (b) population growth that adjusts endogenously to per-capita income — when income rises above subsistence, population grows and dilutes per-capita income back to subsistence. The Malthusian trap predicts long-run stagnation in per-capita living standards regardless of technological progress, because population growth absorbs all productivity gains. Solow's model **breaks** the Malthusian trap by introducing capital as a separately-accumulable factor with diminishing returns: technological progress in the Solow framework raises per-capita output by raising the effective-labor input, and population growth (`n` in Solow's formulation) is exogenous rather than endogenous to income. The empirical break with Malthus occurred at the Industrial Revolution; modern macro frameworks all use Solow / Ramsey / endogenous-growth foundations and treat Malthus as a useful pre-Industrial-Revolution baseline rather than a contemporary model. **Source:** Romer (2019) Ch.1 pp.6-30.

## See Also

- [`ec-production-functions-and-firm`](./ec-production-functions-and-firm.md) — the production-function side `f(K, L)` that Solow inherits
- [`ec-rental-rate-of-capital-microeconomic-foundation`](./ec-rental-rate-of-capital-microeconomic-foundation.md) — supplies the steady-state `r = MPK − δ` condition that Solow uses
- [`ec-ramsey-cass-koopmans-savings`](./ec-ramsey-cass-koopmans-savings.md) — extends Solow's exogenous savings to optimal Euler-equation savings
- [`ec-endogenous-growth-and-convergence`](./ec-endogenous-growth-and-convergence.md) — relaxes Solow's exogenous-technology assumption to model long-run per-capita growth

## Escalate to Raw When

The transitional-dynamics analysis of the convergence rate `λ = (1 − α)(n + g + δ)` and the empirical estimation of `λ` from cross-country growth regressions sit in Romer Ch.1 pp.35-49 and Ch.4 pp.150-187. The integration of Solow with overlapping-generations or infinite-horizon micro-foundations (where saving is derived from utility maximization rather than imposed exogenously) is in Romer Ch.2 (treated in `ec-ramsey-cass-koopmans-savings`). The endogenous-growth literature that relaxes Solow's exogenous-technology assumption is in Romer Ch.3 (treated in `ec-endogenous-growth-and-convergence`). The empirical Solow-residual / growth-accounting decomposition (`Δ log Y = α · Δ log K + (1−α) · Δ log L + Solow residual`) sits in Romer Ch.1 §1.7 pp.30-35. **Source:** Romer (2019) pp.6-49.
