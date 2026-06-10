---
schema_version: "cacg.v0"
id: "hk-stock-option-american-style-assignment"
title: "HK Stock Options Are American-Style: Early Assignment Risk"
reading_id: "07_derivatives_and_volatility"
summary: "Hong Kong single-stock options are American-style: the holder may exercise on any business day up to expiry, so a writer of covered calls or cash-secured puts faces assignment at any time, not only at expiration. Physical delivery of the underlying shares follows on exercise."
tags: ["hong-kong", "stock-options", "american-style", "early-exercise", "assignment", "hkex"]
citations:
  - source_id: "hkex_stock_options_info_sheet"
    chunk_id: "hkex_stock_options_info_sheet:p002:0002"
    chunk_hash: "98443f597118122d13b89eb3b783d3c52383fa6bed42bd3389168aa5bec04f81"
    page_range: [2, 2]
    quote: "Options can be exercised at any time up to 6:45 pm on any business day up to and including the last trading day"
    edge_type: "defines"
card_hash: "7b6422705778dd210203df70b916862a69d8456949f2bcca9f998302e6c70400"
---

## What the rule says

HKEX single-stock options are American-style exercise instruments. The official HKEX Stock Options product specification states the exercise window explicitly: an option can be exercised on any business day up to and including the last trading day. Because exercise is not restricted to expiry, the holder of a long option can assign the writer at any point in the contract's life.

## Why it matters for an option-income writer

For a covered-call or cash-secured-put writer, American-style exercise means assignment is a standing risk on every business day, not a single expiry-day event. Assignment converts the option into a physically-settled stock trade (delivery of the underlying shares), so the writer must be able to deliver (short call) or pay for (short put) the shares whenever assigned.

## Grounding sources (as-of)

Every quote below is verbatim from a free Tier-1 source snapshot, bound by `kb verify` to the ingested chunk shown; HK-official figures are stated as of the snapshot date.

- `hkex_stock_options_info_sheet` — https://www.hkex.com.hk/-/media/HKEX-Market/Products/Listed-Derivatives/Single-Stock/Stock-Options/Publications/Stock-Options-Information-Sheet/HKEX_Stock_Options_EN.pdf (snapshot acquired 2026-06-07; chunk `hkex_stock_options_info_sheet:p002:0002`)
