---
schema_version: "cacg.v0"
id: "ec-monetary-policy-and-inflation"
title: "Monetary Policy and Inflation"
reading_id: "02_economics"
summary: "Modern monetary policy is parameterized via the Taylor rule i_t = r_n + phi_pi*(pi - pi*) + phi_y*(lnY - lnY_n) with the Taylor principle phi_pi > 1 ensuring real-rate stabilization; ZLB binds when the desired Taylor-rule rate is negative; Barro-Gordon dynamic inconsistency motivates rules vs discretion and conservative-central-banker solutions."
tags: ["economics", "monetary-policy"]
citations:
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p627:0912"
    chunk_hash: "79df4b40bf08a89da71dc242ae2bbe0ca9560bfa13a937a119aa6fa0549e57a5"
    page_range: [627, 628]
    quote: "The first is for the nominal interest rate to rise more than one-for-one with inflation, so that the real rate increases when inflation rises."
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p628:0913"
    chunk_hash: "8e7af753bb5a6fcb9399bdc74792f9e1c765c301dfedeb402e3de362a7ea1ec8"
    page_range: [628, 628]
    quote: "Interest-rate rules of the form in (12.44) and (12.45) are known as Taylor rules."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p870:1218"
    chunk_hash: "d3673f9437ea75688d6440281e99c0c4164bebc09e5eece04045e212b54ba7e8"
    page_range: [870, 871]
    quote: "This is the process whereby a central bank’s interest rate gets transmitted through the economy and ultimately affects the rate of increase of prices"
    edge_type: "supports"
card_hash: "00c75132e4a9a2695024e69316141662616c91e25edc4db76e04b8eefa2912fc"
---
# Monetary Policy and Inflation

## Intuition

Modern central banks set nominal interest rates in response to inflation and output-gap deviations from target — the **Taylor rule** `i_t = ρ + φ_π · π_t + φ_y · ỹ_t` (stated in deviations from steady state). The Taylor principle (`φ_π > 1`) ensures that real interest rates rise when inflation rises, which stabilizes inflation in the NK framework. A passive policy `φ_π < 1` would allow self-fulfilling inflationary expectations and indeterminate equilibria. The Taylor rule has become the empirical benchmark for evaluating central-bank behavior since Taylor's (1993) original calibration for the Federal Reserve, with subsequent literature documenting that Fed behavior shifted from passive in the pre-Volcker era to active under Volcker and Greenspan (estimated inflation-response coefficients well above one in the post-1979 sample). **Source:** Romer (2019) Ch.12 pp.578-620.

```
   Taylor rule and zero-lower-bound dynamics

   nominal interest rate i_t
   ^
   |                        ___ ← unconstrained Taylor rule
   |                    ___/        i_t = ρ + φ_π · π_t + φ_y · ỹ_t
   |                ___/
   |            ___/    actual rate (max with ZLB)
   |        ___/
   |       /
   |      /
   |-----+----------------> π_t (inflation)
   |    |  ZLB-binding region:
   |    |  desired Taylor rate is negative
   |    |  but actual i_t cannot go below zero
   0----+_________________________________
                              (i_t = 0 floor)

   ZLB binds when ρ + φ_π · π + φ_y · ỹ < 0
   → unconventional policy needed (QE, forward guidance)
```

The **zero lower bound (ZLB)** on nominal interest rates is the major operational constraint on modern monetary policy. When the desired Taylor-rule rate is negative (typically following a large negative demand shock that pushes inflation and output below target), the central bank cannot mechanically implement it because nominal rates floor at zero (or slightly below, given the cost of holding cash). The **liquidity trap** at the ZLB motivates unconventional tools: quantitative easing (QE), forward guidance (committing to future low rates), and yield-curve control. The Eggertsson-Krugman (2012) analysis shows that at the ZLB, the standard Taylor-rule prescription fails and fiscal policy becomes the active stabilization tool. **Source:** Romer (2019) Ch.12 pp.620-659.

## Definition

The **Taylor rule** parameterizes the central bank's reaction function as: **Source:** Romer (2019) pp.578-659.

```
i_t  =  ρ  +  π_t*  +  φ_π · (π_t − π_t*)  +  φ_y · ỹ_t       [Taylor rule]
```

where `ρ` is the long-run real interest rate, `π_t*` is the inflation target, `φ_π > 1` is the inflation-response coefficient (Taylor principle), and `φ_y > 0` is the output-gap-response coefficient. The original Taylor (1993) parameterization placed `φ_π` modestly above one and `φ_y` smaller still — values that match early-1990s Federal Reserve behavior under Greenspan. Empirical estimation of these coefficients on FOMC data is a benchmark for evaluating central-bank performance. **Source:** Romer (2019) Ch.12 pp.578-620.

The **Fisher equation** decomposes nominal interest rates into the real rate and expected inflation: **Source:** Romer (2019) pp.578-659.

```
i_t  =  r_t  +  E_t[π_{t+1}]                  [Fisher equation]
```

The central bank can target either the nominal rate (Taylor rule sets `i_t`) or the real rate (`r_t = i_t − E_t π_{t+1}`); under sticky prices, these have different short-run effects. The natural rate `r_t^n` is the real rate that would prevail under flexible prices; the policy rate `r_t` deviates from `r_t^n` in the short run as the central bank responds to shocks. The **monetary-policy gap** `r_t − r_t^n` is the wedge that drives the output gap via the dynamic IS curve. **Source:** Romer (2019) Ch.12 pp.578-620.

The **dynamic-inconsistency problem** (Kydland-Prescott 1977, Barro-Gordon 1983) is the canonical argument for rule-based vs discretionary policy. Define the social-loss function: **Source:** Romer (2019) pp.578-659.

```
L  =  (π − π*)^2  +  λ · (y − y_k)^2          [period loss function]
```

where `y_k > y_n` is the politically-desired output (above natural; e.g., reflecting labor-market distortions). Under **discretion**, the central bank re-optimizes each period, creating an inflation bias: agents anticipate the central bank's incentive to inflate above target to push output toward `y_k`, leading to higher equilibrium inflation with no output gain. Under **commitment** (or a rule), the central bank can promise to maintain `π = π*`, eliminating the inflation bias. **Source:** Romer (2019) Ch.12 pp.620-659.

## Mathematical Reasoning

The **Taylor principle** (`φ_π > 1`) is the necessary condition for a unique stable equilibrium in the NK framework. Substituting the Taylor rule into the dynamic IS and NK Phillips curve yields a system with stable / unstable roots that depend on `φ_π`. The condition `φ_π > 1` ensures that real interest rates `r = i − π` rise when inflation rises (so that an exogenous inflation shock is countered by tighter real-rate conditions). If `φ_π < 1`, real rates fall when inflation rises, amplifying the shock — self-fulfilling inflationary expectations can sustain indeterminate equilibria. Empirical evidence that pre-Volcker Fed had `φ_π < 1` is offered as an explanation for the Great Inflation of the 1970s. **Source:** Romer (2019) Ch.12 pp.580-620.

The **Barro-Gordon solution to the inflation bias** under commitment is derived by minimizing the expected lifetime social loss subject to the Phillips curve constraint. Under discretion, the central bank takes expected inflation as given and chooses actual inflation to minimize the period loss, yielding `π = π* + λ · (y_k − y_n) / κ` — inflation is biased above target by an amount proportional to the output-target gap. Under commitment, the central bank promises `π = π*` regardless of the period's realized shocks; this yields lower average inflation but requires reputation / institutional commitment to be time-consistent. The modern resolution is **conservative central-banking** (a central bank with `λ_CB < λ_society`) which mimics commitment-like behavior under discretion. **Source:** Romer (2019) Ch.12 pp.620-659.

The **ZLB regime** modifies the standard analysis dramatically. When the Taylor-rule prescription is `i_t < 0`, the actual rate floors at zero, the real rate is `−E_t π_{t+1}`, and the central bank loses its conventional stabilization lever. Eggertsson-Krugman (2012) show that at the ZLB, the standard NK comparative statics reverse: positive supply shocks become contractionary (deflationary), and the fiscal multiplier rises above one. The policy implications include **forward guidance** (promising to keep rates low even after the recovery, to engineer a temporary above-target inflation that lowers real rates) and **QE / large-scale asset purchases** (which compress term premia and lower long-term real rates even when short rates are pinned at zero). **Source:** Romer (2019) Ch.12 pp.620-659.

## See Also

- [`ec-nominal-rigidity-and-NK-monetary`](./ec-nominal-rigidity-and-NK-monetary.md) — the NK three-equation system that this card extends with Taylor-rule design + ZLB analysis
- [`ec-monetary-fiscal-policy-mechanics-l1`](./ec-monetary-fiscal-policy-mechanics-l1.md) — CFA L1 mechanical policy-transmission framework for exam-depth treatment
- [`ec-fiscal-policy-and-budget-deficits`](./ec-fiscal-policy-and-budget-deficits.md) — fiscal-policy complement that becomes the active tool at the ZLB

## Escalate to Raw When

The full Barro-Gordon discretion-vs-commitment derivation and the Rogoff (1985) conservative-central-banker solution are in Romer Ch.12 pp.620-650. The Eggertsson-Krugman ZLB analysis, the Werning continuous-time generalization, and the forward-guidance literature (including the "forward-guidance puzzle" — why DSGE models predict implausibly large effects of distant promises) are in Romer Ch.12 §12.7-12.9 pp.640-659. The empirical literature on Taylor-rule estimation (Clarida-Galí-Gertler regressions) is benchmark reading for monetary-policy empirics but out of v10 scope as it requires econometric depth covered in 01 QM. The fiscal complement at the ZLB sits in sibling `ec-fiscal-policy-and-budget-deficits`. **Source:** Romer (2019) pp.578-659.

The institutional design literature — central-bank independence as a commitment device, inflation-targeting frameworks (Bernanke-Mishkin 1997), and the modern toolkit of unconventional monetary policy (large-scale asset purchases, yield-curve control, central-bank digital currencies) — is covered partially in Romer Ch.12 §12.10-12.12 pp.650-659 and more comprehensively in central-bank research literature out of v10 scope. The cross-link to the CFA L1 mechanical policy-transmission framework — money-multiplier mechanics, quantity-equation framing, open-market-operations mechanics — sits in sibling `ec-monetary-fiscal-policy-mechanics-l1`, which re-teaches the L1-exam-depth content while preserving consistency with the Romer-anchored framework here. **Source:** Romer (2019) pp.578-659.
