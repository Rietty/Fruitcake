// https://adventofcode.com/2022/day/24

#[allow(unused_imports)]
use hashbrown::HashSet;
#[allow(unused_imports)]
use std::collections::VecDeque;

pub fn solve(data: &Vec<Vec<char>>) -> (i32, i32) {
    // This day was done in Python, code to be ported.
    (0, 0)
}

// Simply use a BFS to find the shortest path between the start and end points.
// Since the blizzards are periodic, we will know exactly if there is a blizzard at any given point or not.
pub fn calculate(_data: &[Vec<char>], _timestep: i32, _start: (i32, i32), _end: (i32, i32)) -> i32 {
    0
}

pub fn parse(data: &[String]) -> Vec<Vec<char>> {
    data.iter().map(|x| x.chars().collect()).collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day24.txt")));
    println!("Day 24:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day24.txt"));
    c.bench_function("Day 24 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 24 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day24.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day24.txt")));
        assert_eq!(res.0, 18);
        println!("Part 1: Expected: 18, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day24.txt")));
        assert_eq!(res.1, 0);
        println!("Part 2: Expected: 0, Actual: {}", res.1);
    }
}
