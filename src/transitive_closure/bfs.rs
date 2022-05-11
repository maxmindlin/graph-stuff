use crate::list_graph::graph::Graph;

use super::mtx::TransitiveClosureMtx;

/// Computes the transitive closure matrix of a given graph
/// by doing a repeated bfs for each vertex.
/// Time complexity:
///   adj-matrix graph: O(V^3)
///   adj-list graph: O(V * (V + E))
pub fn bfs_compute_closure_mtx<V, D>(graph: &Graph<V, D>) -> TransitiveClosureMtx
{
    let mut mtx = TransitiveClosureMtx::from_len(graph.len());
    for (y, _) in graph.nodes().iter().enumerate() {
        for x in graph.bfs(y) {
            mtx[y][x] = true;
        }
    }

    mtx
}

#[cfg(test)]
mod tests {
    use crate::list_graph::graph::Directed;

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
        let mut g = Graph::<(), Directed>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        g.add_edge(a, b, 1);
        g.add_edge(b, a, 1);
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
        let mut g = Graph::<(), Directed>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        g.add_edge(a, b, 1);
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
        let mut g = Graph::<(), Directed>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());
        g.add_edge(a, b, 1);
        g.add_edge(a, c, 1);
        g.add_edge(b, c, 1);
        g.add_edge(c, a, 1);
        g.add_edge(c, d, 1);
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
