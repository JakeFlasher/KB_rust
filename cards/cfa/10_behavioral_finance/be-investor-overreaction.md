---
schema_version: "cacg.v0"
id: "be-investor-overreaction"
title: "Long-Horizon Reversal And Investor Overreaction"
reading_id: "10_behavioral_finance"
summary: "Over three-to-five-year horizons securities overreact to consistent strings of same-direction news: De Bondt-Thaler show past extreme losers later outperform past extreme winners. This is the representativeness-driven overreaction signature, mirror image of short-horizon momentum."
tags: ["behavioral-finance", "overreaction", "long-horizon-reversal", "representativeness", "anomalies"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p129:0146"
    chunk_hash: "b38849a48a7d18f362c274eaf7e711fe146f61dde1cb51cf74f085c2aa5c0c83"
    page_range: [130, 130]
    quote: "The thrust of the evidence is that, over horizons of three to five years, there is a relatively slight negative autocorrelation in stock returns in many markets."
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p129:0145"
    chunk_hash: "48d46c3547e89f7f63917ae958e45f26a27479dd27f14adfbdc110da0c2163c7"
    page_range: [129, 129]
    quote: "after a series of announcements of good news, the investor becomes overly optimistic that future news announcements will also be good and hence overreacts"
    edge_type: "defines"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p120:0137"
    chunk_hash: "3957b4864cb441abe4ac95320470e0f9e4d26e0d37af6b30f88c75b0b36cacb2"
    page_range: [121, 121]
    quote: "over longer horizons of perhaps three to five years, security prices overreact to consistent patterns of news pointing in the same direction"
    edge_type: "supports"
card_hash: "c60a9fb59bf065b343f41a6f3a5bf28e7b6d25c6e590789bedfd0bbb67e042a6"
---
# Long-Horizon Reversal And Investor Overreaction

## Intuition

Over long horizons — roughly three to five years — security prices *overreact* to consistent patterns of news pointing in the same direction. A stock that has posted a long string of good performance becomes overpriced and earns low subsequent returns; a long string of bad performance becomes underpriced and earns high subsequent returns. De Bondt and Thaler (1985), looking at U.S. data back to 1933, DOCUMENT that portfolios of extreme prior-three-year losers dramatically outperform portfolios of extreme prior-three-year winners, even after standard risk adjustment. **Source:** Shleifer (2000) Ch.5 pp.121-121.

Shleifer reads this as the representativeness-driven **overreaction** pole of the two-model taxonomy in [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#intuition). When investors see a *series* of good-news announcements, the representativeness heuristic leads them to treat the streak as representative of a permanent high-quality regime; they become overly optimistic that future news will also be good and push the price to unduly high levels. Subsequent news tends to contradict the over-optimism, so the price reverts and returns are low — the long-horizon reversal. **Source:** Shleifer (2000) Ch.5 pp.120-120.

This is the mirror image of the short-horizon momentum/underreaction of [`be-momentum-anomaly.md`](./be-momentum-anomaly.md#intuition): the same market can drift *with* news at short horizons (conservatism) and *against* the cumulative streak at long horizons (representativeness). The overshoot-then-reversal is exactly the bounded-peak-then-convergence shape of the divergence path in [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition). **Source:** Shleifer (2000) Ch.5 pp.112-113.

## Definition

**Overreaction (long-horizon reversal)** is the property that the average return following a *series* of same-direction announcements is *lower* after a good-news series than after a bad-news series, so extreme prior performance reverses. **Source:** Shleifer (2000) Ch.5 pp.120-120.

**Winner / loser portfolios** are portfolios formed on extreme prior multi-year returns; the De Bondt-Thaler finding is that prior losers subsequently outperform prior winners (negative long-horizon autocorrelation in extreme returns). **Source:** Shleifer (2000) Ch.5 pp.121-121.

**Representativeness** is the Tversky-Kahneman heuristic of judging an outcome by how typical it is of a category and neglecting base rates and the laws of probability; here a streak of good results is taken as representative of a permanent high-quality regime, the source of the over-optimism. **Source:** Shleifer (2000) Ch.5 pp.113-113.

**Aggregate-index reversal** is the related long-horizon negative autocorrelation in market indices and the predictive power of valuation ratios (aggregate dividend yield, book-to-market): high valuations predict low subsequent returns over three-to-five-year horizons. **Source:** Shleifer (2000) Ch.5 pp.121-121.

## Mathematical Reasoning

Overreaction is defined by the conditional-mean inequality over a *series* of signals: `E(r_{t+1} | z_t = G, z_{t-1} = G, ..., z_{t-j} = G) < E(r_{t+1} | z_t = B, z_{t-1} = B, ..., z_{t-j} = B)` for `j >= 1` (and probably larger), the reverse of the single-signal underreaction inequality. The reversal of the inequality between the one-signal and many-signal cases is the formal fingerprint that distinguishes overreaction from underreaction at different horizons. **Source:** Shleifer (2000) Ch.5 pp.120-120.

The representativeness mechanism maps to the over-attribution operator from [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#mathematical-reasoning): after a salient streak the agent attaches probability `~1` to a new high-growth "regime" model and under-weights the chance that the streak is noise, so the implied expected return on the streak's fundamentals is set too high. As the regime hypothesis is disconfirmed by reverting fundamentals, the price falls — the cumulative return `R(t, t+h)` rises through an intermediate peak (the overshoot horizon) and then declines toward the reversal level. (The source documents the reversal empirically and asserts the mechanism without a closed-form derivation.) **Source:** Shleifer (2000) Ch.5 pp.121-122.

```
   cumulative return path under overreaction (winner stock)
   R(t,t+h)
        |        peak (overshoot horizon)
        |         _*_
        |       _/   \_
        |     _/       \__
        |   _/            \___ reversal: low long-horizon returns
     0  +--/-----------------------------> horizon h (years)
        | /
        |/  good-news streak inflates price, then reverts
   loser stock = vertical mirror image (undershoot then recovery)
```

The same overshoot logic underlies the value/glamour evidence: prolonged records of high (low) earnings growth produce the streaks that drive glamour (value) mispricing, so the long-horizon return reversal and the multiple-based value premium of [`be-value-anomaly.md`](./be-value-anomaly.md#mathematical-reasoning) are two measurements of one overreaction phenomenon. **Source:** Shleifer (2000) Ch.5 pp.121-122.

## See Also

- [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#definition) — the representativeness/overreaction pole this anomaly instantiates.
- [`be-momentum-anomaly.md`](./be-momentum-anomaly.md#intuition) — the short-horizon underreaction mirror image that coexists with long-horizon reversal.
- [`be-value-anomaly.md`](./be-value-anomaly.md#intuition) — the multiple-based measurement of the same overreaction-to-streaks mechanism.
- [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition) — the overshoot-then-convergence divergence path the reversal traces.

## Escalate to Raw When

- The De Bondt-Thaler winner/loser construction and the Chopra-Lakonishok-Ritter risk adjustments are needed to reproduce the long-horizon reversal spread. **Source:** Shleifer (2000) Ch.5 pp.121-121.
- The aggregate-index predictability evidence (dividend yield, book-to-market predicting three-to-five-year returns) requires the Campbell-Shiller / Pontiff-Schall specifications. **Source:** Shleifer (2000) Ch.5 pp.121-121.
- The Daniel-Hirshleifer-Subrahmanyam overconfidence model of overreaction needs its explicit private/public-signal weighting algebra rather than the symbolic over-attribution operator. **Source:** Shleifer (2000) Ch.5 pp.120-130.
