// https://adventofcode.com/2022/day/8

pub fn solve(data: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;

    for (i, r) in data.iter().enumerate() {
        for (j, &tree) in r.iter().enumerate() {
            let mut visible_left = true;
            let mut visible_right = true;
            let mut visible_top = true;
            let mut visible_bottom = true;

            for k in 0..j {
                if data[i][k] >= tree {
                    visible_left = false;
                    break;
                }
            }
            for k in (j + 1)..r.len() {
                if data[i][k] >= tree {
                    visible_right = false;
                    break;
                }
            }

            for k in 0..i {
                if data[k][j] >= tree {
                    visible_top = false;
                    break;
                }
            }
            for k in (i + 1)..data.len() {
                if data[k][j] >= tree {
                    visible_bottom = false;
                    break;
                }
            }

            // If the tree is visible from any direction, increment the counter
            if visible_left || visible_right || visible_top || visible_bottom {
                p1 += 1;
            }
        }
    }

    for row in 0..data.len() {
        for col in 0..data[row].len() {
            let curr_height = data[row][col];

            let mut left: i32 = 0;
            let mut right: i32 = 0;
            let mut top: i32 = 0;
            let mut bottom: i32 = 0;

            for k in (0..row).rev() {
                top += 1;
                if data[k][col] >= curr_height {
                    break;
                }
            }

            for k in (0..col).rev() {
                left += 1;
                if data[row][k] >= curr_height {
                    break;
                }
            }

            for k in (row + 1)..data.len() {
                bottom += 1;
                if data[k][col] >= curr_height {
                    break;
                }
            }

            for k in (col + 1)..data[row].len() {
                right += 1;
                if data[row][k] >= curr_height {
                    break;
                }
            }

            p2 = std::cmp::max(p2, left * right * top * bottom);
        }
    }

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<Vec<i32>> {
    data.iter()
        .map(|digits| {
            digits
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day08.txt")));
    println!("Day 08:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day08.txt"));
    c.bench_function("Day 08", |b| b.iter(|| solve(&data)));
}
