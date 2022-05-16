use crate::{
    graph::{EdgeIterator, Graph, NodeIterator},
    mapping::DoubleMapping,
};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

/**
 * Adjacency list implementation of [`Graph`].
 */
pub struct AdjListGraph<N: Hash + Eq + Debug> {
    identifiers: DoubleMapping<N>,
    edges: HashMap<usize, HashSet<usize>>,
    edge_count: usize,
}

impl<N: Hash + Eq + Debug> AdjListGraph<N> {
    /**
     * Creates a new graph.
     */
    pub fn new() -> Self {
        AdjListGraph {
            edges: HashMap::new(),
            identifiers: DoubleMapping::new(false),
            edge_count: 0,
        }
    }

    fn find_or_add(&mut self, n: N) -> usize {
        if let Some(id) = self.identifiers.get_by_obj(&n) {
            id
        } else {
            let id = self.identifiers.insert(n).unwrap();
            self.edges.insert(id, HashSet::new());
            id
        }
    }
}

impl<N: Hash + Eq + Debug> Default for AdjListGraph<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Hash + Eq + Debug> Graph<N> for AdjListGraph<N> {
    fn add_node(&mut self, n: N) {
        self.find_or_add(n);
    }

    fn add_edge(&mut self, f: N, t: N) {
        if self.has_edge(&f, &t) {
            return;
        }
        let fr = self.find_or_add(f);
        let tr = self.find_or_add(t);
        let edges = &mut self.edges;

        let e = edges.get_mut(&fr).unwrap();
        e.insert(tr);
        self.edge_count += 1;
    }

    fn remove_node(&mut self, n: &N) {
        if !self.has_node(n) {
            return;
        }

        let id = self.identifiers.get_by_obj(n).unwrap();
        self.identifiers.remove(id);
        if let Some((_, v)) = self.edges.remove_entry(&id) {
            self.edge_count -= v.len();
        }

        let e = &mut self.edges;
        let mut to_remove = 0;
        e.iter_mut().for_each(|(_, v)| {
            if v.remove(&id) {
                to_remove += 1;
            }
        });
        self.edge_count -= to_remove;
    }

    fn remove_edge(&mut self, f: &N, t: &N) {
        if !self.has_edge(f, t) {
            return;
        }

        let fid = self.identifiers.get_by_obj(f).unwrap();
        let tid = self.identifiers.get_by_obj(t).unwrap();
        let edges = &mut self.edges;

        let e = edges.get_mut(&fid).unwrap();
        if e.remove(&tid) {
            self.edge_count -= 1;
        }
    }

    fn node_count(&self) -> usize {
        self.edges.len()
    }

    fn edge_count(&self) -> usize {
        self.edge_count
    }

    fn has_node(&self, n: &N) -> bool {
        self.identifiers.contains_obj(n)
    }

    fn has_edge(&self, f: &N, t: &N) -> bool {
        self.identifiers
            .get_by_obj(f)
            .filter(|fid| {
                self.identifiers
                    .get_by_obj(t)
                    .filter(|tid| self.edges.get(fid).filter(|v| v.contains(tid)).is_some())
                    .is_some()
            })
            .is_some()
    }

    fn iter_nodes(&self) -> Box<NodeIterator<N>> {
        Box::new(
            self.edges
                .keys()
                .map(|&v| self.identifiers.get_by_id(v).unwrap()),
        )
    }

    fn iter_adj(&self, n: &N) -> Option<Box<NodeIterator<N>>> {
        fn helper<'a, N: Eq + Hash + Debug>(
            m: &'a DoubleMapping<N>,
            v: &'a HashSet<usize>,
        ) -> Box<NodeIterator<'a, N>> {
            Box::new(v.iter().map(|v| m.get_by_id(*v).unwrap()))
        }

        let id = self.identifiers.get_by_obj(n).unwrap();
        let adjacent = self.edges.get(&id);
        adjacent.map(move |v| helper(&self.identifiers, v))
    }

    fn iter_edges(&self) -> Box<EdgeIterator<N>> {
        let v = self.edges.iter().flat_map(|(k, vs)| {
            vs.iter().map(|v| {
                (
                    self.identifiers.get_by_id(*k).unwrap(),
                    self.identifiers.get_by_id(*v).unwrap(),
                )
            })
        });
        Box::new(v)
    }
}

impl<N: Display + Hash + Eq + Debug> Display for AdjListGraph<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for n in self.iter_nodes() {
            writeln!(f, "Node {}", n)?;
        }
        for (x, y) in self.iter_edges() {
            writeln!(f, "{} -> {}", x, y)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut g = AdjListGraph::new();
        assert_eq!(g.node_count(), 0);
        g.add_node(1);
        assert_eq!(g.node_count(), 1);
        assert!(g.has_node(&1));
    }

    #[test]
    fn test_remove_node_no_edges() {
        let mut g = AdjListGraph::new();
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
        let mut g = AdjListGraph::new();
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
