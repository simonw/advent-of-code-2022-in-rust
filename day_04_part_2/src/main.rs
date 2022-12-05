use std::fs;
use std::io;
use std::io::BufRead;

fn range_overlaps_range(range: (u32, u32), other: (u32, u32)) -> bool {
    range.0 <= other.0 && other.0 <= range.1
        || range.0 <= other.1 && other.1 <= range.1
        || other.0 <= range.0 && range.0 <= other.1
        || other.0 <= range.1 && range.1 <= other.1
}

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

        let range1 = (a, b);
        let range2 = (c, d);

        if range_overlaps_range(range1, range2) {
            score += 1;
        }
    }
    println!("Score: {}", score);
    Ok(())
}
