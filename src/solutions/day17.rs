// https://adventofcode.com/2022/day/17

pub fn solve(data: &String) -> (i64, i64) {
    // Create an array of the rocks, modeled as a 2D array of tuples. Each coordinate is a place where the rock is present for that config.
    let rocks: [&[(usize, usize)]; 5] = [
        &[(0, 0), (0, 1), (0, 2), (0, 3)],         // Vertical line.
        &[(0, 1), (1, 0), (1, 2), (2, 1)], // Cross. We don't need to check/mark the middle one, since it's always there and won't be touched by anything else.
        &[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)], // L shape.
        &[(0, 0), (1, 0), (2, 0), (3, 0)], // Horizontal line.
        &[(0, 0), (0, 1), (1, 0), (1, 1)], // Square.
    ];

    // Couple of closures that we can use to calculate various things like height of a column, height of map, and if a piece can fit at a given location.

    // Get the height of the map.
    let get_height = |map: &[[u8; 7]]| -> usize { map.iter().position(|r| r == &[0; 7]).unwrap() };

    // Get the height of each column.
    let get_column_heights = |map: &[[u8; 7]]| -> [usize; 7] {
        let mut heights = [0; 7];
        let curr_height = get_height(map);
        // Iterate over each column, and find the height, then add it to the array.
        #[allow(clippy::needless_range_loop)]
        for i in 0..7 {
            heights[i] = (0..curr_height)
                .find(|&y| map[curr_height - y][i] == b'#')
                .unwrap_or(usize::MAX);
        }
        // Return the array.
        heights
    };

    // Check if a piece can fit at a given location.
    let will_fit =
        |map: &[[u8; 7]], rock: &[(usize, usize)], height: usize, width: usize| -> bool {
            rock.iter().all(|&(delta_height, delta_width)| {
                width + delta_width < 7 && map[height + delta_height][width + delta_width] != b'#'
            })
        };

    // Create a new map, of width 7 and height 10,000.
    let mut map = [[0; 7]; 10_000];

    // Create some indexers for the various offsets/loops that can also be used for modulus operations.
    let mut time = 0;
    let mut height_of_tower = 0;
    let mut i = 0;

    // Finally need to cache stuff, to speed up the lookups and not waste time recalculating things.
    let mut cache: std::collections::HashMap<(usize, usize, [usize; 7]), (usize, usize)> =
        std::collections::HashMap::new();

    // Iterate over the full amount of times we need to go.
    while i < 1_000_000_000_000 {
        // Get the current rock we're using.
        let rock = rocks[i % rocks.len()];

        // Get the current height, and width.
        let mut curr_height = get_height(&map) + 3;
        let mut curr_width = 2;

        // Need to loop and apply the jetstream.
        loop {
            match data.as_bytes()[time % data.len()] {
                b'<' => {
                    if will_fit(&map, rock, curr_height, curr_width - 1) {
                        curr_width -= 1;
                    }
                }

                b'>' => {
                    if will_fit(&map, rock, curr_height, curr_width + 1) {
                        curr_width += 1;
                    }
                }

                _ => {} // Do nothing, since input is well formed.
            }

            // Increase time unit by one.
            time += 1;

            // Check if height is 0, or we can't fit the rock.
            if curr_height == 0 || !will_fit(&map, rock, curr_height - 1, curr_width) {
                break;
            }

            // If we didn't break, then we can move the rock down.
            curr_height -= 1;
        }

        // Next need to iterate over the changes in height and width.
        for (delta_height, delta_width) in rock {
            map[curr_height + delta_height][curr_width + delta_width] = b'#'; // Set the piece down.
        }

        // Calculate the key for the cache.
        let key = (i % rocks.len(), time % data.len(), get_column_heights(&map));

        // Check if we can find the key in the cache.
        if let Some((c_i, c_h)) = cache.get(&key) {
            // If we did, we can calculate the repeats, update the value of i, and then update the overall height.
            let repeating = (1_000_000_000_000 - c_i) / (i - c_i) - 1;
            i += repeating * (i - c_i);
            height_of_tower += repeating * (get_height(&map) - c_h);
        } else {
            // If we didn't, we can add the key to the cache, and then update the height.
            cache.insert(key, (i, get_height(&map)));
        }

        i += 1;
    }

    let p1 = *cache
        .values()
        .find(|&&(i, _useless)| i == 2021)
        .map(|(_useless, h)| h)
        .unwrap();

    let p2 = height_of_tower + get_height(&map);

    (p1 as i64, p2 as i64)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day17.txt")[0]);
    println!("Day 17:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day17.txt"));
    c.bench_function("Day 17 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 17 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day17.txt"));
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
        let res = solve(&crate::library::read_file("testdata/day17.txt")[0]);
        assert_eq!(res.0, 3068);
        println!("Part 1: Expected: 0, Actual {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&crate::library::read_file("testdata/day17.txt")[0]);
        assert_eq!(res.1, 0);
        println!("Part 2: Expected: 0, Actual {}", res.1);
    }
}
