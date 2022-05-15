use crate::graph::Graph;
use std::{collections::HashSet, fmt::Debug, hash::Hash, iter, marker::PhantomData};

use super::Algorithm;

pub struct Dfs<'a, N: Eq + Debug, G: Graph<N>> {
    graph: &'a G,
    _ph: PhantomData<N>,
}

impl<'a, N: Eq + Debug + Hash, G: Graph<N>> Dfs<'a, N, G> {
    pub fn new(graph: &'a G) -> Self {
        Self {
            graph,
            _ph: PhantomData,
        }
    }

    fn inner_dfs(&'a self, visited: &mut HashSet<&'a N>, cur: &mut Vec<&'a N>, to: &'a N) -> bool {
        let from = cur.last().unwrap();
        if *from == to {
            return true;
        }

        visited.insert(from);

        self.graph
            .iter_adj(from)
            .unwrap_or_else(|| Box::new(iter::empty()))
            .any(|a| {
                if !visited.contains(a) {
                    cur.push(a);
                    if self.inner_dfs(visited, cur, to) {
                        return true;
                    }
                    cur.pop();
                }
                false
            })
    }
}

impl<'a, N: Eq + Debug + Ord + Hash, G: 'a + Graph<N>> Algorithm<'a, N> for Dfs<'a, N, G> {
    fn run(&'a self, from: &'a N, to: &'a N) -> Option<Vec<&'a N>> {
        let mut cur = Vec::new();

        if self.graph.has_node(from) && self.graph.has_node(to) {
            let mut visited = HashSet::new();
            cur.push(from);
            self.inner_dfs(&mut visited, &mut cur, to);
        }

        Some(cur).filter(|cur| cur.len() >= 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::{algorithms::test_utils::slice_equal, build_graph};

    use super::*;

    fn dfs<'a, G: Graph<i32>>(g: &'a G, from: i32, to: i32) -> Option<Vec<i32>> {
        Dfs::new(g)
            .run(&from, &to)
            .map(|v| v.into_iter().copied().collect())
    }

    #[test]
    fn works_with_path_present() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 2 => 3, 3 => 4, 4 => 5
        );

        let p = dfs(&g, 1, 5);
        assert!(p.is_some());
        assert!(slice_equal(&p.unwrap(), &[1, 2, 3, 4, 5]));
    }

    #[test]
    fn works_with_path_not_present() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 3 => 4, 4 => 5
        );
        let p = dfs(&g, 1, 5);
        assert!(p.is_none());
    }

    #[test]
    fn works_with_nonexistant_src() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 3 => 4, 4 => 5
        );
        let p = dfs(&g, 0, 5);
        assert!(p.is_none());
    }

    #[test]
    fn works_with_nonexistant_dst() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 3 => 4, 4 => 5
        );
        let p = dfs(&g, 1, 6);
        assert!(p.is_none());
    }
}
