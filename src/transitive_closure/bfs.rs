use std::hash::Hash;

use crate::mtx_graph::graph::{Graph, GraphIdx};

use super::mtx::TransitiveClosureMtx;

pub fn bfs_compute_closure_mtx<T, D, W>(graph: &Graph<T, D, W>) -> TransitiveClosureMtx
where
    T: Hash + Eq + Clone,
{
    let mut mtx = TransitiveClosureMtx::from(vec![vec![false; graph.nodes()]; graph.nodes()]);
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
