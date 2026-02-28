use psp5d_core::{digest_sha256_jcs, validate_against_schema, RunDescriptor};
use psp5d_model_psp5d::text_to_uir;

#[test]
fn generic_text_frontend_is_deterministic_and_schema_valid() {
    let rd_str = include_str!("../../../examples/rd_min.json");
    let text = include_str!("../../../examples/input_small/text_sample.txt");
    let rd: RunDescriptor = serde_json::from_str(rd_str).expect("rd parse");

    let g1 = text_to_uir("sample", text);
    let g2 = text_to_uir("sample", text);
    assert_eq!(g1, g2);

    let v1 = serde_json::to_value(&g1).expect("serialize");
    validate_against_schema("uir", &v1).expect("UIR schema should validate");

    let d1 = digest_sha256_jcs(&v1, &rd).expect("digest1");
    let d2 =
        digest_sha256_jcs(&serde_json::to_value(&g2).expect("serialize2"), &rd).expect("digest2");
    assert_eq!(d1, d2);
}
