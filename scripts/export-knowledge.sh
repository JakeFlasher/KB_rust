#!/usr/bin/env bash
#
# export-knowledge.sh — Tiered export of a CACG knowledge deck into a downstream project.
#
# Copies the cards + the consumer-facing manifests of one deck (default: cfa)
# into a target project in a STRUCTURED, TIERED layout that the `kb` binary and
# the humanize harness can consume directly. The export is read from the local
# WORKING TREE (not `git archive`) because the consumer-critical
# `source_matrix.json` / `chunks_manifest.json` / `summaries.sqlite` are
# gitignored build artifacts that exist on disk but are never committed.
#
# Usage:
#   scripts/export-knowledge.sh <TARGET_DIR> [options]
#
# Options:
#   --tier query|verify|full   What to export (default: query)
#                                query  = browse + `kb search`/`kb show`     (~9 MB)
#                                verify = query + chunks_manifest.json        (~195 MB; enables `kb verify`)
#                                full   = verify + source PDFs + _registry    (~1.3 GB; enables re-ingest)
#   --deck <name>              Deck to export (default: cfa)
#   --dest-root <relpath>      Sub-path under TARGET to write into
#                                (default: .claude/knowledge — where humanize's primer is expected)
#   --no-sqlite                Skip summaries.sqlite (regenerable FTS index; ~600 KB)
#   --no-semantic              Skip out/semantic_cache.json{,.provenance.json}
#   --with-binary              Also copy the built `kb` binary into <dest>/bin/kb (platform-specific)
#   --force                    Overwrite an existing populated dest-root
#   --dry-run                  Print what would be copied; copy nothing
#   -h, --help                 This help
#
# Layout produced under <TARGET>/<dest-root>/ (default .claude/knowledge):
#   INDEX.md                 primer (copy of out/<deck>/INDEX.md; humanize looks here)
#   README.md                generated consumer guide
#   EXPORT_MANIFEST.json     receipt: paths, sizes, sha256, src commit, kb version+sha, tier, counts
#   kb-query.sh              wrapper that cd's into dest-root and runs kb search/show/verify
#   cards/<deck>/...         500 cards (.md + .history.jsonl), grouped by reading_id
#   out/<deck>/...           source_matrix.json, cards_manifest.json, summaries.json, INDEX.md, ...
#   out/semantic_cache.json  (+ provenance) unless --no-semantic
#   sources/<deck>/...       (tier full only) pdfs/ + _registry/
#
set -euo pipefail

# --------------------------------------------------------------------------
# Constants
# --------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
KB_REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
EXPECTED_KB_VERSION="kb 0.1.0"
DEFAULT_DEST_ROOT=".claude/knowledge"
RECEIPT_NAME="EXPORT_MANIFEST.json"

# --------------------------------------------------------------------------
# Helpers
# --------------------------------------------------------------------------
log()  { printf '%s\n' "--- $*"; }
ok()   { printf '%s\n' "[OK] $*"; }
warn() { printf '%s\n' "[WARN] $*" >&2; }
die()  { printf '%s\n' "FATAL: $*" >&2; exit 1; }

usage() { sed -n '2,/^set -euo/p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//; /^set -euo/d'; }

require_cmd()  { command -v "$1" >/dev/null 2>&1 || die "required command not found: $1"; }
require_file() { [ -f "$1" ] || die "required source file missing: $1${2:+  ($2)}"; }

# Portable SHA-256: GNU coreutils `sha256sum`, else BSD/macOS `shasum -a 256`.
if command -v sha256sum >/dev/null 2>&1; then _sha256() { sha256sum "$@"; }
elif command -v shasum >/dev/null 2>&1; then _sha256() { shasum -a 256 "$@"; }
else die "need 'sha256sum' or 'shasum' on PATH (install coreutils on macOS)"; fi
sha256_of()    { _sha256 "$1" | cut -d' ' -f1; }
bytes_of()     { wc -c < "$1" | tr -d ' '; }

# --------------------------------------------------------------------------
# Argument parsing
# --------------------------------------------------------------------------
TARGET=""
TIER="query"
DECK="cfa"
DEST_ROOT="$DEFAULT_DEST_ROOT"
FORCE=0
DRY_RUN=0
COPY_SQLITE=1
COPY_SEMANTIC=1
COPY_BINARY=0

while [ $# -gt 0 ]; do
  case "$1" in
    --tier)        TIER="${2:-}"; shift 2;;
    --deck)        DECK="${2:-}"; shift 2;;
    --dest-root)   DEST_ROOT="${2:-}"; shift 2;;
    --no-sqlite)   COPY_SQLITE=0; shift;;
    --no-semantic) COPY_SEMANTIC=0; shift;;
    --with-binary) COPY_BINARY=1; shift;;
    --force)       FORCE=1; shift;;
    --dry-run)     DRY_RUN=1; shift;;
    -h|--help)     usage; exit 0;;
    --) shift; break;;
    -*) die "unknown flag: $1 (try --help)";;
    *)  [ -z "$TARGET" ] && TARGET="$1" || die "unexpected extra argument: $1"; shift;;
  esac
done

[ -n "$TARGET" ] || { usage; die "TARGET project directory is required"; }
case "$TIER" in query) T=1;; verify) T=2;; full) T=3;; *) die "--tier must be query|verify|full";; esac
[ -n "$DECK" ] || die "--deck must be non-empty"
[ -n "$DEST_ROOT" ] || die "--dest-root must be non-empty"
case "$DEST_ROOT" in /*) die "--dest-root must be relative to TARGET, not absolute";; esac

require_cmd rsync   # the only safe recursive-copy-with-exclude mechanism (cards/ holds 100+ *.lock sidecars)
require_cmd jq

# --------------------------------------------------------------------------
# Resolve source paths and preflight
# --------------------------------------------------------------------------
SRC_OUT="$KB_REPO_ROOT/out"
SRC_DECK_OUT="$SRC_OUT/$DECK"
SRC_CARDS="$KB_REPO_ROOT/cards/$DECK"
SRC_SOURCES="$KB_REPO_ROOT/sources/$DECK"

[ -d "$SRC_CARDS" ] || die "cards/$DECK not found under $KB_REPO_ROOT (bad --deck?)"
# Unconditionally-copied tier-1 files must exist up front so we fail fast with a
# clear message instead of aborting mid-copy (these are regenerable + gitignored).
require_file "$SRC_DECK_OUT/source_matrix.json"  "REQUIRED by every kb verb; regenerate with: kb scaffold-matrix"
require_file "$SRC_DECK_OUT/cards_manifest.json" "regenerate with: kb index cards/$DECK --out out/$DECK"
require_file "$SRC_DECK_OUT/summaries.json"      "regenerate with: kb index cards/$DECK --out out/$DECK"
require_file "$SRC_DECK_OUT/INDEX.md"            "regenerate with: kb index cards/$DECK --out out/$DECK"
require_file "$SRC_DECK_OUT/sources_manifest.json"
require_file "$SRC_DECK_OUT/pdfium_provenance.json"
[ "$T" -ge 2 ] && require_file "$SRC_DECK_OUT/chunks_manifest.json" "needed for tier=verify; ~181 MB; gitignored"
if [ "$T" -ge 3 ]; then
  [ -d "$SRC_SOURCES/pdfs" ] || die "sources/$DECK/pdfs not found (needed for tier=full re-ingest)"
fi

# --------------------------------------------------------------------------
# Resolve / guard destination
# --------------------------------------------------------------------------
if [ "$DRY_RUN" -eq 1 ]; then
  # Resolve an absolute DEST for the banner WITHOUT touching the filesystem.
  case "$TARGET" in /*) TARGET_ABS="$TARGET";; *) TARGET_ABS="$PWD/$TARGET";; esac
else
  mkdir -p "$TARGET"
  TARGET_ABS="$(cd "$TARGET" && pwd)"
fi
DEST="$TARGET_ABS/$DEST_ROOT"

if [ -e "$DEST/$RECEIPT_NAME" ] && [ "$FORCE" -ne 1 ]; then
  die "dest already contains a prior export ($DEST/$RECEIPT_NAME); pass --force to overwrite"
fi
if [ -d "$DEST" ] && [ -n "$(ls -A "$DEST" 2>/dev/null)" ] && [ ! -e "$DEST/$RECEIPT_NAME" ] && [ "$FORCE" -ne 1 ]; then
  die "dest exists and is non-empty but is not a prior export ($DEST); refusing to splat. Pass --force to proceed."
fi

# Free-space precheck for the heavy tiers (STAGE lives next to DEST, so TARGET_ABS is the right FS).
estimate_kb=0
[ "$T" -ge 2 ] && estimate_kb=$(( estimate_kb + $(du -sk "$SRC_DECK_OUT/chunks_manifest.json" | cut -f1) ))
[ "$T" -ge 3 ] && estimate_kb=$(( estimate_kb + $(du -sk "$SRC_SOURCES" | cut -f1) ))
if [ "$estimate_kb" -gt 0 ] && [ "$DRY_RUN" -ne 1 ]; then
  avail_kb=$(df -Pk "$TARGET_ABS" | awk 'NR==2{print $4}')
  if [ -n "$avail_kb" ] && [ "$avail_kb" -lt "$(( estimate_kb + estimate_kb / 10 + 51200 ))" ]; then
    die "insufficient free space at $TARGET_ABS: need ~$(( estimate_kb / 1024 )) MB, have $(( avail_kb / 1024 )) MB"
  fi
fi

# --------------------------------------------------------------------------
# Staging: build into a temp dir ON THE SAME FILESYSTEM as DEST, then swap with
# a true atomic rename(2). RECEIPT_TSV lives OUTSIDE the stage so it never ships.
# (BACKUP is intentionally NOT auto-removed by the trap: if we crash between the
#  two renames, leaving the backup is the safe choice — it is the user's data.)
# --------------------------------------------------------------------------
STAGE=""
RECEIPT_TSV=""
cleanup() {
  [ -n "${STAGE:-}" ]       && rm -rf "$STAGE"       2>/dev/null || true
  [ -n "${REDERIVE_TMP:-}" ] && rm -rf "$REDERIVE_TMP" 2>/dev/null || true
  [ -n "${RECEIPT_TSV:-}" ] && rm -f  "$RECEIPT_TSV" 2>/dev/null || true
}
trap cleanup EXIT

if [ "$DRY_RUN" -eq 1 ]; then
  log "DRY RUN — no files will be written. Planned export:"
  log "  tier=$TIER deck=$DECK dest=$DEST"
  log "  cards: rsync $SRC_CARDS/ -> $DEST/cards/$DECK/ (excluding *.lock, __pycache__)"
  log "  out/$DECK: source_matrix, cards_manifest, summaries, INDEX, sources_manifest, pdfium_provenance$( [ "$COPY_SQLITE" -eq 1 ] && printf ', summaries.sqlite' )"
  [ "$COPY_SEMANTIC" -eq 1 ] && log "  out/: semantic_cache.json, semantic_cache.provenance.json"
  [ "$T" -ge 2 ] && log "  out/$DECK: chunks_manifest.json (~$(du -h "$SRC_DECK_OUT/chunks_manifest.json" | cut -f1))"
  [ "$T" -ge 3 ] && log "  sources/$DECK: pdfs/ + _registry/ (~$(du -sh "$SRC_SOURCES" | cut -f1))"
  exit 0
fi

mkdir -p "$(dirname "$DEST")"   # ensure DEST's parent exists so STAGE can share its filesystem
STAGE="$(mktemp -d "$(dirname "$DEST")/.kb-export.XXXXXX")"
RECEIPT_TSV="$(mktemp "${TMPDIR:-/tmp}/kb-export-receipt.XXXXXX")"   # path<TAB>bytes<TAB>sha256 (dest-root-relative)
: > "$RECEIPT_TSV"

# Excludes shared by every recursive copy: transient/stateful artifacts that must never travel.
RSYNC_EXCL=( --exclude '*.lock' --exclude '__pycache__' --exclude 'ingest_per_source'
             --exclude 'lint_journal.jsonl' --exclude 'v0_baseline'
             --exclude 'v1_reproducibility_status.json' )

record() {  # record <dest-relpath>  (file must already exist under $STAGE)
  local rel="$1" abs="$STAGE/$1"
  printf '%s\t%s\t%s\n' "$rel" "$(bytes_of "$abs")" "$(sha256_of "$abs")" >> "$RECEIPT_TSV"
}
copy_file() {  # copy_file <src-abs> <dest-relpath>
  local src="$1" rel="$2" dst="$STAGE/$2"
  mkdir -p "$(dirname "$dst")"
  cp -p -- "$src" "$dst"
  record "$rel"
}

log "Exporting deck=$DECK tier=$TIER -> $DEST"

# 1. Cards (recursive; rsync excludes the 113 *.lock flock sidecars + any __pycache__).
#    -L materializes any symlink to its bytes so the per-file sha256 + leak guards see a real file.
mkdir -p "$STAGE/cards/$DECK"
rsync -aL "${RSYNC_EXCL[@]}" "$SRC_CARDS/" "$STAGE/cards/$DECK/"
# Record each shipped card file individually so the integrity pass can re-hash it.
while IFS= read -r -d '' f; do
  record "cards/$DECK/${f#"$STAGE/cards/$DECK/"}"
done < <(find "$STAGE/cards/$DECK" -type f -print0)

# 2. Tier-1 manifests (explicit allowlist — NEVER a recursive copy of out/<deck>, which holds 181 MB of ingest_per_source/).
for f in source_matrix.json cards_manifest.json summaries.json INDEX.md sources_manifest.json pdfium_provenance.json; do
  copy_file "$SRC_DECK_OUT/$f" "out/$DECK/$f"
done
[ "$COPY_SQLITE" -eq 1 ] && [ -f "$SRC_DECK_OUT/summaries.sqlite" ] && copy_file "$SRC_DECK_OUT/summaries.sqlite" "out/$DECK/summaries.sqlite"
if [ "$COPY_SEMANTIC" -eq 1 ]; then
  [ -f "$SRC_OUT/semantic_cache.json" ]            && copy_file "$SRC_OUT/semantic_cache.json"            "out/semantic_cache.json"
  [ -f "$SRC_OUT/semantic_cache.provenance.json" ] && copy_file "$SRC_OUT/semantic_cache.provenance.json" "out/semantic_cache.provenance.json"
fi

# 3. Tier-2: chunks manifest (enables `kb verify`).
[ "$T" -ge 2 ] && copy_file "$SRC_DECK_OUT/chunks_manifest.json" "out/$DECK/chunks_manifest.json"
# Small reproducibility lock + frozen baseline travel with verify tier (cross-host verify needs them).
if [ "$T" -ge 2 ]; then
  [ -f "$SRC_SOURCES/_registry/v1_reproducibility_lock.json" ] && copy_file "$SRC_SOURCES/_registry/v1_reproducibility_lock.json" "out/$DECK/v1_reproducibility_lock.json"
  [ -f "$SRC_SOURCES/_registry/release_baseline/index_repro.json" ] && copy_file "$SRC_SOURCES/_registry/release_baseline/index_repro.json" "out/$DECK/index_repro.json"
fi

# 4. Tier-3: source PDFs + registry (re-ingest).
if [ "$T" -ge 3 ]; then
  mkdir -p "$STAGE/sources/$DECK"
  rsync -aL "${RSYNC_EXCL[@]}" "$SRC_SOURCES/pdfs/"     "$STAGE/sources/$DECK/pdfs/"
  [ -d "$SRC_SOURCES/_registry" ] && rsync -aL "${RSYNC_EXCL[@]}" "$SRC_SOURCES/_registry/" "$STAGE/sources/$DECK/_registry/"
fi

# 5. Permissions: source_matrix is mode 0600 on disk; the consumer needs to read it. _registry holds 0600 files too.
chmod 0644 "$STAGE/out/$DECK/source_matrix.json"
find "$STAGE" -type f ! -perm -u+r -exec chmod u+r {} + 2>/dev/null || true
find "$STAGE" -type d -exec chmod u+rwx {} + 2>/dev/null || true

# 6. Primer at dest-root (humanize's next-round prompt looks for <dest-root>/INDEX.md).
cp -p -- "$SRC_DECK_OUT/INDEX.md" "$STAGE/INDEX.md"
record "INDEX.md"

# 7. Optional kb binary.
KB_BIN="$(command -v kb || true)"
if [ -z "$KB_BIN" ]; then
  for c in "$KB_REPO_ROOT/target/release/kb" "$KB_REPO_ROOT/target/debug/kb"; do [ -x "$c" ] && { KB_BIN="$c"; break; }; done
fi
KB_VER="unknown"; KB_SHA="unknown"
if [ -n "$KB_BIN" ]; then
  KB_VER="$("$KB_BIN" --version 2>/dev/null || echo unknown)"
  KB_SHA="$(sha256_of "$KB_BIN")"
fi
if [ "$COPY_BINARY" -eq 1 ]; then
  [ -n "$KB_BIN" ] || die "--with-binary requested but no kb binary found (build with: cargo build --release -p cacg-cli)"
  mkdir -p "$STAGE/bin"; cp -p -- "$KB_BIN" "$STAGE/bin/kb"; chmod 0755 "$STAGE/bin/kb"; record "bin/kb"
  warn "shipped kb binary is platform-specific ($(uname -s)/$(uname -m)); consumers on other platforms must rebuild"
fi

# --------------------------------------------------------------------------
# Provenance facts (consumed by README + the receipt).
# --------------------------------------------------------------------------
SRC_COMMIT="$(git -C "$KB_REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)"
SRC_COMMIT_SHORT="$(git -C "$KB_REPO_ROOT" rev-parse --short HEAD 2>/dev/null || echo unknown)"
# tracked_dirty covers tracked + UNtracked working-tree changes (incl. this script if uncommitted).
# It does NOT certify the exported payload: source_matrix.json / chunks_manifest.json / summaries.sqlite
# are gitignored, so git cannot diff them against HEAD — the re-derivation gate below is what relates
# the shipped cards to the shipped manifest.
GIT_DIRTY=false
[ -n "$(git -C "$KB_REPO_ROOT" status --porcelain 2>/dev/null)" ] && GIT_DIRTY=true
CARD_COUNT_DISK="$(find "$STAGE/cards/$DECK" -name '*.md' | wc -l | tr -d ' ')"
CARD_COUNT_MANIFEST="$(jq '.cards | length' "$STAGE/out/$DECK/cards_manifest.json")"

# kb-query.sh — cd's into dest-root so the manifest `path` fields (cards/<deck>/...) resolve.
cat > "$STAGE/kb-query.sh" <<QUERY_EOF
#!/usr/bin/env bash
# Auto-generated by export-knowledge.sh — query this exported CACG deck.
set -euo pipefail
ROOT="\$(cd "\$(dirname "\${BASH_SOURCE[0]:-\$0}")" && pwd)"
cd "\$ROOT"   # load-bearing: cards_manifest 'path' fields are resolved relative to CWD
KB="\${KB_BIN:-kb}"
DECK="$DECK"
SM="out/\$DECK/source_matrix.json"
CM="out/\$DECK/cards_manifest.json"
SUM="out/\$DECK/summaries.json"
CH="out/\$DECK/chunks_manifest.json"
cmd="\${1:-}"; shift || true
case "\$cmd" in
  search) exec "\$KB" search --source-matrix "\$SM" --summaries "\$SUM" "\$@" ;;
  show)   exec "\$KB" show   --source-matrix "\$SM" --cards-manifest "\$CM" "\$@" ;;
  verify) [ -f "\$CH" ] || { echo "verify needs tier>=verify (chunks_manifest.json absent)" >&2; exit 2; }
          exec "\$KB" verify --source-matrix "\$SM" --chunks-manifest "\$CH" "\$@" ;;
  *) echo "usage: kb-query.sh {search <query> [--top-k N] [--json] | show <card_id> | verify <card.md|--round-summary FILE>}" >&2; exit 2 ;;
esac
QUERY_EOF
chmod +x "$STAGE/kb-query.sh"
record "kb-query.sh"

# README.md — consumer guide.
cat > "$STAGE/README.md" <<README_EOF
# CACG Knowledge Export — deck \`$DECK\` (tier: $TIER)

Exported from \`$KB_REPO_ROOT\` @ \`$SRC_COMMIT_SHORT\` by \`scripts/export-knowledge.sh\`.
$CARD_COUNT_DISK cards. See \`$RECEIPT_NAME\` for the full file receipt (sizes + sha256).

## Layout
\`\`\`
$DEST_ROOT/
  INDEX.md              card index / primer (grouped by reading_id)
  cards/$DECK/<reading_id>/<card_id>.md (+ .history.jsonl)
  out/$DECK/            source_matrix.json, cards_manifest.json, summaries.json, INDEX.md, ...
$( [ "$COPY_SEMANTIC" -eq 1 ] && printf '  out/semantic_cache.json (+ provenance)' )
$( [ "$T" -ge 2 ] && printf '  out/%s/chunks_manifest.json   (enables kb verify)' "$DECK" )
$( [ "$T" -ge 3 ] && printf '  sources/%s/pdfs + _registry  (enables re-ingest)' "$DECK" )
  kb-query.sh           query wrapper (search / show / verify)
\`\`\`

## Querying

Build the \`kb\` binary once (Rust toolchain required), then use the wrapper:
\`\`\`bash
cargo build --release -p cacg-cli   # in the CACG repo; produces target/release/kb (reports "$EXPECTED_KB_VERSION")
export KB_BIN=/path/to/target/release/kb   # or put 'kb' on PATH

cd $DEST_ROOT
./kb-query.sh search "duration convexity" --top-k 5 --json
./kb-query.sh show fi-duration-and-convexity
$( [ "$T" -ge 2 ] && printf './kb-query.sh verify cards/%s/<reading>/<card>.md' "$DECK" )
\`\`\`

> The wrapper \`cd\`s into this directory because \`cards_manifest.json\` stores
> repo-relative card paths (\`cards/$DECK/...\`) that \`kb show\` resolves against the CWD.

## Integrity (run all commands from this directory, $DEST_ROOT)

\`\`\`bash
# Re-hash every exported file against the receipt (macOS: replace 'sha256sum -c -' with 'shasum -a 256 -c -'):
jq -r '.files[] | "\(.sha256)  \(.path)"' $RECEIPT_NAME | sha256sum -c -
# Re-derive the manifests from the cards and diff (needs the pinned kb binary):
KB_FROZEN_CLOCK=1 kb index cards/$DECK --out /tmp/kbchk && diff <(jq -S . out/$DECK/cards_manifest.json) <(jq -S . /tmp/kbchk/cards_manifest.json)
\`\`\`
$( [ "$T" -ge 2 ] && cat <<'TIER2NOTE'

## Tier-2 (verify) caveat

`chunks_manifest.json` chunk hashes are tied to the exact `libpdfium` that
parsed the source PDFs. `kb verify` on a host with a divergent libpdfium can
fail unless the bundled equivalence proof applies or you re-ingest the PDFs
(tier=full). See `out/<deck>/v1_reproducibility_lock.json`.
TIER2NOTE
)

## humanize integration

This export lands \`INDEX.md\` at \`$DEST_ROOT/INDEX.md\`, where the humanize
RLCR harness expects its knowledge primer. With the companion humanize hook
(\`hooks/kb-knowledge-route.sh\`) and \`kb_enabled: true\` in \`.humanize/config.json\`,
each prompt auto-routes matching cards. Round summaries cite consulted cards in a
\`## Knowledge Consulted\` section, which \`kb verify --round-summary\` can content-check.
README_EOF
record "README.md"

# Receipt is generated LAST — after kb-query.sh + README are recorded — so files[]
# captures every shipped file. EXPORT_MANIFEST.json is the only artifact whose own
# hash cannot appear in files[] (it would be self-referential), so it is emitted here.
jq -n \
  --arg schema "cacg.export.v1" \
  --arg tier "$TIER" --arg deck "$DECK" --arg dest_root "$DEST_ROOT" \
  --arg src_repo "$KB_REPO_ROOT" --arg commit "$SRC_COMMIT" --arg commit_short "$SRC_COMMIT_SHORT" \
  --argjson tracked_dirty "$GIT_DIRTY" \
  --arg kb_ver "$KB_VER" --arg kb_sha "$KB_SHA" --arg kb_expected "$EXPECTED_KB_VERSION" \
  --arg uname "$(uname -s)/$(uname -m)" \
  --argjson cards_disk "$CARD_COUNT_DISK" --argjson cards_manifest "$CARD_COUNT_MANIFEST" \
  --rawfile tsv "$RECEIPT_TSV" \
  '{schema_version:$schema, tier:$tier, deck:$deck, dest_root:$dest_root,
    source:{repo:$src_repo, commit:$commit, commit_short:$commit_short, tracked_dirty:$tracked_dirty},
    kb_binary:{observed_version:$kb_ver, observed_sha256:$kb_sha, expected_version:$kb_expected, host:$uname},
    counts:{cards_on_disk:$cards_disk, cards_in_manifest:$cards_manifest},
    files:($tsv | rtrimstr("\n") | split("\n") | map(select(length>0) | split("\t") | {path:.[0], bytes:(.[1]|tonumber), sha256:.[2]}))}' \
  > "$STAGE/$RECEIPT_NAME"

# --------------------------------------------------------------------------
# Integrity checks (on the staged tree, before swap)
# --------------------------------------------------------------------------
log "Running integrity checks"
fail=0

# (a) No transient/stateful artifact leaked through any copy path (detector + diagnostic use the SAME patterns).
LEAK_FIND=( \( -name '*.lock' -o -name '__pycache__' -o -name 'ingest_per_source' \
   -o -name 'lint_journal.jsonl' -o -name 'v0_baseline' -o -name 'v1_reproducibility_status.json' \) )
if find "$STAGE" "${LEAK_FIND[@]}" -print -quit | grep -q .; then
  warn "excluded artifact leaked into export:"; find "$STAGE" "${LEAK_FIND[@]}" -print >&2; fail=1
else ok "no transient/lock/bloat artifacts present"; fi

# (a2) No symlink survived into the export (rsync -aL should have materialized them; this is the type-based backstop).
if find "$STAGE" -type l -print -quit | grep -q .; then
  warn "symlink(s) present in staged export:"; find "$STAGE" -type l -print >&2; fail=1
else ok "no symlinks in export"; fi

# (b) Per-file sha256 (authoritative copy-integrity gate). IFS-safe over tab-delimited receipt.
while IFS=$'\t' read -r rel _bytes sha; do
  [ -n "$rel" ] || continue
  [ "$(sha256_of "$STAGE/$rel")" = "$sha" ] || { warn "sha256 mismatch: $rel"; fail=1; }
done < "$RECEIPT_TSV"
[ "$fail" -eq 0 ] && ok "all $(wc -l < "$RECEIPT_TSV" | tr -d ' ') files match their recorded sha256"

# (c) Card-set parity: every card path in the manifest must be shipped on disk.
#     Extra on-disk .md (retracted cards keep their .md) are allowed, so this is a one-way membership check.
missing_cards=0
while IFS= read -r p; do
  [ -f "$STAGE/$p" ] || { warn "manifest references a card with no shipped .md: $p"; missing_cards=1; }
done < <(jq -r '.cards[].path' "$STAGE/out/$DECK/cards_manifest.json")
[ "$missing_cards" -eq 0 ] \
  && ok "card-set parity (all $CARD_COUNT_MANIFEST manifested cards shipped; $CARD_COUNT_DISK .md on disk)" \
  || fail=1

# (d) source_matrix authorizes every shipped reading dir (anchored key match, no prefix false-match).
for d in "$STAGE/cards/$DECK"/*/; do
  rid="$(basename "$d")"
  jq -e --arg r "$rid" '.allowed | has($r)' "$STAGE/out/$DECK/source_matrix.json" >/dev/null \
    || { warn "source_matrix.allowed is missing reading_id: $rid"; fail=1; }
done
[ "$fail" -eq 0 ] && ok "source_matrix authorizes all shipped reading dirs"

# (e) schema sanity.
jq -e '.schema_version == "cacg.v0"' "$STAGE/out/$DECK/summaries.json" >/dev/null \
  && ok "summaries.json schema_version is cacg.v0" || { warn "unexpected schema_version in summaries.json"; fail=1; }

# (f) Optional hard re-derivation gate: regenerate cards_manifest from a *copy* of the cards
#     (never the export itself) and require byte-equality. Skipped unless the pinned binary is present.
if [ -n "$KB_BIN" ] && [ "$KB_VER" = "$EXPECTED_KB_VERSION" ]; then
  REDERIVE_TMP="$(mktemp -d "${TMPDIR:-/tmp}/kb-rederive.XXXXXX")"
  cp -a "$STAGE/cards" "$REDERIVE_TMP/cards"
  if ( cd "$REDERIVE_TMP" && KB_FROZEN_CLOCK=1 "$KB_BIN" index "cards/$DECK" --out out >/dev/null 2>&1 ); then
    if [ "$(sha256_of "$REDERIVE_TMP/out/cards_manifest.json")" = "$(sha256_of "$STAGE/out/$DECK/cards_manifest.json")" ]; then
      ok "re-derived cards_manifest.json is byte-identical to the export (kb index reproducibility)"
    else
      warn "re-derived cards_manifest.json differs from the export — manifests may be stale vs cards"; fail=1
    fi
  else
    warn "kb index re-derivation could not run; skipping reproducibility gate (non-fatal)"
  fi
  rm -rf "$REDERIVE_TMP"; REDERIVE_TMP=""
else
  log "NOTE: kb binary absent or version != '$EXPECTED_KB_VERSION'; skipped re-derivation gate"
fi

[ "$fail" -eq 0 ] || die "integrity checks FAILED — export NOT published (staging discarded)"

# --------------------------------------------------------------------------
# Atomic publish (STAGE shares DEST's filesystem, so both mv calls are rename(2)).
# --------------------------------------------------------------------------
if [ -d "$DEST" ]; then
  BACKUP="$DEST.bak.$$"
  mv "$DEST" "$BACKUP"                          # DEST now absent
  if mv "$STAGE" "$DEST"; then
    STAGE=""; rm -rf "$BACKUP"
  else
    rm -rf "$DEST"                              # drop any partial DEST so the restore truly replaces it
    mv "$BACKUP" "$DEST" || die "publish failed AND restore failed; previous export preserved at $BACKUP"
    die "publish failed; restored previous dest from backup"
  fi
else
  mv "$STAGE" "$DEST"; STAGE=""
fi
chmod -R u+rwX,go+rX "$DEST" 2>/dev/null || true

echo ""
ok "Export complete"
log "deck=$DECK tier=$TIER cards=$CARD_COUNT_DISK dest=$DEST"
log "receipt: $DEST/$RECEIPT_NAME   primer: $DEST/INDEX.md   query: $DEST/kb-query.sh"
