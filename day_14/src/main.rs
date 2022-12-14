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

// Add Grid simulate_sand() method:
impl Grid {
    fn char_at(&self, coord: (i32, i32)) -> char {
        // Subtract min_x / min_y
        // println!("char_at: {} {}", coord.0, coord.1);
        let x = coord.0 - self.min_x;
        let y = coord.1 - self.min_y;
        let ch;
        // If out of bounds, return ' '
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            ch = ' ';
        } else {
            ch = self.lines[y as usize][x as usize];
        }
        // println!("  translated to: {} {} - result is: '{}'", x, y, ch);
        ch
    }
    fn set_char_at(&mut self, coord: (i32, i32), ch: char) {
        // Add min_x / min_y
        // println!("Setting char {} at {:?}", ch, coord);
        let x = coord.0 - self.min_x;
        let y = coord.1 - self.min_y;
        // println!("  translated to: {} {}", x, y);
        self.lines[y as usize][x as usize] = ch;
    }
    fn simulate_sand(&mut self) -> bool {
        // Returns true if sand was placed, false if it fell into void
        // Sand is added at 500,0
        let mut sand_pos = (500, 0);
        loop {
            // Inspect all three spaces below the sand
            let below = (sand_pos.0, sand_pos.1 + 1);
            let below_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            let below_right = (sand_pos.0 + 1, sand_pos.1 + 1);
            if self.char_at(below) == '.' || self.char_at(below) == ' ' {
                sand_pos = below;
                if self.char_at(sand_pos) == ' ' {
                    return false;
                }
                continue;
            } else if self.char_at(below_left) == '.'  || self.char_at(below_left) == ' ' {
                sand_pos = below_left;
                if self.char_at(sand_pos) == ' ' {
                    return false;
                }
                continue;
            } else if self.char_at(below_right) == '.' || self.char_at(below_right) == ' ' {
                sand_pos = below_right;
                if self.char_at(sand_pos) == ' ' {
                    return false;
                }
                continue;
            } else {
                // Sand lives here now
                self.set_char_at(sand_pos, 'o');
                break;
            }
        }
        true
    }
}


fn main() {
    let instructions = String::from(include_str!("../input.txt"));
    let mut grid = parse_grid(instructions);
    println!("{}", grid);
    // Simulate sand until it falls into the void
    let mut i = 0;
    while grid.simulate_sand() {
        i += 1;
        println!("After {} iterations:", i);
        println!("{}\n\n", grid);
    }
    println!("Final i: {}", i);
}


fn parse_grid(input: String) -> Grid {
    // Each line describes a shape of rock drawn on the grid
    let mut min_x = 100000;
    let mut max_x = -100000;
    let mut min_y = 0;
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
    let mut grid = parse_grid(instructions);
    assert_eq!(grid.width, 10);
    assert_eq!(grid.height, 10);
    assert_eq!(grid.min_x, 494);
    assert_eq!(grid.max_x, 503);
    assert_eq!(grid.min_y, 0);
    assert_eq!(grid.max_y, 9);

    let as_string = format!("{}", grid);
    println!("{}", as_string);
    assert_eq!(
        as_string,
        "
..........
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.
".trim_start()
    );
    grid.simulate_sand();
    println!("{}", grid);
}
