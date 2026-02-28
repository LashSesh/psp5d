use psp5d_core::{digest_sha256_jcs, CanonSettings, FloatPolicy, RunDescriptor, SeedPolicy};
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
fn same_object_and_rd_produce_stable_digest() {
    let obj = json!({"b":2,"a":1.3333333333});
    let d1 = digest_sha256_jcs(&obj, &rd()).expect("digest 1");
    let d2 = digest_sha256_jcs(&obj, &rd()).expect("digest 2");

    assert_eq!(d1, d2);
    assert!(d1.starts_with("sha256:"));
    assert_eq!(d1.len(), 71);
}
