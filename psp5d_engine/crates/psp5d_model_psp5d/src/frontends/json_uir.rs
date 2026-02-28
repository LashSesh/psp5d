use psp5d_core::UirGraph;

use crate::FrontendError;

pub fn load_uir_json(input: &str) -> Result<UirGraph, FrontendError> {
    let mut graph: UirGraph = serde_json::from_str(input).map_err(FrontendError::Json)?;
    graph.canonicalize_order();
    graph.validate_invariants().map_err(FrontendError::Core)?;
    Ok(graph)
}
