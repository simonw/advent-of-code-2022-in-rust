use std::fs;
/*
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
*/

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
