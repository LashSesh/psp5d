use psp5d_core::{digest_sha256_jcs, CoreError, RunDescriptor};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub k1: u64,
    pub k2: u64,
    pub nc_tag: String,
    pub prev_digest: String,
    pub rd_digest: String,
    pub pi_id: String,
    pub pi_params_digest: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockPayload {
    pub uir_digest: String,
    pub astar_digest: String,
    pub metrics: Value,
    pub gate_report: Value,
    pub artifact_digests: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub payload: BlockPayload,
    pub block_digest: String,
}

pub fn make_block(
    header: BlockHeader,
    payload: BlockPayload,
    rd: &RunDescriptor,
) -> Result<Block, CoreError> {
    let canonical = serde_json::json!({"header": &header, "payload": &payload});
    let block_digest = digest_sha256_jcs(&canonical, rd)?;
    Ok(Block {
        header,
        payload,
        block_digest,
    })
}
