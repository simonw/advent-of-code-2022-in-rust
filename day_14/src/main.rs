

struct Grid {
    width: usize,
    height: usize,
    lines: Vec<Vec<char>>,
}

fn main() {
    println!("Hello, world!");
}

fn parse_grid(input: String) -> Grid {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();
    Grid { width, height, lines }
}

// Write a test
// Path: src/main.rs
#[test]
fn test_parse_grid() {
    let instructions = String::from("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9");
    let grid = parse_grid(instructions);
    assert_eq!(grid.width, 23);
    assert_eq!(grid.height, 2);
    assert_eq!(grid.lines, vec![
        vec!['4', '9', '8', ',', '4', ' ', '-', '>', ' ', '4', '9', '8', ',', '6', ' ', '-', '>', ' ', '4', '9', '6', ',', '6'],
        vec!['5', '0', '3', ',', '4', ' ', '-', '>', ' ', '5', '0', '2', ',', '4', ' ', '-', '>', ' ', '5', '0', '2', ',', '9', ' ', '-', '>', ' ', '4', '9', '4', ',', '9']
    ]);
}
