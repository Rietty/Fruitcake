// https://adventofcode.com/2022/day/16

pub fn solve(_data: &[String]) -> (i32, i32) {
    (0, 0)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day16.txt"));
    println!("Day 16:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = crate::library::read_file("data/day16.txt");
    c.bench_function("Day 16", |b| b.iter(|| solve(&data)));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let res = solve(&crate::library::read_file("testdata/day16.txt"));
        assert_eq!(res.0, 0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day16.txt"));
        assert_eq!(res.1, 0);
    }
}
