use crate::graph::{EdgeIterator, Graph, NodeIterator};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

/**
 * Adjacency matrix implementation of [`Graph`].
 */
pub struct AdjMatrixGraph<N: Hash + Eq + Debug> {
    identifiers: HashMap<N, u32>,
    starting_id: u32,
    nodes: HashSet<u32>,
    edges: HashMap<u32, HashSet<u32>>,
    edge_count: usize,
}

impl<N: Hash + Eq + Debug> AdjMatrixGraph<N> {
    /**
     * Creates a new graph.
     */
    pub fn new() -> Self {
        AdjMatrixGraph {
            edges: HashMap::new(),
            nodes: HashSet::new(),
            identifiers: HashMap::new(),
            starting_id: 1,
            edge_count: 0,
        }
    }

    fn inner_add_node(&mut self, n: N) -> u32 {
        let id = self.starting_id;
        self.identifiers.insert(n, id);
        self.starting_id += 1;
        self.nodes.insert(id);
        self.edges.insert(id, HashSet::new());
        id
    }

    fn find_or_add(&mut self, n: N) -> u32 {
        self.identifiers
            .get(&n)
            .copied()
            .or_else(|| {
                let id = self.inner_add_node(n);
                Some(id)
            })
            .unwrap()
    }

    fn reverse_map(&self) -> HashMap<u32, &N> {
        self.identifiers.iter().map(|(k, v)| (*v, k)).collect()
    }
}

impl<N: Hash + Eq + Debug> Graph<N> for AdjMatrixGraph<N> {
    fn add_node(&mut self, n: N) {
        self.inner_add_node(n);
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

        let id = self.identifiers.remove(n).unwrap();
        self.nodes.remove(&id);
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

        let fid = self.identifiers.get(f).unwrap();
        let tid = self.identifiers.get(t).unwrap();
        let edges = &mut self.edges;

        let e = edges.get_mut(fid).unwrap();
        if e.remove(tid) {
            self.edge_count -= 1;
        }
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.edge_count
    }

    fn has_node(&self, n: &N) -> bool {
        self.identifiers
            .get(n)
            .filter(|id| self.nodes.contains(id))
            .is_some()
    }

    fn has_edge(&self, f: &N, t: &N) -> bool {
        self.identifiers
            .get(f)
            .filter(|fid| {
                self.identifiers
                    .get(t)
                    .filter(|tid| self.edges.get(fid).filter(|v| v.contains(tid)).is_some())
                    .is_some()
            })
            .is_some()
    }

    fn iter_nodes(&self) -> Box<NodeIterator<N>> {
        let m = self.reverse_map();
        Box::new(self.nodes.iter().map(move |v| *m.get(v).unwrap()))
    }

    fn iter_adj(&self, n: &N) -> Option<Box<NodeIterator<N>>> {
        let m = self.reverse_map();

        fn helper<'a, N>(m: HashMap<u32, &'a N>, v: &'a HashSet<u32>) -> Box<NodeIterator<'a, N>> {
            Box::new(v.iter().map(move |v| *m.get(v).unwrap()))
        }

        let id = self.identifiers.get(n).unwrap();
        let adjacent = self.edges.get(id);
        adjacent.map(move |v| helper(m, v))
    }

    fn iter_edges(&self) -> Box<EdgeIterator<N>> {
        let m = self.reverse_map();
        let v = self.edges.iter().flat_map(move |(k, vs)| {
            let m = m.clone();
            vs.iter()
                .map(move |v| (*m.get(k).unwrap(), *m.get(v).unwrap()))
        });
        Box::new(v)
    }
}

impl<N: Display + Hash + Eq + Debug> Display for AdjMatrixGraph<N> {
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
        let mut g = AdjMatrixGraph::new();
        assert_eq!(g.node_count(), 0);
        g.add_node(1);
        assert_eq!(g.node_count(), 1);
        assert!(g.has_node(&1));
    }

    #[test]
    fn test_remove_node_no_edges() {
        let mut g = AdjMatrixGraph::new();
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
        let mut g = AdjMatrixGraph::new();
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
