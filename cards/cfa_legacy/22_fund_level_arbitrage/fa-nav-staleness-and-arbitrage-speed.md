---
schema_version: "cacg.v0"
id: "fa-nav-staleness-and-arbitrage-speed"
title: "NAV Staleness & Arbitrage Speed: phi, psi and the Mean-Reversion Half-Life"
reading_id: "22_fund_level_arbitrage"
summary: "Two parameters govern fund-level mispricing dynamics: phi measures how stale NAV is (NAV anchored on past values), while psi is the autocorrelation of the true premium and sets how fast arbitrage corrects pricing errors via the half-life h = ln0.5/ln(psi)."
tags: ["nav-staleness", "arbitrage-speed", "premium-autocorrelation"]
citations:
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p054:0058"
    chunk_hash: "f2de641ac054244314597bb46b75b95924258b3d64fe4cd6259bc872faea3a7d"
    page_range: [54, 54]
    quote: "is a liquidity shock and ψ is the autocorrelation coefficient."
    edge_type: "defines"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p056:0061"
    chunk_hash: "758ce3ab1cde9e49398d561b1fc7576ae7b1e0baad919c5aee891bd42d3e4872"
    page_range: [56, 56]
    quote: "Here 0 ≤ φ ≤ 1 captures possible staleness and"
    edge_type: "defines"
card_hash: "292aeb6a1c8d28f45d23b59ff3a67d9a386c0783cdaa88e917af2c667b8dfcb7"
---
# NAV Staleness & Arbitrage Speed: phi, psi and the Mean-Reversion Half-Life

## Intuition
A fund's premium does not blink in and out instantly. Two frictions stretch it across time. First, the published NAV may not reflect *current* value: if constituent quotes are stale (illiquid bonds, time-zone-closed foreign stocks), NAV drags on yesterday's prices. The staleness parameter phi captures how heavily NAV leans on its own past versus fresh value. Second, even the *true* premium — the deviation of price from expected value, not from NAV — corrects only gradually because arbitragers scale back their trades to avoid price impact, hold limited capital, and bear inventory risk. The persistence of that true premium is the autocorrelation psi: high psi means errors linger for many periods, low psi means they snap back almost at once.

```
   true premium u_t (price - expected value)
        |  AR(1):  u_t = psi*u_{t-1} + eps_t
        v
   psi -> 0   : errors corrected immediately  (fast arbitrage)
   psi -> 1   : errors persist many periods    (slow / crowded)
        |
        +--> half-life h = ln(0.5)/ln(psi)   [periods to halve an error]

   NAV staleness:  n_t = (1-phi)*v_t + phi*n_{t-1} + w_t
        phi = 0  : NAV = current value (true marker)
        phi > 0  : NAV trails value (stale)
```

**Source:** Madhavan (2016) §3.2.3 pp.55-55.

## Definition
- True premium dynamics (Madhavan eq. 3.3): the deviation of price from expected value follows an AR(1) process, u_t = psi*u_{t-1} + eps_t, where eps_t is a liquidity shock and psi is the autocorrelation coefficient. Pricing errors are serially correlated but corrected over time.
- NAV staleness (Madhavan eq. 3.6): NAV is a weighted average of current (noisy) value and past NAV, n_t = (1 - phi)*v_t + phi*n_{t-1} + w_t, with 0 <= phi <= 1 capturing staleness and w_t a microstructure error term.
- Observed premium (eq. 3.5): pi_t = p_t - n_t, the deviation of ETF price from NAV — distinct from the *true* premium u_t = p_t - v_t measured against unobserved expected value v_t.

**Source:** Madhavan (2016) §3.2.2-3.2.4 pp.54-56.

## Mathematical Reasoning
Because the true premium is stationary AR(1), forecasting h periods ahead gives E[u_{t+h}] = psi^h * u_t. Setting the forecast to half the current error, E[u_{t+h}] = 0.5 * u_t, and solving psi^h = 0.5 yields the mean-reversion half-life:

    h = ln(0.5) / ln(psi)

Comparative statics:
- psi -> 0 implies h -> 0: errors are corrected immediately (the extreme efficient case).
- psi -> 1 implies h -> infinity: errors are arbitrarily persistent.
- Lower psi means faster arbitrage and less serial dependence in pricing errors; higher psi means price sits above or below fundamental value for multiple periods.

Economics of psi: it is *positively* related to dealer inventory costs, risk aversion, price impact, and uncertainty over fundamentals, and to the autocorrelation in exogenous flows. Intuitively, expected price impact in both the ETF and the underlying causes arbitragers to scale back trades (recall the optimal trade x_t = u_t/(2*lambda) under quadratic impact lambda*x_t^2), so prices do not jump straight to value; limited dealer capital and crowded one-sided flow reinforce the persistence.

Economics of phi: phi > 0 (staleness) arises when current constituent quotations are unavailable or unrepresentative (illiquid fixed income, last-trade pricing). phi = 0 makes NAV a true marker of value, so observed premia reflect only transitory liquidity shocks; phi < 0 corresponds to overreaction (pricing provider over-weights new information).

**Source:** Madhavan (2016) §3.2.3, §3.2.5 pp.55-57.

## See Also
- [`fa-premium-decomposition-and-estimation`](./fa-premium-decomposition-and-estimation.md) — phi and psi are the two parameters jointly estimated when decomposing the observed premium into price-discovery and transitory-liquidity components.
- [`fa-true-vs-reported-premium-price-discovery-share`](./fa-true-vs-reported-premium-price-discovery-share.md) — distinguishes the true premium u_t = p_t - v_t (the arbitrager's profit) from the reported premium pi_t = p_t - n_t that phi distorts.
- [`fa-limits-to-arbitrage-when-creation-channel-breaks`](./fa-limits-to-arbitrage-when-creation-channel-breaks.md) — extreme persistence (psi near 1) and the TVIX/UNG creation halts show why pricing errors are not instantly eliminated.
- Legacy: the behavioral-finance noise-trader and limits-of-arbitrage cards in the other tree frame the same idea — serially correlated mispricing that survives because arbitrage capacity is bounded — and the risk-management VaR notes connect inventory risk to dealer risk aversion in psi.

## Escalate to Raw When
Pull the raw text when you need the concrete worked illustration — Madhavan's §3.2.6 HYG example reporting the day-over-day NAV-return autocorrelation and the regression R-squared, or the stated half-life of a sample pricing error halving over a given number of days — and when you need the full state-space / moment-condition estimation machinery and the per-asset-class phi/psi estimates (domestic equity, international equity, fixed income), which Madhavan develops in Chapter 4 (the state-space representation, the eight-parameter per-fund fit, and the Table 4.2 asset-class estimates). Those numerical figures and the estimation detail are deliberately abstracted out of this skeleton.

**Source:** Madhavan (2016) §3.2.6 pp.56-57 (HYG illustration); §4.3-4.4 pp.69-72 (state-space estimation; per-asset-class phi/psi estimates, Table 4.2).
