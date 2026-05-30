---
schema_version: "cacg.v0"
id: "pa-valid-benchmark-properties"
title: "Properties of a Valid Benchmark"
reading_id: "15_performance_and_attribution"
summary: "Christopherson, Cariño & Ferson ground a 'desirable benchmark' in four principles (naïve alternative, completeness, simplicity, representative weighting) plus named sub-characteristics—investability, believability, fairness, openness, reliability—and the trade-offs among them."
tags: ["benchmark-quality", "valid-benchmark", "index-construction"]
citations:
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p251:0249"
    chunk_hash: "8b7a6bd9f8b3bfa3084beacd77925de5a6ee9eaf05289c03456f53c85e9916d2"
    page_range: [252, 252]
    quote: "Given the purposes of a benchmark, a benchmark should include all the assets available for investment."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p252:0250"
    chunk_hash: "936a1b31b06237a677dd30a95e2424c748b166b2f124b799e81076ffd88b43d9"
    page_range: [252, 252]
    quote: "This characteristic can be called investability."
    edge_type: "supports"
---
# Properties of a Valid Benchmark

## Intuition

A benchmark is the yardstick against which active skill is judged, so it must measure what a "naïve" investor could have captured without special knowledge. CCF argue that "best" is partly subjective—"the characteristics of a good benchmark are, like beauty, in the eye of the beholder"—because different index users (traders, index funds, individuals, institutions, active managers, index providers) have conflicting agendas. A valid benchmark is one whose construction serves the investor doing the evaluating, not the party with a vested interest. Note: CCF do NOT use the CFA-curriculum SAMURAI mnemonic; their organizing scheme is the four principles below with named sub-characteristics.

**Source:** Christopherson, Cariño & Ferson (2009) Ch. 21 printed pp.235-238 (PDF pp.248-251)

## Definition

CCF organize "the best index" around **four principles of useful indexes**, each spawning named sub-characteristics:

| Principle | Statement | Named sub-characteristics |
|-----------|-----------|---------------------------|
| I — A "Naïve" Alternative | Yields the return/risk an investor could obtain without extraordinary knowledge of the opportunity set | naïve investor / rules based |
| II — Completeness | Should include all the assets available for investment | investability; intuitive believability; fairness |
| III — Simplicity | Constructed in a transparent method that can be replicated by users | simplicity; no proprietary methods or data; openness and clarity; reliability |
| IV — Representative Weighting | Most benchmarks should be capitalization weighted, then float adjusted when possible | float adjusted; accepts market participants' weights implicit in capitalization |

CCF define the sub-characteristics directly: an index that "should reflect the free float or amount of capitalization readily available for purchase"—"This characteristic can be called investability." Believability means the index is "built according to rules widely accepted in the investment community." Fairness means it is "built to minimize as much as possible the advantage of any one class of investors over another." Openness/clarity means "clear unambiguous published rules for construction that can be replicated by a competent analyst." Reliability means "complete and reliable data can be obtained repetitively in a timely manner."

**Source:** Christopherson, Cariño & Ferson (2009) Ch. 21 printed pp.238-241 (PDF pp.251-254)

## Mathematical Reasoning

The principles are not all simultaneously satisfiable. CCF do not prove a formal optimization result; they argue by example that pursuing one property can come at the expense of another, so the card reports their qualitative trade-offs and labels the gap. The book identifies two structural conflicts:

- **Completeness vs investability.** Pushing for full coverage tends to pull thinly traded names into the index that cannot be transacted at reliable prices, so more completeness can come at the cost of investability. CCF's example is the Wilshire 5000, whose smallest names index funds found "essentially untradable."
- **Representativeness vs simplicity.** Keeping the index faithful to an evolving style segment calls for more frequent reconstitution, and reconstitution adds turnover and transaction cost, which works against simplicity. CCF state the direction qualitatively: "The more frequent the reconstitution, the greater are the transaction costs inherent in tracking the index."

The trade-off is illustrated rather than optimized: because the smallest, least liquid names are the ones that completeness would add and that investability would exclude, CCF describe the designer as choosing a workable middle ground rather than a corner — Russell built the Russell 3000 rather than a Russell 6000 to balance coverage against accessibility.

**Source:** Christopherson, Cariño & Ferson (2009) Ch. 21 printed pp.241-243 (PDF pp.254-256)

```
        FOUR PRINCIPLES  ->  named sub-characteristics
        ----------------------------------------------
        I  Naive alt.    ->  rules-based
        II Completeness  ->  investability -+
                            believability   | trade-off
                            fairness         |   (completeness vs investability)
        III Simplicity   ->  openness ------+
                            no-proprietary
                            reliability ----+ trade-off
        IV Representative->  float-adjusted  | (representativeness vs simplicity)
                            cap-weighted ---+
```

**Source:** Christopherson, Cariño & Ferson (2009) Ch. 21 printed pp.238-243 (PDF pp.251-256)

## Boundary Notes

CCF's four-principle scheme is a benchmark-*quality* taxonomy; it does not by itself confirm that a chosen benchmark *fits a given manager's process*. Statistical fit tests (tracking quality, style coverage) live in a separate workflow, and the curriculum's SAMURAI list (Specified in advance, Appropriate, Measurable, Unambiguous, Reflective of opinions, Accountable, Investable) is a parallel CFA mnemonic—overlapping but not identical—and is NOT CCF's terminology.

**Source:** Christopherson, Cariño & Ferson (2009) Ch. 21 printed pp.238-241 (PDF pp.251-254)

## See Also

- [`pa-normal-portfolio-construction.md`](pa-normal-portfolio-construction.md) — building a custom benchmark (normal portfolio) when a standard index fails the four principles.
- [`pa-benchmark-quality-validation-statistics.md`](pa-benchmark-quality-validation-statistics.md) — the statistical fit tests that complement the qualitative properties here.
- [`pa-capitalization-weighting-macroconsistency.md`](pa-capitalization-weighting-macroconsistency.md) — why Principle IV defaults to float-adjusted capitalization weighting.
- [`pa-returns-based-style-analysis.md`](pa-returns-based-style-analysis.md) — inferring the benchmark/style a manager actually tracks. Related cross-vertical: GIPS composite-benchmark disclosure rules (17 ethics GIPS).

## Escalate to Raw When

- You need the worked S&P 500 / Russell coverage and correlation figures (e.g., the 0.99 Wilshire–Russell correlation, the ~75% vs 98% market-cap capture) used to illustrate the trade-offs—these numeric examples are in the source, not reproduced here per Critical Rule 1.
- You must arbitrate between CCF's four-principle scheme and the CFA SAMURAI mnemonic for an exam answer, or map a specific real index against each sub-characteristic.

**Source:** Christopherson, Cariño & Ferson (2009) Ch. 21 printed pp.232-243 (PDF pp.245-256)
