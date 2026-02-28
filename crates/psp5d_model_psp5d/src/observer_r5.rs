use psp5d_core::{digest_sha256_jcs, EngineTrace, RunDescriptor};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct R5View {
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,
}

pub fn observe_r5(trace: &EngineTrace, rd: &RunDescriptor, n: usize) -> R5View {
    let start = trace.len().saturating_sub(n);
    let window = &trace[start..];
    let dig = digest_sha256_jcs(&json!(window), rd).unwrap_or_else(|_| "sha256:0".to_string());
    let bytes = dig.as_bytes();
    let s1 = bytes.iter().step_by(3).fold(0u64, |a, b| a + *b as u64);
    let s2 = bytes
        .iter()
        .skip(1)
        .step_by(3)
        .fold(0u64, |a, b| a + *b as u64);
    let s3 = bytes
        .iter()
        .skip(2)
        .step_by(3)
        .fold(0u64, |a, b| a + *b as u64);
    R5View {
        psi: (s1 % 10_000) as f64 / 10_000.0,
        rho: (s2 % 10_000) as f64 / 10_000.0,
        omega: (s3 % 10_000) as f64 / 10_000.0,
    }
}
