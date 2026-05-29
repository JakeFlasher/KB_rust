//! Standalone trust-kernel example. Backs the `examples/demo.rs`
//! binary and the snapshot test in `tests/demo_snapshot.rs`.
//!
//! Hidden from the public rustdoc surface (`#[doc(hidden)]` at the
//! module level) because the orchestration logic is example-specific
//! and not intended for downstream consumers — downstream callers
//! should compose the trust-kernel primitives directly.

#![doc(hidden)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use thiserror::Error;

use crate::atomic_publish::{atomic_publish, DefaultFs, PublishError, PublishMember};
use crate::canonical_json::canonical_json;
use crate::determinism::DeterminismContext;
use crate::diagnostic::{codes, Diagnostic, Severity};
use crate::frontmatter::parse_card;
use crate::hash::card_hash;
use crate::index::{CardManifestEntry, CardsManifest, SummariesManifest, SummaryEntry};
use crate::journal::{append_entry, JournalEntry};
use crate::normalize::normalize_text;
use crate::schema::SchemaVersion;

const SIDECAR_DIAG: &str = "CACG-IDX-007";
const NON_FILE_DIAG: &str = "CACG-IDX-008";

/// Errors raised by [`demo_run`].
#[derive(Debug, Error)]
pub enum DemoError {
    /// I/O failure reading or writing.
    #[error("demo I/O error at {path:?}: {source}")]
    Io {
        /// Path that errored.
        path: PathBuf,
        /// Underlying error.
        #[source]
        source: std::io::Error,
    },
    /// Frontmatter parse or validation failure.
    #[error("frontmatter rejected: {diagnostics:?}")]
    Frontmatter {
        /// Diagnostics surfaced by the parser/validator.
        diagnostics: Vec<Diagnostic>,
    },
    /// `card_hash` recompute or canonical-JSON failure.
    #[error("hash / canonical-JSON failure: {0}")]
    Hash(String),
    /// Atomic publish failure.
    #[error(transparent)]
    Publish(#[from] PublishError),
    /// Journal append failure.
    #[error(transparent)]
    Journal(#[from] crate::journal::JournalError),
    /// Manifest round-trip deserialization failure.
    #[error("manifest re-load failure: {0}")]
    Reload(String),
}

/// Exercise every public trust-kernel primitive end-to-end against a
/// single card and return a snapshot-stable string describing the
/// outcome.
///
/// Determinism: every nondeterministic origin (event id, timestamp,
/// temp-name) is threaded through `ctx`. `tempdir` is the caller's
/// chosen output directory; using `DeterminismContext::frozen()` plus
/// a constant `tempdir` path yields byte-identical output across runs.
///
/// The returned string deliberately excludes the absolute `tempdir`
/// prefix from any path it contains, so the snapshot file does not
/// embed environment-specific filesystem layouts.
pub fn demo_run(
    card_path: &Path,
    ctx: &DeterminismContext,
    tempdir: &Path,
) -> Result<String, DemoError> {
    // Step 1: load card.
    let text = std::fs::read_to_string(card_path).map_err(|source| DemoError::Io {
        path: card_path.to_path_buf(),
        source,
    })?;

    // Step 2: parse + validate schema.
    let (frontmatter, body) =
        parse_card(&text).map_err(|diagnostics| DemoError::Frontmatter { diagnostics })?;
    if let Err(d) = frontmatter.validate() {
        return Err(DemoError::Frontmatter {
            diagnostics: vec![d],
        });
    }

    // Step 3: compute card_hash + assert against frontmatter's stored value.
    let frontmatter_map = frontmatter_to_map(&frontmatter)
        .map_err(|e| DemoError::Hash(format!("frontmatter -> Map: {e}")))?;
    let computed_card_hash = card_hash(&frontmatter_map, &body)
        .map_err(|e| DemoError::Hash(format!("card_hash: {e}")))?;
    let stored_card_hash = frontmatter.card_hash.clone();
    let card_hash_match = stored_card_hash
        .as_deref()
        .map(|s| s == computed_card_hash)
        .unwrap_or(true);

    // Step 4: normalize body.
    let normalized = normalize_text(&body);

    // Step 5: append journal event.
    std::fs::create_dir_all(tempdir).map_err(|source| DemoError::Io {
        path: tempdir.to_path_buf(),
        source,
    })?;
    let journal_path = tempdir.join("lint_journal.jsonl");
    let mut verification = BTreeMap::new();
    verification.insert("fuzzy".to_string(), false);
    verification.insert("layer1".to_string(), true);
    verification.insert("layer2".to_string(), true);
    let journal_entry = JournalEntry {
        command: "demo".to_string(),
        card_path: frontmatter.id.clone(),
        card_hash_before: None,
        card_hash_after: Some(computed_card_hash.clone()),
        diagnostics: Vec::new(),
        verification,
        latency_ms: 0.0,
    };
    let event_id = ctx.new_uuid();
    let timestamp = ctx.now_iso();
    append_entry(&journal_path, &journal_entry, &event_id, &timestamp)?;

    // Step 6: atomic-publish a 1-card manifest pair.
    let manifest_entry = CardManifestEntry {
        schema_version: SchemaVersion::V0,
        path: frontmatter.id.clone(),
        id: frontmatter.id.clone(),
        title: frontmatter.title.clone(),
        reading_id: frontmatter.reading_id.clone(),
        summary: frontmatter.summary.clone(),
        card_hash: computed_card_hash.clone(),
        citation_count: 0,
        source_ids: Vec::new(),
    };
    let summary_entry = SummaryEntry {
        schema_version: SchemaVersion::V0,
        id: frontmatter.id.clone(),
        title: frontmatter.title.clone(),
        reading_id: frontmatter.reading_id.clone(),
        summary: frontmatter.summary.clone(),
        tags: Vec::new(),
        source_ids: Vec::new(),
        path: frontmatter.id.clone(),
        card_hash: computed_card_hash.clone(),
    };
    let cards_manifest = CardsManifest {
        schema_version: SchemaVersion::V0,
        cards: vec![manifest_entry.clone()],
        retracted_cards: Vec::new(),
        dependency_retracted_cards: Vec::new(),
    };
    let summaries_manifest = SummariesManifest {
        schema_version: SchemaVersion::V0,
        summaries: vec![summary_entry],
    };
    let cards_bytes = canonical_serialize(&cards_manifest)?;
    let summaries_bytes = canonical_serialize(&summaries_manifest)?;
    let manifest_path = tempdir.join("cards_manifest.json");
    let summaries_path = tempdir.join("summaries.json");
    let members = vec![
        PublishMember {
            tmp_path: manifest_path.with_extension("json.tmp"),
            bak_path: manifest_path.with_extension("json.bak"),
            canonical_path: manifest_path.clone(),
            bytes: cards_bytes.into_bytes(),
        },
        PublishMember {
            tmp_path: summaries_path.with_extension("json.tmp"),
            bak_path: summaries_path.with_extension("json.bak"),
            canonical_path: summaries_path,
            bytes: summaries_bytes.into_bytes(),
        },
    ];
    atomic_publish(&members, &DefaultFs, SIDECAR_DIAG, NON_FILE_DIAG)?;

    // Step 7: re-load and verify the published manifest.
    let reloaded = std::fs::read_to_string(&manifest_path).map_err(|source| DemoError::Io {
        path: manifest_path.clone(),
        source,
    })?;
    let parsed: CardsManifest = serde_json::from_str(&reloaded)
        .map_err(|e| DemoError::Reload(format!("CardsManifest parse: {e}")))?;
    let reloaded_card_hash = parsed
        .cards
        .first()
        .map(|c| c.card_hash.clone())
        .unwrap_or_default(); // qg-allow: intentional-discard — Option::unwrap_or_default, empty string when no cards
    let reload_roundtrip_ok = reloaded_card_hash == computed_card_hash;

    // Step 8: emit Diagnostic JSON describing the verification outcome.
    let diagnostic = if card_hash_match && reload_roundtrip_ok {
        Diagnostic::new(
            codes::CLI_001,
            Severity::Info,
            format!(
                "card {} verified end-to-end through cacg_core trust kernel",
                frontmatter.id
            ),
        )
    } else {
        Diagnostic::new(
            codes::CLI_001,
            Severity::Error,
            format!(
                "card {} verification mismatch (stored vs computed hash differ, or manifest reload mismatch)",
                frontmatter.id
            ),
        )
    };
    let diagnostic_json = serde_json::to_string(&diagnostic)
        .map_err(|e| DemoError::Hash(format!("Diagnostic serialize: {e}")))?;

    // Compose snapshot-stable output. Path-bearing lines reference
    // file basenames only so the snapshot doesn't depend on the
    // tempdir prefix.
    let body_len = normalized.chars().count();
    let report = [
        format!("=== cacg-core demo ==="),
        format!("card_id: {}", frontmatter.id),
        format!("title: {}", frontmatter.title),
        format!("reading_id: {}", frontmatter.reading_id),
        format!("computed_card_hash: {computed_card_hash}"),
        format!(
            "stored_card_hash_matches: {}",
            if card_hash_match { "true" } else { "false" }
        ),
        format!("normalized_body_chars: {body_len}"),
        format!("journal_event_id: {event_id}"),
        format!("journal_event_timestamp: {timestamp}"),
        format!("published: cards_manifest.json, summaries.json"),
        format!(
            "reload_roundtrip_ok: {}",
            if reload_roundtrip_ok { "true" } else { "false" }
        ),
        format!("diagnostic: {diagnostic_json}"),
    ]
    .join("\n");
    Ok(report)
}

/// Convert a [`crate::schema::CardFrontmatter`] back into a
/// `serde_json::Map<String, Value>` so [`crate::hash::card_hash`]
/// can canonical-JSON-hash it.
fn frontmatter_to_map(
    fm: &crate::schema::CardFrontmatter,
) -> Result<Map<String, Value>, serde_json::Error> {
    let v = serde_json::to_value(fm)?;
    #[allow(clippy::wildcard_enum_match_arm)]
    match v {
        Value::Object(m) => Ok(m),
        _ => unreachable!("CardFrontmatter serializes to an object"),
    }
}

fn canonical_serialize<T: serde::Serialize>(value: &T) -> Result<String, DemoError> {
    let v = serde_json::to_value(value)
        .map_err(|e| DemoError::Hash(format!("serde_json::to_value: {e}")))?;
    let bytes = canonical_json(&v).map_err(|e| DemoError::Hash(format!("canonical_json: {e}")))?;
    Ok(format!("{bytes}\n"))
}
