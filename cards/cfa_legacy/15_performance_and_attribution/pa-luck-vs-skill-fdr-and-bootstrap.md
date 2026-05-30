---
schema_version: "cacg.v0"
id: "pa-luck-vs-skill-fdr-and-bootstrap"
title: "Skill vs Luck: False-Discovery Rate and the Bootstrap"
reading_id: "15_performance_and_attribution"
summary: "Across thousands of funds a naive count of significant alphas mistakes luck for skill; the FDR approach partitions the population into zero-alpha, unskilled, and skilled funds, and a residual bootstrap handles non-normal t-statistic tails."
tags: ["false-discovery-rate", "luck-vs-skill", "bootstrap"]
citations:
  - source_id: "pa_fischer_wermers_2013"
    chunk_id: "pa_fischer_wermers_2013:p253:0344"
    chunk_hash: "3eca1aeb0bf6a1e73b0b38e9374c44982a9a425e157a76e6c1902fcf87bbb5bc"
    page_range: [253, 253]
    quote: "precisely estimates the proportion of fund managers that are (1) unskilled, (2) zero-alpha, and (3) skilled, even with dependencies in cross-fund estimated alphas."
    edge_type: "defines"
card_hash: "b270b2d1a09f90919c3b890874c9281db6085fa08816139b31fce25ba863e58d"
---
# Skill vs Luck: False-Discovery Rate and the Bootstrap

## Intuition

When you evaluate one manager, a single t-test on alpha is enough. When you sift
through thousands of funds at once, the arithmetic of multiple testing turns
against you: even funds with truly zero alpha throw off "significant" estimates
purely by chance. Pick any fixed significance level and a predictable slice of
zero-alpha funds will land in the tails — lucky ones to the right, unlucky ones to
the left — and a naive count of significant-alpha funds reads these false
discoveries as real skill. The headline question "what fraction of managers can
actually pick stocks?" therefore cannot be answered by counting winners; it must
be answered by first subtracting the luck that the sample size guarantees.

**Source:** Fischer & Wermers (2013) §8.1 pp.251-252 (PDF pp.253-254)

## Definition

Partition a population of M actively managed funds into three categories defined
by stock-selection skill net of trading costs and expenses:

- **Unskilled funds:** managers whose skill is insufficient to recover costs, so
  true alpha is negative (alpha < 0) — an "alpha shortfall."
- **Zero-alpha funds:** managers whose skill just recovers costs (alpha = 0).
- **Skilled funds:** managers who deliver an "alpha surplus" beyond recovering
  costs (alpha > 0).

The population proportions are pi_0 (zero-alpha), pi_A^- (unskilled), and pi_A^+
(skilled). A **false discovery** is a zero-alpha fund whose *estimated* alpha is
significant by luck alone. The False-Discovery Rate (FDR) approach — distilled
from Barras, Scaillet & Wermers (2010) — precisely estimates these proportions
and the location of skilled/unskilled funds in the tails, using only the
per-fund alpha p-values. Its one essential input is pi_0, estimated directly from
the cross-section of p-values rather than imposed as a prior.

**Source:** Fischer & Wermers (2013) §8.2.1.1 pp.252-253 (PDF pp.254-255)

## Mathematical Reasoning

The performance measure is the t-statistic t_i = alpha_hat_i / sigma_hat(alpha_i),
which KTWW (Kosowski, Timmermann, Wermers & White) show dominates raw alpha
because alpha estimates differ in precision across funds of varying lives and
volatilities. Applying a significance level gamma simultaneously across all funds
is a multiple hypothesis test H_{0,i}: alpha_i = 0 versus H_{A,i}: alpha_i != 0
for i = 1, ..., M.

The key identity: at level gamma a zero-alpha fund is "lucky" (positive and
significant) with probability gamma/2, so the expected proportion of lucky funds
is

    E(F_gamma^+) = pi_0 * gamma / 2.

The expected proportion of *truly skilled* funds is the observed significant-right
count net of luck:

    E(T_gamma^+) = E(S_gamma^+) - E(F_gamma^+) = E(S_gamma^+) - pi_0 * gamma / 2,

and symmetrically for the left tail,

    E(T_gamma^-) = E(S_gamma^-) - pi_0 * gamma / 2,

since the unlucky probability also equals gamma/2. As gamma is widened, E(T_gamma^+)
and E(T_gamma^-) converge to the true population proportions pi_A^+ and pi_A^-,
minimizing Type II error; evaluating these equations at several gamma values
reveals *where* skilled funds sit in the tail (concentrated in the extreme right
versus dispersed). The book derives these decomposition identities directly; it
asserts the convergence-to-true-proportions result without a formal proof, so this
card states it as asserted.

A normality caveat closes the loop: the individual-fund t-statistic distributions
are non-normal for most U.S. domestic equity funds, so the empirical
implementation replaces the analytic normal tails with a **residual bootstrap**
(KTWW) to estimate each fund's t-distribution and the associated p-values feeding
the FDR computation.

**Source:** Fischer & Wermers (2013) §8.2.1.1-8.2.1.2 pp.254-256 (PDF pp.256-258)

```
          Cross-sectional estimated-alpha t-distribution (a mixture)
                  weights = pi_A^- , pi_0 , pi_A^+

  left tail (t < t_gamma^-)              right tail (t > t_gamma^+)
  ---------------------------            ---------------------------
  E(S_gamma^-) = significant-neg         E(S_gamma^+) = significant-pos
        |                                       |
        |  subtract luck pi_0*gamma/2           |  subtract luck pi_0*gamma/2
        v                                       v
  E(T_gamma^-) truly UNSKILLED           E(T_gamma^+) truly SKILLED
  (false discoveries removed)            (false discoveries removed)

  central spike: pi_0 ZERO-ALPHA funds -> source of all false discoveries
```

**Source:** Fischer & Wermers (2013) §8.2.1 pp.254-256 (PDF pp.256-258)

## Boundary Notes

The skill definition is *relative to expenses* (surplus alpha net of trading
costs and all fees, except loads and taxes), not skill in an absolute sense; the
book separately redefines skill net of trading costs only in a later section. The
approach also assumes the rare cases (a skilled fund that is very unlucky, or an
unskilled fund that is very lucky) are negligible and ignores them.

**Source:** Fischer & Wermers (2013) §8.2.1.1 pp.253-255 (PDF pp.255-257)

## See Also

- [`pa-regression-appraisal-jensen-treynor.md`](pa-regression-appraisal-jensen-treynor.md) — the single-fund alpha/t-statistic regression that FDR aggregates across the population.
- [`pa-multifactor-alpha-timing-conditional.md`](pa-multifactor-alpha-timing-conditional.md) — the four-factor model whose alphas feed the cross-sectional FDR partition.
- [`pa-dgtw-cs-ct-as-decomposition.md`](pa-dgtw-cs-ct-as-decomposition.md) — characteristic-based skill decomposition, an alternative lens on the same selection-skill question.
- [`pa-variability-ratios-sharpe-information.md`](pa-variability-ratios-sharpe-information.md) — ratio-based skill summaries that, like a naive significant-alpha count, do not adjust for multiple-testing luck.

Related portfolio-management framing on alpha and active skill lives in the pm-*
cards (active vs passive, information ratio); the multiple-testing discipline here
mirrors the backtesting-and-exceedance cautions in the rm-* risk cards.

## Escalate to Raw When

- You need the worked numerical illustration (pi_0 = 75%, pi_A^- = 23%,
  pi_A^+ = 2%, gamma = 0.10 giving E(F_gamma^+) = 3.75% and E(T_gamma^+) = 1.85%)
  — read pp.255-256 (PDF pp.257-258) directly; this card omits the arithmetic per Critical Rule 1.
- You need the Monte Carlo accuracy and cross-sectional-dependence robustness
  evidence, or the empirical 1996-vs-2006 skilled-fund decline.
- You need the residual-bootstrap algorithm details or the absolute-skill
  redefinition (net of trading costs only) and its retests.
