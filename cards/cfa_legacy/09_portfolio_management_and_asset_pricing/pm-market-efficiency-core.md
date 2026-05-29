---
schema_version: "cacg.v0"
id: "pm-market-efficiency-core"
title: "Market Efficiency — Three Forms"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Market Efficiency — Three Forms: defining the three CFA-framing forms of market efficiency (weak, semi-strong, strong) — what each form claims about the price-information relationship, what tests are used to falsify each, and what each implies for the active-management decision"
tags: ["market-efficiency", "fama", "equity-investments"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2181:3200"
    chunk_hash: "61b3817021443ba8577da0e245825189d7e8a8d6d7eb8359a6bfb7a4f880de01"
    page_range: [2181, 2182]
    quote: "In his framework, Fama defines three forms of efficiency: weak, semi-strong, and strong. Each form is defined with respect to the available information that is reflected in prices."
    edge_type: "defines"
card_hash: "b6be3e277f6a36dc8eec04aa8b7c56acc57bc98a24c3e43139d4849c8e890743"
---
# Market Efficiency — Three Forms

## Intuition

A market is informationally efficient if prices reflect all
available information so quickly and completely that no consistent
profit can be made from acting on that information. The CFA L1
framing distinguishes three forms by the information set assumed
to be incorporated. The weak form claims past prices and trading
data are reflected; the semi-strong form claims all public
information is reflected; the strong form claims even private
information is reflected. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

```
        information sets
        ----------------
        +---------------------------------+
        |  STRONG-FORM information         |
        |  (public + private; insider)     |
        |   +-----------------------------+|
        |   |  SEMI-STRONG information    ||
        |   |  (all publicly available)   ||
        |   |   +------------------------+||
        |   |   |  WEAK-FORM information  |||
        |   |   |  (past prices,         |||
        |   |   |   volumes, returns)    |||
        |   |   +------------------------+||
        |   +-----------------------------+|
        +---------------------------------+

        Efficiency in form X means prices already reflect the
        information in set X — no excess return from analysis
        based on that set.
```

The three forms are nested: strong implies semi-strong implies
weak. If a market is efficient in the strong form, all weaker
information sets are also reflected. The empirical question is
typically about which form holds for which market segment, not
whether efficiency holds globally. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.420-441.

## Definition

The weak form of market efficiency claims that current prices
reflect all information contained in past trading data — past
prices, volumes, returns. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

```
weak form holds  iff  E[ r(time+1) | I_past_prices ] = E[ r(time+1) ]
```

Under the weak form, technical analysis (chart patterns, momentum
signals, mean-reversion plays based on price history) cannot earn
expected returns above the unconditional benchmark. Information
already in past prices is already in current prices. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.420-441.

The semi-strong form extends to all publicly available information
— earnings releases, regulatory filings, news, analyst reports.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

```
semi-strong form holds  iff  E[ r(time+1) | I_public ] = E[ r(time+1) ]
```

Under the semi-strong form, fundamental analysis based on public
information cannot earn expected excess returns. Reading the same
10-K everyone else reads gives no edge once the market has digested
it. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

The strong form extends further to private information held by
insiders or specialists. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

```
I_all = I_public union I_private
strong form holds  iff  E[ r(time+1) | I_all ] = E[ r(time+1) ]
```

Under the strong form, even insider information confers no
expected-return advantage. The strong form is widely viewed as too
strong empirically; insider trading regulation exists precisely
because some private information demonstrably confers an advantage.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

## Mathematical Reasoning

The empirical evidence cited in the L1 framing is a graded
distinction. Tests of the weak form examine return autocorrelation,
runs tests, and momentum / reversal pattern profitability. The
broad finding is that weak-form efficiency is approximately
correct for liquid major markets — past-price-based strategies
deliver returns close to but not perfectly equal to the
unconditional benchmark, with documented short-horizon momentum
and long-horizon reversal patterns sitting at the boundary. The
weak form is approximately accepted with anomaly caveats.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

Tests of the semi-strong form use event studies — examining how
prices respond to public news announcements (earnings surprises,
M&A announcements, regulatory rulings). The broad finding is that
prices adjust rapidly (often within minutes) to public news, with
post-announcement drift documented in some categories (post-
earnings-announcement drift, in particular). The semi-strong form
is approximately accepted with documented anomalies. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.420-441.

Tests of the strong form examine the performance of corporate
insiders and exchange specialists. The broad finding is that
insiders' trades earn excess returns that are economically
significant before transaction costs, falsifying the strong form.
The strong form is rejected by the data. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

The implication for the active-vs-passive decision is direct.
Under semi-strong-or-stronger efficiency, the expected alpha for a
representative manager is zero, the active hurdle (cost difference
plus tracking-risk penalty) cannot be cleared in expectation, and
passive dominates. Under weak-form efficiency only, alpha may be
available from fundamental analysis but not from technical
analysis. The active manager's job is to identify segments where
even semi-strong efficiency does not hold — typically less-followed
segments with high information acquisition costs. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.420-441.

The relationship to CAPM is that the CAPM equilibrium derivation
assumes implicit semi-strong efficiency: investors price assets
based on their full information set (the public set, by the
homogeneous-expectations assumption), and equilibrium pricing
follows. Documented empirical anomalies (size, value, momentum)
are simultaneously evidence against semi-strong efficiency and
evidence against the single-factor CAPM specification — the two
findings cannot be cleanly separated empirically. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.540-565.

## See Also

- [`pm-portfolio-perspective.md`](pm-portfolio-perspective.md) — the broad portfolio framing in which efficiency assumptions enter
- [`pm-active-vs-passive-decision.md`](pm-active-vs-passive-decision.md) — the choice rule that takes efficiency form as input
- [`pm-capm-and-sml.md`](pm-capm-and-sml.md) — CAPM equilibrium pricing under which semi-strong efficiency is approximately assumed

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R48 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

- Specific anomaly catalog (size, value, momentum, low-volatility,
  quality, accruals) with citation to the academic literature —
  Vol.6 R48 lists; deeper treatment of each anomaly belongs in the
  AC-42 extension card and in future-05. **Source:** CFA L1
  Curriculum (2022) Vol.6/pp.420-441.
- Behavioral-finance explanations for persistent anomalies (limits
  to arbitrage, investor sentiment, herding) — Vol.6 R48 cites
  these and the deeper development belongs in future-10. **Source:**
  CFA L1 Curriculum (2022) Vol.6/pp.420-441.
- Pedersen efficiently-inefficient framing where prices are
  approximately efficient and alpha exists as compensation for
  active risk-bearing — this is the extension counterpart to the
  L1 framing here. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.420-441.
