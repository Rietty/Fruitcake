// https://adventofcode.com/2022/day/6

use itertools::Itertools;

pub fn solve(data: &[String]) -> (i32, i32) {
    // Get the data as a single string that we can iterate over.
    let s = &data[0];
    (find_marker(s, 4), find_marker(s, 14))
}

fn find_marker(s: &str, size: usize) -> i32 {
    *s.chars()
        .collect_vec()
        .windows(size)
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if unique_check(c.to_vec()) {
                Some(i + size)
            } else {
                None
            }
        })
        .collect_vec()
        .first()
        .unwrap() as i32
}

fn unique_check(c: Vec<char>) -> bool {
    c.clone().into_iter().unique().collect_vec() == c
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day06.txt"));
    println!("Day 06:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = crate::library::read_file("data/day06.txt");
    c.bench_function("Day 06", |b| b.iter(|| solve(&data)));
}
