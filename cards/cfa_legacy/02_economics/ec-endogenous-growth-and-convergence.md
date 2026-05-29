---
schema_version: "cacg.v0"
id: "ec-endogenous-growth-and-convergence"
title: "Endogenous Growth and Convergence"
reading_id: "02_economics"
summary: "Romer Ch.3-4 endogenous growth (AK and R&D-based models) and cross-country convergence taxonomy: absolute (all poor catch up) vs conditional (poor converge to own steady-state fundamentals); empirical conditional convergence at ~2 percent per year."
tags: ["economics", "endogenous-growth"]
citations:
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p117:0154"
    chunk_hash: "b36ea2ad790e54e38488bc2fa157d926faa08603e860ad8818298cfe3c15b715"
    page_range: [117, 118]
    quote: "But they differ from the earlier models in explicitly interpreting the effectiveness of labor as knowledge and in modeling the determinants of its evolution over time."
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p125:0163"
    chunk_hash: "8e528a7b4808c380bf646c65ab887c271eff8f0f9e64f7e224f1996f85cc0ffa"
    page_range: [125, 126]
    quote: "As the phase diagram shows, the economy exhibits ever-increasing growth rather than convergence to a balanced growth path."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p724:1006"
    chunk_hash: "08b52ea2699ccccc4c3c22662eeed108e85d25264798c32e56f12ad38a2a7b91"
    page_range: [724, 724]
    quote: "The position of the LRAS curve is determined by the potential output of the economy."
    edge_type: "supports"
card_hash: "fee1c2abff0f25a7b67ae5aae1d111b91b20412b84b4e785be9b8eef28b925d8"
---
# Endogenous Growth and Convergence

## Intuition

The Solow model treats technological progress as exogenous — the long-run per-capita growth rate `g` is taken as given and not derived from any economic decision. Endogenous-growth theory aims to model `g` itself as the outcome of saving, investment in R&D, or human-capital accumulation. The simplest endogenous-growth model is the **AK model**: production is `Y = A · K` (linear in capital, no diminishing returns); saving and investing a constant fraction `s` of output gives `K̇ / K = sA − δ` — a constant growth rate that depends on the saving rate. The AK structure removes Solow's diminishing-returns brake on growth. **Source:** Romer (2019) Ch.3 pp.99-149.

```
   convergence types (cross-country growth regression):
   d log(y_i) / dt   vs   log(y_i)

   absolute convergence            conditional convergence
   (countries homogeneous)         (different steady states)

   growth                          growth
     ^                                 ^
     |                                 |  *  poor with weak fundamentals
     |\                                |
     | \                               |   *  poor with strong fundamentals
     |  *                              |  /
     |   \  (single line:              | /
     |    \  poor grow fastest)        |/
     |     *                           *           rich with strong fund.
     |      \                          *           rich with weak fund.
     +-------+----------> log(y)       +----------> log(y)
        all poor catch up to             countries cluster by steady-state
        the same level                    fundamentals, conditional convergence
                                          within each cluster
```

The empirical distinction between **absolute** and **conditional** convergence is the central debate of cross-country growth: do all poor countries catch up to all rich countries (absolute)? Or do countries converge only to their own steady-state determined by fundamentals like saving rates, education, institutions, and demographics (conditional)? The empirical literature (Romer Ch.4) finds limited absolute convergence in the broad cross-country sample but strong conditional convergence after controlling for fundamentals. **Source:** Romer (2019) Ch.4 pp.150-187.

## Definition

The **AK model** of endogenous growth uses linear-in-capital production: **Source:** Romer (2019) pp.99-187.

```
Y = A · K                          (no diminishing returns)
K̇ = s · Y − δ · K = (sA − δ) · K
K̇ / K = sA − δ                    (constant growth rate)
```

In contrast to Solow's per-capita growth rate of zero (or exogenous `g`), the AK model produces per-capita growth `sA − δ` that depends on the saving rate. **Source:** Romer (2019) Ch.3 pp.105-115.

The **Romer-Lucas R&D endogenous-growth model** (more sophisticated): output is produced with capital, labor, and a stock of knowledge `A(t)` that grows via R&D investment: **Source:** Romer (2019) pp.99-187.

```
Y(t) = K(t)^α · (A(t) · L_Y(t))^(1−α)        [final-goods production]
Ȧ(t) = δ_A · L_A(t)^γ · A(t)^θ                [knowledge accumulation, R&D]
L_Y + L_A = L                                 [labor allocation]
```

where `L_A` is R&D labor and `L_Y` is goods-production labor; the parameter `θ` measures the strength of intertemporal knowledge spillovers. The steady-state per-capita growth rate `g` depends on `δ_A`, `γ`, `θ`, and the equilibrium allocation `L_A / L`. **Source:** Romer (2019) Ch.3 pp.115-149.

The **convergence equation** loglinearized around the steady state: **Source:** Romer (2019) pp.99-187.

```
d log(y_t) / dt  ≈  λ · (log y* − log y_t)        [convergence to s.s.]
                                                  λ = (1−α)(n+g+δ)
```

**Absolute convergence**: across countries `i`, `d log(y_i) / dt = a + b · log(y_i) + ε_i` with `b < 0`. Empirically `b` is statistically indistinguishable from zero in broad samples — no absolute convergence. **Conditional convergence**: same regression with controls for fundamentals `X_i`: `d log(y_i) / dt = a + b · log(y_i) + c · X_i + ε_i`. Empirically `b` becomes significantly negative after controls — conditional convergence at the often-cited "two-percent-per-year" rate. **Source:** Romer (2019) Ch.4 pp.150-187.

## Mathematical Reasoning

The AK model's permanent-growth result follows directly from the linear production function: with `Y = AK`, the capital-output ratio is constant `K/Y = 1/A`, so growth in `K` translates one-to-one into growth in `Y`. Diminishing returns to capital (the Solow assumption) is what causes the growth rate to slow toward zero as capital accumulates; the AK assumption eliminates this brake by setting `f''(K) = 0`. The cost of the AK assumption is empirical: a capital share near one (vs the much lower empirical share for narrow physical capital) requires the broad-capital aggregate to include human capital and other intangibles to be plausible. The "broad capital" interpretation of AK (where `K` includes physical, human, and organizational capital with combined share near one) is the standard defense. **Source:** Romer (2019) Ch.3 pp.105-115.

The Romer-Lucas R&D model derives the long-run growth rate as the outcome of the allocation of labor between final-goods production and R&D. With knowledge accumulation `Ȧ / A = δ_A · L_A^γ · A^(θ−1)`, the steady-state growth rate `g = Ȧ/A` exists if `θ < 1` (decreasing returns to existing knowledge; the "fishing-out effect" that R&D becomes harder as low-hanging fruit is exhausted) and the equilibrium R&D labor allocation `L_A*` solves the household's intertemporal optimization. Higher `δ_A` (R&D productivity) or higher equilibrium `L_A` raises long-run growth — policy implications include subsidies to R&D and education to internalize knowledge spillovers. **Source:** Romer (2019) Ch.3 pp.115-149.

The **convergence-speed result** `λ = (1 − α)(n + g + δ)` derives from loglinearizing the Solow / Ramsey dynamic around the steady state. Under standard parameter magnitudes (moderate capital share, modest annual depreciation-plus-growth wedge), the theoretical convergence speed implies a multi-decade half-life. Empirical estimates of `λ` from cross-country regressions are systematically slower than the theoretical prediction — the **convergence-rate puzzle**: either capital share is higher in the "broad capital" sense (consistent with AK-style models), or the convergence dynamic is contaminated by transitional shocks that bias `λ` downward. **Source:** Romer (2019) Ch.4 pp.150-187.

### Notes-Ledger Convergence Content (per Codex Round-1 alias closure)

Per the v10 plan's resolved DEC-7.1 closure for the source-ledger `ec-absolute-and-conditional-convergence` candidate, this card carries the canonical convergence-types distinction. The Romer-anchored framework provides the formal convergence equation `d log(y) / dt = λ · (log y* − log y)` and the cross-country growth debate context. **Source:** Romer (2019) Ch.4 pp.150-187.

## See Also

- [`ec-solow-growth-model`](./ec-solow-growth-model.md) — exogenous-technology baseline that endogenous-growth theory relaxes
- [`ec-ramsey-cass-koopmans-savings`](./ec-ramsey-cass-koopmans-savings.md) — optimal-savings framework that nests both Solow and endogenous-growth models
- [`ec-production-functions-and-firm`](./ec-production-functions-and-firm.md) — production-side foundation for both Solow and AK

## Escalate to Raw When

The full Romer-Lucas R&D model derivation (including the patent-protection / market-power assumptions that fund R&D from monopoly profits) is in Romer Ch.3 pp.115-149. The cross-country growth-empirics literature (Mankiw-Romer-Weil regressions, augmented Solow with human capital, institutional-fundamentals controls) is in Romer Ch.4 pp.150-187. The "convergence club" literature (multiple steady states; countries cluster by initial conditions) is in Romer Ch.4 §4.5 pp.175-187. The unified growth theory (transition from Malthusian stagnation through the Industrial Revolution to modern sustained growth) is in research literature out of v10 scope. **Source:** Romer (2019) pp.99-187.
