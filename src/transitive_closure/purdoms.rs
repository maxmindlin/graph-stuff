use std::collections::{HashMap, HashSet};

use crate::list_graph::graph::Graph;

use super::{mtx::TransitiveClosureMtx, tarjan::Tarjan};

/// Calculates the transitive closure matrix
/// of a given graph using purdoms algorithm.
/// Time complexity:
///   O(|E| + 𝜇|V|) where 𝜇 = # of strongly connected components.
pub fn purdoms<V, D, E>(graph: &mut Graph<V, D, E>) -> TransitiveClosureMtx
{
    // 1. Find the strongly connected components of theoriginal graph,
    //    replace each component by a single node, and remove the resulting loops.
    // 2. Perform the topological sort of the acyclic graph 𝐺̃  obtained at stage 1.
    // 3. Calculate the transitive closure of 𝐺̃ , moving from nodes with larger
    //    indices to those with smaller ones.
    // 4. Reconstruct the transitive closure of the original graph from the
    //    transitive closure of 𝐺̃ .

    // 1.
    let sccs = Tarjan::new(&graph).sccs();
    replace_sccs(graph, &sccs);

    // 2.
    let topo = topo_sort(&graph);
    unimplemented!()
}

pub fn replace_sccs<V, D, E>(
    graph: &mut Graph<V, D, E>,
    sccs: &[usize],
) {
    let map = gather_sccs(&sccs);
    for (keep, replaces) in map.iter() {
        graph.replace_nodes(replaces, *keep);
    }
}

fn topo_sort<V, D, E>(graph: &Graph<V, D, E>) -> Vec<usize> {
    let mut topo = Vec::new();
    let mut visited = HashSet::new();
    for i in 0..graph.len() {
        if !visited.contains(&i) {
            dfs_recursive(
                graph,
                &mut topo,
                &mut visited,
                i,
            );
        }
    }
    topo
}

fn dfs_recursive<V, D, E>(
    graph: &Graph<V, D, E>,
    topo: &mut Vec<usize>,
    visited: &mut HashSet<usize>,
    start: usize,
) {
    println!("visiting {}", start);
    visited.insert(start);
    for neighbor in graph.neighbors(start) {
        if !visited.contains(neighbor) {
            dfs_recursive(graph, topo, visited, *neighbor);
        }
    }
    topo.push(start);
    println!("{:?}", topo);
}

fn gather_sccs(sccs: &[usize]) -> HashMap<usize, Vec<usize>> {
    let mut replacements = HashMap::new();
    for (to, fr) in sccs.iter().enumerate() {
        if to == *fr {
            continue;
        }

        let entry = replacements.entry(*fr).or_insert(Vec::new());
        entry.push(to);
    }

    replacements
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::list_graph::graph::Directed;

    use super::*;

    #[test]
    fn gather() {
        let sccs = vec![0, 0, 2, 2, 2, 5, 0];
        let exp = HashMap::from_iter([
            (0, vec![1, 6]),
            (2, vec![3, 4]),
        ]);
        assert_eq!(exp, gather_sccs(&sccs));
    }

    #[test]
    fn topo() {
        let mut g = Graph::<(), Directed>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());
        let e = g.add_node(());
        let f = g.add_node(());
        g.add_edge(c, d, 1);
        g.add_edge(d, b, 1);
        g.add_edge(e, a, 1);
        g.add_edge(e, b, 1);
        g.add_edge(f, a, 1);
        g.add_edge(f, c, 1);
        let topo = topo_sort(&g);
        assert_eq!(topo, vec![0, 1, 3, 2, 4, 5]);
    }
}
