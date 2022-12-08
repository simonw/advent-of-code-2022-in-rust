use std::fmt;
use std::fs;

#[derive(Debug)]
struct Map {
    lines: Vec<String>,
}

struct ScenicScore {
    x: usize,
    y: usize,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    score: usize,
}

impl fmt::Debug for ScenicScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}) top: {}, bottom: {}, left: {}, right: {}, score: {}",
            self.x, self.y, self.top, self.bottom, self.left, self.right, self.score
        )
    }
}
// An explanation of:
//    &mut fmt::Formatter<'_>
// in as much detail as possible:
//
// &mut fmt::Formatter<'_> is a reference to a mutable Formatter object
// that has a lifetime of '_.
//
// The lifetime of a reference is the scope in which it is valid.
// _ means:
//    "I don't care what the lifetime is, just make it compile"

// Method that returns x,y coordinate
impl Map {
    // x is along the line, y is down the lines
    fn get(&self, x: usize, y: usize) -> char {
        self.lines[y].chars().nth(x).unwrap()
    }
    fn item_is_visible(&self, x: usize, y: usize) -> bool {
        // Items on the edges are always visible
        if x == 0 || x == self.lines[0].len() - 1 || y == 0 || y == self.lines.len() - 1 {
            return true;
        }
        let current_char = self.get(x, y);
        let mut visible_from_top = true;
        let mut visible_from_bottom = true;
        let mut visible_from_left = true;
        let mut visible_from_right = true;

        // Visible from top?
        for i in 0..y {
            let char = self.get(x, i);
            if char >= current_char {
                visible_from_top = false;
                break;
            }
        }
        // Visible from bottom?
        for i in y + 1..self.lines.len() {
            let char = self.get(x, i);
            if char >= current_char {
                visible_from_bottom = false;
                break;
            }
        }
        // Visible from left?
        for i in 0..x {
            let char = self.get(i, y);
            if char >= current_char {
                visible_from_left = false;
                break;
            }
        }
        // Visible from right?
        for i in x + 1..self.lines[y].len() {
            let char = self.get(i, y);
            if char >= current_char {
                visible_from_right = false;
                break;
            }
        }
        visible_from_top || visible_from_bottom || visible_from_left || visible_from_right
    }
    // Returns tuple of four integers
    fn viewing_distances(&self, x: usize, y: usize) -> (usize, usize, usize, usize) {
        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;
        let current_char = self.get(x, y);
        // Top: look from current position, in reverse direction
        for i in (0..y).rev() {
            let char = self.get(x, i);
            if char >= current_char {
                break;
            }
            top += 1;
        }
        // Bottom
        for i in y + 1..self.lines.len() {
            let char = self.get(x, i);
            if char >= current_char {
                break;
            }
            bottom += 1;
        }
        // Left - look from current position, in reverse direction
        for i in (0..x).rev() {
            let char = self.get(i, y);
            if char >= current_char {
                break;
            }
            left += 1;
        }
        // Right
        for i in x + 1..self.lines[y].len() {
            let char = self.get(i, y);
            if char >= current_char {
                break;
            }
            right += 1;
        }
        (top, bottom, left, right)
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut map = Map { lines: Vec::new() };
    for line in file_contents.lines() {
        map.lines.push(line.to_string());
    }
    let mut visible_count = 0;
    for y in 0..map.lines.len() {
        for x in 0..map.lines[y].len() {
            let is_visible = map.item_is_visible(x, y);
            if is_visible {
                visible_count += 1;
                print!("X");
            } else {
                print!(" ");
            }
        }
        // Output the line too
        println!("    {}", map.lines[y]);
    }
    println!("\nVisible count: {}", visible_count);

    println!("\nPart 2\n======\n\n");

    let mut scenic_scores = Vec::new();
    for y in 0..map.lines.len() {
        for x in 0..map.lines[y].len() {
            let (top, bottom, left, right) = map.viewing_distances(x, y);
            let score = top * bottom * left * right;
            scenic_scores.push(ScenicScore {
                x,
                y,
                top,
                bottom,
                left,
                right,
                score,
            });
            // Print centered within 8 characters
            print!("{:8}", score);
        }
        println!();
    }
    let max_score = scenic_scores
        .iter()
        .max_by(|a, b| a.score.cmp(&b.score))
        .unwrap();
    println!("Max score: {:#?}", max_score);
}
