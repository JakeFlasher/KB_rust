//! Round-summary parser + verifier library.
//!
//! Mirrors the Python parser/verifier at
//! `legacy_python_oracle/src/cacg/integrate/round_summary.py` byte-equal on its public
//! boundary contracts. See `_research/m5b_round_summary_parser_spec.md`
//! for the line-by-line Python-Rust oracle reference.
//!
//! Public surface:
//!
//! * Parser primitives — [`extract_section`], [`extract_path_from_bullet`],
//!   [`parse_round_summary`], [`is_kb_relevant`], [`resolve_cited_path`].
//! * Carrier types — [`Verdict`], [`PathVerdict`], [`RoundSummaryResult`].
//! * Driver — [`verify_round_summary`] reads the summary, walks the
//!   structural gate ladder, then for each cited card threads through
//!   `cacg_core::verify::verify_one_card` so the
//!   exactly-one-journal-event-per-cited-card cardinality contract
//!   holds end-to-end. Errors surface as [`VerifyRoundSummaryError`].
//!
//! The file's basename matches the `round_summary*` pattern scanned
//! by `xtask::lints::runner_bypass`, which rejects direct
//! `verify_card` / `append_entry` calls. The driver routes every
//! cited card through `verify_one_card` (the allowed runner) and
//! never touches the layer-2 entry or the journal append directly,
//! so the lint stays green.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use cacg_core::chunks_index::ChunksIndex;
use cacg_core::retraction::{RetractionLoadError, RetractionSpec};
use cacg_core::source_matrix::AuthSpec;
use cacg_core::verify::bm25_hints::Bm25HintCache;
use cacg_core::verify::runner::RunnerError;
use cacg_core::verify::{verify_one_card, SemanticEvaluator};
use regex::{Regex, RegexBuilder};

/// Exact N/A sentinel line. Whitespace around the line is tolerated
/// by the caller's `strip()`; trailing non-whitespace content is not.
pub const NA_SENTINEL: &str = "N/A -- task not KB-relevant this round";

fn kb_relevant_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        RegexBuilder::new(r"(?:cards/|\.claude/knowledge/)")
            .unicode(true)
            .multi_line(false)
            .case_insensitive(false)
            .build()
            .expect("kb-relevant regex must compile")
    })
}

fn section_heading_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        RegexBuilder::new(r"^##\s+Knowledge Consulted\s*$")
            .unicode(true)
            .multi_line(true)
            .case_insensitive(false)
            .build()
            .expect("section-heading regex must compile")
    })
}

fn next_heading_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        RegexBuilder::new(r"^##\s")
            .unicode(true)
            .multi_line(true)
            .case_insensitive(false)
            .build()
            .expect("next-heading regex must compile")
    })
}

fn bullet_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        RegexBuilder::new(r"^\s*[-*]\s+([^\s].*?)\s*$")
            .unicode(true)
            .multi_line(false)
            .case_insensitive(false)
            .build()
            .expect("bullet regex must compile")
    })
}

/// Per-path verification outcome carried in [`RoundSummaryResult::paths`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verdict {
    /// Card verified successfully.
    Verified,
    /// Card present but verification failed, or the section itself
    /// produced a structural diagnostic (CACG-RS-002/003/004).
    Stale,
    /// File not found at the cited path.
    Missing,
}

impl Verdict {
    /// Upper-case wire string matching the Python `Verdict` enum values
    /// (`VERIFIED` / `STALE` / `MISSING`).
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "VERIFIED",
            Self::Stale => "STALE",
            Self::Missing => "MISSING",
        }
    }
}

/// One per-path verdict line. `path` is `(section)` for the
/// structural CACG-RS-002/003/004 diagnostics; otherwise it is the
/// cited path string as it appeared in the round summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathVerdict {
    /// Cited path as it appeared in the bullet (or `(section)`).
    pub path: String,
    /// Verification verdict.
    pub verdict: Verdict,
    /// Free-form detail (e.g. `file not found`, `CACG-RS-002: ...`).
    pub detail: String,
}

/// Top-level round-summary verification result mirroring the Python
/// `RoundSummaryResult` dataclass. The [`exit_code`](Self::exit_code)
/// ladder is byte-equal with `round_summary.py` lines 57-65.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoundSummaryResult {
    /// True when the `## Knowledge Consulted` section was exactly the
    /// `NA_SENTINEL` line with no cited bullets and the body has no
    /// KB-relevant references.
    pub is_na: bool,
    /// True when the `## Knowledge Consulted` heading was not found
    /// in the summary text.
    pub section_missing: bool,
    /// True when the summary body mentions `cards/` or
    /// `.claude/knowledge/` anywhere (not only inside the section).
    pub kb_relevant: bool,
    /// Per-path verdicts. Structural diagnostics use
    /// `path == "(section)"`.
    pub paths: Vec<PathVerdict>,
}

impl RoundSummaryResult {
    /// Process exit code matching the Python ladder:
    /// 1. `section_missing && kb_relevant` → 2
    /// 2. `is_na` → 0
    /// 3. any path verdict ≠ Verified → 1
    /// 4. otherwise → 0
    #[must_use]
    pub fn exit_code(&self) -> i32 {
        if self.section_missing && self.kb_relevant {
            return 2;
        }
        if self.is_na {
            return 0;
        }
        if self.paths.iter().any(|p| p.verdict != Verdict::Verified) {
            return 1;
        }
        0
    }
}

/// Return the body of `## Knowledge Consulted` bounded by the next
/// `## ` heading or end of input. Returns `None` when the heading
/// is absent.
#[must_use]
pub fn extract_section(text: &str) -> Option<&str> {
    let m = section_heading_re().find(text)?;
    let body_start = m.end();
    let rest = &text[body_start..];
    let body = match next_heading_re().find(rest) {
        Some(nm) => &rest[..nm.start()],
        None => rest,
    };
    Some(body)
}

/// Extract a path from a `- path -- reason` style bullet, accepting
/// the four documented shapes:
///
/// 1. backtick-quoted path wins: `` - `path/here` -- reason `` → `path/here`;
/// 2. else split on the first occurrence of ` -- `, ` — ` (em dash),
///    ` - `, or ` (` in that order;
/// 3. else return the first whitespace-separated token.
///
/// Returns `None` when the line is not a bullet (no `-`/`*` marker
/// or no non-whitespace body).
#[must_use]
pub fn extract_path_from_bullet(line: &str) -> Option<String> {
    let caps = bullet_re().captures(line)?;
    let raw = caps.get(1)?.as_str().trim();
    if let Some(stripped) = strip_backtick_quoted(raw) {
        return Some(stripped.to_string());
    }
    for separator in [" -- ", " \u{2014} ", " - ", " ("] {
        if let Some(idx) = raw.find(separator) {
            return Some(raw[..idx].trim().to_string());
        }
    }
    raw.split_whitespace().next().map(str::to_string)
}

/// Return the substring inside the FIRST `` ` ... ` `` pair of `raw`
/// when `raw` starts with a backtick AND the closing backtick is at
/// an index > 1 (i.e., the path is non-empty). Returns `None`
/// otherwise; the caller falls through to separator-based splitting.
fn strip_backtick_quoted(raw: &str) -> Option<&str> {
    if !raw.starts_with('`') {
        return None;
    }
    let after_first = &raw[1..];
    let close_offset = after_first.find('`')?;
    if close_offset == 0 {
        return None;
    }
    Some(&after_first[..close_offset])
}

/// Walk the lines of `text`, accumulate cited paths from bullets,
/// and set `is_na` when the exact [`NA_SENTINEL`] line appears (after
/// trimming). Empty lines are skipped. The caller is responsible for
/// detecting the sentinel-collision case (`is_na && !paths.is_empty()`)
/// and routing to the CACG-RS-002 diagnostic — this function does NOT
/// short-circuit on collision.
#[must_use]
pub fn parse_round_summary(text: &str) -> (bool, Vec<String>) {
    let mut is_na = false;
    let mut paths = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line == NA_SENTINEL {
            is_na = true;
            continue;
        }
        if let Some(path) = extract_path_from_bullet(raw) {
            paths.push(path);
        }
    }
    (is_na, paths)
}

/// Return true when the summary text mentions `cards/` or
/// `.claude/knowledge/` anywhere — including code blocks, prose, or
/// the cited bullets themselves. The Python parser does the same
/// unscoped search.
#[must_use]
pub fn is_kb_relevant(text: &str) -> bool {
    kb_relevant_re().is_match(text)
}

/// Resolve a cited path under the documented contract:
/// 1. try `cwd_root / cited`; if it exists, return it;
/// 2. try `summary_dir / cited`; if it exists, return it;
/// 3. else return `cwd_root / cited` so the downstream
///    missing-card diagnostic names where the user meant.
#[must_use]
pub fn resolve_cited_path(cited: &str, cwd_root: &Path, summary_dir: &Path) -> PathBuf {
    let cwd_relative = cwd_root.join(cited);
    if cwd_relative.exists() {
        return cwd_relative;
    }
    let summary_relative = summary_dir.join(cited);
    if summary_relative.exists() {
        return summary_relative;
    }
    cwd_relative
}

/// Errors surfaced by [`verify_round_summary`]. The CLI dispatcher
/// translates each variant to a diagnostic code (`CACG-CLI-001` for
/// read failures, `CACG-MAN-001` for retraction-load failures,
/// `CACG-JNL-001` for runner failures).
#[derive(Debug)]
pub enum VerifyRoundSummaryError {
    /// `std::fs::read_to_string` failed (file missing post-precheck,
    /// permission denied, invalid UTF-8, etc.). Carries the offending
    /// path and the underlying error.
    ReadSummary {
        /// The summary path that could not be read.
        path: PathBuf,
        /// Underlying I/O / UTF-8 error.
        source: std::io::Error,
    },
    /// `RetractionSpec::from_cards_manifest_lenient` rejected a
    /// present-but-malformed `cards_manifest.json` next to the
    /// chunks manifest. Translates to `CACG-MAN-001` at the CLI.
    Retraction(RetractionLoadError),
    /// The per-card `verify_one_card` runner failed at the journal
    /// append layer. Translates to `CACG-JNL-001` at the CLI.
    Runner(RunnerError),
}

impl std::fmt::Display for VerifyRoundSummaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadSummary { path, source } => {
                write!(f, "cannot read round summary {}: {source}", path.display())
            }
            Self::Retraction(e) => write!(f, "{e}"),
            Self::Runner(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for VerifyRoundSummaryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadSummary { source, .. } => Some(source),
            Self::Retraction(e) => Some(e),
            Self::Runner(e) => Some(e),
        }
    }
}

impl From<RetractionLoadError> for VerifyRoundSummaryError {
    fn from(e: RetractionLoadError) -> Self {
        Self::Retraction(e)
    }
}

impl From<RunnerError> for VerifyRoundSummaryError {
    fn from(e: RunnerError) -> Self {
        Self::Runner(e)
    }
}

/// Walk a round-summary file end-to-end: section extraction, state
/// machine, per-batch loads, per-card verifier loop. Mirrors Python
/// `round_summary.py::verify_round_summary` (lines 129–300) on the
/// trust-bearing surface (diagnostic codes, per-path verdicts,
/// per-card journal cardinality).
///
/// `cwd_root` defaults to `std::env::current_dir()` when `None`,
/// matching Python's `Path.cwd()` fallback. Tests typically pass
/// `Some(tempdir.path())` to escape the live process cwd.
///
/// # Errors
///
/// See [`VerifyRoundSummaryError`].
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn verify_round_summary(
    summary_path: &Path,
    chunks_manifest_path: &Path,
    journal_path: &Path,
    fuzzy: bool,
    source_matrix_path: Option<&Path>,
    allow_retracted: bool,
    cwd_root: Option<&Path>,
    semantic: Option<&dyn SemanticEvaluator>,
) -> Result<RoundSummaryResult, VerifyRoundSummaryError> {
    let text = std::fs::read_to_string(summary_path).map_err(|source| {
        VerifyRoundSummaryError::ReadSummary {
            path: summary_path.to_path_buf(),
            source,
        }
    })?;

    let summary_dir = summary_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let resolved_cwd = match cwd_root {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    };

    let kb_relevant = is_kb_relevant(&text);

    let Some(section) = extract_section(&text) else {
        return Ok(RoundSummaryResult {
            is_na: false,
            section_missing: true,
            kb_relevant,
            paths: Vec::new(),
        });
    };

    let (is_na, parsed_paths) = parse_round_summary(section);
    let sentinel_collision = is_na && !parsed_paths.is_empty();

    if parsed_paths.is_empty() {
        if is_na {
            if kb_relevant {
                return Ok(RoundSummaryResult {
                    is_na: false,
                    section_missing: false,
                    kb_relevant: true,
                    paths: vec![PathVerdict {
                        path: "(section)".into(),
                        verdict: Verdict::Stale,
                        detail: "CACG-RS-003: N/A sentinel claimed on KB-relevant work".into(),
                    }],
                });
            }
            return Ok(RoundSummaryResult {
                is_na: true,
                section_missing: false,
                kb_relevant: false,
                paths: Vec::new(),
            });
        }
        if kb_relevant {
            return Ok(RoundSummaryResult {
                is_na: false,
                section_missing: false,
                kb_relevant: true,
                paths: vec![PathVerdict {
                    path: "(section)".into(),
                    verdict: Verdict::Stale,
                    detail: "CACG-RS-004: empty Knowledge Consulted section on KB-relevant work"
                        .into(),
                }],
            });
        }
        return Ok(RoundSummaryResult {
            is_na: false,
            section_missing: false,
            kb_relevant: false,
            paths: Vec::new(),
        });
    }

    if sentinel_collision {
        return Ok(RoundSummaryResult {
            is_na: false,
            section_missing: false,
            kb_relevant,
            paths: vec![PathVerdict {
                path: "(section)".into(),
                verdict: Verdict::Stale,
                detail: "CACG-RS-002: N/A sentinel mixed with cited paths".into(),
            }],
        });
    }

    let chunks_index = ChunksIndex::from_path(chunks_manifest_path).ok(); // qg-allow: intentional-discard — chunks index is optional for round summary
    let auth = AuthSpec::from_optional_path(source_matrix_path);
    let cards_manifest_path = chunks_manifest_path.parent().map_or_else(
        || PathBuf::from("cards_manifest.json"),
        |p| p.join("cards_manifest.json"),
    );
    let retraction =
        RetractionSpec::from_cards_manifest_lenient(&cards_manifest_path, allow_retracted)?;

    let mut verdicts = Vec::with_capacity(parsed_paths.len());
    // One BM25 hint corpus cache for the whole round-summary batch:
    // when multiple cited cards share a source, the per-source
    // tokenized corpus + Bm25Okapi index is built once and reused
    // for every subsequent Layer-2 failure that needs hints, matching
    // Python `_cmd_verify_round_summary`'s per-batch `BM25HintCache()`.
    let mut bm25_hint_cache = Bm25HintCache::new();
    for cited in parsed_paths {
        let candidate = resolve_cited_path(&cited, &resolved_cwd, &summary_dir);
        let result = verify_one_card(
            &candidate,
            chunks_manifest_path,
            journal_path,
            fuzzy,
            false,
            chunks_index.as_ref(),
            if auth.enabled() { Some(&auth) } else { None },
            if retraction.enabled() {
                Some(&retraction)
            } else {
                None
            },
            semantic,
            Some(&mut bm25_hint_cache),
        )?;
        if !candidate.exists() {
            verdicts.push(PathVerdict {
                path: cited,
                verdict: Verdict::Missing,
                detail: "file not found".into(),
            });
            continue;
        }
        if result.verified {
            verdicts.push(PathVerdict {
                path: cited,
                verdict: Verdict::Verified,
                detail: String::new(),
            });
        } else {
            let first_code = result
                .diagnostics
                .first()
                .map_or("CACG-VERIFY-001", |d| d.code.as_str());
            verdicts.push(PathVerdict {
                path: cited,
                verdict: Verdict::Stale,
                detail: format!("verify failed: {first_code}"),
            });
        }
    }

    Ok(RoundSummaryResult {
        is_na: false,
        section_missing: false,
        kb_relevant,
        paths: verdicts,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // ---------- constants + regex sanity ----------

    #[test]
    fn na_sentinel_value_matches_python() {
        assert_eq!(NA_SENTINEL, "N/A -- task not KB-relevant this round");
    }

    #[test]
    fn kb_relevant_re_accepts_documented_prefixes() {
        assert!(is_kb_relevant("see cards/foo.md"));
        assert!(is_kb_relevant(".claude/knowledge/qm/x.md"));
        assert!(is_kb_relevant("inline `cards/foo.md` reference"));
    }

    #[test]
    fn kb_relevant_re_rejects_lookalikes() {
        assert!(!is_kb_relevant("Cards/Foo.md"));
        assert!(!is_kb_relevant("a card/foo.md"));
        assert!(!is_kb_relevant(".claude/Knowledge/foo.md"));
        assert!(!is_kb_relevant("text with no kb hints"));
    }

    // ---------- extract_section ----------

    #[test]
    fn extract_section_returns_body_up_to_next_h2() {
        let text = "intro\n\n## Knowledge Consulted\n- a\n- b\n\n## Next\nignored\n";
        let body = extract_section(text).expect("section present");
        assert_eq!(body, "\n- a\n- b\n\n");
    }

    #[test]
    fn extract_section_returns_rest_when_no_next_heading() {
        let text = "## Knowledge Consulted\n- only\n";
        let body = extract_section(text).expect("section present");
        assert_eq!(body, "\n- only\n");
    }

    #[test]
    fn extract_section_returns_none_when_absent() {
        let text = "## Other Section\nbody\n";
        assert!(extract_section(text).is_none());
    }

    #[test]
    fn extract_section_rejects_lookalike_headings() {
        assert!(extract_section("## knowledge-consulted\n").is_none());
        assert!(extract_section("## Knowledge_Consulted\n").is_none());
        assert!(extract_section("### Knowledge Consulted\n").is_none());
        assert!(extract_section("##Knowledge Consulted\n").is_none());
    }

    #[test]
    fn extract_section_keeps_only_first_when_repeated() {
        let text = "## Knowledge Consulted\n- a\n## Knowledge Consulted\n- b\n";
        let body = extract_section(text).expect("first section");
        // Body is everything between the first heading and the next `## `
        // line, which is the second occurrence of the heading itself.
        assert_eq!(body, "\n- a\n");
    }

    #[test]
    fn extract_section_tolerates_crlf_heading() {
        let text = "## Knowledge Consulted\r\n- a\r\n\r\n## Next\r\n";
        let body = extract_section(text).expect("section present");
        assert!(body.contains("- a"));
    }

    // ---------- extract_path_from_bullet ----------

    #[test]
    fn bullet_double_dash_separator() {
        assert_eq!(
            extract_path_from_bullet("- cards/qm/foo.md -- because reasons"),
            Some("cards/qm/foo.md".to_string()),
        );
    }

    #[test]
    fn bullet_single_dash_separator() {
        assert_eq!(
            extract_path_from_bullet("- cards/qm/foo.md - because"),
            Some("cards/qm/foo.md".to_string()),
        );
    }

    #[test]
    fn bullet_no_separator_returns_first_token() {
        assert_eq!(
            extract_path_from_bullet("- cards/qm/foo.md"),
            Some("cards/qm/foo.md".to_string()),
        );
    }

    #[test]
    fn bullet_asterisk_marker() {
        assert_eq!(
            extract_path_from_bullet("* cards/qm/foo.md -- reason"),
            Some("cards/qm/foo.md".to_string()),
        );
    }

    #[test]
    fn bullet_backtick_quoted_path_wins_over_separator() {
        assert_eq!(
            extract_path_from_bullet("- `cards/qm with spaces.md` -- reason"),
            Some("cards/qm with spaces.md".to_string()),
        );
    }

    #[test]
    fn bullet_paren_separator() {
        assert_eq!(
            extract_path_from_bullet("- cards/qm/foo.md (annotated)"),
            Some("cards/qm/foo.md".to_string()),
        );
    }

    #[test]
    fn bullet_em_dash_separator_with_spaces() {
        assert_eq!(
            extract_path_from_bullet("- cards/qm/foo.md \u{2014} reason"),
            Some("cards/qm/foo.md".to_string()),
        );
    }

    #[test]
    fn bullet_em_dash_no_surrounding_spaces_falls_through_to_first_token() {
        // "a\u{2014}b" has no `" \u{2014} "` substring, no other
        // separator, so the first whitespace-separated token is the
        // whole atom.
        assert_eq!(
            extract_path_from_bullet("- cards/foo\u{2014}bar.md"),
            Some("cards/foo\u{2014}bar.md".to_string()),
        );
    }

    #[test]
    fn bullet_internal_dash_without_surrounding_spaces_keeps_token() {
        // "a--b" has no `" -- "` and no other separator; whole token
        // returned.
        assert_eq!(
            extract_path_from_bullet("- card-id--variant.md"),
            Some("card-id--variant.md".to_string()),
        );
    }

    #[test]
    fn bullet_multiple_leading_spaces_in_body_returns_first_token() {
        // After regex captures and `.trim()`, leading whitespace is
        // gone; the first whitespace-separated token of the body is
        // returned when no separator matches.
        assert_eq!(
            extract_path_from_bullet("-   word1   word2   word3"),
            Some("word1".to_string()),
        );
    }

    #[test]
    fn bullet_no_marker_returns_none() {
        assert!(extract_path_from_bullet("not a bullet line").is_none());
        assert!(extract_path_from_bullet("+ wrong marker").is_none());
        assert!(extract_path_from_bullet("1. numbered").is_none());
    }

    #[test]
    fn bullet_marker_only_returns_none() {
        // `- ` without any body fails the `([^\s]...)` capture
        // because the first captured char must be non-whitespace.
        assert!(extract_path_from_bullet("- ").is_none());
        assert!(extract_path_from_bullet("-").is_none());
    }

    #[test]
    fn bullet_empty_backticks_fall_through() {
        // `\`\`` has the closing backtick at offset 0 within the
        // after-first slice, so the stripper returns None and we
        // fall through. No separator matches, so the first
        // whitespace token (the literal `` `` ``) is returned.
        assert_eq!(extract_path_from_bullet("- ``"), Some("``".to_string()),);
    }

    #[test]
    fn bullet_indented_is_accepted() {
        assert_eq!(
            extract_path_from_bullet("    - cards/foo.md"),
            Some("cards/foo.md".to_string()),
        );
    }

    #[test]
    fn bullet_with_windows_style_separator_keeps_full_token() {
        // Backslash is not whitespace and none of the four
        // separator strings (` -- `, ` \u{2014} `, ` - `, ` (`)
        // appear in the token, so the whole atom is returned as
        // a single whitespace-bounded token (Python parity:
        // Python's `raw.split()[0]` does the same).
        assert_eq!(
            extract_path_from_bullet(r"- cards\reading_01\foo.md"),
            Some(r"cards\reading_01\foo.md".to_string()),
        );
    }

    #[test]
    fn bullet_unquoted_path_with_spaces_keeps_first_whitespace_token() {
        // Unquoted paths cannot carry literal spaces because the
        // bullet body's first whitespace ends the token (Python
        // parity: `raw.split()[0]`). Callers who need spaces must
        // use backtick-quoted form.
        assert_eq!(
            extract_path_from_bullet("- cards/reading 01/foo.md"),
            Some("cards/reading".to_string()),
        );
    }

    // ---------- parse_round_summary ----------

    #[test]
    fn parse_empty_text_returns_no_paths() {
        let (is_na, paths) = parse_round_summary("");
        assert!(!is_na);
        assert!(paths.is_empty());
    }

    #[test]
    fn parse_na_sentinel_only() {
        let (is_na, paths) = parse_round_summary(NA_SENTINEL);
        assert!(is_na);
        assert!(paths.is_empty());
    }

    #[test]
    fn parse_bullets_only() {
        let body = "- cards/a.md\n- cards/b.md -- with reason\n";
        let (is_na, paths) = parse_round_summary(body);
        assert!(!is_na);
        assert_eq!(
            paths,
            vec!["cards/a.md".to_string(), "cards/b.md".to_string()]
        );
    }

    #[test]
    fn parse_na_with_paths_returns_both_signals() {
        // Caller is responsible for detecting the collision and
        // routing to CACG-RS-002; the parser itself does not short
        // circuit here.
        let body = format!("{NA_SENTINEL}\n- cards/a.md\n");
        let (is_na, paths) = parse_round_summary(&body);
        assert!(is_na);
        assert_eq!(paths, vec!["cards/a.md".to_string()]);
    }

    #[test]
    fn parse_skips_empty_lines() {
        let body = "\n\n- cards/a.md\n\n- cards/b.md\n\n";
        let (_, paths) = parse_round_summary(body);
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn parse_preserves_duplicate_paths_in_order() {
        // The parser must NOT dedup; the round-summary batch path
        // re-runs the verifier for every cited occurrence so the
        // journal records one event per visit.
        let body = "- cards/a.md\n- cards/a.md\n- cards/b.md\n- cards/a.md\n";
        let (_, paths) = parse_round_summary(body);
        assert_eq!(
            paths,
            vec![
                "cards/a.md".to_string(),
                "cards/a.md".to_string(),
                "cards/b.md".to_string(),
                "cards/a.md".to_string(),
            ],
        );
    }

    #[test]
    fn parse_tolerates_crlf_line_endings() {
        let body = "- cards/a.md\r\n- cards/b.md\r\n";
        let (_, paths) = parse_round_summary(body);
        assert_eq!(
            paths,
            vec!["cards/a.md".to_string(), "cards/b.md".to_string()]
        );
    }

    // ---------- resolve_cited_path ----------

    #[test]
    fn resolve_returns_cwd_relative_when_it_exists() {
        let dir = TempDir::new().unwrap();
        let cwd = dir.path();
        let target = cwd.join("cards").join("a.md");
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(&target, "x").unwrap();
        let summary_dir = dir.path().join("other");
        fs::create_dir_all(&summary_dir).unwrap();
        let resolved = resolve_cited_path("cards/a.md", cwd, &summary_dir);
        assert_eq!(resolved, target);
    }

    #[test]
    fn resolve_falls_back_to_summary_dir_when_cwd_missing() {
        let dir = TempDir::new().unwrap();
        let cwd = dir.path().join("cwd");
        let summary_dir = dir.path().join("summaries");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(summary_dir.join("cards")).unwrap();
        let target = summary_dir.join("cards").join("a.md");
        fs::write(&target, "x").unwrap();
        let resolved = resolve_cited_path("cards/a.md", &cwd, &summary_dir);
        assert_eq!(resolved, target);
    }

    #[test]
    fn resolve_returns_cwd_relative_fallback_when_neither_exists() {
        let dir = TempDir::new().unwrap();
        let cwd = dir.path().join("cwd");
        let summary_dir = dir.path().join("summaries");
        fs::create_dir_all(&cwd).unwrap();
        fs::create_dir_all(&summary_dir).unwrap();
        let resolved = resolve_cited_path("cards/missing.md", &cwd, &summary_dir);
        assert_eq!(resolved, cwd.join("cards").join("missing.md"));
    }

    #[test]
    fn resolve_prefers_cwd_when_both_exist() {
        // Python parity: CWD-relative wins over summary-dir-relative.
        let dir = TempDir::new().unwrap();
        let cwd = dir.path().join("cwd");
        let summary_dir = dir.path().join("summaries");
        let cwd_target = cwd.join("cards").join("a.md");
        let sum_target = summary_dir.join("cards").join("a.md");
        fs::create_dir_all(cwd_target.parent().unwrap()).unwrap();
        fs::create_dir_all(sum_target.parent().unwrap()).unwrap();
        fs::write(&cwd_target, "from cwd").unwrap();
        fs::write(&sum_target, "from summary").unwrap();
        let resolved = resolve_cited_path("cards/a.md", &cwd, &summary_dir);
        assert_eq!(resolved, cwd_target);
    }

    // ---------- Verdict ----------

    #[test]
    fn verdict_wire_strings_match_python() {
        assert_eq!(Verdict::Verified.as_str(), "VERIFIED");
        assert_eq!(Verdict::Stale.as_str(), "STALE");
        assert_eq!(Verdict::Missing.as_str(), "MISSING");
    }

    // ---------- RoundSummaryResult::exit_code ----------

    fn result(
        is_na: bool,
        section_missing: bool,
        kb_relevant: bool,
        verdicts: &[Verdict],
    ) -> RoundSummaryResult {
        RoundSummaryResult {
            is_na,
            section_missing,
            kb_relevant,
            paths: verdicts
                .iter()
                .map(|v| PathVerdict {
                    path: "(test)".into(),
                    verdict: *v,
                    detail: String::new(),
                })
                .collect(),
        }
    }

    #[test]
    fn exit_missing_section_on_kb_relevant_is_2() {
        let r = result(false, true, true, &[]);
        assert_eq!(r.exit_code(), 2);
    }

    #[test]
    fn exit_missing_section_on_non_kb_relevant_is_0() {
        let r = result(false, true, false, &[]);
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn exit_clean_na_is_0() {
        let r = result(true, false, false, &[]);
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn exit_any_non_verified_is_1() {
        let r = result(false, false, true, &[Verdict::Verified, Verdict::Stale]);
        assert_eq!(r.exit_code(), 1);
        let r = result(false, false, false, &[Verdict::Missing]);
        assert_eq!(r.exit_code(), 1);
    }

    #[test]
    fn exit_all_verified_is_0() {
        let r = result(false, false, true, &[Verdict::Verified, Verdict::Verified]);
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn exit_empty_paths_no_na_is_0() {
        let r = result(false, false, false, &[]);
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn exit_is_na_short_circuits_before_path_check() {
        // Per the Python ladder, `is_na` wins over any stale paths in
        // the list — the collision case where both are set is the
        // caller's responsibility to flag via CACG-RS-002 BEFORE
        // building this result.
        let r = result(true, false, false, &[Verdict::Stale]);
        assert_eq!(r.exit_code(), 0);
    }

    // ---------- verify_round_summary state-machine branches ----------
    //
    // These tests exercise the spec §7 ladder for every branch that
    // does NOT require the cacg-core verify pipeline to run. Each
    // test writes a minimal summary to a tempdir and asserts the
    // emitted `RoundSummaryResult` shape + exit code. The per-card
    // verify loop is covered in the integration suite at
    // `tests/kb_verify_round_summary.rs` where real cards exist.

    fn write_summary(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, body).unwrap();
        path
    }

    fn paths_for_test(dir: &Path) -> (PathBuf, PathBuf) {
        // Chunks manifest does not need to exist for the structural
        // early-return branches — they short-circuit before any load.
        (
            dir.join("chunks_manifest.json"),
            dir.join("lint_journal.jsonl"),
        )
    }

    #[test]
    fn verify_round_summary_section_missing_non_kb_relevant() {
        let dir = TempDir::new().unwrap();
        let summary = write_summary(dir.path(), "round.md", "no relevant heading\n");
        let (chunks, journal) = paths_for_test(dir.path());
        let r = verify_round_summary(
            &summary,
            &chunks,
            &journal,
            false,
            None,
            false,
            Some(dir.path()),
            None,
        )
        .unwrap();
        assert!(r.section_missing);
        assert!(!r.kb_relevant);
        assert!(r.paths.is_empty());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn verify_round_summary_section_missing_kb_relevant_signals_001() {
        let dir = TempDir::new().unwrap();
        let summary = write_summary(
            dir.path(),
            "round.md",
            "summary mentions cards/foo.md in passing\n",
        );
        let (chunks, journal) = paths_for_test(dir.path());
        let r = verify_round_summary(
            &summary,
            &chunks,
            &journal,
            false,
            None,
            false,
            Some(dir.path()),
            None,
        )
        .unwrap();
        assert!(r.section_missing);
        assert!(r.kb_relevant);
        assert!(r.paths.is_empty());
        assert_eq!(r.exit_code(), 2);
    }

    #[test]
    fn verify_round_summary_clean_na_non_kb_relevant() {
        let dir = TempDir::new().unwrap();
        let body = format!("## Knowledge Consulted\n\n{NA_SENTINEL}\n");
        let summary = write_summary(dir.path(), "round.md", &body);
        let (chunks, journal) = paths_for_test(dir.path());
        let r = verify_round_summary(
            &summary,
            &chunks,
            &journal,
            false,
            None,
            false,
            Some(dir.path()),
            None,
        )
        .unwrap();
        assert!(r.is_na);
        assert!(!r.section_missing);
        assert!(!r.kb_relevant);
        assert!(r.paths.is_empty());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn verify_round_summary_na_on_kb_relevant_emits_003() {
        let dir = TempDir::new().unwrap();
        let body = format!(
            "edited cards/foo.md in this round\n\n## Knowledge Consulted\n\n{NA_SENTINEL}\n",
        );
        let summary = write_summary(dir.path(), "round.md", &body);
        let (chunks, journal) = paths_for_test(dir.path());
        let r = verify_round_summary(
            &summary,
            &chunks,
            &journal,
            false,
            None,
            false,
            Some(dir.path()),
            None,
        )
        .unwrap();
        assert!(!r.is_na);
        assert!(!r.section_missing);
        assert!(r.kb_relevant);
        assert_eq!(r.paths.len(), 1);
        let v = &r.paths[0];
        assert_eq!(v.path, "(section)");
        assert_eq!(v.verdict, Verdict::Stale);
        assert!(v.detail.starts_with("CACG-RS-003"));
        assert_eq!(r.exit_code(), 1);
    }

    #[test]
    fn verify_round_summary_empty_section_kb_relevant_emits_004() {
        let dir = TempDir::new().unwrap();
        let body = "edited cards/foo.md in this round\n\n## Knowledge Consulted\n\n";
        let summary = write_summary(dir.path(), "round.md", body);
        let (chunks, journal) = paths_for_test(dir.path());
        let r = verify_round_summary(
            &summary,
            &chunks,
            &journal,
            false,
            None,
            false,
            Some(dir.path()),
            None,
        )
        .unwrap();
        assert!(!r.is_na);
        assert!(!r.section_missing);
        assert!(r.kb_relevant);
        assert_eq!(r.paths.len(), 1);
        let v = &r.paths[0];
        assert_eq!(v.path, "(section)");
        assert_eq!(v.verdict, Verdict::Stale);
        assert!(v.detail.starts_with("CACG-RS-004"));
        assert_eq!(r.exit_code(), 1);
    }

    #[test]
    fn verify_round_summary_empty_section_non_kb_relevant_clean() {
        let dir = TempDir::new().unwrap();
        let body = "## Knowledge Consulted\n\n";
        let summary = write_summary(dir.path(), "round.md", body);
        let (chunks, journal) = paths_for_test(dir.path());
        let r = verify_round_summary(
            &summary,
            &chunks,
            &journal,
            false,
            None,
            false,
            Some(dir.path()),
            None,
        )
        .unwrap();
        assert!(!r.is_na);
        assert!(!r.section_missing);
        assert!(!r.kb_relevant);
        assert!(r.paths.is_empty());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn verify_round_summary_sentinel_collision_emits_002() {
        let dir = TempDir::new().unwrap();
        // Sentinel + bullet inside the section — the per-batch loads
        // must NOT fire (no chunks_manifest.json exists in tempdir).
        let body = format!("## Knowledge Consulted\n\n{NA_SENTINEL}\n- cards/foo.md\n",);
        let summary = write_summary(dir.path(), "round.md", &body);
        let (chunks, journal) = paths_for_test(dir.path());
        let r = verify_round_summary(
            &summary,
            &chunks,
            &journal,
            false,
            None,
            false,
            Some(dir.path()),
            None,
        )
        .unwrap();
        assert!(!r.is_na);
        assert!(!r.section_missing);
        assert_eq!(r.paths.len(), 1);
        let v = &r.paths[0];
        assert_eq!(v.path, "(section)");
        assert_eq!(v.verdict, Verdict::Stale);
        assert!(v.detail.starts_with("CACG-RS-002"));
        assert_eq!(r.exit_code(), 1);
    }

    #[test]
    fn verify_round_summary_missing_summary_path_errors() {
        let dir = TempDir::new().unwrap();
        let missing = dir.path().join("does_not_exist.md");
        let (chunks, journal) = paths_for_test(dir.path());
        let err = verify_round_summary(
            &missing,
            &chunks,
            &journal,
            false,
            None,
            false,
            Some(dir.path()),
            None,
        )
        .unwrap_err();
        match err {
            VerifyRoundSummaryError::ReadSummary { path, .. } => assert_eq!(path, missing),
            VerifyRoundSummaryError::Retraction(_) | VerifyRoundSummaryError::Runner(_) => {
                panic!("expected ReadSummary, got {err:?}")
            }
        }
    }

    /// Proves that `verify_round_summary` threads the supplied
    /// semantic evaluator into EVERY per-cite `verify_one_card`
    /// call — repeated cited paths must produce repeated,
    /// independent evaluator invocations (no within-run
    /// memoization).
    ///
    /// Fixture: a tmpdir with two synthetic cards (card-A and
    /// card-B). Each card has one citation that PINS a real
    /// chunk from the committed parity-corpus chunks_manifest
    /// (so Layer-1 hash + manifest tamper checks pass) but whose
    /// `quote` is intentionally absent from the chunk's text, so
    /// Layer-2 fails (`CACG-VERIFY-001`) on every cite and
    /// Layer-3 fires unconditionally. The summary cites card-A 3x
    /// and card-B 1x. A local `CountingEvaluator` tracks every
    /// `evaluate` call.
    #[test]
    fn round_summary_threads_evaluator_through_each_cite() {
        use cacg_core::verify::{
            SemanticEvaluationError, SemanticMode, SemanticVerdict, SemanticVerdictKind,
        };
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct CountingEvaluator {
            count: AtomicUsize,
        }
        impl SemanticEvaluator for CountingEvaluator {
            fn evaluate(
                &self,
                _chunk_hash: &str,
                _claim_window_hash: &str,
                _quote: &str,
                _chunk_text: &str,
            ) -> Result<SemanticVerdict, SemanticEvaluationError> {
                self.count.fetch_add(1, Ordering::SeqCst);
                Ok(SemanticVerdict {
                    kind: SemanticVerdictKind::Fail,
                    score: 0.1,
                    reasoning: None,
                    mode: SemanticMode::EmbeddingCache,
                })
            }
        }

        let dir = TempDir::new().unwrap();
        let cards_dir = dir.path().join("cards/reading_01");
        fs::create_dir_all(&cards_dir).unwrap();

        // Reuse the committed parity-corpus chunks_manifest so
        // Layer-1 (chunk_hash match + manifest tamper check)
        // passes. The two synthetic cards pin those same chunk
        // ids + chunk hashes but quote text that DOES NOT appear
        // in either chunk's text, so Layer-2 fails and Layer-3
        // fires per cite.
        let workspace_root = {
            let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            p.pop();
            p.pop();
            p
        };
        let chunks_path =
            workspace_root.join("tests/parity_corpus/out_python/chunks_manifest.json");
        assert!(
            chunks_path.is_file(),
            "committed parity-corpus chunks_manifest must exist for the synthetic fixture",
        );

        // Real chunk_hash values from the committed manifest:
        let chunk_a_hash = "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895";
        let chunk_b_hash = "4a911a87c52547070faf29f6ece93965d095f0b2fb3ea49b0540b4af5cb842a9";

        let card_a_body = format!(
            "---\n\
            schema_version: \"cacg.v0\"\n\
            id: \"synthetic-card-a\"\n\
            title: \"Synthetic Card A\"\n\
            reading_id: \"reading_01\"\n\
            summary: \"Synthetic card A used by the round-summary semantic counting test; the citation quote text below is intentionally not a substring of the pinned chunk text so Layer-2 emits CACG-VERIFY-001 and the supplied semantic evaluator is invoked on every visit.\"\n\
            citations:\n\
            \x20\x20- source_id: \"sample\"\n\
            \x20\x20\x20\x20chunk_id: \"sample:p001:0000\"\n\
            \x20\x20\x20\x20chunk_hash: \"{chunk_a_hash}\"\n\
            \x20\x20\x20\x20page_range: [1, 2]\n\
            \x20\x20\x20\x20quote: \"this exact phrase is intentionally not a substring of card A's pinned chunk text\"\n\
            \x20\x20\x20\x20edge_type: \"supports\"\n\
            ---\n\
            Body.\n"
        );
        let card_a = cards_dir.join("synthetic-card-a.md");
        fs::write(&card_a, card_a_body).unwrap();

        let card_b_body = format!(
            "---\n\
            schema_version: \"cacg.v0\"\n\
            id: \"synthetic-card-b\"\n\
            title: \"Synthetic Card B\"\n\
            reading_id: \"reading_01\"\n\
            summary: \"Synthetic card B used by the round-summary semantic counting test; the citation quote text below is intentionally not a substring of the pinned chunk text so Layer-2 emits CACG-VERIFY-001 and the supplied semantic evaluator is invoked on every visit.\"\n\
            citations:\n\
            \x20\x20- source_id: \"sample\"\n\
            \x20\x20\x20\x20chunk_id: \"sample:p002:0001\"\n\
            \x20\x20\x20\x20chunk_hash: \"{chunk_b_hash}\"\n\
            \x20\x20\x20\x20page_range: [2, 3]\n\
            \x20\x20\x20\x20quote: \"this exact phrase is intentionally not a substring of card B's pinned chunk text\"\n\
            \x20\x20\x20\x20edge_type: \"supports\"\n\
            ---\n\
            Body.\n"
        );
        let card_b = cards_dir.join("synthetic-card-b.md");
        fs::write(&card_b, card_b_body).unwrap();

        // Summary cites card-A three times and card-B once. Paths
        // are tmpdir-relative; the round-summary path resolver
        // uses `cwd_root` (passed as `Some(dir.path())` below) to
        // resolve them.
        let summary_body = "## Knowledge Consulted\n\n\
            - cards/reading_01/synthetic-card-a.md\n\
            - cards/reading_01/synthetic-card-a.md\n\
            - cards/reading_01/synthetic-card-a.md\n\
            - cards/reading_01/synthetic-card-b.md\n";
        let summary = dir.path().join("summary.md");
        fs::write(&summary, summary_body).unwrap();
        let journal = dir.path().join("lint_journal.jsonl");

        let evaluator = CountingEvaluator {
            count: AtomicUsize::new(0),
        };
        let result = verify_round_summary(
            &summary,
            &chunks_path,
            &journal,
            /* fuzzy */ false,
            /* source_matrix_path */ None,
            /* allow_retracted */ false,
            /* cwd_root */ Some(dir.path()),
            /* semantic */ Some(&evaluator),
        )
        .expect("verify_round_summary against synthetic fixture");

        // 4 cited paths → 4 path verdicts.
        assert_eq!(
            result.paths.len(),
            4,
            "card-A 3x + card-B 1x = 4 cited paths; got: {result:?}",
        );
        // All four are Stale because Layer-2 emits CACG-VERIFY-001
        // on every cite (the quote isn't in any chunk's text).
        for v in &result.paths {
            assert_eq!(
                v.verdict,
                Verdict::Stale,
                "every cite must be Stale; got: {v:?}",
            );
            assert!(
                v.detail.contains("CACG-VERIFY-001"),
                "Layer-2 must surface CACG-VERIFY-001; got: {v:?}",
            );
        }
        // First three cited paths point at card-A, the fourth at
        // card-B.
        assert!(result.paths[0].path.ends_with("synthetic-card-a.md"));
        assert!(result.paths[1].path.ends_with("synthetic-card-a.md"));
        assert!(result.paths[2].path.ends_with("synthetic-card-a.md"));
        assert!(result.paths[3].path.ends_with("synthetic-card-b.md"));

        // 4 verify_one_card invocations → 4 journal events.
        let journal_lines = fs::read_to_string(&journal)
            .expect("journal must be written")
            .lines()
            .filter(|l| !l.is_empty())
            .count();
        assert_eq!(
            journal_lines, 4,
            "exactly 4 journal events expected (one per cite)",
        );

        // Critical batch-semantic contract: same card visited 3
        // times produces 3 INDEPENDENT verdict evaluations. The
        // counter proves the dispatcher does not memoize the
        // evaluator's verdict across repeated visits.
        assert_eq!(
            evaluator.count.load(Ordering::SeqCst),
            4,
            "evaluator must be invoked once per cite (card-A 3x + card-B 1x = 4); a within-run memo would yield 2",
        );
    }
}
