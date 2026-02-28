use psp5d_core::RunDescriptor;
use psp5d_layer_triton::{DeterministicEvaluator, SpiralState, TritonContext};

fn rd() -> RunDescriptor {
    serde_json::from_str(include_str!("../../../examples/rd_min.json")).expect("rd")
}

#[test]
fn same_rd_same_state_same_spiral_points() {
    let ctx = TritonContext {
        rd: rd(),
        spiral_delta_q: 3,
        solve_threshold_q: 1_000_000,
        coagula_threshold_q: 1_000_000,
        tic_min_points: 3,
    };
    let mut a = SpiralState {
        coords_q: [0; 5],
        momentum_q: [1, 1, 1, 1, 1],
        step: 0,
    };
    let mut b = a.clone();
    let mut eval_a = DeterministicEvaluator;
    let mut eval_b = DeterministicEvaluator;

    for _ in 0..20 {
        let pa = a.next(&mut eval_a, &ctx).expect("step a");
        let pb = b.next(&mut eval_b, &ctx).expect("step b");
        assert_eq!(pa, pb);
    }
}
