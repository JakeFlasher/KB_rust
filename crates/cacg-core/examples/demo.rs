//! Standalone cacg-core trust-kernel demo. Invoke with one card path:
//!
//!     KB_FROZEN_CLOCK=1 cargo run -p cacg-core --example demo -- \
//!         tests/golden/01-content-addressable-identity.md
//!
//! Exercises every public trust-kernel primitive in order: load card,
//! validate schema, compute `card_hash`, normalize body, append journal
//! event, atomic-publish a 1-card manifest pair, re-load and verify,
//! emit Diagnostic JSON. Uses a fresh `tempfile::TempDir` for the
//! publish artifacts; the tempdir is dropped when the program exits.

use std::path::PathBuf;
use std::process::ExitCode;

use cacg_core::demo::demo_run;
use cacg_core::determinism::DeterminismContext;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "usage: {} <card.md>\nExpected one argument: the path to a card markdown file.",
            args.first().map_or("demo", String::as_str)
        );
        return ExitCode::FAILURE;
    }
    let card_path = PathBuf::from(&args[1]);
    let ctx = DeterminismContext::from_env();
    let tempdir = match tempfile::TempDir::new() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("failed to create tempdir: {e}");
            return ExitCode::FAILURE;
        }
    };
    match demo_run(&card_path, &ctx, tempdir.path()) {
        Ok(report) => {
            println!("{report}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("demo failed: {err}");
            ExitCode::FAILURE
        }
    }
}
