use crate::{
    graph::{EdgeIterator, Graph, NodeIterator},
    mapping::DoubleMapping,
};
use std::fmt::Debug;
use std::hash::Hash;

/**
 * Adjacency list implementation of [`Graph`].
 */
pub struct IncMatrixGraph<N: Hash + Eq + Debug> {
    identifiers: DoubleMapping<N>,
    edge_count: usize,
    matrix: Vec<Vec<bool>>,
}

impl<N: Hash + Eq + Debug> IncMatrixGraph<N> {
    /**
     * Creates a new graph.
     */
    pub fn new() -> Self {
        IncMatrixGraph {
            matrix: Vec::new(),
            identifiers: DoubleMapping::new(true),
            edge_count: 0,
        }
    }

    fn find_or_add(&mut self, n: N) -> usize {
        if let Some(id) = self.identifiers.get_by_obj(&n) {
            id - 1
        } else {
            self.add_node(n);
            self.matrix.len() - 1
        }
    }
}

impl<N: Hash + Eq + Debug> Default for IncMatrixGraph<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Hash + Eq + Debug> Graph<N> for IncMatrixGraph<N> {
    fn add_node(&mut self, n: N) {
        self.identifiers.insert(n).unwrap();
        self.matrix.push(vec![false; self.matrix.len()]);
        for v in self.matrix.iter_mut() {
            v.push(false);
        }
    }

    fn add_edge(&mut self, f: N, t: N) {
        if self.has_edge(&f, &t) {
            return;
        }

        let i_f = self.find_or_add(f);
        let i_t = self.find_or_add(t);

        self.matrix[i_f][i_t] = true;
        self.edge_count += 1;
    }

    fn has_node(&self, n: &N) -> bool {
        self.identifiers.contains_obj(n)
    }

    fn has_edge(&self, f: &N, t: &N) -> bool {
        let i_f = self.identifiers.get_by_obj(f);
        let i_t = self.identifiers.get_by_obj(t);

        match (i_f, i_t) {
            (Some(i_f), Some(i_t)) => self.matrix[i_f - 1][i_t - 1],
            _ => false,
        }
    }

    fn remove_node(&mut self, n: &N) {
        let i = self.identifiers.get_by_obj(n);
        if let Some(i) = i {
            self.identifiers.remove(i).unwrap();
            let i = i - 1;
            let row = self.matrix.remove(i);
            self.edge_count -= row.into_iter().filter(|v| *v).count();
            for v in self.matrix.iter_mut() {
                if v[i] {
                    self.edge_count -= 1;
                }
                v.remove(i);
            }
        }
    }

    fn remove_edge(&mut self, f: &N, t: &N) {
        if !self.has_edge(f, t) {
            return;
        }

        let i_f = self.identifiers.get_by_obj(f);
        let i_t = self.identifiers.get_by_obj(t);

        if let (Some(i_f), Some(i_t)) = (i_f, i_t) {
            self.matrix[i_f - 1][i_t - 1] = false;
            self.edge_count -= 1;
        }
    }

    fn node_count(&self) -> usize {
        self.matrix.len()
    }

    fn edge_count(&self) -> usize {
        self.edge_count
    }

    fn iter_nodes(&self) -> Box<NodeIterator<N>> {
        Box::new(self.identifiers.iter_obj())
    }

    fn iter_adj<'s>(&'s self, n: &N) -> Option<Box<NodeIterator<'s, N>>> {
        match self.identifiers.get_by_obj(n) {
            Some(i) => {
                let it = Box::new(self.matrix[i - 1].iter().enumerate().filter_map(|(j, &b)| {
                    if b {
                        Some(self.identifiers.get_by_id(j).unwrap())
                    } else {
                        None
                    }
                }));
                Some(it)
            }
            None => None,
        }
    }

    fn iter_edges(&self) -> Box<EdgeIterator<N>> {
        Box::new(
            self.identifiers
                .iter_obj()
                .flat_map(|f| self.iter_adj(f).unwrap().map(move |t| (f, t))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut g = IncMatrixGraph::new();
        assert_eq!(g.node_count(), 0);
        g.add_node(1);
        assert_eq!(g.node_count(), 1);
        assert!(g.has_node(&1));
    }

    #[test]
    fn test_remove_node_no_edges() {
        let mut g = IncMatrixGraph::new();
        g.add_node(1);
        g.add_node(2);
        g.add_node(3);
        assert_eq!(g.node_count(), 3);
        g.remove_node(&1);
        assert_eq!(g.node_count(), 2);
        g.remove_node(&2);
        assert_eq!(g.node_count(), 1);
        g.remove_node(&3);
        assert_eq!(g.node_count(), 0);
    }

    #[test]
    fn test_remove_node_with_edges() {
        let mut g = IncMatrixGraph::new();
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 1);
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 3);

        g.remove_node(&1);
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);

        g.remove_node(&2);
        assert_eq!(g.node_count(), 1);
        assert_eq!(g.edge_count(), 0);

        g.remove_node(&3);
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.edge_count(), 0);
    }
}
