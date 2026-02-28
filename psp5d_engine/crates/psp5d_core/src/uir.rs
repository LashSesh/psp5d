use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::CoreError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Symbol,
    Func,
    Class,
    Module,
    Literal,
    Type,
    Block,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    Contains,
    Calls,
    Reads,
    Writes,
    Defines,
    Imports,
    DependsOn,
    FlowsTo,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirNode {
    pub id: String,
    pub node_type: NodeType,
    #[serde(default)]
    pub props: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirEdge {
    pub id: String,
    pub edge_type: EdgeType,
    pub src: String,
    pub dst: String,
    #[serde(default)]
    pub props: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UirGraph {
    pub uir_version: String,
    pub nodes: Vec<UirNode>,
    pub edges: Vec<UirEdge>,
}

impl UirGraph {
    pub fn canonicalize_order(&mut self) {
        self.nodes.sort_by(|a, b| a.id.cmp(&b.id));
        self.edges.sort_by(|a, b| a.id.cmp(&b.id));
    }

    pub fn validate_invariants(&self) -> Result<(), CoreError> {
        let mut node_ids = BTreeSet::new();
        for node in &self.nodes {
            if !node_ids.insert(node.id.clone()) {
                return Err(CoreError::InvariantViolation(format!(
                    "duplicate node id '{}'",
                    node.id
                )));
            }
        }

        let mut edge_ids = BTreeSet::new();
        for edge in &self.edges {
            if !edge_ids.insert(edge.id.clone()) {
                return Err(CoreError::InvariantViolation(format!(
                    "duplicate edge id '{}'",
                    edge.id
                )));
            }
            if !node_ids.contains(&edge.src) || !node_ids.contains(&edge.dst) {
                return Err(CoreError::InvariantViolation(format!(
                    "edge '{}' references missing node(s)",
                    edge.id
                )));
            }
        }

        Ok(())
    }
}
