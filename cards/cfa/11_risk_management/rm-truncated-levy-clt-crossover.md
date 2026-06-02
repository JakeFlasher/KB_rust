---
schema_version: "cacg.v0"
id: "rm-truncated-levy-clt-crossover"
title: "Truncated Lévy Flights and the Lévy→Gaussian Convergence Crossover"
reading_id: "11_risk_management"
summary: "Bouchaud & Potters' truncated-Lévy mechanism for fat tails: a power-law tail carrying an exponential cut-off, infinitely divisible for all α and µ, that aggregates Lévy-like below a crossover sample size N* and converges to Gaussian above it — why fat tails dominate short horizons but fade at long ones, per Bouchaud & Potters (2003) Ch.1 §1.8 and Ch.2 §2.3.6."
tags: ["risk-management", "fat-tails", "levy-stable"]
citations:
  - source_id: "rm_bouchaud_potters_2003_theory_financial_risk"
    chunk_id: "rm_bouchaud_potters_2003_theory_financial_risk:p035:0038"
    chunk_hash: "db1d92a523d8e5aa899d82b95d18bd3f4c32ca2841596b94274f266647ee802a"
    page_range: [35, 35]
    quote: "distribution has the interesting property of being infinitely divisible for all values of α and µ"
    edge_type: "defines"
card_hash: "177bccbe9d8085b05b5582c2256f31224f8c51abc61060e401c00a5364084a64"
---
# Truncated Lévy Flights and the Lévy→Gaussian Convergence Crossover

## Intuition
This card continues the **statistical-physics Lévy / stable-Paretian** mechanism for
fat tails — Bouchaud & Potters' route — by fixing the pure Lévy law's embarrassment:
its infinite variance. Real asset returns have fat tails but finite variance, so the
pure power-law cannot be the whole story. The fix is to keep the Lévy *body and
near-tail* but graft an exponential cut-off onto the far tail. The payoff is a
horizon story: when you aggregate (sum returns over longer windows) the truncated
Lévy behaves Lévy-like — fat tails, apparent scale invariance — up to a *crossover
sample size N\**, and only above N* does the central limit theorem finally bite and
the distribution converge to a Gaussian. This is exactly why fat-tail risk is acute
at short horizons (intraday, daily) yet appears to soften at long horizons. It is one
of *four distinct mechanisms* by which fat tails enter risk; the other three are
applied/statistical EVT, heavy-tail probability / ruin asymptotics, and
crash-as-criticality (see See Also). Do not collapse them.

```
  log P>(x)
   ^   pure Lévy: straight power-law tail (slope -µ), variance = ∞
   |  \
   |   \___ truncated Lévy: power-law mid-tail ...
   |       \
   |        \    ... then exponential knee (cut-off α) ► finite variance
   +------------------------------------------------> x
        aggregate N draws:  N << N*  Lévy-like  |  N >> N*  Gaussian
```

**Source:** Bouchaud & Potters (2003) Ch.1 §1.8 printed p.13 (PDF p.35).

## Definition
A **truncated Lévy distribution (TLD)** is obtained by deforming the pure Lévy
characteristic function so that the power-law far-tail is suppressed exponentially
beyond a scale set by a cut-off parameter α:

- **Construction.** Alter L̂_µ(z) = exp(−a_µ|z|^µ) into a form L̂_µ^{(t)}(z) built
  from (α² + z²)^{µ/2} and arctan(|z|/α); it reduces to the pure Lévy law as α → 0
  and to a Gaussian as α → ∞ with a_µ α^{µ−2} held fixed.
- **Stability scope.** The TLD is **infinitely divisible** for all α and µ (the limit
  cases — pure Lévy and Gaussian — included), so it is a legitimate building block
  for a stationary-increment stochastic process.
- **Finite cumulants.** For 1 < µ < 2 the variance c_2 ∝ a_µ α^{µ−2} is finite and
  the third cumulant vanishes in the symmetric case; the kurtosis
  λ_4 ∝ (cos πµ/2)/(a_µ α^µ) is finite and positive, vanishing as µ → 2 (Gaussian)
  and diverging as α → 0 (pure Lévy, infinite c_2 and c_4).

**Source:** Bouchaud & Potters (2003) Ch.1 §1.8 printed p.13 (PDF p.35).

## Mathematical Reasoning
Infinite divisibility is the hinge. Because the TLD is infinitely divisible, a sum of
N iid TLD increments has a characteristic function [L̂_µ^{(t)}(z)]^N that is again of
TLD type, so we can read off how aggregation reshapes the law. Two regimes separate
at a **crossover sample size N\***, governed by the competition between the Lévy scale
a_µ and the cut-off α:

- **N ≪ N\* (Lévy regime).** The cut-off has not yet been "felt": over the bulk of
  the relevant range the summed law looks like a pure Lévy distribution with the same
  stability index µ — fat, apparently scale-invariant tails, large kurtosis.
- **N ≫ N\* (Gaussian regime).** Because each increment has a *finite* variance
  (the cut-off guarantees this), the ordinary CLT applies; the normalized sum
  converges to a Gaussian, and the kurtosis of the aggregate decays as λ_4(N) ∝ 1/N.

Thus the TLD interpolates continuously between the two addition-stable fixed points:
the parameter that decays under aggregation is the kurtosis, and the *crossover
condition* N ≫ N* is exactly the statement "enough terms have been summed for the
finite variance to dominate the truncated tail." Critically, the convergence is
slow near µ = 2, so the Gaussian regime can require horizons of weeks or months —
non-Gaussian effects persist far longer than naïve CLT intuition suggests.

```
   kurtosis λ_4(N)
   ^  \                         crossover at N*
   |   \___ Lévy regime (λ_4 large, ~const)
   |       \
   |        \___ ~ 1/N decay  ─► Gaussian regime (λ_4 → 0)
   +-------------|---------------------------> N (aggregation size)
                 N*
```

**Source:** Bouchaud & Potters (2003) Ch.1 §1.8, Ch.2 §2.3.6 printed pp.13, 35–36 (PDF pp.35, 57–58).

## See Also
- [rm-levy-stable-paretian-tails](./rm-levy-stable-paretian-tails.md) — the pure Lévy / Paretian law this card truncates.
- [rm-evt-gpd-pot-hill](./rm-evt-gpd-pot-hill.md) — the McNeil applied-EVT mechanism for fat tails (contrast).
- [rm-cramer-lundberg-heavy-tail-ruin](./rm-cramer-lundberg-heavy-tail-ruin.md) — the EKM heavy-tail / ruin-asymptotics mechanism (contrast).
- [rm-crash-as-critical-point](./rm-crash-as-critical-point.md) — the Sornette criticality mechanism for fat tails (contrast).

## Escalate to Raw When
You need the explicit truncated-Lévy characteristic-function formula, the closed-form
cumulants c_2 and kurtosis λ_4 in α and µ, the numeric value of the crossover N* for
a given calibration, or the worked aggregation-convergence plots — those formulae and
numbers live in the raw text (Rule 1).

**Source:** Bouchaud & Potters (2003) Ch.1 §1.8, Ch.2 §2.3.6 printed pp.13, 35–36 (PDF pp.35, 57–58).
