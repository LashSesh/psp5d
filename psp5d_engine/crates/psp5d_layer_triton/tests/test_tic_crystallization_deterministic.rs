use psp5d_core::RunDescriptor;
use psp5d_layer_triton::{crystallize, ExplorationPoint, SpectralSignature, TritonContext};

fn rd() -> RunDescriptor {
    serde_json::from_str(include_str!("../../../examples/rd_min.json")).expect("rd")
}

fn point(id: &str, psi: i32, rho: i32, omega: i32) -> ExplorationPoint {
    ExplorationPoint {
        coords_q: [1, 2, 3, 4, 5],
        sigma: SpectralSignature {
            psi_q: psi,
            rho_q: rho,
            omega_q: omega,
        },
        id_digest: id.to_string(),
    }
}

#[test]
fn tic_crystallization_is_deterministic() {
    let ctx = TritonContext {
        rd: rd(),
        spiral_delta_q: 1,
        solve_threshold_q: 100,
        coagula_threshold_q: 100,
        tic_min_points: 2,
    };

    let points = vec![
        point(
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            100,
            100,
            100,
        ),
        point(
            "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            100,
            100,
            100,
        ),
        point(
            "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
            90,
            100,
            100,
        ),
    ];

    let c1 = crystallize(&points, &ctx).expect("c1");
    let c2 = crystallize(&points, &ctx).expect("c2");
    assert_eq!(c1, c2);
}
