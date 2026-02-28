use serde::{Deserialize, Serialize};
use serde_json::to_value;

use crate::{digest_sha256_jcs, CoreError, Evidence, RunDescriptor};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayManifest {
    pub hash_algorithm: String,
    pub rd_digest: String,
    pub input_digest: String,
    pub trace_digest: String,
    pub evidence_digest: String,
    pub head_digest: String,
}

pub fn build_manifest(
    rd: &RunDescriptor,
    input_digest: &str,
    trace_digest: &str,
    evidence: &Evidence,
    head_digest: &str,
) -> Result<ReplayManifest, CoreError> {
    let rd_digest = digest_sha256_jcs(&to_value(rd)?, rd)?;
    let evidence_digest = digest_sha256_jcs(&to_value(evidence)?, rd)?;
    Ok(ReplayManifest {
        hash_algorithm: "sha256".to_string(),
        rd_digest,
        input_digest: input_digest.to_string(),
        trace_digest: trace_digest.to_string(),
        evidence_digest,
        head_digest: head_digest.to_string(),
    })
}
