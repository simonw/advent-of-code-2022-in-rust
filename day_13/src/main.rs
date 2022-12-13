use std::fs;

// A struct representing a nested list of lists of integers
#[derive(Debug, Clone)]
struct NestedInteger {
    // value can be blank or an integer
    value: Option<i32>,
    list: Vec<NestedInteger>,
}

fn main() {
    let file_contents = fs::read_to_string("example.txt").unwrap();
    let mut stack = Vec::new();
    for line in file_contents.lines() {
        // line is a JSON array e.g. [[1, 2], [3, 14]] - parse it
        let mut current = NestedInteger {
            value: None,
            list: Vec::new(),
        };
        let mut has_found_first = false;
        let mut current_integer = 0;
        let mut in_integer = false;
        for c in line.chars() {
            match c {
                '[' => {
                    // Start of a new list
                    if has_found_first {
                        stack.push(current);
                    }
                    has_found_first = true;
                    current = NestedInteger {
                        value: None,
                        list: Vec::new(),
                    };
                    current_integer = 0;
                    in_integer = false;
                }
                ']' => {
                    // End of a list
                    if in_integer {
                        current.list.push(NestedInteger {
                            value: Some(current_integer),
                            list: Vec::new(),
                        });
                    }
                    if let Some(mut parent) = stack.pop() {
                        parent.list.push(current.clone());
                        current = parent;
                    }
                    current_integer = 0;
                    in_integer = false;
                }
                ',' => {
                    // End of an integer
                    if in_integer {
                        current.list.push(NestedInteger {
                            value: Some(current_integer),
                            list: Vec::new(),
                        });
                        current_integer = 0;
                        in_integer = false;
                    }
                }
                '0'..='9' => {
                    // A digit
                    current_integer = current_integer * 10 + (c as i32 - '0' as i32);
                    in_integer = true;
                }
                _ => {}
            }
        }
        println!("{:?}", line);
        println!("{:#?}", current);
    }
}
