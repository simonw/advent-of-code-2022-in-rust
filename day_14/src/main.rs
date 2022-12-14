// Coordinate system: 0,0 is top left, x increases to the right, y increases down

use std::fmt;

struct Grid {
    width: usize,
    height: usize,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    lines: Vec<Vec<char>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for line in &self.lines {
            for c in line {
                s.push(*c);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn main() {
    println!("Hello, world!");
}

fn parse_grid(input: String) -> Grid {
    // Each line describes a shape of rock drawn on the grid
    let mut min_x = 100000;
    let mut max_x = -100000;
    let mut min_y = 100000;
    let mut max_y = -100000;
    let mut grid_lines = Vec::new();
    // First loop to find min/max of everything:
    for line in input.lines() {
        // Line looks like this:
        // 498,4 -> 498,6 -> 496,6
        // First update min/max coordinates
        let mut coords = line.split(" -> ");
        while let Some(coord) = coords.next() {
            let mut coord = coord.split(",");
            let x = coord.next().unwrap().parse::<i32>().unwrap();
            let y = coord.next().unwrap().parse::<i32>().unwrap();
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }
    // Prepare the grid
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    for _ in 0..height {
        let mut line = Vec::new();
        for _ in 0..width {
            line.push('.');
        }
        grid_lines.push(line);
    }
    // Second loop to fill in the grid based on instructions
    for line in input.lines() {
        // 498,4 -> 498,6 -> 496,6
        let mut coords = line.split(" -> ");
        let mut done_first = false;
        let mut prev_coords = (0, 0);
        while let Some(coord) = coords.next() {
            // collect as (i32, i32) tuple:
            let mut bits = coord.split(",");
            let coord = (
                bits.next().unwrap().parse::<i32>().unwrap(),
                bits.next().unwrap().parse::<i32>().unwrap(),
            );
            if !done_first {
                done_first = true;
                prev_coords = coord;
                continue;
            }
            // println!("Drawing line from {:?} to {:?}", prev_coords, coord);

            if prev_coords.0 == coord.0 {
                // Vertical line
                let (y1, y2) = if prev_coords.1 < coord.1 {
                    (prev_coords.1, coord.1)
                } else {
                    (coord.1, prev_coords.1)
                };
                // println!("  Vertical line from {} to {}", y1, y2);
                for y in y1..=y2 {
                    grid_lines[(y - min_y) as usize][(prev_coords.0 - min_x) as usize] = '#';
                }
            } else {
                // Horizontal line
                let (x1, x2) = if prev_coords.0 < coord.0 {
                    (prev_coords.0, coord.0)
                } else {
                    (coord.0, prev_coords.0)
                };
                // println!("  Horizontal line from {} to {}", x1, x2);
                for x in x1..=x2 {
                    grid_lines[(prev_coords.1 - min_y) as usize][(x - min_x) as usize] = '#';
                }
            }
            /*
            println!("{}\n\n", Grid {
                width,
                height,
                min_x,
                max_x,
                min_y,
                max_y,
                lines: grid_lines.clone(),
            });
             */
            prev_coords = coord;
        }
    }
    Grid {
        width,
        height,
        min_x,
        max_x,
        min_y,
        max_y,
        lines: grid_lines,
    }
}

// Write a test
// Path: src/main.rs
#[test]
fn test_parse_grid() {
    let instructions = String::from("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9");
    let grid = parse_grid(instructions);
    assert_eq!(grid.width, 10);
    assert_eq!(grid.height, 6);
    assert_eq!(grid.min_x, 494);
    assert_eq!(grid.max_x, 503);
    assert_eq!(grid.min_y, 4);
    assert_eq!(grid.max_y, 9);

    let as_string = format!("{}", grid);
    println!("{}", as_string);
    assert_eq!(
        as_string,
        "
....#...##
....#...#.
..###...#.
........#.
........#.
#########.
".trim_start()
    );
}
