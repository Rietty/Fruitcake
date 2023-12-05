// https://adventofcode.com/2022/day/13

// Need the deserialize trait for the json parser.
use serde::Deserialize;

// Create an enum for the different types of data. Either a vector or a number.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)] // This was buried in the docs, but it allows us to have a vector or a number in the same enum, and serde will figure out which one it is. If you don't have it, it will try to force stuff in both and fail.
enum Data {
    Num(u8),
    Group(Vec<Data>),
}

// Need to implement ordering for the data type.
impl Eq for Data {}

impl PartialEq for Data {
    // Needed to implement this if I implement Eq.
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Data::*;
        match (self, other) {
            (Num(a), Num(b)) => a.cmp(b),               // If both are numbers.
            (Group(a), Group(b)) => a.cmp(b),           // If both are groups.
            (Num(a), Group(b)) => [Num(*a)][..].cmp(b), // If one is a number and one is a group, we need to treat the number as a group of 1.
            (Group(a), Num(b)) => a.as_slice().cmp(&[Num(*b)]), // If one is a group, and other is a number, we basically do the same as above, just need to rewrite it to handle the other way around.
        }
    }
}

impl PartialOrd for Data {
    // Needed to implement this if I implement Ord.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve(data: &[String]) -> (i32, i32) {
    // Set the data to be a vector of serde_json::from_str::Data objects.
    let mut data = data
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| serde_json::from_str::<Data>(x).unwrap())
        .collect::<Vec<Data>>();

    // Step through the data in sets of 2 lines, a and b, and compare them. If b > a, then add the index of the line to the sum and continue. Use an enumerate to get the index.
    let mut p1 = 0;

    for (i, (a, b)) in data
        .iter()
        .step_by(2)
        .zip(data.iter().skip(1).step_by(2))
        .enumerate()
    {
        if a < b {
            p1 += (i + 1) as i32;
        }
    }

    // Create a group with value of [2], and a group with value of [6].
    let a: Data = serde_json::from_str("[[2]]").unwrap();
    let b: Data = serde_json::from_str("[[6]]").unwrap();

    // Insert the groups into the data vector.
    data.insert(0, a.clone());
    data.insert(1, b.clone());

    // Sort the data.
    data.sort();

    let p2 = (((data.iter().position(|x| x == &a).unwrap()) + 1)
        * ((data.iter().position(|x| x == &b).unwrap()) + 1)) as i32;

    (p1, p2)
}

// Parser that will read in the data and return a vector of strings. If a line is empty, it will be skipped.
#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day13.txt"));
    println!("Day 13:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day13.txt"));
    c.bench_function("Day 13 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 13 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day13.txt"));
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
        let res = solve(&crate::library::read_file("testdata/day13.txt"));
        assert_eq!(res.0, 13);
        println!("Part 1: Expected: 13, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day13.txt"));
        assert_eq!(res.1, 140);
        println!("Part 2: Expected: 140, Actual: {}", res.1);
    }
}
