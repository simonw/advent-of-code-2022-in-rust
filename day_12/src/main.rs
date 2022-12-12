use std::fs;
/*
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
*/

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
    let file_contents = fs::read_to_string("example.txt").unwrap();
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
    println!("{:?}", grid);
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
    println!("{:?}", nodes);
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
                    // Find difference between n.char and node.char
                    let diff = (n.char as i32 - node.char as i32).abs();
                    if diff <= 1 {
                        println!("{} is connected to {}", node.char, n.char);
                        connections_to_add.push((node.idx, n.idx));
                    }
                }
                None => {}
            }
        }
        println!("Node: {:?}", node);
        println!(" . Left: {:?}", left_node);
        println!(" . Right: {:?}", right_node);
        println!(" . Up: {:?}", up_node);
        println!(" . Down: {:?}", down_node);
    }
    // Add all those connections
    for (node_idx, other_node_idx) in connections_to_add.iter() {
        nodes[*node_idx].connections.push(*other_node_idx);
    }

    println!("Nodes: {:?}", nodes);

    /*
    let mut graph: Graph<(), (), Directed> = Graph::new();

    let a = graph.add_node(()); // node with no weight
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    let e = graph.add_node(());
    let f = graph.add_node(());
    let g = graph.add_node(());
    let h = graph.add_node(());

    graph.extend_with_edges(&[
        (a, b),
        (b, c),
        (c, d),
        (d, a),
        (e, f),
        (b, e),
        (f, g),
        (g, h),
        (h, e),
    ]);
    // a ----> b ----> e ----> f
    // ^       |       ^       |
    // |       v       |       v
    // d <---- c       h <---- g
    let res = dijkstra(&graph, b, None, |_| 1);
    println!("{:?}", res);
     */
}
