---
schema_version: "cacg.v0"
id: "ec-permanent-income-consumption"
title: "Permanent Income Hypothesis and Consumption under Uncertainty"
reading_id: "02_economics"
summary: "Friedman's permanent-income hypothesis says consumption is determined by lifetime resources (permanent income), not period income; Hall's (1978) extension under quadratic utility and rational expectations gives the Random Walk result C_t = C_{t-1} + e_t where consumption-change innovations are unpredictable from any t-1 information."
tags: ["economics", "permanent-income"]
citations:
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p389:0552"
    chunk_hash: "93a1aa8afe8b32aeac1b7cf81e244007318407131f5f3e256bcc979bf98f126e"
    page_range: [389, 389]
    quote: "In the terminology of Friedman (1957), the right-hand side of (8.5) is permanent income, and the difference between current and perma"
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p396:0561"
    chunk_hash: "92a1873190a42da87aa5b38c14388de6b0679c7f577a4de6b6d179bd78cb45d8"
    page_range: [396, 396]
    quote: "This is Hall’s famous result that the permanent-income hypothesis implies that consumption follows a random walk (Hall, 1978)."
    edge_type: "defines"
card_hash: "359e48891a55a8855f1be046fa654fc0e18c50d79fb4a432cb5fe00ac045d63a"
---
# Permanent Income Hypothesis and Consumption under Uncertainty

## Intuition

The **Permanent Income Hypothesis (PIH)**, originating with Friedman (1957) and re-derived in modern DSGE form by Hall (1978), says that rational consumers smooth consumption over a long horizon by responding only to **permanent** changes in expected lifetime income, ignoring **transitory** income shocks (which are buffered by saving or borrowing). Under quadratic utility and rational expectations, this implies the **Random Walk result**: consumption changes `Δc_t = c_t − c_{t-1}` are unpredictable from any information dated `t − 1` or earlier — consumption follows a martingale. **Source:** Romer (2019) Ch.8 pp.368-395.

```
   PIH: response to permanent vs transitory income shock

   permanent shock (level shift):   transitory shock (one-period spike):
   Y                                Y
   ^   _______                      ^
   |  /                             |  *
   | /                              | /\
   |/                               |/  \____ (back to baseline)
   +--+---------> t                 +--+-----> t
      0                                0

   c                                c
   ^   _______                      ^
   |  /                             |  ___
   | /                              | /   \____ small bump (annuity value
   |/  (matches Y)                  |/         of one-period shock)
   +--+---------> t                 +-+--------> t
      0                                0

   permanent shock → full           transitory shock → minimal c response
   one-for-one c response           (buffered by saving in period 0)
```

The PIH's empirical predictions are sharp: (i) `Δc_t` is uncorrelated with any lagged information, including past income changes, past wealth, past announcements; (ii) consumption response to a "permanent" shock equals the full shock; (iii) consumption response to a "transitory" shock equals the annuity value of the shock (small). The empirical literature has found systematic violations of (i) — **excess sensitivity** of consumption to predictable income changes — and of (iii) — **excess smoothness** of consumption to large income shocks. These puzzles motivate buffer-stock-savings, liquidity-constraint, and habit-formation extensions. **Source:** Romer (2019) Ch.8 pp.395-419.

## Definition

The **stochastic Euler equation** under uncertainty is derived from the household's intertemporal optimization with a stochastic income process `y_t`: **Source:** Romer (2019) pp.368-419.

```
u'(c_t)  =  β · (1+r) · E_t [ u'(c_{t+1}) ]        [stochastic Euler]
```

Under **quadratic utility** `u(c) = c − (a/2) · c^2` (a tractable closed-form case), the marginal utility is linear: `u'(c) = 1 − a · c`. Substituting into the Euler equation gives `c_t = β · (1+r) · E_t [c_{t+1}]` (up to constants). Imposing `β · (1+r) = 1` (the natural case where the household is patient enough to want to smooth and the rate compensates for impatience) yields **Hall's Random Walk result**: **Source:** Romer (2019) pp.368-419.

```
c_{t+1}  =  c_t  +  ε_{t+1}        where  E_t[ε_{t+1}] = 0    [Hall 1978 RW]
```

Consumption is a martingale — no information dated `t` or earlier predicts the consumption change. The innovation `ε_{t+1}` captures the household's revision of permanent income when new information arrives in period `t + 1`. **Source:** Romer (2019) Ch.8 pp.380-395.

The **closed-form PIH consumption rule** under quadratic utility, infinite horizon, and zero-interest equality `β(1+r) = 1` is: **Source:** Romer (2019) pp.368-419.

```
c_t  =  r / (1+r)  ·  [ a_t  +  ∑_{j=0}^∞  (1+r)^(−j) · E_t y_{t+j} ]
                       [             permanent income            ]
```

Consumption equals the annuity value of current wealth `a_t` plus the present value of expected future labor income — the "permanent income." A permanent unit increase in expected `y` raises permanent income by approximately one and raises consumption by one; a one-shot transitory unit increase in `y_t` raises permanent income by only `r / (1+r)` (the annuity value) and raises consumption by that amount. **Source:** Romer (2019) Ch.8 pp.395-410.

## Mathematical Reasoning

The **Hall Random Walk** derives from combining the stochastic Euler equation with the linearity of marginal utility under quadratic utility and the rate-discount equality `β(1+r) = 1`. The Euler equation `u'(c_t) = β(1+r) · E_t u'(c_{t+1})` becomes `u'(c_t) = E_t u'(c_{t+1})`, so marginal utility is a martingale: `u'(c_{t+1}) = u'(c_t) + η_{t+1}` for some innovation `η`. For quadratic utility `u'(c) = 1 − ac`, this translates one-for-one to consumption itself: `c_{t+1} = c_t + ε_{t+1}` with `E_t ε_{t+1} = 0`. The Random Walk result depends critically on the quadratic-utility assumption — under CRRA utility (sibling `ec-utility-and-choice-under-uncertainty`), the stochastic Euler equation does not collapse to a Random Walk because marginal-utility curvature interacts with the variance of consumption (giving rise to a precautionary-saving motive). **Source:** Romer (2019) Ch.8 pp.380-395.

The **excess-sensitivity puzzle** is the empirical finding that consumption *does* respond to predictable changes in income (e.g., predictable Social Security checks, predictable tax refunds, predictable retirement income drops). Under PIH, all of these should already be reflected in current consumption via the present-value sum of expected income, so the actual realization should produce no consumption change. The leading explanations are: (a) **liquidity constraints** (the household cannot borrow against future income, so consumption tracks current income); (b) **myopia / hyperbolic discounting** (behavioral; future v12 Behavioral Finance); (c) **buffer-stock saving** (Carroll-Deaton precautionary motive under impatience and income uncertainty — households want a buffer stock against bad shocks rather than smoothing perfectly). **Source:** Romer (2019) Ch.8 pp.395-419.

The **excess-smoothness puzzle** is the opposite empirical finding: under PIH, an unanticipated permanent change in income should produce a one-for-one consumption response, but empirically the consumption response to large income innovations is **smaller** than PIH predicts. The leading explanation is that households are uncertain whether a given income innovation is permanent or transitory (signal-extraction problem); they wait for confirmation before fully adjusting consumption. The signal-extraction story is consistent with PIH at the underlying-true-permanent-income level but adds an information-friction layer on top. **Source:** Romer (2019) Ch.8 pp.395-419.

## See Also

- [`ec-ramsey-cass-koopmans-savings`](./ec-ramsey-cass-koopmans-savings.md) — deterministic Ramsey Euler equation that PIH extends to stochastic income
- [`ec-utility-and-choice-under-uncertainty`](./ec-utility-and-choice-under-uncertainty.md) — vNM expected-utility framework that the stochastic Euler equation uses
- [`ec-real-business-cycle-theory`](./ec-real-business-cycle-theory.md) — RBC framework where the representative household solves a PIH-like problem under productivity uncertainty

## Escalate to Raw When

The full derivation of the closed-form PIH consumption rule under quadratic utility, the Random Walk test of Hall's hypothesis, and the formal excess-sensitivity / excess-smoothness regression specifications are in Romer Ch.8 pp.380-410. The Carroll buffer-stock model with CRRA utility and stochastic income (and the asymptotic-impatience condition that makes consumption track income closely under impatience) is in Romer Ch.8 §8.7 pp.410-419. The Campbell-Mankiw rule-of-thumb consumer extension and the Bewley-style heterogeneous-agent precautionary-saving literature are graduate-macro research material out of v10 scope. The consumption-CAPM bridge (where the stochastic Euler equation becomes the basis for asset pricing) is in 09 (`pm-stochastic-discount-factor-intuition`). **Source:** Romer (2019) pp.368-419.

The DEC-4 BOUNDARY-DISCIPLINE for this card: while the stochastic Euler equation `u'(c_t) = β(1+r) · E_t u'(c_{t+1})` is also the foundation for consumption-based asset pricing (the consumption-CAPM and stochastic-discount-factor framework in 09 Portfolio Management), this card stays on the household-side consumption-smoothing interpretation. The asset-pricing dual — using the same Euler equation to price risky claims via `1 = E_t[ M_{t+1} · R_{t+1} ]` with `M = β · u'(c_{t+1}) / u'(c_t)` — is the canonical content of the 09 cards (`pm-stochastic-discount-factor-intuition`, `pm-capm-and-sml`), cross-linked here via `Repo touchpoints:` without re-derivation. This boundary preservation is the same discipline applied across the v10 plan to keep 02 cards focused on real-economy macroeconomics and 09 cards focused on asset-pricing applications. **Source:** Romer (2019) pp.368-419.
