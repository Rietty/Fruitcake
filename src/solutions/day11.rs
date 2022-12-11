// https://adventofcode.com/2022/day/11

// Imports
#[allow(unused_imports)]
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Operation {
    op1: String,
    op2: String,
    oper: char,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    worries: Vec<i64>,
    operation: Operation,
    test: Vec<i64>,
}

// Implement a new monkey.
impl Monkey {
    #[allow(dead_code)]
    pub fn new() -> Monkey {
        Monkey {
            worries: Vec::new(),
            operation: Operation {
                op1: String::new(),
                op2: String::new(),
                oper: ' ',
            },
            test: vec![0, 0, 0],
        }
    }
}

pub fn solve(monkeys: &mut Vec<Monkey>) -> (i64, i64) {
    let mut p1_monkeys = monkeys.to_owned();
    let p1 = calculate(&mut p1_monkeys, 20, String::from("1"));

    let mut p2_monkeys = monkeys.to_owned();
    let p2 = calculate(&mut p2_monkeys, 10_000, String::from("2"));

    (p1, p2)
}

pub fn calculate(monkeys: &mut Vec<Monkey>, iterations: i64, part: String) -> i64 {
    // Closure to calculate the worry of a monkey.
    let calc_worry = |old: i64, new: i64, op: char| -> i64 {
        match op {
            '+' => old + new,
            '*' => old * new,
            _ => panic!("Invalid operator!"),
        }
    };

    // Vec to store the inspected counts of each monkey, size is of the monkeys vec, default value is 0.
    let mut inspected = vec![0; monkeys.len()];

    // Part 2, we set the new worry to modulus of the product of all the values at test[0] for all monkeys.
    let mut product = 1;
    // Use an iterator to get the product of all the values at test[0].
    for m in monkeys.iter() {
        product *= m.test[0];
    }

    // Need to do iterations times.
    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            // Clone the worries of the current monkey.
            let worries = monkeys[i].worries.clone();

            // Iterate over the worries of the current monkey.
            for w in worries {
                let op1 = if monkeys[i].operation.op1 == "old" {
                    w
                } else {
                    monkeys[i].operation.op1.parse::<i64>().unwrap()
                };

                let op2 = if monkeys[i].operation.op2 == "old" {
                    w
                } else {
                    monkeys[i].operation.op2.parse::<i64>().unwrap()
                };

                let mut new_worry = calc_worry(op1, op2, monkeys[i].operation.oper);

                // Based on the part, do different things.
                match part.as_str() {
                    "1" => {
                        // Part 1, we just divide by 3.
                        new_worry /= 3;
                    }

                    "2" => {
                        new_worry %= product;
                    }

                    _ => panic!("Invalid part!"),
                }

                if new_worry % monkeys[i].test[0] == 0 {
                    let next_monkey = monkeys[i].test[1] as usize;
                    monkeys[next_monkey].worries.push(new_worry);
                } else {
                    let next_monkey = monkeys[i].test[2] as usize;
                    monkeys[next_monkey].worries.push(new_worry);
                }

                inspected[i] += 1;
            }

            monkeys[i].worries.clear();
        }
    }

    // Sort the inspected vector.
    inspected.sort();

    // Return the product of the highest and 2nd highest values.
    inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
}

pub fn parse(data: &[String]) -> Vec<Monkey> {
    // Parser will parse the data into a vector of monkeys.
    let mut monkeys: Vec<Monkey> = Vec::new();

    // Regex to parse the data.
    let re_monkey = Regex::new(r"Monkey (\d+):").unwrap();
    let re_starting_items = Regex::new(r"Starting items: (\d+(, \d+)*)").unwrap();
    let re_operation = Regex::new(r"Operation: (\w+) = (\w+) ([\*\+]) (\w+)").unwrap();
    let re_test = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let re_if_true = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let re_if_false = Regex::new(r"If false: throw to monkey (\d+)").unwrap();

    for line in data {
        // If line is empty, skip it.
        if line.is_empty() {
            continue;
        }

        // If the line matches re_monkey, we create a new monkey.
        if re_monkey.is_match(line) {
            monkeys.push(Monkey::new());
        }

        // If the line matches re_starting_items, we parse the starting items, and add them to the worries of the last monkey.
        if re_starting_items.is_match(line) {
            let mut worries = Vec::new();
            for cap in re_starting_items.captures_iter(line) {
                for item in cap[1].split(", ") {
                    worries.push(item.parse::<i64>().unwrap());
                }
            }

            let last_monkey = monkeys.len() - 1;
            monkeys[last_monkey].worries = worries;
        }

        // If the line matches re_operation, we parse the operation.
        if re_operation.is_match(line) {
            let last_monkey = monkeys.len() - 1;
            for cap in re_operation.captures_iter(line) {
                monkeys[last_monkey].operation = Operation {
                    op1: cap[2].to_string(),
                    op2: cap[4].to_string(),
                    oper: cap[3].chars().next().unwrap(),
                };
            }
        }

        // If the line matches re_test, we parse the test.
        if re_test.is_match(line) {
            let last_monkey = monkeys.len() - 1;
            for cap in re_test.captures_iter(line) {
                monkeys[last_monkey].test[0] = cap[1].parse::<i64>().unwrap();
            }
        }

        // If the line matches re_if_true, we parse the if true.
        if re_if_true.is_match(line) {
            let last_monkey = monkeys.len() - 1;
            for cap in re_if_true.captures_iter(line) {
                monkeys[last_monkey].test[1] = cap[1].parse::<i64>().unwrap();
            }
        }

        // If the line matches re_if_false, we parse the if false.
        if re_if_false.is_match(line) {
            let last_monkey = monkeys.len() - 1;
            for cap in re_if_false.captures_iter(line) {
                monkeys[last_monkey].test[2] = cap[1].parse::<i64>().unwrap();
            }
        }
    }

    monkeys
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&mut parse(&crate::library::read_file("data/day11.txt")));
    println!("Day 11:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let mut data = parse(&crate::library::read_file("data/day11.txt"));
    c.bench_function("Day 11", |b| b.iter(|| solve(&mut data)));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let res = solve(&mut parse(&crate::library::read_file("testdata/day11.txt")));
        assert_eq!(res.0, 10605);
    }

    #[test]
    fn part2() {
        let res = solve(&mut parse(&crate::library::read_file("testdata/day11.txt")));
        assert_eq!(res.1, 2713310158);
    }
}
