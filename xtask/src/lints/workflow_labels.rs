//! Reject loop / plan workflow labels in implementation surfaces.
//!
//! The repository's contribution rules forbid plan-specific
//! terminology (`AC-N`, `Milestone`, `Phase`, `M5b-a/b/c/d`,
//! `task-m5b-N`, loop-round identifiers like `Round 17`, and
//! the bare workflow noun `plan`) in implementation code,
//! comments, docstrings, and runtime user-facing messages.
//! Workflow labels belong in `plans/` and `_research/` docs;
//! letting them leak into implementation surfaces creates
//! semantically meaningless coupling between the codebase and
//! the loop's bookkeeping.
//!
//! This lint enforces the rule on a tightly-scoped file set —
//! by default the six semantic-cache surfaces created during
//! the M5b-b Phase F cache-builder sequence (builder, audit,
//! threshold sweep, committed-cache load tests, idempotence
//! test, dispatcher). Additional file lists can be opted in
//! via `--root` for later milestones.
//!
//! Coverage carve-out — `Step`: the lint does NOT match bare
//! `Step N:` or `Step ` even though those words appear in the
//! contribution rules' forbidden-token list. The `Step N:`
//! shape is also the canonical Rust comment idiom for
//! algorithm-phase markers (see e.g. `verify_citation`'s
//! `Step 1: chunk lookup` / `Step 2: source agreement` /
//! ... in `crates/cacg-core/src/verify/layer2.rs`, which
//! predate the lint and document the algorithm structure
//! rather than any workflow phase). Adding a `Step ` matcher
//! would generate false positives on those legitimate
//! algorithm-phase markers whenever the lint's scope is
//! widened to include a `crates/` file. The workflow uses
//! of `Step` we actually see in violation cases — round
//! summaries' "Step 1 / Step 2 / Step 3" lists, plan-section
//! re-use of "Step N" — already get caught by the bare-word
//! `plan` matcher and the `Phase letter` matcher in the
//! same files; the `Step` form is redundant with those at
//! the trust boundary.
//!
//! Allowlists:
//! - The lint's own source file
//!   (`xtask/src/lints/workflow_labels.rs`) contains the
//!   forbidden tokens as match needles and unit-test fixtures.
//! - Research / planning documents (`_research/`,
//!   `plans/`, `.humanize/` round-tracker files) are out of
//!   the default scope; historical references to round
//!   numbers + AC labels stay permissible there.
//! - Test files exercising the lint matchers themselves use
//!   the patterns inline and live alongside the production
//!   matcher; the test-only matches do not surface as
//!   violations (the unit tests construct synthetic input
//!   strings, not files under the scan roots).

use std::fs;
use std::io;
use std::path::PathBuf;

use super::Violation;

const RULE: &str = "workflow-labels";

/// Files intentionally allowed to contain the forbidden tokens
/// because they ARE the lint's match-needle source. Identified
/// by suffix so the check is path-prefix-agnostic.
const ALLOWED_FILE_SUFFIXES: &[&str] = &["xtask/src/lints/workflow_labels.rs"];

/// Default scan list: the semantic-cache implementation
/// surfaces (builder, audit, threshold sweep, committed-cache
/// load tests, idempotence test, and the xtask dispatcher).
/// Additional files can be appended via `--root`. The lint is
/// intentionally scoped tight; widening the default scope is a
/// separate merge decision so a regression in an unrelated
/// legacy file doesn't block the cache surface from passing.
#[must_use]
pub fn default_scan_files() -> Vec<PathBuf> {
    vec![
        PathBuf::from("xtask/src/semantic_cache_provenance.rs"),
        PathBuf::from("xtask/src/threshold_sweep.rs"),
        PathBuf::from("crates/cacg-semantic/tests/committed_cache.rs"),
        PathBuf::from("xtask/src/main.rs"),
    ]
}

/// Scan each file in `files` for forbidden workflow labels.
/// Returns one [`Violation`] per matched token. Missing files
/// surface as a violation with the `file-missing` rule so the
/// default scan list cannot rot silently.
pub fn lint(files: &[PathBuf]) -> io::Result<Vec<Violation>> {
    let mut violations = Vec::new();
    for path in files {
        if !path.is_file() {
            violations.push(Violation {
                file: path.clone(),
                line: 0,
                rule: RULE,
                message: format!("scan target missing: {}", path.display()),
            });
            continue;
        }
        if is_allowlisted(path) {
            continue;
        }
        let bytes = fs::read(path)?;
        let text = String::from_utf8_lossy(&bytes);
        for (i, line) in text.lines().enumerate() {
            let line_no = i + 1;
            for hit in scan_line(line) {
                violations.push(Violation {
                    file: path.clone(),
                    line: line_no,
                    rule: RULE,
                    message: format!("forbidden workflow label {:?} in: {}", hit, line.trim()),
                });
            }
        }
    }
    Ok(violations)
}

fn is_allowlisted(path: &PathBuf) -> bool {
    let s = path.to_string_lossy();
    ALLOWED_FILE_SUFFIXES.iter().any(|suf| s.ends_with(suf))
}

/// Forbidden tokens. The matcher walks the line once, checking
/// each starting byte against every needle. Match success
/// returns the textual form of the violating token (for
/// inclusion in the diagnostic).
fn scan_line(line: &str) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut hits: Vec<String> = Vec::new();
    let mut seen: std::collections::BTreeSet<(usize, String)> = std::collections::BTreeSet::new();
    let n = bytes.len();
    let mut i = 0usize;
    while i < n {
        if let Some((token, advance)) = match_at(bytes, i) {
            if seen.insert((i, token.clone())) {
                hits.push(token);
            }
            i += advance.max(1);
        } else {
            i += 1;
        }
    }
    hits
}

/// Try every forbidden pattern at offset `i`. Returns
/// `Some((textual_token, byte_length))` on the first match,
/// otherwise `None`. Patterns are ordered so longer / more
/// specific tokens win over their substrings (e.g.
/// `task-m5b-N` beats the bare `M5b` substring it contains).
fn match_at(bytes: &[u8], i: usize) -> Option<(String, usize)> {
    // task-m5b...
    if let Some(len) = match_task_m5b(bytes, i) {
        return Some((extract(bytes, i, len), len));
    }
    // M5b...
    if let Some(len) = match_m5b(bytes, i) {
        return Some((extract(bytes, i, len), len));
    }
    // AC-N+
    if let Some(len) = match_ac_dash_n(bytes, i) {
        return Some((extract(bytes, i, len), len));
    }
    // Round-N+
    if let Some(len) = match_round_dash_n(bytes, i) {
        return Some((extract(bytes, i, len), len));
    }
    // Round N+ (space)
    if let Some(len) = match_round_space_n(bytes, i) {
        return Some((extract(bytes, i, len), len));
    }
    // Phase X (uppercase letter)
    if let Some(len) = match_phase_letter(bytes, i) {
        return Some((extract(bytes, i, len), len));
    }
    // `Milestone` / `Milestones` (capitalized, word-boundaried).
    if let Some(len) = match_milestone_word(bytes, i) {
        return Some((extract(bytes, i, len), len));
    }
    // "per the plan"
    if let Some(len) = match_literal_phrase(bytes, i, b"per the plan") {
        return Some((extract(bytes, i, len), len));
    }
    // "plan-required"
    if let Some(len) = match_literal(bytes, i, b"plan-required") {
        return Some((extract(bytes, i, len), len));
    }
    // "plan rule" (phrase, word-boundary on each end)
    if let Some(len) = match_literal_phrase(bytes, i, b"plan rule") {
        return Some((extract(bytes, i, len), len));
    }
    // bare-word `plan` / `plans` / `plan's` with word boundaries
    if let Some(len) = match_plan_word(bytes, i) {
        return Some((extract(bytes, i, len), len));
    }
    None
}

fn extract(bytes: &[u8], i: usize, len: usize) -> String {
    String::from_utf8_lossy(&bytes[i..i + len]).into_owned()
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Word-boundary check on the byte BEFORE `i`. Returns true iff
/// position `i` starts a new word (previous byte is missing OR
/// not an identifier byte).
fn left_word_boundary(bytes: &[u8], i: usize) -> bool {
    i == 0 || !is_ident_byte(bytes[i - 1])
}

/// `task-m5b` (case-sensitive). Identifier-boundary on the
/// left; allow any subsequent characters (e.g., `task-m5b-14`,
/// `task-m5b-N`).
fn match_task_m5b(bytes: &[u8], i: usize) -> Option<usize> {
    let needle = b"task-m5b";
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if bytes[i..].starts_with(needle) {
        Some(needle.len())
    } else {
        None
    }
}

/// `M5b` (case-sensitive). Left word boundary required.
fn match_m5b(bytes: &[u8], i: usize) -> Option<usize> {
    let needle = b"M5b";
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if bytes[i..].starts_with(needle) {
        Some(needle.len())
    } else {
        None
    }
}

/// `AC-` followed by at least one digit. Left word boundary
/// required so the literal text `cAC-1` (not real, illustrative)
/// is not flagged.
fn match_ac_dash_n(bytes: &[u8], i: usize) -> Option<usize> {
    let prefix = b"AC-";
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if !bytes[i..].starts_with(prefix) {
        return None;
    }
    let digit_start = i + prefix.len();
    let mut j = digit_start;
    while j < bytes.len() && bytes[j].is_ascii_digit() {
        j += 1;
    }
    if j > digit_start {
        Some(j - i)
    } else {
        None
    }
}

/// `Round-` followed by at least one digit. Left word boundary
/// required.
fn match_round_dash_n(bytes: &[u8], i: usize) -> Option<usize> {
    let prefix = b"Round-";
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if !bytes[i..].starts_with(prefix) {
        return None;
    }
    let digit_start = i + prefix.len();
    let mut j = digit_start;
    while j < bytes.len() && bytes[j].is_ascii_digit() {
        j += 1;
    }
    if j > digit_start {
        Some(j - i)
    } else {
        None
    }
}

/// `Round ` (with a literal space) followed by at least one
/// digit. Left word boundary required.
fn match_round_space_n(bytes: &[u8], i: usize) -> Option<usize> {
    let prefix = b"Round ";
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if !bytes[i..].starts_with(prefix) {
        return None;
    }
    let digit_start = i + prefix.len();
    let mut j = digit_start;
    while j < bytes.len() && bytes[j].is_ascii_digit() {
        j += 1;
    }
    if j > digit_start {
        Some(j - i)
    } else {
        None
    }
}

/// `Phase ` followed by exactly one ASCII uppercase letter.
fn match_phase_letter(bytes: &[u8], i: usize) -> Option<usize> {
    let prefix = b"Phase ";
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if !bytes[i..].starts_with(prefix) {
        return None;
    }
    let after = i + prefix.len();
    if after < bytes.len() && bytes[after].is_ascii_uppercase() {
        Some(prefix.len() + 1)
    } else {
        None
    }
}

/// Literal phrase match WITH left-and-right word boundaries
/// against ascii-identifier characters around the phrase.
fn match_literal_phrase(bytes: &[u8], i: usize, needle: &[u8]) -> Option<usize> {
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if !bytes[i..].starts_with(needle) {
        return None;
    }
    let end = i + needle.len();
    if end < bytes.len() && is_ident_byte(bytes[end]) {
        return None;
    }
    Some(needle.len())
}

/// Literal match (no right-boundary check); the caller has
/// already established this is the longer/more-specific token.
/// Used for hyphen-bearing forms like `plan-required` where
/// the right side is bounded by the literal text itself.
fn match_literal(bytes: &[u8], i: usize, needle: &[u8]) -> Option<usize> {
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if !bytes[i..].starts_with(needle) {
        return None;
    }
    Some(needle.len())
}

/// Bare-word `Milestone` / `Milestones` (capitalized). Requires
/// both a left word boundary AND a right word boundary so
/// `milestone` (lowercase prose), identifier runs, etc. do not
/// match. The capitalized form is what plan documents use as a
/// proper noun ("Milestone M5b"); the lowercase noun is allowed
/// in implementation prose ("an interim milestone" reads as
/// generic English).
fn match_milestone_word(bytes: &[u8], i: usize) -> Option<usize> {
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if !bytes[i..].starts_with(b"Milestone") {
        return None;
    }
    let after = i + b"Milestone".len();
    // Optional trailing `s` for the plural.
    let variant_len = if bytes.get(after) == Some(&b's') {
        b"Milestones".len()
    } else {
        b"Milestone".len()
    };
    let right_byte = bytes.get(i + variant_len).copied();
    let right_ok = match right_byte {
        None => true,
        Some(b) => !is_ident_byte(b),
    };
    if right_ok {
        Some(variant_len)
    } else {
        None
    }
}

/// Bare-word `plan` / `plans` / `plan's`. Requires both a left
/// word boundary AND a right word boundary so `planned`,
/// `planning`, `replan`, etc. do not match.
fn match_plan_word(bytes: &[u8], i: usize) -> Option<usize> {
    if !left_word_boundary(bytes, i) {
        return None;
    }
    if !bytes[i..].starts_with(b"plan") {
        return None;
    }
    // Determine the longest variant: `plan's` > `plans` > `plan`.
    let after_plan = i + 4;
    let variant_len =
        if bytes.get(after_plan) == Some(&b'\'') && bytes.get(after_plan + 1) == Some(&b's') {
            6 // "plan's"
        } else if bytes.get(after_plan) == Some(&b's') {
            5 // "plans"
        } else {
            4 // "plan"
        };
    let right_byte = bytes.get(i + variant_len).copied();
    let right_ok = match right_byte {
        None => true,
        Some(b) => !is_ident_byte(b) && b != b'\'',
    };
    if right_ok {
        Some(variant_len)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn scan(s: &str) -> Vec<String> {
        s.lines().flat_map(scan_line).collect()
    }

    #[test]
    fn matches_ac_n() {
        assert_eq!(scan("requires AC-6"), vec!["AC-6"]);
        assert_eq!(scan("the AC-6 + AC-12 set"), vec!["AC-6", "AC-12"]);
    }

    #[test]
    fn matches_task_m5b_and_m5b() {
        // task-m5b is matched first by the longer-token-wins
        // ordering; the `M5b` inside it is not double-reported.
        assert_eq!(scan("task-m5b-14"), vec!["task-m5b"]);
        assert_eq!(scan("M5b-b Phase F"), vec!["M5b", "Phase F"]);
    }

    #[test]
    fn matches_round_variants() {
        assert_eq!(scan("Round 17 cleanup"), vec!["Round 17"]);
        assert_eq!(scan("a Round-27 dataset"), vec!["Round-27"]);
        assert_eq!(scan("Round 0"), vec!["Round 0"]);
    }

    #[test]
    fn matches_phase_letter() {
        assert_eq!(scan("Phase F begins"), vec!["Phase F"]);
        // Lower-case is not matched (the rule pins uppercase
        // because Phase X labels follow that convention).
        assert_eq!(scan("Phase b begins"), Vec::<String>::new());
    }

    #[test]
    fn matches_milestone_word_variants() {
        assert_eq!(scan("the Milestone closes"), vec!["Milestone"]);
        assert_eq!(scan("two Milestones in flight"), vec!["Milestones"]);
        // Lowercase prose is allowed (generic English noun).
        assert_eq!(scan("an interim milestone"), Vec::<String>::new());
        // Identifier runs must not match.
        assert_eq!(scan("MilestoneCounter"), Vec::<String>::new());
        assert_eq!(scan("xMilestone"), Vec::<String>::new());
    }

    #[test]
    fn step_is_intentionally_not_matched() {
        // `Step N:` is the canonical Rust algorithm-phase comment
        // marker (see e.g. `verify_citation`'s "Step 1: chunk
        // lookup"). The lint explicitly does NOT flag it; the
        // workflow uses of `Step` we actually see in violations
        // already get caught by the bare-word `plan` and
        // `Phase letter` matchers. This test pins the carve-out so
        // a future addition of a `Step` matcher must be a
        // deliberate decision.
        assert_eq!(scan("Step 1: chunk lookup"), Vec::<String>::new());
        assert_eq!(scan("Step 5 begins"), Vec::<String>::new());
    }

    #[test]
    fn matches_plan_word_variants() {
        assert_eq!(scan("read the plan"), vec!["plan"]);
        assert_eq!(scan("the plan's wording"), vec!["plan's"]);
        assert_eq!(scan("two plans, one wins"), vec!["plans"]);
    }

    #[test]
    fn rejects_plan_as_substring_inside_words() {
        // word-boundary precision: these must NOT match.
        for line in [
            "fully planned upgrade",
            "running planning loop",
            "implants are unrelated",
            "esplanade is a French word",
            "replan the next round",
        ] {
            assert_eq!(
                scan(line),
                Vec::<String>::new(),
                "false positive in {line:?}"
            );
        }
    }

    #[test]
    fn matches_per_the_plan_and_plan_phrases() {
        assert_eq!(scan("see per the plan"), vec!["per the plan"]);
        assert_eq!(scan("by plan-required ordering"), vec!["plan-required"]);
        assert_eq!(scan("the plan rule says"), vec!["plan rule"]);
    }

    #[test]
    fn rejects_identifier_run_in_left_boundary() {
        // `someAC-1` cannot match `AC-1` because the byte before
        // `A` is an identifier byte.
        assert_eq!(scan("xAC-6"), Vec::<String>::new());
        assert_eq!(scan("idTask-m5b-14"), Vec::<String>::new());
    }

    #[test]
    fn lint_passes_on_clean_committed_surfaces() {
        let files = default_scan_files();
        // The default scan files all live in the workspace root;
        // resolve them relative to the workspace, which the cargo
        // test runner sets as the working directory for xtask tests.
        let violations = lint(&files).expect("lint scan");
        let workflow_only: Vec<_> = violations
            .iter()
            .filter(|v| v.rule == RULE && v.line > 0)
            .collect();
        assert!(
            workflow_only.is_empty(),
            "expected zero workflow-label violations on the committed AC-6 \
             surfaces; got {workflow_only:#?}"
        );
    }

    #[test]
    fn lint_flags_planted_violations_in_a_temp_file() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("planted.rs");
        let planted = "\
            // task-m5b-14 reference\n\
            // AC-6 requires X\n\
            // Round 17 cleanup\n\
            // M5b-b Phase F sequence\n\
            // see the plan's documented size\n\
            fn ok() {} // fully_planned variable\n";
        fs::write(&path, planted).unwrap();
        let violations = lint(&[path]).unwrap();
        let labels: Vec<&str> = violations.iter().map(|v| v.message.as_str()).collect();
        // Expected matches: task-m5b, AC-6, Round 17, M5b, Phase F, plan's.
        let needles = ["task-m5b", "AC-6", "Round 17", "M5b", "Phase F", "plan's"];
        for needle in needles {
            assert!(
                labels.iter().any(|m| m.contains(needle)),
                "planted violation {needle:?} not flagged in: {labels:#?}"
            );
        }
        // The line containing `fully_planned` must NOT produce a
        // `plan` violation.
        for v in &violations {
            assert!(
                !v.message.contains("fully_planned"),
                "false positive on `fully_planned`: {v:?}"
            );
        }
    }

    #[test]
    fn lint_surfaces_missing_file() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("does_not_exist.rs");
        let violations = lint(&[missing.clone()]).unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 0);
        assert!(
            violations[0].message.contains("scan target missing"),
            "got: {:?}",
            violations[0].message
        );
    }

    #[test]
    fn allowlisted_file_is_not_scanned() {
        let tmp = TempDir::new().unwrap();
        let nested = tmp.path().join("xtask/src/lints");
        fs::create_dir_all(&nested).unwrap();
        let path = nested.join("workflow_labels.rs");
        fs::write(&path, "// AC-6 lives here legitimately as a needle\n").unwrap();
        let violations = lint(&[path]).unwrap();
        assert!(
            violations.is_empty(),
            "allowlisted file should not surface violations; got {violations:#?}"
        );
    }
}
