//! CACG trust kernel.
//!
//! This crate owns the byte-equal contract surface: canonical JSON, SHA-256
//! hashing primitives, text normalization, schema validation, journal append
//! discipline, multi-file atomic publish, in-memory BM25, bounded Levenshtein,
//! determinism context, and the diagnostic struct + code constants.
//!
//! No Pdfium FFI, no SQLite, no HTTP, no async runtime. Downstream crates
//! re-export this crate's types rather than re-implementing them.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    // Pedantic lints below are not part of the quality gate
    // (gate policy enforces: unwrap_used, wildcard_enum_match_arm,
    // cognitive_complexity, excessive_nesting).
    clippy::doc_markdown,
    clippy::doc_lazy_continuation,
    clippy::missing_panics_doc,
    clippy::similar_names,
    clippy::many_single_char_names,
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::float_cmp,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::manual_let_else,
    clippy::single_match_else,
    clippy::if_not_else,
    clippy::needless_lifetimes,
    clippy::needless_late_init,
    clippy::needless_borrows_for_generic_args,
    clippy::needless_raw_string_hashes,
    clippy::needless_option_as_deref,
    clippy::needless_splitn,
    clippy::redundant_closure_for_method_calls,
    clippy::manual_range_contains,
    clippy::naive_bytecount,
    clippy::useless_format,
    clippy::unreadable_literal,
    clippy::used_underscore_binding,
    clippy::explicit_iter_loop,
    clippy::items_after_statements,
    clippy::struct_excessive_bools,
    clippy::type_complexity,
    clippy::bool_to_int_with_if,
    clippy::must_use_candidate,
    clippy::map_unwrap_or,
    clippy::question_mark,
    clippy::vec_init_then_push,
    clippy::assigning_clones,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::unnecessary_map_or,
    missing_docs,
)]

// M1 trust-kernel modules. The remaining stubs (schema, normalize, hash,
// bm25, fuzzy, journal, history, atomic_publish, determinism, diagnostic,
// source_matrix, frontmatter, chunks_index) land in subsequent task-M1-N
// rounds.

pub mod atomic_publish;
pub mod bm25;
pub mod canonical_json;
pub mod card_loader;
pub mod card_template;
pub mod chunks_index;
pub mod demo;
pub mod determinism;
pub mod diagnostic;
pub mod frontmatter;
pub mod hash;
pub mod history;
pub mod index;
pub mod journal;
pub mod lint;
pub mod normalize;
pub mod parity;
pub mod retract;
pub mod retraction;
pub mod schema;
pub mod source_matrix;
pub mod verify;

/// Placeholder while the trust-kernel modules are scaffolded.
#[must_use]
pub fn cacg_core_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::cacg_core_version;

    #[test]
    fn version_is_nonempty() {
        assert!(!cacg_core_version().is_empty());
    }
}
