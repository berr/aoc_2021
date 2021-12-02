use std::str::FromStr;
use anyhow::Result;
use crate::input_handling::parse_file_line_by_line;

#[derive(Debug, Eq, PartialEq)]
enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Movement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').into_iter().collect();
        if parts.len() != 2 {
            return Err("Wrong number of parts")
        }

        let direction = parts[0];
        let amount = parts[1].parse().or(Err("Amount is not a number"))?;

        match direction {
            "forward" => Ok(Movement::Forward(amount)),
            "down" => Ok(Movement::Down(amount)),
            "up" => Ok(Movement::Up(amount)),
            _ => return Err("Invalid amount"),
        }
    }
}

pub fn part1(input_path: &str) -> Result<i32> {
    let input = parse_file_line_by_line(input_path)?;
    Ok(move_directly(&input))
}

fn move_directly(input: &[Movement]) -> i32 {
    let mut horizontal_distance = 0;
    let mut depth = 0;

    for m in input {
        match m {
            Movement::Forward(l) => horizontal_distance += l,
            Movement::Up(d) => depth -= d,
            Movement::Down(d) => depth += d,
        }
    }

    horizontal_distance * depth
}

pub fn part2(input_path: &str) -> Result<i32> {
    let input = parse_file_line_by_line(input_path)?;
    Ok(move_with_aim(&input))
}

fn move_with_aim(input: &[Movement]) -> i32 {
    let mut horizontal_distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    for m in input {
        match m {
            Movement::Up(d) => aim -= d,
            Movement::Down(d) => aim += d,
            Movement::Forward(l) => {
                horizontal_distance += l;
                depth += l * aim;
            },
        }
    }

    horizontal_distance * depth
}

#[cfg(test)]
mod tests {
    use crate::input_handling::example_input;
    use super::*;
    use Movement::*;

    const EXAMPLE_INPUT: [Movement; 6] = [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];

    #[test]
    fn test_parse_input() {
        let input: Vec<Movement> = parse_file_line_by_line(&example_input(2)).unwrap();
        assert_eq!(input.as_slice(), &EXAMPLE_INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(move_directly(&EXAMPLE_INPUT), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(move_with_aim(&EXAMPLE_INPUT), 900);
    }
}