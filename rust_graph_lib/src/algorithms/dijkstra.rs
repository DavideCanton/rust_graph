use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    hash::Hash,
};

use crate::graph::Graph;

#[derive(Eq, PartialEq)]
struct NodeWithDist<N: Eq + Ord>(N, u32);

impl<N: Eq + Ord> Ord for NodeWithDist<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // order is reversed because BinaryHeap returns the max
        other.1.cmp(&self.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl<N: Eq + Ord> PartialOrd for NodeWithDist<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/**
 * Dijkstra path finding from `from` to `to`.
 */
pub fn dijkstra<'a, N: Ord + Debug + Hash>(
    g: &'a impl Graph<N>,
    from: &'a N,
    to: &'a N,
) -> Option<Vec<&'a N>> {
    let mut preds = HashMap::with_capacity(g.node_count());
    let mut heap = BinaryHeap::with_capacity(g.node_count());

    for n in g.iter_nodes() {
        let cost = if n == from { 0 } else { std::u32::MAX };
        heap.push(NodeWithDist(n, cost));
        preds.insert(n, (None, cost));
    }

    while let Some(NodeWithDist(node, cost)) = heap.pop() {
        if cost == std::u32::MAX || node == to {
            break;
        }
        let adjs = g.iter_adj(node).unwrap();
        for adj in adjs {
            let (_, node_dist) = preds[node];
            let (_, adj_dist) = preds[adj];
            let alt = node_dist + 1;
            if alt < adj_dist {
                *preds.get_mut(adj).unwrap() = (Some(node), alt);
                heap.push(NodeWithDist(adj, alt));
            }
        }
    }

    let not_found_path = matches!(preds.get(to), None | Some((None, _)));
    if not_found_path {
        return None;
    }

    let mut ret = Vec::new();
    let mut cur = to;

    while cur != from {
        ret.push(cur);
        cur = preds[cur].0.unwrap();
    }

    ret.push(from);
    ret.reverse();
    Some(ret)
}

#[cfg(test)]
mod tests {
    use crate::{algorithms::test_utils::slice_equal, build_graph};

    use super::*;

    #[test]
    fn works_with_path_present() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 2 => 3, 3 => 4, 4 => 5
        );

        let p = dijkstra(&g, &1, &5);
        assert!(p.is_some());
        assert!(slice_equal(&p.unwrap(), &[1, 2, 3, 4, 5]));
    }

    #[test]
    fn gets_shortest_path() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 2 => 3, 3 => 4, 4 => 5, 1 => 3, 3 => 5
        );

        let p = dijkstra(&g, &1, &5);
        assert!(p.is_some());
        assert!(slice_equal(&p.unwrap(), &[1, 3, 5]));
    }

    #[test]
    fn works_with_path_not_present() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 3 => 4, 4 => 5
        );
        let p = dijkstra(&g, &1, &5);
        assert!(p.is_none());
    }

    #[test]
    fn works_with_nonexistant_src() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 3 => 4, 4 => 5
        );        let p = dijkstra(&g, &0, &5);
        assert!(p.is_none());
    }

    #[test]
    fn works_with_nonexistant_dst() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 3 => 4, 4 => 5
        );        let p = dijkstra(&g, &1, &6);
        assert!(p.is_none());
    }
}
