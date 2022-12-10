// https://adventofcode.com/2022/day/10

// Struct for an instruction.
#[derive(Debug, Clone)] 
pub struct Instruction {
	instruction: String,
	value: i32,
}

pub fn solve(data: &Vec<Instruction>) -> (i32, i32) {
	// Create a vector that contains durations and pre-seed it with a value of -1
	let mut durations = vec![-1];
	let mut x = 1;

	// Iterate over the instructions
	for instruction in data {
		match instruction.instruction.as_str() {
			"addx" => {
				// Append two values to the vector, one for each cycle.
				durations.push(x);
				durations.push(x);
				// Increment the value of x by the value of the instruction
				x += instruction.value;
			}

			_ => {
				// Append the current value to the vector if it's a no-op
				durations.push(x);
			}
		}
	}

	// Iterate from 20 to 220 in steps of 40.
	let mut p1: i32 = 0;
	for i in (20..221).step_by(40) {
		p1 += durations[i as usize] * i;
	}

	// For part 2 we just need to do a bit of math, and print to screen, so the p2 variable is just a placeholder.
	let p2: i32 = 0;
	for i in (1..241).step_by(40) {
		let mut line = Vec::new();
		for j in i..i + 40 {
			if (durations[j as usize] - (j - 1) % 40).abs() < 2 {
				line.push("#");
			} else {
				line.push(".");
			}
		}
		// Collect the line into a string and print it to screen.
		let line = line.join("");
		println!("{}", line);
	}

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<Instruction> {
	let mut instructions = Vec::new();
    for string in data {
		if string == "noop" {
			instructions.push(Instruction { instruction: "noop".to_string(), value: 0 });
		} else {
			let mut parts = string.split_whitespace();
		
			let instruction = match parts.next() {
				Some(instruction) => instruction.to_string(),
				None => continue, 
			};
	
			let value = match parts.next() {
				Some(value) => i32::from_str_radix(value, 10),
				None => continue, 
			};

			let value = value.unwrap();
			instructions.push(Instruction { instruction, value });
		}
    }

    instructions
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day10.txt")));
    println!("Day 10:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day10.txt"));
    c.bench_function("Day 10", |b| b.iter(|| solve(&data)));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let res = solve(&parse(&crate::library::read_file("testdata/day10.txt")));
        assert_eq!(res.0, 0);
    }

    #[test]
    fn part2() {
        let res = solve(&parse(&crate::library::read_file("testdata/day10.txt")));
        assert_eq!(res.1, 0);
    }
}
