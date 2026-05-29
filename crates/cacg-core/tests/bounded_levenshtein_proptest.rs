#![allow(clippy::unwrap_used)]
//! Seeded randomized cross-validation gate for
//! `cacg_core::verify::fuzzy::bounded_levenshtein`.
//!
//! Generates 10,000 string-pair + threshold cases from a fixed seed,
//! computes each pair's edit distance via a test-local independent
//! Wagner-Fischer reference (`reference_levenshtein`), and per case
//! asserts:
//!
//!   - `bounded_levenshtein(a, b, t).is_some() == (reference_distance <= t)`.
//!   - When `Some(d)`, `d == reference_distance`.
//!
//! On the ASCII-only subset (both strings drawn from
//! `[a-zA-Z0-9]`), additionally asserts that the `edit-distance`
//! 2.2.2 reference crate agrees with the reference on the distance.
//!
//! `edit-distance` is a dev-only dependency declared in
//! `[dev-dependencies]`, so the trust-kernel audit closure
//! (`xtask audit-cacg-core-deps`) is unaffected.

use cacg_core::verify::fuzzy::bounded_levenshtein;
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

const FIXED_SEED: [u8; 32] = [
    0x0B, 0x0E, 0xD0, 0x00, 0x0B, 0x0E, 0xD0, 0x00, // "bounded" marker x2
    0x1E, 0xEE, 0x1E, 0xEE, 0x1E, 0xEE, 0x1E, 0xEE, // distinct from normalize_proptest seed
    0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x20, 0x06, 0x05, 0x21, 0x22, 0x04, 0x17,
    0x00, // 2026-05-21 22-04-17 nibble probe
];
const CASES: usize = 10_000;

/// Independent Wagner-Fischer Levenshtein distance (unbounded).
/// Operates on Unicode scalar values (chars), not bytes. Substitution,
/// insertion, and deletion each cost 1. This is the test-local reference
/// oracle — it must NOT delegate to `cacg_core::verify::fuzzy::bounded_levenshtein`.
fn reference_levenshtein(a: &str, b: &str) -> u32 {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();
    #[allow(clippy::cast_possible_truncation)]
    let mut prev = (0..=n as u32).collect::<Vec<_>>();
    let mut curr = vec![0u32; n + 1];
    for i in 1..=m {
        #[allow(clippy::cast_possible_truncation)]
        {
            curr[0] = i as u32;
        }
        for j in 1..=n {
            let cost = u32::from(a_chars[i - 1] != b_chars[j - 1]);
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

/// One generated case: two strings + a small threshold in `0..=8`.
#[derive(Debug, Clone)]
struct Case {
    a: String,
    b: String,
    threshold: u32,
    a_is_ascii_alnum: bool,
    b_is_ascii_alnum: bool,
}

fn ascii_alnum_char_strategy() -> impl Strategy<Value = char> {
    prop_oneof![
        (b'a'..=b'z').prop_map(|b| b as char),
        (b'A'..=b'Z').prop_map(|b| b as char),
        (b'0'..=b'9').prop_map(|b| b as char),
    ]
}

fn unicode_char_strategy() -> impl Strategy<Value = char> {
    // Mix of ASCII alphanumeric (so a fraction of the "Unicode" branch
    // still happens to be all-ASCII), Latin letters with diacritics,
    // Latin combining marks (NFD probes), CJK, and Greek.
    prop_oneof![
        2 => ascii_alnum_char_strategy(),
        1 => prop::sample::select(vec!['é', 'è', 'ê', 'ë', 'ü', 'ñ', 'ç']),
        1 => prop::sample::select(vec!['\u{0301}', '\u{0300}', '\u{0308}']),
        1 => prop::sample::select(vec!['床', '前', '明', '月', '光', '疑', '是', '地', '上', '霜']),
        1 => prop::sample::select(vec!['α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ']),
    ]
}

fn ascii_string_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(ascii_alnum_char_strategy(), 0..=20)
        .prop_map(|cs| cs.into_iter().collect())
}

fn unicode_string_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(unicode_char_strategy(), 0..=20).prop_map(|cs| cs.into_iter().collect())
}

fn case_strategy() -> impl Strategy<Value = Case> {
    // Two distinct branches so the ASCII-only cross-check against
    // `edit-distance` 2.2.2 exercises a meaningful fraction of the
    // 10,000-case run. Without the split, a per-codepoint ASCII bias
    // is geometrically diluted to ~1% of strings; the ASCII branch
    // here guarantees ~50% of all generated cases qualify.
    let ascii_branch = (ascii_string_strategy(), ascii_string_strategy(), 0u32..=8).prop_map(
        |(a, b, threshold)| Case {
            a,
            b,
            threshold,
            a_is_ascii_alnum: true,
            b_is_ascii_alnum: true,
        },
    );
    let unicode_branch = (
        unicode_string_strategy(),
        unicode_string_strategy(),
        0u32..=8,
    )
        .prop_map(|(a, b, threshold)| {
            let a_is_ascii_alnum = a.chars().all(|c| c.is_ascii_alphanumeric());
            let b_is_ascii_alnum = b.chars().all(|c| c.is_ascii_alphanumeric());
            Case {
                a,
                b,
                threshold,
                a_is_ascii_alnum,
                b_is_ascii_alnum,
            }
        });
    prop_oneof![ascii_branch, unicode_branch]
}

#[test]
fn seeded_proptest_matches_reference_oracle_and_edit_distance_on_ascii_subset() {
    let rng = TestRng::from_seed(RngAlgorithm::ChaCha, &FIXED_SEED);
    let config = Config {
        cases: u32::try_from(CASES).unwrap(),
        ..Config::default()
    };
    let mut runner = TestRunner::new_with_rng(config, rng);

    let strategy = case_strategy();
    let mut cases: Vec<Case> = Vec::with_capacity(CASES);
    for _ in 0..CASES {
        let tree = strategy
            .new_tree(&mut runner)
            .expect("strategy new_tree failed");
        cases.push(tree.current());
    }

    let mut truth_failures: Vec<String> = Vec::new();
    let mut distance_failures: Vec<String> = Vec::new();
    let mut edit_distance_failures: Vec<String> = Vec::new();
    let mut ascii_subset_cases = 0usize;

    for (idx, case) in cases.iter().enumerate() {
        let Case {
            a,
            b,
            threshold,
            a_is_ascii_alnum,
            b_is_ascii_alnum,
        } = case;
        let ref_dist = reference_levenshtein(a, b);
        let rs = bounded_levenshtein(a, b, *threshold);
        let expected_some = ref_dist <= *threshold;
        if rs.is_some() != expected_some {
            if truth_failures.len() < 10 {
                truth_failures.push(format!(
                    "case {idx} threshold={threshold}: rust_is_some={} expected_is_some={} \
                     reference_distance={ref_dist}; a={a:?} b={b:?}",
                    rs.is_some(),
                    expected_some,
                ));
            }
            continue;
        }
        if let Some(d) = rs {
            if d != ref_dist {
                if distance_failures.len() < 10 {
                    distance_failures.push(format!(
                        "case {idx} threshold={threshold}: rust_distance={d} reference_distance={ref_dist}; \
                         a={a:?} b={b:?}",
                    ));
                }
                continue;
            }
        }

        if *a_is_ascii_alnum && *b_is_ascii_alnum {
            ascii_subset_cases += 1;
            let crate_dist = edit_distance::edit_distance(a, b);
            let ref_dist_usize = ref_dist as usize;
            if crate_dist != ref_dist_usize && edit_distance_failures.len() < 10 {
                edit_distance_failures.push(format!(
                    "case {idx}: edit_distance_crate={crate_dist} reference_distance={ref_dist}; \
                     a={a:?} b={b:?}",
                ));
            }
        }
    }

    assert!(
        truth_failures.is_empty(),
        "bounded_levenshtein truth-value diverged from reference on {} case(s) (first 10 shown):\n  {}",
        truth_failures.len(),
        truth_failures.join("\n  "),
    );
    assert!(
        distance_failures.is_empty(),
        "bounded_levenshtein exact-distance diverged from reference on {} case(s) (first 10 shown):\n  {}",
        distance_failures.len(),
        distance_failures.join("\n  "),
    );
    assert!(
        edit_distance_failures.is_empty(),
        "edit-distance crate diverged from reference on {} ASCII-subset case(s) (first 10 shown):\n  {}",
        edit_distance_failures.len(),
        edit_distance_failures.join("\n  "),
    );
    assert!(
        ascii_subset_cases >= 100,
        "ASCII-subset cross-check fired on only {ascii_subset_cases} of {CASES} cases; \
         the strategy weight needs adjustment so the reference gate exercises meaningful cardinality",
    );
}
