#![allow(clippy::unwrap_used)]
//! End-to-end coverage for `kb ingest --format utterances --append`:
//! a living conversational corpus grows WITHOUT invalidating a single
//! previously-published chunk. Every prior chunk must re-derive
//! byte-identical from the new stream — a mid-stream edit, insert, or
//! delete hard-fails with no silent re-anchor — and prior retractions
//! carry over.

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

fn write_stream(dir: &Path, name: &str, records: &[String]) -> std::path::PathBuf {
    let p = dir.join(name);
    let mut lines = vec![HEADER.to_owned()];
    lines.extend_from_slice(records);
    std::fs::write(&p, lines.join("\n")).unwrap();
    p
}

fn cjk_config(dir: &Path) -> std::path::PathBuf {
    let p = dir.join("chunk.yaml");
    std::fs::write(
        &p,
        "chunking:\n  target_tokens: 100000\n  overlap_tokens: 0\n  max_pages_per_chunk: 1\n",
    )
    .unwrap();
    p
}

fn ingest(stream: &Path, out_dir: &Path, cfg: &Path, append: bool) -> std::process::Output {
    let mut args = vec![
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
    ];
    if append {
        args.push("--append");
    }
    Command::new(kb_bin())
        .args(&args)
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .unwrap()
}

fn chunk_pairs(out_dir: &Path) -> Vec<(String, String)> {
    let m: serde_json::Value =
        serde_json::from_slice(&std::fs::read(out_dir.join("chunks_manifest.json")).unwrap())
            .unwrap();
    m["chunks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            (
                c["chunk_id"].as_str().unwrap().to_owned(),
                c["chunk_hash"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

#[test]
fn append_extends_without_touching_prior_chunks() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = cjk_config(tmp.path());
    let out_dir = tmp.path().join("out");

    let v1 = write_stream(
        tmp.path(),
        "v1.jsonl",
        &[
            utterance(1, "u1", "第一句。"),
            utterance(2, "u2", "第二句。"),
        ],
    );
    assert!(ingest(&v1, &out_dir, &cfg, false).status.success());
    let prior = chunk_pairs(&out_dir);
    assert_eq!(prior.len(), 2);

    let v2 = write_stream(
        tmp.path(),
        "v2.jsonl",
        &[
            utterance(1, "u1", "第一句。"),
            utterance(2, "u2", "第二句。"),
            utterance(3, "u3", "新增的第三句。"),
        ],
    );
    let out = ingest(&v2, &out_dir, &cfg, true);
    assert!(
        out.status.success(),
        "append must succeed; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("appended chunks:  1"),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );

    let after = chunk_pairs(&out_dir);
    assert_eq!(after.len(), 3);
    // Every prior (chunk_id, chunk_hash) pair survives byte-identical.
    for p in &prior {
        assert!(after.contains(p), "prior chunk {p:?} must be unchanged");
    }
    // Locator reseals over the new state and covers the new chunk.
    let loc = std::fs::read(out_dir.join("locator_map.json")).unwrap();
    assert!(cacg_ingest::utterances::verify_locator_seal(&loc).unwrap());
    let loc: serde_json::Value = serde_json::from_slice(&loc).unwrap();
    assert_eq!(loc["locators"].as_object().unwrap().len(), 3);
}

#[test]
fn mid_stream_edit_hard_fails_with_no_silent_reanchor() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = cjk_config(tmp.path());
    let out_dir = tmp.path().join("out");

    let v1 = write_stream(
        tmp.path(),
        "v1.jsonl",
        &[
            utterance(1, "u1", "第一句。"),
            utterance(2, "u2", "第二句。"),
        ],
    );
    assert!(ingest(&v1, &out_dir, &cfg, false).status.success());
    let prior = chunk_pairs(&out_dir);

    // EDIT utterance 1's text mid-stream.
    let edited = write_stream(
        tmp.path(),
        "edited.jsonl",
        &[
            utterance(1, "u1", "第一句被改了。"),
            utterance(2, "u2", "第二句。"),
            utterance(3, "u3", "新增。"),
        ],
    );
    let out = ingest(&edited, &out_dir, &cfg, true);
    assert!(!out.status.success(), "edited stream must be rejected");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("did not re-derive byte-identical")
            && stderr.contains("no silent re-anchor"),
        "{stderr}"
    );
    // The prior manifests are untouched.
    assert_eq!(chunk_pairs(&out_dir), prior);

    // INSERT mid-stream (shifts every later page) must also fail.
    let inserted = write_stream(
        tmp.path(),
        "inserted.jsonl",
        &[
            utterance(1, "u1", "第一句。"),
            utterance(2, "u9", "插入的句子。"),
            utterance(3, "u2", "第二句。"),
        ],
    );
    let out = ingest(&inserted, &out_dir, &cfg, true);
    assert!(!out.status.success(), "mid-stream insert must be rejected");
    assert_eq!(chunk_pairs(&out_dir), prior);
}

#[test]
fn identical_stream_has_nothing_to_append() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = cjk_config(tmp.path());
    let out_dir = tmp.path().join("out");
    let v1 = write_stream(tmp.path(), "v1.jsonl", &[utterance(1, "u1", "第一句。")]);
    assert!(ingest(&v1, &out_dir, &cfg, false).status.success());
    let out = ingest(&v1, &out_dir, &cfg, true);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("nothing to append"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn retractions_carry_over_through_append() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = cjk_config(tmp.path());
    let out_dir = tmp.path().join("out");
    let v1 = write_stream(
        tmp.path(),
        "v1.jsonl",
        &[
            utterance(1, "u1", "要被撤回的句子。"),
            utterance(2, "u2", "第二句。"),
        ],
    );
    assert!(ingest(&v1, &out_dir, &cfg, false).status.success());
    let first_chunk = chunk_pairs(&out_dir)[0].0.clone();

    let rt = Command::new(kb_bin())
        .args([
            "retract-chunk",
            &first_chunk,
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        rt.status.success(),
        "{}",
        String::from_utf8_lossy(&rt.stderr)
    );

    let v2 = write_stream(
        tmp.path(),
        "v2.jsonl",
        &[
            utterance(1, "u1", "要被撤回的句子。"),
            utterance(2, "u2", "第二句。"),
            utterance(3, "u3", "新增。"),
        ],
    );
    let out = ingest(&v2, &out_dir, &cfg, true);
    assert!(
        out.status.success(),
        "append over a partial retraction must succeed; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let m: serde_json::Value =
        serde_json::from_slice(&std::fs::read(out_dir.join("chunks_manifest.json")).unwrap())
            .unwrap();
    let active: Vec<&str> = m["chunks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["chunk_id"].as_str().unwrap())
        .collect();
    assert!(
        !active.contains(&first_chunk.as_str()),
        "retracted chunk must NOT be resurrected by the append"
    );
    assert_eq!(m["retracted_chunk_ids"][0], first_chunk);
    assert_eq!(active.len(), 2, "u2 + new u3 active");
}

#[test]
fn append_requires_utterances_format_and_prior_state() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = cjk_config(tmp.path());
    let v1 = write_stream(tmp.path(), "v1.jsonl", &[utterance(1, "u1", "第一句。")]);

    // --append with the pdf default format is a usage error.
    let out = Command::new(kb_bin())
        .args([
            "ingest",
            v1.to_str().unwrap(),
            "--append",
            "--source-id",
            "demo_corpus",
            "--out",
            tmp.path().join("o1").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("--append requires --format utterances"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    // --append into an empty out dir fails closed.
    let out = ingest(&v1, &tmp.path().join("o2"), &cfg, true);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("requires prior manifests"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
