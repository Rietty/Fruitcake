// Imports
use std::env;

dirmod::all!(except template);

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
        "1" => solutions::day01::run(),
        "2" => solutions::day02::run(),
        "3" => solutions::day03::run(),
        "4" => solutions::day04::run(),
        "5" => solutions::day05::run(),
        "6" => solutions::day06::run(),
        "7" => solutions::day07::run(),
        "8" => solutions::day08::run(),
        "9" => solutions::day09::run(),
        "10" => solutions::day10::run(),
        "11" => solutions::day11::run(),
        "12" => solutions::day12::run(),
        "13" => solutions::day13::run(),
        "14" => solutions::day14::run(),
        "15" => solutions::day15::run(),
        "16" => solutions::day16::run(),
        "17" => solutions::day17::run(),
        "18" => solutions::day18::run(),
        "19" => solutions::day19::run(),
        "20" => solutions::day20::run(),
        "21" => solutions::day21::run(),
        "22" => solutions::day22::run(),
        "23" => solutions::day23::run(),
        "24" => solutions::day24::run(),
        "25" => solutions::day25::run(),
        "*" => {
            solutions::day01::run();
            solutions::day02::run();
            solutions::day03::run();
            solutions::day04::run();
            solutions::day05::run();
            solutions::day06::run();
            solutions::day07::run();
            solutions::day08::run();
            solutions::day09::run();
            solutions::day10::run();
            solutions::day11::run();
            solutions::day12::run();
            solutions::day13::run();
            solutions::day14::run();
            solutions::day15::run();
            solutions::day16::run();
            solutions::day17::run();
            solutions::day18::run();
            solutions::day19::run();
            solutions::day20::run();
            solutions::day21::run();
            solutions::day22::run();
            solutions::day23::run();
            solutions::day24::run();
            solutions::day25::run();
        }
        _ => println!("Invalid day provided, exiting."),
    }
}
