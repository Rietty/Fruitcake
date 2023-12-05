// https://adventofcode.com/2022/day/6

pub fn solve(data: &str) -> (i32, i32) {
    let p1 = data
        .as_bytes()
        .windows(4)
        .take_while(|c| not_unique(c))
        .count() as i32;

    let p2 = data
        .as_bytes()
        .windows(14)
        .take_while(|c| not_unique(c))
        .count() as i32;

    (p1 + 4, p2 + 14)
}

fn not_unique(rng: &[u8]) -> bool {
    let mut _mask: i32 = 0;
    for c in rng {
        _mask |= 1 << (*c - b'A');
    }
    _mask.count_ones() != rng.len() as u32
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day06.txt")[0]);
    println!("Day 06:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day06.txt"));
    c.bench_function("Day 06 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 06 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day06.txt"));
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
        let res = solve(&crate::library::read_file("testdata/day06.txt")[0]);
        assert_eq!(res.0, 7);
        println!("Part 1: Expected 7: Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day06.txt")[0]);
        assert_eq!(res.1, 19);
        println!("Part 2: Expected 19: Actual: {}", res.1);
    }
}
