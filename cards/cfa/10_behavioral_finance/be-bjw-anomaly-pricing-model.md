---
schema_version: "cacg.v0"
id: "be-bjw-anomaly-pricing-model"
title: "BJW Single-Preference Anomaly Pricing Model"
reading_id: "10_behavioral_finance"
summary: "A single prospect-theory preference specification, fed empirical volatility, skewness, gain overhang, and beta of the typical stock in each anomaly decile via a GH skewed-t return distribution, generates a broad cross-section of anomaly predictions from one calibrated model."
tags: ["behavioral-finance", "asset-pricing", "anomalies", "calibration"]
citations:
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p023:0029"
    chunk_hash: "42ff917903ddf9a8485a770ef5ab150bd4177b8ecb1477c3fc6f4106ace82e85"
    page_range: [23, 23]
    quote: "To construct the set of anomalies, we start with 10 anomalies drawn from Stambaugh, Yu, and Yuan (2012)."
    edge_type: "defines"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p016:0019"
    chunk_hash: "4ff2114fb200cfdb2c19feeaff9c551ec222922711ba2222ab9f37bc4066d968"
    page_range: [16, 16]
    quote: "to match the empirical volatility and skewness of asset"
    edge_type: "supports"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p015:0018"
    chunk_hash: "55756a7fdfbd6a5b04b6802f0dda6748eb5262697b0bb9c531d8f8191115e256"
    page_range: [15, 15]
    quote: "One distribution that is increasingly seen as a superior way of modeling skewness and fat tails in asset returns is the “generalized hyperbolic (GH) skewed t” distribution, and we adopt it here."
    edge_type: "supports"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p032:0042"
    chunk_hash: "c59b8fb9f8fc1644e01042f9adb3f9a6f24c73e7b85b572387e13043f13cd9e5"
    page_range: [33, 33]
    quote: "Our preference parameter values are therefore24 (α, δ, λ) = (0.7, 0.65, 1.5)."
    edge_type: "supports"
card_hash: "0b1c176289712745f6ab8afad5ae289bffaaa644f3bcf9a5a44a6128aaa44fa7"
---
# BJW Single-Preference Anomaly Pricing Model

## Intuition

The striking feature of the Barberis-Jin-Wang model is parsimony: one fixed preference specification, calibrated to experimental and field data with no knowledge of any anomaly, is used to price the cross-section. To test a given anomaly the authors sort stocks into deciles on the anomaly characteristic, measure four empirical quantities for the *typical* stock in each decile (return volatility, return skewness, capital gain overhang, and beta), feed those into the model, and read off the predicted expected return. The same procedure, with the same preference parameters, is run for all 23 anomalies.
**Source:** Barberis, Jin & Wang (2021) §III pp.22-24.

Because a single model with a single preference vector confronts 23 separate anomalies, the exercise is far more disciplined than a factor model fitted ex post. The Carhart four-factor model was built with full knowledge of size, value, and momentum; the BJW model was not designed with any anomaly in mind, yet it matches the four-factor model's average absolute pricing error. The model's power comes from feeding it different empirical characteristics for each decile -- the preferences never change, only the inputs do.
**Source:** Barberis, Jin & Wang (2021) §IV.D pp.42.

To capture skewness accurately the authors model returns with a generalized hyperbolic (GH) skewed-`t` distribution, which admits one heavy and one semi-heavy tail. Its four parameters are pinned down by the empirical volatility and skewness (plus a fixed degrees-of-freedom `nu`), leaving the location parameter `mu_i` to be solved for by market clearing -- it is `mu_i`, and hence the expected return, that the model determines in equilibrium.
**Source:** Barberis, Jin & Wang (2021) §II.A, §III pp.14-15.

## Definition

**Anomaly decile construction** sorts all NYSE/NASDAQ/Amex stocks monthly on the relevant anomaly characteristic into 10 deciles; the model treats 100 identical stocks per decile in an `N = 1,000`-stock economy, each carrying the empirical volatility, skewness, gain overhang, and beta of the typical stock in that decile.
**Source:** Barberis, Jin & Wang (2021) §III, §III.A pp.23-24, 28.

**GH skewed-t return distribution** is the four-parameter (`mu_i, S_i, zeta_i, nu`) family used for marginal asset returns; `mu_i` is location, `S_i` dispersion, `zeta_i` asymmetry, `nu` tail-heaviness. It is preferred to log-normal and skew-normal because those cannot match high skewness while leaving the mean free for equilibrium determination.
**Source:** Barberis, Jin & Wang (2021) §II.A pp.14-15.

**The calibrated preference vector** is `(alpha, delta, lambda) = (0.7, 0.65, 1.5)` with scaled risk aversion and prospect-theory weight `(gamma-hat, b-hat) = (0.6, 0.6)`, chosen from experimental meta-analyses and a target 6% equity premium with realistic underdiversification, not from anomaly data.
**Source:** Barberis, Jin & Wang (2021) §III.A pp.30-32.

## Mathematical Reasoning

For the GH skewed-`t` the first three moments are closed form,

```
  E(R_i)    = mu_i + (nu/(nu-2)) * zeta_i
  Var(R_i)  = (nu/(nu-2)) * S_i + (2 nu^2 / ((nu-2)^2 (nu-4))) * zeta_i^2
  Skew(R_i) = ... function of (zeta_i, S_i, nu) ...
```

The procedure inverts the variance and skewness formulas: with `nu = 7.5` fixed, the empirical `sigma_i` and skewness of the decile's typical stock are placed on the left-hand sides of the `Std(R_i)` and `Skew(R_i)` equations, which are solved for the two unknowns `S_i` and `zeta_i`. Beta `beta_i` and gain overhang `g_i` are set to their empirical decile values; `sigma_M = 0.25` and gross `R_f = 1`.
**Source:** Barberis, Jin & Wang (2021) §III.A pp.15, 29-30.

The expected return is then NOT an input but the equilibrium output: one searches for the location parameter `mu_i` such that the (rescaled) one-dimensional objective in equation (20) clears the market for asset `i`, after which `E(R_i) = mu_i + (nu/(nu-2)) zeta_i` gives the predicted average return. This is analogous to CAPM practice -- there a second moment (beta) is estimated and used to predict the mean; here second and third moments (beta, volatility, skewness) plus the overhang are estimated and used to predict the mean.
**Source:** Barberis, Jin & Wang (2021) §II.A pp.15.

The 23 anomalies are not cherry-picked: 10 are drawn from Stambaugh, Yu, and Yuan (2012), 12 from the 97 studied by McLean and Pontiff (2016), and one suggested by a referee. The model's verdict on each follows mechanically from the sign and magnitude of the model-predicted alpha spread `alpha_m(10) - alpha_m(1)` between the extreme deciles.
**Source:** Barberis, Jin & Wang (2021) §III pp.22-23.

## See Also

- [be-prospect-theory-three-characteristic-pricing](./be-prospect-theory-three-characteristic-pricing.md#intuition) -- the three characteristics that drive each decile's predicted return.
- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) -- inventory of cross-sectional anomalies, including those tested here.
- [be-prospect-theory-asset-pricing](./be-prospect-theory-asset-pricing.md#intuition) -- the prospect-theory pricing research program this model advances.
- [be-bjw-bounded-rationality-equilibrium](./be-bjw-bounded-rationality-equilibrium.md#intuition) -- the equilibrium concept that closes the model and lets `mu_i` be solved.
- [be-prospect-theory-anomaly-verdict](./be-prospect-theory-anomaly-verdict.md#intuition) -- the resulting scorecard of which anomalies it explains.

## Escalate to Raw When

- You need the full GH skewed-`t` density and the exact moment formulas with the Bessel-function normalizing constant (equation 12, pp.14-15).
- You need the precise calibration sources (Walasek et al., Chapman et al., Booij et al., Calvet-Campbell-Sodini) and the `(gamma-hat, b-hat)` selection logic (pp.30-32).
- You need the exact list of 23 anomalies and their predictor variables, or the alternative Novy-Marx-Velikov set used for robustness (Table I; pp.22-23, 46-47).
