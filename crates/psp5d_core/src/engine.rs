use serde_json::Value;

use crate::{
    digest_sha256_jcs, CoreError, EngineTrace, OperatorRegistry, Program, RunDescriptor, TraceEntry,
};

pub struct Engine {
    pub registry: OperatorRegistry,
}

impl Engine {
    pub fn run(
        &self,
        initial_state: &Value,
        program: &Program,
        rd: &RunDescriptor,
        steps: usize,
    ) -> Result<(Value, EngineTrace), CoreError> {
        let mut state = initial_state.clone();
        let mut trace = Vec::new();

        for i in 0..steps {
            let role = &program.steps[i % program.steps.len()];
            let op = self
                .registry
                .get(role)
                .ok_or_else(|| CoreError::OperatorNotRegistered(role.as_str().to_string()))?;

            let in_digest = digest_sha256_jcs(&state, rd)?;
            let (next, evidence) = op.execute(&state, rd)?;
            let out_digest = digest_sha256_jcs(&next, rd)?;
            let ev_digest = digest_sha256_jcs(&evidence, rd)?;

            trace.push(TraceEntry {
                step: i as u64,
                role: role.as_str().to_string(),
                operator_name: op.name().to_string(),
                in_digest,
                out_digest,
                op_evidence_digest: ev_digest,
            });
            state = next;
        }

        Ok((state, trace))
    }
}
