// https://adventofcode.com/2022/day/3

pub fn solve(data: &[String]) -> (i32, i32) {
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;

    for i in (0..data.len()).step_by(3) {
        // Make a tuple of 3 lines.
        let (a, b, c) = (&data[i], &data[i + 1], &data[i + 2]);

        // Split each line into 2 parts and then get the s1 value for each line.
        let (a1, a2) = a.split_at(a.len() / 2);
        let (b1, b2) = b.split_at(b.len() / 2);
        let (c1, c2) = c.split_at(c.len() / 2);

        s1 += get_value(a1.chars().find(|c| a2.contains(*c)).unwrap());
        s1 += get_value(b1.chars().find(|c| b2.contains(*c)).unwrap());
        s1 += get_value(c1.chars().find(|c| c2.contains(*c)).unwrap());

        // Get the one char that is in all 3 lines.
        s2 += get_value(
            c.chars()
                .find(|c| a.contains(*c) && b.contains(*c))
                .unwrap(),
        );
    }

    (s1, s2)
}

// Function that will get value of a character given to it. a->z = 1 to 26. A->Z = 27 to 52.
fn get_value(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 64 + 26,
        _ => 0,
    }
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day03.txt"));
    println!("Day 03:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day03.txt"));
    c.bench_function("Day 03 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 03 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day03.txt"));
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
        let res = solve(&crate::library::read_file("testdata/day03.txt"));
        assert_eq!(res.0, 157);
        println!("Part 1: Expected: 157, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day03.txt"));
        assert_eq!(res.1, 70);
        println!("Part 2: Expected: 70, Actual: {}", res.1);
    }
}
