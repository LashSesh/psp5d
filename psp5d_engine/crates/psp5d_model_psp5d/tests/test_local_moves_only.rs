use psp5d_model_psp5d::cube::{enforce_local_move, neighbors_local_moves, CubeConfig};

#[test]
fn enforces_local_move_invariant() {
    let cfg = CubeConfig { n: 3, m: 4 };
    let n = neighbors_local_moves(&[2, 2, 2], &cfg).expect("neighbors");
    assert!(n.contains(&vec![1, 2, 2]));
    assert!(n.contains(&vec![2, 3, 2]));

    enforce_local_move(&[1, 1, 1], &[1, 2, 1]).expect("valid local move");
    assert!(enforce_local_move(&[1, 1, 1], &[2, 2, 1]).is_err());
}
