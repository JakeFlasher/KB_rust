---
schema_version: "cacg.v0"
id: "ec-fiscal-policy-and-budget-deficits"
title: "Fiscal Policy and Budget Deficits"
reading_id: "02_economics"
summary: "Romer Ch.13 government intertemporal budget constraint and debt-sustainability dynamics; Ricardian equivalence under Ramsey households (tax timing irrelevant); contrasts with FTPL (Cochrane) where price level adjusts to back debt with future surpluses."
tags: ["economics", "fiscal-policy"]
citations:
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p680:0990"
    chunk_hash: "a263af5bd3082164e7a1b14046e11ecf69aa6a60a2b39b2b47ed1f1b3e4f96ff"
    page_range: [680, 681]
    quote: "The chapter then turns to the sources of budget deficits when Ricardian equivalence fails."
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p678:0988"
    chunk_hash: "cc60abb7088db3f30281288b89df18a746ee83fccf9055b2b8ec09a908f8e42e"
    page_range: [678, 679]
    quote: "From the perspective of macroeconomics, fiscal policy is concerned with the overall levels and broad composition of taxes and government spending and their effects on the aggregate economy."
    edge_type: "defines"
  - source_id: "econ_cochrane_2023_fiscal_theory_price_level"
    chunk_id: "econ_cochrane_2023_fiscal_theory_price_level:p029:0037"
    chunk_hash: "14661c250d66a482876eff91c92d70bc0a128af67d2ada4ddb3ed0566fb557ea"
    page_range: [29, 30]
    quote: "As one simple story, the fiscal theory of the price level answers: Money is valued because the government accepts money for tax payments."
    edge_type: "supports"
card_hash: "90f850f1537f64ae8c576b3a3a75b81f752565ae400e59ed0060f85b2d7bfcef"
---
# Fiscal Policy and Budget Deficits

## Intuition

The government's **intertemporal budget constraint (IBC)** says the present value of future primary surpluses must equal the current debt outstanding: `B_0 = ∑ (1+r)^{-t} · (T_t − G_t)`. Any current deficit (`G_t > T_t`) must be offset by future surpluses, either through higher taxes, lower spending, or — under FTPL — inflation that erodes the real value of nominal debt. The IBC is not an arbitrary policy choice but a market-discipline constraint: bondholders refuse to roll over debt that exceeds the present-value of future surpluses, eventually forcing fiscal adjustment, default, or monetization. **Source:** Romer (2019) Ch.13 pp.660-690.

```
   Ricardian equivalence: tax-cut bond-financed vs tax-financed

   Bond-financed cut today (G constant):
       T_t ↓ → deficit ↑ → debt issued
       households save the tax cut to pay
       the higher future taxes that retire debt
       → C unchanged in aggregate
       → no AD effect (Ricardian equivalence holds)

   Conditions for Ricardian equivalence:
       (a) infinite horizon / dynastic altruism
       (b) lump-sum taxes (no distortions)
       (c) perfect capital markets (no liquidity constraints)
       (d) Ramsey-style household optimization

   Departures:
       finite horizon, OLG → partial Ricardian
       liquidity constraints → tax cuts spent immediately
       distortionary taxes → labor-supply effects
       → fiscal multiplier > 0 in most calibrations
```

The **fiscal multiplier** — the ratio of the change in output to the change in government spending — is the key empirical object for fiscal-policy analysis. Under standard NK calibrations with non-Ricardian features (liquidity constraints, sticky prices, monetary policy at the ZLB), the multiplier is typically below one in normal times but can rise materially above one at the ZLB (Christiano-Eichenbaum-Rebelo, and related ZLB literature). Under Ricardian equivalence the multiplier collapses to the production-side effect (e.g., higher government spending crowds out private consumption one-for-one under utility additively-separable in `c` and `g`); the L1-exam-depth treatment of the multiplier sits in sibling [`ec-monetary-fiscal-policy-mechanics-l1`](./ec-monetary-fiscal-policy-mechanics-l1.md). **Source:** Romer (2019) Ch.13 pp.690-708.

## Definition

The **government intertemporal budget constraint** in present-value form is: **Source:** Romer (2019) pp.660-708.

```
B_0  =  ∑_{t ≥ 0}  (1+r)^(−t) · s_t                  [IBC]
        where  s_t  =  T_t − G_t  (primary surplus)
```

In differential / flow form, the debt evolution is: **Source:** Romer (2019) pp.660-708.

```
Ḃ_t  =  r · B_t  +  G_t  −  T_t  =  r · B_t  −  s_t           [debt dynamic]
Ḃ_t / Y_t  =  (r − γ) · (B_t / Y_t)  −  s_t / Y_t              [debt-to-GDP]
              where γ = GDP growth rate
```

Debt-to-GDP is stable if the primary surplus covers the interest-rate-vs-growth wedge `(r − γ) · (B/Y)`. The empirical observation that `r < γ` in many advanced economies (post-2010, "secular stagnation") means small primary surpluses suffice for debt sustainability — and may make rolling debt safer than recurring taxation. **Source:** Romer (2019) Ch.13 pp.660-690.

The **Ricardian equivalence theorem** (Barro 1974) states that under (a) infinite horizon Ramsey households, (b) lump-sum taxes, and (c) perfect capital markets, the timing of taxes (tax vs bond financing) is irrelevant for aggregate consumption and output. The household internalizes the IBC: a bond-financed tax cut today implies higher taxes later with present value equal to the cut, so household lifetime wealth is unchanged and consumption does not respond. Aggregate saving increases by exactly the amount of the deficit, and there is no aggregate-demand effect. **Source:** Romer (2019) Ch.13 pp.670-690.

The **fiscal multiplier** is the impulse-response coefficient `dY_t / dG_t`: **Source:** Romer (2019) pp.660-708.

```
multiplier  =  dY / dG                              [fiscal multiplier]
            ≈ 1 / (1 − MPC)        (textbook Keynesian; no Ricardian offset)
            ≈ 0                    (full Ricardian equivalence; lump-sum)
            > 1                    (ZLB + sticky prices; Eggertsson-Krugman)
```

The empirical literature uses event studies (military buildups, post-crisis stimulus packages), VAR identification, and DSGE structural estimation; consensus point estimates remain dispersed, with substantial uncertainty even within each methodological tradition. The L1-exam-depth multiplier mechanics — closed-form MPC-and-tax-rate parameterization — are treated in sibling `ec-monetary-fiscal-policy-mechanics-l1`. **Source:** Romer (2019) Ch.13 pp.690-708.

## Mathematical Reasoning

The **Ricardian-equivalence derivation** follows from the Ramsey household IBC. The household's lifetime budget constraint with proportional taxation is `∑ (1+r)^(−t) · c_t = ∑ (1+r)^(−t) · (y_t − T_t) + a_0`. Substituting the government IBC `∑ (1+r)^(−t) · T_t = ∑ (1+r)^(−t) · G_t + B_0` gives `∑ (1+r)^(−t) · c_t = ∑ (1+r)^(−t) · (y_t − G_t) + (a_0 − B_0)`. Net household wealth `a_0 − B_0` is unaffected by reshuffling tax timing (both `T` and `B` rescale by the same amount), so consumption is independent of the timing of taxes. The result fails when any of the three conditions is relaxed: finite horizons (OLG; younger households see less of the future tax burden), distortionary taxes (labor-supply margin), or liquidity constraints (households unable to borrow against future income consume the tax cut). **Source:** Romer (2019) Ch.13 pp.670-690.

The **debt-sustainability calculus** uses the differential equation `ḋ_t = (r − γ) · d_t − σ_t` where `d_t = B_t / Y_t` and `σ_t = s_t / Y_t` is the primary surplus ratio. The steady-state debt-to-GDP ratio (under constant surplus ratio `σ̄`) is `d* = σ̄ / (γ − r)` when `γ > r`. When `r > γ`, the steady state is unstable: positive debt accumulates without bound unless surpluses grow faster than interest. The empirical fact that many advanced economies have run `r < γ` post-2010 means the strict debt-sustainability constraint has been less binding than under the old `r > γ` regime, which is the structural basis for Blanchard's (2019) "public debt and low interest rates" argument. **Source:** Romer (2019) Ch.13 pp.690-708.

### FTPL — Supporting Depth (per v10 DEC-3 Boundary)

The **Fiscal Theory of the Price Level (FTPL)** — Cochrane (2023) supporting depth, per the v10 plan's resolved DEC-3 boundary — is an alternative regime in which the price level `P_t` adjusts to satisfy `B_0 / P_0 = ∑ (1+r)^(−t) · s_t`. Under this view, the government does NOT need to back its debt with future surpluses; instead, the equilibrium price level responds to the ratio of nominal debt to expected real surpluses. FTPL provides an alternative explanation for inflation — fiscal regime change rather than monetary-policy regime change — and has been used to interpret the post-2020 inflation surge as a fiscal phenomenon. Per the v10 boundary, FTPL is treated as supporting depth only within this card; a future plan can elevate Cochrane to primary anchor with its own dedicated card. **Source:** Cochrane (2023) Ch.1-3 pp.1-100; Romer (2019) Ch.13 §13.6 pp.700-708.

## See Also

- [`ec-ramsey-cass-koopmans-savings`](./ec-ramsey-cass-koopmans-savings.md) — Ramsey household framework underlying Ricardian equivalence
- [`ec-monetary-policy-and-inflation`](./ec-monetary-policy-and-inflation.md) — monetary-policy complement; fiscal-monetary policy mix is determined jointly
- [`ec-monetary-fiscal-policy-mechanics-l1`](./ec-monetary-fiscal-policy-mechanics-l1.md) — CFA L1 mechanical policy-transmission framework

## Escalate to Raw When

The full derivation of Ricardian equivalence, the conditions for its failure, and the empirical literature on the fiscal multiplier (Ramey-Zubairy state-dependent multipliers, Auerbach-Gorodnichenko VAR estimates) are in Romer Ch.13 pp.670-708. The Blanchard-style debt-sustainability analysis under `r < γ` (Blanchard 2019 AEA presidential address) is in Romer Ch.13 §13.5 pp.690-700 and broader public-finance literature. The FTPL theory in full depth — Cochrane (2023) book, Sims-Woodford original papers, the FTPL-vs-monetary-dominance regime classification — is in the Cochrane supporting source; per v10 boundary, this card cites Cochrane as supporting and a future plan can promote FTPL to a primary anchor with its own card. **Source:** Romer (2019) pp.660-708.
