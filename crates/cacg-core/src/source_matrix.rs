//! Source-authorization matrix loader mirroring Python
//! `legacy_python_oracle/src/cacg/source_matrix.py`.
//!
//! Opt-in: a verify or lint run that omits `--source-matrix` runs
//! without authorization checks. A run that supplies the flag
//! enforces strictly: any citation whose `source_id` is not on the
//! matrix's allow-list for the citing card's `reading_id` is rejected
//! at both layer-1 and layer-2 trust boundaries (including
//! `kb verify --skip-lint`).
//!
//! The artifact is a canonical-JSON document; absent / malformed /
//! missing-required-fields surface as `CACG-AUTH-000` per card visit,
//! preserving the per-card journal cardinality contract.
//!
//! Note on the M2 CLI invariant: `--source-matrix` is mandatory on
//! `kb lint` / `kb verify` / `kb search` / `kb show` (clap-enforced
//! in `cacg-cli`). The `Option<&Path>` API here is for round-summary
//! library callers and tests; the CLI dispatchers always pass
//! `Some(path)`.

use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::diagnostic::codes;
use crate::schema::SourceMatrix;

/// Raised when `source_matrix.json` cannot be read or fails validation.
/// Callers map this to `CACG-AUTH-000` rather than letting JSON
/// errors leak.
#[derive(Debug, Error)]
pub enum SourceMatrixLoadError {
    /// I/O failure reading the matrix file.
    #[error("source_matrix path is not a regular file: {0}")]
    NonFile(PathBuf),
    /// I/O failure reading the matrix bytes.
    #[error("source_matrix.json is invalid: {path}: {source}")]
    Io {
        /// Matrix path.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// JSON parse / schema validation failure.
    #[error("source_matrix.json is invalid: {0}")]
    Parse(String),
    /// Structural validation failure (duplicate / empty source_ids,
    /// empty reading_id keys).
    #[error("source_matrix.json is invalid: {0}")]
    Validate(String),
}

/// Per-batch authorization state shared across card visits. Mirrors
/// Python `cacg.source_matrix.AuthSpec`. Three valid configurations:
///
/// * `matrix: None`, `load_error: None` — `--source-matrix` omitted;
///   authorization checks are off.
/// * `matrix: Some(...)`, `load_error: None` — matrix loaded
///   successfully; layer-1 / layer-2 enforce CACG-AUTH-001 / 002.
/// * `matrix: None`, `load_error: Some(...)` — matrix path supplied
///   but load failed; each card visit emits one `CACG-AUTH-000`
///   carrying `load_error` so the journal records exactly one event
///   per card.
#[derive(Debug, Clone, Default)]
pub struct AuthSpec {
    /// Loaded matrix; `None` when authorization is off OR when load
    /// failed.
    pub matrix: Option<SourceMatrix>,
    /// Load-error message; populated only when matrix load failed.
    pub load_error: Option<String>,
}

impl AuthSpec {
    /// True iff authorization machinery should run (a matrix is loaded
    /// OR a load_error is recorded for per-card emission).
    #[must_use]
    pub fn enabled(&self) -> bool {
        self.matrix.is_some() || self.load_error.is_some()
    }

    /// Load the matrix from the (optional) path. `None` returns a
    /// disabled spec. A supplied-but-failing path returns a spec
    /// carrying the load-error string so each card visit emits
    /// exactly one `CACG-AUTH-000`.
    ///
    /// Mirrors Python `AuthSpec.from_optional_path`.
    #[must_use]
    pub fn from_optional_path(path: Option<&Path>) -> Self {
        let Some(p) = path else {
            return Self::default();
        };
        match load_source_matrix(p) {
            Ok(matrix) => Self {
                matrix: Some(matrix),
                load_error: None,
            },
            Err(e) => Self {
                matrix: None,
                load_error: Some(e.to_string()),
            },
        }
    }
}

/// Load + structurally-validate a `source_matrix.json` artifact.
///
/// File-shape pre-check follows BitLesson
/// `BL-20260518-shape-check-fs-inputs`: the path must be a regular
/// file. Missing / directory / unreadable paths return
/// [`SourceMatrixLoadError::NonFile`] or `Io`.
///
/// # Errors
///
/// Returns `Err(SourceMatrixLoadError)` on file-shape, I/O, parse,
/// or structural-validation failure.
pub fn load_source_matrix(path: impl AsRef<Path>) -> Result<SourceMatrix, SourceMatrixLoadError> {
    let path = path.as_ref();
    if !path.is_file() {
        return Err(SourceMatrixLoadError::NonFile(path.to_path_buf()));
    }
    let raw = std::fs::read_to_string(path).map_err(|source| SourceMatrixLoadError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let matrix: SourceMatrix =
        serde_json::from_str(&raw).map_err(|e| SourceMatrixLoadError::Parse(e.to_string()))?;
    if let Err(d) = matrix.validate_structurally() {
        return Err(SourceMatrixLoadError::Validate(d.message));
    }
    Ok(matrix)
}

/// Authorization decision for a single citation. Returns
/// `(authorized, failure_code_or_none)`. The failure code is one of
/// [`codes::AUTH_001`] (reading_id unknown) or [`codes::AUTH_002`]
/// (source_id not in the allow-list for the otherwise-known reading_id).
///
/// Mirrors Python `is_citation_authorized` byte-equal in return shape.
#[must_use]
pub fn is_citation_authorized(
    matrix: &SourceMatrix,
    reading_id: &str,
    source_id: &str,
) -> (bool, Option<&'static str>) {
    let Some(allowed) = matrix.allowed.get(reading_id) else {
        return (false, Some(codes::AUTH_001));
    };
    if !allowed.iter().any(|s| s == source_id) {
        return (false, Some(codes::AUTH_002));
    }
    (true, None)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn from_optional_path_none_is_disabled() {
        let spec = AuthSpec::from_optional_path(None);
        assert!(spec.matrix.is_none());
        assert!(spec.load_error.is_none());
        assert!(!spec.enabled());
    }

    #[test]
    fn from_optional_path_missing_records_load_error() {
        let path = Path::new("/nonexistent/__rust_source_matrix_probe__.json");
        let spec = AuthSpec::from_optional_path(Some(path));
        assert!(spec.matrix.is_none());
        assert!(spec.load_error.is_some());
        assert!(spec.enabled());
    }

    #[test]
    fn from_optional_path_malformed_records_load_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "{not valid").unwrap();
        let spec = AuthSpec::from_optional_path(Some(&path));
        assert!(spec.matrix.is_none());
        assert!(spec.load_error.is_some());
        assert!(spec.enabled());
    }

    #[test]
    fn from_optional_path_valid_loads_matrix() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("matrix.json");
        let payload = serde_json::json!({
            "schema_version": "cacg.v0",
            "allowed": {"reading_01": ["sample"]}
        });
        std::fs::write(&path, serde_json::to_string(&payload).unwrap()).unwrap();
        let spec = AuthSpec::from_optional_path(Some(&path));
        assert!(spec.matrix.is_some());
        assert!(spec.load_error.is_none());
        assert!(spec.enabled());
    }

    #[test]
    fn is_citation_authorized_known_pair_ok() {
        let matrix: SourceMatrix = serde_json::from_value(serde_json::json!({
            "schema_version": "cacg.v0",
            "allowed": {"reading_01": ["sample"]}
        }))
        .unwrap();
        let (ok, code) = is_citation_authorized(&matrix, "reading_01", "sample");
        assert!(ok);
        assert!(code.is_none());
    }

    #[test]
    fn is_citation_authorized_unknown_reading_emits_auth_001() {
        let matrix: SourceMatrix = serde_json::from_value(serde_json::json!({
            "schema_version": "cacg.v0",
            "allowed": {"reading_01": ["sample"]}
        }))
        .unwrap();
        let (ok, code) = is_citation_authorized(&matrix, "reading_99", "sample");
        assert!(!ok);
        assert_eq!(code, Some(codes::AUTH_001));
    }

    #[test]
    fn is_citation_authorized_unauthorized_source_emits_auth_002() {
        let matrix: SourceMatrix = serde_json::from_value(serde_json::json!({
            "schema_version": "cacg.v0",
            "allowed": {"reading_01": ["sample"]}
        }))
        .unwrap();
        let (ok, code) = is_citation_authorized(&matrix, "reading_01", "other");
        assert!(!ok);
        assert_eq!(code, Some(codes::AUTH_002));
    }

    #[test]
    fn structurally_invalid_matrix_yields_load_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("matrix.json");
        let payload = serde_json::json!({
            "schema_version": "cacg.v0",
            "allowed": {"reading_01": ["sample", "sample"]}
        });
        std::fs::write(&path, serde_json::to_string(&payload).unwrap()).unwrap();
        let spec = AuthSpec::from_optional_path(Some(&path));
        assert!(spec.matrix.is_none());
        assert!(spec
            .load_error
            .as_deref()
            .map_or(false, |s| s.contains("duplicate")));
    }
}
