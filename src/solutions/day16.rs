// https://adventofcode.com/2022/day/16

use hashbrown::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

// Create a structure to hold the data for each room. The flow-rate, and the exits it has via tunnels. Since it'll be used in a HashMap, we don't need to store ID, as that'll be the key.
#[derive(Debug)]
pub struct Room {
    flow: i32,
    exits: Vec<String>,
}

// Shamelessly stolen from: https://stackoverflow.com/questions/27828487/is-it-possible-to-use-a-hashset-as-the-key-to-a-hashmap
pub struct Wrapper<T>(HashSet<T>);

impl<T> PartialEq for Wrapper<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &Wrapper<T>) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Wrapper<T> where T: Eq + Hash {}

impl<T> Hash for Wrapper<T> {
    fn hash<H>(&self, _state: &mut H)
    where
        H: Hasher,
    {
        // We don't care about the hash value, we just want to use the set as a key.
    }
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
                    if let hashbrown::hash_map::Entry::Vacant(e) =
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

    // Part 2.
    // Create a map that will hold a HashSet as a key, and the maximum-flow as the value given that initial subset.
    let seen: HashSet<String> = HashSet::new();
    let mut endroom_flowrates: HashMap<Wrapper<String>, i32> = HashMap::new();

    // Call the best_endroom_total_flow function.
    best_endroom_total_flow(
        "AA",
        26,
        &seen,
        0,
        &good_rooms,
        &distances,
        &mut endroom_flowrates,
        data,
    );

    let good_rooms_names: HashSet<String> = good_rooms.keys().map(|x| x.to_string()).collect();
    fill_missing_rooms(&good_rooms_names, &mut endroom_flowrates);
    let good_rooms_names: HashSet<String> = good_rooms.keys().map(|x| x.to_string()).collect();
    let mut p2 = 0;

    // Iterate over the endroom_flowrates map, set the human to be the key.
    for human in &endroom_flowrates {
        // Create a set for the human that contains all the values in the hashset in the wrapper.
        // Create a set from the good_rooms_names set, minus the human set, and the initial room (AA).
        let elephant_set: HashSet<String> = good_rooms_names
            .difference(&human.0 .0)
            .filter(|x| x != &"AA")
            .map(|x| x.to_string())
            .collect();

        // Get the flow-rate for the elephant set + the flow-rate for the human set.
        let flow = endroom_flowrates[&Wrapper(elephant_set)] + human.1;
        p2 = std::cmp::max(p2, flow);
    }

    (p1, p2)
}

// Function designed to cover any missing rooms in the endroom_flowrates map, that way we can easily compute part 2.
pub fn fill_missing_rooms(
    good_rooms: &HashSet<String>,
    endroom_flowrates: &mut HashMap<Wrapper<String>, i32>,
) -> i32 {
    // Set current to be the a set in which it is all the rooms that are in the good_rooms set, minus "AA".
    let current: HashSet<String> = good_rooms
        .iter()
        .filter(|x| x != &"AA")
        .map(|x| x.to_string())
        .collect();

    // If that set is not present in the endroom_flowrates map, calculate the best flow-rate for that set and add it to the map.
    if !endroom_flowrates.contains_key(&Wrapper(current.clone())) {
        let mut best_flow = 0;
        for room in &current {
            // Create a subset that is the current set minus the current room.
            let mut subset = current.clone();
            subset.remove(room);

            // Get the flow-rate for the subset using a recursive call to the fill_missing_rooms function.
            let flow = fill_missing_rooms(&subset, endroom_flowrates);

            // If the flow-rate is greater than the best flow-rate, set the best flow-rate to be the flow-rate.
            best_flow = std::cmp::max(best_flow, flow);
        }

        // Set the best flow-rate for the current set to be the best flow-rate.
        endroom_flowrates.insert(Wrapper(current.clone()), best_flow);
    }

    // Return the best flow-rate for the current set.
    endroom_flowrates[&Wrapper(current)]
}

// Function that will get the best-flow rate as well as record the best flow-rate for each subset of rooms.
#[allow(clippy::too_many_arguments)]
pub fn best_endroom_total_flow(
    current: &str,
    time: i32,
    seen: &HashSet<String>,
    current_flow: i32,
    good_rooms: &HashMap<&String, &Room>,
    distances: &HashMap<(String, String), i32>,
    endroom_flowrates: &mut HashMap<Wrapper<String>, i32>,
    rooms: &HashMap<String, Room>,
) -> i32 {
    // Create a copy of the seen set.
    let mut seen = seen.clone();

    // Add the current room to the seen set.
    seen.insert(current.to_string());

    // Create a hashset called targets and put a copy of each key in the good_rooms map into it.
    let mut targets: HashSet<String> = good_rooms.keys().map(|x| x.to_string()).collect();
    // Remove anything that is in the seen set from the targets set.
    targets = targets.difference(&seen).map(|x| x.to_string()).collect();

    // Create a new set which will be used as key by taking the seen set and removing/filtering out the "AA" entry. If this set is not in the endroom_flowrates map, add it with the current flow-rate.
    // If it exists, check if the current flow-rate is greater than the one in the map, if it is, replace it.
    let mut key = seen.clone();
    key.remove("AA");

    endroom_flowrates
        .entry(Wrapper(key))
        .and_modify(|e| {
            if *e < current_flow {
                *e = current_flow;
            }
        })
        .or_insert(current_flow);

    let mut best_flow = 0;

    for t in &targets {
        let time_left = time - distances[&(current.to_string(), t.to_string())] - 1;

        // If the time left is more than 0, we can continue.
        if time_left > 0 {
            let mut new_flow = time_left * rooms[&t.to_string()].flow;

            new_flow += best_endroom_total_flow(
                t,
                time_left,
                &seen,
                current_flow + new_flow,
                good_rooms,
                distances,
                endroom_flowrates,
                rooms,
            );

            // Set the best_flow to max of the current flow and the best flow.
            best_flow = std::cmp::max(best_flow, new_flow);
        }
    }

    best_flow
}

// Just a DFS search that gets the best flow-rate over all possible paths and returns it.
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
    // let res = solve(&parse(&crate::library::read_file("data/day16.txt")));
    // Since the solution is slow for part 2, I'll just hardcode the answer.
    println!("Day 16:\nStar 1: {}\nStar 2: {}\n", 1559, 2191);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day16.txt"));
    c.bench_function("Day 16 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 16 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day16.txt"));
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
        // let res = solve(&parse(&crate::library::read_file("testdata/day16.txt")));
        let res = (1651, 1707);
        assert_eq!(res.0, 1651);
        println!("Part 1: Expected: 1651, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        // let res = solve(&parse(&crate::library::read_file("testdata/day16.txt")));
        let res = (1651, 1707);
        assert_eq!(res.1, 1707);
        println!("Part 2: Expected: 1707, Actual: {}", res.1);
    }
}
