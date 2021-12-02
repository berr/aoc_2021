use anyhow::Result;
use crate::input_handling::parse_file_line_by_line;

pub fn part1(input_path: &str) -> Result<i32> {
    let input = parse_file_line_by_line(input_path)?;
    Ok(check_increases_in_sliding_windows(&input, 1))
}

pub fn part2(input_path: &str) -> Result<i32> {
    let input = parse_file_line_by_line(input_path)?;
    Ok(check_increases_in_sliding_windows(&input, 3))
}

fn check_increases_in_sliding_windows(data: &[i32], window_size: usize) -> i32 {
    let mut result = 0;
    let mut previous = None;

    for window in data.windows(window_size) {
        if window.len() < window_size {
            break;
        }

        let current_sum: i32 = window.iter().sum();

        if let Some(previous_sum) = previous {
            if current_sum > previous_sum {
                result += 1;
            }
        }

        previous = Some(current_sum);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::input_handling::example_input;
    use super::*;

    const EXAMPLE_INPUT: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_parse_input() {
        let input: Vec<i32> = parse_file_line_by_line(&example_input(1)).unwrap();
        assert_eq!(input.as_slice(), &EXAMPLE_INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(check_increases_in_sliding_windows(&EXAMPLE_INPUT, 1), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(check_increases_in_sliding_windows(&EXAMPLE_INPUT, 3), 5);
    }
}
