// https://adventofcode.com/2022/day/2

pub fn solve(data: &[String]) -> (i32, i32) {
    // Get current score for p1.
    let mut p1 = 0;
    let mut p2 = 0;

    for line in data {
        // Destrucure the line into 2 characters and remove any whitespace.
        let (c1, c2) = line.split_at(1);
        let c2 = c2.trim();
        p1 += score_hand_p1(c1, c2);
        p2 += score_hand_p2(c1, c2);
    }

    (p1, p2)
}

// Function to calculate score for a single line.
// Takes in 2 characters and returns score value.
fn score_hand_p1(c1: &str, c2: &str) -> i32 {
    let mut score: i32 = 0;
    // Add the value of the second character to the score (X = 1, Y = 2, Z = 3).
    match c2 {
        "X" => score += 1,
        "Y" => score += 2,
        "Z" => score += 3,
        _ => (),
    }

    match c1 {
        "A" => match c2 {
            "Y" => score += 6,
            "X" => score += 3,
            _ => score += 0,
        },
        "B" => match c2 {
            "Z" => score += 6,
            "Y" => score += 3,
            _ => score += 0,
        },
        "C" => match c2 {
            "X" => score += 6,
            "Z" => score += 3,
            _ => score += 0,
        },
        _ => (),
    }

    score
}

fn score_hand_p2(c1: &str, c2: &str) -> i32 {
    let mut score: i32 = 0;
    // Add the value of the second character to the score (X = 1, Y = 2, Z = 3).
    match c2 {
        "X" => score += 0,
        "Y" => score += 3,
        "Z" => score += 6,
        _ => (),
    }

    // Need to figure out what to do that will cause desired effect.
    // Check based off values of c2, and calculate what c1 should be and add to score.
    match c2 {
        "X" => match c1 {
            "A" => score += 3, // Rock means we lose if we have scissors.
            "B" => score += 1, // Paper means we lose if we have rock.
            "C" => score += 2, // Scissors means we lose if we have paper.
            _ => (),
        },
        "Y" => match c1 {
            // We need to draw if it is Y.
            "A" => score += 1,
            "B" => score += 2,
            "C" => score += 3,
            _ => (),
        },
        "Z" => match c1 {
            // We need to win if it is Z.
            "A" => score += 2,
            "B" => score += 3,
            "C" => score += 1,
            _ => (),
        },
        _ => (),
    }

    score
}
