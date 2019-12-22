use crate::graph::Graph;
use std::collections::hash_set::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub fn dfs<'a, N: Eq + Hash + Debug>(g: &'a impl Graph<N>, from: &'a N, to: &'a N) -> Vec<&'a N> {
    let mut cur = Vec::new();

    if g.has_node(from) && g.has_node(to) {
        let mut visited = HashSet::new();
        cur.push(from);
        inner_dfs(g, &mut visited, &mut cur, to);
    }
    cur
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
