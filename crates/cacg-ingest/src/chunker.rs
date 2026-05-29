//! Paragraph-respecting, token-budget chunker.
//!
//! Byte-equal port of `legacy_python_oracle/src/cacg/chunker.py`. Targets ~350 tokens per
//! chunk with a 50-token overlap carry, never crossing more than two
//! pages per chunk. Tokenization is whitespace splitting on the
//! pre-normalization page text; emitted chunk `text` is the
//! `cacg_core::normalize::normalize_text` of the joined paragraph
//! text. `chunk_hash` reuses `cacg_core::hash::chunk_hash`, so the
//! envelope bytes the hash binds are identical to Python's.
//!
//! Per-page `page_spans` offsets are **character offsets** (Python
//! `str` slicing semantics) measured against the *pre*-normalization
//! paragraph stream. This is a documented quirk of the Python oracle:
//! for ASCII single-space text the offsets are identical to
//! post-normalization positions, but for text with multi-space runs,
//! ligatures, or NFD-form characters the positions can shift. The
//! Rust port replicates the Python behavior bit-for-bit (PY-IS-ORACLE,
//! `_research/09` DEC-1); do not "fix" the offset measurement.

use std::collections::BTreeSet;

use cacg_core::hash::{chunk_hash, PageSpan as HashPageSpan};
use cacg_core::normalize::normalize_text;
use cacg_core::schema::{ChunkRecord, PageSpan, SchemaVersion};
use thiserror::Error;

/// Default target token count per chunk (Python `DEFAULT_TARGET_TOKENS`).
pub const DEFAULT_TARGET_TOKENS: u32 = 350;
/// Default carry-overlap token count (Python `DEFAULT_OVERLAP_TOKENS`).
pub const DEFAULT_OVERLAP_TOKENS: u32 = 50;
/// Default per-chunk page cap (Python `DEFAULT_MAX_PAGES_PER_CHUNK`).
pub const DEFAULT_MAX_PAGES_PER_CHUNK: u32 = 2;

/// Tunable knobs for the chunker. Defaults match Python.
#[derive(Debug, Clone, Copy)]
pub struct ChunkConfig {
    /// Target token count per chunk (soft upper bound — a chunk emits
    /// when the next paragraph would push it past this value).
    pub target_tokens: u32,
    /// Carry-overlap token count: after emit, the trailing paragraphs
    /// whose cumulative token count is at most this many are reused
    /// at the head of the next chunk.
    pub overlap_tokens: u32,
    /// Hard cap on the number of distinct pages a single chunk may span.
    pub max_pages_per_chunk: u32,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            target_tokens: DEFAULT_TARGET_TOKENS,
            overlap_tokens: DEFAULT_OVERLAP_TOKENS,
            max_pages_per_chunk: DEFAULT_MAX_PAGES_PER_CHUNK,
        }
    }
}

/// Errors returned by the chunker. Validation messages mirror Python.
#[derive(Debug, Error)]
pub enum ChunkerError {
    /// `target_tokens` must be positive.
    #[error("target_tokens must be positive")]
    TargetTokensNonPositive,
    /// `overlap_tokens` must be in `[0, target_tokens)`.
    #[error("overlap_tokens must be in [0, target_tokens)")]
    OverlapOutOfRange,
    /// `max_pages_per_chunk` must be positive.
    #[error("max_pages_per_chunk must be positive")]
    MaxPagesNonPositive,
    /// The `chunk_hash` canonical-JSON envelope failed to serialize.
    /// In practice impossible for the values constructed here; the
    /// variant exists only for API symmetry with `cacg_core::hash`.
    #[error("chunk_hash failed: {0}")]
    HashError(String),
}

/// One paragraph: page number + the ordered tokens drawn from a
/// single whitespace split of the paragraph's pre-normalization text.
#[derive(Debug, Clone)]
struct Paragraph {
    page: u32,
    tokens: Vec<String>,
}

impl Paragraph {
    fn token_count(&self) -> u32 {
        // Paragraph token counts are small (≪ u32::MAX) in practice;
        // saturate for safety on pathological inputs.
        u32::try_from(self.tokens.len()).unwrap_or(u32::MAX)
    }

    fn text(&self) -> String {
        self.tokens.join(" ")
    }
}

/// Port of Python `_split_paragraphs`.
///
/// Splits a page's text into paragraphs on a sentence-boundary
/// heuristic: end-of-sentence token (`.`/`?`/`!`) followed by an
/// uppercase-initial token, with the current buffer having at least
/// six tokens. Tokens are obtained by whitespace splitting (Python
/// `str.split()` with no argument == Rust `split_whitespace`).
fn split_paragraphs(page_text: &str) -> Vec<String> {
    if page_text.trim().is_empty() {
        return Vec::new();
    }
    let tokens: Vec<&str> = page_text.split_whitespace().collect();
    let mut parts: Vec<String> = Vec::new();
    let mut buf: Vec<&str> = Vec::new();
    for (i, tok) in tokens.iter().enumerate() {
        buf.push(tok);
        let ends_sentence = tok.ends_with('.') || tok.ends_with('?') || tok.ends_with('!');
        let next_is_capital = tokens
            .get(i + 1)
            .and_then(|s| s.chars().next())
            .is_some_and(char::is_uppercase);
        if ends_sentence && next_is_capital && buf.len() >= 6 {
            parts.push(buf.join(" "));
            buf.clear();
        }
    }
    if !buf.is_empty() {
        parts.push(buf.join(" "));
    }
    parts
}

/// Port of Python `_paragraphs_from_pages`.
fn paragraphs_from_pages(pages: &[(u32, &str)]) -> Vec<Paragraph> {
    let mut paras = Vec::new();
    for (page, page_text) in pages {
        for raw_para in split_paragraphs(page_text) {
            let tokens: Vec<String> = raw_para.split_whitespace().map(str::to_owned).collect();
            if tokens.is_empty() {
                continue;
            }
            paras.push(Paragraph {
                page: *page,
                tokens,
            });
        }
    }
    paras
}

fn pages_in(paras: &[Paragraph]) -> BTreeSet<u32> {
    paras.iter().map(|p| p.page).collect()
}

/// Port of Python `_build_chunk`.
fn build_chunk(
    source_id: &str,
    ordinal: u32,
    paras: &[Paragraph],
) -> Result<ChunkRecord, ChunkerError> {
    debug_assert!(!paras.is_empty(), "build_chunk called with empty paras");

    // The pre-normalization joined text; offsets are measured against
    // this stream (PY-IS-ORACLE quirk per the module docstring).
    let joined_pre_norm: String = paras
        .iter()
        .map(Paragraph::text)
        .collect::<Vec<_>>()
        .join(" ");
    let text = normalize_text(&joined_pre_norm);

    let start_page = paras.iter().map(|p| p.page).min().expect("paras non-empty");
    let end_page = paras.iter().map(|p| p.page).max().expect("paras non-empty");
    let chunk_id = format!("{source_id}:p{start_page:03}:{ordinal:04}");

    // Build page_spans: for each FIRST occurrence of a page in the
    // paragraph stream, record the CHARACTER offset (in the pre-
    // normalization rebuilt text) where its content begins.
    let mut page_spans: Vec<PageSpan> = Vec::new();
    let mut seen_pages: BTreeSet<u32> = BTreeSet::new();
    let mut running_offset: u32 = 0;
    for p in paras {
        if seen_pages.insert(p.page) {
            page_spans.push(PageSpan {
                page: p.page,
                byte_offset_in_chunk: running_offset,
            });
        }
        let para_text = p.text();
        // +1 for the join space between paragraphs; CHARACTER count.
        let para_chars = u32::try_from(para_text.chars().count()).unwrap_or(u32::MAX);
        running_offset = running_offset.saturating_add(para_chars).saturating_add(1);
    }

    // text_preview: first 120 characters of the normalized text.
    let text_preview: String = text.chars().take(120).collect();
    let token_count: u32 = paras.iter().map(Paragraph::token_count).sum();

    // chunk_hash takes its own PageSpan flavor (i64 fields, distinct
    // from the schema's u32-field PageSpan); convert at the boundary.
    // The two types are field-compatible by design — see `cacg_core::hash`.
    let hash_spans: Vec<HashPageSpan> = page_spans
        .iter()
        .map(|s| HashPageSpan {
            page: i64::from(s.page),
            byte_offset_in_chunk: i64::from(s.byte_offset_in_chunk),
        })
        .collect();
    let hash = chunk_hash(
        &text,
        i64::from(start_page),
        i64::from(end_page),
        &hash_spans,
    )
    .map_err(|e| ChunkerError::HashError(format!("{e:?}")))?;

    Ok(ChunkRecord {
        schema_version: SchemaVersion::V0,
        source_id: source_id.to_string(),
        chunk_id,
        chunk_hash: hash,
        ordinal,
        start_page,
        end_page,
        page_spans,
        token_count,
        text,
        text_preview,
    })
}

/// Produce a deterministic list of `ChunkRecord`s from pre-extracted
/// page text. Byte-equal port of Python `chunk_pages`.
///
/// `pages` is a slice of `(page_number_1_indexed, page_text)` pairs in
/// document order. `source_id` becomes the prefix of every emitted
/// `chunk_id`.
///
/// # Errors
///
/// - [`ChunkerError::TargetTokensNonPositive`] when `target_tokens == 0`.
/// - [`ChunkerError::OverlapOutOfRange`] when `overlap_tokens >= target_tokens`.
/// - [`ChunkerError::MaxPagesNonPositive`] when `max_pages_per_chunk == 0`.
/// - [`ChunkerError::HashError`] if `chunk_hash` canonicalization fails
///   (in practice unreachable for inputs this function constructs).
pub fn chunk_pages(
    source_id: &str,
    pages: &[(u32, &str)],
    config: &ChunkConfig,
) -> Result<Vec<ChunkRecord>, ChunkerError> {
    if config.target_tokens == 0 {
        return Err(ChunkerError::TargetTokensNonPositive);
    }
    if config.overlap_tokens >= config.target_tokens {
        return Err(ChunkerError::OverlapOutOfRange);
    }
    if config.max_pages_per_chunk == 0 {
        return Err(ChunkerError::MaxPagesNonPositive);
    }

    let paragraphs = paragraphs_from_pages(pages);
    let mut chunks: Vec<ChunkRecord> = Vec::new();
    let mut current_paras: Vec<Paragraph> = Vec::new();
    let mut current_tokens: u32 = 0;
    let max_pages = config.max_pages_per_chunk as usize;

    for para in paragraphs {
        let next_tokens = current_tokens.saturating_add(para.token_count());
        let mut next_pages = pages_in(&current_paras);
        next_pages.insert(para.page);
        let exceeds_tokens = next_tokens > config.target_tokens;
        let exceeds_pages = next_pages.len() > max_pages;

        if !current_paras.is_empty() && (exceeds_tokens || exceeds_pages) {
            // Emit a chunk from current_paras. Panic on ordinal
            // overflow rather than silently capping at u32::MAX --
            // the `{NNNN}` width in chunk_id is already wrong past
            // chunk #10_000, so a u32-saturated ordinal is unsafe.
            let ordinal = u32::try_from(chunks.len()).expect("ordinal overflow (u32)");
            chunks.push(build_chunk(source_id, ordinal, &current_paras)?);

            // Carry overlap: keep the suffix of current_paras whose
            // cumulative token count is at most overlap_tokens. We
            // walk backward from the tail; the first paragraph whose
            // inclusion would exceed the budget stops the carry, and
            // everything before that index is discarded.
            let mut carry_idx = current_paras.len();
            let mut carry_token_count: u32 = 0;
            for (i, p) in current_paras.iter().enumerate().rev() {
                let next = carry_token_count.saturating_add(p.token_count());
                if next > config.overlap_tokens {
                    break;
                }
                carry_token_count = next;
                carry_idx = i;
            }
            current_paras.drain(..carry_idx);
            current_tokens = carry_token_count;

            // _trim_overlap_for_next: drop carry paragraphs from the
            // front until carry + next_para fits the page budget.
            while !current_paras.is_empty() {
                let mut candidate_pages = pages_in(&current_paras);
                candidate_pages.insert(para.page);
                if candidate_pages.len() <= max_pages {
                    break;
                }
                let dropped = current_paras.remove(0);
                current_tokens = current_tokens.saturating_sub(dropped.token_count());
            }
        }

        let para_tokens = para.token_count();
        current_paras.push(para);
        current_tokens = current_tokens.saturating_add(para_tokens);
    }

    // Final emit.
    if !current_paras.is_empty() {
        let ordinal = u32::try_from(chunks.len()).expect("ordinal overflow (u32)");
        chunks.push(build_chunk(source_id, ordinal, &current_paras)?);
    }

    Ok(chunks)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_target_tokens() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "hello")];
        let config = ChunkConfig {
            target_tokens: 0,
            ..ChunkConfig::default()
        };
        assert!(matches!(
            chunk_pages("test", &pages, &config),
            Err(ChunkerError::TargetTokensNonPositive)
        ));
    }

    #[test]
    fn rejects_overlap_equal_to_target() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "hello world")];
        let config = ChunkConfig {
            target_tokens: 10,
            overlap_tokens: 10,
            ..ChunkConfig::default()
        };
        assert!(matches!(
            chunk_pages("test", &pages, &config),
            Err(ChunkerError::OverlapOutOfRange)
        ));
    }

    #[test]
    fn rejects_zero_max_pages() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "hello")];
        let config = ChunkConfig {
            max_pages_per_chunk: 0,
            ..ChunkConfig::default()
        };
        assert!(matches!(
            chunk_pages("test", &pages, &config),
            Err(ChunkerError::MaxPagesNonPositive)
        ));
    }

    #[test]
    fn empty_input_yields_no_chunks() {
        let pages: Vec<(u32, &str)> = Vec::new();
        let chunks = chunk_pages("test", &pages, &ChunkConfig::default()).unwrap();
        assert!(chunks.is_empty());
    }

    #[test]
    fn whitespace_only_page_yields_no_chunks() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "   \n\t  ")];
        let chunks = chunk_pages("test", &pages, &ChunkConfig::default()).unwrap();
        assert!(chunks.is_empty());
    }

    #[test]
    fn single_short_paragraph_yields_one_chunk_with_expected_metadata() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "Hello world. This is a short paragraph.")];
        let chunks = chunk_pages("test", &pages, &ChunkConfig::default()).unwrap();
        assert_eq!(chunks.len(), 1);
        let c = &chunks[0];
        assert_eq!(c.source_id, "test");
        assert_eq!(c.chunk_id, "test:p001:0000");
        assert_eq!(c.ordinal, 0);
        assert_eq!(c.start_page, 1);
        assert_eq!(c.end_page, 1);
        assert_eq!(c.page_spans.len(), 1);
        assert_eq!(c.page_spans[0].page, 1);
        assert_eq!(c.page_spans[0].byte_offset_in_chunk, 0);
        assert_eq!(c.text, "Hello world. This is a short paragraph.");
        // 7 tokens after whitespace split.
        assert_eq!(c.token_count, 7);
        // 64-hex SHA-256.
        assert_eq!(c.chunk_hash.len(), 64);
    }

    #[test]
    fn two_pages_one_chunk_records_both_pages_in_spans() {
        let pages: Vec<(u32, &str)> = vec![
            (1u32, "Page one short content."),
            (2u32, "Page two short content."),
        ];
        let chunks = chunk_pages("test", &pages, &ChunkConfig::default()).unwrap();
        assert_eq!(chunks.len(), 1);
        let c = &chunks[0];
        assert_eq!(c.start_page, 1);
        assert_eq!(c.end_page, 2);
        assert_eq!(c.page_spans.len(), 2);
        assert_eq!(c.page_spans[0].page, 1);
        assert_eq!(c.page_spans[0].byte_offset_in_chunk, 0);
        assert_eq!(c.page_spans[1].page, 2);
        // First paragraph is "Page one short content." = 23 chars + 1 join space = 24.
        assert_eq!(c.page_spans[1].byte_offset_in_chunk, 24);
    }

    #[test]
    fn page_spans_offset_uses_character_count_not_byte_count() {
        // Page 1's paragraph contains a 4-byte UTF-8 codepoint
        // (U+1F600 smiley = 4 UTF-8 bytes but 1 character). The
        // page-2 byte_offset_in_chunk must be the CHARACTER count
        // of paragraph 1 + 1 (the join space) -- never the byte
        // count. A regression that swaps `.chars().count()` for
        // `.len()` (str byte length) would fail this test.
        let page1 = "Smile this is a paragraph with a smiley \u{1F600}.";
        let page2 = "Second page paragraph here.";
        let pages: Vec<(u32, &str)> = vec![(1u32, page1), (2u32, page2)];
        let chunks = chunk_pages("test", &pages, &ChunkConfig::default()).unwrap();
        assert_eq!(chunks.len(), 1);
        let c = &chunks[0];
        assert_eq!(c.page_spans.len(), 2);
        assert_eq!(c.page_spans[1].page, 2);

        // Sanity: the test is only meaningful if char-count != byte-count.
        assert!(
            page1.chars().count() < page1.len(),
            "fixture must contain a multi-byte char"
        );

        let expected = u32::try_from(page1.chars().count()).unwrap() + 1;
        assert_eq!(
            c.page_spans[1].byte_offset_in_chunk, expected,
            "offset must be character count + 1 (join), not byte count + 1"
        );
    }

    #[test]
    fn trim_overlap_for_next_drains_carry_when_page_budget_blocks() {
        // With max_pages_per_chunk = 1, each page must end up in its
        // own chunk. The carry from chunk 0 (page 1 paragraphs) would
        // put chunk 1's first paragraph (page 2) over the 1-page
        // budget; `_trim_overlap_for_next` must drain the carry to
        // empty so chunk 1 starts cleanly. Without the drain, chunk 1
        // would either be misformed or panic.
        let pages: Vec<(u32, &str)> = vec![
            (1u32, "First page paragraph that fits."),
            (2u32, "Second page paragraph alone."),
        ];
        let config = ChunkConfig {
            max_pages_per_chunk: 1,
            ..ChunkConfig::default()
        };
        let chunks = chunk_pages("test", &pages, &config).unwrap();
        assert_eq!(chunks.len(), 2, "max_pages=1 forces one chunk per page");
        assert_eq!(chunks[0].start_page, 1);
        assert_eq!(chunks[0].end_page, 1);
        assert_eq!(chunks[1].start_page, 2);
        assert_eq!(chunks[1].end_page, 2);
        // Chunk 1's text must be exactly page 2's content (carry
        // drained, no page-1 leak).
        assert_eq!(chunks[1].text, "Second page paragraph alone.");
    }

    #[test]
    fn sentence_boundary_does_not_fire_at_end_of_document() {
        // The boundary heuristic requires the NEXT token to start
        // uppercase; at end-of-document there is no next token
        // (`tokens.get(i+1)` returns None), so the boundary must
        // not fire on the final sentence-end. The full input lands
        // in one chunk with all 8 tokens.
        let pages: Vec<(u32, &str)> = vec![(1u32, "One two three four five six. End here.")];
        let chunks = chunk_pages("test", &pages, &ChunkConfig::default()).unwrap();
        assert_eq!(chunks.len(), 1);
        let c = &chunks[0];
        // 8 tokens: One two three four five six. End here.
        assert_eq!(c.token_count, 8);
        // The two paragraphs are joined with a single space so the
        // chunk text contains the literal "six. End here." (no
        // duplicated separator, no missing token).
        assert!(
            c.text.contains("six. End here."),
            "chunk text must preserve the EOD tokens; got: {}",
            c.text
        );
    }
}
