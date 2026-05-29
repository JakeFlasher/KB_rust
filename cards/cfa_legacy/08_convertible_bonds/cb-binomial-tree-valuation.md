---
schema_version: "cacg.v0"
id: "cb-binomial-tree-valuation"
title: "Credit-Aware Binomial Tree for Convertibles"
reading_id: "08_convertible_bonds"
summary: "The credit-aware binomial tree (CRR with default branch) prices convertibles whose embedded American options on the issuer's own continuation value defeat the special-case bond-plus-call identity; Hull §27.4 sets up the three-branch up/down/default node where default pays the recovery PV and credit risk is taken into account via stock-price-conditioned credit spreads."
tags: ["convertible-bonds", "binomial-tree"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p287:0434"
    chunk_hash: "5c7b7e6b9882c668fe66533792f645b19456fa2701a9915ec311bb5712492fe0"
    page_range: [287, 288]
    quote: "A useful and very popular technique for pricing an option involves constructing a binomial tree."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p650:0966"
    chunk_hash: "e972bd11f1061f9542bb0de121768a78e178cb4a8482c4dec60fa7c3a2225f3b"
    page_range: [650, 651]
    quote: "Credit risk plays an important role in the valuation of convertibles."
    edge_type: "supports"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p128:0165"
    chunk_hash: "e2c05a0651e787c7e62277bd197a690414ecdb8b1e93413e9a02f04da32299f4"
    page_range: [128, 129]
    quote: "Encountering a trigger will indeed happen at depressed share prices."
    edge_type: "supports"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p081:0094"
    chunk_hash: "1d65fc593814bbb3ebd8362b8696d3ad70649bb6f5953ba4f8f67480a35e0700"
    page_range: [81, 82]
    quote: "After debt and equity have been issued, it is the equity owners who decide when to default."
    edge_type: "supports"
card_hash: "148879efa2134b3f8be980debc0b7449decc4ba78547ed0650256eae117a2def"
---
# Credit-Aware Binomial Tree for Convertibles

## Intuition

The bond-plus-call identity from the
[payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning)
holds only for non-callable, non-defaultable, European-conversion
convertibles. Real convertibles have callable / puttable / American
features and are credit-risky, so the embedded-option leg is written
on the convertible's own **continuation value** `V_continue` rather
than on `q · S` directly. This circular dependency is resolved by
backward induction on a **credit-aware binomial tree** that augments
the standard CRR (Cox-Ross-Rubinstein) tree with a default branch at
each node. **Source:** Hull (recent ed.) §10.11 pp.241-242; Hull
(recent ed.) §27.4 pp.650-653.

```
single CRR step with default branch:

           up: S * u, no default       (probability p · (1 - h Δt))
          /
   S(t) -----  default at any time     (probability h Δt)
          \                             V_node = R · F (recovery PV)
           down: S * d, no default     (probability (1-p) · (1 - h Δt))

  u = exp(σ √Δt)     d = 1/u     p = (e^((r-δ)Δt) - d) / (u - d)
  h = hazard rate (default intensity); see cb-credit-spread-machinery
```

## Definition

Let `(S_k, t_k)` index the share-price-time grid with step `Δt =
t_{k+1} − t_k`. The **credit-aware CRR tree** assigns three children
to each node. **Source:** Hull (recent ed.) §27.4 pp.650-653;
DeSpiegeleer et al. (2014) §3.6 pp.95-110; Lando (2004) §4 pp.75-90.

- **Up branch**: `S_{k+1} = S_k · u`, no default; risk-neutral
  probability `p · (1 − h(t_k) · Δt)`. **Source:** Hull (recent ed.)
  §27.4 pp.650-653.
- **Down branch**: `S_{k+1} = S_k · d`, no default; risk-neutral
  probability `(1 − p) · (1 − h(t_k) · Δt)`. **Source:** Hull (recent
  ed.) §27.4 pp.650-653.
- **Default branch**: `S_{k+1} = 0` (issuer absorbed); convertible's
  payoff at the default node is the recovery PV `R · F` where `R` is
  the assumed recovery rate. Probability `h(t_k) · Δt` per step.
  **Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-110; Lando (2004)
  §4 pp.75-90.

The CRR parameter triple is calibrated to match the share-price
volatility `σ` and the share's expected risk-neutral drift `μ - δ`
(`δ` is the dividend yield). **Source:** Hull (recent ed.) §10.11
pp.241-242. The hazard rate `h(t)` is sourced from the credit-spread
machinery (see the
[credit-spread card](./cb-credit-spread-machinery.md#definition)).

The convertible's value at each node is determined by **backward
induction**. **Source:** Hull (recent ed.) §27.4 pp.650-653;
DeSpiegeleer et al. (2014) §3.6 pp.95-110.

```
backward induction at non-terminal node (S_n, t_n):

  V_continue := e^(-r Δt) · [
                  p · (1 - h Δt) · V(S_n · u, t_{n+1})
                + (1-p) · (1 - h Δt) · V(S_n · d, t_{n+1})
                + h Δt · R · F
              ]
              + accrued coupon over Δt

  V_n := max( q · S_n,                  (immediate-conversion value)
              V_continue,                (continuation value)
              P_put(t_n)              ) (put protection, if eligible)

  Issuer-call cap: V_n := min(V_n, K(t_n)) if call-eligible
```

## Mathematical Reasoning

The backward-induction formula above embeds three **decision points**
that the special-case bond-plus-call identity cannot capture.
**Source:** Hull (recent ed.) §27.4 pp.650-653.

- **Holder's American conversion**: at every node where conversion is
  eligible, the holder may exchange `V_continue` for `q · S_n`. This
  is the early-exercise free-boundary problem from the
  [conversion-mechanics card](./cb-conversion-feature-mechanics.md#mathematical-reasoning).
  **Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-110.
- **Issuer's call cap**: if call-eligible at node `n` and the
  continuation value exceeds the call price `K(t_n)`, the issuer
  redeems at `K(t_n)`. The cap is enforced via `V_n := min(V_n,
  K(t_n))`. **Source:** Hull (recent ed.) §27.4 pp.650-653.
- **Holder's put floor**: if put-eligible at node `n`, the holder
  may put the bond back at `P_put(t_n)`. The floor is enforced via
  `V_n := max(V_n, P_put(t_n))`. **Source:** DeSpiegeleer et al.
  (2014) §2.5 pp.65-78.

The default-branch recovery `R · F` is the credit anchor — it bounds
the convertible's value from below at every node, replacing the
straight-bond floor `B(t)` from the
[bond-floor card](./cb-bond-floor-investment-value.md#mathematical-reasoning)
with a node-by-node recovery model. **Source:** Lando (2004) §3-§4
pp.60-90. As `Δt → 0` and `N → ∞`, the tree converges to the
PDE-and-free-boundary continuum solution (see the
[PDE card](./cb-pde-and-free-boundary.md#mathematical-reasoning)).

The **convergence rate** of the credit-aware CRR tree is `O(Δt) =
O(1/N)` for European payoffs and `O(1/N^{1/2})` for early-exercise
boundaries with non-smooth payoffs at the boundary; see Hull §13 for
the standard convergence analysis transferred to the convertible
case. **Source:** Hull (recent ed.) §27.4 pp.650-653.

The tree's **soft-call trigger** handling requires path tracking. A
soft call is gated on a counting indicator `K(t) ≥ M` over the
trailing `N` daily closes (see the
[call-and-put card](./cb-call-and-put-protection.md#mathematical-reasoning)).
A naive tree cannot track the trailing window because each node
collapses paths. The standard fix is to augment each node with the
trailing count, yielding a tree of size `O(N · M)`; alternatively,
practitioners replace the soft-call gate with an "instantaneous"
trigger `S_n ≥ α · K_c` at the cost of slightly conservative pricing.
**Source:** DeSpiegeleer et al. (2014) §2.5 pp.60-78.

Asymptotic behavior (cases below). **Source:** Hull (recent ed.)
§27.4 pp.650-653; Lando (2004) §4 pp.75-90.

- `S → ∞`: `V_continue → q · S_n · D_div(t_n, T)` (deep-ITM holder
  conversion); the issuer-call cap binds early; the tree's effective
  forward equity exposure approaches `q`. **Source:** Hull (recent
  ed.) §27.4 pp.650-653.
- `S → 0` with `h(t)` rising: the default-branch contribution
  dominates; `V_continue → R · F · D_rf(t_n, τ_avg)`; this is the
  numerical realization of the
  [bond-floor card's](./cb-bond-floor-investment-value.md#mathematical-reasoning)
  stressed-credit asymptote. **Source:** Lando (2004) §4 pp.75-90.
- `Δt → 0`: the discrete tree converges to the
  PDE-and-free-boundary solution; the option-side risk-neutralization
  matches the BSM-style limit. **Source:** Hull (recent ed.) §27.4
  pp.650-653.

## See Also

- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — the special-case identity this tree generalizes
- [`cb-credit-vs-equity-decomposition.md`](cb-credit-vs-equity-decomposition.md) — the credit-equity split this tree numerically realizes
- [`cb-credit-spread-machinery.md`](cb-credit-spread-machinery.md) — the hazard rate `h(t)` the tree consumes
- [`cb-pde-and-free-boundary.md`](cb-pde-and-free-boundary.md) — the continuum limit
- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — the issuer-call / holder-put boundary the tree enforces

## Escalate to Raw When

Open Hull (recent ed.) §10.11 pp.241-242 + §27.4 pp.650-653 directly
for the credit-aware tree's defining equations and the
joint-embedded-option backward induction. **Source:** Hull (recent
ed.) §10.11 pp.241-242; Hull (recent ed.) §27.4 pp.650-653.

Open DeSpiegeleer §3.6 pp.95-110 for the standard hybrid-securities
tree implementation, including the soft-call trailing-window
augmentation and the convergence-rate diagnostics. **Source:**
DeSpiegeleer et al. (2014) §3.6 pp.95-110.

Open Lando §3-§4 pp.60-90 for the rigorous treatment of the hazard-
rate models (homogeneous Poisson, doubly-stochastic Poisson) and the
default-branch probability calibration. **Source:** Lando (2004) §3-§4
pp.60-90.
