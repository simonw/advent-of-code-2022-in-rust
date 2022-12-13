use parameterized::parameterized;
use std::fs;

// A struct representing a nested list of lists of integers
#[derive(Debug, Clone, Eq, PartialOrd)]
struct NestedInteger {
    // value can be blank or an integer
    value: Option<i32>,
    list: Vec<NestedInteger>,
}

impl PartialEq for NestedInteger {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.list == other.list
    }
}

fn main() {
    let file_contents = fs::read_to_string("example.txt").unwrap();
    for line in file_contents.lines() {
        let parsed = parse_line(line);
        println!("{:?}", line);
        println!("{:#?}", parsed);
    }
}

fn parse_line(line: &str) -> NestedInteger {
    // line is a JSON array e.g. [[1, 2], [3, 14]] - parse it
    let mut stack = Vec::new();
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
    current
}

#[test]
fn test_parse_line() {
    let line = "[[1, 2], [3, 14]]";
    let parsed = parse_line(line);
    assert_eq!(parsed.list.len(), 2);
    assert_eq!(parsed.list[0].list.len(), 2);
    assert_eq!(parsed.list[0].list[0].value, Some(1));
    assert_eq!(parsed.list[0].list[1].value, Some(2));
    assert_eq!(parsed.list[1].list.len(), 2);
    assert_eq!(parsed.list[1].list[0].value, Some(3));
    assert_eq!(parsed.list[1].list[1].value, Some(14));
}

#[test]
fn test_two_lines_equal() {
    let line = "[[1, 2], [3, 14]]";
    let parsed = parse_line(line);
    let parsed2 = parse_line(line);
    assert_eq!(parsed, parsed2);
}

#[parameterized(
    left = {"[1,1,3,1,1]", "[[1],[2,3,4]]", "[[8,7,6]]", "[[4,4],4,4]", "[7,7,7]", "[]", "[[]]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"},
    right = {"[1,1,5,1,1]", "[[1],4]", "[9]", "[[4,4],4,4,4]", "[7,7,7,7]", "[3]", "[[[]]]", "[1,[2,[3,[4,[5,6,7]]]],8,9]"}
)]
fn test_order_comparison(left: &str, right: &str) {
    // For each example, the one on the left should be < the one on the right
    let parsed_left = parse_line(left);
    let parsed_right = parse_line(right);
    assert_eq!(
        parsed_left < parsed_right,
        true,
        "{} should be < {}",
        left,
        right
    );
}
