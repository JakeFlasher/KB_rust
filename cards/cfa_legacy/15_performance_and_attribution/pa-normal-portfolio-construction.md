---
schema_version: "cacg.v0"
id: "pa-normal-portfolio-construction"
title: "Normal-Portfolio Construction"
reading_id: "15_performance_and_attribution"
summary: "A normal portfolio is the Barra-origin habitat of securities a manager normally chooses, weighted as they normally would. Built in three steps (beginning universe, screen, weight), it separates style return from stock-selection skill."
tags: ["normal-portfolio", "custom-benchmark", "style-vs-skill"]
citations:
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p236:0232"
    chunk_hash: "d46794767d704d03ff2bdbf9d0385c80cbca0bb14432a83a259a784e1f0390bf"
    page_range: [236, 237]
    quote: "It is not clear that there should be a unique or single normal portfolio for a manager."
    edge_type: "defines"
---
# Normal-Portfolio Construction

## Intuition

A broad market index is the wrong yardstick for a specialist. A defensive-stock
manager beats the S&P 500 in bear markets and trails it in bull markets purely
because of *where* they fish, not because of *which fish* they caught — so the
index conflates the manager's style with the manager's skill. A style index
gets closer, but it still holds names the manager would never touch and in
proportions they would never hold. The **normal portfolio** is the tightest
fix: a custom benchmark built to mimic the manager's own "habitat" of securities
as closely as simple selection rules allow. Beat your normal portfolio and you
have demonstrated stock-selection (and sector-allocation) skill *net of* your
style; fail to beat it and the fund "should" just hold the normal passively and
keep the active fee. It is, by design, the most difficult benchmark for a
managed portfolio to outperform.

**Source:** Christopherson, Cariño & Ferson (2009) Ch.20 (Benchmarks and Knowledge) pp.223-225 (PDF pp.236-238)

## Definition

The notion was first introduced by the Barra organization. The word *normal*
captures the idea that for each manager there exists a habitat of securities
whose composition is very similar to the manager's average portfolio over time —
the long-term "typical" or "average" portfolio. Formally:

> "A normal portfolio is a set of securities that contains all of the securities
> from which a manager normally chooses, weighted as the manager would normally
> weight them in a portfolio."

As such a normal portfolio is a *specialized index*. Its object is to improve
understanding of a manager's activities by comparing performance against a
passive alternative that approximately matches what the manager actually does.
The text notes there need not be a *unique* normal portfolio: there is more than
one reasonable "average" one could target, so more than one defensible normal
could be built — a customised-versus-generic choice, not a single right answer.

A normal portfolio is constructed in three steps:

1. **Beginning universe of securities.** Capture the stocks from which the
   manager normally chooses and on which information is reliably available.
   Capitalization is an easy first cut (e.g. a large-cap manager starts from the
   Russell 1000). A too-narrow list can exclude important names the manager holds.
2. **Choosing securities (screening).** Reduce the universe to the subuniverse
   the manager actually selects from, using screening criteria consistent with
   the manager's stock-selection habits (capitalization, yield, price/book,
   earnings growth, beta, earnings variability, etc.). Decision rules give a
   numerical basis for inclusion. There are no unambiguous rules for setting
   screen values; the only good check is the *reasonableness* of the resulting
   subuniverse given the manager's style.
3. **Weighting the securities.** Decide how to combine the subuniverse into an
   index. Broad indexes are cap- or float-weighted, but most active managers tilt
   toward equal weighting — their conviction does not rise with a stock's
   capitalization. Using the correct weighting scheme "does matter enormously."
   (Rebalancing is typically quarterly or semiannually.)

**Source:** Christopherson, Cariño & Ferson (2009) Ch.20 (Normal Portfolios) pp.223-227 (PDF pp.236-240)

## Mathematical Reasoning

Let the manager's portfolio hold weights $w_i$ on securities with returns $r_i$,
and let the normal portfolio assign normal weights $n_i$ to the same screened
subuniverse. Portfolio and normal returns are the weight-return inner products

$$ R_P = \sum_i w_i\, r_i, \qquad R_N = \sum_i n_i\, r_i, \qquad \sum_i w_i = \sum_i n_i = 1 . $$

The value the manager adds *net of style* is the excess over the normal, which
decomposes by adding and subtracting the normal-weighted return into a
weighting/selection component and a residual:

$$ R_P - R_N = \sum_i (w_i - n_i)\, r_i . $$

This is the formal counterpart of the book's assertion that "if his portfolio
return is worse than his normal universe return, then the manager made mistakes
in choosing stocks, in departing from his normal weighting scheme, in choosing
sectors, or all three." Each addend $(w_i - n_i) r_i$ is an active bet relative
to the manager's *own* habitat, not relative to the broad market.

Why the normal is a *tighter* yardstick than the broad market index $R_M$ with
weights $m_i$: write the broad-index excess as the normal excess plus a pure
style term,

$$ R_P - R_M = \underbrace{(R_P - R_N)}_{\text{net of style}} + \underbrace{(R_N - R_M)}_{\text{style return}} . $$

The second bracket — the normal's return over the broad market — is style return
the broad-index comparison wrongly attributes to skill. Subtracting $R_N$ removes
it. The text supports this empirically (not by proof): it *expects* a higher
correlation between the manager's return and the normal than with the broad
market, and a *lower residual variance* relative to the normal — these are stated
expectations used to judge a normal's reasonableness, not derived theorems, and
are asserted here at the source's level of rigor.

The text also flags a screening **limitation** as an assumption, not a result:
the methodology assumes the manager's selection universe can be captured by
fundamental characteristics in screens. Qualitative theses ("a catalyst for an
earnings turnaround", "the stock has a franchise") and dynamic market-timing
bets (moving into cash) cannot be captured by an equities-only static screen.

**Source:** Christopherson, Cariño & Ferson (2009) Ch.20 (Normal Portfolios) pp.225-228 (PDF pp.238-241)

```
  THREE-STEP NORMAL-PORTFOLIO BUILD          what each step removes
  ----------------------------------          --------------------------------
  [1] BEGINNING UNIVERSE                       drops names with no reliable
      (e.g. Russell 1000, cap cut)             data / outside the habitat
            |
            v
  [2] SCREEN  (yield, P/B, growth,             narrows broad index -> the
      beta, earnings variability ...)          subuniverse manager selects from
            |
            v
  [3] WEIGHT  (often equal, not cap)           matches manager's conviction
      + rebalance quarterly/semiannually       structure, not market cap
            |
            v
        NORMAL PORTFOLIO  R_N  ===  the manager's "style" priced passively

  R_P - R_M  =  (R_P - R_N)   +   (R_N - R_M)
                 skill, net of      pure STYLE return
                 style (subtract     (what the broad index
                 to isolate)         miscredits as skill)
```
**Source:** Christopherson, Cariño & Ferson (2009) Ch.20 (Normal Portfolios) pp.223-228 (PDF pp.236-241)

## Boundary Notes

This card covers what a normal portfolio *is* and how it is built. The broader
question of *which* benchmark properties make any benchmark valid (including a
normal) — investability, unambiguousness, measurability, appropriateness,
specified-in-advance — lives in the valid-benchmark-properties sibling, and the
purely statistical tests of a benchmark's *quality* (tracking error, correlation,
coverage) live in the benchmark-quality-validation sibling. Returns-based style
analysis is the regression-based *alternative* to holdings-based normals for
inferring a manager's style. The customised- versus generic-normal trade-off
(how close to the manager's ideal portfolio to come, and the incentive effects of
raising the benchmark bar) is discussed by the source but is a design judgment,
not a construction rule.

**Source:** Christopherson, Cariño & Ferson (2009) Ch.20 (Normal Portfolios) pp.228-229 (PDF pp.241-242)

## See Also

- [`pa-valid-benchmark-properties.md`](pa-valid-benchmark-properties.md) — the validity criteria a normal portfolio must satisfy to serve as a benchmark.
- [`pa-returns-based-style-analysis.md`](pa-returns-based-style-analysis.md) — regression-based alternative to a holdings-based normal for inferring manager style.
- [`pa-benchmark-quality-validation-statistics.md`](pa-benchmark-quality-validation-statistics.md) — statistical tests (correlation, residual variance, coverage) that judge how well a normal fits its manager.
- [`pa-capitalization-weighting-macroconsistency.md`](pa-capitalization-weighting-macroconsistency.md) — the cap- vs equal-weighting debate that drives the normal's step-3 weighting choice.

Related cross-vertical: pm-* portfolio-management cards develop active risk and the active-management mandate that a normal portfolio is meant to appraise; 17 ethics/GIPS material constrains how such custom benchmarks must be disclosed in compliant performance presentations.

## Escalate to Raw When

- You need the worked **Table 20.1** normal-portfolio specification (the growth-firm screen: cap > $2.5bn, yield < 2%, P/B < 2, EPS 5-yr growth < 9%, historical beta > 0.85, earnings variability > -0.5 std; the 50/50 equal-vs-cap weighting bands with the 5% IBM cap) — deferred here per the no-worked-arithmetic rule.
- You need the original Barra/Kritzman methodology ("How to Build a Normal Portfolio in Three Easy Steps", *The Normbook*) or the Divecha–Grinold philosophical treatment cited in the source's footnotes.
- You need the full discussion of the customised- versus generic-normal trade-off, the "are we stealing the manager's alpha?" incentive argument, and "for what should a manager be given credit?" (Ch.20 pp.228-229 / PDF pp.241-242).
