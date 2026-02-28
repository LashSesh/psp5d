use std::fs;
use std::path::Path;

use psp5d_core::{
    build_evidence, build_manifest, digest_sha256_jcs, verify_manifest_consistency, RunDescriptor,
    TraceEntry,
};

#[test]
fn golden_trace_matches_expected_digests_byte_identically() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/golden_trace");
    let rd: RunDescriptor =
        serde_json::from_str(&fs::read_to_string(root.join("rd.json")).expect("read rd"))
            .expect("parse rd");

    let input_raw = fs::read_to_string(root.join("input/input.txt")).expect("read input");
    let input_digest =
        digest_sha256_jcs(&serde_json::json!({"input":input_raw}), &rd).expect("input digest");

    let trace_raw = fs::read_to_string(root.join("expected/trace.jsonl")).expect("read trace");
    let trace: Vec<TraceEntry> = trace_raw
        .lines()
        .map(|l| serde_json::from_str(l).expect("trace line"))
        .collect();

    let evidence = build_evidence(&trace, &rd, input_digest.clone()).expect("evidence");
    let trace_digest = digest_sha256_jcs(&serde_json::to_value(&trace).expect("trace value"), &rd)
        .expect("trace digest");
    let head = trace.last().expect("trace non-empty").out_digest.clone();
    let manifest =
        build_manifest(&rd, &input_digest, &trace_digest, &evidence, &head).expect("manifest");
    verify_manifest_consistency(&evidence, &manifest).expect("self-consistent");

    let expected_evidence =
        fs::read_to_string(root.join("expected/evidence.json")).expect("read expected evidence");
    let expected_manifest =
        fs::read_to_string(root.join("expected/manifest.json")).expect("read expected manifest");

    assert_eq!(
        serde_json::to_string(&evidence).expect("evidence json"),
        expected_evidence
    );
    assert_eq!(
        serde_json::to_string(&manifest).expect("manifest json"),
        expected_manifest
    );
}
