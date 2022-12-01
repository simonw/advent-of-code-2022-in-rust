// Import the 'fs' and 'io' modules from the standard library.
// The 'fs' module provides functions for working with files and directories,
// and the 'io' module provides functions for working with input and output.
use std::fs;
use std::io;

// Import the 'BufRead' trait from the 'io' module.
// The 'BufRead' trait provides methods for reading data from a source
// in a buffered manner.
use std::io::BufRead;

// Define the 'main' function.
// This is the entry point of the program.
// The 'main' function returns a 'Result<(), io::Error>', which is
// an enumeration that represents either a successful value of type '()'
// or an error value of type 'io::Error'.
fn main() -> io::Result<()> {
    // Open the file 'numbers.txt' in read-only mode and bind the result
    // to the 'file' variable. The 'File::open' method returns a 'Result'
    // that contains either the opened file or an error value.
    // The '?' operator is used to propagate any errors that occur.
    let file = fs::File::open("input.txt")?;

    // Create a 'BufReader' for the opened file and bind the result
    // to the 'reader' variable. The 'BufReader' type provides buffered
    // reading capabilities for the file.
    let reader = io::BufReader::new(file);

    // Initialize two integer variables - current and max
    let mut current = 0;
    let mut max = 0;

    // Iterate over each line in the 'reader'.
    // The 'lines' method returns an iterator over the lines in the 'reader'.
    for line in reader.lines() {
        // Bind the current line to the 'line' variable.
        // The 'line' variable is of type 'io::Result<String>', which is
        // an enumeration that represents either a successful value of type
        // 'String' or an error value of type 'io::Error'.
        // The '?' operator is used to propagate any errors that occur.
        let line = line?;

        // If line is empty, figure out if we have a new max
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        }

        // Parse the line as an 'i32' (32-bit signed integer) and bind the
        // result to the 'number' variable.

        // If an error occurs, print it and continue
        // Start that code here:   
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
    // Print the parsed number to the console.
    // The 'println!` macro is used to print a formatted string to the
    // console. In this case, the format string is "{}", which means
    // that the value of the 'max' variable will be printed in place
    // of the "{}" placeholder.
    println!("Max: {}", max);

    // Return an 'Ok' value containing '()'.
    // This indicates that the 'main' function has completed successfully.
    Ok(())
}
