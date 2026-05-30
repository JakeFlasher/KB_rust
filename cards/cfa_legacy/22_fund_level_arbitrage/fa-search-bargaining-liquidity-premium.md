---
schema_version: "cacg.v0"
id: "fa-search-bargaining-liquidity-premium"
title: "Search & Bargaining in OTC Markets: Endogenous Liquidity Premia"
reading_id: "22_fund_level_arbitrage"
summary: "Duffie-Garleanu-Pedersen recast OTC liquidity from the dealer's inventory/adverse-selection costs to the customer's outside options: spreads and the liquidity premium emerge endogenously from search frictions and dealer bargaining power, with no information asymmetry required."
tags: ["search-and-bargaining", "duffie-garleanu-pedersen", "dealer-bargaining-power"]
citations:
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p100:0104"
    chunk_hash: "acc47f0052c6194057a4fa2d967ceb261ec2e490f8f3353b20d459c58b9e611b"
    page_range: [101, 101]
    quote: "access to market makers receive tighter bid-ask spreads"
    edge_type: "defines"
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p103:0108"
    chunk_hash: "576c378bffa21d5b5ec1e4e908cd56bb61f3dd7df98d4e09538f2c18625715d1"
    page_range: [104, 104]
    quote: "The liquidity premium is proportional to the bid-ask spread S charged by market makers and increases in the"
    edge_type: "supports"
---
# Search & Bargaining in OTC Markets: Endogenous Liquidity Premia

## Intuition
On an exchange, liquidity is usually explained as the dealer's compensation for a risk he bears: either the cost of carrying inventory, or the cost of being picked off by an informed trader (adverse selection). Both stories make the bid-ask spread an *exogenous* tax bolted onto the trade price, and both make liquidity a property of the *security*. Duffie, Garleanu, and Pedersen (DGP) shifted the lens from the dealer to the customer. In a true over-the-counter market there is no central book: a buyer or seller must first *search* for a counterparty, and once one is found the price is *bargained* bilaterally. The terms you get therefore depend on your outside options — how many other dealers you could have called, how quickly you can find them, and how badly you need to trade now. A multibillion-dollar manager who can shop ten dealers extracts a tighter spread than a small regional bank that can reach only one. Liquidity becomes a property of *who you are*, not just of the asset. Crucially, this generates a real, cross-sectional liquidity premium even in markets like interest-rate swaps and FX where nobody has material private information — so the premium is not adverse selection in disguise; it is the rent that search frictions and dealer market power carve out of the gains from trade.

```
   exchange view (exogenous spread)        DGP search-and-bargaining (endogenous)
   ------------------------------          -------------------------------------
   dealer bears inventory/                 investor must SEARCH (find dealer ~1/phi periods)
   adverse-selection risk                            |
            |                                        v
            v                                BARGAIN bilaterally over price
   spread = compensation for that risk         |            |
   (same for all investors)              outside       dealer
                                         options   x   bargaining power (z)
                                              \         /
                                               v       v
                                       spread + liquidity premium
                                       (varies investor-by-investor)
```
**Source:** van der Merwe (2015) pp.100-101.

## Definition
- **OTC / dealer market:** a quote-driven venue where a market maker (dealer) intermediates trade and price formation occurs through bilateral bargaining rather than a central limit order book.
- **Search friction (phi):** the per-period probability that an investor locates a market maker; the expected wait to execute is therefore 1/phi periods.
- **Dealer bargaining power (z):** an index of the market maker's market power, equivalently the inverse of the investor's access to multiple competing dealers; z = 1 is a monopolistic dealer, lower z is more competitive.
- **Liquidity premium (DGP sense):** the gap between an asset's fundamental value and its traded price that arises *endogenously* from search costs and bargaining, not from inventory cost or asymmetric information.
- **Investor heterogeneity:** investors differ in liquidity state ("high rollers" bear no holding cost; "low rollers" incur holding cost c) and in their access to dealers; spreads therefore have a cross-sectional component for the *same* security.

**Source:** van der Merwe (2015) pp.100-103.

## Mathematical Reasoning
DGP build an asset-pricing variant of Diamond's coconut search model. Agents are risk-neutral and infinitely lived; market makers hold no inventory (they offload in the interdealer market) and short sales are barred. There is a fixed supply q of a consol bond paying \$1 per period; low rollers pay holding cost c, and investors switch liquidity state with probability psi.

Let r be the riskless rate, S the dealer's bid-ask spread, and z the dealer's market-power index. The ask price an investor pays is, schematically,

    a = (1/r) - L,    where L >= 0 is the liquidity premium

and the equilibrium spread satisfies (the book's eq. 4.13)

    S = a - b  proportional to  c / [ (r + 2*psi) + search/bargaining terms in phi, z ].

Comparative statics that fall out of this:
- **psi -> 0** (valuations never change): L -> 0 and a -> 1/r, the frictionless fundamental value. The premium is a pure friction object.
- **L increasing in z:** the premium and spread rise with dealer bargaining power; the book states the spread "increases in the bargaining power of the market maker." A monopolist (z = 1) keeps price bounded away from fundamental even as search becomes efficient.
- **L increasing in c:** higher holding cost lowers the seller's reservation value, so dealers quote a lower bid; the spread rises in c.
- **Effect of phi:** the premium falls as a dealer becomes easier to find (expected search cost c/phi shrinks) — but with a monopolist the spread can *widen* as more trades clear, so the sign of d S / d phi is ambiguous.
- **Cross-section:** because outside options differ, the *same* security carries different effective spreads across investors — inventory and information models cannot produce this.

**Source:** van der Merwe (2015) pp.103-105.

## Boundary Notes
The Duffie-Garleanu-Pedersen search model rests on explicit assumptions stated in the body: risk-neutral, infinitely-lived agents; dealers hold zero inventory and offload in the interdealer market; short sales barred; fixed asset supply; and no information asymmetry. The comparative statics hold only inside these limits. In particular, dS/dphi (spread vs bargaining power / meeting intensity) is sign-ambiguous under a monopolist dealer, so the "spread falls as dealers get easier to find" intuition does NOT hold at z = 1. Treat these as the model-scope boundaries before exporting the result to richer microstructure settings.

**Source:** van der Merwe (2015) pp.100-106.

## See Also
- [`fa-amihud-mendelson-and-priced-liquidity-risk`](./fa-amihud-mendelson-and-priced-liquidity-risk.md) — Amihud-Mendelson price illiquidity as an exogenous spread haircut; DGP *extends* it by making the spread an endogenous bargaining outcome rather than a fixed transaction cost.
- [`fa-liquidity-measurement-and-price-impact`](./fa-liquidity-measurement-and-price-impact.md) — empirical bid-ask/price-impact proxies that the DGP model rationalizes from primitives (phi, z, c).
- [`fa-shleifer-vishny-limits-to-arbitrage`](./fa-shleifer-vishny-limits-to-arbitrage.md) — the same chapter pivots from this model into limited-arbitrage (on-the-run vs off-the-run), where capital and funding frictions, not just search, keep prices off fundamental.
- [`fa-market-liquidity-dimensions-and-no-arbitrage`](./fa-market-liquidity-dimensions-and-no-arbitrage.md) — situates search-cost liquidity among the broader liquidity dimensions.
- Legacy: this connects to the behavioral-finance treatment of limits-of-arbitrage and noise-trader equilibria, where mispricings persist because real-world frictions block costless correction; consult the behavioral-finance limits-of-arbitrage and noise-trader-equilibrium notes for the demand-side mirror of these supply-side search frictions.

## Escalate to Raw When
Go to van der Merwe (2015) Ch.4 (pp.101-106) and the underlying Duffie-Garleanu-Pedersen (2005) "Over-the-Counter Markets," *Econometrica* 73, when you need the full closed-form ask price (eq. 4.12) and spread (eq. 4.13) with all psi, z, phi, r, c terms, the four-investor-type accounting (high/low roller x owner/nonowner) and its steady-state population dynamics, the Appendix derivation of the spread, or the worked numerical example (Figure 4.2) that traces bid, ask, and mid prices as the meeting intensity phi rises from 1 to 10,000 under a near-monopolistic dealer — including the monopoly limiting case S = c/(r + 2*psi) and the competitive limit where the spread collapses and price approaches 1/r.

**Source:** van der Merwe (2015) pp.101-106.
