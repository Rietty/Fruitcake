// https://adventofcode.com/2022/day/8

pub fn solve(data: &Vec<Vec<i32>>) -> (i32, i32) {
    // Grab the size of the tree vector.
    let rows = data.len();
    let cols = data[0].len();

    // Create a 2D vector of size rows x cols filled with 0s.
    let mut outside_vis = vec![vec![0; cols]; rows];
    let mut scenic_vis = vec![vec![1; cols]; rows];

    // Next we need a simple way to check if the given coordinates are valid, can use a closure for this, and ensure they are in range.
    let is_valid = |x: i32, y: i32| x < rows as i32 && y < cols as i32 && x >= 0 && y >= 0;

    // Create a closure called process that will run a bunch of steps and process the data.
    let mut process_outside = |mut x: usize, mut y: usize, factor: (i32, i32)| {
        let mut max: i32 = -1; // Setting this to -1 will ensure that the first value is always greater than it, especially if it is a 0.

        while is_valid(x as i32, y as i32) {
            if data[x][y] > max {
                outside_vis[x][y] = 1;
                max = data[x][y];
            }

            x += factor.0 as usize;
            y += factor.1 as usize;
        }
    };

    let mut process_scenic = |mut x: usize, mut y: usize, factor: (i32, i32)| {
        // Create an index variable to keep track of stuff for stack.
        let mut index = 0;

        // Create a stack by using a vector that contains a tuple with initial value of (10, 0).
        let mut stack = vec![(10, 0)];

        // While we have a valid index..
        while is_valid(x as i32, y as i32) {
            // While stack's last element's first value is less than the value at the current index...
            while stack.last().unwrap().0 < data[x][y] {
                // Pop the last element off the stack.
                stack.pop();
            }

            // Update the visibility of the current index by multiplying out with the index minus the stack's last element's second value.
            scenic_vis[x][y] *= index - stack.last().unwrap().1;
            // Add to the stack the current value and the current index.
            stack.push((data[x][y], index));

            // Increment the x, y by the factor.
            x += factor.0 as usize;
            y += factor.1 as usize;
            // Increment the index.
            index += 1;
        }
    };

    // For the range from 0 to rows... call the process closure with the current row and the current column, and a factor of 0, 1.
    for i in 0..rows {
        process_outside(i, 0, (0, 1));
        process_outside(i, cols - 1, (0, -1));
        process_scenic(i, 0, (0, 1));
        process_scenic(i, cols - 1, (0, -1));
    }

    // For the range from 0 to cols... call the process closure with the current row and the current column, and a factor of 1, 0.
    for i in 0..cols {
        process_outside(0, i, (1, 0));
        process_outside(rows - 1, i, (-1, 0));
        process_scenic(0, i, (1, 0));
        process_scenic(rows - 1, i, (-1, 0));
    }

    // Sum the outside visibility by flattening the vector and summing the values.
    let p1 = outside_vis.iter().flatten().sum();
    let p2 = *scenic_vis.iter().flatten().max().unwrap();

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<Vec<i32>> {
    data.iter()
        .map(|digits| {
            digits
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day08.txt")));
    println!("Day 08:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day08.txt"));
    c.bench_function("Day 08 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 08 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day08.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day08.txt")));
        assert_eq!(res.0, 21);
        println!("Part 1: Expected: 21, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day08.txt")));
        assert_eq!(res.1, 8);
        println!("Part 2: Expected: 8, Actual: {}", res.1);
    }
}
