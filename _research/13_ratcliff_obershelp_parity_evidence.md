# Ratcliff-Obershelp Ratio Parity Evidence (task-vh-9)

## Purpose

Document that `cacg-core::verify::fuzzy::ratcliff_obershelp_ratio`
matches Python `difflib.SequenceMatcher(None, a, b).ratio()` byte-for-
byte within `1e-9` on a representative cross-validated case set. This
is the first piece of AC-3 (Layer-2 verify oracle); task-vh-10
(bounded Levenshtein) and task-vh-11 (dual-metric `fuzzy_match`)
build on it.

## Algorithm

Ratcliff-Obershelp is a gestalt-pattern-matching algorithm: find the
longest contiguous matching substring between `a` and `b`, then
recurse on the prefix `(a[0..i], b[0..j])` and the suffix
`(a[i+k..], b[j+k..])`. The total match length is `Σ k_i`. The
ratio is:

```
ratio = 2.0 * matches / (len(a) + len(b))
```

with the convention `ratio("", "") == 1.0` (matches Python's
`_calculate_ratio(0, 0)` short-circuit).

### `autojunk=True` heuristic

Python's `SequenceMatcher` defaults to `autojunk=True`. When
`len(b) >= 200`, any element appearing in `b` more than `n // 100 + 1`
times is classified as "popular" and excluded from the position map
(`b2j`) consulted by `find_longest_match`. This prevents pathological
performance on long inputs dominated by a single common character
(whitespace, punctuation).

Popular elements are not discarded entirely: the extend-by-non-junk
passes inside `find_longest_match` re-attach them to existing match
boundaries. They simply can't *seed* a match. The Rust port preserves
this two-stage extend behavior (see `find_longest_match` in
`crates/cacg-core/src/verify/fuzzy.rs`).

### Unicode discipline

The Rust port operates on `Vec<char>` (Unicode code points), NOT raw
bytes, mirroring Python `str` slicing semantics on
`difflib.SequenceMatcher`. This matches the AC-3 character-index
discipline the trust-kernel plan calls out for the verify hot path.

## Cross-validation methodology

### Oracle generator

`scripts/build_ratcliff_obershelp_oracle.py` produces
`tests/parity_corpus/ratcliff_obershelp/oracle.json` deterministically:

- `random.seed(0)` at the top of the script.
- Curated case list (identity, disjoint, single-character difference,
  empty / non-empty, prefix-only / suffix-only matches, repeating-
  character autojunk-relevant cases, palindromes, NFC vs NFD
  combining marks, CJK, whitespace-heavy, newlines, long-but-similar).
- 30 random short ASCII (lengths 0..40, alphabet `[a-z0-9]`).
- 10 random long ASCII (lengths 200..400, biased toward `'e'` so
  autojunk fires).
- 20 random Unicode (mix of ASCII, Latin combining, CJK, Greek).

Total: **86 cases** in the committed fixture.

For each case `(a, b)`, the script computes
`difflib.SequenceMatcher(None, a, b).ratio()` and writes
`{"index", "a", "b", "python_ratio"}` to the JSON entries list. Re-
running the script with a clean checkout produces a byte-identical
JSON file (verified by `sha256sum` before and after re-run).

### Rust parity test

`crates/cacg-core/tests/ratcliff_obershelp_parity.rs` loads the JSON,
computes `ratcliff_obershelp_ratio(&a, &b)` per case, and asserts
`|rust - python| < 1e-9`. The test collects all failures (not bail-on-
first) so a regression surfaces the full list of divergent cases at
once.

## Coverage map

| Category | Case count | Notes |
|---|---:|---|
| Curated (handpicked) | 26 | Identity, disjoint, Python docs example, Unicode NFC/NFD, CJK, palindromes, whitespace, newlines. |
| Random short ASCII | 30 | Lengths 0..40, alphabet `[a-z0-9]`. |
| Random long ASCII (autojunk) | 10 | Lengths 200..400, biased toward `'e'` so autojunk's popular-element filter fires. |
| Random Unicode | 20 | Mix of ASCII, Latin combining marks, CJK, Greek. |
| **Total** | **86** | |

Every case passes within 1e-9 of Python (the Rust test reports zero
failures over the full set).

## Sample numerical results

The following ten cases are representative; the full set lives in
the committed `oracle.json`.

| `a` | `b` | Python ratio |
|---|---|---:|
| `""` | `""` | 1.0 |
| `"abc"` | `"xyz"` | 0.0 |
| `"abcd"` | `"abxd"` | 0.75 |
| `"abc"` | `"abXc"` | 0.857142857… |
| `"hello"` | `"hallo"` | 0.8 |
| `"aaaaa"` | `"aaaa"` | 0.888888888… |
| `"ababab"` | `"bababa"` | 0.833333333… |
| `"café"` (NFC) | `"café"` (NFD) | 0.666666666… |
| `"床前明月光疑是地上霜"` | `"床前明月光"` | 0.666666666… |
| `"a"*50 + "X" + "a"*50` | `"a"*50 + "Y" + "a"*50` | 0.990099009… |

(See `oracle.json` for the full numeric tail of each value.)

## Follow-on work

- **task-vh-10** (bounded Levenshtein): Wagner-Fischer DP with early
  cutoff, also cross-validated against Python.
- **task-vh-11** (dual-metric `fuzzy_match`): combines the Ratcliff-
  Obershelp ratio (gated at ≥ 0.95) AND bounded Levenshtein (gated at
  ≤ 2 edits) over a variable-size sliding window. Both gates must
  hold for a fuzzy match. The implementation lives at
  `cacg-core::verify::fuzzy::fuzzy_match` once task-vh-10 lands.
- **task-vh-12** (layer-2 `verify_card`): consumes
  `fuzzy_match` on the `--fuzzy` codepath and the exact-substring
  oracle otherwise.

## Pointers

- Implementation: `crates/cacg-core/src/verify/fuzzy.rs`.
- Inline unit tests (11): `crates/cacg-core/src/verify/fuzzy.rs::tests`.
- Parity test (2): `crates/cacg-core/tests/ratcliff_obershelp_parity.rs`.
- Oracle fixture: `tests/parity_corpus/ratcliff_obershelp/oracle.json`.
- Oracle generator: `scripts/build_ratcliff_obershelp_oracle.py`.
- Python reference: CPython `Lib/difflib.py::SequenceMatcher.ratio()`
  with `autojunk=True`.
