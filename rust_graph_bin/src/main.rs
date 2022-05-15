use std::rc::Rc;

use rust_graph_lib::{
    adj_list::AdjListGraph,
    algorithms::{Algorithm, Dfs, Dijkstra},
    graph::Graph,
};

fn main() {
    let mut g = AdjListGraph::<u32>::new();
    let max = 10;

    for i in 1u32..=max {
        let i = i.saturating_sub(3);
        let i2 = i + 3;

        for j in i..=i2 {
            if j >= 1 && j <= max && i != j && i % 3 == j % 3 {
                g.add_edge(i, j);
            }
        }
    }

    let g = Rc::new(g);

    println!("Created graph!");
    println!("Node count: {}", g.node_count());
    println!("Edge count: {}", g.edge_count());
    println!("{}", g);

    let from = 1;
    let to = max;

    let ps = {
        let dfs = Dfs::new(g.as_ref());
        match dfs.run(&from, &to) {
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
        match dijkstra.run(&from, &to) {
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
