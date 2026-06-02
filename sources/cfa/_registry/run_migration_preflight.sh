#!/usr/bin/env bash
# Pre-ingest verification for an incoming-skeleton migration. Re-runnable and
# fail-closed: runs both gates' self-tests and their real checks BEFORE any PDF is
# copied or ingested. It does NOT mutate the released corpus and is safe to run on a
# clean checkout (the libpdfium binary and the merged chunks manifest are local-only
# artifacts; their absence is a reported note here, and is promoted to a hard failure
# by the reproducibility lock's --require-ingest-ready mode that gates the ingest step).
#
# The preflight is scoped to the CURRENTLY-INCOMING decks (default: the 10/11
# template decks, 92 = 64 be- + 28 rm-); already-migrated sets (14/15/22) collide
# with their now-live cards by design, so they are not part of this run. Override the
# scope with the MIGRATION_READINGS env var (comma-separated reading_ids).
#
# Steps, fail-closed:
#   1. incoming-skeleton preflight self-test (hermetic).
#   2. incoming-skeleton preflight real run (the incoming skeletons are structurally migratable).
#   3. reproducibility-lock self-test (hermetic recipe + tamper + status classes).
#   4. reproducibility-lock real check (pdfium-render + recorded chunk-hash recompute).
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$REPO"
REG="sources/cfa/_registry"
export KB_FROZEN_CLOCK=1
MIGRATION_READINGS="${MIGRATION_READINGS:-10_behavioral_finance,11_risk_management}"
status=0

echo "== [1/4] incoming-skeleton preflight self-test =="
python3 "$REG/build_incoming_skeleton_preflight.py" --self-test || status=1

echo
echo "== [2/4] incoming-skeleton preflight (incoming decks: $MIGRATION_READINGS) =="
python3 "$REG/build_incoming_skeleton_preflight.py" --report --readings "$MIGRATION_READINGS" || status=1

echo
echo "== [3/4] reproducibility-lock self-test =="
python3 "$REG/check_ingest_reproducibility_lock.py" --self-test || status=1

echo
echo "== [4/4] reproducibility-lock check =="
python3 "$REG/check_ingest_reproducibility_lock.py" || status=1

echo
if [ "$status" -eq 0 ]; then
  echo "== migration preflight PASS =="
else
  echo "== migration preflight FAIL ==" >&2
fi
exit "$status"
