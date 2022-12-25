// https://adventofcode.com/2022/day/24

use hashbrown::HashSet;
use std::collections::VecDeque;

pub fn solve(data: &Vec<Vec<char>>) -> (i32, i32) {
    // Get the size of the grid.
    let n = data.len();
    let m = data[0].len();

    // Solve part 1.
    let p1 = calculate(data, 0, (0, 1), (n as i32 - 1, m as i32 - 2));

    (p1, 0)
}

// Simply use a BFS to find the shortest path between the start and end points.
// Since the blizzards are periodic, we will know exactly if there is a blizzard at any given point or not.
pub fn calculate(data: &Vec<Vec<char>>, timestep: i32, start: (i32, i32), end: (i32, i32)) -> i32 {
    // Create a wall hashset to store the walls.
    let mut walls = std::collections::HashSet::new();
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
    c.bench_function("Day 24", |b| b.iter(|| solve(&data)));
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
