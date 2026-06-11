# cacg.v1 design — utterance anchors in the trust kernel

Status: **design, not yet implemented.** The operational needs that motivated
v1 are already served without a schema bump (see "What ships today"); v1 is
the durable shape that moves speaker policy from convention + sidecars into
the hash-pinned kernel itself. Implement when the first deck authors against
the utterances backend (expert #2, uid 9650668145) or when a corpus refresh
must survive mid-stream insertions without re-authoring.

## What ships today (no schema bump)

- `kb ingest --format utterances` (`cacg.utterances.v1` JSONL contract):
  conversational corpora ingest with no pdfium/font/page-geometry in the
  trust chain; chunk text is the pure utterance text.
- `locator_map.json` sidecar, sealed with SHA-256 over its canonical JSON
  and published in the same atomic group as the manifest pair:
  `chunk_id -> [{ordinal, utterance_id, speaker, is_author, authored_at,
  refs}]`. Attribution is machine-checkable (`verify_locator_seal`), but by
  TOOLING (the deck faithfulness linter), not by `kb verify` itself.
- `kb ingest --append`: a corpus grows without invalidating any prior chunk —
  every previously-published chunk must re-derive byte-identical or the
  append hard-fails (no silent re-anchor); retractions carry over.
- `kb retract-chunk` / `kb retract-source`: staleness and whole-source
  takedown with card cascade.

What this CANNOT do — and only v1 can:

1. **Kernel-enforced speaker policy.** `kb verify` still passes a card whose
   quote is verbatim inside a cited chunk even if the chunk is a commenter's
   words. Today the deck linter (G2) catches this; the kernel does not.
2. **Insert-tolerant anchors.** Chunk identity is page-positional
   (`<sid>:p<NNN>:<ord>`); a mid-stream insertion (e.g. backfilling the 11
   missing timeline posts) renumbers every later page. `--append` correctly
   refuses; v1 anchors make such refreshes non-events.

## v1 schema deltas

All deltas live behind `schema_version: "cacg.v1"`; v0 stays fully supported
read-only forever (cfa + hkex never migrate; `EXPECTED_BASELINE=268`
immutable).

### ChunkRecord (v1)

```jsonc
{
  "schema_version": "cacg.v1",
  "source_id": "...",
  "chunk_id": "<sid>:u:<utterance_id>",   // identity = stable utterance id,
                                          // NOT page position
  "chunk_hash": "<sha256>",               // envelope below
  "anchor_type": "utterance",             // "page" for PDF sources
  "ordinal": 17,                          // display/order only; NOT identity
  "speaker": "狗不叫",
  "is_author": true,
  "authored_at": "2022-06-12T06:20:00+08:00",  // nullable
  "refs": {"post_id": "222375639", "comment_id": "244701940"},
  "token_count": 42,
  "text": "...",
  "text_preview": "..."
}
```

v1 hash envelope (canonical-JSON, alphabetical):
`{anchor_type, authored_at, is_author, refs, speaker, text, utterance_id}`.
Page numbers are OUT of the envelope — re-ordering or insertion cannot stale
an untouched utterance. For `anchor_type: "page"` (PDF), the envelope keeps
the v0 fields `{end_page, page_spans, start_page, text}` so PDF sources
hash identically under both versions.

### Citation (v1)

`page_range` generalizes to `anchor`:

```yaml
- source_id: "goubujiao_xueqiu_corpus"
  chunk_id: "goubujiao_xueqiu_corpus:u:c244701940"
  chunk_hash: "..."
  anchor: { type: "utterance", utterance_id: "c244701940" }
  quote: "汪！"
  edge_type: "supports"
  speaker_role: "author"        # NEW; see speaker policy
```

### Kernel speaker policy (the point of v1)

`kb verify` on a v1 citation additionally checks:

- `speaker_role: "author"` (the default) requires the cited chunk's
  `is_author: true`. Citing a commenter chunk fails with a new diagnostic
  (`CACG-CITE-007 speaker policy`).
- `speaker_role: "context"` permits a non-author chunk but the card body
  must mark the passage as context (lint-enforced), never as the expert's
  words.

This moves the deck linter's G2 INSIDE the hash-pinned trust boundary: a
card cannot pass `kb verify` while quoting a commenter as the expert.

### `--append` under v1

With utterance-keyed identity, the byte-identical re-derivation check
becomes per-utterance instead of per-page: mid-stream insertions no longer
shift identities, so backfilling earlier posts appends cleanly; only an
EDIT of a previously-published utterance still hard-fails (upstream edits
must surface as explicit retraction + re-ingest, never silent).

## Migration order (when implemented)

1. `SchemaVersion::V1` variant + v1 envelope in `hash.rs` (fixtures first).
2. v1 `ChunkRecord`/`Citation` parse + structural validation (v0 untouched).
3. utterances backend emits v1 manifests behind `--schema-version v1`;
   locator sidecar retires (its fields move into the chunk records).
4. verify layer-2: anchor-aware window slicing + speaker policy
   (`CACG-CITE-007`).
5. lint layer-1: v1 citation shape checks.
6. expert #2 deck authors natively on v1; hkex Layer-B optionally re-anchors
   via the existing quote+pin resolver (reviewed overrides), or stays v0.

## Non-goals

- Migrating any v0 hash. v0 decks are frozen contracts.
- Multi-utterance chunks for conversations (one utterance = one chunk is the
  proven shape; thread context belongs to retrieval, not the chunk).
