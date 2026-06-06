---
schema_version: "cacg.v0"
id: "hk-covered-call-shares-as-margin-collateral"
title: "HK Covered Call: Long Shares Replace Cash Margin, No Margin Call"
reading_id: "07_derivatives_and_volatility"
summary: "A Hong Kong covered-call writer who already owns the underlying shares may pledge those shares instead of cash as margin for the short call, with no margin call, provided the long stock's value is at least the option's notional — the mechanism behind covered-call income on HK single-stock options."
tags: ["hong-kong", "stock-options", "covered-call", "margin", "collateral", "hkex"]
citations:
  - source_id: "hkex_stock_options_corner_guide"
    chunk_id: "hkex_stock_options_corner_guide:p014:0014"
    chunk_hash: "a88f45762c2550ed35579540809ed68a8b72982d3667229e3b81de5ecbdf88fd"
    page_range: [14, 15]
    quote: "Investors owning the underlying stock of the derived stock option may use shares of the underlying stock to replace cash for the margin of writing the call"
    edge_type: "supports"
---

## What the rule says

The HKEX Introductory Guide to the Stock Options Corner describes the covered-call margin mechanism: an investor who owns the underlying stock of a stock option may use those shares, instead of cash, as the margin for writing the call, as long as the value of the long stock position is at least the notional value represented by the short option. In that covered case there is no margin call to short the call.

## Why it matters for an option-income writer

This is the structural foundation of a covered-call income programme: the shares the investor already holds both cover potential delivery on assignment and satisfy the margin requirement, so writing the call generates premium without tying up additional cash and without exposure to margin calls on the short option.

## Grounding sources (as-of)

Every quote below is verbatim from a free Tier-1 source snapshot, bound by `kb verify` to the ingested chunk shown; HK-official figures are stated as of the snapshot date.

- `hkex_stock_options_corner_guide` — https://www.hkex.com.hk/-/media/HKEX-Market/Products/Listed-Derivatives/Single-Stock/Stock-Options/Publications/Introductory-Guide-to-the-Stock-Options-Corner/SO_Corner_Guide.pdf (snapshot acquired 2026-06-07; chunk `hkex_stock_options_corner_guide:p014:0014`)
