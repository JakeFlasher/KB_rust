# M5b Round-Summary Parser Specification (Python → Rust Port)

**Date:** 2026-05-24
**Source oracle:** `src/cacg/integrate/round_summary.py` (328 lines)
**Target port:** `crates/cacg-cli/src/round_summary.rs` (new file)
**Authored under:** RLCR Round 0 of the M5b loop (`.humanize/rlcr/2026-05-24_14-58-59`)
**Plan task:** `task-m5b-1` (analyze tag, routed to Codex via `/humanize:ask-codex`)
**Target acceptance criteria:** AC-3, AC-3.1, AC-3.2 of `.humanize/.humanize/plans/cacg-layer3-semantic-port-plan.md`
**BitLessons applied:**
- `BL-20260518-shape-check-fs-inputs` — §3 (verify_round_summary), §8 (per-batch one-shot loads), §13 documents the exact filesystem-shape expectations for `summary_path` (is_file) and `chunks_manifest_path`, mirroring Python's lenient try/except fallback.
- `BL-20260522-port-pydantic-validators-not-just-fields` — §3, §5, §7, §11 enumerate every semantic gate (lines 171–233 of the Python source) so the Rust port replicates ALL validators rather than just the dataclass field shapes.

**Convergence status:** This document is the **definitive byte-equality oracle reference** for `task-m5b-2` (the actual Rust port). When Rust behaviour disagrees with this spec, the Python source at the cited line range wins — re-derive from `round_summary.py`, then update this spec to reflect the corrected reading.

---

# 1. Module-level constants and regexes

Source anchors:
- [round_summary.py lines 30-34](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:30)
- Module docstring contract: [lines 1-22](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:1)

## 1.1 `NA_SENTINEL`

- Python source:
  - Line 30:
    ```python
    NA_SENTINEL = "N/A -- task not KB-relevant this round"
    ```
- Exact value:
  - `N/A -- task not KB-relevant this round`
- It is not a regex.
- It is compared after `raw.strip()` in `parse_round_summary`.
- Leading whitespace around the line is accepted.
- Trailing whitespace around the line is accepted.
- Any trailing non-whitespace content is rejected as a sentinel.
- Case must match exactly.
- The hyphen sequence must be exactly two ASCII hyphens surrounded by single ASCII spaces.
- Rust equivalent:
  ```rust
  const NA_SENTINEL: &str = "N/A -- task not KB-relevant this round";
  ```
- Rust parity comparison:
  ```rust
  if line.trim() == NA_SENTINEL { ... }
  ```
- Use Unicode-aware `trim()` only if accepting Rust’s Unicode whitespace parity with Python `str.strip()`.
- Python `str.strip()` removes Unicode whitespace, not only ASCII whitespace.

## 1.2 `KB_RELEVANT_REGEX`

- Python source:
  - Line 31:
    ```python
    KB_RELEVANT_REGEX = re.compile(r"(?:cards/|\.claude/knowledge/)")
    ```
- Regex flavour:
  - Python `re`.
  - No explicit flags.
  - Python 3 string regexes are Unicode-aware by default.
  - Not `re.MULTILINE`.
  - Not `re.IGNORECASE`.
  - Not `re.DOTALL`.
- Exact Python pattern:
  ```regex
  (?:cards/|\.claude/knowledge/)
  ```
- Anchoring:
  - Unanchored.
  - Searched anywhere in the full summary text.
- Scope:
  - Entire round-summary markdown text, not only the `## Knowledge Consulted` section.
- Subtle whitespace:
  - None.
- Matches:
  - `cards/`
  - `.claude/knowledge/`
- Does not match:
  - `Cards/`
  - `cards\`
  - `.claude\knowledge\`
  - `card/`
  - `.claude/Knowledge/`
- Rust equivalent with `regex` crate:
  ```rust
  let kb_relevant_re = regex::RegexBuilder::new(r"(?:cards/|\.claude/knowledge/)")
      .unicode(true)
      .multi_line(false)
      .dot_matches_new_line(false)
      .case_insensitive(false)
      .build()?;
  ```
- The default `regex::Regex::new(...)` also works if Unicode defaults remain enabled.
- Do not normalize path separators before applying this regex if byte parity is required.

## 1.3 `SECTION_HEADING_REGEX`

- Python source:
  - Line 32:
    ```python
    SECTION_HEADING_REGEX = re.compile(r"^##\s+Knowledge Consulted\s*$", re.MULTILINE)
    ```
- Regex flavour:
  - Python `re`.
  - Explicit flag: `re.MULTILINE`.
  - Unicode whitespace semantics.
  - Not `re.IGNORECASE`.
  - Not `re.DOTALL`.
- Exact Python pattern:
  ```regex
  ^##\s+Knowledge Consulted\s*$
  ```
- Anchoring:
  - `^` anchors to start of string or just after a newline because `MULTILINE` is enabled.
  - `$` anchors to end of string or just before a newline because `MULTILINE` is enabled.
- Heading spelling:
  - Must be exactly `Knowledge Consulted`.
  - Case-sensitive.
  - Space between `Knowledge` and `Consulted` is literal ASCII space.
- Prefix rules:
  - Must start at column 0 with `##`.
  - Leading spaces before `##` make it fail.
- Whitespace after `##`:
  - `\s+` accepts one or more Python Unicode whitespace characters.
  - This includes ASCII space and tab.
  - It also includes newline in general regex semantics, but because the literal text must follow, practical matches require whitespace before `Knowledge Consulted`.
- Trailing whitespace:
  - `\s*` accepts zero or more Python Unicode whitespace characters before end of line.
  - With CRLF, this can consume `\r` before `$` matches before `\n`.
- Rust equivalent:
  ```rust
  let section_heading_re = regex::RegexBuilder::new(r"^##\s+Knowledge Consulted\s*$")
      .unicode(true)
      .multi_line(true)
      .dot_matches_new_line(false)
      .case_insensitive(false)
      .build()?;
  ```
- Required Rust detail:
  - `multi_line(true)` is required.
  - Without it, only a file beginning with the heading would match.

## 1.4 `NEXT_HEADING_REGEX`

- Python source:
  - Line 33:
    ```python
    NEXT_HEADING_REGEX = re.compile(r"^##\s", re.MULTILINE)
    ```
- Regex flavour:
  - Python `re`.
  - Explicit flag: `re.MULTILINE`.
  - Unicode whitespace semantics.
  - Not `re.IGNORECASE`.
  - Not `re.DOTALL`.
- Exact Python pattern:
  ```regex
  ^##\s
  ```
- Anchoring:
  - `^` anchors to start of string or just after newline.
- Heading boundary:
  - Any line beginning at column 0 with `##` followed by any Unicode whitespace starts the next section.
  - `## Next` matches.
  - `##\tNext` matches.
  - `### Next` does not match because the third character is `#`, not whitespace.
  - `##Next` does not match because there is no whitespace after `##`.
  - `  ## Next` does not match because of leading spaces.
- Rust equivalent:
  ```rust
  let next_heading_re = regex::RegexBuilder::new(r"^##\s")
      .unicode(true)
      .multi_line(true)
      .dot_matches_new_line(false)
      .case_insensitive(false)
      .build()?;
  ```

## 1.5 `BULLET_REGEX`

- Python source:
  - Line 34:
    ```python
    BULLET_REGEX = re.compile(r"^\s*[-*]\s+([^\s].*?)\s*$")
    ```
- Regex flavour:
  - Python `re`.
  - No explicit flags.
  - Unicode whitespace semantics.
  - Not `re.MULTILINE`.
  - Not `re.DOTALL`.
- Exact Python pattern:
  ```regex
  ^\s*[-*]\s+([^\s].*?)\s*$
  ```
- Anchoring:
  - Anchored to the whole input string with `^...$`.
  - It is applied with `BULLET_REGEX.match(line)`.
  - Because the pattern is fully anchored, `match` behaves like full-line matching except for Python `$` end behavior.
- Bullet marker:
  - Accepts `-`.
  - Accepts `*`.
  - Does not accept `+`.
  - Does not accept numbered bullets.
- Leading indentation:
  - `^\s*` accepts leading Unicode whitespace before the bullet marker.
  - Indented bullets are accepted.
- Required whitespace after marker:
  - `\s+` requires at least one Unicode whitespace character after `-` or `*`.
  - `-path` does not match.
- Captured group:
  - `([^\s].*?)`
  - The first captured character must be non-whitespace.
  - The rest is lazy `.*?`, excluding newlines unless the input string itself contains unusual embedded characters and DOTALL is off.
- Trailing trim:
  - Final `\s*$` consumes trailing Unicode whitespace.
- Rust equivalent:
  ```rust
  let bullet_re = regex::RegexBuilder::new(r"^\s*[-*]\s+([^\s].*?)\s*$")
      .unicode(true)
      .multi_line(false)
      .dot_matches_new_line(false)
      .case_insensitive(false)
      .build()?;
  ```
- Important Rust parity details:
  - Keep Unicode mode enabled so `\s` and `[^\s]` behave closest to Python.
  - Apply this regex to one line at a time, mirroring Python `parse_round_summary(text).splitlines()`.
  - Do not enable `multi_line`; it is unnecessary and could hide call-site mistakes.

# 2. Data types

Source anchors:
- [round_summary.py lines 37-65](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:37)
- CLI printing: [cli.py lines 1035-1055](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/cli.py:1035)

## 2.1 `Verdict`

Python source:
```python
class Verdict(str, Enum):
    VERIFIED = "VERIFIED"
    STALE = "STALE"
    MISSING = "MISSING"
```

Semantics:
- String enum.
- Values are uppercase ASCII strings.
- Python enum members compare by identity in this module:
  - `p.verdict is not Verdict.VERIFIED`
  - `verdict.verdict is Verdict.VERIFIED`
- CLI output uses `verdict.verdict.value`.

Rust equivalent:
```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Verdict {
    Verified,
    Stale,
    Missing,
}
```

Canonical JSON note:
- The Python module itself does not serialize these dataclasses to disk.
- If Rust expected fixtures serialize them, the verdict must serialize as:
  - `"VERIFIED"`
  - `"STALE"`
  - `"MISSING"`
- Do not serialize as lowercase or Rust variant names.

## 2.2 `PathVerdict`

Python source:
```python
@dataclass(frozen=True, slots=True)
class PathVerdict:
    path: str
    verdict: Verdict
    detail: str = ""
```

Fields:
- `path: str`
  - The originally cited string, not necessarily the resolved filesystem path.
  - Structural diagnostics use `"(section)"`.
- `verdict: Verdict`
  - One of `VERIFIED`, `STALE`, `MISSING`.
- `detail: str`
  - Empty string on success.
  - Human-facing diagnostic detail for failures.

Rust equivalent:
```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct PathVerdict {
    path: String,
    verdict: Verdict,
    #[serde(default)]
    detail: String,
}
```

Canonical JSON note:
- Preserve field names exactly if serializing:
  - `path`
  - `verdict`
  - `detail`
- Preserve path list order.
- Do not omit `detail` if byte-equal JSON fixtures include it as `""`.
- If fixture schema chooses to omit default empty strings, that is a Rust-side fixture decision, not behavior present in Python.

## 2.3 `RoundSummaryResult`

Python source:
```python
@dataclass(frozen=True, slots=True)
class RoundSummaryResult:
    is_na: bool
    section_missing: bool
    kb_relevant: bool
    paths: list[PathVerdict]
```

Fields:
- `is_na`
  - True only for a clean accepted N/A sentinel branch.
  - False for N/A misuse on KB-relevant work.
  - False for sentinel collision.
- `section_missing`
  - True only when `extract_section(text)` returned `None`.
- `kb_relevant`
  - Result of scanning the entire summary text with `KB_RELEVANT_REGEX`.
- `paths`
  - Verification results or structural section-level `PathVerdict`s.
  - Empty for clean N/A, clean empty non-KB-relevant section, and missing non-KB-relevant section.

Rust equivalent:
```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct RoundSummaryResult {
    is_na: bool,
    section_missing: bool,
    kb_relevant: bool,
    paths: Vec<PathVerdict>,
}
```

Canonical JSON note:
- If serialized, preserve exact snake_case names.
- If canonical JSON is used, sort object keys only if the repo’s canonical serializer requires it.
- The semantic contract depends more on field values and `paths` ordering than object key order.
- Python’s module does not itself define `model_dump`, `to_json`, or canonical serialization.

## 2.4 `RoundSummaryResult.exit_code`

Python source:
```python
@property
def exit_code(self) -> int:
    if self.section_missing and self.kb_relevant:
        return 2
    if self.is_na:
        return 0
    if any(p.verdict is not Verdict.VERIFIED for p in self.paths):
        return 1
    return 0
```

Exact branching ladder:
1. If `section_missing and kb_relevant`: return `2`.
2. Else if `is_na`: return `0`.
3. Else if any path verdict is not `VERIFIED`: return `1`.
4. Else return `0`.

Rust equivalent:
```rust
impl RoundSummaryResult {
    fn exit_code(&self) -> i32 {
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
```

CLI constants:
- [cli.py lines 37-39](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/cli.py:37)
- `EXIT_OK = 0`
- `EXIT_FAIL = 1`
- `EXIT_USAGE = 2`

# 3. Public functions (full contract per function)

## 3.1 `extract_section`

Source anchor:
- [round_summary.py lines 68-77](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:68)

Python signature:
```python
def extract_section(text: str) -> str | None:
```

Rust signature:
```rust
fn extract_section(text: &str, section_re: &Regex, next_heading_re: &Regex) -> Option<String>
```

Rust alternative:
```rust
fn extract_section<'a>(text: &'a str, section_re: &Regex, next_heading_re: &Regex) -> Option<&'a str>
```

Pre-conditions:
- `text` is already decoded as UTF-8 by caller.
- The function trusts `text` as a plain string.
- The function does not validate markdown structure globally.
- The function assumes regexes match Python semantics.

Algorithm:
1. Search the full text for `SECTION_HEADING_REGEX`.
2. If no match:
   - Return `None`.
3. Let `body_start = m.end()`.
4. Let `rest = text[body_start:]`.
5. Search `rest` for `NEXT_HEADING_REGEX`.
6. If a next heading is found:
   - `body = rest[: next_m.start()]`.
7. Else:
   - `body = rest`.
8. Return `body`.

Important details:
- The returned body begins immediately after the matched heading.
- If the heading line ends with `\n`, the body usually begins with that newline.
- The body is not stripped.
- Blank lines are preserved.
- Only the first `## Knowledge Consulted` section is used.
- If a second `## Knowledge Consulted` appears later, it is just the next `## ` heading and terminates the first section body.
- `###` headings inside the section do not terminate the section.
- A next heading must begin at column 0.

Handles:
- Section at beginning of file.
- Section after arbitrary prior content.
- Section ending at next `## ` heading.
- Section ending at EOF.
- CRLF reasonably, because `\s*` in the heading can consume `\r`.

Deliberately does not handle:
- Heading case variants.
- Heading punctuation variants.
- Leading indentation before heading.
- ATX headings without a space after `##`.
- Markdown parsing beyond simple regex search.
- Multiple section merging.

## 3.2 `_extract_path_from_bullet`

Source anchor:
- [round_summary.py lines 80-102](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:80)

Python signature:
```python
def _extract_path_from_bullet(line: str) -> str | None:
```

Rust signature:
```rust
fn extract_path_from_bullet(line: &str, bullet_re: &Regex) -> Option<String>
```

Pre-conditions:
- Input is intended to be a single logical line.
- Caller may pass raw untrimmed line.
- Function validates bullet shape with `BULLET_REGEX`.
- Function does not validate the returned path as a filesystem-safe path.
- Function does not normalize separators.
- Function does not deduplicate.

Algorithm:
1. Match `line` against `BULLET_REGEX`.
2. If no match:
   - Return `None`.
3. Let `raw = m.group(1).strip()`.
4. If `raw.startswith("`")`:
   - Find the next backtick with `raw.find("`", 1)`.
   - If `end > 1`, return `raw[1:end]`.
5. For each separator in this exact order:
   - `" -- "`
   - `" — "`
   - `" - "`
   - `" ("`
6. If the separator occurs in `raw`:
   - Return `raw.split(separator, 1)[0].strip()`.
7. Else:
   - Return `raw.split()[0]`.

Precedence:
- Backtick-quoted path wins over all separator splitting.
- Separator order is fixed.
- First whitespace token fallback happens only after all separators fail.

Handles:
- `- path -- reason`
- `- path - reason`
- `- path`
- ``- `path` -- reason``
- `* path`
- Indented bullets.
- Parenthetical notes after a path if they are introduced by space-parenthesis: `" ("`.

Deliberately does not handle:
- Paths with spaces unless backtick-quoted or before a recognized separator.
- Escaped backticks.
- Nested backticks.
- Markdown links.
- Quoted strings with single or double quotes.
- Parentheses without preceding space.
- Separators without surrounding spaces, such as `a--b`.

## 3.3 `parse_round_summary`

Source anchor:
- [round_summary.py lines 105-122](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:105)

Python signature:
```python
def parse_round_summary(text: str) -> tuple[bool, list[str]]:
```

Rust signature:
```rust
fn parse_round_summary(section_text: &str, bullet_re: &Regex) -> (bool, Vec<String>)
```

Pre-conditions:
- Caller already extracted the section body.
- This function trusts that `text` is the `## Knowledge Consulted` body.
- It does not call `extract_section`.
- It does not inspect full-summary KB relevance.

Algorithm:
1. Initialize `is_na = False`.
2. Initialize `paths = []`.
3. Iterate `for raw in text.splitlines()`.
4. Let `line = raw.strip()`.
5. If `line` is empty:
   - Continue.
6. If `line == NA_SENTINEL`:
   - Set `is_na = True`.
   - Continue.
7. Call `_extract_path_from_bullet(raw)`.
8. If a path string is returned and is truthy:
   - Append it to `paths`.
9. Return `(is_na, paths)`.

Important details:
- Uses `splitlines()`, not `split("\n")`.
- `splitlines()` strips line terminators from `raw`.
- CRLF line endings are generally tolerated.
- Sentinel comparison uses stripped line.
- Bullet parsing receives the original `raw`, not the stripped `line`.
- A sentinel and paths can both be accumulated in one pass.
- Duplicate paths are retained.
- Output path order is input order.

Handles:
- Blank lines.
- Non-bullet prose lines ignored.
- N/A sentinel line anywhere inside the section.
- Multiple N/A sentinel lines.
- Bullets before and after an N/A sentinel.
- `-` and `*` bullets.

Deliberately does not handle:
- Enforcing sentinel exclusivity.
- Verifying paths.
- Resolving paths.
- Deduplicating paths.
- Markdown nested list semantics.
- Code block suppression.
- Comments or HTML blocks.

## 3.4 `is_kb_relevant`

Source anchor:
- [round_summary.py lines 125-126](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:125)

Python signature:
```python
def is_kb_relevant(text: str) -> bool:
```

Rust signature:
```rust
fn is_kb_relevant(text: &str, kb_relevant_re: &Regex) -> bool
```

Pre-conditions:
- Input is full summary text, not only the section.
- Caller has already decoded UTF-8.
- No markdown parsing is performed.

Algorithm:
1. Return `bool(KB_RELEVANT_REGEX.search(text))`.

Handles:
- Any occurrence of `cards/`.
- Any occurrence of `.claude/knowledge/`.
- Occurrences in prose, code blocks, headings, links, bullets, or quoted text.

Deliberately does not handle:
- Backslash path separators.
- Case-insensitive matching.
- Semantic understanding of whether work actually touched KB.
- Avoiding false positives in code blocks.

## 3.5 `verify_round_summary`

Source anchors:
- [round_summary.py lines 129-300](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:129)
- CLI wrapper and filesystem precheck: [cli.py lines 973-1055](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/cli.py:973)

Python signature:
```python
def verify_round_summary(
    summary_path: str | Path,
    *,
    chunks_manifest_path: str | Path,
    journal_path: str | Path,
    cwd: str | Path | None = None,
    fuzzy: bool = False,
    source_matrix_path: str | Path | None = None,
    allow_retracted: bool = False,
    semantic: "object | None" = None,
) -> RoundSummaryResult:
```

Rust signature:
```rust
fn verify_round_summary(
    summary_path: impl AsRef<Path>,
    chunks_manifest_path: impl AsRef<Path>,
    journal_path: impl AsRef<Path>,
    cwd: Option<&Path>,
    fuzzy: bool,
    source_matrix_path: Option<&Path>,
    allow_retracted: bool,
    semantic: Option<&SemanticSpec>,
) -> Result<RoundSummaryResult, RoundSummaryError>
```

Pre-conditions:
- `summary_path` must be readable UTF-8 text for the library function to succeed.
- The library function itself does not call `is_file()`.
- The Python CLI wrapper does call `summary_path.is_file()` before invoking it.
- `chunks_manifest_path` is not prechecked by the CLI for structural-only outcomes.
- `journal_path` is trusted until `verify_one_card` or journal code uses it.
- `cwd`, if supplied, is authoritative for CWD-relative cited paths.
- `semantic`, if supplied, is passed through unchanged to `verify_one_card`.

Filesystem-shape expectations:
- `summary_path`:
  - CLI path: must be a regular file, checked by `Path.is_file()` at [cli.py lines 978-989](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/cli.py:978).
  - Library path: `summary_p.read_text(encoding="utf-8")` is called directly at [round_summary.py line 153](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:153).
  - If missing, directory, unreadable, or invalid UTF-8 in library use, an exception escapes.
  - CLI catches `OSError` and `UnicodeDecodeError` and maps to `CACG-CLI-001`.
- `chunks_manifest_path`:
  - Not required to exist for structural early returns.
  - Loaded only after the gate ladder and sentinel collision branch.
  - Missing, directory, unreadable, or invalid manifest is caught by `ChunksIndex.from_path` as `ChunksIndexLoadError`, then converted to `chunks_index = None`.
  - Per-card `verify_one_card` retries the load to preserve per-card `CACG-MAN-001` journal cardinality.
- `cards_manifest.json`:
  - Derived from `Path(chunks_manifest_path).parent / "cards_manifest.json"`.
  - Missing means retraction disabled.
  - Present but malformed can raise `RetractionLoadError`, caught by CLI as `CACG-MAN-001`.

Algorithm:
1. Import `verify_one_card`.
2. Convert `summary_path` to `Path`.
3. Read summary text as UTF-8.
4. Determine `cwd_root`:
   - If `cwd is not None`, use `Path(cwd)`.
   - Else use `Path.cwd()`.
5. Determine `summary_dir = summary_p.parent`.
6. Extract section with `extract_section(text)`.
7. Compute `kb_relevant = is_kb_relevant(text)`.
8. If section is missing:
   - Return `RoundSummaryResult(is_na=False, section_missing=True, kb_relevant=kb_relevant, paths=[])`.
9. Parse section into `(is_na, paths)`.
10. Compute `sentinel_collision = is_na and bool(paths)`.
11. Run the strict gate ladder for `not paths`.
12. If no paths and bad N/A on KB-relevant work:
   - Return `CACG-RS-003` structural `PathVerdict`.
13. If no paths and clean N/A on non-KB-relevant work:
   - Return clean `is_na=True`.
14. If no paths and empty section on KB-relevant work:
   - Return `CACG-RS-004` structural `PathVerdict`.
15. If no paths and empty section on non-KB-relevant work:
   - Return clean empty result.
16. Initialize `verdicts = []`.
17. If `sentinel_collision`:
   - Return `CACG-RS-002` structural `PathVerdict`.
18. Load per-batch resources exactly once.
19. Iterate every cited path string in `paths`.
20. Resolve cited path using `_resolve_cited_path`.
21. Call `verify_one_card(...)` for each cited path.
22. If the resolved candidate does not exist:
   - Append `PathVerdict(path=cited, verdict=MISSING, detail="file not found")`.
   - Continue.
23. If `result.verified`:
   - Append `PathVerdict(path=cited, verdict=VERIFIED, detail="")`.
24. Else:
   - Determine `first_code = result.diagnostics[0].code if result.diagnostics else "CACG-VERIFY-001"`.
   - Append `PathVerdict(path=cited, verdict=STALE, detail=f"verify failed: {first_code}")`.
25. Return `RoundSummaryResult(is_na=False, section_missing=False, kb_relevant=kb_relevant, paths=verdicts)`.

Handles:
- Missing section.
- Clean N/A sentinel.
- N/A sentinel misuse on KB-relevant work.
- Empty KB-relevant section.
- Sentinel collision.
- Missing cited card files.
- Stale cited cards.
- Verified cited cards.
- Duplicate cited paths, by re-verifying each occurrence.
- Per-batch shared resource loading.

Deliberately does not handle:
- Rejecting absolute paths.
- Rejecting `..` traversal.
- Deduplicating cited paths.
- Suppressing verification for duplicate paths.
- Markdown code block awareness.
- Structured markdown AST parsing.
- CLI JSON serialization.
- Prechecking `chunks_manifest_path`.
- Prechecking `journal_path`.

## 3.6 `_resolve_cited_path`

Source anchor:
- [round_summary.py lines 303-328](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:303)

Python signature:
```python
def _resolve_cited_path(cited: str, cwd_root: Path, summary_dir: Path) -> Path:
```

Rust signature:
```rust
fn resolve_cited_path(cited: &str, cwd_root: &Path, summary_dir: &Path) -> PathBuf
```

Pre-conditions:
- `cited` is the path string extracted by `_extract_path_from_bullet`.
- Backticks have already been stripped if the path was backtick-quoted.
- The function does not validate that `cited` is relative.
- The function does not normalize separators.
- The function trusts `cwd_root` and `summary_dir`.

Algorithm:
1. Compute `cwd_relative = cwd_root / cited`.
2. If `cwd_relative.exists()`:
   - Return `cwd_relative`.
3. Compute `summary_relative = summary_dir / cited`.
4. If `summary_relative.exists()`:
   - Return `summary_relative`.
5. Return `cwd_relative`.

Handles:
- CWD-relative paths.
- Summary-directory-relative fallback.
- Missing paths.
- Absolute paths according to platform `pathlib` semantics.
- Paths containing spaces, parentheses, and backticks that survived parsing.

Deliberately does not handle:
- Security containment.
- Rejecting path traversal.
- Canonicalization.
- Symlink policy.
- Regular-file checking.
- Platform-independent Windows separator normalization.

# 4. Bullet parsing decision tree

Source anchor:
- [round_summary.py lines 80-102](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:80)

## 4.1 Decision tree

Given one raw line:

1. Match `BULLET_REGEX`.
2. If it does not match:
   - Return `None`.
3. Let `raw = captured_group.strip()`.
4. If `raw` starts with a backtick:
   - Find the next backtick after index 0.
   - If found at index greater than 1:
     - Return substring between the first and second backtick.
5. If `raw` contains `" -- "`:
   - Return text before first `" -- "`, stripped.
6. Else if `raw` contains `" — "`:
   - Return text before first `" — "`, stripped.
7. Else if `raw` contains `" - "`:
   - Return text before first `" - "`, stripped.
8. Else if `raw` contains `" ("`:
   - Return text before first `" ("`, stripped.
9. Else:
   - Return first whitespace-separated token from `raw.split()[0]`.

## 4.2 Required precedence

- Backtick path wins over all separators.
- `" -- "` wins over `" — "`.
- `" — "` wins over `" - "`.
- `" - "` wins over `" ("`.
- Separator matching requires exact surrounding characters.
- The em dash separator is exactly:
  - space
  - U+2014 em dash
  - space
- ASCII hyphen separator is exactly:
  - space
  - `-`
  - space
- Double-hyphen separator is exactly:
  - space
  - `--`
  - space

## 4.3 Enumerated input shapes

| Input line | Regex match? | Extracted path | Explanation |
|---|---:|---|---|
| `- path -- reason` | yes | `path` | `" -- "` separator wins. |
| `- path - reason` | yes | `path` | `" - "` separator applies after double-hyphen and em dash fail. |
| `- path` | yes | `path` | No separator; first token. |
| ``- `path` -- reason`` | yes | `path` | Backtick branch returns before separator logic. |
| `- path (note)` | yes | `path` | `" ("` separator applies. |
| `- path — reason` | yes | `path` | The literal has spaces around U+2014, so `" — "` applies. |
| `- path—reason` | yes | `path—reason` | No surrounding spaces; separator does not apply; first token is whole string. |
| `* path` | yes | `path` | `[-*]` accepts asterisk bullets. |
| `-  multiple  spaces  path` | yes | `multiple` | Spaces after marker are consumed; no separator; first token only. |
| `-` | no | `None` | Requires whitespace and non-whitespace content after marker. |
| `- ` | no | `None` | No non-whitespace content. |
| `-    ` | no | `None` | No non-whitespace content. |
| `* ` | no | `None` | No non-whitespace content. |
| `- a--b` | yes | `a--b` | No `" -- "` separator. |
| `- a-b` | yes | `a-b` | No `" - "` separator. |
| `- a —b` | yes | `a`? no: `a` only if exact `" — "` exists | For `a —b`, there is no trailing space after em dash; fallback first token is `a`. |
| `- a— b` | yes | `a—` | No leading space before em dash separator; fallback first token. |
| `- a -- b - c` | yes | `a` | First separator in precedence is `" -- "`. |
| `- a - b -- c` | yes | `a - b`? no: `a - b` only if `" -- "` appears after; double-hyphen wins and splits later | Because `" -- "` is present, split before it. |
| ``- `a b` -- reason`` | yes | `a b` | Backtick branch allows spaces inside returned path. |
| ``- `a` `b` -- reason`` | yes | `a` | First closing backtick ends path. |
| ``- `` -- reason`` | yes | ```` | Closing backtick is at index 1, not greater than 1; separator returns text before `" -- "`, i.e. two backticks. |
| ``- `unterminated -- reason`` | yes | `` `unterminated`` | Backtick branch fails; `" -- "` separator returns prefix with leading backtick. |
| ` - path` | yes | `path` | Leading indentation is accepted. |
| `\t- path` | yes | `path` | Leading Unicode whitespace is accepted. |
| `- path\t` | yes | `path` | Trailing whitespace consumed by regex and `strip`. |
| `- path with spaces` | yes | `path` | No separator; first whitespace token. |
| `- path(with parens)` | yes | `path(with`? no: `path(with` only if whitespace splits there; actual first token is `path(with`? | Since no whitespace in `path(with parens)`, first token is `path(with`. |
| `- path(note)` | yes | `path(note)` | No `" ("`; first token. |
| `- path (note) -- later` | yes | `path` | `" -- "` appears after parenthetical, but precedence checks `" -- "` first; prefix is `path (note)`, then stripped, so extracted path is `path (note)` if `" -- "` is present. |
| `- path (note)` | yes | `path` | No earlier separator; `" ("` applies. |
| `+ path` | no | `None` | Plus marker not accepted. |
| `1. path` | no | `None` | Ordered list marker not accepted. |
| `-  ` followed by NBSP only | likely no or yes depending content | Unicode-dependent | Python `\s` treats NBSP as whitespace, so NBSP alone is not path content. |

Corrected note for `- path (note) -- later`:
- Raw is `path (note) -- later`.
- Separator order checks `" -- "` before `" ("`.
- `" -- "` exists.
- Split before `" -- "` returns `path (note)`.
- Therefore extracted path is `path (note)`, not `path`.
- This precedence matters.

## 4.4 Bullets with no path content

No path is returned when:
- The whole line fails `BULLET_REGEX`.
- The line has only a marker.
- The line has marker plus whitespace but no non-whitespace content.

Path may still be odd but non-empty when:
- The captured content is a single backtick.
- The captured content is punctuation.
- The captured content is `()`.
- The captured content is `--`.
- The captured content is `N/A`.

## 4.5 Leading whitespace behavior

- Leading whitespace before `-` or `*` is accepted.
- Whitespace after the marker is consumed by `\s+`.
- The captured group starts at the first non-whitespace after the marker.
- `raw.strip()` trims leading and trailing Unicode whitespace from the captured group.
- For `-  multiple  spaces  path`, the extracted path is `multiple`.

# 5. Diagnostic emission table

Primary source anchors:
- Structural gates: [round_summary.py lines 181-233](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:181)
- Per-card loop: [round_summary.py lines 264-293](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:264)
- Missing-section CLI diagnostic: [cli.py lines 1038-1044](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/cli.py:1038)

## 5.1 Important correction: `CACG-RS-001`

The requested description says:

- `CACG-RS-001` — per-card verify failure.
- Emitted in the loop via `first_code`.
- Fallback when diagnostics are empty.

That is not what the Python source does.

Observed Python oracle:
- The per-card loop emits `detail=f"verify failed: {first_code}"`.
- `first_code` is:
  ```python
  result.diagnostics[0].code if result.diagnostics else "CACG-VERIFY-001"
  ```
- The fallback code is `CACG-VERIFY-001`, not `CACG-RS-001`.
- `round_summary.py` does not construct `CACG-RS-001`.
- `CACG-RS-001` is printed by the CLI wrapper for missing `## Knowledge Consulted` on KB-relevant work.
- That CLI diagnostic is not represented as a `PathVerdict`.

Rust port consequence:
- If porting only `round_summary.py`, do not invent a `CACG-RS-001` `PathVerdict`.
- If porting `kb verify --round-summary` CLI behavior, emit `CACG-RS-001` when:
  - `result.section_missing && result.kb_relevant`
- For per-card stale results, use the first underlying diagnostic code, or fallback `CACG-VERIFY-001`.

## 5.2 Diagnostic table

| Code / condition | Python location | Trigger predicate | Output shape | Message text |
|---|---|---|---|---|
| `CACG-RS-001` missing section | `cli.py` lines 1038-1044 | `result.section_missing and result.kb_relevant` | No `PathVerdict`; stderr line; exit `2` | `CACG-RS-001: ## Knowledge Consulted section missing on KB-relevant work` |
| Per-card stale, first diagnostic code | `round_summary.py` lines 285-293 | `candidate.exists() and not result.verified and bool(result.diagnostics)` | `PathVerdict(path=cited, verdict=STALE, detail=f"verify failed: {result.diagnostics[0].code}")` | `verify failed: <first_code>` |
| Per-card stale, fallback | `round_summary.py` lines 285-293 | `candidate.exists() and not result.verified and not result.diagnostics` | `PathVerdict(path=cited, verdict=STALE, detail="verify failed: CACG-VERIFY-001")` | `verify failed: CACG-VERIFY-001` |
| `CACG-RS-002` sentinel collision | `round_summary.py` lines 218-233 | `sentinel_collision` where `sentinel_collision = is_na and bool(paths)` | `PathVerdict(path="(section)", verdict=STALE, detail="CACG-RS-002: N/A sentinel mixed with cited paths")` | `CACG-RS-002: N/A sentinel mixed with cited paths` |
| `CACG-RS-003` bad N/A on KB-relevant work | `round_summary.py` lines 181-193 | `not paths and is_na and kb_relevant` | `PathVerdict(path="(section)", verdict=STALE, detail="CACG-RS-003: N/A sentinel claimed on KB-relevant work")` | `CACG-RS-003: N/A sentinel claimed on KB-relevant work` |
| `CACG-RS-004` empty KB-relevant section | `round_summary.py` lines 200-210 | `not paths and not is_na and kb_relevant` | `PathVerdict(path="(section)", verdict=STALE, detail="CACG-RS-004: empty Knowledge Consulted section on KB-relevant work")` | `CACG-RS-004: empty Knowledge Consulted section on KB-relevant work` |
| Missing cited file | `round_summary.py` lines 278-284 | `not candidate.exists()` after `verify_one_card` returns | `PathVerdict(path=cited, verdict=MISSING, detail="file not found")` | `file not found` |

## 5.3 Rust-side diagnostic carrier

For parser-result parity:
```rust
struct PathVerdict {
    path: String,
    verdict: Verdict,
    detail: String,
}
```

For CLI structural `CACG-RS-001`:
```rust
enum RoundSummaryCliDiagnostic {
    MissingKnowledgeConsultedOnKbRelevantWork,
}
```

Or, if Rust unifies CLI diagnostics into a struct:
```rust
struct DiagnosticLine {
    code: String,
    message: String,
    stream: Stream,
    exit_code: i32,
}
```

Required byte strings:
- `CACG-RS-001: ## Knowledge Consulted section missing on KB-relevant work`
- `CACG-RS-002: N/A sentinel mixed with cited paths`
- `CACG-RS-003: N/A sentinel claimed on KB-relevant work`
- `CACG-RS-004: empty Knowledge Consulted section on KB-relevant work`
- `verify failed: CACG-VERIFY-001`
- `file not found`

# 6. Exit-code matrix

Source anchor:
- [round_summary.py lines 57-65](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:57)

Definitions:
- `section_missing`: result field.
- `kb_relevant`: result field.
- `is_na`: result field.
- `all_verified`: `true` if every `PathVerdict.verdict == VERIFIED`.
- For an empty `paths` list, `all_verified` is vacuously true.

Exact ladder:
1. `section_missing && kb_relevant` returns `2`.
2. `is_na` returns `0`.
3. Any non-verified path returns `1`.
4. Otherwise returns `0`.

Truth table:

| section_missing | kb_relevant | is_na | all_verified | exit_code | Reachability from `verify_round_summary` |
|---:|---:|---:|---:|---:|---|
| false | false | false | false | 1 | reachable: cited path stale/missing in non-KB-relevant full text |
| false | false | false | true | 0 | reachable: empty non-KB section or all cited paths verified |
| false | false | true | false | 0 | unreachable: clean N/A returns empty paths, so all_verified true |
| false | false | true | true | 0 | reachable: clean N/A on non-KB-relevant work |
| false | true | false | false | 1 | reachable: RS-003, RS-004, RS-002, stale/missing cited path |
| false | true | false | true | 0 | reachable: all cited paths verified despite KB relevance |
| false | true | true | false | 0 | unreachable: KB-relevant N/A returns `is_na=False` with RS-003 |
| false | true | true | true | 0 | unreachable: KB-relevant N/A returns `is_na=False` with RS-003 |
| true | false | false | false | 1 | unreachable: missing section returns empty paths |
| true | false | false | true | 0 | reachable: missing section on non-KB-relevant summary |
| true | false | true | false | 0 | unreachable: missing section branch sets `is_na=False` |
| true | false | true | true | 0 | unreachable: missing section branch sets `is_na=False` |
| true | true | false | false | 2 | unreachable by result shape because missing section returns empty paths, but ladder would return 2 |
| true | true | false | true | 2 | reachable: missing section on KB-relevant summary |
| true | true | true | false | 2 | unreachable: missing section branch sets `is_na=False` |
| true | true | true | true | 2 | unreachable: missing section branch sets `is_na=False` |

Rust process exit code emission:
- Library method should return `i32` or `ExitCode`.
- CLI wrapper should:
  - Return `0` for `result.is_na`.
  - Return `2` and print `CACG-RS-001...` for `result.section_missing && result.kb_relevant`.
  - Return `0` and print informational text for `result.section_missing && !result.kb_relevant`.
  - Otherwise print each `PathVerdict` and return `result.exit_code()`.

# 7. State machine for the main verify loop

Source anchors:
- [round_summary.py lines 160-233](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:160)
- Per-card loop begins at [line 264](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:264)

## 7.1 Ordered decision graph

1. Read full summary text.
   - Source: line 153.
   - Hard failure on invalid path/UTF-8 in library mode.

2. Compute `section = extract_section(text)`.
   - Source: line 160.

3. Compute `kb_relevant = is_kb_relevant(text)`.
   - Source: line 161.
   - This happens before the missing-section return.

4. Decision: `section is None`.
   - Source: lines 162-168.
   - Return:
     ```python
     RoundSummaryResult(
         is_na=False,
         section_missing=True,
         kb_relevant=kb_relevant,
         paths=[],
     )
     ```
   - No manifest load.
   - No source matrix load.
   - No retraction load.
   - No BM25 cache.
   - No per-card verify.

5. Parse section.
   - Source: line 170.
   - `is_na, paths = parse_round_summary(section)`.

6. Compute sentinel collision.
   - Source: line 171.
   - `sentinel_collision = is_na and bool(paths)`.

7. Decision: `not paths and is_na and kb_relevant`.
   - Source: lines 181-193.
   - Return `CACG-RS-003`.
   - `is_na` is forced to `False` in the result.
   - No per-batch loads.

8. Decision: `not paths and is_na and not kb_relevant`.
   - Source: lines 181-199.
   - Return clean N/A:
     ```python
     RoundSummaryResult(is_na=True, section_missing=False, kb_relevant=False, paths=[])
     ```
   - No per-batch loads.

9. Decision: `not paths and not is_na and kb_relevant`.
   - Source: lines 200-210.
   - Return `CACG-RS-004`.
   - No per-batch loads.

10. Decision: `not paths and not is_na and not kb_relevant`.
    - Source: lines 211-216.
    - Return clean empty result:
      ```python
      RoundSummaryResult(is_na=False, section_missing=False, kb_relevant=False, paths=[])
      ```
    - No per-batch loads.

11. Initialize `verdicts`.
    - Source: line 218.

12. Decision: `sentinel_collision`.
    - Source: lines 219-233.
    - Predicate is exactly `is_na and bool(paths)`.
    - Return `CACG-RS-002`.
    - No per-batch loads.
    - No cited paths verified.

13. Load per-batch resources.
    - Source: lines 235-262.
    - Happens only after all structural early returns.

14. Iterate `for cited in paths`.
    - Source: line 264.
    - Duplicates are not removed.

15. Resolve cited path.
    - Source: line 265.

16. Call `verify_one_card`.
    - Source: lines 266-277.
    - Called before the `candidate.exists()` check in this loop.

17. Decision: `not candidate.exists()`.
    - Source: lines 278-284.
    - Append `MISSING`.
    - Continue.

18. Decision: `result.verified`.
    - Source: lines 285-286.
    - Append `VERIFIED`.

19. Else:
    - Source: lines 287-293.
    - Append `STALE` with first diagnostic code or fallback `CACG-VERIFY-001`.

20. Return aggregate result.
    - Source: lines 295-300.

## 7.2 ASCII flowchart

```text
read summary_path as UTF-8
        |
extract_section(text); kb_relevant = is_kb_relevant(text)
        |
        +-- section is None
        |       -> RoundSummaryResult(section_missing=true, paths=[])
        |
parse_round_summary(section) -> (is_na, paths)
sentinel_collision = is_na && !paths.is_empty()
        |
        +-- paths empty?
        |       |
        |       +-- is_na && kb_relevant
        |       |       -> RS-003 PathVerdict, exit_code 1
        |       |
        |       +-- is_na && !kb_relevant
        |       |       -> clean N/A, exit_code 0
        |       |
        |       +-- !is_na && kb_relevant
        |       |       -> RS-004 PathVerdict, exit_code 1
        |       |
        |       +-- !is_na && !kb_relevant
        |               -> clean empty, exit_code 0
        |
        +-- sentinel_collision
        |       -> RS-002 PathVerdict, exit_code 1
        |
load ChunksIndex once, AuthSpec once, RetractionSpec once, BM25HintCache once
        |
for each cited path, in order, including duplicates:
        |
resolve path
        |
verify_one_card(...)
        |
        +-- candidate missing -> MISSING
        +-- verified          -> VERIFIED
        +-- else              -> STALE, "verify failed: <first_code>"
        |
return aggregate result
```

# 8. Per-batch one-shot loads

Source anchors:
- [round_summary.py lines 235-262](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:235)
- `ChunksIndex`: [chunks_index.py lines 74-84](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/chunks_index.py:74)
- `AuthSpec`: [source_matrix.py lines 24-57](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/source_matrix.py:24)
- `RetractionSpec`: [retraction.py lines 78-105](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/retraction.py:78)
- `BM25HintCache`: [bm25_hints.py lines 39-65](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/verify/bm25_hints.py:39)

## 8.1 Load ordering

The resources are loaded exactly once per `verify_round_summary` call, but only after:
- Section exists.
- Parsed paths are non-empty.
- Clean and error empty-section branches have returned.
- Clean and error N/A branches have returned.
- Sentinel collision has returned.

The order is:
1. `ChunksIndex.from_path(chunks_manifest_path)`.
2. `AuthSpec.from_optional_path(source_matrix_path)`.
3. Derive `cards_manifest_path`.
4. `RetractionSpec.from_cards_manifest_lenient(cards_manifest_path, allow_retracted=allow_retracted)`.
5. `BM25HintCache()`.

## 8.2 `ChunksIndex.from_path(chunks_manifest_path)`

Python source:
```python
try:
    chunks_index: ChunksIndex | None = ChunksIndex.from_path(chunks_manifest_path)
except ChunksIndexLoadError:
    chunks_index = None
```

Failure mode:
- Silently falls back to `None` on `ChunksIndexLoadError`.
- Does not abort the batch at this point.
- Does not print or emit a `PathVerdict`.
- Per-card `verify_one_card` retries the manifest load or handles `None`.
- This preserves one `CACG-MAN-001` per card when the manifest is invalid.

Filesystem shape:
- `chunks_manifest_path` is not prechecked by CLI.
- Missing path, directory path, unreadable path, invalid UTF-8, invalid JSON, and schema validation errors become `ChunksIndexLoadError`.
- Wrong-shape paths therefore fall back to `chunks_index = None`.

Rust parity:
```rust
let chunks_index = match ChunksIndex::from_path(chunks_manifest_path) {
    Ok(idx) => Some(idx),
    Err(ChunksIndexLoadError { .. }) => None,
};
```

Do not:
- Abort early on missing chunks manifest.
- Emit one batch-level manifest diagnostic.
- Precheck existence before structural early returns.

## 8.3 `AuthSpec.from_optional_path(source_matrix_path)`

Python source:
```python
auth = AuthSpec.from_optional_path(source_matrix_path)
```

Failure mode:
- Opt-in.
- If `source_matrix_path is None`, auth is disabled.
- If path is supplied and valid, auth is enabled with matrix.
- If path is supplied but missing, directory, unreadable, invalid UTF-8, invalid JSON, or schema-invalid:
  - Returns `AuthSpec(matrix=None, load_error=<message>)`.
  - Does not throw.
  - Each card emits one `CACG-AUTH-000` through `verify_one_card`.

Rust parity:
```rust
let auth = AuthSpec::from_optional_path(source_matrix_path);
let auth_arg = if auth.enabled() { Some(&auth) } else { None };
```

Do not:
- Abort batch immediately on malformed source matrix.
- Drop the load error.
- Emit a single batch-level auth diagnostic.

## 8.4 `RetractionSpec.from_cards_manifest_lenient(...)`

Python source:
```python
cards_manifest_path = Path(chunks_manifest_path).parent / "cards_manifest.json"
retraction = RetractionSpec.from_cards_manifest_lenient(
    cards_manifest_path, allow_retracted=allow_retracted,
)
```

Derived path:
- Always based on `chunks_manifest_path.parent`.
- Filename is exactly `cards_manifest.json`.

Failure mode:
- Missing or non-regular `cards_manifest.json`:
  - Returns empty `RetractionSpec`.
  - Retraction disabled unless `allow_retracted` itself makes `enabled` true.
- Present but malformed:
  - Raises `RetractionLoadError`.
  - CLI catches it as a manifest-like exception and prints:
    - `CACG-MAN-001: cards_manifest.json could not be loaded: <exc>`
  - Exit code is `1`.

Rust parity:
```rust
let cards_manifest_path = chunks_manifest_path.parent().unwrap_or(Path::new(""))
    .join("cards_manifest.json");
let retraction = RetractionSpec::from_cards_manifest_lenient(
    &cards_manifest_path,
    allow_retracted,
)?;
let retraction_arg = if retraction.enabled() { Some(&retraction) } else { None };
```

Do not:
- Derive from summary path.
- Derive from CWD.
- Load before structural early returns.
- Ignore malformed present manifests.

## 8.5 `BM25HintCache()`

Python source:
```python
bm25_hint_cache = BM25HintCache()
```

Semantics:
- Empty cache at batch start.
- Shared across all per-card `verify_one_card` calls in this round summary.
- Populated only when failed citations need hints.
- Avoids rebuilding same per-source BM25 corpus repeatedly.
- No load failure at construction.

Rust parity:
```rust
let mut bm25_hint_cache = BM25HintCache::new();
```

Per-card pass-through:
```rust
verify_one_card(..., bm25_hint_cache: Some(&mut bm25_hint_cache))
```

# 9. Path resolution rules

Source anchor:
- [round_summary.py lines 303-328](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:303)

## 9.1 Contract

`_resolve_cited_path(cited, cwd_root, summary_dir)` follows this order:

1. `cwd_root` is authoritative when supplied to `verify_round_summary`.
2. If `cwd` is not supplied, `Path.cwd()` is used.
3. `summary_dir = summary_p.parent`.
4. Try CWD-relative first:
   ```python
   cwd_relative = cwd_root / cited
   if cwd_relative.exists():
       return cwd_relative
   ```
5. Try summary-dir-relative second:
   ```python
   summary_relative = summary_dir / cited
   if summary_relative.exists():
       return summary_relative
   ```
6. If neither exists:
   ```python
   return cwd_relative
   ```

## 9.2 CWD vs summary-dir precedence

Observed Python behavior:
- CWD-relative wins when both paths exist.
- Summary-dir-relative is only fallback when CWD-relative does not exist.

Plan conflict:
- `.humanize/.humanize/plans/cacg-layer3-semantic-port-plan.md` line 73 says:
  - “Summary-dir-relative path resolution wins over CWD when both could resolve.”
- That is stale relative to the Python oracle.
- Rust must mirror Python source for byte parity:
  - CWD wins.

## 9.3 Missing paths

If neither candidate exists:
- Return the CWD-relative candidate.
- This is deliberate.
- The downstream missing-card diagnostic points where the user “meant” relative to CWD.

## 9.4 Absolute paths

Python `pathlib` behavior:
- On POSIX, `Path(cwd_root) / "/abs/path.md"` returns `/abs/path.md`.
- The absolute right-hand side discards the left-hand side.
- Same for `summary_dir / cited`.
- Therefore absolute cited paths are accepted and checked as absolute paths.
- If the absolute file exists, it is returned.
- If it does not exist, the returned missing candidate is the absolute path.

Rust parity:
- `PathBuf::from(cwd_root).join(abs_path)` also discards the base on major platforms.
- Do not reject absolute paths if byte parity with current Python is required.

## 9.5 `..` traversal

Python behavior:
- No rejection.
- No normalization before `.exists()`.
- `cwd_root / "../x.md"` is checked as a filesystem path.
- If it exists, it is returned.
- If neither CWD nor summary fallback exists, CWD-relative traversal path is returned.

Rust parity:
- Do not canonicalize before checking.
- Do not reject `..` unless intentionally diverging from this oracle.

## 9.6 Windows-style separators

Python behavior is platform-dependent:
- On POSIX:
  - Backslash is an ordinary filename character.
  - `cards\foo.md` does not mean `cards/foo.md`.
- On Windows:
  - Backslash is a path separator in `pathlib`.
  - `cards\foo.md` can resolve as nested path.
- `KB_RELEVANT_REGEX` still only detects forward slashes.

Rust parity risk:
- Rust `Path` has platform-dependent separator behavior too.
- Tests running on Linux should expect POSIX behavior.
- Cross-platform byte parity requires explicit fixture expectations per OS or a compatibility layer.

## 9.7 Paths containing spaces

Parser behavior matters first:
- Unquoted `- path with spaces` extracts only `path`.
- Backtick quoted ``- `path with spaces.md` `` extracts `path with spaces.md`.
- `_resolve_cited_path` then accepts the space-containing string as path content.

## 9.8 Paths containing backticks

Parser behavior:
- If raw starts with backtick and has a second backtick after at least one character:
  - The returned cited path excludes the outer first pair.
- Backticks elsewhere can survive.
- `_resolve_cited_path` treats surviving backticks as literal path characters.

Examples:
- ``- `a b.md` `` resolves `a b.md`.
- ``- a`b.md`` resolves `a`b.md` if extracted as first token.
- ``- `a`b.md`` resolves `a`.

## 9.9 Paths containing parentheses

Parser behavior:
- `- path (note)` extracts `path`.
- `- path(note)` extracts `path(note)`.
- `- path (note) -- later` extracts `path (note)` because `" -- "` has higher precedence than `" ("`.

Resolver behavior:
- Parentheses are literal path characters if they survive parsing.

# 10. KB-relevance detection (`is_kb_relevant`)

Source anchors:
- Regex: [round_summary.py line 31](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:31)
- Function: [round_summary.py lines 125-126](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:125)
- Use in verifier: [round_summary.py line 161](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:161)

## 10.1 Exact scan

The regex is:
```regex
(?:cards/|\.claude/knowledge/)
```

It is searched over:
- The entire summary text.
- All headings.
- All prose.
- All code blocks.
- The `Knowledge Consulted` section.
- Any content before or after the section.

It is not restricted to:
- Cited bullet paths.
- Changed file lists.
- Markdown links.
- Actual filesystem paths.

## 10.2 False-positive surfaces

These count as KB-relevant:
```markdown
Mentioned `cards/foo.md` only as an example.
```

```markdown
```text
cards/example.md
```
```

```markdown
The string .claude/knowledge/ appears in old notes.
```

```markdown
A URL like https://example.test/cards/foo also contains cards/
```

These do not count:
```markdown
cards\foo.md
```

```markdown
Cards/foo.md
```

```markdown
.claude\knowledge\foo.md
```

## 10.3 Rust regex considerations

- Use forward-slash pattern exactly.
- Do not preprocess Windows separators into forward slashes.
- Do not lowercase input.
- Do not parse markdown.
- Rust `regex` crate supports this pattern directly.
- Unicode mode does not materially affect this pattern, but keep it enabled for consistency.

# 11. Sentinel-collision semantics

Source anchors:
- Parse accumulation: [round_summary.py lines 110-121](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:110)
- Collision predicate: [round_summary.py line 171](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:171)
- Collision branch: [round_summary.py lines 218-233](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:218)
- Per-batch loads start after it: [round_summary.py lines 235-262](/home/jakeshea/knowledge_base_framework_discovery/src/cacg/integrate/round_summary.py:235)

Predicate:
```python
sentinel_collision = is_na and bool(paths)
```

Key semantics:
- `parse_round_summary` can set `is_na = True` and also append paths in the same pass.
- This happens if the section contains the exact N/A sentinel and any parseable bullet path.
- The sentinel can appear before paths.
- The sentinel can appear after paths.
- The sentinel can appear between paths.
- Multiple sentinel lines do not change the predicate beyond `is_na=True`.

Collision output:
```python
RoundSummaryResult(
    is_na=False,
    section_missing=False,
    kb_relevant=kb_relevant,
    paths=[PathVerdict(
        path="(section)",
        verdict=Verdict.STALE,
        detail="CACG-RS-002: N/A sentinel mixed with cited paths",
    )],
)
```

Short-circuit ordering:
- Sentinel collision returns before:
  - `ChunksIndex.from_path`.
  - `AuthSpec.from_optional_path`.
  - `RetractionSpec.from_cards_manifest_lenient`.
  - `BM25HintCache()`.
  - Any `verify_one_card` call.

Rust requirement:
- Mirror this ordering exactly.
- Do not verify the paths in a sentinel-collision section.
- Do not load manifests in a sentinel-collision section.
- Do not treat N/A as merely informational when paths are present.

Plan conflict:
- `.humanize/.humanize/plans/cacg-layer3-semantic-port-plan.md` line 72 says:
  - “`N/A` with paths → paths still verified (N/A is informational, Python parity).”
- That is stale relative to the current Python oracle.
- Current Python behavior is `CACG-RS-002` structural failure and zero per-card verification.

# 12. Strict mappings to AC-3 and AC-3.2 from the plan

Plan source:
- [.humanize/plans/cacg-layer3-semantic-port-plan.md lines 62-82](/home/jakeshea/knowledge_base_framework_discovery/.humanize/plans/cacg-layer3-semantic-port-plan.md:62)

## 12.1 AC-3: native round-summary port

Plan item:
- `crates/cacg-cli/src/round_summary.rs` hosts parser.
- Dispatcher delegates each cited card to `verify_one_card`.
- `CACG-RS-001 / 002 / 003 / 004` byte-equal with Python.

Python oracle mapping:
- Parser constants:
  - `round_summary.py` lines 30-34.
- Data types:
  - `round_summary.py` lines 37-65.
- Parser functions:
  - `extract_section`: lines 68-77.
  - `_extract_path_from_bullet`: lines 80-102.
  - `parse_round_summary`: lines 105-122.
  - `is_kb_relevant`: lines 125-126.
- Driver:
  - `verify_round_summary`: lines 129-300.
- Path resolution:
  - `_resolve_cited_path`: lines 303-328.
- CLI `CACG-RS-001`:
  - `cli.py` lines 1038-1044.

Observed behavior:
- `CACG-RS-001` is CLI-level missing-section diagnostic, not a `PathVerdict`.
- `CACG-RS-002/003/004` are `PathVerdict` structural failures in `round_summary.py`.
- Per-card verification failures use underlying first diagnostic code, fallback `CACG-VERIFY-001`.

## 12.2 AC-3.1 positive: 5-10 golden fixtures

Plan item:
- Representative markdown fixtures with expected Python-dispatcher output JSON.
- Rust dispatcher byte-compares against expected JSON.

Python oracle mapping:
- Entire CLI path:
  - `_cmd_verify_round_summary`: `cli.py` lines 973-1055.
- Library result:
  - `verify_round_summary`: `round_summary.py` lines 129-300.
- Output printing:
  - `cli.py` lines 1035-1055.

Required fixture dimensions:
- Clean N/A.
- Missing non-KB section.
- Missing KB-relevant section.
- Empty non-KB section.
- Empty KB-relevant section.
- Sentinel collision.
- Verified cited card.
- Missing cited card.
- Stale cited card.
- Duplicate cited path.

Observed behavior:
- CLI output is line-oriented stdout/stderr, not JSON in Python source.
- If expected JSON is introduced for fixtures, it must be derived from Python behavior, not from an existing Python JSON serializer in this module.

## 12.3 AC-3.1 negative: modifying fixture body fails parity

Plan item:
- Modifying any fixture markdown body without updating expected JSON causes parity test failure.

Python oracle mapping:
- Any parser-sensitive change flows through:
  - `extract_section`
  - `parse_round_summary`
  - `is_kb_relevant`
  - `_resolve_cited_path`
  - per-card `verify_one_card`

Observed behavior:
- Even prose outside `Knowledge Consulted` can change `kb_relevant`.
- A body-only change that inserts `cards/` can alter exit code.
- A code-block change that inserts `cards/` can alter exit code.
- A heading spelling change can remove the section entirely.

## 12.4 AC-3.2 positive: `-` and `*` bullet markers

Plan item:
- `## Knowledge Consulted` section parsed with both `-` and `*` bullet markers.

Python oracle:
- `BULLET_REGEX` line 34.
- `_extract_path_from_bullet` lines 90-102.

Observed behavior:
- Both markers are accepted.
- Both require whitespace after marker.
- Both use identical path extraction logic.

## 12.5 AC-3.2 positive: empty section

Plan item:
- Empty section → 0 cards verified, 0 diagnostics, exit 0.

Python oracle:
- `parse_round_summary`: lines 110-122.
- Empty non-KB branch: lines 181, 200, 211-216.
- Exit code: lines 57-65.

Observed behavior:
- Empty section with `kb_relevant=False` returns:
  - `is_na=False`
  - `section_missing=False`
  - `kb_relevant=False`
  - `paths=[]`
  - exit `0`
- Empty section with `kb_relevant=True` returns `CACG-RS-004`, exit `1`.

## 12.6 AC-3.2 positive: N/A inside section

Plan item:
- `N/A` inside the section → 0 cards verified, exit 0.

Python oracle:
- Sentinel detection: lines 112-118.
- Clean N/A branch: lines 181-199.
- Bad N/A branch: lines 181-193.

Observed behavior:
- If full summary is not KB-relevant:
  - Clean N/A.
  - 0 cards verified.
  - exit `0`.
- If full summary is KB-relevant:
  - `CACG-RS-003`.
  - 0 cards verified.
  - exit `1`.

## 12.7 AC-3.2 positive: N/A with paths

Plan item:
- `N/A` with paths → paths still verified (N/A is informational, Python parity).

Python oracle:
- Collision predicate: line 171.
- Collision branch: lines 218-233.

Observed behavior:
- The plan item is stale.
- Current Python does not verify paths.
- Current Python returns:
  - `PathVerdict(path="(section)", verdict=STALE, detail="CACG-RS-002: N/A sentinel mixed with cited paths")`
  - exit `1`
- No manifests are loaded.
- No journal verify events are emitted.

## 12.8 AC-3.2 positive: summary-dir-relative wins over CWD

Plan item:
- Summary-dir-relative path resolution wins over CWD when both could resolve.

Python oracle:
- `_resolve_cited_path`: lines 320-325.

Observed behavior:
- The plan item is stale.
- Current Python checks CWD-relative first.
- If CWD-relative exists, it returns that path.
- Summary-dir-relative is only fallback.
- Rust byte parity target is CWD wins.

## 12.9 AC-3.2 negative: duplicate cited paths

Plan item:
- Duplicate cited paths → exactly Python parity behavior.

Python oracle:
- `parse_round_summary`: paths appended at lines 119-121.
- Per-card loop: line 264.

Observed behavior:
- No deduplication.
- Every occurrence is appended.
- Every occurrence is visited in order.
- Every occurrence calls `verify_one_card`.
- Duplicate path entries produce duplicate `PathVerdict`s.
- Journal cardinality is one verify event per occurrence, assuming no structural early return.

## 12.10 AC-3.2 negative: Windows-style path separators

Plan item:
- Windows-style path separators → Python parity behavior.

Python oracle:
- Bullet parser returns strings unchanged except backtick/separator logic.
- Resolver: lines 320-328.
- KB relevance regex: line 31.

Observed behavior:
- Parser does not convert `\` to `/`.
- Resolver delegates to platform `Path`.
- On POSIX, backslash is literal filename character.
- On Windows, backslash behaves as separator.
- KB relevance does not detect backslashes.

## 12.11 AC-3.2 negative: paths with spaces, backticks, parentheses

Plan item:
- Paths containing spaces, backticks, parentheses → parsed per Python parity.

Python oracle:
- `_extract_path_from_bullet`: lines 93-102.

Observed behavior:
- Spaces:
  - Unquoted paths with spaces usually truncate to first token.
  - Backtick-quoted paths can include spaces.
- Backticks:
  - First matched backtick pair at beginning strips path.
  - Backticks not forming a valid first pair can survive.
- Parentheses:
  - `" ("` separator truncates only after higher-precedence separators fail.
  - Parentheses without preceding space survive.

## 12.12 AC-3.2 negative: multiple `## Knowledge Consulted` sections

Plan item:
- Multiple sections → Python parity behavior.

Python oracle:
- `extract_section`: lines 70-77.

Observed behavior:
- First matching section wins.
- Body ends before the next `##\s` heading.
- A later `## Knowledge Consulted` is not merged.
- Paths in later sections are ignored unless they are inside the first section body before a next heading, which cannot happen for a proper `## ` heading.

## 12.13 AC-3.2 negative: heading lookalikes

Plan item:
- `## knowledge-consulted`, `## Knowledge_Consulted` are not parsed.

Python oracle:
- `SECTION_HEADING_REGEX`: line 32.

Observed behavior:
- Case-sensitive.
- Literal space between words.
- No underscore.
- No hyphen.
- No colon.
- No leading indentation.
- Requires `##` plus whitespace.

## 12.14 AC-3.2 negative: empty markdown file

Plan item:
- Empty markdown file → 0 cards, exit 0.

Python oracle:
- `extract_section`: lines 70-72.
- `is_kb_relevant`: line 126.
- Missing-section branch: lines 162-168.
- CLI printing: lines 1038-1046.
- Exit code: lines 57-65.

Observed behavior:
- `section_missing=True`.
- `kb_relevant=False`.
- `paths=[]`.
- CLI prints:
  - `(no Knowledge Consulted section; round not KB-relevant)`
- exit `0`.

## 12.15 AC-3.2 negative: missing `## Knowledge Consulted` section

Plan item:
- Missing section → exit 0 with informational diagnostic.

Python oracle:
- Missing-section branch: lines 162-168.
- CLI missing-section output: lines 1038-1046.

Observed behavior:
- If `kb_relevant=False`:
  - stdout informational message.
  - exit `0`.
- If `kb_relevant=True`:
  - stderr `CACG-RS-001: ## Knowledge Consulted section missing on KB-relevant work`.
  - exit `2`.
- The plan item is incomplete unless it specifies non-KB-relevant input.

## 12.16 AC-3.2 negative: very large summaries

Plan item:
- Very large summaries >10k cards process without OOM.

Python oracle:
- `extract_section` uses regex search and slicing.
- `parse_round_summary` accumulates all paths in a list.
- `verify_round_summary` accumulates all verdicts in a list.
- Per-batch resources are shared once.
- Per-card loop is sequential.

Observed behavior:
- Memory is O(summary text + path count + verdict count + loaded manifests/cache).
- No streaming verification.
- Rust can improve allocation details, but behavior must preserve ordering and duplicate visits.

# 13. Risks / unresolved ambiguities for the Rust port

## 13.1 `CACG-RS-001` source mismatch

- `round_summary.py` does not emit `CACG-RS-001`.
- CLI emits it for missing section on KB-relevant work.
- Per-card fallback is `CACG-VERIFY-001`.
- Rust implementers must decide whether `round_summary.rs` owns CLI printing or only the parser result.
- Byte parity requires matching `cli.py` too.

## 13.2 Plan conflicts with current source

Two plan AC-3.2 items are stale:
- N/A with paths:
  - Plan says paths still verified.
  - Source returns `CACG-RS-002` before loads.
- Summary-dir-relative wins:
  - Plan says summary-dir wins.
  - Source says CWD wins.

The Python file is the oracle.

## 13.3 Backtick handling

Source:
- Lines 94-98.

Ambiguities:
- Only a leading backtick triggers quote stripping.
- Only the first later backtick is used.
- No escaping.
- No nested handling.
- Closing backtick must be at index greater than 1.
- Empty quoted path ```` is not stripped by the backtick branch.
- A malformed leading backtick can survive into separator or split fallback.

Rust risk:
- Do not use a markdown parser for code spans.
- Do not strip all backticks.
- Do not require balanced code-span semantics.

## 13.4 Unicode whitespace

Source:
- Regex `\s` and Python `strip()` / `split()` all use Unicode whitespace semantics.

Risk:
- Rust `str::trim()` is Unicode-aware.
- Rust `regex` `\s` is Unicode-aware by default.
- If Rust disables Unicode mode for performance, behavior diverges.
- NBSP and other Unicode spaces matter in edge fixtures.

## 13.5 `BULLET_REGEX` first captured character

Pattern:
```regex
^\s*[-*]\s+([^\s].*?)\s*$
```

Implications:
- The first captured character must be non-whitespace.
- Marker plus whitespace-only content fails.
- Trailing whitespace is trimmed by regex and then by `raw.strip()`.
- Non-whitespace punctuation counts as content.
- A line like `- \u00A0` fails because NBSP is whitespace in Python.

## 13.6 CRLF handling

Relevant functions:
- `extract_section` uses multiline regexes.
- `parse_round_summary` uses `splitlines()`.

Behavior:
- `SECTION_HEADING_REGEX` can match CRLF heading lines because trailing `\s*` can consume `\r`.
- `m.end()` may land before `\n` after consuming `\r`.
- `rest` can begin with `\n`.
- `splitlines()` removes line terminators during parsing.
- Bullet parsing sees lines without trailing `\r`.

Rust risk:
- `str::lines()` removes `\n` but leaves a trailing `\r` on CRLF lines.
- Python `splitlines()` removes `\r\n` as a unit.
- For parity, implement Python-like `splitlines()` or trim raw line handling carefully.
- If using `lines()`, bullet regex trailing `\s*$` can consume a remaining `\r`, so most bullet cases still work.
- Sentinel comparison with `line.trim()` also works.
- Exact section slicing around CRLF is the main subtlety.

## 13.7 Encoding

Source:
- `summary_p.read_text(encoding="utf-8")` at line 153.

Behavior:
- Invalid UTF-8 raises `UnicodeDecodeError`.
- Library does not catch it.
- CLI catches it and prints `CACG-CLI-001: cannot read round summary ...`.

Rust risk:
- `std::fs::read_to_string` mirrors UTF-8 requirement.
- Map invalid UTF-8 to the same CLI diagnostic if implementing CLI parity.
- Do not lossy-decode.

## 13.8 Filesystem shape

BL-20260518-shape-check-fs-inputs applies.

Observed Python:
- CLI checks `summary_path.is_file()`.
- Library does not.
- Chunks manifest path is intentionally not prechecked.
- Source matrix path, if supplied, is checked with `is_file()` inside `load_source_matrix`.
- Cards manifest missing is lenient.
- Cards manifest present but malformed is hard failure.

Rust risk:
- Prechecking chunks manifest would break structural early-return behavior.
- Treating a chunks manifest directory as immediate CLI error would diverge.
- Treating missing source matrix as disabled would diverge if the user supplied the path; it must become per-card `CACG-AUTH-000`.

## 13.9 `verify_one_card` call before missing check

Source:
- Lines 266-278.

Behavior:
- The loop calls `verify_one_card` before checking `candidate.exists()`.
- The comment says missing cards also produce journal events.
- Then the round-summary layer appends `MISSING`.

Rust risk:
- If Rust checks `exists()` before calling `verify_one_card`, journal cardinality changes.
- For byte parity, every cited path reaches the runner.

## 13.10 Existence vs regular file

Source:
- `_resolve_cited_path` uses `.exists()`.
- Per-card missing check uses `candidate.exists()`.

Behavior:
- A directory candidate “exists”.
- It will not be marked `MISSING` by round-summary.
- The runner must handle wrong-shape card paths.

Rust risk:
- Do not use `is_file()` in the round-summary missing branch if matching Python.
- Let the runner produce the file-shape diagnostic.

## 13.11 Absolute and traversal paths

Current Python:
- Allows absolute paths.
- Allows `..`.
- Does not canonicalize.
- Does not enforce allowed roots.

Rust risk:
- Security-hardening would diverge.
- If hardening is required later, declare a diagnostic-parity divergence.

## 13.12 Markdown code blocks

Current Python:
- Does not know code blocks.
- `cards/` in a code block triggers KB relevance.
- Bullet-looking lines in a code block inside the section can be parsed as paths.

Rust risk:
- Do not use a markdown AST parser that suppresses code blocks.

## 13.13 Multiple headings

Current Python:
- First exact section only.
- Ends at next `##\s`.
- Does not end at `# ` or `### `.
- Does not accept lookalikes.

Rust risk:
- Markdown heading-level parsing would diverge.

## 13.14 Canonical JSON ambiguity

Current Python:
- `RoundSummaryResult` is a dataclass, not a Pydantic model.
- No canonical JSON output is defined in this module.
- The plan mentions expected JSON fixtures, but the source does not.

Rust risk:
- Fixture JSON schema must be explicitly chosen.
- Once chosen, serialize verdicts as uppercase strings and preserve path order.

# 14. Pseudocode for the Rust port entry point

```rust
fn verify_round_summary(
    summary_path: &Path,
    chunks_manifest_path: &Path,
    journal_path: &Path,
    cwd: Option<&Path>,
    fuzzy: bool,
    source_matrix_path: Option<&Path>,
    allow_retracted: bool,
    semantic: Option<&SemanticSpec>,
) -> Result<RoundSummaryResult, RoundSummaryError> {
    let text = std::fs::read_to_string(summary_path)?;

    let cwd_root: PathBuf = match cwd {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir()?,
    };
    let summary_dir: PathBuf = summary_path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_path_buf();

    let section = extract_section(&text);
    let kb_relevant = is_kb_relevant(&text);

    if section.is_none() {
        return Ok(RoundSummaryResult {
            is_na: false,
            section_missing: true,
            kb_relevant,
            paths: vec![],
        });
    }

    let (is_na, cited_paths) = parse_round_summary(section.unwrap());
    let sentinel_collision = is_na && !cited_paths.is_empty();

    if cited_paths.is_empty() {
        if is_na {
            if kb_relevant {
                return Ok(RoundSummaryResult {
                    is_na: false,
                    section_missing: false,
                    kb_relevant: true,
                    paths: vec![PathVerdict {
                        path: "(section)".to_string(),
                        verdict: Verdict::Stale,
                        detail: "CACG-RS-003: N/A sentinel claimed on KB-relevant work".to_string(),
                    }],
                });
            }

            return Ok(RoundSummaryResult {
                is_na: true,
                section_missing: false,
                kb_relevant: false,
                paths: vec![],
            });
        }

        if kb_relevant {
            return Ok(RoundSummaryResult {
                is_na: false,
                section_missing: false,
                kb_relevant: true,
                paths: vec![PathVerdict {
                    path: "(section)".to_string(),
                    verdict: Verdict::Stale,
                    detail: "CACG-RS-004: empty Knowledge Consulted section on KB-relevant work".to_string(),
                }],
            });
        }

        return Ok(RoundSummaryResult {
            is_na: false,
            section_missing: false,
            kb_relevant: false,
            paths: vec![],
        });
    }

    if sentinel_collision {
        return Ok(RoundSummaryResult {
            is_na: false,
            section_missing: false,
            kb_relevant,
            paths: vec![PathVerdict {
                path: "(section)".to_string(),
                verdict: Verdict::Stale,
                detail: "CACG-RS-002: N/A sentinel mixed with cited paths".to_string(),
            }],
        });
    }

    let chunks_index = match ChunksIndex::from_path(chunks_manifest_path) {
        Ok(index) => Some(index),
        Err(ChunksIndexLoadError { .. }) => None,
    };

    let auth = AuthSpec::from_optional_path(source_matrix_path);

    let cards_manifest_path = chunks_manifest_path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .join("cards_manifest.json");

    let retraction = RetractionSpec::from_cards_manifest_lenient(
        &cards_manifest_path,
        allow_retracted,
    )?;

    let mut bm25_hint_cache = BM25HintCache::new();

    let mut verdicts = Vec::new();

    for cited in cited_paths {
        let candidate = resolve_cited_path(&cited, &cwd_root, &summary_dir);

        let result = verify_one_card(VerifyOneCardArgs {
            card_path: candidate.clone(),
            chunks_manifest_path: chunks_manifest_path.to_path_buf(),
            journal_path: journal_path.to_path_buf(),
            fuzzy,
            skip_lint: false,
            chunks_index: chunks_index.as_ref(),
            auth: if auth.enabled() { Some(&auth) } else { None },
            retraction: if retraction.enabled() { Some(&retraction) } else { None },
            semantic,
            bm25_hint_cache: Some(&mut bm25_hint_cache),
        })?;

        if !candidate.exists() {
            verdicts.push(PathVerdict {
                path: cited,
                verdict: Verdict::Missing,
                detail: "file not found".to_string(),
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
                .map(|d| d.code.as_str())
                .unwrap_or("CACG-VERIFY-001");

            verdicts.push(PathVerdict {
                path: cited,
                verdict: Verdict::Stale,
                detail: format!("verify failed: {}", first_code),
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
```
