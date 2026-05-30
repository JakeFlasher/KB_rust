#!/usr/bin/env bash
# Pre-ingest verification for the microstructure / performance / fund-level
# migration. Re-runnable and fail-closed: runs both gates' self-tests and their
# real checks BEFORE any PDF is copied or ingested. It does NOT mutate the
# released corpus and is safe to run on a clean checkout (the libpdfium binary
# and the merged chunks manifest are local-only artifacts; their absence is a
# reported note here, and is promoted to a hard failure by the reproducibility
# lock's --require-ingest-ready mode that gates the actual ingest step).
#
# Steps, fail-closed:
#   1. incoming-skeleton preflight self-test (hermetic).
#   2. incoming-skeleton preflight real run (134 skeletons structurally migratable).
#   3. reproducibility-lock self-test (hermetic recipe + tamper + status classes).
#   4. reproducibility-lock real check (pdfium-render + recorded chunk-hash recompute).
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$REPO"
REG="sources/cfa_legacy/_registry"
export KB_FROZEN_CLOCK=1
status=0

echo "== [1/4] incoming-skeleton preflight self-test =="
python3 "$REG/build_incoming_skeleton_preflight.py" --self-test || status=1

echo
echo "== [2/4] incoming-skeleton preflight (134 skeletons) =="
python3 "$REG/build_incoming_skeleton_preflight.py" --report || status=1

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
