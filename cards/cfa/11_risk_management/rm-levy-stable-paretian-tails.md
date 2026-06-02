---
schema_version: "cacg.v0"
id: "rm-levy-stable-paretian-tails"
title: "Lévy-Stable Laws and Paretian Tails: The Physics Origin of Fat Tails"
reading_id: "11_risk_management"
summary: "Bouchaud & Potters' statistical-physics Lévy/stable mechanism for fat tails: symmetric Lévy laws via the characteristic function exp(-a_µ|z|^µ), power-law Paretian tails L_µ(x) ~ µA±^µ/|x|^{1+µ} with tail amplitudes as generalized cumulants and asymmetry β, recovering the Gaussian at µ=2, per Bouchaud & Potters (2003) Ch.1 §1.8."
tags: ["risk-management", "fat-tails", "levy-stable"]
citations:
  - source_id: "rm_bouchaud_potters_2003_theory_financial_risk"
    chunk_id: "rm_bouchaud_potters_2003_theory_financial_risk:p032:0035"
    chunk_hash: "ea250fd600e452d838850fc4638dc95d29f428b158b182de5dd4b01fee54aa1c"
    page_range: [32, 32]
    quote: "distributions is their power-law behaviour for large arguments, often called Pareto"
    edge_type: "defines"
card_hash: "2cb3ea84d8c50c2b41782f622bd0a87867e7c60723fbc056dde0fefc497e207d"
---
# Lévy-Stable Laws and Paretian Tails: The Physics Origin of Fat Tails

## Intuition
This card is the **statistical-physics Lévy / stable-Paretian** mechanism for fat
tails — Bouchaud & Potters' route, in which heavy tails are not fitted to data after
the fact but emerge as a *structural* consequence of the only addition-stable laws
beyond the Gaussian. Lévy laws are the fixed points of the central-limit operation
when the variance is infinite: just as a Gaussian is stable under summation, a Lévy
law reproduces itself under summation, but with tails so fat that the variance
diverges. This is the physicist's lens on "multiscale" phenomena — earthquakes,
personal income, asset returns — where both very small and gigantic values are
routinely observed. It is one of *four distinct mechanisms* by which fat tails enter
risk; the other three are applied/statistical EVT, heavy-tail probability / ruin
asymptotics, and crash-as-criticality (see See Also). Do not collapse them: here the
tail exponent µ is a *stability index*, not merely an estimated shape parameter.

```
   P(x)            Gaussian (µ=2): tails die exp-fast
   ^   .                 Lévy (µ<2): sharper body, fatter power-law tails
   |  / \                  .
   | /   \              . | |
   |/     \          .    | |  .
   +-------------------------------> x
   body set by A      tail ~ µA±^µ / |x|^{1+µ}
```

**Source:** Bouchaud & Potters (2003) Ch.1 §1.8 printed pp.10–11 (PDF pp.32–33).

## Definition
A **symmetric Lévy distribution** L_µ(x) is most cleanly defined through its
characteristic function (Fourier transform):

- **Characteristic function:** L̂_µ(z) = exp(−a_µ |z|^µ), with stability index
  0 < µ ≤ 2 (µ = 2 is the Gaussian boundary; µ > 2 is not a valid law) and a_µ > 0 a
  scale constant proportional to the tail *parameter* A^µ (a_µ ∝ A^µ, not A itself).
- **Paretian tails (strict 0 < µ < 2):** for large arguments the density decays as a
  power law, L_µ(x) ~ µA±^µ / |x|^{1+µ} as x → ±∞, so the exceedance probability is
  P>(x) = (A+/x)^µ for large positive x. At µ = 2 the law is Gaussian and this
  power-law tail does not exist.
- **Tail amplitudes A±** ("scale parameters" or *generalized cumulants*): A± sets the
  order of magnitude of the large positive / negative fluctuations; for the symmetric
  case A = A+ = A− fixes the overall scale.
- **Asymmetry parameter:** β ≡ (A+^µ − A−^µ)/(A+^µ + A−^µ) measures the relative
  weight of the two tails; β = 0 is symmetric, β = 1 fully one-sided (strictly
  positive variables).
- **Special closed forms:** µ = 1 is the Cauchy/Lorentzian; µ = 2 is the Gaussian.

**Source:** Bouchaud & Potters (2003) Ch.1 §1.8 printed pp.10–11 (PDF pp.32–33).

## Mathematical Reasoning
The defining property is **stability under addition**: a sum of iid Lévy variables is
again Lévy with the same µ, because the characteristic function exponentiates —
[L̂_µ(z)]^N = exp(−N a_µ |z|^µ) is still of the form exp(−a'_µ |z|^µ). This is the
same algebraic closure that makes the Gaussian stable, and it singles out the Lévy
family as the *only* attractors of summed iid variables.

Two limits anchor the family:

- **µ → 2 (Gaussian recovery).** exp(−a_µ|z|^µ) → exp(−a_2 z^2), the Gaussian
  characteristic function; the power-law tails vanish because they are "eaten up" by
  the exponentially decaying body, and the variance becomes finite.
- **µ < 2 (infinite variance).** Because the density decays only as |x|^{−1−µ}, the
  second moment integral diverges for µ < 2 (the source phrases the condition as µ ≤ 2,
  the boundary µ = 2 being the Gaussian, which has finite variance); for µ ≤ 1 even the
  mean and mean absolute deviation fail to
  exist, though the median and most probable value remain.

A practical caveat: the leading tail term µA±^µ/|x|^{1+µ} is only asymptotic. There
are subleading corrections, so a naïve power-law fit of a finite-x tail returns an
*apparent* exponent larger than µ that drifts toward µ only very slowly — slower the
nearer µ is to the Gaussian value 2. This is why empirical tail-index estimates are
treacherous near µ = 2.

```
  char. fn.   L̂_µ(z) = exp(-a_µ |z|^µ)
              µ=2 ─► exp(-a z^2)  (Gaussian, thin tails, finite variance)
              µ<2 ─► non-analytic |z|^µ  (Lévy, Pareto tails, infinite variance)
```

**Source:** Bouchaud & Potters (2003) Ch.1 §1.8 printed pp.11–12 (PDF pp.33–34).

## See Also
- [rm-truncated-levy-clt-crossover](./rm-truncated-levy-clt-crossover.md) — the truncated-Lévy continuation: how the power-law tail acquires a cut-off and converges to Gaussian.
- [rm-evt-gpd-pot-hill](./rm-evt-gpd-pot-hill.md) — the McNeil applied-EVT mechanism for fat tails (contrast).
- [rm-cramer-lundberg-heavy-tail-ruin](./rm-cramer-lundberg-heavy-tail-ruin.md) — the EKM heavy-tail / ruin-asymptotics mechanism (contrast).
- [rm-crash-as-critical-point](./rm-crash-as-critical-point.md) — the Sornette criticality mechanism for fat tails (contrast).
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — the loss-distribution object whose tail this law describes.

## Escalate to Raw When
You need the explicit moment formula |x|^ν for the symmetric Lévy law, the full
subleading asymptotic series for L_µ(x), the closed-form constant a_µ ↔ A^µ
relations, or the worked exponential-tail limiting case A^µ = (µ/α)^µ — those
explicit formulae and any numeric tail-fit illustrations live in the raw text
(Rule 1).

**Source:** Bouchaud & Potters (2003) Ch.1 §1.8 printed pp.10–13 (PDF pp.32–35).
