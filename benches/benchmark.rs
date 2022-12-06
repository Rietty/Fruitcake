// Benchmarking system for my Advent of Code solutions.
use automation::include_days_with_paths;
#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Load library module.
#[path = "../src/library.rs"]
mod library;

// Load modules for my solutions.
include_days_with_paths!();

criterion_group!(
    benches,
    day01::benchmark,
    day02::benchmark,
    day03::benchmark,
    day04::benchmark,
    day05::benchmark,
    day06::benchmark,
);

criterion_main!(benches);
