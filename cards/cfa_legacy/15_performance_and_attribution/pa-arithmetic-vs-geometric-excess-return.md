---
schema_version: "cacg.v0"
id: "pa-arithmetic-vs-geometric-excess-return"
title: "Arithmetic vs Geometric Excess Return"
reading_id: "15_performance_and_attribution"
summary: "Two excess-return measures: arithmetic a=r-b (vs initial capital) and geometric g=(1+r)/(1+b)-1 (vs the notional fund). Arithmetic exceeds geometric in rising markets and falls below it in falling markets; geometric is convertible and compoundable."
tags: ["excess-return", "geometric-attribution", "benchmark-relative"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p151:0187"
    chunk_hash: "bc5b140948bb80547369e5b679eaa22c3daacbdd2b0c7ab5155f379304a7b554"
    page_range: [151, 151]
    quote: "in rising markets the arithmetic excess return is always greater than the geometric excess return and in falling markets the reverse is true"
    edge_type: "defines"
card_hash: "d7571622df554996baba0dfd2f3a3f4e88ff5be8f47836e66e0d1324b7fcf6ce"
---
# Arithmetic vs Geometric Excess Return

## Intuition

Once you have a portfolio return r and a benchmark return b, the obvious next move is to compare them — to ask "how much did the manager add?". There are two natural ways to phrase the answer, and they answer subtly different questions. The arithmetic version subtracts the two returns and reports added value as a fraction of the **initial** amount invested. The geometric version takes the ratio of growth factors and reports added value as a fraction of the **final** value of the notional (benchmark) fund — i.e. the amount the client *would* have had if they had simply tracked the benchmark.

The cash added value is identical under both — both are trying to explain the same dollar profit. They differ only in the denominator they normalize that profit against: initial capital (arithmetic) versus the grown notional fund (geometric).

**Source:** Bacon (2023) "Excess Return" printed pp.127-128 (PDF pp.149-150)

## Definition

**Arithmetic excess return** — profit in excess of a notional or benchmark fund expressed as a percentage of the initial amount invested:

    a = r - b

where a = arithmetic excess return, r = portfolio return, b = benchmark return.

**Geometric excess return** — profit in excess of the notional or benchmark fund expressed as a percentage of the final value of the notional or benchmark fund:

    g = (1 + r)/(1 + b) - 1

where g = geometric excess return.

Bacon argues the geometric method is the more technically correct of the two, resting on three properties: **proportionality**, **convertibility** (across currencies), and **compoundability** (across periods). Arithmetic remains more common worldwide, defended mostly on ease of use, simplicity, and intuitive feel. Neither dominates globally.

**Source:** Bacon (2023) "Excess Return" printed pp.127-128 (PDF pp.149-150)

## Mathematical Reasoning

Rearranging the geometric definition exposes the exact link between the two measures:

    g = (1 + r)/(1 + b) - 1 = (1 + r)/(1 + b) - (1 + b)/(1 + b) = (r - b)/(1 + b) = a/(1 + b)

So the geometric excess is the arithmetic excess discounted by the benchmark growth factor (1 + b). The denominator (1 + b) is the entire story:

- In a **rising market** (b > 0), the divisor (1 + b) > 1 shrinks a, so g < a — the arithmetic excess is always the larger number.
- In a **falling market** (b < 0), the divisor (1 + b) < 1 inflates a, so g > a — the inequality reverses.

Bacon states this relationship as established (not a separate proof): it "demonstrates that in rising markets the arithmetic excess return is always greater than the geometric excess return and in falling markets the reverse is true," and wryly notes that managers may favour arithmetic because it looks better in most market conditions. Because g normalizes against the *final* notional value rather than the start, it is the same regardless of the reporting currency (convertibility) and chains cleanly across periods (compoundability) — properties the arithmetic measure lacks.

```
                a (arithmetic excess)
   g  =  ---------------------------------
                  ( 1 + b )

   rising market  b>0  =>  (1+b)>1  =>  g < a   (arithmetic looks better)
   flat market    b=0  =>  (1+b)=1  =>  g = a
   falling market b<0  =>  (1+b)<1  =>  g > a   (geometric looks better)
```

**Source:** Bacon (2023) "Excess Return" Eq.4.26 printed pp.128-129 (PDF pp.150-151)

## See Also

- [`pa-geometric-vs-arithmetic-linking-choice.md`](pa-geometric-vs-arithmetic-linking-choice.md) — extends this single-period choice to the multi-period smoothing/linking decision.
- [`pa-geometric-attribution-brinson-extended.md`](pa-geometric-attribution-brinson-extended.md) — geometric excess as the base for compoundable attribution effects.
- [`pa-currency-attribution-karnosky-singer.md`](pa-currency-attribution-karnosky-singer.md) — the convertibility argument made concrete: geometric excess is currency-invariant.
- [`pa-multilevel-attribution-successive-notional-funds.md`](pa-multilevel-attribution-successive-notional-funds.md) — the notional-fund framing that geometric excess builds on.

## Escalate to Raw When

- You need the worked dollar example (Exhibit 4.4): a $1,000,000 portfolio ending at $1,070,000 against a 5% benchmark, where the arithmetic excess is 2% and the geometric excess is 1.9% — Bacon (2023) pp.150.
- You need the proportionality illustration (Exhibit 4.5) with a halved portfolio versus a quartered benchmark, where geometric (+100%) and arithmetic (+25%) diverge sharply — Bacon (2023) pp.151.
- You need the currency-conversion exhibit (Exhibit 4.6) showing arithmetic excess drifting from 2.0% to 2.2% under a currency change while geometric stays fixed — Bacon (2023) pp.151-152.
