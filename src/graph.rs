use std::fmt::Debug;

pub type Edge<'s, N> = (&'s N, &'s N);
pub type NodeIterator<'s, N> = dyn Iterator<Item = &'s N> + 's;
pub type EdgeIterator<'s, N> = dyn Iterator<Item = Edge<'s, N>> + 's;

pub trait Graph<N: Eq + Debug> {
    fn add_node(&mut self, n: N);
    fn add_edge(&mut self, f: N, t: N);
    fn has_node(&self, n: &N) -> bool;
    fn has_edge(&self, f: &N, t: &N) -> bool;
    fn remove_node(&mut self, n: &N);
    fn remove_edge(&mut self, f: &N, t: &N);
    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn iter_nodes(&self) -> Box<NodeIterator<N>>;
    fn iter_adj<'s>(&'s self, n: &N) -> Option<Box<NodeIterator<'s, N>>>;
    fn iter_edges(&self) -> Box<EdgeIterator<N>>;
}
