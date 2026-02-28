use std::collections::BTreeMap;

use psp5d_core::{EdgeType, NodeType, UirEdge, UirGraph, UirNode};

pub fn text_to_uir(module_id: &str, input: &str) -> UirGraph {
    let mut nodes = vec![UirNode {
        id: format!("module:{module_id}"),
        node_type: NodeType::Module,
        props: BTreeMap::new(),
    }];

    let mut edges = Vec::new();
    let mut prev_token_node: Option<String> = None;

    for (idx, token) in input.split_whitespace().enumerate() {
        let token_node_id = format!("symbol:{idx}:{token}");

        let mut props = BTreeMap::new();
        props.insert(
            "token".to_string(),
            serde_json::Value::String(token.to_string()),
        );

        nodes.push(UirNode {
            id: token_node_id.clone(),
            node_type: NodeType::Symbol,
            props,
        });

        edges.push(UirEdge {
            id: format!("contains:{idx}"),
            edge_type: EdgeType::Contains,
            src: format!("module:{module_id}"),
            dst: token_node_id.clone(),
            props: BTreeMap::new(),
        });

        if let Some(prev) = prev_token_node {
            edges.push(UirEdge {
                id: format!("flow:{idx}"),
                edge_type: EdgeType::FlowsTo,
                src: prev,
                dst: token_node_id.clone(),
                props: BTreeMap::new(),
            });
        }

        prev_token_node = Some(token_node_id);
    }

    let mut graph = UirGraph {
        uir_version: "1".to_string(),
        nodes,
        edges,
    };
    graph.canonicalize_order();
    graph
}
