use psp5d_core::{CoreError, EngineOperator, OperatorRole, RunDescriptor};
use serde_json::{json, Value};

use crate::solve_coagula_gate::evaluate_solve_coagula;
use crate::spectral::{SpectralEvaluator, SpectralSignature, TritonContext};
use crate::spiral::{ExplorationPoint, SpiralState};
use crate::tic_crystallize::crystallize;

#[derive(Debug, Clone)]
pub struct DeterministicEvaluator;

impl SpectralEvaluator for DeterministicEvaluator {
    fn evaluate(&mut self, coords_q: [i32; 5], _ctx: &TritonContext) -> SpectralSignature {
        let psi_q = coords_q
            .iter()
            .step_by(2)
            .fold(0_i32, |a, b| a.saturating_add(*b));
        let rho_q = coords_q
            .iter()
            .skip(1)
            .step_by(2)
            .fold(0_i32, |a, b| a.saturating_add(*b));
        let omega_q = coords_q
            .iter()
            .enumerate()
            .fold(0_i32, |a, (i, v)| a.saturating_add(*v * (i as i32 + 1)));
        SpectralSignature {
            psi_q,
            rho_q,
            omega_q,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TritonStep {
    pub ctx: TritonContext,
}

impl EngineOperator for TritonStep {
    fn name(&self) -> &'static str {
        "triton_step"
    }
    fn role(&self) -> OperatorRole {
        OperatorRole::Solve
    }
    fn execute(&self, state: &Value, _rd: &RunDescriptor) -> Result<(Value, Value), CoreError> {
        let mut spiral = state
            .get("triton")
            .and_then(|v| v.get("spiral"))
            .map(|v| serde_json::from_value::<SpiralState>(v.clone()))
            .transpose()?
            .unwrap_or(SpiralState {
                coords_q: [0; 5],
                momentum_q: [0; 5],
                step: 0,
            });

        let mut evaluator = DeterministicEvaluator;
        let point = spiral.next(&mut evaluator, &self.ctx)?;

        let mut next = state.clone();
        next["triton"]["spiral"] = serde_json::to_value(&spiral)?;
        next["triton"]["last_point"] = serde_json::to_value(&point)?;

        Ok((
            next,
            json!({"point_digest":point.id_digest,"step":spiral.step}),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct TritonMeasureSigma;

impl EngineOperator for TritonMeasureSigma {
    fn name(&self) -> &'static str {
        "triton_measure_sigma"
    }
    fn role(&self) -> OperatorRole {
        OperatorRole::Measure
    }
    fn execute(&self, state: &Value, _rd: &RunDescriptor) -> Result<(Value, Value), CoreError> {
        let point: ExplorationPoint =
            serde_json::from_value(state["triton"]["last_point"].clone())?;
        let mut next = state.clone();
        next["triton"]["sigma_measured"] = serde_json::to_value(&point.sigma)?;
        Ok((next, json!({"measured":point.sigma})))
    }
}

#[derive(Debug, Clone)]
pub struct TritonGateSolveCoagula {
    pub ctx: TritonContext,
}

impl EngineOperator for TritonGateSolveCoagula {
    fn name(&self) -> &'static str {
        "triton_gate_solve_coagula"
    }
    fn role(&self) -> OperatorRole {
        OperatorRole::Gate
    }
    fn execute(&self, state: &Value, _rd: &RunDescriptor) -> Result<(Value, Value), CoreError> {
        let sigma: SpectralSignature =
            serde_json::from_value(state["triton"]["sigma_measured"].clone())?;
        let gate = evaluate_solve_coagula(&sigma, &self.ctx)?;
        let mut next = state.clone();
        next["triton"]["gate"] = serde_json::to_value(&gate)?;
        next["triton"]["gate_pass"] = json!(gate.outcome);
        Ok((next, serde_json::to_value(gate)?))
    }
}

#[derive(Debug, Clone)]
pub struct TritonTICCrystallize {
    pub ctx: TritonContext,
}

impl EngineOperator for TritonTICCrystallize {
    fn name(&self) -> &'static str {
        "triton_tic_crystallize"
    }
    fn role(&self) -> OperatorRole {
        OperatorRole::Emit
    }
    fn execute(&self, state: &Value, _rd: &RunDescriptor) -> Result<(Value, Value), CoreError> {
        let gate_pass = state["triton"]["gate_pass"].as_bool().unwrap_or(false);
        if !gate_pass {
            return Ok((
                state.clone(),
                json!({"emitted":false,"reason":"gate_failed"}),
            ));
        }

        let mut points: Vec<ExplorationPoint> = state
            .get("triton")
            .and_then(|v| v.get("points"))
            .map(|v| serde_json::from_value(v.clone()))
            .transpose()?
            .unwrap_or_default();

        if let Some(last) = state
            .get("triton")
            .and_then(|t| t.get("last_point"))
            .cloned()
        {
            points.push(serde_json::from_value(last)?);
        }

        let crystal = crystallize(&points, &self.ctx)?;
        let mut next = state.clone();
        next["triton"]["points"] = serde_json::to_value(points)?;
        next["triton"]["tic"] = serde_json::to_value(&crystal)?;

        Ok((
            next,
            json!({"emitted":true,"crystal_digest":crystal.crystal_digest}),
        ))
    }
}
