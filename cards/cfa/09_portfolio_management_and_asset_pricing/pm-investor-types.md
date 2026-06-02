---
schema_version: "cacg.v0"
id: "pm-investor-types"
title: "Investor Types"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Investor Types: surveying the retail-vs-institutional investor spectrum at L1 depth — what each category looks like, how each typifies the IPS objective and constraint profile, and where deeper investor-type taxonomy lives"
tags: ["portfolio-management", "investor-types", "institutional"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3055:4566"
    chunk_hash: "7e477d9c6bc3f894c73d49c94e7ee7e0e2ca0c92a0e1f4c998f149befb51d7a6"
    page_range: [3055, 3056]
    quote: "Each of these segments has distinctive characteristics and needs, as discussed in the following sub-sections."
    edge_type: "supports"
card_hash: "ec39c50b21eafcb682d4b7824978aaf55b92d8bec241168b13569129e59c7e67"
---
# Investor Types

## Intuition

Investors come in two broad categories at L1 framing: retail
(individuals and households) and institutional (pension funds,
foundations, endowments, insurance companies, banks, sovereign
wealth funds). The categories differ along three dimensions: the
size and composition of the asset base, the structure of the
liability or spending need, and the regulatory or governance regime
the investor operates under. Each category produces a typical IPS
profile — characteristic ability and willingness to bear risk,
characteristic horizon, characteristic constraint families that
bind. The L1 card surveys the spectrum; deeper per-type
construction belongs in the wealth and institutional vertical.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

```
       investor spectrum
       =================

       retail              <-- household / individual
         |                     - small-to-medium AUM
         |                     - taxable typically
         |                     - finite horizon (life cycle)
         |                     - personal liability profile
         |
       institutional       <-- pooled / mandate-bound
         |
         +-- DB pension        - long-horizon liability stream
         +-- DC pension        - participant-driven choice
         +-- endowment         - perpetual horizon, spend rate
         +-- foundation        - mission-driven distribution
         +-- insurance         - liability-matched, regulated
         +-- bank              - net-interest-margin focus
         +-- sovereign wealth  - macroeconomic mandate
```

The retail-vs-institutional split is the foundational L1 division
because the two require materially different IPS-construction
techniques and operate under different governance frameworks. Within
the institutional category, the subtypes differ enough that a
specialist IPS template applies to each — the L1 framing
distinguishes them at intuition level only. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

## Definition

A retail investor is an individual or household whose portfolio
funds personal consumption goals — retirement income, education,
home purchase, bequest. The asset base is owned by the investor;
the IPS is constructed around the household's life cycle and
personal preferences. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

```
retail:  individual / household / family
         portfolio funds personal consumption goals
         IPS shaped by life-cycle stage and tax status
```

An institutional investor is a pooled or mandate-bound entity that
invests on behalf of beneficiaries, members, or a stated mission.
The asset base is held in trust; the IPS is constructed around the
mandate, the liability or spending need, and the governance and
regulatory framework. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

```
institutional:  pension fund (DB or DC) / endowment / foundation /
                 insurance general account / bank treasury /
                 sovereign wealth fund
                 IPS shaped by mandate, liability profile, regulator
```

The L1 framing distinguishes the institutional subtypes by their
liability structure and horizon. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.420-441.

A defined-benefit (DB) pension fund pays a contractual stream of
retirement benefits to plan participants. The portfolio's job is
to fund the projected benefit payment stream; the liability is
long-duration and inflation-sensitive depending on the contract.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

A defined-contribution (DC) pension fund is a participant-directed
account where each participant chooses their own allocation from
sponsor-curated options. The sponsor's IPS sets the menu rather
than the allocation. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

An endowment is a perpetual-horizon institution (typical for
universities and cultural institutions) that funds a steady stream
of operating expenses from the asset base. The L1 framing notes
that the perpetual horizon supports a high-equity allocation
because the institution can absorb drawdowns over very long
recovery periods. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

A foundation is a mission-driven entity (often grant-making) that
funds programmatic distributions per its charter. Foundations may
be perpetual or limited-life depending on the founder's intent.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

An insurance company holds a general-account portfolio whose job
is to fund the insurer's policy liabilities. The liability profile
varies by line of business: life insurance liabilities are long-
duration and inflation-sensitive; property-casualty liabilities are
shorter-duration but more tail-risk concentrated. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

A bank holds a treasury portfolio that supports asset-liability
management for the deposit / loan balance sheet. The L1 framing
notes that bank portfolio construction is dominated by net-interest-
margin and liquidity-coverage considerations rather than total-
return optimization. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

A sovereign wealth fund holds national-savings or commodity-rents
assets on behalf of a state. Mandates vary widely by fund: some
target stabilization (smoothing fiscal volatility), some target
inter-generational savings, some target development. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.420-441.

## Mathematical Reasoning

The L1 framing produces typical IPS-objective profiles for each
type. The mathematical reasoning here is about which IPS
parameters tend to which values across types — a categorical
mapping rather than a derivation. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.420-441.

For DB pensions, the return objective is shaped by the actuarial
discount rate applied to the liability stream; the risk objective
is shaped by the funding-status volatility tolerance; the horizon
is the duration of the projected benefit stream. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

```
DB pension:  R_target  ≈ actuarial discount rate
              risk_obj  ≈ tolerance for funding-ratio volatility
              horizon   ≈ liability duration (typically long)
```

For endowments, the return objective is shaped by the spending
rate plus expected long-run inflation; the risk objective is
shaped by the spending-stream-stability tolerance; the horizon is
perpetual. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

```
endowment:  R_target  ≈ spending_rate + long_run_inflation
              risk_obj  ≈ tolerance for spending volatility
              horizon   ≈ perpetual (very long)
```

For insurance general accounts, the return objective is shaped by
the policy-liability discount rate plus the desired margin; the
risk objective is shaped by solvency capital requirements; the
horizon matches the liability-stream duration. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

```
insurance:  R_target  ≈ liability discount rate + margin
              risk_obj  ≈ regulated by solvency capital framework
              horizon   ≈ matches policy liability duration
```

For retail investors, the L1 typology distinguishes by life-cycle
stage. Early-career retail investors typically have long horizons,
high human-capital share of total wealth, and high ability to bear
risk; near-retirement investors have short horizons, small human-
capital share, and low ability. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.5-25.

```
retail:  R_target  ≈ funding-need-driven (retirement spend, etc.)
          risk_obj  ≈ life-cycle-driven (long horizon -> wider band)
          horizon   ≈ life-cycle-stage-driven
```

A specific implication for IPS construction: the type identifies
which template applies. A retail IPS uses life-cycle objectives
and personal-preference questionnaires; a DB-pension IPS uses
asset-liability matching and funding-status risk objectives; an
endowment IPS uses spending-rule construction and inflation-
adjusted return objectives; an insurance IPS uses solvency-capital-
constrained construction. The type does not change the four-pillar
IPS structure (purpose, objectives, constraints, governance); it
changes the population of each pillar. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

## See Also

- [`pm-portfolio-perspective.md`](pm-portfolio-perspective.md) — the portfolio-process loop common to all investor types
- [`pm-investment-policy-statement.md`](pm-investment-policy-statement.md) — the four-pillar IPS structure populated by per-type templates
- [`pm-portfolio-constraints.md`](pm-portfolio-constraints.md) — the LLTTU families that bind differently across investor types

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R48 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

- Defined-benefit pension asset-liability management mechanics
  (immunization, surplus optimization, contribution-volatility
  management) — Vol.6 R48 introduces; deeper construction belongs
  in future-13. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.420-441.
- Endowment spending-rule choices (constant-percentage, smoothing-
  rule, hybrid) — Vol.6 R48 introduces; deeper development belongs
  in future-13. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.420-441.
- Insurance-company portfolio construction under risk-based-capital
  / Solvency II regimes — Vol.6 R48 mentions; deeper coverage
  belongs in future-13. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.420-441.
- Sovereign wealth fund governance frameworks (Santiago Principles,
  stabilization vs savings mandates) — Vol.6 R48 mentions; deeper
  coverage belongs in future-13. **Source:** CFA L1 Curriculum
  (2022) Vol.6/pp.420-441.
