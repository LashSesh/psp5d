use psp5d_core::{digest_sha256_jcs, CoreError};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::spectral::{SpectralEvaluator, SpectralSignature, TritonContext};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorationPoint {
    pub coords_q: [i32; 5],
    pub sigma: SpectralSignature,
    pub id_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpiralState {
    pub coords_q: [i32; 5],
    pub momentum_q: [i32; 5],
    pub step: u64,
}

impl SpiralState {
    pub fn next<E: SpectralEvaluator>(
        &mut self,
        evaluator: &mut E,
        ctx: &TritonContext,
    ) -> Result<ExplorationPoint, CoreError> {
        for i in 0..5 {
            let twist = i32::try_from((self.step + i as u64) % 3)
                .map_err(|_| CoreError::DeterminismViolation("twist conversion".to_string()))?
                - 1;
            self.momentum_q[i] = self.momentum_q[i].saturating_add(twist * ctx.spiral_delta_q);
            self.coords_q[i] = self.coords_q[i].saturating_add(self.momentum_q[i]);
        }
        self.step = self.step.saturating_add(1);

        let sigma = evaluator.evaluate(self.coords_q, ctx);
        let payload = json!({"coords_q":self.coords_q,"sigma":&sigma,"step":self.step});
        let id_digest = digest_sha256_jcs(&payload, &ctx.rd)?;
        Ok(ExplorationPoint {
            coords_q: self.coords_q,
            sigma,
            id_digest,
        })
    }
}
