use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Digest(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sigma {
    pub sigma_id: String,
    pub payload: Value,
    pub digest: Digest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceStep {
    pub index: u64,
    pub op_code: String,
    pub op_params_canonical: String,
    pub sigma_before_digest: Digest,
    pub sigma_after_digest: Digest,
    pub evidence_ref: String,
}

pub type Trace = Vec<TraceStep>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub hash_algorithm: String,
    pub rd_hash: Digest,
    pub initial_sigma_hash: Digest,
    pub trace_hash: Digest,
    pub evidence_hashes: Vec<Digest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoredValue {
    pub score: f64,
    pub payload: Value,
}
