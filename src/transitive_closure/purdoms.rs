use std::{hash::Hash, collections::{HashSet, VecDeque}};

use crate::mtx_graph::graph::{Graph, transpose, GraphIdx};

use super::mtx::TransitiveClosureMtx;

/// Calculates the transitive closure matrix
/// of a given graph using purdoms algorithm.
/// Time complexity:
///   O(|E| + 𝜇|V|) where 𝜇 = # of strongly connected components.
pub fn purdoms<T, D, W>(graph: &Graph<T, D, W>) -> TransitiveClosureMtx
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    // 1. Find the strongly connected components of theoriginal graph,
    //    replace each component by a single node, and remove the resulting loops.
    // 2. Perform the topological sort of the acyclic graph 𝐺̃  obtained at stage 1.
    // 3. Calculate the transitive closure of 𝐺̃ , moving from nodes with larger
    //    indices to those with smaller ones.
    // 4. Reconstruct the transitive closure of the original graph from the
    //    transitive closure of 𝐺̃ .

    unimplemented!()
}

/// Calculates the strongly connected components
/// of a graph using Tarjan's algorithm.
/// Time complexity:
///   adj-matrix graph: O(V^2)
///   adj-list graph: O(E + V)
///
/// TODO: Make this cleaner OMG gross.
pub struct Tarjan<'g, T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    n: usize,
    id: usize,
    scc_count: usize,
    ids: Vec<isize>,
    low: Vec<usize>,
    on_stack: Vec<bool>,
    stack : Vec<usize>,
    graph: &'g Graph<T, D, W>,
}

impl<'g, T, D, W> Tarjan<'g, T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    pub fn new(graph: &'g Graph<T, D, W>) -> Self {
        let n = graph.nodes();
        let ids = vec![-1; n];
        let low = vec![0; n];
        let on_stack = vec![false; n];
        Self {
            n,
            id: 0,
            scc_count: 0,
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
                self.dfs(i);
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
        for neighbor in self.graph
            .edges(GraphIdx(at))
            .iter()
            .enumerate()
            .filter(|(_, e)| **e > 0)
            .map(|(i, _)| i)
        {
            if self.ids[neighbor] == -1 {
                self.dfs(neighbor);
            }

            if self.on_stack[neighbor] {
                self.low[at] = std::cmp::min(self.low[at], self.low[neighbor]);
            }
        }

        if self.ids[at] == self.low[at] as isize {
            while let Some(node) = self.stack.pop() {
                self.on_stack[node] = false;
                self.low[node] = self.ids[at] as usize;
                if node == at { break; }
            }
            self.scc_count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mtx_graph::graph::Directed;

    use super::*;

    #[test]
    fn tarjan_base() {
        let mut g = Graph::<(), Directed>::default();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());
        let e = g.add_node(());
        g.add_edge(b, a);
        g.add_edge(a, c);
        g.add_edge(c, b);
        g.add_edge(a, d);
        g.add_edge(d, e);
        let r = Tarjan::new(&g).sccs();
        println!("{:?}", r);
    }
}
