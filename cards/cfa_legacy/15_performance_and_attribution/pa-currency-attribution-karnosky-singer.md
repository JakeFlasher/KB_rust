---
schema_version: "cacg.v0"
id: "pa-currency-attribution-karnosky-singer"
title: "Currency Attribution: The Karnosky-Singer Framework"
reading_id: "15_performance_and_attribution"
summary: "Karnosky-Singer splits multi-currency excess return into a local-premium leg (local return minus local cash rate) and a currency leg, prices forwards via interest-rate differentials, and uses continuously compounded returns to sidestep compounding."
tags: ["currency-attribution", "karnosky-singer", "multi-currency"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p322:0388"
    chunk_hash: "b64612d9ed04ed43eb2998d41f02ce380d5bfe08ab0e1f56627e64bc6668ba18"
    page_range: [322, 322]
    quote: "by using continuously compounded returns in their model and solve the forward premium concern"
    edge_type: "defines"
  - source_id: "pa_colin_2016"
    chunk_id: "pa_colin_2016:p086:0090"
    chunk_hash: "c6a8b5c66aef198b035bd74a8ce501688ba47754ae34a24cfd587395430628a8"
    page_range: [86, 87]
    quote: "currency attribution is that spot exchange rates are insufficient to decide"
    edge_type: "supports"
---
# Currency Attribution: The Karnosky-Singer Framework

## Intuition

When a manager holds assets in many currencies and can hedge with forward
contracts, the naive split — local market return plus spot FX return — misjudges
currency decisions. The reason is that hedging into a currency earns (or pays)
that currency's cash rate, so two managers with identical FX exposure but
different hedges can earn very different returns whenever interest rates differ
across currencies. Karnosky and Singer's 1994 framework fixes this by moving the
local cash (interest) rate out of the currency leg and treating each currency
decision as an exposure that carries its own interest-rate differential. The
core idea is that spot exchange rates alone are not enough to grade a hedged
currency bet.

**Source:** Colin (2016) §4.12-4.13 pp.86-88

## Definition

Karnosky-Singer attribution decomposes the **total return** of a multi-currency
portfolio into a **local-premium** component and a **currency** component:

- **Local premium** for a currency = local-currency return minus that currency's
  local interest (cash) rate. This is the real, market-driven excess return of
  holding the asset above the risk-free rate of its country.
- **Currency** component = the FX/currency surprise plus the local interest rate
  (the "base return" / hedged Eurodollar rate). Forward contracts enter through
  the interest-rate differential against the base currency, not through spot
  moves alone.

Bacon frames this as resolving the two defects of the earlier Ankrim-Hensel
scheme: (1) Ankrim-Hensel use an arithmetic return premium that ignores the
compounding interaction between market and currency returns and smears it across
allocation, selection, and interaction; (2) it isolates the forward premium as a
free-standing factor when that effect is really a consequence of the asset-
allocation decision. Karnosky-Singer absorb the forward premium into the
benchmark return premium (so it sits with asset allocation) and remove the
compounding smear by working in continuously compounded (log) returns.

**Source:** Bacon (2023) §6 pp.322-324

## Mathematical Reasoning

Bacon states the Karnosky-Singer portfolio total return as a weighted sum over
currencies `i` of a local-return part and a currency part:

```
r = sum w_i*rL_i  +  sum w_i*c_i                       (total return)

  = sum w_i*(rL_i - x_i)  +  sum w_i*(c_i + x_i)        (split out cash rate x_i)
        +-- local premium --+   +-- currency (base) --+
```

where `rL_i` is the local-currency return, `c_i` the currency return, and `x_i` the
local interest rate. Adding the cash rate `x_i` to the currency leg and
subtracting it from the local leg is the algebraic heart of the framework: it
re-assigns the interest carry from "market" to "currency," which is where the
hedging decision actually earns or pays it.

Forward contracts are isolated by a third term `sum w~_i*f_i`, and **using
continuously compounded returns** the forward return reduces to a pure
interest-rate differential against the base currency:

```
f_i = c_i + x_i - x_B            (x_B = base-currency interest rate)
```

This identity is exactly why log returns "sidestep compounding": the forward
premium becomes additive in the rate differential rather than a multiplicative
cross-term. When the forward weights net to zero, `sum w~_i = 0`, the forward term
collapses into the currency term and the total return simplifies to a clean two-
part sum (Bacon Eq. 6.61); the benchmark takes the identical form (Eq. 6.62).
Subtracting benchmark from portfolio then yields the two attribution blocks:

```
                 EXCESS RETURN  r - b
        +-----------------------+-----------------------+
  LOCAL-PREMIUM ATTRIBUTION                CURRENCY ATTRIBUTION
  sum w_i(rL_i-x_i) - sum W_i(bL_i-x_i)   sum(w_i+w~_i)(c_i+x_i) - sum(W_i+W~_i)(c_i+x_i)
        |                                          |
        +-- Brinson-Fachler applied separately ----+
            allocation A_i = (w_i-W_i)(l_i' - l'),  l_i' = bL_i - x_i
```

Bacon then applies the standard Brinson-Fachler quadrant logic *separately* to
the local-premium part and the currency part, so allocation `A_i = (w_i - W_i) *
(l_i' - l')` uses the benchmark **return premium** `l_i' = bL_i - x_i`. This
definition folds the forward-premium effect into the benchmark return premium —
the correction Bacon flagged against Ankrim-Hensel. Bacon presents these as
identities/definitions rather than proving optimality; the card asserts to the
same depth.

**Source:** Bacon (2023) §6 pp.322-324

## Boundary Notes

The decomposition is exact in continuously compounded (log) space; Bacon notes
that asset owners generally will not accept reporting in continuously compounded
returns, so the model's clean additivity is a presentation/conversion caveat in
practice, not a flaw in the attribution. Colin adds that the framework applies to
both hedged and unhedged portfolios — with no hedging it reduces to the unhedged
case — at the cost of requiring a cash return for each market.

**Source:** Colin (2016) §4.14 pp.86-88

## See Also

- [`pa-brinson-fachler-benchmark-relative-allocation.md`](pa-brinson-fachler-benchmark-relative-allocation.md) — the benchmark-relative allocation engine Karnosky-Singer applies separately to each leg.
- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — why the compounding interaction matters and how log returns neutralize it.
- [`pa-fi-carry-rolldown-pulltopar-time-decomposition.md`](pa-fi-carry-rolldown-pulltopar-time-decomposition.md) — sibling "carry vs surprise" decomposition logic, here on the fixed-income side.
- [`pa-multilevel-attribution-successive-notional-funds.md`](pa-multilevel-attribution-successive-notional-funds.md) — nesting currency and local-premium legs within a multilevel attribution scheme.

## Escalate to Raw When

- You need the worked numerical example: Bacon's Table 6.31 (Ankrim-Hensel) and
  Colin's Tables 4.10-4.11 carry the actual percent allocation/selection/currency
  figures and the Germany-overweight illustration (return premium 7% - 5% = 2%
  vs. the 0.66% average) — read those pages directly rather than reproducing
  arithmetic here.
- You need the full benchmark-side derivation (Bacon Eqs. 6.62-6.65) including the
  average benchmark return premium `l' = sum W*l_i'` and the currency-side allocation
  quadrants verbatim.
- You need the formal Karnosky-Singer algorithm statement: Colin's Appendix A
  gives the step-by-step procedure behind the Table 4.11 results.
- You are reconciling against the Ankrim-Hensel approach: Bacon p.322 lists its
  three specific problems (arithmetic premium, isolated forward premium,
  hedging-invariant benchmark currency effect `e`).
