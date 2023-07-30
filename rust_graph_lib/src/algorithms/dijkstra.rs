use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    iter,
};

use crate::graph::Graph;

use super::Algorithm;

struct NodeWithDist<G: Graph>(G::Index, u32);

impl<G: Graph> PartialEq for NodeWithDist<G> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<G: Graph> Eq for NodeWithDist<G> {}

impl<G: Graph> Ord for NodeWithDist<G> {
    fn cmp(&self, other: &Self) -> Ordering {
        // order is reversed because BinaryHeap returns the max
        other.1.cmp(&self.1)
    }
}

impl<G: Graph> PartialOrd for NodeWithDist<G> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Dijkstra<'a, G: Graph> {
    graph: &'a G,
}

impl<'a, G: Graph> Dijkstra<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        Self { graph }
    }
}

impl<'a, I: Hash + Eq + Copy, G: Graph<Index = I>> Algorithm<G> for Dijkstra<'a, G> {
    fn run(&self, from: G::Index, to: G::Index) -> Option<Vec<G::Index>> {
        let mut preds = HashMap::with_capacity(self.graph.node_count());
        let mut heap = BinaryHeap::<NodeWithDist<G>>::with_capacity(self.graph.node_count());

        for node in self.graph.iter_nodes() {
            let cost = if node == from { 0 } else { std::u32::MAX };
            heap.push(NodeWithDist(node, cost));
            preds.insert(node, (None, cost));
        }

        while let Some(NodeWithDist(node, cost)) = heap.pop() {
            if cost == std::u32::MAX || node == to {
                break;
            }

            self.graph
                .iter_adj(node)
                .unwrap_or_else(|| Box::new(iter::empty()))
                .for_each(|adj| {
                    let (_, node_dist) = preds[&node];
                    let (_, adj_dist) = preds[&adj];
                    let alt = node_dist + 1;
                    if alt < adj_dist {
                        *preds.get_mut(&adj).unwrap() = (Some(node), alt);
                        heap.push(NodeWithDist(adj, alt));
                    }
                });
        }

        let not_found_path = matches!(preds.get(&to), None | Some((None, _)));
        if not_found_path {
            return None;
        }

        let mut ret = Vec::new();
        let mut cur = to;

        while cur != from {
            ret.push(cur);
            cur = preds[&cur].0.unwrap();
        }

        ret.push(from);
        ret.reverse();
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::{Algorithm, Dijkstra};
    use crate::{algorithms::test_utils::slice_equal, graph::Graph, impls::adj_list::AdjListGraph};
    use std::hash::Hash;

    fn dijkstra<I: Hash + Eq + Copy, G: Graph<Index = I>>(
        g: &G,
        from: I,
        to: I,
    ) -> Option<Vec<G::Index>> {
        Dijkstra::new(g).run(from, to)
    }

    #[test]
    fn works_with_path_present() {
        let mut g = AdjListGraph::new();
        let id1 = g.add_node();
        let id2 = g.add_node();
        let id3 = g.add_node();
        let id4 = g.add_node();
        let id5 = g.add_node();

        g.add_edge(id1, id2);
        g.add_edge(id2, id3);
        g.add_edge(id3, id4);
        g.add_edge(id4, id5);

        let p = dijkstra(&g, id1, id5);
        assert!(p.is_some());
        assert!(slice_equal(&p.unwrap(), &[id1, id2, id3, id4, id5]));
    }

    #[test]
    fn gets_shortest_path() {
        let mut g = AdjListGraph::new();
        let id1 = g.add_node();
        let id2 = g.add_node();
        let id3 = g.add_node();
        let id4 = g.add_node();
        let id5 = g.add_node();

        g.add_edge(id1, id2);
        g.add_edge(id2, id3);
        g.add_edge(id3, id4);
        g.add_edge(id4, id5);
        g.add_edge(id1, id3);
        g.add_edge(id3, id5);

        let p = dijkstra(&g, id1, id5);
        assert!(p.is_some());
        assert!(slice_equal(&p.unwrap(), &[id1, id3, id5]));
    }

    #[test]
    fn works_with_path_not_present() {
        let mut g = AdjListGraph::new();
        let id1 = g.add_node();
        let id2 = g.add_node();
        let id3 = g.add_node();
        let id4 = g.add_node();
        let id5 = g.add_node();

        g.add_edge(id1, id2);
        g.add_edge(id3, id4);
        g.add_edge(id4, id5);

        let p = dijkstra(&g, id1, id5);
        assert!(p.is_none());
    }
}
