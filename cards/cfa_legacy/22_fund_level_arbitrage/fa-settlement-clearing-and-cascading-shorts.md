---
schema_version: "cacg.v0"
id: "fa-settlement-clearing-and-cascading-shorts"
title: "Settlement, Clearing & the Cascading-Shorts Daisy Chain"
reading_id: "22_fund_level_arbitrage"
summary: "Why ETFs dominate short-interest reports despite no foul play: NSCC/DTC continuous net settlement clears trades nightly while market makers get an extended window to settle, and share-lending daisy chains let notional short interest exceed shares outstanding without phantom shares."
tags: ["cascading-shorts", "continuous-net-settlement", "short-interest"]
citations:
  - source_id: "fa_hill_2025_cfa_rf_etfs_2e"
    chunk_id: "fa_hill_2025_cfa_rf_etfs_2e:p026:0033"
    chunk_hash: "c6f734a575d0801c95aff9a4c4e037fba25a579859ccef90a78d1d9e54830765"
    page_range: [26, 26]
    quote: "After each trade is cleared, the DTC tallies up the total of all trades in a process of continuous net settlement."
    edge_type: "defines"
  - source_id: "fa_hill_2025_cfa_rf_etfs_2e"
    chunk_id: "fa_hill_2025_cfa_rf_etfs_2e:p026:0034"
    chunk_hash: "da40d35a393dfff93002323a7a3a7195359aab8c5fdb0f78828df682eba475a2"
    page_range: [27, 27]
    quote: "market makers have up to six days to settle their accounts by buying or borrowing the missing securities in question."
    edge_type: "supports"
  - source_id: "fa_hill_2025_cfa_rf_etfs_2e"
    chunk_id: "fa_hill_2025_cfa_rf_etfs_2e:p028:0036"
    chunk_hash: "3722e6f2390bfab39df8f16311853c2f41f7716e33e9124b7dde03bb8905887a"
    page_range: [28, 28]
    quote: "These specific ETF shares are now notionally 200% short. See Exhibit 11 for an illustration of this process."
    edge_type: "supports"
  - source_id: "fa_hill_2015_cfa_rf_etfs_1e"
    chunk_id: "fa_hill_2015_cfa_rf_etfs_1e:p046:0053"
    chunk_hash: "4a853d0148cbf83ec425cf9faa84e51fd869e9f047e7da44eb8a90c59f9b3636"
    page_range: [46, 46]
    quote: "For this reason, market makers are given up to six days to settle their accounts."
    edge_type: "supports"
---
# Settlement, Clearing & the Cascading-Shorts Daisy Chain

## Intuition
ETFs routinely top the SEC's most-shorted lists, and the popular press reads that as fragility. It is not. From an ordinary investor's seat, an ETF clears and settles like any other listed equity: trades are batched to the NSCC at day's end, matched, guaranteed by the NSCC, and netted down through continuous net settlement so that a firm both owed and owing the same security is treated as "whole." The wrinkle is that market makers — the agents who keep ETF prices in line — are deliberately allowed to be short into the settlement window, because they sell shares first and manufacture them later via creation. A market maker prefers to delay creation as long as possible to defer creation fees and execution costs (and, while short, to keep collecting the fund's fees/expenses). Layer share-lending on top and the same physical shares get re-promised down a chain, so reported short interest can mechanically exceed shares outstanding without any "phantom" shares existing.

```
ETF SETTLEMENT vs. CASCADING-SHORTS DAISY CHAIN

  trades --> [NSCC nightly batch: match + guarantee] --> [DTC continuous
              net settlement: owed==owes => "whole"] --> settle (T+1)

  MM exception: sell short now, settle within an EXTENDED window
                (delay creation -> defer fees -> still short the shares)

  Day 1:  II1 --buy--> AP1 --borrow(+collateral)--> MM1     | shares 100% short
  Day 2:  II1 --lend--> II2 --sell short--> Buyer           | shares 200% short
          (only the final unencumbered buyer truly "owns"; recall -> create)
```

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.26-28.

## Definition
- **NSCC / DTC.** The National Security Clearing Corporation (NSCC) is a subsidiary of the Depository Trust Company (DTC); the DTC holds the book of accounts (who owns what) aggregated at the member-firm level, not the individual-investor level.
- **Clearing & guarantee.** End-of-day trades are submitted to the NSCC, which matches and clears most trades in a nightly batch; once both parties agree on the trade, the NSCC becomes the guarantor and the trade is considered cleared, vesting the buyer with beneficial ownership even if the seller fails before settlement.
- **Continuous net settlement.** After clearing, the DTC tallies all trades and nets offsetting obligations so a firm that both owes and is owed equal quantities is "whole"; only net positions are debited/credited. The current cycle is T+1 (implemented May 2024, updating the prior T+2 from 2017).
- **Market-maker settlement exception.** All APs are market makers (not vice versa); market makers may legitimately be short at day's end and have an extended window to settle by buying or borrowing the missing securities — what would be "naked shorting" for anyone else.
- **Cascading shorts / daisy chain.** A scenario where one set of ETF shares is successively borrowed, lent, and short-sold so that notional short interest stacks (100% -> 200% -> ...) while only the buyer at the chain's end holds an unencumbered claim.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.26-29.

## Mathematical Reasoning
Let `S_out` be ETF shares outstanding and `SI` be reported short interest. Continuous net settlement enforces a conservation law per security: a firm's net obligation `N = (owed) - (owes)`, and a firm with `N = 0` settles nothing physically ("whole"). Settlement therefore acts on `sum_i N_i = 0` (the market nets to zero), not on gross flow.

Short interest, by contrast, counts *gross* borrow-and-sell legs. Each relending of the *same* physical share adds another short leg without creating a new share. If a single share is relent and shorted along a chain of length `k`, the notional short count from that share is `k`, while the share contributes only `1` to `S_out`. Aggregating, `SI = sum over shares of (chain length)`, so

  `SI / S_out = average chain length >= 1`,

and `SI > S_out` whenever the average relending depth exceeds 1 — the daisy chain takes it from 100% to 200% short at depth 2, etc. No identity is violated: ownership is conserved (exactly one unencumbered final holder per chain), and every short leg is collateralized, so market risk is offset and the chain can unwind by recall, which "most likely leads to the creation of new shares." Reporting lags (data trailing by days/weeks) inflate the *observed* `SI` further, exaggerating the apparent inconsistency without changing the underlying conservation.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.27-29.

## See Also
- [`fa-etf-creation-redemption-mechanism`](./fa-etf-creation-redemption-mechanism.md) — the daisy chain unwinds because shorts can be closed by *creating* fresh shares with the issuer; settlement delay is exactly delayed creation.
- [`fa-limits-to-arbitrage-when-creation-channel-breaks`](./fa-limits-to-arbitrage-when-creation-channel-breaks.md) — the benign cascade depends on creation/recall working; if that channel jams, the unwind story breaks.
- [`fa-volume-neq-liquidity-idts-ebils-components`](./fa-volume-neq-liquidity-idts-ebils-components.md) — like short interest, raw trade/volume counts overstate the "real" ETF footprint once netting and re-routing are accounted for.

Legacy cross-reference (other tree, prose only): the China CSDC settlement-mechanics card covers the parallel question of how a central depository nets and settles, and how T+0 versus T+1 cycles shape arbitrage timing, in the convertible-bond market — a useful contrast to the US NSCC/DTC continuous-net-settlement model described here.

## Escalate to Raw When
Go to the raw text when you need the concrete worked illustration: the 2e SPY netting example (E*TRADE owes Schwab 500 shares, Schwab owes BofA Merrill Lynch the same 500, so Schwab is "whole" and only E*TRADE and BofA are debited/credited), and the XRT episode where reported shares sold short vastly exceeded AUM and shares outstanding (the Exhibit 10 chart). Also escalate for the step-by-step Exhibit 11 Day-1/Day-2 cascade dialogue (II1/AP1/MM1/II2 collateral exchanges) and the precise market-maker incentive arithmetic for deferring creation fees and collecting fund fees while short. The 1e edition gives the older T+2/T+3 timing baseline for comparison.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.26-29.
