const DIAL_LEN: i32 = 100;

enum Rotate {
    Left(i32),
    Right(i32),
}

impl std::str::FromStr for Rotate {
    type Err = String;

    fn from_str(input: &str) -> Result<Rotate, Self::Err> {
        let (dir, steps) = input
            .split_at_checked(1)
            .ok_or(format!("Failed to split {input}"))?;
        let steps: i32 = steps.parse().map_err(|e| {
            format!(
                "Failed to parse number of steps in rotation `{}`: {}",
                input, e
            )
        })?;
        Ok(match dir {
            "L" => Rotate::Left(steps),
            "R" => Rotate::Right(steps),
            _ => return Err(format!("Rotation should start with L or R: {}", input)),
        })
    }
}

// returns (end_pos, count_of_zeros) or an error string
fn rotate(start_pos: i32, rotation: &str) -> Result<(i32, i32), String> {
    let mut count_of_zeros = 0;

    let rotate: Rotate = rotation.parse()?;
    let end_pos = match rotate {
        Rotate::Left(steps) => start_pos - steps,
        Rotate::Right(steps) => start_pos + steps,
    }
    .rem_euclid(DIAL_LEN);

    if end_pos == 0 {
        count_of_zeros += 1;
    }

    Ok((end_pos, count_of_zeros))
}

// returns (end_pos, count_of_zeros) or an error string
fn rotate2(start_pos: i32, rotation: &str) -> Result<(i32, i32), String> {
    let mut count_of_zeros = 0;

    let rotate = rotation.parse()?;

    let end_pos = match rotate {
        Rotate::Right(steps) => {
            count_of_zeros += (start_pos + steps).div_euclid(DIAL_LEN);
            (start_pos + steps).rem_euclid(DIAL_LEN)
        }
        Rotate::Left(steps) => {
            // imagine the dial has a back side that looks exactly the same as the front, then
            //   1. if we poke number 1 from the front we will also hit number 99 in the back
            //      similarly, if we poke number x from the front we will hit number
            //      (DIAL_LEN - start_pos) % DIAL_LEN in the back
            //   2. rotating Left in the front is equivalent to rotating Right in the back
            let imaginary_pos = (DIAL_LEN - start_pos) % DIAL_LEN;
            count_of_zeros += (imaginary_pos + steps).div_euclid(DIAL_LEN);

            (start_pos - steps).rem_euclid(DIAL_LEN)
        }
    };

    // println!("({}, {}) --> ({}, {})", start_pos, rotation, end_pos, count_of_zeros);
    Ok((end_pos, count_of_zeros))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rotations = include_str!("rotations.txt");
    let rotations: Vec<&str> = rotations.lines().collect();

    let mut pos = 50;
    let mut count_of_zeros_part_1 = 0;
    for rotation in &rotations {
        let (end_pos, count_of_zeros) = rotate(pos, rotation)?;
        pos = end_pos;
        count_of_zeros_part_1 += count_of_zeros;
    }
    println!("Answer: {}", count_of_zeros_part_1);

    // Part Two
    let mut pos = 50;
    let mut count_of_zeros_part_2 = 0;
    for rotation in &rotations {
        let (end_pos, count_of_zeros) = rotate2(pos, rotation)?;
        pos = end_pos;
        count_of_zeros_part_2 += count_of_zeros;
    }
    println!("Answer: {}", count_of_zeros_part_2);

    Ok(())
}
