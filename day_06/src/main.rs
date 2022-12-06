use clap::Parser;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to read
    filename: String,

    /// Length of characters to seek
    #[arg(short, long, default_value_t = 4)]
    length: u16,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let length = args.length;

    // Read the first line of the file as a string
    let file = fs::File::open(args.filename)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let line = lines.next().unwrap()?;
    // Turn that into a vector of chars
    let chars: Vec<char> = line.chars().collect();

    for i in args.length..(chars.len() as u16) {
        // Put previous X characters in a set
        let mut set = std::collections::HashSet::new();
        for j in i - args.length..i {
            set.insert(chars[j as usize]);
        }
        // If the set has X characters, print last X
        if set.len() == args.length.into() {
            println!("Found it");
            let x_chars = chars
                .iter()
                .skip((i - (length - 1)).into())
                .take(length.into());
            println!("{}", x_chars.collect::<String>());
            println!("Index: {}", i);
            break;
        }
    }
    Ok(())
}
