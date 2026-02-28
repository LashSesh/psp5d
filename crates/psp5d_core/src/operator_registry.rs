use std::collections::BTreeMap;

use crate::{EngineOperator, OperatorRole};

pub struct OperatorRegistry {
    by_role: BTreeMap<OperatorRole, Box<dyn EngineOperator>>,
}

impl OperatorRegistry {
    pub fn new() -> Self {
        Self {
            by_role: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, op: Box<dyn EngineOperator>) {
        self.by_role.insert(op.role(), op);
    }

    pub fn get(&self, role: &OperatorRole) -> Option<&dyn EngineOperator> {
        self.by_role.get(role).map(Box::as_ref)
    }
}

impl Default for OperatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
