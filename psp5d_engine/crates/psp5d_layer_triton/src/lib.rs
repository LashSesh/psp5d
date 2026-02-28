pub mod operators_triton;
pub mod solve_coagula_gate;
pub mod spectral;
pub mod spiral;
pub mod tic_crystallize;

pub use operators_triton::{
    DeterministicEvaluator, TritonGateSolveCoagula, TritonMeasureSigma, TritonStep,
    TritonTICCrystallize,
};
pub use solve_coagula_gate::{evaluate_solve_coagula, TritonGateEvidence};
pub use spectral::{q16_from_f64, q16_to_f64, SpectralEvaluator, SpectralSignature, TritonContext};
pub use spiral::{ExplorationPoint, SpiralState};
pub use tic_crystallize::{crystallize, TicCrystallization};
