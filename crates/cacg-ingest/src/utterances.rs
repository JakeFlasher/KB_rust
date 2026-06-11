//! Conversational-corpus ingest backend: a versioned JSONL utterance
//! stream replaces the synthetic-PDF detour for sources that are
//! already structured text (crawled posts/comments of a human expert).
//!
//! # Exchange contract (`cacg.utterances.v1`)
//!
//! The input file is JSON Lines:
//!
//! - line 1: a header object
//!   `{"schema_version": "cacg.utterances.v1", "source_kind": <str>}`
//!   (plus an optional free-form `"metadata"` object);
//! - every subsequent line: one utterance record with required fields
//!   `ordinal` (1-based, contiguous, ascending), `utterance_id`
//!   (non-empty, unique), `speaker` (non-empty), `is_author` (bool),
//!   `text` (non-empty after trim), and optional `authored_at`
//!   (string) and `refs` (string-to-string map, e.g.
//!   `{"post_id": "...", "comment_id": "..."}`).
//!
//! Validation is fail-closed: an unknown schema version, an unknown
//! field, a broken ordinal sequence, a duplicate `utterance_id`,
//! empty text, or a control/replacement character in `text` rejects
//! the whole stream — there is no partial ingest.
//!
//! # Mapping into the existing trust chain
//!
//! Each utterance becomes one logical "page" (`page N` =
//! `ordinal N`), feeding the SAME chunker + manifest builder +
//! atomic publisher the PDF path uses. With the conversational chunk
//! config (`max_pages_per_chunk: 1`, `overlap_tokens: 0`) a chunk is
//! exactly one utterance, so a verbatim quote can never straddle an
//! utterance boundary. No pdfium, no fonts, no page geometry is in
//! the chain; `source_sha256` is computed over the raw JSONL bytes.
//!
//! # Locator sidecar
//!
//! The anchors the synthetic-PDF path smuggled in-band as `@@…@@`
//! text markers live OUT of band here: [`build_locator_map`] emits a
//! `locator_map.json` mapping every `chunk_id` to the utterance
//! identities it covers (`utterance_id`, `speaker`, `is_author`,
//! `authored_at`, `refs`), sealed with a SHA-256 over its canonical
//! JSON so a tampered sidecar is mechanically detectable
//! ([`verify_locator_seal`]).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use cacg_core::canonical_json::canonical_json;
use cacg_core::hash::source_sha256;
use cacg_core::schema::ChunkRecord;

/// The only utterance-stream schema version this backend accepts.
pub const UTTERANCES_SCHEMA_VERSION: &str = "cacg.utterances.v1";

/// Schema version stamped into the emitted locator sidecar.
pub const LOCATOR_MAP_SCHEMA_VERSION: &str = "cacg.locator_map.v1";

/// Parser-identity name recorded in `SourceRecord` for this backend
/// (the PDF path records `pdfium-render`; this path is honest about
/// not touching pdfium at all).
pub const UTTERANCES_PARSER_NAME: &str = "cacg-utterances";

/// Errors rejecting an utterance stream. Every variant is fail-closed:
/// nothing is ingested from a stream that raises any of these.
#[derive(Debug, Error)]
pub enum UtterancesError {
    /// The file was empty or had no header line.
    #[error("utterance stream is empty (no header line)")]
    Empty,
    /// Line 1 failed to parse as the header object.
    #[error("utterance stream header (line 1) is malformed: {detail}")]
    BadHeader {
        /// Parse/validation detail.
        detail: String,
    },
    /// The header carried a schema version this backend does not know.
    #[error("unknown utterance schema_version {got:?} (expected {UTTERANCES_SCHEMA_VERSION:?})")]
    UnknownVersion {
        /// The version string found in the header.
        got: String,
    },
    /// A record line failed strict deserialization.
    #[error("utterance record at line {line} is malformed: {detail}")]
    BadRecord {
        /// 1-based line number in the stream file.
        line: usize,
        /// Parse/validation detail.
        detail: String,
    },
    /// `ordinal` did not start at 1 or increment by exactly 1.
    #[error("utterance ordinal break at line {line}: expected {expected}, got {got}")]
    OrdinalBreak {
        /// 1-based line number in the stream file.
        line: usize,
        /// The ordinal required by the contiguous sequence.
        expected: u64,
        /// The ordinal actually present.
        got: u64,
    },
    /// Two records carried the same `utterance_id`.
    #[error("duplicate utterance_id {id:?} at line {line}")]
    DuplicateId {
        /// 1-based line number of the second occurrence.
        line: usize,
        /// The duplicated id.
        id: String,
    },
    /// A required string field was empty (or text was whitespace-only).
    #[error("utterance at line {line} has an empty {field}")]
    EmptyField {
        /// 1-based line number in the stream file.
        line: usize,
        /// Which field was empty.
        field: &'static str,
    },
    /// `text` contained a control or replacement character that the
    /// chunk control-char policy forbids.
    #[error("utterance at line {line} contains a forbidden control/replacement char {ch:?}")]
    ForbiddenChar {
        /// 1-based line number in the stream file.
        line: usize,
        /// The offending character.
        ch: char,
    },
    /// More than `u32::MAX` utterances (page numbers are u32).
    #[error("utterance count overflows u32 page numbering")]
    TooManyUtterances,
}

/// The line-1 header of an utterance stream.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UtterancesHeader {
    /// Must equal [`UTTERANCES_SCHEMA_VERSION`].
    pub schema_version: String,
    /// Free descriptor of what the stream holds (e.g. `"conversation"`).
    pub source_kind: String,
    /// Optional producer metadata, recorded verbatim (not validated).
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// One utterance record (one line of the stream after the header).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Utterance {
    /// 1-based position in the stream; must be contiguous ascending.
    pub ordinal: u64,
    /// Stable platform identifier of this utterance (unique in-stream).
    pub utterance_id: String,
    /// Who wrote it (display handle or stable user id).
    pub speaker: String,
    /// Whether the speaker is the corpus's distilled author. Only
    /// author utterances are ever citable; the rest are context.
    pub is_author: bool,
    /// ISO-8601 timestamp of the utterance, when the producer has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<String>,
    /// Producer-defined stable references (e.g. post_id / comment_id).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub refs: BTreeMap<String, String>,
    /// The utterance text (non-empty; control-char-clean).
    pub text: String,
}

/// A parsed, validated utterance stream.
#[derive(Debug, Clone)]
pub struct UtteranceStream {
    /// The validated header.
    pub header: UtterancesHeader,
    /// The validated records, in ordinal order.
    pub utterances: Vec<Utterance>,
}

/// Mirror of the deck-side chunk control-char gate: C0 controls
/// (except `\n` and `\t`), DEL, C1 controls, and the U+FFFE/U+FFFD
/// noncharacter/replacement marks that signal an extraction defect.
fn forbidden_char(c: char) -> bool {
    let o = c as u32;
    (o < 0x20 && c != '\n' && c != '\t')
        || o == 0x7F
        || (0x80..=0x9F).contains(&o)
        || c == '\u{FFFE}'
        || c == '\u{FFFD}'
}

/// Parse + validate an utterance stream from raw file bytes.
///
/// # Errors
///
/// Any [`UtterancesError`]; the stream is rejected as a whole.
pub fn parse_utterances(bytes: &[u8]) -> Result<UtteranceStream, UtterancesError> {
    let text = std::str::from_utf8(bytes).map_err(|e| UtterancesError::BadHeader {
        detail: format!("stream is not UTF-8: {e}"),
    })?;
    let mut lines = text.lines().enumerate();

    let (_, header_line) = lines
        .by_ref()
        .find(|(_, l)| !l.trim().is_empty())
        .ok_or(UtterancesError::Empty)?;
    let header: UtterancesHeader =
        serde_json::from_str(header_line).map_err(|e| UtterancesError::BadHeader {
            detail: e.to_string(),
        })?;
    if header.schema_version != UTTERANCES_SCHEMA_VERSION {
        return Err(UtterancesError::UnknownVersion {
            got: header.schema_version,
        });
    }
    if header.source_kind.trim().is_empty() {
        return Err(UtterancesError::BadHeader {
            detail: "source_kind must be non-empty".into(),
        });
    }

    let mut utterances: Vec<Utterance> = Vec::new();
    let mut seen_ids: BTreeSet<String> = BTreeSet::new();
    for (idx, raw) in lines {
        let line_no = idx + 1;
        if raw.trim().is_empty() {
            continue;
        }
        let u: Utterance = serde_json::from_str(raw).map_err(|e| UtterancesError::BadRecord {
            line: line_no,
            detail: e.to_string(),
        })?;
        let expected = utterances.len() as u64 + 1;
        if u.ordinal != expected {
            return Err(UtterancesError::OrdinalBreak {
                line: line_no,
                expected,
                got: u.ordinal,
            });
        }
        if u.utterance_id.trim().is_empty() {
            return Err(UtterancesError::EmptyField {
                line: line_no,
                field: "utterance_id",
            });
        }
        if !seen_ids.insert(u.utterance_id.clone()) {
            return Err(UtterancesError::DuplicateId {
                line: line_no,
                id: u.utterance_id,
            });
        }
        if u.speaker.trim().is_empty() {
            return Err(UtterancesError::EmptyField {
                line: line_no,
                field: "speaker",
            });
        }
        if u.text.trim().is_empty() {
            return Err(UtterancesError::EmptyField {
                line: line_no,
                field: "text",
            });
        }
        if let Some(ch) = u.text.chars().find(|&c| forbidden_char(c)) {
            return Err(UtterancesError::ForbiddenChar { line: line_no, ch });
        }
        utterances.push(u);
    }
    if utterances.is_empty() {
        return Err(UtterancesError::Empty);
    }
    if u32::try_from(utterances.len()).is_err() {
        return Err(UtterancesError::TooManyUtterances);
    }
    Ok(UtteranceStream { header, utterances })
}

/// The per-utterance identity entry stored in the locator sidecar.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocatorEntry {
    /// The utterance's 1-based ordinal (== its logical page number).
    pub ordinal: u64,
    /// Stable platform identifier of the utterance.
    pub utterance_id: String,
    /// Who wrote it.
    pub speaker: String,
    /// Whether the speaker is the corpus's distilled author.
    pub is_author: bool,
    /// ISO-8601 timestamp, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authored_at: Option<String>,
    /// Producer-defined stable references.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub refs: BTreeMap<String, String>,
}

/// Errors building or verifying the locator sidecar.
#[derive(Debug, Error)]
pub enum LocatorError {
    /// A chunk covered a page with no corresponding utterance ordinal.
    #[error("chunk {chunk_id} covers page {page} but the stream has only {count} utterances")]
    PageOutOfRange {
        /// The chunk whose page range overflowed the stream.
        chunk_id: String,
        /// The offending 1-based page.
        page: u32,
        /// Total utterances in the stream.
        count: usize,
    },
    /// Canonical-JSON serialization failed.
    #[error("locator canonical-JSON failed: {0}")]
    Canonical(String),
}

/// Build the sealed locator-map JSON bytes for a chunked utterance
/// stream: `chunk_id -> [LocatorEntry…]` for every utterance (page)
/// the chunk covers, plus a `seal` = SHA-256 over the canonical JSON
/// of the payload without the seal field.
///
/// # Errors
///
/// [`LocatorError`] when a chunk references a page beyond the stream
/// or serialization fails.
pub fn build_locator_map(
    source_id: &str,
    stream_bytes: &[u8],
    utterances: &[Utterance],
    chunks: &[ChunkRecord],
) -> Result<Vec<u8>, LocatorError> {
    let mut locators: BTreeMap<String, Vec<LocatorEntry>> = BTreeMap::new();
    for c in chunks {
        let mut entries = Vec::new();
        for page in c.start_page..=c.end_page {
            let idx = page as usize - 1;
            let u = utterances
                .get(idx)
                .ok_or_else(|| LocatorError::PageOutOfRange {
                    chunk_id: c.chunk_id.clone(),
                    page,
                    count: utterances.len(),
                })?;
            entries.push(LocatorEntry {
                ordinal: u.ordinal,
                utterance_id: u.utterance_id.clone(),
                speaker: u.speaker.clone(),
                is_author: u.is_author,
                authored_at: u.authored_at.clone(),
                refs: u.refs.clone(),
            });
        }
        locators.insert(c.chunk_id.clone(), entries);
    }

    let payload = serde_json::json!({
        "schema_version": LOCATOR_MAP_SCHEMA_VERSION,
        "source_id": source_id,
        "utterances_sha256": source_sha256(stream_bytes),
        "locators": serde_json::to_value(&locators)
            .map_err(|e| LocatorError::Canonical(e.to_string()))?,
    });
    let unsealed =
        canonical_json(&payload).map_err(|e| LocatorError::Canonical(format!("{e:?}")))?;
    let seal = source_sha256(unsealed.as_bytes());

    let mut sealed = payload;
    if let serde_json::Value::Object(ref mut m) = sealed {
        m.insert("seal".to_owned(), serde_json::Value::String(seal));
    }
    let out = canonical_json(&sealed).map_err(|e| LocatorError::Canonical(format!("{e:?}")))?;
    Ok(out.into_bytes())
}

/// Verify a locator-map's seal: recompute the SHA-256 over the
/// canonical JSON of the document minus its `seal` field and compare.
///
/// Returns `Ok(true)` when the seal matches, `Ok(false)` when it does
/// not (or the `seal` field is missing/not a string).
///
/// # Errors
///
/// [`LocatorError::Canonical`] when the bytes are not valid JSON or
/// canonicalization fails.
pub fn verify_locator_seal(bytes: &[u8]) -> Result<bool, LocatorError> {
    let mut value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|e| LocatorError::Canonical(e.to_string()))?;
    let Some(obj) = value.as_object_mut() else {
        return Ok(false);
    };
    let Some(serde_json::Value::String(stored)) = obj.remove("seal") else {
        return Ok(false);
    };
    let unsealed = canonical_json(&value).map_err(|e| LocatorError::Canonical(format!("{e:?}")))?;
    Ok(source_sha256(unsealed.as_bytes()) == stored)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn header_line() -> String {
        format!(
            r#"{{"schema_version":"{UTTERANCES_SCHEMA_VERSION}","source_kind":"conversation"}}"#
        )
    }

    fn record(ordinal: u64, id: &str, text: &str) -> String {
        format!(
            r#"{{"ordinal":{ordinal},"utterance_id":"{id}","speaker":"狗不叫","is_author":true,"authored_at":"2022-06-12T06:20:00+08:00","refs":{{"post_id":"222375639"}},"text":"{text}"}}"#
        )
    }

    fn stream(lines: &[String]) -> Vec<u8> {
        lines.join("\n").into_bytes()
    }

    #[test]
    fn accepts_a_minimal_valid_stream() {
        let s = stream(&[
            header_line(),
            record(1, "p1", "上個月sell call，行使價被調整。"),
            record(2, "c1", "回覆：不弄丟底倉。"),
        ]);
        let parsed = parse_utterances(&s).unwrap();
        assert_eq!(parsed.utterances.len(), 2);
        assert_eq!(parsed.utterances[0].utterance_id, "p1");
        assert_eq!(parsed.utterances[1].ordinal, 2);
        assert_eq!(
            parsed.utterances[0].refs.get("post_id").map(String::as_str),
            Some("222375639")
        );
    }

    #[test]
    fn rejects_empty_and_headerless_streams() {
        assert!(matches!(
            parse_utterances(b"").unwrap_err(),
            UtterancesError::Empty
        ));
        assert!(matches!(
            parse_utterances(b"\n\n").unwrap_err(),
            UtterancesError::Empty
        ));
        // header-only (no records) is also Empty.
        let s = stream(&[header_line()]);
        assert!(matches!(
            parse_utterances(&s).unwrap_err(),
            UtterancesError::Empty
        ));
    }

    #[test]
    fn rejects_unknown_schema_version() {
        let s = stream(&[
            r#"{"schema_version":"cacg.utterances.v999","source_kind":"conversation"}"#.to_owned(),
            record(1, "p1", "text"),
        ]);
        assert!(matches!(
            parse_utterances(&s).unwrap_err(),
            UtterancesError::UnknownVersion { .. }
        ));
    }

    #[test]
    fn rejects_unknown_fields_strictly() {
        let s = stream(&[
            header_line(),
            r#"{"ordinal":1,"utterance_id":"x","speaker":"s","is_author":false,"text":"t","extra_field":1}"#
                .to_owned(),
        ]);
        assert!(matches!(
            parse_utterances(&s).unwrap_err(),
            UtterancesError::BadRecord { .. }
        ));
    }

    #[test]
    fn rejects_ordinal_breaks_and_duplicate_ids() {
        let s = stream(&[header_line(), record(2, "p1", "t")]);
        assert!(matches!(
            parse_utterances(&s).unwrap_err(),
            UtterancesError::OrdinalBreak {
                expected: 1,
                got: 2,
                ..
            }
        ));
        let s = stream(&[header_line(), record(1, "p1", "t"), record(2, "p1", "t2")]);
        assert!(matches!(
            parse_utterances(&s).unwrap_err(),
            UtterancesError::DuplicateId { .. }
        ));
    }

    #[test]
    fn rejects_empty_text_and_forbidden_chars() {
        let s = stream(&[header_line(), record(1, "p1", "  ")]);
        assert!(matches!(
            parse_utterances(&s).unwrap_err(),
            UtterancesError::EmptyField { field: "text", .. }
        ));
        let s = stream(&[header_line(), record(1, "p1", "bad\\u0002char")]);
        assert!(matches!(
            parse_utterances(&s).unwrap_err(),
            UtterancesError::ForbiddenChar { ch: '\u{2}', .. }
        ));
        let s = stream(&[header_line(), record(1, "p1", "bad\\ufffdchar")]);
        assert!(matches!(
            parse_utterances(&s).unwrap_err(),
            UtterancesError::ForbiddenChar { ch: '\u{FFFD}', .. }
        ));
    }

    fn chunk(chunk_id: &str, start: u32, end: u32) -> ChunkRecord {
        use cacg_core::schema::{PageSpan, SchemaVersion};
        ChunkRecord {
            schema_version: SchemaVersion::V0,
            source_id: "sid".into(),
            chunk_id: chunk_id.into(),
            chunk_hash: "a".repeat(64),
            ordinal: 0,
            start_page: start,
            end_page: end,
            page_spans: vec![PageSpan {
                page: start,
                byte_offset_in_chunk: 0,
            }],
            token_count: 1,
            text: "t".into(),
            text_preview: "t".into(),
        }
    }

    #[test]
    fn locator_map_covers_multi_page_chunks_and_seals() {
        let s = stream(&[
            header_line(),
            record(1, "p1", "первый"),
            record(2, "c1", "второй"),
        ]);
        let parsed = parse_utterances(&s).unwrap();
        let chunks = vec![chunk("sid:p001:0000", 1, 2)];
        let bytes = build_locator_map("sid", &s, &parsed.utterances, &chunks).unwrap();
        assert!(verify_locator_seal(&bytes).unwrap(), "seal must verify");

        let doc: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let entries = doc["locators"]["sid:p001:0000"].as_array().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0]["utterance_id"], "p1");
        assert_eq!(entries[1]["utterance_id"], "c1");
        assert_eq!(doc["schema_version"], LOCATOR_MAP_SCHEMA_VERSION);

        // A tampered byte must break the seal.
        let tampered = String::from_utf8(bytes).unwrap().replace("p1", "px");
        assert!(!verify_locator_seal(tampered.as_bytes()).unwrap());
    }

    #[test]
    fn locator_map_rejects_out_of_range_pages() {
        let s = stream(&[header_line(), record(1, "p1", "x")]);
        let parsed = parse_utterances(&s).unwrap();
        let chunks = vec![chunk("sid:p001:0000", 1, 5)];
        let err = build_locator_map("sid", &s, &parsed.utterances, &chunks).unwrap_err();
        assert!(matches!(err, LocatorError::PageOutOfRange { page: 2, .. }));
    }
}
