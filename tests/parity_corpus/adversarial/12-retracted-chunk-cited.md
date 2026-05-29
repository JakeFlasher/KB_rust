---
schema_version: "cacg.v0"
id: "retracted-chunk-fixture-card"
title: "Retracted Chunk Fixture"
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
Adversarial fixture: the card is structurally valid. The retraction case
fires when `sample:p001:0000` (the cited chunk_id) appears in
`chunks_manifest.retracted_chunk_ids`. The test in
`tests/test_source_chunk_retraction.py` retracts the chunk via
`kb retract-chunk sample:p001:0000`, then verifies the card and asserts
CACG-RETR-003 emission across both layer-1 (normal) and layer-2
(--skip-lint) paths.
