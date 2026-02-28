use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateConfig {
    pub eps_diam: f64,
    pub eps_pi: f64,
    pub theta_consensus: f64,
    pub weights: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateReport {
    pub tic_diam: bool,
    pub tic_pi: bool,
    pub score: f64,
    pub passed: bool,
}

pub fn consensus_gate(cfg: &GateConfig, diam_astar: f64, delta_pi: f64) -> GateReport {
    let tic_diam = diam_astar <= cfg.eps_diam;
    let tic_pi = delta_pi <= cfg.eps_pi;
    let w0 = *cfg.weights.first().unwrap_or(&1.0);
    let w1 = *cfg.weights.get(1).unwrap_or(&1.0);
    let score = w0 * f64::from(tic_diam as u8) + w1 * f64::from(tic_pi as u8);
    let passed = score >= cfg.theta_consensus;
    GateReport {
        tic_diam,
        tic_pi,
        score,
        passed,
    }
}
