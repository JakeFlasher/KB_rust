---
schema_version: "cacg.v0"
id: "mt-vwap-pov-volume-targeting"
title: "Volume-Targeting Execution: VWAP, Percentage-of-Volume (POV) and Percentage-of-Cumulative-Volume"
reading_id: "14_microstructure_and_trading"
summary: "Schedule-following algorithms (POV, POCV, VWAP) size child orders as a fraction of market volume so a large parent order blends into the day's traded flow rather than revealing urgency."
tags: ["microstructure", "execution-algorithms", "vwap", "pov", "volume-targeting"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p231:0301"
    chunk_hash: "3788fdd56c367ff37952faf87cf58f80b05eab15249ba2af5defde90a72dbd0b"
    page_range: [231, 231]
    quote: "Strategies that target (i) are often called percentage of volume (POV) and we label strategies that target (ii) as percentage of cumulative volume (POCV)."
    edge_type: "defines"
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p436:0734"
    chunk_hash: "8cadef2a7c6bc18b1ae0383a32f1102fd84cc14f768d2000f764ee3e79413336"
    page_range: [437, 437]
    quote: "Traders like the VWAP benchmark because they would like to trade at least as well as the average"
    edge_type: "supports"
card_hash: "d9f2909d2747824be98c3dcbdfac8d11f107c36b679d71c0b4a475dde5925e77"
---
# Volume-Targeting Execution: VWAP, Percentage-of-Volume (POV) and Percentage-of-Cumulative-Volume

## Intuition
A trader who must move a large block over a day faces a tension: trading too fast pushes the price against herself (market impact), while trading too slowly leaves the position exposed to price drift. A camouflage-oriented response is to *let the market set the pace*. Instead of dumping a fixed schedule onto the tape, the agent ties her own rate of trading to the rate at which everyone else is trading, so her child orders dissolve into the ambient order flow and never spike the one-sided pressure that signals urgency.

Three closely related schedule-following targets formalize this. **Percentage-of-Volume (POV)** trades a fixed fraction *p* of the *instantaneous rate* of other participants' market orders. **Percentage-of-Cumulative-Volume (POCV)** instead tracks a fixed fraction of the *cumulative volume traded so far* over the horizon. **VWAP** (volume-weighted average price) is the benchmark these algorithms are usually trying to match: the day's average trade price weighted by trade size. If you can keep your per-instant share of volume roughly constant, your realized average price ends up near the market's VWAP almost by construction.

```
   market volume rate
   (U-shaped over day)        agent's child-order rate
        |                          |
   high |#         #          high |x         x
        | #       #               | x       x      <- POV: agent rate = p * market rate
        |  ##   ##                |  xx   xx
    low |    ###          ->  low |    xxx
        +----------- t            +----------- t
        open   mid   close        agent "rides" the same shape, scaled by p
```

**Source:** Cartea, Jaimungal & Penalva (2015) §9.1 Introduction pp.231.

## Definition
Let the agent acquire or liquidate a parent order over horizon `[0, T]`. Two execution targets are defined as trading a number of shares equal to a fraction of:

- **(i)** the *rate* at which other participants are sending market orders — strategies targeting (i) are called **percentage of volume (POV)**;
- **(ii)** the *total (cumulative) volume* that has been traded over the entire horizon — strategies targeting (ii) are called **percentage of cumulative volume (POCV)**.

The **VWAP** benchmark over interval `[T1, T2]` is the trade-size-weighted average midprice, computed most simply as total traded dollar value divided by total traded volume. POV/POCV algorithms exist largely to track VWAP: matching a fixed proportion of the market's trading rate at each instant keeps the agent's average fill near the market's volume-weighted average.

**Source:** Cartea, Jaimungal & Penalva (2015) §9.1 Introduction pp.231; Harris (2003) §21.3.1.4 pp.437.

## Mathematical Reasoning
Let `v_t` denote the agent's liquidation speed and let `μ_t` be the speed (rate) at which the rest of the market trades. A pure POV target sets `v_t = p · μ_t` for some `0 < p < 1`. Because cumulative quantities are time-integrals of rates, the rate target (i) and the cumulative-volume target (ii) are linked: total volume is the integral of `μ_t`, so a strategy that holds `v_t / μ_t = p` pointwise also accumulates the fraction `p` of cumulative volume — yet the two are *not* identical optimization problems, since matching cumulative volume permits intertemporal substitution that pointwise rate-matching forbids.

The VWAP-tracking property follows from a weighting argument. VWAP weights each market trade price by its size; if the agent's own execution at each instant is a *constant* fraction of the contemporaneous market volume, her fills carry the same time-weighting profile as the market's, so her realized average price converges toward the market VWAP. Cartea–Jaimungal–Penalva note that targeting a fraction of the rate of trading at every instant "ensures that the investor is tracking the average price," and that smoothing execution while adamantly holding a fixed proportion of the market's rate drives the average cost close to VWAP.

Two structural obstacles fall out of the mechanism. First, neither (i) nor (ii) is automatically compatible with completing the full parent quantity by `T`: the realized sum of volume-fractions need not equal the target inventory, because future volume is random. Second, the timing and size of other traders' market-order arrivals cannot be anticipated, so `μ_t` is stochastic — this volume uncertainty is an extra risk dimension on top of price risk, and an optimal solution must trade off a POV-tracking penalty against the temporary impact cost of deviating from it.

**Source:** Cartea, Jaimungal & Penalva (2015) §9.1–§9.2 pp.230-231.

## Boundary Notes
- **Volume is stochastic and U-shaped.** Empirically intraday volume is high at the open and close and low midday, but realized volume deviates from this average pattern day to day; an algorithm calibrated only to the mean profile bears tracking error on atypical days. **Source:** Cartea, Jaimungal & Penalva (2015) §9.1 pp.231.
- **Completion is not guaranteed.** Strict POV/POCV can finish over- or under-filled relative to the parent order, because the realized fractions of an unknown future volume need not sum to the target inventory; production algorithms bolt on completion constraints (e.g. TWAP-like terms near the horizon) that distort pure volume-tracking. **Source:** Cartea, Jaimungal & Penalva (2015) §9.1 pp.231.
- **VWAP as a benchmark has gaming and bias issues.** A trader who *is* the dominant volume that day mechanically moves the VWAP toward her own prints, so VWAP can understate true transaction cost for large or impactful orders — a reason it differs from an arrival-price / implementation-shortfall benchmark. **Source:** Harris (2003) §21.3.1.4 pp.437.
- **POV ≠ a hard volume cap.** Targeting a fraction of others' speed is distinct from a strategy that merely caps the agent's speed at a fraction of market speed; the former smooths around the target, the latter only constrains the maximum. **Source:** Cartea, Jaimungal & Penalva (2015) §9.2 pp.230.

## See Also
- [`mt-almgren-chriss-optimal-execution`](./mt-almgren-chriss-optimal-execution.md) -- the impact-vs-timing-risk trade-off that volume-targeting schedules implicitly resolve.
- [`mt-implementation-shortfall`](./mt-implementation-shortfall.md) -- the arrival-price benchmark that contrasts with VWAP for measuring execution quality.
- [`mt-effective-cost-trade-benchmark`](./mt-effective-cost-trade-benchmark.md) -- VWAP as a transaction-cost estimator and its gaming/bias limitations.
- [`mt-buy-side-trader-best-execution`](./mt-buy-side-trader-best-execution.md) -- why buy-side desks adopt volume-tracking algos to demonstrate best execution.

## Escalate to Raw When
Cartea–Jaimungal–Penalva Chapter 9 actually *solves* the stochastic-control problem (Hamilton–Jacobi–Bellman / PIDE formulation) for optimal POV and POCV liquidation, including the explicit optimal trading-rate corrections, the exponential-utility (price-risk) extension, and the permanent-impact case — this card only states the targets and their VWAP-tracking and completion properties without reproducing the value-function derivations. Re-read §9.2–§9.5 (pp. ~229-250) for the PIDEs, boundary conditions, and the POV-penalty `φ → ∞` limiting analysis. For the VWAP-benchmark bias and gaming arguments, re-read Harris (2003) §21.3.1 around pp.437-440.
