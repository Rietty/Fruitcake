// https://adventofcode.com/2022/day/1

pub fn solve(data: &[String]) -> (i32, i32) {
    // For the data, sum each set of numbers separated by an empty line and store them in an array.
    let mut sums: Vec<i32> = vec![];
    let mut sum: i32 = 0;

    for line in data {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    sums.sort();
    sums.reverse();

    // Return the largest, and then the sum of 3 largest values in the vector.
    (sums[0], sums.iter().take(3).sum())
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day01.txt"));
    println!("Day 01:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day01.txt"));
    c.bench_function("Day 01 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 01 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day01.txt"));
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
        let res = solve(&crate::library::read_file("testdata/day01.txt"));
        assert_eq!(res.0, 24000);
        println!("Part 1: Expected: 24000, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day01.txt"));
        assert_eq!(res.1, 45000);
        println!("Part 2: Expected: 45000, Actual: {}", res.1);
    }
}
