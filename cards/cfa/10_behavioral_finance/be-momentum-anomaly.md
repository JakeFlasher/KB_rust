---
schema_version: "cacg.v0"
id: "be-momentum-anomaly"
title: "Momentum As Underreaction To News"
reading_id: "10_behavioral_finance"
summary: "Short-horizon return continuation (momentum, post-earnings-announcement drift) is the conservatism-driven underreaction signature: positive return autocorrelation over ~one month to one year as prices slowly incorporate news, documented by Bernard's SUE drift and Jegadeesh-Titman / Chan et al. six-month momentum."
tags: ["behavioral-finance", "momentum", "underreaction", "conservatism", "anomalies"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p123:0140"
    chunk_hash: "cb89304782bc5444505ad3b1c323ab87ffa73d6e89fe4b4e87e7a516a22137ad"
    page_range: [123, 123]
    quote: "By underreaction we mean that the average return on the company's stock in the period following an announcement of good news"
    edge_type: "defines"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p125:0142"
    chunk_hash: "b0547c1477d56ae56cee0df2e166f127ddfdda84795fa4e08907b41dd666e826"
    page_range: [125, 125]
    quote: "the market underreacts to the earnings announcement in revising a"
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p125:0142"
    chunk_hash: "b0547c1477d56ae56cee0df2e166f127ddfdda84795fa4e08907b41dd666e826"
    page_range: [126, 126]
    quote: "investors typically (but not always) believe that earnings are more stationary than they really are."
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p126:0143"
    chunk_hash: "1fad58fac73c7d681c525e108215dc4d7937520e3017341c9f8c01db00be024b"
    page_range: [126, 126]
    quote: "underperforms the past six-month winner portfolio by nearly nine percent."
    edge_type: "supports"
card_hash: "d95315f0fc68fe87406aae4b4b3aba0c7b03f7a0591d588acb1434e2c4252051"
---
# Momentum As Underreaction To News

## Intuition

Momentum is the empirical regularity that securities with good recent returns or good recent news keep outperforming, and recent losers keep underperforming, over horizons of roughly one month to one year. Shleifer frames this as the conservatism-driven **underreaction** signature of the two-model taxonomy in [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#intuition): prices do not jump fully to the new fundamental value when news arrives but adjust part way and then drift, so current news predicts not only the immediate reaction but also future returns. **Source:** Shleifer (2000) Ch.5 pp.114-114.

The cleanest version is post-earnings-announcement drift. Bernard (1992) sorts stocks into deciles by standardized unexpected earnings (SUE); after the announcement, high-SUE stocks continue to earn positive abnormal returns and low-SUE stocks negative ones — the highest-SUE decile beats the lowest by 4.2 percent over the 60 trading days *after* portfolio formation. Stale information (the SUE) predicts future risk-adjusted returns, a violation of semi-strong efficiency. The behavioral root is that investors treat earnings as more stationary (more random-walk-like) than they truly are, missing the positive short-horizon autocorrelation in earnings changes. **Source:** Shleifer (2000) Ch.5 pp.116-117.

Return-based momentum tells the same story: Jegadeesh-Titman (1993) and Chan, Jegadeesh, and Lakonishok (1996) find that ranking by the prior six-month return predicts the next six months — the loser portfolio underperforms the winner portfolio by nearly nine percent, and losers have negative earnings surprises both before and after formation. Momentum and earnings drift are thus the same underreaction phenomenon. This is the upward-drift segment of the divergence path in [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition). **Source:** Shleifer (2000) Ch.5 pp.117-120.

## Definition

**Underreaction** is the property that the average return in the period *following* a good-news announcement exceeds the average return following a bad-news announcement, i.e., `E(r_{t+1} | z_t = G) > E(r_{t+1} | z_t = B)`, so the stock under-adjusts at the announcement and the impact spreads over time. **Source:** Shleifer (2000) Ch.5 pp.114-114.

**Momentum** is positive autocorrelation of returns over relatively short horizons (about six months to one year): past winners continue to win, past losers continue to lose, even after standard risk adjustment. **Source:** Shleifer (2000) Ch.5 pp.117-117.

**Post-earnings-announcement drift (SUE drift)** is the continued cumulative abnormal return in the direction of an earnings surprise after the announcement date, measured by sorting on standardized unexpected earnings. **Source:** Shleifer (2000) Ch.5 pp.115-116.

**Conservatism** is the psychological tendency (Edwards 1968) to update beliefs too slowly in the face of new evidence; here investors believe earnings are more stationary than they are, so they discount the information content of a surprise and under-revise. **Source:** Shleifer (2000) Ch.5 pp.113-117.

## Mathematical Reasoning

Underreaction is defined by the conditional-mean inequality `E(r_{t+1} | z_t = G) > E(r_{t+1} | z_t = B)` where `z_t in {G, B}` is the period-`t` news. Because the announcement is only partly impounded at `t`, its residual impact appears as a predictable next-period return — stale information has predictive power, the formal violation of semi-strong efficiency. **Source:** Shleifer (2000) Ch.5 pp.114-114.

The conservatism mechanism is captured by the belief-revision operator from [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#mathematical-reasoning): the posterior `B(t+1) = (1 - lambda)·B(t) + lambda·B_Bayes(t+1)` with `lambda < 1`, so the agent moves toward the Bayesian update but under-shoots; lower `lambda` (stronger conservatism) lengthens the continuation drift. The empirical counterpart is the autocorrelation structure of earnings changes: Bernard-Thomas (1990) report quarterly-earnings-change autocorrelations of about `0.34, 0.19, 0.06` at lags 1–3 and `-0.24` at lag 4 — investors who assume a random walk ignore the positive short-lag autocorrelation, hence the drift. **Source:** Shleifer (2000) Ch.5 pp.117-117.

```
   cumulative abnormal return after a positive earnings surprise
   CAR
        |                         _____ high SUE (drift up)
        |                  ______/
        |    announce  ___/
     0  +-------|------/------------------------> event time
        |       |   \__
        |            \____
        |                 \______ low SUE (drift down)
   underreaction: price keeps moving in the news direction
   for ~60 trading days AFTER the jump (no immediate full adjustment)
```

The Chan-Jegadeesh-Lakonishok evidence ties the two: ranking on the prior six-month return, the loser decile's subsequent-quarter standardized unexpected earnings are sharply negative while the winner decile's are sharply positive — momentum portfolios *are* SUE portfolios. (The source presents these decile means as documented data, not a derived model.) **Source:** Shleifer (2000) Ch.5 pp.117-120.

## See Also

- [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#definition) — the conservatism/underreaction side of the taxonomy this anomaly instantiates.
- [`be-value-anomaly.md`](./be-value-anomaly.md#intuition) — the overreaction counterpart at long horizons that coexists with short-horizon momentum.
- [`be-investor-overreaction.md`](./be-investor-overreaction.md#intuition) — the long-horizon reversal that follows once continuation overshoots.
- [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition) — the slow-convergence segment of the divergence path that drift traces.

## Escalate to Raw When

- The Barberis-Shleifer-Vishny conservatism model needs its explicit two-state regime-switching derivation rather than the symbolic under-shoot operator. **Source:** Shleifer (2000) Ch.5 pp.131-135.
- The Bernard SUE methodology (decile sorting, cumulative abnormal returns, the 4.2 percent post-formation spread) must be reproduced from the figure rather than summarized. **Source:** Shleifer (2000) Ch.5 pp.115-116.
- The drift-after-corporate-events evidence (repurchases, dividend initiations, splits, seasoned equity offerings) is needed to show underreaction is pervasive beyond earnings. **Source:** Shleifer (2000) Ch.5 pp.120-120.
