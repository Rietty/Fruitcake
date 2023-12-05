// https://adventofcode.com/2022/day/23

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(data: &mut HashSet<(i32, i32)>) -> (i32, i32) {
    // Create an array of tuples that represent the 8 directions as offsets from the current position.
    let directions = [
        // Positive Y = Down (South)
        // Positive X = Right (East)
        (-1, -1), // Top Left
        (0, -1),  // Top
        (1, -1),  // Top Right
        (-1, 0),  // Left
        (1, 0),   // Right
        (-1, 1),  // Bottom Left
        (0, 1),   // Bottom
        (1, 1),   // Bottom Right
    ];

    let mut p1 = 0;
    let mut p2 = 0;

    // Iterate over a time-step, looping until we find a stable state.
    for t in 0.. {
        // This is a neat trick where we can infinite loop as needed, but keep track of time too.
        // Create a new hashmap to hold future elf positions.
        let mut futures: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for &(x, y) in data.iter() {
            // So basically check to see if the data set contains the current position + the direction offset of any of the 8 directions.
            // So for example if I have x, y. Then I want to check if the data set contains coordinates (x-1, y-1), (x-1, y), etc..
            let neighbours = directions
                .iter()
                .map(|&(dx, dy)| (x + dx, y + dy))
                .map(|pos| data.contains(&pos))
                .collect_vec();

            // Check to see if all neighbours are false, if so we just continue to the next iteration.
            // Since if there are no neighbours, then we don't need to do anything/elf won't move.
            if neighbours.iter().all(|&n| !n) {
                continue;
            }

            // Create an array for possible future positions, based on current neighbours present.
            let possibilities = [
                (
                    !neighbours[1] && !neighbours[2] && !neighbours[0],
                    (x, y - 1),
                ), // If no elf on N, NE or NW, move to N.
                (
                    !neighbours[6] && !neighbours[7] && !neighbours[5],
                    (x, y + 1),
                ), // No elf S, SE or SW, move to S.
                (
                    !neighbours[0] && !neighbours[5] && !neighbours[3],
                    (x - 1, y),
                ), // No elf W, NW or SW, move to W.
                (
                    !neighbours[2] && !neighbours[7] && !neighbours[4],
                    (x + 1, y),
                ), // No elf E, NE or SE, move to E.
            ];

            for i in 0..4 {
                // Get the current possibility, make sure we account for where in the possibilities array to start, since it rotates.
                let (can_move, pos) = possibilities[(i + t) % 4];
                if can_move {
                    futures.entry(pos).or_default().push((x, y));
                    break;
                }
            }
        }

        // Check if we have moved any elves, if not, we have reached a stable state.
        let mut moved_elves = false;
        // Go through the positions and possibilities from the futures hashmap.
        for (pos, possibilities) in futures {
            // If there is only one possibility, then we can move the elf to that position.
            if possibilities.len() == 1 {
                // Remove the current position from the data set.
                data.remove(&possibilities[0]);
                // Add the new position to the data set.
                data.insert(pos);
                // Set moved_elves to true, since we have moved an elf.
                moved_elves = true;
            }
        }

        // If we have moved no elves, then we have reached a stable state.
        if !moved_elves {
            p2 = (t as i32) + 1;
            break;
        }

        // If the time is 9, then we cna solve part 1.
        if t == 9 {
            // Calculate the minimum and maximum x and y coordinates.
            let (min_x, max_x) = data.iter().map(|&(x, _)| x).minmax().into_option().unwrap();
            let (min_y, max_y) = data.iter().map(|&(_, y)| y).minmax().into_option().unwrap();
            p1 = (1 + max_x - min_x) * (1 + max_y - min_y) - data.len() as i32;
        }
    }

    (p1, p2)
}

pub fn parse(data: &[String]) -> HashSet<(i32, i32)> {
    // Create a HashSet of the data by iterating over the lines and for each location, get the x and y coordinates and put them in a tuple if the character is a '#'.
    data.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&mut parse(&crate::library::read_file("data/day23.txt")));
    println!("Day 23:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day23.txt"));
    c.bench_function("Day 23 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 23 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day23.txt"));
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
        let res = solve(&mut parse(&crate::library::read_file("testdata/day23.txt")));
        assert_eq!(res.0, 110);
        println!("Part 1: Expected: 110, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&mut parse(&crate::library::read_file("testdata/day23.txt")));
        assert_eq!(res.1, 20);
        println!("Part 2: Expected: 20, Actual: {}", res.1);
    }
}
