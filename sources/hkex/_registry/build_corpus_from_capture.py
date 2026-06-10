#!/usr/bin/env python3
r"""Deterministic corpus builder: crawler capture -> corpus_complete.md.

This script is the previously-missing FIRST provenance hop. It consumes the
xueqiu crawler's `output/<uid>/` capture directory (the inter-repo exchange
format) and deterministically emits the HN-style threaded corpus markdown that
`corpus_model.py` parses and `render_corpus_pdf.py` renders, plus a committed
provenance record carrying the crawler's own completeness attestation.

Exchange contract consumed (fail-closed on violations):

  <capture-dir>/posts.json   list[post]; post = {id, created_at(ms epoch),
                             created_at_iso, like_count, reply_count,
                             retweet_count, text(no newlines), url, user,
                             comments: list[comment]}; comment = {id,
                             created_at_iso, like_count, text(no newlines),
                             user{id, screen_name}, reply_to{screen_name}|None,
                             reply_to_id|None, reply_to_text|None, root_id}
  <capture-dir>/meta.json    crawl accounting: target uid, complete flag,
                             run_status, status_counts, timeline population /
                             fetched / shortfall, cap_causes, incomplete_posts

Completeness is attested, never assumed: when `meta.json` says the capture is
incomplete the builder REFUSES to run unless `--allow-incomplete` is given,
and either way the full shortfall accounting is copied into the provenance
record. The emitted header states the attested status instead of claiming a
"complete" corpus.

Rendering rules (reverse-engineered from the proven corpus and verified
against it utterance-by-utterance before this script existed):

  - posts sorted ascending by (created_at, id); timestamps rendered in
    Beijing time (UTC+8) as `YYYY-MM-DD HH:MM`
  - post section: `## POST <pid> | <ts> BJ | <like> <reply> <retweet>` /
    `URL: <url>` / blank / `<author-tag> [POST <pid>]: <text>` / blank /
    (`### Comments (<rendered rows>):` + thread lines | one extra blank);
    sections separated by one blank line; no trailing blank at EOF
  - comment forest per post: parent = reply_to_id when that comment id exists
    in the SAME post's comment set, else the comment renders top-level;
    siblings sorted ascending by (created_at_iso, id); indent = 2 spaces per
    depth; the same logical comment may legitimately render under several
    posts (the capture lists it under each; never deduplicated here)
  - thread line: `- <who> . <ts> . <likes> . [c<cid>][ (ctx)]: <text>` where
    who = `<author-tag>` when user.id == author uid else `@<screen_name>`
  - context parenthetical `(in reply to @X: "...")` only when the parent
    comment is NOT in this post's set and reply_to_text is non-empty;
    reply_to_text longer than 70 chars truncates to 70 + ellipsis

Modes:
  --write              build the corpus + write the provenance record
  --check              rebuild to a temp file; assert SHA equality with the
                       existing corpus AND the committed provenance
  --self-test          hermetic checks of the pure helpers
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
import tempfile
from datetime import datetime, timedelta, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
DEFAULT_OUT = ROOT / "_research/xueqiu_goubujiao/corpus_complete.md"
DEFAULT_PROVENANCE = ROOT / "sources/hkex/_registry/corpus_provenance.json"

BJ = timezone(timedelta(hours=8))
CTX_TRUNCATE_AT = 70
SCHEMA_VERSION = "hkex.corpus_builder.v1"

POST_REQUIRED = ("id", "created_at", "created_at_iso", "like_count",
                 "reply_count", "retweet_count", "text", "url", "comments")
COMMENT_REQUIRED = ("id", "created_at_iso", "like_count", "text", "user")


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def bj_minute(iso_or_ms) -> str:
    """Render a capture timestamp as Beijing-time `YYYY-MM-DD HH:MM`."""
    if isinstance(iso_or_ms, (int, float)):
        dt = datetime.fromtimestamp(iso_or_ms / 1000, tz=timezone.utc)
    else:
        dt = datetime.fromisoformat(str(iso_or_ms))
        if dt.tzinfo is None:
            raise SystemExit(f"naive timestamp in capture (no tz): {iso_or_ms!r}")
    return dt.astimezone(BJ).strftime("%Y-%m-%d %H:%M")


def ctx_quote(reply_to_text: str) -> str:
    if len(reply_to_text) > CTX_TRUNCATE_AT:
        return reply_to_text[:CTX_TRUNCATE_AT] + "…"
    return reply_to_text


def require(obj: dict, keys: tuple[str, ...], what: str) -> None:
    missing = [k for k in keys if k not in obj]
    if missing:
        raise SystemExit(f"capture contract violation: {what} missing keys {missing}")


def assert_renderable_text(text: str, what: str) -> None:
    """A corpus line is one physical line; embedded newlines would silently
    corrupt the line grammar `corpus_model.py` fails closed on."""
    if "\n" in text or "\r" in text:
        raise SystemExit(f"capture contract violation: newline inside {what} text")


def speaker_tag(comment: dict, author_uid: int, author_tag: str, where: str) -> str:
    user = comment.get("user") or {}
    uid = user.get("id")
    if uid == author_uid:
        return author_tag
    name = user.get("screen_name")
    if not name:
        raise SystemExit(f"capture contract violation: comment {where} has no screen_name")
    if any(ch.isspace() for ch in name):
        # `corpus_model._THREAD` matches `@\S+`; a spaced name would corrupt
        # the grammar. Fail closed rather than silently mangle the speaker.
        raise SystemExit(f"capture contract violation: spaced screen_name {name!r} at {where}")
    return f"@{name}"


def build_forest(comments: list[dict]) -> tuple[list[dict], dict[str, list[dict]]]:
    """Return (roots, children-by-cid). Parent = reply_to_id when present in
    THIS post's comment set; cycle membership fails closed."""
    by_id: dict[str, dict] = {}
    for c in comments:
        cid = str(c["id"])
        if cid in by_id:
            raise SystemExit(f"capture contract violation: duplicate comment id {cid} within one post")
        by_id[cid] = c

    def sort_key(c: dict):
        return (datetime.fromisoformat(str(c["created_at_iso"])), int(c["id"]))

    roots: list[dict] = []
    children: dict[str, list[dict]] = {}
    for c in comments:
        rid = c.get("reply_to_id")
        rid = str(rid) if rid else None
        if rid and rid in by_id:
            children.setdefault(rid, []).append(c)
        else:
            roots.append(c)
    roots.sort(key=sort_key)
    for v in children.values():
        v.sort(key=sort_key)

    # Cycle guard: every comment must be reachable from a root.
    reachable = 0
    stack = [str(c["id"]) for c in roots]
    seen: set[str] = set()
    while stack:
        cid = stack.pop()
        if cid in seen:
            raise SystemExit(f"capture contract violation: reply cycle at comment {cid}")
        seen.add(cid)
        reachable += 1
        stack.extend(str(k["id"]) for k in children.get(cid, []))
    if reachable != len(comments):
        orphaned = sorted(set(by_id) - seen)
        raise SystemExit(
            f"capture contract violation: {len(comments) - reachable} comment(s) in a "
            f"parent cycle unreachable from any root: {orphaned[:5]}"
        )
    return roots, children


def render_thread_line(c: dict, depth: int, parent_present: bool,
                       author_uid: int, author_tag: str, post_id: str,
                       stats: dict) -> str:
    cid = str(c["id"])
    where = f"post {post_id} comment {cid}"
    assert_renderable_text(c["text"], where)
    who = speaker_tag(c, author_uid, author_tag, where)
    ts = bj_minute(c["created_at_iso"])
    ctx = ""
    reply_to_text = c.get("reply_to_text") or ""
    if not parent_present and c.get("reply_to_id") and reply_to_text:
        rt_user = (c.get("reply_to") or {}).get("screen_name")
        if not rt_user:
            raise SystemExit(f"capture contract violation: {where} has orphan-parent "
                             f"context text but no reply_to.screen_name")
        assert_renderable_text(reply_to_text, f"{where} reply_to_text")
        quoted = ctx_quote(reply_to_text)
        if quoted != reply_to_text:
            stats["context_truncations"] += 1
        ctx = f' (↪in reply to @{rt_user}: "{quoted}")'
        stats["context_parentheticals"] += 1
    return f'{"  " * depth}- {who} · {ts} · 👍{c["like_count"]} · [c{cid}]{ctx}: {c["text"]}'


def render_post_section(post: dict, author_uid: int, author_tag: str,
                        stats: dict) -> list[str]:
    require(post, POST_REQUIRED, f"post {post.get('id')}")
    pid = str(post["id"])
    assert_renderable_text(post["text"], f"post {pid} body")
    head = (f"## POST {pid} | {bj_minute(post['created_at'])} BJ | "
            f"👍{post['like_count']} 💬{post['reply_count']} 🔁{post['retweet_count']}")
    lines = [head, f"URL: {post['url']}", "", f"★AUTHOR_TAG [POST {pid}]: {post['text']}", ""]
    # The author tag is substituted below so the f-string stays readable.
    lines[3] = f"{author_tag} [POST {pid}]: {post['text']}"

    comments = post["comments"]
    for c in comments:
        require(c, COMMENT_REQUIRED, f"post {pid} comment {c.get('id')}")
    if not comments:
        lines.append("")
        return lines

    roots, children = build_forest(comments)
    rendered: list[str] = []

    def walk(c: dict, depth: int, parent_present: bool) -> None:
        rendered.append(render_thread_line(c, depth, parent_present,
                                           author_uid, author_tag, pid, stats))
        if (c.get("user") or {}).get("id") == author_uid:
            stats["author_reply_rows"] += 1
        for k in children.get(str(c["id"]), []):
            walk(k, depth + 1, True)

    for r in roots:
        walk(r, 0, False)
    lines.append(f"### Comments ({len(rendered)}):")
    lines.extend(rendered)
    return lines


def build(posts: list[dict], author_uid: int, author_name: str,
          target_uid: str, attested_complete: bool) -> tuple[str, dict]:
    author_tag = f"★AUTHOR({author_name})"
    ordered = sorted(posts, key=lambda p: (p["created_at"], int(p["id"])))
    status_word = "attested COMPLETE" if attested_complete else "attested INCOMPLETE"
    header = [
        f"# {author_name} (财主) — Xueqiu corpus (captured posts + comments + threaded context)",
        f"# Xueqiu user {target_uid}. {len(ordered)} posts; comments rebuilt into HN-style reply trees.",
        f"# AUTHOR statements are tagged {author_tag}; all other lines are commenters (NEVER cite as the author).",
        (f"# Source: posts.json ({status_word} by meta.json — see corpus_provenance.json). "
         f"IDs are stable Xueqiu ids (post_id / c<comment_id>). "
         f"Generator: sources/hkex/_registry/build_corpus_from_capture.py."),
    ]
    stats = {
        "posts": len(ordered),
        "comment_rows": sum(len(p.get("comments", [])) for p in ordered),
        "author_reply_rows": 0,
        "context_parentheticals": 0,
        "context_truncations": 0,
    }
    sections = [render_post_section(p, author_uid, author_tag, stats) for p in ordered]
    parts: list[str] = ["\n".join(header), ""]
    for s in sections:
        parts.append("\n".join(s))
    text = "\n\n".join([parts[0]] + parts[2:]) + "\n"
    return text, stats


def load_capture(capture_dir: Path) -> tuple[list[dict], dict, dict]:
    posts_path = capture_dir / "posts.json"
    meta_path = capture_dir / "meta.json"
    for p in (posts_path, meta_path):
        if not p.is_file():
            raise SystemExit(f"capture contract violation: missing {p}")
    posts = json.loads(posts_path.read_text(encoding="utf-8"))
    meta = json.loads(meta_path.read_text(encoding="utf-8"))
    if not isinstance(posts, list) or not posts:
        raise SystemExit("capture contract violation: posts.json is not a non-empty list")
    for k in ("target", "complete", "run_status", "total_posts"):
        if k not in meta:
            raise SystemExit(f"capture contract violation: meta.json missing key {k!r}")
    if int(meta["total_posts"]) != len(posts):
        raise SystemExit(f"capture contract violation: meta total_posts={meta['total_posts']} "
                         f"!= posts.json length {len(posts)}")
    input_hashes = {
        "posts_json_sha256": sha256_file(posts_path),
        "posts_json_bytes": posts_path.stat().st_size,
        "meta_json_sha256": sha256_file(meta_path),
    }
    return posts, meta, input_hashes


def attestation_from_meta(meta: dict) -> dict:
    timeline = meta.get("timeline") or {}
    return {
        "complete": bool(meta.get("complete")),
        "run_status": meta.get("run_status"),
        "total_posts": meta.get("total_posts"),
        "status_counts": meta.get("status_counts"),
        "timeline_population": timeline.get("population"),
        "timeline_fetched_unique": timeline.get("fetched_unique"),
        "timeline_shortfall": timeline.get("shortfall"),
        "comments_capped": (meta.get("cap_causes") or {}).get("comments_capped"),
        "incomplete_posts": [p.get("key") for p in (meta.get("incomplete_posts") or [])],
    }


def do_write(args) -> int:
    posts, meta, input_hashes = load_capture(args.capture_dir)
    attestation = attestation_from_meta(meta)
    if str(meta["target"]) != str(args.target_uid):
        raise SystemExit(f"capture is for uid {meta['target']}, expected {args.target_uid}")
    if not attestation["complete"] and not args.allow_incomplete:
        raise SystemExit(
            "capture is attested INCOMPLETE by meta.json "
            f"(run_status={attestation['run_status']!r}, "
            f"timeline shortfall={attestation['timeline_shortfall']}, "
            f"incomplete posts={len(attestation['incomplete_posts'] or [])}). "
            "Re-run the crawler to completion, or pass --allow-incomplete to "
            "proceed with the shortfall recorded in provenance."
        )
    text, stats = build(posts, args.author_uid, args.author_name,
                        str(args.target_uid), attestation["complete"])
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(text, encoding="utf-8")
    prov = {
        "schema_version": SCHEMA_VERSION,
        "generator": "sources/hkex/_registry/build_corpus_from_capture.py",
        "capture_dir": str(args.capture_dir),
        "target_uid": str(args.target_uid),
        "author_uid": args.author_uid,
        "author_name": args.author_name,
        "input": input_hashes,
        "capture_attestation": attestation,
        "allow_incomplete": bool(args.allow_incomplete),
        "render_rules": {
            "timezone": "UTC+8 (Beijing), minute precision",
            "post_order": "ascending (created_at, id)",
            "sibling_order": "ascending (created_at_iso, id)",
            "parent_rule": "reply_to_id nests only when the parent comment is in the same post's set",
            "context_rule": ("parenthetical only for orphan parents with non-empty reply_to_text; "
                             f"truncated at {CTX_TRUNCATE_AT} chars + ellipsis"),
            "duplicates": "a comment captured under several posts renders under each (no dedup)",
        },
        "stats": stats,
        "output": {
            "path": str(args.out.relative_to(ROOT)) if args.out.is_relative_to(ROOT) else str(args.out),
            "sha256": sha256_file(args.out),
            "bytes": args.out.stat().st_size,
        },
    }
    args.provenance.write_text(
        json.dumps(prov, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"corpus": prov["output"], "stats": stats,
                      "capture_complete": attestation["complete"]},
                     ensure_ascii=False, indent=2, sort_keys=True))
    return 0


def do_check(args) -> int:
    """Rebuild to a temp path; assert byte-equality with the existing corpus
    and SHA-equality with the committed provenance record."""
    if not args.out.is_file() or not args.provenance.is_file():
        print("FAIL: corpus or provenance missing; run --write first", file=sys.stderr)
        return 1
    posts, meta, input_hashes = load_capture(args.capture_dir)
    prov = json.loads(args.provenance.read_text(encoding="utf-8"))
    verdicts = {
        "input_posts_json_unchanged":
            prov.get("input", {}).get("posts_json_sha256") == input_hashes["posts_json_sha256"],
    }
    attestation = attestation_from_meta(meta)
    text, _ = build(posts, args.author_uid, args.author_name,
                    str(args.target_uid), attestation["complete"])
    with tempfile.NamedTemporaryFile("w", encoding="utf-8", suffix=".md", delete=False) as f:
        f.write(text)
        rebuilt = Path(f.name)
    try:
        rebuilt_sha = sha256_file(rebuilt)
        verdicts["rebuild_matches_corpus"] = rebuilt_sha == sha256_file(args.out)
        verdicts["rebuild_matches_provenance"] = rebuilt_sha == prov.get("output", {}).get("sha256")
    finally:
        rebuilt.unlink(missing_ok=True)
    ok = all(verdicts.values())
    print(json.dumps({"verdicts": verdicts, "verdict": "PASS" if ok else "FAIL"}, indent=2))
    return 0 if ok else 1


def _self_test() -> int:
    failures: list[str] = []
    if ctx_quote("x" * 70) != "x" * 70:
        failures.append("70-char context must not truncate")
    if ctx_quote("x" * 71) != "x" * 70 + "…":
        failures.append("71-char context must truncate to 70 + ellipsis")
    if bj_minute(1655708801000) != "2022-06-20 15:06":
        failures.append(f"epoch-ms BJ conversion wrong: {bj_minute(1655708801000)}")
    if bj_minute("2022-06-11T22:20:00+00:00") != "2022-06-12 06:20":
        failures.append(f"iso BJ conversion wrong: {bj_minute('2022-06-11T22:20:00+00:00')}")

    # Forest: nesting by reply_to_id-in-set, orphan parent context, sorting,
    # and a duplicate-cid guard.
    posts = [{
        "id": 100, "created_at": 1655000000000, "created_at_iso": "2022-06-12T02:13:20+00:00",
        "like_count": 1, "reply_count": 2, "retweet_count": 0, "text": "本帖正文",
        "url": "https://xueqiu.com/2424206371/100",
        "comments": [
            {"id": 2, "created_at_iso": "2022-06-12T03:00:00+00:00", "like_count": 0,
             "text": "回复@财主A: 子评论", "user": {"id": 7, "screen_name": "乙"},
             "reply_to": {"screen_name": "财主A"}, "reply_to_id": 1, "reply_to_text": "根评论", "root_id": -1},
            {"id": 1, "created_at_iso": "2022-06-12T02:30:00+00:00", "like_count": 3,
             "text": "根评论", "user": {"id": 9, "screen_name": "财主A"},
             "reply_to": None, "reply_to_id": None, "reply_to_text": "", "root_id": -1},
            {"id": 3, "created_at_iso": "2022-06-12T04:00:00+00:00", "like_count": 0,
             "text": "孤儿回复", "user": {"id": 9, "screen_name": "财主A"},
             "reply_to": {"screen_name": "删除者"}, "reply_to_id": 999,
             "reply_to_text": "被删除的话" * 20, "root_id": -1},
        ],
    }]
    text, stats = build(posts, author_uid=9, author_name="财主A",
                        target_uid="42", attested_complete=False)
    lines = text.splitlines()
    if "attested INCOMPLETE" not in text.splitlines()[3]:
        failures.append("header must carry the incompleteness attestation")
    root_idx = next((i for i, l in enumerate(lines) if "[c1]" in l), None)
    child_idx = next((i for i, l in enumerate(lines) if "[c2]" in l), None)
    orphan_idx = next((i for i, l in enumerate(lines) if "[c3]" in l), None)
    if root_idx is None or child_idx is None or orphan_idx is None:
        failures.append("not all comments rendered")
    else:
        if not lines[root_idx].startswith("- ★AUTHOR(财主A) ·"):
            failures.append(f"root author line wrong: {lines[root_idx][:60]}")
        if not lines[child_idx].startswith("  - @乙 ·"):
            failures.append(f"child must nest at depth 1: {lines[child_idx][:60]}")
        if root_idx > child_idx:
            failures.append("root must render before its child")
        if "(↪in reply to @删除者: \"" not in lines[orphan_idx]:
            failures.append(f"orphan parent context missing: {lines[orphan_idx][:80]}")
        if "…\")" not in lines[orphan_idx]:
            failures.append("long orphan context must truncate with ellipsis")
        if "(↪in reply to" in lines[child_idx]:
            failures.append("nested child must NOT carry a context parenthetical")
    if stats["context_truncations"] != 1 or stats["context_parentheticals"] != 1:
        failures.append(f"stats wrong: {stats}")
    if not text.endswith("孤儿回复\n") or text.endswith("\n\n"):
        failures.append("EOF must be the last line + single newline")

    # Determinism: same input, same bytes.
    text2, _ = build(json.loads(json.dumps(posts)), 9, "财主A", "42", False)
    if text2 != text:
        failures.append("build is not deterministic on identical input")

    # Duplicate cid within one post fails closed.
    dup = json.loads(json.dumps(posts))
    dup[0]["comments"].append(dict(dup[0]["comments"][0]))
    try:
        build(dup, 9, "财主A", "42", False)
        failures.append("duplicate cid within a post must fail closed")
    except SystemExit:
        pass

    # Newline inside text fails closed.
    bad = json.loads(json.dumps(posts))
    bad[0]["text"] = "第一行\n第二行"
    try:
        build(bad, 9, "财主A", "42", False)
        failures.append("newline in post text must fail closed")
    except SystemExit:
        pass

    # Parser round-trip: corpus_model must parse the built text cleanly.
    sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
    import corpus_model  # noqa: E402
    tmp = Path(tempfile.mkstemp(suffix=".md")[1])
    try:
        # corpus_model is pinned to the 狗不叫 author tag; rebuild the sample
        # with the production author so the round-trip exercises the real tag.
        text3, _ = build(posts, author_uid=9, author_name="狗不叫", target_uid="42",
                         attested_complete=False)
        tmp.write_text(text3, encoding="utf-8")
        utts = corpus_model.parse(tmp)
        if len(utts) != 4 or sum(1 for u in utts if u.kind == "post") != 1:
            failures.append(f"corpus_model round-trip wrong: {len(utts)} utterances")
        if sum(1 for u in utts if u.is_author) != 3:
            failures.append("author tagging lost in round-trip")
    finally:
        tmp.unlink(missing_ok=True)

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (build_corpus_from_capture: rules + round-trip)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--write", action="store_true")
    g.add_argument("--check", action="store_true")
    g.add_argument("--self-test", action="store_true")
    ap.add_argument("--capture-dir", type=Path,
                    help="crawler output/<uid>/ directory (required for --write/--check)")
    ap.add_argument("--target-uid", default="2424206371")
    ap.add_argument("--author-uid", type=int, default=2424206371)
    ap.add_argument("--author-name", default="狗不叫")
    ap.add_argument("--out", type=Path, default=DEFAULT_OUT)
    ap.add_argument("--provenance", type=Path, default=DEFAULT_PROVENANCE)
    ap.add_argument("--allow-incomplete", action="store_true",
                    help="proceed on an attested-incomplete capture (recorded in provenance)")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()
    if not args.capture_dir:
        ap.error("--capture-dir is required for --write/--check")
    if args.write:
        return do_write(args)
    return do_check(args)


if __name__ == "__main__":
    raise SystemExit(main())
