AGREE: Structured top plan has AC/task bidirectional coverage: no uncovered AC; no task targets non-AC/FUT/DEC. AC-6 free grounding and AC-7 capped practitioner subset are valid current-scope gates, not deferral leaks. DEC<->FUT linkage is complete for DEC-3..DEC-7.

DISAGREE: Not fully converged as written: AC-2 still tests two renders of `corpus_full.md`, contradicting the new complete-corpus ingest source.

REQUIRED_CHANGES: Fix AC-2 Positive Tests to render `corpus_complete.md`, not `corpus_full.md`; require adjacency/mapping to stable `post_id`/`comment_id` as applicable. Tighten task10 to explicitly gitignore `corpus_complete.md` and `batches/`, not only `corpus_full.md` + corpus PDF.

OPTIONAL_IMPROVEMENTS: Add an omission ledger for FUT-2 candidates explaining why each non-v1 candidate is deferred/excluded. Add page/chunk-count and runtime sanity checks for the full 266-post/6704-comment PDF ingest.

UNRESOLVED: none
