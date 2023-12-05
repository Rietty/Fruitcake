// https://adventofcode.com/2022/day/25

pub fn solve(data: &[String]) -> (String, String) {
    // Solve p1 by converting the SNAFU to a decimal number, summing them, and converting the sum back to SNAFU.
    let sum = data.iter().map(|s| snafu_to_decimal(s)).sum::<i64>();
    // Use the recursive function to create the SNAFU for the sum.
    (decimal_to_snafu(sum), "Congratulations!".to_string())
}

fn snafu_to_decimal(s: &str) -> i64 {
    s.chars().fold(0, |n, d| {
        // https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold
        n * 5
            + match d {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(), // https://doc.rust-lang.org/std/macro.unreachable.html
            }
    })
}

fn decimal_to_snafu(n: i64) -> String {
    if n == 0 {
        "".to_string()
    } else {
        decimal_to_snafu((n + 2) / 5) + ["0", "1", "2", "=", "-"][n as usize % 5]
    }
}

pub fn parse(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day25.txt")));
    println!("Day 25:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day25.txt"));
    c.bench_function("Day 25 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 25 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day25.txt"));
            solve(&data)
        })
    });
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let res = solve(&parse(&crate::library::read_file("testdata/day25.txt")));
        assert_eq!(res.0, "2=-1=0");
        println!("Part 1: Expected: 2=-1=0, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day25.txt")));
        assert_eq!(res.1, "Congratulations!");
        println!("Part 2: Expected: Congratulations!, Actual: {}", res.1);
    }
}
