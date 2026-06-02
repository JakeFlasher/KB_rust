---
schema_version: "cacg.v0"
id: "rm-lppl-precursor-formula-contested"
title: "The Log-Periodic Power-Law (LPPL/LPPLS) Crash Precursor — A Contested Stance"
reading_id: "11_risk_management"
summary: "Sornette's contested LPPL precursor: the simplest log-periodic correction F(t)=A+B(t_c−t)^m[1+C cos(ω ln((t_c−t)/T))] to a power-law singularity at t_c, with discrete scale invariance and preferred scaling ratio λ≈2, presented as a diagnostic stance on bubble dynamics, not settled crash-prediction fact, per Sornette (2017) Ch.7."
tags: ["risk-management", "log-periodic", "bubbles"]
citations:
  - source_id: "rm_sornette_2017_why_stock_markets_crash"
    chunk_id: "rm_sornette_2017_why_stock_markets_crash:p257:0314"
    chunk_hash: "9fc9ae961cf8e5ae09efe4bf9de7fb74d66a47bd53a2eb09907174f0bfa3b006"
    page_range: [258, 258]
    quote: "This equation is the simplest example of a log-periodic correction to a pure power law for an observable exhibiting a singularity at the time tc at which the crash has the highest probability"
    edge_type: "defines"
card_hash: "8c2f694fb16a76ad512bd0de9ad77a95fb2040caa5e20c5d60430a7a76ec34a6"
---
# The Log-Periodic Power-Law (LPPL/LPPLS) Crash Precursor — A Contested Stance

## Intuition
If the criticality picture (`rm-crash-as-critical-point`) is right, a bubble should
leave a *signature* in the price path on the approach to the critical time t_c: not
just an acceleration, but an acceleration decorated with **oscillations that speed up**
as t_c is neared. Plotting a bubble, Sornette's eye sees a cusp-like power-law
acceleration plus systematic oscillatory deviations whose period shrinks
geometrically. The log-periodic power-law (LPPL, later LPPLS) form is the simplest
mathematical object that reproduces both features at once: a finite-time singularity
(the power law) modulated by a cosine of the *logarithm* of the time-to-crash (the
log-periodicity). The intuition is "discrete scale invariance" — the market repeats
the same qualitative move at a preferred ratio of time-distances to t_c, like a
self-similar staircase compressing into the critical date.

```
   price F(t)
   ^                                       . (singularity at t_c)
   |                                    . /|
   |                          .  _.--'   / |
   |                  . _.--''   oscillations  <- log-periodic
   |          ._.--''       compress as t -> t_c   (period shrinks
   |   __.--''                                       by ratio lambda)
   +----------------------------------------> time t
                                          t_c (most-probable crash)
```

**Source:** Sornette (2017) Ch.7 printed pp.231–232 (PDF pp.257–258).

## Definition
Two nested parameterizations describe the accelerating bubble. The **pure power law**
captures the acceleration toward a singularity:

    F_pow(t) = A + B (t_c − t)^m

where t_c is the time at which the fit has a (theoretically) diverging slope. The
**log-periodic power law (LPPL)** adds the simplest log-periodic correction:

    F_lp(t) = A + B (t_c − t)^m · [ 1 + C cos( ω · ln((t_c − t)/T) ) ]

(the source writes the modulation as a cosine of the logarithm of the *dimensionless*
ratio (t_c − t)/T; ω is the log-angular frequency and T a normalizing time scale, so the
−ω ln T piece is just a constant phase — equivalent to the cos(ω′ ln(t_c − t)) form used
in the derivation below). Reading the symbols:

| symbol | meaning |
|--------|---------|
| t_c | critical time — where the slope diverges; the *most-probable* crash time |
| m | power-law exponent of the acceleration (0 < m < 1) |
| A, B | level and amplitude of the power-law trend |
| C | relative amplitude of the log-periodic oscillation |
| ω, T | ω is the log-angular frequency of the oscillation; T a normalizing time scale (its log enters only as a constant phase) |

The log-periodicity implies the index becomes **discretely scale-invariant** near t_c,
producing a hierarchy of characteristic times (t_c − t_n) with a **preferred scaling
ratio λ** (the book's g / λ): consecutive ratios (t_c − t_n)/(t_c − t_{n+1}) ≈ λ. The
cited October-1987 fit gives λ ≈ 1.5–1.7, reported as approximately universal across
crashes (the book elsewhere quotes per-crash values around λ ≈ 2).

**Source:** Sornette (2017) Ch.7 printed pp.232–233 (PDF pp.258–259).

## Mathematical Reasoning
The LPPL form is presented here as a *definition / functional spec*, not a worked
calculation. Symbolically its structure follows from two assumptions:

- **Finite-time singularity.** Near a critical point an observable acquires a power-law
  form (t_c − t)^m; the diverging slope as t → t_c encodes the accelerating instability
  derived in the criticality card.
- **Discrete (not continuous) scale invariance.** Continuous scale invariance forces a
  pure power law; allowing only *discrete* scale invariance — invariance under
  rescaling t_c − t by a fixed factor λ — promotes the exponent m to a complex value
  m + iω', and the real part of (t_c − t)^{m+iω'} is exactly a power law times a cosine
  of ω' ln(t_c − t). Hence the log-periodic cosine is the generic leading correction,
  and λ = exp(2π/ω') ties the preferred scaling ratio to the log-frequency.

The fitted parameter values that appear in the figures (e.g. the reported t_c, m, and
λ ≈ 1.5–1.7 for the October 1987 S&P 500 fit) are **regression outputs**, not
plug-and-chug steps — they illustrate one calibration of the form, never a computation
the reader should reproduce. A richer nonlinear generalization (via bifurcation theory)
extends the simple form to longer pre-crash windows and to "log-frequency modulation,"
but the simple LPPL is the canonical object.

```
   continuous scale invariance ──► pure power law (t_c - t)^m
        |  restrict to DISCRETE scale invariance (factor lambda)
        v
   exponent m  ->  m + i*omega'  (complex)
        |  take real part of (t_c - t)^{m + i omega'}
        v
   (t_c - t)^m * [1 + C cos(omega' ln(t_c - t))]   <- LPPL form
        with  lambda = exp(2*pi / omega')  (preferred ratio, ~2)
```

**Source:** Sornette (2017) Ch.7 printed pp.232–233 (PDF pp.258–259).

## Boundary Notes
The LPPL precursor is a **CONTESTED** model family, stated here as Sornette's stance —
NOT as established fact that "crashes can be predicted." Several source-internal
qualifications keep the claim bounded, and must travel with any citation of this card:

- **t_c is the most-probable, not the certain, crash time.** It is included in the
  log-periodic structure of the bubble, but the crash is *randomly triggered* with a
  biased probability that strengthens as t → t_c; fits "overshoot," systematically
  landing later than the real crash. The date is a peak of a distribution, not a
  prediction.
- **A bubble can deflate without any crash.** Under the rational-expectation benchmark
  there is always a finite probability that a bubble lands smoothly; so-called "false
  alarms" are consistent with the theory, not refutations of it. Predictive power is
  therefore inherently probabilistic.
- **Prediction of regime-change is generically unreliable.** Sornette explicitly notes
  that techniques are good at recognizing a trend already underway but bad at calling
  reversals; hard point-numbers on predictions are "misleading," and only the
  probability distribution of success carries information.

So treat LPPL as a *diagnostic* for positive-feedback / critical-regime behavior and a
hypothesis-generator for stress and scenario work, never as a settled crash oracle. Do
not over-claim beyond the source's own rigor.

**Source:** Sornette (2017) Ch.9 printed pp.320–322 (PDF pp.346–348); Ch.9 printed p.346 (PDF p.372).

## See Also
- [rm-crash-as-critical-point](./rm-crash-as-critical-point.md) — the criticality mechanism this precursor form operationalizes.
- [rm-scenario-analysis](./rm-scenario-analysis.md) — the probabilistic-scenario discipline that contextualizes any LPPL signal.
- [rm-stress-testing](./rm-stress-testing.md) — where a bubble-regime diagnostic would feed forward-looking stress design.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — the loss/drawdown anatomy that crashes punctuate.

## Escalate to Raw When
You need the fitted LPPL parameter tables, the calibrated t_c / m / λ values, the
worked variance-ratio goodness-of-fit numbers, or the forward-prediction track record
(successes and false alarms) — those numeric recipes and fitted figures live in the raw
text (Rule 1).

**Source:** Sornette (2017) Ch.7 printed pp.231–235 (PDF pp.257–261); Ch.9 printed pp.320–346 (PDF pp.346–372).
