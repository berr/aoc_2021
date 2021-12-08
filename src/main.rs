mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod input_handling;

use std::error::Error;
use crate::input_handling::DEFAULT_INPUT_FOLDER;

fn main() -> Result<(), Box<dyn Error>> {
    type Exercise = fn(&str) -> anyhow::Result<i32>;

    let exercises = [
        (1, 1, day1::part1 as Exercise),
        (1, 2, day1::part2),
        (2, 1, day2::part1),
        (2, 2, day2::part2),
        (3, 1, day3::part1),
        (3, 2, day3::part2),
        (4, 1, day4::part1),
        (4, 2, day4::part2),
        (5, 1, day5::part1),
        (5, 2, day5::part2),
    ];

    for (day, part, fp) in &exercises {
        let current_path = format!("{}/{}.txt", DEFAULT_INPUT_FOLDER, day);

        match fp(&current_path) {
            Ok(result) => println!("Day {}, part {}: Result = {}", day, part, result),
            Err(e) => println!("Day {}, part {}: Failed ({})", day, part, e),
        }
    }

    Ok(())
}
