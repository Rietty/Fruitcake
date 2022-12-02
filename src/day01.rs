// https://adventofcode.com/2022/day/1

pub fn solve(data: &[String]) -> (i32, i32) {
    // For the data, sum each set of numbers separated by an empty line and store them in an array.
    let mut sums: Vec<i32> = vec![];
    let mut sum: i32 = 0;

    for line in data {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    sums.sort();
    sums.reverse();

    // Return the largest, and then the sum of 3 largest values in the vector.
    (sums[0], sums.iter().take(3).sum())
}
