//! `kb show` dispatch module.

use std::path::Path;
use std::process::ExitCode;

use cacg_core::card_loader::load_card;
use cacg_core::hash::card_hash;
use cacg_core::index::{CardManifestEntry, CardsManifest};
use cacg_core::schema::{CardFrontmatter, CitationEdge, SourceMatrix};
use cacg_core::source_matrix::{is_citation_authorized, load_source_matrix};

use cacg_cli::ShowArgs;

use crate::dispatch_lint::emit_lint_diagnostics;

// Unicode non-printable codepoint ranges (`static NONPRINTABLE_RANGES`),
// generated from CPython `str.isprintable()` data.
include!("nonprintable_table.rs");

/// `kb show <card_id>` dispatcher. Mirrors the historical `_cmd_show`
/// behavior: resolve `card_id` against
/// `cards_manifest.json`, load the card (honoring the optional
/// `--path` override), cross-check a `--path` override's on-disk
/// `id`/`card_hash` against the manifest, run the authorization gate
/// on the loaded card's `fm.reading_id` BEFORE the retraction gate,
/// and print the pinned human-readable card view.
///
/// A `--path` value containing a `..` component or an absolute path is
/// rejected with `CACG-SHOW-003` before any filesystem read. Python
/// `_cmd_show` performs the identical check, so `kb show --path` is
/// byte-equal across both implementations.
pub(crate) fn dispatch_show(args: &ShowArgs) -> ExitCode {
    // CACG-SHOW-003: reject an absolute / `..`-traversal `--path`
    // before any filesystem read, writing nothing to stdout.
    if let Some(path) = args.path.as_deref() {
        if path.is_absolute()
            || path
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            eprintln!(
                "CACG-SHOW-003: --path {} rejected: absolute paths and `..` \
                 traversal components are not permitted",
                path.display()
            );
            return ExitCode::FAILURE;
        }
    }

    if !args.cards_manifest.is_file() {
        eprintln!(
            "CACG-MAN-001: cards_manifest.json not found or not a regular file: {}",
            args.cards_manifest.display()
        );
        return ExitCode::FAILURE;
    }
    let matrix = match load_source_matrix(&args.source_matrix) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("CACG-AUTH-000: cannot load source-matrix: {err}");
            return ExitCode::FAILURE;
        }
    };
    let manifest: CardsManifest = match std::fs::read_to_string(&args.cards_manifest)
        .map_err(|e| e.to_string())
        .and_then(|raw| serde_json::from_str(&raw).map_err(|e| e.to_string()))
    {
        Ok(m) => m,
        Err(err) => {
            eprintln!("CACG-MAN-001: cards_manifest.json is invalid: {err}");
            return ExitCode::FAILURE;
        }
    };
    // Enforce the `CardsManifest` structural invariants (duplicate
    // card ids, retraction-list shape, retraction disjointness) at the
    // trust boundary BEFORE any card resolution / authorization /
    // retraction decision — matching Python `CardsManifest`'s
    // load-time validators, so an invariant-invalid manifest cannot
    // leak retraction state.
    if let Err(diag) = manifest.validate_structurally() {
        eprintln!(
            "CACG-MAN-001: cards_manifest.json is invalid: {}",
            diag.message
        );
        return ExitCode::FAILURE;
    }

    let Some(entry) = manifest.cards.iter().find(|c| c.id == args.card_id) else {
        eprintln!(
            "CACG-CLI-001: card_id {} is not present in cards_manifest.cards",
            py_repr(&args.card_id)
        );
        return ExitCode::FAILURE;
    };

    // Load the card BEFORE the auth gate so the gate cross-checks the
    // loaded card's own citations, not just the manifest's cached
    // `source_ids` (a tampered card cannot bypass via manifest staleness).
    let card_path = args
        .path
        .clone()
        .unwrap_or_else(|| std::path::PathBuf::from(&entry.path));
    let doc = match load_card(&card_path) {
        Ok(d) => d,
        Err(err) => {
            emit_lint_diagnostics(&err.diagnostics);
            return ExitCode::FAILURE;
        }
    };

    if args.path.is_some()
        && !show_path_cross_check(&card_path, &doc.frontmatter, &doc.body_raw, entry)
    {
        return ExitCode::FAILURE;
    }

    let fm = &doc.frontmatter;
    if !show_authorized(&matrix, fm, &args.card_id) {
        return ExitCode::FAILURE;
    }

    // Retraction gate runs AFTER the auth gate so retraction state is
    // never leaked to an unauthorized caller via `CACG-SHOW-001`.
    let retraction_status = show_retraction_status(&manifest, &args.card_id);
    if let Some(status) = retraction_status {
        if !args.allow_retracted {
            eprintln!(
                "CACG-SHOW-001: card {} is {status} and `--allow-retracted` was not supplied",
                py_repr(&args.card_id)
            );
            return ExitCode::FAILURE;
        }
    }

    render_card_view(fm, entry, retraction_status);
    ExitCode::SUCCESS
}

/// `--path`-override cross-check: the on-disk card's `id` and its
/// recomputed `card_hash` must both agree with the manifest entry.
/// Prints `CACG-SHOW-002` and returns `false` on any disagreement.
fn show_path_cross_check(
    card_path: &Path,
    fm: &CardFrontmatter,
    body_raw: &str,
    entry: &CardManifestEntry,
) -> bool {
    let fm_value = serde_json::to_value(fm).expect("CardFrontmatter serializes to JSON");
    let serde_json::Value::Object(mut fm_map) = fm_value else {
        unreachable!("CardFrontmatter serializes to a JSON object")
    };
    // `card_hash` strips this itself; the explicit removal mirrors the
    // `cacg_core::index` recompute path for clarity.
    fm_map.remove("card_hash");
    let on_disk_hash =
        card_hash(&fm_map, body_raw).expect("card_hash canonicalizes a validated frontmatter");
    if fm.id == entry.id && on_disk_hash == entry.card_hash {
        return true;
    }
    eprintln!(
        "CACG-SHOW-002: --path {} disagrees with cards_manifest: \
         on-disk id={} card_hash={on_disk_hash}; manifest id={} card_hash={}",
        card_path.display(),
        py_repr(&fm.id),
        py_repr(&entry.id),
        entry.card_hash,
    );
    false
}

/// Authorization gate: the loaded card's `reading_id` must be a
/// `source_matrix` key and every citation's `source_id` must be on that
/// reading's allow-list. Prints `CACG-AUTH-001` / `CACG-AUTH-002` and
/// returns `false` on the first failure.
fn show_authorized(matrix: &SourceMatrix, fm: &CardFrontmatter, card_id: &str) -> bool {
    if !matrix.allowed.contains_key(&fm.reading_id) {
        eprintln!(
            "CACG-AUTH-001: card {} reading_id {} is not present in source_matrix.allowed",
            py_repr(card_id),
            py_repr(&fm.reading_id),
        );
        return false;
    }
    for cit in &fm.citations {
        let (ok, code) = is_citation_authorized(matrix, &fm.reading_id, &cit.source_id);
        if !ok {
            eprintln!(
                "{}: card {} cites source_id {} which is not authorized for reading_id {}",
                code.unwrap_or(cacg_core::diagnostic::codes::AUTH_002),
                py_repr(card_id),
                py_repr(&cit.source_id),
                py_repr(&fm.reading_id),
            );
            return false;
        }
    }
    true
}

/// The retraction status of `card_id` per the manifest's
/// `retracted_cards` / `dependency_retracted_cards` lists, or `None`
/// for an active card.
fn show_retraction_status(manifest: &CardsManifest, card_id: &str) -> Option<&'static str> {
    if manifest
        .retracted_cards
        .iter()
        .any(|c| c.as_str() == card_id)
    {
        Some("RETRACTED")
    } else if manifest
        .dependency_retracted_cards
        .iter()
        .any(|c| c.as_str() == card_id)
    {
        Some("DEPENDENCY-RETRACTED")
    } else {
        None
    }
}

/// Print the pinned human-readable `kb show` card view: the optional
/// `STATUS:` line, the `# {title}` / `**{summary}**` lines, the four
/// scalar lines, and one `- citation …` line per citation. Mirrors
/// Python `_cmd_show`'s output block byte-for-byte.
fn render_card_view(
    fm: &CardFrontmatter,
    entry: &CardManifestEntry,
    retraction_status: Option<&str>,
) {
    if let Some(status) = retraction_status {
        println!("STATUS: {status}");
    }
    println!("# {}", fm.title);
    println!("**{}**", fm.summary);
    println!("id: {}", fm.id);
    println!("reading_id: {}", fm.reading_id);
    println!("schema_version: {}", fm.schema_version.as_str());
    println!("card_hash: {}", entry.card_hash);
    for cit in &fm.citations {
        let pages: String = cit
            .page_range
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join("-");
        println!(
            "- citation source_id={} chunk_id={} pages={pages} chunk_hash={} \
             edge_type={} quote={}",
            cit.source_id,
            cit.chunk_id,
            cit.chunk_hash,
            citation_edge_str(cit.edge_type),
            py_repr(&cit.quote),
        );
    }
}

/// The `snake_case` wire form of a citation `edge_type`, matching the
/// Python `CitationEdge` string literal `_cmd_show` prints.
fn citation_edge_str(edge: CitationEdge) -> &'static str {
    match edge {
        CitationEdge::Supports => "supports",
        CitationEdge::Defines => "defines",
        CitationEdge::Extends => "extends",
        CitationEdge::ContrastsWith => "contrasts_with",
        CitationEdge::DependsOn => "depends_on",
        CitationEdge::AppliesTo => "applies_to",
    }
}

/// `true` iff `c` is printable per Python `str.isprintable()` — i.e.
/// Python `repr()` renders it raw, not escaped. Non-printable scalars
/// are Unicode categories `C*` / `Z*` (minus `U+0020`); the set is
/// `NONPRINTABLE_RANGES`, generated from `CPython`. Meaningful only for
/// `c >= 0x80` (lower scalars are handled by `py_repr` directly).
fn is_printable(c: char) -> bool {
    let cp = c as u32;
    NONPRINTABLE_RANGES
        .binary_search_by(|&(lo, hi)| {
            if cp < lo {
                std::cmp::Ordering::Greater
            } else if cp > hi {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_err()
}

/// Render a string the way Python's `repr()` renders a `str`, so the
/// `kb show` lines that interpolate `{value!r}` (Python `_cmd_show`'s
/// `card_id` / `reading_id` / `source_id` / citation `quote`) are
/// byte-equal. `CPython` chooses `'` unless the string contains `'`
/// and not `"`; it escapes `\`, the chosen quote, and `\n` / `\r` /
/// `\t`, and `\xHH`-escapes C0 control characters + DEL. Every other
/// scalar Python `str.isprintable()` reports non-printable (C1
/// controls, the `Cf` format category such as soft hyphen, the
/// `Zl` / `Zp` separators, …) is escaped at Python's exact width:
/// `\xXX` (<= `U+00FF`), `\uXXXX` (<= `U+FFFF`), or `\UXXXXXXXX`.
pub(crate) fn py_repr(s: &str) -> String {
    let quote = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c as u32 == 0x7f => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\x{:02x}", c as u32);
            }
            // Printable ASCII (`0x20..=0x7e`; C0 + DEL handled above).
            c if (c as u32) < 0x7f => out.push(c),
            // `0x80`+: raw iff Python `str.isprintable()`, else escaped
            // at Python's exact width.
            c if is_printable(c) => out.push(c),
            c => {
                use std::fmt::Write as _;
                let cp = c as u32;
                if cp <= 0xff {
                    let _ = write!(out, "\\x{cp:02x}");
                } else if cp <= 0xffff {
                    let _ = write!(out, "\\u{cp:04x}");
                } else {
                    let _ = write!(out, "\\U{cp:08x}");
                }
            }
        }
    }
    out.push(quote);
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::py_repr;

    #[test]
    fn py_repr_escapes_nonprintable_characters_like_python() {
        // Every expected value is exactly `repr(<the input str>)` from
        // CPython — `kb show` must byte-match it.
        assert_eq!(py_repr("a\u{00ad}b"), r"'a\xadb'"); // soft hyphen (Cf)
        assert_eq!(py_repr("\u{200b}"), r"'\u200b'"); // zero-width space (Cf)
        assert_eq!(py_repr("\u{2028}"), r"'\u2028'"); // line separator (Zl)
        assert_eq!(py_repr("\u{0085}"), r"'\x85'"); // C1 control (NEL)
        assert_eq!(py_repr("\u{00a0}"), r"'\xa0'"); // no-break space (Zs)
        assert_eq!(py_repr("\u{e0001}"), r"'\U000e0001'"); // non-BMP Cf
    }

    #[test]
    fn py_repr_keeps_printable_characters_raw() {
        assert_eq!(py_repr("hello"), "'hello'");
        assert_eq!(py_repr("café"), "'café'"); // é (U+00E9) is printable
        assert_eq!(py_repr("\u{1f600}"), "'\u{1f600}'"); // emoji is printable
    }
}
