---
schema_version: "cacg.v0"
id: "hk-short-call-assignment-settlement-timing"
title: "HK Exercised-Option Settlement: Exercise Day = T, Delivery T+2"
reading_id: "07_derivatives_and_volatility"
summary: "When a Hong Kong stock option is exercised the exercise day is trade day T and stock settles T+2; because assignment results are known only after T's cash-market close, an assigned short-call writer cannot simply buy the shares on T+1 to deliver on T+2 and may need stock borrowing to settle on time."
tags: ["hong-kong", "stock-options", "settlement", "assignment", "seoch", "stock-borrowing"]
citations:
  - source_id: "seoch_operational_procedures_chap08"
    chunk_id: "seoch_operational_procedures_chap08:p001:0000"
    chunk_hash: "0196afc2e87d1d15a39a6f2d18f4ac23d9afb5eab49fd122b2af0f9a30e62e8e"
    page_range: [1, 1]
    quote: "it may not be possible for a SEOCH Participant to settle its delivery obligations on T+2 by acquiring the underlying securities in the underlying cash market on T+1"
    edge_type: "supports"
card_hash: "3931b32061f8ffc608c73d0ad507e77522c2deb6d7f6c4e8213051abe9ece75f"
---

## What the rule says

SEOCH Operational Clearing Procedures Chapter 8 sets the settlement timeline for exercised options. The exercise day is treated as trade day (T) for the resulting stock transaction, which is then due for settlement on a T+2 basis. Assignment results, however, are only available after the day-end processing on T — after the underlying cash market has closed.

## Why it matters for an option-income writer

An assigned short-call writer who does not already hold the shares is in a timing bind: learning of assignment only after T's close, they cannot acquire the shares in the cash market on T+1 with normal T+2 settlement and still deliver on the option's T+2. SEOCH itself notes participants may need stock borrowing or other arrangements to deliver on time — a concrete reason covered (share-backed) call writing is operationally safer than naked short calls in Hong Kong.

## Grounding sources (as-of)

Every quote below is verbatim from a free Tier-1 source snapshot, bound by `kb verify` to the ingested chunk shown; HK-official figures are stated as of the snapshot date.

- `seoch_operational_procedures_chap08` — https://www.hkex.com.hk/-/media/HKEX-Market/Services/Rules-and-Forms-and-Fees/Rules/SEOCH/Operational-Procedures/CHAP08.pdf (snapshot acquired 2026-06-07; chunk `seoch_operational_procedures_chap08:p001:0000`)
