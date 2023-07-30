use crate::graph::Graph;

pub trait Algorithm<G: Graph> {
    fn run(&self, from: G::Index, to: G::Index) -> Option<Vec<G::Index>>;
}
