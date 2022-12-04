// Benchmarking system for my Advent of Code solutions.
#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Load library module.
#[path = "../src/library.rs"]
mod library;

// Load modules for my solutions.
#[path = "../src/day01.rs"]
mod day01;

#[path = "../src/day02.rs"]
mod day02;

#[path = "../src/day03.rs"]
mod day03;

#[path = "../src/day04.rs"]
mod day04;

criterion_group!(
    benches,
    day01::benchmark,
    day02::benchmark,
    day03::benchmark,
    day04::benchmark
);

criterion_main!(benches);
