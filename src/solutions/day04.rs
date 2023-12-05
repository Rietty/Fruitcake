// https://adventofcode.com/2022/day/4

#[allow(clippy::type_complexity)]
pub fn solve(data: &[((i32, i32), (i32, i32))]) -> (i32, i32) {
    // Process the data
    data.iter()
        .fold((0, 0), |(mut c1, mut c2), (first, second)| {
            if (first.0 <= second.0 && first.1 >= second.1)
                || (second.0 <= first.0 && second.1 >= first.1)
            {
                c1 += 1;
            }

            if (first.0 <= second.0 && first.1 >= second.0)
                || (second.0 <= first.0 && second.1 >= first.0)
            {
                c2 += 1;
            }

            (c1, c2)
        })
}

fn parse(data: &[String]) -> Vec<((i32, i32), (i32, i32))> {
    data.iter()
        .map(|x| {
            let mut parts = x.split(',');
            let mut first = parts.next().unwrap().split('-');
            let mut second = parts.next().unwrap().split('-');
            let first = (
                first.next().unwrap().parse::<i32>().unwrap(),
                first.next().unwrap().parse::<i32>().unwrap(),
            );
            let second = (
                second.next().unwrap().parse::<i32>().unwrap(),
                second.next().unwrap().parse::<i32>().unwrap(),
            );
            (first, second)
        })
        .collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day04.txt")));
    println!("Day 04:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day04.txt"));
    c.bench_function("Day 04 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 04 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day04.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day04.txt")));
        assert_eq!(res.0, 2);
        println!("Part 1: Expected: 2, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day04.txt")));
        assert_eq!(res.1, 4);
        println!("Part 2: Expected: 4, Actual: {}", res.1);
    }
}
