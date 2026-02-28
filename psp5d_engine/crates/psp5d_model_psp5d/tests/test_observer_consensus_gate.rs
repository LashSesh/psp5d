use psp5d_core::RunDescriptor;
use psp5d_model_psp5d::gates_r5::{consensus_gate, GateConfig};
use psp5d_model_psp5d::observer_r5::observe_r5;
use psp5d_model_psp5d::run_10_steps;

#[test]
fn observer_and_consensus_gate_are_deterministic() {
    let rd: RunDescriptor =
        serde_json::from_str(include_str!("../../../examples/rd_min.json")).expect("rd");
    let (_state, trace) = run_10_steps(&rd).expect("engine run");
    let (_state2, trace2) = run_10_steps(&rd).expect("engine run 2");
    assert_eq!(trace, trace2);
    let r5 = observe_r5(&trace, &rd, 5);
    let cfg = GateConfig {
        eps_diam: 0.1,
        eps_pi: 0.1,
        theta_consensus: 1.0,
        weights: vec![1.0, 1.0],
    };
    let rep = consensus_gate(&cfg, r5.psi / 10.0, r5.rho / 10.0);
    assert!(rep.passed);
}
