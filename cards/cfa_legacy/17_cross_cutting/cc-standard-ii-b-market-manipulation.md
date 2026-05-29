---
schema_version: "cacg.v0"
id: "cc-standard-ii-b-market-manipulation"
title: "Standard II(B) Market Manipulation"
reading_id: "17_cross_cutting"
summary: "Standard II.B PROHIBITS market manipulation: practices that distort security prices or trading volume with intent to deceive market participants. Covers both information-based manipulation (false rumors, pump-and-dump schemes) and transaction-based manipulation (wash trades, spoofing, marking the close); preserves legitimate trading on perceived market inefficiencies."
tags: ["cfa-ethics", "standard-ii"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3624:5446"
    chunk_hash: "ee6f01176a69715aa25cd5f2d877ddae7a238c9d062722801525638a5fb63edc"
    page_range: [3624, 3625]
    quote: "Market manipulation includes practices that distort security prices or trading volume with the intent to deceive people or entities that rely on information in the market"
    edge_type: "defines"
card_hash: "87a1e49106bd2d70e587bef4e401ffa6e7028e2d3d2616fa83c9617f3f2bf733"
---
# Standard II(B) Market Manipulation

## Intuition

Standard II.B PROHIBITS Members and Candidates from engaging in
practices that distort prices or artificially inflate trading
volume with the intent to mislead market participants. The
prohibition divides into two structurally distinct conduct classes:
information-based manipulation (false or misleading information
disseminated to move prices) and transaction-based manipulation
(trading patterns designed to distort price discovery without a
genuine economic purpose). **Source:** CFA Institute (2022) L1
Vol.6/pp.355-357.

The intent predicate is what distinguishes II.B violations from
legitimate market-making, hedging, or liquidity-provision activity:
a market-maker's quote that adjusts price in response to inventory
risk is NOT manipulation even though the quote affects price; a
trader's wash trade that creates the appearance of volume without
changing economic ownership IS manipulation because the intent is
to mislead other participants. **Source:** CFA Institute (2022) L1
Vol.6/pp.355-360.

```
<!-- primitive: ethics-applicability-gate source: _diagram_primitives.md -->
                 +-------------------------+
                 | Does the fact pattern   |
                 | involve a Member or     |
                 | Candidate?              |
                 +-----------+-------------+
                             |
                  +----------+----------+
                  |                     |
                 yes                    no
                  |                     |
                  v                     v
        +-------------------+   +-----------------+
        | Is the conduct in |   | Code/Standards  |
        | professional      |   | do not apply    |
        | activities?       |   | (out of scope)  |
        +---------+---------+   +-----------------+
                  |
        +---------+---------+
        |                   |
       yes                  no
        |                   |
        v                   v
  +---------------+    +---------------------------+
  | Map to        |    | Personal-conduct fallback:|
  | controlling   |    | I.D Misconduct may still  |
  | Standard      |    | apply if conduct reflects |
  | (I.A..VII.B)  |    | adversely on integrity    |
  +---------------+    +---------------------------+
```

## Definition

Standard II.B PROHIBITS Members and Candidates from engaging in
practices that distort prices or artificially inflate trading
volume with the intent to mislead market participants. The
Standard reaches both information-based and transaction-based
manipulation. **Source:** CFA Institute (2022) L1 Vol.6/pp.355-357.

Information-based manipulation includes disseminating false
information through any channel (press, social media, research
reports, analyst calls), pump-and-dump schemes (issuing a positive
recommendation while preparing to sell from inventory), bear raids
(disseminating false negative information to profit from short
positions), and "talking the book" — promoting an existing position
through misleading information. The key predicate is the
information's falsity and the intent to mislead; truthful information
disseminated lawfully is NOT manipulation even if it moves prices.
**Source:** CFA Institute (2022) L1 Vol.6/pp.355-357.

Transaction-based manipulation includes wash trades (buying and
selling the same security at the same price between affiliated
accounts to create the appearance of volume without economic
substance), painting the tape (round-robin trades among colluding
parties), marking the close (large end-of-day trades intended to
set the day's reference price for portfolio valuation or
derivative settlement), and spoofing/layering (placing large
visible orders the trader never intends to execute, designed to
move price away from the visible order so the trader can execute
the opposite side at favorable price). **Source:** CFA Institute
(2022) L1 Vol.6/pp.357-360.

The legitimate-activity carve-outs (PERMITS) cover market-making
(continuous bid/ask quotes adjusting for inventory risk), hedging
(trades that lay off existing risk), liquidity-provision (offering
to trade in thin markets), block trading (large institutional
orders executed via negotiation with willing counterparties), and
arbitrage (trades exploiting genuine price discrepancies across
markets or related securities). The carve-outs all share the
absence of an intent to mislead. **Source:** CFA Institute (2022)
L1 Vol.6/pp.357-360.

## Mathematical Reasoning

The intent-to-mislead predicate (source PROHIBITS) is what
distinguishes II.B violations from any trade that incidentally
affects price. The Standards' Guidance recognizes that nearly all
trades affect price to some degree (informed-trader theory of
market microstructure ASSERTS that price moves in response to
order flow); a legitimate trade whose price impact comes from the
trade's information content is NOT manipulation. The violation
predicate requires intent to create a misleading price or volume
signal that other participants would interpret as substantive when
it is not. The PCP's evidence for intent typically includes
order-book patterns (visible orders quickly canceled before
execution), trade-to-quote ratios (high quote activity with low
trade conversion), or direct communications showing the
manipulative scheme. **Source:** CFA Institute (2022) L1
Vol.6/pp.355-360.

The information-vs-transaction distinction (source ASSERTS) maps to
two structurally distinct violation patterns: information-based
manipulation operates on the public-information channel (the trader
distorts the information set the market uses to price the security);
transaction-based manipulation operates on the order-flow channel
(the trader distorts the order book the market reads as a signal
of demand or supply). Both share the intent-to-mislead predicate;
they differ in the channel through which the manipulation flows.
The Standards treat both as equally serious because both inject
false signals into the price-discovery process. **Source:** CFA
Institute (2022) L1 Vol.6/pp.355-360.

The legitimate-activity carve-outs (source PERMITS) define a safe
harbor: a market-maker quoting both sides of a security with
inventory-driven price adjustments is PROVIDING liquidity and is
NOT manipulating, even though the maker's quotes set the
price-discovery anchor. The Standards REQUIRE that the quoting
firm distinguish its inventory-adjustment trades from any
self-dealing trades that would constitute manipulation; the
distinction rests on whether the trades have a genuine economic
purpose beyond price/volume distortion. Spoofing fails this test
because the spoofing trader never intends to execute the visible
orders. **Source:** CFA Institute (2022) L1 Vol.6/pp.357-360.

## See Also

- [`cc-standard-ii-a-material-nonpublic`](cc-standard-ii-a-material-nonpublic.md)
  — sibling Integrity-of-Capital-Markets Standard; II.A and II.B
  together comprise the capital-markets-integrity family
- [`cc-standard-iii-a-b-loyalty-prudence-and-fair-dealing`](cc-standard-iii-a-b-loyalty-prudence-and-fair-dealing.md)
  — III.B's fair-dealing obligation overlaps with II.B when a
  Member's manipulative trade harms specific clients

## Escalate to Raw When

Open CFA L1 Vol.6 Reading 58 Standard II.B section directly for the
full Application examples — particularly the high-frequency-trading
microstructure cases and the social-media-manipulation fact patterns
— that the card omits. **Source:** CFA Institute (2022) L1
Vol.6/pp.355-360.

- The reader needs the full social-media manipulation case studies
  (Twitter/X-based rumor schemes; Reddit-coordinated short squeezes);
  these are recent additions that CFA Institute's most current
  Standards Handbook revisions cover but the 2022 PDF treats more
  abstractly. **Source:** CFA Institute (2022) L1 Vol.6/pp.355-360.
- The reader needs the HFT microstructure detail (latency arbitrage,
  quote stuffing, sub-millisecond order cancellations); the card
  states spoofing/layering abstractly but does not work the
  microstructure mechanics. **Source:** CFA Institute (2022) L1
  Vol.6/pp.357-360.
- The reader needs the regulatory cross-reference detail (Rule
  10b-5 in the U.S.; MAR in the EU; MAS in Singapore); the card
  states the Standard's text but does not work the regulatory
  enforcement context. **Source:** CFA Institute (2022) L1
  Vol.6/pp.355-360.
