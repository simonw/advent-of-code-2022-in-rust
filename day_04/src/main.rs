use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), std::io::Error> {
    // Read file line by line
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut score = 0;
    for line in reader.lines() {
        let line = line?;
        // Line is e.g. 41-47,40-80 - parse into a-b,c-d
        // first use split_once('-')
        let (one, two) = line.split_once(',').unwrap();
        let (a, b) = one.split_once('-').unwrap();
        let (c, d) = two.split_once('-').unwrap();
        // Parse those into numbers
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();
        let c = c.parse::<u32>().unwrap();
        let d = d.parse::<u32>().unwrap();
        // Check if range a-b contains c-d or vice versa
        if (a <= c && c <= b) || (a <= d && d <= b) {
            println!("{}-{} fully overlaps {}-{}", a, b, c, d);
            score += 1;
        }
    }
    println!("Score: {}", score);
    Ok(())
}

