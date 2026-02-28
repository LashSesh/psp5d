use psp5d_core::{digest_sha256_jcs, CoreError, RunDescriptor};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PiProjection {
    pub pi_id: String,
    pub pi_params_digest: String,
    pub value: f64,
}

pub fn project_pi(uir: &Value, rd: &RunDescriptor) -> Result<PiProjection, CoreError> {
    let d = digest_sha256_jcs(uir, rd)?;
    let val = d.as_bytes().iter().fold(0u64, |a, b| a + *b as u64) as f64 / 1000.0;
    let params = json!({"method":"stable-byte-sum"});
    Ok(PiProjection {
        pi_id: "pi_projection_v1".to_string(),
        pi_params_digest: digest_sha256_jcs(&params, rd)?,
        value: val,
    })
}
