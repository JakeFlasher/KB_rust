#!/usr/bin/env python3
r"""Faithful Python port of the kernel `cacg-core::normalize::normalize_text`.

Used ONLY to PROPOSE / parity-check candidate matches; the real authority for a
verbatim bind is `kb verify`. This port is fixture-verified against the kernel's
own documented test vectors (`crates/cacg-core/src/normalize.rs`) by `--self-test`
so a silent normalize drift cannot let the parity checker falsely pass.

Five-step pipeline (kernel-identical):
  1. Unicode NFC.
  2. Replace the seven Latin ligatures (U+FB00..U+FB06) with multi-char forms.
  3. Rejoin hyphenated line breaks: remove every `-\s*\n\s*`.
  4. Collapse runs of Python-`re` `\s` whitespace to a single U+0020.
  5. Strip leading/trailing whitespace.

Python `re` `\s` (str/UNICODE) matches the same set the kernel pins, including the
file-separator controls U+001C..U+001F, so a plain `re` implementation is byte-equal.
"""
from __future__ import annotations

import re
import sys
import unicodedata

_LIGATURES = {
    "ﬀ": "ff", "ﬁ": "fi", "ﬂ": "fl", "ﬃ": "ffi",
    "ﬄ": "ffl", "ﬅ": "ft", "ﬆ": "st",
}
_HYPHEN_LINEBREAK = re.compile(r"-\s*\n\s*")
_WS_RUN = re.compile(r"\s+")


def normalize_text(text: str) -> str:
    s = unicodedata.normalize("NFC", text)
    if any(ch in s for ch in _LIGATURES):
        for src, dst in _LIGATURES.items():
            if src in s:
                s = s.replace(src, dst)
    s = _HYPHEN_LINEBREAK.sub("", s)
    s = _WS_RUN.sub(" ", s)
    return s.strip()


# Kernel test vectors transcribed from crates/cacg-core/src/normalize.rs.
_VECTORS = [
    ("hello world", "hello world"),
    ("café", "café"),                 # NFC combining acute
    ("oﬁce", "ofice"),                      # fi ligature
    ("oﬃce", "office"),                     # ffi ligature
    ("baﬄe", "baffle"),                     # ffl ligature
    ("beﬆ", "best"),                        # st ligature
    ("oﬀ", "off"),                          # ff ligature
    ("word-\nnext", "wordnext"),
    ("word-  \n  next", "wordnext"),
    ("word-\n\nnext", "wordnext"),
    ("a  b\t\tc", "a b c"),
    ("a\nb", "a b"),
    ("a b", "a b"),                         # NBSP collapses
    ("  hello  ", "hello"),
    ("\n\nhello\t\t", "hello"),
    ("  oﬃce sup-\nplies  ", "office supplies"),
    ("ab", "a b"),                         # file separator -> space
    ("ab", "a b"),
    ("ab", "a b"),
    ("ab", "a b"),
    ("hello", "hello"),
]


def _self_test() -> int:
    failures = []
    for inp, want in _VECTORS:
        got = normalize_text(inp)
        if got != want:
            failures.append(f"normalize_text({inp!r}) -> {got!r}, want {want!r}")
        # idempotence
        if normalize_text(got) != got:
            failures.append(f"not idempotent for {inp!r}: {got!r}")
    # CJK: no internal whitespace must be introduced for a clean run.
    cjk = "上個月特別股息行使價被調整為11.31元"
    if normalize_text(cjk) != cjk:
        failures.append(f"CJK clean run altered: {normalize_text(cjk)!r}")
    # CJK with an interior newline collapses to a SPACE (the existential risk):
    if normalize_text("行使\n價") != "行使 價":
        failures.append("CJK interior newline did not collapse to a single space")
    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print(f"SELF-TEST PASSED (cacg_normalize: {len(_VECTORS)} kernel vectors + CJK)")
    return 0


if __name__ == "__main__":
    sys.exit(_self_test())
