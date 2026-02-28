use psp5d_core::{digest_sha256_jcs, sort_by_score_then_digest, CoreError, ScoredValue};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::spectral::{q16_to_f64, TritonContext};
use crate::spiral::ExplorationPoint;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicCrystallization {
    pub selected_point_digests: Vec<String>,
    pub crystal_digest: String,
}

pub fn crystallize(
    points: &[ExplorationPoint],
    ctx: &TritonContext,
) -> Result<TicCrystallization, CoreError> {
    let mut scored: Vec<ScoredValue> = points
        .iter()
        .map(|p| {
            let score =
                q16_to_f64(p.sigma.psi_q) + q16_to_f64(p.sigma.rho_q) + q16_to_f64(p.sigma.omega_q);
            ScoredValue {
                score,
                payload: json!({"id_digest":p.id_digest}),
            }
        })
        .collect();

    sort_by_score_then_digest(&mut scored, &ctx.rd)?;
    let take_n = scored.len().min(ctx.tic_min_points.max(1));
    let selected_point_digests: Vec<String> = scored
        .iter()
        .take(take_n)
        .map(|s| {
            s.payload["id_digest"]
                .as_str()
                .unwrap_or_default()
                .to_string()
        })
        .collect();

    let crystal_digest = digest_sha256_jcs(&json!({"selected":&selected_point_digests}), &ctx.rd)?;
    Ok(TicCrystallization {
        selected_point_digests,
        crystal_digest,
    })
}
