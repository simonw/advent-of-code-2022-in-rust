use parameterized::parameterized;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
enum NestedInteger {
    Value(i32),
    Children(Vec<NestedInteger>),
}

impl Ord for NestedInteger {
    fn cmp(&self, other: &Self) -> Ordering {
        println!("Comparing {:?} and {:?}", self, other);
        match (self, other) {
            (NestedInteger::Value(a), NestedInteger::Value(b)) => a.cmp(b),
            // If one side is a value and the other a list, compare to [value] - in both directions:
            (NestedInteger::Value(_), _) => {
                NestedInteger::Children(vec![self.clone()]).cmp(other)
            }
            (_, NestedInteger::Value(_)) => {
                self.cmp(&NestedInteger::Children(vec![other.clone()]))
            }
            // If both sides are lists, compare them one element at a time
            (NestedInteger::Children(a), NestedInteger::Children(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    println!("  a: {:?}, b: {:?}", a, b);
                    match (a.next(), b.next()) {
                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => continue,
                            other => return other,
                        },
                        // If the right list runs out of items first, the inputs are not in the right order
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
        }
    }
}

impl PartialOrd for NestedInteger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    for line in file_contents.lines() {
        let parsed = parse_line(line);
        println!("{:?}", line);
        println!("{:#?}", parsed);
    }
    // Determine which pairs of packets are already in the right order.
    // What is the sum of the indices of those pairs?
    // First, do pairs
    let mut left = None;
    let mut right;
    let mut indexes = Vec::new();
    let mut index = 1; // Indexes are 1-based
    for line in file_contents.lines() {
        // Skip blank lines
        if line.is_empty() {
            continue;
        }
        let parsed = parse_line(line);
        if left.is_none() {
            left = Some(parsed);
            continue;
        } else {
            right = Some(parsed);
            // Is this pair in the right order?
            if left.unwrap() < right.unwrap() {
                indexes.push(index);
            }
            left = None;
            index += 1;
        }
    }
    println!("Indexes: {:?}", indexes);
    println!("Sum: {}", indexes.iter().sum::<i32>());
    println!("Last index: {}", index);
}

fn parse_line(line: &str) -> NestedInteger {
    // line is a JSON array e.g. [[1, 2], [3, 14]] - parse it
    let mut chars = line.chars();
    let mut current = NestedInteger::Children(Vec::new());
    let mut stack = Vec::new();
    let mut current_integer = 0;
    let mut in_integer = false;
    let mut has_found_first = false;
    while let Some(c) = chars.next() {
        match c {
            '[' => {
                // Start of a new list
                if has_found_first {
                    stack.push(current);
                }
                has_found_first = true;
                current = NestedInteger::Children(Vec::new());
                current_integer = 0;
                in_integer = false;
            }
            ']' => {
                // End of a list
                if in_integer {
                    // Push onto Children list in current
                    match &mut current {
                        NestedInteger::Children(children) => {
                            children.push(NestedInteger::Value(current_integer));
                        }
                        _ => panic!("Unexpected type"),
                    }
                }
                if let Some(mut parent) = stack.pop() {
                    match &mut parent {
                        NestedInteger::Children(children) => {
                            children.push(current.clone());
                        }
                        _ => panic!("Unexpected type"),
                    }
                    current = parent;
                }
                current_integer = 0;
                in_integer = false;
            }
            ',' => {
                // End of an integer
                if in_integer {
                    match &mut current {
                        NestedInteger::Children(children) => {
                            children.push(NestedInteger::Value(current_integer));
                        }
                        _ => panic!("Unexpected type"),
                    }
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

/*

fn old_parse_ine(line: &str) -> NestedInteger {
    let mut stack = Vec::new();
    let mut current;
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
                current = NestedInteger::Children(Vec::new());
                current_integer = 0;
                in_integer = false;
            }
            ']' => {
                // End of a list
                if in_integer {
                    // Push onto Children list in current
                    match &mut current {
                        NestedInteger::Children(children) => {
                            children.push(NestedInteger::Value(current_integer));
                        }
                        _ => panic!("Unexpected type"),
                    }
                }
                if let Some(mut parent) = stack.pop() {
                    parent.push(current.clone());
                    current = parent;
                }
                current_integer = 0;
                in_integer = false;
            }
            ',' => {
                // End of an integer
                if in_integer {
                    current.list.push(NestedInteger::Value(current_integer));
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
*/

#[test]
fn test_parse_line() {
    let line = "[[1, 2], [3, 14]]";
    let parsed = parse_line(line);
    assert_eq!(
        parsed,
        NestedInteger::Children(vec![
            NestedInteger::Children(vec![NestedInteger::Value(1), NestedInteger::Value(2)]),
            NestedInteger::Children(vec![NestedInteger::Value(3), NestedInteger::Value(14)])
        ])
    );
}

#[test]
fn test_two_lines_equal() {
    let line = "[[1, 2], [3, 14]]";
    let parsed = parse_line(line);
    let parsed2 = parse_line(line);
    assert_eq!(parsed, parsed2);
}

#[parameterized(
    left = {"[1,1,3,1,1]", "[[1],[2,3,4]]", "[[8,7,6]]", "[[4,4],4,4]", "[7,7,7]", "[]", "[[]]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", "[[0,7,7,[1,[3,8,10,8],[3],[2,2]],10],[[0,5,[1,2],[0],[5,7,5]],1,4,[2]],[]]", "[[0]]"},
    right = {"[1,1,5,1,1]", "[[1],4]", "[9]", "[[4,4],4,4,4]", "[7,7,7,7]", "[3]", "[[[]]]", "[1,[2,[3,[4,[5,6,7]]]],8,9]", "[[[2,[]]],[],[9,1]]", "[[[2]]]"}
)]
fn test_order_comparison(left: &str, right: &str) {
    // For each example, the one on the left should be < the one on the right
    let parsed_left = parse_line(left);
    let parsed_right = parse_line(right);
    println!("Left: {:?}", parsed_left);
    println!("Right: {:?}", parsed_right);

    assert_eq!(
        parsed_left < parsed_right,
        true,
        "{} should be < {}",
        left,
        right
    );
}
