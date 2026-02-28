#[cfg(feature = "python-ts")]
use std::collections::BTreeMap;

#[cfg(feature = "python-ts")]
use psp5d_core::{EdgeType, NodeType, UirEdge, UirGraph, UirNode};

#[cfg(feature = "python-ts")]
pub fn python_to_uir(module_id: &str, input: &str) -> Result<UirGraph, String> {
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_python::language();
    parser
        .set_language(language)
        .map_err(|e| format!("language setup failed: {e}"))?;
    let tree = parser
        .parse(input, None)
        .ok_or_else(|| "python parse failed".to_string())?;

    let mut nodes = vec![UirNode {
        id: format!("module:{module_id}"),
        node_type: NodeType::Module,
        props: BTreeMap::new(),
    }];

    let root = tree.root_node();
    nodes.push(UirNode {
        id: "root:0".to_string(),
        node_type: NodeType::Block,
        props: BTreeMap::new(),
    });

    let edges = vec![UirEdge {
        id: "contains:root".to_string(),
        edge_type: EdgeType::Contains,
        src: format!("module:{module_id}"),
        dst: "root:0".to_string(),
        props: {
            let mut m = BTreeMap::new();
            m.insert(
                "kind".to_string(),
                serde_json::Value::String(root.kind().to_string()),
            );
            m
        },
    }];

    let mut graph = UirGraph {
        uir_version: "1".to_string(),
        nodes,
        edges,
    };
    graph.canonicalize_order();
    Ok(graph)
}
