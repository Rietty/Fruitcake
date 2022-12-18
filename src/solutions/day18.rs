// https://adventofcode.com/2022/day/18

use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve(data: &[(i32, i32, i32)]) -> (i32, i32) {
    // Return a vector of all adjacent points to a given point using a closure.
    let adjacent = |(x, y, z): (i32, i32, i32)| {
        vec![
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
    };

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

    // Create a closure that will tell us if a specific cube area is boxed by other cubes.
    let boxed = |(x, y, z): (i32, i32, i32)| -> bool {
        // Create a queue of points.
        let mut queue = VecDeque::new();
        // Create a set of points.
        let mut set = HashSet::new();

        // Check if the x, y, z point is not in the visited and not in the original data vector, if both are true then add it to the queue and set.
        if !data.contains(&(x, y, z)) && !set.contains(&(x, y, z)) {
            queue.push_back((x, y, z));
            set.insert((x, y, z));
        }

        while !queue.is_empty() {
            // Pop off the first point in the queue.
            let (x, y, z) = queue.pop_front().unwrap();

            // If the minimum is below -1 or maximum is above 20 for any of the x, y, z values then return false.
            // Basically just scanned input manually to find the highest/lowest value.
            if !(-1..=20).contains(&x) || !(-1..=20).contains(&y) || !(-1..=20).contains(&z) {
                return false;
            }

            // For every adjacent point, add it using the same logic as above.
            for (x, y, z) in adjacent((x, y, z)) {
                if !data.contains(&(x, y, z)) && !set.contains(&(x, y, z)) {
                    queue.push_back((x, y, z));
                    set.insert((x, y, z));
                }
            }
        }

        true
    };

    // Create a set of points for all boxed in points.
    let mut boxed_points = HashSet::new();

    // Iterate over each x, y, z point from -1 to 20 and check if it is boxed in, if it is, add it to the boxed_points set.
    for x in -1..=20 {
        for y in -1..=20 {
            for z in -1..=20 {
                if boxed((x, y, z)) {
                    boxed_points.insert((x, y, z));
                }
            }
        }
    }

    let mut p2 = 0;

    // For every point in the initial data vector, get all of the neighbors, and if a neighbor is not in the boxed_points set or the original data vector, increment p2.
    for (x, y, z) in data.iter() {
        for (x, y, z) in adjacent((*x, *y, *z)) {
            if !boxed_points.contains(&(x, y, z)) && !data.contains(&(x, y, z)) {
                p2 += 1;
            }
        }
    }

    (p1 as i32, p2)
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
    c.bench_function("Day 18", |b| b.iter(|| solve(&data)));
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
