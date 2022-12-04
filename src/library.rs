// Module to handle/act as a library for all my various solutions.

// Imports
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Read a file from a given path and return a vector of strings.
#[allow(unused)]
pub fn read_file(path: &str) -> Vec<String> {
    // Print path for debugging.
    let mut lines: Vec<String> = vec![];
    if let Ok(fs) = read_lines(path) {
        for line in fs.flatten() {
            lines.push(line);
        }
    }
    lines
}

#[derive(Debug)]
pub struct Pair {
    pub x: i32,
    pub y: i32,
}

// Function to read a file and return a vector that contains 2 pairs.
pub fn read_file_vttuples(path: &str) -> Vec<(Pair, Pair)> {
    // Read lines from file, of format #-#,#-#.
    // Put each line into a vector of tuples of tuples.
    let mut ranges: Vec<(Pair, Pair)> = vec![];
    if let Ok(fs) = read_lines(path) {
        for line in fs.flatten() {
            let mut split = line.split(',');
            let mut split2 = split.next().unwrap().split('-');
            let mut split3 = split.next().unwrap().split('-');
            ranges.push((
                Pair {
                    x: split2.next().unwrap().parse().unwrap(),
                    y: split2.next().unwrap().parse().unwrap(),
                },
                Pair {
                    x: split3.next().unwrap().parse().unwrap(),
                    y: split3.next().unwrap().parse().unwrap(),
                },
            ));
        }
    }

    ranges
}

// Read lines from a file at a given file-name.
// Open file relative to the base directory of the project.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
