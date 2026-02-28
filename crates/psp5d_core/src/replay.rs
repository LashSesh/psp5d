use serde::{Deserialize, Serialize};

use crate::{CoreError, EngineTrace, Evidence, ReplayManifest};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstDivergence {
    pub step: u64,
    pub role: String,
    pub operator_name: String,
    pub expected_out_digest: String,
    pub actual_out_digest: String,
}

pub fn first_divergence(expected: &EngineTrace, actual: &EngineTrace) -> Option<FirstDivergence> {
    let n = expected.len().min(actual.len());
    for i in 0..n {
        let e = &expected[i];
        let a = &actual[i];
        if e.out_digest != a.out_digest || e.op_evidence_digest != a.op_evidence_digest {
            return Some(FirstDivergence {
                step: e.step,
                role: e.role.clone(),
                operator_name: e.operator_name.clone(),
                expected_out_digest: e.out_digest.clone(),
                actual_out_digest: a.out_digest.clone(),
            });
        }
    }
    if expected.len() != actual.len() {
        let step = n as u64;
        let (role, operator_name, expected_out_digest, actual_out_digest) =
            if expected.len() > actual.len() {
                let e = &expected[n];
                (
                    e.role.clone(),
                    e.operator_name.clone(),
                    e.out_digest.clone(),
                    "<missing>".to_string(),
                )
            } else {
                let a = &actual[n];
                (
                    a.role.clone(),
                    a.operator_name.clone(),
                    "<missing>".to_string(),
                    a.out_digest.clone(),
                )
            };
        return Some(FirstDivergence {
            step,
            role,
            operator_name,
            expected_out_digest,
            actual_out_digest,
        });
    }
    None
}

pub fn verify_manifest_consistency(
    evidence: &Evidence,
    manifest: &ReplayManifest,
) -> Result<(), CoreError> {
    if evidence.trace_digest != manifest.trace_digest
        || evidence.input_digest != manifest.input_digest
    {
        return Err(CoreError::DeterminismViolation(
            "manifest/evidence digest mismatch".to_string(),
        ));
    }
    Ok(())
}
