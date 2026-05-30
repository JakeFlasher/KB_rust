---
schema_version: "cacg.v0"
id: "pa-factor-model-types-and-covariance-decomposition"
title: "Factor-Model Types and the Covariance Decomposition (Euler/MCR)"
reading_id: "15_performance_and_attribution"
summary: "The three return-factor-model types (statistical, macroeconomic, characteristic), the linear decomposition x = a + Bf + e that splits return covariance into common-factor B Cf B' plus idiosyncratic Ce, and the Euler/positive-homogeneity gradient d(w'Cw)/dw = 2Cw that makes marginal risk contributions value-additive across a nonlinear risk budget."
tags: ["factor-model", "covariance-decomposition", "marginal-contribution-to-risk"]
citations:
  - source_id: "pa_connor_goldberg_korajczyk_2010"
    chunk_id: "pa_connor_goldberg_korajczyk_2010:p046:0045"
    chunk_hash: "b8883fee7cda118bf074b0d95313838dc0caca29da6971bb49318bf3aa8ac922"
    page_range: [46, 47]
    quote: "As we explain below, the optimal portfolio weights do follow a linear equation, but are written in units of marginal risk."
    edge_type: "defines"
card_hash: "4bfb0a0679ea51c140779ac618c8f5d70fb73dca87f3b1534e3cf5b998bc08a6"
---
# Factor-Model Types and the Covariance Decomposition (Euler/MCR)

## Intuition

A portfolio's risk is not a fixed sum you ladle out across holdings the way a
cash budget is — Sharpe's point is that risk is *not additive*, so the
"budget" you are allocating is nonlinear (quadratic in the weights). That seems
to doom the appealing idea of a risk budget. The rescue is a calculus fact: even
though the *level* of variance is quadratic, its *gradient* — how variance moves
for a small tilt in a given direction — is linear in the weights. So you can
still hand each asset, desk, or sub-portfolio a coherent share of risk by
talking in marginal contributions rather than levels. Behind this sits a second
clean idea: any return covariance matrix splits into a part driven by a handful
of pervasive factors (common factor risk) and a leftover, near-diagonal part
unique to each name (idiosyncratic risk). The factor models that supply that
split come in three flavours — statistical, macroeconomic, and characteristic —
differing only in where the factors and exposures come from.

**Source:** Connor, Goldberg & Korajczyk (2010) §1.3.7-1.3.8 pp.46-47

## Definition

**The three factor-model types.** Connor, Goldberg & Korajczyk classify
return-factor models as **statistical** (factors and exposures are both extracted
from the return data, e.g. by principal-components / factor analysis),
**macroeconomic** (factors are observable economic series such as inflation or
interest-rate changes, and exposures are estimated betas), and
**characteristic-based** (exposures are observable firm attributes such as
value, size, or yield, and factor returns are inferred). The book calls
statistical models "the most technically difficult of the three classic types
but also the most fundamental."

**The linear factor decomposition.** Writing the vector of asset excess returns
as `x = a + Bf + e`, where `B` is the `n x k` matrix of factor betas, `f` the
random vector of factor returns, and `e` the `n`-vector of asset-specific
returns (`a` chosen so `E[e] = 0`), and defining `B = cov(x, f) Cf^-1` so that
`cov(f, e) = 0`, the return covariance matrix decomposes into a **common-factor**
part and an **idiosyncratic** part.

**Marginal contribution to risk (MCR).** The marginal contribution to risk of a
unit-cost tilt in direction `v` for current portfolio `w` is the directional
derivative of portfolio standard deviation `sigma_w`; setting `v` to the `i`-th unit
vector gives the MCR of asset `i`, which "can be negative."

**Source:** Connor, Goldberg & Korajczyk (2010) §1.3.6 pp.45-46, §4.1.1 p.104, §6.2.2 p.147 (three-type estimation distinctions)

## Mathematical Reasoning

**Covariance decomposition.** Because `f` and `e` are uncorrelated, the return
covariance matrix and portfolio variance split cleanly:

```
   C        =   B Cf B'        +        Ce
 (total)       (common         (idiosyncratic /
                factor risk)    asset-specific risk)

 var(x_w)  =  w' B Cf B' w   +   w' Ce w
```

where `Cf` is the factor covariance matrix and `Ce` is the covariance of
idiosyncratic returns (diagonal under a *strict* factor model). This is an exact
algebraic identity, not an approximation.

**Source:** Connor, Goldberg & Korajczyk (2010) §4.1.1 pp.104-105 (`C = BCf B' + Ce`)

**The Euler / positive-homogeneity gradient.** Portfolio variance `w'Cw` is a
homogeneous-degree-2 form in `w`, so its gradient is linear:

```
   d(w' C w) / dw  =  2 C w.
```

This is the engine behind value-additive risk budgeting: although the risk
constraint `w'Cw = sigmabar^2` is quadratic (nonlinear), the marginal change in
variance per unit of allocation is a *linear* function of the weights. The
directional MCR follows from the same form: `dsigma_w/dv = (1/sigma_w) v'Cw`, the
standard-deviation gradient being the variance gradient scaled by `1/sigma_w`. At a
risk-budget optimum the marginal expected return must be proportional to the
marginal risk, `d(w'mu)/dw = lambda * d(w'Cw)/dw`, so each unit's expected excess
return lines up with its variance contribution. Substituting the factor
decomposition lets the same marginal contribution be read either at the factor
level (`w'BCf B'w`) or the idiosyncratic level (`w'Ce w`), which is why a single
position can carry a large *negative* marginal contribution and still make a net
positive contribution to risk-return optimality.

The book *asserts* the value-additivity result and exhibits the gradient
identities; it does not re-derive Euler's theorem for homogeneous functions from
first principles, and this card asserts at the same level and labels that gap.

**Source:** Connor, Goldberg & Korajczyk (2010) §1.3.6-1.3.8 Eq.(1.24)-(1.27) pp.45-47

## See Also

- [`pa-multifactor-alpha-timing-conditional.md`](pa-multifactor-alpha-timing-conditional.md) — uses the same `x = a + Bf + e` regression to separate factor-explained return from alpha at the fund level.
- [`pa-active-risk-tracking-error-ex-ante-vs-ex-post.md`](pa-active-risk-tracking-error-ex-ante-vs-ex-post.md) — active variance `w_A' C w_A` is the same quadratic form, decomposed into factor vs idiosyncratic tracking error.
- [`pa-fi-shift-twist-butterfly-and-krd.md`](pa-fi-shift-twist-butterfly-and-krd.md) — shift/twist/butterfly are the characteristic-based factor exposures for default-free fixed income, a special case of this `B`.
- [`pa-dgtw-cs-ct-as-decomposition.md`](pa-dgtw-cs-ct-as-decomposition.md) — characteristic-based attribution at the holdings level, a cousin of the characteristic factor-model type. The Euler `2Cw` gradient is the same positive-homogeneity capital-allocation principle used in rm-* risk cards (Euler-principle capital allocation) and underlies the tracking-error budgeting in pm-* portfolio-management cards.

## Escalate to Raw When

- You need worked numbers for any MCR, factor-variance, or risk-budget allocation
  (these are deferred per the no-worked-arithmetic rule).
- You must implement the strict / approximate factor-model estimation (asymptotic
  principal components, "small-n" vs "large-n" techniques, number-of-factors
  determination) — see Connor, Goldberg & Korajczyk (2010) §4.1-4.6 pp.104-130.
- You need the exact risk-budget first-order conditions and Sharpe's (2002)
  fund-planning application, or Litterman's (1996) multi-perspective budgeting
  procedure — CGK §1.3.7-1.3.8 pp.46-47.
- You need the macroeconomic or characteristic-based estimation specifics
  (Chen-Roll-Ross style macro factors; Rosenberg / Fama-French attribute scaling)
  — CGK Ch.5-6.
