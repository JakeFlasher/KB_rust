# BM25 Retrieval Audit for Rust Port

This document is the implementation contract for porting CACG's BM25 retrieval paths to Rust. It audits:

- `.venv/lib/python3.14/site-packages/rank_bm25.py`
- `src/cacg/normalize.py`
- `src/cacg/search.py`
- `src/cacg/search_sqlite.py`
- `src/cacg/verify/bm25_hints.py`

The in-memory search and hint paths use `rank_bm25.BM25Okapi`. The SQLite path is an FTS5 sidecar and is not score-compatible with `rank_bm25`; it is a separate retrieval backend with its own fallback contract.

## 1. `rank_bm25.BM25Okapi` Exact Algorithm

### Constructor parameters

`BM25Okapi.__init__(corpus, tokenizer=None, k1=1.5, b=0.75, epsilon=0.25)` stores:

- `self.k1 = k1`
- `self.b = b`
- `self.epsilon = epsilon`

Then it delegates to `BM25.__init__`, which initializes:

- `self.corpus_size = 0`
- `self.avgdl = 0`
- `self.doc_freqs = []`
- `self.idf = {}`
- `self.doc_len = []`
- `self.tokenizer = tokenizer`

If `tokenizer` is supplied, the corpus is first transformed with `Pool(cpu_count()).map(self.tokenizer, corpus)`. CACG passes already-tokenized corpora and does not use this tokenizer path.

### `_initialize(corpus)`

For each tokenized document in corpus order:

1. Append document length:

   ```text
   doc_len[i] = len(document_i)
   ```

2. Add document length to a running total:

   ```text
   num_doc += len(document_i)
   ```

3. Build a per-document frequency dictionary in first-token-encounter order:

   ```text
   frequencies[word] starts at 0 if absent
   frequencies[word] += 1 for every token occurrence
   ```

4. Append the frequency dictionary:

   ```text
   doc_freqs[i] = frequencies
   ```

5. Update document-frequency dictionary `nd`, iterating `frequencies.items()` in Python dict insertion order:

   ```text
   nd[word] += 1 if word already exists
   nd[word] = 1 otherwise
   ```

   `nd[word]` is the number of documents containing `word`, not the total corpus frequency.

6. Increment corpus size:

   ```text
   corpus_size += 1
   ```

After all documents:

```text
avgdl = num_doc / corpus_size
return nd
```

`num_doc` is the total number of tokens across the corpus. `avgdl` is therefore average document length. The variable name is misleading; it is not a document count.

### `_calc_idf(nd)`

`BM25Okapi._calc_idf` iterates `nd.items()` in insertion order and computes an IDF for each token:

```text
N = self.corpus_size
freq = nd[word]

idf = log(N - freq + 0.5) - log(freq + 0.5)
```

where `log` is `math.log`, i.e. natural logarithm using Python/C double precision.

During this same ordered pass:

```text
self.idf[word] = idf
idf_sum += idf
if idf < 0:
    negative_idfs.append(word)
```

After the pass:

```text
average_idf = idf_sum / len(self.idf)
eps = epsilon * average_idf
for word in negative_idfs:
    self.idf[word] = eps
```

Important consequences:

- The default floor multiplier is `epsilon = 0.25`.
- The epsilon floor is not guaranteed positive. If `average_idf` is negative, then `eps` is negative.
- The floor is applied only to terms whose original computed `idf < 0`.
- Terms whose IDF is exactly `0.0` are not in `negative_idfs`.
- `average_idf` is computed over the original IDF values before replacing negative terms.
- `len(self.idf)` equals the number of unique tokens in the corpus.

### `get_scores(query)`

`get_scores` returns a NumPy array of length `corpus_size`. It starts with:

```text
score = np.zeros(self.corpus_size)
doc_len = np.array(self.doc_len)
```

Then, for every query token `q` in query order, including duplicates:

```text
q_freq = np.array([(doc.get(q) or 0) for doc in self.doc_freqs])

score += (self.idf.get(q) or 0) * (
    q_freq * (self.k1 + 1)
    /
    (
        q_freq
        + self.k1 * (1 - self.b + self.b * doc_len / self.avgdl)
    )
)
```

Per document `d`, the contribution for query token `q` is:

```text
tf_dq = doc_freqs[d].get(q) or 0
dl_d = doc_len[d]

contribution_dq =
    (idf.get(q) or 0)
    *
    (
        tf_dq * (k1 + 1)
        /
        (
            tf_dq
            + k1 * (1 - b + b * dl_d / avgdl)
        )
    )

score[d] += contribution_dq
```

With defaults:

```text
k1 = 1.5
b = 0.75
epsilon = 0.25
```

Fallback semantics are Python truthiness, not just missing-key behavior:

- `doc.get(q) or 0` returns the stored frequency if truthy; absent terms become `0`. Stored zero frequencies are impossible in normal construction, but would also become `0`.
- `self.idf.get(q) or 0` returns the IDF if truthy; absent terms become `0`. An exact `0.0` IDF is also replaced by integer `0`, which is numerically equivalent for scoring.

The algorithm does not deduplicate query tokens. A query list `["x", "x"]` adds the `x` contribution twice.

## 2. Edge Cases and Failure Modes

### Construction failures

`BM25Okapi([])` raises before `_calc_idf`:

```text
avgdl = num_doc / corpus_size = 0 / 0
```

This is a Python `ZeroDivisionError` during `BM25._initialize`.

Non-empty corpora with an empty token union, such as `[[]]` or `[[], []]`, pass `_initialize` but fail in `_calc_idf`:

```text
self.idf == {}
average_idf = idf_sum / len(self.idf) = 0 / 0
```

This is also `ZeroDivisionError`.

Corpora with at least one token construct successfully, even if some documents are empty. Example: `[[], ["x"]]` has `corpus_size = 2`, `num_doc = 1`, `avgdl = 0.5`, and one IDF entry.

### Guards in `search.py`

`SummariesIndex.__init__` builds:

```text
self._entries = list(manifest.summaries)
self._corpus = [_tokenize(_corpus_text_for(e)) for e in self._entries]
self._bm25 = BM25Okapi(self._corpus) if self._corpus else None
```

This guards only the truly empty corpus case: no summary entries means no call to `BM25Okapi([])`.

It does not explicitly guard the empty-token-union case. If `manifest.summaries` is non-empty but every `_corpus_text_for(entry)` tokenizes to `[]`, then `BM25Okapi(self._corpus)` raises `ZeroDivisionError` from `_calc_idf`. In normal valid summaries, `title` and `summary` are required non-empty strings, so this case requires pathological whitespace-only or otherwise token-empty retrieval text.

`SummariesIndex.search` also returns `[]` without scoring when:

- `_bm25 is None`
- `top_k <= 0`
- tokenized query is empty

### Guards in `bm25_hints.py`

`top_k` returns `[]` before building/scoring when:

- `chunks` is empty
- tokenized `quote` is empty

`BM25HintCache.get_or_build` uses:

```text
corpus = [_tokenize(c.text) for c in chunks]
bm25 = BM25Okapi(corpus) if corpus else _EmptyBM25()
```

The cache path guards only an empty `chunks` sequence, via `_EmptyBM25` if called directly with no chunks and via the earlier `top_k` check in normal use. It does not guard non-empty chunks whose texts all tokenize to `[]`; such a corpus raises `ZeroDivisionError` in `BM25Okapi`.

The uncached path in `top_k` similarly builds `BM25Okapi(corpus)` for non-empty `chunks` and has the same all-empty-token-union failure mode.

### `_EmptyBM25`

`_EmptyBM25.get_scores(_query)` returns `[]`. It is a sentinel for cacheability of empty corpora. In normal `top_k` flow, empty `chunks` returns before the sentinel is needed.

### Can `0.0 / 0.0` produce `nan` in `get_scores`?

For a successfully constructed `BM25Okapi` using default parameters, `0.0 / 0.0` should not arise in `get_scores`.

Reason:

- Successful construction implies `corpus_size > 0` and `avgdl > 0`; all-empty-token-union corpora fail during construction.
- With defaults `k1 = 1.5` and `b = 0.75`, the denominator is:

  ```text
  q_freq + 1.5 * (0.25 + 0.75 * dl / avgdl)
  ```

- For absent terms in an empty document, `q_freq = 0` and `dl = 0`, so the denominator is `1.5 * 0.25 = 0.375`, not zero.
- For present terms, `q_freq > 0`, so the denominator is positive.

Thus absent terms contribute `0 / positive = 0`, not `nan`.

If a Rust port exposes arbitrary `k1`/`b` values and tries to match `rank_bm25` beyond CACG's default use, unusual parameter choices can create different denominator behavior. CACG's contract pins the defaults above.

### Query/document terms absent from corpus

If a query token is absent from the corpus:

```text
self.idf.get(q) or 0 == 0
```

The term contributes exactly zero to every document, even though `q_freq` is still computed as a zero array.

If a query token is absent from a specific document but present elsewhere:

```text
doc_freqs[d].get(q) or 0 == 0
```

The term contributes zero to that document because the numerator contains `q_freq`.

## 3. Determinism Concerns for Byte-Equal Rust Port

### Dict insertion order affects `average_idf`

Python 3 dicts preserve insertion order. `rank_bm25` relies on this implicitly:

1. Each per-document `frequencies` dict is ordered by first occurrence of a token in that document.
2. The global `nd` dict is ordered by first corpus encounter of each unique token, using document order and within-document first-token order.
3. `_calc_idf` iterates `nd.items()` in that order.
4. `idf_sum += idf` is floating-point addition in that order.

Floating-point addition is not associative. A Rust port that uses `HashMap` iteration order to compute `idf_sum` can produce a different `average_idf`, and therefore a different `eps` for negative-IDF terms. This can change scores and final rounded output.

Port requirement: preserve a deterministic token-union order equal to Python's `nd` insertion order. Use an ordered map or a `Vec` of first-seen terms plus a map for counts. Sum IDFs in that exact first-seen order.

### NumPy float64 scoring semantics

`get_scores` uses NumPy arrays:

- `np.zeros(self.corpus_size)` defaults to `float64`.
- `np.array(self.doc_len)` starts as an integer array, but `doc_len / self.avgdl` produces floating values.
- `q_freq` is an integer NumPy array; arithmetic promotes to floating point.
- `score += ...` accumulates into a `float64` array.

Python `math.log` returns a C double. NumPy arithmetic here is ordinary IEEE-754 double precision for this platform.

Port requirement: use `f64`, natural log, and preserve the same operation grouping as much as practical. Do not use `f32`. Do not reorder score accumulation by grouping query terms or documents differently if byte-equal rounded output is required.

### Output rounding boundary

CACG rounds scores only at output payload construction:

- `search.py`: `SearchHit.score = round(raw_score, 6)`
- `bm25_hints.py`: hint `"score": round(float(score), 6)`
- `search_sqlite.py`: `SqliteSearchHit.score = round(-float(rank), 6)`

Python `round(x, 6)` uses round-half-to-even for exact ties, applied to the binary floating-point value. Because most decimal half cases are not exactly representable, matching this exactly in Rust requires care. Formatting to six decimals is not the same as storing Python's rounded float unless the downstream JSON/text layer also matches Python behavior.

Port requirement: if serialized output must be byte-equal, define and test the full float-to-output path, not only the numeric BM25 formula.

### Sort and tie-break rules

`search.py` sorts after all filters and before rounding:

```text
raw_hits.sort(key=lambda t: (-t[0], self._entries[t[1]].id))
```

Tie-break is `card_id` ascending. The primary key is full-precision raw score descending, not rounded score.

`bm25_hints.py` sorts:

```text
sorted(enumerate(scores), key=lambda x: (-x[1], x[0]))[:k]
```

Tie-break is original chunk index ascending.

`search_sqlite.py` asks SQLite for:

```sql
ORDER BY rank, card_id ASC
```

FTS5 `bm25()` returns smaller values for better matches; CACG negates the returned `rank` for output. The SQLite backend's tie-break is `card_id ASC`.

## 4. Tokenizer Contract

### Normalization pipeline

`normalize_for_lookup(text)` is exactly:

```python
normalize_text(text).casefold()
```

`_tokenize(text)` in both `search.py` and `bm25_hints.py` is exactly:

```python
normalize_for_lookup(text).split()
```

`normalize_text` runs these steps, in this order, before case folding:

1. Unicode NFC:

   ```python
   unicodedata.normalize("NFC", text)
   ```

2. Replace the following Latin ligatures if present:

   ```text
   ﬀ -> ff
   ﬁ -> fi
   ﬂ -> fl
   ﬃ -> ffi
   ﬄ -> ffl
   ﬅ -> ft
   ﬆ -> st
   ```

3. Rejoin hyphenated line breaks using regex `r"-\s*\n\s*"`:

   ```text
   "word-\nnext" -> "wordnext"
   ```

   This also removes optional whitespace around the newline after the hyphen.

4. Collapse whitespace runs using regex `r"\s+"`:

   ```text
   every run of Unicode regex whitespace -> single ASCII space
   ```

5. Strip leading and trailing whitespace:

   ```python
   normalized.strip()
   ```

Only after those steps does lookup normalization call `.casefold()`.

### `casefold()` is not `lower()`

Python `str.casefold()` performs full Unicode case folding. A Rust port must not replace it with ASCII lowercase or ordinary Unicode lowercase.

Important examples:

- German sharp s:

  ```text
  "ß".casefold() == "ss"
  "ß".lower() == "ß"
  ```

- Greek final sigma folds to standard sigma:

  ```text
  "ς".casefold() == "σ"
  ```

- Latin capital I with dot above expands to two code points:

  ```text
  "İ".casefold() == "i\u0307"
  ```

These expansions affect token strings, token lengths, hashes/signatures if reused, and BM25 matching.

### `split()` with no arguments

Python `str.split()` with no separator:

- Splits on runs of Unicode whitespace.
- Discards leading/trailing empty fields.
- Discards empty fields between whitespace runs.

Examples:

```text
"  a \t b\n\nc  ".split() == ["a", "b", "c"]
"".split() == []
"   ".split() == []
```

Because `normalize_text` already collapses whitespace and strips, most lookup text reaches `split()` with single ASCII spaces, but the exact Python no-argument split behavior is still part of the contract.

## 5. `search.py` `SummariesIndex` Contract

### Corpus surface

Each summary entry contributes exactly:

```python
" ".join([entry.title, entry.summary, *entry.tags])
```

The corpus excludes card body text, citations, `reading_id`, `card_id`, `path`, `card_hash`, and `source_ids`.

Each corpus string is tokenized with:

```python
normalize_for_lookup(text).split()
```

Then `BM25Okapi(self._corpus)` is built, unless there are no entries.

### `from_path` legacy `source_ids` detection

`SummariesIndex.from_path(path)` reads the raw file as UTF-8 text and parses JSON before Pydantic validation. This is intentional because Pydantic would auto-populate missing `source_ids` with `[]`.

Behavior:

1. If raw JSON parse fails, raise:

   ```text
   ValueError("CACG-MAN-001: cannot parse summaries.json: ...")
   ```

2. If the parsed top-level value is a dict, iterate `raw.get("summaries", [])`.
3. For every dict entry, if `"source_ids"` is absent, raise:

   ```text
   ValueError("CACG-SUM-007: legacy summaries.json lacks `source_ids` ...")
   ```

4. Then validate with:

   ```python
   SummariesManifest.model_validate_json(raw_text)
   ```

This fail-closed check is specific to pre-Phase-4 `summaries.json` artifacts.

### Search order: score, filter, sort, cap, round

`SummariesIndex.search` executes in this order:

1. Return `[]` if `_bm25 is None` or `top_k <= 0`.
2. Tokenize the query.
3. Return `[]` if query tokens are empty.
4. Build `query_token_set = set(query_tokens)`.
5. Score the full corpus:

   ```python
   scores = self._bm25.get_scores(query_tokens)
   ```

   This happens before authorization, retraction, and lexical-overlap filtering.

6. Prepare optional filters:

   - `allowed_set = set(allowed_reading_ids)` if supplied.
   - `matrix_sets = {reading_id: frozenset(srcs) ...}` if `source_matrix` is supplied.
   - `retracted_set = frozenset(retracted_card_ids)` if supplied, otherwise empty.

7. Iterate all scored documents in corpus order and apply filters:

   - Exclude if `entry.id in retracted_set`.
   - If `source_matrix` is supplied, it supersedes `allowed_reading_ids`:
     - Exclude if `entry.reading_id` is not a key in the matrix.
     - Exclude if `entry.source_ids` is empty.
     - Exclude if `set(entry.source_ids).issubset(allowed_sources)` is false.
   - Else, if `allowed_reading_ids` is supplied, exclude if `entry.reading_id not in allowed_set`.
   - Exclude if no lexical overlap:

     ```python
     if not (query_token_set & set(doc_tokens)):
         continue
     ```

8. Append surviving raw hits as `(float(score), idx)`.
9. Sort full-precision raw hits:

   ```python
   raw_hits.sort(key=lambda t: (-t[0], self._entries[t[1]].id))
   ```

10. Apply `top_k` cap with `raw_hits[:top_k]`.
11. Construct `SearchHit` objects, rounding only now:

    ```python
    score=round(raw_score, 6)
    ```

### Match semantics and lexical overlap

BM25 score alone is not the match predicate. A card is a match only if:

```text
set(query_tokens) intersects set(document_tokens)
```

This prevents small-corpus negative-IDF behavior from turning non-overlapping documents into apparent matches or excluding all overlapping negative-score documents by a positive-score threshold. There is no score threshold.

Repeated query tokens affect BM25 ranking because `get_scores` sees the full query token list, but repeated tokens do not affect lexical-overlap filtering because the filter uses a set.

### Evidence-required source authorization

When `source_matrix` is supplied, an entry with empty `source_ids` is excluded even though the empty set is mathematically a subset of every allow-list. CACG requires positive citation-side evidence for search authorization.

This rule exists in both the in-memory and SQLite search backends.

## 6. `search_sqlite.py` FTS5 Sidecar Contract

The SQLite sidecar is an optimization for `kb search`, not a source of truth and not a BM25Okapi-compatible scorer.

### Constants

```text
SIDECAR_FILENAME = "summaries.sqlite"
SIDECAR_SCHEMA_VERSION = "cacg.v0.fts1"
BUILDER_VERSION = "cacg.fts5.builder.v1"
TOKENIZE = "unicode61 remove_diacritics 1"
```

### FTS5 availability probe

`fts5_available()`:

1. Opens `sqlite3.connect(":memory:")`.
2. Attempts:

   ```sql
   CREATE VIRTUAL TABLE _probe USING fts5(content)
   ```

3. Returns `True` if successful.
4. Closes the connection in a `finally`.
5. Returns `False` only when `sqlite3.OperationalError` is raised.

### Build behavior and schema

`build_sidecar(out_dir, summaries_json_bytes, manifest, replace=os.replace, warn_when_unavailable=True)`:

1. If FTS5 is unavailable:
   - If `warn_when_unavailable`, print `CACG-FTS-002` to stderr.
   - Return `None`.

2. Define:

   ```text
   sidecar_path = out_dir / "summaries.sqlite"
   tmp_path = out_dir / "summaries.sqlite.tmp"
   ```

3. If `tmp_path` already exists, raise `FileExistsError` with `CACG-MAN-002`.

4. Compute seal:

   ```text
   summaries_hash = sha256(summaries_json_bytes).hexdigest()
   summaries_count = len(manifest.summaries)
   schema_version = "cacg.v0.fts1"
   builder_version = "cacg.fts5.builder.v1"
   ```

5. Connect to `tmp_path`.

6. Set:

   ```sql
   PRAGMA journal_mode=DELETE
   PRAGMA synchronous=NORMAL
   ```

7. Create FTS table:

   ```sql
   CREATE VIRTUAL TABLE cards_fts USING fts5(
       card_id UNINDEXED,
       reading_id UNINDEXED,
       path UNINDEXED,
       card_hash UNINDEXED,
       title,
       summary,
       tags,
       source_ids,
       tokenize = 'unicode61 remove_diacritics 1'
   )
   ```

8. Create meta table:

   ```sql
   CREATE TABLE meta (
       key TEXT PRIMARY KEY,
       value TEXT NOT NULL
   )
   ```

9. Insert meta rows:

   ```text
   ("schema_version", "cacg.v0.fts1")
   ("builder_version", "cacg.fts5.builder.v1")
   ("summaries_hash", sha256 hex of summaries_json_bytes)
   ("summaries_count", decimal string of len(manifest.summaries))
   ```

10. Insert one `cards_fts` row per summary:

    ```text
    card_id    = entry.id
    reading_id = entry.reading_id
    path       = entry.path
    card_hash  = entry.card_hash
    title      = entry.title
    summary    = entry.summary
    tags       = " ".join(entry.tags)
    source_ids = " ".join(entry.source_ids)
    ```

11. Commit, close, then publish atomically with:

    ```python
    replace(str(tmp_path), str(sidecar_path))
    ```

12. Return `sidecar_path`.

If any exception escapes after connect, the connection is closed in `finally`. The caller in `index.py` treats sidecar build failure as non-fatal and prints `CACG-FTS-002: summaries.sqlite sidecar build failed ...`; it does not roll back `summaries.json`.

### Seal loading and stale detection

`load_seal(conn)` reads:

```sql
SELECT key, value FROM meta
```

into a dict, then constructs:

```text
schema_version  = rows.get("schema_version", "")
builder_version = rows.get("builder_version", "")
summaries_hash  = rows.get("summaries_hash", "")
summaries_count = int(rows.get("summaries_count", "0") or "0")
```

`SummariesSqliteIndex.__init__(sidecar_path, expected_summaries_hash)`:

1. Opens read-only URI:

   ```text
   file:{sidecar_path}?mode=ro
   ```

2. Calls `load_seal`.
3. If `load_seal` raises `sqlite3.OperationalError`, close and raise `SidecarStaleError` with `CACG-FTS-001`.
4. If `schema_version != SIDECAR_SCHEMA_VERSION`, close and raise `SidecarStaleError` with `CACG-FTS-001`.
5. If `builder_version != BUILDER_VERSION`, close and raise `SidecarStaleError` with `CACG-FTS-001`.
6. If `seal.summaries_hash != expected_summaries_hash`, close and raise `SidecarStaleError` with `CACG-FTS-001`.
7. Store `_seal`.

`expected_summaries_hash` is computed by the CLI from the current `summaries.json` bytes using `compute_summaries_hash`, i.e. raw SHA-256 of the on-disk bytes.

Note: `summaries_count` is loaded and exposed in `seal`, but `SummariesSqliteIndex.__init__` does not compare it to an independently computed expected count. Staleness is enforced by schema version, builder version, and `summaries_hash`.

### Query tokenization and MATCH construction

`SummariesSqliteIndex.search`:

1. Return `[]` if `top_k <= 0`.
2. Tokenize query with:

   ```python
   tokens = [t for t in normalize_for_lookup(query).split() if t]
   ```

   The final `if t` is redundant after `split()` but part of the implementation.

3. Return `[]` if `tokens` is empty.
4. Quote-escape every token for FTS5:

   ```python
   escaped = [f'"{t.replace(chr(34), chr(34) * 2)}"' for t in tokens]
   fts_expr = " OR ".join(escaped)
   ```

This wraps every token in double quotes and doubles embedded double quote characters. The tokens are joined with `OR`, intentionally matching the in-memory backend's any-token match surface rather than FTS5's default stricter behavior.

Example:

```text
tokens:    ['alpha', 'a"b']
escaped:   ['"alpha"', '"a""b"']
fts_expr:  "alpha" OR "a""b"
```

### SQLite search execution and filtering

The query is:

```sql
SELECT card_id, reading_id, path, card_hash, title, summary,
       tags, source_ids, bm25(cards_fts) AS rank
FROM cards_fts
WHERE cards_fts MATCH ?
ORDER BY rank, card_id ASC
```

with `fts_expr` as the parameter.

If `sqlite3.OperationalError` is raised during execute, it is converted to `SidecarStaleError` with `CACG-FTS-001`. The class does not perform fallback itself; it exposes the failure in the same exception family used for stale/untrusted sidecars.

Rows are streamed in SQLite rank order. For each row:

1. Exclude if `card_id in retracted_set`.
2. Parse `source_ids` with:

   ```python
   source_ids = source_ids_str.split() if source_ids_str else []
   ```

3. If `source_matrix` is supplied:
   - Exclude if `reading_id` is absent from matrix.
   - Exclude if parsed `source_ids` is empty.
   - Exclude if `set(source_ids).issubset(allowed)` is false.

4. Append a `SqliteSearchHit`:

   ```text
   tags = tuple(tags_str.split()) if tags_str else ()
   score = round(-float(rank), 6)
   ```

5. Break once `len(hits) >= top_k`.

The SQL query itself has no `LIMIT`; CACG filters authorization and retractions before applying `top_k`, matching the in-memory backend's filter-before-cap policy. The result stream is bounded by the number of MATCHing rows.

### `CACG-FTS-001` vs `CACG-FTS-002`

`CACG-FTS-001` is query/open-time sidecar invalidity or unreadability. It means `summaries.sqlite` exists but cannot be trusted or queried. The sidecar class reports this condition; fallback is a caller responsibility. Sources include:

- unreadable `meta` table
- schema version mismatch
- builder version mismatch
- `summaries_hash` seal mismatch
- FTS5 `MATCH` execution failure
- CLI catch-all when opening `summaries.sqlite` raises an unexpected exception

`CACG-FTS-002` is build-time sidecar unavailability or build failure. It is non-fatal because the sidecar is an optimization. Sources include:

- runtime sqlite3 lacks FTS5 support: `build_sidecar` returns `None` and warns
- sidecar build raises during index publish: caller prints `CACG-FTS-002` and keeps the published `summaries.json`

### Backend fallback

The CLI tries the SQLite sidecar first only if `summaries.sqlite` exists. It does not eagerly construct `SummariesIndex` because that would build in-memory BM25 and defeat the cold-open sidecar path.

Fallback cases:

- No sidecar file: build `SummariesIndex.from_path(summaries_path)`.
- `SummariesSqliteIndex.__init__` raises `SidecarStaleError`: print the `CACG-FTS-001` message, then build in-memory BM25.
- Unexpected sidecar open exception: print `CACG-FTS-001: cannot open summaries.sqlite ...`, then build in-memory BM25.
- Query-time `SummariesSqliteIndex.search` `SidecarStaleError`: the class signals the same `CACG-FTS-001` condition, but it does not rebuild or dispatch to `SummariesIndex` internally. A caller that wants query-time fallback must catch this exception around `search()`.

The sidecar's raw SQLite bytes are explicitly not part of the byte-determinism contract. Determinism is on query results and the sealed `meta.summaries_hash`.

## 7. `bm25_hints.py` Contract

BM25 hints are diagnostic only. They never grant verification authority.

### Tokenizer

Hint tokenization is the same lookup tokenizer as search:

```python
def _tokenize(text: str) -> list[str]:
    return normalize_for_lookup(text).split()
```

### Scope

Hints are scored only over the chunks supplied by the caller. The module-level policy states these are chunks from the same `source_id` as the citation. It does not search across sources.

### Cache key

`BM25HintCache` is process-local. It caches per:

```text
(source_id, chunks_signature)
```

`_chunks_signature(chunks)` is exactly:

```python
"|".join(c.chunk_hash for c in chunks)
```

The signature is sensitive to:

- chunk count
- chunk order
- chunk content, assuming `chunk_hash` is content-addressable

The literal separator is the pipe character `|`.

Cached value shape:

```text
(BM25-like object, tuple[ChunkRecord, ...])
```

The snapshot tuple is used later for stable hint payload assembly.

### `top_k`

Signature:

```python
top_k(
    quote: str,
    chunks: Sequence[ChunkRecord],
    *,
    k: int = 3,
    cache: BM25HintCache | None = None,
    source_id: str | None = None,
) -> list[dict[str, Any]]
```

Default `k` is `3`.

Execution:

1. If `chunks` is empty, return `[]`.
2. Tokenize `quote`.
3. If query tokens are empty, return `[]`.
4. If both `cache` and `source_id` are supplied, call:

   ```python
   bm25, snapshot = cache.get_or_build(source_id, chunks)
   ```

5. Otherwise build per call:

   ```python
   corpus = [_tokenize(c.text) for c in chunks]
   bm25 = BM25Okapi(corpus)
   snapshot = tuple(chunks)
   ```

6. Score:

   ```python
   scores = bm25.get_scores(query)
   ```

7. Rank:

   ```python
   ranked = sorted(
       enumerate(scores),
       key=lambda x: (-x[1], x[0]),
   )[:k]
   ```

8. Emit one dict per ranked hit:

   ```python
   {
       "hint_only": True,
       "chunk_id": c.chunk_id,
       "score": round(float(score), 6),
       "text_preview": c.text_preview,
   }
   ```

Tie-break is lower original chunk index. No lexical-overlap filter is applied here; every chunk receives a score and can appear in the top `k`, including zero-score chunks if the query has no terms present in the corpus but query tokens are non-empty. However, absent query tokens contribute zero across all chunks, and sorting then falls back to index order.

## 8. Rust Port Implications Checklist

A naive Rust port will diverge unless it handles every item below.

### BM25 construction and IDF

- Preserve `k1=1.5`, `b=0.75`, `epsilon=0.25` defaults.
- Compute `avgdl` exactly as `total_token_count as f64 / corpus_size as f64`.
- Raise or otherwise reproduce failure for empty corpus (`0 / 0` in Python construction), rather than silently creating an empty index, unless the caller layer intentionally keeps the Python guard behavior.
- Raise or reproduce failure for non-empty all-empty-token-union corpora (`idf_sum / len(idf)` with `len == 0`).
- Compute document frequency as number of documents containing token, not total token count.
- Compute IDF as `ln(N - freq + 0.5) - ln(freq + 0.5)`.
- Compute `average_idf` from original IDFs before flooring negatives.
- Floor only original `idf < 0` terms to `epsilon * average_idf`.
- Do not force the epsilon floor positive.
- Preserve Python first-seen token-union order when summing `idf_sum`.

### Scoring

- Use `f64`, not `f32`.
- Accumulate query token contributions in query list order.
- Do not deduplicate repeated query tokens.
- Treat absent query IDF as zero.
- Treat absent document term frequency as zero.
- Keep the operation grouping close to:

  ```text
  idf * (qf * (k1 + 1) / (qf + k1 * (1 - b + b * dl / avgdl)))
  ```

- For CACG defaults, do not add special `nan` handling in normal scoring; successful Python construction avoids `0.0 / 0.0`.

### Tokenization and normalization

- Implement Unicode NFC before ligature replacement, hyphen-linebreak removal, whitespace collapse, and strip.
- Replace exactly the seven listed ligature code points.
- Implement regex-equivalent `-\s*\n\s*` removal.
- Implement regex-equivalent `\s+` whitespace collapse.
- Apply full Unicode `casefold`, not lowercase.
- Match Python no-argument `split()` semantics for Unicode whitespace and empty-field discarding.

### Search ranking pipeline

- Build the search corpus from `title + summary + tags` only.
- Score the full corpus before authorization/retraction/lexical filters.
- Apply retraction and source authorization before top-k cap.
- If `source_matrix` is present, ignore `allowed_reading_ids`.
- Enforce evidence-required exclusion for empty `source_ids`.
- Enforce lexical overlap as the match predicate using sets of query and document tokens.
- Sort by full-precision score descending, then `card_id` ascending.
- Apply `top_k` after sorting filtered hits.
- Round to six decimals only in the output object, not before sorting.

### Hint ranking pipeline

- Scope hints to the caller-supplied same-source chunks.
- Use cache key `(source_id, chunk_hashes_joined_with_pipe)`.
- Preserve chunk order in the cache snapshot.
- Default `k` to `3`.
- Sort by score descending, then original chunk index ascending.
- Emit exactly `hint_only`, `chunk_id`, `score`, and `text_preview`.
- Round hint scores with Python-compatible `round(score, 6)` behavior at output.

### SQLite sidecar

- Do not expect score parity between SQLite FTS5 and `rank_bm25`.
- Preserve sidecar schema, meta keys, and seal hash over raw `summaries.json` bytes.
- Preserve FTS5 tokenizer setting `unicode61 remove_diacritics 1`.
- Quote every MATCH token and double embedded double quotes.
- Join MATCH tokens with `OR`.
- Stream ordered by `rank, card_id ASC`; negate rank for output score.
- Filter retractions and source authorization before enforcing `top_k`.
- Treat sidecar open/query invalidity as `CACG-FTS-001` fallback.
- Treat build-time FTS5 unavailability/failure as `CACG-FTS-002` non-fatal.

### Byte-equal output

- Match Python dict-order-dependent IDF summation.
- Match Python/NumPy `float64` arithmetic closely enough for the six-decimal boundary.
- Match Python half-even rounding semantics for `round(x, 6)`.
- Match final JSON/stdout float formatting in the caller, not just internal score values.
- Add golden tests that include:
  - repeated query tokens
  - negative-IDF floor terms
  - absent query tokens
  - empty documents mixed with non-empty documents
  - near-tie scores that differ before six-decimal rounding
  - equal-score tie-breaks by `card_id` and by chunk index
  - Unicode casefold cases: `ß`, `ς`, and `İ`
