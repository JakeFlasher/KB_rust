//! `kb search` dispatch module.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use cacg_core::canonical_json::canonical_json;
use cacg_core::index::CardsManifest;
use cacg_core::schema::SourceMatrix;
use cacg_core::source_matrix::load_source_matrix;
use cacg_search::{SearchHit, SummariesIndex, SummariesSqliteIndex};

use cacg_cli::SearchArgs;

/// `kb search <query>` dispatcher. Mirrors Python
/// `legacy_python_oracle/src/cacg/cli.py::_cmd_search`: lexical retrieval over
/// `summaries.json`, gated by the mandatory `--source-matrix`. The
/// `summaries.sqlite` FTS5 sidecar is probed first; the in-memory
/// `SummariesIndex` BM25 backend is built lazily only on fallback
/// (an absent / stale / unreadable / failed sidecar).
pub(crate) fn dispatch_search(args: &SearchArgs) -> ExitCode {
    // `is_file()` (not `exists()`) so a directory or dangling symlink
    // also routes through the not-a-regular-file path
    // (BL-20260518-shape-check-fs-inputs).
    if !args.summaries.is_file() {
        eprintln!(
            "CACG-CLI-001: summaries.json not found or not a regular file: {}",
            args.summaries.display()
        );
        return ExitCode::FAILURE;
    }
    if !args.source_matrix.is_file() {
        eprintln!(
            "CACG-AUTH-000: source-matrix not found or not a regular file: {}",
            args.source_matrix.display()
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
    let retracted = match search_retraction_set(&args.summaries) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("CACG-MAN-001: cannot load cards_manifest.json for retraction filter: {err}");
            return ExitCode::FAILURE;
        }
    };

    // Python clamps `--top-k` via `max(top_k, 0)`; both backends also
    // treat a non-positive `top_k` as "no results".
    let top_k = args.top_k.max(0);

    // Probe the `summaries.sqlite` FTS5 sidecar first (Python
    // `_cmd_search`). On an absent / stale / unreadable / failed sidecar,
    // fall back to the in-memory BM25 backend — built lazily so a fresh
    // sealed sidecar never pays the in-memory build cost.
    let hits = match open_search_sidecar(&args.summaries) {
        Some(sidecar) => match sidecar.search(&args.query, top_k, Some(&matrix), &retracted) {
            Ok(hits) => hits,
            Err(stale) => {
                eprintln!("{stale}");
                match search_in_memory(&args.summaries, &args.query, top_k, &matrix, &retracted) {
                    Some(hits) => hits,
                    None => return ExitCode::FAILURE,
                }
            }
        },
        None => match search_in_memory(&args.summaries, &args.query, top_k, &matrix, &retracted) {
            Some(hits) => hits,
            None => return ExitCode::FAILURE,
        },
    };

    if args.json {
        let value = serde_json::to_value(&hits).expect("SearchHit list serializes to JSON");
        let text = canonical_json(&value).expect("search-hit JSON is canonical-serializable");
        println!("{text}");
    } else if hits.is_empty() {
        println!("(no matches)");
    } else {
        for h in &hits {
            println!(
                "{score:8.4}  {card_id}  [{reading_id}]  {title}  -> {path}",
                score = h.score,
                card_id = h.card_id,
                reading_id = h.reading_id,
                title = h.title,
                path = h.path,
            );
        }
    }
    ExitCode::SUCCESS
}

/// The retraction set for `kb search`: the union of `retracted_cards`
/// and `dependency_retracted_cards` from the sibling
/// `<summaries-dir>/cards_manifest.json`. An absent sibling manifest is
/// tolerated (the set is empty); a present-but-malformed manifest is a
/// fail-closed error the caller maps to `CACG-MAN-001`.
fn search_retraction_set(summaries_path: &Path) -> Result<Vec<String>, String> {
    let cards_manifest_path = summaries_path.parent().map_or_else(
        || PathBuf::from("cards_manifest.json"),
        |p| p.join("cards_manifest.json"),
    );
    if !cards_manifest_path.is_file() {
        return Ok(Vec::new());
    }
    let raw = std::fs::read_to_string(&cards_manifest_path).map_err(|e| e.to_string())?;
    let manifest: CardsManifest = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    // Reject an invariant-invalid sibling manifest (the caller maps the
    // error to `CACG-MAN-001`) — matching Python `_cmd_search`, which
    // loads the manifest via `CardsManifest.model_validate_json` and so
    // fails closed on the same invariants.
    manifest.validate_structurally().map_err(|d| d.message)?;
    let mut retracted = manifest.retracted_cards;
    retracted.extend(manifest.dependency_retracted_cards);
    Ok(retracted)
}

/// Probe the `summaries.sqlite` FTS5 sidecar sitting beside
/// `summaries.json`. Returns the opened sealed index, `None` when no
/// sidecar is present (a silent fallback — no diagnostic), or `None`
/// after emitting a `CACG-FTS-001` line when the sidecar is stale or
/// unreadable. Mirrors Python `_cmd_search`'s sidecar probe.
fn open_search_sidecar(summaries_path: &Path) -> Option<SummariesSqliteIndex> {
    let sidecar_path = summaries_path.parent().map_or_else(
        || PathBuf::from(cacg_search::SIDECAR_FILENAME),
        |p| p.join(cacg_search::SIDECAR_FILENAME),
    );
    if !sidecar_path.is_file() {
        return None;
    }
    let summaries_bytes = std::fs::read(summaries_path).ok()?;
    let expected_hash = cacg_search::compute_summaries_hash(&summaries_bytes);
    match SummariesSqliteIndex::open(&sidecar_path, &expected_hash) {
        Ok(index) => Some(index),
        Err(stale) => {
            eprintln!("{stale}");
            None
        }
    }
}

/// Build the in-memory BM25 backend and run the query. Returns `None`
/// after emitting `CACG-MAN-001` when `summaries.json` cannot be loaded.
fn search_in_memory(
    summaries_path: &Path,
    query: &str,
    top_k: i64,
    matrix: &SourceMatrix,
    retracted: &[String],
) -> Option<Vec<SearchHit>> {
    match SummariesIndex::from_path(summaries_path) {
        Ok(index) => Some(index.search(query, top_k, Some(matrix), retracted)),
        Err(err) => {
            eprintln!("CACG-MAN-001: cannot load summaries.json: {err}");
            None
        }
    }
}
