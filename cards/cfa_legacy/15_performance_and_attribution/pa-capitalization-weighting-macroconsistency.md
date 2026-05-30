---
schema_version: "cacg.v0"
id: "pa-capitalization-weighting-macroconsistency"
title: "Capitalization Weighting and Macroconsistency"
reading_id: "15_performance_and_attribution"
summary: "Index-weighting taxonomy (price/cap/equal/fundamental) plus float adjustment; only cap-weighting is macroconsistent (holdable by all investors at once), links to the CAPM market portfolio and low turnover, and makes a portfolio ratio a cap-weighted average."
tags: ["capitalization-weighting", "macroconsistency", "benchmark-construction"]
citations:
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p261:0261"
    chunk_hash: "1cf57524a6b9d1febb334c6d5ac8cb9482d596dfed3eabe9e62aee375197b1f9"
    page_range: [261, 262]
    quote: "With other weighting methods, not all investors could hold the index."
    edge_type: "defines"
card_hash: "89f601c6ac755cf57a527d16bfce5bc9b4771b4a03f38bb3f272a67c0fe6a95c"
---
# Capitalization Weighting and Macroconsistency

## Intuition

How an index distributes weight across its constituents is not a cosmetic detail: the weights *are* the return rule, so the gap between a manager's weights and the index's weights drives relative performance. Three families exist — price weighting (hold one share of each, so weight tracks price), capitalization weighting (hold each firm's outstanding shares, so weight tracks market cap), and "alternative" everything-else (equal weighting, fundamental weighting, volume weighting). Cap weighting earns its status as the default for one deep reason: it is the only scheme that *everyone can hold at the same time*. If all investors bought the same cap-weighted fund, every share would be absorbed with none left over — there is no aggregate consistency requirement that equal- or fundamental-weighting can satisfy.

**Source:** Christopherson, Cariño & Ferson (2009) Ch.22 pp.247-249

## Definition

**Weighting taxonomy.** In a *price-weighted* index the number of shares is 1 for every constituent, so weights are proportional to price (e.g. the Dow Jones Industrial Average). In *pure capitalization weighting* the index holds each firm's full outstanding share count, so price times shares is market cap and weight is proportional to market cap. *Float adjustment* removes shares that are illiquid or unavailable to the public, leaving *free-float shares*; the resulting *float-adjusted market cap* (or simply *float*) defines a float-weighted index, which is still a cap-weighted index. *Alternative weighting* covers equal weighting, volume weighting, and fundamental weighting (Arnott's RAFI uses a composite of sales, cash flow, book value, and dividends), each of which must recompute and rebalance share counts periodically.

**Macroconsistency.** Cap weighting has the property that if all investors held cap-weighted index funds and there were no active investors, all shares would be held with none left over; with other methods not every investor could hold the index. This makes a cap-weighted index a better representation of the *typical investor's opportunity set*, and it is one step from a central CAPM result: a cap-weighted portfolio of all stocks is the efficient market portfolio that all CAPM investors hold (combined with cash sized to risk tolerance).

**Source:** Christopherson, Cariño & Ferson (2009) Ch.22 pp.247-250

## Mathematical Reasoning

Two structural properties follow without any worked arithmetic.

**Low turnover (buy-and-hold invariance).** A change in *prices alone* does not add or remove shares from a cap-weighted index — after a price move the index is still cap-weighted, because both the numerator (price times shares) and the denominator (total market cap) move with price. Only corporate actions (mergers, spin-offs) and dividends force rebalancing trades. Every non-cap scheme requires periodic rebalancing to restore its target weights, incurring transaction costs that the essentially buy-and-hold cap-weighted strategy avoids.

**Portfolio ratio = cap-weighted average.** For a per-share ratio it is cleaner to invert (use book-to-price `BV/MV`). With total book value `BV = sum BV_n` and total market value `MV = sum MV_n`, the portfolio-level ratio decomposes as

```
 BV     sum BV_n      / MV_n   BV_n \              / BV_n \
---- = ---------- = sum| ----- * ----- |  =  sum w_n * | ----- |
 MV        MV         \ MV     MV_n /              \ MV_n /

  where  w_n = MV_n / MV   is the market-cap weight,
         BV_n/MV_n         is firm n's book-to-price ratio.
```

So the ratio for the portfolio as a whole is the **cap-weighted average** of the constituent ratios — not the equal-weighted average. Equal-weighting a per-share statistic (e.g. average price-to-book) cannot answer "what is total market value divided by total book value?", whereas cap-weighting can. The source asserts macroconsistency and the CAPM efficiency link by reference to Siegel and the original CAPM rather than re-deriving CAPM here; this card asserts likewise and labels the derivation as out of scope (see Escalate to Raw).

**Source:** Christopherson, Cariño & Ferson (2009) Ch.22 pp.248-252

## Boundary Notes

Cap weighting's disadvantages are not inconsequential and are scope caveats, not contradictions: large-cap managers often deliberately underweight single names for diversification (an implicit sector/stock bet), cap weighting tilts toward sectors more heavily than the average institutional manager, and during bubbles it carries the investor along with herding-driven prices. But the alternatives have their own theoretical defects — not every investor can hold an equal-weighted portfolio, and non-cap benchmarks make passive replication hard. The text's key point: a cap-weighted index is the *neutral* weighting, and all non-cap-weighted indexes should be benchmarked against it.

**Source:** Christopherson, Cariño & Ferson (2009) Ch.22 pp.250-253

## See Also

- [`pa-valid-benchmark-properties.md`](pa-valid-benchmark-properties.md) — macroconsistency and investability are inputs to what makes a benchmark valid.
- [`pa-normal-portfolio-construction.md`](pa-normal-portfolio-construction.md) — custom weighting schemes generalize the cap-weighting baseline into a manager-specific normal portfolio.
- [`pa-returns-based-style-analysis.md`](pa-returns-based-style-analysis.md) — style indexes against which a manager is decomposed inherit a cap-weighting convention.
- [`pa-benchmark-quality-validation-statistics.md`](pa-benchmark-quality-validation-statistics.md) — quality tests assume a neutral (cap-weighted) reference against which tilts are measured.

The CAPM market-portfolio result that macroconsistency points toward is developed in the pm-* portfolio-management cards; GIPS composite-benchmark disclosure (17 ethics GIPS) requires stating the benchmark's weighting basis.

## Escalate to Raw When

- You need the worked two-stock numerical example (caps 80 and 20, P/B of 4 and 10) showing equal-weighted P/B = 7 versus market-to-book ~= 4.5 — deferred per Critical Rule 1; see Table 22.1 on pp.252-253.
- You need the full CAPM derivation establishing the market portfolio's efficiency rather than the asserted one-step link from macroconsistency.
- You need the float-adjustment mechanics or corporate-action rebalancing rules for a specific index provider.
- You need Arnott's fundamental-indexation argument (RAFI composite, the claim that cap weighting systematically overweights expensive stocks) in full — pp.253 onward.
