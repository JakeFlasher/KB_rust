#![allow(clippy::unwrap_used)]
//! Multi-file pair-atomic publish discipline: fault-injection
//! coverage of every required failure point + the pre-publish refusal
//! contracts + the post-success cleanup contract.
//!
//! Each test wires an `atomic_publish` call through a `FaultInjector`
//! that can be configured to fail specific `write_tmp` or `rename`
//! invocations matched by their path arguments. After each scenario
//! the test enumerates the publish directory and asserts the on-disk
//! state matches the documented invariant: success leaves the
//! canonicals with new bytes + no sidecars; failure leaves the
//! canonicals in their prior state + no sidecars.

use std::cell::RefCell;
use std::io;
use std::path::{Path, PathBuf};

use cacg_core::atomic_publish::{
    atomic_publish, DefaultFs, FsSyscalls, PublishError, PublishMember,
};
use tempfile::TempDir;

const SIDECAR_DIAG: &str = "CACG-MAN-002";
const NON_FILE_DIAG: &str = "CACG-MAN-003";

/// `FsSyscalls` wrapper that records every call and can be configured
/// to fail specific operations matched by their path arguments.
struct FaultInjector {
    inner: DefaultFs,
    fail_write_tmps: RefCell<Vec<PathBuf>>,
    fail_renames: RefCell<Vec<(PathBuf, PathBuf)>>,
    fail_unlinks: RefCell<Vec<PathBuf>>,
    log: RefCell<Vec<String>>,
}

impl FaultInjector {
    fn new() -> Self {
        Self {
            inner: DefaultFs,
            fail_write_tmps: RefCell::new(Vec::new()),
            fail_renames: RefCell::new(Vec::new()),
            fail_unlinks: RefCell::new(Vec::new()),
            log: RefCell::new(Vec::new()),
        }
    }
    fn fail_write_tmp(&self, path: PathBuf) {
        self.fail_write_tmps.borrow_mut().push(path);
    }
    fn fail_rename(&self, src: PathBuf, dst: PathBuf) {
        self.fail_renames.borrow_mut().push((src, dst));
    }
    fn fail_unlink(&self, path: PathBuf) {
        self.fail_unlinks.borrow_mut().push(path);
    }
}

impl FsSyscalls for FaultInjector {
    fn write_tmp(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        self.log
            .borrow_mut()
            .push(format!("write_tmp:{}", path.display()));
        if self.fail_write_tmps.borrow().iter().any(|p| p == path) {
            return Err(io::Error::other(format!(
                "injected write_tmp failure: {}",
                path.display()
            )));
        }
        self.inner.write_tmp(path, bytes)
    }
    fn rename(&self, src: &Path, dst: &Path) -> io::Result<()> {
        self.log
            .borrow_mut()
            .push(format!("rename:{}->{}", src.display(), dst.display()));
        if self
            .fail_renames
            .borrow()
            .iter()
            .any(|(s, d)| s == src && d == dst)
        {
            return Err(io::Error::other(format!(
                "injected rename failure: {} -> {}",
                src.display(),
                dst.display()
            )));
        }
        self.inner.rename(src, dst)
    }
    fn unlink_if_exists(&self, path: &Path) -> io::Result<()> {
        self.log
            .borrow_mut()
            .push(format!("unlink_if_exists:{}", path.display()));
        // Only fail when the file actually exists: this matches the
        // production semantic where `remove_file` on an absent path
        // returns NotFound (which `unlink_if_exists` converts to Ok).
        // Injecting a failure on a missing file would mis-model the
        // syscall contract.
        if path.exists() && self.fail_unlinks.borrow().iter().any(|p| p == path) {
            return Err(io::Error::other(format!(
                "injected unlink_if_exists failure: {}",
                path.display()
            )));
        }
        self.inner.unlink_if_exists(path)
    }
    fn path_exists(&self, path: &Path) -> bool {
        self.inner.path_exists(path)
    }
    fn path_is_file(&self, path: &Path) -> bool {
        self.inner.path_is_file(path)
    }
    fn mkdir_p(&self, path: &Path) -> io::Result<()> {
        self.inner.mkdir_p(path)
    }
}

/// Build a 2-member pair: `sources_manifest.json` + `chunks_manifest.json`.
fn two_member_pair(dir: &Path, sources_bytes: &[u8], chunks_bytes: &[u8]) -> Vec<PublishMember> {
    let sources = dir.join("sources_manifest.json");
    let chunks = dir.join("chunks_manifest.json");
    vec![
        PublishMember {
            tmp_path: sources.with_extension("json.tmp"),
            bak_path: sources.with_extension("json.bak"),
            canonical_path: sources,
            bytes: sources_bytes.to_vec(),
        },
        PublishMember {
            tmp_path: chunks.with_extension("json.tmp"),
            bak_path: chunks.with_extension("json.bak"),
            canonical_path: chunks,
            bytes: chunks_bytes.to_vec(),
        },
    ]
}

fn one_member(dir: &Path, name: &str, bytes: &[u8]) -> Vec<PublishMember> {
    let canonical = dir.join(name);
    let tmp = canonical.with_extension(format!(
        "{}.tmp",
        canonical.extension().and_then(|s| s.to_str()).unwrap_or("")
    ));
    let bak = canonical.with_extension(format!(
        "{}.bak",
        canonical.extension().and_then(|s| s.to_str()).unwrap_or("")
    ));
    vec![PublishMember {
        canonical_path: canonical,
        tmp_path: tmp,
        bak_path: bak,
        bytes: bytes.to_vec(),
    }]
}

/// Return the names of all entries in `dir` (sorted) so a test can
/// assert exactly which files are present after a scenario.
fn list_dir(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    names
}

#[test]
fn success_2_file_pair_with_no_prior() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG).expect("publish ok");
    assert_eq!(
        list_dir(dir.path()),
        vec!["chunks_manifest.json", "sources_manifest.json"]
    );
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"sources-new"
    );
    assert_eq!(
        std::fs::read(&members[1].canonical_path).unwrap(),
        b"chunks-new"
    );
}

#[test]
fn success_2_file_pair_with_prior() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    std::fs::write(&members[0].canonical_path, b"sources-old").unwrap();
    std::fs::write(&members[1].canonical_path, b"chunks-old").unwrap();
    atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG).expect("publish ok");
    assert_eq!(
        list_dir(dir.path()),
        vec!["chunks_manifest.json", "sources_manifest.json"],
        "no `.tmp` / `.bak` sidecar must be left on success"
    );
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"sources-new"
    );
    assert_eq!(
        std::fs::read(&members[1].canonical_path).unwrap(),
        b"chunks-new"
    );
}

#[test]
fn success_1_file_publish_with_no_prior() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = one_member(dir.path(), "summaries.json", b"summaries-new");
    atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG).expect("publish ok");
    assert_eq!(list_dir(dir.path()), vec!["summaries.json"]);
}

#[test]
fn success_1_file_publish_with_prior() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = one_member(dir.path(), "summaries.json", b"summaries-new");
    std::fs::write(&members[0].canonical_path, b"summaries-old").unwrap();
    atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG).expect("publish ok");
    assert_eq!(list_dir(dir.path()), vec!["summaries.json"]);
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"summaries-new"
    );
}

#[test]
fn fail_at_tmp_write_first_member() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    std::fs::write(&members[0].canonical_path, b"sources-old").unwrap();
    std::fs::write(&members[1].canonical_path, b"chunks-old").unwrap();
    fs.fail_write_tmp(members[0].tmp_path.clone());
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    assert!(
        matches!(r, Err(PublishError::Io { .. })),
        "expected Io err, got {r:?}"
    );
    assert_eq!(
        list_dir(dir.path()),
        vec!["chunks_manifest.json", "sources_manifest.json"],
        "first-tmp failure leaves on-disk state untouched"
    );
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"sources-old"
    );
    assert_eq!(
        std::fs::read(&members[1].canonical_path).unwrap(),
        b"chunks-old"
    );
}

#[test]
fn fail_at_tmp_write_second_member() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    std::fs::write(&members[0].canonical_path, b"sources-old").unwrap();
    std::fs::write(&members[1].canonical_path, b"chunks-old").unwrap();
    fs.fail_write_tmp(members[1].tmp_path.clone());
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    assert!(matches!(r, Err(PublishError::Io { .. })));
    assert_eq!(
        list_dir(dir.path()),
        vec!["chunks_manifest.json", "sources_manifest.json"],
        "second-tmp failure must clean up first tmp"
    );
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"sources-old"
    );
}

#[test]
fn fail_at_bak_snapshot_first_canonical() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    std::fs::write(&members[0].canonical_path, b"sources-old").unwrap();
    std::fs::write(&members[1].canonical_path, b"chunks-old").unwrap();
    fs.fail_rename(
        members[0].canonical_path.clone(),
        members[0].bak_path.clone(),
    );
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    assert!(matches!(r, Err(PublishError::Io { .. })));
    assert_eq!(
        list_dir(dir.path()),
        vec!["chunks_manifest.json", "sources_manifest.json"],
        "snapshot failure must clean up all tmps + bakkups"
    );
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"sources-old"
    );
    assert_eq!(
        std::fs::read(&members[1].canonical_path).unwrap(),
        b"chunks-old"
    );
}

#[test]
fn fail_at_replace_first_canonical() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    std::fs::write(&members[0].canonical_path, b"sources-old").unwrap();
    std::fs::write(&members[1].canonical_path, b"chunks-old").unwrap();
    fs.fail_rename(
        members[0].tmp_path.clone(),
        members[0].canonical_path.clone(),
    );
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    assert!(matches!(r, Err(PublishError::Io { .. })));
    assert_eq!(
        list_dir(dir.path()),
        vec!["chunks_manifest.json", "sources_manifest.json"],
        "first-replace failure must restore prior canonicals and clean every sidecar"
    );
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"sources-old"
    );
    assert_eq!(
        std::fs::read(&members[1].canonical_path).unwrap(),
        b"chunks-old"
    );
}

#[test]
fn fail_at_replace_second_canonical() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    std::fs::write(&members[0].canonical_path, b"sources-old").unwrap();
    std::fs::write(&members[1].canonical_path, b"chunks-old").unwrap();
    fs.fail_rename(
        members[1].tmp_path.clone(),
        members[1].canonical_path.clone(),
    );
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    assert!(matches!(r, Err(PublishError::Io { .. })));
    assert_eq!(
        list_dir(dir.path()),
        vec!["chunks_manifest.json", "sources_manifest.json"],
        "second-replace failure must restore BOTH canonicals and clean every sidecar"
    );
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"sources-old",
        "first canonical must be RESTORED from its `.bak` after the second-replace failure"
    );
    assert_eq!(
        std::fs::read(&members[1].canonical_path).unwrap(),
        b"chunks-old"
    );
}

#[test]
fn fail_at_replace_second_canonical_with_no_prior_for_first() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    std::fs::write(&members[1].canonical_path, b"chunks-old").unwrap();
    fs.fail_rename(
        members[1].tmp_path.clone(),
        members[1].canonical_path.clone(),
    );
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    assert!(matches!(r, Err(PublishError::Io { .. })));
    assert_eq!(
        list_dir(dir.path()),
        vec!["chunks_manifest.json"],
        "no-prior member must be UNLINKED on second-replace failure (not left at sources-new)"
    );
    assert_eq!(
        std::fs::read(&members[1].canonical_path).unwrap(),
        b"chunks-old"
    );
}

#[test]
fn refuse_pre_existing_tmp_sidecar() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"new", b"new");
    std::fs::write(&members[0].tmp_path, b"leftover").unwrap();
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    match r {
        Err(PublishError::PreexistingSidecars { diagnostic, paths }) => {
            assert_eq!(diagnostic, SIDECAR_DIAG);
            assert_eq!(paths, vec![members[0].tmp_path.clone()]);
        }
        other => panic!("expected PreexistingSidecars, got {other:?}"),
    }
    assert_eq!(
        list_dir(dir.path()),
        vec!["sources_manifest.json.tmp"],
        "leftover tmp must be preserved for human inspection"
    );
}

#[test]
fn refuse_pre_existing_bak_sidecar() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"new", b"new");
    std::fs::write(&members[1].bak_path, b"leftover").unwrap();
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    match r {
        Err(PublishError::PreexistingSidecars { diagnostic, paths }) => {
            assert_eq!(diagnostic, SIDECAR_DIAG);
            assert!(paths.contains(&members[1].bak_path));
        }
        other => panic!("expected PreexistingSidecars, got {other:?}"),
    }
}

#[test]
fn refuse_non_file_canonical_target() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"new", b"new");
    std::fs::create_dir(&members[0].canonical_path).unwrap();
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    match r {
        Err(PublishError::NonFileCanonical { diagnostic, paths }) => {
            assert_eq!(diagnostic, NON_FILE_DIAG);
            assert_eq!(paths, vec![members[0].canonical_path.clone()]);
        }
        other => panic!("expected NonFileCanonical, got {other:?}"),
    }
    assert!(
        members[0].canonical_path.is_dir(),
        "the directory must NOT have been touched by the refusal"
    );
}

#[test]
fn cleanup_leaves_no_bak_on_success_with_prior() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"new", b"new");
    std::fs::write(&members[0].canonical_path, b"old-a").unwrap();
    std::fs::write(&members[1].canonical_path, b"old-b").unwrap();
    atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG).expect("publish");
    let entries = list_dir(dir.path());
    assert!(
        entries.iter().all(|n| {
            let p = std::path::Path::new(n);
            !p.extension().is_some_and(|e| e.eq_ignore_ascii_case("bak"))
                && !p.extension().is_some_and(|e| e.eq_ignore_ascii_case("tmp"))
        }),
        "post-success directory MUST be free of every sidecar: got {entries:?}"
    );
    let unlink_count = fs
        .log
        .borrow()
        .iter()
        .filter(|s| s.starts_with("unlink_if_exists:"))
        .count();
    assert!(
        unlink_count >= members.len(),
        "expected at least {} bak-unlink calls on success; saw {} fs ops: {:?}",
        members.len(),
        unlink_count,
        fs.log.borrow()
    );
}

#[test]
fn recovery_from_leftover_bak_after_crash_refuses_next_publish() {
    let dir = TempDir::new().unwrap();
    let members = two_member_pair(dir.path(), b"new", b"new");
    std::fs::write(&members[0].canonical_path, b"a-current").unwrap();
    std::fs::write(&members[1].canonical_path, b"b-current").unwrap();
    // Simulate a crash that left a `.bak` on disk from a prior run.
    std::fs::write(&members[1].bak_path, b"b-crash-remnant").unwrap();

    // Next publish attempt must refuse so a human can inspect the
    // leftover, not silently overwrite it.
    let fs = FaultInjector::new();
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    match r {
        Err(PublishError::PreexistingSidecars { diagnostic, paths }) => {
            assert_eq!(diagnostic, SIDECAR_DIAG);
            assert!(paths.contains(&members[1].bak_path));
        }
        other => panic!("expected PreexistingSidecars, got {other:?}"),
    }
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"a-current",
        "refusal must not touch the canonical state"
    );
    assert_eq!(
        std::fs::read(&members[1].bak_path).unwrap(),
        b"b-crash-remnant",
        "leftover bak must remain on disk for human inspection"
    );
}

/// Build a 4-member index-style publish (`cards_manifest.json` + INDEX.md
/// plus 2 additional sidecar files exercising the N=4 path). The four
/// distinct extensions force the engine to derive correct `.tmp` / `.bak`
/// sidecars per member.
fn four_member_index(dir: &Path, bytes: [&[u8]; 4]) -> Vec<PublishMember> {
    let names = [
        "cards_manifest.json",
        "INDEX.md",
        "summaries.json",
        "sources_manifest.json",
    ];
    names
        .iter()
        .zip(bytes.iter())
        .map(|(name, payload)| {
            let canonical = dir.join(name);
            let ext = canonical
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            let tmp = canonical.with_extension(format!("{ext}.tmp"));
            let bak = canonical.with_extension(format!("{ext}.bak"));
            PublishMember {
                canonical_path: canonical,
                tmp_path: tmp,
                bak_path: bak,
                bytes: payload.to_vec(),
            }
        })
        .collect()
}

#[test]
fn success_4_file_index_with_priors() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = four_member_index(
        dir.path(),
        [
            b"manifest-new",
            b"index-new",
            b"summaries-new",
            b"sources-new",
        ],
    );
    let priors: [&[u8]; 4] = [
        b"manifest-old",
        b"index-old",
        b"summaries-old",
        b"sources-old",
    ];
    for (i, prior) in priors.iter().enumerate() {
        std::fs::write(&members[i].canonical_path, prior).unwrap();
    }
    atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG).expect("publish ok");
    assert_eq!(
        list_dir(dir.path()),
        vec![
            "INDEX.md",
            "cards_manifest.json",
            "sources_manifest.json",
            "summaries.json"
        ],
        "post-success directory must hold exactly the 4 canonicals; no sidecars"
    );
    let expected_bytes = [
        b"manifest-new" as &[u8],
        b"index-new",
        b"summaries-new",
        b"sources-new",
    ];
    for (i, want) in expected_bytes.iter().enumerate() {
        assert_eq!(
            std::fs::read(&members[i].canonical_path).unwrap(),
            *want,
            "member {i} must hold new bytes"
        );
    }
}

#[test]
fn success_4_file_index_with_no_priors() {
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = four_member_index(
        dir.path(),
        [
            b"manifest-new",
            b"index-new",
            b"summaries-new",
            b"sources-new",
        ],
    );
    atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG).expect("publish ok");
    assert_eq!(
        list_dir(dir.path()),
        vec![
            "INDEX.md",
            "cards_manifest.json",
            "sources_manifest.json",
            "summaries.json"
        ]
    );
    // No-prior case must not create any `.bak` files (best-effort
    // cleanup would unlink them anyway, but the snapshot phase
    // shouldn't run at all when no canonical exists).
    let bak_count = fs
        .log
        .borrow()
        .iter()
        .filter(|s| s.contains(".bak"))
        .filter(|s| !s.starts_with("unlink_if_exists:"))
        .count();
    assert_eq!(
        bak_count, 0,
        "no-prior publish must NOT rename any canonical→bak"
    );
}

#[test]
fn fail_at_replace_fourth_canonical_with_mixed_priors() {
    // Mixed prior/no-prior scenario: members 0 and 2 have priors;
    // members 1 and 3 do not. Failure injected at the 4th tmp→canonical
    // rename so rollback must:
    //  - restore members 0 + 2 from `.bak`
    //  - unlink the newly-created canonicals for members 1 + 3 (no prior)
    //  - clean up all `.tmp` (none should remain since all 4 were
    //    staged; member 3's tmp is the one that failed to replace)
    //  - clean up all `.bak` snapshots
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = four_member_index(dir.path(), [b"m0-new", b"m1-new", b"m2-new", b"m3-new"]);
    std::fs::write(&members[0].canonical_path, b"m0-old").unwrap();
    std::fs::write(&members[2].canonical_path, b"m2-old").unwrap();
    // Members 1 and 3 have no prior canonical.

    fs.fail_rename(
        members[3].tmp_path.clone(),
        members[3].canonical_path.clone(),
    );
    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    assert!(matches!(r, Err(PublishError::Io { .. })));

    let entries = list_dir(dir.path());
    assert_eq!(
        entries,
        vec!["cards_manifest.json", "summaries.json"],
        "after rollback only the members with priors (0 + 2) must exist; got {entries:?}"
    );
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"m0-old",
        "member 0 must be RESTORED from its `.bak`"
    );
    assert_eq!(
        std::fs::read(&members[2].canonical_path).unwrap(),
        b"m2-old",
        "member 2 must be RESTORED from its `.bak`"
    );
    assert!(
        !members[1].canonical_path.exists(),
        "member 1 had no prior; its new canonical must be UNLINKED"
    );
    assert!(
        !members[3].canonical_path.exists(),
        "member 3 had no prior and failed mid-replace; its canonical must NOT exist"
    );
}

#[test]
fn post_success_bak_cleanup_failure_still_returns_ok() {
    // Cleanup is best-effort: a failure to unlink a `.bak` after the
    // publish has committed must NOT trigger rollback (the data has
    // already been published; rolling back would undo the commit).
    // The stranded `.bak` then triggers the NEXT publish's
    // PreexistingSidecars refusal so a human can investigate.
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    std::fs::write(&members[0].canonical_path, b"sources-old").unwrap();
    std::fs::write(&members[1].canonical_path, b"chunks-old").unwrap();
    fs.fail_unlink(members[0].bak_path.clone());

    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    assert!(
        r.is_ok(),
        "cleanup failure after commit must NOT roll back: {r:?}"
    );
    // The new canonicals are in place.
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"sources-new"
    );
    assert_eq!(
        std::fs::read(&members[1].canonical_path).unwrap(),
        b"chunks-new"
    );
    // The stranded `.bak` remains on disk for human inspection.
    assert!(
        members[0].bak_path.exists(),
        "the bak whose unlink failed must remain on disk"
    );

    // Next publish must refuse rather than silently clobber the stranded sidecar.
    let fs2 = FaultInjector::new();
    let next = two_member_pair(dir.path(), b"sources-v2", b"chunks-v2");
    let r2 = atomic_publish(&next, &fs2, SIDECAR_DIAG, NON_FILE_DIAG);
    match r2 {
        Err(PublishError::PreexistingSidecars { diagnostic, paths }) => {
            assert_eq!(diagnostic, SIDECAR_DIAG);
            assert!(paths.contains(&next[0].bak_path));
        }
        other => panic!("expected PreexistingSidecars from next publish, got {other:?}"),
    }
}

#[test]
fn rollback_tmp_cleanup_failure_strands_tmp_but_returns_original_io_error() {
    // Mid-rollback cleanup failure must NOT mask the original
    // PublishError::Io that triggered the rollback. The canonical
    // state still gets restored (the `.bak.replace` step happens
    // BEFORE the cleanup loop), but the stranded `.tmp` then triggers
    // the next publish's PreexistingSidecars refusal so a human can
    // investigate the leftover.
    //
    // Note on choice of stranded sidecar: rollback restores each
    // snapshotted canonical from its `.bak` BEFORE the bak cleanup
    // loop runs, so under normal flow every bak is renamed away
    // before unlink would touch it; the bak cleanup loop is therefore
    // a no-op for restored members. The `.tmp` unlink, by contrast,
    // runs on a tmp that was successfully staged but never replaced,
    // so a failure here legitimately strands a sidecar on disk.
    let dir = TempDir::new().unwrap();
    let fs = FaultInjector::new();
    let members = two_member_pair(dir.path(), b"sources-new", b"chunks-new");
    std::fs::write(&members[0].canonical_path, b"sources-old").unwrap();
    std::fs::write(&members[1].canonical_path, b"chunks-old").unwrap();
    // Trigger rollback by failing the SECOND replace; AND fail the
    // unlink of member 1's tmp during rollback cleanup. (Member 1's
    // tmp was staged but never replaced, so it survives until the
    // rollback unlink loop.)
    fs.fail_rename(
        members[1].tmp_path.clone(),
        members[1].canonical_path.clone(),
    );
    fs.fail_unlink(members[1].tmp_path.clone());

    let r = atomic_publish(&members, &fs, SIDECAR_DIAG, NON_FILE_DIAG);
    assert!(
        matches!(r, Err(PublishError::Io { .. })),
        "must surface the original rename failure, not the cleanup failure: {r:?}"
    );
    assert_eq!(
        std::fs::read(&members[0].canonical_path).unwrap(),
        b"sources-old",
        "member 0's canonical must be restored from its `.bak` even when cleanup later fails"
    );
    assert_eq!(
        std::fs::read(&members[1].canonical_path).unwrap(),
        b"chunks-old",
        "member 1's canonical was never replaced; it stays at prior bytes"
    );
    assert!(
        members[1].tmp_path.exists(),
        "the tmp whose unlink failed during rollback must remain on disk"
    );

    // Next publish must refuse rather than silently clobber the
    // stranded sidecar.
    let fs2 = FaultInjector::new();
    let next = two_member_pair(dir.path(), b"sources-v2", b"chunks-v2");
    let r2 = atomic_publish(&next, &fs2, SIDECAR_DIAG, NON_FILE_DIAG);
    match r2 {
        Err(PublishError::PreexistingSidecars { diagnostic, paths }) => {
            assert_eq!(diagnostic, SIDECAR_DIAG);
            assert!(paths.contains(&next[1].tmp_path));
        }
        other => panic!("expected PreexistingSidecars from next publish, got {other:?}"),
    }
}
