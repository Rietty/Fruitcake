// Load modules for my solutions.
mod day01;
mod day02;

mod library;

fn main() {
    {
        // Day 1:
        let res = day01::solve(&library::read_file("data/day01.txt"));
        println!("Day 01:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
    }
    {
        // Day 2:
        let res = day02::solve(&library::read_file("data/day02.txt"));
        println!("Day 02:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
    }
}
