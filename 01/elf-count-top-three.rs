use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    // Create a new, empty vector of integers.
    let mut elves = Vec::new();

    let mut current = 0;

    for line in reader.lines() {
        let line = line?;

        // If line is empty, save the elf
        if line.is_empty() {
            elves.push(current);
            current = 0;
        }

        match line.parse::<i32>() {
            Ok(n) => {
                current += n;
            },
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
    }

    // Sort the elves vector in descending order
    elves.sort_by(|a, b| b.cmp(a));

    // New vector with just the top three:
    let top_three = &elves[0..3];

    // Print out the top three
    let mut sum = 0;
    for elf in top_three {
        println!("{}", elf);
        sum += elf;
    }

    // Print out the sum of the top three
    println!("Sum: {}", sum);

    Ok(())
}
