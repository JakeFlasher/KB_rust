# truncated_journal scenario

Input: `input/lint_journal.jsonl` ends mid-write on line 2 (the JSON object is incomplete; closing brace + newline are removed). Line 1 remains a valid Python `append_entry` output. `cacg.journal.validate_jsonl` returns the bad-line index list in `expected.json`. The Rust validator MUST return the same list. Validators fail closed on truncated journals; recovery is operator-driven.
