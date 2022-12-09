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
    head: (i32, i32),
    tail: (i32, i32),
    visited_positions: HashSet<(i32, i32)>,
}

// Implement the rope structure.
impl Rope {
    fn new() -> Rope {
        Rope {
            head: (0, 0),
            tail: (0, 0),
            visited_positions: HashSet::new(),
        }
    }

    // Move the head, and then the tail accordingly.
    fn move_dir(&mut self, dir: (i32, i32)) {
        // Modify the head position by adding the direction vector.
        self.head.0 += dir.0;
        self.head.1 += dir.1;

        if self
            .head
            .0
            .abs_diff(self.tail.0)
            .max(self.head.1.abs_diff(self.tail.1))
            > 1
        {
            match self.head.0.cmp(&self.tail.0) {
                std::cmp::Ordering::Less => self.tail.0 -= 1,
                std::cmp::Ordering::Greater => self.tail.0 += 1,
                std::cmp::Ordering::Equal => {}
            }

            match self.head.1.cmp(&self.tail.1) {
                std::cmp::Ordering::Less => self.tail.1 -= 1,
                std::cmp::Ordering::Greater => self.tail.1 += 1,
                std::cmp::Ordering::Equal => {}
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
                self.mark_visited(self.tail);
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
    let mut rope = Rope::new();
    let movements = parse_movements(data);
    rope.simulate(movements);
    let p1 = rope.visited_positions.len();
    (p1.try_into().unwrap(), 0)
}

// Parsing function takes a vector of strings with format like: D #, where D is a direction and # is a number.

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day09.txt"));
    println!("Day 09:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = crate::library::read_file("data/day09.txt");
    c.bench_function("Day 09", |b| b.iter(|| solve(&data)));
}
