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
    items_inspected: i32,
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
            items_inspected: 0,
        };
        monkeys.push(monkey);
    }
    // Run 20 rounds
    for round in 1..21 {
        // Loop through and process every monkey
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut items_to_send = Vec::new();
            while monkey.items.len() > 0 {
                // Take left-most item from monkey.items
                let item = monkey.items.remove(0);
                monkey.items_inspected += 1;
                let mut result = 0;
                // First the monkey applies the operation to the item
                match monkey.operation_op {
                    Op::Plus => {
                        result = match monkey.operation_left {
                            Operand::Int(i) => i,
                            Operand::Old => item,
                        } + match monkey.operation_right {
                            Operand::Int(i) => i,
                            Operand::Old => item,
                        };
                    }
                    Op::Times => {
                        result = match monkey.operation_left {
                            Operand::Int(i) => i,
                            Operand::Old => item,
                        } * match monkey.operation_right {
                            Operand::Int(i) => i,
                            Operand::Old => item,
                        };
                    }
                }
                // After each monkey inspects an item but before it tests your worry level,
                // worry level divides by 3 rounded down to nearest integer
                result = result / 3;
                let target_monkey_idx = match result % monkey.test_divisible_by {
                    0 => monkey.if_true_monkey,
                    _ => monkey.if_false_monkey,
                };
                items_to_send.push((target_monkey_idx, result));
            }
            println!(
                "  Monkey {}, items_to_send = {:?}",
                monkey.id, items_to_send
            );
            // Now we send the items to the other monkeys
            for (target_monkey_idx, item) in &items_to_send {
                monkeys[*target_monkey_idx as usize].items.push(*item);
            }
        }
        println!("\nAfter {} rounds:\n\n", round);
        for monkey in &monkeys {
            println!("Monkey {:?}", monkey);
        }
        println!("\nLevel of monkey business (product of num inspected for two busiest monkeys):");
        let items_inspected = monkeys
            .iter()
            .map(|x| x.items_inspected)
            .collect::<Vec<i32>>();
        // Pick the top two
        let mut top_two = items_inspected.clone();
        top_two.sort();
        top_two.reverse();
        let top_two = top_two[0..2].to_vec();
        let level_of_monkey_business = top_two[0] * top_two[1];
        println!("  {}", level_of_monkey_business);
    }
}
