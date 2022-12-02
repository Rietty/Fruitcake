// Load modules for my solutions.
mod day01;
mod library;

fn main() {
    {
        // Day 1:
        let res = day01::solve(&library::read_file("data/day01.txt"));
        println!("Star 1: {}\nStar 2: {}\n", res.0, res.1);
    }
}
