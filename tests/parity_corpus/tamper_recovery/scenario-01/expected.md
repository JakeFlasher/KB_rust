# tamper_recovery scenario

Input: `input/lint_journal.jsonl` is a 2-event journal built by `cacg.journal.append_entry` under `KB_FROZEN_CLOCK=1`. Line 2's `event_checksum` field is mutated post-write to a wrong value. `cacg.journal.validate_jsonl` returns the bad-line index list in `expected.json`. The Rust validator (cacg_core::journal::validate_jsonl) MUST return the same list.
