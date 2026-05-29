---
schema_version: "cacg.v0"
id: "semantic-parity-card"
title: "Semantic Parity Card"
reading_id: "reading_01"
summary: "Synthetic card for the semantic parity row: quotes a phrase that does NOT appear in the cited chunk so Layer-2 emits CACG-VERIFY-001 and the Layer-3 cache lookup fires. The committed cache pins a single entry keyed on the computed (chunk_hash, claim_window_hash) pair with verdict=fail."
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
    page_range: [1, 2]
    quote: "this exact phrase is intentionally not a substring of the pinned chunk"
    edge_type: "supports"
---

Body text for the semantic parity card. Layer-2 fails because the quote
above is not a substring of the cited chunk; Layer-3 then consults the
committed semantic_cache.json.
