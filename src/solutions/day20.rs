// https://adventofcode.com/2022/day/20

pub fn solve(data: &[i64]) -> (i64, i64) {
    let p1 = mix(data, 1, 1);
    let p2 = mix(data, 10, 811589153);
    (p1, p2)
}

pub fn mix(data: &[i64], repetitions: i64, decrypt_key: i64) -> i64 {
    // Create a vector of numbers which are the values of the data vector multiplied by the decrypt_key.
    let data = data.iter().map(|x| x * decrypt_key).collect::<Vec<i64>>();
    // Create a vector of numbers which are the indices of the data vector.. basically [0, 1, 2... etc]
    let mut mixed = (0..data.len()).collect::<Vec<usize>>();

    // Iterate.
    for _ in 0..repetitions {
        for (i, &n) in data.iter().enumerate() {
            // Get the position of a number of a value in the mixed vector.
            let pos = mixed.iter().position(|&j| j == i).unwrap();
            // Remove the number at that position.
            mixed.remove(pos);
            // Insert the number from the data vector at the correct position, this way the # of values in the mixed vector are correct.
            mixed.insert((pos as i64 + n).rem_euclid(mixed.len() as i64) as usize, i);
        }
    }

    // Return the sum of the values at the correct positions.
    [1000, 2000, 3000]
        .iter()
        .map(|i| {
            data[mixed[(mixed
                .iter()
                .position(|&i| i == data.iter().position(|&i| i == 0).unwrap())
                .unwrap()
                + i)
                % mixed.len()]]
        })
        .sum::<i64>()
}

pub fn parse(data: &[String]) -> Vec<i64> {
    // Read in a number per line into the output vector, numbers can be negative.
    let mut res = Vec::new();
    for line in data {
        res.push(line.parse::<i64>().unwrap());
    }
    res
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day20.txt")));
    println!("Day 20:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day20.txt"));
    c.bench_function("Day 20 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 20 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day20.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day20.txt")));
        assert_eq!(res.0, 3);
        println!("Part 1: Expected: 3, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day20.txt")));
        assert_eq!(res.1, 1623178306);
        println!("Part 2: Expected: 1623178306, Actual: {}", res.0);
    }
}
