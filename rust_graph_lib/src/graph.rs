pub type Edge<Idx> = (Idx, Idx);
pub type NodeIterator<'s, Idx> = dyn Iterator<Item = Idx> + 's;
pub type EdgeIterator<'s, Idx> = dyn Iterator<Item = Edge<Idx>> + 's;

/**
 * Graph trait.
 */
pub trait Graph {
    type Index: Ord;

    /**
     * Add a node to the graph.
     */
    fn add_node(&mut self) -> Self::Index;
    /**
     * Add an edge to the graph.
     */
    fn add_edge(&mut self, f: Self::Index, t: Self::Index);
    /**
     * Checks if an edge is in the graph.
     */
    fn has_edge(&self, f: Self::Index, t: Self::Index) -> bool;
    /**
     * Remove a node from the graph.
     */
    fn remove_node(&mut self, n: Self::Index);
    /**
     * Remove an edge from the graph.
     */
    fn remove_edge(&mut self, f: Self::Index, t: Self::Index);
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
    fn iter_nodes(&self) -> Box<NodeIterator<Self::Index>>;
    /**
     * Returns an iterator over nodes adjacent to the specified node in the graph.
     */
    fn iter_adj(&self, n: Self::Index) -> Option<Box<NodeIterator<Self::Index>>>;
    /**
     * Returns an iterator over all edges in the graph.
     */
    fn iter_edges(&self) -> Box<EdgeIterator<Self::Index>>;
}
