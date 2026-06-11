#![allow(clippy::unwrap_used)]
//! End-to-end coverage for `kb ingest --format utterances`: a versioned
//! JSONL conversation stream ingests through the standard chunker +
//! manifest publisher with NO pdfium in the chain, and publishes a
//! sealed `locator_map.json` in the same atomic group.

use std::path::Path;
use std::process::Command;

use cacg_ingest::utterances::verify_locator_seal;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

const HEADER: &str = r#"{"schema_version":"cacg.utterances.v1","source_kind":"conversation"}"#;

fn utterance(ordinal: u64, id: &str, speaker: &str, is_author: bool, text: &str) -> String {
    format!(
        r#"{{"ordinal":{ordinal},"utterance_id":"{id}","speaker":"{speaker}","is_author":{is_author},"authored_at":"2022-06-12T06:20:00+08:00","refs":{{"post_id":"222375639"}},"text":"{text}"}}"#
    )
}

/// One-utterance-per-chunk config (the conversational deck shape).
fn write_cjk_config(dir: &Path) -> std::path::PathBuf {
    let p = dir.join("chunk.yaml");
    std::fs::write(
        &p,
        "chunking:\n  target_tokens: 100000\n  overlap_tokens: 0\n  max_pages_per_chunk: 1\n",
    )
    .unwrap();
    p
}

fn run_ingest(stream_path: &Path, out_dir: &Path, config: &Path) -> std::process::Output {
    Command::new(kb_bin())
        .args([
            "ingest",
            stream_path.to_str().unwrap(),
            "--format",
            "utterances",
            "--source-id",
            "demo_corpus",
            "--config",
            config.to_str().unwrap(),
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .unwrap()
}

#[test]
fn utterances_stream_ingests_with_sealed_locator_and_no_markers() {
    let tmp = tempfile::tempdir().unwrap();
    let stream = tmp.path().join("corpus.jsonl");
    std::fs::write(
        &stream,
        [
            HEADER.to_owned(),
            utterance(
                1,
                "p222375639",
                "狗不叫",
                true,
                "上個月sell call，行使價被調整為11.31元。",
            ),
            utterance(
                2,
                "c244701940",
                "狗不叫",
                true,
                "回复@ForeverFight命運: 汪！",
            ),
            utterance(3, "c244700573", "清香绿茶1997", false, "看起来好像财主的ID"),
        ]
        .join("\n"),
    )
    .unwrap();
    let cfg = write_cjk_config(tmp.path());
    let out_dir = tmp.path().join("out");

    let out = run_ingest(&stream, &out_dir, &cfg);
    assert!(
        out.status.success(),
        "ingest must succeed; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Chunk == utterance; chunk text is the PURE utterance text (no
    // in-band @@ markers, the locator carries the anchors instead).
    let chunks: serde_json::Value =
        serde_json::from_slice(&std::fs::read(out_dir.join("chunks_manifest.json")).unwrap())
            .unwrap();
    let arr = chunks["chunks"].as_array().unwrap();
    assert_eq!(arr.len(), 3, "one chunk per utterance");
    assert_eq!(arr[0]["text"], "上個月sell call，行使價被調整為11.31元。");
    assert!(
        !arr[0]["text"].as_str().unwrap().contains("@@"),
        "no in-band markers in chunk text"
    );

    // Sources manifest records the honest non-pdfium parser identity
    // and hashes the raw JSONL bytes.
    let sources: serde_json::Value =
        serde_json::from_slice(&std::fs::read(out_dir.join("sources_manifest.json")).unwrap())
            .unwrap();
    assert_eq!(sources["sources"][0]["parser_name"], "cacg-utterances");
    assert_eq!(sources["sources"][0]["page_count"], 3);

    // The locator sidecar exists, its seal verifies, and it maps every
    // chunk to the right utterance identity (speaker + author flag).
    let locator_bytes = std::fs::read(out_dir.join("locator_map.json")).unwrap();
    assert!(
        verify_locator_seal(&locator_bytes).unwrap(),
        "seal must verify"
    );
    let locator: serde_json::Value = serde_json::from_slice(&locator_bytes).unwrap();
    let c0 = arr[0]["chunk_id"].as_str().unwrap();
    let c2 = arr[2]["chunk_id"].as_str().unwrap();
    assert_eq!(locator["locators"][c0][0]["utterance_id"], "p222375639");
    assert_eq!(locator["locators"][c0][0]["is_author"], true);
    assert_eq!(locator["locators"][c2][0]["speaker"], "清香绿茶1997");
    assert_eq!(locator["locators"][c2][0]["is_author"], false);
    assert_eq!(locator["locators"][c0][0]["refs"]["post_id"], "222375639");
}

#[test]
fn two_frozen_runs_are_byte_identical() {
    let tmp = tempfile::tempdir().unwrap();
    let stream = tmp.path().join("corpus.jsonl");
    std::fs::write(
        &stream,
        [
            HEADER.to_owned(),
            utterance(1, "u1", "a", true, "第一句话。"),
            utterance(2, "u2", "b", false, "第二句话。"),
        ]
        .join("\n"),
    )
    .unwrap();
    let cfg = write_cjk_config(tmp.path());

    let mut digests = Vec::new();
    for run in 0..2 {
        let out_dir = tmp.path().join(format!("out{run}"));
        let out = run_ingest(&stream, &out_dir, &cfg);
        assert!(out.status.success());
        let mut d = Vec::new();
        for f in [
            "sources_manifest.json",
            "chunks_manifest.json",
            "locator_map.json",
        ] {
            d.push(std::fs::read(out_dir.join(f)).unwrap());
        }
        digests.push(d);
    }
    assert_eq!(
        digests[0], digests[1],
        "frozen-clock runs must be byte-identical"
    );
}

#[test]
fn malformed_streams_fail_closed_with_ingest_001() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = write_cjk_config(tmp.path());

    let cases: &[(&str, String)] = &[
        (
            "bad-version",
            format!(
                "{}\n{}",
                r#"{"schema_version":"cacg.utterances.v999","source_kind":"conversation"}"#,
                utterance(1, "u1", "a", true, "text")
            ),
        ),
        (
            "ordinal-break",
            format!("{HEADER}\n{}", utterance(2, "u1", "a", true, "text")),
        ),
        (
            "dup-id",
            format!(
                "{HEADER}\n{}\n{}",
                utterance(1, "u1", "a", true, "text"),
                utterance(2, "u1", "a", true, "text2")
            ),
        ),
        ("header-only", HEADER.to_owned()),
    ];
    for (name, content) in cases {
        let stream = tmp.path().join(format!("{name}.jsonl"));
        std::fs::write(&stream, content).unwrap();
        let out_dir = tmp.path().join(format!("out-{name}"));
        let out = run_ingest(&stream, &out_dir, &cfg);
        assert!(!out.status.success(), "{name} must fail");
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("CACG-INGEST-001"),
            "{name} must surface CACG-INGEST-001; got: {stderr}"
        );
        assert!(
            !out_dir.join("chunks_manifest.json").exists(),
            "{name} must publish nothing"
        );
    }
}

#[test]
fn second_ingest_into_same_out_dir_refuses_clobber() {
    let tmp = tempfile::tempdir().unwrap();
    let stream = tmp.path().join("corpus.jsonl");
    std::fs::write(
        &stream,
        format!(
            "{HEADER}\n{}",
            utterance(1, "u1", "a", true, "原則是不弄丟底倉。")
        ),
    )
    .unwrap();
    let cfg = write_cjk_config(tmp.path());
    let out_dir = tmp.path().join("out");

    assert!(run_ingest(&stream, &out_dir, &cfg).status.success());
    let second = run_ingest(&stream, &out_dir, &cfg);
    assert!(!second.status.success(), "second publish must fail closed");
    assert!(
        String::from_utf8_lossy(&second.stderr).contains("CACG-INGEST-003"),
        "clobber refusal must carry CACG-INGEST-003"
    );
}

#[test]
fn pdf_format_remains_the_default_path() {
    // A JSONL stream WITHOUT --format utterances goes down the PDF
    // path and is rejected as a malformed PDF — proving the default
    // path is untouched.
    let tmp = tempfile::tempdir().unwrap();
    let stream = tmp.path().join("corpus.jsonl");
    std::fs::write(
        &stream,
        format!("{HEADER}\n{}", utterance(1, "u1", "a", true, "text")),
    )
    .unwrap();
    let out = Command::new(kb_bin())
        .args([
            "ingest",
            stream.to_str().unwrap(),
            "--source-id",
            "demo",
            "--out",
            tmp.path().join("out").to_str().unwrap(),
        ])
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("CACG-INGEST-001"),
        "non-PDF bytes down the pdf path must fail with CACG-INGEST-001"
    );
}
