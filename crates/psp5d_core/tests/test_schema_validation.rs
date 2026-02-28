use psp5d_core::validate_against_schema;
use serde_json::json;

#[test]
fn validates_minimal_rd() {
    let rd = json!({
        "spec_version": "1.1",
        "engine_version": "0.1.0",
        "model_pack_version": "0.1.0",
        "seed_policy": "none",
        "io_policy": [],
        "normalization_profile": "default",
        "canon": { "float_policy": "q16_16_round_half_even" }
    });

    validate_against_schema("rd", &rd).expect("rd should validate");
}

#[test]
fn rejects_invalid_manifest_hash_algorithm() {
    let manifest = json!({
        "hash_algorithm": "sha512",
        "rd_hash": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        "initial_sigma_hash": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        "trace_hash": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        "evidence_hashes": []
    });

    assert!(validate_against_schema("manifest", &manifest).is_err());
}
