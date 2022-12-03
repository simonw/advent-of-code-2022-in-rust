use std::fs;
use std::io;
use std::io::BufRead;

fn find_duplicate_character(s1: &str, s2: &str) -> Option<char> {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                return Some(c1);
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut score: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        // Split the line exactly in half into two strings
        let (left, right) = line.split_at(line.len() / 2);

        let duplicate = find_duplicate_character(left, right).unwrap();

        /*
        Lowercase item types a through z have priorities 1 through 26.
        Uppercase item types A through Z have priorities 27 through 52.
        */
        let priority = match duplicate {
            'a'..='z' => duplicate as u8 - 'a' as u8 + 1,
            'A'..='Z' => duplicate as u8 - 'A' as u8 + 27,
            _ => 0,
        } as i32;
        println!("{}: {}", line, priority);
        score += priority;
    }
    println!("{}", score);
    Ok(())
}
