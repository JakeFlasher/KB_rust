---
schema_version: "cacg.v0"
id: "mt-liquidity-feedback-corporate-investment"
title: "Feedback from Secondary-Market Liquidity to Real Investment and Corporate Policy"
reading_id: "14_microstructure_and_trading"
summary: "Informative secondary-market prices feed back into the real economy: managers learn from the order flow, so more informed trading raises price informativeness and the sensitivity of corporate investment to stock prices."
tags: ["microstructure", "price-discovery", "informed-trading", "corporate-investment", "feedback-effect"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p340:0516"
    chunk_hash: "9fe95e56ce13dfd5a30f714c36566af23019061330dbcd49d279105656bdac59"
    page_range: [340, 340]
    quote: "the sensitivity of investment to stock prices is greater"
    edge_type: "supports"
---
# Feedback from Secondary-Market Liquidity to Real Investment and Corporate Policy

## Intuition
Microstructure usually treats the order flow as a way to *reveal* an exogenous fundamental value. The feedback channel reverses the arrow: when a firm's managers do not know everything about a pending project, they can *learn* from how the market reacts to its announcement, and then make the real investment decision that the price implies. Causation now runs in both directions — investment determines cash flows (and hence price), but price also determines investment.

```
         private signal (prob gamma)
                 |
                 v
  announce ---> SECONDARY MARKET ---> price p reveals buy/sell ---> manager
  project        (informed + noise        (Bayesian update)         invests
  at date 0       traders, date 1)                                  or not (date 2)
                 ^                                                     |
                 |                                                     v
                 +------------- future cash flows / firm value <-------+
                                    (the "feedback" loop)
```

The market is useful precisely when the manager is *uninformed*: a trade at the ask says "informed buyers think this is high-quality," which can tip a borderline project from "skip" to "invest." Empirically this predicts that firms whose prices are more informative (more informed trading, higher PIN) should show investment that tracks their stock price more closely — which is what Chen, Goldstein, and Jiang (2007) find.

**Source:** Foucault, Pagano & Röell (2013) §10.4.1 pp.339-340

## Definition
Setup (one-period feedback model). At date 0 a firm of base value V announces a project that is high-quality (H) or low-quality (L) with equal prior probability. The project changes firm value by ΔV = G > 0 if it invests and the project is H, ΔV = −I if it invests and the project is L, and ΔV = 0 if it does not invest. Maintained "non-viability" assumption: G < I, so the unconditional NPV ½G − ½I < 0.

At date 1 the stock trades as in a Glosten–Milgrom market: with probability π an informed risk-neutral speculator (who knows H/L) trades, with probability 1 − π a liquidity trader buys or sells with equal odds; competitive uninformed market makers quote ask a (buy) and bid b (sell), so the transaction price reveals trade direction. Independently, with probability γ the manager privately learns quality. At date 2 the manager invests using his private signal if he has one, otherwise using the price.

**Source:** Foucault, Pagano & Röell (2013) §10.4.1 pp.339-340

## Mathematical Reasoning
The manager Bayesian-updates on the trade direction. A buy executes at the ask, a sell at the bid, giving
Pr(H | p = a) = (1 + π)/2 and Pr(H | p = b) = (1 − π)/2.
After a trade at the ask his posterior expected NPV is Pr(H|a)·G + (1 − Pr(H|a))·(−I) = [(1+π)/2]G − [(1−π)/2]I. He invests on a buy order iff this is positive, i.e. iff the **informativeness condition**

  π ≥ (I − G)/(I + G).

This is easier to satisfy when I ≈ G (a borderline project), since a little positive news then flips the sign. A trade at the bid only lowers an already-negative prior, so he never invests on a sell.

Allocative value: compare firm value with vs. without a listing (assume the condition holds). Unlisted, the manager invests only on positive private news, V_private = V + (γ/2)G. Listed, he additionally invests when a noise trader happens to buy, so
V_public = V_private + (1 − γ)·[ (π/2)G − ((1−π)/2)·((I − G)/2) ],
an **informational gain** (investing when an informed buy signals H) minus a **loss from noise** = (1 − γ)·((1−π)/4)·(I − G) (over-investing when a noise buy misleads, since I > G). The net gain V_public − V_private is positive whenever the informativeness condition holds and is increasing in π.

Comparative statics tying microstructure to the real economy: the equilibrium spread S = πG − [(1−π)(1−γ)/2](I − G) is increasing in π. Thus raising informed trading π simultaneously (i) raises price informativeness and the market's ability to guide investment (larger V_public − V_private) and (ii) *reduces* liquidity (wider spread). It also raises investment frequency — from γ/2 (below the threshold) up to 1/2 (above it) — and raises the average price, because better allocation lifts expected value.

**Source:** Foucault, Pagano & Röell (2013) §10.4.1 pp.339-340

## Boundary Notes
- The result that the market *encourages* investment is not robust: it depends on the non-viability assumption G < I. Reverse it (G > I, project viable on average) and the market becomes a *brake*, prompting the manager to forgo investment when traders sell.
- The feedback channel produces a counterexample to the Chapter-9 view that liquidity is universally value-enhancing: here more informed trading *lowers* liquidity (wider spread) yet *raises* firm value, so liquidity and informativeness can trade off rather than move together.
- Requires the manager to be sometimes uninformed (γ < 1): if he always knows quality, the price adds nothing. The channel also assumes the manager cannot himself manipulate the price he learns from; simple settings rule out profitable manipulation, but it can reappear in richer feedback models.
- This is a learning/real-efficiency mechanism, distinct from the liquidity-as-discount-rate channel that lowers the cost of capital; both can raise investment but through different economics.

**Source:** Foucault, Pagano & Röell (2013) §10.4.1 pp.339-340

## See Also
- [`mt-informed-traders-price-efficiency`](./mt-informed-traders-price-efficiency.md) -- supplies the informed-vs-noise trading that makes π (price informativeness) the key feedback driver
- [`mt-information-shares-price-discovery`](./mt-information-shares-price-discovery.md) -- how prices aggregate private signals, the input the manager learns from
- [`mt-liquidity-premium-asset-pricing`](./mt-liquidity-premium-asset-pricing.md) -- the contrasting cost-of-capital channel through which liquidity affects value and investment

## Escalate to Raw When
The source derives the full set of equilibrium quotes — ask (10.8), bid (10.9), speculator profits (10.10)–(10.11), spread (10.12), midprice — and verifies the conjectured informed strategy is optimal; this card only sketches the spread and value comparative statics. Re-read §10.4.1 pp.339-340 for the exact bid/ask algebra and the manipulation/robustness exercises, and §10.4.2 for the parallel executive-compensation (pay-for-performance) extension.
