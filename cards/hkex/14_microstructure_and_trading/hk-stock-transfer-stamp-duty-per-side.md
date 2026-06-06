---
schema_version: "cacg.v0"
id: "hk-stock-transfer-stamp-duty-per-side"
title: "HK Stamp Duty: Ad Valorem Contract-Note Duty on Every Side"
reading_id: "14_microstructure_and_trading"
summary: "Hong Kong stamp duty on a stock transfer is an ad valorem contract-note duty charged on BOTH the bought note and the sold note (i.e. per side); the 0.1% rate that applied 1 Sep 2001 to 31 Jul 2021 is levied on the rounded consideration, a fixed round-trip transaction cost on every HK equity trade."
tags: ["hong-kong", "stamp-duty", "transaction-cost", "trading", "ird", "equities"]
citations:
  - source_id: "ird_stamp_duty_hk_stock_rates"
    chunk_id: "ird_stamp_duty_hk_stock_rates:p001:0000"
    chunk_hash: "d81b02baacc9403654857e74d5000bb8f0b3f3c0debd74d30618d2bd185e69b3"
    page_range: [1, 1]
    quote: "Contract Note for sale or purchase of any Hong Kong stock 0.1% of the amount of the consideration or of its value on every sold note and every bought note"
    edge_type: "supports"
---

## What the rule says

The Inland Revenue Department's schedule of stamp-duty rates on Hong Kong stock shows the duty is a contract-note duty charged on every sold note and every bought note — i.e. on both sides of a transfer — as a percentage of the consideration. The rate was 0.1% per side from 1 September 2001 to 31 July 2021; the schedule also records later changes, so any current figure is as of the snapshot date below.

## Why it matters for an option-income writer

Stamp duty is a guaranteed round-trip cost on HK cash-equity transactions, including the stock legs created when an option is exercised or assigned (each exercise/assignment becomes an individual stock trade for stamp-duty purposes). It is a fixed drag that an income strategy's premium must clear before the trade is net-profitable.

## Grounding sources (as-of)

Every quote below is verbatim from a free Tier-1 source snapshot, bound by `kb verify` to the ingested chunk shown; HK-official figures are stated as of the snapshot date.

- `ird_stamp_duty_hk_stock_rates` — https://www.ird.gov.hk/eng/pdf/sd_stock_rates.pdf (snapshot acquired 2026-06-07; chunk `ird_stamp_duty_hk_stock_rates:p001:0000`)
