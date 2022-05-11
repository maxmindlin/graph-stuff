use super::mtx::TransitiveClosureMtx;

/// Calculates the transitive closure matrix
/// of a given graph using purdoms algorithm.
/// Time complexity:
///   O(|E| + 𝜇|V|) where 𝜇 = # of strongly connected components.
pub fn purdoms() -> TransitiveClosureMtx
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
