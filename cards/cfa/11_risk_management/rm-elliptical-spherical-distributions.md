---
schema_version: "cacg.v0"
id: "rm-elliptical-spherical-distributions"
title: "Spherical and Elliptical Distributions: When VaR Behaves and Markowitz Holds"
reading_id: "11_risk_management"
summary: "Spherical/elliptical laws defined via the stochastic representation X = mu + RAS and the characteristic-generator form; inside a common elliptical world VaR is subadditive and coherent and mean-variance (Markowitz) optimization is justified, marking the boundary of Gaussian-style risk intuition, per McNeil et al. (2015) Ch.6 and Ch.8."
tags: ["risk-management", "elliptical-distributions", "coherence"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p317:0453"
    chunk_hash: "c7d5ca4ba152bf064f5ee109630cb0cdbd64a9c94303f2baf1f2af81b52c340f"
    page_range: [318, 318]
    quote: "Part (2) gives a special case where the VaR risk measure is subadditive and therefore coherent"
    edge_type: "defines"
card_hash: "015d958b995540085261ff73b774131213fcb1412eaed9530435be976ac8e1b7"
---
# Spherical and Elliptical Distributions: When VaR Behaves and Markowitz Holds

## Intuition
A lot of "standard" risk intuition — diversification always reduces VaR, correlation
fully describes co-movement, mean-variance optimization is optimal — is *not*
generally true. It is true inside one privileged world: the elliptical
distributions, of which the multivariate normal is the canonical member. Elliptical
laws have density contours that are concentric ellipsoids; a portfolio's loss is a
scalar location-scale family driven by a single radial variable. That structure is
exactly what makes VaR behave (subadditive, hence coherent) and makes Markowitz
correct. So this card marks the *boundary*: as long as your risk factors are
elliptical, Gaussian-style reasoning is safe; step outside (fat tails, tail
dependence, asymmetric losses) and that reasoning can fail.

```
   elliptical density: concentric ellipsoids
        ___
      /  .  \        every linear portfolio L = m + λ'X
     | ( o ) |       is a location-scale shift of one
      \ _._ /        radial variable R  ==>  VaR ~ scale
        contours of constant (x-μ)'Σ⁻¹(x-μ)
```

**Source:** McNeil et al. (2015) Ch.6 §6.3.2 printed pp.200–201 (PDF pp.221–222).

## Definition
**Spherical:** Y is spherical if it has a stochastic representation Y = R·S, where S
is uniform on the unit sphere and R ≥ 0 is an independent radial variable; equivalently
its characteristic function is φ_Y(t) = ψ(t'·t) for a characteristic generator ψ, and
its density (when it exists) is constant on hyperspheres, f(x) = g(x'x).

**Elliptical (Def. 6.25):** X is elliptical if X =_d μ + AY with Y spherical;
equivalently (Prop. 6.27) it admits the **stochastic representation**
X =_d μ + R·A·S, with S uniform on the unit sphere, R ≥ 0 independent of S, and
AA' = Σ. The characteristic function is φ_X(t) = e^{i t'μ} ψ(t'Σt). We write
X ∼ E_d(μ, Σ, ψ) with location μ, dispersion Σ, generator ψ. The normal and t laws
are elliptical (affine images of spherical special cases).

**Source:** McNeil et al. (2015) Ch.6 §6.3.2 printed pp.200–201 (PDF pp.221–222).

## Mathematical Reasoning
On the space M of linear portfolios L = m + λ'X with X elliptical, any
positive-homogeneous, translation-invariant, law-invariant risk measure ρ reduces to
ρ(L) = E(L) + sqrt(λ'Σλ)·ρ(Y) for a standardized radial variable Y. The risk is
therefore a constant multiple of the portfolio standard deviation. Three consequences
(implications of Theorem 8.28):

- **VaR is subadditive, hence coherent** on M — the one regular case where VaR
  obeys the coherence axioms it violates in general.
- **Markowitz holds** — minimizing ρ at fixed mean is identical to minimizing
  variance, so the risk-minimizing portfolio is the Markowitz variance-minimizing
  portfolio, *whatever* the (law-invariant, coherent) risk measure.
- The coherent risk-measure scenario sets are **ellipsoids**, differing only in
  radius ρ(Y).

Aggregation also becomes principled: under ellipticality the correlation-based
summation rule for mean-adjusted VaR/ES is justified (Prop. 8.29). Outside the
elliptical world the supports collapse: VaR can be *super*additive (e.g. two
independent exponentials), and the correlation-only rule ignores tail dependence.

**Source:** McNeil et al. (2015) Ch.8 §8.3.2 printed pp.297–301 (PDF pp.318–322).

## Boundary Notes
The elliptical assumption is the load-bearing idealization. Empirically it is
"unlikely to hold in practice": real risk factors are fat-tailed and exhibit tail
dependence, so the coherence-of-VaR and correlation-aggregation guarantees lapse, and
diversification can *increase* tail risk. Treat elliptical results as the clean
benchmark, not the operating assumption.

**Source:** McNeil et al. (2015) Ch.8 §8.4.1 printed pp.301–302 (PDF pp.322–323).

## See Also
- [rm-tail-dependence-coefficients](./rm-tail-dependence-coefficients.md) — the Gauss vs t contrast that lives inside the elliptical family.
- [rm-copulas-sklar-dependence](./rm-copulas-sklar-dependence.md) — Gauss/t copulas are the elliptical copulas.
- [rm-risk-measure-axioms](./rm-risk-measure-axioms.md) — the coherence axioms VaR satisfies only here.
- [rm-parametric-var](./rm-parametric-var.md) — the Gaussian/parametric VaR that this world underwrites.

## Escalate to Raw When
You need the explicit characteristic generators of the normal and t laws, the worked
two-exponential counterexample where VaR is super-additive, or the figure-based
ellipsoid scenario sets — those worked constructions and numeric figures live in the
raw text (Rule 1).

**Source:** McNeil et al. (2015) Ch.6 §6.3 / Ch.8 §8.3.2 printed pp.200–206, 297–304 (PDF pp.221–227, 318–325).
