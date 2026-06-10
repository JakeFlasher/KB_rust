---
schema_version: "cacg.v0"
id: "hk-special-dividend-option-contract-adjustment"
title: "HK Option Adjustment: The 2%-of-Close Special-Dividend Threshold"
reading_id: "07_derivatives_and_volatility"
summary: "HKEX adjusts an option's strike and contract size for a special, extraordinary or cash-bonus distribution only when that distribution is 2 per cent or more of the share's closing price on the announcement day; an ordinary cash dividend triggers no adjustment, so the option holder bears the ex-dividend price drop."
tags: ["hong-kong", "stock-options", "contract-adjustment", "special-dividend", "capital-adjustment", "hkex"]
citations:
  - source_id: "hkex_contract_adjustments_chap08"
    chunk_id: "hkex_contract_adjustments_chap08:p001:0001"
    chunk_hash: "d1603450eed816af4fc4ccd77a616540443c89d240d18ae0fb32b532b8f0c4b9"
    page_range: [1, 2]
    quote: "the Exchange will not perform any capital adjustment on option positions unless the value of the payment is 2 per cent or more of the share's closing price on the day of the announcement"
    edge_type: "supports"
card_hash: "427ba142f99f2e4f038f06dc0371b439f08c089d5bd5b67a60267891d9137238"
---

## What the rule says

Under HKEX Operational Trading Procedures Chapter 8 (Special Events), an ordinary cash dividend produces NO capital adjustment to option contracts. For other cash distributions — a special dividend, cash bonus or extraordinary dividend — the Exchange adjusts the option's exercise price and contract size only if the distribution is 2% or more of the share's closing price on the announcement day. Rights issues, bonus issues and spin-offs are always adjusted regardless of size.

## Why it matters for an option-income writer

The threshold determines who absorbs a dividend. Below 2%, the strike is unchanged and the option holder eats the ex-dividend drop in the share price; at or above 2%, the strike (and contract size) are scaled to keep the contract value unchanged across the ex-date. A writer pricing calls and puts around a distribution must know which side of the 2% line the payout falls on.

## Grounding sources (as-of)

Every quote below is verbatim from a free Tier-1 source snapshot, bound by `kb verify` to the ingested chunk shown; HK-official figures are stated as of the snapshot date.

- `hkex_contract_adjustments_chap08` — https://www.hkex.com.hk/-/media/HKEX-Market/Products/Listed-Derivatives/Single-Stock/Stock-Options/Trading-Information/Contract-Adjustments/chap08.pdf (snapshot acquired 2026-06-07; chunk `hkex_contract_adjustments_chap08:p001:0001`)
