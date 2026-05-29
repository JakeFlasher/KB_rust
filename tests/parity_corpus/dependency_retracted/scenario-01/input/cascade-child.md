---
schema_version: "cacg.v0"
id: "cascade-child"
title: "Cascade Child"
reading_id: "reading_01"
summary: "Child card that depends_on the parent. When the parent is retracted, this card lands in cards_manifest.dependency_retracted_cards via the cascade."
card_edges:
  - target: "cascade-parent-retracted"
    edge_type: "depends_on"
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
    page_range: [1, 2]
    quote: "Content-addressable identity is the verification primitive a"
    edge_type: "supports"
---

Body text for cascade-child.
