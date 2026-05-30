---
schema_version: "cacg.v0"
id: "fa-etf-creation-redemption-mechanism"
title: "The ETF Creation-Redemption Mechanism"
reading_id: "22_fund_level_arbitrage"
summary: "Only authorized participants (APs) swap an in-kind basket for creation units one-for-one at fair value with the issuer, making the ETF share float elastic. Because each exchange is value-neutral, creating or destroying shares leaves per-share NAV unchanged."
tags: ["creation-redemption", "creation-unit", "authorized-participant"]
citations:
  - source_id: "fa_hill_2025_cfa_rf_etfs_2e"
    chunk_id: "fa_hill_2025_cfa_rf_etfs_2e:p021:0025"
    chunk_hash: "bdb2956c8cec86f637f4c498810bfb0abeab829cbaf4531b9b043231fc66335e"
    page_range: [22, 22]
    quote: "The exchange is one for one: One carefully articulated basket of underlying securities is exchanged for one block of ETF shares."
    edge_type: "defines"
  - source_id: "fa_hill_2025_cfa_rf_etfs_2e"
    chunk_id: "fa_hill_2025_cfa_rf_etfs_2e:p021:0024"
    chunk_hash: "aeadd6c53c51e062099765c83f51514070d4e54b1037b630a254db7258817cdc"
    page_range: [21, 21]
    quote: "ETF shares can be created or redeemed only by a special group of institutional investors—APs— who are designated when the ETF is issued."
    edge_type: "supports"
card_hash: "778194b100df0e8bd603c56380431dab38c9214f42fe4a33015de4ff7206f741"
---
# The ETF Creation-Redemption Mechanism

## Intuition
Unlike a stock (fixed float from an IPO) or a mutual fund (shares bought directly from the firm), an ETF gets its shares from a continuous primary-market plumbing layer. A privileged class of broker/dealers — authorized participants (APs) — are the only agents allowed to mint or retire shares, and they do so by handing the issuer a basket of the underlying securities in exchange for a large block of ETF shares (or the reverse). The genius is that the swap is *value-neutral*: the basket the AP delivers is worth the same as the block of shares it receives, so the act of expanding or shrinking the share float transfers no wealth into or out of the existing holders. This elastic float is what lets the secondary-market price be tethered to underlying value rather than to ETF supply and demand alone.

```
        PRIMARY MARKET (AP <-> Issuer)            SECONDARY MARKET
   in-kind basket  -->  +---------+               (exchange)
   (value = block)      |  ETF    |  ETF shares    Buyer <--> Seller
   <-- creation unit    | Issuer  |  ---------->    via brokers
   (value = basket)     +---------+
        ^ value-neutral 1-for-1 swap          ^ price set by supply/demand,
          float expands / contracts             pinned to NAV by AP arbitrage
```

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.21-22.

## Definition
- **Authorized Participant (AP):** a designated large broker/dealer (often a market maker) that alone may transact directly with the issuer through the primary-market creation/redemption process. ETF shares can be created or redeemed *only* by this group.
- **Creation unit:** the large block in which AP-issuer transactions occur, traditionally 50,000 ETF shares.
- **Creation / redemption basket:** the publicly disclosed list of underlying securities (with specified share counts) the issuer wants to acquire (creation) or divest (redemption) that day.
- **In-kind exchange:** the AP delivers the basket of securities and receives the creation unit of ETF shares (creation), or delivers shares and receives the basket (redemption); most ETFs operate in-kind, though some (notably certain bond ETFs) transact in cash.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.21-22.

## Mathematical Reasoning
Let a creation unit be $Q$ ETF shares and the basket be holdings $\{w_i\}$ of underlying securities priced $\{p_i\}$. The exchange is one-for-one in *value*, so the basket value equals the block value:

$$\sum_i w_i\, p_i \;=\; Q \cdot \text{NAV}_{\text{share}}.$$

Now consider the conservation identity. Before creation the fund holds assets $A$ across $N$ shares, so $\text{NAV}_{\text{share}} = A/N$. After a creation the fund receives basket value $B = \sum_i w_i p_i$ and issues $Q = B/\text{NAV}_{\text{share}}$ new shares:

$$\text{NAV}'_{\text{share}} = \frac{A + B}{\,N + Q\,} = \frac{A + B}{\,N + B/(A/N)\,} = \frac{A}{N} = \text{NAV}_{\text{share}}.$$

The per-share NAV is invariant to $B$: creating or destroying shares does **not** dilute or concentrate the remaining holders. The float $N$ is therefore *elastic* — it adjusts to demand while leaving $\text{NAV}_{\text{share}}$ fixed. (Redemption is the same identity with $B<0$.) Symmetric inequalities on the secondary price relative to this fair value are what the AP exploits; that band is treated in the sibling arbitrage card.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.21-22; CFA RF *Guide to ETFs* 1e pp.40-41.

## See Also
- [`fa-iiv-iopv-intraday-fair-value`](./fa-iiv-iopv-intraday-fair-value.md) — the published intraday fair value that the AP compares against the live ETF price to decide whether to create or redeem.
- [`fa-etf-creation-redemption-arbitrage-band`](./fa-etf-creation-redemption-arbitrage-band.md) — how the value-neutral swap, net of AP costs, pins the secondary price into a band around fair value.
- [`fa-in-kind-basket-design-and-fees`](./fa-in-kind-basket-design-and-fees.md) — issuer design choices (creation-unit size, fees, cash-in-lieu) that govern whether the AP steps in.
- [`fa-dual-rail-pricing-nav-vs-market`](./fa-dual-rail-pricing-nav-vs-market.md) — the two coexisting price rails (NAV vs market) the mechanism reconciles.
- Legacy: this is structurally analogous to the China CSDC settlement plumbing covered under the convertible-bond tree's account/settlement mechanics, where the depository governs how the in-kind legs ultimately clear.

## Escalate to Raw When
Go to the raw source when you need the worked secondary-market arbitrage walk-through — the book's Exhibit 9 scenario plugs concrete bid, fair-value, and AP-cost figures to show how the AP profits from a temporarily inflated ETF price by selling shares and buying the basket, and the mirror case for a discount. Also escalate for the full settlement timing (baskets are exchanged after the close via the overnight process even though the AP quotes and trades against the basket all day), the published creation-basket disclosure rules, and the issuer's design levers over creation-unit size and fees.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.22-23.
