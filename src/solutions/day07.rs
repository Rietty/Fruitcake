// https://adventofcode.com/2022/day/7

use std::collections::HashMap;
use std::path::PathBuf;

pub fn solve(data: &[String]) -> (i32, i32) {
    // Create a vector to store the current directory-path we're in, hash-map to store the size of each directory.
    let mut path = Vec::new();
    let mut dir_sizes: HashMap<PathBuf, i32> = HashMap::new();

    for line in data {
        // Need to skip if our op is a lookup or a dir op.
        if line.starts_with("dir") || line.starts_with("$ ls") {
            continue;
        }

        // Match for either the change directories, or on the file size + name. Can match an entire vector at once.
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            // Really cool syntax to match an array's parts all at once.
            // If we leave a directory, pop that out of our path vector.
            ["$", "cd", ".."] => {
                path.pop();
            }

            // Need to do next part second because if you put it before other it causes issues due to the capture variable resulting in unreached statements in the match expression.
            ["$", "cd", name] => {
                // The random value can be captured into name variable. Kinda like a regex.
                path.push(name);
            }

            // If we have a file, that is a size and then file_name we can just add the size to the hash-map with the key as a collection of the path.
            [size, _name] => {
                // Don't need _name, but use variable since it can be discarded and changed.
                let size: i32 = size.parse().unwrap();
                // For each path, that is for example we have something like dev/dir/a/b/c, we want to add the size at each step, so each directory always reflects the true size.
                for i in 0..path.len() {
                    let p = PathBuf::from_iter(&path[..=i]); // Build a path from the path vector so we can use it as a key.
                    *dir_sizes.entry(p).or_insert(0) += size; // So access entry, if it doesn't exist init it to 0.
                }
            }

            _ => {}
        };
    }

    // Sum of all directories under 100k. Need to look into how I can make the numbers easier to read.
    // Clone the hashmap so we can iterate over it twice.
    // The _ is for reading numbers in easily.
    let p1 = dir_sizes
        .clone()
        .into_values()
        .filter(|s| *s <= 100_000)
        .sum();

    // Find and delete the smallest available directory that will result in the space needed.
    let usable = 70_000_000 - dir_sizes.get(&PathBuf::from("/")).unwrap();
    let p2 = dir_sizes
        .into_values()
        .filter(|s| usable + s >= 30_000_000)
        .min()
        .unwrap();

    (p1, p2)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day07.txt"));
    println!("Day 07:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day07.txt"));
    c.bench_function("Day 07 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 07 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day07.txt"));
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
        let res = solve(&crate::library::read_file("testdata/day07.txt"));
        assert_eq!(res.0, 95437);
        println!("Part 1: Expected: 95437, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day07.txt"));
        assert_eq!(res.1, 24933642);
        println!("Part 2: Expected: 24933642, Actual: {}", res.1);
    }
}
