use crate::list_graph::graph::Graph;

use std::cmp;

/// Calculates the strongly connected components
/// of a graph using Tarjan's algorithm.
/// Time complexity:
///   adj-matrix graph: O(V^2)
///   adj-list graph: O(E + V)
pub struct Tarjan<'g, V, D, E> {
    n: usize,
    id: usize,
    ids: Vec<isize>,
    low: Vec<usize>,
    on_stack: Vec<bool>,
    stack : Vec<usize>,
    graph: &'g Graph<V, D, E>,
}

impl<'g, V, D, E> Tarjan<'g, V, D, E> {
    pub fn new(graph: &'g Graph<V, D, E>) -> Self {
        let n = graph.len();
        let ids = vec![-1; n];
        let low = vec![0; n];
        let on_stack = vec![false; n];
        Self {
            n,
            id: 0,
            ids,
            low,
            on_stack,
            stack: Vec::new(),
            graph,
        }
    }

    pub fn sccs(mut self) -> Vec<usize> {
        for i in 0..self.n {
            if self.ids[i] == -1 {
                self.dfs(i)
            }
        }

        self.low
    }

    fn dfs(&mut self, at: usize) {
        self.stack.push(at);
        self.on_stack[at] = true;
        self.id += 1;
        self.ids[at] = self.id as isize;
        self.low[at] = self.id;
        for neighbor in self.graph.neighbors(at) {
            if self.ids[*neighbor] == -1 {
                self.dfs(*neighbor);
            }

            if self.on_stack[*neighbor] {
                self.low[at] = cmp::min(self.low[at], self.low[*neighbor]);
            }
        }

        if self.ids[at] == self.low[at] as isize {
            while let Some(node) = self.stack.pop() {
                self.on_stack[node] = false;
                self.low[node] = self.ids[at] as usize;
                if node == at { break; }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list_graph::graph::Directed;

    use std::{iter::FromIterator, collections::HashSet};

    use super::*;

    #[test]
    fn tarjan_base() {
        let mut g = Graph::<(), Directed>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());
        let e = g.add_node(());
        g.add_edge(b, a, 1);
        g.add_edge(a, c, 1);
        g.add_edge(c, b, 1);
        g.add_edge(a, d, 1);
        g.add_edge(d, e, 1);
        let r = Tarjan::new(&g).sccs();
        let num = HashSet::<&usize>::from_iter(r.iter()).len();
        assert_eq!(num, 3);
    }
}
