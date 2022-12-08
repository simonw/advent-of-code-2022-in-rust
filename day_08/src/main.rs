use std::fs;

#[derive(Debug)]
struct Map {
    lines: Vec<String>
}

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

        // TODO: Fix this incorrect logic:

        // Visible from top?
        for i in 0..y {
            let char = self.get(x, i);
            if char > current_char {
                visible_from_top = false;
                break;
            }
        }
        // Visible from bottom?
        for i in y+1..self.lines.len() {
            let char = self.get(x, i);
            if char > current_char {
                visible_from_bottom = false;
                break;
            }
        }
        // Visible from left?
        for i in 0..x {
            let char = self.get(i, y);
            if char > current_char {
                visible_from_left = false;
                break;
            }
        }
        // Visible from right?
        for i in x+1..self.lines[y].len() {
            let char = self.get(i, y);
            if char > current_char {
                visible_from_right = false;
                break;
            }
        }
        visible_from_top || visible_from_bottom || visible_from_left || visible_from_right
    }
}

fn main() {
    let file_contents = fs::read_to_string("example.txt").unwrap();
    let mut map = Map {
        lines: Vec::new(),
    };
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
}
