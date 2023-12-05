// https://adventofcode.com/2022/day/XX

pub fn solve(data: &[String]) -> (i32, i32) {
    (0, 0)
}

pub fn parse(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/dayXX.txt")));
    println!("Day XX:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/dayXX.txt"));
    c.bench_function("Day XX", |b| b.iter(|| solve(&data)));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let res = solve(&parse(&crate::library::read_file("testdata/dayXX.txt")));
        assert_eq!(res.0, 0);
        println!("Part 1: Expected: 0, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/dayXX.txt")));
        assert_eq!(res.1, 0);
        println!("Part 2: Expected: 0, Actual: {}", res.1);
    }
}
