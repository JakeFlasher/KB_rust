---
schema_version: "cacg.v0"
id: "eq-payout-policy-and-growth"
title: "Payout Policy and Growth"
reading_id: "05_equity"
summary: "Framing the sustainable-growth identity `g = retention · ROE` as a valuation INPUT — the simplest decomposition of long-run growth into the fraction of earnings retained and the return on equity earned on that reinvestment. Higher retention or higher ROE supports higher sustainable growth; the identity surfaces inconsistencies between assumed growth and assumed payout."
tags: ["equity", "payout-policy"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p400:0493"
    chunk_hash: "176ddf316f9bcec7d96a661636d340d6270ca421bf419e58a834214b5a56e6f2"
    page_range: [400, 400]
    quote: "The simplest relationship determining growth is one based on the retention ratio (percentage of earnings retained in the firm) and the return on equity on its projects."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p489:0611"
    chunk_hash: "e9ddd1bd458b6b5362f23ec32a68ad0730143967d144949e040b60939b49d418"
    page_range: [489, 490]
    quote: "The estimate of ROE matters because the payout ratio in stable growth must be consistent: Payout ratio = 1 − Stable growth rate Stable period ROE"
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2313:3392"
    chunk_hash: "6c0a925ca09c47ba94de891b7d86314c12e3d454139a6413392341b942bd56ab"
    page_range: [2313, 2314]
    quote: "g = b × ROE where g = dividend growth rate b = earnings retention rate = (1 – Dividend payout ratio) ROE = return on equity"
    edge_type: "supports"
card_hash: "d844b884a0562229c8a052ad4cb08b90c6e9f84426860f252c15ae966dbd1b8b"
---
# Payout Policy and Growth

## Intuition

Sustainable growth is the rate at which a firm can grow its
dividends — and, more fundamentally, its earnings per share —
without raising new equity capital and without changing its capital
structure. The sustainable-growth identity says that long-run
earnings growth equals the fraction of earnings reinvested
(retention ratio) times the rate of return earned on that
reinvestment (ROE). A firm that retains a larger share of earnings,
or that earns a higher return on retained earnings, can sustain a
higher growth rate; a firm that pays out everything (zero retention)
has zero sustainable growth. **Source:** Damodaran (2012) Ch.11
pp.384-428.

The identity is a valuation INPUT, not a policy prescription. The
DDM and DCF models consume the growth rate `g` as a parameter; the
sustainable-growth identity decomposes that `g` into retention and
ROE so the analyst can sanity-check whether the assumed growth is
internally consistent with the firm's payout policy and reinvestment
returns. A firm assumed to grow at a high rate while paying out most
of its earnings is implicitly assumed to earn unrealistically high
ROE on the small amount it retains — the identity surfaces that
inconsistency. **Source:** Damodaran (2012) Ch.11 pp.384-428.

```
sustainable-growth identity decomposition

   total earnings E
        |
        v
   +---------+   +----------+
   | dividend|   | retained |
   |  D = E  |   | earnings |
   |  · payout |   | E · (1 - payout)
   +---------+   +----------+
                       |
                       v
              reinvested at ROE
                       |
                       v
              next-period EPS adds
              (1 - payout) · ROE = g
              percent on retained capital
                       |
                       v
                growth rate g
```

## Definition

The retention ratio `b = 1 - payout` is the fraction of earnings the
firm retains rather than distributes to shareholders. The payout
ratio is the complementary share `payout = D / E`, where `D` is the
period dividend and `E` is the period earnings per share. By
construction `payout + b = 1`. **Source:** Damodaran (2012) Ch.11
pp.384-428.

The return on equity `ROE = NI / book equity` measures the
accounting-rate-of-return the firm earns on its equity capital.
Damodaran distinguishes between the average ROE (what the firm has
historically earned across all its assets) and the marginal ROE
(what the firm expects to earn on the NEXT dollar of retained
earnings); the sustainable-growth identity's `ROE` is properly the
marginal ROE on incremental investment. **Source:** Damodaran (2012)
Ch.11 pp.384-428.

The sustainable-growth identity is `g = b · ROE`. It is derived from
the accounting identity that next-period book equity equals current
book equity plus retained earnings, combined with the assumption
that ROE is held constant on incremental investment. The identity
binds: a firm CANNOT grow earnings faster than `b · ROE` over the
long run without raising external equity capital or increasing
leverage. **Source:** Damodaran (2012) Ch.11 pp.384-428.

The DDM consumes `g` as the dividend-growth rate (see
[`eq-dividend-discount-models`](./eq-dividend-discount-models.md));
the sustainable-growth identity provides the bridge between the
firm's payout policy and the growth assumption that DDM requires.
For a stable-growth firm, the Gordon-DDM `V_0 = D_1 / (r - g)`
collapses cleanly when `g` is set equal to the sustainable-growth
identity. **Source:** Damodaran (2012) Ch.14 pp.487-537.

The CFA L1 frame presents the sustainable-growth identity at L1
depth as the bridge between the firm's payout policy and the growth
assumption that DDM requires; deeper dividend-policy and buyback-
decision content lives outside the L1 Equity readings. **Source:**
CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## Mathematical Reasoning

The derivation starts from the accounting identity that next-period
book equity `B_1` equals current book equity `B_0` plus retained
earnings `b · E_0`. Dividing both sides by `B_0`, next-period book
equity grows at `(1 + b · E_0 / B_0) = (1 + b · ROE)`. If ROE is
held constant on incremental investment, next-period earnings
(`E_1 = ROE · B_1`) grow at the same `(1 + b · ROE)` rate, giving
the sustainable-growth identity `g = b · ROE`. **Source:** Damodaran
(2012) Ch.11 pp.384-428.

The identity has a leveraged extension that operates through ROE
itself rather than as a separate adjustment to `g`. Damodaran's
DuPont-style leverage bridge expresses ROE as a function of return
on capital (ROC, the firm's operating return on its operating-asset
base) and after-tax cost of debt (`kd_at = i · (1 - t)`):
`ROE = ROC + (D/E) · (ROC - kd_at)`, where `D/E` is the debt-to-
equity ratio. Substituting back into the sustainable-growth identity
gives the combined form `g = b · [ROC + (D/E) · (ROC - kd_at)]`.
The bridge collapses to the unlevered case (`ROE = ROC`) when
`D/E = 0` or when `ROC = kd_at`. Crucially, leverage boosts
sustainable growth ONLY when `ROC > kd_at` — operating return on
capital exceeds the after-tax debt cost. (Stating the break-even
condition in terms of ROE itself would be circular because ROE is
the variable the leverage bridge is decomposing.) **Source:**
Damodaran (2012) Ch.11 pp.384-428.

The identity bounds the consistency of any DDM/DCF growth assumption
against the firm's payout policy. If an analyst projects high growth
while assuming a high payout, the implied ROE on retained earnings
must be commensurately high — the identity makes that implication
arithmetic, not a hidden assumption. The identity does NOT determine
the firm's optimal payout policy; that question belongs to future-04
(Corporate Finance), where the trade-offs between dividend policy,
buybacks, and capital-structure choices are derived. **Source:**
Damodaran (2012) Ch.11 pp.384-428.

## See Also

- [`eq-dividend-discount-models`](./eq-dividend-discount-models.md) — the DDM/Gordon-growth model that consumes the sustainable-growth `g`
- [`eq-intrinsic-value`](./eq-intrinsic-value.md) — the intrinsic-value frame that grounds growth as a cash-flow-stream input

## Escalate to Raw When

Open Damodaran Ch.11 directly when any of the criteria below applies.
**Source:** Damodaran (2012) Ch.11 pp.384-428.

- the firm has a multi-segment or transitional capital structure that makes the unlevered identity insufficient — Damodaran Ch.11 develops the leveraged extension and segment-weighted variants. **Source:** Damodaran (2012) Ch.11 pp.384-428.
- the marginal ROE on incremental investment differs materially from the historical average ROE — Damodaran Ch.11 develops the marginal-ROE adjustment for valuation work. **Source:** Damodaran (2012) Ch.11 pp.384-428.
- a comprehensive dividend-policy / buyback / capital-structure decision is required (the firm is choosing payout policy rather than reading it as an input) — defer to future-04 (Corporate Finance). **Source:** Damodaran (2012) Ch.11 pp.384-428.
