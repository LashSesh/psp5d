use psp5d_core::{digest_sha256_jcs, RunDescriptor};
use psp5d_model_psp5d::block::{make_block, BlockHeader, BlockPayload};
use psp5d_model_psp5d::gates_r5::{consensus_gate, GateConfig};
use serde_json::json;

#[test]
fn block_digest_is_stable_when_gate_passes() {
    let rd: RunDescriptor =
        serde_json::from_str(include_str!("../../../examples/rd_min.json")).expect("rd");
    let rd_digest =
        digest_sha256_jcs(&serde_json::to_value(&rd).expect("rd val"), &rd).expect("rd digest");
    let gate = consensus_gate(
        &GateConfig {
            eps_diam: 1.0,
            eps_pi: 1.0,
            theta_consensus: 1.0,
            weights: vec![1.0, 1.0],
        },
        0.5,
        0.5,
    );
    assert!(gate.passed);

    let header = BlockHeader {
        k1: 1,
        k2: 1,
        nc_tag: "nc-default".to_string(),
        prev_digest: "sha256:0000000000000000000000000000000000000000000000000000000000000000"
            .to_string(),
        rd_digest,
        pi_id: "pi_projection_v1".to_string(),
        pi_params_digest: "sha256:1111111111111111111111111111111111111111111111111111111111111111"
            .to_string(),
    };
    let payload = BlockPayload {
        uir_digest: "sha256:2222222222222222222222222222222222222222222222222222222222222222"
            .to_string(),
        astar_digest: "sha256:3333333333333333333333333333333333333333333333333333333333333333"
            .to_string(),
        metrics: json!({"diam":0.5,"delta_pi":0.5}),
        gate_report: serde_json::to_value(gate).expect("gate val"),
        artifact_digests: vec![
            "sha256:4444444444444444444444444444444444444444444444444444444444444444".to_string(),
        ],
    };

    let b1 = make_block(header.clone(), payload.clone(), &rd).expect("block1");
    let b2 = make_block(header, payload, &rd).expect("block2");
    assert_eq!(b1.block_digest, b2.block_digest);
}
