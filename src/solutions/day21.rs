// https://adventofcode.com/2022/day/21

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Monkey {
    name: String,
    value: Option<i128>,
    op: Option<char>,
    requirements: Option<(String, String)>,
}

pub fn solve(data: &VecDeque<Monkey>) -> (i128, i128) {
    // Part 1.
    let (a, b, _) = calculate(data, None);
    let p1 = a + b;

    // Part 2. Calculate an upper and lower-bound and then hone-in on answer.
    let (mut lower, mut upper): (i128, i128) = (0, 0);

    let (m1, m2, _) = calculate(data, Some(0));
    let (m3, m4, _) = calculate(data, Some(100));

    #[allow(unused_assignments)]
    let mut increasing: bool = false;

    if m1 - m2 >= m3 - m4 {
        increasing = false;
    } else {
        increasing = true;
    }

    for i in 0..20 {
        upper = i128::pow(10, i);
        let (a, b, _) = calculate(data, Some(upper));
        if !increasing {
            if a - b < 0 {
                break;
            } else {
                lower = upper;
            }
        } else if a - b > 0 {
            break;
        } else {
            lower = upper;
        }
    }

    let mut p2: i128 = 0;
    let mut res: i128 = -1;

    while res != 0 {
        p2 = lower + ((upper - lower) / 2);
        let (a, b, _) = calculate(data, Some(p2));
        res = a - b;
        match a.cmp(&b) {
            std::cmp::Ordering::Greater => {
                if increasing {
                    upper = p2;
                } else {
                    lower = p2;
                }
            }
            std::cmp::Ordering::Equal => {
                break;
            }
            std::cmp::Ordering::Less => {
                if increasing {
                    lower = p2;
                } else {
                    upper = p2;
                }
            }
        }
    }

    (p1, p2)
}

// This function is designed to basically do both parts, based on if we pass a value to the human or not.
// Part 2 is basically just an extension of part 1, but we call the function by changing the human value using a binary search.
pub fn calculate(data: &VecDeque<Monkey>, human: Option<i128>) -> (i128, i128, bool) {
    // Create a lookup HashMap which will contain the values of each given key/monkey.
    let mut lookup: HashMap<String, i128> = HashMap::new();
    let (mut left, mut right) = (String::new(), String::new());

    // We need to loop while the lookup table does not contain the root key..
    while !lookup.contains_key("root") {
        // Iterate over the data deque...
        for monkey in data.iter() {
            // Check to see if we have they current monkey's name be "humn"..
            if monkey.name == "humn" && human.is_some() {
                if let Some(human) = human {
                    lookup.insert("humn".to_string(), human);
                }
            }

            // If the monkey name is root, and both the sub-keys are in database we can return info..
            if monkey.name == "root" {
                let requirements = monkey.requirements.as_ref().unwrap();
                left = requirements.0.to_string();
                right = requirements.1.to_string();
                if lookup.contains_key(&left) && lookup.contains_key(&right) {
                    return (
                        *lookup.get(&left).unwrap(),
                        *lookup.get(&right).unwrap(),
                        lookup.get(&requirements.0).unwrap()
                            == lookup.get(&requirements.1).unwrap(),
                    );
                }
            }

            // If the name is not contained in keys... we need to do the operation..
            if !lookup.contains_key(&monkey.name) {
                if monkey.op.is_some() {
                    // Grab requirements..
                    let requirements = monkey.requirements.as_ref().unwrap();
                    if lookup.contains_key(&requirements.0) && lookup.contains_key(&requirements.1)
                    {
                        let a = lookup.get(&requirements.0).unwrap();
                        let b = lookup.get(&requirements.1).unwrap();
                        match monkey.op.unwrap() {
                            '+' => {
                                lookup.insert(monkey.name.clone(), a + b);
                            }
                            '-' => {
                                lookup.insert(monkey.name.clone(), a - b);
                            }
                            '*' => {
                                lookup.insert(monkey.name.clone(), a * b);
                            }
                            '/' => {
                                lookup.insert(monkey.name.clone(), a / b);
                            }
                            _ => {}
                        }
                    }
                } else {
                    lookup.insert(monkey.name.clone(), monkey.value.unwrap());
                }
            }
        }
    }

    (
        *lookup.get(&left).unwrap(),
        *lookup.get(&right).unwrap(),
        false,
    )
}

pub fn parse(data: &[String]) -> VecDeque<Monkey> {
    // Create a new monkey dequeue I can add/pop off and remove from specific points.
    let mut monkeys: VecDeque<Monkey> = VecDeque::new();

    // Iterate through the data and add each monkeys info into the vector, depending on if it has specific amount of entries in the line or not.
    for line in data {
        let info = line.split_whitespace().collect::<Vec<_>>();

        // Depending on how many items are in the vector, we do different things.
        match info.len() {
            2 => {
                let n = info[0].trim_matches(':').to_string();
                let v = info[1].parse::<i128>().unwrap();
                monkeys.push_back(Monkey {
                    name: n,
                    value: Some(v),
                    op: None,
                    requirements: None,
                });
            }
            4 => {
                let n = info[0].trim_matches(':').to_string();
                let o = info[2].chars().next().unwrap();
                let a = info[1].to_string();
                let b = info[3].to_string();
                monkeys.push_back(Monkey {
                    name: n,
                    value: None,
                    op: Some(o),
                    requirements: Some((a, b)),
                });
            }
            _ => {}
        }
    }

    monkeys
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day21.txt")));
    println!("Day 21:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day21.txt"));
    c.bench_function("Day 21 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 21 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day21.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day21.txt")));
        assert_eq!(res.0, 152);
        println!("Part 1: Expected: 152, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day21.txt")));
        assert_eq!(res.1, 301);
        println!("Part 1: Expected: 301, Actual: {}", res.0);
    }
}
