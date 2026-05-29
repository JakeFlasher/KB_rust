//! Struct definitions and validation for the index manifest artifacts.
//!
//! Contains [`CardManifestEntry`], [`CardsManifest`], [`SummaryEntry`],
//! [`SummariesManifest`], [`IndexError`], plus the [`canonical_serialize`]
//! and [`build_index_md`] helpers that operate on these types.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::atomic_publish::PublishError;
use crate::diagnostic::{codes, Diagnostic, Severity};
use crate::history::HistoryError;
use crate::journal::JournalError;
use crate::schema::{is_64_hex, validate_retracted_list, SchemaVersion};

/// One entry in `cards_manifest.json.cards`. Mirrors Python's
/// `cacg.index.CardManifestEntry` field-for-field so the two
/// implementations canonical-serialize to byte-identical output.
/// `deny_unknown_fields` mirrors Python's `_StrictModel` (`extra="forbid"`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardManifestEntry {
    /// Schema version (always `cacg.v0`).
    pub schema_version: SchemaVersion,
    /// Path to the card markdown file (as supplied to `build_index`).
    pub path: String,
    /// Card identifier from the frontmatter.
    pub id: String,
    /// Title from the frontmatter.
    pub title: String,
    /// Reading identifier from the frontmatter.
    pub reading_id: String,
    /// Summary text from the frontmatter.
    pub summary: String,
    /// 64-hex SHA-256 recomputed during indexing.
    pub card_hash: String,
    /// Count of frontmatter citations.
    pub citation_count: u32,
    /// Sorted-unique list of cited `source_id`s. Defaults to `[]` when
    /// omitted, mirroring Python's `Field(default_factory=list)`.
    #[serde(default)]
    pub source_ids: Vec<String>,
}

/// The persisted `cards_manifest.json` artifact shape. Mirrors
/// Python's `cacg.index.CardsManifest`: `cards` (sorted by
/// `(reading_id, id)`), `dependency_retracted_cards`,
/// `retracted_cards`, `schema_version`. `deny_unknown_fields` mirrors
/// Python's `_StrictModel`; the two retraction lists default to `[]`
/// (Python `Field(default_factory=list)`) so a pre-Phase-4 manifest
/// that omits them still loads.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardsManifest {
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Cards sorted by `(reading_id, id)`.
    pub cards: Vec<CardManifestEntry>,
    /// Card-ids of cards retracted via `kb retract`. Empty for a
    /// fresh corpus.
    #[serde(default)]
    pub retracted_cards: Vec<String>,
    /// Card-ids of cards cascade-retracted by `kb retract-source` /
    /// `kb retract-chunk`. Empty for a fresh corpus.
    #[serde(default)]
    pub dependency_retracted_cards: Vec<String>,
}

impl CardManifestEntry {
    /// Validate one `cards_manifest.json` entry, mirroring Python
    /// `cacg.index.CardManifestEntry`'s field validators: every string
    /// field is non-empty, `card_hash` is 64-hex SHA-256, and
    /// `source_ids` entries are non-empty, unique, and sorted. `ctx`
    /// (e.g. `cards[3]`) prefixes the diagnostic so the offending entry
    /// is identifiable.
    pub(super) fn validate(&self, ctx: &str) -> Result<(), Diagnostic> {
        for (field, value) in [
            ("path", &self.path),
            ("id", &self.id),
            ("title", &self.title),
            ("reading_id", &self.reading_id),
            ("summary", &self.summary),
            ("card_hash", &self.card_hash),
        ] {
            if value.is_empty() {
                return Err(Diagnostic::new(
                    codes::MAN_001,
                    Severity::Error,
                    format!("{ctx}.{field}: must be a non-empty string"),
                ));
            }
        }
        if !is_64_hex(&self.card_hash) {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.card_hash: card_hash must be 64-hex SHA256"),
            ));
        }
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for sid in &self.source_ids {
            if sid.is_empty() {
                return Err(Diagnostic::new(
                    codes::MAN_001,
                    Severity::Error,
                    format!("{ctx}.source_ids: entries must be non-empty strings"),
                ));
            }
            if !seen.insert(sid.as_str()) {
                return Err(Diagnostic::new(
                    codes::MAN_001,
                    Severity::Error,
                    format!("{ctx}.source_ids: must be unique (got duplicate {sid:?})"),
                ));
            }
        }
        let mut sorted = self.source_ids.clone();
        sorted.sort();
        if self.source_ids != sorted {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.source_ids: must be sorted (got non-canonical ordering)"),
            ));
        }
        Ok(())
    }
}

impl CardsManifest {
    /// Validate the `cards_manifest.json` trust boundary, mirroring
    /// Python `cacg.index.CardsManifest`'s load-time validators:
    ///
    /// * every `cards[*]` entry passes [`CardManifestEntry::validate`];
    /// * active `cards[*].id` are unique
    ///   (`BL-20260518-reject-duplicate-keys-at-trust-boundary`);
    /// * `retracted_cards` and `dependency_retracted_cards` are each
    ///   non-empty / unique / sorted;
    /// * `cards.id ∩ retracted_cards = ∅` (a directly-retracted card
    ///   leaves the active set);
    /// * `retracted_cards ∩ dependency_retracted_cards = ∅` (direct
    ///   retraction subsumes cascade);
    /// * `dependency_retracted_cards ⊆ cards.id` (cascade overlays an
    ///   otherwise-active card).
    ///
    /// Every violation is a `CACG-MAN-001` [`Diagnostic`]. Consumers
    /// (`kb show`, the `kb search` retraction filter) call this right
    /// after `serde_json` deserialization so an invariant-invalid
    /// manifest is rejected before any trust decision is made.
    ///
    /// # Errors
    ///
    /// Returns the first violating [`Diagnostic`].
    pub fn validate_structurally(&self) -> Result<(), Diagnostic> {
        for (i, card) in self.cards.iter().enumerate() {
            card.validate(&format!("cards[{i}]"))?;
        }
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut duplicates: BTreeSet<&str> = BTreeSet::new();
        for card in &self.cards {
            if !seen.insert(card.id.as_str()) {
                duplicates.insert(card.id.as_str());
            }
        }
        if !duplicates.is_empty() {
            let dups: Vec<&str> = duplicates.into_iter().collect();
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!(
                    "cards_manifest invariant violated: duplicate card_id(s) in `cards`: {dups:?}"
                ),
            ));
        }
        validate_retracted_list("retracted_cards", &self.retracted_cards)?;
        validate_retracted_list(
            "dependency_retracted_cards",
            &self.dependency_retracted_cards,
        )?;
        let active: BTreeSet<&str> = self.cards.iter().map(|c| c.id.as_str()).collect();
        let retracted: BTreeSet<&str> = self.retracted_cards.iter().map(String::as_str).collect();
        let dependency: BTreeSet<&str> = self
            .dependency_retracted_cards
            .iter()
            .map(String::as_str)
            .collect();
        let active_retracted: Vec<&str> = active.intersection(&retracted).copied().collect();
        if !active_retracted.is_empty() {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!(
                    "cards_manifest invariant violated: card_id(s) {active_retracted:?} appear in both `cards` and `retracted_cards`"
                ),
            ));
        }
        let retracted_dependency: Vec<&str> =
            retracted.intersection(&dependency).copied().collect();
        if !retracted_dependency.is_empty() {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!(
                    "cards_manifest invariant violated: card_id(s) {retracted_dependency:?} appear in both `retracted_cards` and `dependency_retracted_cards`"
                ),
            ));
        }
        let dependency_outside: Vec<&str> = dependency.difference(&active).copied().collect();
        if !dependency_outside.is_empty() {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!(
                    "cards_manifest invariant violated: card_id(s) {dependency_outside:?} appear in `dependency_retracted_cards` but are not in `cards`"
                ),
            ));
        }
        Ok(())
    }
}

/// One entry in `summaries.json.summaries`. Mirrors Python's
/// `cacg.index.SummaryEntry` field-for-field. Note: `tags` is
/// included (empty by default in the parity corpus), and
/// `citation_count` is intentionally omitted (Python's `SummaryEntry`
/// does not carry it). `deny_unknown_fields` mirrors Python's
/// `_StrictModel` (`extra="forbid"`) so an unknown field is rejected
/// at deserialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummaryEntry {
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Card identifier from the frontmatter.
    pub id: String,
    /// Title from the frontmatter.
    pub title: String,
    /// Reading identifier from the frontmatter.
    pub reading_id: String,
    /// Summary text from the frontmatter.
    pub summary: String,
    /// Frontmatter `tags`. `#[serde(default)]` mirrors Python's
    /// `Field(default_factory=list)`: an omitted `tags` key loads as
    /// `[]` so a Python-valid `summaries.json` stays loadable.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Sorted-unique list of cited `source_id`s. Deliberately carries
    /// no `#[serde(default)]`: an omitted `source_ids` KEY is the
    /// `CACG-SUM-007` legacy-migration trigger, detected from the raw
    /// JSON before deserialization.
    pub source_ids: Vec<String>,
    /// Path to the card markdown file (as supplied to `build_index`).
    pub path: String,
    /// 64-hex SHA-256 recomputed during indexing.
    pub card_hash: String,
}

/// The persisted `summaries.json` artifact shape. `deny_unknown_fields`
/// mirrors Python's `_StrictModel` (`extra="forbid"`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummariesManifest {
    /// Schema version.
    pub schema_version: SchemaVersion,
    /// Summaries sorted by `(reading_id, id)`.
    pub summaries: Vec<SummaryEntry>,
}

/// Errors raised by [`super::build_index`].
#[derive(Debug, Error)]
pub enum IndexError {
    /// The `cards_dir` does not exist or is not a directory.
    #[error("cards_dir {0:?} is not a directory")]
    NonDirCardsDir(PathBuf),
    /// I/O failure reading the cards directory or writing artifacts.
    #[error("index I/O error at {path:?}: {source}")]
    Io {
        /// Path that errored.
        path: PathBuf,
        /// Underlying error.
        #[source]
        source: std::io::Error,
    },
    /// A card's frontmatter failed parse or cross-field validation.
    #[error("card {path:?} frontmatter rejected: {diagnostics:?}")]
    InvalidFrontmatter {
        /// Card path.
        path: PathBuf,
        /// One or more diagnostics from the parser/validator.
        diagnostics: Vec<Diagnostic>,
    },
    /// A card frontmatter is missing the `card_hash` field.
    #[error("card {0:?} frontmatter is missing `card_hash`; run `kb hash` first")]
    MissingCardHash(PathBuf),
    /// Canonical-JSON serialization failed.
    #[error("canonical JSON failure: {0}")]
    Canonical(String),
    /// Atomic publish of the manifest pair failed.
    #[error(transparent)]
    Publish(#[from] PublishError),
    /// Journal append failed.
    #[error(transparent)]
    Journal(#[from] JournalError),
    /// History append failed.
    #[error(transparent)]
    History(#[from] HistoryError),
}

/// Serialize a value to canonical JSON via `canonical_json`.
pub(super) fn canonical_serialize<T: Serialize>(value: &T) -> Result<String, IndexError> {
    let v = serde_json::to_value(value)
        .map_err(|e| IndexError::Canonical(format!("serde_json::to_value: {e}")))?;
    // Python writes `path.write_text(canonical_json(...))` verbatim
    // with no trailing newline; the parity byte-equal contract holds
    // only when Rust emits the same bytes.
    crate::canonical_json::canonical_json(&v)
        .map_err(|e| IndexError::Canonical(format!("canonical_json: {e}")))
}

/// Render INDEX.md matching Python's `_build_index_md`:
/// `# Card Index\n\nschema_version: cacg.v0\n\n## <reading_id>\n\n`
/// + table header + per-card rows sorted by id + blank line per group,
/// joined with `\n`, ending with a single trailing newline.
pub(crate) fn build_index_md(entries: &[CardManifestEntry]) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push("# Card Index".to_string());
    lines.push(String::new());
    lines.push("schema_version: cacg.v0".to_string());
    lines.push(String::new());

    let mut by_reading: BTreeMap<String, Vec<&CardManifestEntry>> = BTreeMap::new();
    for e in entries {
        by_reading.entry(e.reading_id.clone()).or_default().push(e);
    }
    for (reading, mut reading_entries) in by_reading {
        lines.push(format!("## {reading}"));
        lines.push(String::new());
        lines.push("| id | title | citations | card_hash |".to_string());
        lines.push("|----|-------|-----------|-----------|".to_string());
        reading_entries.sort_by(|a, b| a.id.cmp(&b.id));
        for e in reading_entries {
            // card_hash[:12] + U+2026 (horizontal ellipsis) — matches
            // Python's `f"{e.card_hash[:12]}…"` rendering.
            let hash_prefix: String = e.card_hash.chars().take(12).collect();
            lines.push(format!(
                "| {} | {} | {} | {hash_prefix}\u{2026} |",
                e.id, e.title, e.citation_count
            ));
        }
        lines.push(String::new());
    }
    format!("{}\n", lines.join("\n"))
}
