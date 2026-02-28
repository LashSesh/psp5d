use psp5d_model_psp5d::time_trimoebius::{K1Policy, TriMoebiusState};

#[test]
fn k1_k2_bridge_policy_is_deterministic() {
    let mut s = TriMoebiusState {
        k1: 0,
        k2: 0,
        nc_tag: "nc-default".to_string(),
        policy: K1Policy::MirrorOnCommit,
    };
    s.tick();
    s.tick();
    assert_eq!(s.k1, 0);
    assert_eq!(s.k2, 2);
    s.commit();
    assert_eq!(s.k1, 2);
    assert_eq!(s.k2, 2);
}
