use crate::graph::{EdgeIterator, Graph, NodeIterator};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

pub struct AdjMatrixGraph<N: Hash + Eq + Debug> {
    nodes: Vec<Rc<N>>,
    edges: HashMap<Rc<N>, Vec<Rc<N>>>,
    edge_count: usize,
}

impl<N: Hash + Eq + Debug> AdjMatrixGraph<N> {
    pub fn new() -> Self {
        AdjMatrixGraph {
            edges: HashMap::new(),
            nodes: Vec::new(),
            edge_count: 0,
        }
    }

    fn find_or_add(&mut self, n: N) -> Rc<N> {
        if !self.has_node(&n) {
            self.add_node(n);
            Rc::clone(self.nodes.last().unwrap())
        } else {
            Rc::clone(self.nodes.iter().find(|&r| **r == n).unwrap())
        }
    }

    fn node_ref(&self, n: &N) -> Option<Rc<N>> {
        self.nodes.iter().find(|&x| **x == *n).map(|x| Rc::clone(x))
    }
}

impl<N: Hash + Eq + Debug> Graph<N> for AdjMatrixGraph<N> {
    fn add_node(&mut self, n: N) {
        let rc = Rc::new(n);
        self.nodes.push(Rc::clone(&rc));
        self.edges.insert(Rc::clone(&rc), Vec::new());
    }

    fn add_edge(&mut self, f: N, t: N) {
        if self.has_edge(&f, &t) {
            return;
        }
        let fr = self.find_or_add(f);
        let tr = self.find_or_add(t);
        let edges = &mut self.edges;

        let e = edges.get_mut(&fr).unwrap();
        e.push(tr);
        self.edge_count += 1;
    }

    fn remove_node(&mut self, n: &N) {
        if !self.has_node(n) {
            return;
        }

        remove_from_vec(&mut self.nodes, n);
        if let Some((_, v)) = self.edges.remove_entry(n) {
            self.edge_count -= v.len();
        }

        let e = &mut self.edges;
        let mut to_remove = 0;
        e.iter_mut().for_each(|(_, v)| {
            if remove_from_vec(v, n) {
                to_remove += 1;
            }
        });
        self.edge_count -= to_remove;
    }

    fn remove_edge(&mut self, f: &N, t: &N) {
        if !self.has_edge(f, t) {
            return;
        }

        let edges = &mut self.edges;

        let e = edges.get_mut(f).unwrap();
        if remove_from_vec(e, t) {
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
        self.nodes.iter().any(|r| **r == *n)
    }

    fn has_edge(&self, f: &N, t: &N) -> bool {
        let e = self.edges.get(f);
        match e {
            Some(v) => v.iter().any(|r| **r == *t),
            None => false,
        }
    }

    fn iter_nodes(&self) -> Box<NodeIterator<N>> {
        Box::new(self.nodes.iter().map(|v| v.as_ref()))
    }

    fn iter_adj(&self, n: &N) -> Option<Box<NodeIterator<N>>> {
        fn helper<N>(v: &[Rc<N>]) -> Box<NodeIterator<N>> {
            Box::new(v.iter().map(|v| v.as_ref()))
        };

        let adjs = self.edges.get(n);
        adjs.map(|v| helper(v.as_slice()))
    }

    fn iter_edges<'s>(&'s self) -> Box<EdgeIterator<'s, N>> {
        Box::new(
            self.edges
                .iter()
                .flat_map(|(k, vs)| vs.iter().map(move |v| (k.as_ref(), v.as_ref()))),
        )
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

impl<N: Debug + Hash + Eq> Debug for AdjMatrixGraph<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for n in self.iter_nodes() {
            let node_ref = self.node_ref(n).unwrap();
            writeln!(
                f,
                "Node {:?} has {} strong refs, {} weak refs",
                n,
                Rc::strong_count(&node_ref),
                Rc::weak_count(&node_ref)
            )?;
        }

        for (key, val) in self.iter_edges() {
            let kr = self.node_ref(key).unwrap();
            let vr = self.node_ref(val).unwrap();

            writeln!(
                f,
                "Left node {:?} has {} strong refs, {} weak refs",
                key,
                Rc::strong_count(&kr),
                Rc::weak_count(&kr)
            )?;

            writeln!(
                f,
                "Right node {:?} has {} strong refs, {} weak refs",
                val,
                Rc::strong_count(&vr),
                Rc::weak_count(&vr)
            )?;
        }
        Ok(())
    }
}

fn remove_from_vec<T: Eq, U: Deref<Target = T>>(vec: &mut Vec<U>, item: &T) -> bool {
    if let Some(i) = vec.iter().position(|n| **n == *item) {
        vec.remove(i);
        true
    } else {
        false
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
