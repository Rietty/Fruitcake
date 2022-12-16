// https://adventofcode.com/2022/day/16

use std::collections::HashMap;
use std::collections::HashSet;

// Create a structure to hold the data for each room. The flow-rate, and the exits it has via tunnels. Since it'll be used in a HashMap, we don't need to store ID, as that'll be the key.
#[derive(Debug)]
pub struct Room {
    flow: i32,
    exits: Vec<String>,
}

pub fn solve(data: &HashMap<String, Room>) -> (i32, i32) {
    // We can just filter out any rooms that have a flow-rate of 0 from the map. Except for the start room, which we'll need to keep (AA).
    let good_rooms = data
        .iter()
        .filter(|(k, v)| v.flow > 0 || k == &"AA")
        .collect::<HashMap<&String, &Room>>();

    // Let's also create a map of the distance from every room to any other room. (Only for the good rooms.)
    let mut distances: HashMap<(String, String), i32> = HashMap::new();

    // Iterate over all possible rooms.
    for room in data {
        // Check if the room is in the good rooms list, if it isn't, we can skip it.
        if !good_rooms.contains_key(room.0) {
            continue;
        }

        let mut curr: HashSet<String> = HashSet::new();
        let mut next: HashSet<String> = HashSet::new();
        let mut dist: i32 = 0;

        // First add the distance from the current room to itself as 0 to the distances map.
        distances.insert((room.0.to_string(), room.0.to_string()), 0);
        // Insert the current room into the current set.
        curr.insert(room.0.to_string());

        // Iterate while the current set of rooms is not empty.
        while !curr.is_empty() {
            // Increment distance as we are moving.
            dist += 1;

            // Go through each Vector in the current set.
            for position in &curr {
                // Iterate over all possible exits from the current room.
                for exit in &data[position].exits {
                    // If the tuple (room, exit) is not in the distances map, add it.
                    // Then also insert the exit into the next set.
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        distances.entry((room.0.to_string(), exit.to_string()))
                    {
                        e.insert(dist);
                        next.insert(exit.to_string());
                    }
                }
            }

            // Copy the next set into the current set.
            curr = next.clone();
            // Clear the next set.
            next.clear();
        }
    }

    // Part 1.
    // Need to find the best flow-rate after arriving at the start room (AA) and then maximising the flow-rate.
    // This part is a basic DFS.
    let seen: HashSet<String> = HashSet::new();
    let targets: HashSet<String> = good_rooms.keys().map(|x| x.to_string()).collect();

    let p1 = best_total_flow("AA", 30, &seen, targets, data, &distances);

    (p1, 0)
}

pub fn best_total_flow(
    current: &str,
    time: i32,
    seen: &HashSet<String>,
    targets: HashSet<String>,
    rooms: &HashMap<String, Room>,
    distances: &HashMap<(String, String), i32>,
) -> i32 {
    // Create a copy of the seen and targets sets.
    let mut seen = seen.clone();
    let mut targets = targets;

    // Seen is a set of rooms that we have already visited. Add the current room to the seen set.
    seen.insert(current.to_string());

    // Remove anything that is in the seen set from the targets set.
    targets = targets.difference(&seen).map(|x| x.to_string()).collect();

    let mut best_flow = 0;

    for t in &targets {
        let time_left = time - distances[&(current.to_string(), t.to_string())] - 1;

        // If the time left is more than 0, we can continue.
        if time_left > 0 {
            let curr_flow = time_left * rooms[&t.to_string()].flow;
            let curr_flow =
                curr_flow + best_total_flow(t, time_left, &seen, targets.clone(), rooms, distances);

            // Set the best_flow to max of the current flow and the best flow.
            best_flow = std::cmp::max(best_flow, curr_flow);
        }
    }

    best_flow
}

pub fn parse(data: &[String]) -> HashMap<String, Room> {
    // Create a hashmap to store the rooms in.
    let mut rooms: HashMap<String, Room> = HashMap::new();

    for line in data {
        let words: Vec<&str> = line.split_whitespace().collect();
        let room_name = words[1];

        let flow_rate = words[4]
            .strip_suffix(';')
            .unwrap()
            .strip_prefix("rate=")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut exits = Vec::new();

        for word in words[9..].iter() {
            let mut exit = word.to_string();
            if exit.strip_suffix(',').is_some() {
                exit = exit.strip_suffix(',').unwrap().to_string();
            }
            exits.push(exit);
        }

        let room = Room {
            flow: flow_rate,
            exits,
        };

        rooms.insert(room_name.to_string(), room);
    }

    rooms
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day16.txt")));
    println!("Day 16:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day16.txt"));
    c.bench_function("Day 16", |b| b.iter(|| solve(&data)));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let res = solve(&parse(&crate::library::read_file("testdata/day16.txt")));
        assert_eq!(res.0, 1651);
        println!("Part 1: Expected: 1651, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day16.txt")));
        assert_eq!(res.1, 0);
        println!("Part 2: Expected: 0, Actual: {}", res.1);
    }
}
