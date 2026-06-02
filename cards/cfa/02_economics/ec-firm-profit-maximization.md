---
schema_version: "cacg.v0"
id: "ec-firm-profit-maximization"
title: "Firm Profit Maximization"
reading_id: "02_economics"
summary: "MWG Ch.5 competitive firm's profit-maximization problem: choose inputs s.t. value of MP equals input price; derive supply function and conditional factor demands via Hotelling's lemma (envelope theorem applied to π(p,w,r))."
tags: ["economics", "firm-profit"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p155:0251"
    chunk_hash: "1761fb45d20f150bddfd8e42d8d9273e93f69fd770e10ae8630454eb4f596957"
    page_range: [155, 156]
    quote: "For L = 2, this says that the slope of the transformation frontier at the profit-maximizing production pbu must be equal to the negative of the price ratio, as shown in Figure 5.C.1."
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p156:0253"
    chunk_hash: "5c74ce73a87e7c20f8a768359c19c5b11aa38b636990e44b3f047e06efab0fcd"
    page_range: [156, 157]
    quote: "(vi) (Hotelling's lemma) If y(p ) consists of a single point, then n( ·) is differentiable at p and Vn(p) = y(p)."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p642:0884"
    chunk_hash: "73d9011abe75902090bd9688c017853375fabd1440c039dbc5d3005a43637070"
    page_range: [642, 642]
    quote: "Unlike some economic concepts, perfect competition is not merely an ideal based on assump"
    edge_type: "supports"
card_hash: "53b7f9457119f0b3e00926b553ac315cd6341fcc0cdd37eb11baf1f13be24c9c"
---
# Firm Profit Maximization

## Intuition

A competitive firm takes prices as given (one output price `p` and a vector of input prices `(w, r, ...)`) and chooses inputs to maximize profit `π = p · y − w · L − r · K − ...`. The first-order conditions equate the value of each input's marginal product to its price — `p · MPL = w`, `p · MPK = r` — so the firm hires each input up to the point where the last unit's contribution to revenue covers its cost. The optimal output is the supply function `y(p, w, r)`; the optimal inputs are the conditional factor demands `L(p, w, r)` and `K(p, w, r)`. **Source:** Mas-Colell et al. (1995) Ch.5 pp.135-148.

```
   p · MPL
   ^
   |
   |    .
   |     .
   |      .                    horizontal: input price w
   |       .   .   .   .   .   .   .   .   .   .   .   . w
   |          .
   |             .
   |               .         (decreasing because MPL decreases as L rises;
   |                  .       value-of-marginal-product curve)
   +-------------+------+-------------+----> L (labor)
                       L*

   optimal L: hire labor up to where p · MPL = w
   slope of VMPL curve traces the firm's conditional labor demand
```

The profit function `π(p, w, r) = max p · f(K, L) − wL − rK` is a key object because by the **envelope theorem** its derivatives are exactly the firm's choices: `∂π/∂p = y(p, w, r)` (supply equals output partial of profit), `∂π/∂w = −L(p, w, r)` (negative of labor demand), `∂π/∂r = −K(p, w, r)` (negative of capital demand). This is **Hotelling's lemma**, the firm-side analog of Shephard's lemma on the consumer cost side. **Source:** Mas-Colell et al. (1995) Ch.5 pp.142-148.

## Definition

The competitive firm's **profit-maximization problem** with production function `f(K, L)`, output price `p`, and input prices `(w, r)` is: **Source:** Mas-Colell et al. (1995) pp.135-152.

```
max_{K ≥ 0, L ≥ 0}  π(K, L; p, w, r) = p · f(K, L)  −  w · L  −  r · K
```

The **first-order conditions** (interior solution) are: **Source:** Mas-Colell et al. (1995) pp.135-152.

```
p · ∂f/∂L = w        (value of marginal product of labor = wage)
p · ∂f/∂K = r        (value of marginal product of capital = rental rate)
```

The **profit function** `π(p, w, r)` is the maximized value: **Source:** Mas-Colell et al. (1995) pp.135-152.

```
π(p, w, r) = max_{K, L}  [ p · f(K, L) − wL − rK ]
```

**Hotelling's lemma** (envelope theorem applied to `π`): **Source:** Mas-Colell et al. (1995) pp.135-152.

```
∂π / ∂p  =  y(p, w, r)        (supply)
∂π / ∂w  =  − L(p, w, r)      (negative of labor demand)
∂π / ∂r  =  − K(p, w, r)      (negative of capital demand)
```

The **supply function** `y(p, w, r)` satisfies the comparative-statics property of being non-decreasing in `p` (Law of Supply); the conditional factor demands `L(p, w, r), K(p, w, r)` are non-increasing in their own price. **Source:** Mas-Colell et al. (1995) Ch.5 pp.135-152.

## Mathematical Reasoning

The first-order conditions translate the technology (marginal products) into the firm's behavior (input demands). At the interior optimum, the value of each input's marginal product equals its price; equivalently the ratio of marginal products equals the price ratio: `MPL / MPK = w / r`. This is the MRTS = (w/r) tangency condition from `ec-production-functions-and-firm` — the firm cost-minimizes for the chosen output level, then chooses output to maximize profit. **Source:** Mas-Colell et al. (1995) Ch.5 pp.140-145.

The profit function `π(p, w, r)` is **convex in `(p, w, r)`**, monotone non-decreasing in `p`, non-increasing in `(w, r)`, and **homogeneous of degree 1**: `π(λp, λw, λr) = λ · π(p, w, r)` for `λ > 0`. Convexity follows because `π` is the maximum of linear functions in `(p, w, r)` (each fixed `(K, L)` gives a linear-in-prices function); the max of linear functions is convex. Convexity of `π` in `p` implies that supply `y(p) = ∂π/∂p` is non-decreasing in `p`, recovering the Law of Supply without separately proving it. **Source:** Mas-Colell et al. (1995) Ch.5 pp.142-148.

The CRS case is **special**: if `f` exhibits CRS, then `π(p, w, r) = 0` along any positive-profit equilibrium (otherwise the firm could scale up indefinitely and obtain unbounded profit). At zero profit, factor payments exhaust output (`w · L + r · K = p · y`), and CRS plus constant factor shares (e.g., Cobb-Douglas) give the macro accounting identity `Y = wL + rK`. The competitive firm under CRS is indeterminate in scale — only the input proportions are pinned down, not the absolute size; aggregate equilibrium resolves the scale at the level of industry / economy supply meeting demand. The decreasing-returns case (DRS) gives a well-defined per-firm scale with positive profit; IRS gives a natural-monopoly tendency where one firm captures the market. **Source:** Mas-Colell et al. (1995) Ch.5 pp.146-152.

## See Also

- [`ec-production-functions-and-firm`](./ec-production-functions-and-firm.md) — the technology side `f(K, L)` that this card maximizes profit over
- [`ec-perfect-competition-equilibrium`](./ec-perfect-competition-equilibrium.md) — how the firm's supply combines with market demand to clear prices
- [`ec-monopoly-pricing`](./ec-monopoly-pricing.md) — what changes when the firm faces a downward-sloping demand curve instead of a fixed `p`
- [`ec-investment-cost-of-capital`](./ec-investment-cost-of-capital.md) — extends the static profit-max to dynamic investment under adjustment costs (Romer Ch.9)

## Escalate to Raw When

The full proof of Hotelling's lemma via the envelope theorem requires differentiability of `π` and uses the maximum theorem to pass derivatives through the constrained optimization — re-open MWG Ch.5 pp.142-145. The duality between profit function and production set (recovering `Y` from `π`) is in MWG pp.146-152. For the CRS-indeterminacy resolution at industry level (long-run supply curves) see MWG Ch.10 pp.311-349, treated in the sibling `ec-perfect-competition-equilibrium` card. The dynamic investment problem with adjustment costs and Tobin's q is in Romer Ch.9 and lives in `ec-investment-cost-of-capital`. **Source:** Mas-Colell et al. (1995) pp.135-152.
