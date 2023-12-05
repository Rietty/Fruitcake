// https://adventofcode.com/2022/day/14

use std::collections::HashSet;

pub fn solve(data: &HashSet<(i32, i32)>) -> (i32, i32) {
    // Maximum depth, Set it to one higher than the maximum value in second part.
    let mut max_depth = 0;

    for p in data {
        max_depth = std::cmp::max(p.1, max_depth);
    }

    max_depth += 1;

    // Create a copy of the data.
    let mut p1_data = data.clone();
    let mut p2_data = data.clone();

    // Get the size of the initial data.
    let mut p1 = p1_data.len();
    let mut p2 = p2_data.len();

    'outer: loop {
        let mut s = (500, 0);
        let mut counter = 0;
        loop {
            counter += 1;
            if counter > max_depth {
                p1 = p1_data.len() - p1;
                break 'outer;
            }

            if !p1_data.contains(&(s.0, s.1 + 1)) {
                s = (s.0, s.1 + 1);
                continue;
            } else if !p1_data.contains(&(s.0 - 1, s.1 + 1)) {
                s = (s.0 - 1, s.1 + 1);
                continue;
            } else if !p1_data.contains(&(s.0 + 1, s.1 + 1)) {
                s = (s.0 + 1, s.1 + 1);
                continue;
            }
            p1_data.insert(s);
            break;
        }
    }

    loop {
        let mut s = (500, 0);

        if p2_data.contains(&s) {
            p2 = p2_data.len() - p2;
            break;
        }

        loop {
            if s.1 == max_depth {
                p2_data.insert(s);
                break;
            }

            if !p2_data.contains(&(s.0, s.1 + 1)) {
                s = (s.0, s.1 + 1);
                continue;
            } else if !p2_data.contains(&(s.0 - 1, s.1 + 1)) {
                s = (s.0 - 1, s.1 + 1);
                continue;
            } else if !p2_data.contains(&(s.0 + 1, s.1 + 1)) {
                s = (s.0 + 1, s.1 + 1);
                continue;
            }

            p2_data.insert(s);
            break;
        }
    }

    (p1 as i32, p2 as i32)
}

pub fn parse(data: &[String]) -> HashSet<(i32, i32)> {
    // Parse as a pair of coordinates and add into basically a grid.
    let data = data
        .iter()
        .map(|s| s.trim().split(" -> ").map(String::from).collect::<Vec<_>>())
        .map(|coord| {
            coord
                .iter()
                .map(|s| {
                    let xy: Vec<&str> = s.split(',').collect::<Vec<_>>();
                    (xy[0].parse::<i32>().unwrap(), xy[1].parse::<i32>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut pairs: HashSet<(i32, i32)> = HashSet::new();

    for line in data {
        for (_, (s, e)) in line.iter().zip(line.iter().skip(1)).enumerate() {
            // Figure out if we are going horizontal or vertical.
            if s.0 == e.0 {
                for y in std::cmp::min(s.1, e.1)..std::cmp::max(s.1, e.1) + 1 {
                    pairs.insert((s.0, y));
                }
            } else {
                for x in std::cmp::min(s.0, e.0)..std::cmp::max(s.0, e.0) + 1 {
                    pairs.insert((x, s.1));
                }
            }
        }
    }

    pairs
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day14.txt")));
    println!("Day 14:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day14.txt"));
    c.bench_function("Day 14 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 14 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day14.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day14.txt")));
        assert_eq!(res.0, 0);
        println!("Part 1: Expected: 24, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day14.txt")));
        assert_eq!(res.1, 0);
        println!("Part 1: Expected: 93, Actual: {}", res.1);
    }
}
