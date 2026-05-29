//! CACG search backends.
//!
//! [`summaries`] is the in-memory BM25 lexical search over `summaries.json`
//! — reusing `cacg-core`'s `Bm25Okapi`. [`sqlite`] is the persistent
//! `SQLite` `FTS5` `summaries.sqlite` sidecar: a builder plus a sealed
//! read-only query backend that `kb search` probes first, falling back
//! to the in-memory backend on a stale/absent/unreadable sidecar.
//! `rusqlite` is confined to this crate.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

pub mod sqlite;
pub mod summaries;

pub use sqlite::{
    build_sidecar, compute_summaries_hash, fts5_available, SidecarBuildError, SidecarSeal,
    SidecarStaleError, SummariesSqliteIndex, SIDECAR_FILENAME,
};
pub use summaries::{validate_manifest, SearchHit, SummariesIndex, SummariesLoadError};

/// The `cacg-search` crate version string.
#[must_use]
pub fn cacg_search_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::cacg_search_version;

    #[test]
    fn version_is_nonempty() {
        assert!(!cacg_search_version().is_empty());
    }
}
