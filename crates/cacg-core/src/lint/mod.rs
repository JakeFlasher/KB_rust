//! Layer-1 lint surface for the verify hot path.
//!
//! Mirrors Python `legacy_python_oracle/src/cacg/lint/` package layout: the trust-bearing
//! mechanical checks live in [`layer1`]; CLI dispatch + journal append
//! live one layer up in `cacg-cli`.
//!
//! Auxiliary lint codes (`CACG-SUM-*`, `CACG-SKILL-*`, `CACG-DEP-*`,
//! `CACG-ROLE-*`) are intentionally out of scope for this surface. They
//! continue to be emitted by the M1 schema / frontmatter modules
//! (`crates/cacg-core/src/schema.rs`, `crates/cacg-core/src/frontmatter.rs`)
//! on their own parity surface; the lint-pass surface here covers only
//! the trust-bearing subset `CACG-FM-*`, `CACG-CITE-*`, `CACG-HASH-*`,
//! `CACG-MAN-*`, `CACG-CLI-001`, `CACG-AUTH-*`, `CACG-RETR-*`,
//! `CACG-JNL-001`.

pub mod layer1;
