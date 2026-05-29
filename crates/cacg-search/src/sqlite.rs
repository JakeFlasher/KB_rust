//! The `SQLite` `FTS5` `summaries.sqlite` search sidecar.
//!
//! Mirrors Python `legacy_python_oracle/src/cacg/search_sqlite.py`. The sidecar is a
//! performance artifact: query results, the `meta` seal, and the
//! fallback behaviour are parity-bearing; the raw `SQLite` bytes are not
//! (page allocation / freelist / B-tree splits make them vary), so they
//! are excluded from the `KB_FROZEN_CLOCK=1` byte-determinism contract.
//! `rusqlite` is confined to this crate.
//!
//! [`build_sidecar`] writes `summaries.sqlite` atomically (tmp +
//! rename, `journal_mode=DELETE` so no WAL/SHM files leak).
//! [`SummariesSqliteIndex`] opens it read-only, verifies the
//! `meta.summaries_hash` seal against the live `summaries.json`, and
//! serves `MATCH` queries — or raises [`SidecarStaleError`]
//! (`CACG-FTS-001`) so the caller falls back to the in-memory backend.

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use cacg_core::index::SummariesManifest;
use cacg_core::normalize::tokenize_for_lookup;
use cacg_core::schema::SourceMatrix;
use rusqlite::{Connection, OpenFlags};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::summaries::{round6, SearchHit};

/// Canonical sidecar filename, a sibling of `summaries.json`.
pub const SIDECAR_FILENAME: &str = "summaries.sqlite";

/// Sidecar schema version sealed into `meta`; a reader rejects any
/// other value with `CACG-FTS-001`. Mirrors Python `SIDECAR_SCHEMA_VERSION`.
const SIDECAR_SCHEMA_VERSION: &str = "cacg.v0.fts1";

/// Builder version sealed into `meta`. Mirrors Python `BUILDER_VERSION`.
const BUILDER_VERSION: &str = "cacg.fts5.builder.v1";

/// FTS5 tokenizer directive. Mirrors Python `TOKENIZE`.
const TOKENIZE: &str = "unicode61 remove_diacritics 1";

/// Snapshot of the sidecar `meta` table — the seal a reader checks
/// against the live `summaries.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarSeal {
    /// Sidecar schema version.
    pub schema_version: String,
    /// Builder version.
    pub builder_version: String,
    /// SHA-256 hex of the `summaries.json` bytes the sidecar was built from.
    pub summaries_hash: String,
    /// Count of `cards_fts` rows.
    pub summaries_count: usize,
}

/// Failure modes of [`build_sidecar`].
#[derive(Debug, Error)]
pub enum SidecarBuildError {
    /// `summaries.json` is parseable but invariant-invalid (a duplicate
    /// summary `id`, or a non-canonical `source_ids` list). No sidecar
    /// file — canonical, tmp, WAL, SHM, or journal — is created.
    #[error("summaries.json failed validation; no sidecar built: {0}")]
    InvalidSummaries(String),
    /// A `summaries.sqlite.tmp` already exists — refuse to clobber it,
    /// it may be a prior crashed run's recovery evidence.
    #[error(
        "CACG-MAN-002: refusing to clobber existing sidecar tmp: {0}; \
         remove it and re-run kb index"
    )]
    TmpExists(PathBuf),
    /// Filesystem failure staging or publishing the sidecar.
    #[error("summaries.sqlite sidecar I/O failure: {0}")]
    Io(#[from] std::io::Error),
    /// `SQLite` failure building the `FTS5` table or `meta` seal.
    #[error("summaries.sqlite sidecar SQLite failure: {0}")]
    Sqlite(#[from] rusqlite::Error),
}

/// Raised when the sidecar cannot be trusted for the live
/// `summaries.json` — a `meta` seal mismatch, an unreadable `meta`
/// table, an open failure, or a `MATCH` failure. The caller maps it to
/// a `CACG-FTS-001` stderr line and falls back to the in-memory backend.
#[derive(Debug, Error)]
#[error("{0}")]
pub struct SidecarStaleError(pub String);

/// SHA-256 hex digest of the `summaries.json` bytes — what the
/// `meta.summaries_hash` seal stores and a reader recomputes on open.
/// Mirrors Python `compute_summaries_hash`.
#[must_use]
pub fn compute_summaries_hash(summaries_json_bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut hasher = Sha256::new();
    hasher.update(summaries_json_bytes);
    hasher
        .finalize()
        .iter()
        .fold(String::with_capacity(64), |mut hex, b| {
            let _ = write!(hex, "{b:02x}");
            hex
        })
}

/// Probe whether the runtime `SQLite` build supports `FTS5` by creating
/// and dropping an in-memory virtual table. Always `true` for this
/// crate's `bundled` `SQLite`, but kept for parity with Python
/// `fts5_available` so the `kb index` post-step's `CACG-FTS-002` path
/// stays real.
#[must_use]
pub fn fts5_available() -> bool {
    let Ok(conn) = Connection::open_in_memory() else {
        return false;
    };
    conn.execute_batch("CREATE VIRTUAL TABLE _probe USING fts5(content)")
        .is_ok()
}

/// Build `summaries.sqlite` in `out_dir` from `manifest`, sealed with
/// the SHA-256 of `summaries_json_bytes`.
///
/// Returns `Ok(Some(path))` on success and `Ok(None)` when the runtime
/// `SQLite` build lacks `FTS5` — the caller emits `CACG-FTS-002` and
/// skips the sidecar (a non-trust performance artifact). The build is
/// atomic: a `summaries.sqlite.tmp` is written with `journal_mode=DELETE`
/// (no WAL/SHM siblings) then renamed onto the canonical path; a
/// pre-existing `.tmp` is refused, not clobbered.
///
/// # Errors
///
/// [`SidecarBuildError`] on an invariant-invalid manifest (validated
/// before any file write, so no sidecar artifact is created), a
/// pre-existing tmp, an I/O failure, or a `SQLite` failure. On a
/// staging error the partial tmp is removed.
pub fn build_sidecar(
    out_dir: &Path,
    summaries_json_bytes: &[u8],
    manifest: &SummariesManifest,
) -> Result<Option<PathBuf>, SidecarBuildError> {
    // Validate the manifest at the trust boundary BEFORE the FTS5
    // probe, the tmp-path check, or any file write — a parseable but
    // invariant-invalid `summaries.json` (duplicate summary id,
    // non-canonical `source_ids`) must leave NO sidecar artifact,
    // exactly as the in-memory `SummariesIndex` loader rejects it.
    crate::summaries::validate_manifest(manifest)
        .map_err(|e| SidecarBuildError::InvalidSummaries(e.to_string()))?;
    if !fts5_available() {
        return Ok(None);
    }
    let sidecar_path = out_dir.join(SIDECAR_FILENAME);
    let tmp_path = out_dir.join(format!("{SIDECAR_FILENAME}.tmp"));
    if tmp_path.exists() {
        return Err(SidecarBuildError::TmpExists(tmp_path));
    }
    let seal = SidecarSeal {
        schema_version: SIDECAR_SCHEMA_VERSION.to_string(),
        builder_version: BUILDER_VERSION.to_string(),
        summaries_hash: compute_summaries_hash(summaries_json_bytes),
        summaries_count: manifest.summaries.len(),
    };
    if let Err(err) = write_sidecar_tmp(&tmp_path, manifest, &seal) {
        // The tmp is ours by construction (we refused a pre-existing
        // one above), so cleaning our own partial write is correct.
        let _ = std::fs::remove_file(&tmp_path);
        return Err(err);
    }
    // `std::fs::rename` atomically replaces an existing destination
    // file on every platform -- on Windows it issues a `FILE_RENAME_INFO`
    // with `FILE_RENAME_FLAG_REPLACE_IF_EXISTS` -- so re-running
    // `kb index` over an existing `summaries.sqlite` refreshes the
    // sidecar in place, mirroring Python `os.replace`.
    std::fs::rename(&tmp_path, &sidecar_path)?;
    Ok(Some(sidecar_path))
}

/// Populate a fresh `summaries.sqlite.tmp`. Factored out so a failure
/// can clean the partial tmp at the [`build_sidecar`] call site. The
/// connection is dropped (closed) when this returns, before the rename.
fn write_sidecar_tmp(
    tmp_path: &Path,
    manifest: &SummariesManifest,
    seal: &SidecarSeal,
) -> Result<(), SidecarBuildError> {
    let conn = Connection::open(tmp_path)?;
    // `journal_mode=DELETE` keeps the rollback journal transient so no
    // `-wal` / `-shm` siblings leak next to the committed sidecar.
    conn.execute_batch("PRAGMA journal_mode=DELETE; PRAGMA synchronous=NORMAL;")?;
    conn.execute_batch(&format!(
        "CREATE VIRTUAL TABLE cards_fts USING fts5(\
         card_id UNINDEXED, reading_id UNINDEXED, path UNINDEXED, \
         card_hash UNINDEXED, title, summary, tags, source_ids, \
         tokenize = '{TOKENIZE}'); \
         CREATE TABLE meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);"
    ))?;
    conn.execute(
        "INSERT INTO meta(key, value) VALUES \
         ('schema_version', ?1), ('builder_version', ?2), \
         ('summaries_hash', ?3), ('summaries_count', ?4)",
        rusqlite::params![
            seal.schema_version,
            seal.builder_version,
            seal.summaries_hash,
            seal.summaries_count.to_string(),
        ],
    )?;
    {
        let mut stmt = conn.prepare(
            "INSERT INTO cards_fts \
             (card_id, reading_id, path, card_hash, title, summary, tags, source_ids) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )?;
        for entry in &manifest.summaries {
            stmt.execute(rusqlite::params![
                entry.id,
                entry.reading_id,
                entry.path,
                entry.card_hash,
                entry.title,
                entry.summary,
                entry.tags.join(" "),
                entry.source_ids.join(" "),
            ])?;
        }
    }
    Ok(())
}

/// Read the `meta` table into a [`SidecarSeal`].
fn load_seal(conn: &Connection) -> Result<SidecarSeal, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT key, value FROM meta")?;
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    for row in rows {
        let (k, v) = row?;
        map.insert(k, v);
    }
    Ok(SidecarSeal {
        schema_version: map.get("schema_version").cloned().unwrap_or_default(), // qg-allow: intentional-discard — optional seal metadata
        builder_version: map.get("builder_version").cloned().unwrap_or_default(), // qg-allow: intentional-discard — optional seal metadata
        summaries_hash: map.get("summaries_hash").cloned().unwrap_or_default(), // qg-allow: intentional-discard — optional seal metadata
        summaries_count: map
            .get("summaries_count")
            .and_then(|v| v.parse().ok()) // qg-allow: intentional-discard — parse failure defaults to 0
            .unwrap_or(0),
    })
}

/// One raw `cards_fts` row plus its BM25 rank.
struct SqliteRow {
    card_id: String,
    reading_id: String,
    path: String,
    card_hash: String,
    title: String,
    summary: String,
    tags: String,
    source_ids: String,
    rank: f64,
}

/// FTS5-backed read-only search over `summaries.sqlite`.
///
/// Verifies the `meta` seal against the live `summaries.json` on
/// [`open`](Self::open); a mismatch is [`SidecarStaleError`]
/// (`CACG-FTS-001`) so the caller falls back to the in-memory backend.
#[derive(Debug)]
pub struct SummariesSqliteIndex {
    conn: Connection,
    seal: SidecarSeal,
}

impl SummariesSqliteIndex {
    /// Open `sidecar_path` read-only and verify its `meta` seal against
    /// `expected_summaries_hash` (the SHA-256 of the live `summaries.json`).
    ///
    /// # Errors
    ///
    /// [`SidecarStaleError`] when the file cannot be opened, the `meta`
    /// table is unreadable, or the sealed `schema_version` /
    /// `builder_version` / `summaries_hash` does not match — in every
    /// case the caller emits `CACG-FTS-001` and falls back to in-memory.
    pub fn open(
        sidecar_path: &Path,
        expected_summaries_hash: &str,
    ) -> Result<Self, SidecarStaleError> {
        let conn = Connection::open_with_flags(sidecar_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(|e| {
                SidecarStaleError(format!(
                    "CACG-FTS-001: cannot open summaries.sqlite \
                         (falling back to in-memory BM25): {e}"
                ))
            })?;
        let seal = load_seal(&conn).map_err(|e| {
            SidecarStaleError(format!(
                "CACG-FTS-001: sidecar meta table unreadable: {e}; \
                 falling back to in-memory BM25"
            ))
        })?;
        // The seal-check messages below are byte-identical with Python
        // `search_sqlite.py` (single-quoted values mirror Python `!r`)
        // so a stale-sidecar `CACG-FTS-001` stderr line parity-matches.
        if seal.schema_version != SIDECAR_SCHEMA_VERSION {
            return Err(SidecarStaleError(format!(
                "CACG-FTS-001: sidecar schema_version='{}' differs from \
                 current='{SIDECAR_SCHEMA_VERSION}'; falling back to in-memory \
                 BM25 (operator must re-run `kb index` to rebuild)",
                seal.schema_version
            )));
        }
        if seal.builder_version != BUILDER_VERSION {
            return Err(SidecarStaleError(format!(
                "CACG-FTS-001: sidecar builder_version='{}' differs from \
                 current='{BUILDER_VERSION}'; falling back to in-memory BM25",
                seal.builder_version
            )));
        }
        if seal.summaries_hash != expected_summaries_hash {
            return Err(SidecarStaleError(format!(
                "CACG-FTS-001: sidecar seal mismatch — meta.summaries_hash='{}' \
                 vs. current summaries.json hash='{expected_summaries_hash}'; \
                 falling back to in-memory BM25",
                seal.summaries_hash
            )));
        }
        Ok(Self { conn, seal })
    }

    /// The verified seal.
    #[must_use]
    pub fn seal(&self) -> &SidecarSeal {
        &self.seal
    }

    /// Search the sidecar, mirroring Python `SummariesSqliteIndex.search`:
    /// query tokens are normalized, quote-escaped, and `OR`-joined into
    /// an FTS5 `MATCH` expression; rows come back `ORDER BY rank,
    /// card_id ASC`; the retraction-set and source-matrix filters run
    /// before the `top_k` cap. Cross-backend score/rank parity with the
    /// in-memory backend is intentionally NOT a contract.
    ///
    /// # Errors
    ///
    /// [`SidecarStaleError`] if the FTS5 `MATCH` query fails — the
    /// caller falls back to the in-memory backend.
    pub fn search(
        &self,
        query: &str,
        top_k: i64,
        source_matrix: Option<&SourceMatrix>,
        retracted_card_ids: &[String],
    ) -> Result<Vec<SearchHit>, SidecarStaleError> {
        if top_k <= 0 {
            return Ok(Vec::new());
        }
        let tokens = tokenize_for_lookup(query);
        if tokens.is_empty() {
            return Ok(Vec::new());
        }
        // Each token is quote-escaped (embedded `"` doubled) so FTS5
        // keywords (AND/OR/NOT/NEAR) and punctuation are literal, then
        // joined with OR — FTS5 defaults to AND, too strict here.
        let match_expr = tokens
            .iter()
            .map(|t| format!("\"{}\"", t.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(" OR ");
        let retracted: HashSet<&str> = retracted_card_ids.iter().map(String::as_str).collect();
        let cap = usize::try_from(top_k).unwrap_or(usize::MAX);

        let mut stmt = self
            .conn
            .prepare(
                "SELECT card_id, reading_id, path, card_hash, title, summary, \
                 tags, source_ids, bm25(cards_fts) AS rank \
                 FROM cards_fts WHERE cards_fts MATCH ?1 \
                 ORDER BY rank, card_id ASC",
            )
            .map_err(|e| {
                SidecarStaleError(format!(
                    "CACG-FTS-001: FTS5 query prepare failed on \
                     summaries.sqlite: {e}; falling back to in-memory BM25"
                ))
            })?;
        let rows = stmt
            .query_map(rusqlite::params![match_expr], |row| {
                Ok(SqliteRow {
                    card_id: row.get(0)?,
                    reading_id: row.get(1)?,
                    path: row.get(2)?,
                    card_hash: row.get(3)?,
                    title: row.get(4)?,
                    summary: row.get(5)?,
                    tags: row.get(6)?,
                    source_ids: row.get(7)?,
                    rank: row.get(8)?,
                })
            })
            .map_err(|e| {
                SidecarStaleError(format!(
                    "CACG-FTS-001: FTS5 MATCH failed on summaries.sqlite: {e}; \
                     falling back to in-memory BM25"
                ))
            })?;

        let mut hits: Vec<SearchHit> = Vec::new();
        for row in rows {
            let row = row.map_err(|e| {
                SidecarStaleError(format!(
                    "CACG-FTS-001: FTS5 row decode failed on summaries.sqlite: \
                     {e}; falling back to in-memory BM25"
                ))
            })?;
            if retracted.contains(row.card_id.as_str()) {
                continue;
            }
            let source_ids: Vec<&str> = row.source_ids.split_whitespace().collect();
            if let Some(matrix) = source_matrix {
                let Some(allowed) = matrix.allowed.get(&row.reading_id) else {
                    continue;
                };
                // Evidence-required: a card with no cited sources is
                // never authorized even though {} ⊆ every allow-list.
                if source_ids.is_empty() {
                    continue;
                }
                let allowed_set: HashSet<&str> = allowed.iter().map(String::as_str).collect();
                if !source_ids.iter().all(|s| allowed_set.contains(s)) {
                    continue;
                }
            }
            hits.push(SearchHit {
                card_id: row.card_id,
                reading_id: row.reading_id,
                title: row.title,
                summary: row.summary,
                tags: row.tags.split_whitespace().map(str::to_string).collect(),
                path: row.path,
                card_hash: row.card_hash,
                // FTS5 `bm25()` returns negative (smaller = better);
                // negate so larger = better, matching the BM25 convention.
                score: round6(-row.rank),
            });
            if hits.len() >= cap {
                break;
            }
        }
        Ok(hits)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use cacg_core::index::{SummariesManifest, SummaryEntry};
    use cacg_core::schema::SchemaVersion;
    use std::collections::BTreeMap;

    fn entry(
        id: &str,
        reading: &str,
        title: &str,
        summary: &str,
        tags: &[&str],
        source_ids: &[&str],
    ) -> SummaryEntry {
        SummaryEntry {
            schema_version: SchemaVersion::V0,
            id: id.to_string(),
            title: title.to_string(),
            reading_id: reading.to_string(),
            summary: summary.to_string(),
            tags: tags.iter().map(|s| (*s).to_string()).collect(),
            source_ids: source_ids.iter().map(|s| (*s).to_string()).collect(),
            path: format!("cards/{id}.md"),
            card_hash: "a".repeat(64),
        }
    }

    fn manifest(entries: Vec<SummaryEntry>) -> SummariesManifest {
        SummariesManifest {
            schema_version: SchemaVersion::V0,
            summaries: entries,
        }
    }

    /// A sample manifest + its canonical JSON bytes.
    fn sample() -> (SummariesManifest, Vec<u8>) {
        let m = manifest(vec![
            entry(
                "alpha-card",
                "R1",
                "Intrinsic valuation",
                "discounted cash flow valuation of equities",
                &["valuation", "equity"],
                &["src-a"],
            ),
            entry(
                "beta-card",
                "R1",
                "Relative valuation",
                "pricing multiples and comparables",
                &["multiples"],
                &["src-b"],
            ),
        ]);
        let bytes = serde_json::to_vec(&m).expect("serialize manifest");
        (m, bytes)
    }

    fn matrix(pairs: &[(&str, &[&str])]) -> SourceMatrix {
        let mut allowed: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (reading, srcs) in pairs {
            allowed.insert(
                (*reading).to_string(),
                srcs.iter().map(|s| (*s).to_string()).collect(),
            );
        }
        SourceMatrix {
            schema_version: SchemaVersion::V0,
            allowed,
        }
    }

    #[test]
    fn fts5_is_available_in_the_bundled_build() {
        assert!(fts5_available(), "bundled rusqlite must ship FTS5");
    }

    #[test]
    fn compute_summaries_hash_is_sha256_hex() {
        // SHA-256 of the empty input is a well-known constant.
        assert_eq!(
            compute_summaries_hash(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        let h = compute_summaries_hash(b"summaries");
        assert_eq!(h.len(), 64);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn build_sidecar_creates_a_clean_single_file() {
        let dir = tempfile::tempdir().unwrap();
        let (m, bytes) = sample();
        let path = build_sidecar(dir.path(), &bytes, &m)
            .expect("build ok")
            .expect("FTS5 available");
        assert_eq!(path, dir.path().join("summaries.sqlite"));
        assert!(path.is_file());
        // No tmp / WAL / SHM / journal siblings survive a clean build.
        for leftover in [
            "summaries.sqlite.tmp",
            "summaries.sqlite-wal",
            "summaries.sqlite-shm",
            "summaries.sqlite-journal",
        ] {
            assert!(
                !dir.path().join(leftover).exists(),
                "{leftover} must not survive a clean build"
            );
        }
    }

    #[test]
    fn build_sidecar_refuses_to_clobber_a_pre_existing_tmp() {
        let dir = tempfile::tempdir().unwrap();
        let (m, bytes) = sample();
        std::fs::write(dir.path().join("summaries.sqlite.tmp"), b"sentinel").unwrap();
        let err =
            build_sidecar(dir.path(), &bytes, &m).expect_err("a pre-existing tmp must be refused");
        assert!(
            matches!(err, SidecarBuildError::TmpExists(_)),
            "got {err:?}"
        );
        // The pre-existing tmp is untouched; no canonical sidecar appeared.
        assert_eq!(
            std::fs::read(dir.path().join("summaries.sqlite.tmp")).unwrap(),
            b"sentinel"
        );
        assert!(!dir.path().join("summaries.sqlite").exists());
    }

    #[test]
    fn build_sidecar_replaces_an_existing_sidecar() {
        // Re-running `kb index` rebuilds the sidecar over an existing
        // `summaries.sqlite`. `std::fs::rename` replaces an existing
        // destination file on every platform (on Windows via
        // `FILE_RENAME_FLAG_REPLACE_IF_EXISTS`), so the rebuild refreshes
        // the sidecar in place -- mirroring Python `os.replace`.
        let dir = tempfile::tempdir().unwrap();
        let (m, bytes) = sample();
        build_sidecar(dir.path(), &bytes, &m)
            .expect("first build ok")
            .expect("FTS5 available");

        // Rebuild over the now-existing sidecar with a different manifest.
        let m2 = manifest(vec![entry(
            "gamma-card",
            "R1",
            "Solo",
            "the only remaining card",
            &[],
            &["src-g"],
        )]);
        let bytes2 = serde_json::to_vec(&m2).expect("serialize manifest");
        let path = build_sidecar(dir.path(), &bytes2, &m2)
            .expect("a rebuild over an existing sidecar succeeds")
            .expect("FTS5 available");

        // The canonical file now reflects the SECOND build: it opens
        // against the new summaries hash and seals the new count.
        let index = SummariesSqliteIndex::open(&path, &compute_summaries_hash(&bytes2))
            .expect("the rebuilt sidecar opens against the new summaries hash");
        assert_eq!(index.seal().summaries_count, 1);
        // No tmp leftover survives the rebuild.
        assert!(!dir.path().join("summaries.sqlite.tmp").exists());
    }

    #[test]
    fn sealed_index_opens_and_searches() {
        let dir = tempfile::tempdir().unwrap();
        let (m, bytes) = sample();
        build_sidecar(dir.path(), &bytes, &m).unwrap().unwrap();
        let hash = compute_summaries_hash(&bytes);
        let index = SummariesSqliteIndex::open(&dir.path().join("summaries.sqlite"), &hash)
            .expect("fresh sidecar opens");
        assert_eq!(index.seal().summaries_count, 2);
        let hits = index.search("valuation", 10, None, &[]).expect("search ok");
        assert_eq!(hits.len(), 2, "both cards mention valuation");
        assert!(hits.iter().any(|h| h.card_id == "alpha-card"));
    }

    #[test]
    fn seal_mismatch_is_a_stale_error() {
        let dir = tempfile::tempdir().unwrap();
        let (m, bytes) = sample();
        build_sidecar(dir.path(), &bytes, &m).unwrap().unwrap();
        let err = SummariesSqliteIndex::open(
            &dir.path().join("summaries.sqlite"),
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .expect_err("a hash mismatch must be stale");
        assert!(err.0.contains("CACG-FTS-001"), "got {}", err.0);
        assert!(err.0.contains("seal mismatch"), "got {}", err.0);
    }

    #[test]
    fn search_top_k_zero_and_empty_query_return_empty() {
        let dir = tempfile::tempdir().unwrap();
        let (m, bytes) = sample();
        build_sidecar(dir.path(), &bytes, &m).unwrap().unwrap();
        let hash = compute_summaries_hash(&bytes);
        let index =
            SummariesSqliteIndex::open(&dir.path().join("summaries.sqlite"), &hash).unwrap();
        assert!(index.search("valuation", 0, None, &[]).unwrap().is_empty());
        assert!(index.search("valuation", -1, None, &[]).unwrap().is_empty());
        assert!(index.search("   ", 10, None, &[]).unwrap().is_empty());
    }

    #[test]
    fn search_applies_retraction_and_source_matrix_filters() {
        let dir = tempfile::tempdir().unwrap();
        let (m, bytes) = sample();
        build_sidecar(dir.path(), &bytes, &m).unwrap().unwrap();
        let hash = compute_summaries_hash(&bytes);
        let index =
            SummariesSqliteIndex::open(&dir.path().join("summaries.sqlite"), &hash).unwrap();

        // Retraction: alpha-card filtered out.
        let hits = index
            .search("valuation", 10, None, &["alpha-card".to_string()])
            .unwrap();
        assert!(hits.iter().all(|h| h.card_id != "alpha-card"));

        // Source-matrix: only src-a authorized for R1 → beta-card (src-b) excluded.
        let mtx = matrix(&[("R1", &["src-a"])]);
        let hits = index.search("valuation", 10, Some(&mtx), &[]).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].card_id, "alpha-card");
    }

    #[test]
    fn search_results_are_deterministic_across_two_runs() {
        let dir = tempfile::tempdir().unwrap();
        let (m, bytes) = sample();
        build_sidecar(dir.path(), &bytes, &m).unwrap().unwrap();
        let hash = compute_summaries_hash(&bytes);
        let index =
            SummariesSqliteIndex::open(&dir.path().join("summaries.sqlite"), &hash).unwrap();
        let a = index.search("valuation equity", 10, None, &[]).unwrap();
        let b = index.search("valuation equity", 10, None, &[]).unwrap();
        assert_eq!(a, b, "FTS5 query results must be deterministic");
    }

    /// Assert `build_sidecar` rejects an invariant-invalid manifest and
    /// leaves no sidecar artifact of any kind on disk.
    fn assert_invalid_summaries_build_no_sidecar(manifest: &SummariesManifest) {
        let dir = tempfile::tempdir().unwrap();
        let bytes = serde_json::to_vec(manifest).expect("serialize manifest");
        let err = build_sidecar(dir.path(), &bytes, manifest)
            .expect_err("an invariant-invalid manifest must not build a sidecar");
        assert!(
            matches!(err, SidecarBuildError::InvalidSummaries(_)),
            "expected InvalidSummaries, got {err:?}"
        );
        for leftover in [
            "summaries.sqlite",
            "summaries.sqlite.tmp",
            "summaries.sqlite-wal",
            "summaries.sqlite-shm",
            "summaries.sqlite-journal",
        ] {
            assert!(
                !dir.path().join(leftover).exists(),
                "{leftover} must not exist after a rejected build"
            );
        }
    }

    #[test]
    fn build_sidecar_rejects_duplicate_summary_ids() {
        let m = manifest(vec![
            entry("dup", "R1", "Title A", "summary a", &[], &["s1"]),
            entry("dup", "R1", "Title B", "summary b", &[], &["s1"]),
        ]);
        assert_invalid_summaries_build_no_sidecar(&m);
    }

    #[test]
    fn build_sidecar_rejects_unsorted_source_ids() {
        let m = manifest(vec![entry(
            "card-a",
            "R1",
            "Title",
            "summary",
            &[],
            &["s2", "s1"],
        )]);
        assert_invalid_summaries_build_no_sidecar(&m);
    }

    #[test]
    fn build_sidecar_rejects_duplicate_source_ids() {
        let m = manifest(vec![entry(
            "card-a",
            "R1",
            "Title",
            "summary",
            &[],
            &["s1", "s1"],
        )]);
        assert_invalid_summaries_build_no_sidecar(&m);
    }

    #[test]
    fn build_sidecar_rejects_empty_string_source_id() {
        let m = manifest(vec![entry("card-a", "R1", "Title", "summary", &[], &[""])]);
        assert_invalid_summaries_build_no_sidecar(&m);
    }

    #[test]
    fn build_sidecar_rejects_a_bad_card_hash() {
        let mut e = entry("card-a", "R1", "Title", "summary", &[], &["s1"]);
        e.card_hash = "not-64-hex".to_string();
        assert_invalid_summaries_build_no_sidecar(&manifest(vec![e]));
    }

    #[test]
    fn build_sidecar_rejects_an_empty_required_field() {
        let mut e = entry("card-a", "R1", "Title", "summary", &[], &["s1"]);
        e.title = String::new();
        assert_invalid_summaries_build_no_sidecar(&manifest(vec![e]));
    }
}
