//! Audit `cacg-core`'s resolved dependency graph against the
//! trust-kernel allowlist.
//!
//! Walks the full transitive normal + build (non-dev) dependency
//! closure via `cargo metadata --format-version=1` so the audit
//! cannot be fooled by Cargo's renamed-package alias
//! (`pdf = { package = "pdfium-render", ... }`), by
//! `[dependencies.<name>]` dependency-table syntax, by target-specific
//! dependency tables (`[target.'cfg(unix)'.dependencies]`), by
//! `[build-dependencies]` declarations, or by a forbidden package
//! pulled in transitively through a benign-looking direct dependency.
//! Every match resolves through the `packages[].name` field rather
//! than the local dep key.
//!
//! Dev-dependency exemption is preserved deliberately: pure-dev edges
//! (every entry in `dep_kinds[]` has `kind == "dev"`) are NOT
//! followed. Normal (`kind == null`) and build (`kind == "build"`)
//! edges ARE followed, matching the surface `cargo tree -e features
//! -p cacg-core` displays. A heavy `[build-dependencies]` would still
//! pull a forbidden crate into cacg-core's build graph and weigh
//! against the "small, dependency-light trust kernel" invariant.

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
    /// Human-readable explanation of why this package is forbidden.
    pub reason: &'static str,
    /// One package ID along the dep chain from cacg-core to this
    /// violation, useful for debugging transitive flags.
    pub chain_hint: Vec<String>,
}

/// Forbidden packages. Matched against the RESOLVED package name in
/// the `packages[].name` field, not the local dep key.
const FORBIDDEN: &[(&str, &str)] = &[
    ("pdfium-render", "Pdfium is M4 ingest-only; must not enter the trust kernel"),
    ("rusqlite", "SQLite is M5 search-only; must not enter the trust kernel"),
    ("clap", "CLI parsing belongs in cacg-cli; the trust kernel must not depend on a CLI framework"),
    ("tokio", "async runtimes belong in optional downstream crates; the trust kernel is sync-only"),
    ("async-std", "async runtimes belong in optional downstream crates; the trust kernel is sync-only"),
    ("smol", "async runtimes belong in optional downstream crates; the trust kernel is sync-only"),
    ("hyper", "HTTP belongs in optional downstream crates; the trust kernel must not depend on a network stack"),
    ("reqwest", "HTTP belongs in optional downstream crates; the trust kernel must not depend on a network stack"),
    ("actix-web", "HTTP belongs in optional downstream crates; the trust kernel must not depend on a network stack"),
    ("axum", "HTTP belongs in optional downstream crates; the trust kernel must not depend on a network stack"),
    ("rocket", "HTTP belongs in optional downstream crates; the trust kernel must not depend on a network stack"),
    ("cacg-search", "downstream cacg-* crates must depend on cacg-core, not the reverse"),
    ("cacg-cli", "downstream cacg-* crates must depend on cacg-core, not the reverse"),
    ("cacg-ingest", "downstream cacg-* crates must depend on cacg-core, not the reverse"),
    ("cacg-semantic", "downstream cacg-* crates must depend on cacg-core, not the reverse"),
];

/// Errors raised by the audit.
#[derive(Debug)]
pub enum AuditError {
    /// `cargo metadata` invocation failed.
    CargoMetadata(String),
    /// The metadata JSON could not be parsed.
    Json(serde_json::Error),
    /// cacg-core was not found in the metadata `packages[]` list.
    CacgCoreNotFound,
    /// cacg-core was found in `packages[]` but not in `resolve.nodes[]`.
    CacgCoreNodeMissing,
}

impl std::fmt::Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CargoMetadata(msg) => write!(f, "cargo metadata failed: {msg}"),
            Self::Json(e) => write!(f, "cargo metadata JSON parse failed: {e}"),
            Self::CacgCoreNotFound => {
                write!(f, "cacg-core not found in cargo metadata packages list")
            }
            Self::CacgCoreNodeMissing => write!(
                f,
                "cacg-core node missing from cargo metadata resolve.nodes"
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
    /// `None` = normal dep; `Some("build")` = build dep — both are
    /// followed by the audit. `Some("dev")` is the pure-dev case and
    /// is exempt; an edge with at least one normal or build entry is
    /// still followed regardless of other dev entries on the same edge.
    kind: Option<String>,
}

/// Parse `metadata_json` (the output of
/// `cargo metadata --format-version=1`) and return every violation
/// found in cacg-core's transitive non-dev dep closure.
pub fn audit(metadata_json: &str) -> Result<Vec<Violation>, AuditError> {
    let metadata: Metadata = serde_json::from_str(metadata_json).map_err(AuditError::Json)?;

    // Build a `package id -> package name` index.
    let mut id_to_name: std::collections::BTreeMap<&str, &str> = std::collections::BTreeMap::new();
    for pkg in &metadata.packages {
        id_to_name.insert(pkg.id.as_str(), pkg.name.as_str());
    }

    // Find cacg-core's package id.
    let cacg_core_id = metadata
        .packages
        .iter()
        .find(|p| p.name == "cacg-core")
        .map(|p| p.id.as_str())
        .ok_or(AuditError::CacgCoreNotFound)?;

    // Build a `node id -> &Node` index.
    let mut id_to_node: std::collections::BTreeMap<&str, &Node> = std::collections::BTreeMap::new();
    for node in &metadata.resolve.nodes {
        id_to_node.insert(node.id.as_str(), node);
    }
    if !id_to_node.contains_key(cacg_core_id) {
        return Err(AuditError::CacgCoreNodeMissing);
    }

    // BFS from cacg-core's node, following only edges that pass
    // `is_audited_edge` — i.e., edges with at least one normal
    // (`kind: null`) or build (`kind: "build"`) `dep_kind` entry.
    // Pure-dev edges (every entry is `kind: "dev"`) are skipped.
    // Track visited package IDs to avoid cycles. Record the parent
    // chain so a transitive violation can report how it got there.
    let mut visited: BTreeSet<&str> = BTreeSet::new();
    let mut parent: std::collections::BTreeMap<&str, &str> = std::collections::BTreeMap::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    visited.insert(cacg_core_id);
    queue.push_back(cacg_core_id);

    let mut violations: Vec<Violation> = Vec::new();

    while let Some(current_id) = queue.pop_front() {
        // Check whether the current package itself is forbidden,
        // excluding cacg-core (the root) so we don't flag ourselves.
        if current_id != cacg_core_id {
            if let Some(name) = id_to_name.get(current_id) {
                if let Some(&(_, reason)) = FORBIDDEN.iter().find(|(n, _)| *n == *name) {
                    violations.push(Violation {
                        package: (*name).to_string(),
                        reason,
                        chain_hint: build_chain(cacg_core_id, current_id, &parent),
                    });
                    // Continue walking so we can also report sibling
                    // forbidden packages on the same level; don't
                    // descend into a flagged subtree (would multiply
                    // noise). Skip the dep recursion via `continue`.
                    continue;
                }
            }
        }

        // Walk this node's audited (normal + build) deps.
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

/// Should the BFS follow this edge? Returns true if `dep_kinds`
/// contains a normal (`kind == None`) or build (`kind == "build"`)
/// entry. Pure-dev edges (every entry is `kind == "dev"`) return
/// false. Matches the surface `cargo tree -e features -p cacg-core`
/// displays.
fn is_audited_edge(dep_kinds: &[DepKind]) -> bool {
    dep_kinds
        .iter()
        .any(|dk| matches!(dk.kind.as_deref(), None | Some("build")))
}

fn build_chain<'a>(
    root: &'a str,
    target: &'a str,
    parent: &std::collections::BTreeMap<&'a str, &'a str>,
) -> Vec<String> {
    let mut chain: Vec<&str> = vec![target];
    let mut cur = target;
    while let Some(&p) = parent.get(cur) {
        chain.push(p);
        if p == root {
            break;
        }
        cur = p;
    }
    chain.reverse();
    chain.iter().map(|s| (*s).to_string()).collect()
}

/// Invoke `cargo metadata --format-version=1` from `workspace_root`
/// and return the audit result.
pub fn audit_workspace(workspace_root: &std::path::Path) -> Result<Vec<Violation>, AuditError> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1"])
        .current_dir(workspace_root)
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

    const CACG_CORE_ID: &str = "path+file:///workspace/cacg-core#0.1.0";

    /// Build a minimal `cargo metadata` JSON blob with cacg-core +
    /// `extra_packages` + edges. Edges are pairs of
    /// `(from_package_name, dep_pkg_name, kinds)` where `kinds` is
    /// `Vec<Option<&str>>` mapping to `DepKind { kind }`.
    fn make_metadata(
        extra_packages: &[(&str, &str)],
        edges: &[(&str, &str, Vec<Option<&str>>)],
    ) -> String {
        let mut packages = vec![serde_json::json!({
            "id": CACG_CORE_ID,
            "name": "cacg-core",
        })];
        for (name, id) in extra_packages {
            packages.push(serde_json::json!({"id": id, "name": name}));
        }
        let mut name_to_id: std::collections::HashMap<&str, &str> =
            std::collections::HashMap::new();
        name_to_id.insert("cacg-core", CACG_CORE_ID);
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
        for (_, id) in std::iter::once(("cacg-core", CACG_CORE_ID))
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
    fn clean_metadata_passes() {
        let m = make_metadata(
            &[
                ("serde", "registry#serde@1.0.0"),
                ("sha2", "registry#sha2@0.10.0"),
            ],
            &[
                ("cacg-core", "serde", vec![None]),
                ("cacg-core", "sha2", vec![None]),
            ],
        );
        let v = audit(&m).unwrap();
        assert!(v.is_empty(), "got: {v:?}");
    }

    #[test]
    fn flags_direct_renamed_package() {
        // Local alias `pdf` resolves through `packages[].name = "pdfium-render"`.
        let m = make_metadata(
            &[("pdfium-render", "registry#pdfium-render@0.8.0")],
            &[("cacg-core", "pdfium-render", vec![None])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert_eq!(v[0].package, "pdfium-render");
    }

    #[test]
    fn flags_dep_table_syntax_via_resolved_name() {
        // Whether the on-disk Cargo.toml syntax was
        // `[dependencies.pdfium-render]` or `pdfium-render = { ... }`,
        // metadata represents both identically. Same trigger fires.
        let m = make_metadata(
            &[("pdfium-render", "registry#pdfium-render@0.8.0")],
            &[("cacg-core", "pdfium-render", vec![None])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn flags_target_specific_normal_dep() {
        // dep_kinds with kind=null (normal) is followed regardless of
        // the `target` cfg.
        let m = make_metadata(
            &[("tokio", "registry#tokio@1.0.0")],
            &[("cacg-core", "tokio", vec![None])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].package, "tokio");
    }

    #[test]
    fn ignores_dev_dependencies() {
        let m = make_metadata(
            &[("clap", "registry#clap@4.0.0")],
            &[("cacg-core", "clap", vec![Some("dev")])],
        );
        let v = audit(&m).unwrap();
        assert!(v.is_empty(), "dev-only edge must not flag; got: {v:?}");
    }

    #[test]
    fn flags_build_only_forbidden_dependency() {
        // A `[build-dependencies] clap = ...` declaration shows up in
        // `cargo tree -e features -p cacg-core` and pulls the forbidden
        // crate into cacg-core's build graph. Build edges are inside
        // the audit scope, so the violation must surface.
        let m = make_metadata(
            &[("clap", "registry#clap@4.0.0")],
            &[("cacg-core", "clap", vec![Some("build")])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(
            v.len(),
            1,
            "build-only edge to a forbidden crate MUST flag; got: {v:?}"
        );
        assert_eq!(v[0].package, "clap");
    }

    #[test]
    fn flags_transitive_forbidden_package_via_build_edge() {
        // cacg-core --build--> intermediate --normal--> tokio. The
        // build edge must be followed so the transitive `tokio` is
        // caught.
        let m = make_metadata(
            &[
                ("intermediate", "registry#intermediate@1.0.0"),
                ("tokio", "registry#tokio@1.0.0"),
            ],
            &[
                ("cacg-core", "intermediate", vec![Some("build")]),
                ("intermediate", "tokio", vec![None]),
            ],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert_eq!(v[0].package, "tokio");
        assert!(
            v[0].chain_hint.len() >= 2,
            "transitive build-edge violation must include parent chain: {:?}",
            v[0].chain_hint
        );
    }

    #[test]
    fn follows_normal_when_also_dev() {
        // An edge with BOTH dev and normal kinds is followed (the
        // normal entry alone qualifies the edge as audited, even
        // when other entries on the same edge are pure-dev).
        let m = make_metadata(
            &[("clap", "registry#clap@4.0.0")],
            &[("cacg-core", "clap", vec![None, Some("dev")])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].package, "clap");
    }

    #[test]
    fn flags_transitive_forbidden_package() {
        // cacg-core -> intermediate -> tokio (all normal kinds).
        let m = make_metadata(
            &[
                ("intermediate", "registry#intermediate@1.0.0"),
                ("tokio", "registry#tokio@1.0.0"),
            ],
            &[
                ("cacg-core", "intermediate", vec![None]),
                ("intermediate", "tokio", vec![None]),
            ],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert_eq!(v[0].package, "tokio");
        assert!(
            v[0].chain_hint.len() >= 2,
            "transitive violation must include parent chain: {:?}",
            v[0].chain_hint
        );
    }

    #[test]
    fn flags_downstream_cacg_crate() {
        let m = make_metadata(
            &[("cacg-search", "path+file:///workspace/cacg-search#0.1.0")],
            &[("cacg-core", "cacg-search", vec![None])],
        );
        let v = audit(&m).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].package, "cacg-search");
        assert!(v[0].reason.contains("downstream"));
    }

    #[test]
    fn flags_multiple_distinct_forbidden_packages() {
        let m = make_metadata(
            &[
                ("clap", "registry#clap@4.0.0"),
                ("tokio", "registry#tokio@1.0.0"),
            ],
            &[
                ("cacg-core", "clap", vec![None]),
                ("cacg-core", "tokio", vec![None]),
            ],
        );
        let v = audit(&m).unwrap();
        let mut names: Vec<&str> = v.iter().map(|x| x.package.as_str()).collect();
        names.sort();
        assert_eq!(names, vec!["clap", "tokio"]);
    }

    #[test]
    fn missing_cacg_core_returns_error() {
        let m = serde_json::json!({
            "packages": [],
            "resolve": {"nodes": []},
        })
        .to_string();
        let r = audit(&m);
        assert!(matches!(r, Err(AuditError::CacgCoreNotFound)));
    }

    #[test]
    fn audit_workspace_returns_zero_on_current_repo() {
        // Integration-style: invoke real cargo metadata against the
        // workspace this test lives in.
        let mut workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        workspace_root.pop();
        let r = audit_workspace(&workspace_root)
            .unwrap_or_else(|e| panic!("audit_workspace failed: {e}"));
        assert!(
            r.is_empty(),
            "current workspace MUST have zero forbidden cacg-core deps; got: {r:?}"
        );
    }
}
