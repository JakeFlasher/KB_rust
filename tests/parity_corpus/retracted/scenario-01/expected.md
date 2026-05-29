# Retracted scenario 01

After `kb retract input/retracted-card-01.md`, the expected outputs are:

- A new event appended to `<card>.history.jsonl` with `is_retracted: true`
  and the `__cacg_retracted__` marker in `frontmatter_field_changes`.
- `cards_manifest.json` MUST move the card_id into `retracted_cards` (sorted, unique).
- `summaries.json` MUST exclude the retracted card.
- A subsequent `kb verify input/retracted-card-01.md` exits 1 with primary code
  `CACG-RETR-001` (severity error).
- With `--allow-retracted`, exit code stays 0 but severity downgrades to warning.
