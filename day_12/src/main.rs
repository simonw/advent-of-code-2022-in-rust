use std::fs;

use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;

#[derive(Debug, Clone)]
struct Node {
    idx: usize,
    char: char,
    x: i32,
    y: i32,
    // Vector of indexes in the `nodes` vector of connected nodes
    connections: Vec<usize>,
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    // Create grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_coord = (0, 0);
    let mut end_coord = (0, 0);
    let mut line_number = 0;
    for line in file_contents.lines() {
        let mut chars: Vec<char> = line.chars().collect();
        // Look for S and replace it with an 'a'
        let index_of_capital_s = match chars.iter().position(|&c| c == 'S') {
            Some(i) => i as i32,
            None => -1,
        };
        if index_of_capital_s != -1 {
            start_coord = (line_number, index_of_capital_s);
            chars[index_of_capital_s as usize] = 'a';
        }
        // Look for E and replace it with a 'z'
        let index_of_capital_e = match chars.iter().position(|&c| c == 'E') {
            Some(i) => i as i32,
            None => -1,
        };
        if index_of_capital_e != -1 {
            end_coord = (line_number, index_of_capital_e);
            chars[index_of_capital_e as usize] = 'z';
        }
        grid.push(chars);
        line_number += 1;
    }
    println!("Start coord: {:?}", start_coord);
    println!("End coord: {:?}", end_coord);
    // Now figure out which nodes are connected to which

    let mut nodes: Vec<Node> = Vec::new();
    let mut node_idx = 0;
    // Iterate through grid getting back an index each time
    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            let node = Node {
                idx: node_idx as usize,
                char: *char,
                x: x as i32,
                y: y as i32,
                connections: Vec::new(),
            };
            nodes.push(node);
            node_idx += 1;
        }
    }
    // Now populate connections for each node
    let mut connections_to_add = Vec::new();
    let nodes_clone = nodes.clone();
    for node in nodes_clone.iter() {
        let left_node = nodes_clone
            .iter()
            .find(|n| n.x == node.x - 1 && n.y == node.y);
        let right_node = nodes_clone
            .iter()
            .find(|n| n.x == node.x + 1 && n.y == node.y);
        let up_node = nodes_clone
            .iter()
            .find(|n| n.x == node.x && n.y == node.y - 1);
        let down_node = nodes_clone
            .iter()
            .find(|n| n.x == node.x && n.y == node.y + 1);
        for other_node in vec![left_node, right_node, up_node, down_node] {
            match other_node {
                Some(n) => {
                    // Is other_node no more than 1 higher than node?
                    let diff = n.char as i32 - node.char as i32;
                    println!(
                        " . node: {}, other_node: {}, diff: {}",
                        node.char, n.char, diff
                    );
                    if diff <= 1 {
                        connections_to_add.push((node.idx, n.idx));
                    }
                }
                None => {}
            }
        }
    }
    // Add all those connections
    for (node_idx, other_node_idx) in connections_to_add.iter() {
        nodes[*node_idx].connections.push(*other_node_idx);
    }

    // println!("{:#?}", nodes);

    // Build a graph
    let mut graph: Graph<(), (), Directed> = Graph::new();
    let mut graph_nodes = Vec::new();

    // Add a blank node to the graph for every node
    for _ in nodes.iter() {
        let graph_node = graph.add_node(());
        graph_nodes.push(graph_node);
    }

    // Populate with connections
    for node in nodes.iter() {
        for other_node_idx in node.connections.iter() {
            graph.add_edge(graph_nodes[node.idx], graph_nodes[*other_node_idx], ());
        }
    }

    // println!("{:?}", graph);

    // Run dijkstra's algorithm
    let start_node_index = nodes
        .iter()
        .position(|n| n.x == start_coord.1 && n.y == start_coord.0)
        .expect("Couldn't find start node");
    let end_node_index = nodes
        .iter()
        .position(|n| n.x == end_coord.1 && n.y == end_coord.0)
        .expect("Couldn't find end node");

    let start_graph_node = graph_nodes[start_node_index];
    let end_graph_node = graph_nodes[end_node_index];

    let shortest_path = dijkstra(&graph, start_graph_node, Some(end_graph_node), |_e| 1);

    println!("\nShortest path: {:?}", shortest_path);
    println!("");
    println!("end graph node: {:?}", end_graph_node);
    println!("");
    println!("Shortest path length: {}", shortest_path[&end_graph_node]);
}
