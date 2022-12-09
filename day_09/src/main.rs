use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let mut knots: Vec<(i32, i32)> = Vec::new();
    for _ in 0..10 {
        knots.push((0, 0));
    }
    let mut tail_visited: Vec<(i32, i32)> = Vec::new();
    tail_visited.push((0, 0));

    // print_board(&knots);

    for line in file_contents.lines() {
        // Line is "R 4" - split into move, distance (integer)
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<i32>().unwrap();
        println!("direction = {} distance = {}", direction, distance);
        // First move the head
        for _ in 0..distance {
            match direction {
                "R" => knots[0].0 += 1,
                "L" => knots[0].0 -= 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => panic!("Unknown direction {}", direction),
            }
            println!("  head_pos = {:?}", knots[0]);
            // Now move each other knot in turn, if it needs to be moved
            for i in 1..knots.len() {
                // Is knot within one of head in either direction?
                let mut knot = knots[i];
                let prev = knots[i - 1];
                if (prev.0 == knot.0 && (prev.1 - knot.1).abs() == 1)
                    || (prev.1 == knot.1 && (prev.0 - knot.0).abs() == 1)
                {
                    println!("  Touching, no need to move");
                }
                // Are tail and head diagonally touching?
                else if (prev.0 - knot.0).abs() == 1 && (prev.1 - knot.1).abs() == 1 {
                    println!("  Diagonally touching, no need to move");
                } else {
                    // Need to move the tail
                    // If tail is not in same row AND column as head...
                    if prev.0 != knot.0 && prev.1 != knot.1 {
                        // Move it diagonally in the right direction
                        knot.0 += to_one(prev.0 - knot.0);
                        knot.1 += to_one(prev.1 - knot.1);
                    } else {
                        // Move it in the right direction
                        if prev.0 == knot.0 {
                            knot.1 += to_one(prev.1 - knot.1);
                        } else {
                            knot.0 += to_one(prev.0 - knot.0);
                        }
                    }
                    knots[i] = knot;
                    println!("  knot = {:?}", knot);
                    if i == 9 {
                        tail_visited.push(knot);
                    }
                }
            }
            // print_board(&head_pos, &tail_pos);
        }
    }
    println!("tail_visited = {:?}", tail_visited);
    // Count unique tuples in that
    let mut unique_tuples = Vec::new();
    for tuple in tail_visited {
        if !unique_tuples.contains(&tuple) {
            unique_tuples.push(tuple);
        }
    }
    println!("Number of unique locations = {:?}", unique_tuples.len());
}
/*
fn print_board(knots: &knot) {
    for y in 0..10 {
        for x in 0..10 {
            // If both in same spot do an X
            if x == head_pos.0 && y == head_pos.1 && x == tail_pos.0 && y == tail_pos.1 {
                print!("X");
            } else if x == head_pos.0 && y == head_pos.1 {
                print!("H");
            } else if x == tail_pos.0 && y == tail_pos.1 {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
 */


fn to_one(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}
