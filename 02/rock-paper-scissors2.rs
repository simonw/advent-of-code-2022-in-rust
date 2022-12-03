use std::fs;
use std::io;
use std::io::BufRead;
fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line = line?;

        let mut parts = line.split(" ");
        let opponent_code = parts.next().unwrap();
        let outcome_code = parts.next().unwrap();
        
        /*
        A = rock
        B = paper
        C = scissors

        X = I lose
        Y = draw
        Z = I win
        */
        let opponent = match opponent_code {
            "A" => "rock",
            "B" => "paper",
            "C" => "scissors",
            _ => "unknown",
        };
        let outcome = match outcome_code {
            "X" => "lose",
            "Y" => "draw",
            "Z" => "win",
            _ => "unknown",
        };

        // Need to figure out what I picked, based on opponent + outcome
        let me = match (opponent, outcome) {
            ("rock", "win") => "paper",
            ("rock", "draw") => "rock",
            ("rock", "lose") => "scissors",
            ("paper", "win") => "scissors",
            ("paper", "draw") => "paper",
            ("paper", "lose") => "rock",
            ("scissors", "win") => "rock",
            ("scissors", "draw") => "scissors",
            ("scissors", "lose") => "paper",
            _ => "unknown",
        };
        
        let win_score = match outcome {
            "win" => 6,
            "draw" => 3,
            "lose" => 0,
            _ => 0,
        };

        // And an extra score based on what I picked
        let extra_score = match me {
            "rock" => 1,
            "paper" => 2,
            "scissors" => 3,
            _ => 0,
        };
        println!("{} {} {} {}", opponent, me, win_score, extra_score);
        score += extra_score;
    }
    println!("{}", score);
    Ok(())
}
