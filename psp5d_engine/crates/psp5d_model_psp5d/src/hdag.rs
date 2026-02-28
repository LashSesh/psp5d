use std::collections::{BTreeMap, BTreeSet};

use psp5d_core::{digest_sha256_jcs, CoreError, RunDescriptor, UirGraph};
use serde_json::json;

pub fn topo_rank(
    graph: &UirGraph,
    rd: &RunDescriptor,
) -> Result<BTreeMap<String, usize>, CoreError> {
    graph.validate_invariants()?;
    let mut indeg: BTreeMap<String, usize> =
        graph.nodes.iter().map(|n| (n.id.clone(), 0)).collect();
    let mut out: BTreeMap<String, Vec<String>> = graph
        .nodes
        .iter()
        .map(|n| (n.id.clone(), Vec::new()))
        .collect();

    for e in &graph.edges {
        *indeg.get_mut(&e.dst).expect("known node") += 1;
        out.get_mut(&e.src).expect("known node").push(e.dst.clone());
    }

    let mut rank = BTreeMap::new();
    let mut frontier: BTreeSet<String> = indeg
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(k, _)| k.clone())
        .collect();
    let mut idx = 0usize;

    while !frontier.is_empty() {
        let mut cands: Vec<(String, String)> = frontier
            .iter()
            .map(|id| {
                let d = digest_sha256_jcs(&json!({"id": id}), rd)
                    .unwrap_or_else(|_| "sha256:".to_string());
                (id.clone(), d)
            })
            .collect();
        cands.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
        let chosen = cands[0].0.clone();
        frontier.remove(&chosen);
        rank.insert(chosen.clone(), idx);
        idx += 1;

        for dst in out.get(&chosen).cloned().unwrap_or_default() {
            let d = indeg.get_mut(&dst).expect("exists");
            *d -= 1;
            if *d == 0 {
                frontier.insert(dst);
            }
        }
    }

    if rank.len() != graph.nodes.len() {
        return Err(CoreError::InvariantViolation("HDAG has cycle".to_string()));
    }
    Ok(rank)
}

pub fn enforce_acyclic_active_edges(graph: &UirGraph, rd: &RunDescriptor) -> Result<(), CoreError> {
    let rank = topo_rank(graph, rd)?;
    for e in &graph.edges {
        let u = rank
            .get(&e.src)
            .ok_or_else(|| CoreError::InvariantViolation("missing src rank".to_string()))?;
        let v = rank
            .get(&e.dst)
            .ok_or_else(|| CoreError::InvariantViolation("missing dst rank".to_string()))?;
        if u >= v {
            return Err(CoreError::InvariantViolation(format!(
                "edge '{}' violates Topo(u)<Topo(v)",
                e.id
            )));
        }
    }
    Ok(())
}
