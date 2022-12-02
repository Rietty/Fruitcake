// Imports
use std::env;

// Load modules for my solutions.
mod day01;
mod day02;

mod library;

fn main() {
    // Use a pattern matching system to run a solution for a given day, or a wildcard to run all solutions.
    // Get arguments from the command line.
    let args: Vec<String> = env::args().collect();
    let mut day: &str = "*";
    // If there are no arguments (i.e size is 1), run all solutions.
    if args.len() == 1 {
        println!("No arguments provided, running all solutions.");
    } else if args.len() == 2 {
        // If there is an argument, use the first one as the day to run.
        day = &args[1];
    } else {
        // If there are more than 2 arguments, print an error and exit.
        println!("Too many arguments provided, exiting.");
        std::process::exit(1);
    }

    // Run the solution for the given day, or all days if we have a *.
    match day {
        "1" => day01::run(),
        "2" => day02::run(),
        // "3" => day03::run(),
        // "4" => day04::run(),
        // "5" => day05::run(),
        // "6" => day06::run(),
        // "7" => day07::run(),
        // "8" => day08::run(),
        // "9" => day09::run(),
        // "10" => day10::run(),
        // "11" => day11::run(),
        // "12" => day12::run(),
        // "13" => day13::run(),
        // "14" => day14::run(),
        // "15" => day15::run(),
        // "16" => day16::run(),
        // "17" => day17::run(),
        // "18" => day18::run(),
        // "19" => day19::run(),
        // "20" => day20::run(),
        // "21" => day21::run(),
        // "22" => day22::run(),
        // "23" => day23::run(),
        // "24" => day24::run(),
        // "25" => day25::run(),
        "*" => {
            day01::run();
            day02::run();
            // day03::run();
            // day04::run();
            // day05::run();
            // day06::run();
            // day07::run();
            // day08::run();
            // day09::run();
            // day10::run();
            // day11::run();
            // day12::run();
            // day13::run();
            // day14::run();
            // day15::run();
            // day16::run();
            // day17::run();
            // day18::run();
            // day19::run();
            // day20::run();
            // day21::run();
            // day22::run();
            // day23::run();
            // day24::run();
            // day25::run();
        }
        _ => println!("Invalid day provided, exiting."),
    }
}
