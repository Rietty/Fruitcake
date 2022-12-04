// https://adventofcode.com/2022/day/4

use crate::library::Pair;

pub fn solve(data: &[(Pair, Pair)]) -> (i32, i32) {
    data.iter()
        .fold((0, 0), |(mut c1, mut c2), (first, second)| {
            if (first.x <= second.x && first.y >= second.y)
                || (second.x <= first.x && second.y >= first.y)
            {
                c1 += 1;
            }

            if (first.x <= second.x && first.y >= second.x)
                || (second.x <= first.x && second.y >= first.x)
            {
                c2 += 1;
            }

            (c1, c2)
        })
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
