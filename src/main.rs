#[allow(dead_code)]
mod adj_matrix;
#[allow(dead_code)]
mod graph;
#[allow(dead_code)]
mod utils;

use crate::adj_matrix::AdjMatrixGraph;
use crate::graph::Graph;

fn main() {
    let mut g = AdjMatrixGraph::<i32>::new();
    let max = 100;

    for i in 1..=max {
        for j in i - 3..=i + 3 {
            if j >= 1 && j <= max {
                g.add_edge(i, j);
            }
        }
    }

    println!("Created graph!");
    println!("Node count: {}", g.node_count());
    println!("Edge count: {}", g.edge_count());

    let from = 1;
    let to = max;
    let path = utils::dfs(&g, &from, &to);

    let ps = path
        .iter()
        .map(|&p| p.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!("Path from {} to {}", from, to);
    println!("{}", ps);
}
