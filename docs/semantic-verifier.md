# CACG Layer-3 Semantic Verifier (Phase 3, Milestone 4)

Layer-3 is the **opt-in** semantic escape hatch above Layer-2's mechanical exact-substring + fuzzy. It fires ONLY when:

1. Layer-2 exact-substring match has failed.
2. `--fuzzy` is disabled OR fuzzy itself rejected the near-match.
3. The operator explicitly passed `--semantic <cache>` (B1) or `--semantic-judge` (B2) on `kb verify` or `kb verify --round-summary`.

The verdict rides INSIDE the same per-card journal event as Layer-2's `CACG-VERIFY-001`. There is exactly one journal event per card visit regardless of which layers fire (AC-V1 cardinality contract).

## DEC-1: Umbrella mode (B1 + B2)

Both sub-modes ship behind separate flags, mutually exclusive at the argparse layer:

- `--semantic <cache_path>` → B1 (cache-as-oracle dict lookup).
- `--semantic-judge` → B2 (LLM-judge via Claude Haiku).

The default `kb verify` path runs Layer-1 + Layer-2 + optional `--fuzzy` and never invokes `run_semantic_check`. There is no transitive `sentence-transformers` import on the default path AND no outbound network call; the embedding model is the cache-builder, not the runtime. (The CLI module imports `cacg.verify.semantic` to construct the optional `SemanticSpec` argument; the import is cheap and the module's runtime functions stay unreachable when neither `--semantic` nor `--semantic-judge` is supplied.)

## DEC-9: Cache-as-oracle for B1

The runtime is a strict dict lookup against a frozen `out/semantic_cache.json` built offline on a canonical platform (Linux x86_64 + pinned `sentence-transformers/all-MiniLM-L6-v2`). Per-instance O(1) lookup via `SemanticCache.lookup(chunk_hash, claim_window_hash)`:

```json
{
  "schema_version": "cacg.v0",
  "entries": [
    {
      "chunk_hash": "<64-hex>",
      "claim_window_hash": "<64-hex>",
      "verdict": "pass",
      "score": 0.92
    }
  ]
}
```

`claim_window_hash` is SHA256 over `normalize_text(quote)` (NFC + whitespace + ligature unification). The cache file is committed to the repo so cross-platform reproducibility is achieved by NOT running the model at verify time. An unknown key returns `verdict: abstain, score: 0.0` — Layer-3 makes no guess on uncached citations.

**Portability note:** the runtime never invokes `sentence-transformers`. The offline cache-builder script (out of scope for this milestone; future Phase 3.1) does. Operators on platforms where `sentence-transformers` / PyTorch are unavailable can still consume B1 by checking out the pre-built cache file.

## B2 LLM-judge (CI-only)

`--semantic-judge` dispatches through `cacg.verify.semantic._judge_via_claude`. The default stub raises `NotImplementedError` so a forgotten monkey-patch in tests surfaces loudly. Real deploys monkey-patch the helper with a Claude Haiku call returning `SemanticVerdict(verdict, score, reasoning, mode="llm-judge")`.

**Isolation contract (AC-V3):** the default `kb verify` path makes **zero outbound network calls**. Tests verify this by patching `_judge_via_claude` to raise `RuntimeError("default path made an outbound call!")` and asserting the default-flag run never hits it.

## `SemanticVerdict` (strict-validated)

```python
class SemanticVerdict(_StrictModel):
    verdict: Literal["pass", "fail", "abstain"]
    score: float  # bounded [0, 1], NaN/inf rejected
    reasoning: str | None = None
    mode: Literal["embedding-cache", "llm-judge"] = "embedding-cache"
```

A misbehaving B2 LLM-judge that returns `verdict: "bogus"`, `score: nan`, or `score: 1.5` is rejected at construction — the bad payload cannot propagate into the journal event.

## Diagnostic shape

`CACG-VERIFY-002` rides inside the same diagnostics array as `CACG-VERIFY-001`:

```
diagnostics:
  - code: "CACG-VERIFY-001"
    severity: "error"
    message: "citations[0]: quote not found in pinned chunk src:p001:0000 (fuzzy=False)"
    hints:
      - chunk_id: "src:p001:0000"
        score: -0.457755
        text_preview: "..."
  - code: "CACG-VERIFY-002"
    severity: "error"           # per-verdict severity contract; see below
    message: "semantic verdict=fail score=0.050000 mode=embedding-cache"
    hints:
      - semantic_verdict: "fail"
        semantic_score: 0.05
        semantic_mode: "embedding-cache"
```

Per-verdict severity contract:

- `verdict: "pass"` → `severity: "warning"`. Semantic-only "support" is not the verification authority (Layer-2 exact-match is the gate); a pass verdict at Layer-3 documents the semantic case without flipping the run to success.
- `verdict: "fail"` → `severity: "error"`. Layer-3 actively rejects the claim window; the run already fails on `CACG-VERIFY-001`.
- `verdict: "abstain"` → `severity: "info"`. Cache miss / no signal, not an error; the run still fails on `CACG-VERIFY-001` but the abstain is informational only.

## Failure modes

| Trigger | Code | Path |
|---|---|---|
| `--semantic <path>` to nonexistent / unreadable / malformed cache | `CACG-MAN-001` | stderr; exit 1; pipeline never starts |
| Both `--semantic` and `--semantic-judge` supplied | argparse exit 2 | stderr (mutually exclusive group) |
| `SemanticSpec` misconfigured at runtime (both modes active) | `CACG-VERIFY-002` | inside the journal event |
| Verifier raises any exception | `CACG-VERIFY-002` with `message=...exc` | inside the journal event |
| Unknown `(chunk_hash, claim_window_hash)` in B1 cache | `CACG-VERIFY-002` with `verdict: abstain` | inside the journal event |

## Determinism

- B1 cache file is canonical JSON (`sort_keys=True`, no whitespace) and read-only at runtime; two `kb verify --semantic <path>` runs over the same corpus produce byte-identical journal events under `KB_FROZEN_CLOCK=1`.
- B2 LLM-judge is non-deterministic by design (the LLM may differ run-to-run). Use B2 in CI-only review jobs; do NOT gate merges on B2 verdicts.

## Future work

- Phase 3.1: ship `kb build-semantic-cache <model> --out semantic_cache.json` for operators to refresh the cache when the corpus grows.
- Phase 3.2: cross-validation pass that flips a `pass` semantic verdict against a fuzzy-rejected Layer-2 quote into a `CACG-VERIFY-002 warning` (currently the same code) so consumers can route warnings through a separate gate.
