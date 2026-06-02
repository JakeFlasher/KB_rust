---
schema_version: "cacg.v0"
id: "pm-factor-models-intuition"
title: "Factor Models — Intuition"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Factor Models — Intuition: extending the single-factor CAPM into a multifactor return-decomposition intuition — what a factor is, why a single factor is restrictive, and how additional factors enter as siblings of the market factor in the expected-return generator"
tags: ["portfolio-management", "factor-models", "multifactor"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3171:4729"
    chunk_hash: "7c38514023177ca2176eb8888c1d5313ad1bfe8461e42c7e55c3c4d5e63e8bff"
    page_range: [3171, 3172]
    quote: "A multi-factor model allows more than one variable to be considered in estimating returns and can be built using different kinds of factors"
    edge_type: "supports"
card_hash: "7bdee566f0086867bfb075fbe3a1bfb1bda7fad55e5bf2edea6d4934795c06d7"
---
# Factor Models — Intuition

## Intuition

CAPM uses one factor — the market portfolio — to price every asset.
Empirically, single-factor pricing leaves systematic patterns in
the residuals: portfolios sorted on size, value, or momentum show
average returns that the market beta alone does not explain. A
multifactor model preserves the structure of CAPM (expected return
is a linear combination of factor exposures times factor risk
premia) but allows multiple factors to enter the same equation. The
intuition is that the market factor captures the broadest co-
movement; additional factors capture systematic patterns that
remain after the market move is netted out. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.560-572.

```
<!-- primitive: factor-exposure-ladder source: _diagram_primitives.md -->
E[R contribution]
   |
   |  +-----------------+    alpha
   |  | residual alpha  |     <-- manager skill / model error
   |  +-----------------+
   |  | quality factor  |
   |  +-----------------+
   |  | momentum factor |
   |  +-----------------+
   |  | value factor    |
   |  +-----------------+
   |  | size factor     |
   |  +-----------------+
   |  | market beta * MRP |  <-- systematic risk premium
   |  +-----------------+
   |  | Rf              |     <-- risk-free baseline
   |  +-----------------+
   |
   +-> stacked decomposition (single portfolio)
```

The investor's view of factors is functional: each factor is a
labeled bundle of co-movement risk that earns a risk premium for
being borne. The risk premium exists because the factor is
systematic — diversifiable into zero only if you also short the
factor's loading. The single-factor restriction in CAPM is what
breaks empirically; the multifactor extension reflects that
documented co-movement patterns extend beyond a single market
direction. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.

## Definition

A factor is a source of systematic return co-movement across
assets. For each factor `j`, every asset `i` has a loading
`beta_(i,j)` that measures the asset's sensitivity to that
factor's return realization. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.560-572.

The multifactor expected-return relation generalizes the CAPM
single-factor SML to a sum of factor risk premia weighted by the
asset's loadings on each factor. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.560-572.

```
E[r_i] = R_f + sum_j  beta_(i,j) · lambda_j
```

Here `lambda_j` is the risk premium per unit of exposure to factor
`j` (the analog of the market risk premium `E[r_M] - R_f` in
CAPM). The CAPM is the special case `j = 1` where the only factor
is the market and `lambda_1 = E[r_M] - R_f`. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.560-572.

The realized-return decomposition writes the asset return as the
sum of factor contributions plus a residual idiosyncratic term.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.

```
r_i = R_f + sum_j  beta_(i,j) · F_j  +  epsilon_i
```

`F_j` is the realized return of factor `j` net of the risk-free
rate, and `epsilon_i` is the residual specific to asset `i` and
uncorrelated with the factors by construction. The key contrast
with CAPM is that the model accommodates multiple systematic
return sources rather than collapsing them into a single market
beta. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.

## Mathematical Reasoning

The L1 framing presents factor models at intuition level rather
than as a formal pricing theorem. The key mathematical claim is
that the factor structure explains a substantial fraction of the
realized return variance across many securities, leaving
idiosyncratic residuals that are largely diversifiable. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.560-572.

Variance decomposition under a multifactor model parallels the
single-factor case but with a covariance term across factors.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.

```
var(r_i) = sum_j sum_k  beta_(i,j) · beta_(i,k) · cov(F_j, F_k)
        + var(epsilon_i)
```

When factors are constructed to be approximately uncorrelated (the
typical Fama-French style construction does this through long-
short factor-mimicking portfolios), the cross-factor covariance
terms drop and the variance decomposition simplifies to a sum of
factor-loading-squared terms plus residual variance. The
idiosyncratic residual remains diversifiable in a broad portfolio
just as in the CAPM case. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.560-572.

The empirical literature documents three or four factors as the
core extension at L1 framing: size (small-cap minus large-cap),
value (high book-to-market minus low book-to-market), momentum
(past-winners minus past-losers), and sometimes profitability /
quality. Each factor is a long-short portfolio whose realized
return loads onto a documented anomaly. The factor model frames
the anomaly as a priced systematic exposure rather than as
mispricing. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.

A specific implication for portfolio construction: if the
investor's portfolio loads on size and value factors with positive
betas, the portfolio earns the size and value risk premia in
expectation, but bears the size and value factor risks. A
"diversified" portfolio in single-factor CAPM terms (matching
market beta) may carry concentrated factor exposures in
multifactor terms. The multifactor frame surfaces these exposures
explicitly. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.

The boundary with the AC-42 extension card is that this card
treats factor models as the L1-level core intuition (single-factor
to multifactor stepping stone). Theoretical derivations of
multifactor pricing — APT, Cochrane SDF — and the equilibrium
arguments justifying which factors should be priced live in the
extension card. The card here remains within Vol.6 R50 framing
without invoking SDF or APT proof machinery. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.560-572.

## See Also

- [`pm-beta-and-factor-exposure.md`](pm-beta-and-factor-exposure.md) — single-factor beta as the special case `j = 1` of the multifactor model
- [`pm-capm-and-sml.md`](pm-capm-and-sml.md) — the single-factor pricing equation that this card generalizes
- [`pm-systematic-vs-idiosyncratic-risk.md`](pm-systematic-vs-idiosyncratic-risk.md) — variance decomposition that the multifactor model refines

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R50 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.560-572.

- The arbitrage-pricing-theory (APT) derivation that justifies
  multifactor pricing without the strict CAPM assumptions —
  Vol.6 R50 introduces APT at intuition level; the deeper
  derivation lives in the Cochrane-anchored extension card.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.
- Specific factor-construction methodology (Fama-French 3-factor /
  5-factor; Carhart momentum; Q-factor model) — Vol.6 R50 lists
  the major models; security-level construction belongs in
  future-05. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.
- Risk-premium forecasting and time-variation in `lambda_j` —
  these are active-management questions deferred to the AC-42
  Pedersen-anchored card. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.560-572.
