use psp5d_core::{canonicalize_with_rd, digest_sha256_jcs, validate_against_schema, RunDescriptor};
use psp5d_model_psp5d::load_uir_json;

#[test]
fn json_frontend_outputs_schema_valid_uir_and_stable_digest() {
    let input = include_str!("../../../examples/input_small/uir_sample.json");
    let rd_str = include_str!("../../../examples/rd_min.json");
    let rd: RunDescriptor = serde_json::from_str(rd_str).expect("rd parse");

    let graph = load_uir_json(input).expect("json frontend should parse");
    let value = serde_json::to_value(&graph).expect("serialize graph");

    validate_against_schema("uir", &value).expect("UIR schema should validate");

    let c1 = canonicalize_with_rd(&value, &rd).expect("canon1");
    let c2 = canonicalize_with_rd(&value, &rd).expect("canon2");
    assert_eq!(c1, c2);

    let d1 = digest_sha256_jcs(&value, &rd).expect("digest1");
    let d2 = digest_sha256_jcs(&value, &rd).expect("digest2");
    assert_eq!(d1, d2);
}
