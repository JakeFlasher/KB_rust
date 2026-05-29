//! SHA-256 hash primitives byte-equal to Python `cacg.hash` + `cacg.checksum`.
//!
//! Four named primitives, each a Rust counterpart to the Python implementation
//! at `legacy_python_oracle/src/cacg/hash.py` / `legacy_python_oracle/src/cacg/checksum.py`:
//!
//! - [`source_sha256`] — SHA-256 over raw bytes of a source artifact (PDF, etc.).
//! - [`chunk_hash`] — SHA-256 over a canonical-JSON envelope binding
//!   `{text, start_page, end_page, page_spans}` so a tampered chunks manifest
//!   that changes page metadata while keeping the text identical surfaces as
//!   a hash mismatch.
//! - [`card_hash`] — SHA-256 over `(canonical_json(frontmatter without
//!   card_hash) + "\n--BODY--\n" + normalize_text(body))`. Excluding
//!   `card_hash` from its own payload prevents self-referential
//!   staleness when `kb index` rewrites the field. The body is the RAW
//!   card body; normalization happens internally via
//!   [`crate::normalize::normalize_text`] so the API matches Python
//!   `cacg.hash.card_hash(frontmatter, body)` byte-for-byte. Use
//!   [`card_hash_normalized_body`] when the caller has already
//!   normalized the body upstream (e.g., batched index computation).
//! - [`compute_event_checksum`] — SHA-256 over canonical-JSON(payload
//!   without `event_checksum`). The cornerstone of the
//!   `BL-20260518-checksum-chain-jsonl` tamper-evident chain used by
//!   `lint_journal.jsonl` and per-card `<slug>.history.jsonl` artifacts.
//!
//! All four primitives are determinism-pure (no clock / filesystem access)
//! except [`source_sha256`], which only reads the bytes the caller hands in.

use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::canonical_json::{canonical_json, CanonicalError};
use crate::normalize::normalize_text;

/// Field name excluded from the `card_hash` payload (so the hash can
/// be stored back into the frontmatter without becoming self-referential).
pub const CARD_HASH_FIELD: &str = "card_hash";

/// Field name excluded from the `event_checksum` payload (so the checksum
/// can be embedded in the event itself without breaking the chain).
pub const EVENT_CHECKSUM_FIELD: &str = "event_checksum";

/// Sentinel between canonical-JSON frontmatter bytes and normalized body
/// bytes inside the `card_hash` payload. Matches Python's literal
/// `b"\n--BODY--\n"` in `legacy_python_oracle/src/cacg/hash.py`.
const CARD_BODY_DELIM: &[u8] = b"\n--BODY--\n";

/// One element of a chunk's `page_spans` array. Mirrors the Python
/// `PageSpan` Pydantic model: an integer `page` plus a non-negative
/// `byte_offset_in_chunk`.
///
/// The Rust struct uses `i64` to match the Python representation when
/// canonicalized via `serde_json::Value::from(i)` (Python ints round-trip
/// through `json.dumps` as base-10 integers; `i64` covers the full range
/// any realistic page count plus offset would ever inhabit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageSpan {
    /// 1-based page number containing the start of this span.
    pub page: i64,
    /// Byte offset within the chunk where this page-span begins.
    pub byte_offset_in_chunk: i64,
}

/// SHA-256 of the raw bytes of a source artifact, lowercase 64-hex.
///
/// Mirrors Python `hashlib.sha256(Path(path).read_bytes()).hexdigest()`.
/// Caller is responsible for reading the file; this primitive operates
/// on the byte slice directly so it stays I/O-free and trivially testable.
#[must_use]
pub fn source_sha256(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

/// SHA-256 over the canonical-JSON envelope binding a chunk's identity.
///
/// The envelope is the ordered dictionary
/// `{"text": <text>, "start_page": <i>, "end_page": <i>,
/// "page_spans": [{"page": <i>, "byte_offset_in_chunk": <i>}, ...]}`.
/// `cacg_core::canonical_json` produces the byte-identical serialization
/// to Python's `json.dumps(envelope, sort_keys=True, separators=(",",":"),
/// ensure_ascii=False)`; the result is then UTF-8 encoded and SHA-256
/// hashed.
///
/// `text` MUST be pre-normalized (the hash binds *normalized* text, not
/// raw); this function does NOT normalize -- keeping the hash boundary
/// explicit and audit-friendly per Python `cacg.hash.chunk_hash`'s
/// docstring.
///
/// # Errors
///
/// Returns [`CanonicalError`] only if canonicalization fails, which is
/// impossible for the inputs this function constructs (all are
/// guaranteed-valid JSON-domain values). The Result wrapper is retained
/// for API symmetry with the rest of cacg-core.
pub fn chunk_hash(
    text: &str,
    start_page: i64,
    end_page: i64,
    page_spans: &[PageSpan],
) -> Result<String, CanonicalError> {
    let spans_json: Vec<Value> = page_spans
        .iter()
        .map(|s| {
            let mut m = serde_json::Map::with_capacity(2);
            m.insert(
                "byte_offset_in_chunk".to_string(),
                Value::from(s.byte_offset_in_chunk),
            );
            m.insert("page".to_string(), Value::from(s.page));
            Value::Object(m)
        })
        .collect();
    let mut envelope = serde_json::Map::with_capacity(4);
    envelope.insert("end_page".to_string(), Value::from(end_page));
    envelope.insert("page_spans".to_string(), Value::Array(spans_json));
    envelope.insert("start_page".to_string(), Value::from(start_page));
    envelope.insert("text".to_string(), Value::String(text.to_string()));
    let payload = canonical_json(&Value::Object(envelope))?;
    let mut h = Sha256::new();
    h.update(payload.as_bytes());
    Ok(format!("{:x}", h.finalize()))
}

/// SHA-256 over `canonical_json(frontmatter without card_hash)` joined to
/// `normalize_text(body)` via the literal `\n--BODY--\n` delimiter.
///
/// Matches Python `cacg.hash.card_hash(frontmatter, body)` byte-for-byte:
/// the body is normalized internally via [`crate::normalize::normalize_text`]
/// so callers pass the raw card body (the `# Title\n\n...` markdown text
/// between the frontmatter `---` fences).
///
/// # Errors
///
/// Returns [`CanonicalError`] only if canonicalization of the frontmatter
/// fails (e.g., a non-finite float reached the writer, which the
/// upstream schema validator should have rejected first).
pub fn card_hash(
    frontmatter: &serde_json::Map<String, Value>,
    body: &str,
) -> Result<String, CanonicalError> {
    let normalized = normalize_text(body);
    card_hash_normalized_body(frontmatter, &normalized)
}

/// Lower-level helper that hashes against an ALREADY-normalized body.
///
/// Use this only when the caller has demonstrated upstream that the body
/// is the output of [`crate::normalize::normalize_text`]; the parity
/// oracle generator calls this directly after invoking
/// `normalize_text` itself so the test can keep `raw_body` and the
/// pre-computed normalized form distinct in the committed JSON fixture.
///
/// # Errors
///
/// Returns [`CanonicalError`] only if canonicalization of the frontmatter
/// fails.
pub fn card_hash_normalized_body(
    frontmatter: &serde_json::Map<String, Value>,
    normalized_body: &str,
) -> Result<String, CanonicalError> {
    // Build a clone with `card_hash` removed; keep allocations minimal.
    let mut without_hash = serde_json::Map::with_capacity(frontmatter.len());
    for (k, v) in frontmatter.iter() {
        if k == CARD_HASH_FIELD {
            continue;
        }
        without_hash.insert(k.clone(), v.clone());
    }
    let frontmatter_canonical = canonical_json(&Value::Object(without_hash))?;
    let mut h = Sha256::new();
    h.update(frontmatter_canonical.as_bytes());
    h.update(CARD_BODY_DELIM);
    h.update(normalized_body.as_bytes());
    Ok(format!("{:x}", h.finalize()))
}

/// SHA-256 over `canonical_json(payload without event_checksum)`.
///
/// The cornerstone of the per-event checksum chain used by
/// `lint_journal.jsonl` and per-card `<slug>.history.jsonl`. Excluding
/// `event_checksum` from its own payload lets the field be embedded
/// inline in the event without breaking the chain.
///
/// # Errors
///
/// Returns [`CanonicalError`] only if canonicalization fails.
pub fn compute_event_checksum(
    payload: &serde_json::Map<String, Value>,
) -> Result<String, CanonicalError> {
    let mut cleaned = serde_json::Map::with_capacity(payload.len());
    for (k, v) in payload.iter() {
        if k == EVENT_CHECKSUM_FIELD {
            continue;
        }
        cleaned.insert(k.clone(), v.clone());
    }
    let canonical = canonical_json(&Value::Object(cleaned))?;
    let mut h = Sha256::new();
    h.update(canonical.as_bytes());
    Ok(format!("{:x}", h.finalize()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn source_sha256_empty_input() {
        // SHA-256 of the empty byte string is the canonical
        // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855.
        assert_eq!(
            source_sha256(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn source_sha256_lowercase_64_hex() {
        let out = source_sha256(b"hello\n");
        assert_eq!(out.len(), 64);
        assert!(out
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn chunk_hash_envelope_shape() {
        // Construct a canonical envelope with known content and confirm
        // the hash is deterministic + lowercase 64-hex. Exact byte-equality
        // against Python is exercised by the integration test against
        // hash_oracles.json.
        let spans = [
            PageSpan {
                page: 1,
                byte_offset_in_chunk: 0,
            },
            PageSpan {
                page: 2,
                byte_offset_in_chunk: 706,
            },
        ];
        let r = chunk_hash("hello world", 1, 2, &spans).expect("chunk_hash");
        assert_eq!(r.len(), 64);
        // Determinism: same inputs -> same hash.
        let r2 = chunk_hash("hello world", 1, 2, &spans).expect("chunk_hash");
        assert_eq!(r, r2);
    }

    #[test]
    fn card_hash_excludes_card_hash_field() {
        let mut fm = serde_json::Map::new();
        fm.insert("id".to_string(), Value::String("test".to_string()));
        fm.insert(
            "card_hash".to_string(),
            Value::String("ignored-value".to_string()),
        );
        let h1 = card_hash(&fm, "body").expect("card_hash");

        let mut fm2 = serde_json::Map::new();
        fm2.insert("id".to_string(), Value::String("test".to_string()));
        // No card_hash field at all.
        let h2 = card_hash(&fm2, "body").expect("card_hash");

        assert_eq!(h1, h2, "card_hash must ignore the existing card_hash field");
    }

    #[test]
    fn compute_event_checksum_excludes_event_checksum_field() {
        let mut p = serde_json::Map::new();
        p.insert("seq".to_string(), Value::from(0i64));
        p.insert("kind".to_string(), Value::String("test".to_string()));
        p.insert(
            "event_checksum".to_string(),
            Value::String("ignored".to_string()),
        );
        let h1 = compute_event_checksum(&p).expect("compute_event_checksum");

        let mut p2 = serde_json::Map::new();
        p2.insert("seq".to_string(), Value::from(0i64));
        p2.insert("kind".to_string(), Value::String("test".to_string()));
        let h2 = compute_event_checksum(&p2).expect("compute_event_checksum");

        assert_eq!(
            h1, h2,
            "compute_event_checksum must ignore the existing event_checksum field"
        );
    }

    #[test]
    fn compute_event_checksum_deterministic_under_key_reorder() {
        let mut p1 = serde_json::Map::new();
        p1.insert("a".to_string(), Value::from(1i64));
        p1.insert("b".to_string(), Value::from(2i64));
        let mut p2 = serde_json::Map::new();
        // Different insertion order.
        p2.insert("b".to_string(), Value::from(2i64));
        p2.insert("a".to_string(), Value::from(1i64));
        assert_eq!(
            compute_event_checksum(&p1).unwrap(),
            compute_event_checksum(&p2).unwrap(),
            "canonical_json sorts keys -> checksum is order-independent"
        );
    }
}
