---
schema_version: "cacg.v0"
id: "pm-multifactor-asset-pricing-intuition"
title: "Multifactor Asset Pricing — Intuition"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Multifactor Asset Pricing — Intuition: extending the single-factor CAPM into a linear multifactor pricing intuition — what makes a \"factor\", why expected return is linear in factor exposures, and how the multibeta representation connects to mean-variance frontier geometry"
tags: ["portfolio-management", "multifactor", "asset-pricing"]
citations:
  - source_id: "pm_cochrane_2005_asset_pricing_revised"
    chunk_id: "pm_cochrane_2005_asset_pricing_revised:p097:0111"
    chunk_hash: "fdc5c6685baf0fb5b3f68ee976c5b13bfd1453e850048304822cdaa99301a3b0"
    page_range: [97, 98]
    quote: "βi, a is interpreted as the amount of exposure of asset i to factor a risks, and λa is interpreted as the price of such risk exposure."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3832:5828"
    chunk_hash: "6880f6dd6335640d98bb23c16f1332a72427fbfdc57266fd56512c49ef2b1795"
    page_range: [3832, 3832]
    quote: "of the expected return of a security given certain parameters and estimates of the values of the independent variables in the model."
    edge_type: "supports"
card_hash: "9a718cd6e04ce838aa84af42b1d891a1666b38da0d979e053a56d08daa8dd698"
---
# Multifactor Asset Pricing — Intuition

## Intuition

CAPM uses the market portfolio as the single risk factor that prices
every asset. The multifactor extension preserves the same algebraic
structure — expected return is linear in risk-exposure coefficients
times factor risk premia — but allows multiple factors to enter the
pricing equation. The motivation is empirical: portfolios sorted on
size, value, momentum, profitability, and investment patterns
display systematic average-return differences that the single
market beta does not explain. Each documented pattern can be
reframed as exposure to an additional priced factor in the same
linear pricing structure. **Source:** Cochrane (2005)
pp.77-97.

```
              factor pricing structure
              =======================
              E[R_i] = gamma + sum_j  beta_(i,j) · lambda_j

              gamma          = intercept (Rf when a riskless asset
                                exists)
              beta_(i,j)     = regression coefficient of asset i's
                                return on factor j's return
              lambda_j       = factor risk premium per unit of
                                exposure to factor j

              special case: j = 1, factor = market portfolio M:
                E[R_i] = Rf + beta_i · (E[R_M] - Rf)        [CAPM]

              extensions:    Fama-French 3-factor (market, size, value),
                             Carhart 4-factor (+ momentum),
                             Fama-French 5-factor (+ profitability,
                              investment), ICAPM (state variables),
                             APT (statistical factors)
```

The key distinction Cochrane emphasizes is that betas are regression
coefficients, not asset characteristics. A small-cap stock has a
high size-factor beta because its returns covary with the size
factor's returns; not because it carries the label "small". Two
otherwise-different assets with the same factor-loading vector earn
the same expected return under the model. This regression-
coefficient discipline is what prevents trivial reframings of
characteristic-based observed patterns from looking like genuine
pricing structure. **Source:** Cochrane (2005) pp.77-97.

## Definition

The multifactor expected-return relation extends the CAPM to `J`
factors. **Source:** Cochrane (2005) pp.77-97.

```
E[R_i] = gamma + sum_(j=1..J)  beta_(i,j) · lambda_j
```

Each `beta_(i,j)` is the multiple-regression coefficient from
projecting asset `i`'s realized excess return on the time series
of all `J` factor realizations. **Source:** Cochrane (2005)
pp.77-97.

```
R_i(time) - Rf = a_i + sum_(j=1..J)  beta_(i,j) · F_j(time) + epsilon_i(time)
```

The `lambda_j` are the prices of factor risk — the expected
return per unit of exposure to factor `j` — and they are common
across all assets in the cross-section. The `gamma` intercept
equals `Rf` whenever a riskless asset exists; absent a riskless
asset, `gamma` is the zero-beta-portfolio expected return.
**Source:** Cochrane (2005) pp.77-97.

When the factors are themselves traded excess returns, the
no-arbitrage restriction implies that the price of factor `j`'s
own risk equals its own expected excess return: `lambda_j =
E[F_j]`. **Source:** Cochrane (2005) pp.77-97.

## Mathematical Reasoning

The multifactor model has two complementary representations that
Cochrane shows are mathematically equivalent: the expected-return-
beta representation and the linear-discount-factor representation.
**Source:** Cochrane (2005) pp.99-110.

```
expected-return-beta:     E[R_i] = gamma + sum_j  beta_(i,j) · lambda_j
linear-discount-factor:   m = a + b' · f
                          E[m · R_i] = price of asset i

equivalence (Cochrane Ch.6):
  if m = a + b' · f prices all assets, then there exist (gamma, lambda)
  such that E[R_i] = gamma + sum_j beta_(i,j) · lambda_j prices all
  assets, and vice versa.
```

The duality is conceptually decisive: the multifactor pricing
structure is equivalent to the existence of a discount factor
that is a linear function of the factors. The discount-factor
representation lives in the next sibling card; here the focus is
the beta-pricing form. **Source:** Cochrane (2005) pp.99-110.

The choice of factors is the substantive question. Cochrane Ch.9
catalogs three derivation paths that produce candidate factors
from economic theory rather than from statistical fishing.
**Source:** Cochrane (2005) pp.149-183.

The CAPM derivation produces the market portfolio as the single
factor under the assumption that consumption is perfectly correlated
with market wealth. The ICAPM derivation produces additional state-
variable factors when investment opportunities vary stochastically
(time-varying expected returns, stochastic interest rates, stochastic
volatility). The APT derivation produces factors statistically as
the few common-variance directions in a large cross-section, under
the no-arbitrage assumption alone. **Source:** Cochrane (2005)
pp.149-183.

A specific implication for portfolio framing: a "well-diversified"
portfolio in the CAPM sense (matching market beta) may carry
concentrated exposures along non-market factors. A small-cap value
portfolio held for risk premium load on the size and value factors
explicitly accepts those exposures. The multifactor frame surfaces
this exposure structure in a way the single-factor CAPM cannot.
**Source:** Cochrane (2005) pp.77-97.

The mean-variance-frontier connection extends the CAPM tangency
geometry into a higher-dimensional analog. The mean-variance
frontier with `J` factors is spanned by the riskless asset plus
`J` factor-mimicking portfolios; any factor risk premium `lambda_j`
equals the expected excess return on a properly normalized
factor-mimicking portfolio. The single-factor CML reappears as the
`J = 1` slice of this higher-dimensional frontier. **Source:**
Cochrane (2005) pp.77-97.

The boundary between this card and the L1-core
`pm-factor-models-intuition.md` sibling is depth-of-derivation. The
core card states the linear multifactor structure as an empirical
extension of CAPM without justifying its derivation. This
extension card adds the discount-factor / beta-representation
duality, the regression-coefficient (not characteristic) discipline
on betas, and the three derivation paths (CAPM, ICAPM, APT) that
produce factors from economic primitives rather than from data
mining. **Source:** Cochrane (2005) pp.99-183.

## See Also

- [`pm-factor-models-intuition.md`](pm-factor-models-intuition.md) — L1-core multifactor stepping stone that this card extends with formal derivations
- [`pm-capm-and-sml.md`](pm-capm-and-sml.md) — single-factor `J = 1` special case of the multifactor pricing equation
- [`pm-stochastic-discount-factor-intuition.md`](pm-stochastic-discount-factor-intuition.md) — discount-factor representation of the same pricing structure (Cochrane Ch.6 equivalence)

## Escalate to Raw When

Open Cochrane (2005) Ch.5 / Ch.6 / Ch.9 directly when any of the
criteria below applies. **Source:** Cochrane (2005) pp.77-183.

- Mean-variance-frontier construction with the Hansen-Jagannathan
  bound on the discount factor — Cochrane Ch.5 §5.6 develops the
  bound and its implications. **Source:** Cochrane (2005)
  pp.92-97.
- Detailed CAPM / ICAPM / APT derivations from primitives — Ch.9
  works each derivation through the consumption-based first-order
  condition and the assumed factor structure. **Source:** Cochrane
  (2005) pp.149-183.
- The `J`-factor cross-sectional regression machinery and pricing-
  error testing — Cochrane Pt.II (Ch.10-15) covers GMM, Fama-MacBeth,
  time-series regression. These belong to estimation rather than
  intuition. **Source:** Cochrane (2005) pp.185-308.
