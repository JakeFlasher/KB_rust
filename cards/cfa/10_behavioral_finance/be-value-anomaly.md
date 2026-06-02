---
schema_version: "cacg.v0"
id: "be-value-anomaly"
title: "Value Premium As Expectational Error"
reading_id: "10_behavioral_finance"
summary: "The value premium (high book-to-market / low-multiple stocks outperform glamour stocks) reflects expectational error: investors extrapolate past growth, overprice glamour and underprice value; La Porta's analyst-forecast sort shows the high-growth-forecast stocks earn the lowest returns, not a pure risk story."
tags: ["behavioral-finance", "value-premium", "overreaction", "expectational-error", "anomalies"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p130:0147"
    chunk_hash: "295283cc3a09786eb03acba2afdc5c00b56c31c41510c815713848889fd3a62f"
    page_range: [130, 130]
    quote: "Stocks with very high valuations relative to their assets or earnings (growth or glamour stocks), which tend to"
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p132:0150"
    chunk_hash: "082b78bcd36c3a63e571b0e25fa8cdf71cc3c5d1878392fffd05bfb2cfd1020e"
    page_range: [133, 133]
    quote: "Investor expectations of future growth implied by the pricing multiples appear to be excessively tied to past growth despite the fact that future growth rates are more stationary."
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p133:0151"
    chunk_hash: "624e591d60e31fd523e8f8cdefed447ea2edb831cb5917d658560427bfde3e2c"
    page_range: [133, 133]
    quote: "He finds that analysts are excessively bullish about the stocks they are most optimistic about and excessively bearish about the stocks they are most pessimistic about."
    edge_type: "supports"
card_hash: "a502ad7539213ba69ac01acb68d209b0af8bfe0be0ab43ac2d6cddc8ac21c4d1"
---
# Value Premium As Expectational Error

## Intuition

Sorting stocks by valuation multiples — book-to-market, cash-flow-to-price, earnings-to-price — produces a robust spread in subsequent returns: cheap "value" stocks (high book-to-market, low multiples) earn high risk-adjusted returns while expensive "glamour"/growth stocks earn low returns. Lakonishok, Shleifer, and Vishny DOCUMENT spreads of 8–10 percent per year between the extreme value and glamour deciles in U.S. data 1968–1989, with the pattern extended to Europe, Japan, and emerging markets. **Source:** Shleifer (2000) Ch.5 pp.122-122.

Shleifer's behavioral reading is that this is the representativeness-driven **overreaction** signature of the two-model taxonomy in [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#intuition). Glamour portfolios typically have prolonged prior records of high earnings growth; value portfolios have records of consistently poor growth. Investors extrapolate that record into the indefinite future, even though future growth rates are far more stationary (they revert toward the cross-sectional mean). The over-extrapolation overprices glamour and underprices value; the subsequent reversal is the realized value premium. **Source:** Shleifer (2000) Ch.5 pp.122-122.

The strongest discriminating evidence against a pure risk story comes from looking at expectations directly. La Porta (1996) sorts stocks by analysts' long-term earnings-growth forecasts and finds the highest-forecast decile earns far *lower* future returns than the lowest-forecast decile — and the high-forecast stocks earn negative returns when they subsequently announce earnings. In an efficient market, stocks with optimistic growth forecasts should not earn low returns; the pattern points to expectational error in prices, not just in analysts. This connects the anomaly to the divergence-path framing of [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition). **Source:** Shleifer (2000) Ch.5 pp.124-124.

## Definition

**Value (cheap) stocks** are stocks with low prices relative to book value, earnings, cash flow, or dividends (high book-to-market, low multiples); they earn relatively high subsequent risk-adjusted returns. **Glamour / growth stocks** are the mirror image — high multiples, low subsequent returns. **Source:** Shleifer (2000) Ch.5 pp.122-122.

**Value premium** is the positive spread in average risk-adjusted returns of value over glamour deciles, measured by book-to-market or cash-flow-to-price sorts. **Source:** Shleifer (2000) Ch.5 pp.122-122.

**Expectational-error (extrapolation) interpretation** is Lakonishok-Shleifer-Vishny's account that the multiples embed expectations of future growth that are excessively tied to past growth; because growth mean-reverts, glamour disappoints and value over-delivers, generating the reversal. **Source:** Shleifer (2000) Ch.5 pp.124-124.

**Risk (Fama-French) interpretation** is the competing rational claim that high book-to-market proxies for "distress" risk, so the value return is fair compensation; Shleifer notes direct evidence for this conjecture had not been provided and that the comovement Fama-French cite is also consistent with common *sentiment*. **Source:** Shleifer (2000) Ch.5 pp.122-124.

## Mathematical Reasoning

The overreaction-to-a-series operator from the two-model taxonomy applies: after a series of good-news announcements `z_t = G, z_{t-1} = G, ..., z_{t-j} = G`, the investor over-attributes the streak to a structural high-growth regime, so the expected subsequent return is *lower* than after a comparable bad-news series — formally `E(r_{t+1} | z_t = G, ..., z_{t-j} = G) < E(r_{t+1} | z_t = B, ..., z_{t-j} = B)`. Glamour stocks are the empirical realization of the long good-news streak, value stocks of the long bad-news streak. **Source:** Shleifer (2000) Ch.5 pp.120-121.

The expectational-error mechanism can be stated as an inconsistency between the *implied* and the *true* growth process. Investors set the multiple as if expected growth `g^e` tracks recent realized growth `g_past`, but the true conditional expectation `E[g_future | g_past]` is far closer to the unconditional mean (growth is more stationary). The mispricing is the gap `g^e - E[g_future | g_past]`, positive for glamour and negative for value, and the value premium is its reversal. (The source documents these as empirical regularities and asserts the mechanism without a closed-form pricing derivation.) **Source:** Shleifer (2000) Ch.5 pp.124-124.

```
   expected vs. realized future growth (extrapolation error)
   growth
     high |  o investor-implied g^e  (tracks past streak)
          |   \
          |    \   *  realized future growth (mean-reverts)
   mean   |- - -*- - - - - - - - - - - - - - - - -
          |    *
          |   /  o  value: implied too low, realized higher
     low  |  o
          +------------------------------------> glamour ... value
   gap (o minus *) = mispricing; its later reversal = value premium
```

La Porta's sort makes the error observable: ranking by analysts' expected-growth decile, raw subsequent returns *decline monotonically* from the lowest-growth-forecast decile (~30 percent) to the highest (~9 percent), the opposite of what a growth-premium would require, and high-forecast stocks earn negative announcement returns. (The source presents this as a documented monotone empirical pattern.) **Source:** Shleifer (2000) Ch.5 pp.125-125.

## See Also

- [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#definition) — the representativeness-driven overreaction side of the taxonomy this anomaly instantiates.
- [`be-investor-overreaction.md`](./be-investor-overreaction.md#intuition) — the De Bondt-Thaler long-horizon reversal, the return-based sibling of the multiple-based value premium.
- [`be-momentum-anomaly.md`](./be-momentum-anomaly.md#intuition) — the conservatism/underreaction signature that coexists with value overreaction at shorter horizons.
- [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition) — the divergence-then-convergence path the extrapolation error traces.

## Escalate to Raw When

- The Fama-French risk-vs-overreaction debate must be adjudicated with the explicit risk-adjustment and comovement evidence rather than the card's qualitative contrast. **Source:** Shleifer (2000) Ch.5 pp.122-124.
- The Lakonishok et al. decile construction (book-to-market and cash-flow-to-price, size-adjusted abnormal returns over five post-formation years) is needed to reproduce the spread. **Source:** Shleifer (2000) Ch.5 pp.122-123.
- The La Porta expectational-error result requires the analyst-forecast sorting methodology and the announcement-return tests, not just the monotone return pattern. **Source:** Shleifer (2000) Ch.5 pp.124-124.
