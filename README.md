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

- `ingest`, `new`, `lint`, `verify`, `index`
- `retract-chunk`
- `search`, `show`

The parser still reserves the full historical subcommand surface. These verbs
are recognized but intentionally fail with `CACG-CLI-NOT-IMPLEMENTED-*` until a
Rust implementation exists: `history`, `retract`, `retract-source`,
`scaffold-matrix`, `scaffold-role-map`, and `migrate-summaries`.

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
docs/                    current operator and architecture documentation
```

## License

MIT.
