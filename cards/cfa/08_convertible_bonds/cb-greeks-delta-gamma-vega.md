---
schema_version: "cacg.v0"
id: "cb-greeks-delta-gamma-vega"
title: "Convertible Greeks: Delta, Gamma, Vega"
reading_id: "08_convertible_bonds"
summary: "Convertible Greeks: Delta, Gamma, Vega — placeholder summary                    "
tags: ["convertible-bonds", "greeks-delta"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p062:0069"
    chunk_hash: "179e713c205d43b92db8a159d2cd496f7ac00321b04d674542202ea7c4263737"
    page_range: [62, 62]
    quote: "The most important and probably most looked at risk component of a convertible bond is its underlying equity risk. This is measured by the delta."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p062:0070"
    chunk_hash: "3976cb6629efe93e473fdc974d68319b8dfdc08ae1762cedba01fb21283c4c80"
    page_range: [62, 63]
    quote: "For high share prices, the convertible’s price converges to parity and the convertible bond adopts equity-like behavior."
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p063:0067"
    chunk_hash: "18a763cb03a107721b192ba959880a4a7baba76377fc17f344261dcf9825b616"
    page_range: [63, 64]
    quote: "The first greek to understand is the delta. The convertible’s delta measures the change in the convertible’s price (CV) with respect to the change in the underlying common-stock price (S)."
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p242:0371"
    chunk_hash: "834ae1b123738b60e5d1e1c1a09c41767bff9374213356771e8430f1eef484b0"
    page_range: [242, 242]
    quote: "They are therefore bonds with an embedded call option on the company’s stock."
    edge_type: "supports"
card_hash: "529ca7bd44659b6c95f202d6abd9c6b8b96ed8ebf86d3cbd45640a3a87d8b095"
---
# Convertible Greeks: Delta, Gamma, Vega

## Intuition

A convertible's risk profile is a blend of bond and option exposures. The
**delta** measures equity sensitivity, **gamma** measures convexity,
**vega** measures volatility sensitivity, **rho** measures interest-rate
sensitivity, and **theta** measures time decay. Practitioners watch all
five plus the **dividend epsilon** (sensitivity to changes in dividend
yield) and **borrow psi** (sensitivity to the share's borrow cost) because
these directly drive convertible-arbitrage P&L.
**Source:** DeSpiegeleer et al. (2014) §2.3 pp.40-58.

```
Greek profile across regimes (qualitative):

      delta   gamma   vega   rho    theta   eps    psi
S→0     0+    small    +     bond     -      0     0
S=K_c   ~q/2  large    +     mixed   --     -ε    -ψ
S→∞     q     small    0    bond     0      0     0
```

## Definition

For a convertible price `V(S, t)` with cited frontmatter `(F, q, K_c, c, T)`,
the standard Greeks are partial derivatives of `V` with respect to the
named market inputs. **Source:** DeSpiegeleer et al. (2014) §2.3 pp.45-58.

- **Delta** `Δ := ∂V/∂S` — equity sensitivity in absolute terms. The
  practitioner-quoted `Δ_S := Δ × (N / 100)` is the number of underlying
  shares to short to delta-neutralize one bond at percentage quote (N is
  number of shares per face). The percentage delta `Δ_% := Δ / q` is the
  bond price's % move per 1% move in the share price. **Source:**
  DeSpiegeleer et al. (2014) §2.3 pp.45-50.
- **Gamma** `Γ := ∂²V/∂S²` — second-order convexity. Convertibles have
  positive gamma in the balanced regime, which is the foundation of
  convertible arbitrage (a delta-hedged long-CB position generates gamma
  P&L from share-price oscillation). **Source:** Calamos (2003) §11
  pp.260-285.
- **Vega** `ν := ∂V/∂σ` — sensitivity to model volatility. Practitioner
  scaling is per 1% absolute increase in `σ`. **Source:** DeSpiegeleer
  et al. (2014) §2.3 pp.50-55.
- **Rho** `ρ := ∂V/∂r` — interest-rate sensitivity. Splits across the
  bond leg (negative, like a straight bond) and the call leg (positive).
  **Source:** DeSpiegeleer et al. (2014) §2.3 pp.55-58.
- **Theta** `Θ := -∂V/∂(T-t)` — time decay. Has two components: the call
  leg's theta (always negative for the holder, decaying time value) and
  the bond leg's theta (positive — accrued interest). **Source:**
  DeSpiegeleer et al. (2014) §2.3 pp.55-58.
- **Epsilon** `ε := ∂V/∂q_div` — sensitivity to dividend yield (typically
  scaled to a 10% relative bump). Negative for the holder because higher
  dividends transfer value to shareholders, not bondholders. **Source:**
  DeSpiegeleer et al. (2014) §2.3 pp.50-58.
- **Psi** `ψ := ∂V/∂b` — sensitivity to the share's borrow cost. Relevant
  to convertible-arbitrage funding because the delta-hedger pays borrow
  on the short share leg. **Source:** Calamos (2003) §11 pp.285-300.

## Mathematical Reasoning

Under the special-case bond-plus-call decomposition (see the
[payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning))
the convertible Greeks decompose linearly into bond-leg and call-leg
contributions. **Source:** Hull (recent ed.) §10.11 pp.241-242;
DeSpiegeleer et al. (2014) §2.3 pp.45-58.

```
∂V/∂S     =  q · ∂c/∂S            (delta -- equity exposure)
∂^2V/∂S^2 =  q · ∂^2c/∂S^2        (gamma -- convexity)
∂V/∂σ     =  q · ∂c/∂σ            (vega  -- vol sensitivity)
∂V/∂r     =  ∂B/∂r + q · ∂c/∂r    (rho   -- splits across legs)
∂V/∂q_div =  q · ∂c/∂(q_div)      (epsilon -- equity-side only)
```

The decomposition fails for callable / credit-risky convertibles because
the call-leg Greeks must be evaluated against `V_continue` (the
convertible's own continuation value), not against `q · S` directly. In
practice the Greeks are computed numerically from the credit-aware tree
or PDE: `∂V/∂S ≈ (V(S + ΔS, t) − V(S − ΔS, t)) / (2 · ΔS)` with a small
finite-difference bump. **Source:** Hull (recent ed.) §27.4 pp.650-653.

Asymptotic Greek behavior matches the dual-floor inequality. **Source:**
DeSpiegeleer et al. (2014) §2.3 pp.50-58.

- `S → ∞`: delta `→ q`, gamma `→ 0` (deep-ITM call is linear in `S`),
  vega `→ 0` (no time value left), rho approaches the call-leg
  contribution. **Source:** DeSpiegeleer et al. (2014) §2.3 pp.50-58.
- `S → 0` with stable credit: delta `→ 0`, gamma `→ 0` (deep-OTM call
  is flat), vega `→ 0`, rho approaches the bond-leg contribution.
  **Source:** DeSpiegeleer et al. (2014) §2.3 pp.50-58.
- `S ≈ K_c` (balanced regime): all Greeks at peak magnitude. Maximum
  gamma + vega is the practitioner sweet spot for convertible arbitrage.
  **Source:** Calamos (2003) §11 pp.260-300.

Convertible **convexity profile** can be **double-signed** in stress (see
the [bond-floor card](./cb-bond-floor-investment-value.md#mathematical-reasoning)):
when issuer credit deteriorates as the share price falls, the convertible's
gamma can flip sign relative to a constant-credit pricing model. This is
the canonical "double-signed gamma" risk that convertible arbitrageurs
hedge with separate credit-default-swap protection. **Source:**
DeSpiegeleer et al. (2014) §2.4 pp.45-58; Calamos (2003) §11 pp.290-300.

## See Also

- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — the underlying identity the Greeks come from
- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — `q`, `K_c` parameters
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — double-signed gamma context

## Escalate to Raw When

Open DeSpiegeleer §2.3 pp.40-58 for the exhaustive Greek table (delta,
gamma, vega, rho, theta, omega, lambda, vanna, volga, ...) and the
practitioner scaling conventions. **Source:** DeSpiegeleer et al. (2014)
§2.3 pp.40-58.

Open Calamos §11 pp.260-300 for the convertible-arbitrage P&L attribution
that decomposes realized return into delta, gamma, vega, theta, and
borrow-cost components. **Source:** Calamos (2003) §11 pp.260-300.

Open Hull §10.11 pp.241-242 + §27.4 pp.650-653 for the credit-aware tree
that prices the Greeks under joint embedded-option exposures. **Source:**
Hull (recent ed.) §10.11 pp.241-242; Hull (recent ed.) §27.4 pp.650-653.
