use std::env;
use std::fs;
use std::io;
use std::io::BufRead;



fn main() -> Result<(), std::io::Error> {
    // Get the arguments as an iterator
    let args: Vec<String> = env::args().collect();

    // Ensure that at least one argument was passed
    if args.len() < 2 {
        println!("error: missing required argument 'filename'");
        return Ok(());
    }

    // Get the value of the filename argument
    let filename = &args[1];

    // Check if the multistack flag was passed
    let multistack = args.iter().any(|arg| arg == "--multistack" || arg == "-m");
    /*
                    [V]     [C]     [M]
    [V]     [J]     [N]     [H]     [V]
    [R] [F] [N]     [W]     [Z]     [N]
    [H] [R] [D]     [Q] [M] [L]     [B]
    [B] [C] [H] [V] [R] [C] [G]     [R]
    [G] [G] [F] [S] [D] [H] [B] [R] [S]
    [D] [N] [S] [D] [H] [G] [J] [J] [G]
    [W] [J] [L] [J] [S] [P] [F] [S] [L]
    1   2   3   4   5   6   7   8   9 
    */
    // Start by reading the file up to the blank line
    // and parsing it into a vector of vector-char stacks
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut stacks = Vec::new();
    // Populate with 9 empty stacks of chars
    for _ in 0..9 {
        stacks.push(Vec::new());
    }
    let mut line_strings: Vec<String> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        line_strings.push(line);
    }

    let mut line_index = 0;
    loop {
        let line = &line_strings[line_index];
        // Stop when line starts with " 1"
        if line.starts_with(" 1") {
            break;
        }
        // Add one space to the end of the line
        let line = format!("{} ", line);
        // Read it four characters at a time
        let mut stack_index = 0;
        for chunk in line.as_bytes().chunks(4) {
            // Look at second character in chunk
            let c = chunk[1] as char;
            if c != ' ' {
                stacks[stack_index].push(c);
            }
            stack_index += 1;
        }
        line_index += 1;
    }

    // Reverse the order of every stack
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    // debug print the stacks
    for stack in &stacks {
        println!("{:?}", stack);
    }
    // Next read the remaining lines and apply each one
    // to the stacks - skip 2 (the stack names and the blank line)
    for line in line_strings.iter().skip(line_index + 2) {
        // Format: "move 10 from 8 to 1"
        // Split on spaces, convert that to a vector
        let parts: Vec<&str> = line.split(" ").collect();
        // num_cards is 1st index, from_stack is 3rd, to_stack is 5th
        let num_cards: i32 = parts[1 as usize].parse().unwrap();
        let from_stack: i32 = parts[3 as usize].parse().unwrap();
        let to_stack: i32 = parts[5 as usize].parse().unwrap();

        println!("{} cards move from {} to {}", num_cards, from_stack, to_stack);
        println!(" from stack before: {:?}", stacks[(from_stack - 1) as usize]);
        println!(" to stack before: {:?}", stacks[(to_stack - 1) as usize]);

        // Move the cards
        if multistack {
            // Create vector for cards to move
            let mut cards_to_move = Vec::new();
            for _ in 0..num_cards {
                let card = stacks[from_stack as usize - 1].pop().unwrap();
                cards_to_move.push(card);
            }
            cards_to_move.reverse();
            // Move them
            for card in cards_to_move {
                stacks[to_stack as usize - 1].push(card);
            }
        } else {
            // Move cards one at a time
            for _ in 0..num_cards {
                let card = stacks[from_stack as usize - 1].pop().unwrap();
                stacks[to_stack as usize - 1].push(card);
            }
        }
        
        println!(" from stack after: {:?}", stacks[(from_stack - 1) as usize]);
        println!(" to stack after: {:?}", stacks[(to_stack - 1) as usize]);

    }
    // debug print the stacks
    for stack in &stacks {
        println!("{:?}", stack);
    }
    // Create string with top card of each stack
    let mut result = String::new();
    for stack in &stacks {
        if stack.len() > 0 {
            result.push(stack[stack.len() - 1]);
        }
    }
    println!("Result: {}", result);
    Ok(())
}
