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
    worries: Vec<i32>,
    operation: Operation,
    test: Vec<i32>,
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

pub fn solve(_data: &[String]) -> (i32, i32) {
    let mut monkeys = vec![
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

    // Closure to modify the worry of a monkey.
    let calc_worry = |old: i32, new: i32, op: char| -> i32 {
        match op {
            '+' => old + new,
            '*' => old * new,
            _ => panic!("Invalid operator!"),
        }
    };

    // Vec to store the inspected counts of each monkey, size is of the monkeys vec, default value is 0.
    let mut inspected = vec![0; monkeys.len()];

    // For loop to run 20 times.
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            // Clone the worries of the current monkey.
            let worries = monkeys[i].worries.clone();

            // Iterate over the worries of the current monkey.
            for w in worries {
                let op1 = if monkeys[i].operation.op1 == "old" {
                    w
                } else {
                    monkeys[i].operation.op1.parse::<i32>().unwrap()
                };

                let op2 = if monkeys[i].operation.op2 == "old" {
                    w
                } else {
                    monkeys[i].operation.op2.parse::<i32>().unwrap()
                };

                let new_worry = calc_worry(op1, op2, monkeys[i].operation.oper);

                let new_worry = new_worry / 3;

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
    // P1 is the highest value multiplied by the 2nd highest value.
    let p1 = inspected[inspected.len() - 1] * inspected[inspected.len() - 2];
    println!("{:?}", inspected);

    (p1, 0)
}

pub fn _parse(_data: &[String]) -> Vec<Monkey> {
    Vec::new()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day11.txt"));
    println!("Day 11:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = crate::library::read_file("data/day11.txt");
    c.bench_function("Day 11", |b| b.iter(|| solve(&data)));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let res = solve(&crate::library::read_file("testdata/day11.txt"));
        assert_eq!(res.0, 0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day11.txt"));
        assert_eq!(res.1, 0);
    }
}
