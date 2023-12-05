// https://adventofcode.com/2022/day/2

pub fn solve(data: &[String]) -> (i32, i32) {
    // Get current score for p1.
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;

    // Get the ASCII value for character's 'A' and 'X'.
    let a = b'A';
    let x = b'X';

    for line in data {
        // Destructure the line into 2 characters and remove any whitespace.
        let (c1, c2) = line.split_at(1);
        let c2 = c2.trim();

        // Get the ASCII value for the character and subtract a or x to get the relative position.
        let c1 = c1.as_bytes()[0] - a;
        let c2 = c2.as_bytes()[0] - x;

        p1 += (c2 + 1 + ((c2 - c1 + 4) % 3) * 3) as i32;
        p2 += (c2 * 3 + ((c2 + c1 + 2) % 3) + 1) as i32;
    }

    (p1, p2)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day02.txt"));
    println!("Day 02:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day02.txt"));
    c.bench_function("Day 02 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 02 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day02.txt"));
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
        let res = solve(&crate::library::read_file("testdata/day02.txt"));
        assert_eq!(res.0, 15);
        println!("Part 1: Expected: 15, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day02.txt"));
        assert_eq!(res.1, 12);
        println!("Part 2: Expected: 12, Actual: {}", res.1);
    }
}
