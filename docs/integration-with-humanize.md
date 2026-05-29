# Integrating CACG with humanize RLCR

CACG provides `kb verify --round-summary` as a CLI-only handshake into humanize's `## Knowledge Consulted` contract. The integration adds *content* verification on top of humanize's existing provenance check (which only verifies that the section exists and lists concrete paths).

## Contract

A round summary is any Markdown file produced by humanize's RLCR loop. The integration looks for a `## Knowledge Consulted` heading; the section body extends until the next `## ` heading or end of file. Valid section bodies are one of:

1. The exact sentinel line:
   ```
   N/A -- task not KB-relevant this round
   ```
   Exits 0. Stdout: `N/A acknowledged`.
2. A list of card paths, one per Markdown bullet (`- ` or `* `). Bullets may include trailing prose after `--`, `—`, ` - `, or `(`. The first token is the path. Each cited path runs through `kb lint` + `kb verify`. Exit codes:
   - 0 if every cited path is `VERIFIED`.
   - 1 if any cited path is `STALE` (lint or verify failed; file present) or `MISSING` (file does not resolve).

If the `## Knowledge Consulted` section is absent and the surrounding round summary text contains references to `cards/` or `.claude/knowledge/`, the integration exits 2 with `CACG-RS-001`. If the section is absent and no KB-relevant work is mentioned, the integration exits 0 with an informational note.

The KB-relevant heuristic is a single regex: `(?:cards/|\.claude/knowledge/)`. Humanize integrators that want a different heuristic can run their own pass before calling CACG.

## Exit-code summary

| Section state | Result | Exit |
|---------------|--------|------|
| `## Knowledge Consulted` present, lists VERIFIED paths | per-path `VERIFIED` lines | 0 |
| `## Knowledge Consulted` present, exact N/A sentinel, body NOT KB-relevant | `N/A acknowledged` | 0 |
| `## Knowledge Consulted` present, any STALE/MISSING path | per-path verdicts on stderr | 1 |
| `## Knowledge Consulted` present, exact N/A sentinel BUT body mentions `cards/` or `.claude/knowledge/` | `CACG-RS-003` STALE | 1 |
| `## Knowledge Consulted` present, no paths and no N/A, body KB-relevant | `CACG-RS-004` STALE | 1 |
| `## Knowledge Consulted` present, mixed N/A sentinel + cited paths | `CACG-RS-002` STALE + per-path verdicts | 1 |
| Section missing AND text mentions `cards/` or `.claude/knowledge/` | `CACG-RS-001` | 2 |
| Section missing AND no KB-relevant mention | informational note | 0 |

## CLI usage

```bash
kb verify --round-summary path/to/round-N-summary.md \
  --chunks-manifest out/chunks_manifest.json \
  --source-matrix out/source_matrix.json
```

The `--chunks-manifest` defaults to `./out/chunks_manifest.json`. `--source-matrix` is MANDATORY (per Round 6 Trust-Depth contract). Cards are resolved relative to the round summary's parent directory; paths that fail to resolve there are tried relative to CWD before being reported as `MISSING`.

### Optional Layer-3 semantic verifier

Phase 3 Milestone 4 adds two mutually-exclusive flags that compose with `--round-summary`:

```bash
# B1: cache-as-oracle (deterministic, opt-in)
kb verify --round-summary path/to/round-N-summary.md \
  --chunks-manifest out/chunks_manifest.json \
  --source-matrix out/source_matrix.json \
  --semantic out/semantic_cache.json

# B2: LLM-judge via Claude Haiku (non-deterministic, CI-only)
kb verify --round-summary path/to/round-N-summary.md \
  --chunks-manifest out/chunks_manifest.json \
  --source-matrix out/source_matrix.json \
  --semantic-judge
```

Layer-3 fires per cited card iff Layer-2 exact-match fails AND `--fuzzy` rejects. The verdict rides inside the same `command="verify"` journal event as the Layer-2 diagnostic (one event per card; AC-V1 cardinality). See `docs/semantic-verifier.md` for the contract.

## Sample round summary that passes

```markdown
# Round 3 Summary

## Work Completed
- Implemented kb verify --round-summary

## Knowledge Consulted

- cards/reading_01/g.md -- canonical golden card
- cards/reading_01/h.md -- determinism reference

## BitLesson Delta
- Action: none
- Lesson ID(s): NONE
```

## Bringing an external KB's cards into CACG as a fixture

Per T37 analysis (see `docs/analyses/T37-cfa-smoke-verifier.md`), the recommended pattern for stress-testing the verifier against a real read-only KB is:

1. Copy or symlink the source PDF into `tests/fixtures/<name>/`.
2. Run `kb ingest tests/fixtures/<name>/source.pdf --source-id <slug>`.
3. Author a `cacg.v0` companion card whose `citations` use the chunk_id, chunk_hash, page_range, and an actual quote from the chunk text (not the external card's inline `**Source:**` labels).
4. Run `kb verify` to learn which claims map cleanly, which need `--fuzzy`, and which the framework cannot verify mechanically.

This pattern enables CACG to participate in any knowledge-base ecosystem without ever modifying the source repo.

## Recommended deployment posture: opt-in `--source-matrix` in CI

`--source-matrix` is **MANDATORY** on `kb lint`, `kb verify`, and `kb verify --round-summary`. Invocations without it exit 2 with a clear "the following arguments are required: --source-matrix" message from argparse. This is a deliberate break-compat change in the trust-depth phase to ensure every CLI invocation explicitly states its authorization posture.

Deployment posture:

1. Commit a `source_matrix.json` to the repository (canonical JSON via `cacg.hash.canonical_json`, schema documented in `docs/schema.md`).
2. Bootstrap the matrix from an existing indexed corpus with `kb scaffold-matrix --cards-manifest <path> --chunks-manifest <path> --out <path>`. The scaffold-matrix subcommand walks each card's citations and builds `allowed[reading_id] = [every source_id actually cited]`, producing a permissive starting matrix that operators tighten as needed. Retracted cards are excluded from the scaffold output.
3. Run `kb verify --source-matrix <path>` (and `kb verify --round-summary --source-matrix <path>`) in CI so unauthorized citations fail closed before merge.

See AC-5 / AC-6 in `.humanize/plans/cacg-trust-depth-plan.md` for the full contract; DEC-2 (Round 6) for the break-compat rationale.

## Retraction integration: card-level, source-level, chunk-level

CACG supports three retraction granularities. All three are atomic via a tmp/bak/replace publish discipline; physical artifacts are preserved on disk for audit.

**Card retraction**: `kb retract <card> --out <out_dir>`
1. Appends a tombstone history event (sentinel marker `__cacg_retracted__` in `frontmatter_field_changes`, `is_retracted=true`) and atomically rewrites `cards_manifest.json` to remove the card from `cards` and add its `id` to `retracted_cards`.
2. The physical `.md` file is preserved on disk as a historical artifact.
3. Any subsequent `kb verify --round-summary` whose Knowledge Consulted section cites the retracted card emits `CACG-RETR-001`. The round summary's `STALE` verdict reflects the rejection.
4. `--allow-retracted` downgrades the diagnostic severity from `"error"` to `"warning"`. The diagnostic is still emitted and journaled; only the exit code changes. Note: `--allow-retracted` only applies to RETR-001 (card-level); RETR-002 and RETR-003 (source / chunk) are always `"error"`.
5. Re-running `kb index` preserves the retracted state; un-retraction requires explicitly re-authoring the card so its `id` re-enters `cards_manifest.cards`.

**Source retraction**: `kb retract-source <source_id> --out <out_dir>`
1. Atomically rewrites `chunks_manifest.json` to remove every chunk with the target `source_id` from `chunks_manifest.chunks` and add the `source_id` to `chunks_manifest.retracted_source_ids`.
2. Any card citing the retracted source fails verify with `CACG-RETR-002` at both the layer-1 trust boundary (normal verify) and the layer-2 trust boundary (`kb verify --unsafe-skip-lint`).
3. The Pydantic disjointness invariant on `ChunksManifest` rejects manifests where a `source_id` appears in both `chunks[*].source_id` AND `retracted_source_ids`.

**Chunk retraction**: `kb retract-chunk <chunk_id> --out <out_dir>`
1. Atomically rewrites `chunks_manifest.json` to remove the matching chunk and add its `chunk_id` to `chunks_manifest.retracted_chunk_ids`.
2. Citations of the retracted chunk fail verify with `CACG-RETR-003` at both layer-1 and layer-2.
3. Same disjointness invariant applies to `chunk_id`.

All three retraction kinds compose: a corpus can have card-level retractions in `cards_manifest.retracted_cards`, source-level in `chunks_manifest.retracted_source_ids`, and chunk-level in `chunks_manifest.retracted_chunk_ids` simultaneously without cross-coupling.

## Scaffolding a source_matrix from an existing corpus

`kb scaffold-matrix --cards-manifest <path> --chunks-manifest <path> --out <path>` synthesizes a permissive `source_matrix.json` by walking each card's citations to discover which `(reading_id, source_id)` pairs are actually used. The output is canonical JSON. Retracted cards (those in `cards_manifest.retracted_cards`) are excluded from the scaffold walk. Operators typically run scaffold-matrix once after `kb index`, then tighten the matrix manually (e.g., removing source_ids that should not be authorized for a given reading).
