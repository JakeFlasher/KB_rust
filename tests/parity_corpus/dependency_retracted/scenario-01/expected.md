# Dependency-retracted cascade scenario

Input cards: `input/cascade-parent-retracted.md` (parent) and `input/cascade-child.md` (child).

After `kb retract-source <source-id> --cards-dir cards/`:

- `cards_manifest.retracted_cards` contains every card sourcing the retracted source.
- `cards_manifest.dependency_retracted_cards` contains every card transitively depending on a retracted card via `card_edges`.

The `kb show` presentation contract (AC-S4):

- `kb show cascade-child --allow-retracted --source-matrix m.json` prints a `STATUS: DEPENDENCY-RETRACTED` line followed by the documented card view and exits 0.
- `kb show cascade-child --source-matrix m.json` (without `--allow-retracted`) exits 1 with `CACG-SHOW-001`.

**Oracle scope (M0):** The committed Python-built oracle artifacts under `out_python/dependency_retracted/scenario-01/` cover the full scenario: `expected.json` (the post-`kb retract-source` cascade result), `cards_manifest.json` (the live manifest snapshot), `show_default.json` (default-mode `kb show` refusal with `CACG-SHOW-001`), and `show_allow_retracted.json` (the `--allow-retracted` downgrade with the documented status line). The Rust port's M5 task-M5-4 will be byte-compared against the show oracles when it lands. PE-04 (`_research/09_dec_proposed_defaults.md`) records the minimum-pull-forward rationale.
