use std::collections::{HashSet, VecDeque};

use super::graph::Graph;

pub struct BFS<'g, V, D, E> {
    graph: &'g Graph<V, D, E>,
    frontier: VecDeque<usize>,
    visited: HashSet<usize>,
}

impl<'g, V, D, E> BFS<'g, V, D, E> {
    pub fn new(graph: &'g Graph<V, D, E>, start: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert(start);
        let mut frontier = VecDeque::new();
        frontier.push_front(start);
        Self {
            frontier,
            visited,
            graph,
        }
    }
}

impl<'g, V, D, E> Iterator for BFS<'g, V, D, E> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.frontier.pop_front() {
            for neighbor in self.graph.edges(next) {
                if !self.visited.contains(&neighbor.next) {
                    self.frontier.push_back(neighbor.next);
                    self.visited.insert(neighbor.next);
                }
            }
            Some(next)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::list_graph::graph::Directed;

    use super::*;

    #[test]
    fn basic_directed() {
        let mut g = Graph::<(), Directed>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(a, b, 1);
        g.add_edge(a, c, 1);

        let mut bfs = BFS::new(&g, a);
        // fully explore
        while bfs.next().is_some() {};
        let exp: HashSet<usize> = HashSet::from_iter(vec![a, b, c]);
        assert_eq!(bfs.visited, exp);

        let mut bfs = BFS::new(&g, b);
        while bfs.next().is_some() {};
        let exp: HashSet<usize> = HashSet::from_iter(vec![b]);
        assert_eq!(bfs.visited, exp);

        let mut bfs = BFS::new(&g, c);
        while bfs.next().is_some() {};
        let exp: HashSet<usize> = HashSet::from_iter(vec![c]);
        assert_eq!(bfs.visited, exp);
    }

    #[test]
    fn basic_undirected() {
        let mut g = Graph::<()>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(a, b, 1);
        g.add_edge(a, c, 1);

        let mut bfs = BFS::new(&g, a);
        while bfs.next().is_some() {};
        let exp: HashSet<usize> = HashSet::from_iter(vec![a, b, c]);
        assert_eq!(bfs.visited, exp);

        let mut bfs = BFS::new(&g, b);
        while bfs.next().is_some() {};
        let exp: HashSet<usize> = HashSet::from_iter(vec![a, b, c]);
        assert_eq!(bfs.visited, exp);

        let mut bfs = BFS::new(&g, c);
        while bfs.next().is_some() {};
        let exp: HashSet<usize> = HashSet::from_iter(vec![a, b, c]);
        assert_eq!(bfs.visited, exp);
    }
}
