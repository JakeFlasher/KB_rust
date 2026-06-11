# Expert-distillation runbook

How to turn a crawled human expert's conversational corpus into a verified
knowledge deck. The hkex/狗不叫 deck (uid 2424206371) is the reference
execution; every step below names the committed tool that performs it.
Expert-specific values live in per-deck CONFIG files — the checker and
builder logic is expert-agnostic.

## 0. Inputs: the corpus-exchange contract

The crawler repo (`xueqiu_anti_crawler_goubujiao`) and this repo stay
separate; `output/<uid>/` is the interface:

- `posts.json` — list of posts; each with stable `id`, `created_at`(+iso),
  engagement counts, `text` (single-line plain text), `url`, and `comments`:
  flat list with stable `id`, `created_at_iso`, `text`, `user{id,
  screen_name}`, `reply_to_id`/`reply_to`/`reply_to_text` threading pointers.
- `meta.json` — the crawl's own completeness attestation (`complete`,
  `run_status`, timeline population/fetched/shortfall, per-post caps).

The consumer NEVER assumes completeness: `build_corpus_from_capture.py`
refuses an attested-incomplete capture unless `--allow-incomplete` is
passed, and either way copies the full shortfall accounting into the
committed `corpus_provenance.json`.

## 1. Pipeline (per expert)

| Step | Tool (committed) | Gate |
|---|---|---|
| Capture → corpus | `build_corpus_from_capture.py --write` (parametric: `--target-uid/--author-uid/--author-name`) | `--check`: rebuild == corpus == provenance; deterministic |
| Corpus → utterances | `export_utterances.py --write` (`cacg.utterances.v1` JSONL; empty utterances skipped + counted) | self-test |
| Ingest | `kb ingest --format utterances` (no pdfium; sealed `locator_map.json`) — the hkex pilot used the legacy PDF route (`render_corpus_pdf.py` + parity + spike), kept for its committed gates | locator seal; chunk==utterance |
| Refresh | re-run builder + `kb ingest --append` | every prior chunk re-derives byte-identical or hard-fail |
| Mining | agent fan-out over the corpus → `card_inventory.json` (candidates with `citation_seeds[{post_id, comment_id?, quote_zh}]`) | seeds must bind: spike resolver (next row) |
| Seed binding | `check_cjk_ingest_spike.py` resolver (`resolve_seed`): au=1 only, `//@` repost spans rejected, exactly-one chunk, lengthen-or-fail, reviewed `seed_overrides.json` | zero ambiguous / zero unprincipled rejections |
| Verdicts | adversarial judge per candidate (one agent each; every verdict CHANGE independently refuted) → `verifications_refresh.json` with `{verdict, issues, corrected_summary, recurrence_distinct(+evidence), new_context_notes}` | conservative default: keep prior verdict unless evidence cited by id |
| Authoring | spec-driven: `practitioner_cards.json` → `emit_practitioner_cards.py --emit` (all-or-nothing; thesis must equal the verdict summary verbatim) | emit PASS |
| Deck QA | `faithfulness_lint.py --check` with the deck's `faithfulness_config.json` (G1 verdict-link, G2 attribution re-resolution under the same pins+overrides, G3 risk-spine, G4 recurrence honesty, G5 dating, G6 naming, G7 dual-cite) | PASS |
| Kernel | `kb lint --all-readings`, per-card `kb verify`, `validate_source_matrix.py`, `KB_FROZEN_CLOCK=1 kb index` ×2 byte-identical, `export-knowledge.sh --deck <d> --tier verify` | all green |
| Accounting | omission ledger: every candidate AUTHORED / DEFERRED / EXCLUDED with reasons | committed |

Per-deck CONFIG surfaces (what expert #2 supplies instead of code):
`faithfulness_config.json` (author tag, verifications path, risk-spine
phrases, mandatory/dated tags, naming bans, excluded titles, Xueqiu-only
flag), builder flags (uid/author name), `source_matrix.json`,
`seed_overrides.json`, `practitioner_cards.json`.

Known per-expert code residue (generalize when expert #2 starts):
`corpus_model.py` hard-pins the ★AUTHOR tag and corpus path;
`check_cjk_ingest_spike.py` hard-pins the corpus source id, inventory path,
and the author's `//@`-handle list. Both are single-file, single-purpose
parsers — parameterize, don't fork.

## 2. Stratification policy for large corpora (expert #2: uid 9650668145)

The next expert is ~37,537 posts (141× the pilot). Exhaustive mining is not
the bar — ACCOUNTED coverage is. Policy:

1. **Never silently truncate.** Every stratum that is not mined is named in
   the deck's omission ledger with its size. "Covered everything" may only
   be claimed when the ledger says so.
2. **Mine in attested strata, author-first:**
   - S1: all author POST bodies (the expert's own long-form) — always 100%.
   - S2: all author replies in threads the expert started.
   - S3: author replies elsewhere, ranked by engagement (likes/replies),
     top-N with N recorded.
   - S4: commenter context — never mined for claims, loaded only as thread
     context for S1–S3 candidates (the pilot's lesson: context changes
     verdicts; 2 of 73 flipped on full nesting).
3. **Candidate budget per round, loop-until-dry:** mine S1→S3 in rounds;
   stop when a round yields no new candidate that survives verdict judging
   (not when a count is hit).
4. **The capture's `meta.json` shortfall is part of the deck's provenance**
   — an incomplete crawl caps the claims the deck may make, and the corpus
   header must say "attested INCOMPLETE", never "complete".
5. **Scale knobs already proven:** `kb ingest --format utterances` ingested
   7,383 utterances in 0.57s; the same path is linear and holds at 10⁶
   utterances. The synthetic-PDF route must NOT be used at this scale.

## 3. Kernel-promotion ledger (deck Python → kernel, when triggers fire)

| Gate (today, deck-local) | Kernel home (trigger) |
|---|---|
| Author-origin + exactly-one binding (`resolve_seed`) | `kb verify` speaker policy `CACG-CITE-007` — with cacg.v1 anchors (see `docs/cacg-v1-design.md`); trigger: first v1 deck |
| Reviewed override format (`seed_overrides.json`) | kernel-recognized override file consumed by verify; same trigger |
| Faithfulness QA (G1–G7) | deck-pluggable `kb lint` pass driven by per-deck config; trigger: expert #2 deck adopts the linter unchanged → promote |
| Per-source merge + matrix scaffold (deck `merge_hkex_manifests.py`) | `kb` subcommands; trigger unchanged from FUT-4 (third deck) |

The mission change (conversation distillation as primary) fired the
PLANNING trigger; the implementation triggers above are deliberately tied
to the first consumer that needs each promotion, so no kernel surface ships
without a real user.
