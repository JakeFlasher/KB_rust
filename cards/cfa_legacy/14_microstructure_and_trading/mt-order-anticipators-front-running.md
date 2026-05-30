---
schema_version: "cacg.v0"
id: "mt-order-anticipators-front-running"
title: "Order Anticipators: Front-Running and Parasitic Trading"
reading_id: "14_microstructure_and_trading"
summary: "Order anticipators (front runners, sentiment-oriented technicians, quote matchers) profit by predicting others' order flow rather than fundamental value, raising large traders' costs and prompting hiding and order-splitting defenses."
tags: ["microstructure", "front-running", "order-anticipation", "quote-matching", "parasitic-traders", "execution-cost"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p257:0423"
    chunk_hash: "c776501def1626ef99efa7c4fed1a2406177f7a5ab3cbaf232bcc3b294926119"
    page_range: [258, 258]
    quote: "Order anticipators are parasitic traders. They profit only when they can"
    edge_type: "defines"
card_hash: "bc098591d523e4a9b11ecf9c5bb320e410f49c5ecd7da7e292b5271e4b5b5853"
---
# Order Anticipators: Front-Running and Parasitic Trading

## Intuition
An order anticipator is a speculator who tries to profit by trading *before* other traders trade, betting on how those traders' orders will move prices — not on what the security is fundamentally worth. Harris groups three species under this label: **front runners** (who learn about trades others have already decided to arrange), **sentiment-oriented technical traders** (who predict what uninformed traders will do), and **squeezers** (who corner traders who must trade). Because none of them carries fundamental-value information, Harris classes order anticipators as **parasitic traders**: they earn nothing from making prices more informative or markets more liquid, and profit only by extracting value from the very traders whose flow they anticipate.

The clearest victim is the large patient trader. A large trader normally splits a big order to price-discriminate — filling first with the cheapest liquidity, then paying up only as needed — so the average price beats filling the whole block at one clearing price. A front runner short-circuits this: it grabs the cheap liquidity first, then sells it back to the large trader at worse prices, forcing the large trader toward a single uniform (worse) price.

```
   Large buyer's intended fill (split, price-discriminating):
   p_1 < p_2 < ... < p_n     (climbs the supply ladder only as needed)
          ^
   Front runner buys the cheap rungs (near p_1) first ...
   ... then resells to the large buyer higher up the ladder
   => large buyer pays a HIGHER average price (pushed toward p_n)
```

**Source:** Harris (2003) *Trading and Exchanges* ch.11 (§11.1) pp.245-248.

## Definition
**Order anticipators** are speculators who try to profit by trading ahead of other traders, profiting either when they correctly anticipate how others will move prices or when they extract option values from standing orders. They are *parasitic* — they profit only by preying on other traders and add neither informativeness nor liquidity.

- **Front runner:** trades ahead of a trader whose order it has learned about (from public sources, from the traded party, or from a broker). Illegal when the information is obtained by violating a confidential brokerage relationship or by eavesdropping; *legal* when an observant trader merely infers the order (e.g., from how a broker behaves).
- **Quote matcher (penny jumper):** a front runner of *passive* (liquidity-supplying) traders who trades in front of, and on the same side as, a large patient limit order, in order to extract that order's option value.
- **Sentiment-oriented technical trader:** front-runs *uninformed* traders by predicting their order flow.

**Source:** Harris (2003) *Trading and Exchanges* ch.11 (§11.1, §11.1.2) pp.245-249.

## Mathematical Reasoning
The quote matcher's payoff is **option-like: bounded loss, unbounded gain**. Suppose a large patient buy limit order rests at price `L`. A quote matcher buys just in front of it at `L + Δ` (one price increment above) and is filled by an incoming market sell. Now the quote matcher holds a long position:

- If price *rises*, the quote matcher gains to the full extent of the rise.
- If price *falls*, the quote matcher unwinds by hitting the resting order at `L`, capping the loss near `Δ`.

So the long stock position combined with the *protective put* implicit in the resting buy limit order (the right to sell at `L`) replicates a **long call**:
```
   long stock  +  long put (from the resting buy order)  =  long call
   payoff:   loss capped at ~Δ      |      gain unbounded as price ^
```
Symmetrically, selling in front of a large sell order = short stock + the call implicit in that order = a synthetic **long put**.

Comparative statics from this structure:
- **Time precedence + a large minimum price increment ⇒ less profitable quote matching.** To trade ahead the matcher must improve price by at least one increment `Δ`; that `Δ` is a transaction cost paid not to the front-run order but to the counterparty that would otherwise have hit the resting order. Larger `Δ` shrinks gains and deepens losses.
- **Speed dependence.** The matcher must react to changing conditions faster than the passive trader it shadows, so successful matchers are floor traders (floor markets) or computerized traders (screen markets).
- **Survival of the option.** Profit requires the front-run order to *still be standing* when the matcher needs to exit. If passive traders cancel, frequently re-price, or get filled by others, the protective option vanishes.

**Source:** Harris (2003) *Trading and Exchanges* ch.11 (§11.1.2) pp.248-250.

## Boundary Notes
- **When front running can *help* a large trader (rare):** only if the front runner is a *better* trader who can consolidate the other side and deliver it more cheaply, does *not* compete with rival front runners (competition magnifies price impact), and does *not* then squeeze. These conditions are rarely simultaneous; a large trader who genuinely benefits should instead become a *sunshine trader* (publicize the order) or hire a block dealer.
- **Defenses raise the victim's own costs.** Liquidity suppliers defend by hiding orders via floor brokers, breaking up orders, switching from limit to market orders, and trapping quote matchers (baiting with a large limit order, then selling to the matcher and canceling). These responses cut displayed size and transparency and increase the defender's transaction costs.
- **Market-efficiency effect is sign-dependent.** Front-running *uninformed* traders makes prices *less* informative; front-running *informed* traders moves prices toward fundamentals sooner, but long-run erodes informed-trader profits, driving them out and reducing informativeness.
- **Liquidity:** because trading is zero-sum and front runners supply no offsetting service, their profits are pure transaction costs — markets are generally *less* liquid with them.
- **Not bluffing:** order anticipators react to *real* anticipated flow; bluffers fabricate misleading flow (manipulation) — a separate parasitic species.

**Source:** Harris (2003) *Trading and Exchanges* ch.11 (§11.1, §11.1.1-§11.1.4) pp.247-251.

## See Also
- [`mt-market-manipulation-bluffing`](./mt-market-manipulation-bluffing.md) -- sibling parasitic species; bluffers fabricate flow rather than anticipate genuine flow.
- [`mt-block-trader-upstairs-depth`](./mt-block-trader-upstairs-depth.md) -- large traders hire block brokers/dealers as a defense against anticipators.
- [`mt-informed-traders-price-efficiency`](./mt-informed-traders-price-efficiency.md) -- contrast: informed traders trade on value and aid price discovery; anticipators do not.
- [`mt-order-imbalance-signal`](./mt-order-imbalance-signal.md) -- the order-flow signal anticipators try to read ahead of others.

## Escalate to Raw When
Harris develops the *legal vs. illegal* line (confidentiality violations, broker order-exposure duties), the worked option-replication argument for quote matching, the squeeze case, sunshine trading, and the full market-efficiency/liquidity analysis with more nuance than this card sketches. Re-read ch.11 §§11.1-11.1.4 (pp.245-252) for the precise conditions under which front running helps vs. harms, and ch.8 (pp.207-210) for where order anticipators sit in the full parasitic-trader taxonomy; this card asserts the option-payoff equivalence and comparative statics but does not reproduce Harris's numerical front-running ladder example.
