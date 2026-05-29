---
schema_version: "cacg.v0"
id: "auth-unknown-reading-card"
title: "Auth Unknown Reading"
reading_id: "reading_99"
summary: "Phase 3 test-fixture summary populated for AC-S1 bounded-summary enforcement; this card carries a non-empty body for retrieval surface tests."
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
    page_range: [1, 2]
    quote: "Content-addressable identity is the verification primitive at the core of the framework."
    edge_type: "supports"
---
Adversarial: reading_id is not present in source_matrix. Expected CACG-AUTH-001
(only fires when --source-matrix is supplied; without the flag the card is
structurally valid and lints clean).
