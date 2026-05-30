---
schema_version: "cacg.v0"
id: "mt-trading-industry-participants"
title: "The Trading Industry: Who Trades, Brokers, and the Buy Side"
reading_id: "14_microstructure_and_trading"
summary: "The trading industry splits into a buy side that purchases exchange services (chiefly liquidity) and a sell side of dealers and brokers that supplies them; brokers act as agents and owe an agency duty of best execution."
tags: ["microstructure", "trading-industry", "buy-side", "sell-side", "best-execution", "broker-dealer"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p045:0062"
    chunk_hash: "273457a5eb58bfed4f71630ab8e66aa4254640f456dd1f3657b1ea3e93783eec"
    page_range: [45, 46]
    quote: "The trading industry has a buy side and a sell side. The buy side consists of traders who buy exchange services."
    edge_type: "defines"
---
# The Trading Industry: Who Trades, Brokers, and the Buy Side

## Intuition
Harris organizes the entire trading world along a single axis: who *buys* exchange
services versus who *sells* them. This is the deliberately counterintuitive part —
the "buy side" and "sell side" labels have nothing to do with whether a participant
is currently a buyer or a seller of a *security*. Both sides buy and sell securities
all day. The labels refer to who buys versus sells *exchange services*, the most
important of which is liquidity: the ability to trade when you want to trade. Buy-side
traders (individuals, pension funds, mutual funds, endowments, corporations,
governments) come to markets to solve problems that originate *outside* trading —
moving wealth through time, hedging operating risk, exchanging assets. Sell-side
firms (dealers and brokers) exist only because the buy side will pay for the
services they provide.

A second axis cuts across the first: *for whose account* do you trade? Proprietary
traders trade for their own account and profit by buying low and selling high.
Brokers (also called agency, commission, or order traders) trade as *agents* on
behalf of clients and profit from commissions, not from price moves. Dealers are
proprietary traders who supply liquidity by standing ready to take the other side.
A single firm often does both — a broker-dealer, or "dual trader."

```
                THE TRADING INDUSTRY
                        |
        +---------------+----------------+
     BUY SIDE                         SELL SIDE
  (buys exchange svcs)            (sells exchange svcs)
        |                                |
  Investors / Borrowers           Dealers (principal:
  Hedgers / Asset-exchangers       take other side)
        |                          Brokers (agent: arrange
  use buy-side TRADERS to           trades for clients)
  implement decisions              Broker-dealers (both)
        |                                |
        +----- liquidity flows ----------+
                ($ commissions + spreads)
```

**Source:** Harris (2003) ch.3 §3.1 pp.45-46.

## Definition
*Trading industry structure (Harris):* the industry "has a buy side and a sell
side." The **buy side** consists of traders who *buy* exchange services — investors,
borrowers, hedgers, asset exchangers, and gamblers — using markets to solve problems
that originate outside of trading. The **sell side** consists of **dealers** and
**brokers** who *provide* exchange services to the buy side.

- **Proprietary trader:** trades for its own account.
- **Broker (agency trader):** an agent who arranges trades for clients in exchange
  for commissions; does not take the other side of the client's trade.
- **Dealer:** accommodates client trades by trading *with* them on its own account;
  profits from buying low and selling high.
- **Broker-dealer / dual trader:** a firm that both deals and brokers.
- **Investment sponsors / advisers / beneficiaries:** sponsors (pension funds, mutual
  funds, trusts, endowments, foundations) employ advisers (portfolio managers) who in
  turn employ buy-side traders; beneficiaries ultimately benefit from the funds.

**Source:** Harris (2003) ch.3 §3.1.1–§3.1.2 pp.45-46.

## Mathematical Reasoning
The structure is institutional rather than algebraic, but it rests on an economic
mechanism. The sell side "exists only because the buy side will pay for its services":
sell-side profitability is *derived* from buy-side demand for liquidity. Hence one
cannot characterize when the sell side is profitable without first understanding why
the buy side trades — a sequencing argument Harris makes explicit.

The broker relationship is governed by the **principal-agent problem**: an agent is
supposed to act for the principal but may instead act for itself. Because brokerage
*quality of service* is hard to measure — clients generally cannot tell whether a
broker obtained the best available price, or failed to trade because no counterparty
existed versus because the broker was not aggressive — measures of broker
productivity are "invariably imprecise." This measurement friction is what gives the
agency problem teeth: "you cannot manage what you cannot measure." Clients respond
with carrots and sticks (explicit contracts and order flow as reward; lawsuits and
withdrawal of business as penalty), supplemented by rating agencies, consultants,
and regulators.

Best execution then layers on top: "When brokers take client orders, they assume an
agency responsibility to obtain best execution." The comparative logic is that the
*content* of "best execution" scales with the client's ability to measure it.
Unsophisticated clients read it as an absolute standard (best price for a market
order, fastest fill for a limit order); sophisticated clients read it relative to
what they pay; the most sophisticated read it relative to *auditing cost* — a broker
will not supply execution quality it cannot be credited for, because in competitive
brokerage any such broker is undercut by one that does not spend the resources.

**Source:** Harris (2003) ch.3 §3.1.2 p.46; ch.7 §7.4–§7.4.2 pp.159-161.

## Boundary Notes
- The buy/sell-side terminology is *orthogonal* to being a buyer or seller of an
  instrument; conflating the two is the classic error Harris flags.
- The buy/sell split is not exhaustive of "who helps traders trade": exchanges,
  clearing and settlement agents, depositories, and custodians are **trade
  facilitators**, a separate category from the sell-side dealers/brokers.
- Dual traders (broker-dealers) face an *unavoidable* conflict of interest, sharpest
  when they internalize orders — what is best for the client (low buy price / high
  sell price) is never best for the dealer in the short run. The clean agency duty of
  a pure broker blurs once principal trading is added.
- "Best execution" is explicitly *not well defined* in the source; the card sketches
  the spectrum of meanings but does not adopt a single legal standard.

**Source:** Harris (2003) ch.3 §3.2 p.46; ch.7 §7.4.3 pp.160-161.

## See Also
- [`mt-buy-side-trader-best-execution`](./mt-buy-side-trader-best-execution.md) -- deepens the broker agency duty and best-execution standard introduced here.
- [`mt-execution-systems-quote-vs-order-driven`](./mt-execution-systems-quote-vs-order-driven.md) -- the venue/exchange side that the sell side and buy side trade through.
- [`mt-order-types-market-limit-stop`](./mt-order-types-market-limit-stop.md) -- the order instructions clients hand brokers to represent.

## Escalate to Raw When
You need the full taxonomy tables (Harris Tables 3-1 and 3-2 enumerate buy-side
trader types/instruments and sell-side trader types), the institutional detail on
investment sponsors/advisers/beneficiaries chains, or the rigorous treatment of
best execution (Harris defers a full account to chapter 25) and the dual-trading
conflict (chapter 7 §7.4.3). Re-read Harris (2003) ch.3 pp.45-46 for structure and
ch.7 §7.4 pp.159-161 for the agency/best-execution mechanics.
