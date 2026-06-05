#!/usr/bin/env python3
"""Archive and restore repository PDFs without storing them in git.

The archive stores repo-relative paths plus a SHA-256 manifest, so PDFs can be
restored into the same locations after cloning a source-only copy of the repo.
"""

from __future__ import annotations

import argparse
import datetime as dt
import fnmatch
import hashlib
import json
import os
import shutil
import stat
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path, PurePosixPath
from typing import Iterable, Sequence


SCRIPT_DIR = Path(__file__).resolve().parent
DEFAULT_ROOT = SCRIPT_DIR.parent
DEFAULT_ARCHIVE = ".pdf-archives/repo-pdfs.zip"
MANIFEST_NAME = "__pdf_archive_manifest__.json"
FORMAT_VERSION = "repo-pdf-archive-v1"
DEFAULT_EXCLUDES = (
    ".git/**",
    "target/**",
    ".pdf-archives/**",
    ".venv/**",
    "__pycache__/**",
)
CHUNK_SIZE = 1024 * 1024


def die(message: str) -> None:
    print(f"FATAL: {message}", file=sys.stderr)
    raise SystemExit(1)


def warn(message: str) -> None:
    print(f"[WARN] {message}", file=sys.stderr)


def resolve_root(value: str | None) -> Path:
    root = Path(value).expanduser() if value else DEFAULT_ROOT
    root = root.resolve()
    if not root.is_dir():
        die(f"repo root does not exist or is not a directory: {root}")
    return root


def resolve_archive(root: Path, value: str | None) -> Path:
    archive = Path(value or DEFAULT_ARCHIVE).expanduser()
    if not archive.is_absolute():
        archive = root / archive
    return archive.resolve()


def rel_to_root(root: Path, path: Path) -> str:
    try:
        return path.resolve().relative_to(root).as_posix()
    except ValueError:
        die(f"path is outside repo root: {path}")


def safe_manifest_path(rel: str) -> PurePosixPath:
    if "\\" in rel:
        die(f"unsafe archive path contains backslash: {rel}")
    pure = PurePosixPath(rel)
    if pure.is_absolute() or not pure.parts:
        die(f"unsafe archive path: {rel}")
    if any(part in ("", ".", "..") for part in pure.parts):
        die(f"unsafe archive path: {rel}")
    return pure


def path_from_manifest(root: Path, rel: str) -> Path:
    pure = safe_manifest_path(rel)
    target = root.joinpath(*pure.parts).resolve()
    try:
        target.relative_to(root)
    except ValueError:
        die(f"archive path escapes repo root: {rel}")
    return target


def is_pdf(path: Path) -> bool:
    return path.suffix.lower() == ".pdf"


def is_excluded(rel: str, patterns: Sequence[str]) -> bool:
    return any(fnmatch.fnmatchcase(rel, pattern) for pattern in patterns)


def iter_include_paths(root: Path, includes: Sequence[str]) -> Iterable[Path]:
    search_roots = includes or ["."]
    for include in search_roots:
        candidate = Path(include).expanduser()
        if not candidate.is_absolute():
            candidate = root / candidate
        candidate = candidate.resolve()
        try:
            candidate.relative_to(root)
        except ValueError:
            die(f"--include path is outside repo root: {include}")
        if not candidate.exists():
            die(f"--include path does not exist: {include}")
        if candidate.is_file():
            yield candidate
        else:
            yield from candidate.rglob("*")


def collect_pdf_files(
    root: Path,
    includes: Sequence[str],
    excludes: Sequence[str],
) -> list[tuple[str, Path]]:
    seen: set[str] = set()
    files: list[tuple[str, Path]] = []
    for path in iter_include_paths(root, includes):
        if not path.is_file() or not is_pdf(path):
            continue
        rel = rel_to_root(root, path)
        if is_excluded(rel, excludes):
            continue
        if rel in seen:
            continue
        seen.add(rel)
        files.append((rel, path))
    return sorted(files, key=lambda item: item[0])


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(CHUNK_SIZE), b""):
            digest.update(chunk)
    return digest.hexdigest()


def format_bytes(size: int) -> str:
    units = ("B", "KB", "MB", "GB", "TB")
    value = float(size)
    for unit in units:
        if value < 1024 or unit == units[-1]:
            return f"{value:.1f} {unit}" if unit != "B" else f"{size} B"
        value /= 1024
    return f"{size} B"


def zip_datetime_from_ns(mtime_ns: int) -> tuple[int, int, int, int, int, int]:
    timestamp = max(0, mtime_ns / 1_000_000_000)
    value = dt.datetime.fromtimestamp(timestamp)
    if value.year < 1980:
        return (1980, 1, 1, 0, 0, 0)
    return (value.year, value.month, value.day, value.hour, value.minute, value.second)


def build_manifest(root: Path, files: Sequence[tuple[str, Path]]) -> dict:
    entries = []
    total = 0
    for rel, path in files:
        info = path.stat()
        total += info.st_size
        entries.append(
            {
                "path": rel,
                "bytes": info.st_size,
                "sha256": sha256_file(path),
                "mtime_ns": info.st_mtime_ns,
                "mode": stat.S_IMODE(info.st_mode),
            }
        )
    return {
        "format": FORMAT_VERSION,
        "created_utc": dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z"),
        "root_hint": root.name,
        "file_count": len(entries),
        "total_bytes": total,
        "files": entries,
    }


def load_manifest(archive: Path) -> dict:
    if not archive.is_file():
        die(f"archive not found: {archive}")
    with zipfile.ZipFile(archive, "r") as zf:
        if MANIFEST_NAME not in zf.namelist():
            die(f"archive is missing {MANIFEST_NAME}: {archive}")
        try:
            manifest = json.loads(zf.read(MANIFEST_NAME).decode("utf-8"))
        except json.JSONDecodeError as exc:
            die(f"invalid archive manifest JSON: {exc}")
    if manifest.get("format") != FORMAT_VERSION:
        die(
            "unsupported archive format: "
            f"{manifest.get('format')!r}; expected {FORMAT_VERSION!r}"
        )
    if not isinstance(manifest.get("files"), list):
        die("archive manifest is missing files list")
    for entry in manifest["files"]:
        if not isinstance(entry, dict):
            die("archive manifest contains a non-object file entry")
        safe_manifest_path(str(entry.get("path", "")))
    return manifest


def write_archive(
    root: Path,
    archive: Path,
    files: Sequence[tuple[str, Path]],
    manifest: dict,
    overwrite: bool,
    verify: bool,
) -> None:
    if archive.exists() and not overwrite:
        die(f"archive already exists; pass --overwrite to replace it: {archive}")
    archive.parent.mkdir(parents=True, exist_ok=True)
    fd, temp_name = tempfile.mkstemp(
        prefix=f".{archive.name}.", suffix=".tmp", dir=str(archive.parent)
    )
    os.close(fd)
    temp_archive = Path(temp_name)
    try:
        with zipfile.ZipFile(
            temp_archive,
            "w",
            compression=zipfile.ZIP_DEFLATED,
            compresslevel=6,
        ) as zf:
            for rel, path in files:
                info = path.stat()
                zip_info = zipfile.ZipInfo(rel)
                zip_info.date_time = zip_datetime_from_ns(info.st_mtime_ns)
                zip_info.compress_type = zipfile.ZIP_DEFLATED
                zip_info.external_attr = (stat.S_IMODE(info.st_mode) & 0o777) << 16
                with path.open("rb") as source:
                    with zf.open(zip_info, "w") as target:
                        shutil.copyfileobj(source, target, CHUNK_SIZE)
            manifest_info = zipfile.ZipInfo(MANIFEST_NAME)
            manifest_info.date_time = dt.datetime.now().timetuple()[:6]
            manifest_info.external_attr = 0o644 << 16
            zf.writestr(
                manifest_info,
                json.dumps(manifest, indent=2, sort_keys=True) + "\n",
                compress_type=zipfile.ZIP_DEFLATED,
            )
        if verify:
            with zipfile.ZipFile(temp_archive, "r") as zf:
                bad_member = zf.testzip()
                if bad_member:
                    die(f"zip CRC verification failed for member: {bad_member}")
        os.replace(temp_archive, archive)
    finally:
        temp_archive.unlink(missing_ok=True)


def tracked_pdf_paths(root: Path) -> set[str]:
    try:
        result = subprocess.run(
            ["git", "-C", str(root), "ls-files", "-z"],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except (FileNotFoundError, subprocess.CalledProcessError) as exc:
        detail = getattr(exc, "stderr", b"").decode("utf-8", "replace").strip()
        die(f"could not list git-tracked files{': ' + detail if detail else ''}")
    paths = result.stdout.decode("utf-8", "surrogateescape").split("\0")
    return {path for path in paths if path and path.lower().endswith(".pdf")}


def git_rm_cached(root: Path, rels: Sequence[str], dry_run: bool) -> None:
    tracked = tracked_pdf_paths(root)
    targets = sorted(set(rels).intersection(tracked))
    if not targets:
        print("No tracked PDFs matched; nothing to remove from the git index.")
        return
    print(
        f"{'Would remove' if dry_run else 'Removing'} {len(targets)} tracked PDFs "
        "from the git index; local files stay on disk.",
        flush=True,
    )
    prefix = ["git", "-C", str(root), "rm", "--cached"]
    if dry_run:
        prefix.append("--dry-run")
    prefix.append("--")
    chunk_size = 200
    for index in range(0, len(targets), chunk_size):
        subprocess.run(prefix + targets[index : index + chunk_size], check=True)


def command_pack(args: argparse.Namespace, patch_mode: bool = False) -> int:
    root = resolve_root(args.root)
    archive = resolve_archive(root, args.archive)
    excludes = tuple(DEFAULT_EXCLUDES) + tuple(args.exclude)
    files = collect_pdf_files(root, args.include, excludes)
    total = sum(path.stat().st_size for _, path in files)
    verb = "Patch" if patch_mode else "Pack"
    print(f"{verb} plan: {len(files)} PDFs, {format_bytes(total)}")
    print(f"Archive: {archive}")
    if not files:
        return 0
    if args.dry_run:
        for rel, _ in files[:20]:
            print(f"  {rel}")
        if len(files) > 20:
            print(f"  ... {len(files) - 20} more")
        if getattr(args, "git_rm_cached", False) or patch_mode:
            git_rm_cached(root, [rel for rel, _ in files], dry_run=True)
        return 0
    manifest = build_manifest(root, files)
    write_archive(
        root=root,
        archive=archive,
        files=files,
        manifest=manifest,
        overwrite=args.overwrite,
        verify=not args.no_verify,
    )
    print(f"Wrote {archive} ({format_bytes(archive.stat().st_size)})")
    if getattr(args, "git_rm_cached", False) or patch_mode:
        git_rm_cached(root, [rel for rel, _ in files], dry_run=False)
    return 0


def command_unpack(args: argparse.Namespace) -> int:
    root = resolve_root(args.root)
    archive = resolve_archive(root, args.archive)
    manifest = load_manifest(archive)
    entries = manifest["files"]
    conflicts: list[str] = []
    identical = 0
    missing = 0
    for entry in entries:
        rel = str(entry["path"])
        target = path_from_manifest(root, rel)
        if not target.exists():
            missing += 1
            continue
        if sha256_file(target) == entry["sha256"]:
            identical += 1
            continue
        conflicts.append(rel)
    print(
        "Unpack plan: "
        f"{len(entries)} PDFs from {archive} "
        f"({identical} already current, {missing} missing)"
    )
    if conflicts and not args.overwrite:
        for rel in conflicts[:20]:
            print(f"  conflict: {rel}", file=sys.stderr)
        if len(conflicts) > 20:
            print(f"  ... {len(conflicts) - 20} more conflicts", file=sys.stderr)
        die("existing PDFs differ; pass --overwrite to replace them")
    if args.dry_run:
        return 0

    written = 0
    skipped = 0
    with zipfile.ZipFile(archive, "r") as zf:
        names = set(zf.namelist())
        for entry in entries:
            rel = str(entry["path"])
            if rel not in names:
                die(f"archive member missing for manifest path: {rel}")
            target = path_from_manifest(root, rel)
            if target.exists() and sha256_file(target) == entry["sha256"]:
                skipped += 1
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            fd, temp_name = tempfile.mkstemp(
                prefix=f".{target.name}.", suffix=".tmp", dir=str(target.parent)
            )
            os.close(fd)
            temp_path = Path(temp_name)
            digest = hashlib.sha256()
            try:
                with zf.open(rel, "r") as source, temp_path.open("wb") as output:
                    for chunk in iter(lambda: source.read(CHUNK_SIZE), b""):
                        output.write(chunk)
                        digest.update(chunk)
                if digest.hexdigest() != entry["sha256"]:
                    die(f"sha256 verification failed while extracting: {rel}")
                os.chmod(temp_path, int(entry.get("mode", 0o644)) & 0o777)
                if "mtime_ns" in entry:
                    mtime_ns = int(entry["mtime_ns"])
                    os.utime(temp_path, ns=(mtime_ns, mtime_ns))
                os.replace(temp_path, target)
                written += 1
            finally:
                temp_path.unlink(missing_ok=True)
    print(f"Restored {written} PDFs; skipped {skipped} already-current PDFs.")
    return 0


def command_status(args: argparse.Namespace) -> int:
    root = resolve_root(args.root)
    archive = resolve_archive(root, args.archive)
    excludes = tuple(DEFAULT_EXCLUDES) + tuple(args.exclude)
    local_files = collect_pdf_files(root, args.include, excludes)
    local_map = {rel: path for rel, path in local_files}
    print(f"Local PDFs: {len(local_files)}")
    if not archive.exists():
        warn(f"archive not found, skipping manifest comparison: {archive}")
        tracked = tracked_pdf_paths(root)
        print(f"Tracked PDFs: {len(tracked)}")
        return 1 if tracked else 0

    manifest = load_manifest(archive)
    manifest_map = {str(entry["path"]): entry for entry in manifest["files"]}
    missing = sorted(set(manifest_map) - set(local_map))
    extra = sorted(set(local_map) - set(manifest_map))
    changed = []
    for rel in sorted(set(local_map).intersection(manifest_map)):
        if sha256_file(local_map[rel]) != manifest_map[rel]["sha256"]:
            changed.append(rel)
    print(f"Archive PDFs: {len(manifest_map)}")
    print(f"Missing locally: {len(missing)}")
    print(f"Changed locally: {len(changed)}")
    print(f"Extra locally: {len(extra)}")
    for label, paths in (
        ("missing", missing),
        ("changed", changed),
        ("extra", extra),
    ):
        for rel in paths[:10]:
            print(f"  {label}: {rel}")
        if len(paths) > 10:
            print(f"  ... {len(paths) - 10} more {label}")
    return 1 if missing or changed or extra else 0


def command_git_untrack(args: argparse.Namespace) -> int:
    root = resolve_root(args.root)
    excludes = tuple(DEFAULT_EXCLUDES) + tuple(args.exclude)
    include_prefixes: list[str] = []
    for include in args.include:
        candidate = Path(include).expanduser()
        if not candidate.is_absolute():
            candidate = root / candidate
        include_rel = rel_to_root(root, candidate.resolve())
        include_prefixes.append("" if include_rel == "." else include_rel.rstrip("/"))
    tracked = sorted(
        rel
        for rel in tracked_pdf_paths(root)
        if not is_excluded(rel, excludes)
        and (
            not args.include
            or any(
                not prefix or rel == prefix or rel.startswith(prefix + "/")
                for prefix in include_prefixes
            )
        )
    )
    git_rm_cached(root, tracked, dry_run=args.dry_run)
    return 0


def add_common(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--root",
        help=f"repository root (default: {DEFAULT_ROOT})",
    )
    parser.add_argument(
        "--archive",
        default=DEFAULT_ARCHIVE,
        help=f"zip archive path (default: {DEFAULT_ARCHIVE})",
    )


def add_scan_options(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--include",
        action="append",
        default=[],
        help="file or directory to scan, relative to --root; repeatable (default: .)",
    )
    parser.add_argument(
        "--exclude",
        action="append",
        default=[],
        help="extra fnmatch pattern to exclude, using repo-relative paths",
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Pack repo PDFs into a path-preserving zip, or unpack that zip after "
            "cloning a source-only repo."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    pack = subparsers.add_parser("pack", help="create a PDF archive")
    add_common(pack)
    add_scan_options(pack)
    pack.add_argument("--overwrite", action="store_true", help="replace existing archive")
    pack.add_argument("--dry-run", action="store_true", help="show what would be archived")
    pack.add_argument(
        "--git-rm-cached",
        action="store_true",
        help="after writing the archive, remove tracked PDFs from the git index only",
    )
    pack.add_argument(
        "--no-verify",
        action="store_true",
        help="skip zip CRC verification after writing",
    )
    pack.set_defaults(func=command_pack)

    patch = subparsers.add_parser(
        "patch",
        help="alias for pack plus git index removal of tracked PDFs",
    )
    add_common(patch)
    add_scan_options(patch)
    patch.add_argument("--overwrite", action="store_true", help="replace existing archive")
    patch.add_argument("--dry-run", action="store_true", help="show what would change")
    patch.add_argument(
        "--no-verify",
        action="store_true",
        help="skip zip CRC verification after writing",
    )
    patch.set_defaults(func=lambda args: command_pack(args, patch_mode=True))

    unpack = subparsers.add_parser("unpack", help="restore PDFs from an archive")
    add_common(unpack)
    unpack.add_argument(
        "--overwrite",
        action="store_true",
        help="replace existing PDFs whose bytes differ from the archive",
    )
    unpack.add_argument("--dry-run", action="store_true", help="show what would be restored")
    unpack.set_defaults(func=command_unpack)

    unpatch = subparsers.add_parser("unpatch", help="alias for unpack")
    add_common(unpatch)
    unpatch.add_argument(
        "--overwrite",
        action="store_true",
        help="replace existing PDFs whose bytes differ from the archive",
    )
    unpatch.add_argument("--dry-run", action="store_true", help="show what would be restored")
    unpatch.set_defaults(func=command_unpack)

    status = subparsers.add_parser(
        "status", help="compare local PDFs with an archive manifest"
    )
    add_common(status)
    add_scan_options(status)
    status.set_defaults(func=command_status)

    git_untrack = subparsers.add_parser(
        "git-untrack", help="remove tracked PDFs from the git index only"
    )
    git_untrack.add_argument(
        "--root",
        help=f"repository root (default: {DEFAULT_ROOT})",
    )
    add_scan_options(git_untrack)
    git_untrack.add_argument("--dry-run", action="store_true", help="show git command only")
    git_untrack.set_defaults(func=command_git_untrack)

    return parser


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    return int(args.func(args))


if __name__ == "__main__":
    raise SystemExit(main())
