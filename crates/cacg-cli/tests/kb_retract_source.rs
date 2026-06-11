#![allow(clippy::unwrap_used)]
//! End-to-end coverage for `kb retract-source`: the whole-source
//! takedown verb (e.g. a scraped human author requesting removal).
//! Ingest a small utterances corpus, retract the source, and assert
//! every chunk is staled, the retraction list is updated atomically,
//! re-retraction fails closed, and `kb verify` rejects a card citing
//! the retracted source.

use std::path::Path;
use std::process::Command;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

const HEADER: &str = r#"{"schema_version":"cacg.utterances.v1","source_kind":"conversation"}"#;

fn utterance(ordinal: u64, id: &str, text: &str) -> String {
    format!(
        r#"{{"ordinal":{ordinal},"utterance_id":"{id}","speaker":"狗不叫","is_author":true,"text":"{text}"}}"#
    )
}

fn ingest_demo(dir: &Path) -> std::path::PathBuf {
    let stream = dir.join("corpus.jsonl");
    std::fs::write(
        &stream,
        [
            HEADER.to_owned(),
            utterance(1, "u1", "我sell call sell put的原則是不弄丟底倉。"),
            utterance(2, "u2", "不用杠桿是底線，忌貪。"),
        ]
        .join("\n"),
    )
    .unwrap();
    let cfg = dir.join("chunk.yaml");
    std::fs::write(
        &cfg,
        "chunking:\n  target_tokens: 100000\n  overlap_tokens: 0\n  max_pages_per_chunk: 1\n",
    )
    .unwrap();
    let out_dir = dir.join("out");
    let out = Command::new(kb_bin())
        .args([
            "ingest",
            stream.to_str().unwrap(),
            "--format",
            "utterances",
            "--source-id",
            "demo_corpus",
            "--config",
            cfg.to_str().unwrap(),
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    out_dir
}

#[test]
fn retract_source_stales_every_chunk_and_fails_closed_on_repeat() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = ingest_demo(tmp.path());

    let out = Command::new(kb_bin())
        .args([
            "retract-source",
            "demo_corpus",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "retract-source must succeed; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("chunks removed:                2"),
        "{stdout}"
    );
    assert!(
        stdout.contains("active chunks remaining:       0"),
        "{stdout}"
    );

    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(out_dir.join("chunks_manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["chunks"].as_array().unwrap().len(), 0);
    assert_eq!(manifest["retracted_source_ids"][0], "demo_corpus");
    // No leftover publish sidecars.
    assert!(!out_dir.join("chunks_manifest.json.tmp").exists());
    assert!(!out_dir.join("chunks_manifest.json.bak").exists());

    // Retraction is append-only and not idempotent: a second run fails closed.
    let again = Command::new(kb_bin())
        .args([
            "retract-source",
            "demo_corpus",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!again.status.success());
    assert!(
        String::from_utf8_lossy(&again.stderr).contains("already retracted"),
        "{}",
        String::from_utf8_lossy(&again.stderr)
    );
}

#[test]
fn retract_source_unknown_source_fails_closed() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = ingest_demo(tmp.path());
    let out = Command::new(kb_bin())
        .args([
            "retract-source",
            "no_such_source",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("refusing to retract an unknown source"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn verify_rejects_card_citing_retracted_source() {
    let tmp = tempfile::tempdir().unwrap();
    let out_dir = ingest_demo(tmp.path());

    // Author a card against the live chunk BEFORE retraction.
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(out_dir.join("chunks_manifest.json")).unwrap())
            .unwrap();
    let chunk = &manifest["chunks"][0];
    let card = format!(
        r#"---
schema_version: "cacg.v0"
id: "demo-card"
title: "Demo"
reading_id: "07_derivatives_and_volatility"
summary: "{}"
tags: ["xueqiu-2022h1"]
citations:
  - source_id: "demo_corpus"
    chunk_id: "{}"
    chunk_hash: "{}"
    page_range: [1, 1]
    quote: "原則是不弄丟底倉"
    edge_type: "supports"
---

body
"#,
        "s".repeat(90),
        chunk["chunk_id"].as_str().unwrap(),
        chunk["chunk_hash"].as_str().unwrap(),
    );
    let card_path = tmp.path().join("demo-card.md");
    std::fs::write(&card_path, card).unwrap();
    let matrix = tmp.path().join("source_matrix.json");
    std::fs::write(
        &matrix,
        r#"{"schema_version":"cacg.v0","allowed":{"07_derivatives_and_volatility":["demo_corpus"]}}"#,
    )
    .unwrap();

    let verify = |label: &str| {
        let out = Command::new(kb_bin())
            .args([
                "verify",
                card_path.to_str().unwrap(),
                "--chunks-manifest",
                out_dir.join("chunks_manifest.json").to_str().unwrap(),
                "--source-matrix",
                matrix.to_str().unwrap(),
                "--journal",
                tmp.path()
                    .join(format!("j-{label}.jsonl"))
                    .to_str()
                    .unwrap(),
            ])
            .env("KB_FROZEN_CLOCK", "1")
            .output()
            .unwrap();
        out
    };

    let before = verify("before");
    assert!(
        before.status.success(),
        "card must verify green before retraction; stderr={} stdout={}",
        String::from_utf8_lossy(&before.stderr),
        String::from_utf8_lossy(&before.stdout)
    );

    let rt = Command::new(kb_bin())
        .args([
            "retract-source",
            "demo_corpus",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(rt.status.success());

    let after = verify("after");
    assert!(
        !after.status.success(),
        "card citing a retracted source must FAIL verify"
    );
    let surface = format!(
        "{}{}",
        String::from_utf8_lossy(&after.stdout),
        String::from_utf8_lossy(&after.stderr)
    );
    assert!(
        surface.contains("CACG-RETR") || surface.contains("retract"),
        "failure surface must cite retraction; got: {surface}"
    );
}
