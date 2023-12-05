// https://adventofcode.com/2022/day/9

// Imports
use std::collections::HashSet;

// Define some constant directions for movement.
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);
const UP: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (0, -1);

// Rope structure that contains postion of the head and tail.
struct Rope {
    knots: Vec<(i32, i32)>,
    visited_positions: HashSet<(i32, i32)>,
}

// Implement the rope structure.
impl Rope {
    fn new(size: usize) -> Rope {
        Rope {
            knots: vec![(0, 0); size],
            visited_positions: HashSet::new(),
        }
    }

    // Move the head, and then the tail accordingly.
    fn move_dir(&mut self, dir: (i32, i32)) {
        // Modify the head position by adding the direction vector.
        self.knots[0].0 += dir.0;
        self.knots[0].1 += dir.1;

        // Set up a loop to move all the knots.
        for i in 1..self.knots.len() {
            // Make a head variable equal to the previous knot.
            let head = self.knots[i - 1];
            // Tail is the last knot.
            let tail = self.knots[i];

            if head.0.abs_diff(tail.0).max(head.1.abs_diff(tail.1)) > 1 {
                match head.0.cmp(&tail.0) {
                    std::cmp::Ordering::Less => self.knots[i].0 -= 1,
                    std::cmp::Ordering::Greater => self.knots[i].0 += 1,
                    std::cmp::Ordering::Equal => {}
                }

                match head.1.cmp(&tail.1) {
                    std::cmp::Ordering::Less => self.knots[i].1 -= 1,
                    std::cmp::Ordering::Greater => self.knots[i].1 += 1,
                    std::cmp::Ordering::Equal => {}
                }
            }
        }
    }

    fn mark_visited(&mut self, pos: (i32, i32)) {
        self.visited_positions.insert(pos);
    }

    // Simulate according to a set of instructions.
    fn simulate(&mut self, moves: Vec<(char, i32)>) {
        for mv in moves {
            // What happens if we split the moves by whitespace, and then filter out the empty strings?
            let (dir, steps) = mv;

            let dir = match dir {
                'L' => LEFT,
                'R' => RIGHT,
                'U' => UP,
                'D' => DOWN,
                _ => panic!("Invalid direction!"),
            };

            // Move that many times using the move_dir function.
            for _ in 0..steps {
                self.move_dir(dir);
                self.mark_visited(self.knots[self.knots.len() - 1]);
            }
        }
    }
}

fn parse_movements(movements: &[String]) -> Vec<(char, i32)> {
    let mut parsed_movements = Vec::new();

    // Iterate over the lines in the movements string
    for line in movements {
        // Parse the direction and number of steps from the line
        let direction = line.chars().next().unwrap();
        let steps = line[1..].trim().parse::<i32>().unwrap();

        // Add the parsed movement to the vector
        parsed_movements.push((direction, steps));
    }

    parsed_movements
}

pub fn solve(data: &[String]) -> (i32, i32) {
    let mut rope = Rope::new(2);
    let movements = parse_movements(data);
    rope.simulate(movements);

    let p1 = rope.visited_positions.len();

    let mut rope = Rope::new(10);
    let movements = parse_movements(data);
    rope.simulate(movements);

    let p2 = rope.visited_positions.len();

    (p1.try_into().unwrap(), p2.try_into().unwrap())
}

// Parsing function takes a vector of strings with format like: D #, where D is a direction and # is a number.

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day09.txt"));
    println!("Day 09:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day09.txt"));
    c.bench_function("Day 09 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 09 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day09.txt"));
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
        let res = solve(&crate::library::read_file("testdata/day09.txt"));
        assert_eq!(res.0, 13);
        println!("Part 1: Expected: 13, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day09.txt"));
        assert_eq!(res.1, 1);
        println!("Part 2: Expected: 1, Actual: {}", res.1);
    }
}
