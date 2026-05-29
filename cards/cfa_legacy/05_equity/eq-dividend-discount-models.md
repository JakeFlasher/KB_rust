---
schema_version: "cacg.v0"
id: "eq-dividend-discount-models"
title: "Dividend Discount Models"
reading_id: "05_equity"
summary: "The DDM is the canonical equity intrinsic-value formula: equity value equals the present value of expected future dividends discounted at the cost of equity. Gordon-growth, two-stage, and H-model variants specialize the formula. Damodaran Ch.14 develops the DDM family; CFA R38 frames it as a foundational present-value valuation model."
tags: ["equity", "dividend-discount"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p487:0607"
    chunk_hash: "ee9e2c083b2ce3e4e8399a7bf6da7c2e29a587e4f416e432890d00bef8d8eb7d"
    page_range: [487, 487]
    quote: "In the strictest sense, the only cash flow you receive when you buy shares in a publicly traded firm is a dividend."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p489:0610"
    chunk_hash: "62d297b383eb796eb2c063f12d18c3d1c4920ed8ddd705999f896486956dd1e0"
    page_range: [489, 489]
    quote: "First, since the growth rate in the firm’s dividends is expected to last forever, the firm’s other operating metrics (including revenues and earnings) can also be expected to grow at the same rate."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2309:3384"
    chunk_hash: "3fd5b07bea2c5c6f31a71b7ecd95eb69f208c2ed854103f33a2b9bc965921ff6"
    page_range: [2309, 2310]
    quote: "Dividend discount models focus on expected dividends. How is the required rate of return for use in present value models estimated?"
    edge_type: "supports"
card_hash: "42e89270d2b22294acccba5afb2213bfe1bf1e81529af0923db6db18f2ba7f17"
---
# Dividend Discount Models

## Intuition

The dividend discount model (DDM) is the canonical equity intrinsic-
value formula: the value of an equity claim today is the present
value of the dividends the holder expects to receive over the
holding horizon, discounted at a rate reflecting the riskiness of
those dividends. Dividends are the cash flows that actually accrue
to the equity holder; everything else (earnings, free cash flow) is
either a candidate dividend or a measure of dividend-paying capacity.
**Source:** Damodaran (2012) Ch.14 pp.487-537.

In its purest form the DDM says: pay attention to what the firm
actually distributes to shareholders. Earnings that are retained
build value only if they generate future dividends; earnings that
are paid out today ARE the dividend. The DDM converts the dividend
stream — finite or perpetual, constant or growing — into a single
present-value number that the analyst can compare against the
observed share price. **Source:** Damodaran (2012) Ch.14 pp.487-537.

```
share price S_0   <-->   PV of expected dividend stream
        ?                 +-------+
        |                 | D_1   |  / (1 + r)^1
        |                 +-------+
        |                 | D_2   |  / (1 + r)^2
        |                 +-------+
        |                 | D_3   |  / (1 + r)^3
        |                 +-------+
        |                 |  ...  |  ...
        |                 +-------+
        |                 | D_T   |  / (1 + r)^T
        |                 +-------+
        |                 |  TV_T |  / (1 + r)^T  <-- terminal
        |                 +-------+               value
        |
        +-> compare S_0 to V_0; gap is mispricing diagnostic
```

## Definition

The general DDM expresses the value of an equity claim as the present
value of all expected future dividends discounted at the cost of
equity. **Source:** Damodaran (2012) Ch.14 pp.487-537.

```
V_0 = sum over horizon of  D_i / (1 + r)^i
```

The Gordon growth model is the DDM specialized to a stable perpetual
dividend stream growing at a constant rate `g` strictly less than the
cost of equity `r`. The closed-form expression is `V_0 = D_1 / (r -
g)`, where `D_1` is the next-period expected dividend. Gordon growth
applies to mature, stable-growth firms — the assumption breaks down
for firms in transition or with growth that exceeds long-run
sustainable rates. **Source:** Damodaran (2012) Ch.14 pp.487-537.

The two-stage and three-stage DDM extensions handle firms in
transition: an explicit-forecast period of high or transitional
growth (where dividends and growth are forecast year by year) is
followed by a Gordon-growth terminal phase capturing the stable
mature state. If the explicit forecast ends at `T`, the terminal value
at `T` is `TV_T = D_{T+1} / (r_stable - g_stable)` and is itself
discounted back to today. **Source:** Damodaran (2012) Ch.14
pp.487-537.

The H-model is a third specialization that linearly interpolates
between an initial high-growth rate and a stable terminal growth
rate over an extraordinary-growth transition assumed to last `2H`
periods. It captures decay from initial to stable growth without
requiring a piecewise growth schedule, while holding payout and cost
of equity constant in the H-model form. **Source:** Damodaran (2012)
Ch.14 pp.487-537.

## Mathematical Reasoning

The Gordon-growth derivation starts from the perpetuity sum of
dividends growing at constant rate `g`. The discounted sum is a
geometric series whose first term is the period-1 discounted dividend
and whose common ratio is the growth-discount factor. Provided the
sum converges, it collapses to the closed form `V_0 = D_1 / (r - g)`,
where `D_1` is the next-period expected dividend and `r` is the cost
of equity. **Source:** Damodaran (2012) Ch.14 pp.487-537.

The convergence condition `g < r` is structural, not arbitrary. If
`g >= r` the geometric series diverges and the model breaks; in
practice `g < r` is enforced as a sustainability constraint — a firm
cannot grow its dividend forever at a rate exceeding its cost of
equity (which is bounded below by the riskless rate). **Source:**
Damodaran (2012) Ch.14 pp.487-537.

The DDM specializes the general intrinsic-value formula `V_0 = sum
CF_i / (1 + r)^i` (see [`eq-intrinsic-value`](./eq-intrinsic-value.md))
by setting `CF_i = D_i` (the period dividend). FCFE-based valuation
substitutes potential dividends (FCFE) for actual dividends — the two
models agree when actual payout equals potential payout, and diverge
when the firm distributes less or more than its potential. **Source:**
Damodaran (2012) Ch.14 pp.487-537.

The cost of equity `r` in the DDM denominator comes from the asset-
pricing model in use (see
[`eq-discount-rate-and-required-return-foundations`
](./eq-discount-rate-and-required-return-foundations.md)). Damodaran
defaults to CAPM as the foundational pricing model. **Source:**
Damodaran (2012) Ch.14 pp.487-537.

The CFA L1 frame mirrors the DDM as the canonical equity-valuation
intuition tool, presents Gordon-growth as the simplest closed form,
and notes its sensitivity to the `(r - g)` denominator near
convergence. **Source:** CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-intrinsic-value`](./eq-intrinsic-value.md) — the general intrinsic-value frame the DDM specializes
- [`eq-discount-rate-and-required-return-foundations`](./eq-discount-rate-and-required-return-foundations.md) — `r` in the DDM denominator
- [`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md) — the justified P/E ratio derived from Gordon growth

## Escalate to Raw When

Open Damodaran Ch.14 directly when any of the criteria below applies.
**Source:** Damodaran (2012) Ch.14 pp.487-537.

- the dividend stream is non-stable and a multi-stage / H-model variant is needed beyond the textbook two-stage form — Damodaran Ch.14 develops the variants. **Source:** Damodaran (2012) Ch.14 pp.487-537.
- the firm pays substantially less (or more) than its potential dividend (free-cash-flow-to-equity gap) — switch to an FCFE model — see [`eq-fcfe-fcff-decomposition`](./eq-fcfe-fcff-decomposition.md). **Source:** Damodaran (2012) Ch.14 pp.487-537.
- the cost of equity changes through the explicit-forecast period — Damodaran Ch.14 derives the period-by-period discounting machinery for non-constant `r`. **Source:** Damodaran (2012) Ch.14 pp.487-537.
