// https://adventofcode.com/2022/day/12

use pathfinding::prelude::bfs; // https://docs.rs/pathfinding/latest/pathfinding/directed/bfs/fn.bfs.html
use pathfinding::prelude::Matrix; // https://docs.rs/pathfinding/latest/pathfinding/matrix/index.html

pub fn solve(data: &str) -> (i32, i32) {
    // Parse the input data into a grid of characters.
    let mut grid = Matrix::from_rows(data.lines().map(str::bytes)).unwrap();

    // Find the start and end points.
    let start = grid.indices().find(|&i| grid[i] == b'S').unwrap();
    let end = grid.indices().find(|&i| grid[i] == b'E').unwrap();

    // Replace the start and end points with values of 'a' and 'z' as bytes.
    grid[start] = b'a';
    grid[end] = b'z';

    // Run the BFS search, from the start to the end and get the path length.
    let (g, s, e) = &(grid, start, end);

    let p1 = bfs(
        s,
        |&i| g.neighbours(i, false).filter(move |&j| g[j] <= g[i] + 1),
        |&i| i == *e,
    )
    .unwrap()
    .len() as i32;

    let p2 = bfs(
        e,
        |&i| g.neighbours(i, false).filter(move |&j| g[i] <= g[j] + 1),
        |&i| g[i] == b'a',
    )
    .unwrap()
    .len() as i32;

    (p1 - 1, p2 - 1)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day12.txt").join("\n"));
    println!("Day 12:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day12.txt"));
    c.bench_function("Day 12 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 12 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day12.txt"));
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
        let res = solve(&crate::library::read_file("testdata/day12.txt").join("\n"));
        assert_eq!(res.0, 31);
        println!("Part 1: Expected: 31, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day12.txt").join("\n"));
        assert_eq!(res.1, 29);
        println!("Part 2: Expected: 29, Actual: {}", res.1);
    }
}
