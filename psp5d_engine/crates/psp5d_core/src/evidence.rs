use serde::{Deserialize, Serialize};
use serde_json::to_value;

use crate::{digest_sha256_jcs, CoreError, EngineTrace, RunDescriptor};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceStep {
    pub step: u64,
    pub role: String,
    pub operator_name: String,
    pub in_digest: String,
    pub out_digest: String,
    pub op_evidence_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    pub input_digest: String,
    pub rd_digest: String,
    pub trace_digest: String,
    pub steps: Vec<EvidenceStep>,
}

pub fn build_evidence(
    trace: &EngineTrace,
    rd: &RunDescriptor,
    input_digest: String,
) -> Result<Evidence, CoreError> {
    let rd_digest = digest_sha256_jcs(&to_value(rd)?, rd)?;
    let trace_digest = digest_sha256_jcs(&to_value(trace)?, rd)?;
    let steps = trace
        .iter()
        .map(|t| EvidenceStep {
            step: t.step,
            role: t.role.clone(),
            operator_name: t.operator_name.clone(),
            in_digest: t.in_digest.clone(),
            out_digest: t.out_digest.clone(),
            op_evidence_digest: t.op_evidence_digest.clone(),
        })
        .collect();

    Ok(Evidence {
        input_digest,
        rd_digest,
        trace_digest,
        steps,
    })
}
