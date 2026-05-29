# concurrent_journal scenario

Input: `input/lint_journal.jsonl` simulates a racing-writer corruption. Line 1 is a valid Python `append_entry` output (seq=0); line 2 is a copy of line 1 (still seq=0; should have been seq=1). The chain's monotonic-seq invariant is broken. `cacg.journal.validate_jsonl` returns the bad-line index list in `expected.json`. Production deploys must use `cacg_core::journal::flock` around the validate+write window so this race is structurally impossible.
