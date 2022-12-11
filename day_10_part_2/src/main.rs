use std::fs;

fn main() {
    let file_contents = fs::read_to_string("../day_10/input.txt").unwrap();

    /*
    File format:
    addx 4
    noop
    addx -1
    */
    let mut register_x = 1;
    let mut cycle = 0;

    const CAPTURE_AT: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut captured = Vec::new();

    for line in file_contents.lines() {
        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap();
        match command {
            "addx" => {
                // addx V takes two cycles to complete. After two cycles, the
                // X register is increased by the value V. (V can be negative.)
                let value = parts.next().unwrap().parse::<i32>().unwrap();
                cycle += 1;
                draw(cycle, register_x);
                if CAPTURE_AT.contains(&cycle) {
                    captured.push((cycle, register_x, cycle * register_x));
                }
                cycle += 1;
                draw(cycle, register_x);
                if CAPTURE_AT.contains(&cycle) {
                    captured.push((cycle, register_x, cycle * register_x));
                }
                register_x += value;
            }
            "noop" => {
                // takes one cycle to complete
                cycle += 1;
                draw(cycle, register_x);
                if CAPTURE_AT.contains(&cycle) {
                    captured.push((cycle, register_x, cycle * register_x));
                }
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}

fn draw(cycle: i32, register_x: i32) {
    let pos = (cycle - 1) % 40;
    // If register X within -1/0/1 of pos
    if (register_x - pos).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }
    if cycle % 40 == 0 {
        println!();
    }
}
