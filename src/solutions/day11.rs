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
            test: Vec::new(),
        }
    }
}

pub fn solve(monkeys: &mut Vec<Monkey>) -> (i64, i64) {
    let mut p1_monkeys = monkeys.to_owned();
    let p1 = calculate(&mut p1_monkeys, 20, String::from("1"));

    let mut p2_monkeys = monkeys.to_owned();
    let p2 = calculate(&mut p2_monkeys, 10000, String::from("2"));

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
                        // Part 2, we set the new worry to modulus of the product of all the values at test[0] for all monkeys.
                        let mut product = 1;
                        // Use an iterator to get the product of all the values at test[0].
                        for m in monkeys.iter() {
                            product *= m.test[0];
                        }

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
    inspected.reverse();

    // Return the product of the highest and 2nd highest values.
    inspected[0] * inspected[1]
}

pub fn parse(_data: &[String]) -> Vec<Monkey> {
    let monkeys = vec![
        Monkey {
            worries: vec![78, 53, 89, 51, 52, 59, 58, 85],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("3"),
                oper: '*',
            },
            test: vec![5, 2, 7],
        },
        Monkey {
            worries: vec![64],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("7"),
                oper: '+',
            },
            test: vec![2, 3, 6],
        },
        Monkey {
            worries: vec![71, 93, 65, 82],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("5"),
                oper: '+',
            },
            test: vec![13, 5, 4],
        },
        Monkey {
            worries: vec![67, 73, 95, 75, 56, 74],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("8"),
                oper: '+',
            },
            test: vec![19, 6, 0],
        },
        Monkey {
            worries: vec![85, 91, 90],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("4"),
                oper: '+',
            },
            test: vec![11, 3, 1],
        },
        Monkey {
            worries: vec![67, 96, 69, 55, 70, 83, 62],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("2"),
                oper: '*',
            },
            test: vec![3, 4, 1],
        },
        Monkey {
            worries: vec![53, 86, 98, 70, 64],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("6"),
                oper: '+',
            },
            test: vec![7, 7, 0],
        },
        Monkey {
            worries: vec![88, 64],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("old"),
                oper: '*',
            },
            test: vec![17, 2, 5],
        },
    ];

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
        assert_eq!(res.0, 0);
    }

    #[test]
    fn part2() {
        let res = solve(&mut parse(&crate::library::read_file("testdata/day11.txt")));
        assert_eq!(res.1, 0);
    }
}
