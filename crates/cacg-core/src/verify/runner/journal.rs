//! Journal event construction helpers for the verify runner.
//!
//! `retraction_diagnostic` evaluates the retraction contract and
//! returns an optional diagnostic. `append_verify_event` serializes
//! one `command="verify"` journal event, enforcing the
//! "exactly one event per card per invocation" cardinality pin.

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Instant;

use serde_json::Value;

use crate::card_loader::CardDoc;
use crate::determinism::DeterminismContext;
use crate::diagnostic::{codes, Diagnostic, Severity};
use crate::journal::{append_entry, JournalEntry, JournalError};
use crate::lint::layer1::freeze_aware_latency;
use crate::retraction::RetractionSpec;

pub(super) fn retraction_diagnostic(
    doc: Option<&CardDoc>,
    card_path: &Path,
    retraction: Option<&RetractionSpec>,
) -> Option<Diagnostic> {
    let r = retraction?;
    if !r.enabled() {
        return None;
    }
    let d = doc?;
    let cid = d.frontmatter.id.as_str();
    if !r.is_retracted(cid) {
        return None;
    }
    let severity = if r.allow_retracted {
        Severity::Warning
    } else {
        Severity::Error
    };
    let msg = format!(
        "card_id {cid:?} is retracted (in cards_manifest.retracted_cards); refusing to \
         verify as active"
    );
    Some(Diagnostic::new(codes::RETR_001, severity, msg).with_file(card_path.display().to_string()))
}

#[allow(clippy::too_many_arguments)]
pub(super) fn append_verify_event(
    journal_path: &Path,
    card_path: &Path,
    card_hash: Option<&str>,
    diagnostics: &[Diagnostic],
    layer1: bool,
    layer2: bool,
    fuzzy: bool,
    start: Instant,
) -> Result<(), JournalError> {
    let diagnostics_json: Vec<Value> = diagnostics
        .iter()
        .map(|d| serde_json::to_value(d).unwrap_or(Value::Null))
        .collect();
    let mut verification: BTreeMap<String, bool> = BTreeMap::new();
    verification.insert("layer1".to_string(), layer1);
    verification.insert("layer2".to_string(), layer2);
    verification.insert("fuzzy".to_string(), fuzzy);
    let entry = JournalEntry {
        command: "verify".to_string(),
        card_path: card_path.display().to_string(),
        card_hash_before: card_hash.map(String::from),
        card_hash_after: card_hash.map(String::from),
        diagnostics: diagnostics_json,
        verification,
        latency_ms: freeze_aware_latency(start),
    };
    let ctx = DeterminismContext::from_env();
    let event_id = ctx.new_uuid();
    let timestamp = ctx.now_iso();
    append_entry(journal_path, &entry, &event_id, &timestamp)?;
    Ok(())
}
