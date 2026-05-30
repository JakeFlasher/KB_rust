---
schema_version: "cacg.v0"
id: "mt-insider-trading-policy"
title: "Insider Trading: Adverse Selection, Market Quality, and Policy"
reading_id: "14_microstructure_and_trading"
summary: "Insider trading is informed trading on material nonpublic information; it raises adverse-selection costs and widens spreads for uninformed traders, motivating prohibition, yet speeds price discovery — the core regulatory tension."
tags: ["microstructure", "insider-trading", "adverse-selection", "liquidity", "price-discovery", "regulation"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p604:1020"
    chunk_hash: "4f5201e11e5dc66f256f40fb95d11c32293e8ab2564fee36685ab3979d369f33"
    page_range: [604, 605]
    quote: "Insider trading—like all informed trading—hurts traders who supply"
    edge_type: "defines"
---
# Insider Trading: Adverse Selection, Market Quality, and Policy

## Intuition
Insider trading is just the most acute form of *informed trading*: a trader acts on material information about a security's value that the rest of the market does not yet have. Because corporate managers know things about their own firms before the public does, most inside information originates inside the issuer. The microstructure question is not whether insiders are unfair (that is a normative debate) but what their trading *does* to the people who stand on the other side: dealers and limit-order traders who supply liquidity.

Liquidity suppliers cannot tell, trade by trade, whether the counterparty is an uninformed liquidity trader or an informed insider. They only know that *some* fraction of incoming orders carries private information that will move price against them. To survive against that adverse selection, they quote a wider bid/ask spread. The wider spread is paid by *everyone* — including the uninformed traders the insider never directly faced. In a roughly zero-sum trading game, the insider's profit is the uninformed traders' loss, collected through transaction costs.

```
   Insider buys on good private news
            |
            v
   Dealer/limit-order trader sells, then loses as price rises
            |
            v
   Suppliers widen quotes to recoup expected losses
            |
            v
   ALL uninformed traders pay a wider spread  <-- the externality
```

The mirror image of this harm is a benefit: an insider's buy *pushes price toward fundamental value*, so prices become more informative sooner. That single fact — same trades, harmful to liquidity but helpful to price discovery — is the entire regulatory tension.

**Source:** Harris (2003) Trading and Exchanges, ch.29 §29.3 pp.604-605

## Definition
**Inside information** is material information about the value of a security that is not publicly available. **Insider trading** is trading based on such material nonpublic information; in most jurisdictions it is illegal and may also include "tipping" the information to others.

**Adverse selection (in this context):** the systematic loss liquidity suppliers incur from the unobservable subset of order flow that is informed. Insider trading is a canonical source of adverse selection because insiders trade only when they expect a profit, i.e., systematically on the side that proves correct.

Harris frames the policy debate as three arguments *for* restriction — fairness, liquidity, and corporate-control — versus three arguments *for* permitting it — informative prices, enforcement cost, and managerial entrepreneurial incentives.

**Source:** Harris (2003) Trading and Exchanges, ch.29 §29.1, §29.3.1-29.3.2 pp.585, 591-594

## Mathematical Reasoning
The liquidity argument rests on a zero-sum accounting identity, not on a numeric example. Let trading among insiders and uninformed traders be (approximately) zero-sum gross of costs. Then:

- Insider expected gross gain = uninformed traders' expected gross loss.
- Uninformed traders are, by assumption, on the profitable side about as often as the unprofitable side, so their *information-driven* expected gain is ~0.
- Therefore their realized average loss must be attributable to **transaction costs** (the spread).

Symbolically, if uninformed aggregate P&L ≈ −(insider profit) and uninformed directional edge ≈ 0, the residual loss is the spread they pay. Removing insiders removes a class of informed counterparties, lowering the adverse-selection component of the spread:

```
spread  =  order-processing cost
         +  inventory cost
         +  adverse-selection cost   <-- falls when insiders are excluded
```

Comparative statics within the source:
- The harm is *largest* when a single insider holds information the firm will not soon release: the monopolist insider trades slowly and unobtrusively to extract maximum value, imposing the greatest cost on others.
- The harm *shrinks* when many insiders learn simultaneously and race to trade: competition pushes price to fundamentals quickly and dissipates the private rent.
- Net effect on corporate value: greater insider earnings raise valuations, but the illiquidity tax offsets this, so permitting insider trading need not change firm value on net.

No closed-form pricing model is asserted beyond these directional (sign-of-effect) statements; the source links the spread mechanics to the dealer/limit-order adverse-selection treatments of its earlier chapters.

**Source:** Harris (2003) Trading and Exchanges, ch.29 §29.3.1.2 pp.592-593

## Boundary Notes
- **When the liquidity harm holds:** asymmetric, durable private information held by few traders; liquidity suppliers cannot distinguish informed from uninformed flow. This is the adverse-selection regime.
- **When it weakens or breaks:** competitive simultaneous insiders (rent dissipated by racing), or information that would become public almost immediately (small incremental price-discovery value lost from restriction).
- **Price-discovery vs liquidity are genuinely opposed:** restrictions narrow bid/ask spreads but make prices less informative in the short run. Neither side wins on microstructure grounds alone.
- **Largest real cost may be non-microstructural:** Harris argues the corporate-control / managerial-labor-market effects (insiders hoarding information, distorting projects, front-running firm trades) may matter more than spread effects.
- **Contrast with manipulation:** insider trading exploits *true* private information that moves price toward value; manipulation (bluffing) injects *false* signals to move price away from value — opposite effects on price informativeness.

**Source:** Harris (2003) Trading and Exchanges, ch.29 §29.3.2.1 (Rebuttal), §29.4 Summary pp.594, 598

## See Also
- [`mt-glosten-milgrom-adverse-selection`](./mt-glosten-milgrom-adverse-selection.md) -- the formal spread-from-asymmetric-information mechanism this card invokes informally
- [`mt-market-manipulation-bluffing`](./mt-market-manipulation-bluffing.md) -- the false-information mirror image of trading on true private information
- [`mt-market-viability-no-trade-breakdown`](./mt-market-viability-no-trade-breakdown.md) -- the limit where adverse selection is severe enough that liquidity supply collapses

## Escalate to Raw When
The card only sketches the zero-sum accounting intuition for why uninformed losses equal transaction costs; Harris ch.29 develops the full debate structure (fairness, liquidity, corporate-control arguments with rebuttals and rejoinders) and the managerial-labor-market consequences. Re-read pp.591-598 for the complete for/against ledger, and chapters 13-14 for the dealer and limit-order adverse-selection mechanics that generate the spread widening asserted here.
