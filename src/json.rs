//! Utilities for serializing graphs to render using Sigma.js

use num_bigint::BigUint;
use num_traits::{Zero, One};
use serde_json::Value;
use serde_json::map::Map;

use crate::hypergraph::DirectedGraph;

pub fn to_sigma_json(graph: DirectedGraph) -> Value {
    // The top-level JSON object.
    // Should contain "nodes" and "edges"
    let mut root: Map<String, Value> = Map::new();

    // The nodes list.
    let mut nodes: Vec<Value> = Vec::new();

    // Find highest node number
    let node_top: &BigUint = graph.edges()
        .iter()
        .fold(&Zero::zero(), |acc: &BigUint, (lhs, rhs)| {
            match (lhs > acc, rhs > acc, lhs > rhs) {
                (false, false, _) => acc,
                (true, _, true) => lhs,
                (_, true, false) => rhs,
                (_, _, _) => panic!("ints don't work like that idiot"),
            }
        });

    // Add nodes to array
    let mut i: BigUint = Zero::zero();
    while i <= *node_top {
        nodes.push(new_node(&i, None));
        i = i + One::one();
    }

    root.insert("nodes".to_string(), Value::Array(nodes));

    let mut edges: Vec<Value> = Vec::new();

    Value::Object(root)
}

/// Make a new node entry given an ID and a label
fn new_node(id: &BigUint, label: Option<&str>) -> Value {
    let mut node: Map<String, Value> = Map::new();

    node.insert("id".to_string(), Value::String(format!("n{}", id.to_str_radix(16))));

    if let Some(label) = label {
        node.insert("label".to_string(), Value::String(label.to_string()));
    }

    node.insert("size".to_string(), Value::String(format!("{}", 1u32)));

    Value::Object(node)
}

/// Make a new edge entry given a source and a destination
fn new_edge(src: &BigUint, dst: &BigUint) -> Value {
    let mut edge: Map<String, Value> = Map::new();

    edge.insert("id".to_string(), Value::String(format!("e{}..{}", src.to_str_radix(16), dst.to_str_radix(16))));
    edge.insert("source".to_string(), Value::String(format!("n{}", src.to_str_radix(16))));
    edge.insert("target".to_string(), Value::String(format!("n{}", dst.to_str_radix(16))));

    Value::Object(edge)
}
