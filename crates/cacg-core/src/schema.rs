//! Rust port of the `cacg.v0` card-frontmatter schema mirroring Python
//! `legacy_python_oracle/src/cacg/schema.py` (Pydantic v2 with `extra="forbid"`).
//!
//! Three nested struct types form the strict card-frontmatter shape:
//!
//! - [`CardFrontmatter`] — the top-level card frontmatter.
//! - [`Citation`] — one entry of `citations`.
//! - [`CardEdge`] — one entry of `card_edges`.
//!
//! Each uses `#[serde(deny_unknown_fields)]` so a typo in a field name
//! surfaces immediately. Cross-field invariants (chunk_hash 64-hex,
//! page_range ascending, tags slug regex / count, etc.) live in
//! [`CardFrontmatter::validate`] and are surfaced as
//! [`crate::diagnostic::Diagnostic`] entries carrying the CACG-* codes
//! documented in `docs/lint-codes.md`.
//!
//! The full `parse_card` flow that combines YAML pre-scanning (anchor /
//! alias / tag / duplicate-key rejection) with this schema lives in
//! `crate::frontmatter`. This module owns only the data shapes + the
//! `validate` cross-field rules; the loader composes them with
//! `serde_yaml_bw` and `yaml-rust2`.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::diagnostic::{codes, Diagnostic, Severity};
use crate::normalize::normalize_text;

/// Schema version literal. Currently only `cacg.v0` is admissible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchemaVersion {
    /// `cacg.v0` (the current and only supported version).
    #[serde(rename = "cacg.v0")]
    V0,
}

impl SchemaVersion {
    /// Canonical string form of this schema version.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V0 => "cacg.v0",
        }
    }
}

/// Edge classifier for a citation (`Citation.edge_type`). Mirrors
/// Python `CitationEdge = Literal["supports","defines","extends",
/// "contrasts_with","depends_on","applies_to"]` exactly (R16 fix per
/// Codex R15-REVIEW-2: R15 had the wrong 4-value set).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationEdge {
    /// The citation directly supports the card's claim.
    Supports,
    /// The citation defines a term the card uses.
    Defines,
    /// The citation extends the card's coverage with a related claim.
    Extends,
    /// The citation contrasts with the card's claim.
    ContrastsWith,
    /// The card depends on the cited chunk's claim.
    DependsOn,
    /// The card applies the cited chunk to a new context.
    AppliesTo,
}

/// Edge classifier for a card-to-card relation (`CardEdge.edge_type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardEdgeType {
    /// This card depends on the target card.
    DependsOn,
    /// This card extends the target card's coverage.
    Extends,
}

/// Bounds matching Python's `SUMMARY_MIN_LENGTH` / `SUMMARY_MAX_LENGTH`
/// (locked by DEC-11 in the Phase-3 plan).
pub const SUMMARY_MIN_LENGTH: usize = 80;
/// Upper bound for `summary`; oversized maps to CACG-SUM-002.
pub const SUMMARY_MAX_LENGTH: usize = 400;
/// Per-tag minimum length.
pub const TAG_MIN_LENGTH: usize = 2;
/// Per-tag maximum length.
pub const TAG_MAX_LENGTH: usize = 40;
/// Maximum number of tags per card.
pub const TAGS_MAX_COUNT: usize = 10;

/// One entry of `CardFrontmatter.card_edges`: an edge from this card to
/// another card by `id`. `target` is another card's `id`, NOT a chunk_id
/// or source_id. Validation that the target card exists in the live
/// `cards_manifest` is performed at lint time (CACG-DEP-001), not here.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardEdge {
    /// Target card's `id`.
    pub target: String,
    /// Relation classifier. Defaults to `depends_on`.
    #[serde(default = "default_card_edge_type")]
    pub edge_type: CardEdgeType,
}

fn default_card_edge_type() -> CardEdgeType {
    CardEdgeType::DependsOn
}

/// One entry of `CardFrontmatter.citations`: pins one piece of card
/// claim to one chunk of source text by hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Citation {
    /// Source identifier (matches the `source_id` of a `SourceRecord`).
    pub source_id: String,
    /// Chunk identifier (matches the `chunk_id` of a `ChunkRecord`).
    pub chunk_id: String,
    /// 64-hex SHA-256 of the chunk envelope (see [`crate::hash::chunk_hash`]).
    pub chunk_hash: String,
    /// `[start_page, end_page]` (1-based, inclusive). The two-element
    /// constraint is enforced by `validate` since `Vec<i64>` is the
    /// permissive deserialization target.
    pub page_range: Vec<i64>,
    /// Quoted text from the chunk.
    pub quote: String,
    /// Edge classifier. Defaults to `supports`.
    #[serde(default = "default_citation_edge")]
    pub edge_type: CitationEdge,
}

fn default_citation_edge() -> CitationEdge {
    CitationEdge::Supports
}

/// Top-level frontmatter of a card. All nine fields below are the strict
/// `cacg.v0` schema; `#[serde(deny_unknown_fields)]` mirrors Python's
/// `extra="forbid"` so any typo surfaces as CACG-FM-003.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardFrontmatter {
    /// Schema version (must be `cacg.v0`).
    pub schema_version: SchemaVersion,
    /// Card identifier (non-empty).
    pub id: String,
    /// Human-readable title (non-empty).
    pub title: String,
    /// Reading identifier (non-empty).
    pub reading_id: String,
    /// Card summary; length in [80, 400] per DEC-11.
    pub summary: String,
    /// Phase-3 retrieval-surface tags (default: empty list).
    #[serde(default)]
    pub tags: Vec<String>,
    /// Card-to-card edges (default: empty list).
    #[serde(default)]
    pub card_edges: Vec<CardEdge>,
    /// Citations binding card claims to chunks. Must be non-empty.
    pub citations: Vec<Citation>,
    /// Card hash, set by `kb index`. Absent on freshly authored cards.
    #[serde(default)]
    pub card_hash: Option<String>,
}

impl CardFrontmatter {
    /// Run cross-field validators (the parts Serde cannot express
    /// natively): chunk_hash 64-hex, page_range shape + ordering, quote
    /// non-empty after normalization, summary length bounds, tags slug
    /// regex / count, card_edges duplicate-pair rejection, card_hash
    /// 64-hex if present.
    ///
    /// Returns the FIRST cross-field violation as a single Diagnostic
    /// (matching Python's behavior where Pydantic's first error
    /// drives the diagnostic mapping).
    pub fn validate(&self) -> Result<(), Diagnostic> {
        // Summary length bounds (CACG-SUM-001 / CACG-SUM-002).
        if self.summary.chars().count() < SUMMARY_MIN_LENGTH {
            return Err(Diagnostic::new(
                codes::SUM_001,
                Severity::Error,
                format!(
                    "summary missing or too short: String should have at least {SUMMARY_MIN_LENGTH} characters"
                ),
            ));
        }
        if self.summary.chars().count() > SUMMARY_MAX_LENGTH {
            return Err(Diagnostic::new(
                codes::SUM_002,
                Severity::Error,
                format!(
                    "summary oversized: String should have at most {SUMMARY_MAX_LENGTH} characters"
                ),
            ));
        }

        // card_hash 64-hex if present.
        if let Some(ref h) = self.card_hash {
            if !is_64_hex(h) {
                return Err(Diagnostic::new(
                    codes::FM_008,
                    Severity::Error,
                    format!(
                        "validation error at card_hash: Value error, card_hash must be 64-hex SHA256 (or absent for not-yet-indexed cards)"
                    ),
                ));
            }
        }

        // citations non-empty (matches Python `min_length=1`).
        if self.citations.is_empty() {
            return Err(Diagnostic::new(
                codes::FM_008,
                Severity::Error,
                "validation error at citations: List should have at least 1 item after validation, not 0".to_string(),
            ));
        }

        // Per-citation cross-field validators.
        for (idx, cit) in self.citations.iter().enumerate() {
            if let Err(d) = cit.validate(idx) {
                return Err(d);
            }
        }

        // Empty non-empty string fields (Pydantic min_length=1).
        for (field, value) in [
            ("id", &self.id),
            ("title", &self.title),
            ("reading_id", &self.reading_id),
        ] {
            if value.is_empty() {
                return Err(Diagnostic::new(
                    codes::FM_008,
                    Severity::Error,
                    format!("validation error at {field}: String should have at least 1 character"),
                ));
            }
        }

        // tags slug regex + length + duplicate + count.
        if self.tags.len() > TAGS_MAX_COUNT {
            return Err(Diagnostic::new(
                codes::SUM_004,
                Severity::Error,
                format!("tags oversized: List should have at most {TAGS_MAX_COUNT} entries"),
            ));
        }
        let mut seen_tags: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        for tag in &self.tags {
            let len = tag.chars().count();
            if !(TAG_MIN_LENGTH..=TAG_MAX_LENGTH).contains(&len) {
                return Err(Diagnostic::new(
                    codes::SUM_003,
                    Severity::Error,
                    format!(
                        "tag non-conforming: tag {tag:?} length {len} outside [{TAG_MIN_LENGTH}, {TAG_MAX_LENGTH}]"
                    ),
                ));
            }
            if !is_tag_slug(tag) {
                return Err(Diagnostic::new(
                    codes::SUM_003,
                    Severity::Error,
                    format!("tag non-conforming: tag {tag:?} does not match slug regex"),
                ));
            }
            if !seen_tags.insert(tag.as_str()) {
                return Err(Diagnostic::new(
                    codes::SUM_003,
                    Severity::Error,
                    format!("tag non-conforming: tags contains duplicate entry {tag:?}"),
                ));
            }
        }

        // card_edges duplicate-pair rejection + per-edge target min_length=1.
        let mut seen_edges: std::collections::BTreeSet<(String, &str)> =
            std::collections::BTreeSet::new();
        for edge in &self.card_edges {
            if edge.target.is_empty() {
                return Err(Diagnostic::new(
                    codes::FM_008,
                    Severity::Error,
                    "validation error at card_edges: Value error, target must be a non-empty string".to_string(),
                ));
            }
            let et = match edge.edge_type {
                CardEdgeType::DependsOn => "depends_on",
                CardEdgeType::Extends => "extends",
            };
            if !seen_edges.insert((edge.target.clone(), et)) {
                return Err(Diagnostic::new(
                    codes::FM_008,
                    Severity::Error,
                    format!(
                        "validation error at card_edges: Value error, card_edges contains duplicate entry target={:?} edge_type={:?}",
                        edge.target, et
                    ),
                ));
            }
        }

        Ok(())
    }
}

impl Citation {
    /// Validate a single citation's cross-field invariants. `idx` is
    /// the 0-based position in `CardFrontmatter.citations` for the
    /// diagnostic `loc` path.
    pub fn validate(&self, idx: usize) -> Result<(), Diagnostic> {
        // chunk_hash 64-hex (CACG-CITE-002).
        if !is_64_hex(&self.chunk_hash) {
            return Err(Diagnostic::new(
                codes::CITE_002,
                Severity::Error,
                format!(
                    "citations.{idx}.chunk_hash: chunk_hash must be 64-hex SHA256 (Value error, chunk_hash must be 64-hex SHA256)"
                ),
            ));
        }
        // page_range: 2-element list of i64, both >= 1, ascending.
        if self.page_range.len() != 2 {
            return Err(Diagnostic::new(
                codes::CITE_003,
                Severity::Error,
                format!(
                    "citations.{idx}.page_range: page_range must be ascending start<=end (Value error, expected exactly 2 elements; got {})",
                    self.page_range.len()
                ),
            ));
        }
        let start = self.page_range[0];
        let end = self.page_range[1];
        if start < 1 || end < 1 {
            return Err(Diagnostic::new(
                codes::CITE_003,
                Severity::Error,
                format!(
                    "citations.{idx}.page_range: page_range must be ascending start<=end (Value error, page_range entries must be >= 1)"
                ),
            ));
        }
        if end < start {
            return Err(Diagnostic::new(
                codes::CITE_003,
                Severity::Error,
                format!(
                    "citations.{idx}.page_range: page_range must be ascending start<=end (Value error, page_range must be ascending (start <= end))"
                ),
            ));
        }
        // source_id + chunk_id + quote non-empty.
        if self.source_id.is_empty() {
            return Err(Diagnostic::new(
                codes::FM_008,
                Severity::Error,
                format!(
                    "validation error at citations.{idx}.source_id: String should have at least 1 character"
                ),
            ));
        }
        if self.chunk_id.is_empty() {
            return Err(Diagnostic::new(
                codes::FM_008,
                Severity::Error,
                format!(
                    "validation error at citations.{idx}.chunk_id: String should have at least 1 character"
                ),
            ));
        }
        // quote non-empty AND non-empty after normalization (whitespace-only
        // quotes are rejected per the Python `_quote_non_empty_after_normalize`
        // validator; same code path emits CACG-FM-008 since the
        // diagnostic mapper doesn't have a dedicated CITE-* code for this).
        if self.quote.is_empty() {
            return Err(Diagnostic::new(
                codes::FM_008,
                Severity::Error,
                format!(
                    "validation error at citations.{idx}.quote: String should have at least 1 character"
                ),
            ));
        }
        if normalize_text(&self.quote).is_empty() {
            return Err(Diagnostic::new(
                codes::FM_008,
                Severity::Error,
                format!(
                    "validation error at citations.{idx}.quote: Value error, quote must contain non-whitespace text after normalization"
                ),
            ));
        }
        Ok(())
    }
}

/// One page's contribution to a chunk: page number plus offset within
/// `ChunkRecord.text`. Mirrors Python `cacg.schema.PageSpan`.
///
/// Naming caveat: `byte_offset_in_chunk` is named for spec compatibility
/// but the value is a CHARACTER index into `chunk.text` (Python `str`
/// positions). Layer-2 slices `chunk.text` with these offsets via
/// `chars().skip(start).take(end - start).collect()`, which matches
/// Python's `chunk_text[window[0]:window[1]]` on Unicode chunks. A
/// future schema bump (`cacg.v1`) may rename the field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PageSpan {
    /// 1-based page number.
    pub page: u32,
    /// Character offset into the containing chunk's `text`.
    pub byte_offset_in_chunk: u32,
}

/// One entry of `ChunksManifest.chunks`. Mirrors Python
/// `cacg.schema.ChunkRecord` field-for-field. Cross-field validators
/// (run via `ChunksManifest::validate_structurally`) enforce:
///
/// 1. `chunk_hash` is 64-hex SHA-256.
/// 2. `end_page >= start_page`.
/// 3. `page_spans` is non-empty.
/// 4. `byte_offset_in_chunk` values are strictly monotonic ascending.
/// 5. Each `byte_offset_in_chunk <= len(text)` (character count).
/// 6. Each span's `page` falls within `[start_page, end_page]`.
/// 7. No two spans share the same `page`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChunkRecord {
    /// Schema version (must be `cacg.v0`).
    pub schema_version: SchemaVersion,
    /// Source identifier (non-empty).
    pub source_id: String,
    /// Chunk identifier (non-empty).
    pub chunk_id: String,
    /// 64-hex SHA-256 of the chunk envelope.
    pub chunk_hash: String,
    /// 0-based ordinal of this chunk within its source.
    pub ordinal: u32,
    /// First page this chunk covers (1-based, inclusive).
    pub start_page: u32,
    /// Last page this chunk covers (1-based, inclusive).
    pub end_page: u32,
    /// Per-page offsets into `text` (non-empty, strictly monotonic).
    pub page_spans: Vec<PageSpan>,
    /// Approximate token count (>= 1).
    pub token_count: u32,
    /// Chunk's normalized text (non-empty).
    pub text: String,
    /// Short preview of `text` for diagnostic surfacing.
    pub text_preview: String,
}

impl ChunkRecord {
    /// Validate cross-field invariants for one chunk (called by
    /// `ChunksManifest::validate_structurally`). Returns the first
    /// violation as a diagnostic with `CACG-MAN-001`.
    pub fn validate(&self, ctx: &str) -> Result<(), Diagnostic> {
        if !is_64_hex(&self.chunk_hash) {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.chunk_hash: chunk_hash must be 64-hex SHA256"),
            ));
        }
        if self.source_id.is_empty() {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.source_id: must be a non-empty string"),
            ));
        }
        if self.chunk_id.is_empty() {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.chunk_id: must be a non-empty string"),
            ));
        }
        if self.start_page < 1 || self.end_page < 1 {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}: start_page and end_page must be >= 1"),
            ));
        }
        if self.end_page < self.start_page {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}: end_page must be >= start_page"),
            ));
        }
        if self.token_count < 1 {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.token_count: must be >= 1"),
            ));
        }
        if self.text.is_empty() {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.text: must be non-empty"),
            ));
        }
        if self.page_spans.is_empty() {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.page_spans: must contain at least one span"),
            ));
        }
        let text_len = u32::try_from(self.text.chars().count()).unwrap_or(u32::MAX);
        let mut prior_offset: Option<u32> = None;
        let mut seen_pages: BTreeSet<u32> = BTreeSet::new();
        for (i, span) in self.page_spans.iter().enumerate() {
            if let Some(p) = prior_offset {
                if span.byte_offset_in_chunk <= p {
                    return Err(Diagnostic::new(
                        codes::MAN_001,
                        Severity::Error,
                        format!(
                            "{ctx}.page_spans[{i}].byte_offset_in_chunk {} is not strictly greater than the prior span ({p})",
                            span.byte_offset_in_chunk
                        ),
                    ));
                }
            }
            if span.byte_offset_in_chunk > text_len {
                return Err(Diagnostic::new(
                    codes::MAN_001,
                    Severity::Error,
                    format!(
                        "{ctx}.page_spans[{i}].byte_offset_in_chunk {} exceeds len(text)={text_len}",
                        span.byte_offset_in_chunk
                    ),
                ));
            }
            if span.page < self.start_page || span.page > self.end_page {
                return Err(Diagnostic::new(
                    codes::MAN_001,
                    Severity::Error,
                    format!(
                        "{ctx}.page_spans[{i}].page {} falls outside [start_page={}, end_page={}]",
                        span.page, self.start_page, self.end_page
                    ),
                ));
            }
            if !seen_pages.insert(span.page) {
                return Err(Diagnostic::new(
                    codes::MAN_001,
                    Severity::Error,
                    format!("{ctx}.page_spans contains duplicate page {}", span.page),
                ));
            }
            prior_offset = Some(span.byte_offset_in_chunk);
        }
        Ok(())
    }
}

/// One entry of `SourcesManifest.sources`. Mirrors Python
/// `cacg.schema.SourceRecord`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRecord {
    /// Schema version (must be `cacg.v0`).
    pub schema_version: SchemaVersion,
    /// Source identifier (non-empty).
    pub source_id: String,
    /// Path to the source artifact (non-empty).
    pub source_path: String,
    /// 64-hex SHA-256 of the source bytes.
    pub source_sha256: String,
    /// Parser name (non-empty).
    pub parser_name: String,
    /// Parser version (non-empty).
    pub parser_version: String,
    /// Number of pages (>= 1).
    pub page_count: u32,
    /// ISO-8601 extraction timestamp (non-empty).
    pub extracted_at: String,
}

impl SourceRecord {
    /// Cross-field validators: `source_sha256` 64-hex, non-empty
    /// strings, `page_count >= 1`.
    pub fn validate(&self, ctx: &str) -> Result<(), Diagnostic> {
        if !is_64_hex(&self.source_sha256) {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.source_sha256: source_sha256 must be 64-hex SHA256"),
            ));
        }
        if self.source_id.is_empty()
            || self.source_path.is_empty()
            || self.parser_name.is_empty()
            || self.parser_version.is_empty()
            || self.extracted_at.is_empty()
        {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}: non-empty string fields are required"),
            ));
        }
        if self.page_count < 1 {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{ctx}.page_count: must be >= 1"),
            ));
        }
        Ok(())
    }
}

/// The persisted `sources_manifest.json` artifact shape. Mirrors
/// Python `cacg.schema.SourcesManifest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcesManifest {
    /// Schema version (must be `cacg.v0`).
    pub schema_version: SchemaVersion,
    /// Sources contained in this manifest.
    pub sources: Vec<SourceRecord>,
}

impl SourcesManifest {
    /// Validate every source record; return the first violation.
    pub fn validate_structurally(&self) -> Result<(), Diagnostic> {
        for (i, s) in self.sources.iter().enumerate() {
            s.validate(&format!("sources[{i}]"))?;
        }
        Ok(())
    }
}

/// The persisted `chunks_manifest.json` artifact shape. Mirrors
/// Python `cacg.schema.ChunksManifest`. Field default policy matches
/// Python: `retracted_source_ids` and `retracted_chunk_ids` default to
/// empty when omitted from the JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChunksManifest {
    /// Schema version (must be `cacg.v0`).
    pub schema_version: SchemaVersion,
    /// Active chunks (validated by `validate_structurally`).
    pub chunks: Vec<ChunkRecord>,
    /// Sorted, unique, non-empty list of retracted source_ids.
    #[serde(default)]
    pub retracted_source_ids: Vec<String>,
    /// Sorted, unique, non-empty list of retracted chunk_ids.
    #[serde(default)]
    pub retracted_chunk_ids: Vec<String>,
}

impl ChunksManifest {
    /// Run cross-field validators mirroring Python's `_sorted_unique`
    /// + `_validate_retraction_disjointness` model validators.
    pub fn validate_structurally(&self) -> Result<(), Diagnostic> {
        validate_retracted_list("retracted_source_ids", &self.retracted_source_ids)?;
        validate_retracted_list("retracted_chunk_ids", &self.retracted_chunk_ids)?;
        for (i, c) in self.chunks.iter().enumerate() {
            c.validate(&format!("chunks[{i}]"))?;
        }
        let active_source_ids: BTreeSet<&str> =
            self.chunks.iter().map(|c| c.source_id.as_str()).collect();
        let active_chunk_ids: BTreeSet<&str> =
            self.chunks.iter().map(|c| c.chunk_id.as_str()).collect();
        let retracted_source: BTreeSet<&str> = self
            .retracted_source_ids
            .iter()
            .map(String::as_str)
            .collect();
        let retracted_chunk: BTreeSet<&str> = self
            .retracted_chunk_ids
            .iter()
            .map(String::as_str)
            .collect();
        let source_overlap: Vec<&&str> = active_source_ids
            .intersection(&retracted_source)
            .collect::<Vec<_>>();
        if !source_overlap.is_empty() {
            let mut overlap_strs: Vec<&str> = source_overlap.into_iter().copied().collect();
            overlap_strs.sort_unstable();
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!(
                    "chunks_manifest invariant violated: source_id(s) {overlap_strs:?} appear in both `chunks[*].source_id` and `retracted_source_ids`"
                ),
            ));
        }
        let chunk_overlap: Vec<&&str> = active_chunk_ids
            .intersection(&retracted_chunk)
            .collect::<Vec<_>>();
        if !chunk_overlap.is_empty() {
            let mut overlap_strs: Vec<&str> = chunk_overlap.into_iter().copied().collect();
            overlap_strs.sort_unstable();
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!(
                    "chunks_manifest invariant violated: chunk_id(s) {overlap_strs:?} appear in both `chunks[*].chunk_id` and `retracted_chunk_ids`"
                ),
            ));
        }
        Ok(())
    }
}

pub(crate) fn validate_retracted_list(field: &str, items: &[String]) -> Result<(), Diagnostic> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut duplicates: Vec<&str> = Vec::new();
    for item in items {
        if item.is_empty() {
            return Err(Diagnostic::new(
                codes::MAN_001,
                Severity::Error,
                format!("{field}: retracted ids must be non-empty strings"),
            ));
        }
        if !seen.insert(item.as_str()) {
            duplicates.push(item.as_str());
        }
    }
    if !duplicates.is_empty() {
        let mut dedup: BTreeSet<&str> = BTreeSet::new();
        for d in duplicates {
            dedup.insert(d);
        }
        let sorted: Vec<&str> = dedup.into_iter().collect();
        return Err(Diagnostic::new(
            codes::MAN_001,
            Severity::Error,
            format!("{field}: duplicate retracted id(s): {sorted:?}"),
        ));
    }
    let mut sorted_copy: Vec<&str> = items.iter().map(String::as_str).collect();
    sorted_copy.sort_unstable();
    let original: Vec<&str> = items.iter().map(String::as_str).collect();
    if original != sorted_copy {
        return Err(Diagnostic::new(
            codes::MAN_001,
            Severity::Error,
            format!("{field}: retracted ids must be sorted (got non-canonical ordering)"),
        ));
    }
    Ok(())
}

/// Source-authorization matrix: per-reading allow-list of source_ids.
/// Mirrors Python `cacg.schema.SourceMatrix`. `BTreeMap` keys serialize
/// in canonical sorted order so `dump_source_matrix` is byte-stable.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceMatrix {
    /// Schema version (must be `cacg.v0`).
    pub schema_version: SchemaVersion,
    /// Per-reading allow-list: `reading_id -> [source_id, ...]`.
    #[serde(default)]
    pub allowed: BTreeMap<String, Vec<String>>,
}

impl SourceMatrix {
    /// Validate per-reading allow-list: non-empty `reading_id` keys,
    /// non-empty + unique `source_id` entries within each list.
    pub fn validate_structurally(&self) -> Result<(), Diagnostic> {
        for (reading_id, source_ids) in &self.allowed {
            if reading_id.is_empty() {
                return Err(Diagnostic::new(
                    codes::AUTH_000,
                    Severity::Error,
                    "reading_id keys must be non-empty strings".to_string(),
                ));
            }
            let mut seen: BTreeSet<&str> = BTreeSet::new();
            for sid in source_ids {
                if sid.is_empty() {
                    return Err(Diagnostic::new(
                        codes::AUTH_000,
                        Severity::Error,
                        format!("allowed[{reading_id:?}] contains an empty source_id"),
                    ));
                }
                if !seen.insert(sid.as_str()) {
                    return Err(Diagnostic::new(
                        codes::AUTH_000,
                        Severity::Error,
                        format!("allowed[{reading_id:?}] contains duplicate source_id {sid:?}"),
                    ));
                }
            }
        }
        Ok(())
    }
}

/// SHA-256 64-hex predicate (matches Python `_is_64_hex`): exactly 64
/// lowercase-hex characters. `pub` so `cacg-search`'s `summaries.json`
/// validator can reuse the one canonical `card_hash` predicate rather
/// than fork its own.
#[must_use]
pub fn is_64_hex(s: &str) -> bool {
    s.len() == 64
        && s.chars()
            .all(|c| c.is_ascii_digit() || matches!(c, 'a'..='f'))
}

/// Tag slug regex predicate `^[a-z0-9][a-z0-9_-]*$` (matches Python
/// `_TAG_SLUG_RE`).
fn is_tag_slug(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn is_64_hex_checks() {
        assert!(is_64_hex(
            "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
        ));
        assert!(!is_64_hex("not-a-real-sha256-hash-at-all"));
        assert!(!is_64_hex("")); // empty
        assert!(!is_64_hex(
            "1694A2598D8402EE06F8C0BE6A1DEFD8A3F85B2D7F896C25FA67B8C671A37895"
        )); // uppercase
        assert!(!is_64_hex(
            "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a3789"
        )); // 63 chars
    }

    #[test]
    fn citation_edge_six_valid_variants() {
        // Codex R15-REVIEW-2: every value Python's CitationEdge Literal
        // accepts must deserialize cleanly here.
        for s in [
            "\"supports\"",
            "\"defines\"",
            "\"extends\"",
            "\"contrasts_with\"",
            "\"depends_on\"",
            "\"applies_to\"",
        ] {
            let r: Result<CitationEdge, _> = serde_json::from_str(s);
            assert!(r.is_ok(), "citation_edge {s} must deserialize: {r:?}");
        }
    }

    #[test]
    fn citation_edge_rejects_old_round15_variants() {
        // Codex R15-REVIEW-2: `refutes` and `contextualizes` (R15's
        // wrong variants) must NOT deserialize -- they are not in
        // Python's vocabulary. Same for any arbitrary unknown value.
        for s in [
            "\"refutes\"",
            "\"contextualizes\"",
            "\"arbitrary-unknown-value\"",
        ] {
            let r: Result<CitationEdge, _> = serde_json::from_str(s);
            assert!(
                r.is_err(),
                "citation_edge {s} must NOT deserialize (not in Python's set)"
            );
        }
    }

    #[test]
    fn tag_slug_checks() {
        assert!(is_tag_slug("foo"));
        assert!(is_tag_slug("foo-bar"));
        assert!(is_tag_slug("foo_bar"));
        assert!(is_tag_slug("a"));
        assert!(is_tag_slug("9first"));
        assert!(!is_tag_slug("Foo")); // uppercase
        assert!(!is_tag_slug("_foo")); // starts with _
        assert!(!is_tag_slug("-foo")); // starts with -
        assert!(!is_tag_slug("")); // empty
    }
}
