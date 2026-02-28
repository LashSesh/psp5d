use psp5d_core::{digest_sha256_jcs, validate_against_schema, RunDescriptor};
use psp5d_layer_triton::{evaluate_solve_coagula, SpectralSignature, TritonContext};

fn rd() -> RunDescriptor {
    serde_json::from_str(include_str!("../../../examples/rd_min.json")).expect("rd")
}

#[test]
fn gate_evidence_is_replay_stable() {
    let ctx = TritonContext {
        rd: rd(),
        spiral_delta_q: 2,
        solve_threshold_q: 500_000,
        coagula_threshold_q: 250_000,
        tic_min_points: 2,
    };
    let sigma = SpectralSignature {
        psi_q: 10,
        rho_q: 20,
        omega_q: 30,
    };

    let g1 = evaluate_solve_coagula(&sigma, &ctx).expect("gate1");
    let g2 = evaluate_solve_coagula(&sigma, &ctx).expect("gate2");
    assert_eq!(g1, g2);

    let v = serde_json::to_value(&g1).expect("value");
    validate_against_schema("triton_gate_evidence", &v).expect("schema valid");

    let d1 = digest_sha256_jcs(&v, &ctx.rd).expect("d1");
    let d2 = digest_sha256_jcs(&serde_json::to_value(&g2).expect("value2"), &ctx.rd).expect("d2");
    assert_eq!(d1, d2);
}
