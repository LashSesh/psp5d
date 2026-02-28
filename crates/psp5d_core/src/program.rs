use crate::OperatorRole;

#[derive(Debug, Clone)]
pub struct Program {
    pub steps: Vec<OperatorRole>,
}

impl Program {
    pub fn psp_core_default_cycle() -> Self {
        Self {
            steps: vec![
                OperatorRole::Ingest,
                OperatorRole::Canon,
                OperatorRole::Measure,
                OperatorRole::Solve,
                OperatorRole::Morph,
                OperatorRole::Gate,
                OperatorRole::Select,
                OperatorRole::Merge,
                OperatorRole::Emit,
            ],
        }
    }
}
