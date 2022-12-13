use std::fs;

// A struct representing a nested list of lists of integers
#[derive(Debug, Clone)]
struct NestedInteger {
    value: i32,
    list: Vec<NestedInteger>,
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut stack = Vec::new();
    for line in file_contents.lines() {
        // line is a JSON array e.g. [[1, 2], [3, 14]] - parse it
        let mut current = NestedInteger { value: 0, list: Vec::new() };
        for c in line.chars() {
            match c {
                '[' => {
                    // Start of a new list
                    stack.push(current);
                    current = NestedInteger { value: 0, list: Vec::new() };
                }
                ']' => {
                    // End of a list
                    let mut parent = stack.pop().unwrap();
                    parent.list.push(current);
                    current = parent;
                }
                ',' => {
                    // End of an integer
                    current.list.push(current.clone());
                    current = NestedInteger { value: 0, list: Vec::new() };
                }
                '0'..='9' => {
                    // A digit
                    current.value = current.value * 10 + (c as i32 - '0' as i32);
                }
                _ => {}
            }
        }
        println!("{:?}", line);
        println!("{:?}\n", current);
    }
}