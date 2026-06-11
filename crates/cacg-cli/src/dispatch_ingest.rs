//! `kb ingest` dispatch module.

use std::process::ExitCode;

use cacg_cli::IngestArgs;

/// `kb ingest <pdf>` dispatcher. Reads the PDF, runs pdfium-render
/// text extraction via cacg-ingest, builds the
/// `sources_manifest.json` + `chunks_manifest.json` pair, and
/// pair-atomically publishes them into `--out`. Diagnostics + exit
/// codes mirror Python `_cmd_ingest` (with `parser_name` /
/// `parser_version` a declared divergence per the DEC-2 resolution).
///
/// Available only when the `ingest` Cargo feature is enabled (the
/// shipped binary builds it by default). With
/// `--no-default-features`, this falls through to
/// `unimplemented_subcommand("ingest")`.
#[cfg(feature = "ingest")]
pub(crate) fn dispatch_ingest(args: &IngestArgs) -> ExitCode {
    use cacg_cli::IngestFormat;
    use cacg_core::atomic_publish::PublishError;
    use cacg_core::determinism::DeterminismContext;
    use cacg_ingest::config::load_config;
    use cacg_ingest::extract_pages;
    use cacg_ingest::manifest::{
        build_manifests_with_config, build_manifests_with_parser, publish_manifests,
        publish_manifests_replacing, publish_manifests_with_locator, validate_append,
        ManifestError,
    };
    use cacg_ingest::utterances::{
        build_locator_map, parse_utterances, verify_locator_seal, UTTERANCES_PARSER_NAME,
    };
    use cacg_ingest::IngestError;

    use crate::dispatch_show::py_repr;

    if !args.pdf.is_file() {
        eprintln!(
            "CACG-CLI-001: pdf not found or not a regular file: {}",
            args.pdf.display()
        );
        return ExitCode::FAILURE;
    }

    if args.append && args.format != IngestFormat::Utterances {
        eprintln!("CACG-CLI-003: --append requires --format utterances");
        return ExitCode::from(2);
    }

    if args.out.exists() && !args.out.is_dir() {
        eprintln!(
            "CACG-CLI-001: out path is not a directory: {}",
            args.out.display()
        );
        return ExitCode::FAILURE;
    }

    let source_id = args
        .source_id
        .clone()
        .unwrap_or_else(|| slugify_source(&args.pdf));
    if !is_valid_source_id(&source_id) {
        // Python `_cmd_ingest` formats this with `!r` on both the
        // regex pattern AND the source_id; the pattern is plain ASCII
        // so the literal here is already single-quoted, but the
        // source_id needs py_repr to byte-match Python's repr().
        eprintln!(
            "CACG-CLI-003: source_id must match '^[a-z0-9][a-z0-9_]*$': {}",
            py_repr(&source_id)
        );
        return ExitCode::from(2);
    }

    // Optional YAML config: absent → ChunkConfig::default(); present
    // → load + validate, mapping every ConfigError variant to
    // CACG-CLI-004 with exit 2 (mirrors Python `_cmd_ingest`'s
    // `except (ConfigError, ValueError)` block at cli.py:659-661).
    let chunk_config = match load_config(args.config.as_deref()) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("CACG-CLI-004: {err}");
            return ExitCode::from(2);
        }
    };

    let pdf_bytes = match std::fs::read(&args.pdf) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("CACG-INGEST-001: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // source_path stored verbatim in SourceRecord matches Python's
    // `str(p)` for the input Path -- callers control relative vs
    // absolute; the parity oracle uses a relative path.
    let source_path = args.pdf.display().to_string();
    let det = DeterminismContext::from_env();

    // The utterances backend: parse + validate the JSONL stream
    // (fail-closed), map one utterance per logical page into the SAME
    // chunker/manifest path the PDF route uses (no pdfium anywhere),
    // and publish a sealed locator sidecar in the same atomic group.
    let (output, locator_bytes) = if args.format == IngestFormat::Utterances {
        let stream = match parse_utterances(&pdf_bytes) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("CACG-INGEST-001: utterances stream rejected: {}", e);
                return ExitCode::FAILURE;
            }
        };
        let pages: Vec<(u32, &str)> = stream
            .utterances
            .iter()
            .map(|u| {
                let page_num = u32::try_from(u.ordinal).expect("validated to fit in u32");
                (page_num, u.text.as_str())
            })
            .collect();
        let output = match build_manifests_with_parser(
            &source_id,
            &source_path,
            &pdf_bytes,
            &pages,
            &det,
            &chunk_config,
            UTTERANCES_PARSER_NAME,
            cacg_ingest::cacg_ingest_version(),
        ) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("CACG-INGEST-001: {}", e);
                return ExitCode::FAILURE;
            }
        };
        // Fail-closed append: the prior locator seal must verify, every
        // previously-published chunk must re-derive byte-identical from
        // the new stream, prior retractions carry over, and only then is
        // the replace-capable publish authorized.
        let output = if args.append {
            let chunks_path = args.out.join("chunks_manifest.json");
            let locator_path = args.out.join("locator_map.json");
            if !chunks_path.is_file() || !locator_path.is_file() {
                eprintln!(
                    "CACG-INGEST-003: --append requires prior manifests + locator in {}",
                    args.out.display()
                );
                return ExitCode::FAILURE;
            }
            let prior_locator = match std::fs::read(&locator_path) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("CACG-INGEST-001: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            match verify_locator_seal(&prior_locator) {
                Ok(true) => {}
                Ok(false) => {
                    eprintln!(
                        "CACG-HASH-003: prior locator_map.json seal does not verify;                          refusing to append over a tampered sidecar"
                    );
                    return ExitCode::FAILURE;
                }
                Err(e) => {
                    eprintln!("CACG-INGEST-001: prior locator unreadable: {}", e);
                    return ExitCode::FAILURE;
                }
            }
            let prior: cacg_core::schema::ChunksManifest = match std::fs::read(&chunks_path)
                .map_err(|e| e.to_string())
                .and_then(|b| serde_json::from_slice(&b).map_err(|e| e.to_string()))
            {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("CACG-MAN-001: chunks_manifest.json is invalid: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            match validate_append(&prior, &output, &source_id) {
                Ok((merged, appended)) => {
                    println!("appended chunks:  {}", appended);
                    merged
                }
                Err(e) => {
                    eprintln!("CACG-INGEST-003: append rejected: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        } else {
            output
        };
        let locator = match build_locator_map(
            &source_id,
            &pdf_bytes,
            &stream.utterances,
            &output.chunks.chunks,
        ) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("CACG-INGEST-003: locator sidecar build failed: {}", e);
                return ExitCode::FAILURE;
            }
        };
        (output, Some(locator))
    } else {
        let pages_owned = match extract_pages(&pdf_bytes) {
            Ok(pages) => pages,
            Err(IngestError::Corrupt { detail }) => {
                eprintln!("CACG-INGEST-001: {}", detail);
                return ExitCode::FAILURE;
            }
            Err(e) => {
                // Other IngestError variants (Empty, PublishFailed) are
                // not produced by extract_pages; fold defensively under
                // CACG-INGEST-001 if they ever surface here.
                eprintln!("CACG-INGEST-001: {}", e);
                return ExitCode::FAILURE;
            }
        };

        let pages: Vec<(u32, &str)> = pages_owned
            .iter()
            .enumerate()
            .map(|(idx, text)| {
                let page_num = u32::try_from(idx + 1).expect("page index fits in u32");
                (page_num, text.as_str())
            })
            .collect();

        let output = match build_manifests_with_config(
            &source_id,
            &source_path,
            &pdf_bytes,
            &pages,
            &det,
            &chunk_config,
        ) {
            Ok(o) => o,
            Err(ManifestError::NoPages) => {
                eprintln!(
                    "CACG-INGEST-002: no chunks produced (empty PDF text?) for {}",
                    args.pdf.display()
                );
                return ExitCode::FAILURE;
            }
            Err(e) => {
                eprintln!("CACG-INGEST-001: {}", e);
                return ExitCode::FAILURE;
            }
        };
        (output, None)
    };

    if output.chunks.chunks.is_empty() {
        eprintln!(
            "CACG-INGEST-002: no chunks produced (empty PDF text?) for {}",
            args.pdf.display()
        );
        return ExitCode::FAILURE;
    }

    let publish_result = match (locator_bytes.as_ref(), args.append) {
        (Some(bytes), false) => publish_manifests_with_locator(&args.out, &output, Some(bytes)),
        (Some(bytes), true) => publish_manifests_replacing(&args.out, &output, Some(bytes)),
        (None, _) => publish_manifests(&args.out, &output),
    };
    if let Err(err) = publish_result {
        match err {
            ManifestError::Publish(PublishError::PreexistingSidecars { paths, .. }) => {
                eprintln!(
                    "CACG-MAN-002: refusing to clobber existing manifest \
                     sidecar(s): {:?}",
                    paths
                );
            }
            ManifestError::Publish(PublishError::NonFileCanonical { paths, .. }) => {
                eprintln!(
                    "CACG-MAN-003: refusing to overwrite non-file canonical \
                     target(s): {:?}",
                    paths
                );
            }
            ManifestError::PriorManifestsPresent { .. } => {
                // Display already carries CACG-INGEST-003.
                eprintln!("{}", err);
            }
            ManifestError::EmptySourceId
            | ManifestError::EmptySourcePath
            | ManifestError::NoPages
            | ManifestError::PageCountOverflow
            | ManifestError::Chunker(_)
            | ManifestError::Canonical(_)
            | ManifestError::Publish(PublishError::Io { .. }) => {
                eprintln!("CACG-INGEST-003: manifest publish failed: {}", err);
            }
        }
        return ExitCode::FAILURE;
    }

    let sources_path = args.out.join("sources_manifest.json");
    let chunks_path = args.out.join("chunks_manifest.json");
    println!("sources_manifest: {}", sources_path.display());
    println!("chunks_manifest:  {}", chunks_path.display());
    println!("chunks_count:     {}", output.chunks.chunks.len());
    println!("source_id:        {}", source_id);
    println!(
        "source_sha256:    {}",
        output.sources.sources[0].source_sha256
    );

    ExitCode::SUCCESS
}

/// Stub fallback when the `ingest` Cargo feature is disabled. Keeps
/// the `kb` parser surface stable for the help-snapshot parity row
/// while making the absence of pdfium obvious to the operator.
#[cfg(not(feature = "ingest"))]
pub(crate) fn dispatch_ingest(_args: &IngestArgs) -> ExitCode {
    cacg_cli::unimplemented_subcommand("ingest")
}

#[cfg(feature = "ingest")]
/// Derive a `source_id` from a PDF path stem. Byte-equal port of
/// Python `_slugify_source`: `stem.lower()` ->
/// `re.sub(r"[^a-z0-9]+", "_", _)` -> `strip("_")` ->
/// `"source"` fallback on empty result.
fn slugify_source(path: &std::path::Path) -> String {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let mut out = String::with_capacity(stem.len());
    let mut last_was_underscore = false;
    for c in stem.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            out.push(c);
            last_was_underscore = false;
        } else if !last_was_underscore {
            out.push('_');
            last_was_underscore = true;
        }
    }
    let trimmed = out.trim_matches('_').to_owned();
    if trimmed.is_empty() {
        "source".to_owned()
    } else {
        trimmed
    }
}

#[cfg(feature = "ingest")]
/// Validate a `source_id` against the Python `_SOURCE_ID_RE` pattern
/// `^[a-z0-9][a-z0-9_]*$`.
fn is_valid_source_id(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}
