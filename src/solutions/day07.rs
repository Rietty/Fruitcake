// https://adventofcode.com/2022/day/7

pub fn solve(data: &[String]) -> (i32, i32) {
    (0, 0)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day07.txt"));
    println!("Day 07:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = crate::library::read_file("data/day07.txt");
    c.bench_function("Day 07", |b| b.iter(|| solve(&data)));
}
