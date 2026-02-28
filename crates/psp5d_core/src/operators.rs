use serde_json::Value;

use crate::{CoreError, RunDescriptor};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperatorRole {
    Ingest,
    Canon,
    Measure,
    Solve,
    Morph,
    Gate,
    Select,
    Merge,
    Emit,
}

impl OperatorRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ingest => "ingest",
            Self::Canon => "canon",
            Self::Measure => "measure",
            Self::Solve => "solve",
            Self::Morph => "morph",
            Self::Gate => "gate",
            Self::Select => "select",
            Self::Merge => "merge",
            Self::Emit => "emit",
        }
    }
}

pub trait EngineOperator {
    fn name(&self) -> &'static str;
    fn role(&self) -> OperatorRole;
    fn execute(&self, state: &Value, rd: &RunDescriptor) -> Result<(Value, Value), CoreError>;
}
