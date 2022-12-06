use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("error: missing required argument 'filename'");
        return Ok(());
    }
    let filename = &args[1];

    // Read the first line of the file as a string
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let line = lines.next().unwrap()?;
    // Turn that into a vector of chars
    let chars: Vec<char> = line.chars().collect();

    for i in 4..chars.len() {
        // Put previous four characters in a set
        let mut set = std::collections::HashSet::new();
        for j in i - 4..i {
            set.insert(chars[j]);
        }
        // If the set has 4 characters, print last 4
        if set.len() == 4 {
            println!(
                "{}{}{}{}",
                chars[i - 3],
                chars[i - 2],
                chars[i - 1],
                chars[i]
            );
            println!("{}", i);
            break;
        }
    }

    Ok(())
}
