use crate::graph::Graph;
use std::{collections::HashSet, hash::Hash, iter};

use super::Algorithm;

pub struct Dfs<'a, G: Graph> {
    graph: &'a G,
}

// TODO remove copy
impl<'a, I: Hash + Eq + Copy, G: Graph<Index = I>> Dfs<'a, G> {
    pub fn new(graph: &'a G) -> Self {
        Self { graph }
    }

    fn inner_dfs(
        &'a self,
        visited: &mut HashSet<G::Index>,
        cur: &mut Vec<G::Index>,
        to: G::Index,
    ) -> bool {
        let from = cur.last().unwrap();
        if *from == to {
            return true;
        }

        visited.insert(*from);

        self.graph
            .iter_adj(*from)
            .unwrap_or_else(|| Box::new(iter::empty()))
            .any(|a| {
                if !visited.contains(&a) {
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

impl<'a, I: Hash + Eq + Copy, G: Graph<Index = I>> Algorithm<G> for Dfs<'a, G> {
    fn run(&self, from: G::Index, to: G::Index) -> Option<Vec<G::Index>> {
        let mut cur = Vec::new();

        let mut visited = HashSet::new();
        cur.push(from);
        self.inner_dfs(&mut visited, &mut cur, to);

        Some(cur).filter(|cur| cur.len() >= 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::{algorithms::test_utils::slice_equal, graph::Graph, impls::adj_list::AdjListGraph};

    use super::{Algorithm, Dfs};
    use std::hash::Hash;

    fn dfs<I: Hash + Eq + Copy, G: Graph<Index = I>>(
        g: &G,
        from: G::Index,
        to: G::Index,
    ) -> Option<Vec<G::Index>> {
        Dfs::new(g).run(from, to)
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

        let p = dfs(&g, id1, id5);
        assert!(p.is_some());
        assert!(slice_equal(&p.unwrap(), &[id1, id2, id3, id4, id5]));
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

        let p = dfs(&g, id1, id5);
        assert!(p.is_none());
    }
}
