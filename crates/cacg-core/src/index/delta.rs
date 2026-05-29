//! Card-hash recomputation and in-place frontmatter rewriting.
//!
//! Contains [`recompute_card_hash`], [`rewrite_card_hash_in_place`],
//! [`insert_card_hash_line`], and [`replace_card_hash_line`].

use std::path::Path;

use serde_json::Value;

use crate::atomic_publish::{atomic_publish, DefaultFs, PublishMember};

use super::manifest::IndexError;
use super::{NON_FILE_DIAG, SIDECAR_DIAG};

/// Recompute the canonical card_hash by stripping the stored
/// `card_hash` field from the frontmatter map representation, then
/// hashing the resulting object + body via
/// `cacg_core::hash::card_hash`. Mirrors Python's
/// `fm.pop("card_hash", None); compute_card_hash(fm, body)`.
pub(super) fn recompute_card_hash(
    fm: &crate::schema::CardFrontmatter,
    body: &str,
) -> Result<String, IndexError> {
    use crate::hash::card_hash;
    let v = serde_json::to_value(fm)
        .map_err(|e| IndexError::Canonical(format!("frontmatter -> Value: {e}")))?;
    #[allow(clippy::wildcard_enum_match_arm)]
    let mut map = match v {
        Value::Object(m) => m,
        _ => unreachable!("CardFrontmatter serializes to an object"),
    };
    map.remove("card_hash");
    card_hash(&map, body).map_err(|e| IndexError::Canonical(format!("card_hash: {e}")))
}

/// In-place rewrite of the card's stored `card_hash` frontmatter line.
/// Locates the FIRST `card_hash:` line within the YAML frontmatter
/// region (between the opening `---` and the closing `---` markers)
/// and replaces its quoted scalar value with `new_hash`. When the
/// frontmatter has no `card_hash:` line at all (e.g., a freshly
/// authored card from `kb new` whose template omits the field),
/// falls back to inserting a new `card_hash: "<hash>"` line
/// immediately before the closing `---` -- mirroring Python's
/// `_render_with_hash`, which re-renders the entire card via the
/// canonical writer and therefore always emits the `card_hash`
/// field. Without this insertion the per-card history loop would
/// append an event for a card whose stored `card_hash` never moves
/// off `None`, and every subsequent `kb index` run would re-enter
/// the stale branch and emit a duplicate history event for the
/// same unchanged hash. Preserves line-leading indentation and the
/// rest of the file verbatim. Uses the existing atomic-publisher
/// framework so the write is durable (the publisher snapshots the
/// prior canonical to `.bak`, stages `.tmp`, replaces atomically,
/// then unlinks `.bak`).
pub(super) fn rewrite_card_hash_in_place(
    card_path: &Path,
    new_hash: &str,
    original_text: &str,
) -> Result<(), IndexError> {
    // Concurrent-edit guard: re-read the on-disk bytes and compare to
    // `original_text` BEFORE publishing. Publishing `original_text +
    // new_hash` unconditionally would silently clobber any user edit
    // made between the parse loop and this rewrite (atomic_publish
    // renames the edited canonical to `.bak`, replaces it, then drops
    // `.bak` on success). Refusing to publish when on-disk differs
    // from parse-time preserves the user's edit; the per-card
    // transaction's rewrite-failure branch then rolls back any prior
    // committed work + manifests.
    let current_text = std::fs::read_to_string(card_path).map_err(|source| IndexError::Io {
        path: card_path.to_path_buf(),
        source,
    })?;
    if current_text != original_text {
        return Err(IndexError::Canonical(format!(
            "card text at {} changed between parse and rewrite; refusing to clobber concurrent edit",
            card_path.display()
        )));
    }
    // Use the parse-time `original_text` (the bytes that fed
    // `recompute_card_hash` to produce `new_hash`) so the committed
    // `card_hash` matches the card content the manifest + history
    // events describe. The above guard ensures no concurrent edit
    // is silently overwritten.
    let new_text = match replace_card_hash_line(original_text, new_hash) {
        Some(t) => t,
        None => insert_card_hash_line(original_text, new_hash)?,
    };
    let tmp_suffix = "md.tmp";
    let bak_suffix = "md.bak";
    let tmp_path = card_path.with_extension(tmp_suffix);
    let bak_path = card_path.with_extension(bak_suffix);
    let members = vec![PublishMember {
        tmp_path,
        bak_path,
        canonical_path: card_path.to_path_buf(),
        bytes: new_text.into_bytes(),
    }];
    atomic_publish(&members, &DefaultFs, SIDECAR_DIAG, NON_FILE_DIAG)?;
    Ok(())
}

/// Insert a new `card_hash: "<hash>"` line immediately before the
/// closing `---` of the YAML frontmatter region. Used by
/// `rewrite_card_hash_in_place` when the existing frontmatter has no
/// `card_hash:` line to replace. Mirrors the canonical writer's
/// placement of `card_hash` last in `_CARD_FRONTMATTER_KEY_ORDER` so
/// the resulting file shape matches what `kb new` + `kb index` would
/// produce had the card been re-rendered from scratch. Returns an
/// `IndexError::Canonical` if no closing `---` is found, which would
/// indicate an upstream parse defect (the caller already accepted the
/// card via `parse_card`).
pub(super) fn insert_card_hash_line(text: &str, new_hash: &str) -> Result<String, IndexError> {
    let mut out = String::with_capacity(text.len() + new_hash.len() + 16);
    let mut delimiters_seen = 0u8;
    let mut inserted = false;
    for raw in text.split_inclusive('\n') {
        let line_no_nl = raw.strip_suffix('\n').unwrap_or(raw);
        let trimmed = line_no_nl.trim();
        if trimmed == "---" {
            delimiters_seen += 1;
            if delimiters_seen == 2 && !inserted {
                out.push_str("card_hash: \"");
                out.push_str(new_hash);
                out.push_str("\"\n");
                inserted = true;
            }
            out.push_str(raw);
            continue;
        }
        out.push_str(raw);
    }
    if !inserted {
        return Err(IndexError::Canonical(
            "card frontmatter has no closing `---` delimiter; cannot insert card_hash".to_string(),
        ));
    }
    Ok(out)
}

/// Replace the first `card_hash: "..."` line within the YAML
/// frontmatter region. Returns the rewritten text, or None when no
/// such line is found. Preserves leading whitespace + line endings.
pub(super) fn replace_card_hash_line(text: &str, new_hash: &str) -> Option<String> {
    let mut out = String::with_capacity(text.len() + new_hash.len());
    let mut in_frontmatter = false;
    let mut delimiters_seen = 0u8;
    let mut replaced = false;
    for raw in text.split_inclusive('\n') {
        // Strip the trailing newline (if any) for trim-based checks
        // while preserving it on emit.
        let line_no_nl = raw.strip_suffix('\n').unwrap_or(raw);
        let trimmed = line_no_nl.trim();
        if trimmed == "---" {
            delimiters_seen += 1;
            in_frontmatter = delimiters_seen == 1;
            out.push_str(raw);
            continue;
        }
        if in_frontmatter && !replaced && line_no_nl.trim_start().starts_with("card_hash:") {
            let prefix_len = line_no_nl.len() - line_no_nl.trim_start().len();
            out.push_str(&line_no_nl[..prefix_len]);
            out.push_str("card_hash: \"");
            out.push_str(new_hash);
            out.push('"');
            if raw.ends_with('\n') {
                out.push('\n');
            }
            replaced = true;
            continue;
        }
        out.push_str(raw);
    }
    if replaced {
        Some(out)
    } else {
        None
    }
}
