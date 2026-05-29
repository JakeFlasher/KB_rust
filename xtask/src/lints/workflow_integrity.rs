use std::io;
use std::path::Path;

const REQUIRED_CI_CHECKS: &[&str] = &[
    "cargo fmt --all -- --check",
    "cargo clippy --workspace --all-targets --all-features -- -D warnings",
    "cargo deny check",
    "cargo run -p xtask -- lint-determinism",
    "cargo run -p xtask -- lint-trust-leak",
    "cargo run -p xtask -- lint-runner-bypass",
    "cargo run -p xtask -- lint-rename-outside-publisher",
    "cargo run -p xtask -- audit-cacg-core-deps",
    "cargo run -p xtask -- audit-default-kb-deps",
    "cargo run -p xtask -- audit-schema-fixtures",
    "cargo run -p xtask -- lint-workflow-labels",
    "cargo run -p xtask -- audit-semantic-cache-provenance",
    "cargo run -p xtask -- lint-unwrap",
    "cargo run -p xtask -- lint-error-swallow",
    "cargo run -p xtask -- lint-structural",
    "cargo test --workspace --all-targets",
];

pub fn lint(workspace_root: &Path) -> io::Result<()> {
    let mut violations: Vec<String> = Vec::new();

    validate_parity_workflow(workspace_root, &mut violations)?;
    validate_ci_workflow(workspace_root, &mut violations)?;

    if violations.is_empty() {
        eprintln!("xtask lint-workflow-integrity: PASS");
        Ok(())
    } else {
        for v in &violations {
            eprintln!("  {v}");
        }
        Err(io::Error::other(format!(
            "xtask lint-workflow-integrity: {} violation(s)",
            violations.len()
        )))
    }
}

fn validate_parity_workflow(workspace_root: &Path, violations: &mut Vec<String>) -> io::Result<()> {
    let parity_path = workspace_root.join(".github/workflows/parity.yml");
    if !parity_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "parity workflow missing at .github/workflows/parity.yml",
        ));
    }
    let text = std::fs::read_to_string(&parity_path)?;

    if !text.contains("pull_request:") {
        violations.push("parity.yml missing on.pull_request trigger".to_string());
    }
    if !text.contains("push:") {
        violations.push("parity.yml missing on.push trigger".to_string());
    }
    if !text.contains("cargo xtask parity") && !text.contains("cargo run -p xtask -- parity") {
        violations.push("parity.yml has no unconditional cargo xtask parity step".to_string());
    }
    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed == "if: false" || trimmed == "if: 'false'" {
            violations.push(format!("parity.yml:{}: forbidden if:false bypass", i + 1));
        }
        if trimmed.starts_with("continue-on-error:") && trimmed.contains("true") {
            violations.push(format!(
                "parity.yml:{}: forbidden continue-on-error: true",
                i + 1
            ));
        }
        if (trimmed.contains("|| true") || trimmed.contains("; true") || trimmed.contains(";:"))
            && trimmed.contains("parity")
        {
            violations.push(format!(
                "parity.yml:{}: exit-code masking on parity step",
                i + 1
            ));
        }
    }
    Ok(())
}

fn validate_ci_workflow(workspace_root: &Path, violations: &mut Vec<String>) -> io::Result<()> {
    let ci_path = workspace_root.join(".github/workflows/ci.yml");
    if !ci_path.is_file() {
        violations.push("ci.yml missing at .github/workflows/ci.yml".to_string());
        return Ok(());
    }
    let text = std::fs::read_to_string(&ci_path)?;

    for required in REQUIRED_CI_CHECKS {
        if !text.contains(required) {
            violations.push(format!("ci.yml missing required check: {required}"));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_ci_checks_match_gate_membership() {
        use crate::gate::GATE_CHECKS;

        let ci_check_commands: Vec<&str> = REQUIRED_CI_CHECKS
            .iter()
            .map(|c| {
                if c.starts_with("cargo run -p xtask -- ") {
                    &c["cargo run -p xtask -- ".len()..]
                } else if c.starts_with("cargo fmt") {
                    "cargo-fmt"
                } else if c.starts_with("cargo clippy") {
                    "cargo-clippy"
                } else if c.starts_with("cargo deny") {
                    "cargo-deny"
                } else if c.starts_with("cargo test") {
                    "cargo-test"
                } else {
                    *c
                }
            })
            .collect();

        // These checks run in _validate-workflows.yml, not ci.yml
        let meta_validation_checks = ["lint-platform-cfg", "lint-workflow-integrity"];

        for gate_check in GATE_CHECKS {
            if meta_validation_checks.contains(gate_check) {
                continue;
            }
            assert!(
                ci_check_commands.contains(gate_check),
                "gate check {gate_check:?} is not represented in REQUIRED_CI_CHECKS"
            );
        }
    }
}
