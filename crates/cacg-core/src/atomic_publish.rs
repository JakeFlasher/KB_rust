//! Crash-recoverable multi-file pair-atomic publish.
//!
//! Single API that drives single-file, 2-file pair, and N-file
//! publish discipline:
//! 1. For each canonical target, write `.tmp` (caller supplies the
//!    pre-validated bytes).
//! 2. For each canonical that exists, `rename(canonical, .bak)` so a
//!    later step can roll back.
//! 3. For each member in order, `rename(.tmp, canonical)`.
//! 4. On success: unlink every `.bak` (best-effort; cleanup failure
//!    does NOT trigger rollback because the publish has committed at
//!    this point).
//! 5. On any failure during steps 1-3: restore canonicals from `.bak`
//!    (or unlink new canonicals where no prior existed), unlink any
//!    `.tmp` we wrote that did not get replaced, and unlink any `.bak`
//!    snapshots we created. The directory ends in the prior canonical
//!    state.
//!
//! The single `os.rename` syscall is atomic on POSIX. For multi-file
//! publishes the sequence of renames is NOT transactional across
//! files, but the per-file atomicity combined with the `.bak`
//! rollback closes the window between renames so no partial valid
//! state is observable to outside readers.
//!
//! `BL-20260518-multi-file-replace-rollback` directly applies: this
//! module is the Rust port of the lesson's documented pair-atomic
//! pattern, including the
//! `BL-20260518-no-prior-pair-atomic` edge case (rollback unlinks the
//! newly-created canonical when no prior existed).
//!
//! `BL-20260518-no-clobber-foreign-sidecars` applies to the
//! pre-publish sidecar refusal: an existing `.tmp` or `.bak` could be
//! a leftover from a prior crash, a user-owned backup, or another
//! tool's artifact; silently overwriting / unlinking it would destroy
//! data the publisher does not own.
//!
//! `BL-20260518-shape-check-fs-inputs` applies to the non-file
//! canonical refusal: a directory at the canonical path would be
//! moved to `.bak` and then the success-cleanup `unlink` would crash,
//! leaving partial state on disk.
//!
//! The diagnostic strings (`CACG-MAN-002`, `CACG-MAN-003`,
//! `CACG-IDX-007`, `CACG-IDX-008`, `CACG-IDX-006`) are caller-supplied
//! so this module stays independent of the downstream manifest /
//! index modules that own those code namespaces.

use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// Filesystem syscalls used by [`atomic_publish`]. Pulled behind a
/// trait so tests can inject failures at any step without monkey-
/// patching `std::fs`. `DefaultFs` is the production implementation.
pub trait FsSyscalls {
    /// Write `bytes` to `path` (creating or truncating). A failure
    /// may leave a partial file on disk; the publisher's rollback
    /// path is responsible for unlinking it.
    fn write_tmp(&self, path: &Path, bytes: &[u8]) -> io::Result<()>;
    /// Atomic POSIX rename. Used for canonical-to-.bak snapshots,
    /// tmp-to-canonical replacement, and .bak-to-canonical rollback.
    fn rename(&self, src: &Path, dst: &Path) -> io::Result<()>;
    /// Best-effort unlink. Returns `Ok(())` if the path is already
    /// absent; any other failure surfaces to the caller.
    fn unlink_if_exists(&self, path: &Path) -> io::Result<()>;
    /// `true` iff the path currently exists (any file type).
    fn path_exists(&self, path: &Path) -> bool;
    /// `true` iff the path exists AND is a regular file.
    fn path_is_file(&self, path: &Path) -> bool;
    /// Idempotent `mkdir -p` for the publish output directory.
    fn mkdir_p(&self, path: &Path) -> io::Result<()>;
}

/// Production [`FsSyscalls`] implementation that wraps `std::fs`.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultFs;

impl FsSyscalls for DefaultFs {
    fn write_tmp(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        std::fs::write(path, bytes)
    }
    fn rename(&self, src: &Path, dst: &Path) -> io::Result<()> {
        std::fs::rename(src, dst)
    }
    fn unlink_if_exists(&self, path: &Path) -> io::Result<()> {
        match std::fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e),
        }
    }
    fn path_exists(&self, path: &Path) -> bool {
        path.exists()
    }
    fn path_is_file(&self, path: &Path) -> bool {
        path.is_file()
    }
    fn mkdir_p(&self, path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(path)
    }
}

/// One file in a publish transaction: its canonical path, the
/// `.tmp` / `.bak` sidecar paths the publisher will use, and the
/// pre-validated bytes the caller wants installed at the canonical
/// path. The caller owns the byte serialization; the publisher only
/// commits bytes.
#[derive(Debug, Clone)]
pub struct PublishMember {
    /// Final on-disk path where `bytes` will live after success.
    pub canonical_path: PathBuf,
    /// Staging path used during the publish. MUST be in the same
    /// directory as `canonical_path` so POSIX `rename` is atomic.
    pub tmp_path: PathBuf,
    /// Snapshot of the prior canonical (if any) for rollback. MUST
    /// be in the same directory as `canonical_path`.
    pub bak_path: PathBuf,
    /// Bytes to write to `canonical_path` (already canonicalized and
    /// Pydantic-/serde-validated by the caller).
    pub bytes: Vec<u8>,
}

/// Errors raised by [`atomic_publish`]. After any of these errors the
/// publisher has rolled back to the prior canonical state.
#[derive(Debug, Error)]
pub enum PublishError {
    /// One or more `.tmp` / `.bak` sidecar paths already exist. The
    /// publisher refuses to clobber them. `diagnostic` is the
    /// caller-supplied diagnostic code (`CACG-MAN-002`,
    /// `CACG-IDX-006`, or `CACG-IDX-007` depending on the caller).
    #[error("{diagnostic}: refusing to clobber existing sidecar(s): {paths:?}")]
    PreexistingSidecars {
        /// Caller-supplied diagnostic code.
        diagnostic: &'static str,
        /// The sidecar paths that already exist.
        paths: Vec<PathBuf>,
    },
    /// One or more canonical targets exist but are not regular files.
    /// `diagnostic` is the caller-supplied diagnostic code
    /// (`CACG-MAN-003` or `CACG-IDX-008`).
    #[error("{diagnostic}: refusing to overwrite non-file canonical: {paths:?}")]
    NonFileCanonical {
        /// Caller-supplied diagnostic code.
        diagnostic: &'static str,
        /// The canonical paths that exist but are not regular files.
        paths: Vec<PathBuf>,
    },
    /// I/O error during the publish; the publisher has already rolled
    /// back to the prior canonical state by the time this is returned.
    #[error("publish I/O error: {source}")]
    Io {
        /// Underlying I/O failure that triggered the rollback.
        #[source]
        source: io::Error,
    },
}

impl PublishError {
    fn io(source: io::Error) -> Self {
        PublishError::Io { source }
    }
}

/// Publish `members` atomically. Returns `Ok(())` after every
/// canonical has been replaced and every `.bak` cleaned up. On any
/// I/O failure the publisher rolls back to the prior canonical state
/// before returning [`PublishError::Io`].
///
/// `sidecar_diagnostic` is returned in [`PublishError::PreexistingSidecars`]
/// when a `.tmp` / `.bak` already exists (e.g., `CACG-MAN-002` for
/// the manifest publisher, `CACG-IDX-007` for the index publisher).
/// `non_file_diagnostic` is returned in [`PublishError::NonFileCanonical`]
/// when a canonical target exists but is not a regular file.
///
/// Empty `members` is a no-op success.
pub fn atomic_publish<S: FsSyscalls>(
    members: &[PublishMember],
    syscalls: &S,
    sidecar_diagnostic: &'static str,
    non_file_diagnostic: &'static str,
) -> Result<(), PublishError> {
    if members.is_empty() {
        return Ok(());
    }

    let mut existing_sidecars: Vec<PathBuf> = Vec::new();
    for m in members {
        if syscalls.path_exists(&m.tmp_path) {
            existing_sidecars.push(m.tmp_path.clone());
        }
        if syscalls.path_exists(&m.bak_path) {
            existing_sidecars.push(m.bak_path.clone());
        }
    }
    if !existing_sidecars.is_empty() {
        return Err(PublishError::PreexistingSidecars {
            diagnostic: sidecar_diagnostic,
            paths: existing_sidecars,
        });
    }

    let mut bad_canonicals: Vec<PathBuf> = Vec::new();
    for m in members {
        if syscalls.path_exists(&m.canonical_path) && !syscalls.path_is_file(&m.canonical_path) {
            bad_canonicals.push(m.canonical_path.clone());
        }
    }
    if !bad_canonicals.is_empty() {
        return Err(PublishError::NonFileCanonical {
            diagnostic: non_file_diagnostic,
            paths: bad_canonicals,
        });
    }

    if let Some(first) = members.first() {
        if let Some(parent) = first
            .canonical_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
        {
            if let Err(e) = syscalls.mkdir_p(parent) {
                return Err(PublishError::io(e));
            }
        }
    }

    let mut tmps_written: Vec<usize> = Vec::with_capacity(members.len());
    let mut snapshotted: Vec<bool> = vec![false; members.len()];
    let mut replaced: Vec<bool> = vec![false; members.len()];

    let result: Result<(), io::Error> = (|| -> Result<(), io::Error> {
        for (i, m) in members.iter().enumerate() {
            syscalls.write_tmp(&m.tmp_path, &m.bytes)?;
            tmps_written.push(i);
        }
        for (i, m) in members.iter().enumerate() {
            if syscalls.path_exists(&m.canonical_path) {
                syscalls.rename(&m.canonical_path, &m.bak_path)?;
                snapshotted[i] = true;
            }
        }
        for (i, m) in members.iter().enumerate() {
            syscalls.rename(&m.tmp_path, &m.canonical_path)?;
            replaced[i] = true;
        }
        Ok(())
    })();

    if let Err(source) = result {
        for (i, m) in members.iter().enumerate() {
            if replaced[i] {
                if snapshotted[i] {
                    let _ = syscalls.rename(&m.bak_path, &m.canonical_path);
                } else {
                    let _ = syscalls.unlink_if_exists(&m.canonical_path);
                }
            } else if snapshotted[i] {
                let _ = syscalls.rename(&m.bak_path, &m.canonical_path);
            }
        }
        for i in &tmps_written {
            if !replaced[*i] {
                let _ = syscalls.unlink_if_exists(&members[*i].tmp_path);
            }
        }
        for (i, m) in members.iter().enumerate() {
            if snapshotted[i] {
                let _ = syscalls.unlink_if_exists(&m.bak_path);
            }
        }
        return Err(PublishError::io(source));
    }

    for (i, m) in members.iter().enumerate() {
        if snapshotted[i] {
            let _ = syscalls.unlink_if_exists(&m.bak_path);
        }
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn default_fs_write_then_rename_then_unlink_round_trip() {
        let dir = TempDir::new().unwrap();
        let fs = DefaultFs;
        let tmp = dir.path().join("a.tmp");
        let canonical = dir.path().join("a.json");
        fs.write_tmp(&tmp, b"hello").unwrap();
        assert!(fs.path_exists(&tmp));
        assert!(fs.path_is_file(&tmp));
        fs.rename(&tmp, &canonical).unwrap();
        assert!(!fs.path_exists(&tmp));
        assert!(fs.path_is_file(&canonical));
        fs.unlink_if_exists(&canonical).unwrap();
        assert!(!fs.path_exists(&canonical));
        fs.unlink_if_exists(&canonical).unwrap();
    }

    #[test]
    fn empty_members_is_noop_success() {
        let fs = DefaultFs;
        let result = atomic_publish::<DefaultFs>(&[], &fs, "X", "Y");
        assert!(result.is_ok());
    }
}
