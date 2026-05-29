# 10k-card Stress Test

This is the **local-only** scaling validation for CACG at 10000-card scale.
It is intentionally NOT gated in CI because the generated PDF (~10 MB) and
the verify pass over 10000 cards run longer than the CI budget. Run it
manually after large refactors or before tagging a release.

## Build and Run

```bash
# 1. Generate the 10000-card corpus (deterministic under KB_FROZEN_CLOCK=1).
KB_FROZEN_CLOCK=1 N_CARDS=10000 N_PAGES=2000 \
  legacy_python_oracle/.venv/bin/python legacy_python_oracle/scripts/build_stress_fixture.py

# 2. Build the auth matrix (one entry per generated reading).
legacy_python_oracle/.venv/bin/python -c "
import json
allowed = {f'reading_{i:03d}': ['source'] for i in range(1, 401)}
with open('/tmp/stress_10k_matrix.json', 'w') as f:
    json.dump({'allowed': allowed, 'schema_version': 'cacg.v0'},
              f, sort_keys=True, separators=(',', ':'))
"

# 3. Build a synthetic round summary that cites every generated card.
( echo '# Stress 10k'; echo; echo '## Knowledge Consulted'; echo
  find .stress/cards -name '*.md' | sort \
    | sed 's|^|- `|; s|$|` -- stress 10k|' ) > /tmp/stress_10k_summary.md

# 4. Run the batched verify with --source-matrix enabled.
rm -f /tmp/stress_10k_journal.jsonl
KB_FROZEN_CLOCK=1 time legacy_python_oracle/.venv/bin/kb verify \
  --round-summary /tmp/stress_10k_summary.md \
  --chunks-manifest .stress/out/chunks_manifest.json \
  --journal /tmp/stress_10k_journal.jsonl \
  --source-matrix /tmp/stress_10k_matrix.json
```

## Accepted Runtime Budget

- `kb verify --round-summary`: under 30 seconds wall-clock (3.0 ms/card amortized).
- `kb index` (cold rebuild): under 15 seconds wall-clock.
- `kb ingest` (one-time PDF parse for the 2000-page synthetic): under 60 seconds.

The 1000-card stress fixture in CI uses the same generator with `N_CARDS=1000
N_PAGES=200` and the matching budget is 3.5 seconds; CI enforces that.

## Determinism

`KB_FROZEN_CLOCK=1` collapses timestamps and UUIDs in journal events and
history files to zero values; two consecutive runs over an identical
corpus produce byte-identical journal output. The 10k journal is large
(~200 MB) so byte-identical comparison is impractical; instead, compare the
sorted set of `(seq, command, card_path, diagnostics[0].code if any)`
tuples across runs.

## What This Validates

- **AC-11/AC-12 scaling**: the ChunksIndex cache shared by `lint_directory`
  and `verify_round_summary` keeps per-card cost flat (no quadratic growth)
  even at 10000-card scale.
- **Manifest-load failure cardinality**: corrupting the chunks_manifest
  before the run and re-running should emit exactly 10000 journal events
  each carrying `CACG-MAN-001` (AC-3 at scale).
- **Source-matrix authorization at scale**: a tampered matrix (e.g., one
  reading removed) emits 25 `CACG-AUTH-001` events (the per-reading
  bucket size); a matrix listing the wrong source emits one
  `CACG-AUTH-002` per cited card (AC-5, AC-6 at scale).
- **Determinism under load**: byte-identical output across runs.

## Cleanup

```bash
rm -rf .stress/  # ~10 MB; gitignored
rm -f /tmp/stress_10k_*.{json,jsonl,md}
```
