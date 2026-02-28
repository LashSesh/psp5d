use psp5d_core::{
    sort_by_score_then_digest, CanonSettings, FloatPolicy, RunDescriptor, ScoredValue, SeedPolicy,
};
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
fn deterministic_tie_break_uses_digest_ascending() {
    let mut values = vec![
        ScoredValue {
            score: 10.0,
            payload: json!({"id":"b"}),
        },
        ScoredValue {
            score: 10.0,
            payload: json!({"id":"a"}),
        },
        ScoredValue {
            score: 11.0,
            payload: json!({"id":"top"}),
        },
    ];

    sort_by_score_then_digest(&mut values, &rd()).expect("sort should pass");

    assert_eq!(values[0].payload, json!({"id":"top"}));
    assert_eq!(values[1].payload, json!({"id":"a"}));
    assert_eq!(values[2].payload, json!({"id":"b"}));
}
