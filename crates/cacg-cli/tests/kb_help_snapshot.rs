#![allow(clippy::unwrap_used)]
//! Help-snapshot regression for the trust-kernel verbs.
//!
//! `kb lint` and `kb verify` are both backed by real, wired
//! dispatchers (`dispatch_lint` / `dispatch_verify`), yet their clap
//! `about` text historically carried a stale `(unimplemented)` marker
//! copied from the pre-port scaffold. This test pins the invariant
//! that the rendered help for the two implemented trust-kernel verbs
//! never advertises itself as unimplemented; re-introducing the
//! `(unimplemented)` substring into either enum-variant docstring in
//! `crates/cacg-cli/src/lib.rs` fails it.
//!
//! Scope note: this guards ONLY `lint` and `verify`. Other
//! subcommands whose Rust runtime is intentionally absent today
//! legitimately keep the marker, so a whole-`kb --help` assertion
//! would be wrong here.

use std::process::Command;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

/// Spawn `kb <verb> --help`, assert it renders successfully (exit 0),
/// and return its stdout.
fn help_stdout(verb: &str) -> String {
    let output = Command::new(kb_bin())
        .arg(verb)
        .arg("--help")
        .output()
        .unwrap_or_else(|e| panic!("spawn kb {verb} --help: {e}"));
    assert!(
        output.status.success(),
        "kb {verb} --help must exit 0; status={:?}, stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("kb --help stdout must be UTF-8")
}

#[test]
fn lint_help_does_not_advertise_unimplemented() {
    let help = help_stdout("lint");
    assert!(
        !help.to_ascii_lowercase().contains("unimplemented"),
        "kb lint --help must not contain `unimplemented` (lint is wired to dispatch_lint); got:\n{help}"
    );
    // Positive anchors: fail loudly if the about text is gutted rather
    // than merely de-staled, so the negative assertion can't pass
    // vacuously against an empty/changed help surface.
    assert!(
        help.contains("lint") && help.contains("--source-matrix"),
        "kb lint --help must still document the lint verb and its required --source-matrix; got:\n{help}"
    );
}

#[test]
fn verify_help_does_not_advertise_unimplemented() {
    let help = help_stdout("verify");
    assert!(
        !help.to_ascii_lowercase().contains("unimplemented"),
        "kb verify --help must not contain `unimplemented` (verify is wired to dispatch_verify); got:\n{help}"
    );
    assert!(
        help.contains("verify") && help.contains("--source-matrix"),
        "kb verify --help must still document the verify verb and its required --source-matrix; got:\n{help}"
    );
}
