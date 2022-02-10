use std::fmt::Debug;

pub type Edge<'s, N> = (&'s N, &'s N);
pub type NodeIterator<'s, N> = dyn Iterator<Item = &'s N> + 's;
pub type EdgeIterator<'s, N> = dyn Iterator<Item = Edge<'s, N>> + 's;

/**
 * Graph trait.
 */
pub trait Graph<N: Eq + Debug> {
    /**
     * Add a node to the graph.
     */
    fn add_node(&mut self, n: N);
    /**
     * Add an edge to the graph.
     */
    fn add_edge(&mut self, f: N, t: N);
    /**
     * Checks if a node is in the graph.
     */
    fn has_node(&self, n: &N) -> bool;
    /**
     * Checks if an edge is in the graph.
     */
    fn has_edge(&self, f: &N, t: &N) -> bool;
    /**
     * Remove a node from the graph.
     */
    fn remove_node(&mut self, n: &N);
    /**
     * Remove an edge from the graph.
     */
    fn remove_edge(&mut self, f: &N, t: &N);
    /**
     * Returns the count of nodes in the graph.
     */
    fn node_count(&self) -> usize;
    /**
     * Returns the count of edges in the graph.
     */
    fn edge_count(&self) -> usize;
    /**
     * Returns an iterator over all nodes in the graph.
     */
    fn iter_nodes(&self) -> Box<NodeIterator<N>>;
    /**
     * Returns an iterator over nodes adjacent to the specified node in the graph.
     */
    fn iter_adj<'s>(&'s self, n: &N) -> Option<Box<NodeIterator<'s, N>>>;
    /**
     * Returns an iterator over all edges in the graph.
     */
    fn iter_edges(&self) -> Box<EdgeIterator<N>>;
}
