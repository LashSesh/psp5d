use psp5d_core::{first_divergence, TraceEntry};

#[test]
fn reports_first_divergence_operator_and_step() {
    let expected = vec![
        TraceEntry {
            step: 0,
            role: "ingest".to_string(),
            operator_name: "ingest_v1".to_string(),
            in_digest: "sha256:a".to_string(),
            out_digest: "sha256:b".to_string(),
            op_evidence_digest: "sha256:c".to_string(),
        },
        TraceEntry {
            step: 1,
            role: "canon".to_string(),
            operator_name: "canon_v1".to_string(),
            in_digest: "sha256:b".to_string(),
            out_digest: "sha256:d".to_string(),
            op_evidence_digest: "sha256:e".to_string(),
        },
    ];
    let mut actual = expected.clone();
    actual[1].out_digest = "sha256:DIFF".to_string();

    let div = first_divergence(&expected, &actual).expect("must diverge");
    assert_eq!(div.step, 1);
    assert_eq!(div.role, "canon");
    assert_eq!(div.operator_name, "canon_v1");
}
