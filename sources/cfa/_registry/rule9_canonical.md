# Critical Rule 9 — canonical pin (reproducible governance source-of-record)

This file is the **tracked, in-repo source-of-record** for Critical Rule 9 of the
CFA-legacy knowledge base. `check_quarantine_invariant.py` (the notes-taint quarantine
invariant run by `run_corpus_gate.sh`) validates Rule 9 against this committed pin so the
gate is **reproducible on any checkout / CI box** and does NOT depend on the author's
local legacy sibling at `/home/jakeshea/CFA_reading/CLAUDE.md`.

Provenance: the block between the `CANONICAL-RULE9` markers below is copied **verbatim**
from the legacy `CLAUDE.md` "Critical Rules (Always Apply)" section, rule 9 (source lines
71–85). The sha256 of that verbatim block is
`4e490cec58bd6a489798a7962c80583376ad7bde677de2b8ae2ddc18a6dcaf90`. When the legacy sibling is present, the gate additionally cross-checks the live
source against the same key clauses to detect upstream drift; when it is absent, only
this pin is enforced.

Only the text between the `CANONICAL-RULE9` markers is validated, so this prose header
cannot mask a weakening of the rule itself.

<!-- CANONICAL-RULE9-BEGIN -->
9. **User-volatile folders hard-block.** No card, volume draft, SKILL,
   `_source_role_map.md`, `_chapter_overviews.md`, `_dependency_order.md`,
   `_style_guide.md`, `INDEX.md`, `STATUS.md`, or other deliverable doc
   may cite `notes/` or `scripts/` paths as a `Primary raw source:`,
   `Supporting sources:`, or `**Source:**` reference. These folders hold
   user-volatile content (`notes/` is hand-authored study material;
   `scripts/` is project tooling) lacking the stable provenance required
   by Critical Rule 2. `scripts/kb/*.py` MAY still be **referenced as
   tooling infrastructure** in non-`**Source:**` prose (e.g., "run
   `scripts/kb/lint_cards.py` to validate"). When a topic is only covered
   by a `notes/` page, either locate an admitted alternative in
   `_corpus_planning/05_source_matrix.md` or mark the claim
   `[UNCITED — verify]` per Critical Rule 4 and defer the card. Lint
   enforcement: `scripts/kb/lint_cards.py` NOTES-001/002 + SCRIPTS-001/002
   + `scripts/kb/build_manifest.py` `_validate_rows` mirror.
<!-- CANONICAL-RULE9-END -->
