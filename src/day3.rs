use std::str::FromStr;
use anyhow::Result;
use crate::input_handling::parse_file_line_by_line;

#[derive(Debug, Eq, PartialEq, Clone)]
struct SubmarineDiagnosticLine {
    bits: Vec<bool>
}

impl SubmarineDiagnosticLine {

    fn new<T: Into<Vec<bool>>>(bits: T) -> Self {
        SubmarineDiagnosticLine{bits: bits.into()}
    }

    fn to_decimal(&self) -> i32 {
        self.bits.iter().rev()
            .enumerate()
            .filter_map(|(i, &b)| if b { Some(1 << i) } else { None })
            .sum()
    }
}

impl FromStr for SubmarineDiagnosticLine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let mut bits = vec![];
        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => bits.push(false),
                '1' => bits.push(true),
                _ => return Err("Encountered invalid character"),
            }
        }

        Ok(SubmarineDiagnosticLine::new(bits))
    }
}

pub fn part1(input_path: &str) -> Result<i32> {
    let input = parse_file_line_by_line(input_path)?;
    Ok(part1_impl(&input))
}

fn part1_impl(input: &[SubmarineDiagnosticLine]) -> i32 {
    let (gamma, epsilon) = calculate_gamma_epsilon_rates(input);
    gamma * epsilon
}

fn calculate_gamma_epsilon_rates(input: &[SubmarineDiagnosticLine]) -> (i32, i32) {
    let mut ones_in_position = count_ones_in_each_position(input);
    let majority_count = input.len() / 2;

    let gamma_bits = ones_in_position.iter().map(|&c| if c > majority_count { 1 } else { 0 });
    let gamma: i32 = gamma_bits.rev().enumerate().map(|(i, b)| (1 << i) * b).sum();

    let number_bits = ones_in_position.len();
    let mask = (1 << (number_bits)) - 1 ;
    let epsilon = !gamma & mask;

    (gamma, epsilon)
}

fn count_ones_in_each_position(input: &[SubmarineDiagnosticLine]) -> Vec<usize> {
    let max_bits = input.iter().map(|l| l.bits.len()).max().expect("Empty input");
    let mut ones_in_position = vec![0; max_bits];

    for line in input {
        for (p, &bit) in line.bits.iter().enumerate() {
            if bit {
                ones_in_position[p] += 1;
            }
        }
    }

    ones_in_position
}

pub fn part2(input_path: &str) -> Result<i32> {
    let input = parse_file_line_by_line(input_path)?;
    Ok(part2_impl(&input))
}

fn part2_impl(input: &[SubmarineDiagnosticLine]) -> i32 {
    let (oxygen_rate, co2_rate) = calculate_oxygen_and_co2_rates(input);
    oxygen_rate * co2_rate
}

fn calculate_oxygen_and_co2_rates(input: &[SubmarineDiagnosticLine]) -> (i32, i32) {
    let max_bits = input.iter().map(|l| l.bits.len()).max().expect("Empty input");
    let oxygen_rate = filter_gas_rate(input, true);
    let co2_rate = filter_gas_rate(input, false);
    (oxygen_rate, co2_rate)
}

fn filter_gas_rate(input: &[SubmarineDiagnosticLine], oxygen: bool) -> i32 {
    let max_bits = input.iter().map(|l| l.bits.len()).max().expect("Empty input");

    let mut gas_rate: Vec<_> = input.iter().cloned().collect();
    for i in 0..max_bits {
        if gas_rate.len() == 1 {
            break;
        }

        if gas_rate.is_empty() {
            panic!("Wrong!");
        }

        let counters = count_ones_in_each_position(&gas_rate);
        let majority_count = if gas_rate.len() % 2 == 0 {
            gas_rate.len() / 2
        } else {
            1 + gas_rate.len() / 2
        };

        let filter_by = if counters[i] >= majority_count {
            false ^ oxygen
        } else {
            true ^ oxygen
        };

        gas_rate.retain(|e| e.bits[i] == filter_by);
    }

    gas_rate[0].to_decimal()
}

#[cfg(test)]
mod tests {
    use crate::input_handling::example_input;
    use super::*;

    #[test]
    fn test_parse_input() {
        let input: Vec<SubmarineDiagnosticLine> = parse_file_line_by_line(&example_input(3)).unwrap();
        assert_eq!(input.as_slice(), &create_example_input());
    }

    #[test]
    fn test_calculate_gamma_epsilon_rates() {
        assert_eq!(calculate_gamma_epsilon_rates(&create_example_input()), (22, 9));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(&create_example_input()), 198);
    }

    #[test]
    fn test_calculate_oxygen_co2_rates() {
        assert_eq!(calculate_oxygen_and_co2_rates(&create_example_input()), (23, 10));
    }

    fn create_example_input() -> Vec<SubmarineDiagnosticLine> {
        vec![
            SubmarineDiagnosticLine::new(vec![false, false, true, false, false]),
            SubmarineDiagnosticLine::new(vec![true, true, true, true, false]),
            SubmarineDiagnosticLine::new(vec![true, false, true, true, false]),
            SubmarineDiagnosticLine::new(vec![true, false, true, true, true]),
            SubmarineDiagnosticLine::new(vec![true, false, true, false, true]),
            SubmarineDiagnosticLine::new(vec![false, true, true, true, true]),
            SubmarineDiagnosticLine::new(vec![false, false, true, true, true]),
            SubmarineDiagnosticLine::new(vec![true, true, true, false, false]),
            SubmarineDiagnosticLine::new(vec![true, false, false, false, false]),
            SubmarineDiagnosticLine::new(vec![true, true, false, false, true]),
            SubmarineDiagnosticLine::new(vec![false, false, false, true, false]),
            SubmarineDiagnosticLine::new(vec![false, true, false, true, false]),
        ]
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(&create_example_input()), 230);
    }
}