mod day1;
mod day2;
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
