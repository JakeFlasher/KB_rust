---
schema_version: "cacg.v0"
id: "ec-production-functions-and-firm"
title: "Production Functions and the Firm"
reading_id: "02_economics"
summary: "MWG derives the firm's production function from a production set; single-output case f(z) gives the maximum amount of output producible from input bundle z; returns-to-scale classified via homogeneity (CRS = degree-1 homogeneity), and the Cobb-Douglas form f(z1,z2) = z1^a z2^b has CRS iff a+b=1, IRS for a+b>1, DRS for a+b<1."
tags: ["economics", "production-functions"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p148:0239"
    chunk_hash: "814909b587520ca9615562dd4b821771781ccddb119a3ae13ba314b3bcc7e475"
    page_range: [148, 148]
    quote: "A single-output technology is commonly described by means of a production function f(z) that gives the maximum amount q of output that can be produced using input amounts"
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p151:0244"
    chunk_hash: "7bca383d170b5bbdb3ee738c922071f06ae0ccd3834ac4e8628419ed1ad82ef1"
    page_range: [151, 152]
    quote: "The production set Y exhibits constant returns to scale if y E Y implies cxy E Y for any scalar ex"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p642:0884"
    chunk_hash: "73d9011abe75902090bd9688c017853375fabd1440c039dbc5d3005a43637070"
    page_range: [642, 642]
    quote: "We start with the most competitive environment, perfect competition. Unlike some economic concepts, perfect competition is not merely an ideal based on assump"
    edge_type: "supports"
card_hash: "45d39a76b3fd39d2adb2bc468e2c807ccd8b4054a735cc54c5b123af9d0e9c21"
---
# Production Functions and the Firm

## Intuition

The firm transforms input bundles into output. MWG starts from a **production set** `Y ⊂ R^L` (the set of feasible net-output vectors, with input coordinates negative) and derives the **production function** `f(K, L)` for the special case of a single output. The function's first derivatives are the marginal products `MPK = ∂f/∂K` and `MPL = ∂f/∂L`; the slope ratio along an isoquant is the **marginal rate of technical substitution** `MRTS = MPL / MPK`, the rate at which capital can substitute for labor while holding output constant. Returns to scale describe how `f` responds to proportional input scaling: constant (CRS), increasing (IRS), or decreasing (DRS). **Source:** Mas-Colell et al. (1995) Ch.5 pp.127-140.

```
   K
   ^
   |                                isoquant: f(K, L) = y_0
   |                              .
   |                            .
   |                          .
   |                       .
   |                    .                  slope at any point
   |                 .                     =  -MPL / MPK = -MRTS
   |              .
   |           .   .   .   .   .   .       isocost: w·L + r·K = C
   |
   +------------------------------------------> L

   cost minimization at output y_0:
       MRTS = w / r
   (slope of isoquant = slope of isocost at the tangent point)
```

CRS is the structural assumption behind the Solow growth model and most aggregate macro frameworks: doubling capital and labor doubles output, so per-capita variables `y = Y/L` and `k = K/L` form a closed dynamic system independent of population scale. The Cobb-Douglas specification `Y = K^α · L^(1-α)` with `α ∈ (0, 1)` is the canonical CRS form; its elasticity of substitution is exactly 1. **Source:** Mas-Colell et al. (1995) Ch.5 pp.140-148.

## Definition

A **production set** `Y ⊂ R^L` is the set of feasible net-output vectors. Convention: positive coordinates are outputs, negative coordinates are inputs. Standard properties include nonemptiness, closedness, no free lunch (`Y ∩ R^L_+ ⊂ {0}`), and free disposal. **Source:** Mas-Colell et al. (1995) Ch.5 pp.128-132.

A **production function** `f: R^M_+ → R_+` (single output, `M` inputs) summarizes the technology when there is one output. For two inputs `K` (capital) and `L` (labor),. **Source:** Mas-Colell et al. (1995) pp.127-148.

```
Y = f(K, L)
MPK = ∂f / ∂K        (marginal product of capital)
MPL = ∂f / ∂L        (marginal product of labor)
MRTS = MPL / MPK     (marginal rate of technical substitution)
```

**Source:** Mas-Colell et al. (1995) Ch.5 pp.132-138.

**Returns to scale**: a production function `f` exhibits constant returns to scale (CRS) iff `f(λK, λL) = λ · f(K, L)` for all `λ > 0`; increasing returns (IRS) iff `f(λK, λL) > λ · f(K, L)`; decreasing returns (DRS) iff `f(λK, λL) < λ · f(K, L)`. CRS is equivalent to homogeneity of degree 1. **Source:** Mas-Colell et al. (1995) Ch.5 pp.135-138.

**Elasticity of substitution** between `K` and `L` measures the curvature of an isoquant. **Source:** Mas-Colell et al. (1995) pp.127-148.

```
σ = d log(K/L) / d log(MRTS)        (computed along an isoquant)
```

Canonical specifications: **Source:** Mas-Colell et al. (1995) pp.127-148.

```
Cobb-Douglas:   Y = A · K^α · L^(1-α)            σ = 1,     α ∈ (0, 1)
CES:            Y = A · [a K^ρ + (1-a) L^ρ]^(1/ρ)   σ = 1 / (1-ρ)
Leontief:       Y = A · min(K/v, L/u)            σ = 0
Linear:         Y = A · (K + L)                   σ = ∞
```

Cobb-Douglas and Leontief are limiting cases of CES (`ρ → 0` and `ρ → −∞` respectively); the linear form corresponds to `ρ = 1`. **Source:** Mas-Colell et al. (1995) Ch.5 pp.140-148.

## Mathematical Reasoning

The production function's curvature properties translate algebraically into the firm's cost-minimization problem and the macro growth model's steady-state characterization. For a CRS Cobb-Douglas `Y = K^α · L^(1-α)`, the marginal products are `MPK = α · Y/K` and `MPL = (1-α) · Y/L`; the input shares are constant — `MPK · K / Y = α` (capital share) and `MPL · L / Y = 1 - α` (labor share). Constant factor shares with competitive factor markets give a clean macro accounting identity. **Source:** Mas-Colell et al. (1995) Ch.5 pp.140-145.

The MRTS condition `MRTS = w/r` at the cost-minimizing input bundle is the firm's analog of the consumer's `MRS = p_x / p_y` tangency condition. The two slopes (isoquant and isocost) must equate for an interior optimum. Out of an interior solution, the firm hires more capital when `MPK / r > MPL / w` (capital's bang-for-the-buck exceeds labor's) and vice versa, until the bang-for-the-buck ratios equalize. **Source:** Mas-Colell et al. (1995) Ch.5 pp.145-148.

Returns-to-scale classification has empirical content for industry structure. CRS is consistent with arbitrary firm size; IRS implies a natural-monopoly tendency (one large firm produces at lower average cost than two small ones); DRS limits firm scale. The CRS specification dominates aggregate macro modeling because it produces well-defined per-capita dynamics. The IRS / DRS specifications appear in industrial-organization studies of specific industries and in endogenous-growth models that allow for production externalities; these extensions are out of scope for this card and live in Romer Ch.3 (treated under `ec-endogenous-growth-and-convergence`). **Source:** Mas-Colell et al. (1995) Ch.5 pp.140-148.

## See Also

- [`ec-firm-profit-maximization`](./ec-firm-profit-maximization.md) — competitive firm objective and supply derivation under given `f` and prices `(w, r, p)`
- [`ec-rental-rate-of-capital-microeconomic-foundation`](./ec-rental-rate-of-capital-microeconomic-foundation.md) — DEC-4 bridge card extending MPK to the rental rate of capital
- [`ec-solow-growth-model`](./ec-solow-growth-model.md) — uses CRS Cobb-Douglas `f` as the canonical macro production function
- [`ec-consumer-utility-and-demand`](./ec-consumer-utility-and-demand.md) — symmetric tangency-based derivation on the consumer side

## Escalate to Raw When

The full proof of the cost-minimization equivalence to MRTS-equating tangency requires the second-order conditions plus quasi-concavity of `f` — re-open MWG Ch.5 pp.145-148. The duality between cost function and production function (Shephard's lemma applied to cost) is in MWG pp.139-141. For continuous-time dynamic firm investment with adjustment costs (Tobin's q theory), see Romer Ch.9 and the sibling `ec-investment-cost-of-capital` card; the static one-period framing here does not include investment dynamics. **Source:** Mas-Colell et al. (1995) pp.127-148.
