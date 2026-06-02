---
schema_version: "cacg.v0"
id: "be-bjw-bounded-rationality-equilibrium"
title: "BJW Bounded-Rationality Heterogeneous-Holdings Equilibrium"
reading_id: "10_behavioral_finance"
summary: "With prospect-theory preferences a homogeneous-holdings equilibrium fails to exist; BJW close the model with a bounded-rationality assumption (each investor treats other holdings as market weights), reducing the problem to one dimension and yielding two market-clearing maxima per asset."
tags: ["behavioral-finance", "equilibrium", "bounded-rationality", "market-clearing"]
citations:
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p018:0022"
    chunk_hash: "6da4533441bcfb7488584fe79a598c30515c2d597068faa5c168843b3b36163e"
    page_range: [18, 18]
    quote: "An equilibrium in which investors have identical holdings for all assets therefore does not exist."
    edge_type: "defines"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p018:0023"
    chunk_hash: "caa61ac7f8dc1d5ab6f290ee7527e26ac57f855a0dfd55eda3f7556037cbdc02"
    page_range: [19, 19]
    quote: "to asset i that maximizes the objective function in (9), an investor assumes that his holdings of the other"
    edge_type: "supports"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p020:0025"
    chunk_hash: "95418fb66e92756b5791fc95033255b801acdae668dd63688f46788f0c6e24f9"
    page_range: [20, 20]
    quote: "both of which are nonnegative. These maxima straddle the market supply"
    edge_type: "supports"
card_hash: "9eb6dc066ebb0d02f2e5d01afb6141a8a7e46af351773523084f9d85b45ffc90"
---
# BJW Bounded-Rationality Heterogeneous-Holdings Equilibrium

## Intuition

The model's structure is simple, but solving for equilibrium prices is not. All investors are identical in preferences, wealth, and prior gains, so in an Expected Utility world they would hold identical portfolios -- each the market portfolio. With prospect-theory preferences, however, that natural equilibrium does not exist: as the planner lowers an asset's location parameter `mu_1` to clear the market, the maximizing allocation `Theta_1*` jumps discontinuously from above the market supply to below it, so no `mu_1` makes the optimum equal to market supply.
**Source:** Barberis, Jin & Wang (2021) §II.B pp.16-17.

A fully rational heterogeneous-holdings equilibrium (investors split across multiple global maxima) would clear the market in principle, but with `N` on the order of 1,000 stocks it is computationally infeasible -- checking `100^N` candidate location vectors and solving an `N`-dimensional optimization at each is impossible. The authors break this logjam with a mild bounded-rationality assumption: when choosing his allocation to asset `i`, an investor assumes his holdings of the other `N-1` assets equal the market supply. This is psychologically plausible (not just a trick) and converts the multivariate problem into a one-dimensional one per asset.
**Source:** Barberis, Jin & Wang (2021) §II.B pp.17-18.

Once the problem is one-dimensional, it is easy to check whether the objective has one or two global maxima. The authors find that, for assets with a positive gain overhang and high expected return, the objective has two maxima straddling the market supply: a lower optimum `Theta_i*` (lock in prior gains by selling some holdings) and an upper optimum `Theta_i**` (benefit from the high expected return by buying more). Assigning most investors to the lower optimum and a few to the upper one clears the market. These two maxima exist precisely because prospect theory embeds risk-seeking (over moderate-probability losses and low-probability gains); under uniform risk aversion the objective would be concave with a single maximum.
**Source:** Barberis, Jin & Wang (2021) §II.B pp.18-20.

## Definition

**Homogeneous-holdings (full-rationality) equilibrium** is the standard Expected-Utility structure in which all identical investors hold the market supply `Theta_{M,i}` of each asset. The BJW model demonstrates this equilibrium does not exist for a wide range of prospect-theory parameters.
**Source:** Barberis, Jin & Wang (2021) §II.B pp.16-17.

**Bounded-rationality assumption** is that, when solving for his allocation `Theta_i` to asset `i`, each investor takes his holdings of every other asset `j != i` to equal the market supply `Theta_{M,j}`. This need not be exactly true but has negligible impact on predictions, and converts equation (9) into the univariate objective (16).
**Source:** Barberis, Jin & Wang (2021) §II.B pp.18.

**Bounded-rationality heterogeneous-holdings equilibrium** is a location vector `(mu_1, ..., mu_N)` such that, for each asset, the univariate objective either has a unique maximum at `Theta_i = Theta_{M,i}` or two maxima straddling `Theta_{M,i}`, so the market clears by splitting investors between the lower optimum `Theta_i*` and upper optimum `Theta_i**`.
**Source:** Barberis, Jin & Wang (2021) §II.B pp.18-19.

## Mathematical Reasoning

Imposing `Theta_j = Theta_{M,j}` for all `j != i` and dropping terms constant in `Theta_i`, the per-asset objective (up to a linear transformation) is

```
  Theta_i (mu_i + (nu/(nu-2)) zeta_i - R_f)
    - (gamma-hat/2)(Theta_i^2 sigma_i^2 + 2 Theta_i sum_{j!=i} Theta_{M,j} sigma_ij)
    - lambda * b-hat * INT_{loss}  (Theta_i(R_f - R_i) - Theta_{i,-1} g_i)^alpha dw(P(R_i))
    -          b-hat * INT_{gain}  (Theta_i(R_i - R_f) + Theta_{i,-1} g_i)^alpha dw(1 - P(R_i))
```

with `gamma-hat = gamma W0` and `b-hat = b W0^{alpha-1}`. The first row is the mean-variance piece; the loss and gain integrals are the narrowly-framed prospect-theory value of asset `i`.
**Source:** Barberis, Jin & Wang (2021) §II.B pp.18.

Equilibrium requires, for each `i`, that this function have either a unique global maximum at the market supply or two global maxima `Theta_i* < Theta_{M,i} < Theta_i**`, both nonnegative. Market clearing then assigns a fraction of investors to `Theta_i*` and the rest to `Theta_i**` so the value-weighted holdings equal supply. Because `Theta_i*` is always much closer to `Theta_{M,i}` than `Theta_i**` is, the vast majority of investors sit at the lower optimum and a small minority hold a concentrated position at the upper one. The result is the average investor holding a diversified core plus a few concentrated bets -- an underdiversification level comparable to actual household portfolios.
**Source:** Barberis, Jin & Wang (2021) §II.B pp.19-20.

The two maxima are not a knife-edge artifact: for a wide range of parameters, whenever holdings are not identical a two-maximum structure exists, and the asset's expected return adjusts until both maxima have equal height. No investor shorts in the calibrated application -- the lower optimum is always nonnegative.
**Source:** Barberis, Jin & Wang (2021) §II.B, §III.B pp.20-21, 33-34.

## See Also

- [be-noise-trader-equilibrium](./be-noise-trader-equilibrium.md#intuition) -- an alternative equilibrium with non-rational demand and bounded arbitrage.
- [be-limits-of-arbitrage](./be-limits-of-arbitrage.md#intuition) -- why arbitrageurs do not eliminate the mispricing these preferences create.
- [be-bjw-anomaly-pricing-model](./be-bjw-anomaly-pricing-model.md#mathematical-reasoning) -- uses this equilibrium to solve for the location parameter and expected return.
- [be-prospect-theory-three-characteristic-pricing](./be-prospect-theory-three-characteristic-pricing.md#mathematical-reasoning) -- the preference objective whose risk-seeking region produces the two maxima.

## Escalate to Raw When

- You need the exact rescaled equilibrium equations (20)-(22) with `theta_i = Theta_i/Theta_{M,R}` and the `gamma-hat, b-hat` rescaling (p.21).
- You need the numerical illustration of unique vs. two maxima (momentum deciles 1 and 10, Figures 3-4) with the actual `mu_i`, `theta*`, `theta**` values (pp.32-34).
- You need why `Theta_1*` jumps discontinuously as `mu_1` falls, tied to the powers of `Theta_1` introduced when `b > 0` (footnote 13, p.17).
