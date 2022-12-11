use regex::Regex;
use std::fs;

#[derive(Debug)]
enum Operand {
    // Can either be an integer or a 'Old'
    Int(i32),
    Old,
}
#[derive(Debug)]
enum Op {
    Plus,
    Times,
}

#[derive(Debug)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation_left: Operand,
    operation_right: Operand,
    operation_op: Op,
    test_divisible_by: i32,
    if_true_monkey: i32,
    if_false_monkey: i32,
}

fn main() {
    let mut monkeys = Vec::new();

    let file_contents = fs::read_to_string("example.txt").unwrap();

    let re = Regex::new(
        r"Monkey (\d+):\s*
  Starting items: (\d+(?:,\s*\d+)*)\s*
  Operation: new = old (\*|\+) (old|\d+)\s*
  Test: divisible by (\d+)\s*
    If true: throw to monkey (\d+)\s*
    If false: throw to monkey (\d+)",
    )
    .unwrap();

    // Use find_iter to find all matches for that regex
    // and then extract the data from each match
    for cap in re.captures_iter(&file_contents) {
        println!("Found match: {:?}", cap);
        let monkey_id = cap[1].parse::<i32>().unwrap();
        let items = cap[2]
            .split(", ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let operation_op = match &cap[3] {
            "+" => Op::Plus,
            "*" => Op::Times,
            _ => panic!("Unknown operation"),
        };
        let operation_left = Operand::Old;
        let mut operation_right = Operand::Old;
        println!("Dealing with {}", &cap[4]);
        match &cap[4] == "old" {
            true => operation_right = Operand::Old,
            false => operation_right = Operand::Int(cap[4].parse::<i32>().unwrap()),
        }
        let test_divisible_by = cap[5].parse::<i32>().unwrap();
        let if_true_monkey = cap[6].parse::<i32>().unwrap();
        let if_false_monkey = cap[7].parse::<i32>().unwrap();

        let monkey = Monkey {
            id: monkey_id,
            items,
            operation_left,
            operation_right,
            operation_op,
            test_divisible_by,
            if_true_monkey,
            if_false_monkey,
        };
        monkeys.push(monkey);
    }
    for monkey in &monkeys {
        println!("{:?}", monkey);
    }
}
