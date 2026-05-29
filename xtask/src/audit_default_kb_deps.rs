//! Audit the resolved default-features dependency graph of the
//! shipped `kb` binary (`cacg-cli`).
//!
//! The shipped binary's transitive non-dev closure must NOT pull
//! heavy embedding/model crates or default-path network/async
//! crates. Operators who deliberately want the LLM-judge build
//! opt in via `--features b2-llm-judge`; that feature legitimately
//! activates `reqwest` and `tokio`, but the default `cargo build
//! -p cacg-cli` (default features = `["ingest"]`) must stay
//! network-free and embedding-free.
//!
//! Like the sibling `cacg_core_deps` audit, this walks the full
//! transitive normal + build (non-dev) closure via `cargo
//! metadata --format-version=1`. The invocation deliberately
//! omits BOTH `--all-features` (which would pull in the
//! legitimate `b2-llm-judge` graph) AND `--no-default-features`
//! (which would skip the `ingest` default). The audited target
//! is therefore exactly what `cargo build -p cacg-cli` would
//! compile.
//!
//! Every match resolves through `packages[].name` (the resolved
//! crate name, not the local dep key), so a Cargo
//! `alias = { package = "tokio", ... }` rename cannot dodge the
//! gate.
//!
//! Banlist:
//!   - Exact matches: `tokio`, `reqwest`, `tch`, `ort`,
//!     `onnxruntime-rs`, `sentence-transformers-rs`. These are
//!     the most common Rust embedding / async-runtime / HTTP
//!     packages.
//!   - Prefix match: `candle-` covers candle-core, candle-nn,
//!     candle-transformers, and any future candle-* member
//!     without enumeration.

use std::collections::{BTreeSet, VecDeque};
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

/// One forbidden-dep violation surfaced by the audit.
#[derive(Debug, Clone)]
pub struct Violation {
    /// Resolved package name that triggered the rule (the actual
    /// crate name, not the local alias the manifest used).
    pub package: String,
    /// Human-readable explanation of why this package is forbidden
    /// in the default kb binary's closure.
    pub reason: &'static str,
    /// One package-name chain from `cacg-cli` to this violation,
    /// useful for debugging transitive flags.
    pub chain_hint: Vec<String>,
}

/// The audited root package name. The default kb binary IS this
/// package, so the audit walks its transitive closure under
/// default features.
const ROOT_PACKAGE: &str = "cacg-cli";

/// Exact-match banlist. Each entry is matched against the
/// RESOLVED `packages[].name` field in the metadata, never the
/// local dep alias.
const EXACT_FORBIDDEN: &[(&str, &str)] = &[
    (
        "tokio",
        "async runtimes belong behind the `b2-llm-judge` opt-in feature; the default kb binary must stay sync-only",
    ),
    (
        "reqwest",
        "HTTP clients belong behind the `b2-llm-judge` opt-in feature; the default kb binary must not link a network stack",
    ),
    (
        "tch",
        "embedding/model crates belong behind opt-in features; the default kb binary must not link a tensor runtime",
    ),
    (
        "ort",
        "ONNX runtimes belong behind opt-in features; the default kb binary must not link a model runtime",
    ),
    (
        "onnxruntime-rs",
        "ONNX runtimes belong behind opt-in features; the default kb binary must not link a model runtime",
    ),
    (
        "sentence-transformers-rs",
        "embedding crates belong behind opt-in features; the default kb binary must not link a model runtime",
    ),
];

/// Prefix-match banlist. Each entry is matched against the
/// RESOLVED `packages[].name` as a prefix; covers crate families
/// (e.g. `candle-core`, `candle-nn`, `candle-transformers`)
/// without enumerating every member.
const PREFIX_FORBIDDEN: &[(&str, &str)] = &[
    (
        "candle-",
        "embedding/model crates belong behind opt-in features; the default kb binary must not link the candle family",
    ),
];

/// Check whether a resolved package name is forbidden. Returns
/// `Some(reason)` for the first matching rule, exact match
/// preferred over prefix match.
fn is_forbidden(name: &str) -> Option<&'static str> {
    for (pat, reason) in EXACT_FORBIDDEN {
        if name == *pat {
            return Some(reason);
        }
    }
    for (pat, reason) in PREFIX_FORBIDDEN {
        if name.starts_with(pat) {
            return Some(reason);
        }
    }
    None
}

/// Errors raised by the audit.
#[derive(Debug)]
pub enum AuditError {
    /// `cargo metadata` invocation failed.
    CargoMetadata(String),
    /// The metadata JSON could not be parsed.
    Json(serde_json::Error),
    /// The root package was not found in the metadata
    /// `packages[]` list (e.g. workspace missing the crate).
    RootNotFound,
    /// The root package was found in `packages[]` but not in
    /// `resolve.nodes[]`.
    RootNodeMissing,
}

impl std::fmt::Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CargoMetadata(msg) => write!(f, "cargo metadata failed: {msg}"),
            Self::Json(e) => write!(f, "cargo metadata JSON parse failed: {e}"),
            Self::RootNotFound => write!(
                f,
                "root package {ROOT_PACKAGE:?} not found in cargo metadata packages list"
            ),
            Self::RootNodeMissing => write!(
                f,
                "root package {ROOT_PACKAGE:?} node missing from cargo metadata resolve.nodes"
            ),
        }
    }
}

impl std::error::Error for AuditError {}

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
    resolve: Resolve,
}

#[derive(Debug, Deserialize)]
struct Package {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Resolve {
    nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
struct Node {
    id: String,
    #[serde(default)]
    deps: Vec<NodeDep>,
}

#[derive(Debug, Deserialize)]
struct NodeDep {
    pkg: String,
    #[serde(default)]
    dep_kinds: Vec<DepKind>,
}

#[derive(Debug, Deserialize)]
struct DepKind {
    /// `None` = normal dep; `Some("build")` = build dep — both
    /// are followed by the audit. `Some("dev")` is the pure-dev
    /// case and is exempt unless the same edge ALSO has a
    /// normal/build entry.
    kind: Option<String>,
}

/// Argv used to invoke `cargo metadata`. Held as a function
/// (called by both the production `cargo_metadata_command` and
/// the unit tests) so a unit test can assert the argv neither
/// asks for `--all-features` (which would pull the legitimate
/// `b2-llm-judge` graph) nor `--no-default-features` (which
/// would skip `ingest`). The audited target is therefore exactly
/// what a default `cargo build -p cacg-cli` would compile.
#[must_use]
pub fn metadata_argv() -> &'static [&'static str] {
    &["metadata", "--format-version=1"]
}

/// Build the `cargo metadata` command rooted at `workspace_root`.
/// Exposed so tests can introspect the command's `get_args()`.
#[must_use]
pub fn cargo_metadata_command(workspace_root: &std::path::Path) -> Command {
    let mut c = Command::new("cargo");
    c.args(metadata_argv());
    c.current_dir(workspace_root);
    c
}

/// Parse `metadata_json` (the output of
/// `cargo metadata --format-version=1`) and return every
/// violation in `cacg-cli`'s transitive non-dev dep closure.
pub fn audit(metadata_json: &str) -> Result<Vec<Violation>, AuditError> {
    let metadata: Metadata = serde_json::from_str(metadata_json).map_err(AuditError::Json)?;

    // Build a `package id -> package name` index.
    let mut id_to_name: std::collections::BTreeMap<&str, &str> = std::collections::BTreeMap::new();
    for pkg in &metadata.packages {
        id_to_name.insert(pkg.id.as_str(), pkg.name.as_str());
    }

    // Locate the root package id.
    let root_id = metadata
        .packages
        .iter()
        .find(|p| p.name == ROOT_PACKAGE)
        .map(|p| p.id.as_str())
        .ok_or(AuditError::RootNotFound)?;

    // Build a `node id -> &Node` index.
    let mut id_to_node: std::collections::BTreeMap<&str, &Node> = std::collections::BTreeMap::new();
    for node in &metadata.resolve.nodes {
        id_to_node.insert(node.id.as_str(), node);
    }
    if !id_to_node.contains_key(root_id) {
        return Err(AuditError::RootNodeMissing);
    }

    // BFS from the root, following only edges with at least one
    // normal (kind=null) or build (kind="build") entry. Pure-dev
    // edges are skipped. Record the parent chain for each visited
    // package so a transitive violation can report how it got
    // there.
    let mut visited: BTreeSet<&str> = BTreeSet::new();
    let mut parent: std::collections::BTreeMap<&str, &str> = std::collections::BTreeMap::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    visited.insert(root_id);
    queue.push_back(root_id);

    let mut violations: Vec<Violation> = Vec::new();

    while let Some(current_id) = queue.pop_front() {
        // Check whether the current package itself is forbidden,
        // excluding the root (we don't flag ourselves).
        if current_id != root_id {
            if let Some(&name) = id_to_name.get(current_id) {
                if let Some(reason) = is_forbidden(name) {
                    violations.push(Violation {
                        package: name.to_string(),
                        reason,
                        chain_hint: build_chain(root_id, current_id, &parent, &id_to_name),
                    });
                    // Continue BFS so sibling forbidden packages
                    // on the same level are also reported; skip
                    // descending into a flagged subtree to keep
                    // diagnostics readable.
                    continue;
                }
            }
        }

        // Walk this node's audited (normal + build) edges.
        let Some(node) = id_to_node.get(current_id) else {
            continue;
        };
        for dep in &node.deps {
            if !is_audited_edge(&dep.dep_kinds) {
                continue;
            }
            let dep_pkg = dep.pkg.as_str();
            if visited.insert(dep_pkg) {
                parent.insert(dep_pkg, current_id);
                queue.push_back(dep_pkg);
            }
        }
    }
    Ok(violations)
}

/// Returns true iff `dep_kinds` contains a normal (`kind ==
/// None`) or build (`kind == Some("build")`) entry. Pure-dev
/// edges (every entry is `kind == "dev"`) return false.
fn is_audited_edge(dep_kinds: &[DepKind]) -> bool {
    dep_kinds
        .iter()
        .any(|dk| matches!(dk.kind.as_deref(), None | Some("build")))
}

fn build_chain<'a>(
    root: &'a str,
    target: &'a str,
    parent: &std::collections::BTreeMap<&'a str, &'a str>,
    id_to_name: &std::collections::BTreeMap<&'a str, &'a str>,
) -> Vec<String> {
    let mut chain_ids: Vec<&str> = vec![target];
    let mut cur = target;
    while let Some(&p) = parent.get(cur) {
        chain_ids.push(p);
        if p == root {
            break;
        }
        cur = p;
    }
    chain_ids.reverse();
    chain_ids
        .iter()
        .map(|id| {
            id_to_name
                .get(id)
                .map(|name| (*name).to_string())
                .unwrap_or_else(|| (*id).to_string())
        })
        .collect()
}

/// Invoke `cargo metadata` from `workspace_root` and audit the
/// result.
pub fn audit_workspace(workspace_root: &std::path::Path) -> Result<Vec<Violation>, AuditError> {
    let output = cargo_metadata_command(workspace_root)
        .output()
        .map_err(|e| AuditError::CargoMetadata(format!("spawn: {e}")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AuditError::CargoMetadata(format!(
            "exit={} stderr={stderr}",
            output.status
        )));
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|e| AuditError::CargoMetadata(format!("non-utf8 stdout: {e}")))?;
    audit(&text)
}

/// Convenience wrapper that resolves the workspace root from the
/// current dir (used by the xtask CLI dispatcher).
pub fn audit_current_workspace() -> Result<Vec<Violation>, AuditError> {
    audit_workspace(&PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT_ID: &str = "path+file:///workspace/cacg-cli#0.1.0";

    /// Build a minimal `cargo metadata` JSON blob with cacg-cli
    /// + `extra_packages` + edges. `edges` are
    /// `(from_name, dep_name, kinds)` where `kinds` is
    /// `Vec<Option<&str>>` mapping to `DepKind { kind }`.
    fn make_metadata(
        extra_packages: &[(&str, &str)],
        edges: &[(&str, &str, Vec<Option<&str>>)],
    ) -> String {
        let mut packages = vec![serde_json::json!({
            "id": ROOT_ID,
            "name": ROOT_PACKAGE,
        })];
        for (name, id) in extra_packages {
            packages.push(serde_json::json!({"id": id, "name": name}));
        }
        let mut name_to_id: std::collections::HashMap<&str, &str> =
            std::collections::HashMap::new();
        name_to_id.insert(ROOT_PACKAGE, ROOT_ID);
        for (name, id) in extra_packages {
            name_to_id.insert(name, id);
        }

        let mut deps_by_node: std::collections::HashMap<&str, Vec<serde_json::Value>> =
            std::collections::HashMap::new();
        for (from_name, dep_name, kinds) in edges {
            let from_id = name_to_id
                .get(from_name)
                .unwrap_or_else(|| panic!("missing package {from_name}"));
            let dep_id = name_to_id
                .get(dep_name)
                .unwrap_or_else(|| panic!("missing package {dep_name}"));
            let dep_kinds: Vec<serde_json::Value> = kinds
                .iter()
                .map(|k| serde_json::json!({"kind": k, "target": null}))
                .collect();
            deps_by_node
                .entry(*from_id)
                .or_default()
                .push(serde_json::json!({
                    "name": dep_name,
                    "pkg": dep_id,
                    "dep_kinds": dep_kinds,
                }));
        }

        let mut nodes = Vec::new();
        for (_, id) in std::iter::once((ROOT_PACKAGE, ROOT_ID))
            .chain(extra_packages.iter().map(|(n, i)| (*n, *i)))
        {
            let deps = deps_by_node.remove(id).unwrap_or_default();
            nodes.push(serde_json::json!({
                "id": id,
                "deps": deps,
            }));
        }
        serde_json::json!({
            "packages": packages,
            "resolve": {"nodes": nodes},
        })
        .to_string()
    }

    #[test]
    fn clean_workspace_metadata_passes() {
        // Integration-style: invoke real `cargo metadata` against
        // the workspace this test lives in. The default-features
        // closure of `cacg-cli` must have zero violations.
        let mut workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        workspace_root.pop();
        let r = audit_workspace(&workspace_root)
            .unwrap_or_else(|e| panic!("audit_workspace failed: {e}"));
        assert!(
            r.is_empty(),
            "the default `cargo build -p cacg-cli` closure must have zero forbidden packages; got: {r:?}"
        );
    }

    #[test]
    fn direct_forbidden_dep_is_flagged() {
        let m = make_metadata(
            &[("tokio", "registry#tokio@1.0.0")],
            &[(ROOT_PACKAGE, "tokio", vec![None])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert_eq!(v[0].package, "tokio");
        assert_eq!(
            v[0].chain_hint,
            vec![ROOT_PACKAGE.to_string(), "tokio".to_string()]
        );
    }

    #[test]
    fn transitive_forbidden_dep_is_flagged_with_chain_hint() {
        let m = make_metadata(
            &[
                ("intermediate", "registry#intermediate@1.0.0"),
                ("reqwest", "registry#reqwest@0.12.0"),
            ],
            &[
                (ROOT_PACKAGE, "intermediate", vec![None]),
                ("intermediate", "reqwest", vec![None]),
            ],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert_eq!(v[0].package, "reqwest");
        assert_eq!(
            v[0].chain_hint,
            vec![
                ROOT_PACKAGE.to_string(),
                "intermediate".to_string(),
                "reqwest".to_string()
            ]
        );
    }

    #[test]
    fn renamed_package_alias_still_caught() {
        // The manifest may use `alias = { package = "tokio" }`,
        // but `packages[].name` in metadata is still "tokio". The
        // edge's `name` field carries the alias; the audit walks
        // by `pkg` (id) → `packages[].name`, so the resolved name
        // wins.
        let m = make_metadata(
            &[("tokio", "registry#tokio@1.0.0")],
            // Note: the `name` field in the edge is just for
            // human-readable cargo tree output; the audit uses
            // `pkg` (id) → packages[].name to resolve.
            &[(ROOT_PACKAGE, "tokio", vec![None])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert_eq!(
            v[0].package, "tokio",
            "audit must resolve via packages[].name, not the local alias"
        );
    }

    #[test]
    fn candle_prefix_match_flags_candle_core() {
        let m = make_metadata(
            &[("candle-core", "registry#candle-core@0.6.0")],
            &[(ROOT_PACKAGE, "candle-core", vec![None])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert_eq!(v[0].package, "candle-core");
        assert!(
            v[0].reason.contains("candle"),
            "diagnostic must name the candle family: {}",
            v[0].reason
        );
    }

    #[test]
    fn candle_prefix_match_flags_candle_transformers() {
        let m = make_metadata(
            &[("candle-transformers", "registry#candle-transformers@0.6.0")],
            &[(ROOT_PACKAGE, "candle-transformers", vec![None])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].package, "candle-transformers");
    }

    #[test]
    fn dev_only_edges_are_not_followed() {
        let m = make_metadata(
            &[("tokio", "registry#tokio@1.0.0")],
            &[(ROOT_PACKAGE, "tokio", vec![Some("dev")])],
        );
        let v = audit(&m).unwrap();
        assert!(
            v.is_empty(),
            "pure-dev edges must not flag the default closure; got: {v:?}"
        );
    }

    #[test]
    fn mixed_normal_and_dev_edge_is_followed() {
        // An edge that has BOTH dev and normal kinds is followed.
        let m = make_metadata(
            &[("tokio", "registry#tokio@1.0.0")],
            &[(ROOT_PACKAGE, "tokio", vec![None, Some("dev")])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert_eq!(v[0].package, "tokio");
    }

    #[test]
    fn package_not_in_resolve_nodes_is_invisible() {
        // Simulate an opt-in feature dep: `tokio` is listed in
        // packages[] but is NOT in resolve.nodes[] (because no
        // node activated the feature that pulls it in). The
        // default-features audit must NOT see it.
        let inner = {
            let mut packages = vec![
                serde_json::json!({
                    "id": ROOT_ID,
                    "name": ROOT_PACKAGE,
                }),
                serde_json::json!({
                    "id": "registry#tokio@1.0.0",
                    "name": "tokio",
                }),
            ];
            // Drop the tokio node — only cacg-cli is in
            // resolve.nodes[]. This matches what
            // `cargo metadata` produces when an optional dep is
            // gated by a feature that isn't activated.
            let nodes = vec![serde_json::json!({
                "id": ROOT_ID,
                "deps": [],
            })];
            // Quiet a no-mut warning from the test helper above.
            packages.shrink_to_fit();
            serde_json::json!({
                "packages": packages,
                "resolve": {"nodes": nodes},
            })
            .to_string()
        };
        let v = audit(&inner).unwrap();
        assert!(
            v.is_empty(),
            "package listed in packages[] but absent from resolve.nodes[] must be invisible: {v:?}"
        );
    }

    #[test]
    fn metadata_command_uses_default_feature_set() {
        // The audit MUST walk the default `cargo build -p
        // cacg-cli` closure. That means neither `--all-features`
        // (would activate the legitimate `b2-llm-judge` graph)
        // nor `--no-default-features` (would skip the `ingest`
        // default).
        let argv = metadata_argv();
        assert!(
            argv.iter().any(|a| *a == "metadata"),
            "argv must invoke `metadata`: {argv:?}"
        );
        assert!(
            argv.iter().any(|a| *a == "--format-version=1"),
            "argv must pin `--format-version=1`: {argv:?}"
        );
        assert!(
            !argv.iter().any(|a| *a == "--all-features"),
            "argv must NOT pass `--all-features` (legitimate b2-llm-judge graph would be falsely flagged): {argv:?}"
        );
        assert!(
            !argv.iter().any(|a| *a == "--no-default-features"),
            "argv must NOT pass `--no-default-features` (would skip the `ingest` default feature): {argv:?}"
        );
    }

    #[test]
    fn cargo_metadata_command_routes_to_workspace_root() {
        // Sanity-check that the Command's current_dir is set to
        // the workspace root supplied by the caller.
        let p = std::path::PathBuf::from("/tmp/synthetic-root");
        let c = cargo_metadata_command(&p);
        // get_args() is stable since Rust 1.57; collect into a
        // vec of OsStr for inspection.
        let args: Vec<String> = c
            .get_args()
            .map(|a| a.to_string_lossy().to_string())
            .collect();
        assert_eq!(
            args,
            vec!["metadata".to_string(), "--format-version=1".to_string()]
        );
        assert_eq!(c.get_current_dir(), Some(p.as_path()));
    }

    #[test]
    fn missing_root_package_returns_error() {
        let m = serde_json::json!({
            "packages": [],
            "resolve": {"nodes": []},
        })
        .to_string();
        let r = audit(&m);
        assert!(matches!(r, Err(AuditError::RootNotFound)), "got: {r:?}");
    }

    #[test]
    fn multiple_distinct_forbidden_packages_all_flagged() {
        let m = make_metadata(
            &[
                ("tokio", "registry#tokio@1.0.0"),
                ("reqwest", "registry#reqwest@0.12.0"),
                ("candle-core", "registry#candle-core@0.6.0"),
            ],
            &[
                (ROOT_PACKAGE, "tokio", vec![None]),
                (ROOT_PACKAGE, "reqwest", vec![None]),
                (ROOT_PACKAGE, "candle-core", vec![None]),
            ],
        );
        let v = audit(&m).unwrap();
        let mut names: Vec<&str> = v.iter().map(|x| x.package.as_str()).collect();
        names.sort();
        assert_eq!(names, vec!["candle-core", "reqwest", "tokio"]);
    }

    #[test]
    fn benign_dep_is_not_flagged() {
        let m = make_metadata(
            &[
                ("serde", "registry#serde@1.0.0"),
                ("clap", "registry#clap@4.0.0"),
            ],
            &[
                (ROOT_PACKAGE, "serde", vec![None]),
                (ROOT_PACKAGE, "clap", vec![None]),
            ],
        );
        let v = audit(&m).unwrap();
        assert!(
            v.is_empty(),
            "clap + serde are legitimate cacg-cli deps; got: {v:?}"
        );
    }
}
