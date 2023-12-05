// https://adventofcode.com/2022/day/5

pub fn solve(mut cr: Vec<Vec<char>>, data: &[(i32, i32, i32)]) -> (String, String) {
    // Create a copy of the crates for the second part
    let mut cr2 = cr.clone();

    // Combine the two loops into one to solve both parts at the same time.
    for (a, b, c) in data {
        // Create a temporary vector to hold the crates.
        let mut temp = Vec::new();

        for _ in 0..*a {
            // Get last element in vector B - 1 and push it to the temporary vector.
            let last = cr[*b as usize - 1].pop().unwrap_or_default();
            cr[*c as usize - 1].push(last);

            let last = cr2[*b as usize - 1].pop().unwrap_or_default();
            if last != '\0' {
                temp.push(last);
            }
        }

        // Reverse the temporary vector.
        temp.reverse();

        // Push the elements from the temporary vector to the end of vector C - 1.
        for i in temp {
            cr2[*c as usize - 1].push(i);
        }
    }

    // Print the last letter of each vector inside crates as a single string.
    let p1 = cr.iter().map(|v| v.last().unwrap()).collect::<String>();
    let p2 = cr2.iter().map(|v| v.last().unwrap()).collect::<String>();

    // Return the result and the number of crates.
    (p1, p2)
}

#[allow(clippy::type_complexity)]
fn parse(data: &[String]) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>) {
    // Data comes in a series of lines. Split into two different vectors of strings based on the first empty line.
    let (header, data) = data.split_at(data.iter().position(|s| s.is_empty()).unwrap());

    // Get rid of the last line in the header, then reverse the header.
    let header = header[0..header.len()]
        .iter()
        .rev()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // Create the initial crates.
    let mut cr = vec![vec![]; header[0].split_whitespace().count()];

    // For each line in header except first line.
    for line in header[1..].iter() {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| cr[i].push(c));
    }

    // Read in the data and return a vector of (i32, i32, i32), of the numbers in the data.
    let mut numbers = Vec::new();

    // Iterate over the vector of strings, skipping the first line.
    for line in data.iter().skip(1) {
        // Split the line into a vector of strings.
        let split: Vec<&str> = line.split(' ').collect();

        let a = split[1].parse::<i32>().unwrap();
        let b = split[3].parse::<i32>().unwrap();
        let c = split[5].parse::<i32>().unwrap();

        // Push the tuple onto the vector.
        numbers.push((a, b, c));
    }

    (cr, numbers)
}

#[allow(dead_code)]
pub fn run() {
    let (header, data) = &parse(&crate::library::read_file("data/day05.txt"));
    let res = solve(header.to_vec(), data);
    println!("Day 05:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day05.txt"));
    c.bench_function("Day 05 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 05 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day05.txt"));
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
        let (header, data) = &parse(&crate::library::read_file("testdata/day05.txt"));
        let res = solve(header.to_vec(), data);
        assert_eq!(res.0, "CMZ");
        println!("Part 1: Expected: CMZ, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let (header, data) = &parse(&crate::library::read_file("testdata/day05.txt"));
        let res = solve(header.to_vec(), data);
        assert_eq!(res.1, "MCD");
        println!("Part 2: Expected: MCD, Actual: {}", res.1);
    }
}
