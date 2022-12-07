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
);

criterion_main!(benches);
