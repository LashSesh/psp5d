use psp5d_core::{EdgeType, NodeType, RunDescriptor, UirEdge, UirGraph, UirNode};
use psp5d_model_psp5d::hdag::enforce_acyclic_active_edges;
use std::collections::BTreeMap;

#[test]
fn detects_hdag_cycles() {
    let rd: RunDescriptor =
        serde_json::from_str(include_str!("../../../examples/rd_min.json")).expect("rd");
    let mk = |id: &str| UirNode {
        id: id.to_string(),
        node_type: NodeType::Symbol,
        props: BTreeMap::new(),
    };
    let mut g = UirGraph {
        uir_version: "1".to_string(),
        nodes: vec![mk("a"), mk("b")],
        edges: vec![
            UirEdge {
                id: "e1".to_string(),
                edge_type: EdgeType::FlowsTo,
                src: "a".to_string(),
                dst: "b".to_string(),
                props: BTreeMap::new(),
            },
            UirEdge {
                id: "e2".to_string(),
                edge_type: EdgeType::FlowsTo,
                src: "b".to_string(),
                dst: "a".to_string(),
                props: BTreeMap::new(),
            },
        ],
    };
    g.canonicalize_order();
    assert!(enforce_acyclic_active_edges(&g, &rd).is_err());
}
