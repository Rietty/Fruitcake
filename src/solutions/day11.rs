// https://adventofcode.com/2022/day/11

// Imports
#[allow(unused_imports)]
use regex::Regex;

#[derive(Debug)]
pub struct Operation {
    op1: String,
    op2: String,
    oper: char,
}

#[derive(Debug)]
pub struct Monkey {
    worries: Vec<i32>,
    operation: Operation,
    test: Vec<i32>,
    inspected: i32,
}

// Implement a new monkey.
impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            worries: Vec::new(),
            operation: Operation {
                op1: String::new(),
                op2: String::new(),
                oper: ' ',
            },
            test: Vec::new(),
            inspected: 0,
        }
    }
}

pub fn solve(_data: &[String]) -> (i32, i32) {
    // Hardcode the input for now.
    let mut monkeys = vec![
        Monkey {
            worries: vec![79, 98],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("19"),
                oper: '*',
            },
            test: vec![23, 2, 3],
            inspected: 0,
        },
        Monkey {
            worries: vec![54, 65, 75, 74],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("6"),
                oper: '+',
            },
            test: vec![19, 2, 0],
            inspected: 0,
        },
        Monkey {
            worries: vec![79, 60, 97],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("old"),
                oper: '*',
            },
            test: vec![13, 1, 3],
            inspected: 0,
        },
        Monkey {
            worries: vec![74],
            operation: Operation {
                op1: String::from("old"),
                op2: String::from("3"),
                oper: '+',
            },
            test: vec![17, 0, 1],
            inspected: 0,
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

    // For loop to run 20 times.
    for i in 0..20 {
        // Iterate over all the monkeys and print out their worries.
        for m in monkeys.iter_mut().for_each(|m| {
            m.worries.iter_mut().for_each(|w| {
                let op = &m.operation;

                let op1 = if op.op1 == "old" {
                    *w
                } else {
                    op.op1.parse::<i32>().unwrap()
                };

                let op2 = if op.op2 == "old" {
                    *w
                } else {
                    op.op2.parse::<i32>().unwrap()
                };

                *w = calc_worry(op1, op2, op.oper);

                m.inspected += 1;

                w = (*w as f32 / 3.0).floor() as i32;
                
                let remainder = *w % m.test[0];
                if remainder == 0 {
                    monkeys[m.test[1] as usize].worries.push(*w);
                } else {
                    monkeys[m.test[2] as usize].worries.push(*w);
                }
            });
            m.worries.clear();
        });
    }

    (0, 0)
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
