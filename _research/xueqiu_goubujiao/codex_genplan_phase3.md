CORE_RISKS:
- Synthetic-PDF trust gap: `kb verify` will prove quotes exist in the rendered Xueqiu PDF, not that the PDF faithfully represents original Xueqiu posts. Need a corpus→PDF parity proof, not only PDF→chunks.
- CJK extraction remains existential. Chromium layout, font fallback, PDF text ordering, punctuation normalization, and hidden controls can all make exact containment green in spike but drift later.
- PDF byte determinism is under-specified. `KB_FROZEN_CLOCK` freezes CACG artifacts, not necessarily Chromium PDF metadata, font rendering, or source SHA. Regenerating the Xueqiu PDF may change `source_sha256` and all chunk hashes.
- Faithfulness target is aggressive: only 21/73 are faithful. A ~30 practitioner-card v1 depends heavily on corrected summaries and disciplined exclusion; otherwise the deck will launder overgeneralized forum remarks into doctrine.
- Time-sensitive HK facts can silently age. Snapshotting solves citation stability, not current correctness; cards must distinguish “official as-of snapshot” from “current rule.”
- Licensing/privacy risk is not peripheral. Committing `corpus_full.md` plus a full rendered public-post PDF is a distribution decision, not just an ingest detail.
- Deck isolation is brittle if scripts or gates assume `cfa`. The plan says “touch no CFA gates,” but export/gate code and global semantic artifacts may still have CFA-shaped assumptions.

MISSING_REQUIREMENTS:
- Explicit source-of-truth chain for Xueqiu: original capture artifact, batch manifest, post IDs, author/reply boundaries, and deterministic corpus renderer input hash.
- A policy for whether practitioner cards may cite only Xueqiu, Xueqiu + grounding, or must cite both for operational claims.
- A rule for stale/current official facts: required `as-of` language, snapshot date in title/body/tag, and when a card must be retracted or refreshed.
- A legal/redistribution decision for the corpus PDF and public-post quotes before committing artifacts.
- Stable source-id convention for all HKEX/IRD/IFEC/Bennett sources; avoid URL-derived or title-derived churn.
- Minimum/maximum quote length and uniqueness rules for Chinese quotes, including duplicates across comments and repeated slogans.
- A cross-deck See Also validator. Body prose references to `cfa` ids are otherwise unchecked and can rot.
- A card-selection rubric for reducing 46 authorable candidates to ~30 without bias toward flashy tactical claims.
- Required treatment for comments with `(to @user:"…")`: author reply may be usable, parent-context text is not; the draft’s blanket wording risks rejecting valid author replies.
- Procedure for HTML sources: since ingest is PDF-only, every HTML snapshot needs a deterministic print-to-PDF recipe and hash record.
- Export target/tier requirements. `--dry-run` alone is weak; verify-tier export should be exercised at least once.
- Decision on `out/semantic_cache`: current export copies global cache by default, likely CFA-biased unless disabled or regenerated per deck.

TECHNICAL_GAPS:
- `kb ingest` refuses to publish into an existing manifest directory. Phase A’s “ingest each into `out/hkex/`” will fail unless the plan uses per-source `out/hkex/ingest_per_source/<source_id>/` plus a deterministic merge helper.
- The draft needs a hkex-specific merge gate: validate one source per per-source manifest, recompute source SHA, detect duplicate `source_id`/`chunk_id`, sort canonically, and atomically write merged `sources_manifest.json`/`chunks_manifest.json`.
- `--config` works in the current dispatcher, but CLI help claims it is not implemented. Add a smoke test for `max_pages_per_chunk: 1`; do not rely on prose.
- CJK chunking assumption is incomplete: whitespace tokenization is inert for CJK, but paragraph/page extraction order and page text concatenation can still produce multi-utterance chunks and duplicate quote matches.
- “100% single-chunk containment” is necessary but insufficient. Need exact quote round-trip from `corpus_full.md` normalized text to PDF extracted text, including post_id adjacency.
- Renderer determinism must be pinned: Chromium version, command flags, locale, fonts, CSS, page size, margins, and generated PDF hash.
- Hand-authored `source_matrix` needs its own validator. It should fail unknown reading ids, unauthorized citations, unused source ids, overly broad Xueqiu authorization, and missing source ids from merged manifest.
- The plan authorizes the Xueqiu source “for every practitioner reading_id,” which is convenient but weakens matrix intent. Prefer explicit per-reading authorization generated from candidate inventory.
- `kb lint --all-readings` defaults to `cards`; hkex gates must pass `--cards-dir cards/hkex`, `--chunks-manifest out/hkex/chunks_manifest.json`, and `--source-matrix out/hkex/source_matrix.json`.
- Export requires `out/hkex/pdfium_provenance.json`, `sources_manifest.json`, `cards_manifest.json`, `summaries.json`, `INDEX.md`, and probably `summaries.sqlite` handling. The draft does not define how all are produced for a new deck.
- Export script comments and optional semantic cache are CFA-shaped. Use `--no-semantic` until hkex has its own cache/provenance.
- Determinism gate should compare byte hashes across two clean rebuilds: render PDF, ingest per-source, merge manifests, index cards, export receipt. Current AC only says “byte-reproducible” at index.
- `card_hash` stamping rewrites cards and histories. The plan should separate pre-index authored cards from post-index committed state and check no stale hashes remain.

ALTERNATIVE_DIRECTIONS:
- Minimal official-HK deck first: ship only Phase A plus 8-12 tightly grounded practitioner cards. Lower faithfulness and CJK blast radius; less complete distillation.
- Xueqiu as appendix source only: grounding cards cite official PDFs; practitioner cards quote Xueqiu sparingly as examples. More authoritative; less faithful to “distill the investor.”
- One-utterance-per-page Xueqiu PDF: maximizes containment and attribution clarity; produces many pages and larger manifests.
- One-post-per-page PDF with resolver requiring post_id + quote: smaller and easier to browse; higher risk that comments/long posts straddle chunks or duplicate quotes.
- Non-PDF manifest generator for Xueqiu chunks: deterministic and text-native; violates current PDF-only contract unless treated as a kernel/tooling change.
- Defer commercial theory entirely: focus on HKEX/IRD/IFEC/Bennett + existing CFA crosslinks. Fast and legally clean; leaves covered-call/CSP workflow less externally grounded.
- Split decks: `hkex-grounding` and `gbj-practitioner`. Stronger provenance boundaries; more export/query complexity.

QUESTIONS_FOR_USER:
- Are you comfortable committing the full rendered Xueqiu corpus PDF and `corpus_full.md`, or should only card-level excerpts be committed?
- Should practitioner operational claims require both Xueqiu evidence and an official/grounding citation, or may some remain explicitly Xueqiu-only?
- Is v1 allowed to be smaller than ~45 if the faithfulness gate leaves fewer high-confidence cards?
- Should HK official snapshots represent current rules as of ingest date, or only historical grounding for a 2022-H1 practitioner corpus?
- Do you want a deck-local helper layer copied from CFA registry patterns, or should any missing merge/scaffold functionality be implemented in Rust `kb` first?
- Should export include semantic cache, or should hkex initially export with `--no-semantic`?

CANDIDATE_CRITERIA:
- `cargo build --workspace` passes.
- `kb ingest --config <hkex-cjk.yaml>` smoke test proves `max_pages_per_chunk: 1` is honored.
- Xueqiu renderer gate: two clean renders produce identical PDF SHA-256.
- Corpus parity gate: rendered PDF extracted text contains 100% of selected corpus quote substrings after CACG normalization and maps each to the expected `post_id`.
- CJK containment gate: all 73 candidates plus edge cases resolve to exactly one active chunk; zero duplicate/zero/cross-chunk binds.
- Control-char gate: merged `chunks_manifest.json` contains zero disallowed controls, STX, U+FFFE, replacement chars, or unexpected private-use artifacts.
- Per-source ingest gate: every source ingests into fresh per-source output; merge helper validates source SHA and chunk hash recomputation; duplicate ids fail.
- Source-matrix gate: every active card citation is authorized; every matrix source exists; no unused or overbroad reading authorization without allowlist justification.
- Faithfulness gate: every practitioner card maps to `faithful` or exact `corrected_summary`; weak/misattributed ids are absent.
- Attribution gate: no citation quote includes `//@`; no quote uses parenthetical third-party text from `(to @...)`; author reply text remains allowed only when parser proves ownership.
- Risk-spine gate: every options/overlay card contains required no-leverage/full-cash/full-underlying/ordinary-investor warning text.
- Dating gate: every practitioner card has `xueqiu-2022h1`; tactical cards have `dated-levels` and first body section `## Dated State`.
- Lint/verify gate: `kb lint --all-readings --cards-dir cards/hkex --chunks-manifest out/hkex/chunks_manifest.json --source-matrix out/hkex/source_matrix.json` exits 0, and every card verifies exactly without fuzzy.
- Index reproducibility gate: two `KB_FROZEN_CLOCK=1 kb index cards/hkex --out <tmp>` runs produce byte-identical `cards_manifest.json`, `summaries.json`, and `INDEX.md`.
- Export gate: `scripts/export-knowledge.sh <tmp-target> --deck hkex --tier verify --no-semantic --force` publishes and its receipt hash check passes.
- Isolation gate: `git diff --exit-code -- cards/cfa out/cfa sources/cfa` passes and CFA count gates remain at 500 active / 268 baseline.
