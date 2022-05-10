use std::hash::Hash;

use crate::mtx_graph::graph::{Graph, GraphIdx};

use super::mtx::TransitiveClosureMtx;

/// Computes the transitive closure matrix of a given graph
/// by doing a repeated bfs for each vertex.
/// Time complexity:
///   adj-matrix graph: O(V^3)
///   adj-list graph: O(V * (V + E))
pub fn bfs_compute_closure_mtx<T, D, W>(graph: &Graph<T, D, W>) -> TransitiveClosureMtx
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    let mut mtx = TransitiveClosureMtx::from_len(graph.nodes());
    for y in 0..graph.nodes() {
        let idx = GraphIdx(y);
        for x in graph.bfs(idx) {
            mtx[y][x.0] = true;
        }
    }

    mtx
}

#[cfg(test)]
mod tests {
    use crate::mtx_graph::graph::Directed;

    use super::*;

    #[test]
    fn basic_2x2() {
        // A two way 2-node graph
        //
        // (a) <-> (b)
        //
        //   a b
        // a 1 1
        // b 1 1
        //
        let mut g = Graph::<(), Directed>::default();
        let a = g.add_node(());
        let b = g.add_node(());
        g.add_edge(a, b);
        g.add_edge(b, a);
        let mtx = bfs_compute_closure_mtx(&g);
        let exp = TransitiveClosureMtx::from(
            vec![
                vec![true, true],
                vec![true, true],
            ]
        );
        assert_eq!(exp, mtx);
    }

    #[test]
    fn one_way_2x2() {
        // A one way 2-node graph
        //
        // (a) -> (b)
        //
        //   a b
        // a 1 1
        // b 0 1
        //
        let mut g = Graph::<(), Directed>::default();
        let a = g.add_node(());
        let b = g.add_node(());
        g.add_edge(a, b);
        let mtx = bfs_compute_closure_mtx(&g);
        let exp = TransitiveClosureMtx::from(
            vec![
                vec![true, true],
                vec![false, true],
            ]
        );
        assert_eq!(exp, mtx);
    }

    #[test]
    fn bigger() {
        // graph edges:
        //   a b c d
        // a 0 1 1 0
        // b 0 0 1 0
        // c 1 0 0 1
        // d 0 0 0 0
        let mut g = Graph::<(), Directed>::default();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());
        g.add_edge(a, b);
        g.add_edge(a, c);
        g.add_edge(b, c);
        g.add_edge(c, a);
        g.add_edge(c, d);
        let mtx = bfs_compute_closure_mtx(&g);
        let exp = TransitiveClosureMtx::from(
            vec![
                vec![true, true, true, true],
                vec![true, true, true, true],
                vec![true, true, true, true],
                vec![false, false, false, true],
            ]
        );
        assert_eq!(exp, mtx);
    }
}
