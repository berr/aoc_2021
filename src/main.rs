use std::error::Error;
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

fn day_one_1st(input: &str) -> Option<u32> {
    let file = File::open(input).expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut previous: Option<u32> = None;

    for line in reader.lines() {
        let current = line.ok()?.parse().ok()?;

        if let Some(p) = previous {
            if current > p {
                result += 1;
            }
        }

        previous = Some(current);
    }

    Some(result)
}

fn day_one_2nd(input: &str) -> Option<u32> {
    let file = File::open(input).expect("Couldn't open file");
    let reader = BufReader::new(file);
    let parsed_input: Vec<u32> = reader
        .lines()
        .map(|l|
            l.expect("Couldn't read file")
                .parse().expect("Couldn't parse line"))
        .collect();

    let result = check_increases_in_sliding_windows(&parsed_input, 3);

    Some(result)
}

fn check_increases_in_sliding_windows(data: &[u32], window_size: usize) -> u32 {
    let mut result = 0;
    let mut previous = None;

    for window in data.windows(window_size) {
        if window.len() < window_size {
            break;
        }

        for i in window {
            println!("{}", i);
        }
        println!();
        let current_sum: u32 = window.iter().sum();
        println!("{}", current_sum);
        println!("=================");

        if let Some(previous_sum) = previous {
            if current_sum > previous_sum {
                result += 1;
            }
        }

        previous = Some(current_sum);
    }

    result
}



fn main() -> Result<(), Box<dyn Error>> {
    let day_one_first = day_one_1st("/home/berr/dev/advent_of_code_2021/inputs/1.txt").expect("Failed day 1-1");
    println!("Day one 1st: {}", day_one_first);
    let day_one_second = day_one_2nd("/home/berr/dev/advent_of_code_2021/inputs/1.txt").expect("Failed day 1-2");
    println!("Day one 2nd: {}", day_one_second);
    let input = [199,200,208,210,200,207,240,269,260,263];
    check_increases_in_sliding_windows(&input, 3);

    Ok(())
}
