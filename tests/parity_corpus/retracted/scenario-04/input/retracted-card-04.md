---
schema_version: "cacg.v0"
id: "retracted-card-04"
title: "Retracted Card 4"
reading_id: "reading_01"
summary: "Retracted-card scenario fixture. Input is a valid card; the expected behavior is that running `kb retract <card>` appends a tombstone history event and moves the card_id into cards_manifest.retracted_cards. Subsequent `kb verify` of this card emits CACG-RETR-001."
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
    page_range: [1, 2]
    quote: "Content-addressable identity is the verification primitive a"
    edge_type: "supports"
---

Body text for Retracted Card 4.
