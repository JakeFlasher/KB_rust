# Multi-file pair-atomic publish rollback scenario

Input: `input/` simulates the directory state after a crashed prior `kb ingest`: the canonical `chunks_manifest.json` exists, AND so do leftover `chunks_manifest.json.tmp` and `chunks_manifest.json.bak` sidecars.

Expected behavior:

- A subsequent `kb ingest <pdf> --out input/` MUST refuse to clobber the leftover sidecars and exit with `CACG-MAN-002`.
- Operator removes the sidecars manually before re-running ingest.
- The Rust `cacg_core::atomic_publish` module MUST match this refusal posture at every recovery point (write, persist, rename, fsync, cleanup).
