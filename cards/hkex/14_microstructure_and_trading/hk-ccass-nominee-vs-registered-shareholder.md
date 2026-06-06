---
schema_version: "cacg.v0"
id: "hk-ccass-nominee-vs-registered-shareholder"
title: "HK CCASS: Electronic Shares Sit Under HKSCC Nominees"
reading_id: "14_microstructure_and_trading"
summary: "Shares held electronically in Hong Kong's CCASS are registered in the name of HKSCC Nominees Limited, not the investor, so the beneficial holder is not on the company's register and must act through intermediaries for corporate-event information and voting — the default holding form behind a brokerage account."
tags: ["hong-kong", "ccass", "nominee", "custody", "shareholder-rights", "microstructure"]
citations:
  - source_id: "ifec_physical_share_certificates"
    chunk_id: "ifec_physical_share_certificates:p001:0000"
    chunk_hash: "01af9868e267952c1be804c200a4e878e4088ca457fd992ad912adde8dc0896b"
    page_range: [1, 1]
    quote: "shares held in electronic form are deposited into the CCASS and registered under the name of CCASS nominee (i.e. HKSCC Nominees Limited)"
    edge_type: "defines"
---

## What the source says

The Investor and Financial Education Council explains that shares held in electronic form are deposited into CCASS and registered under the name of the CCASS nominee, HKSCC Nominees Limited. The investor's name is therefore not recorded in the company's register of shareholders; to receive corporate-event information or to vote, the beneficial holder must act through the intermediary, who relays instructions to HKSCC Nominees.

## Why it matters

Almost all retail HK positions held through a broker sit in CCASS under the nominee. This is the custody backdrop for an option strategy on HK single stocks: the shares used to cover calls or received on assignment are held in nominee form, and any dividend or corporate-action entitlement flows through that nominee chain rather than directly to the investor.

## Grounding sources (as-of)

Every quote below is verbatim from a free Tier-1 source snapshot, bound by `kb verify` to the ingested chunk shown; HK-official figures are stated as of the snapshot date.

- `ifec_physical_share_certificates` — https://www.ifec.org.hk/web/en/blog/2024/07/physical-share-certificates.page (snapshot acquired 2026-06-07; chunk `ifec_physical_share_certificates:p001:0000`)
