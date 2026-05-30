---
schema_version: "cacg.v0"
id: "pa-partial-moment-ratios-sortino-omega-kappa"
title: "Partial-Moment Ratios: Sortino, Omega, and Kappa"
reading_id: "15_performance_and_attribution"
summary: "Postmodern reward-to-downside ratios built on lower partial moments: downside variance is the 2nd LPM and downside potential the 1st; Omega, Omega-Sharpe, Sortino and Kappa are all special cases of the generalised Kappa K_l family indexed by moment order l."
tags: ["downside-risk", "partial-moments", "risk-adjusted-return"]
citations:
  - source_id: "pa_bacon_2022_rapm"
    chunk_id: "pa_bacon_2022_rapm:p150:0135"
    chunk_hash: "8d97436b705cb6016f1c71afdea5f3e06d0473ec81039ac38085ae8f96ef5831"
    page_range: [151, 151]
    quote: "For l = 1, K1 is the Omega–Sharpe ratio and for l = 2, K2 is the Sortino ratio."
    edge_type: "defines"
---
# Partial-Moment Ratios: Sortino, Omega, and Kappa

## Intuition

Standard deviation and the Sharpe ratio treat upside and downside dispersion
symmetrically. Postmodern Portfolio Theory rejects that symmetry: asset owners
welcome upside surprises and only fear shortfalls below a minimum target return
`rT`. Partial-moment ratios formalise that asymmetry by measuring reward against
*one-sided* risk — they ignore desirable (upside) variability and penalise only
undesirable (downside) outcomes. Sortino, Omega, Omega-Sharpe and Kappa are not
four unrelated measures; they are one family, differing only in which lower
partial moment sits in the denominator and whether the numerator is excess
return or upside potential.

**Source:** Bacon (2022) Ch.6 (Partial Moments) printed pp.121-122 (PDF pp.143-144)

## Definition

Fix a minimum target return `rT` (risk-free rate, benchmark, zero, or any
threshold). Truncating returns at the target defines the *lower partial moments*
(LPMs) and *upper partial moments* (UPMs):

- **Downside potential** `mu_D` — the 1st lower partial moment: the average of
  shortfalls `max[(rT - ri), 0]` (by convention the sign is dropped).
- **Downside risk** `sigma_D` (semi-standard deviation) — the root of the 2nd lower
  partial moment: the average of squared shortfalls `min[(ri - rT), 0]^2`.
- **Upside potential** `mu_U` — the 1st upper partial moment: the average of
  excesses `max[(ri - rT), 0]`.

The family ratios are then:

| Ratio | Numerator | Denominator |
|-------|-----------|-------------|
| Omega `Omega` | upside potential `mu_U` | downside potential `mu_D` |
| Omega-Sharpe | excess return `(r_avg - rT_avg)` | downside potential `mu_D` |
| Sortino | excess return `(r_avg - rT_avg)` | downside risk `sigma_D` |
| Kappa `K3` | excess return `(r_avg - rT_avg)` | cube-root of 3rd LPM |

Generalised, these collapse to a single Sortino-Satchell / Kappa form `K_l`
parameterised by the moment order `l`. The book reports that Kaplan and Knowles
demonstrate both the Sortino ratio and the Omega-Sharpe ratio are special cases
of Kappa.

**Source:** Bacon (2022) Ch.6 (Partial Moments) printed pp.122-129 (PDF pp.144-151)

## Mathematical Reasoning

The bridge from gain-loss to Sharpe form is an identity, not a numerical claim.
Excess return decomposes exactly into upside potential minus downside potential:

```
r_avg - rT_avg  =  mu_U - mu_D   (per-period average of max(ri-rT,0) - max(rT-ri,0))
```

Dividing through by downside potential `mu_D`:

```
Omega-Sharpe = (r_avg - rT_avg)/mu_D = (mu_U - mu_D)/mu_D = mu_U/mu_D - 1 = Omega - 1
```

So Omega and Omega-Sharpe are monotone transforms of each other and rank
portfolios identically; Omega-Sharpe is merely expressed in the familiar
reward-per-unit-risk Sharpe format. The generalised ratio nests the whole
family through the choice of moment order `l` in the denominator:

```
                r_avg - rT_avg
   K_l  =  ----------------------------
            ( (1/n) sum [max(rT-ri,0)]^l )^(1/l)

   l = 1  ->  K1  =  Omega-Sharpe   (1st LPM in denominator)
   l = 2  ->  K2  =  Sortino        (2nd LPM, i.e. downside risk sigma_D)
   l = 3  ->  K3  =  Kappa          (3rd LPM)
```

Larger `l` weights extreme downside outcomes more heavily, encoding greater risk
aversion; `l` need not be an integer and is set by investor preference. Higher
moments are why Omega "implicitly adjusts for both skewness and kurtosis" of the
return distribution. The Prospect-Theory direction extends the same machinery:
Farinelli-Tibiletti ratios raise both the upside and downside truncated returns
to chosen powers `u` and `l` (e.g. `u<1, l>1` is risk-averse; `u=3, l=3` gives
gain-loss skewness), and downside skewness/kurtosis push to the 3rd and 4th
LPMs — though Bacon flags that he is "not convinced of the value" of those last
two and includes them only because they appear in the literature.

**Source:** Bacon (2022) Ch.6 (Partial Moments) printed pp.125-134 (PDF pp.147-156)

## See Also

- [`pa-variability-ratios-sharpe-information.md`](pa-variability-ratios-sharpe-information.md) — the symmetric two-sided parents (Sharpe, Information) that these one-sided ratios refine.
- [`pa-regression-appraisal-jensen-treynor.md`](pa-regression-appraisal-jensen-treynor.md) — regression-based appraisal ratios; complementary skill-vs-risk lens on the same returns.
- [`pa-drawdown-ratios-calmar-sterling-ulcer.md`](pa-drawdown-ratios-calmar-sterling-ulcer.md) — path-dependent downside ratios (drawdown-based) sitting alongside these moment-based ones.
- [`pa-m2-risk-adjusted-return-transform.md`](pa-m2-risk-adjusted-return-transform.md) — leverage-equivalent return transform, another reward-to-risk reframing for ranking portfolios.

## Escalate to Raw When

- You need the worked Exhibit 6.1 / Table 6.1 numbers (monthly returns, summed
  squared deviations, annualised `sigma_D`, computed Omega/Sortino/UPR/variability
  skewness values) — these are explicit numerical examples deferred to the book.
- You need the half-variance, pure-downside, loss/gain standard deviation, mean
  absolute moment, Bernardo-Ledoit, d-ratio, upside potential ratio, volatility
  or variability skewness exact formulae rather than the family relationships.
- You need the precise annualisation convention (multiplying by sqrt(t)) or guidance
  on minimum-observation counts that make a downside calculation meaningful.
- You need the full citation chain (Shadwick-Keating, Sortino-van der Meer,
  Kaplan-Knowles, Farinelli-Tibiletti) behind each named ratio.
