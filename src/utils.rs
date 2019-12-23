use crate::graph::Graph;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub fn dfs<'a, N: Eq + Hash + Debug>(
    g: &'a impl Graph<N>,
    from: &'a N,
    to: &'a N,
) -> Option<Vec<&'a N>> {
    let mut cur = Vec::new();

    if g.has_node(from) && g.has_node(to) {
        let mut visited = HashSet::new();
        cur.push(from);
        inner_dfs(g, &mut visited, &mut cur, to);
    }

    if cur.len() < 2 {
        None
    } else {
        Some(cur)
    }
}

fn inner_dfs<'a, 'b, 'c, N: Eq + Hash + Debug>(
    g: &'a impl Graph<N>,
    visited: &'b mut HashSet<&'a N>,
    cur: &'c mut Vec<&'a N>,
    to: &'a N,
) -> bool {
    let from = cur.last().unwrap();
    if *from == to {
        return true;
    }

    if let Some(adj) = g.iter_adj(&from) {
        visited.insert(&from);
        for a in adj {
            if !visited.contains(&a) {
                cur.push(&a);
                if inner_dfs(g, visited, cur, &to) {
                    return true;
                }
                cur.pop();
            }
        }
    }

    false
}

#[derive(Eq, PartialEq)]
struct NodeWithDist<N: Eq + Ord>(N, u32);

impl<N: Eq + Ord> Ord for NodeWithDist<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl<N: Eq + Ord> PartialOrd for NodeWithDist<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra<'a, N: Ord + Debug + Hash>(
    g: &'a impl Graph<N>,
    from: &'a N,
    to: &'a N,
) -> Option<Vec<&'a N>> {
    let mut preds: HashMap<&N, (Option<&N>, u32)> = HashMap::with_capacity(g.node_count());
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
            let (_, distu) = preds[node];
            let (_, dista) = preds[adj];
            let alt = distu + 1;
            if alt < dista {
                let p = preds.get_mut(adj).unwrap();
                p.0 = Some(node);
                p.1 = alt;
                heap.push(NodeWithDist(adj, alt));
            }
        }
    }

    let found_path = match preds.get(to) {
        None => false,
        Some((Some(_), cost)) => *cost != std::u32::MAX,
        _ => true,
    };

    if !found_path {
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
