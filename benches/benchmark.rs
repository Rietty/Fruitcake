// Benchmarking system for my Advent of Code solutions.
#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Load library module.
#[path = "../src/library.rs"]
mod library;

// Load the solutions folder mod.rs
#[path = "../src/solutions/mod.rs"]
mod solutions;

criterion_group!(
    benches,
    solutions::day01::benchmark,
    solutions::day02::benchmark,
    solutions::day03::benchmark,
    solutions::day04::benchmark,
    solutions::day05::benchmark,
    solutions::day06::benchmark,
    solutions::day07::benchmark,
    solutions::day08::benchmark,
    solutions::day09::benchmark,
    solutions::day10::benchmark,
    solutions::day11::benchmark,
    solutions::day12::benchmark,
    solutions::day13::benchmark,
    solutions::day14::benchmark,
    solutions::day15::benchmark,
    solutions::day16::benchmark,
    solutions::day17::benchmark,
    solutions::day18::benchmark,
    solutions::day19::benchmark,
    solutions::day20::benchmark,
    solutions::day21::benchmark,
    solutions::day22::benchmark,
    solutions::day23::benchmark,
    solutions::day24::benchmark,
    solutions::day25::benchmark
);

criterion_main!(benches);
