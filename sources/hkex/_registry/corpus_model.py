#!/usr/bin/env python3
r"""Parse `_research/xueqiu_goubujiao/corpus_complete.md` into utterances.

The complete corpus is HN-style threaded markdown. Each renderable utterance is
one post body, author reply, or commenter line. The parser separates, for every
utterance:

  - the citable TEXT (what the speaker wrote),
  - the speaker identity (`is_author` = the ★AUTHOR(狗不叫) tag),
  - the stable id (`post_id` for a post body, `comment_id` for a reply/comment),
  - the parent-context parenthetical `(↪in reply to @X: "…")` (NEVER citable; it
    is another user's text quoted by Xueqiu — held separately for the AC-8
    attribution gate).

Only `is_author` utterances are ever cited; commenter text is rendered as context
so the deck's chunks carry the whole discussion, but it is never quoted.
"""
from __future__ import annotations

import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
CORPUS = ROOT / "_research/xueqiu_goubujiao/corpus_complete.md"

AUTHOR_TAG = "★AUTHOR(狗不叫)"

_POST_HEADER = re.compile(r"^## POST (?P<pid>\d+) \|")
_POST_BODY = re.compile(r"^★AUTHOR\(狗不叫\) \[POST (?P<pid>\d+)\]: (?P<text>.*)$")
# Indented thread line: a comment or an author reply. The context parenthetical
# `(↪in reply to @X: "…")` may itself contain `)`, so match lazily up to the
# `): ` that separates metadata from the speaker's text (NOT `[^)]*`, which would
# drop the whole line — and thus the utterance — on the first inner paren).
_THREAD = re.compile(
    r"^(?P<indent>\s*)- (?P<who>★AUTHOR\(狗不叫\)|@\S+) · (?P<date>.*?) · 👍(?P<likes>\d+) · "
    r"\[c(?P<cid>\d+)\](?:\s*\((?P<ctx>↪in reply to.*?)\))?: (?P<text>.*)$"
)
# A line that LOOKS like a thread line; used to fail closed if `_THREAD` ever
# misses one (format drift) instead of silently dropping the utterance.
_THREAD_LOOSE = re.compile(r"^\s*- (?:★AUTHOR\(狗不叫\)|@)\S* · ")
# Leading Xueqiu reply boilerplate the author did not author as substance.
_REPLY_PREFIX = re.compile(r"^回复@\S+?[:：]\s*")


@dataclass
class Utterance:
    kind: str            # "post" | "areply" | "comment"
    post_id: str
    comment_id: str | None
    is_author: bool
    depth: int
    text: str            # full citable text the speaker wrote
    author_words: str    # text minus a leading `回复@X:` reply prefix
    context: str | None  # the `(↪in reply to …)` parent-context, never citable
    line_no: int = field(default=0)


def parse(path: Path = CORPUS) -> list[Utterance]:
    if not path.is_file():
        raise SystemExit(f"corpus missing: {path}")
    out: list[Utterance] = []
    unparsed: list[tuple[int, str]] = []
    current_pid = ""
    for i, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        mh = _POST_HEADER.match(raw)
        if mh:
            current_pid = mh.group("pid")
            continue
        mb = _POST_BODY.match(raw)
        if mb:
            text = mb.group("text")
            out.append(Utterance(
                kind="post", post_id=mb.group("pid"), comment_id=None, is_author=True,
                depth=0, text=text, author_words=text, context=None, line_no=i,
            ))
            continue
        mt = _THREAD.match(raw)
        if mt:
            if not current_pid:
                raise SystemExit(f"thread line before any '## POST' header at line {i}: {raw[:80]}")
            text = mt.group("text")
            is_author = mt.group("who") == AUTHOR_TAG
            aw = _REPLY_PREFIX.sub("", text) if is_author else text
            out.append(Utterance(
                kind="areply" if is_author else "comment",
                post_id=current_pid,
                comment_id=mt.group("cid"),
                is_author=is_author,
                depth=len(mt.group("indent")) // 2,
                text=text,
                author_words=aw,
                context=mt.group("ctx"),
                line_no=i,
            ))
            continue
        if _THREAD_LOOSE.match(raw):
            unparsed.append((i, raw))
    if unparsed:
        sample = "; ".join(f"L{n}:{r[:60]}" for n, r in unparsed[:5])
        raise SystemExit(f"{len(unparsed)} thread-like line(s) failed to parse (format drift) -> {sample}")
    return out


def author_utterances(utts: list[Utterance]) -> list[Utterance]:
    return [u for u in utts if u.is_author]


def _self_test() -> int:
    sample = "\n".join([
        "## POST 222375639 | 2022-06-12 06:20 BJ | 👍134 💬264 🔁53",
        "URL: https://xueqiu.com/2424206371/222375639",
        "",
        "★AUTHOR(狗不叫) [POST 222375639]: 上個月sell call，行使價被調整為11.31元。",
        "",
        "### Comments (349):",
        "- @清香绿茶1997 · 2022-06-12 07:15 · 👍5 · [c244700573]: 看起来好像财主的ID@管我财",
        "  - ★AUTHOR(狗不叫) · 2022-06-12 07:55 · 👍51 · [c244701940]: 回复@ForeverFight命運: 汪！",
        "- @道术自然 · 2022-06-12 11:40 · 👍4 · [c244717103] (↪in reply to @丹书铁券: \"讨论已被 丹书铁券 删除\"): 回复@丹书铁券: 笑死 财主好好玩",
        "  - ★AUTHOR(狗不叫) · 2022-06-12 16:26 · 👍5 · [c244736206] (↪in reply to @柴迷: \"讨论已被 柴迷 删除\"): 回复@柴迷: 肯定是太寶呀",
        # regression: a `)` INSIDE the context quote must not drop the utterance.
        "- @nested · 2022-06-12 18:00 · 👍0 · [c244799999] (↪in reply to @x: \"看(00700)腾讯\"): 这条不能丢",
    ])
    tmp = Path("/tmp/_corpus_model_selftest.md")
    tmp.write_text(sample, encoding="utf-8")
    utts = parse(tmp)
    failures = []

    post = [u for u in utts if u.kind == "post"]
    if len(post) != 1 or post[0].post_id != "222375639" or not post[0].is_author:
        failures.append(f"post body parse wrong: {post}")
    if post and post[0].text != "上個月sell call，行使價被調整為11.31元。":
        failures.append(f"post text wrong: {post[0].text!r}")

    areplies = [u for u in utts if u.kind == "areply"]
    if len(areplies) != 2:
        failures.append(f"expected 2 author replies, got {len(areplies)}")
    if areplies:
        a0 = areplies[0]
        if not a0.is_author or a0.comment_id != "244701940" or a0.post_id != "222375639":
            failures.append(f"author reply ids wrong: {a0}")
        if a0.author_words != "汪！":
            failures.append(f"author_words prefix strip wrong: {a0.author_words!r}")
        a1 = areplies[1]
        if not a1.context or "柴迷" not in a1.context:
            failures.append(f"author reply context not captured: {a1.context!r}")
        if a1.author_words != "肯定是太寶呀":
            failures.append(f"author reply author_words wrong: {a1.author_words!r}")

    comments = [u for u in utts if u.kind == "comment"]
    if len(comments) != 3:
        failures.append(f"expected 3 comments, got {len(comments)}")
    if comments and any(c.is_author for c in comments):
        failures.append("a commenter was misflagged as author")
    # The parenthetical context belongs to the COMMENT line too, never citable.
    cwith = [c for c in comments if c.context]
    if not cwith or not cwith[0].context or "丹书铁券" not in cwith[0].context:
        failures.append("comment parent-context not captured")
    # The `)`-in-context line must survive and keep its real text.
    nested = [c for c in comments if c.comment_id == "244799999"]
    if not nested or nested[0].text != "这条不能丢":
        failures.append(f"`)`-in-context utterance dropped or mis-split: {nested}")

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print(f"SELF-TEST PASSED (corpus_model: {len(utts)} utterances from sample; "
          f"{len(post)} post / {len(areplies)} areply / {len(comments)} comment)")
    return 0


if __name__ == "__main__":
    sys.exit(_self_test())
