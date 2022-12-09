use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);
    let mut tail_visited = Vec::new();
    tail_visited.push(tail_pos);

    print_board(&head_pos, &tail_pos);

    for line in file_contents.lines() {
        // Line is "R 4" - split into move, distance (integer)
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<i32>().unwrap();
        println!("direction = {} distance = {}", direction, distance);
        // First move the head
        for _ in 0..distance {
            match direction {
                "R" => head_pos.0 += 1,
                "L" => head_pos.0 -= 1,
                "U" => head_pos.1 += 1,
                "D" => head_pos.1 -= 1,
                _ => panic!("Unknown direction {}", direction),
            }
            println!("  head_pos = {:?}", head_pos);
            // Now move the tail, if it needs to be moved
            // Is tail within one of head in either direction?
            if (head_pos.0 == tail_pos.0 && (head_pos.1 - tail_pos.1).abs() == 1)
                || (head_pos.1 == tail_pos.1 && (head_pos.0 - tail_pos.0).abs() == 1)
            {
                println!("  Touching, no need to move");
            }
            // Are tail and head diagonally touching?
            else if (head_pos.0 - tail_pos.0).abs() == 1 && (head_pos.1 - tail_pos.1).abs() == 1 {
                println!("  Diagonally touching, no need to move");
            } else {
                // Need to move the tail
                // If tail is not in same row AND column as head...
                if head_pos.0 != tail_pos.0 && head_pos.1 != tail_pos.1 {
                    // Move it diagonally in the right direction
                    tail_pos.0 += to_one(head_pos.0 - tail_pos.0);
                    tail_pos.1 += to_one(head_pos.1 - tail_pos.1);
                } else {
                    // Move it in the right direction
                    if head_pos.0 == tail_pos.0 {
                        tail_pos.1 += to_one(head_pos.1 - tail_pos.1);
                    } else {
                        tail_pos.0 += to_one(head_pos.0 - tail_pos.0);
                    }
                }
                println!("  tail_pos = {:?}", tail_pos);
                tail_visited.push(tail_pos);
            }
            print_board(&head_pos, &tail_pos);
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

fn print_board(head_pos: &(i32, i32), tail_pos: &(i32, i32)) {
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


fn to_one(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}
