use rust_graph_lib::{
    adj_matrix::AdjMatrixGraph,
    algorithms::{dfs, dijkstra},
    graph::Graph,
};

fn main() {
    let mut g = AdjMatrixGraph::<u32>::new();
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

    println!("Created graph!");
    println!("Node count: {}", g.node_count());
    println!("Edge count: {}", g.edge_count());
    println!("{}", g);

    let from = 1;
    let to = max;

    let ps = match dfs(&g, &from, &to) {
        None => "No path found with DFS".to_string(),
        Some(ps) => ps
            .iter()
            .map(|&p| p.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    };

    let pd = match dijkstra(&g, &from, &to) {
        None => "No path found with Dijkstra".to_string(),
        Some(pd) => pd
            .iter()
            .map(|&p| p.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    };

    println!("Paths from {} to {}", from, to);
    println!("DFS: {}", ps);
    println!("DIJKSTRA: {}", pd);
}
