//! Utilities for serializing graphs to render using Sigma.js

use num::bigint::BigUint;
use num::{One, Zero};
use serde_json::map::Map;
use serde_json::Value;

use crate::hypergraph::DirectedGraph;

pub fn to_sigma_json(graph: DirectedGraph) -> Value {
    // The top-level JSON object.
    // Should contain "nodes" and "edges"
    let mut root: Map<String, Value> = Map::new();

    // The nodes list.
    let mut nodes: Vec<Value> = Vec::new();

    // Find highest node number
    let mut node_top: BigUint = BigUint::new(vec![0]);
    for (lhs, rhs) in graph.edges().iter() {
        if node_top < *lhs {
            node_top = lhs.clone();
        }
        if node_top < *rhs {
            node_top = rhs.clone();
        }
    }

    // Add nodes to array
    let mut i = BigUint::new(vec![0]);
    while i <= node_top {
        nodes.push(new_node(&i, None));
        i = i + BigUint::new(vec![1]);
    }

    root.insert("nodes".to_string(), Value::Array(nodes));

    let mut edges: Vec<Value> = Vec::new();

    for (src, dst) in graph.edges().iter() {
        edges.push(new_edge(src, dst));
    }

    root.insert("edges".to_string(), Value::Array(edges));

    Value::Object(root)
}

/// Make a new node entry given an ID and a label
fn new_node(id: &BigUint, label: Option<&str>) -> Value {
    let mut node: Map<String, Value> = Map::new();

    node.insert(
        "id".to_string(),
        Value::String(format!("n{}", id.to_str_radix(16))),
    );

    if let Some(label) = label {
        node.insert("label".to_string(), Value::String(label.to_string()));
    }

    node.insert("size".to_string(), Value::String(format!("{}", 1u32)));

    Value::Object(node)
}

/// Make a new edge entry given a source and a destination
fn new_edge(src: &BigUint, dst: &BigUint) -> Value {
    let mut edge: Map<String, Value> = Map::new();

    edge.insert(
        "id".to_string(),
        Value::String(format!(
            "e{}..{}",
            src.to_str_radix(16),
            dst.to_str_radix(16)
        )),
    );
    edge.insert(
        "source".to_string(),
        Value::String(format!("n{}", src.to_str_radix(16))),
    );
    edge.insert(
        "target".to_string(),
        Value::String(format!("n{}", dst.to_str_radix(16))),
    );

    Value::Object(edge)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hypergraph::{DirectedGraph, DirectedHyperGraph};

    #[test]
    fn smoke_json() {
        let hg = DirectedHyperGraph::ternary_self_loop();
        let unrolled = hg.unroll_to_graph();

        let json = serde_json::to_string_pretty(&to_sigma_json(unrolled)).expect("wtf");
        println!("{}", json);
    }
}
