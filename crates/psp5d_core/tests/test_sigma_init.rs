use std::collections::BTreeMap;

use psp5d_core::{
    init_sigma_from_uir, validate_against_schema, EdgeType, NodeType, RunDescriptor, UirEdge,
    UirGraph, UirNode,
};

#[test]
fn sigma_initialization_is_stable_and_schema_valid() {
    let rd_json = include_str!("../../../examples/rd_min.json");
    let rd: RunDescriptor = serde_json::from_str(rd_json).expect("rd parse");

    let graph = UirGraph {
        uir_version: "1".to_string(),
        nodes: vec![
            UirNode {
                id: "module:sample".to_string(),
                node_type: NodeType::Module,
                props: BTreeMap::new(),
            },
            UirNode {
                id: "symbol:0:hello".to_string(),
                node_type: NodeType::Symbol,
                props: BTreeMap::new(),
            },
        ],
        edges: vec![UirEdge {
            id: "contains:0".to_string(),
            edge_type: EdgeType::Contains,
            src: "module:sample".to_string(),
            dst: "symbol:0:hello".to_string(),
            props: BTreeMap::new(),
        }],
    };

    let s1 = init_sigma_from_uir("sigma0", &graph, &rd).expect("sigma init 1");
    let s2 = init_sigma_from_uir("sigma0", &graph, &rd).expect("sigma init 2");

    assert_eq!(s1.digest, s2.digest);

    let s1_json = serde_json::to_value(&s1).expect("serialize sigma");
    validate_against_schema("state", &s1_json).expect("state schema should validate");
}
