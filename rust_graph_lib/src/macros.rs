#[macro_export]
macro_rules! build_graph {
    (
        $( $n:expr ),*;
        $( $f:expr => $t: expr ),*
    ) => {{
        use $crate::adj_matrix::AdjMatrixGraph;
        use $crate::graph::Graph;

        let mut graph = AdjMatrixGraph::new();

        $(
            graph.add_node($n);
        )*

        $(
            graph.add_edge($f, $t);
        )*

        graph
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::graph::Graph;

    #[test]
    fn test_graph() {
        let g = build_graph!(
            1, 2, 3, 4, 5;
            1 => 2, 1 => 3, 2 => 4, 4 => 5
        );

        assert_eq!(g.node_count(), 5);
        assert_eq!(g.edge_count(), 4);

        assert_eq!(
            g.iter_nodes().copied().collect::<HashSet<_>>(),
            HashSet::from_iter(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            g.iter_edges().map(|t| (*t.0, *t.1)).collect::<HashSet<_>>(),
            HashSet::from_iter(vec![(1, 2), (1, 3), (2, 4), (4, 5)])
        );
    }

    #[test]
    fn test_struct_graph() {
        #[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
        struct S(u32);

        let g = build_graph!(
            S(1), S(2), S(3), S(4), S(5);
            S(1) => S(2), S(1) => S(3), S(2) => S(4), S(4) => S(5)
        );

        assert_eq!(g.node_count(), 5);
        assert_eq!(g.edge_count(), 4);

        assert_eq!(
            g.iter_nodes().cloned().collect::<HashSet<_>>(),
            HashSet::from_iter(vec![1, 2, 3, 4, 5].into_iter().map(|n| S(n)))
        );
        assert_eq!(
            g.iter_edges().map(|t| (*t.0, *t.1)).collect::<HashSet<_>>(),
            HashSet::from_iter(
                vec![(1, 2), (1, 3), (2, 4), (4, 5)]
                    .into_iter()
                    .map(|(f, t)| (S(f), S(t)))
            )
        );
    }
}
