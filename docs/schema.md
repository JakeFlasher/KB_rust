# CACG Schema (`cacg.v0`)

Every persisted artifact carries `schema_version: "cacg.v0"`. The field is **required** on every model that loads from disk; a missing `schema_version` is rejected at parse time (Pydantic `Field required`) instead of being silently filled in. Unknown major versions are rejected with `CACG-FM-002`; unknown top-level fields are rejected with `CACG-FM-003` (Pydantic `extra="forbid"`).

## Source manifest (`sources_manifest.json`)

```json
{
  "schema_version": "cacg.v0",
  "sources": [
    {
      "schema_version": "cacg.v0",
      "source_id": "sample",
      "source_path": "tests/fixtures/sample.pdf",
      "source_sha256": "<64-hex SHA256 of raw bytes>",
      "parser_name": "pypdfium2",
      "parser_version": "5.8.0+pdfium149.0.7825.0",
      "page_count": 3,
      "extracted_at": "1970-01-01T00:00:00Z"
    }
  ]
}
```

## Chunks manifest (`chunks_manifest.json`)

```json
{
  "schema_version": "cacg.v0",
  "chunks": [
    {
      "schema_version": "cacg.v0",
      "source_id": "sample",
      "chunk_id": "sample:p001:0000",
      "chunk_hash": "<64-hex SHA256 over (normalized chunk text + start_page + end_page + page_spans)>",
      "ordinal": 0,
      "start_page": 1,
      "end_page": 2,
      "page_spans": [
        {"page": 1, "byte_offset_in_chunk": 0},
        {"page": 2, "byte_offset_in_chunk": 706}
      ],
      "token_count": 219,
      "text": "Content-addressable identity is the verification primitive ...",
      "text_preview": "Content-addressable identity is the verification primitive at the core of ..."
    }
  ],
  "retracted_source_ids": [],
  "retracted_chunk_ids": []
}
```

`retracted_source_ids` is a sorted, unique list of `source_id` strings withdrawn via `kb retract-source`. `retracted_chunk_ids` is a sorted, unique list of `chunk_id` strings withdrawn via `kb retract-chunk`. Both default to `[]`; older manifests without these fields load with empty defaults so backward compatibility is preserved.

Disjointness invariant: a `source_id` in `chunks[*].source_id` MUST NOT also appear in `retracted_source_ids`, and a `chunk_id` in `chunks[*].chunk_id` MUST NOT also appear in `retracted_chunk_ids`. The Pydantic `ChunksManifest` model rejects manifests violating either invariant; `kb retract-source` and `kb retract-chunk` enforce by removing the matching active entries before adding to the retracted lists. Citations of retracted sources/chunks fail verify with `CACG-RETR-002` / `CACG-RETR-003` respectively.

Field semantics:

- `chunk_id` is a stable label `<source_id>:p<NNN>:<ordinal>` where `NNN` is the chunk's `start_page` zero-padded and `ordinal` is a per-document monotonic counter starting at 0.
- `chunk_hash` is SHA256 over a canonical hash envelope binding `(normalized chunk text + start_page + end_page + page_spans)` — NOT text-only. Including the page metadata makes a chunk's identity sensitive to its byte-window scoping so a re-paginated source emits a different `chunk_hash` even if the text bytes are byte-identical. The `cacg.hash.chunk_hash` helper uses the same `cacg.normalize.normalize_text` pipeline as verify time over the text component.
- `end_page >= start_page`. No chunk spans more than `max_pages_per_chunk` distinct pages (default 2).

## Card frontmatter

Cards are Markdown files with a YAML frontmatter block delimited by `---` lines. The body is everything after the closing `---`.

```yaml
---
schema_version: "cacg.v0"
id: "content-addressable-identity"
title: "Content-Addressable Identity"
reading_id: "reading_01"
summary: "Required bounded-length summary used by `kb search` and the eager-load `summaries.json` artifact. Length must fall inside [SUMMARY_MIN_LENGTH=80, SUMMARY_MAX_LENGTH=400] chars."
tags:
  - "regression"
  - "ols"
card_edges:
  - target: "neighbor-card-id"
    edge_type: "depends_on"
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "<64-hex>"
    page_range: [1, 2]
    quote: "Content-addressable identity is the verification primitive ..."
    edge_type: "supports"
card_hash: "<64-hex SHA256 of frontmatter (excluding card_hash) + normalized body>"
---
Body Markdown goes here.
```

- `card_hash` is updated by `kb index`; manual edits to the body without re-running `kb index` surface as `CACG-HASH-002` (stale card_hash).
- `summary` is required (Phase 3 AC-S1) and bounded to `[80, 400]` chars. Empty / short summaries fail parse with `CACG-SUM-001`; oversized with `CACG-SUM-002`. Pre-Phase-3 cards migrate via `kb migrate-summaries [--auto-heuristic|--strict]`.
- `tags` is additive (default `[]`). Each entry matches the lowercase-slug regex `^[a-z0-9][a-z0-9_-]*$`, length `[2, 40]`, up to `TAGS_MAX_COUNT=10` entries, no duplicates. Slug / length / duplicate failures emit `CACG-SUM-003`; count overflow emits `CACG-SUM-004`. `tags` participate in `kb search` BM25 ranking alongside title + summary.
- `card_edges` is additive (default `[]`). Each entry is `{target: card_id, edge_type: "depends_on" | "extends"}`. The `edge_type` literal is INTENTIONALLY narrower than the citation-level `edge_type` enum (which includes `supports`, `defines`, `contrasts_with`, `applies_to`) so the two graphs (card-to-chunk vs card-to-card) maintain separate namespaces. Card-edge validation runs at lint time with `CACG-DEP-001..004` covering unknown / cycle / orphan / dangling-retracted respectively.
- Citation-level `edge_type` values: `supports`, `defines`, `extends`, `contrasts_with`, `depends_on`, `applies_to`.
- `page_range` is `[start, end]` with `start <= end` (`CACG-CITE-003` on reversal). It must intersect the cited chunk's `[start_page, end_page]` (`CACG-CITE-005` on disjoint).
- `citations` is required and non-empty. A card with `citations: []` fails the loader because every cacg.v0 card must commit to at least one PDF-grounded claim.

## Summaries manifest (`summaries.json`) — Phase 3

Eager-load card-of-cards artifact published at `kb index` time via the sequenced Phase D publisher. Strict Pydantic (`extra="forbid"`, `schema_version="cacg.v0"`). Entries sorted by `(reading_id, id)`. Retracted cards are EXCLUDED (the canonical retraction state lives in `cards_manifest.retracted_cards`).

```json
{
  "schema_version": "cacg.v0",
  "summaries": [
    {
      "schema_version": "cacg.v0",
      "id": "content-addressable-identity",
      "title": "Content-Addressable Identity",
      "reading_id": "reading_01",
      "summary": "Required bounded-length summary ...",
      "tags": ["regression", "ols"],
      "path": "cards/reading_01/g.md",
      "card_hash": "<64-hex>"
    }
  ]
}
```

The publisher invariant `_validate_summaries_match_cards_manifest` asserts every summary entry's `(id, card_hash, reading_id, title, summary, path)` matches the corresponding `cards_manifest.cards[*]` entry (mechanical sameness; `tags` is summaries-only and not cross-checked). Duplicate ids are rejected at load time.

## Per-reading source role map (`out/role_maps/<reading_id>.json`) — Phase 3

Optional, operator-authored per-reading artifact. Strict Pydantic. Each entry maps `card_id → {primary, supporting, stance, relevance}` where the closed-set vocabularies are hardcoded per DEC-2 v1: `stance ∈ {primary, supporting, deferred}`; `relevance ∈ {core, adjacent, extension}`. Authored once via `kb scaffold-role-map --reading <id>` then curated by hand. Symmetric validation at lint time emits `CACG-ROLE-001/002/003`.

```json
{
  "schema_version": "cacg.v0",
  "reading_id": "reading_01",
  "entries": [
    {
      "card_id": "content-addressable-identity",
      "primary": {"source_id": "sample", "page_span": [1, 2]},
      "supporting": [],
      "stance": "supporting",
      "relevance": "core"
    }
  ]
}
```

## SKILL.md routers (`cards/reading_<NN>/SKILL.md`) — Phase 3, optional

Optional per-reading task-shaped router files. Strict Pydantic frontmatter: `name` (slug regex), `description` (bounded), optional `routes_to: list[card_id]`, optional `trigger_keywords: list[str]`. Per-reading-only routing (DEC-5 default): `routes_to` entries must point at active cards in the SAME reading. Validation emits `CACG-SKILL-001/002/003`.

## Cards manifest (`cards_manifest.json`)

```json
{
  "schema_version": "cacg.v0",
  "cards": [
    {
      "schema_version": "cacg.v0",
      "path": "cards/reading_01/g.md",
      "id": "content-addressable-identity",
      "title": "Content-Addressable Identity",
      "reading_id": "reading_01",
      "summary": "",
      "card_hash": "<64-hex>",
      "citation_count": 1
    }
  ],
  "retracted_cards": []
}
```

`retracted_cards` is a sorted, unique list of `id`s withdrawn via `kb retract <card>`. The Pydantic model enforces the disjointness invariant: a single `id` may never appear in both `cards[*].id` and `retracted_cards` (validation rejects the manifest with a `CACG-MAN-001`-mapped error). `verify_one_card` emits `CACG-RETR-001` when a target card's `id` is on the list.

`INDEX.md` is a human-readable companion that lists every card by `reading_id` with truncated `card_hash` prefixes.

## Source matrix (`source_matrix.json`) — mandatory

Authorization artifact. The `--source-matrix <path>` flag is **MANDATORY** on `kb lint`, `kb verify`, `kb verify --round-summary`, and `kb search` (Round 6 trust-depth contract; reaffirmed by Phase-3 DEC-8). Invocations without it exit 2 with the argparse usage error. Bootstrap a permissive matrix from an existing indexed corpus via `kb scaffold-matrix --cards-manifest <p> --chunks-manifest <p> --out <p>`.

```json
{
  "schema_version": "cacg.v0",
  "allowed": {
    "reading_01": ["sample"],
    "reading_02": ["sample", "appendix_a"]
  }
}
```

`allowed` maps `reading_id -> [allowed source_id, ...]`. A citation passes authorization iff the card's `reading_id` is a key and the citation's `source_id` is in the corresponding list. Layer-1 emits `CACG-AUTH-001` (unknown reading) or `CACG-AUTH-002` (unauthorized source); layer-2 emits the same codes independently so `kb verify --unsafe-skip-lint` still enforces. Malformed or unreadable matrices emit `CACG-AUTH-000` per card visit (preserving batch journal cardinality).

## Lint journal (`lint_journal.jsonl`)

Append-only JSONL. Each line is one `LintEvent`:

```json
{
  "schema_version": "cacg.v0",
  "seq": 3,
  "event_id": "<uuid4 or zero-UUID under frozen clock>",
  "timestamp": "<ISO-8601 or 1970-01-01T00:00:00Z under frozen clock>",
  "command": "lint" | "verify",
  "card_path": "cards/reading_01/g.md",
  "card_hash_before": "<64-hex or null>",
  "card_hash_after": "<64-hex or null>",
  "diagnostics": [{"code": "CACG-...", "severity": "error", "message": "...", "file": "...", "hints": []}],
  "verification": {"layer1": true, "layer2": true, "fuzzy": false},
  "latency_ms": 0.0,
  "prev_checksum": "<event_checksum of prior line or null on first line>",
  "event_checksum": "<SHA256 of canonical JSON without the event_checksum field>"
}
```

Tamper detection: validators recompute `event_checksum` per line and verify the chain back to the previous line's recorded `event_checksum`. Same-`seq` rewrites break the chain.

## Per-card history (`<card>.history.jsonl`)

Same shape; one event per `card_hash` change. `kb retract` appends a tombstone event with `is_retracted: true` and the marker string `__cacg_retracted__` in `frontmatter_field_changes`:

```json
{
  "schema_version": "cacg.v0",
  "seq": 0,
  "timestamp": "1970-01-01T00:00:00Z",
  "prev_card_hash": null,
  "new_card_hash": "<64-hex>",
  "cited_chunk_set_delta": {"added": [], "removed": []},
  "frontmatter_field_changes": [],
  "cited_chunk_ids_snapshot": [],
  "frontmatter_snapshot": {},
  "is_retracted": false,
  "prev_checksum": null,
  "event_checksum": "<SHA256>"
}
```

## Determinism contract

- All hash inputs go through `cacg.normalize.normalize_text` (NFC, ligature unification, hyphenated line-end rejoin, whitespace collapse).
- All JSON output uses `sort_keys=True, separators=(",", ":"), ensure_ascii=False`.
- Under `KB_FROZEN_CLOCK=1`, timestamps become `1970-01-01T00:00:00Z` and UUIDs become `00000000-0000-0000-0000-000000000000`.
- `pypdfium2` is pinned to `==5.8.0`. Bumping the pin requires re-running the fixture diff suite and intentionally regenerating chunk hashes if extracted text changes (see `docs/analyses/T30-pypdfium2-determinism.md`).

## Schema evolution policy

- `schema_version: cacg.v0` is the only major version recognized today.
- Additive minor fields (e.g. `cacg.v0.1` with ignorable extras) are deferred until a real need arises.
- Major-version bumps require a migration tool and a Plan Evolution entry in the goal tracker.
