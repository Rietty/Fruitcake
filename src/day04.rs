// https://adventofcode.com/2022/day/4

use crate::library::Pair;

pub fn solve(data: &Vec<(Pair, Pair)>) -> (i32, i32) {
    let mut c1 = 0;
    let mut c2 = 0;

    for line in data {
        let (one, two) = line;

        // Count how many ranges are fully enclosed by the other.
        if (one.x <= two.x && one.y >= two.y) || (two.x <= one.x && two.y >= one.y) {
            c1 += 1;
        }

        // Count how many ranges overlap at any point.
        if (one.x <= two.x && one.y >= two.x) || (two.x <= one.x && two.y >= one.x) {
            c2 += 1;
        }
    }

    (c1, c2)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file_vttuples("data/day04.txt"));
    println!("Day 04:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = crate::library::read_file_vttuples("data/day04.txt");
    c.bench_function("Day 04", |b| b.iter(|| solve(&data)));
}
