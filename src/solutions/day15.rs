// https://adventofcode.com/2022/day/15

use regex::Regex;

#[derive(Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

pub fn solve(data: &Vec<(Point, Point)>) -> (i64, i64) {
    // Variables to check against for p1/p2.
    let y_col = 2_000_000;
    let max = 4_000_000;

    // Closure to calculate manhattan distance between two points.
    let manhattan = |a: &Point, b: &Point| (a.x - b.x).abs() + (a.y - b.y).abs();

    // Solve part 1 by finding intervals we need.
    let mut intervals = Vec::new();

    // Iterate over all the lines in vector and use the scanner and beacon to find the intervals.
    for (s, b) in data {
        let d = manhattan(s, b) - (s.y - y_col).abs();
        if d >= 0 {
            let x1 = s.x - d;
            let x2 = s.x + d;
            intervals.push((x1, x2));
        }
    }

    // Iterate over the intervals and find the points that are in any of them and add them to a set.
    let mut set = Vec::new();
    for (a, b) in intervals {
        for i in a..=b {
            set.push(i);
        }
    }

    // Sort the set.
    set.sort();

    let mut p1 = 0;

    // Iterate over the set and find the points that are not equal to the next point.
    for i in 0..set.len() - 1 {
        if set[i] != set[i + 1] {
            p1 += 1;
        }
    }

    // Create a vector of tuples that contain the (x, y, dist) coordinates of the scanner and their manhattan distance to the beacon.
    let mut vec = Vec::new();
    for (s, b) in data {
        vec.push((s.x, s.y, manhattan(s, b)));
    }

    // Create a hashset to store the points for the various lines.
    let mut set_a1 = std::collections::HashSet::new(); // x - y + dist + 1.
    let mut set_a2 = std::collections::HashSet::new(); // x - y - dist - 1.
    let mut set_b1 = std::collections::HashSet::new(); // x + y + dist + 1.
    let mut set_b2 = std::collections::HashSet::new(); // x + y - dist - 1.

    for (x, y, dist) in vec {
        set_a1.insert(x - y + dist + 1);
        set_a2.insert(x - y - dist - 1);
        set_b1.insert(x + y + dist + 1);
        set_b2.insert(x + y - dist - 1);
    }

    // Find intersections of sets a1 and a2 and of pair b1 and b2.
    // It should be one number in each set. (i.e. the first and only element)
    let a = set_a1.intersection(&set_a2).collect::<Vec<_>>()[0];
    let b = set_b1.intersection(&set_b2).collect::<Vec<_>>()[0];

    // The intersection of a and b is the point we need so we can just calculate based off that.
    let p2 = (a + b) * max / 2 + (b - a) / 2;

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<(Point, Point)> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let mut res = Vec::new();

    for s in data {
        if let Some(caps) = re.captures(s) {
            let sensor = Point {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
            };
            let beacon = Point {
                x: caps[3].parse().unwrap(),
                y: caps[4].parse().unwrap(),
            };
            res.push((sensor, beacon));
        }
    }

    res
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day15.txt")));
    println!("Day 15:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day15.txt"));
    c.bench_function("Day 15 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 15 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day15.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day15.txt")));
        assert_eq!(res.0, 26);
        println!("Part 1: Expected: 26, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day15.txt")));
        assert_eq!(res.1, 56000011);
        println!("Part 2: Expected: 56000011, Actual: {}", res.1);
    }
}
