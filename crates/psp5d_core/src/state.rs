use serde_json::to_value;

use crate::digest::digest_sha256_jcs;
use crate::errors::CoreError;
use crate::rd::RunDescriptor;
use crate::schema_validate::validate_against_schema;
use crate::types::{Digest, Sigma};
use crate::uir::UirGraph;

pub fn init_sigma_from_uir(
    sigma_id: impl Into<String>,
    graph: &UirGraph,
    rd: &RunDescriptor,
) -> Result<Sigma, CoreError> {
    graph.validate_invariants()?;
    let payload = to_value(graph)?;
    let digest = Digest(digest_sha256_jcs(&payload, rd)?);

    let sigma = Sigma {
        sigma_id: sigma_id.into(),
        payload,
        digest,
    };

    validate_against_schema("state", &to_value(&sigma)?)?;
    Ok(sigma)
}
