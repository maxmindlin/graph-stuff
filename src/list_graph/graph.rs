use std::{marker::PhantomData, ops::Index};

use super::{node::Node, edge::Edge, iter::BFS};

#[derive(Debug, Clone, Copy)]
pub enum Directed {}

#[derive(Debug, Clone, Copy)]
pub enum Undirected {}

/// Implementation of an adjacency-list backed
/// Graph.
///
/// TODO handle node deletes
pub struct Graph<V, D = Undirected, E = u32> {
    nodes: Vec<Node<V, E>>,
    pd: PhantomData<D>,
}

impl<V, D, E> Graph<V, D, E> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            pd: PhantomData,
        }
    }

    pub fn add_node(&mut self, val: V) -> usize {
        self.nodes.push(Node::new(val, Vec::new()));
        self.nodes.len() - 1
    }

    pub fn edges(&self, idx: usize) -> &[Edge<E>] {
        &self.nodes[idx].edges
    }

    pub fn nodes(&self) -> &[Node<V, E>] {
        &self.nodes
    }

    pub fn neighbors(&self, idx: usize) -> impl Iterator<Item = &usize> {
        self.edges(idx).iter().map(|e| &e.next)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn bfs(&self, start: usize) -> BFS<V, D, E> {
        BFS::new(&self, start)
    }
}

impl<V, E> Graph<V, Directed, E> {
    pub fn add_edge(&mut self, from: usize, to: usize, weight: E) {
        let node = &mut self.nodes[from];
        node.edges.push(Edge::new(weight, to));
    }
}

impl<V, E> Graph<V, Undirected, E>
where
    E: Copy
{
    pub fn add_edge(&mut self, a: usize, b: usize, weight: E) {
        let node_a = &mut self.nodes[a];
        node_a.edges.push(Edge::new(weight, b));
        let node_b = &mut self.nodes[b];
        node_b.edges.push(Edge::new(weight, a));
    }
}

impl<V, D, E> Index<usize> for Graph<V, D, E> {
    type Output = Node<V, E>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_directed() {
        let mut g = Graph::<(), Directed>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        g.add_edge(a, b, 1);
        assert_eq!(g.edges(a).len(), 1);
        assert_eq!(g.edges(b).len(), 0);
        assert_eq!(g.nodes.len(), 2);
    }

    #[test]
    fn basic_undirected() {
        let mut g = Graph::<()>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        g.add_edge(a, b, 1);
        assert_eq!(g.edges(a).len(), 1);
        assert_eq!(g.edges(b).len(), 1);
    }
}
