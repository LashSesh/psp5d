//! PSP5D model pack: deterministic frontends and model algorithms.

use std::error::Error;
use std::fmt::{Display, Formatter};

use psp5d_core::{
    digest_sha256_jcs, init_sigma_from_uir, Engine, EngineOperator, EngineTrace, OperatorRegistry,
    OperatorRole, Program, RunDescriptor,
};
use psp5d_layer_triton::{
    TritonContext, TritonGateSolveCoagula, TritonMeasureSigma, TritonStep, TritonTICCrystallize,
};
use serde_json::{json, Value};

pub mod block;
pub mod cube;
pub mod frontends;
pub mod gates_r5;
pub mod hdag;
pub mod observer_r5;
pub mod projection_pi;
pub mod resonant_kernel;
pub mod time_trimoebius;

#[derive(Debug)]
pub enum FrontendError {
    Json(serde_json::Error),
    Core(psp5d_core::CoreError),
    Message(String),
}

impl Display for FrontendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(e) => write!(f, "json error: {e}"),
            Self::Core(e) => write!(f, "core error: {e}"),
            Self::Message(m) => write!(f, "{m}"),
        }
    }
}
impl Error for FrontendError {}

pub use frontends::generic_text::text_to_uir;
pub use frontends::json_uir::load_uir_json;
#[cfg(feature = "python-ts")]
pub use frontends::python_ts::python_to_uir;

struct JsonPatchOp {
    role: OperatorRole,
    name: &'static str,
}

impl EngineOperator for JsonPatchOp {
    fn name(&self) -> &'static str {
        self.name
    }
    fn role(&self) -> OperatorRole {
        self.role.clone()
    }
    fn execute(
        &self,
        state: &Value,
        rd: &RunDescriptor,
    ) -> Result<(Value, Value), psp5d_core::CoreError> {
        let mut next = state.clone();
        let count = next
            .get("count")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            .saturating_add(1);
        next["count"] = json!(count);
        next["last_role"] = json!(self.role.as_str());
        let evidence = json!({
            "role": self.role.as_str(),
            "name": self.name,
            "rd_digest": digest_sha256_jcs(&serde_json::to_value(rd)?, rd)?,
        });
        Ok((next, evidence))
    }
}

pub fn build_engine() -> Engine {
    let mut reg = OperatorRegistry::new();
    let roles = [
        (OperatorRole::Ingest, "ingest_v1"),
        (OperatorRole::Canon, "canon_v1"),
        (OperatorRole::Measure, "measure_v1"),
        (OperatorRole::Solve, "solve_v1"),
        (OperatorRole::Morph, "morph_v1"),
        (OperatorRole::Gate, "gate_v1"),
        (OperatorRole::Select, "select_v1"),
        (OperatorRole::Merge, "merge_v1"),
        (OperatorRole::Emit, "emit_v1"),
    ];
    for (role, name) in roles {
        reg.register(Box::new(JsonPatchOp { role, name }));
    }
    Engine { registry: reg }
}

pub fn build_triton_engine(rd: &RunDescriptor) -> Engine {
    let mut reg = OperatorRegistry::new();
    // Triton operators for Solve, Measure, Gate, Emit roles:
    reg.register(Box::new(TritonStep {
        ctx: TritonContext {
            rd: rd.clone(),
            spiral_delta_q: 1,
            solve_threshold_q: 500_000,
            coagula_threshold_q: 250_000,
            tic_min_points: 1,
        },
    }));
    reg.register(Box::new(TritonMeasureSigma));
    reg.register(Box::new(TritonGateSolveCoagula {
        ctx: TritonContext {
            rd: rd.clone(),
            spiral_delta_q: 1,
            solve_threshold_q: 500_000,
            coagula_threshold_q: 250_000,
            tic_min_points: 1,
        },
    }));
    reg.register(Box::new(TritonTICCrystallize {
        ctx: TritonContext {
            rd: rd.clone(),
            spiral_delta_q: 1,
            solve_threshold_q: 500_000,
            coagula_threshold_q: 250_000,
            tic_min_points: 1,
        },
    }));
    // JsonPatchOp for the remaining roles: Ingest, Canon, Morph, Select, Merge
    for (role, name) in [
        (OperatorRole::Ingest, "ingest_v1"),
        (OperatorRole::Canon, "canon_v1"),
        (OperatorRole::Morph, "morph_v1"),
        (OperatorRole::Select, "select_v1"),
        (OperatorRole::Merge, "merge_v1"),
    ] {
        reg.register(Box::new(JsonPatchOp { role, name }));
    }
    Engine { registry: reg }
}

pub fn run_10_steps(
    rd: &RunDescriptor,
) -> Result<(Value, psp5d_core::EngineTrace), psp5d_core::CoreError> {
    let engine = build_engine();
    let program = Program::psp_core_default_cycle();
    engine.run(&json!({"count":0_u64}), &program, rd, 10)
}

pub fn run_with_input(
    input_text: &str,
    steps: usize,
    rd: &RunDescriptor,
) -> Result<(Value, EngineTrace), psp5d_core::CoreError> {
    let graph = text_to_uir("input", input_text);
    let sigma = init_sigma_from_uir("run", &graph, rd)?;
    let engine = build_engine();
    let program = Program::psp_core_default_cycle();
    engine.run(&sigma.payload, &program, rd, steps)
}

pub fn run_with_input_triton(
    input_text: &str,
    steps: usize,
    rd: &RunDescriptor,
) -> Result<(Value, EngineTrace), psp5d_core::CoreError> {
    let graph = text_to_uir("input", input_text);
    let sigma = init_sigma_from_uir("run", &graph, rd)?;
    let engine = build_triton_engine(rd);
    let program = Program::psp_core_default_cycle();
    engine.run(&sigma.payload, &program, rd, steps)
}
