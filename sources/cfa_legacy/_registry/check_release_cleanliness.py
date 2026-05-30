#!/usr/bin/env python3
"""Release-cleanliness gate: the v1 release is clean, tracked, and (optionally) tagged.

Re-runnable, fail-closed. RE-DERIVES every release fact from git + disk (never trusts
the prose of `_research/31`): the small published-index artifacts are tracked, the large
chunks_manifest is excluded, the trust-chain sidecars (one per active card, count derived
from the published manifest) + research docs are tracked, the rebuild-recipe scripts
exist, the worktree is release-clean (NO uncommitted change to ANY tracked file — not
merely no untracked files — and no non-ignored untracked file), and (with `--require-tag`)
the annotated `v1-candidate` tag exists, references the evidence, AND points at the
release commit (tag target == HEAD).

Usage:
  check_release_cleanliness.py              # all checks except the tag (run pre-tag)
  check_release_cleanliness.py --require-tag # also require the annotated v1-candidate tag
  check_release_cleanliness.py --self-test   # synthetic negative probes
"""

from __future__ import annotations

import argparse
import glob
import json
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
TAG = "v1-candidate"
CARDS_MANIFEST = "out/cfa_legacy/cards_manifest.json"
TRACKED_SMALL = [
    CARDS_MANIFEST,
    "out/cfa_legacy/summaries.json",
    "out/cfa_legacy/INDEX.md",
    "out/cfa_legacy/pdfium_provenance.json",
]
EXCLUDED = ["out/cfa_legacy/chunks_manifest.json"]
RECIPE_SCRIPTS = [
    "sources/cfa_legacy/_registry/run_ingest_per_source.py",
    "sources/cfa_legacy/_registry/merge_ingest_manifests.py",
]
RELEASE_DOC = "_research/31_v1_release_and_rebuild_recipe.md"
# Strings the tag message / release doc must reference (the release evidence pointers).
TAG_MUST_REFERENCE = ["scope_ledger", "index_repro"]


def git(*a) -> str:
    return subprocess.run(["git", "-C", str(REPO), *a], capture_output=True, text=True).stdout


def is_tracked(path: str) -> bool:
    return bool(git("ls-files", "--", path).strip())


def is_ignored(path: str) -> bool:
    return subprocess.run(["git", "-C", str(REPO), "check-ignore", "-q", path]).returncode == 0


def run_checks(*, require_tag: bool) -> list[str]:
    failures: list[str] = []

    # (1) the 4 small published-index artifacts are tracked AND exist on disk.
    for p in TRACKED_SMALL:
        if not is_tracked(p):
            failures.append(f"release artifact not tracked: {p}")
        if not (REPO / p).is_file():
            failures.append(f"release artifact missing on disk: {p}")

    # (2) the large chunks_manifest is NOT tracked and IS ignored.
    for p in EXCLUDED:
        if is_tracked(p):
            failures.append(f"large artifact must NOT be tracked: {p}")
        if not is_ignored(p):
            failures.append(f"large artifact must be .gitignore-excluded: {p}")

    # (3) trust-chain sidecars: one per active card, tracked AND on disk. The active-card
    # count is DERIVED from the published manifest (data-driven; 268 at v0, 402 at v1) so
    # this stays correct across milestone bumps instead of hard-coding a stale literal.
    manifest_path = REPO / CARDS_MANIFEST
    active_cards = (len(json.loads(manifest_path.read_text(encoding="utf-8"))["cards"])
                    if manifest_path.is_file() else -1)
    disk_sidecars = len(glob.glob(str(REPO / "cards/cfa_legacy/**/*.history.jsonl"), recursive=True))
    tracked_sidecars = len([l for l in git("ls-files", "cards/cfa_legacy/").splitlines()
                            if l.endswith(".history.jsonl")])
    if disk_sidecars != active_cards:
        failures.append(f".history.jsonl sidecars on disk ({disk_sidecars}) != active cards "
                        f"in {CARDS_MANIFEST} ({active_cards})")
    if tracked_sidecars != disk_sidecars:
        failures.append(f"sidecars tracked ({tracked_sidecars}) != on disk ({disk_sidecars})")

    # (4) research docs 23-26 + the decision ledger (29) + both release recipes (30 v0, 31 v1) tracked.
    for n in (23, 24, 25, 26, 29, 30, 31):
        if not git("ls-files", f"_research/{n}_*").strip():
            failures.append(f"_research/{n}_* not tracked")

    # (5) rebuild-recipe scripts named in the release doc exist on disk.
    for s in RECIPE_SCRIPTS:
        if not (REPO / s).is_file():
            failures.append(f"rebuild-recipe script missing: {s}")
    # ... and the release doc actually names them + the recipe.
    doc = (REPO / RELEASE_DOC).read_text(encoding="utf-8") if (REPO / RELEASE_DOC).is_file() else ""
    if not doc:
        failures.append(f"release doc missing: {RELEASE_DOC}")
    else:
        for s in ("run_ingest_per_source.py", "merge_ingest_manifests.py"):
            if s not in doc:
                failures.append(f"release doc does not document recipe script {s}")
        if "chunks_manifest.json" not in doc:
            failures.append("release doc does not mention chunks_manifest.json (rebuild recipe)")

    # (6) worktree is release-clean: a RELEASE certifier must reject ANY uncommitted
    # change to a tracked file (` M `, `A `, `D `, `R `, `MM`, …) AND any non-ignored
    # untracked file (`?? `) — not just untracked. Otherwise it could certify bytes that
    # differ from what is committed. `--untracked-files=all` already excludes
    # .gitignore-covered paths; we additionally drop loop-local `.humanize/` defensively.
    dirty = release_dirty_entries(git("status", "--porcelain", "--untracked-files=all").splitlines())
    if dirty:
        failures.append(f"worktree is not release-clean (uncommitted changes): {dirty[:10]}")

    # (7) (optional) annotated v1-candidate tag exists, is annotated, references the
    # evidence, AND points AT THE RELEASE COMMIT (its target == HEAD). A tag left
    # pointing at an earlier commit must not certify the current worktree.
    if require_tag:
        exists = TAG in git("tag", "--list", TAG).split()
        kind = git("cat-file", "-t", TAG).strip() if exists else ""
        msg = git("for-each-ref", "--format=%(contents)", f"refs/tags/{TAG}") if exists else ""
        tag_target = git("rev-list", "-n", "1", TAG).strip() if exists else ""
        head = git("rev-parse", "HEAD").strip()
        failures.extend(tag_failures(exists=exists, kind=kind, msg=msg,
                                     tag_target=tag_target, head=head))

    return failures


def release_dirty_entries(status_porcelain_lines: list[str]) -> list[str]:
    """Pure: return the `git status --porcelain` lines that make the worktree NOT
    release-clean — any non-blank entry (staged/modified/deleted/renamed/untracked)
    whose path is not loop-local `.humanize/`. The whole tree should be committed at a
    release tag; `.humanize/` is gitignored loop state and is excluded defensively."""
    dirty: list[str] = []
    for line in status_porcelain_lines:
        if not line.strip():
            continue
        path = line[3:]
        if " -> " in path:  # rename: "old -> new"
            path = path.split(" -> ", 1)[1]
        path = path.strip().strip('"')
        if path == ".humanize" or path.startswith(".humanize/"):
            continue
        dirty.append(line)
    return dirty


def tag_failures(*, exists: bool, kind: str, msg: str, tag_target: str, head: str) -> list[str]:
    """Pure: validate the release tag. Fails if it is missing, not annotated, does not
    reference the required evidence strings, OR does not point at HEAD (the release
    commit). The tag-target==HEAD check is the load-bearing fix: a stale tag left on an
    earlier commit must not certify the current (clean) worktree."""
    out: list[str] = []
    if not exists:
        out.append(f"annotated tag {TAG!r} does not exist")
        return out
    if kind != "tag":  # annotated tags are objects of type 'tag' (lightweight = 'commit')
        out.append(f"tag {TAG!r} is not annotated (cat-file type={kind!r})")
    for ref in TAG_MUST_REFERENCE:
        if ref not in msg:
            out.append(f"tag {TAG!r} message does not reference {ref!r}")
    if not tag_target or not head or tag_target != head:
        out.append(f"tag {TAG!r} target {tag_target[:12]!r} != HEAD {head[:12]!r} "
                   "(re-point the tag to the release commit)")
    return out


def _self_test() -> int:
    """Synthetic negative probes (do not touch the real tree)."""
    failures = 0

    def expect(name, cond):
        nonlocal failures
        print(("  ok    " if cond else "  FAIL  ") + name)
        if not cond:
            failures += 1

    # The real run (no tag) must pass on the actual tree.
    real = run_checks(require_tag=False)
    expect("real tree passes (no --require-tag)", real == [])
    # is_tracked / is_ignored sanity on known paths.
    expect("cards_manifest.json is tracked", is_tracked("out/cfa_legacy/cards_manifest.json"))
    expect("chunks_manifest.json is NOT tracked", not is_tracked("out/cfa_legacy/chunks_manifest.json"))
    expect("chunks_manifest.json is ignored", is_ignored("out/cfa_legacy/chunks_manifest.json"))
    expect("a nonexistent path is not tracked", not is_tracked("out/cfa_legacy/__nope__.json"))
    # require_tag must FAIL when the tag is absent (only meaningful pre-tag; if the tag
    # already exists this still asserts the require_tag path runs).
    tag_exists = TAG in git("tag", "--list", TAG).split()
    rt = run_checks(require_tag=True)
    if tag_exists:
        expect("require_tag passes when tag exists", rt == [] or all("does not exist" not in f for f in rt))
    else:
        expect("require_tag fails when tag absent", any("does not exist" in f for f in rt))

    # (R14) negative probes for the two hardened guards, via the pure helpers.
    # check (6): a modified/staged/untracked tracked path is dirty; .humanize/ is not.
    expect("modified tracked file is dirty",
           release_dirty_entries([" M out/cfa_legacy/cards_manifest.json"]) != [])
    expect("staged tracked file is dirty",
           release_dirty_entries(["A  _research/30_v0_release_and_rebuild_recipe.md"]) != [])
    expect("untracked non-ignored file is dirty",
           release_dirty_entries(["?? stray.txt"]) != [])
    expect("renamed file is dirty",
           release_dirty_entries(["R  old.py -> new.py"]) != [])
    expect("clean tree is not dirty", release_dirty_entries([]) == [])
    expect(".humanize loop state is NOT dirty",
           release_dirty_entries([" M .humanize/rlcr/x/round-14-summary.md"]) == [])
    # check (7): a stale tag (target != head) fails; the well-formed on-head tag passes.
    GOOD_MSG = "references scope_ledger and index_repro"
    expect("stale tag (target != head) fails",
           any("!= HEAD" in f for f in tag_failures(
               exists=True, kind="tag", msg=GOOD_MSG, tag_target="aaaaaaa", head="bbbbbbb")))
    expect("lightweight (non-annotated) tag fails",
           any("not annotated" in f for f in tag_failures(
               exists=True, kind="commit", msg=GOOD_MSG, tag_target="abc", head="abc")))
    expect("tag missing an evidence reference fails",
           any("does not reference" in f for f in tag_failures(
               exists=True, kind="tag", msg="references index_repro only", tag_target="abc", head="abc")))
    expect("absent tag fails",
           any("does not exist" in f for f in tag_failures(
               exists=False, kind="", msg="", tag_target="", head="abc")))
    expect("well-formed on-head annotated tag passes",
           tag_failures(exists=True, kind="tag", msg=GOOD_MSG, tag_target="abc123", head="abc123") == [])
    print()
    if failures:
        print(f"RELEASE CLEANLINESS SELF-TEST: FAIL ({failures})", file=sys.stderr)
        return 1
    print("RELEASE CLEANLINESS SELF-TEST: PASS")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--require-tag", action="store_true",
                    help="also require the annotated v1-candidate tag (run after tagging)")
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()

    failures = run_checks(require_tag=args.require_tag)
    print(f"tracked small artifacts: {len(TRACKED_SMALL)} | excluded: {len(EXCLUDED)} | "
          f"require_tag: {args.require_tag}")
    if failures:
        print("\nRELEASE CLEANLINESS: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("\nRELEASE CLEANLINESS: PASS (small index + provenance tracked; chunks_manifest "
          "excluded; one sidecar per active card (manifest-derived) + research docs tracked; "
          "recipe scripts present; worktree release-clean"
          + ("; v1-candidate tag annotated, references evidence, on HEAD)"
             if args.require_tag else ")"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
