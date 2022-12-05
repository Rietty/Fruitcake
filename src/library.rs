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

// Function to split a string into a tuple using a given delimiter.
pub fn split_string(s: &str, delim: &str) -> (String, String) {
    let mut split = s.split(delim);
    let first = split.next().unwrap().to_string();
    let second = split.next().unwrap().to_string();
    (first, second)
}

// Function that will take a line of data and use split_string to split it into two tuples of strings.
pub fn split_line(s: &str, fdelim: &str, sdelim: &str) -> ((String, String), (String, String)) {
    let mut split = s.split(fdelim);
    let first = split_string(split.next().unwrap(), sdelim);
    let second = split_string(split.next().unwrap(), sdelim);
    (first, second)
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
