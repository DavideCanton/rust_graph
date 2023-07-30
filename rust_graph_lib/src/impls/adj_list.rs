use crate::graph::{EdgeIterator, Graph, NodeIterator};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct Index(usize);

impl Index {
    pub fn next(&self) -> Self {
        Index(self.0 + 1)
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/**
 * Adjacency list implementation of [`Graph`].
 */
pub struct AdjListGraph {
    edges: HashMap<Index, HashSet<Index>>,
    edge_count: usize,
    next_id: Index,
}

impl AdjListGraph {
    /**
     * Creates a new graph.
     */
    pub fn new() -> Self {
        AdjListGraph {
            edges: HashMap::new(),
            edge_count: 0,
            next_id: Index(1),
        }
    }
}

impl Default for AdjListGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph for AdjListGraph {
    type Index = Index;

    fn add_node(&mut self) -> Index {
        let id = self.next_id;
        self.next_id = self.next_id.next();
        self.edges.insert(id, HashSet::new());
        id
    }

    fn add_edge(&mut self, f: Index, t: Index) {
        self.edges.entry(t).or_insert_with(HashSet::new);
        self.edges.entry(f).or_insert_with(HashSet::new).insert(t);
        self.edge_count += 1;
    }

    fn remove_node(&mut self, n: Index) {
        if !self.edges.contains_key(&n) {
            return;
        }

        if let Some((_, v)) = self.edges.remove_entry(&n) {
            self.edge_count -= v.len();
        }

        let mut to_remove = 0;
        self.edges.iter_mut().for_each(|(_, v)| {
            if v.remove(&n) {
                to_remove += 1;
            }
        });
        self.edge_count -= to_remove;
    }

    fn remove_edge(&mut self, f: Index, t: Index) {
        if let Some(adjacents) = self.edges.get_mut(&f) {
            if adjacents.remove(&t) {
                self.edge_count -= 1;
            }
        }
    }

    fn node_count(&self) -> usize {
        self.edges.len()
    }

    fn edge_count(&self) -> usize {
        self.edge_count
    }

    fn has_edge(&self, f: Index, t: Index) -> bool {
        self.edges.get(&f).map_or(false, |v| v.contains(&t))
    }

    fn iter_nodes(&self) -> Box<NodeIterator<Index>> {
        Box::new(self.edges.keys().copied())
    }

    fn iter_adj(&self, n: Index) -> Option<Box<NodeIterator<Index>>> {
        self.edges.get(&n).map(|adj| {
            let it = adj.iter();
            let map: Box<dyn Iterator<Item = Index>> = Box::new(it.copied());
            map
        })
    }

    fn iter_edges(&self) -> Box<EdgeIterator<Index>> {
        let it = self
            .edges
            .iter()
            .flat_map(|(k, vs)| vs.iter().map(|v| (*k, *v)));

        Box::new(it)
    }
}

impl Display for AdjListGraph {
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
        let id1 = g.add_node();
        assert_eq!(g.node_count(), 1);
        let id2 = g.add_node();
        assert_eq!(g.node_count(), 2);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_remove_node_no_edges() {
        let mut g = AdjListGraph::new();
        let id1 = g.add_node();
        let id2 = g.add_node();
        let id3 = g.add_node();
        assert_eq!(g.node_count(), 3);
        g.remove_node(id1);
        assert_eq!(g.node_count(), 2);
        g.remove_node(id2);
        assert_eq!(g.node_count(), 1);
        g.remove_node(id3);
        assert_eq!(g.node_count(), 0);
    }

    #[test]
    fn test_remove_node_with_edges() {
        let mut g = AdjListGraph::new();

        let id1 = g.add_node();
        let id2 = g.add_node();
        let id3 = g.add_node();

        g.add_edge(id1, id2);
        g.add_edge(id2, id3);
        g.add_edge(id3, id1);
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 3);
        g.remove_node(id1);
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);

        g.remove_node(id2);
        assert_eq!(g.node_count(), 1);
        assert_eq!(g.edge_count(), 0);

        g.remove_node(id3);
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.edge_count(), 0);
    }
}
