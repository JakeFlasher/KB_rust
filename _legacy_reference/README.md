# Legacy CFA KB — Reference Bundle

This tree holds **scrubbed copies** of legacy CFA KB artifacts that the cacg.v0
migration consults for context but does NOT migrate verbatim. Modifications here
are deliberate cleanups (e.g. notes-taint scrub per Critical Rule 9); the
upstream legacy KB at `/home/jakeshea/CFA_reading/CFA_reading/` is **never**
modified by this workspace.

## Tree

```
_legacy_reference/
  cfa_legacy/
    cards/
      17_cross_cutting/
        cc-material-info-and-dissemination-delay.md   # scrubbed (7 alias citations removed)
        _chapter_overviews.md                          # scrubbed (1 alias citation removed, L222)
        _style_guide.md                                # scrubbed (alias-citation policy escape hatch removed, L151 block)
      05_equity/
        _source_role_map.md                            # scrubbed (notes-derived `primary-cfa` stance admission removed, L53 block)
```

## Why these files were scrubbed

Per `_research/27_independent_legacy_cfa_audit_and_bootstrap_plan.md` §3.5 +
§Phase 6 and `_research/28_legacy_cfa_bootstrap_verification_and_refined_plan.md`
§2.5 + §4.2, the legacy KB's `NOTES-001/002` lint rules check only `notes/`
path prefixes. Several legacy citations bypass that check by using the bare
alias form `CFA_note_<N>` followed by an OCR-date parenthetical and a
`pp.<P-Q>` page span — same content as the path-prefixed citation, but no
`notes/` path prefix.

The 4 files staged here contain the only known occurrences of that pattern in
the legacy active-card surface (8 total citations) plus the 2 policy files
that explicitly admit the pattern. All occurrences have been removed; the
scrubbed copies are byte-different from their legacy originals.

## Defense-in-depth against bare-alias citations

The cacg.v0 framework's structural defenses already prevent this pattern from
being a load-bearing citation:

1. `source_id` regex `^[a-z0-9][a-z0-9_]*$` (see `crates/cacg-core/src/schema.rs`)
   structurally rejects `CFA_note_N` in any structured `source_id:` field
   because the regex disallows uppercase letters.
2. cacg.v0 cards use structured `citations:` arrays (typed `source_id` +
   `chunk_id` + `chunk_hash` + `quote` + `page_range`) rather than free-form
   body `**Source:**` markers, so the alias cannot serve as a citation.

The bootstrap adds two more layers:

3. `sources/cfa_legacy/excluded/legacy_notes_taint_manifest.json` uses the
   substring marker `CFA_note` to quarantine all 8 alias-bearing legacy cards
   from the migration queue.
4. `sources/cfa_legacy/_registry/validate_no_alias_residue.py` is a
   pipeline-level pre-emit check that scans `_legacy_reference/`,
   `cards/cfa_legacy/`, and `sources/cfa_legacy/_registry/page_coordinate_maps/`
   for the regex `\bCFA_note_\d+\s*\(`. Exit non-zero on any hit.

A CFA-specific lint rule is NOT added to `crates/cacg-core/src/lint/`. That
surface is intentionally framework-neutral; per its module docstring it carries
only the trust-bearing mechanical checks (`CACG-FM-*`, `CACG-CITE-*`,
`CACG-HASH-*`, `CACG-MAN-*`, `CACG-CLI-001`, `CACG-AUTH-*`, `CACG-RETR-*`,
`CACG-JNL-001`). The legacy bare-alias pattern is a CFA-domain content issue,
not a framework-generic structural issue, so the right architectural layer is
the migration pipeline.

## When these scrubbed copies become authoritative

When the corresponding subcorpora migrate (17_cross_cutting after the L1
combined-volume offset table is built; 05_equity in its own slice), the cacg.v0
emitter SHOULD read from these scrubbed `_legacy_reference/` copies, not from
the legacy KB tree. The scrubbed copies preserve the content, identity, and
citation intent of the originals while removing all user-volatile-source
references.
