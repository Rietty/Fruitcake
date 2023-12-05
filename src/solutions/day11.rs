// https://adventofcode.com/2022/day/11

#[derive(Debug, Clone)]
pub struct Operation {
    op: String,
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
                op: String::new(),
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

    let mut inspected = vec![0; monkeys.len()];
    let product = monkeys.iter().fold(1, |acc, m| acc * m.test[0]);

    // Need to do iterations times.
    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            // Clone the worries of the current monkey.
            let worries = monkeys[i].worries.clone();

            // Iterate over the worries of the current monkey.
            for w in worries {
                let op = if monkeys[i].operation.op == "old" {
                    w
                } else {
                    monkeys[i].operation.op.parse::<i64>().unwrap()
                };

                let mut new_worry = calc_worry(w, op, monkeys[i].operation.oper);

                if part.as_str() == "1" {
                    new_worry /= 3;
                } else if part.as_str() == "2" {
                    new_worry %= product;
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
    let mut line_num = 0;

    for line in data {
        if line_num == 6 {
            line_num = 0;
            continue;
        }

        let monkeys_len = monkeys.len();

        match line_num {
            0 => {
                monkeys.push(Monkey::new());
            }

            1 => {
                let split = line.trim().split(' ').collect::<Vec<&str>>();
                split.iter().skip(2).for_each(|s| {
                    monkeys[monkeys_len - 1]
                        .worries
                        .push(s.replace(',', "").parse::<i64>().unwrap());
                });
            }

            2 => {
                let split = line.trim().split(' ').collect::<Vec<&str>>();
                monkeys[monkeys_len - 1].operation.op = split[5].to_string();
                monkeys[monkeys_len - 1].operation.oper = split[4].chars().next().unwrap();
            }

            3 | 4 | 5 => {
                let split = line.trim().split(' ').collect::<Vec<&str>>();
                monkeys[monkeys_len - 1].test[line_num - 3] =
                    split[split.len() - 1].parse::<i64>().unwrap();
            }

            _ => panic!("Invalid line number!"),
        }

        line_num += 1;
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
    let data = parse(&crate::library::read_file("data/day11.txt"));
    c.bench_function("Day 11 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 11 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day11.txt"));
            solve(&data)
        })
    });
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let res = solve(&mut parse(&crate::library::read_file("testdata/day11.txt")));
        assert_eq!(res.0, 10605);
        println!("Part 1: Expected: 10605, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&mut parse(&crate::library::read_file("testdata/day11.txt")));
        assert_eq!(res.1, 2713310158);
        println!("Part 2: Expected: 2713310158, Actual: {}", res.1);
    }
}
