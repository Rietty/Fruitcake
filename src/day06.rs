// https://adventofcode.com/2022/day/6

use itertools::Itertools;

pub fn solve(data: &[String]) -> (i32, i32) {
    let s = &data[0];

    let p1 = s
        .chars()
        .collect_vec()
        .windows(4)
        .take_while(|c| not_unique(c.iter().collect::<String>().as_str()))
        .count() as i32;

    let p2 = s
        .chars()
        .collect_vec()
        .windows(14)
        .take_while(|c| not_unique(c.iter().collect::<String>().as_str()))
        .count() as i32;

    (p1 + 4, p2 + 14)
}

fn not_unique(rng: &str) -> bool {
    let mut _mask = 0;
    for c in rng.chars() {
        _mask |= 1 << (c as u64 - 'A' as u64);
    }
    let mut set = std::collections::HashSet::new();
    for c in rng.chars() {
        set.insert(c);
    }
    set.len() != rng.len()
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
