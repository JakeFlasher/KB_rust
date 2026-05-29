#![allow(
    clippy::map_unwrap_or,
    clippy::wildcard_enum_match_arm,
    clippy::unwrap_used
)]
//! Semantic-help parity for the implemented argument-inventory checks.
//!
//! Shells out to `legacy_python_oracle/scripts/export_argparse_tree.py` to capture Python
//! argparse's parse tree as canonical JSON, introspects Rust clap via
//! `cacg_cli::command()`, and asserts both parsers agree on:
//!
//!   1. Set equality of the 14 subcommand names.
//!   2. `--source-matrix` is a REQUIRED flag on `lint` / `verify` /
//!      `search` / `show` in BOTH parsers.
//!   3. Attribution: invoking the binary `kb <verb> <every-other-required-
//!      positional>` (omitting `--source-matrix`) exits 2 in clap,
//!      matching argparse's usage-error contract. Each subcommand's
//!      positional comes from the Python tree so the test cannot
//!      drift if Python adds a new required positional later. The
//!      same attribution check is also run against Python `-m
//!      cacg.cli` so both implementations exit 2.
//!   4. Full per-subcommand surface equality: ordered positional list
//!      (dest + required + default), option set (dest + long names +
//!      required + default), and mutex-group membership sets. Default
//!      values are normalized to a canonical `Option<String>` so that
//!      argparse's typed defaults (`null`, `"out"`, `10`, `False`)
//!      compare byte-for-byte with clap's `default_value(...)` /
//!      `ArgAction::SetTrue` representation.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

/// The 14 canonical subcommand names. Used as a fixed expectation
/// against both parsers; if Python or Rust diverges, the test
/// surfaces the symmetric-difference set.
const EXPECTED_SUBCOMMAND_NAMES: &[&str] = &[
    "ingest",
    "new",
    "lint",
    "verify",
    "history",
    "index",
    "retract",
    "retract-source",
    "retract-chunk",
    "scaffold-matrix",
    "scaffold-role-map",
    "search",
    "show",
    "migrate-summaries",
];

/// Subcommands whose trust-kernel contract requires `--source-matrix`.
const SOURCE_MATRIX_REQUIRED: &[&str] = &["lint", "verify", "search", "show"];

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn python_exe(ws: &std::path::Path) -> PathBuf {
    // Post-quarantine resolver: return the legacy oracle venv unconditionally.
    // If the venv is missing, the probe in the calling test will fail (since
    // `Command::new(non-existent).output()` returns Err) and the test will
    // skip cleanly. No silent fallback to system `python3` — running the
    // help-parity test against an interpreter without `cacg.cli` installed
    // would surface as a spurious assertion failure rather than a clean skip.
    ws.join("legacy_python_oracle/.venv/bin/python")
}

/// Skip the test gracefully when the host cannot run Python or the
/// shim. Returns the captured JSON tree on success.
fn run_shim_or_skip() -> Option<Value> {
    let ws = workspace_root();
    let shim = ws.join("legacy_python_oracle/scripts/export_argparse_tree.py");
    if !shim.is_file() {
        eprintln!("skipping semantic-help parity test: shim not present at {shim:?}");
        return None;
    }
    let python = python_exe(&ws);
    let probe = Command::new(&python).arg("--version").output();
    if !matches!(&probe, Ok(o) if o.status.success()) {
        eprintln!("skipping semantic-help parity test: python not runnable at {python:?}");
        return None;
    }
    let output = Command::new(&python)
        .arg(&shim)
        .output()
        .expect("spawn export_argparse_tree.py");
    assert!(
        output.status.success(),
        "shim should exit 0; got status={:?}, stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("shim stdout must be UTF-8");
    let tree: Value = serde_json::from_str(&stdout).expect("shim stdout must be canonical JSON");
    Some(tree)
}

fn python_subcommand_names(tree: &Value) -> BTreeSet<String> {
    tree["subcommands"]
        .as_array()
        .expect("subcommands array")
        .iter()
        .map(|s| {
            s["name"]
                .as_str()
                .expect("subcommand.name is a string")
                .to_string()
        })
        .collect()
}

fn rust_subcommand_names() -> BTreeSet<String> {
    cacg_cli::command()
        .get_subcommands()
        .map(|c| c.get_name().to_string())
        // Filter out clap's auto-generated `help` subcommand which is
        // not part of the Python surface.
        .filter(|n| n != "help")
        .collect()
}

#[test]
fn rust_clap_lists_exactly_the_14_expected_subcommand_names() {
    let actual = rust_subcommand_names();
    let expected: BTreeSet<String> = EXPECTED_SUBCOMMAND_NAMES
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let only_in_rust: Vec<&String> = actual.difference(&expected).collect();
    let only_in_expected: Vec<&String> = expected.difference(&actual).collect();
    assert!(
        only_in_rust.is_empty() && only_in_expected.is_empty(),
        "Rust subcommand-name set diverges from expected. only_in_rust={only_in_rust:?}; missing_from_rust={only_in_expected:?}"
    );
}

#[test]
fn python_argparse_lists_exactly_the_14_expected_subcommand_names() {
    let Some(tree) = run_shim_or_skip() else {
        return;
    };
    let actual = python_subcommand_names(&tree);
    let expected: BTreeSet<String> = EXPECTED_SUBCOMMAND_NAMES
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let only_in_python: Vec<&String> = actual.difference(&expected).collect();
    let only_in_expected: Vec<&String> = expected.difference(&actual).collect();
    assert!(
        only_in_python.is_empty() && only_in_expected.is_empty(),
        "Python subcommand-name set diverges from expected. only_in_python={only_in_python:?}; missing_from_python={only_in_expected:?}"
    );
}

#[test]
fn rust_and_python_subcommand_sets_are_equal() {
    let Some(tree) = run_shim_or_skip() else {
        return;
    };
    let rust = rust_subcommand_names();
    let python = python_subcommand_names(&tree);
    let only_in_rust: Vec<&String> = rust.difference(&python).collect();
    let only_in_python: Vec<&String> = python.difference(&rust).collect();
    assert!(
        only_in_rust.is_empty() && only_in_python.is_empty(),
        "Rust and Python subcommand sets diverge. only_in_rust={only_in_rust:?}; only_in_python={only_in_python:?}"
    );
}

#[test]
fn source_matrix_is_required_in_python_argparse_for_lint_verify_search_show() {
    let Some(tree) = run_shim_or_skip() else {
        return;
    };
    let subcommands = tree["subcommands"].as_array().expect("subcommands");
    for verb in SOURCE_MATRIX_REQUIRED {
        let sub = subcommands
            .iter()
            .find(|s| s["name"].as_str() == Some(*verb))
            .unwrap_or_else(|| panic!("subcommand {verb} present in Python tree"));
        let options = sub["options"].as_array().expect("options array");
        // argparse stores `--source-matrix` with `dest=source_matrix`
        // (dashes become underscores). The mutex-group + required
        // semantics surface as `required: true` on the action.
        let source_matrix_action = options
            .iter()
            .find(|o| o["dest"].as_str() == Some("source_matrix"))
            .unwrap_or_else(|| panic!("--source-matrix option present on {verb}"));
        assert_eq!(
            source_matrix_action["required"].as_bool(),
            Some(true),
            "Python {verb}: --source-matrix must be required; action = {source_matrix_action:?}"
        );
    }
}

#[test]
fn source_matrix_is_required_in_rust_clap_for_lint_verify_search_show() {
    let root = cacg_cli::command();
    for verb in SOURCE_MATRIX_REQUIRED {
        let sub = root
            .get_subcommands()
            .find(|c| c.get_name() == *verb)
            .unwrap_or_else(|| panic!("Rust subcommand {verb} present"));
        let arg = sub
            .get_arguments()
            .find(|a| a.get_id().as_str() == "source_matrix")
            .unwrap_or_else(|| panic!("Rust {verb}: --source-matrix arg present"));
        assert!(
            arg.is_required_set(),
            "Rust {verb}: --source-matrix must be required; arg = {arg:?}"
        );
    }
}

/// Normalized per-positional surface. `dest` is the canonical name
/// (argparse `dest`, equivalently clap's arg id). `required` mirrors
/// argparse's `required` bool, which is `False` for `nargs="?"` /
/// `nargs="*"` positionals; clap's equivalent is
/// `arg.is_required_set()`, which returns `false` for `Option<...>`
/// fields and for positionals declared with `#[arg(default_value)]`.
/// `default` is the canonical default-value string (see
/// `normalize_python_default` / `normalize_rust_default`).
///
/// These helper types + functions support the
/// `rust_and_python_per_subcommand_surfaces_are_equal` test, which
/// is gated behind the `b2-llm-judge` Cargo feature (the surface
/// comparison only succeeds when Rust's clap exposes
/// `--semantic-judge`, matching Python's always-on surface).
/// `#[allow(dead_code)]` suppresses the warning under the default
/// feature set where the helpers are unused but still compiled.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct PositionalSurface {
    dest: String,
    required: bool,
    default: Option<String>,
}

/// Normalized per-option surface used for cross-parser comparison.
/// Merges aliased argparse actions (multiple `add_argument` calls
/// with the same `dest`) into a single entry whose `long_names` is
/// the union of their `option_strings`, matching clap's natural
/// representation of an `Arg` with `alias`. `default` carries the
/// canonical default-value string; for store-true/store-false flags
/// argparse implicitly carries `False` / `True` so the canonical
/// representation is `Some("false")` / `Some("true")` rather than
/// `None`.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct OptionSurface {
    long_names: std::collections::BTreeSet<String>,
    required: bool,
    default: Option<String>,
}

/// Normalized per-subcommand surface. `positionals` preserves
/// declaration order so positional-ordering parity can be asserted
/// alongside per-positional required-ness and default values.
/// `options` maps each option `dest` to its `OptionSurface`.
/// `mutex_groups` is the set of mutex-group member-`dest` sets
/// (order-insensitive across groups).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct SubcommandSurface {
    positionals: Vec<PositionalSurface>,
    options: std::collections::BTreeMap<String, OptionSurface>,
    mutex_groups: std::collections::BTreeSet<std::collections::BTreeSet<String>>,
}

#[allow(dead_code)]
/// Normalize a default value emitted by `legacy_python_oracle/scripts/export_argparse_tree.py`
/// into the canonical `Option<String>` shape. argparse defaults reach
/// the shim as one of: `None` (no default declared), `bool` (store-
/// true/store-false), `int`/`float` (numeric defaults), or `str`
/// (string defaults). The shim serializes each as the matching JSON
/// scalar; we map each variant to the same string shape clap's
/// `get_default_values()` produces so cross-parser equality is byte-
/// stable.
fn normalize_python_default(v: &Value) -> Option<String> {
    match v {
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        Value::String(s) => Some(s.clone()),
        // Null / arrays / objects are treated as not-set; argparse
        // never emits arrays/objects via the shim.
        Value::Null | Value::Array(_) | Value::Object(_) => None,
    }
}

/// Normalize a clap `Arg`'s default into the canonical
/// `Option<String>` shape. Prefers the first explicit
/// `default_value(...)` entry; otherwise infers `Some("false")` for
/// `ArgAction::SetTrue` and `Some("true")` for `ArgAction::SetFalse`
/// to mirror argparse's implicit `default=False` / `default=True` on
/// store-true / store-false actions.
#[allow(dead_code, clippy::wildcard_enum_match_arm)]
fn normalize_rust_default(arg: &clap::Arg) -> Option<String> {
    if let Some(d) = arg.get_default_values().first() {
        return Some(d.to_string_lossy().to_string());
    }
    match arg.get_action() {
        clap::ArgAction::SetTrue => Some("false".to_string()),
        clap::ArgAction::SetFalse => Some("true".to_string()),
        _ => None,
    }
}

#[allow(dead_code)]
fn python_subcommand_surface(tree: &Value, name: &str) -> SubcommandSurface {
    let subs = tree["subcommands"].as_array().expect("subcommands");
    let sub = subs
        .iter()
        .find(|s| s["name"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("python: subcommand {name} not found"));
    let positionals_arr = sub["positionals"].as_array().expect("positionals");
    let positionals: Vec<PositionalSurface> = positionals_arr
        .iter()
        .map(|p| PositionalSurface {
            dest: p["dest"].as_str().unwrap().to_string(),
            required: p["required"].as_bool().unwrap_or(false),
            default: normalize_python_default(&p["default"]),
        })
        .collect();
    let options_arr = sub["options"].as_array().expect("options");
    let mut options: std::collections::BTreeMap<String, OptionSurface> =
        std::collections::BTreeMap::new();
    for opt in options_arr {
        let dest = opt["dest"].as_str().unwrap().to_string();
        let required = opt["required"].as_bool().unwrap_or(false);
        let option_strings: Vec<String> = opt["option_strings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_str().unwrap().to_string())
            .collect();
        let this_default = normalize_python_default(&opt["default"]);
        let entry = options.entry(dest).or_insert(OptionSurface {
            long_names: std::collections::BTreeSet::new(),
            required: false,
            default: None,
        });
        for s in option_strings {
            entry.long_names.insert(s);
        }
        entry.required = entry.required || required;
        // First non-None default wins. argparse permits multiple
        // `add_argument(..., default=X)` calls sharing one `dest`
        // (e.g., verify's `--unsafe-skip-lint` + hidden `--skip-lint`
        // alias both set `default=False`); first-non-None is
        // sufficient because all sibling calls in the cacg.cli surface
        // declare the same default.
        if entry.default.is_none() {
            entry.default = this_default;
        }
    }
    let mutex_groups_arr = sub["mutex_groups"].as_array().expect("mutex_groups");
    let mutex_groups: std::collections::BTreeSet<std::collections::BTreeSet<String>> =
        mutex_groups_arr
            .iter()
            .map(|g| {
                g["member_dests"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s.as_str().unwrap().to_string())
                    .collect()
            })
            .collect();
    SubcommandSurface {
        positionals,
        options,
        mutex_groups,
    }
}

#[allow(dead_code)]
fn rust_subcommand_surface(name: &str) -> SubcommandSurface {
    let root = cacg_cli::command();
    let sub = root
        .find_subcommand(name)
        .unwrap_or_else(|| panic!("rust: subcommand {name} not found"));
    // Positionals appear in declaration order via `get_positionals()`
    // (clap's filtered iterator over args with no long/short flag).
    let positionals: Vec<PositionalSurface> = sub
        .get_positionals()
        .map(|a| PositionalSurface {
            dest: a.get_id().as_str().to_string(),
            required: a.is_required_set(),
            default: normalize_rust_default(a),
        })
        .collect();
    let mut options: std::collections::BTreeMap<String, OptionSurface> =
        std::collections::BTreeMap::new();
    for arg in sub.get_opts() {
        let dest = arg.get_id().as_str().to_string();
        let mut long_names: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        if let Some(long) = arg.get_long() {
            long_names.insert(format!("--{long}"));
        }
        if let Some(aliases) = arg.get_all_aliases() {
            for a in aliases {
                long_names.insert(format!("--{a}"));
            }
        }
        options.insert(
            dest,
            OptionSurface {
                long_names,
                required: arg.is_required_set(),
                default: normalize_rust_default(arg),
            },
        );
    }
    // clap's `#[derive(Args)]` macro auto-creates an implicit ArgGroup
    // named after the struct (e.g., `HistoryArgs`, `LintArgs`) that
    // contains every field. Filter those out by suffix so the
    // comparison only sees user-declared mutex groups.
    let mut mutex_groups: std::collections::BTreeSet<std::collections::BTreeSet<String>> =
        std::collections::BTreeSet::new();
    for group in sub.get_groups() {
        let id = group.get_id().as_str();
        if id.ends_with("Args") {
            continue;
        }
        let members: std::collections::BTreeSet<String> =
            group.get_args().map(|id| id.as_str().to_string()).collect();
        if !members.is_empty() {
            mutex_groups.insert(members);
        }
    }
    SubcommandSurface {
        positionals,
        options,
        mutex_groups,
    }
}

#[cfg(feature = "b2-llm-judge")]
#[test]
fn rust_and_python_per_subcommand_surfaces_are_equal() {
    // Comprehensive cross-parser comparison: for each of the 14
    // subcommands, build a normalized SubcommandSurface from both
    // Python argparse (via the shim) and Rust clap (via
    // CommandFactory introspection), then assert byte-for-byte
    // equality on:
    //   * the ordered positional list, including each positional's
    //     `required` (argparse `nargs="?"` => not required) and
    //     `default` (e.g. `index.cards_dir`/`migrate-summaries.cards_dir`
    //     both default to "cards"),
    //   * option dest set + per-dest long-flag set + required-ness +
    //     default value (string-canonicalized; store-true flags
    //     canonicalize to `Some("false")` on both sides),
    //   * mutex-group membership.
    //
    // Gated behind `b2-llm-judge` because Python's verify surface
    // always exposes `--semantic-judge`, whereas Rust's default-
    // feature build hides the flag (the default build contract
    // is that `--semantic-judge` is invisible to clap and is
    // rejected at argv parse time as an unknown argument). The
    // full surface comparison only matches Python when Rust
    // compiles with the feature on.
    let Some(tree) = run_shim_or_skip() else {
        return;
    };
    let names = [
        "history",
        "index",
        "ingest",
        "lint",
        "migrate-summaries",
        "new",
        "retract",
        "retract-chunk",
        "retract-source",
        "scaffold-matrix",
        "scaffold-role-map",
        "search",
        "show",
        "verify",
    ];
    for name in names {
        let py = python_subcommand_surface(&tree, name);
        let rs = rust_subcommand_surface(name);
        assert_eq!(
            py.positionals, rs.positionals,
            "{name}: positional surfaces (ordered dest + required + default) differ.\n  python={pp:#?}\n  rust  ={rp:#?}",
            pp = py.positionals,
            rp = rs.positionals,
        );
        // Compare option dest sets first; if those agree, compare
        // per-dest long_names + required + default.
        let py_dests: std::collections::BTreeSet<&String> = py.options.keys().collect();
        let rs_dests: std::collections::BTreeSet<&String> = rs.options.keys().collect();
        assert_eq!(
            py_dests,
            rs_dests,
            "{name}: option dest sets differ.\n  python_only={py_only:?}\n  rust_only={rs_only:?}",
            py_only = py_dests.difference(&rs_dests).collect::<Vec<_>>(),
            rs_only = rs_dests.difference(&py_dests).collect::<Vec<_>>(),
        );
        for dest in py_dests {
            let py_opt = py.options.get(dest).unwrap();
            let rs_opt = rs.options.get(dest).unwrap();
            assert_eq!(
                py_opt.long_names, rs_opt.long_names,
                "{name}.{dest}: long names differ.\n  python={py_opt:?}\n  rust  ={rs_opt:?}"
            );
            assert_eq!(
                py_opt.required,
                rs_opt.required,
                "{name}.{dest}: required-ness differs (python={p}, rust={r})",
                p = py_opt.required,
                r = rs_opt.required
            );
            assert_eq!(
                py_opt.default,
                rs_opt.default,
                "{name}.{dest}: default value differs (python={pd:?}, rust={rd:?})",
                pd = py_opt.default,
                rd = rs_opt.default,
            );
        }
        assert_eq!(
            py.mutex_groups,
            rs.mutex_groups,
            "{name}: mutex group membership sets differ.\n  python={pg:#?}\n  rust  ={rg:#?}",
            pg = py.mutex_groups,
            rg = rs.mutex_groups
        );
    }
}

#[test]
fn attribution_missing_source_matrix_exits_2_in_python_argparse() {
    // Mirror the Rust attribution check against the Python
    // implementation: spawn `python -m cacg.cli <verb> <synthetic-
    // positional>` (omitting `--source-matrix`) for each of the 4
    // trust-kernel subcommands, and assert Python's argparse rejects
    // with exit 2 + stderr references `--source-matrix`. The positional
    // value comes from the same Python tree the Rust attribution test
    // uses, so any future Python rename keeps both checks in lockstep.
    let Some(tree) = run_shim_or_skip() else {
        return;
    };
    let ws = workspace_root();
    let python = python_exe(&ws);
    let src_dir = ws.join("src");
    if !src_dir.is_dir() {
        eprintln!("skipping python attribution test: src/ not present at {src_dir:?}");
        return;
    }
    let subcommands = tree["subcommands"].as_array().expect("subcommands");

    for verb in SOURCE_MATRIX_REQUIRED {
        let sub = subcommands
            .iter()
            .find(|s| s["name"].as_str() == Some(*verb))
            .unwrap();
        let positionals = sub["positionals"].as_array().expect("positionals array");
        let positional_dest = positionals
            .first()
            .and_then(|p| p["dest"].as_str())
            .unwrap_or_else(|| panic!("python {verb} must declare at least one positional"));
        let synthetic_value = format!("synthetic_{positional_dest}_value");

        let output = Command::new(&python)
            .arg("-m")
            .arg("cacg.cli")
            .arg(verb)
            .arg(&synthetic_value)
            .env("PYTHONPATH", &src_dir)
            .output()
            .unwrap_or_else(|e| panic!("spawn python -m cacg.cli {verb} {synthetic_value}: {e}"));
        let code = output.status.code();
        assert_eq!(
            code,
            Some(2),
            "python -m cacg.cli {verb} {synthetic_value} (no --source-matrix) must exit 2; status={:?}, stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("source-matrix") || stderr.contains("source_matrix"),
            "python {verb} stderr must reference --source-matrix; got: {stderr}"
        );
    }
}

#[test]
fn attribution_missing_source_matrix_exits_2_with_python_positional_supplied() {
    // For each of the 4 trust-kernel subcommands, supply the
    // Python-canonical required positional (lint card / verify card /
    // search query / show card_id) but OMIT --source-matrix. Both
    // parsers must reject with exit code 2 (argparse-equivalent usage
    // error), and the rejection must be attributable to the missing
    // flag, not to a different parse error. The positional values come
    // from the Python tree so the test stays in lockstep with any
    // future positional rename on the Python side.
    let Some(tree) = run_shim_or_skip() else {
        return;
    };
    let subcommands = tree["subcommands"].as_array().expect("subcommands");
    let kb_bin = env!("CARGO_BIN_EXE_kb");

    for verb in SOURCE_MATRIX_REQUIRED {
        let sub = subcommands
            .iter()
            .find(|s| s["name"].as_str() == Some(*verb))
            .unwrap();
        // Collect the FIRST required-or-nargs-? positional. Each of the
        // four trust-kernel subcommands has exactly one positional in
        // its mutex group (card / query / card_id), so a single string
        // value is sufficient to satisfy that surface for parse-arity.
        let positionals = sub["positionals"].as_array().expect("positionals array");
        let positional_dest = positionals
            .first()
            .and_then(|p| p["dest"].as_str())
            .unwrap_or_else(|| panic!("subcommand {verb} must declare at least one positional"));
        // Synthesize a string value that "looks like" the positional
        // accepts (the verbs' positionals accept arbitrary strings).
        let synthetic_value = format!("synthetic_{positional_dest}_value");

        let output = Command::new(kb_bin)
            .arg(verb)
            .arg(&synthetic_value)
            .output()
            .unwrap_or_else(|e| panic!("spawn kb {verb} {synthetic_value}: {e}"));
        let code = output.status.code();
        assert_eq!(
            code,
            Some(2),
            "kb {verb} {synthetic_value} (no --source-matrix) must exit 2 (clap usage error); status={:?}, stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("source-matrix") || stderr.contains("source_matrix"),
            "kb {verb} stderr must reference --source-matrix; got: {stderr}"
        );
    }
}
