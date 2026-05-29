//! Read-side retraction enforcement mirroring Python `legacy_python_oracle/src/cacg/retraction.py`.
//!
//! `kb retract` writes a card's id into `cards_manifest.retracted_cards`.
//! At verify time the runner consults this set so the same card_id is
//! rejected uniformly across `kb verify <card>`, `kb verify --round-summary`,
//! and any other batch verify call site that funnels through
//! `verify_one_card`.
//!
//! The `--allow-retracted` flag downgrades CACG-RETR-001 from
//! severity="error" to severity="warning" without changing the
//! diagnostic code, message, or journal-event structure.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::index::CardsManifest;

/// Raised when `cards_manifest.json` exists but cannot be parsed for
/// retraction state. Distinct from "manifest absent" so callers don't
/// silently fail open on a malformed manifest.
#[derive(Debug, Error)]
pub enum RetractionLoadError {
    /// I/O failure reading the manifest file.
    #[error("cards_manifest.json is invalid: {path}: {source}")]
    Io {
        /// Path of the manifest.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// JSON parse / schema validation failure.
    #[error("cards_manifest.json is invalid: {0}")]
    Parse(String),
}

/// Shared per-batch retraction state.
///
/// `retracted_ids` is the sorted-unique set of card_ids withdrawn via
/// `kb retract`. `allow_retracted` is the CLI flag that downgrades
/// CACG-RETR-001 from `error` to `warning` but still emits the
/// diagnostic + journal event.
#[derive(Debug, Clone, Default)]
pub struct RetractionSpec {
    /// Sorted-unique set of retracted card ids.
    pub retracted_ids: BTreeSet<String>,
    /// `--allow-retracted` flag pass-through.
    pub allow_retracted: bool,
}

impl RetractionSpec {
    /// True iff there are retracted ids OR the warning-downgrade flag
    /// is set (matches Python `RetractionSpec.enabled`).
    #[must_use]
    pub fn enabled(&self) -> bool {
        !self.retracted_ids.is_empty() || self.allow_retracted
    }

    /// True iff `card_id` is in the retracted set.
    #[must_use]
    pub fn is_retracted(&self, card_id: &str) -> bool {
        self.retracted_ids.contains(card_id)
    }

    /// Strict load: missing file yields a disabled spec; present-but-
    /// malformed raises [`RetractionLoadError`]. Mirrors Python
    /// `RetractionSpec.from_cards_manifest`.
    ///
    /// # Errors
    ///
    /// Returns `Err(RetractionLoadError::Io)` when the file is present
    /// but unreadable, and `Err(RetractionLoadError::Parse)` when JSON
    /// parsing or `CardsManifest::validate_structurally` fails.
    pub fn from_cards_manifest(
        cards_manifest_path: impl AsRef<Path>,
        allow_retracted: bool,
    ) -> Result<Self, RetractionLoadError> {
        let path = cards_manifest_path.as_ref();
        if !path.is_file() {
            return Ok(Self {
                retracted_ids: BTreeSet::new(),
                allow_retracted,
            });
        }
        let raw = std::fs::read_to_string(path).map_err(|source| RetractionLoadError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let manifest: CardsManifest =
            serde_json::from_str(&raw).map_err(|e| RetractionLoadError::Parse(e.to_string()))?;
        // `serde_json::from_str` enforces only JSON shape; the Python
        // reference loads via `CardsManifest.model_validate_json`, which
        // also runs the Pydantic invariant validators. Run the ported
        // `validate_structurally` so an invariant-invalid manifest
        // (e.g. a duplicate active card_id, or a card in both `cards`
        // and `retracted_cards`) fails closed here instead of yielding
        // a retraction decision Python would refuse to make.
        manifest
            .validate_structurally()
            .map_err(|d| RetractionLoadError::Parse(d.message))?;
        let retracted_ids: BTreeSet<String> = manifest.retracted_cards.into_iter().collect();
        Ok(Self {
            retracted_ids,
            allow_retracted,
        })
    }

    /// Lenient load: covers ONLY the missing-file case. A present-but-
    /// malformed manifest still raises [`RetractionLoadError`] so the
    /// caller (typically `cacg-cli`) maps it to a per-card
    /// `CACG-MAN-001` diagnostic (fail-closed at the trust boundary).
    /// Mirrors Python `RetractionSpec.from_cards_manifest_lenient`
    /// post-Phase-4 fail-closed-on-malformed behavior.
    ///
    /// # Errors
    ///
    /// Same as [`Self::from_cards_manifest`]. The "lenient" qualifier
    /// covers only the missing-file case; malformed manifests still
    /// propagate.
    pub fn from_cards_manifest_lenient(
        cards_manifest_path: impl AsRef<Path>,
        allow_retracted: bool,
    ) -> Result<Self, RetractionLoadError> {
        Self::from_cards_manifest(cards_manifest_path, allow_retracted)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn absent_manifest_returns_disabled_spec_lenient() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nope.json");
        let spec = RetractionSpec::from_cards_manifest_lenient(&path, false).unwrap();
        assert!(spec.retracted_ids.is_empty());
        assert!(!spec.allow_retracted);
        assert!(!spec.enabled());
    }

    #[test]
    fn absent_manifest_with_allow_flag_is_enabled() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nope.json");
        let spec = RetractionSpec::from_cards_manifest_lenient(&path, true).unwrap();
        assert!(spec.retracted_ids.is_empty());
        assert!(spec.allow_retracted);
        assert!(spec.enabled());
    }

    #[test]
    fn malformed_manifest_fails_closed_lenient() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "{not valid json").unwrap();
        let err = RetractionSpec::from_cards_manifest_lenient(&path, false)
            .expect_err("malformed manifest must NOT silently disable retraction");
        match err {
            RetractionLoadError::Parse(_) => {}
            RetractionLoadError::Io { .. } => {
                panic!("expected a Parse error, got an Io error")
            }
        }
    }

    #[test]
    fn malformed_manifest_fails_closed_strict() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "[]").unwrap();
        let err = RetractionSpec::from_cards_manifest(&path, false)
            .expect_err("schema-invalid JSON must raise");
        match err {
            RetractionLoadError::Parse(_) => {}
            RetractionLoadError::Io { .. } => {
                panic!("expected a Parse error, got an Io error")
            }
        }
    }

    #[test]
    fn valid_manifest_extracts_retracted_ids() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cards_manifest.json");
        let manifest = serde_json::json!({
            "schema_version": "cacg.v0",
            "cards": [],
            "retracted_cards": ["card-a", "card-b"],
            "dependency_retracted_cards": [],
        });
        std::fs::write(&path, serde_json::to_string(&manifest).unwrap()).unwrap();
        let spec = RetractionSpec::from_cards_manifest_lenient(&path, false).unwrap();
        assert!(spec.is_retracted("card-a"));
        assert!(spec.is_retracted("card-b"));
        assert!(!spec.is_retracted("card-c"));
        assert!(spec.enabled());
    }

    #[test]
    fn duplicate_active_card_id_manifest_fails_closed() {
        // A `cards_manifest.json` with a duplicate active `cards[*].id`
        // is JSON-shape-valid (it deserializes) but violates a
        // `CardsManifest` invariant. Python's
        // `RetractionSpec.from_cards_manifest` loads via
        // `CardsManifest.model_validate_json` and rejects it; Rust must
        // fail closed identically rather than build a `RetractionSpec`.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cards_manifest.json");
        let card_hash = "0".repeat(64);
        let manifest = serde_json::json!({
            "schema_version": "cacg.v0",
            "cards": [
                {
                    "schema_version": "cacg.v0",
                    "path": "cards/a.md",
                    "id": "dup-card",
                    "title": "Title A",
                    "reading_id": "reading_01",
                    "summary": "Summary A",
                    "card_hash": card_hash,
                    "citation_count": 0,
                    "source_ids": [],
                },
                {
                    "schema_version": "cacg.v0",
                    "path": "cards/b.md",
                    "id": "dup-card",
                    "title": "Title B",
                    "reading_id": "reading_01",
                    "summary": "Summary B",
                    "card_hash": card_hash,
                    "citation_count": 0,
                    "source_ids": [],
                },
            ],
            "retracted_cards": [],
            "dependency_retracted_cards": [],
        });
        std::fs::write(&path, serde_json::to_string(&manifest).unwrap()).unwrap();
        let err = RetractionSpec::from_cards_manifest(&path, false)
            .expect_err("a duplicate active card_id manifest must fail closed");
        match err {
            RetractionLoadError::Parse(msg) => assert!(
                msg.contains("duplicate card_id"),
                "expected a structural-validation message, got {msg:?}"
            ),
            RetractionLoadError::Io { .. } => {
                panic!("expected a Parse error, got an Io error")
            }
        }
    }

    #[test]
    fn active_retracted_overlap_manifest_fails_closed() {
        // A card_id in BOTH `cards` and `retracted_cards` violates the
        // `CardsManifest` retraction-disjointness invariant. Without
        // `validate_structurally` Rust would happily place the id in
        // `retracted_ids` while the same card is still active — a
        // retraction decision Python refuses to make.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cards_manifest.json");
        let manifest = serde_json::json!({
            "schema_version": "cacg.v0",
            "cards": [
                {
                    "schema_version": "cacg.v0",
                    "path": "cards/x.md",
                    "id": "overlap-card",
                    "title": "Title X",
                    "reading_id": "reading_01",
                    "summary": "Summary X",
                    "card_hash": "0".repeat(64),
                    "citation_count": 0,
                    "source_ids": [],
                },
            ],
            "retracted_cards": ["overlap-card"],
            "dependency_retracted_cards": [],
        });
        std::fs::write(&path, serde_json::to_string(&manifest).unwrap()).unwrap();
        let err = RetractionSpec::from_cards_manifest(&path, false)
            .expect_err("an active/retracted overlap manifest must fail closed");
        match err {
            RetractionLoadError::Parse(msg) => assert!(
                msg.contains("`cards` and `retracted_cards`"),
                "expected a structural-validation message, got {msg:?}"
            ),
            RetractionLoadError::Io { .. } => {
                panic!("expected a Parse error, got an Io error")
            }
        }
    }
}
