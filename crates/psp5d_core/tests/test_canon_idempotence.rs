use psp5d_core::{canonicalize_with_rd, CanonSettings, FloatPolicy, RunDescriptor, SeedPolicy};
use serde_json::json;

fn rd() -> RunDescriptor {
    RunDescriptor {
        spec_version: "1.1".to_string(),
        engine_version: "0.1.0".to_string(),
        model_pack_version: "0.1.0".to_string(),
        seed_policy: SeedPolicy::None,
        io_policy: vec![],
        normalization_profile: "default".to_string(),
        canon: CanonSettings {
            float_policy: FloatPolicy::Q16_16RoundHalfEven,
        },
    }
}

#[test]
fn canon_is_idempotent_structurally() {
    let input = json!({
        "z": [1.2345678, 1.5, -2.5],
        "a": {"k": "v", "n": 1.0}
    });

    let c1 = canonicalize_with_rd(&input, &rd()).expect("canonicalization 1");
    let parsed: serde_json::Value = serde_json::from_slice(&c1).expect("parse canon output");
    let c2 = canonicalize_with_rd(&parsed, &rd()).expect("canonicalization 2");

    assert_eq!(c1, c2);
}
