use std::{collections::{HashSet, VecDeque}, hash::Hash};

use super::graph::{Graph, GraphIdx};

/// Implementation of a depth-first search
/// algorithm. Generic over the graph type.
pub struct DFS<'g, T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    frontier: Vec<GraphIdx>,
    visited: HashSet<GraphIdx>,
    graph: &'g Graph<T, D, W>,
}

/// Implementation of a breadth-first search
/// algorithm. Generic over the graph type.
pub struct BFS<'g, T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    frontier: VecDeque<GraphIdx>,
    visited: HashSet<GraphIdx>,
    graph: &'g Graph<T, D, W>,
}

impl<'g, T, D, W> DFS<'g, T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    pub fn new(graph: &'g Graph<T, D, W>, start: GraphIdx) -> Self {
        let mut visited = HashSet::new();
        visited.insert(start);
        Self {
            frontier: vec![start],
            visited,
            graph,
        }
    }
}

impl<'g, T, D, W> BFS<'g, T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    pub fn new(graph: &'g Graph<T, D, W>, start: GraphIdx) -> Self {
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

impl<'g, T, D, W> Iterator for DFS<'g, T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    type Item = GraphIdx;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.frontier.pop() {
            for neighbor in self
                .graph
                .edges(next)
                .iter()
                .enumerate()
                .filter(|(_, e)| **e > 0)
                .map(|(i, _)| GraphIdx(i))
            {
                if !self.visited.contains(&neighbor) {
                    self.frontier.push(neighbor);
                    self.visited.insert(neighbor);
                }
            }
            Some(next)
        } else {
            None
        }
    }
}

impl <'g, T, D, W> Iterator for BFS<'g, T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    type Item = GraphIdx;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.frontier.pop_front() {
            for neighbor in self
                .graph
                .edges(next)
                .iter()
                .enumerate()
                .filter(|(_, e)| **e > 0)
                .map(|(i, _)| GraphIdx(i))
            {
                if !self.visited.contains(&neighbor) {
                    self.frontier.push_back(neighbor);
                    self.visited.insert(neighbor);
                }
            }
            Some(next)
        } else {
            None
        }
    }
}
