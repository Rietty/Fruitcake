// https://adventofcode.com/2022/day/19

use hashbrown::HashSet; // Speeds up HashSet operation by 2x.
use rayon::prelude::*;
use std::cmp::max;
use std::collections::VecDeque;

// Single blueprint. Each blueprint has 8 resources: ore, clay, obsidian, geode, ore-robots, clay-robots, obsidian-robots, geode-robots.
// It's more like a state of the blueprint, but I'm calling it blueprint for simplicity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Blueprint {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    rb_ore: i32,
    rb_clay: i32,
    rb_obsidian: i32,
    rb_geode: i32,
}

pub fn solve(data: &[(i32, i32, i32, i32, i32, i32)]) -> (i32, i32) {
    // Solve Part 1. (Sum of all the blueprints * their index).
    let p1: i32 = data
        .par_iter()
        .enumerate()
        .map(|(i, &blueprint)| process(blueprint, 24) * (i as i32 + 1))
        .sum();

    // Solve Part 2. Take the first 3 blueprints and find their product over 32 minutes.
    let p2: i32 = data[..3]
        .par_iter()
        .map(|&blueprint| process(blueprint, 32))
        .product();

    (p1, p2)
}

// This function essentially increments the count of the resources by the count of the robots. I.e. it "mines" the resources.
pub fn mine(mut blueprint: Blueprint) -> Blueprint {
    blueprint.ore += blueprint.rb_ore;
    blueprint.clay += blueprint.rb_clay;
    blueprint.obsidian += blueprint.rb_obsidian;
    blueprint.geode += blueprint.rb_geode;
    blueprint
}

// Function to process and figure out maximum yield per blueprint.
pub fn process((a, b, c, d, e, f): (i32, i32, i32, i32, i32, i32), time: i32) -> i32 {
    // A = Ore-Robot Cost
    // B = Clay-Robot Cost
    // C = Obsidian-Robot Cost (Ore)
    // D = Obsidian-Robot Cost (Clay)
    // E = Geode-Robot Cost (Ore)
    // F = Geode-Robot Cost (Obsidian)

    // Get the maximum ore cost out of all the robots.
    let ore_cost = a.max(b).max(c).max(e);

    // Create a visited hashset to keep track of the visited blueprints.
    let mut visited: HashSet<Blueprint> = HashSet::new();

    // Create a queue to keep track of the blueprints to be processed.
    let mut queue: VecDeque<(i32, Blueprint)> = VecDeque::new();

    // Add the initial blueprint to the queue, with a time of 0.
    queue.push_back((
        0,
        Blueprint {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            rb_ore: 1, // We always start with 1 ore robot.
            rb_clay: 0,
            rb_obsidian: 0,
            rb_geode: 0,
        },
    ));

    let mut res: i32 = 0;

    // Process the queue until it's empty.
    while let Some((t, mut blueprint)) = queue.pop_front() {
        // Need to check if we have passed or hit the time limit.
        if t >= time {
            res = max(res, blueprint.geode);
            continue;
        }

        // If we have already visited this blueprint, then we can skip it.
        if visited.contains(&blueprint) {
            continue;
        }

        // Remove all elements from the queue which match the filter:
        queue.retain(|(_, bp)| {
            bp.ore > blueprint.ore
                || bp.clay > blueprint.clay
                || bp.obsidian > blueprint.obsidian
                || bp.geode > blueprint.geode
                || bp.rb_ore > blueprint.rb_ore
                || bp.rb_clay > blueprint.rb_clay
                || bp.rb_obsidian > blueprint.rb_obsidian
                || bp.rb_geode > blueprint.rb_geode
        });

        // Pruning:
        // Geode Robot: Make sure we have at least 1 Obsidian Robot. Make sure we have enough ore.
        // Obsidian Robot: Make sure we have at least 1 Clay Robot. Make sure we don't make more than the obsidian required by the Geode Robot.
        // Ore Robot: Make a maximum that is equal to the highest ore count required by any robot.
        // Clay Robot: Make sure we only make a maximum of the amount of clay required by an obsidian robot.

        // Prune out based on: If geodes collected + geode robots * time remaining + T(time remaining) <= best total geodes found so far.
        // This prune is thanks to a reddit comment.
        if (blueprint.geode + blueprint.rb_geode * (time - t) + (time - t)) <= res {
            continue;
        }

        // Prune away excess resources.
        if blueprint.rb_ore >= ore_cost && blueprint.ore >= ore_cost {
            blueprint.ore = ore_cost;
        }

        if blueprint.rb_clay >= d && blueprint.ore >= ore_cost {
            blueprint.ore = ore_cost;
        }

        if blueprint.rb_obsidian >= f && blueprint.ore >= ore_cost && blueprint.clay >= d {
            blueprint.ore = ore_cost;
            blueprint.clay = d;
        }

        // Add the blueprint to the visited set.
        visited.insert(blueprint);

        // Need to make a robot which makes geodes, but don't bother if we don't have any obsidian robots.
        if blueprint.rb_obsidian >= 1 && blueprint.ore >= e && blueprint.obsidian >= f {
            // Debug:
            // println!("Making geode robot at time {t}");

            let mut blueprint = mine(blueprint);
            // Decrease the count of ore by the cost.
            blueprint.ore -= e;
            // Decrease the count of obsidian by the cost.
            blueprint.obsidian -= f;
            // Increase the count of geode robots by 1.
            blueprint.rb_geode += 1;
            // Add the new blueprint to the queue.
            queue.push_back((t + 1, blueprint));
            // Continue since we don't want to make any other robots/can't do much else here.
            continue;
        }

        // Need to make a robot which makes obsidian, but don't bother if we don't have any clay robots.
        // We also don't need to make more than the geode obsidian cost.
        if blueprint.rb_clay >= 1
            && blueprint.rb_obsidian < f
            && blueprint.ore >= c
            && blueprint.clay >= d
        {
            // Debug:
            // println!("Making obsidian robot at time {t}");

            let mut blueprint = mine(blueprint);
            // Decrease the count of ore by the cost.
            blueprint.ore -= c;
            // Decrease the count of clay by the cost.
            blueprint.clay -= d;
            // Increase the count of obsidian robots by 1.
            blueprint.rb_obsidian += 1;
            // Add the new blueprint to the queue.
            queue.push_back((t + 1, blueprint));
        }

        // For the next two robots, we don't continue, since we don't use a ton of resources.
        // Need to make these robots with whatever is left whenever we can to maximize production.
        if blueprint.rb_ore < ore_cost && blueprint.ore >= a {
            // Debug:
            // println!("Making ore robot at time {t}");

            let mut blueprint = mine(blueprint);
            // Decrease the count of ore by the cost.
            blueprint.ore -= a;
            // Increase the count of ore robots by 1.
            blueprint.rb_ore += 1;
            // Add the new blueprint to the queue.
            queue.push_back((t + 1, blueprint));
        }

        // Need to make a robot which makes clay.
        if blueprint.rb_clay < d && blueprint.ore >= b {
            // Debug:
            // println!("Making clay robot at time {t}");

            let mut blueprint = mine(blueprint);
            // Decrease the count of ore by the cost.
            blueprint.ore -= b;
            // Increase the count of clay robots by 1.
            blueprint.rb_clay += 1;
            // Add the new blueprint to the queue.
            queue.push_back((t + 1, blueprint));
        }

        // Next push-back the next iteration of the blueprint.
        queue.push_back((t + 1, mine(blueprint)));
    }

    res
}

pub fn parse(data: &[String]) -> Vec<(i32, i32, i32, i32, i32, i32)> {
    // Each line is of folowing format:  Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 9 obsidian.
    // Read the numbers into variables: i, a, b, c, d, e, f.
    let mut res = Vec::new();

    for line in data {
        let mut numbers = Vec::new();

        // Split the line into words
        let words: Vec<&str> = line.split_whitespace().collect();

        // Iterate over the words, trying to parse each one as a number
        for word in words {
            if let Ok(num) = word.parse::<i32>() {
                numbers.push(num);
            }
        }

        let (a, b, c, d, e, f) = (
            numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5],
        );

        res.push((a, b, c, d, e, f));
    }

    res
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day19.txt")));
    println!("Day 19:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day19.txt"));
    c.bench_function("Day 19 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 19 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day19.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day19.txt")));
        assert_eq!(res.0, 33);
        println!("Part 1: Expected: 33, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day19.txt")));
        assert_eq!(res.1, 62);
        println!("Part 2: Expected: 62, Actual: {}", res.1);
    }
}
