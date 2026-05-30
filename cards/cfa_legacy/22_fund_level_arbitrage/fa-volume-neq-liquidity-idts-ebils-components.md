---
schema_version: "cacg.v0"
id: "fa-volume-neq-liquidity-idts-ebils-components"
title: "Volume != Liquidity: Implied Liquidity, IDTS & the EBILS Grid"
reading_id: "22_fund_level_arbitrage"
summary: "An ETF's traded volume (ADV) is backward-looking and understates how much can trade. An ETF inherits the forward-looking implied liquidity of its basket: IDTS is the min-over-names of tradable ETF shares, one of four additive liquidity components mapped onto the EBILS grid."
tags: ["implied-liquidity", "idts", "ebils"]
citations:
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p142:0186"
    chunk_hash: "86261d6b4545c213305ae48d1c45d23c8d4631af35d03d1b8a95c0dac46a753d"
    page_range: [143, 143]
    quote: "The smallest IDTS becomes the constraint on how many shares can potentially be traded and is therefore the ETF implied liquidity."
    edge_type: "defines"
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p131:0174"
    chunk_hash: "b95dabcfc2844293f174bf4397f31cb90a9e2699cc7f70cdf226c3d0c8c90e54"
    page_range: [132, 132]
    quote: "of how many shares of an ETF are available to trade."
    edge_type: "supports"
  - source_id: "fa_hill_2015_cfa_rf_etfs_1e"
    chunk_id: "fa_hill_2015_cfa_rf_etfs_1e:p078:0090"
    chunk_hash: "23e5ca651bae14d3edbdde717c649de9b71d5436fb37f91172b1cd4097770170"
    page_range: [78, 78]
    quote: "Liquidity for an ETF with a lower percentage of assets trading daily can still be good if the underlying securities are liquid or if the ETF is similar to a very active ETF that can be used for hedging by market makers."
    edge_type: "supports"
---
# Volume != Liquidity: Implied Liquidity, IDTS & the EBILS Grid

## Intuition
The single most misread number in the ETF world is on-screen volume. Average daily volume (ADV) records what *did* trade in the past, so a thinly-printed fund looks "illiquid" even when its basket holds the most liquid names on earth. But an ETF is a wrapper: a creation/redemption channel lets a market maker assemble (or unwind) the basket and hand you ETF shares at the basket's implied price. So the fund inherits the liquidity of what it holds — its *implied* liquidity — which is forward-looking. ETF liquidity is properly measured as how many shares are *available* to trade, not how many happened to trade yesterday. Culling the universe by ADV throws away most usable funds; a tiny-volume fund whose basket can absorb a billion dollars is, for a block trader, deeply liquid.

```
        backward-looking            forward-looking
        +-------------+             +------------------------+
ADV --> | what traded |   vs.       | IDTS = what CAN trade  | <-- basket
        |  yesterday  |             |  via creation/redempt. |
        +-------------+             +------------------------+
                 \                          /
                  \                        /
   total liquidity = ADV + IDTS(basket) + derivatives + correlated vehicles
```

**Source:** Abner (2016) *The ETF Handbook* 2e §5 pp.131-135.

## Definition
ETF **trading volume / ADV** = the average over a trailing window of shares that have actually printed in the ETF; a historical statistic. ETF **liquidity** = how many shares of the ETF are *available* to trade (forward-looking). The book frames it directly: "ETF liquidity is a measure of how many shares of an ETF are available to trade."

**Total ETF liquidity** is additively composed of four components: (1) the liquidity of the underlying basket (quantified as ETF implied liquidity), (2) the ETF's own ADV, (3) derivatives written on the ETF (futures, options), and (4) correlated-but-different trading vehicles used to offset positions.

**Implied Daily Tradable Shares (IDTS)** = the number of ETF shares creatable from each constituent's tradable volume over one day with no market impact; the *smallest* per-name IDTS binds: "The smallest IDTS becomes the constraint on how many shares can [potentially be traded] and is therefore the ETF implied liquidity." The **EBILS** (ETF Basket Implied Liquidity Scale) maps each fund onto a letter rating A-E by implied daily tradable *dollars* (IDT$) crossed with a 1-5 level by implied daily tradable *shares*, forming a grid box per fund.

**Source:** Abner (2016) *The ETF Handbook* 2e §5 pp.132-134, 143, 148-151.

## Mathematical Reasoning
Let the basket have constituents i, each with 30-day average daily volume ADV_i and weight w_i = (shares of i per creation unit) / (creation-unit size). For a variable cap VP (default VP = 0.25, i.e. take at most 25% of a name's ADV):

    IDTS_i = (ADV_i * VP) / (constituent shares of i per CU) * (creation-unit size)

The fund-level implied liquidity is the binding constraint across names:

    IDTS_fund = min_i IDTS_i

Key comparative statics:
- IDTS_fund is a min, so it is set by the *most constrained* name, which need not be the lowest-ADV name — a low-volume stock with a tiny weight w_i can leave IDTS_i large, while a moderate-volume stock with a heavy weight can bind. The constraint depends on the ratio ADV_i / (shares per CU), not ADV_i alone.
- Total liquidity dominates ADV: L_total = ADV_ETF + IDTS + L_deriv + L_corr >= ADV_ETF, with the inequality typically strict and often large (basket can swamp on-screen volume by orders of magnitude).
- Tightening VP downward (more conservative) shrinks every IDTS_i proportionally, hence shrinks the min; raising the minimum-weight inclusion threshold (dropping long-tail micro-weights) relaxes the binding constraint and raises IDTS_fund.
- IDT$ = IDTS * price, so two funds with identical basket dollar-liquidity but different share prices show different IDTS yet similar IDT$ — which is why EBILS uses *both* a dollar letter (A-E) and a share level (1-5).

A supporting independent statement: liquidity can be good even when on-screen turnover is low when the underlying securities are liquid — "Liquidity for an ETF with a lower percentage of assets trading daily can still be good if the [underlying securities are liquid]."

**Source:** Abner (2016) *The ETF Handbook* 2e §5 pp.143-144, 148-151; CFA Institute Research Foundation, *A Comprehensive Guide to Exchange-Traded Funds* (2015) p.78.

## See Also
- [`fa-etf-spread-below-basket-adverse-selection`](./fa-etf-spread-below-basket-adverse-selection.md) — the same hedge-supply logic (futures + correlated vehicles) that lets the on-screen ETF spread sit *inside* the raw basket spread; that card extends the four-component liquidity picture into bid/ask formation.
- [`fa-etf-creation-redemption-mechanism`](./fa-etf-creation-redemption-mechanism.md) — the channel that converts basket liquidity into ETF-share liquidity, the engine behind IDTS.
- [`fa-tracking-error-attribution-and-tco`](./fa-tracking-error-attribution-and-tco.md) — implied-liquidity / TCA framing connects to the total-cost-of-ownership view of trading an ETF.
- [`fa-market-liquidity-dimensions-and-no-arbitrage`](./fa-market-liquidity-dimensions-and-no-arbitrage.md) — the general depth/tightness/resiliency dimensions that IDTS operationalizes for one wrapper.

Legacy cross-refs (other tree, prose only): the portfolio-management note on tracking error and active risk (pm-tracking-error-and-active-risk) connects because choosing a fund by EBILS rather than ADV widens the implementable universe without raising the implementation shortfall that feeds active risk.

## Escalate to Raw When
Go to the raw Abner chapter when you need the worked IDTS computation on a real basket (the book carries a full per-stock example showing how a micro-weight name with a small share count and a given 30-day ADV becomes the binding constraint, and how raising the minimum-weight filter dramatically lifts the fund's implied-liquidity figure), the exact EBILS dollar bands (A: >\$1bn down to E: <\$1mn) and share-level cutoffs (1: >100mn shares down to 5: <250k shares), the EBILS grid exhibit showing which letter-level boxes are populated vs. unlikely (e.g. an A4 box), or the empirical claim about how many funds are usable at C-or-higher vs. how few survive a 100k-shares/day ADV screen. Those concrete figures, tables, and the plug-in arithmetic are deliberately excluded here per the no-worked-arithmetic rule.

**Source:** Abner (2016) *The ETF Handbook* 2e §5 pp.143-152.
