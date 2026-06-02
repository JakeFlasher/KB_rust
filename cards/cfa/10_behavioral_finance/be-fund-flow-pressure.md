---
schema_version: "cacg.v0"
id: "be-fund-flow-pressure"
title: "Performance-Based Arbitrage And Fund-Flow Pressure"
reading_id: "10_behavioral_finance"
summary: "Under performance-based arbitrage (Shleifer-Vishny 1997), investors allocate capital to arbitrageurs by past returns, so deepening mispricing produces interim losses, fund withdrawals, and forced liquidation exactly when expected returns are highest — arbitrage is least effective and least resilient in panics."
tags: ["behavioral-finance", "limits-of-arbitrage", "fund-flows", "agency", "fire-sales"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p097:0108"
    chunk_hash: "114c7160cb22213e74a3987d13fdf1901f13a459e38beb2c3b80ae2bc4c8fdf5"
    page_range: [98, 98]
    quote: "We refer to the phenomenon of responsiveness of funds under management to past returns as performance based arbitrage."
    edge_type: "defines"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p098:0109"
    chunk_hash: "07c806da83ef5920df6c0bcfcdac87ec184a133f7efc67584cf833dc4a4cda11"
    page_range: [99, 99]
    quote: "Poor performance can thus erode both the equity base and the borrowing capacity of an arbitrageur, regardless of the attractiveness of arbitrage opportunities he faces."
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p102:0114"
    chunk_hash: "f70eaa90abe0a4a3cb14755bf4b01f44fa1f23fb56bd1bd756668d042efff196"
    page_range: [102, 102]
    quote: "in a particular segment at time 2 is an increasing function of arbitrageurs' gross return"
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p109:0123"
    chunk_hash: "487fb7c7bcc963a05a7ecc40985ae1777d5509ca7b7aa4103b5ab82ec6c7eb14"
    page_range: [109, 109]
    quote: "a market driven by PBA loses its resiliency in extreme"
    edge_type: "supports"
card_hash: "775183042ac59c68c28c634c0eb986fc083d6e6553ef32dff303736aa428eb76"
---
# Performance-Based Arbitrage And Fund-Flow Pressure

## Intuition

Real arbitrage is mostly conducted by specialized professionals managing *other people's* money — hedge funds, mutual funds, lenders against collateral. Brains and capital are separated by an agency relationship: the investors who supply the capital do not understand the trade and observe only the arbitrageur's returns. When a mispricing deepens, the arbitrageur posts an interim loss; the investors, unable to tell a deepening-mispricing loss from incompetence, rationally infer lower ability and *withdraw* capital. This makes the arbitrageur most capital-constrained precisely when the opportunity is best. **Source:** Shleifer (2000) Ch.4 pp.89-99.

Shleifer and Vishny (1997) call this **performance-based arbitrage (PBA)**: capital follows past returns rather than expected returns. It breaks the comforting link, assumed in the simple model of [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#intuition), that arbitrageurs grow more aggressive as prices move further from value. Under PBA, after a price shock the arbitrageur faces redemptions and may be forced to *liquidate* the very position that has become most underpriced, pushing the price still further from fundamentals. **Source:** Shleifer (2000) Ch.4 pp.90-90.

The consequence is that arbitrage is least effective in "extreme circumstances" — large displacements, fully invested arbitrageurs, low probability of near-term recovery. A market driven by PBA loses its resiliency in panics, and the analysis connects directly to the fire-sale literature (Shleifer-Vishny 1992, Stein 1996, Pulvino 1998): an asset is liquidated involuntarily exactly when its natural buyers (other arbitrageurs in the same trade) are also capital-starved. **Source:** Shleifer (2000) Ch.4 pp.101-101.

## Definition

**Performance-based arbitrage (PBA)** is the responsiveness of funds under management to *past* returns: investors increase (decrease) the capital they allocate to an arbitrageur after good (poor) past performance, rather than allocating on expected future returns from the trades. **Source:** Shleifer (2000) Ch.4 pp.98-98.

**Agency separation of brains and resources** is the structural feature that the arbitrageur has the knowledge while outside investors hold the capital, observe only returns, and cannot evaluate the strategy — the reason capital must be allocated on the noisy signal of past performance. **Source:** Shleifer (2000) Ch.4 pp.89-89.

**Forced liquidation / loss of resiliency** is the outcome under PBA in which an adverse interim shock triggers withdrawals that compel the arbitrageur to sell the underpriced asset, so prices fall *more* than one-for-one with the noise-trader shock and the market's price-recovery capacity collapses. **Source:** Shleifer (2000) Ch.4 pp.108-109.

**Extreme circumstances** are the parameter region where the initial displacement is very large, the arbitrageur is fully invested at the first date, and a further deepening of mispricing is likely — the regime in which PBA makes arbitrage perversely least aggressive. **Source:** Shleifer (2000) Ch.4 pp.97-98.

## Mathematical Reasoning

A specific asset has fundamental value `V` (known to arbitrageurs but not investors), three dates `t = 1, 2, 3`, and price `p_t`. Pessimistic noise traders generate aggregate demand `QN(t) = [V - S_t]/p_t`, where `S_t` is the period-`t` pessimism shock; there is some chance the shock deepens, `S_2 > S_1`, before it corrects at `t = 3` where `p_3 = V`. Arbitrageurs have cumulative resources `F_t` (including borrowing capacity). With demand `D_1` invested at date 1, `p_1 = V - S_1 + D_1`; if the shock deepens and arbitrageurs invest all of `F_2`, `p_2 = V - S_2 + F_2`. **Source:** Shleifer (2000) Ch.4 pp.91-92.

The PBA mechanism is the fund-supply function: `F_2 = F_1·G[(D_1/F_1)·(p_2/p_1) + (F_1 - D_1)/F_1]` with `G(1) = 1, G' >= 1, G'' <= 0`, so funds under management at date 2 are an increasing function of the arbitrageur's gross return `p_2/p_1`. Using the linear form `G(x) = a·x + 1 - a` with `a >= 1` gives `F_2 = a·[D_1·(p_2/p_1) + (F_1 - D_1)] + (1 - a)·F_1`, equivalently `F_2 = F_1 - a·D_1·(1 - p_2/p_1)`: when `p_2 = p_1` funds are unchanged, when `p_2 < p_1` funds are withdrawn, and higher `a` means stronger sensitivity (at `a = 1` no gains are added; at `a > 1` capital is actively withdrawn after losses). **Source:** Shleifer (2000) Ch.4 pp.94-95.

The first-order condition to the arbitrageur's date-1 problem is `(1 - q)·(V/p_1 - 1) + q·(p_2/p_1 - 1)·(V/p_2) >= 0`, where `q` is the probability the shock deepens. The first term is the gain from extra investment if the market recovers; the second is the foregone option of investing more if the price falls further. For `q < q*` arbitrageurs invest fully at date 1 (`D_1 = F_1`); for `q > q*` they hold reserves (`D_1 < F_1`). **Source:** Shleifer (2000) Ch.4 pp.97-98.

```
   PBA in extreme circumstances (fully invested at t=1)
   price
     V  |  *................................* p_3 = V (correction)
        |   \                              /
        |    \  (1) shock S_1               
   p_1  |     *                            /
        |      \   redemptions force       
   p_2  |       *----- liquidation -------/  p_2 < p_1: price falls
        |        (shock deepens to S_2)        MORE than 1-for-1
        +-------------------------------------> t   1     2     3
   dp_2/dS < -1: arbitrage least aggressive when mispricing worst
```

In the fully-invested equilibrium, `p_2 = [V - S - a·F_1 + F_1]/[1 - a·F_1/p_1]` and Proposition 4 establishes `dp_2/dS < -1` with `d^2 p_2/(dS)^2 < 0`: when arbitrageurs are fully invested, prices fall *more* than one-for-one with the noise shock, and the per-unit price decline rises as PBA intensifies (`a` larger). This contrasts with the simple model where arbitrageurs are most aggressive when prices are furthest from value; here the times they lose money are precisely when prices are far from fundamentals, so their trading has the weakest stabilizing effect — consistent with Friedman's observation that destabilizing arbitrage requires arbitrageurs to lose money on average, which PBA reconciles (they make money on average, not always). **Source:** Shleifer (2000) Ch.4 pp.100-101.

## See Also

- [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning) — the base limits-to-arbitrage gate; PBA supplies the agency/capital-constraint channel that bounds it in panics.
- [`be-investor-clientele-segmentation.md`](./be-investor-clientele-segmentation.md#intuition) — the segmentation of arbitrage capital into narrow strategy "segments" that PBA assumes.
- [`be-destabilizing-arbitrage-positive-feedback.md`](./be-destabilizing-arbitrage-positive-feedback.md#intuition) — the other channel (Ch.6) where arbitrage fails to stabilize, via anticipatory amplification.
- [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#intuition) — the noise-trader-risk source of the mispricing PBA fails to correct.

## Escalate to Raw When

- The full equilibrium with the corner and interior solutions (Propositions 1–4) and the role of `q*` requires the chapter's own case analysis rather than the card's first-order-condition summary. **Source:** Shleifer (2000) Ch.4 pp.97-100.
- The mapping to which markets attract arbitrage (bond, FX vs. equity) and the fire-sale empirical evidence (Pulvino 1998, Stein 1996) is needed for application. **Source:** Shleifer (2000) Ch.4 pp.101-101.
- The signal-extraction problem that determines the responsiveness `G'` (separating bad luck, deepening sentiment, and inferior ability) needs the chapter's own treatment. **Source:** Shleifer (2000) Ch.4 pp.94-94.
