use psp5d_core::digest_sha256_jcs;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::spectral::{q16_to_f64, SpectralSignature, TritonContext};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TritonGateEvidence {
    pub predicate_id: String,
    pub predicate_ver: String,
    pub solve_threshold_q: i32,
    pub coagula_threshold_q: i32,
    pub measured_psi_q: i32,
    pub measured_rho_q: i32,
    pub measured_omega_q: i32,
    pub solve_pass: bool,
    pub coagula_pass: bool,
    pub outcome: bool,
    pub evidence_digest: String,
}

pub fn evaluate_solve_coagula(
    sigma: &SpectralSignature,
    ctx: &TritonContext,
) -> Result<TritonGateEvidence, psp5d_core::CoreError> {
    let magnitude_q = sigma
        .psi_q
        .unsigned_abs()
        .saturating_add(sigma.rho_q.unsigned_abs())
        .saturating_add(sigma.omega_q.unsigned_abs());
    let solve_pass = i64::from(magnitude_q) <= i64::from(ctx.solve_threshold_q.max(0) as u32);

    let omega_abs = sigma.omega_q.unsigned_abs();
    let coagula_pass = i64::from(omega_abs) <= i64::from(ctx.coagula_threshold_q.max(0) as u32);

    let outcome = solve_pass && coagula_pass;
    let val = json!({
        "predicate_id":"triton.solve_coagula",
        "predicate_ver":"1",
        "solve_threshold_q":ctx.solve_threshold_q,
        "coagula_threshold_q":ctx.coagula_threshold_q,
        "measured":{
            "psi":q16_to_f64(sigma.psi_q),
            "rho":q16_to_f64(sigma.rho_q),
            "omega":q16_to_f64(sigma.omega_q)
        },
        "outcome":outcome
    });
    let evidence_digest = digest_sha256_jcs(&val, &ctx.rd)?;

    Ok(TritonGateEvidence {
        predicate_id: "triton.solve_coagula".to_string(),
        predicate_ver: "1".to_string(),
        solve_threshold_q: ctx.solve_threshold_q,
        coagula_threshold_q: ctx.coagula_threshold_q,
        measured_psi_q: sigma.psi_q,
        measured_rho_q: sigma.rho_q,
        measured_omega_q: sigma.omega_q,
        solve_pass,
        coagula_pass,
        outcome,
        evidence_digest,
    })
}
