# CACG: Content-Addressable Card Graph

A Rust workspace for authoring small Markdown knowledge cards from PDF
sources with deterministic, hash-pinned verification.

The active implementation lives under `crates/` and `xtask/`. The old live
Python tree has been retired; byte-equal compatibility is now enforced against
committed fixture bytes under `tests/parity_corpus/out_python/`, with no Python
installation required.

## Status

The shipped `kb` binary natively implements the core authoring and verification
flow:

- `ingest` (PDF and `--format utterances`), `new`, `lint`, `verify`, `index`
- `retract-chunk`, `retract-source`
- `search`, `show`

The parser still reserves the full historical subcommand surface. These verbs
are recognized but intentionally fail with `CACG-CLI-NOT-IMPLEMENTED-*` until a
Rust implementation exists: `history`, `retract`, `scaffold-matrix`,
`scaffold-role-map`, and `migrate-summaries`.

## Quick Start

Build and run the Rust workspace:

```bash
cargo build --workspace
cargo test --workspace --all-targets
cargo run -p xtask -- parity --corpus tests/parity_corpus/
```

Run a verification pass against the committed parity fixtures:

```bash
cargo run -p cacg-cli --bin kb -- verify \
  tests/parity_corpus/valid/01-content-addressable-identity.md \
  --chunks-manifest tests/parity_corpus/out_python/chunks_manifest.json \
  --source-matrix tests/parity_corpus/out_python/source_matrix.json
```

Run search and show against committed fixture manifests:

```bash
cargo run -p cacg-cli --bin kb -- search identity \
  --source-matrix tests/parity_corpus/kb_search/source_matrix.json \
  --summaries tests/parity_corpus/kb_search/summaries.json \
  --json

cargo run -p cacg-cli --bin kb -- show content-addressable-identity \
  --cards-manifest tests/parity_corpus/kb_show/cards_manifest.json \
  --source-matrix tests/parity_corpus/kb_show/source_matrix.json
```

For a scratch ingest run, write to a fresh output directory. `kb ingest`
deliberately refuses to clobber existing manifests.

```bash
rm -rf /tmp/cacg-demo-out
KB_FROZEN_CLOCK=1 cargo run -p cacg-cli --bin kb -- ingest \
  tests/parity_corpus/pdfs/sample.pdf \
  --source-id sample \
  --out /tmp/cacg-demo-out
```

`KB_FROZEN_CLOCK=1` collapses timestamps and UUIDs in generated artifacts so
fixture runs are byte-stable.

## PDF archive transport

PDFs are intentionally treated as an external payload: keep the source code in
git, upload/download the PDF zip somewhere else, then restore the PDFs locally
when needed. The archive stores repo-relative paths and SHA-256 hashes, so new
PDFs in future subfolders are picked up automatically when the archive is
rebuilt.

Create or refresh the local archive:

```bash
scripts/pdf-archive.py pack --overwrite
```

Create the archive and stage all tracked PDFs for removal from git while leaving
the local files on disk:

```bash
scripts/pdf-archive.py patch --overwrite
# equivalent to:
# scripts/pdf-archive.py pack --overwrite --git-rm-cached
```

The default archive path is `.pdf-archives/repo-pdfs.zip`, which is ignored by
git. Upload that zip outside git. After cloning the source-only repo elsewhere,
download the zip and restore the PDFs into their original paths:

```bash
scripts/pdf-archive.py unpatch --archive /path/to/repo-pdfs.zip
scripts/pdf-archive.py status --archive /path/to/repo-pdfs.zip
```

## Exporting a deck into another project

`scripts/export-knowledge.sh` copies one deck (cards + the consumer-facing
manifests) into a downstream project in a structured, tiered layout — for
example as a `.claude/knowledge/` knowledge base for the humanize RLCR harness.
It reads from the local **working tree** (not `git archive`): the
consumer-critical `source_matrix.json`, `chunks_manifest.json`, and
`summaries.sqlite` are gitignored build artifacts that exist on disk but are
never committed.

```bash
# Default: query tier (browse + `kb search`/`kb show`), deck cfa, into <target>/.claude/knowledge
scripts/export-knowledge.sh /path/to/target-project

# Verify tier also ships chunks_manifest.json so `kb verify` works in the target
scripts/export-knowledge.sh /path/to/target-project --tier verify

# Preview the plan without writing anything
scripts/export-knowledge.sh /path/to/target-project --tier full --dry-run
```

### Tiers

| Tier | Size | Adds | Enables |
|------|------|------|---------|
| `query` (default) | ~9 MB | cards + small manifests (`source_matrix`, `cards_manifest`, `summaries`, `INDEX.md`, `summaries.sqlite`, semantic cache) | browse, `kb search`, `kb show` |
| `verify` | ~195 MB | + `chunks_manifest.json` (+ reproducibility lock) | `kb verify`, `kb verify --round-summary` |
| `full` | ~1.3 GB | + `sources/<deck>/` (PDFs + `_registry/`) | re-ingest |

The large `out/<deck>/ingest_per_source/` and any `*.lock` / `lint_journal` /
`v0_baseline` artifacts are always excluded.

### Options

```
<TARGET_DIR>             (required) project directory to export into
--tier query|verify|full what to copy (default: query)
--deck <name>            deck under cards/ and out/ (default: cfa)
--dest-root <relpath>    sub-path under TARGET (default: .claude/knowledge)
--no-sqlite              skip summaries.sqlite (regenerable FTS index)
--no-semantic            skip out/semantic_cache.json{,.provenance.json}
--with-binary            also copy the built kb binary into <dest>/bin/kb (platform-specific)
--force                  overwrite an existing populated dest-root
--dry-run                print the plan; copy nothing
-h, --help               usage
```

Requires `rsync`, `jq`, and `sha256sum` (or `shasum`).

### Output layout

```
<TARGET>/.claude/knowledge/
  INDEX.md               card index / primer (humanize reads this)
  README.md              generated consumer guide
  EXPORT_MANIFEST.json   receipt: per-file size + sha256, source commit, kb version, tier
  kb-query.sh            wrapper: ./kb-query.sh {search|show|verify} ...
  cards/<deck>/...       cards (.md + .history.jsonl), grouped by reading_id
  out/<deck>/...         source_matrix.json, cards_manifest.json, summaries.json, INDEX.md, ...
  out/semantic_cache.json (+ provenance)
  sources/<deck>/...     (full tier only) pdfs/ + _registry/
```

The script builds into a staging directory **on the target's filesystem** and
swaps it into place with an atomic rename only after integrity checks pass:
every copied file is re-hashed against the receipt, every manifested card path
must be present, `source_matrix.json` must authorize each shipped reading, and —
when the pinned `kb` binary is available — a `kb index` re-derivation must
reproduce `cards_manifest.json` byte-for-byte.

### Querying an export

```bash
cargo build --release -p cacg-cli          # produces target/release/kb (reports "kb 0.1.0")
cd /path/to/target-project/.claude/knowledge
KB_BIN=/path/to/target/release/kb ./kb-query.sh search "duration convexity" --top-k 5 --json
KB_BIN=/path/to/target/release/kb ./kb-query.sh show fi-duration-and-convexity
```

The wrapper `cd`s into the export root because `cards_manifest.json` stores
repo-relative card paths that `kb show` resolves against the working directory.
Verify the export at any time with the commands printed in the generated
`README.md` (per-file sha256 against the receipt, plus a `kb index` re-derivation
diff).

### humanize integration

A query-tier export lands `INDEX.md` at `.claude/knowledge/INDEX.md`, where the
humanize RLCR harness expects its knowledge primer. With the companion
`kb-knowledge-route.sh` `UserPromptSubmit` hook and `kb_enabled: true` in the
project's `.humanize/config.json`, each prompt auto-routes matching cards; a
round summary's `## Knowledge Consulted` section can then be content-verified
with `kb verify --round-summary` (verify tier). See
[`docs/integration-with-humanize.md`](docs/integration-with-humanize.md).

## Verification Model

```
+------------------------------------------------------------+
|  L3: AUTHORING LOOP                                        |
|     round summaries can list Knowledge Consulted paths      |
+------------------------------------------------------------+
|  L2: CARD-LIFECYCLE LOOP                                   |
|     kb new -> author -> kb lint -> kb verify -> kb index    |
|     hash-pinned chunks; append-only card history            |
+------------------------------------------------------------+
|  L1: VERIFICATION ENGINE                                   |
|     Layer 1: schema, citation, auth, retraction checks      |
|     Layer 2: normalized exact substring containment         |
|     Optional: BM25/fuzzy/semantic diagnostic surfaces       |
+------------------------------------------------------------+
```

Design principles:

- **Hash pinning over retrieval.** Cards cite `chunk_id + chunk_hash`; source
  or chunk drift mechanically stales the citation.
- **Exact containment first.** Layer 2 verifies normalized exact substrings
  against pinned chunks. BM25 and fuzzy matching are diagnostic or opt-in
  surfaces, not the primary oracle.
- **Deterministic publish.** Manifest writes use atomic publication and
  canonical JSON. Journals and card histories preserve tamper-evident chains.
- **No PDF parsing on the common path.** `kb lint`, `kb verify`, `kb search`,
  and `kb show` consume committed manifests and cards, not PDFs.

## Quality Gate

`cargo run -p xtask -- gate` runs the unified local gate: formatting, clippy,
supply-chain policy, static lints, dependency audits, semantic-cache provenance,
schema-fixture audit, and the workspace test suite.

CI currently requires:

- `Rust workspace tests`
- `Committed-fixture byte-equal parity`
- `Workflow integrity (parity gate cannot be silently disabled)`

See [`docs/rust-quality-gate.md`](docs/rust-quality-gate.md) and
[`docs/release-discipline.md`](docs/release-discipline.md).

## Layout

```
crates/cacg-core/        trust kernel: schemas, lint, verify, index, retract
crates/cacg-cli/         kb binary and command dispatchers
crates/cacg-ingest/      PDF extraction and source/chunk manifest builder
crates/cacg-search/      BM25/FTS search sidecar support
crates/cacg-semantic/    frozen semantic-cache loader
xtask/                   parity harness, gates, audits, static lints
tests/parity_corpus/     committed oracle fixtures and PDFs
tests/golden/            valid card fixtures
tests/adversarial/       12 one-code adversarial fixtures
cards/cfa/        active migrated CFA cards and history sidecars
sources/cfa/      staged CFA source corpus and registry
out/cfa/          committed release manifests and rebuild recipes
scripts/                 operator scripts (export-knowledge.sh: tiered deck export)
docs/                    current operator and architecture documentation
```

## License

MIT.
