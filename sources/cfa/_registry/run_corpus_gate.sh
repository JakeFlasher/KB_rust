#!/usr/bin/env bash
# CFA-legacy v0 corpus gate (re-runnable, deterministic under
# KB_FROZEN_CLOCK=1). Four checks, fail-closed:
#
#   1. Scope ledger + count reconciliation (build_scope_ledger.py).
#   2. Full-corpus Layer-1 lint into a FRESH journal; assert exit 0 and
#      exactly one journal entry per emitted card.
#   3. Per-card Layer-2 verify over every cards_manifest.json path
#      (verify has no --cards-dir, so this is an external loop); capture
#      a pass/fail tally that treats ANY non-zero per-card exit as a
#      failure (no masking). Parallelism is tunable via GATE_PARALLELISM.
#
# Exits non-zero if any check fails. The production Rust `kb` binary is
# the sole verification authority.
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$REPO"

KB="./target/debug/kb"
CARDS_DIR="cards/cfa"
CHUNKS="out/cfa/chunks_manifest.json"
MATRIX="out/cfa/source_matrix.json"
MANIFEST="out/cfa/cards_manifest.json"
BASE="sources/cfa/_registry/release_baseline"
LINT_JOURNAL="$BASE/lint_journal_baseline.jsonl"
VERIFY_TALLY="$BASE/verify_baseline_tally.json"
VERIFY_FAILS="$BASE/verify_baseline_failures.txt"
PAR="${GATE_PARALLELISM:-4}"

mkdir -p "$BASE"
export KB_FROZEN_CLOCK=1
status=0

echo "== [1/4] scope ledger + count reconciliation =="
python3 sources/cfa/_registry/build_scope_ledger.py || status=1

echo
echo "== [2/4] full-corpus Layer-1 lint (fresh journal) =="
rm -f "$LINT_JOURNAL"
if "$KB" lint --all-readings --cards-dir "$CARDS_DIR" \
      --chunks-manifest "$CHUNKS" --source-matrix "$MATRIX" \
      --journal "$LINT_JOURNAL"; then
  echo "lint: exit 0 (clean)"
else
  echo "lint: NON-ZERO EXIT" >&2
  status=1
fi
disk_cards=$(find "$CARDS_DIR" -name '*.md' ! -name '*.history*' ! -name '_*' ! -name 'INDEX.md' | wc -l | tr -d ' ')
journal_entries=$(wc -l < "$LINT_JOURNAL" 2>/dev/null | tr -d ' ')
journal_entries=${journal_entries:-0}
echo "lint journal entries=$journal_entries; emitted cards on disk=$disk_cards"
if [ "$journal_entries" != "$disk_cards" ]; then
  echo "lint journal entry count != emitted-card count (expected one entry per card)" >&2
  status=1
fi

echo
echo "== [3/4] per-card Layer-2 verify (parallel=$PAR) =="
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

verify_one() {
  local p="$1"
  # Each verify writes to its OWN throwaway journal: the parallel loop must not
  # share (and race on) the default audit journal, and a re-run must not depend
  # on the state of any persistent journal. verify is a read-only check here.
  local j; j="$(mktemp -u)"
  env KB_FROZEN_CLOCK=1 ./target/debug/kb verify "$p" \
      --chunks-manifest "out/cfa/chunks_manifest.json" \
      --source-matrix "out/cfa/source_matrix.json" \
      --journal "$j" >/dev/null 2>&1
  local rc=$?
  rm -f "$j"
  if [ "$rc" -eq 0 ]; then
    printf 'PASS\t%s\n' "$p"
  else
    printf 'FAIL\t%d\t%s\n' "$rc" "$p"
  fi
}
export -f verify_one

jq -r '.cards[].path' "$MANIFEST" | sort > "$tmpdir/paths.txt"
total=$(wc -l < "$tmpdir/paths.txt" | tr -d ' ')
xargs -P "$PAR" -I{} bash -c 'verify_one "$@"' _ {} < "$tmpdir/paths.txt" > "$tmpdir/results.txt" || true

passes=$(grep -c '^PASS' "$tmpdir/results.txt" || true)
fails=$(grep -c '^FAIL' "$tmpdir/results.txt" || true)
passes=${passes:-0}; fails=${fails:-0}
grep '^FAIL' "$tmpdir/results.txt" > "$VERIFY_FAILS" 2>/dev/null || : > "$VERIFY_FAILS"

jq -n \
  --argjson total "$total" \
  --argjson passes "$passes" \
  --argjson fails "$fails" \
  '{schema_version:"cfa_verify_baseline/v1",
    total:$total, passes:$passes, failures:$fails,
    all_pass:($fails==0)}' > "$VERIFY_TALLY"

echo "verify tally: total=$total passes=$passes failures=$fails"
if [ "$total" != "$disk_cards" ]; then
  echo "verify loop covered $total cards but $disk_cards are on disk" >&2
  status=1
fi
if [ "$fails" -ne 0 ]; then
  echo "verify: $fails card(s) failed (see $VERIFY_FAILS)" >&2
  status=1
else
  echo "verify: all $passes cards passed"
fi

echo
echo "== [4/4] notes-taint quarantine invariant =="
if python3 sources/cfa/_registry/check_quarantine_invariant.py; then
  echo "quarantine invariant: PASS"
else
  echo "quarantine invariant: FAIL" >&2
  status=1
fi

echo
if [ "$status" -eq 0 ]; then
  echo "CORPUS GATE: PASS"
else
  echo "CORPUS GATE: FAIL" >&2
fi
exit "$status"
