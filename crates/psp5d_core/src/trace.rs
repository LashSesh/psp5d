use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceEntry {
    pub step: u64,
    pub role: String,
    pub operator_name: String,
    pub in_digest: String,
    pub out_digest: String,
    pub op_evidence_digest: String,
}

pub type EngineTrace = Vec<TraceEntry>;
