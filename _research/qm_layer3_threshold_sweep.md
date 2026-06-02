# QM Layer-3 Threshold Calibration Sweep

This document records the MiniLM cosine-similarity threshold
calibration for the committed QM B1 semantic cache. It is the
empirical companion to the cache-rebuild ceremony at
`_research/20_b1_cache_provisioning.md` — the ceremony writes the
cache bytes; this document explains why the threshold takes the
locked value it does, and exhibits the per-threshold verdict
distribution a reviewer can independently verify.

## 1. Source data

| Property | Value |
|----------|-------|
| Cache file under sweep | `out/semantic_cache.json` |
| Cache schema | `cacg.v0` |
| Total entries scanned | 227 |
| Paraphrase entries | 222 |
| Negative-fixture entries (forced `score = 0.0`) | 5 |
| Score precision | 6 decimal places (rounded at build time) |
| Min paraphrase score | 0.449246 |
| Max paraphrase score | 0.757522 |
| Median paraphrase score | 0.617257 |
| Mean paraphrase score | 0.614603 |
| Paraphrase score stdev | 0.059733 |

The score column reflects the linear-rescaled cosine similarity
`(cos + 1) / 2` between the chunk text embedding and the
paraphrase claim embedding, with both inputs L2-normalized by
`sentence-transformers.encode(..., normalize_embeddings=True)`.

The 5 negative fixtures are forced to `score = 0.0` at cache
build time (intentionally unrelated chunk/claim pairs), so they
fail at any positive threshold by construction.

## 2. Sweep parameters

| Parameter | Default | Value used for this report |
|-----------|---------|----------------------------|
| Vertical  | `qm`    | `qm` (only frozen label set) |
| `--from`  | 0.40    | 0.40 |
| `--to`    | 0.99    | 0.99 |
| `--step`  | 0.01    | 0.01 |
| Row count | 60      | 60 |

Sweep replay command:

```bash
cargo xtask threshold-sweep --vertical qm
```

The sweep walks the cache's stored `score` field; the embedding
model is NOT re-invoked. The output is canonical JSON to stdout
and a brief operator summary to stderr.

## 3. Per-threshold verdict distribution

The table below reports `(pass, fail)` counts at every swept
threshold in `[0.40, 0.99]`, step `0.01`. Total entries = 227 at
every row; `pass = count(score >= threshold)`,
`fail = 227 - pass - abstain`, `abstain = 0`.

The `abstain` column is reserved by the AC-7 distribution contract
(`pass / fail / abstain`) for the future B2 LLM-judge cache shape;
the B1 in-cache score sweep cannot emit abstain because every
committed entry stored a binary pass/fail verdict at build time.
For the same reason, `abstain = 0` is reported at every row, and
the sweep's machine-readable JSON output preserves the column so
downstream tooling does not need a B1/B2 schema branch.

| threshold | pass | fail | abstain | pass-rate |
|-----------|------|------|---------|-----------|
| 0.40 | 222 | 5  | 0 | 97.80% |
| 0.41 | 222 | 5  | 0 | 97.80% |
| 0.42 | 222 | 5  | 0 | 97.80% |
| 0.43 | 222 | 5  | 0 | 97.80% |
| 0.44 | 222 | 5  | 0 | 97.80% |
| 0.45 | 221 | 6  | 0 | 97.36% |
| 0.46 | 220 | 7  | 0 | 96.92% |
| 0.47 | 220 | 7  | 0 | 96.92% |
| 0.48 | 219 | 8  | 0 | 96.48% |
| 0.49 | 219 | 8  | 0 | 96.48% |
| **0.50** | **213** | **14** | **0** | **93.83%** |
| 0.51 | 211 | 16 | 0 | 92.95% |
| 0.52 | 209 | 18 | 0 | 92.07% |
| 0.53 | 202 | 25 | 0 | 88.99% |
| 0.54 | 198 | 29 | 0 | 87.22% |
| 0.55 | 189 | 38 | 0 | 83.26% |
| 0.56 | 184 | 43 | 0 | 81.06% |
| 0.57 | 175 | 52 | 0 | 77.09% |
| 0.58 | 158 | 69 | 0 | 69.60% |
| 0.59 | 145 | 82 | 0 | 63.88% |
| 0.60 | 138 | 89 | 0 | 60.79% |
| 0.61 | 124 | 103 | 0 | 54.63% |
| 0.62 | 104 | 123 | 0 | 45.81% |
| 0.63 |  89 | 138 | 0 | 39.21% |
| 0.64 |  75 | 152 | 0 | 33.04% |
| 0.65 |  60 | 167 | 0 | 26.43% |
| 0.66 |  48 | 179 | 0 | 21.15% |
| 0.67 |  39 | 188 | 0 | 17.18% |
| 0.68 |  34 | 193 | 0 | 14.98% |
| 0.69 |  23 | 204 | 0 | 10.13% |
| 0.70 |  20 | 207 | 0 |  8.81% |
| 0.71 |  14 | 213 | 0 |  6.17% |
| 0.72 |   7 | 220 | 0 |  3.08% |
| 0.73 |   4 | 223 | 0 |  1.76% |
| 0.74 |   3 | 224 | 0 |  1.32% |
| 0.75 |   2 | 225 | 0 |  0.88% |
| 0.76 — 0.99 |   0 | 227 | 0 |  0.00% |

The bolded `0.50` row is the locked threshold (see §4).

The pass count is monotonically non-increasing across rising
thresholds (the sweep tests assert this invariant on every row).
The `pass + fail + abstain == total_entries` partition invariant
is asserted at every row by the sweep tests. Beyond `0.76`, no
entry has `score >= threshold` because the empirical maximum
paraphrase score is `0.757522`.

## 4. Locked threshold

| Locked value | 0.5 |
|--------------|-----|
| Paraphrase pass-rate | 213 / 222 = 95.95% |
| Negative-fixture pass-rate | 0 / 5 = 0% (all 5 negatives fail by construction) |
| Abstain rate | 0 / 227 = 0% (B1 in-cache score sweep cannot emit abstain; reserved for B2) |
| Distance from paraphrase mean | mean − threshold = 0.114603 (≈ 1.92 σ below mean) |

### 4.1 Rationale

The locked value is `0.5`, and the choice criterion is **"keep
the cache's verdict semantics permissive but discriminating":**

1. All five forced-`0.0` negative fixtures fail at any positive
   threshold, so the negative side of the verdict surface is
   saturated for any reasonable choice. The locked value cannot
   meaningfully change negative-fixture behaviour.
2. The empirical paraphrase score distribution is unimodal
   around `0.62` with `σ ≈ 0.06`. Choosing the threshold
   substantially below the mean preserves a high paraphrase
   pass-rate (213 of 222 = 95.95% at `0.5`).
3. The sweep table shows no sharp knee in the pass count: the
   curve descends gradually from `97.80%` at `0.40` to `60.79%`
   at `0.60`, then steepens. There is no natural cut-point at
   which the distribution discontinuously segregates "truthful"
   from "untruthful" paraphrases — the model's score is a
   gradient, not a binary label.
4. `0.5` is exactly the midpoint of the rescaled cosine domain
   `[0, 1]`; it corresponds to raw cosine `0.0` (an
   uncorrelated chunk/claim pair). Any paraphrase scoring above
   `0.5` is at least weakly correlated with the chunk text in the
   model's embedding space, which is the minimum useful
   discriminant.
5. `0.5` is a clean integer multiple of `0.01` that round-trips
   through `serde_json` without precision drift (the sweep tests
   assert this), so the locked value can safely be cross-checked
   by the Rust audit.

The choice is conservative: under-fitting toward "permissive"
rather than over-fitting to a hand-picked knee. The cache's role
is to flag low-similarity verdict surfaces to the runtime; the
runtime composes Layer-3 verdicts with Layer-1/Layer-2 results
upstream, so a permissive Layer-3 contract favours observability
over strict gating.

### 4.2 Aspirational pass-rate note

This verifier is **measurement, not a merge gate.** A low MiniLM
pass-rate is an accepted outcome, not a release blocker. The
sweep produces empirical data; the threshold choice is a
documented policy decision, not a proof of correctness.

In particular:

- The locked value is NOT chosen to maximize any specific
  pass-rate target. There is no claim that `95.95%` is the
  "right" pass-rate for the QM corpus.
- A future revision may relax or tighten the locked value if
  empirical evidence accumulates (e.g., per-card retract events
  flag systematic under- or over-firing). Such a revision is a
  maintenance event (see §5) accompanied by a new iteration of
  this document.

## 5. Where the locked value lives

The locked value `0.5` is mirrored at two active surfaces. Drift fails either
the provenance audit or the threshold-sweep tests.

| Surface | Symbol | Drift detector |
|---------|--------|----------------|
| `xtask/src/semantic_cache_provenance.rs` | `pub const EXPECTED_THRESHOLD: f64 = 0.5;` | `cargo run -p xtask -- audit-semantic-cache-provenance` fails if `provenance.threshold != EXPECTED_THRESHOLD`. The unit test `wrong_threshold_fails_with_documented_locked_value` covers the fail mode. |
| `xtask/src/threshold_sweep.rs` (test only) | `locked_threshold_lies_in_default_sweep_range` | Compile-time + test assertion that `EXPECTED_THRESHOLD ∈ [DEFAULT_FROM, DEFAULT_TO]` and lands on a clean step boundary. |

### 5.1 Additional drift detectors enforced by the sweep preflight

`cargo run -p xtask -- threshold-sweep --vertical qm` now reads the
committed provenance sidecar (default
`out/semantic_cache.provenance.json`) before emitting any rows
and asserts the frozen-count contract. A cache whose committed
counts disagree with the frozen QM label set fails preflight
with no sweep output.

| Preflight assertion | Diagnostic on failure |
|---------------------|-----------------------|
| `provenance.schema_version == "cacg.v0"` | "provenance schema_version mismatch: expected …, got …" |
| `provenance.paraphrase_count == 222` (frozen QM label set) | "non-frozen QM label set: provenance.paraphrase_count must be 222; got N" — closes AC-7's negative gate (sweep on non-frozen label set MUST fail). |
| `provenance.negative_fixture_count ∈ [5, 10]` | "provenance.negative_fixture_count must be in [5, 10]; got N" |
| `provenance.entry_count == paraphrase_count + negative_fixture_count` | "provenance.entry_count must equal paraphrase_count + negative_fixture_count = E; got N" |
| `cache.entries.len() == provenance.entry_count` | "cache.entries.len() = M does not match provenance.entry_count = N" |

## 6. Regeneration pointer

To relock at a different threshold value:

1. Update the active threshold surfaces in one commit:
   - `xtask/src/semantic_cache_provenance.rs::EXPECTED_THRESHOLD`
   - The §4.1 / §4.2 rationale paragraphs in this document.
2. Regenerate the cache through a maintained builder path. The old Python
   builder has been retired; `_research/20_b1_cache_provisioning.md` records
   the current frozen-cache policy.
3. Commit the regenerated `out/semantic_cache.json` and
   `out/semantic_cache.provenance.json` alongside the threshold edits.
4. Re-run the sweep + regenerate the §3 table in this document:

   ```bash
   cargo run -p xtask -- threshold-sweep --vertical qm
   ```

5. Verify the audit + integration tests:

   ```bash
   cargo run -p xtask -- audit-semantic-cache-provenance
   cargo test -p cacg-semantic --test committed_cache
   ```

## 7. Reproducibility evidence

Empirical byte-equal idempotence of the cache rebuild at the locked threshold
was verified before the builder was retired:

| Run | Output cache SHA-256 |
|-----|----------------------|
| Committed | `a905e42224046ae2e4f617902c110986bb62488cad9c85fa521b832f448ee09c` |
| Rebuild #1 (temp-out) | `a905e42224046ae2e4f617902c110986bb62488cad9c85fa521b832f448ee09c` |
| Rebuild #2 (temp-out) | `a905e42224046ae2e4f617902c110986bb62488cad9c85fa521b832f448ee09c` |
| Rebuild #3 (in-place at `out/`) | `a905e42224046ae2e4f617902c110986bb62488cad9c85fa521b832f448ee09c` |

The provenance JSON byte-content depends on the resolved `cache_path` field.
The committed files are now treated as frozen fixtures unless a deliberate
regeneration task replaces the retired builder.

The audit reports the verified count contract
(`227 entries clean (222 paraphrase + 5 negative)`) and matching
two-hash commitments
(`hash_b=57e997076ac8..., hash_c=a905e4222404...`).
