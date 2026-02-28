use psp5d_core::{CoreError, RunDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpectralSignature {
    pub psi_q: i32,
    pub rho_q: i32,
    pub omega_q: i32,
}

#[derive(Debug, Clone)]
pub struct TritonContext {
    pub rd: RunDescriptor,
    pub spiral_delta_q: i32,
    pub solve_threshold_q: i32,
    pub coagula_threshold_q: i32,
    pub tic_min_points: usize,
}

pub trait SpectralEvaluator {
    fn evaluate(&mut self, coords_q: [i32; 5], ctx: &TritonContext) -> SpectralSignature;
}

pub fn q16_from_f64(value: f64) -> Result<i32, CoreError> {
    let scaled = value * 65536.0;
    let quant = round_half_even(scaled);
    i32::try_from(quant).map_err(|_| CoreError::DeterminismViolation("q16 overflow".to_string()))
}

pub fn q16_to_f64(value_q: i32) -> f64 {
    f64::from(value_q) / 65536.0
}

fn round_half_even(value: f64) -> i64 {
    let floor = value.floor();
    let frac = value - floor;
    if frac < 0.5 {
        floor as i64
    } else if frac > 0.5 {
        (floor + 1.0) as i64
    } else if (floor as i64) % 2 == 0 {
        floor as i64
    } else {
        (floor + 1.0) as i64
    }
}
