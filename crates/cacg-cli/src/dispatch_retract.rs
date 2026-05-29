//! `kb retract-chunk` dispatch module.

use std::process::ExitCode;

use cacg_cli::RetractChunkArgs;

use crate::dispatch_show::py_repr;

/// `kb retract-chunk <chunk_id>` dispatcher. Mirrors Python
/// `_cmd_retract_chunk` (`legacy_python_oracle/src/cacg/cli.py:1218`). Adds
/// `chunk_id` to `chunks_manifest.retracted_chunk_ids`,
/// removes it from `chunks_manifest.chunks`, and — when
/// `--cards-dir` is supplied AND a sibling `cards_manifest.json`
/// exists — cascades the retraction into
/// `cards_manifest.dependency_retracted_cards`. The
/// atomic-update discipline (`.tmp` → `.bak` → rename, refuse
/// to clobber pre-existing sidecars) lives in
/// `cacg_core::retract::retract_chunk`.
///
/// Exit-code surface byte-equal with Python `_cmd_retract_chunk`
/// (`legacy_python_oracle/src/cacg/cli.py:1218-1242`):
///   - `CACG-CLI-001` (exit 1): RetractError variants — chunks
///     manifest missing, unknown chunk_id, already-retracted
///     (with or without cascade), already-retracted +
///     up-to-date cascade no-op, cascade publish failure
///     (wrapped via `CACG-RET-003:`).
///   - `CACG-MAN-001` (exit 1): chunks_manifest OR cards_manifest
///     fails schema validation on load. Python emits both with
///     the same `chunks_manifest.json is invalid:` prefix
///     (even for cards_manifest errors — a Python quirk we
///     preserve for byte parity).
///   - `CACG-MAN-002` (exit 1): pre-existing chunks_manifest
///     `.tmp`/`.bak` sidecar collision (refusing to clobber).
///   - `CACG-IDX-004` (exit 1): bare-Exception arm — I/O
///     failure outside the publish path.
pub(crate) fn dispatch_retract_chunk(args: &RetractChunkArgs) -> ExitCode {
    use cacg_core::retract::{py_list_of_str_repr, retract_chunk, RetractError};

    let result = retract_chunk(&args.chunk_id, &args.out, args.cards_dir.as_deref());
    match result {
        Ok(report) => {
            println!("retracted chunk_id:            {}", report.chunk_id);
            println!("active chunks remaining:       {}", report.chunks_remaining);
            println!(
                "retracted_chunk_ids total:     {}",
                report.retracted_chunk_ids_total
            );
            ExitCode::SUCCESS
        }
        Err(err) => {
            match err {
                RetractError::ChunksManifestMissing(path) => {
                    eprintln!(
                        "CACG-CLI-001: chunks_manifest.json not found in {}; \
                         run `kb ingest` before retracting a chunk",
                        path.display()
                    );
                }
                RetractError::AlreadyRetracted(chunk_id) => {
                    eprintln!(
                        "CACG-CLI-001: chunk {} is already retracted; \
                         retraction is append-only and not idempotent",
                        py_repr(&chunk_id)
                    );
                }
                RetractError::AlreadyRetractedNoOp(chunk_id) => {
                    eprintln!(
                        "CACG-CLI-001: chunk {} is already retracted and the \
                         dependency cascade is already up-to-date",
                        py_repr(&chunk_id)
                    );
                }
                RetractError::UnknownChunk(chunk_id) => {
                    eprintln!(
                        "CACG-CLI-001: chunk {} is not present in \
                         chunks_manifest.chunks; refusing to retract an \
                         unknown chunk",
                        py_repr(&chunk_id)
                    );
                }
                RetractError::ManifestInvalid(inner) => {
                    eprintln!("CACG-MAN-001: chunks_manifest.json is invalid: {inner}");
                }
                RetractError::PreexistingSidecars(paths) => {
                    eprintln!(
                        "CACG-MAN-002: refusing to clobber existing \
                         chunks_manifest sidecar(s): {}; remove them and re-run",
                        py_list_of_str_repr(&paths)
                    );
                }
                RetractError::CascadePublish(msg) => {
                    // `msg` already starts with `CACG-RET-003:` per Python's
                    // wire surface (`retract.py:601-606`). The dispatcher
                    // prepends `CACG-CLI-001: ` to match Python's
                    // `except RetractError` arm (`cli.py:1227-1228`).
                    eprintln!("CACG-CLI-001: {msg}");
                }
                RetractError::Io(inner) => {
                    // Python `_cmd_retract_chunk`'s bare-Exception arm
                    // (`cli.py:1236-1238`).
                    eprintln!("CACG-IDX-004: retract-chunk publish failed: {inner}");
                }
            }
            ExitCode::FAILURE
        }
    }
}
