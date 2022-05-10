use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Mul},
};

use super::iter::{DFS, BFS};

#[derive(Clone)]
pub enum Weighted {}

#[derive(Clone)]
pub enum Unweighted {}

#[derive(Clone)]
pub enum Directed {}

#[derive(Clone)]
pub enum Undirected {}

/// Implementation of adjacency matrix backed Graph structure.
/// Can be any combination of Directed/Undirected and Weighted/Unweighted.
/// Defaults to Undirected and Unweighted. Markers Weighted, Unweighted,
/// Directed and Undirected can be used to specify.
///
/// # Examples
///
/// ```
/// use graph_stuff::mtx_graph::graph::*;
///
/// // An undirected, unweighted graph whose node
/// // values are unit type.
/// let _ = Graph::<()>::default();
///
/// // An undirected, unweighted graph whose node
/// // values are u32.
/// let _ = Graph::<u32>::default();
///
/// // A directed, unweighted graph whose node
/// // values are u32.
/// let _ = Graph::<u32, Directed>::default();
///
/// // A directed, weighted graph whose node
/// // values are u32.
/// let _ = Graph::<u32, Directed, Weighted>::default();
/// ```
#[derive(Clone)]
pub struct Graph<T, D = Undirected, W = Unweighted>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    n: usize,
    mtx: Vec<usize>,
    node_map: HashMap<T, GraphIdx>,
    vals: Vec<T>,
    pw: PhantomData<W>,
    pd: PhantomData<D>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GraphIdx(pub(crate) usize);

impl<T: Hash + Eq + Clone> Graph<T> {
    pub fn add_edge(&mut self, x: GraphIdx, y: GraphIdx) {
        self.add_edge_weight(x, y, 1);
        self.add_edge_weight(y, x, 1);
    }
}

impl<T: Hash + Eq + Clone> Graph<T, Directed, Unweighted> {
    pub fn add_edge(&mut self, x: GraphIdx, y: GraphIdx) {
        self.add_edge_weight(x, y, 1);
    }
}

impl<T: Hash + Eq + Clone> Graph<T, Undirected, Weighted> {
    pub fn add_edge(&mut self, x: GraphIdx, y: GraphIdx, weight: usize) {
        self.add_edge_weight(x, y, weight);
        self.add_edge_weight(y, x, weight);
    }
}

impl<T: Hash + Eq + Clone> Graph<T, Directed, Weighted> {
    pub fn add_edge(&mut self, x: GraphIdx, y: GraphIdx, weight: usize) {
        self.add_edge_weight(x, y, weight);
    }
}

impl<T, D> Graph<T, D, Weighted>
where
    T: Hash + Eq + Clone,
    D: Clone,
{
    pub fn edge_weight(&self, x: GraphIdx, y: GraphIdx) -> usize {
        let idx = calc_2d_to_1d(x, y, self.n);
        self.mtx[idx]
    }
}

impl<T, D, W> Graph<T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    /// Sets the edge weight between two nodes to the given
    /// weight.
    fn add_edge_weight(&mut self, x: GraphIdx, y: GraphIdx, weight: usize) {
        let idx = calc_2d_to_1d(x, y, self.n);
        self.mtx[idx] = weight;
    }

    /// Adds a node to the node set.
    pub fn add_node(&mut self, val: T) -> GraphIdx {
        self.n += 1;
        let ncells = self.n.pow(2) - (self.n - 1).pow(2);
        for _ in 0..ncells {
            self.mtx.push(0);
        }

        self.vals.push(val.clone());

        // return the idx, not the count
        let idx = GraphIdx(self.n - 1);
        self.node_map.insert(val, idx);
        idx
    }

    /// Returns how many nodes are currently
    /// in the graph.
    pub fn nodes(&self) -> usize {
        self.n
    }

    /// Returns an array of all the edge weights for
    /// a given node.
    pub fn edges(&self, idx: GraphIdx) -> &[usize] {
        let r = idx * self.n;
        &self.mtx[r..(r + self.n)]
    }

    pub fn get_node(&self, idx: GraphIdx) -> &T {
        &self.vals[idx.0]
    }

    pub fn get_idx(&self, val: &T) -> Option<GraphIdx> {
        self.node_map.get(val).cloned()
    }

    pub fn has_edge(&self, x: GraphIdx, y: GraphIdx) -> bool {
        let idx = calc_2d_to_1d(x, y, self.n);
        self.mtx[idx] > 0
    }

    pub fn dfs(&self, start: GraphIdx) -> DFS<T, D, W> {
        DFS::new(&self, start)
    }

    pub fn bfs(&self, start: GraphIdx) -> BFS<T, D, W> {
        BFS::new(&self, start)
    }

    /// Implementation of Dijkstra's path finding algorithm.
    ///
    /// The max_cost determines how far out to discover nodes (computed via edge weights). If no max is
    /// provided, then it will search all traversable nodes.
    ///
    /// If a target is provided, the search algorithm with halt when the target node is found.
    ///
    /// Returns a linked list of nodes and how they were traversed to in the form of a hashmap.
    pub fn dijkstra(
        &self,
        start: GraphIdx,
        max_cost: Option<i32>,
        target: Option<GraphIdx>,
    ) -> HashMap<GraphIdx, Option<GraphIdx>> {
        let mut frontier = BinaryHeap::<QueueNode>::new();
        frontier.push(QueueNode::new(start, 0));

        let mut came_from = HashMap::<GraphIdx, Option<GraphIdx>>::new();
        came_from.insert(start, None);

        let mut cost_so_far = HashMap::<GraphIdx, i32>::new();
        cost_so_far.insert(start, 0);

        while let Some(current) = frontier.pop() {
            if let Some(t) = target {
                if t == current.idx {
                    break;
                }
            }

            let edges = self.edges(current.idx);
            for (neighbor, edge) in edges
                .iter()
                .enumerate()
                .filter(|(_, e)| **e > 0)
                .map(|(i, e)| (GraphIdx(i), *e as i32))
            {
                let new_cost = cost_so_far.get(&current.idx).unwrap_or(&0) + edge;
                let next_cost = *cost_so_far.get(&neighbor).unwrap_or(&0);
                if (!cost_so_far.contains_key(&neighbor) || new_cost < next_cost)
                    && max_cost.map_or(true, |max| new_cost <= max)
                {
                    cost_so_far.insert(neighbor, new_cost);
                    came_from.insert(neighbor, Some(current.idx));
                    frontier.push(QueueNode::new(neighbor, new_cost));
                }
            }
        }

        came_from
    }

    /// Calculates a path from a starting node to a target node, if there is one. Internally
    /// utilizes Dijkstra's algorithm for path finding.
    pub fn path_to(&self, start: GraphIdx, target: GraphIdx) -> Option<Vec<GraphIdx>> {
        let came_from = self.dijkstra(start, None, Some(target));
        let mut curr = target;
        let mut path = Vec::<GraphIdx>::new();
        while curr != start {
            path.push(curr);
            let next = *came_from.get(&curr)?;
            curr = next?;
        }

        Some(path)
    }
}

/// Mutates a graph in place to its transpose.
pub fn transpose<T, D, W>(graph: &mut Graph<T, D, W>)
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    let len = graph.nodes();
    for y in 0..len {
        for x in y..len {
            let idxy = GraphIdx(y);
            let idxx = GraphIdx(x);
            let a = calc_2d_to_1d(idxx, idxy, len);
            let b = calc_2d_to_1d(idxy, idxx, len);
            let tmp = graph.mtx[a];
            graph.mtx[a] = graph.mtx[b];
            graph.mtx[b] = tmp;
        }
    }
}

impl<T, D, W> Default for Graph<T, D, W>
where
    T: Hash + Eq + Clone,
    D: Clone,
    W: Clone,
{
    fn default() -> Self {
        Self {
            n: 0,
            mtx: Vec::new(),
            vals: Vec::new(),
            node_map: HashMap::new(),
            pw: PhantomData,
            pd: PhantomData,
        }
    }
}

#[derive(PartialEq, Eq)]
struct QueueNode {
    idx: GraphIdx,
    weight: i32,
}

impl QueueNode {
    fn new(idx: GraphIdx, weight: i32) -> Self {
        Self { idx, weight }
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn calc_2d_to_1d(x: GraphIdx, y: GraphIdx, len: usize) -> usize {
    // [0 0 1 0]
    // [0 0 0 0]
    // [1 0 0 0]
    // [0 0 0 0]
    //
    // =>
    //
    //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
    // [0 0 1 0 0 0 0 0 1 0 0 0 0 0 0 0]
    //
    // n = 4
    // x = 0
    // y = 2
    //
    // 0 * 4 + 2 = 2
    // 2 * 4 + 0 = 8

    x * len + y
}

impl Mul<usize> for GraphIdx {
    type Output = usize;

    fn mul(self, rhs: usize) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<GraphIdx> for usize {
    type Output = usize;

    fn mul(self, rhs: GraphIdx) -> Self::Output {
        self * rhs.0
    }
}

impl Add<usize> for GraphIdx {
    type Output = usize;

    fn add(self, rhs: usize) -> Self::Output {
        self.0 + rhs
    }
}

impl Add<GraphIdx> for usize {
    type Output = usize;

    fn add(self, rhs: GraphIdx) -> Self::Output {
        self + rhs.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let g = Graph::<()>::default();
        assert_eq!(g.nodes(), 0);
        assert!(g.mtx.is_empty());
    }

    #[test]
    fn singleton() {
        let mut g = Graph::<()>::default();
        g.add_node(());
        assert_eq!(g.nodes(), 1);
        assert_eq!(g.mtx.len(), 1);
    }

    #[test]
    fn multi() {
        let mut g = Graph::<()>::default();
        g.add_node(());
        g.add_node(());
        g.add_node(());
        assert_eq!(g.nodes(), 3);
        assert_eq!(g.mtx.len(), 9);
    }

    #[test]
    fn has_edge() {
        let mut g = Graph::<()>::default();
        let i1 = g.add_node(());
        g.add_node(());
        let i3 = g.add_node(());
        g.add_edge(i1, i3);
        assert!(g.has_edge(i1, i3));
        assert!(g.has_edge(i3, i1));
    }

    #[test]
    fn get_val() {
        let mut g = Graph::<&str>::default();
        let i = g.add_node("a");
        let v = g.get_node(i);
        assert_eq!(*v, "a");
    }

    #[test]
    fn single_direction() {
        let mut g = Graph::<(), Directed>::default();
        let a = g.add_node(());
        let b = g.add_node(());
        g.add_edge(a, b);
        assert!(g.has_edge(a, b));
        assert!(!g.has_edge(b, a));
    }

    #[test]
    fn edges() {
        let mut g = Graph::<()>::default();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(a, c);
        g.add_edge(b, c);
        let exp_a = vec![0, 0, 1];
        let exp_b = vec![0, 0, 1];
        let exp_c = vec![1, 1, 0];
        assert_eq!(g.edges(a), &exp_a);
        assert_eq!(g.edges(b), &exp_b);
        assert_eq!(g.edges(c), &exp_c);
    }

    #[test]
    fn transpose_g() {
        let mut g = Graph::<(), Directed>::default();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(a, b);
        g.add_edge(c, b);
        let exp_a = vec![0, 1, 0];
        let exp_b = vec![0, 0, 0];
        let exp_c = vec![0, 1, 0];
        assert_eq!(g.edges(a), &exp_a);
        assert_eq!(g.edges(b), &exp_b);
        assert_eq!(g.edges(c), &exp_c);
        transpose(&mut g);
        let exp_a = vec![0, 0, 0];
        let exp_b = vec![1, 0, 1];
        let exp_c = vec![0, 0, 0];
        assert_eq!(g.edges(a), &exp_a);
        assert_eq!(g.edges(b), &exp_b);
        assert_eq!(g.edges(c), &exp_c);
    }
}
