#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../../.."

export KB_FROZEN_CLOCK="${KB_FROZEN_CLOCK:-1}"
case ":${LD_LIBRARY_PATH:-}:" in
  *:/usr/lib:*) ;;
  *) export LD_LIBRARY_PATH="/usr/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" ;;
esac

exec python3 sources/cfa/_registry/run_ingest_per_source.py "$@"
