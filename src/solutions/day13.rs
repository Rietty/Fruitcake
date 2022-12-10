// https://adventofcode.com/2022/day/13

pub fn solve(data: &[String]) -> (i32, i32) {
    (0, 0)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day13.txt"));
    println!("Day 13:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = crate::library::read_file("data/day13.txt");
    c.bench_function("Day 13", |b| b.iter(|| solve(&data)));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let res = solve(&crate::library::read_file("testdata/day13.txt"));
        assert_eq!(res.0, 0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day13.txt"));
        assert_eq!(res.1, 0);
    }
}
