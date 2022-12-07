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
        .map(|x| crate::library::split_line(x, ",", "-"))
        .map(|x| {
            let first = x.0;
            let second = x.1;
            let first = (
                first.0.parse::<i32>().unwrap(),
                first.1.parse::<i32>().unwrap(),
            );
            let second = (
                second.0.parse::<i32>().unwrap(),
                second.1.parse::<i32>().unwrap(),
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
    let data = &parse(&crate::library::read_file("data/day04.txt"));
    c.bench_function("Day 04", |b| b.iter(|| solve(data)));
}
