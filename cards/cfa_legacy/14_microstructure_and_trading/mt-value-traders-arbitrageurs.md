---
schema_version: "cacg.v0"
id: "mt-value-traders-arbitrageurs"
title: "Value Traders and Arbitrageurs: Resilience and Cross-Market Liquidity Transfer"
reading_id: "14_microstructure_and_trading"
summary: "Value traders supply depth and resilience by trading against deviations from fundamental value, while arbitrageurs transfer liquidity across related markets and enforce the law of one price."
tags: ["microstructure", "value-traders", "arbitrage", "resilience", "law-of-one-price", "liquidity-supply"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p353:0587"
    chunk_hash: "e37e8a6178d11bed5dd8167fae624242b1454b396885b2066c7ff69dbe1877f2"
    page_range: [353, 354]
    quote: "Value traders make markets resilient by standing"
    edge_type: "defines"
card_hash: "269f32b2ef5ce1788c9abb15a4915c067b61c0b247aa2fbde7fd382b90da59d8"
---
# Value Traders and Arbitrageurs: Resilience and Cross-Market Liquidity Transfer

## Intuition
Two of Harris's four informed-trader types share an unintended public service: they push prices toward fundamental value and, in doing so, supply liquidity to whoever traded against value. A **value trader** estimates an instrument's fundamental value from first principles, then buys when the market price sits below that estimate and sells when it sits above. Because the value trader stands ready to do this whenever price drifts away from value, the market becomes *resilient*: transient price impacts from uninformed order flow are absorbed and reversed rather than left to persist. Value traders are, in Harris's phrase, the liquidity suppliers of last resort.

An **arbitrageur** does not need an opinion about whether any single instrument is correctly priced. It works on *relative* value across two or more instruments whose prices share common valuation factors, simultaneously buying the relatively cheap leg and selling the relatively expensive leg. The buying lifts the cheap price and the selling depresses the expensive one, dragging the pair back into the consistent relationship implied by their shared factors. Across separated venues or related instruments this is *cross-market liquidity transfer*: the arbitrageur connects a buyer in one market segment to a seller in another, so traders can find depth wherever they trade.

```
   value trader (single instrument)          arbitrageur (paired instruments)
   price > value  --> SELL  ---.              cheap leg  --> BUY  --.
                                |--> price                          |--> prices
   price < value  --> BUY  ---'    reverts    expensive --> SELL --'    converge
   ==> RESILIENCE (depth restored)            ==> LAW OF ONE PRICE (venues linked)
```

**Source:** Harris (2003) ch.10 §10.5.1, ch.16 §16.1.3 pp.227-228, 340; ch.19 §19.5 pp.408-409.

## Definition
**Value trader.** A trader who estimates the entire fundamental value of an instrument by gathering all available information and applying economic models, then trades when market price diverges from that estimate. Because value traders know values well, "they often supply liquidity to large traders... they are the liquidity suppliers of last resort." **Market resiliency** is the property that uninformed traders cannot move prices substantially; value traders create it "by standing ready to trade when prices move away from fundamental values."

**Arbitrageur.** An informed trader who "simultaneously buy[s] and sell[s] similar instruments," seeking instruments inconsistently priced relative to each other given their common fundamental valuation factors, buying the cheaper and selling the more expensive. The **law of one price** holds that identical instruments should have identical prices, and that similar (non-identical) instruments should have prices consistent with the values of their common factors.

**Source:** Harris (2003) §10.5.1 p.228, §10.5.4 pp.233-234, §16.1.3 p.340.

## Mathematical Reasoning
Let an instrument have fundamental value `V` and market price `P`. A value trader's decision rule is a sign condition on the deviation `P - V`: trade in the direction `-sign(P - V)` (buy when `P < V`, sell when `P > V`), so the trader's order flow is *negatively correlated* with the transient price deviation. This negative feedback is exactly what restores depth and yields resilience: each unit of uninformed price pressure is met by offsetting value-trader supply, so the post-shock price reverts toward `V` rather than staying displaced. Value traders quote an **outside spread** — the band of prices at which they will transact — that widens with their costs and risks (adverse selection, winner's curse). Wider risk implies a wider band, hence weaker resilience; the comparative static is monotone in risk.

For two similar instruments with values driven by a common factor vector, write `V_1 = f_1(F)` and `V_2 = f_2(F)`. The arbitrageur does not estimate `F` or either absolute value; it estimates the *relative* relation and trades when observed prices `(P_1, P_2)` depart from the model-implied consistent relation. By buying the relatively cheap leg and selling the relatively expensive one, the arbitrageur is hedged against common-factor moves: if all prices rise because the instruments are jointly undervalued, gains on the long leg offset losses on the short leg, and net profit depends only on the *relative* mispricing. Their buying pushes the cheap price up and their selling pushes the expensive price down, so by construction the trade reduces the gap — an automatic enforcement of the law of one price. The price impact of these trades is itself a transaction cost: smaller impact means more captured profit, so arbitrageurs prefer venues where their footprint is small.

**Source:** Harris (2003) §10.5.4 pp.233-234, §16.1.3 p.340, §16.2 p.341.

## Boundary Notes
The resilience and liquidity-transfer effects hold only when value traders and arbitrageurs are *correct*. A value trader operating on inconsistent assumptions or incomplete information may buy an overvalued instrument and lose when price falls; the protective negative feedback then becomes a destabilizing wrong-way bet. Value traders are deliberately *slow* — the layered review that protects them from bias also delays their response — so resilience is restored over a horizon, not instantaneously; very-hard-to-value instruments get weaker value-trader support and thus thinner resilience.

Arbitrage profits assume the two instruments really are mispriced *relative to each other* and that the relation will reconverge. Harris distinguishes the case where an idiosyncratic factor specific to one leg moved its price — then the instruments were and remain correctly priced relative to each other, the "arbitrage" loses on round-trip transaction costs, and no liquidity transfer service was rendered. Arbitrageurs are exposed to common-factor risk only to the extent the hedge is imperfect (similar, not identical, instruments). Note also that arbitrage trading can *transmit* volatility across linked markets (e.g., the index-arbitrage debate around 1987), which is the flip side of the same cross-market linkage. Contrast with dealers, who supply *immediacy* via inventory across time rather than depth across related markets.

**Source:** Harris (2003) §10.5.1 pp.227-228, §10.5.4 p.234, ch.17 introduction pp.347-348.

## See Also
- [`mt-liquidity-supplier-taxonomy`](./mt-liquidity-supplier-taxonomy.md) -- places value traders (depth/resilience) and arbitrageurs (cross-market transfer) within the full supplier taxonomy alongside dealers and precommitted traders
- [`mt-pairs-trading-cointegration-statarb`](./mt-pairs-trading-cointegration-statarb.md) -- the quantitative statistical-arbitrage / cointegration formalization of the relative-value mechanism sketched here
- [`mt-informed-traders-price-efficiency`](./mt-informed-traders-price-efficiency.md) -- value traders and arbitrageurs as two of the four informed-trader types making prices more informative

## Escalate to Raw When
Harris proves nothing formally here; he characterizes mechanisms in prose. Re-read ch.10 §10.5.4 and Table 10-2 for the full list of common valuation factors and the three-case taxonomy of when relative-value trades win or lose; re-read ch.16 for the value trader's outside-spread determinants (adverse selection, winner's curse) that govern *how much* resilience is supplied; and re-read ch.17 and ch.26 for the quantity characterization of arbitrage (supplying, moving, and producing liquidity) and how arbitrage keeps fragmented markets linked. For any rigorous mean-reversion / cointegration model of the arbitrage relation, escalate to the statarb sibling card rather than this taxonomy card.
