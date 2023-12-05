// https://adventofcode.com/2022/day/22

// Create an enum to handle either an integer or a character, fields are "direction" and "distance"
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Direction(char),
    Distance(i32),
}

pub fn solve(data: &(Vec<String>, Vec<Instruction>)) -> (i32, i32) {
    // Separate the data into the map and the instructions.
    let (map, instructions) = data;

    // Calculate the password for the map and the instructions, for part 1.
    let p1 = calculate_password(map.to_vec(), instructions.to_vec());

    // P2 was done in Python, so we will just return 0 for now.

    (p1, 0)
}

pub fn calculate_password(map: Vec<String>, instructions: Vec<Instruction>) -> i32 {
    // Starting at top, so that is line 0. Going down is south, so we increase y, and going up is north, so we decrease y.
    let mut y: i32 = 0;
    // X can be the index of the first non-space character in the first line.
    let mut x: i32 = map[y as usize].find(|c| c != ' ').unwrap() as i32;
    // Start facing East, so we can mark that as our direction.
    // 0 = East, 1 = South, 2 = West, 3 = North
    let mut direction: i32 = 0;

    // A closure to calculate the next square we are moving to.
    let get_next_square = |x: i32, y: i32| -> char {
        if y >= 0 && y < map.len() as i32 && x >= 0 && x < map[y as usize].len() as i32 {
            return map[y as usize].chars().nth(x as usize).unwrap();
        }

        ' '
    };

    // Iterate through the instructions.
    for instruction in instructions {
        // Check if the instruction is a direction or a distance.
        match instruction {
            Instruction::Direction(d) => {
                // If the instruction is a direction, we need to update the direction we are facing based on the current direction and if we move "Right" or "Left".
                // If we are facing East and we turn right, we are now facing South, if we turn left, we are facing North.
                match d {
                    'R' => direction = (direction + 1) % 4,
                    'L' => direction = (direction + 3) % 4,
                    _ => (),
                }
            }

            Instruction::Distance(d) => {
                // If the instruction is a distance, we need to move in the direction we are facing for the distance.
                // Set our DX and DY based on the direction we are facing.
                let (dx, dy) = match direction {
                    0 => (1, 0),
                    1 => (0, 1),
                    2 => (-1, 0),
                    3 => (0, -1),
                    _ => (0, 0),
                };

                // Move the number of times we are supposed to move.
                for _ in 0..d {
                    // Calculate the new x and y.
                    let mut new_x = x + dx;
                    let mut new_y = y + dy;

                    // Get the next square we are moving to.
                    let mut next_square = get_next_square(new_x, new_y);

                    // If the next square is a space, we need to wrap around to the other side of the map.
                    if next_square == ' ' {
                        // Loop through the map and keep subtracting dx and dy from the x and y until we find a non-space character by checking if the next square is not a space.
                        loop {
                            new_x -= dx;
                            new_y -= dy;

                            if get_next_square(new_x, new_y) == ' ' {
                                break;
                            }
                        }

                        // Add the dx and dy back to the x and y.
                        new_x += dx;
                        new_y += dy;
                        // Get the next square.
                        next_square = get_next_square(new_x, new_y);
                    }

                    // If the next square is a wall.. we can't do anything.
                    if next_square == '#' {
                        break;
                    }

                    // Update x and y.
                    x = new_x;
                    y = new_y;
                }
            }
        }
    }

    // Calculate the score by multiplying 1000 with y + 1, and 4 with x + 1, then add the value of the direction we are facing.
    1000 * (y + 1) + 4 * (x + 1) + direction
}

pub fn parse(data: &[String]) -> (Vec<String>, Vec<Instruction>) {
    // Let data be the everything except the last 2 lines.
    let res = data[..data.len() - 2].to_vec();
    // Parse the last entry into a vector of instructions
    let instructions = parse_instructions(&data[data.len() - 1]);

    (res, instructions)
}

fn parse_instructions(data: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for c in data.chars() {
        match c {
            'R' | 'L' => instructions.push(Instruction::Direction(c)),
            '0'..='9' => instructions.push(Instruction::Distance(c.to_digit(10).unwrap() as i32)),
            _ => (),
        }
    }

    // Go through the instructions and combine the distances if they are next to each other. Do this by multiplying the first distance by 10 and adding the second distance.
    let mut i = 0;
    while i < instructions.len() - 1 {
        if let Instruction::Distance(d1) = instructions[i] {
            if let Instruction::Distance(d2) = instructions[i + 1] {
                instructions[i] = Instruction::Distance(d1 * 10 + d2);
                instructions.remove(i + 1);
            }
        }
        i += 1;
    }

    instructions
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day22.txt")));
    println!("Day 22:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day22.txt"));
    c.bench_function("Day 22 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 22 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day22.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day22.txt")));
        assert_eq!(res.0, 6032);
        println!("Part 1: Expected: 6032, Actual: {}", res.0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day22.txt")));
        assert_eq!(res.1, 5031);
        println!("Part 2: Expected: 5031, Actual: {}", res.1);
    }
}
