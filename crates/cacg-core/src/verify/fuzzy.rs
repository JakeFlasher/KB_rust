//! Ratcliff-Obershelp gestalt-pattern-matching ratio + future fuzzy
//! primitives.
//!
//! `ratcliff_obershelp_ratio` mirrors CPython
//! `Lib/difflib.py::SequenceMatcher.ratio()` with `autojunk=True`
//! (the default). The algorithm finds the longest matching substring,
//! then recurses on the prefix + suffix of both sides; the ratio is
//! `2 * total_matches / (len(a) + len(b))`.
//!
//! `autojunk` is Python's heuristic to treat any element in `b` that
//! appears more than `n // 100 + 1` times as "popular" (when
//! `len(b) >= 200`), excluding it from the position map used in
//! `find_longest_match`. The popular element is still walked when
//! extending an existing match — the heuristic only affects the
//! initial best-match seeding loop. This matters for long Unicode
//! chunk texts where a single character (whitespace, common
//! punctuation) would otherwise drive the matching cost.
//!
//! Operates on Unicode code points (`Vec<char>`), NOT raw bytes, to
//! match Python `str` slicing semantics on `difflib.SequenceMatcher`.
//! Future callers in the verify hot path slice chunk text by
//! character indices for the same reason — page-window slicing on
//! non-ASCII chunk text requires code-point-aware offsets, not raw
//! byte offsets.

use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Bounded Wagner-Fischer Levenshtein distance with early cutoff.
///
/// Returns `Some(d)` iff the true Levenshtein edit distance between
/// `a` and `b` is `<= max_edits`; returns `None` otherwise. Mirrors
/// the truth value of Python `legacy_python_oracle/src/cacg/verify/fuzzy.py::levenshtein(a, b) <= max_dist`
/// on every input, but exits early when the answer cannot possibly
/// fit the budget.
///
/// Fast paths:
///
/// * `a == b` → `Some(0)` (no DP needed).
/// * `|len_chars(a) - len_chars(b)| > max_edits` → `None` (length-
///   difference lower bound: any path requires at least that many
///   inserts or deletes).
///
/// Otherwise, runs the full Wagner-Fischer DP over `Vec<char>`
/// slices, tracking each row's minimum value. When every cell in a
/// row exceeds `max_edits`, no descendant can be `<= max_edits` and
/// the function returns `None`.
///
/// Operates on Unicode code points: combining marks count as
/// separate code points and therefore separate edit positions.
#[must_use]
pub fn bounded_levenshtein(a: &str, b: &str, max_edits: u32) -> Option<u32> {
    if a == b {
        return Some(0);
    }
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let n = a_chars.len();
    let m = b_chars.len();
    let len_diff = if n >= m { n - m } else { m - n };
    if u32::try_from(len_diff).unwrap_or(u32::MAX) > max_edits {
        return None;
    }
    if n == 0 {
        let d = u32::try_from(m).unwrap_or(u32::MAX);
        return if d <= max_edits { Some(d) } else { None };
    }
    if m == 0 {
        let d = u32::try_from(n).unwrap_or(u32::MAX);
        return if d <= max_edits { Some(d) } else { None };
    }

    // Wagner-Fischer: previous-row buffer + current-row buffer.
    // The classic Python loop uses `prev = [0..m]; for i in a:
    // curr[0] = i; curr[j] = min(curr[j-1]+1, prev[j]+1, prev[j-1]+cost)`.
    // We mirror that exactly and then track the row minimum.
    let max_edits_usize = max_edits as usize;
    let mut prev: Vec<u32> = (0..=u32::try_from(m).unwrap_or(u32::MAX)).collect();
    let mut curr: Vec<u32> = vec![0u32; m + 1];
    for (i, &ca) in a_chars.iter().enumerate() {
        let i_plus_one = u32::try_from(i + 1).unwrap_or(u32::MAX);
        curr[0] = i_plus_one;
        let mut row_min = i_plus_one;
        for (j, &cb) in b_chars.iter().enumerate() {
            let cost: u32 = if ca == cb { 0 } else { 1 };
            let ins = curr[j].saturating_add(1);
            let del = prev[j + 1].saturating_add(1);
            let sub = prev[j].saturating_add(cost);
            let v = ins.min(del).min(sub);
            curr[j + 1] = v;
            if v < row_min {
                row_min = v;
            }
        }
        // Early cutoff: every cell in this row exceeds max_edits,
        // so no descendant can be <= max_edits.
        if row_min > max_edits {
            return None;
        }
        std::mem::swap(&mut prev, &mut curr);
        // `curr` now holds last row's values from the swap; it'll be
        // overwritten in the next iteration starting at index 0.
        // Counting on the loop body to fully overwrite curr[0..=m] in
        // the next iteration; the leftover values don't leak because
        // curr[0] is written first and curr[j+1] is written in order.
        // For safety reset the leftover sentinels — micro-optimization
        // skipped here for readability.
        let _ = max_edits_usize; // suppress unused if cutoff removed later
    }
    // After the swap, the final row's values are in `prev`.
    let d = prev[m];
    if d <= max_edits {
        Some(d)
    } else {
        None
    }
}

/// Default Levenshtein budget for `fuzzy_match`. Mirrors Python
/// `legacy_python_oracle/src/cacg/verify/fuzzy.py::DEFAULT_MAX_DIST`.
pub const DEFAULT_MAX_DIST: u32 = 2;

/// Default Ratcliff-Obershelp ratio floor for `fuzzy_match`. Mirrors
/// Python `legacy_python_oracle/src/cacg/verify/fuzzy.py::DEFAULT_MIN_RATIO`.
pub const DEFAULT_MIN_RATIO: f64 = 0.95;

/// Dual-metric fuzzy substring matcher.
///
/// Returns `true` iff some sliding window of `chunk_text` (sized
/// between `q - max_dist` and `q + max_dist`, where `q` is the
/// code-point length of `quote`) passes BOTH the Ratcliff-Obershelp
/// ratio gate (`ratcliff_obershelp_ratio(quote, candidate) >= min_ratio`)
/// AND the bounded Levenshtein gate
/// (`bounded_levenshtein(quote, candidate, max_dist).is_some()`).
///
/// Mirrors Python `legacy_python_oracle/src/cacg/verify/fuzzy.py::fuzzy_match`:
///
/// * Empty `quote` → `true` (Python's `if not quote: return True`).
/// * `min_window > len_chars(chunk_text)` → `false` (chunk too small
///   even after allowing `max_dist` deletions).
/// * Otherwise sweeps `window` from `max(1, q - max_dist)` to
///   `min(len_chars(chunk_text), q + max_dist)`, inclusive; for each
///   window size, slides `start` over the chunk by code-point offsets.
///
/// The ratio gate is checked first as a cheap pre-filter; the
/// bounded Levenshtein DP only fires when the ratio passes. First
/// match short-circuits the search.
///
/// Operates on Unicode code points: a combining mark counts as a
/// separate code point and therefore a separate match position.
#[must_use]
pub fn fuzzy_match(quote: &str, chunk_text: &str, max_dist: u32, min_ratio: f64) -> bool {
    if quote.is_empty() {
        return true;
    }
    let quote_chars: Vec<char> = quote.chars().collect();
    let chunk_chars: Vec<char> = chunk_text.chars().collect();
    let q = quote_chars.len();
    let t = chunk_chars.len();
    let max_dist_usize = max_dist as usize;
    let min_window = q.saturating_sub(max_dist_usize).max(1);
    let max_window = t.min(q + max_dist_usize);
    if min_window > t {
        return false;
    }
    for window in min_window..=max_window {
        // start positions: 0..=t-window inclusive
        for start in 0..=(t - window) {
            let candidate_chars = &chunk_chars[start..start + window];
            let candidate: String = candidate_chars.iter().collect();
            if ratcliff_obershelp_ratio(quote, &candidate) < min_ratio {
                continue;
            }
            if bounded_levenshtein(quote, &candidate, max_dist).is_some() {
                return true;
            }
        }
    }
    false
}

/// Computes the Ratcliff-Obershelp ratio between two strings, matching
/// Python `difflib.SequenceMatcher(None, a, b).ratio()` byte-for-byte
/// within `f64` precision (cross-validated at 1e-9 in
/// `tests/ratcliff_obershelp_parity.rs`).
///
/// Returns a value in `[0.0, 1.0]`. Identical strings return `1.0`;
/// empty-and-empty also returns `1.0` (matches Python's
/// `_calculate_ratio(matches=0, length=0)` short-circuit). Disjoint
/// strings return `0.0`.
///
/// The autojunk heuristic is always on (matches Python's default).
#[must_use]
pub fn ratcliff_obershelp_ratio(a: &str, b: &str) -> f64 {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let total = a_chars.len() + b_chars.len();
    if total == 0 {
        return 1.0;
    }
    let matcher = SequenceMatcher::new(&a_chars, &b_chars);
    let matches = matcher.matches_total();
    (2.0 * matches as f64) / (total as f64)
}

struct SequenceMatcher<'a> {
    a: &'a [char],
    b: &'a [char],
    b2j: BTreeMap<char, Vec<usize>>,
    bjunk: BTreeSet<char>,
}

impl<'a> SequenceMatcher<'a> {
    fn new(a: &'a [char], b: &'a [char]) -> Self {
        // Step 1: build `b2j`, the position-list map keyed by element.
        let mut b2j: BTreeMap<char, Vec<usize>> = BTreeMap::new();
        for (i, &elt) in b.iter().enumerate() {
            b2j.entry(elt).or_default().push(i);
        }

        // Step 2: junk filter (no `isjunk` callback in our caller, so
        // `bjunk` stays empty — Python keeps the conceptual two-stage
        // structure, and so do we).
        let bjunk: BTreeSet<char> = BTreeSet::new();

        // Step 3: autojunk filter. When `len(b) >= 200` AND
        // `autojunk=True`, any element with count > `n // 100 + 1` is
        // popular and excluded from b2j. Popular elements are still
        // walked by the extend-by-junk passes inside
        // `find_longest_match`.
        let n = b.len();
        if n >= 200 {
            let ntest = n / 100 + 1;
            let popular: Vec<char> = b2j
                .iter()
                .filter_map(|(&elt, positions)| {
                    if positions.len() > ntest {
                        Some(elt)
                    } else {
                        None
                    }
                })
                .collect();
            for elt in popular {
                b2j.remove(&elt);
            }
        }

        Self { a, b, b2j, bjunk }
    }

    /// Find the longest contiguous matching subsequence of
    /// `a[alo..ahi]` and `b[blo..bhi]`. Returns `(i, j, k)` where
    /// `a[i..i+k] == b[j..j+k]` and `k` is maximal. Mirrors CPython's
    /// `SequenceMatcher.find_longest_match` semantics including the
    /// extend-by-non-junk + extend-by-junk passes.
    fn find_longest_match(
        &self,
        alo: usize,
        ahi: usize,
        blo: usize,
        bhi: usize,
    ) -> (usize, usize, usize) {
        let (mut besti, mut bestj, mut bestsize) = (alo, blo, 0usize);
        // j2len: for each j in b, how long the match ending at b[j-1]
        // currently is. Re-built per a-iteration.
        let mut j2len: HashMap<usize, usize> = HashMap::new();
        let empty: Vec<usize> = Vec::new();

        for i in alo..ahi {
            let positions = self.b2j.get(&self.a[i]).unwrap_or(&empty);
            let mut newj2len: HashMap<usize, usize> = HashMap::new();
            for &j in positions {
                if j < blo {
                    continue;
                }
                if j >= bhi {
                    break;
                }
                // j-1 underflow guard: when j == 0, the prior length is 0.
                let prior = if j == 0 {
                    0
                } else {
                    j2len.get(&(j - 1)).copied().unwrap_or(0)
                };
                let k = prior + 1;
                newj2len.insert(j, k);
                if k > bestsize {
                    besti = i + 1 - k;
                    bestj = j + 1 - k;
                    bestsize = k;
                }
            }
            j2len = newj2len;
        }

        // Extend by non-junk on each end. "Popular" elements were
        // removed from b2j and thus were skipped in the main loop;
        // walking them here re-attaches the popular characters at
        // the match boundaries.
        while besti > alo
            && bestj > blo
            && !self.bjunk.contains(&self.b[bestj - 1])
            && self.a[besti - 1] == self.b[bestj - 1]
        {
            besti -= 1;
            bestj -= 1;
            bestsize += 1;
        }
        while besti + bestsize < ahi
            && bestj + bestsize < bhi
            && !self.bjunk.contains(&self.b[bestj + bestsize])
            && self.a[besti + bestsize] == self.b[bestj + bestsize]
        {
            bestsize += 1;
        }

        // Extend by junk on each end (only fires when `isjunk`
        // callback was provided; for us `bjunk` is empty so both
        // loops are no-ops). Kept for structural parity with CPython.
        while besti > alo
            && bestj > blo
            && self.bjunk.contains(&self.b[bestj - 1])
            && self.a[besti - 1] == self.b[bestj - 1]
        {
            besti -= 1;
            bestj -= 1;
            bestsize += 1;
        }
        while besti + bestsize < ahi
            && bestj + bestsize < bhi
            && self.bjunk.contains(&self.b[bestj + bestsize])
            && self.a[besti + bestsize] == self.b[bestj + bestsize]
        {
            bestsize += 1;
        }

        (besti, bestj, bestsize)
    }

    /// Total length of matching substrings across the recursive
    /// gestalt decomposition. Mirrors the `matches` sum CPython's
    /// `ratio()` computes from `get_matching_blocks`. Implemented
    /// iteratively (explicit stack) to avoid recursion-depth issues
    /// on degenerate inputs.
    fn matches_total(&self) -> usize {
        let mut total = 0usize;
        let mut stack: Vec<(usize, usize, usize, usize)> = vec![(0, self.a.len(), 0, self.b.len())];
        while let Some((alo, ahi, blo, bhi)) = stack.pop() {
            if alo >= ahi || blo >= bhi {
                continue;
            }
            let (i, j, k) = self.find_longest_match(alo, ahi, blo, bhi);
            if k == 0 {
                continue;
            }
            total += k;
            // Recurse on the prefix (alo..i, blo..j) and the suffix
            // (i+k..ahi, j+k..bhi). Python's `__helper` does this
            // recursively and appends the match between; we don't
            // need the matches list itself, just the sum, so order
            // doesn't matter.
            if alo < i && blo < j {
                stack.push((alo, i, blo, j));
            }
            if i + k < ahi && j + k < bhi {
                stack.push((i + k, ahi, j + k, bhi));
            }
        }
        total
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn identity_returns_one() {
        assert_eq!(ratcliff_obershelp_ratio("foo", "foo"), 1.0);
        assert_eq!(ratcliff_obershelp_ratio("a", "a"), 1.0);
        assert_eq!(
            ratcliff_obershelp_ratio("longer string with words", "longer string with words"),
            1.0
        );
    }

    #[test]
    fn empty_and_empty_returns_one() {
        // Matches Python's _calculate_ratio(0, 0) -> 1.0 short-circuit.
        assert_eq!(ratcliff_obershelp_ratio("", ""), 1.0);
    }

    #[test]
    fn empty_versus_nonempty_returns_zero() {
        assert_eq!(ratcliff_obershelp_ratio("", "x"), 0.0);
        assert_eq!(ratcliff_obershelp_ratio("x", ""), 0.0);
        assert_eq!(ratcliff_obershelp_ratio("", "hello"), 0.0);
    }

    #[test]
    fn fully_disjoint_returns_zero() {
        // No characters in common.
        assert_eq!(ratcliff_obershelp_ratio("abc", "xyz"), 0.0);
        assert_eq!(ratcliff_obershelp_ratio("abcdef", "ghijkl"), 0.0);
    }

    #[test]
    fn python_docs_example_abcd_abxd() {
        // From CPython difflib docs:
        // SequenceMatcher(None, "abcd", "abxd").ratio() == 0.75
        // matches = 3 (a, b, d), total len = 8 → 6/8 = 0.75.
        let r = ratcliff_obershelp_ratio("abcd", "abxd");
        assert!(approx_eq(r, 0.75, 1e-9), "expected 0.75, got {r}");
    }

    #[test]
    fn one_character_insertion() {
        // "abc" → "abXc": 3 matches, total 7 → 6/7
        let r = ratcliff_obershelp_ratio("abc", "abXc");
        assert!(approx_eq(r, 6.0 / 7.0, 1e-9), "got {r}");
    }

    #[test]
    fn unicode_combining_marks() {
        // The string "café" can be NFC-encoded ("café") or NFD-
        // decomposed ("cafe\u{0301}"). The matcher operates on Unicode
        // code points so NFC == NFC → 1.0 and NFC vs NFD has a
        // partial match (3 chars in common).
        let nfc = "café";
        let nfd = "cafe\u{0301}";
        assert_eq!(ratcliff_obershelp_ratio(nfc, nfc), 1.0);
        let mixed = ratcliff_obershelp_ratio(nfc, nfd);
        // nfc has 4 chars (c, a, f, é). nfd has 5 chars (c, a, f, e,
        // combining-acute). Common prefix is c-a-f (3 chars). Total
        // 9. Ratio 6/9 = 0.666...
        assert!(approx_eq(mixed, 6.0 / 9.0, 1e-9), "got {mixed}");
    }

    #[test]
    fn cjk_full_match() {
        let s = "床前明月光疑是地上霜";
        assert_eq!(ratcliff_obershelp_ratio(s, s), 1.0);
    }

    #[test]
    fn autojunk_trigger_threshold() {
        // Build a 200-char string where 'a' appears 100 times: this
        // exceeds the ntest = 200/100 + 1 = 3 threshold by a wide
        // margin, so 'a' becomes "popular" and is excluded from b2j.
        // The match still works correctly because the extend-by-non-
        // junk passes re-attach popular characters at match
        // boundaries.
        let mut s = String::new();
        for _ in 0..100 {
            s.push('a');
            s.push('b');
        }
        assert_eq!(s.chars().count(), 200);
        // Self-similarity must be 1.0 even under autojunk.
        assert_eq!(ratcliff_obershelp_ratio(&s, &s), 1.0);
    }

    #[test]
    fn returns_value_in_unit_interval() {
        // Sanity: every output must lie in [0.0, 1.0].
        let cases: &[(&str, &str)] = &[
            ("", ""),
            ("a", ""),
            ("", "a"),
            ("hello", "world"),
            ("abcdefghij", "abcdEfgHij"),
            ("床前明月光", "床前明月"),
        ];
        for (a, b) in cases {
            let r = ratcliff_obershelp_ratio(a, b);
            assert!(
                (0.0..=1.0).contains(&r),
                "ratio({a:?}, {b:?}) = {r} not in [0,1]"
            );
        }
    }

    #[test]
    fn bounded_levenshtein_identity_returns_zero() {
        assert_eq!(bounded_levenshtein("", "", 0), Some(0));
        assert_eq!(bounded_levenshtein("foo", "foo", 0), Some(0));
        assert_eq!(bounded_levenshtein("foo", "foo", 2), Some(0));
        assert_eq!(bounded_levenshtein("床前明月光", "床前明月光", 2), Some(0));
    }

    #[test]
    fn bounded_levenshtein_single_substitution_returns_one() {
        assert_eq!(bounded_levenshtein("foo", "boo", 2), Some(1));
        assert_eq!(bounded_levenshtein("foo", "boo", 1), Some(1));
        assert_eq!(bounded_levenshtein("foo", "boo", 0), None);
        assert_eq!(bounded_levenshtein("abcd", "abxd", 2), Some(1));
    }

    #[test]
    fn bounded_levenshtein_single_insertion_returns_one() {
        // a has 3 chars, b has 4 (one inserted).
        assert_eq!(bounded_levenshtein("abc", "abXc", 2), Some(1));
        assert_eq!(bounded_levenshtein("abc", "abcX", 2), Some(1));
        assert_eq!(bounded_levenshtein("abc", "Xabc", 2), Some(1));
    }

    #[test]
    fn bounded_levenshtein_single_deletion_returns_one() {
        assert_eq!(bounded_levenshtein("abXc", "abc", 2), Some(1));
        assert_eq!(bounded_levenshtein("abcX", "abc", 2), Some(1));
        assert_eq!(bounded_levenshtein("Xabc", "abc", 2), Some(1));
    }

    #[test]
    fn bounded_levenshtein_over_budget_returns_none() {
        // "kitten" -> "sitting": 3 edits (k->s, e->i, +g). Budget 2 < 3.
        assert_eq!(bounded_levenshtein("kitten", "sitting", 2), None);
        // Budget 3 accepts.
        assert_eq!(bounded_levenshtein("kitten", "sitting", 3), Some(3));
        // Budget 5 also accepts (still returns the true distance).
        assert_eq!(bounded_levenshtein("kitten", "sitting", 5), Some(3));
    }

    #[test]
    fn bounded_levenshtein_empty_strings_return_correct_distance() {
        assert_eq!(bounded_levenshtein("", "", 0), Some(0));
        assert_eq!(bounded_levenshtein("", "abc", 3), Some(3));
        assert_eq!(bounded_levenshtein("", "abc", 2), None);
        assert_eq!(bounded_levenshtein("abc", "", 3), Some(3));
        assert_eq!(bounded_levenshtein("abc", "", 2), None);
    }

    #[test]
    fn bounded_levenshtein_length_difference_short_circuit_returns_none() {
        // 5 vs 3 -> |5-3| = 2; budget 1 too tight -> None even though
        // some 2-edit path exists.
        assert_eq!(bounded_levenshtein("aaaaa", "bbb", 1), None);
        // budget 2 admits a length diff of 2.
        let d = bounded_levenshtein("aaaaa", "bbb", 5).unwrap();
        assert!(d >= 2);
    }

    #[test]
    fn bounded_levenshtein_unicode_combining_marks_count_per_codepoint() {
        // NFC "café" (4 code points: c, a, f, é) vs NFD
        // "cafe\u{0301}" (5 code points: c, a, f, e, combining-acute).
        // Distance is 2: substitute é -> e + insert combining-acute,
        // OR delete é + insert e + insert combining-acute (which is
        // 3). Wagner-Fischer picks the minimum, so the answer is 2.
        let d = bounded_levenshtein("café", "cafe\u{0301}", 5).unwrap();
        assert_eq!(d, 2);
        // Budget 2 admits, budget 1 rejects.
        assert_eq!(bounded_levenshtein("café", "cafe\u{0301}", 2), Some(2));
        assert_eq!(bounded_levenshtein("café", "cafe\u{0301}", 1), None);
    }

    #[test]
    fn bounded_levenshtein_cjk_substitution_counts_one() {
        // CJK characters are single code points; substituting one
        // counts as one edit (NOT one per UTF-8 byte).
        assert_eq!(bounded_levenshtein("床前明月光", "床前明月X", 2), Some(1));
        assert_eq!(bounded_levenshtein("床前明月光", "床前明月X", 0), None);
    }

    #[test]
    fn bounded_levenshtein_max_edits_zero_only_accepts_identity() {
        assert_eq!(bounded_levenshtein("foo", "foo", 0), Some(0));
        assert_eq!(bounded_levenshtein("foo", "fox", 0), None);
        assert_eq!(bounded_levenshtein("", "", 0), Some(0));
        assert_eq!(bounded_levenshtein("", "x", 0), None);
    }

    #[test]
    fn bounded_levenshtein_returns_actual_distance_when_within_budget() {
        // The fuzzy_match consumer only needs the truth value, but the
        // bounded_levenshtein return shape preserves the actual edit
        // count when within budget. Pin that contract.
        assert_eq!(bounded_levenshtein("abc", "xyz", 5), Some(3));
        assert_eq!(bounded_levenshtein("hello", "halloo", 5), Some(2));
    }

    #[test]
    fn symmetry_a_to_b_equals_b_to_a() {
        // Ratcliff-Obershelp ratio is NOT in general symmetric in
        // Python (because find_longest_match's tie-break depends on
        // which string is "a" vs "b"). On most simple inputs it IS
        // symmetric. Verify symmetry for the canonical example.
        let r1 = ratcliff_obershelp_ratio("abcd", "abxd");
        let r2 = ratcliff_obershelp_ratio("abxd", "abcd");
        assert!(approx_eq(r1, r2, 1e-9), "{r1} vs {r2}");
    }

    #[test]
    fn fuzzy_match_empty_quote_returns_true() {
        assert!(fuzzy_match("", "", DEFAULT_MAX_DIST, DEFAULT_MIN_RATIO));
        assert!(fuzzy_match(
            "",
            "anything",
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_identity_returns_true() {
        assert!(fuzzy_match(
            "hello world",
            "hello world",
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_single_substitution_returns_true() {
        // Single substitution: ratio between "hello" and "hallo" is
        // 4/5 = 0.8, well above 0.95? No — only 8/10 = 0.8. So this
        // single-substitution case needs a longer string so the
        // single-edit cost is amortized. "hello world" vs "hallo world"
        // is 1 substitution in 11 chars → ratio = 20/22 ≈ 0.909 < 0.95
        // → would fail. So we use a single-edit in a 30-char chunk:
        let quote = "the quick brown fox jumps over";
        let chunk = "the quick brown fox jumps ovor";
        assert!(fuzzy_match(
            quote,
            chunk,
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_single_insertion_returns_true() {
        // One inserted char in a long quote, with chunk containing
        // the inserted form somewhere inside a larger envelope.
        let quote = "the quick brown fox jumps over";
        let chunk = "prefix the quick brown fox jumps overr suffix";
        assert!(fuzzy_match(
            quote,
            chunk,
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_single_deletion_returns_true() {
        let quote = "the quick brown fox jumps over";
        let chunk = "prefix the quick brown fox jumps ove suffix";
        assert!(fuzzy_match(
            quote,
            chunk,
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_unicode_substitution_returns_true() {
        // CJK single substitution within a long quote.
        let quote = "床前明月光疑是地上霜举头望明月";
        let chunk = "床前明月光疑是地上霜举头望明日";
        assert!(fuzzy_match(
            quote,
            chunk,
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_ratio_fail_returns_false() {
        // Short strings where two substitutions land within the
        // bounded-Levenshtein budget but the ratio gate is the
        // decisive failure. `"ab"` vs `"cd"`: bounded_levenshtein at
        // max_dist=2 returns Some(2) (within budget), but the ratio
        // is 0 (no matching characters) → ratio gate fails.
        let quote = "ab";
        let chunk = "cd";
        // Pin the preconditions: this MUST be a ratio-fail case,
        // not a Lev-fail case in disguise. If a future change to
        // either primitive shifts this case's category, the test
        // surfaces here at the underlying-primitive level.
        let ratio = ratcliff_obershelp_ratio(quote, chunk);
        assert!(
            ratio < DEFAULT_MIN_RATIO,
            "expected ratio < {DEFAULT_MIN_RATIO}; got {ratio:.4} — this case is no longer a ratio-fail"
        );
        assert!(
            bounded_levenshtein(quote, chunk, DEFAULT_MAX_DIST).is_some(),
            "expected bounded_levenshtein to be within budget for the ratio-fail pin"
        );
        assert!(!fuzzy_match(
            quote,
            chunk,
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_levenshtein_fail_returns_false() {
        // Three well-spaced single-character substitutions in a
        // 79-character string. The chunk has the same length as the
        // quote (so the whole-chunk window is itself a candidate),
        // and the substitutions are placed so that the
        // Ratcliff-Obershelp ratio stays above the 0.95 gate (the
        // unsubstituted runs are long enough that matches/total
        // is roughly 76/79 ≈ 0.962), while the true Levenshtein
        // distance is 3 (over the max_dist=2 budget).
        //
        // Externally pre-verified against Python:
        //   ratio(q, chunk)        = 0.9620
        //   levenshtein(q, chunk)  = 3
        //   fuzzy_match(q, chunk)  = False
        //   no other candidate window in [q - 2, q + 2] passes both
        //   gates (so the False is genuinely Levenshtein-driven).
        let quote =
            "the quick brown fox jumps over the lazy dog and runs fast across the wide field";
        let chunk =
            "the qXick brown fox jumps over the lazy dog and runs fXst across the wXde field";
        // Pin the preconditions: this MUST be a Lev-fail case, not
        // a ratio-fail case in disguise. A prior version of this
        // test used `"aaaaaaaaaabbbbb"` vs `"aaaaaaaaaaccccc"`,
        // where the ratio gate also failed (ratio ≈ 0.667), so the
        // Levenshtein branch was never the decisive failure — the
        // explicit ratio precondition below catches that class of
        // regression.
        let ratio = ratcliff_obershelp_ratio(quote, chunk);
        assert!(
            ratio >= DEFAULT_MIN_RATIO,
            "expected ratio >= {DEFAULT_MIN_RATIO}; got {ratio:.4} — this case is no longer a Lev-fail"
        );
        assert!(
            bounded_levenshtein(quote, chunk, DEFAULT_MAX_DIST).is_none(),
            "expected bounded_levenshtein to exceed max_dist=2 for the Lev-fail pin"
        );
        assert!(!fuzzy_match(
            quote,
            chunk,
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_empty_chunk_text_returns_false_for_non_empty_quote() {
        assert!(!fuzzy_match(
            "hello",
            "",
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_chunk_smaller_than_window_returns_false() {
        // quote length 10, chunk length 5; min_window = max(1, 10-2)
        // = 8 > chunk length 5, so the bail fires.
        assert!(!fuzzy_match(
            "abcdefghij",
            "abcde",
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_matches_when_quote_is_substring_of_chunk() {
        // The common case: an exact (or near-exact) substring of a
        // larger envelope. Sliding-window sweep must find it.
        let quote = "the quick brown fox";
        let chunk = "preface: the quick brown fox jumps over the lazy dog. END";
        assert!(fuzzy_match(
            quote,
            chunk,
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }

    #[test]
    fn fuzzy_match_unicode_substring_match() {
        // Code-point slicing: the chunk has multi-byte characters
        // before the matching span; byte-offset slicing would
        // misalign. Code-point offsets must hit the span exactly.
        let quote = "明月光疑是地";
        let chunk = "床前明月光疑是地上霜";
        assert!(fuzzy_match(
            quote,
            chunk,
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO
        ));
    }
}
