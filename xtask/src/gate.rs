//! Unified quality gate: runs all lint/audit/test/format checks in sequence.

use std::io;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub status: CheckStatus,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    Pass,
    Fail,
    AdvisoryWarn,
}

struct GateCheck {
    name: &'static str,
    kind: CheckKind,
    advisory: bool,
}

enum CheckKind {
    Cargo(Vec<&'static str>),
    CargoWithEnv(Vec<&'static str>, Vec<(&'static str, &'static str)>),
    Xtask(Vec<&'static str>),
}

pub const GATE_CHECKS: &[&str] = &[
    "cargo-fmt",
    "cargo-clippy",
    "cargo-deny",
    "lint-determinism",
    "lint-platform-cfg",
    "lint-trust-leak",
    "lint-rename-outside-publisher",
    "lint-runner-bypass",
    "lint-workflow-integrity",
    "lint-workflow-labels",
    "audit-cacg-core-deps",
    "audit-default-kb-deps",
    "audit-schema-fixtures",
    "audit-semantic-cache-provenance",
    "lint-unwrap",
    "lint-error-swallow",
    "lint-structural",
    "cargo-test",
];

fn gate_checks() -> Vec<GateCheck> {
    vec![
        GateCheck {
            name: "cargo-fmt",
            kind: CheckKind::Cargo(vec!["fmt", "--all", "--", "--check"]),
            advisory: false,
        },
        GateCheck {
            name: "cargo-clippy",
            kind: CheckKind::Cargo(vec![
                "clippy",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ]),
            advisory: false,
        },
        // Only check bans+licenses locally; CI runs `cargo deny check` (all
        // advisors included).  The advisory DB now contains CVSS 4.0 entries
        // that cargo-deny 0.18.x cannot parse, so the local gate intentionally
        // skips the advisories category to avoid false failures on developer
        // machines whose toolchain may lag behind CI.
        GateCheck {
            name: "cargo-deny",
            kind: CheckKind::Cargo(vec!["deny", "check", "bans", "licenses"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-determinism",
            kind: CheckKind::Xtask(vec!["lint-determinism"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-platform-cfg",
            kind: CheckKind::Xtask(vec!["lint-platform-cfg"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-trust-leak",
            kind: CheckKind::Xtask(vec!["lint-trust-leak"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-rename-outside-publisher",
            kind: CheckKind::Xtask(vec!["lint-rename-outside-publisher"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-runner-bypass",
            kind: CheckKind::Xtask(vec!["lint-runner-bypass"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-workflow-integrity",
            kind: CheckKind::Xtask(vec!["lint-workflow-integrity"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-workflow-labels",
            kind: CheckKind::Xtask(vec!["lint-workflow-labels"]),
            advisory: false,
        },
        GateCheck {
            name: "audit-cacg-core-deps",
            kind: CheckKind::Xtask(vec!["audit-cacg-core-deps"]),
            advisory: false,
        },
        GateCheck {
            name: "audit-default-kb-deps",
            kind: CheckKind::Xtask(vec!["audit-default-kb-deps"]),
            advisory: false,
        },
        GateCheck {
            name: "audit-schema-fixtures",
            kind: CheckKind::Xtask(vec!["audit-schema-fixtures"]),
            advisory: false,
        },
        GateCheck {
            name: "audit-semantic-cache-provenance",
            kind: CheckKind::Xtask(vec!["audit-semantic-cache-provenance"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-unwrap",
            kind: CheckKind::Xtask(vec!["lint-unwrap"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-error-swallow",
            kind: CheckKind::Xtask(vec!["lint-error-swallow"]),
            advisory: false,
        },
        GateCheck {
            name: "lint-structural",
            kind: CheckKind::Xtask(vec!["lint-structural"]),
            advisory: false,
        },
        GateCheck {
            name: "cargo-test",
            kind: CheckKind::CargoWithEnv(
                vec!["test", "--workspace", "--all-targets"],
                vec![("KB_FROZEN_CLOCK", "1")],
            ),
            advisory: false,
        },
    ]
}

fn find_workspace_root() -> io::Result<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ws = manifest_dir
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "cannot find workspace root"))?;
    Ok(ws.to_path_buf())
}

fn run_check(check: &GateCheck, ws: &Path) -> CheckResult {
    let start = Instant::now();
    let output = match &check.kind {
        CheckKind::Cargo(args) => Command::new("cargo").args(args).current_dir(ws).output(),
        CheckKind::CargoWithEnv(args, envs) => {
            let mut cmd = Command::new("cargo");
            cmd.args(args).current_dir(ws);
            for (k, v) in envs {
                cmd.env(k, v);
            }
            cmd.output()
        }
        CheckKind::Xtask(args) => Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("xtask")
            .arg("--quiet")
            .arg("--")
            .args(args)
            .current_dir(ws)
            .output(),
    };
    let elapsed = start.elapsed().as_millis() as u64;

    match output {
        Ok(out) if out.status.success() => {
            if check.advisory {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let has_violations = stderr.contains("violation");
                CheckResult {
                    name: check.name.to_string(),
                    status: if has_violations {
                        CheckStatus::AdvisoryWarn
                    } else {
                        CheckStatus::Pass
                    },
                    duration_ms: elapsed,
                    message: if has_violations {
                        Some(stderr.trim().to_string())
                    } else {
                        None
                    },
                }
            } else {
                CheckResult {
                    name: check.name.to_string(),
                    status: CheckStatus::Pass,
                    duration_ms: elapsed,
                    message: None,
                }
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            if check.advisory {
                CheckResult {
                    name: check.name.to_string(),
                    status: CheckStatus::AdvisoryWarn,
                    duration_ms: elapsed,
                    message: Some(stderr.trim().to_string()),
                }
            } else {
                CheckResult {
                    name: check.name.to_string(),
                    status: CheckStatus::Fail,
                    duration_ms: elapsed,
                    message: Some(stderr.trim().to_string()),
                }
            }
        }
        Err(e) => CheckResult {
            name: check.name.to_string(),
            status: if check.advisory {
                CheckStatus::AdvisoryWarn
            } else {
                CheckStatus::Fail
            },
            duration_ms: elapsed,
            message: Some(format!("failed to execute: {e}")),
        },
    }
}

pub fn run_gate(report_mode: bool) -> io::Result<Vec<CheckResult>> {
    let ws = find_workspace_root()?;
    let checks = gate_checks();
    let mut results = Vec::with_capacity(checks.len());
    let mut any_failed = false;

    for check in &checks {
        let result = run_check(check, &ws);
        let status_label = match &result.status {
            CheckStatus::Pass => "PASS",
            CheckStatus::Fail => "FAIL",
            CheckStatus::AdvisoryWarn => "ADVISORY",
        };
        eprintln!(
            "  [{status_label}] {} ({} ms)",
            result.name, result.duration_ms
        );
        if result.status == CheckStatus::Fail {
            if let Some(msg) = &result.message {
                for line in msg.lines().take(10) {
                    eprintln!("         {line}");
                }
            }
            any_failed = true;
        }
        results.push(result);

        if any_failed && !report_mode {
            break;
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_membership_is_complete() {
        let checks = gate_checks();
        let names: Vec<&str> = checks.iter().map(|c| c.name).collect();
        assert_eq!(
            names.as_slice(),
            GATE_CHECKS,
            "gate_checks() must match GATE_CHECKS in order"
        );
    }

    #[test]
    fn check_result_json_format() {
        let result = CheckResult {
            name: "cargo-fmt".to_string(),
            status: CheckStatus::Pass,
            duration_ms: 42,
            message: None,
        };
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["name"], "cargo-fmt");
        assert_eq!(json["status"], "pass");
        assert_eq!(json["duration_ms"], 42);
        assert!(json.get("message").is_none());
    }

    #[test]
    fn check_result_json_format_with_message() {
        let result = CheckResult {
            name: "cargo-clippy".to_string(),
            status: CheckStatus::Fail,
            duration_ms: 100,
            message: Some("error: clippy violation".to_string()),
        };
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["name"], "cargo-clippy");
        assert_eq!(json["status"], "fail");
        assert_eq!(json["duration_ms"], 100);
        assert_eq!(json["message"], "error: clippy violation");
    }

    #[test]
    fn advisory_status_serializes_correctly() {
        let result = CheckResult {
            name: "lint-structural".to_string(),
            status: CheckStatus::AdvisoryWarn,
            duration_ms: 50,
            message: Some("2 violations (advisory)".to_string()),
        };
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["status"], "advisory_warn");
    }

    #[test]
    fn report_array_golden_test() {
        let results = vec![
            CheckResult {
                name: "cargo-fmt".to_string(),
                status: CheckStatus::Pass,
                duration_ms: 100,
                message: None,
            },
            CheckResult {
                name: "cargo-clippy".to_string(),
                status: CheckStatus::Fail,
                duration_ms: 200,
                message: Some("error: clippy warning".to_string()),
            },
            CheckResult {
                name: "lint-structural".to_string(),
                status: CheckStatus::AdvisoryWarn,
                duration_ms: 50,
                message: Some("1 advisory violation".to_string()),
            },
        ];
        let json_str = serde_json::to_string_pretty(&results).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed.len(), 3);

        assert_eq!(parsed[0]["name"], "cargo-fmt");
        assert_eq!(parsed[0]["status"], "pass");
        assert_eq!(parsed[0]["duration_ms"], 100);
        assert!(parsed[0].get("message").is_none());

        assert_eq!(parsed[1]["name"], "cargo-clippy");
        assert_eq!(parsed[1]["status"], "fail");
        assert_eq!(parsed[1]["duration_ms"], 200);
        assert_eq!(parsed[1]["message"], "error: clippy warning");

        assert_eq!(parsed[2]["name"], "lint-structural");
        assert_eq!(parsed[2]["status"], "advisory_warn");

        for entry in &parsed {
            assert!(entry.get("name").is_some(), "missing name field");
            assert!(entry.get("status").is_some(), "missing status field");
            assert!(
                entry.get("duration_ms").is_some(),
                "missing duration_ms field"
            );
        }
    }

    #[test]
    fn malformed_json_fails_golden_parse() {
        let malformed = r#"[{"name": "cargo-fmt", "status": "pass", "duration_ms": 42, BROKEN}]"#;
        let result: Result<Vec<CheckResult>, _> = serde_json::from_str(malformed);
        assert!(
            result.is_err(),
            "malformed JSON must fail to parse as Vec<CheckResult>"
        );
    }
}
