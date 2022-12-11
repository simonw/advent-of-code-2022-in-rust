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

// Method on Monkey called "x"
impl Monkey {
    fn push_item(&mut self, item: i32) {
        self.items.push(item);
    }
    fn take_turn(&self, monkeys: &mut Vec<Monkey>) {
        // Iterate over the item indexes
        for i in 0..self.items.len() {
            let (target_monkey_idx, result) = self.process_item(i);
            monkeys[target_monkey_idx].push_item(result);
        }
    }

    fn process_item(&self, item_idx: usize) -> (usize, i32) {
        let mut result = 0;
        let old = self.items[item_idx as usize];
        // First the monkey applies the operation to the item
        match self.operation_op {
            Op::Plus => {
                result = match self.operation_left {
                    Operand::Int(i) => i,
                    Operand::Old => old,
                } + match self.operation_right {
                    Operand::Int(i) => i,
                    Operand::Old => old,
                };
            }
            Op::Times => {
                result = match self.operation_left {
                    Operand::Int(i) => i,
                    Operand::Old => old,
                } * match self.operation_right {
                    Operand::Int(i) => i,
                    Operand::Old => old,
                };
            }
        }
        let target_monkey_idx = match result % self.test_divisible_by {
            0 => self.if_true_monkey,
            _ => self.if_false_monkey,
        };
        return (target_monkey_idx as usize, result);
    }
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
        let mut operation_left = Operand::Old;
        println!("Dealing with {}", &cap[4]);
        let operation_right = match &cap[4] == "old" {
            true => Operand::Old,
            false => Operand::Int(cap[4].parse::<i32>().unwrap()),
        };
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
    for monkey in &mut monkeys {
        monkey.take_turn(&mut monkeys);
    }
}
