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
    inspected: i32,
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
    for _ in 0..20 {
        // Create a clone of the monkeys, so we can modify them while we process the original.
        let mut new_monkeys = monkeys.clone();

        // Iterate over the current monkeys.
        for (index, monkey) in monkeys.iter().enumerate() {
            // Iterate over the worries of the current monkey.
            for worry in monkey.worries.iter() {
                // Get the first operand, if it is "old", set it to the value of the worry, else parse it as an integer.
                let op1 = if monkey.operation.op1 == "old" {
                    *worry
                } else {
                    monkey.operation.op1.parse::<i32>().unwrap()
                };

                // Get the second operand, if it is "old", set it to the value of the worry, else parse it as an integer.
                let op2 = if monkey.operation.op2 == "old" {
                    *worry
                } else {
                    monkey.operation.op2.parse::<i32>().unwrap()
                };

                // Calculate the new worry.
                let new_worry = calc_worry(op1, op2, monkey.operation.oper);

                // Divide by 3 and round down (floor) as an integer.
                let new_worry = (new_worry as f32 / 3.0).floor() as i32;

                // Check if new_worry is divisible by monkey.test[0], if so, push it to the worries of monkey.test[1], else push it to the worries of monkey.test[2].
                if new_worry % monkey.test[0] == 0 {
                    new_monkeys[monkey.test[1] as usize].worries.push(new_worry);
                } else {
                    new_monkeys[monkey.test[2] as usize].worries.push(new_worry);
                }

                // Print the test vector.
                println!("Monkey {} : {:?}", index, monkey.test);

                // Increment the inspected counter.
                new_monkeys[index].inspected += 1;
            }

            // Clear the worries of the current monkey.
            new_monkeys[index].worries.clear();
        }

        // Set the monkeys to the new monkeys.
        monkeys = new_monkeys;
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
