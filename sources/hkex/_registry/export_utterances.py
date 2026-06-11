#!/usr/bin/env python3
r"""Export the corpus as a `cacg.utterances.v1` JSONL stream.

The stream is the input contract of the native conversational ingest
backend (`kb ingest --format utterances`), which replaces the
synthetic-PDF detour: one utterance per logical page, no pdfium, no
fonts, and the post/comment/speaker anchors carried OUT of band in a
sealed `locator_map.json` instead of in-band `@@…@@` text markers.

Reads `corpus_complete.md` through the same `corpus_model` parser the
renderer uses, so the utterance set is identical to the rendered-PDF
page set (same ordering, same texts) — the two backends describe the
same corpus.

  --write [--out PATH]   write the JSONL stream (default:
                         _research/xueqiu_goubujiao/corpus_utterances.jsonl)
  --self-test            hermetic checks of the record shape
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
from corpus_model import parse, CORPUS  # noqa: E402  # pyright: ignore[reportMissingImports]

DEFAULT_OUT = ROOT / "_research/xueqiu_goubujiao/corpus_utterances.jsonl"
SCHEMA_VERSION = "cacg.utterances.v1"


def utterance_record(ordinal: int, u) -> dict:
    refs = {"post_id": u.post_id}
    if u.comment_id:
        refs["comment_id"] = u.comment_id
    rec = {
        "ordinal": ordinal,
        "utterance_id": (f"c{u.comment_id}" if u.comment_id else f"p{u.post_id}"),
        "speaker": "狗不叫" if u.is_author else "commenter",
        "is_author": u.is_author,
        "refs": refs,
        "text": u.text,
    }
    return rec


def render_stream(utts) -> tuple[str, int]:
    """Render the JSONL stream; returns (text, skipped_empty_count).

    Empty-text utterances (e.g. a bare `回复@user:` reply) are SKIPPED,
    not exported: the backend's contract requires non-empty text (an
    empty utterance can never be cited and carries no readable
    context), and the skip is counted in the header metadata so the
    omission is recorded, never silent."""
    seen: dict[str, int] = {}
    records = []
    skipped = 0
    for u in utts:
        if not u.text.strip():
            skipped += 1
            continue
        rec = utterance_record(len(records) + 1, u)
        # The same comment can legitimately render under several posts
        # (cross-post duplicates); utterance_id must still be unique, so
        # duplicates get a stable `#<n>` occurrence suffix.
        base = rec["utterance_id"]
        n = seen.get(base, 0)
        seen[base] = n + 1
        if n:
            rec["utterance_id"] = f"{base}#{n}"
        records.append(rec)
    header = {"schema_version": SCHEMA_VERSION, "source_kind": "conversation",
              "metadata": {"generator": "sources/hkex/_registry/export_utterances.py",
                           "corpus": "goubujiao xueqiu corpus",
                           "skipped_empty_utterances": skipped}}
    lines = [json.dumps(header, ensure_ascii=False, sort_keys=True)]
    lines.extend(json.dumps(r, ensure_ascii=False, sort_keys=True) for r in records)
    return "\n".join(lines) + "\n", skipped


def do_write(out: Path) -> int:
    utts = parse(CORPUS)
    text, skipped = render_stream(utts)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8")
    print(json.dumps({
        "out": str(out),
        "utterances": len(utts) - skipped,
        "skipped_empty": skipped,
        "sha256": hashlib.sha256(text.encode()).hexdigest(),
        "author_utterances": sum(1 for u in utts if u.is_author and u.text.strip()),
    }, ensure_ascii=False, indent=2))
    return 0


def _self_test() -> int:
    from types import SimpleNamespace
    failures = []
    post = SimpleNamespace(post_id="1", comment_id=None, is_author=True, text="正文")
    com = SimpleNamespace(post_id="1", comment_id="9", is_author=False, text="评论")
    empty = SimpleNamespace(post_id="1", comment_id="10", is_author=False, text="  ")
    s, skipped = render_stream([post, empty, com, com])
    lines = s.strip().splitlines()
    hdr = json.loads(lines[0])
    if hdr["schema_version"] != SCHEMA_VERSION:
        failures.append("header schema_version wrong")
    if skipped != 1 or hdr["metadata"]["skipped_empty_utterances"] != 1:
        failures.append("empty-text utterance must be skipped and counted")
    if len(lines) != 4:
        failures.append(f"expected header + 3 records, got {len(lines)} lines")
    r1, r2, r3 = (json.loads(x) for x in lines[1:])
    if (r1["ordinal"], r2["ordinal"], r3["ordinal"]) != (1, 2, 3):
        failures.append("ordinals not contiguous")
    if r1["utterance_id"] != "p1" or r2["utterance_id"] != "c9":
        failures.append(f"utterance ids wrong: {r1['utterance_id']}, {r2['utterance_id']}")
    if r3["utterance_id"] != "c9#1":
        failures.append(f"duplicate occurrence suffix wrong: {r3['utterance_id']}")
    if r1["is_author"] is not True or r2["is_author"] is not False:
        failures.append("author flags wrong")
    if r2["refs"] != {"post_id": "1", "comment_id": "9"}:
        failures.append(f"refs wrong: {r2['refs']}")
    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (export_utterances: header + ordinals + ids + dup suffix)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--write", action="store_true")
    g.add_argument("--self-test", action="store_true")
    ap.add_argument("--out", type=Path, default=DEFAULT_OUT)
    args = ap.parse_args()
    if args.self_test:
        return _self_test()
    return do_write(args.out)


if __name__ == "__main__":
    raise SystemExit(main())
