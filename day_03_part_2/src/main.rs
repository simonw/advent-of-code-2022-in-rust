use std::fs;
use std::io;
use std::io::BufRead;


fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut score: i32 = 0;

    // Read the file three lines at a time
    let mut lines = reader.lines();
    let mut found_char = 'X';

    loop {
        // Try this but break if it is an error:
        let line1 = match lines.next() {
            Some(Ok(line)) => line,
            _ => break,
        };
        let line2 = lines.next().unwrap()?;
        let line3 = lines.next().unwrap()?;

        // What character is present in all three lines?
        let mut found = false;
        for c1 in line1.chars() {
            for c2 in line2.chars() {
                for c3 in line3.chars() {
                    if c1 == c2 && c2 == c3 {
                        found = true;
                        found_char = c1;
                    }
                }
                if found {
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            let priority = match found_char {
                'a'..='z' => found_char as u8 - 'a' as u8 + 1,
                'A'..='Z' => found_char as u8 - 'A' as u8 + 27,
                _ => 0,
            } as i32;
            score += priority;
        } else {
            // Quit with error
            panic!("No character found that was in all 3 lines");
        }
    }
    println!("{}", score);
    Ok(())
}
