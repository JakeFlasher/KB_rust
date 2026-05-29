---
schema_version: "cacg.v0"
id: "fi-oas-and-effective-duration"
title: "OAS and Effective Duration"
reading_id: "06_fixed_income_and_credit"
summary: "OAS and Effective Duration — CFA Vol.5/pp.290-310 (PDF 2927-2947) is at end of derivatives readings / start of alt inv; OAS/effective-duration treatment in CFA L1 lives in Vol.5/R43 (Vol.5 ~pp.6-50).; Hull pp.241-260 (PDF 241-260) is Ch.11 Trading Strategies / Tax in 11ed; no OAS / callable-bond / embedded-o"
tags: ["fixed-income", "oas-effective"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2511:3694"
    chunk_hash: "3a8007f9c2eaa9b892e0070a085a07e50007d146d5f420161acc8296501910e4"
    page_range: [2511, 2512]
    quote: "An option-adjusted spread (OAS) on a callable bond is the Z-spread:"
    edge_type: "defines"
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p508:0654"
    chunk_hash: "7f4f83e288ce78bc3208cd08ebcd48a93b7b97bb878134d59ef5edc66b3ae480"
    page_range: [508, 508]
    quote: "a bond-specific spread or option adjusted spread (OAS) would be added to the rates for valuation."
    edge_type: "supports"
card_hash: "0a6ce860a6a1c752eb03cb76ef8a958131c8fc3f7bbb884ddc860b4d17d34f73"
---
# OAS and Effective Duration

## Intuition

A vanilla bond's Z-spread (zero-volatility spread) is the
constant spread over the zero curve that prices the
bond. For an option-embedded bond, the Z-spread
double-counts the option's value because the spread
absorbs both credit risk and embedded-option asymmetry.
Option-Adjusted Spread (OAS) strips out the option value
first and reports the residual spread as the credit /
liquidity premium proper. Effective duration is the
shock-based duration that accommodates the option's
non-linear price-yield response. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.290-310.

```
yield (%)
   ^
   |
   |   Z-spread of callable bond
   |   ----------+
   |             |   option value
   |             |   (subtract from Z-spread to get OAS)
   |   -------------
   |   OAS (credit / liquidity)
   |   -------------
   |   risk-free zero curve
   |
   +------------------------------> tenor
```

## Definition

The Z-spread is the constant spread `s_Z` such that
`P_observed = ∑_i c_i · D(t_i; z(t_i) + s_Z)` where
`D(·; z + s_Z)` is the discount factor under the zero
curve shifted by `s_Z`. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.290-310.

The Option-Adjusted Spread is computed from a stochastic
interest-rate model: simulate (or lattice-evaluate) the
short-rate path, apply the option's exercise rule at
each path-state, and find the constant spread `s_OAS`
that prices the option-aware cashflow stream to the
observed price. By construction, `s_OAS < s_Z` for
short-call bonds (callable, MBS) because the option
value contributes positively to `s_Z`. **Source:** CFA
L1 Curriculum (2022) Vol.5/pp.290-310;
Hull §10 pp.241-260.

Effective duration is computed by shocking the entire
zero curve up and down by `Δy`, repricing through the
option-aware model, and taking
`D_eff = (P_- - P_+) / (2 · P_0 · Δy)`. The shock-and-
reprice approach handles the non-linearity that the
analytical Macaulay derivation cannot. **Source:** CFA
L1 Curriculum (2022) Vol.5/pp.290-310.

## Mathematical Reasoning

OAS is well-defined provided the option-aware pricing
model is calibrated to today's market. A model
calibration mismatch contaminates the OAS calculation
because the difference between Z-spread and OAS is
attributed to "option value" — but if the model
under-estimates the option, OAS over-estimates the
credit residual. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.290-310.

Effective duration of a callable bond from
[`fi-callable-and-putable-bonds.md`](./fi-callable-and-putable-bonds.md#mathematical-reasoning)
is bounded above by the call-truncated horizon: as the
call option moves into the money, effective duration
collapses toward the time-to-call rather than time-to-
final-maturity. The negative-convexity signature is the
mirror of this duration compression. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.290-310;
Hull §10 pp.241-260.

For MBS, the effective duration combines the bond's
own price-yield response with the prepayment-driven
cashflow re-weighting from
[`fi-prepayment-risk-intuition.md`](./fi-prepayment-risk-intuition.md#mathematical-reasoning).
The effective duration of an MBS at par can be
materially shorter than the maturity-aligned vanilla
analog because prepayment acceleration absorbs part of
the rate sensitivity. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.290-310.

The basic duration view of
[`fi-duration-and-convexity.md`](./fi-duration-and-convexity.md#definition)
is the option-free baseline; OAS-effective-duration
machinery extends it to the option-embedded case. The
two coincide for vanilla bonds. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.290-310.

## See Also

- [`fi-callable-and-putable-bonds.md`](fi-callable-and-putable-bonds.md) — option-embedded bonds whose pricing the OAS framework supports
- [`fi-duration-and-convexity.md`](fi-duration-and-convexity.md) — option-free baseline duration / convexity
- [`fi-prepayment-risk-intuition.md`](fi-prepayment-risk-intuition.md) — MBS prepayment risk that OAS must absorb

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 45 directly when
any of the criteria below applies. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.290-310.

- A specific OAS-calibration model (Black-Karasinski,
  Hull-White, two-factor models) requires recipe-level
  detail. **Source:** Hull §10 pp.241-260.
- The bond combines call + prepayment + credit features
  and the OAS decomposition becomes ambiguous (which
  option's value goes into `s_Z - s_OAS`). **Source:**
  CFA L1 Curriculum (2022) Vol.5/pp.290-310.
- Convertible-bond OAS extends to dual underlying
  factors (rate + equity); the convertible-specific
  treatment lives in subcorpus 08. **Source:** Hull
  §10 pp.241-260.
