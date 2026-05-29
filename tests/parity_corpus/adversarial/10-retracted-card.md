---
schema_version: "cacg.v0"
id: "retracted-fixture-card"
title: "Retracted Fixture Card"
reading_id: "reading_01"
summary: "Phase 3 test-fixture summary populated for AC-S1 bounded-summary enforcement; this card carries a non-empty body for retrieval surface tests."
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
    page_range: [1, 2]
    quote: "Content-addressable identity is the verification primitive at the core of the framework."
    edge_type: "supports"
---
Adversarial fixture: the card itself is structurally valid (lints and verifies
clean on a fresh corpus). The retraction case fires when this card's `id`
("retracted-fixture-card") is in `cards_manifest.retracted_cards`. The
test in `tests/test_retraction.py` builds the corpus, places the id on the
retracted set, then asserts CACG-RETR-001 emission across error and
`--allow-retracted` warning severities.
