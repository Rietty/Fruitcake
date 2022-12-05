// https://adventofcode.com/2022/day/5

pub fn solve(data: &[(i32, i32, i32)]) -> (String, String) {
    // Just hard-code initial values
    let mut cr = vec![
        vec!['B', 'Q', 'C'],
        vec!['R', 'Q', 'W', 'Z'],
        vec!['B', 'M', 'R', 'L', 'V'],
        vec!['C', 'Z', 'H', 'V', 'T', 'W'],
        vec!['D', 'Z', 'H', 'B', 'N', 'V', 'G'],
        vec!['H', 'N', 'P', 'C', 'J', 'F', 'V', 'Q'],
        vec!['D', 'G', 'T', 'R', 'W', 'Z', 'S'],
        vec!['C', 'G', 'H', 'N', 'B', 'W', 'Z', 'P'],
        vec!['N', 'J', 'B', 'M', 'W', 'Q', 'F', 'P'],
    ];

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

fn parse(data: &[String]) -> Vec<(i32, i32, i32)> {
    // Data comes in a series of lines. Split into two different vectors of strings based on the first empty line.
    let (_header, data) = data.split_at(data.iter().position(|s| s.is_empty()).unwrap());

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
    numbers
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day05.txt")));
    println!("Day 05:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day05.txt"));
    c.bench_function("Day 05", |b| b.iter(|| solve(&data)));
}
