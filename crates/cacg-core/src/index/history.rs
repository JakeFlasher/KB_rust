//! Per-card history-event construction for [`super::build_index`].
//!
//! Contains [`append_per_card_history`], [`frontmatter_snapshot_for`],
//! [`sorted_changed_keys`], [`is_additive_list_field`], and
//! [`is_nonempty_array`].

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

use crate::history::{history_path_for, last_event_snapshot, HistoryEntry};
use crate::schema::CardFrontmatter;

use super::manifest::IndexError;
use super::IndexedCard;

/// Construct the `HistoryEntry` for a stale-hash card update and
/// append it via `cacg_core::history::append_history_event`. The
/// payload mirrors Python `HistoryEvent(prev_card_hash, new_card_hash,
/// cited_chunk_set_delta, frontmatter_field_changes,
/// cited_chunk_ids_snapshot, frontmatter_snapshot, is_retracted=false)`
/// field-for-field. Delta sourcing matches `legacy_python_oracle/src/cacg/index.py`:
/// `prior_chunk_ids` + `prior_frontmatter` come from the LAST
/// persisted history event's snapshot (or empty when no prior event
/// exists), and `prev_card_hash` comes from the stored frontmatter
/// hash being replaced -- the two sources MUST NOT be conflated.
pub(super) fn append_per_card_history(
    entry: &IndexedCard,
    ctx: &crate::determinism::DeterminismContext,
) -> Result<crate::history::AppendOutcome, IndexError> {
    let history_path = history_path_for(&entry.card_path);
    let (prior_chunk_ids, prior_frontmatter) = last_event_snapshot(&history_path)?;

    let new_chunk_ids: Vec<String> = {
        let mut v: Vec<String> = entry
            .frontmatter
            .citations
            .iter()
            .map(|c| c.chunk_id.clone())
            .collect();
        v.sort();
        v.dedup();
        v
    };

    let prior_chunk_set: BTreeSet<String> = prior_chunk_ids.iter().cloned().collect();
    let new_chunk_set: BTreeSet<String> = new_chunk_ids.iter().cloned().collect();
    let added: Vec<String> = new_chunk_set
        .difference(&prior_chunk_set)
        .cloned()
        .collect();
    let removed: Vec<String> = prior_chunk_set
        .difference(&new_chunk_set)
        .cloned()
        .collect();
    let mut cited_chunk_set_delta: BTreeMap<String, Vec<String>> = BTreeMap::new();
    cited_chunk_set_delta.insert("added".to_string(), added);
    cited_chunk_set_delta.insert("removed".to_string(), removed);

    let new_frontmatter_snapshot = frontmatter_snapshot_for(&entry.frontmatter)?;

    let frontmatter_field_changes =
        sorted_changed_keys(&prior_frontmatter, &new_frontmatter_snapshot);

    let history_entry = HistoryEntry {
        prev_card_hash: entry.prev_hash.clone(),
        new_card_hash: entry.new_hash.clone(),
        cited_chunk_set_delta,
        frontmatter_field_changes,
        cited_chunk_ids_snapshot: new_chunk_ids,
        frontmatter_snapshot: new_frontmatter_snapshot,
        is_retracted: false,
    };
    let timestamp = ctx.now_iso();
    let outcome = crate::history::append_history_event_with_outcome(
        &history_path,
        &history_entry,
        &timestamp,
    )?;
    Ok(outcome)
}

/// Build the `frontmatter_snapshot` value the history event will
/// store. Mirrors Python's `new_doc.frontmatter.model_dump(mode="json")`
/// then `fm.pop("card_hash", None)`: serialize the frontmatter as JSON,
/// drop the `card_hash` field (which always differs by construction
/// here -- the snapshot must reflect the NEW frontmatter sans hash so
/// the next index run's field-diff doesn't spuriously flag it).
///
/// Also drops `tags` and `card_edges` when they are empty lists, to
/// match Python's `model_dump(mode="json", exclude_defaults=False)`
/// behavior that emits them as `[]`. Both Rust serde and Python
/// already emit `[]` for empty Vecs/lists, so no explicit handling
/// is needed beyond the JSON serialization.
pub(super) fn frontmatter_snapshot_for(
    fm: &CardFrontmatter,
) -> Result<Map<String, Value>, IndexError> {
    let v = serde_json::to_value(fm)
        .map_err(|e| IndexError::Canonical(format!("frontmatter -> Value: {e}")))?;
    #[allow(clippy::wildcard_enum_match_arm)]
    let mut map = match v {
        Value::Object(m) => m,
        _ => unreachable!("CardFrontmatter serializes to an object"),
    };
    map.remove("card_hash");
    Ok(map)
}

/// Sorted-alphabetical union of keys whose values differ between
/// `prior` and `new`. Mirrors Python's `sorted(k for k in (prior.keys()
/// | new.keys()) if _frontmatter_field_changed(prior, new, k))`. The
/// `_frontmatter_field_changed` helper treats absent-vs-empty-list as
/// equivalent for `tags` and `card_edges` (the additive list fields)
/// so older history snapshots round-trip without spurious change
/// events.
pub(super) fn sorted_changed_keys(
    prior: &Map<String, Value>,
    new: &Map<String, Value>,
) -> Vec<String> {
    let mut keys: BTreeSet<String> = BTreeSet::new();
    for k in prior.keys() {
        keys.insert(k.clone());
    }
    for k in new.keys() {
        keys.insert(k.clone());
    }
    let mut changed: Vec<String> = Vec::new();
    for k in keys {
        let p = prior.get(&k);
        let n = new.get(&k);
        if is_additive_list_field(&k) {
            let p_norm = is_nonempty_array(p).then_some(p).flatten();
            let n_norm = is_nonempty_array(n).then_some(n).flatten();
            if p_norm != n_norm {
                changed.push(k);
            }
        } else if p != n {
            changed.push(k);
        }
    }
    changed
}

/// Additive list fields whose default `[]` is semantically
/// equivalent to absence. Mirrors Python
/// `cacg.frontmatter._ADDITIVE_LIST_FIELDS`.
pub(super) fn is_additive_list_field(name: &str) -> bool {
    matches!(name, "tags" | "card_edges")
}

pub(super) fn is_nonempty_array(v: Option<&Value>) -> bool {
    matches!(v, Some(Value::Array(a)) if !a.is_empty())
}
