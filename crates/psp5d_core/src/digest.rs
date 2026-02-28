use crate::canon_jcs::canonicalize_with_rd;
use crate::errors::CoreError;
use crate::rd::RunDescriptor;
use serde_json::Value;
use sha2::{Digest as _, Sha256};

pub fn digest_sha256_jcs(value: &Value, rd: &RunDescriptor) -> Result<String, CoreError> {
    let bytes = canonicalize_with_rd(value, rd)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = hasher.finalize();
    Ok(format!("sha256:{}", hex::encode(hash)))
}
