---
schema_version: "cacg.v0"
id: "mt-three-dimensions-liquidity"
title: "The Three Dimensions of Liquidity: Market, Funding, and Monetary"
reading_id: "14_microstructure_and_trading"
summary: "Liquidity has three interrelated meanings — market liquidity (asset tradability), funding liquidity (intermediaries' access to credit), and monetary liquidity — and funding shocks feed back into market liquidity through liquidity spirals."
tags: ["microstructure", "liquidity", "funding-liquidity", "market-liquidity", "liquidity-spiral"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p017:0018"
    chunk_hash: "29d0407ec0ec4b01563ce6790858c1d34e3553bf67db9c27606c32f9b14facff"
    page_range: [17, 17]
    quote: "the ability to trade a security quickly at a price close to its consensus value, that is, in the sense of “market liquidity.”"
    edge_type: "defines"
---
# The Three Dimensions of Liquidity: Market, Funding, and Monetary

## Intuition
The word "liquidity" is used loosely in the financial press to mean three related but distinct things, and conflating them causes real confusion in policy debates. Foucault, Pagano and Roell separate them cleanly. **Market liquidity** is the property of an *asset*: can you sell it fast at a price near its fundamental (consensus) value? **Funding liquidity** is the property of a *trader or bank*: can it raise cash or roll over credit on acceptable terms to meet its obligations? **Monetary liquidity** is a *macroeconomic* quantity: how much money (cash, bank reserves, the M1/M2/M3 aggregates) is sloshing through the system?

The reason microstructure cares about all three — rather than just the first — is that they feed back on one another. A dealer who cannot borrow (funding shock) must shrink inventory and widen quotes, so the *asset* becomes harder to trade (market-liquidity shock). The channel runs the other way too: when an asset is expected to be liquid, brokers charge lower margins on it, so holding it is a cheaper way to obtain funding.

```
   MONETARY LIQUIDITY            (central bank / money supply)
        |  open-market ops, QE -> more bank reserves
        v
   FUNDING LIQUIDITY  <-------------------------+
   (can dealers/banks borrow?)                  |
        |  cheap credit -> dealers hold          | lower margins on
        |  larger inventory, tighter quotes      | liquid assets ->
        v                                        | cheaper leverage
   MARKET LIQUIDITY  --------------------------+
   (can the ASSET be traded fast near value?)
        |
        +--- crisis: feedback turns vicious -> LIQUIDITY SPIRAL
```

**Source:** Foucault, Pagano & Roell (2013) Introduction §0.4 pp.17-18.

## Definition
Let the *consensus value* of a security be the market's common estimate of its fundamental worth. The book's working definition of liquidity is **market liquidity**: the ability to trade a security quickly at a price close to that consensus value. This is the dimension the rest of the book formalizes (via spreads, depth, and price impact).

The two other dimensions are named, not modeled, in this section:

| Dimension | Object it describes | Operational meaning |
|---|---|---|
| Market liquidity | An asset / security | Trade quickly at a price near consensus value |
| Funding liquidity | A trader / bank / firm | Hold cash or obtain credit on acceptable terms to meet obligations without large losses |
| Monetary liquidity | The macro-economy | Stock of money: monetary base, or aggregates M1 / M2 / M3 |

The ordering by market liquidity gives the canonical ranking: cash (most liquid) > financial securities (bonds, stocks) > real estate (least liquid, requiring time or a large price concession to sell).

**Source:** Foucault, Pagano & Roell (2013) Introduction §0.4.1-§0.4.3 pp.17-19.

## Mathematical Reasoning
This section is conceptual rather than model-based, but the linkages are stated as directional (signed) qualitative comparative statics, which is what a later chapter formalizes:

- **Funding -> market (positive).** The more abundant and cheaper a market maker's funding liquidity, the greater the market liquidity it can supply: with more borrowing capacity the dealer carries larger inventory and posts a narrower bid-ask spread and larger size. A credit crunch (drop in funding liquidity) reverses the sign — dealers *widen* spreads and *reduce* maximum order size.

- **Market -> funding (positive).** Margin (collateral) requirements are decreasing in an asset's expected market liquidity: more liquid, less volatile securities carry lower margins, so a more liquid market lets traders fund leveraged purchases and short sales more cheaply.

- **Monetary -> funding -> market (positive chain).** Monetary expansion raises the supply of funds to banks, increasing funding liquidity, and "with it" market liquidity; a monetary contraction reduces both.

Because the funding<->market relation has *positive feedback in both directions*, the system can have an amplifying (rather than self-correcting) response to a shock. In crisis this is the **liquidity spiral** (Brunnermeier and Pedersen, 2009): a funding shock cuts market liquidity, which raises margins, which tightens funding further, so market liquidity "suddenly dries up for many securities at once." The source asserts this amplification and attributes the formal model to Brunnermeier-Pedersen; it does not re-derive the fixed-point conditions here.

**Source:** Foucault, Pagano & Roell (2013) Introduction §0.4.2-§0.4.3 pp.18-19.

## Boundary Notes
- The book *adopts* "market liquidity" as its operative definition of the unqualified word "liquidity"; the funding and monetary dimensions are introduced to forestall confusion, not as the book's primary object of study. Later spread/depth/impact machinery measures market liquidity only.
- The three feedback channels are stated as qualitative directional links, not closed-form results in this introductory section. The signs are "neither mechanical nor stable": intermediaries can generate different funding liquidity at the same money supply (e.g., responding to a monetary-base expansion by hoarding reserves rather than lending), so the monetary->funding link can be weak or broken.
- The three notions are affected by *different policy levers*: market liquidity by securities-market regulation; funding liquidity by banking regulation and the central bank as lender of last resort. Treating them as one variable in a policy debate is the error the section warns against.
- Contrast with the depth/immediacy/width decomposition (Harris): that taxonomy refines *market* liquidity into its measurable sub-dimensions; this card sits one level up, distinguishing market liquidity from the funding and monetary senses entirely.

**Source:** Foucault, Pagano & Roell (2013) Introduction §0.4 pp.17-19.

## See Also
- [`mt-liquidity-depth-immediacy-width`](./mt-liquidity-depth-immediacy-width.md) -- refines the *market* dimension into depth, immediacy, and width.
- [`mt-funding-liquidity-fire-sales`](./mt-funding-liquidity-fire-sales.md) -- formalizes the funding->market feedback into fire sales and limits to arbitrage.
- [`mt-liquidity-premium-asset-pricing`](./mt-liquidity-premium-asset-pricing.md) -- prices the consequences of (il)liquidity into expected returns.
- [`fa-market-liquidity-dimensions-and-no-arbitrage`](../22_fund_level_arbitrage/fa-market-liquidity-dimensions-and-no-arbitrage.md) — cross-set: market-liquidity dimensions and measures (reading-14 owns the liquidity trichotomy and the spread/depth/resiliency measures; reading-22 puts them in a no-arbitrage framing).
## Escalate to Raw When
You need the *formal* funding-market feedback model: this card only asserts the directional links and the liquidity-spiral label. The closed-form margin/inventory mechanism and the spiral's fixed-point conditions are developed in Chapter 9 (cited at p.598 / p.612 of the intro pointing to Brunnermeier-Pedersen 2009) — re-read FPR §0.4.2 pp.18 first, then Chapter 9 for the model.
