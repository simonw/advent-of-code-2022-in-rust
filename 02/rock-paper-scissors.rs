use std::fs;
use std::io;
use std::io::BufRead;
fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line = line?;

        // Line is of form "opponent me" - split on the space
        let mut parts = line.split(" ");
        let opponent = parts.next().unwrap();
        let me = parts.next().unwrap();
        
        /*
        A/X = rock
        B/Y = paper
        C/Z = scissors
        */

        // Who won? Opponent A beats me Z, B beats me X, C beats me Y
        let win_score = match (opponent, me) {
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            _ => 3,
        };
        score += win_score;
        // And an extra score based on what I picked
        // X = 1, Y = 2, Z = 3 - use match for that too:
        let extra_score = match me {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };
        println!("{} {} {} {}", opponent, me, win_score, extra_score);
        score += extra_score;
    }
    println!("{}", score);
    Ok(())
}
