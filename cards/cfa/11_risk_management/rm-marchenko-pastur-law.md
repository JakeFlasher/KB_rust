---
schema_version: "cacg.v0"
id: "rm-marchenko-pastur-law"
title: "Marchenko-Pastur Law: The Null Spectrum of a Sample Covariance Matrix"
reading_id: "11_risk_management"
summary: "The Marchenko-Pastur law gives the limiting eigenvalue density of a white-Wishart sample covariance matrix with spectral edges (1±√q)² and a (q-1)/q delta-mass for q>1, derived from a self-consistent Stieltjes equation, per Potters & Bouchaud Ch.4."
tags: ["risk-management", "random-matrix-theory", "covariance-cleaning"]
citations:
  - source_id: "rm_potters_bouchaud_2020_random_matrix_theory"
    chunk_id: "rm_potters_bouchaud_2020_random_matrix_theory:p070:0077"
    chunk_hash: "a62210aea8bbb11faff25bc1bc884f1c4e37a57104325222906558c530bcf09c"
    page_range: [71, 71]
    quote: "In the large N limit we obtain the following self-consistent equation for g(z)"
    edge_type: "defines"
  - source_id: "rm_potters_bouchaud_2020_random_matrix_theory"
    chunk_id: "rm_potters_bouchaud_2020_random_matrix_theory:p070:0077"
    chunk_hash: "a62210aea8bbb11faff25bc1bc884f1c4e37a57104325222906558c530bcf09c"
    page_range: [71, 71]
    quote: "The argument of the square-root is quadratic in z and its roots (the edge of spectrum) are given by"
    edge_type: "supports"
card_hash: "2e7817c2d10bc1a433669b7b3de532ba7c5a1ba626ae082ee236e1952ab4acfb"
---
# Marchenko-Pastur Law: The Null Spectrum of a Sample Covariance Matrix

## Intuition
A risk manager estimates an `N×N` covariance from `T` observations, but with `N` and `T`
both large and comparable the sample covariance matrix is a badly distorted picture of the
truth. The Marchenko-Pastur (MP) law answers the prior question: *if the true covariance
were the identity (pure noise), what would the eigenvalue histogram look like anyway?* The
answer is a sharp-edged "bulk" between `λ_−` and `λ_+`. Any sample eigenvalue landing inside
that band is statistically indistinguishable from noise — there is no information in its
exact value. This is the null hypothesis against which real spectra are read: only
eigenvalues poking out beyond `λ_+` carry signal, and the noisy bulk must be shrunk before
the matrix is inverted for a portfolio.

```
   ρ(λ)
   ^                  Marchenko-Pastur bulk (q < 1)
   |        _______
   |      /         \_
   |    /             \
   |   |               \         o   <- signal eigenvalue (beyond λ_+)
   +---+-------+--------+----+----------> λ
       λ_-              λ_+
       (1-√q)²          (1+√q)²
```

**Source:** Potters & Bouchaud (2020) Ch.4 §4.2 printed pp.48–51 (PDF pp.69–72).

## Definition
Let `E = (1/T) H Hᵀ` be the sample covariance matrix of `N` zero-mean variables observed
`T` times, in the "white" case where the population covariance is the identity (`C = 1`).
Write `q = N/T`. In the large-`N` limit the eigenvalue density converges to the
Marchenko-Pastur law,

```
ρ_MP(x) = √[ (λ_+ − x)(x − λ_−) ]₊ / (2π q x)  +  ((q−1)/q) δ(x) · 𝟙(q > 1),
```

with spectral edges

```
λ_± = (1 ± √q)² ,
```

where `[a]₊ := max{a, 0}`. For `q ≤ 1` the spectrum is a single bulk on `[λ_−, λ_+]`. For
`q > 1` the data matrix is rank-deficient, producing `N − T` exactly-zero eigenvalues; these
appear as the `(q−1)/q` delta-mass at the origin.

**Source:** Potters & Bouchaud (2020) Ch.4 §4.2.2 printed pp.50–51 (PDF pp.71–72).

## Mathematical Reasoning
The derivation uses the Stieltjes transform `g(z) = τ((z𝟙 − W)⁻¹)`, the normalized-trace
resolvent. Applying the cavity / Schur-complement method to a single removed variable and
taking the large-`N` limit yields the self-consistent equation for the white Wishart case,

```
1/g(z) = z − 1 + q − q z g(z).
```

This is a quadratic in `g(z)`. Solving and selecting the branch with the correct analytic
behavior gives

```
g(z) = [ z − (1 − q) − √(z − λ_+) √(z − λ_−) ] / (2 q z).
```

The square-root's argument is quadratic in `z`; its roots are exactly the spectral edges
`λ_± = (1 ± √q)²`. The density is recovered from the resolvent by the inversion formula
`ρ(x) = (1/π) lim_{η→0⁺} Im g(x − iη)`: an imaginary part exists only where
`(x − λ_+)(x − λ_−) < 0`, i.e. strictly inside `(λ_−, λ_+)`, which reproduces the bulk
formula above. Examining `g(z)` near `z = 0` reveals a pole when `q > 1`, contributing the
`((q−1)/q) δ(x)` mass that counts the trivial zero eigenvalues. The risk consequence: even
under pure noise the sample spectrum has width `λ_+ − λ_− = 4√q > 0`, so finite-`T`
estimation always inflates the dispersion of eigenvalues relative to the (flat) truth.

**Source:** Potters & Bouchaud (2020) Ch.4 §4.2.1–4.2.2 printed pp.48–51 (PDF pp.69–72).

## See Also
- [rm-rotationally-invariant-estimator](./rm-rotationally-invariant-estimator.md) — the cleaning rule that shrinks the noisy MP bulk.
- [rm-integrated-firm-wide-risk-aggregation](./rm-integrated-firm-wide-risk-aggregation.md) — where the cleaned covariance feeds firm-wide aggregation.
- [rm-elliptical-spherical-distributions](./rm-elliptical-spherical-distributions.md) — the distributional setting in which covariance estimation is meaningful.

## Escalate to Raw When
You need the explicit moments of the MP density, the worked `q = 1/2` vs `q = 2` density
plots, or the exercise verifying the rescaling `ρ_{1/q}(λ) = q² ρ_q(qλ)` — those numeric
and plotted results live in the raw text (Rule 1).

**Source:** Potters & Bouchaud (2020) Ch.4 §4.2 printed pp.48–52 (PDF pp.69–72).
