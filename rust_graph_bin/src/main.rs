use std::{collections::HashMap, rc::Rc};

use rust_graph_lib::{
    algorithms::{Algorithm, Dfs, Dijkstra},
    graph::Graph,
    impls::adj_list::AdjListGraph,
};

fn main() {
    let mut g = AdjListGraph::new();
    let max = 10;
    let mut indexes = HashMap::new();

    for i in 1u32..=max {
        let i = i.saturating_sub(3);
        let i2 = i + 3;

        for j in i..=i2 {
            if j >= 1 && j <= max && i != j && i % 3 == j % 3 {
                let idi = *indexes.entry(i).or_insert_with(|| g.add_node());
                let idj = *indexes.entry(j).or_insert_with(|| g.add_node());
                g.add_edge(idi, idj);
            }
        }
    }

    let g = Rc::new(g);

    println!("Created graph!");
    println!("Node count: {}", g.node_count());
    println!("Edge count: {}", g.edge_count());
    println!("{}", g);

    let from = *indexes.get(&1).unwrap();
    let to = *indexes.get(&max).unwrap();

    let ps = {
        let dfs = Dfs::new(g.as_ref());
        match dfs.run(from, to) {
            None => "No path found with DFS".to_string(),
            Some(ps) => ps
                .iter()
                .map(|&p| p.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        }
    };

    let pd = {
        let dijkstra = Dijkstra::new(g.as_ref());
        match dijkstra.run(from, to) {
            None => "No path found with Dijkstra".to_string(),
            Some(pd) => pd
                .iter()
                .map(|&p| p.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        }
    };

    println!("Paths from {} to {}", from, to);
    println!("DFS: {}", ps);
    println!("DIJKSTRA: {}", pd);
}
