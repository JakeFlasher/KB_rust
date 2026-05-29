//! Baseline comparison for iai-callgrind benchmarks.

use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, serde::Deserialize)]
pub struct Baseline {
    pub bench: String,
    pub benchmark: String,
    pub event: String,
    pub instructions: u64,
    pub max_regression_pct: f64,
}

pub fn load_baselines(baselines_dir: &Path) -> io::Result<Vec<Baseline>> {
    let mut baselines = Vec::new();
    let files = [
        "bm25_iai.json",
        "verify_iai.json",
        "lint_iai.json",
        "index_iai.json",
    ];
    for file in &files {
        let path = baselines_dir.join(file);
        if !path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("missing baseline file: {}", path.display()),
            ));
        }
        let content = std::fs::read_to_string(&path)?;
        let baseline: Baseline = serde_json::from_str(&content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("malformed baseline {}: {e}", path.display()),
            )
        })?;
        baselines.push(baseline);
    }
    Ok(baselines)
}

pub fn find_callgrind_output(ws: &Path, bench_name: &str) -> io::Result<PathBuf> {
    let target_dir = ws.join("target/release/deps");
    if !target_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("target/release/deps not found at {}", target_dir.display()),
        ));
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(&target_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(bench_name) && !name.contains('.') && entry.file_type()?.is_file() {
            candidates.push(entry.path());
        }
    }
    candidates.sort();
    candidates.last().cloned().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "no benchmark binary found for {bench_name} in {}",
                target_dir.display()
            ),
        )
    })
}

pub fn parse_instruction_count(ws: &Path, bench_name: &str, benchmark_fn: &str) -> io::Result<u64> {
    let callgrind_file = find_callgrind_file(ws, bench_name, benchmark_fn)?;
    let content = std::fs::read_to_string(&callgrind_file)?;
    parse_ir_from_callgrind(&content).map_err(|msg| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{msg} in {}", callgrind_file.display()),
        )
    })
}

fn parse_ir_from_callgrind(content: &str) -> Result<u64, String> {
    let mut ir_col: Option<usize> = None;
    for line in content.lines() {
        if line.starts_with("events:") {
            let events: Vec<&str> = line
                .trim_start_matches("events:")
                .split_whitespace()
                .collect();
            ir_col = events.iter().position(|&e| e == "Ir");
            if ir_col.is_none() {
                return Err(format!("events line does not contain Ir: {line}"));
            }
        }
        if line.starts_with("summary:") {
            let values: Vec<&str> = line
                .trim_start_matches("summary:")
                .split_whitespace()
                .collect();
            let col = match ir_col {
                Some(c) => c,
                // Single-value summary files don't need an events: line — column 0
                // is unambiguously the only event counter.
                None if values.len() == 1 => 0,
                None => {
                    return Err(format!(
                        "summary has {} columns but no events: line to identify Ir column",
                        values.len()
                    ));
                }
            };
            let val_str = values
                .get(col)
                .ok_or_else(|| format!("summary line has no column {col}"))?;
            return val_str
                .parse::<u64>()
                .map_err(|e| format!("cannot parse Ir value '{val_str}': {e}"));
        }
    }
    Err("no summary line found".to_string())
}

fn find_callgrind_file(ws: &Path, bench_name: &str, benchmark_fn: &str) -> io::Result<PathBuf> {
    let iai_dir = ws.join("target/iai/cacg-core").join(bench_name);
    if !iai_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("iai-callgrind output dir not found: {}", iai_dir.display()),
        ));
    }
    fn walk_for_callgrind(dir: &Path, fn_name: &str) -> io::Result<Option<PathBuf>> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Ok(Some(found)) = walk_for_callgrind(&path, fn_name) {
                    return Ok(Some(found));
                }
            } else {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if name == format!("callgrind.{fn_name}.out") {
                    return Ok(Some(path));
                }
            }
        }
        Ok(None)
    }
    walk_for_callgrind(&iai_dir, benchmark_fn)?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "callgrind.{benchmark_fn}.out not found under {}",
                iai_dir.display()
            ),
        )
    })
}

pub struct CheckResult {
    pub bench: String,
    pub benchmark: String,
    pub baseline_instructions: u64,
    pub current_instructions: u64,
    pub delta_pct: f64,
    pub passed: bool,
}

pub fn check_baselines(ws: &Path) -> io::Result<Vec<CheckResult>> {
    let baselines_dir = ws.join("tests/perf_baselines");
    let baselines = load_baselines(&baselines_dir)?;
    let mut results = Vec::new();

    for baseline in &baselines {
        let current = match parse_instruction_count(ws, &baseline.bench, &baseline.benchmark) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "xtask bench --check: cannot read current counts for {}: {e}",
                    baseline.bench
                );
                results.push(CheckResult {
                    bench: baseline.bench.clone(),
                    benchmark: baseline.benchmark.clone(),
                    baseline_instructions: baseline.instructions,
                    current_instructions: 0,
                    delta_pct: f64::NAN,
                    passed: false,
                });
                continue;
            }
        };

        #[allow(clippy::cast_precision_loss)]
        let delta_pct = if baseline.instructions == 0 {
            0.0
        } else {
            let curr = current as f64;
            let base = baseline.instructions as f64;
            ((curr - base) / base) * 100.0
        };
        let passed = delta_pct <= baseline.max_regression_pct;

        eprintln!(
            "  {bench}: {current} instructions (baseline: {base}, delta: {delta:+.2}%) [{status}]",
            bench = baseline.bench,
            base = baseline.instructions,
            delta = delta_pct,
            status = if passed { "PASS" } else { "FAIL" },
        );

        results.push(CheckResult {
            bench: baseline.bench.clone(),
            benchmark: baseline.benchmark.clone(),
            baseline_instructions: baseline.instructions,
            current_instructions: current,
            delta_pct,
            passed,
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ir_first_column() {
        let content = "\
events: Ir Dr Dw
summary: 428897 72554 39269
";
        assert_eq!(parse_ir_from_callgrind(content).unwrap(), 428_897);
    }

    #[test]
    fn ir_not_first_column() {
        let content = "\
events: Dr Dw Ir I1mr
summary: 72554 39269 428897 316
";
        assert_eq!(parse_ir_from_callgrind(content).unwrap(), 428_897);
    }

    #[test]
    fn no_events_line_defaults_to_first_column() {
        // Single-value summary: column 0 is unambiguous even without events: line.
        let content = "summary: 999\n";
        assert_eq!(parse_ir_from_callgrind(content).unwrap(), 999);
    }

    #[test]
    fn multi_column_summary_without_events_errors() {
        let content = "summary: 72554 39269 428897\n";
        let err = parse_ir_from_callgrind(content).unwrap_err();
        assert!(
            err.contains("no events: line"),
            "expected 'no events: line' error, got: {err}"
        );
    }

    #[test]
    fn missing_ir_in_events() {
        let content = "\
events: Dr Dw
summary: 100 200
";
        let err = parse_ir_from_callgrind(content).unwrap_err();
        assert!(err.contains("does not contain Ir"), "{err}");
    }

    #[test]
    fn no_summary_line() {
        let content = "events: Ir\n";
        let err = parse_ir_from_callgrind(content).unwrap_err();
        assert!(err.contains("no summary"), "{err}");
    }

    #[test]
    fn summary_fewer_columns_than_events() {
        let content = "\
events: Dr Dw Ir I1mr
summary: 72554 39269
";
        let err = parse_ir_from_callgrind(content).unwrap_err();
        assert!(err.contains("no column"), "{err}");
    }
}
