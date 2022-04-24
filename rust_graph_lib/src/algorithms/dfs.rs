use crate::graph::Graph;
use std::{collections::HashSet, fmt::Debug, hash::Hash};

/**
 * Depth first search from `from` to `to`.
 */
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

    Some(cur).filter(|cur| cur.len() >= 2)
}

fn inner_dfs<'a, N: Eq + Hash + Debug>(
    g: &'a impl Graph<N>,
    visited: &mut HashSet<&'a N>,
    cur: &mut Vec<&'a N>,
    to: &'a N,
) -> bool {
    let from = cur.last().unwrap();
    if *from == to {
        return true;
    }

    if let Some(adj) = g.iter_adj(from) {
        visited.insert(from);
        for a in adj {
            if !visited.contains(&a) {
                cur.push(a);
                if inner_dfs(g, visited, cur, to) {
                    return true;
                }
                cur.pop();
            }
        }
    }

    false
}
