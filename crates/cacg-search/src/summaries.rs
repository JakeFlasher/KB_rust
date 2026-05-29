//! In-memory BM25 lexical search over the `summaries.json` artifact.
//!
//! Mirrors Python `legacy_python_oracle/src/cacg/search.py::SummariesIndex` field-for-field:
//! the per-entry BM25 corpus is the lookup-tokenized `title + summary +
//! tags`; `search` scores the FULL corpus, then filters by retraction,
//! source-matrix authorization, and lexical overlap, then sorts on the
//! full-precision score (with a `card_id` tie-break), caps at `top_k`,
//! and rounds scores to 6 decimals only for the emitted [`SearchHit`].
//!
//! `from_path` validates at the load trust boundary: a legacy entry
//! missing the `source_ids` key is rejected with `CACG-SUM-007`,
//! duplicate summary `id`s are rejected, and each `source_ids` list must
//! be sorted, unique, and free of empty strings (an empty list is
//! allowed). Reference: `_research/15_bm25_retrieval_audit.md`.

use std::collections::HashSet;
use std::path::Path;

use cacg_core::bm25::Bm25Okapi;
use cacg_core::index::{SummariesManifest, SummaryEntry};
use cacg_core::normalize::tokenize_for_lookup;
use cacg_core::schema::{is_64_hex, SourceMatrix};
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

/// One row of `kb search` output. Mirrors Python `search.SearchHit`.
///
/// Serializes to the eight-field object Python's `kb search --json`
/// emits (`card_id`, `reading_id`, `title`, `summary`, `tags`, `path`,
/// `card_hash`, `score`).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SearchHit {
    /// The matched card's identifier.
    pub card_id: String,
    /// The matched card's reading identifier.
    pub reading_id: String,
    /// The card title.
    pub title: String,
    /// The card summary.
    pub summary: String,
    /// The card's frontmatter tags.
    pub tags: Vec<String>,
    /// Path to the card markdown file.
    pub path: String,
    /// 64-hex SHA-256 card hash.
    pub card_hash: String,
    /// BM25 score against the query, rounded to 6 decimal places.
    pub score: f64,
}

/// Failure modes of [`SummariesIndex::from_path`].
#[derive(Debug, Error)]
pub enum SummariesLoadError {
    /// I/O failure reading the file; the CLI maps this to `CACG-CLI-001`.
    #[error("cannot read summaries.json: {0}")]
    Io(#[from] std::io::Error),
    /// A legacy `summaries.json` whose entries lack the `source_ids` key.
    #[error(
        "CACG-SUM-007: legacy summaries.json lacks `source_ids` on one or \
         more entries; run `kb migrate-summaries --out <out>` to populate \
         source_ids from the in-corpus cards, then re-run `kb search`."
    )]
    LegacyMissingSourceIds,
    /// Malformed JSON, failed schema deserialization, or a load-time
    /// validation failure (duplicate id, non-canonical `source_ids`); the
    /// CLI maps this to `CACG-MAN-001`.
    #[error("CACG-MAN-001: {0}")]
    Malformed(String),
}

/// Process-local in-memory BM25 index over `summaries.json`.
#[derive(Debug)]
pub struct SummariesIndex {
    entries: Vec<SummaryEntry>,
    corpus: Vec<Vec<String>>,
    bm25: Option<Bm25Okapi>,
}

impl SummariesIndex {
    /// Load and validate `summaries.json` from disk into a typed index.
    ///
    /// # Errors
    ///
    /// Returns [`SummariesLoadError`] on an I/O failure, a parse failure, a
    /// legacy entry missing the `source_ids` key, or a load-time validation
    /// failure (duplicate summary `id`, non-canonical `source_ids`).
    pub fn from_path(path: &Path) -> Result<Self, SummariesLoadError> {
        let raw_text = std::fs::read_to_string(path)?;
        // Inspect the raw JSON before typed deserialization: a pre-Phase-4
        // entry that lacks the `source_ids` KEY must be rejected with
        // CACG-SUM-007, since an additive-schema default would otherwise
        // mask the missing key.
        let raw: Value = serde_json::from_str(&raw_text).map_err(|e| {
            SummariesLoadError::Malformed(format!("cannot parse summaries.json: {e}"))
        })?;
        if let Some(summaries) = raw.get("summaries").and_then(Value::as_array) {
            for entry in summaries {
                if entry.is_object() && entry.get("source_ids").is_none() {
                    return Err(SummariesLoadError::LegacyMissingSourceIds);
                }
            }
        }
        let manifest: SummariesManifest = serde_json::from_value(raw).map_err(|e| {
            SummariesLoadError::Malformed(format!("summaries.json failed schema validation: {e}"))
        })?;
        Self::from_manifest(manifest)
    }

    /// Build the index from an already-deserialized manifest, running the
    /// load-time validations.
    fn from_manifest(manifest: SummariesManifest) -> Result<Self, SummariesLoadError> {
        validate_manifest(&manifest)?;
        let entries = manifest.summaries;
        let corpus: Vec<Vec<String>> = entries
            .iter()
            .map(|e| tokenize_for_lookup(&corpus_text_for(e)))
            .collect();
        let bm25 = if corpus.is_empty() {
            // Zero entries: Python `SummariesIndex.__init__` sets
            // `_bm25 = None` (its `if self._corpus` guard is false).
            None
        } else {
            // A non-empty corpus: Python calls `BM25Okapi(self._corpus)`.
            // `rank_bm25` raises `ZeroDivisionError` when no entry
            // contributes a token (every `title + summary + tags`
            // tokenizes to nothing), and `_cmd_search` surfaces that as
            // `CACG-MAN-001`. `Bm25Okapi::new` is deliberately total, so
            // detect the condition on the built index and reject here,
            // mirroring the Python failure rather than loading an
            // empty-IDF index that would silently return no matches.
            let index = Bm25Okapi::new(&corpus);
            if index.is_token_empty() {
                return Err(SummariesLoadError::Malformed(
                    "summaries.json has no searchable text: every entry's \
                     title, summary, and tags are empty after normalization"
                        .to_string(),
                ));
            }
            Some(index)
        };
        Ok(Self {
            entries,
            corpus,
            bm25,
        })
    }

    /// The number of summary entries in the index.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the index holds no summary entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Search the index, byte-equal with Python `SummariesIndex.search`.
    ///
    /// BM25 is scored over the FULL corpus; then per candidate the
    /// retraction-set filter, the source-matrix per-source authorization
    /// filter (the entry's `reading_id` is a matrix key, every `source_ids`
    /// element is authorized, and `source_ids` is non-empty), and the
    /// lexical-overlap match gate are applied; then results are sorted by
    /// full-precision score descending with a `card_id` ascending
    /// tie-break; then `top_k` caps the list. Scores are rounded to 6
    /// decimals only for the emitted [`SearchHit`].
    ///
    /// Returns an empty vector for an empty corpus, `top_k <= 0`, or a
    /// query that tokenizes to nothing.
    #[must_use]
    pub fn search(
        &self,
        query: &str,
        top_k: i64,
        source_matrix: Option<&SourceMatrix>,
        retracted_card_ids: &[String],
    ) -> Vec<SearchHit> {
        let Some(bm25) = self.bm25.as_ref() else {
            return Vec::new();
        };
        if top_k <= 0 {
            return Vec::new();
        }
        let query_tokens = tokenize_for_lookup(query);
        if query_tokens.is_empty() {
            return Vec::new();
        }
        let query_token_set: HashSet<&str> = query_tokens.iter().map(String::as_str).collect();
        let scores = bm25.get_scores(&query_tokens);
        let retracted: HashSet<&str> = retracted_card_ids.iter().map(String::as_str).collect();

        // Pass 1: collect (raw_score, index) for every surviving candidate.
        let mut raw_hits: Vec<(f64, usize)> = Vec::new();
        for (idx, &score) in scores.iter().enumerate() {
            let entry = &self.entries[idx];
            if retracted.contains(entry.id.as_str()) {
                continue;
            }
            if let Some(matrix) = source_matrix {
                let Some(allowed) = matrix.allowed.get(&entry.reading_id) else {
                    continue;
                };
                // Evidence-required policy: a card with no cited sources
                // cannot be authorized even though the empty set is a
                // subset of every allow-list.
                if entry.source_ids.is_empty() {
                    continue;
                }
                let allowed_set: HashSet<&str> = allowed.iter().map(String::as_str).collect();
                if !entry
                    .source_ids
                    .iter()
                    .all(|s| allowed_set.contains(s.as_str()))
                {
                    continue;
                }
            }
            // Lexical-overlap gate: BM25 score alone is not the match
            // predicate (small-corpus IDF can drive non-overlapping docs
            // positive); require at least one shared token.
            if !self.corpus[idx]
                .iter()
                .any(|t| query_token_set.contains(t.as_str()))
            {
                continue;
            }
            raw_hits.push((score, idx));
        }

        // Sort on the full-precision raw score descending, tie-break on
        // `card_id` ascending. Rounding before sorting could reorder
        // near-equal hits, so it happens only at SearchHit construction.
        raw_hits.sort_by(|a, b| {
            b.0.partial_cmp(&a.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| self.entries[a.1].id.cmp(&self.entries[b.1].id))
        });

        let cap = usize::try_from(top_k).unwrap_or(usize::MAX);
        raw_hits
            .into_iter()
            .take(cap)
            .map(|(raw, idx)| {
                let e = &self.entries[idx];
                SearchHit {
                    card_id: e.id.clone(),
                    reading_id: e.reading_id.clone(),
                    title: e.title.clone(),
                    summary: e.summary.clone(),
                    tags: e.tags.clone(),
                    path: e.path.clone(),
                    card_hash: e.card_hash.clone(),
                    score: round6(raw),
                }
            })
            .collect()
    }
}

/// Validate a deserialized `summaries.json` manifest at the load trust
/// boundary: every entry's `source_ids` list is free of empty strings,
/// unique, and sorted; and summary `id`s are unique across the manifest
/// (`BL-20260518-reject-duplicate-keys-at-trust-boundary`).
///
/// This is the single shared validator both retrieval backends run:
/// the in-memory [`SummariesIndex::from_manifest`] and the FTS5
/// `cacg_search::sqlite::build_sidecar`. Without it, the sidecar
/// builder would seal a `summaries.sqlite` over a manifest the
/// in-memory loader rejects — an un-called validator is the same hole
/// (`BL-20260522-port-pydantic-validators-not-just-fields`).
///
/// # Errors
///
/// Returns [`SummariesLoadError::Malformed`] on a duplicate summary
/// `id` or a non-canonical `source_ids` list.
pub fn validate_manifest(manifest: &SummariesManifest) -> Result<(), SummariesLoadError> {
    let mut seen: HashSet<&str> = HashSet::with_capacity(manifest.summaries.len());
    let mut duplicates: Vec<&str> = Vec::new();
    for entry in &manifest.summaries {
        validate_entry_fields(entry)?;
        validate_source_ids(&entry.id, &entry.source_ids)?;
        if !seen.insert(entry.id.as_str()) {
            duplicates.push(entry.id.as_str());
        }
    }
    if !duplicates.is_empty() {
        duplicates.sort_unstable();
        duplicates.dedup();
        return Err(SummariesLoadError::Malformed(format!(
            "summaries.json has duplicate summary id(s): {duplicates:?}"
        )));
    }
    Ok(())
}

/// The BM25 corpus text for one entry: `title + summary + tags` joined by a
/// single space, mirroring Python `_corpus_text_for`.
fn corpus_text_for(entry: &SummaryEntry) -> String {
    let mut parts: Vec<&str> = Vec::with_capacity(2 + entry.tags.len());
    parts.push(entry.title.as_str());
    parts.push(entry.summary.as_str());
    parts.extend(entry.tags.iter().map(String::as_str));
    parts.join(" ")
}

/// Validate one entry's persisted `source_ids`: every element non-empty,
/// the list unique and sorted. An empty list is valid. Mirrors the Python
/// `SummaryEntry._source_ids_sorted_unique` validator.
fn validate_source_ids(card_id: &str, source_ids: &[String]) -> Result<(), SummariesLoadError> {
    if source_ids.iter().any(String::is_empty) {
        return Err(SummariesLoadError::Malformed(format!(
            "summary {card_id:?}: source_ids entries must be non-empty strings"
        )));
    }
    let mut sorted: Vec<&str> = source_ids.iter().map(String::as_str).collect();
    sorted.sort_unstable();
    let mut deduped = sorted.clone();
    deduped.dedup();
    if deduped.len() != sorted.len() {
        return Err(SummariesLoadError::Malformed(format!(
            "summary {card_id:?}: source_ids must be unique"
        )));
    }
    if sorted
        .iter()
        .copied()
        .ne(source_ids.iter().map(String::as_str))
    {
        return Err(SummariesLoadError::Malformed(format!(
            "summary {card_id:?}: source_ids must be sorted"
        )));
    }
    Ok(())
}

/// Validate one `SummaryEntry`'s scalar fields, mirroring Python
/// `cacg.index.SummaryEntry`'s field validators: `id`, `title`,
/// `reading_id`, `summary`, and `path` are non-empty, and `card_hash`
/// is 64-hex SHA-256. `source_ids` is checked separately by
/// `validate_source_ids`; an unknown field is rejected at
/// deserialization by the struct's `#[serde(deny_unknown_fields)]`.
fn validate_entry_fields(entry: &SummaryEntry) -> Result<(), SummariesLoadError> {
    for (field, value) in [
        ("id", &entry.id),
        ("title", &entry.title),
        ("reading_id", &entry.reading_id),
        ("summary", &entry.summary),
        ("path", &entry.path),
    ] {
        if value.is_empty() {
            return Err(SummariesLoadError::Malformed(format!(
                "summary {:?}: {field} must be a non-empty string",
                entry.id
            )));
        }
    }
    if !is_64_hex(&entry.card_hash) {
        return Err(SummariesLoadError::Malformed(format!(
            "summary {:?}: card_hash must be 64-hex SHA256",
            entry.id
        )));
    }
    Ok(())
}

/// Round to 6 decimal places, matching Python `round(x, 6)`.
pub(crate) fn round6(x: f64) -> f64 {
    format!("{x:.6}")
        .parse()
        .expect("a fixed-precision float format always re-parses")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{SummariesIndex, SummariesLoadError};
    use std::collections::BTreeMap;

    use cacg_core::schema::{SchemaVersion, SourceMatrix};

    /// A summaries manifest written to a fresh tempdir. The `TempDir`
    /// is held so the file outlives the test; `path()` exposes the
    /// fixed-named manifest. A `TempDir` + fixed filename (not
    /// `NamedTempFile`'s random suffix) keeps the helper deterministic
    /// per the `lint-determinism` gate.
    struct SummariesFile {
        _dir: tempfile::TempDir,
        path: std::path::PathBuf,
    }

    impl SummariesFile {
        fn path(&self) -> &std::path::Path {
            &self.path
        }
    }

    fn write_summaries(json: &str) -> SummariesFile {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("summaries.json");
        std::fs::write(&path, json).expect("write summaries.json");
        SummariesFile { _dir: dir, path }
    }

    fn entry_json(
        id: &str,
        reading: &str,
        title: &str,
        summary: &str,
        source_ids: &[&str],
    ) -> String {
        let sids: Vec<String> = source_ids.iter().map(|s| format!("{s:?}")).collect();
        format!(
            r#"{{"schema_version":"cacg.v0","id":{id:?},"title":{title:?},"reading_id":{reading:?},"summary":{summary:?},"tags":[],"source_ids":[{}],"path":"cards/{id}.md","card_hash":"{hash}"}}"#,
            sids.join(","),
            hash = "a".repeat(64),
        )
    }

    fn manifest_json(entries: &[String]) -> String {
        format!(
            r#"{{"schema_version":"cacg.v0","summaries":[{}]}}"#,
            entries.join(",")
        )
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
    fn from_path_loads_a_valid_manifest() {
        let f = write_summaries(&manifest_json(&[entry_json(
            "card-a",
            "R1",
            "alpha title",
            "the summary text here",
            &["s1"],
        )]));
        let index = SummariesIndex::from_path(f.path()).expect("valid manifest loads");
        assert_eq!(index.len(), 1);
        assert!(!index.is_empty());
    }

    #[test]
    fn from_path_rejects_missing_source_ids_key_with_sum_007() {
        // Entry lacks the `source_ids` key entirely (legacy artifact).
        let legacy = r#"{"schema_version":"cacg.v0","id":"card-a","title":"t","reading_id":"R1","summary":"the summary text here","tags":[],"path":"cards/card-a.md","card_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#;
        let f = write_summaries(&manifest_json(&[legacy.to_string()]));
        let err = SummariesIndex::from_path(f.path()).expect_err("missing key rejected");
        assert!(matches!(err, SummariesLoadError::LegacyMissingSourceIds));
    }

    #[test]
    fn from_path_rejects_duplicate_summary_ids() {
        let dup = manifest_json(&[
            entry_json("card-a", "R1", "alpha", "summary one text here", &["s1"]),
            entry_json("card-a", "R1", "beta", "summary two text here", &["s2"]),
        ]);
        let f = write_summaries(&dup);
        let err = SummariesIndex::from_path(f.path()).expect_err("duplicate id rejected");
        assert!(matches!(err, SummariesLoadError::Malformed(_)));
    }

    #[test]
    fn from_path_rejects_unsorted_source_ids_but_accepts_empty_list() {
        let unsorted = manifest_json(&[entry_json(
            "card-a",
            "R1",
            "alpha",
            "summary text here",
            &["s2", "s1"],
        )]);
        let f = write_summaries(&unsorted);
        assert!(matches!(
            SummariesIndex::from_path(f.path()),
            Err(SummariesLoadError::Malformed(_))
        ));
        // An empty `source_ids` list is valid at load time.
        let empty = write_summaries(&manifest_json(&[entry_json(
            "card-a",
            "R1",
            "alpha",
            "summary text here",
            &[],
        )]));
        assert!(SummariesIndex::from_path(empty.path()).is_ok());
    }

    #[test]
    fn from_path_rejects_malformed_json() {
        let f = write_summaries("{ not valid json");
        assert!(matches!(
            SummariesIndex::from_path(f.path()),
            Err(SummariesLoadError::Malformed(_))
        ));
    }

    #[test]
    fn search_empty_query_and_nonpositive_top_k_yield_no_hits() {
        let f = write_summaries(&manifest_json(&[entry_json(
            "card-a",
            "R1",
            "alpha title",
            "the summary text here",
            &["s1"],
        )]));
        let index = SummariesIndex::from_path(f.path()).unwrap();
        assert!(index.search("", 10, None, &[]).is_empty());
        assert!(index.search("   ", 10, None, &[]).is_empty());
        assert!(index.search("alpha", 0, None, &[]).is_empty());
        assert!(index.search("alpha", -1, None, &[]).is_empty());
    }

    #[test]
    fn search_is_deterministic_across_runs() {
        let f = write_summaries(&manifest_json(&[
            entry_json(
                "card-a",
                "R1",
                "discounted cash flow",
                "valuation model",
                &["s1"],
            ),
            entry_json(
                "card-b",
                "R1",
                "weighted average cost",
                "capital structure",
                &["s1"],
            ),
        ]));
        let index = SummariesIndex::from_path(f.path()).unwrap();
        let m = matrix(&[("R1", &["s1"])]);
        let first = index.search("cash flow", 10, Some(&m), &[]);
        let second = index.search("cash flow", 10, Some(&m), &[]);
        assert_eq!(first, second);
        assert!(!first.is_empty());
    }

    #[test]
    fn search_excludes_retracted_and_unauthorized_cards() {
        let f = write_summaries(&manifest_json(&[
            entry_json("card-a", "R1", "alpha cash", "summary about cash", &["s1"]),
            entry_json("card-b", "R1", "beta cash", "summary about cash", &["s9"]),
        ]));
        let index = SummariesIndex::from_path(f.path()).unwrap();
        let m = matrix(&[("R1", &["s1"])]);
        // card-b cites s9, which is not on R1's allow-list -> excluded.
        let hits = index.search("cash", 10, Some(&m), &[]);
        assert!(hits.iter().all(|h| h.card_id != "card-b"));
        // Retracting card-a leaves no hits.
        let retracted = vec!["card-a".to_string()];
        assert!(index.search("cash", 10, Some(&m), &retracted).is_empty());
    }

    #[test]
    fn search_pathological_query_stays_bounded() {
        // CI-safe perf smoke: a pathological query — thousands of
        // repeated tokens — must not panic, must not allocate
        // unboundedly, and must still cap the result at `top_k`. The
        // search is `O(query_len x corpus)`, never quadratic in the
        // corpus, so a hostile query cannot blow up the fixture.
        let f = write_summaries(&manifest_json(&[
            entry_json(
                "card-a",
                "R1",
                "alpha valuation",
                "discounted cash flow",
                &["s1"],
            ),
            entry_json(
                "card-b",
                "R1",
                "beta multiples",
                "relative valuation",
                &["s1"],
            ),
        ]));
        let index = SummariesIndex::from_path(f.path()).unwrap();
        let m = matrix(&[("R1", &["s1"])]);
        let pathological = "valuation ".repeat(10_000);
        let hits = index.search(&pathological, 5, Some(&m), &[]);
        assert!(hits.len() <= 5, "results must stay capped at top_k");
        // A pathological zero-overlap query simply yields nothing.
        let noise = "zzz ".repeat(10_000);
        assert!(index.search(&noise, 5, Some(&m), &[]).is_empty());
    }

    #[test]
    fn from_path_rejects_a_bad_card_hash() {
        // Python `SummaryEntry._hash_format` rejects a non-64-hex
        // `card_hash` with a `CACG-MAN-001`-class error; Rust must too.
        let bad = r#"{"schema_version":"cacg.v0","id":"a","title":"T","reading_id":"R1","summary":"S","tags":[],"source_ids":["s1"],"path":"cards/a.md","card_hash":"not-64-hex"}"#;
        let f = write_summaries(&manifest_json(&[bad.to_string()]));
        let err = SummariesIndex::from_path(f.path())
            .expect_err("a non-64-hex card_hash must be rejected");
        assert!(
            matches!(err, SummariesLoadError::Malformed(_)),
            "got {err:?}"
        );
    }

    #[test]
    fn from_path_rejects_an_empty_required_field() {
        // Python `SummaryEntry` declares `title` (and id/reading_id/
        // summary/path) `Field(min_length=1)`; an empty value rejects.
        let bad = r#"{"schema_version":"cacg.v0","id":"a","title":"","reading_id":"R1","summary":"S","tags":[],"source_ids":["s1"],"path":"cards/a.md","card_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#;
        let f = write_summaries(&manifest_json(&[bad.to_string()]));
        let err = SummariesIndex::from_path(f.path())
            .expect_err("an empty required string field must be rejected");
        assert!(
            matches!(err, SummariesLoadError::Malformed(_)),
            "got {err:?}"
        );
    }

    #[test]
    fn from_path_rejects_an_unknown_field() {
        // `#[serde(deny_unknown_fields)]` mirrors Python `_StrictModel`'s
        // `extra="forbid"`: an unknown entry field is rejected at load.
        let bad = r#"{"schema_version":"cacg.v0","id":"a","title":"T","reading_id":"R1","summary":"S","tags":[],"source_ids":["s1"],"path":"cards/a.md","card_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","surprise":1}"#;
        let f = write_summaries(&manifest_json(&[bad.to_string()]));
        let err =
            SummariesIndex::from_path(f.path()).expect_err("an unknown field must be rejected");
        assert!(
            matches!(err, SummariesLoadError::Malformed(_)),
            "got {err:?}"
        );
    }

    #[test]
    fn from_path_accepts_an_omitted_tags_key() {
        // Python `SummaryEntry.tags` is `Field(default_factory=list)`;
        // an entry that carries `source_ids` but omits the additive
        // `tags` key is Python-valid and must load (`tags = []`), not
        // fail with a `CACG-MAN-001`-class error.
        let no_tags = r#"{"schema_version":"cacg.v0","id":"a","title":"T","reading_id":"R1","summary":"S","source_ids":["s1"],"path":"cards/a.md","card_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}"#;
        let f = write_summaries(&manifest_json(&[no_tags.to_string()]));
        let index = SummariesIndex::from_path(f.path())
            .expect("a summaries.json that omits the additive tags key must load");
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn from_path_rejects_a_summaries_json_with_no_searchable_text() {
        // Schema-valid (title/summary are length-1, non-empty) but every
        // entry's title + summary + tags tokenizes to nothing. Python
        // builds `BM25Okapi([[]])`, which raises `ZeroDivisionError`, and
        // `_cmd_search` surfaces it as `CACG-MAN-001`. Rust must reject
        // it the same way rather than loading and returning no matches.
        let blank = entry_json("card-a", "R1", " ", " ", &["s1"]);
        let f = write_summaries(&manifest_json(&[blank]));
        let err = SummariesIndex::from_path(f.path())
            .expect_err("an all-whitespace summaries.json must be rejected");
        assert!(
            matches!(err, SummariesLoadError::Malformed(_)),
            "got {err:?}"
        );
    }

    #[test]
    fn from_path_accepts_a_zero_entry_manifest() {
        // Zero entries is valid: Python `SummariesIndex` sets
        // `_bm25 = None` and `kb search` prints `(no matches)`, exit 0.
        // Distinct from a non-empty manifest with no searchable text,
        // which is rejected above.
        let f = write_summaries(&manifest_json(&[]));
        let index = SummariesIndex::from_path(f.path()).expect("a 0-entry manifest must load");
        assert!(index.is_empty());
        assert!(index.search("anything", 10, None, &[]).is_empty());
    }
}
