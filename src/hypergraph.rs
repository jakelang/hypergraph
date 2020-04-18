//! Hypergraph data structure and utilities

use num_bigint::BigUint;
use num_traits::Zero;
use rayon::prelude::*;

/// A list of relations describing a hypergraph.
/// Indices are considered nodes.
/// Unlike a graph, in which edges have only two extremities, hypergraphs have hyperedges
/// which can connect any number of vertices.
pub struct DirectedHyperGraph(Vec<Vec<BigUint>>);

/// A normal directed graph. This is used when outputting to JSON, so that Sigma.js can render it
/// (Sigma.js's data model does not accomodate hyperedges, so we instead convert them to edges for
/// visualization)
pub struct DirectedGraph(Vec<(BigUint, BigUint)>);

impl DirectedHyperGraph {
    /// Creates the simplest ternary hypergraph, one with a single relation.
    pub fn ternary_self_loop() -> Self {
        let relations = vec![vec![Zero::zero(), Zero::zero(), Zero::zero()]];

        DirectedHyperGraph(relations)
    }

    /// Adds a relation of any number of vertices.
    pub fn add_relation(&mut self, rel: Vec<BigUint>) {
        self.0.push(rel);
    }

    /// Converts any n-ary relations to normal edges
    pub fn unroll_to_graph(self) -> DirectedGraph {
        let mut bin_rels: Vec<(BigUint, BigUint)> = Vec::new();

        for rel in self.0.iter() {
            let mut new_rels: Vec<(BigUint, BigUint)> = Vec::new(); 
            
            let mut rel_iter = rel.iter().peekable();
            while let Some(vertex) = rel_iter.next() {
                if let Some(vertex_1) = rel_iter.peek() {
                    new_rels.push((vertex.clone(), (*vertex_1).clone()));
                }
            }

            bin_rels.append(&mut new_rels);
        }

        DirectedGraph(bin_rels)
    }
}

impl DirectedGraph {
    pub fn num_edges(&self) -> usize {
        self.0.len()
    }

    pub fn edges(&self) -> &Vec<(BigUint, BigUint)> {
        &self.0
    }
}

impl Default for DirectedHyperGraph {
    fn default() -> Self {
        DirectedHyperGraph(Vec::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_unroll() {
        let hg = DirectedHyperGraph::ternary_self_loop();

        let unrolled = hg.unroll_to_graph();

        assert_eq!(2, unrolled.num_edges());

        let edges = unrolled.edges();
        assert_eq!(edges[0], (Zero::zero(), Zero::zero()));
        assert_eq!(edges[1], (Zero::zero(), Zero::zero()));
    }
}
