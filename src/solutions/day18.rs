// https://adventofcode.com/2022/day/18

use std::collections::HashSet;

pub fn solve(data: &[(i32, i32, i32)]) -> (i32, i32) {
    // Scan through the data and obtain the maximum and minimum value of any axis.
    let min = data
        .iter()
        .fold((i32::MAX, i32::MAX, i32::MAX), |(x, y, z), (x1, y1, z1)| {
            (x.min(*x1), y.min(*y1), z.min(*z1))
        });

    let max = data
        .iter()
        .fold((i32::MIN, i32::MIN, i32::MIN), |(x, y, z), (x1, y1, z1)| {
            (x.max(*x1), y.max(*y1), z.max(*z1))
        });

    // Iterate over the data vector and sum up the number of points that are adjacent to a given point but not in the data vector.
    let p1 = data
        .iter()
        .map(|point| {
            adjacent(*point)
                .iter()
                .filter(|p| !data.contains(p))
                .count()
        })
        .sum::<usize>();

    let visible = visible(data, min.0 as usize, max.0 as usize);
    let p2 = data
        .iter()
        .flat_map(|point| adjacent(*point))
        .filter(|p| visible.contains(p))
        .count();

    (p1 as i32, p2 as i32)
}

// Get all exposed/visible points to all cubes in a HashSet.
pub fn visible(data: &[(i32, i32, i32)], min: usize, max: usize) -> HashSet<(i32, i32, i32)> {
    let mut visible = HashSet::new();
    let start_point = (0, 0, 0);

    let mut stack = Vec::new();
    stack.push(start_point);

    let mut visited = HashSet::new();
    visited.insert(start_point);

    while let Some(point) = stack.pop() {
        // For each adjacent point..
        for p in adjacent(point) {
            // If the data contains the neighbour or the neighbour is not within the bounds of (min - 1, min - 1, min - 1) to (max + 1, max + 1, max + 1)..
            if data.contains(&p)
                || p.0 < min as i32 - 1
                || p.1 < min as i32 - 1
                || p.2 < min as i32 - 1
                || p.0 > max as i32 + 1
                || p.1 > max as i32 + 1
                || p.2 > max as i32 + 1
            {
                continue;
            }

            if visited.insert(p) {
                stack.push(p);
                visible.insert(p);
            }
        }
    }

    visible
}

// Get all adjacent points to a given point.
pub fn adjacent((x, y, z): (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

pub fn parse(data: &[String]) -> Vec<(i32, i32, i32)> {
    data.iter()
        .map(|line| {
            let mut iter = line.split(',');
            (
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32, i32)>>()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day18.txt")));
    println!("Day 18:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day18.txt"));
    c.bench_function("Day 18 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 18 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day18.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day18.txt")));
        assert_eq!(res.0, 64);
        println!("Part 1: Expected: 64, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day18.txt")));
        assert_eq!(res.1, 58);
        println!("Part 2: Expected: 58, Actual: {}", res.1);
    }
}
