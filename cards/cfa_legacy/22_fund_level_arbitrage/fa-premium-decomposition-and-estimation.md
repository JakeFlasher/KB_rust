---
schema_version: "cacg.v0"
id: "fa-premium-decomposition-and-estimation"
title: "Premium Decomposition: Price Discovery vs Transitory Liquidity"
reading_id: "22_fund_level_arbitrage"
summary: "The observed ETF premium pi = p - n mixes three things: lagged price discovery from stale NAV, NAV pricing noise, and transitory liquidity. The Madhavan-Sobczyk state-space model splits price into fundamental v plus a mean-reverting shock u, and a Kalman filter recovers the unobserved true premium u from prices and NAVs."
tags: ["premium-decomposition", "state-space", "kalman-filter"]
citations:
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p050:0054"
    chunk_hash: "a66575d5d61feaf2fa2c7ad5bf095ec17669ca58af6c5f29248edba3c43eb868"
    page_range: [52, 52]
    quote: "We can use the general framework to analyze questions concerning price discovery, the dynamics of premiums and discounts, return autocorrelations, performance and tracking relative to benchmark, transaction costs, and liquidity sourcing in underlying and secondary markets."
    edge_type: "defines"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p069:0080"
    chunk_hash: "d0d700f4c61bd67c7311fff25548ef2ad5e0d317bf44c0f4dffc0d946f853160"
    page_range: [69, 70]
    quote: "because it aligns with the model directly and lets us explicitly estimate the unobserved true premium,"
    edge_type: "supports"
---
# Premium Decomposition: Price Discovery vs Transitory Liquidity

## Intuition
The number a screen reports as an ETF's "premium" — price minus NAV — is not a clean mispricing signal. It bundles together three economically distinct forces. First, if NAV is **stale** (it lags the true value), then even a perfectly priced ETF will look like it is at a premium simply because NAV has not yet caught up to news; this component is really *price discovery* happening in the more-current ETF tape. Second, NAV carries its own *pricing noise* (e.g., bid-side bond marks). Third, there is a *transitory liquidity* component: genuine, mean-reverting deviations of price from fundamental value driven by flow pressure. By modeling how price and NAV each evolve over time, the premium can be statistically pulled apart into these pieces, so an investor can ask whether a large observed premium is benign (just stale NAV) or a real, fade-able pricing error.

```
   observed premium  pi_t = p_t - n_t
                 |
   +-------------+-------------------------+
   |             |                          |
 PRICE        NAV PRICING               TRANSITORY
 DISCOVERY    NOISE                     LIQUIDITY
 (staleness   (w_t marks,              (u_t: mean-reverting
  phi * past   bid convention)          flow shock, fades
  fundamental                           at rate psi)
  returns)
   |                                      |
 stale NAV lags v   <----arbitrage----   p pulled back to v
```

**Source:** Madhavan (2016) §3.1, §3.3.1 pp.52-53, 59.

## Definition
Let `v_t` be the unobserved expected (fundamental) value, `p_t` the ETF price, and `n_t` the NAV. The model sets price equal to fundamental value plus a transitory shock, `p_t = v_t + u_t`, where `u_t` is the "true premium" arising from transitory liquidity pressure. NAV is a (possibly stale, possibly noisy) weighted average of current and past value, `n_t = (1 - phi) v_t + phi n_{t-1} + w_t`, with staleness parameter `0 <= phi <= 1` and microstructure noise `w_t`. The transitory shock itself mean-reverts: `u_t = psi u_{t-1} + eps_t`, where `psi` is the autocorrelation coefficient (inverse arbitrage speed) and `eps_t` is a liquidity shock. The *observed* premium is defined as the deviation of the ETF price from the NAV of the fund, `pi_t = p_t - n_t`. The *true* premium is `u_t = p_t - v_t`, which equals the observed premium only when NAV tracks value exactly (`phi = 0`, `w_t = 0`).

**Source:** Madhavan (2016) §3.2.2, §3.2.4 pp.54, 56.

## Mathematical Reasoning
Substituting `p_t = v_t + u_t` into `pi_t = p_t - n_t` and solving out the NAV recursion yields the central decomposition:

```
pi_t = phi(r_t + phi*r_{t-1} + ...)        <- price discovery (staleness x past fundamental returns)
     + (1 - phi)(w_t + phi*w_{t-1} + ...)  <- NAV pricing noise
     + (eps_t + psi*eps_{t-1} + ...)       <- transitory liquidity (past flow innovations)
```

So the premium is a sum of three weighted moving averages keyed by the two structural parameters `phi` and `psi`. Comparative statics:
- **Staleness limit (`phi -> 0`, `w_t -> 0`):** the first two terms vanish and `pi_t = eps_t + psi*eps_{t-1} + ...`, a pure weighted average of past flow shocks — every premium is transitory liquidity, fully fade-able.
- **Speed of arbitrage:** lower `psi` => faster error correction (the `psi=0` extreme corrects errors immediately). Higher `psi` => past shocks persist and weigh more on today's premium.
- **Half-life:** since `E[u_{t+h}] = psi^h * u_t`, setting `E[u_{t+h}] = 0.5 u_t` gives `h = ln(0.5) / ln(psi)`, the time to halve a pricing error — monotonically increasing in `psi`.
- **Cross-asset staleness ordering:** `phi` rises as one moves from the most liquid asset classes (domestic equity, `phi ~ 0`) to less liquid ones (international, fixed income), so reported premiums in international/illiquid funds are disproportionately *discovery*, not mispricing.

Because `v_t` and `u_t` are unobserved, the eight parameters (`phi`, `psi`, and the means/variances of `eps_t`, `w_t`, `r_t`) are estimated by casting the system in state-space form — a measurement equation in `(p_t, n_t)` and a random-walk transition equation in `v_t` — and running a Kalman filter, the optimal estimator for extracting signal from noisy observations, which also recovers the latent path of `u_t`.

**Source:** Madhavan (2016) §3.2.3, §3.3.1, §4.3.2, §4.4 pp.55, 59, 69-71.

## See Also
- [`fa-true-vs-reported-premium-price-discovery-share`](./fa-true-vs-reported-premium-price-discovery-share.md) — extends this decomposition to compute what *share* of an observed premium is true (fade-able) vs reported (stale-NAV) discovery.
- [`fa-etf-vs-cef-premium-discount`](./fa-etf-vs-cef-premium-discount.md) — premiums/discounts as a structural phenomenon; this card supplies the dynamic model behind why ETF premiums mean-revert while CEF ones can persist.
- [`fa-nav-staleness-and-arbitrage-speed`](./fa-nav-staleness-and-arbitrage-speed.md) — isolates the `phi` (staleness) and `psi` (arbitrage speed) parameters that drive the three-term split.
- [`fa-international-price-discovery-and-enav`](./fa-international-price-discovery-and-enav.md) — the international/illiquid funds where `phi` is largest and the discovery term dominates.

Legacy (other tree, prose only): this connects to the behavioral-finance noise-trader and limits-of-arbitrage cards (be-noise-trader-equilibrium, be-limits-of-arbitrage), since the transitory `u_t` component is exactly a flow-driven, slowly-corrected pricing error; and to the convertible-arb cb-arbitrage-strategy framing, where an arbitrageur similarly buys the cheaper of two linked instruments to capture a convergent spread.

## Escalate to Raw When
Go to Madhavan (2016) Chapters 3-4 when you need the worked HYG (iShares iBoxx High-Yield) crisis case in which the recovered state vector `v_t` is plotted against price and NAV to show NAV staleness widening the discount in September 2008 and price leading NAV upward in March 2009; the EEM example reporting concrete average/min/max daily premiums; the maximum-likelihood estimation details and the descriptive parameter tables giving fitted mean `phi` and `psi` by asset class with their significance fractions and the implied half-lives; and the augmented state-space variant that adds ETF flows as three extra parameters (flow autocorrelation, return sensitivity, market-impact coefficient). Those carry the concrete fitted numbers deliberately omitted here.

**Source:** Madhavan (2016) §3.3.2, §4.4, §4.5.1 pp.60, 70-72.
