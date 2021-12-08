use core::iter;
use std::ops::RangeInclusive;
use std::str::FromStr;
use anyhow::Result;
use itertools::Itertools;
use crate::input_handling::parse_file_line_by_line;

pub fn part1(path: &str) -> Result<i32> {
    let input = parse_input(path)?;
    let plot = plot_line_overlap(&input, false);
    Ok(count_overlaping_lines(&plot))
}

pub fn part2(path: &str) -> Result<i32> {
    let input = parse_input(path)?;
    let plot = plot_line_overlap(&input, true);
    Ok(count_overlaping_lines(&plot))
}


#[derive(Debug, Eq, PartialEq, Clone)]
struct VentLine {
    start: VentPoint,
    end: VentPoint,
}

impl VentLine {
    fn new(start: VentPoint, end: VentPoint) -> Self {
        Self{start, end}
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct VentPoint {
    x: usize,
    y: usize,
}

impl VentPoint {
    fn new(x: usize, y: usize) -> Self {
        Self{x, y}
    }
}

impl FromStr for VentPoint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(",").collect();
        if parts.len() != 2 {
            return Err(anyhow::Error::msg("Wrong number of coordinates"));
        }

        let x = parts[0].parse()?;
        let y = parts[1].parse()?;

        Ok(VentPoint{x, y})
    }
}

impl FromStr for VentLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("->").collect();
        if parts.len() != 2 {
            return Err(anyhow::Error::msg("Wrong number of parts"));
        }

        let start = parts[0].trim().parse()?;
        let end = parts[1].trim().parse()?;

        Ok(VentLine{start, end})

    }
}


fn parse_input(path: &str) -> anyhow::Result<Vec<VentLine>> {
    parse_file_line_by_line(path)
}

fn plot_line_overlap(input: &[VentLine], count_diagonals: bool) -> Vec<Vec<i32>> {
    let width = input.iter()
        .map(|l| l.start.x.max(l.end.x))
        .max()
        .expect("Input is empty") as usize + 1;
    let height = input.iter()
        .map(|l| l.start.y.max(l.end.y))
        .max()
        .expect("Input is empty") as usize + 1;

    let mut plane = vec![vec![0; width]; height];

    for line in input {
        let mut x_changes = line.start.x != line.end.x;
        let mut y_changes = line.start.y != line.end.y;

        if (x_changes && y_changes) && !count_diagonals {
            continue;
        }

        let x_iter = iter_changes(line.start.x, line.end.x);
        let y_iter = iter_changes(line.start.y, line.end.y);

        let results: Vec<_> = x_iter.zip(y_iter).collect();

        for (x, y) in results {
            plane[y][x] += 1;
        }
    }

    plane
}

enum ChangesIter {
    Increasing(RangeInclusive<usize>),
    Decreasing(std::iter::Rev<RangeInclusive<usize>>),
    Constant(std::iter::Repeat<usize>),
}

struct Changes {
    internal_iter: ChangesIter,
}

impl Changes {
    fn new(start: usize, end: usize) -> Self {
        if start < end {
            Self{internal_iter: ChangesIter::Increasing(start..=end)}
        } else if start > end {
            Self{internal_iter: ChangesIter::Decreasing((end..=start).rev())}
        } else {
            Self{internal_iter: ChangesIter::Constant(iter::repeat(start))}
        }
    }
}

impl Iterator for Changes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.internal_iter {
            ChangesIter::Increasing(iter) => iter.next(),
            ChangesIter::Decreasing(iter) => iter.next(),
            ChangesIter::Constant(iter) => iter.next(),
        }
    }
}

fn iter_changes(start: usize, end: usize) -> Changes {
    Changes::new(start, end)
}


fn count_overlaping_lines(plot: &Vec<Vec<i32>>) -> i32 {
    plot.iter()
        .flatten()
        .filter(|&e| *e >= 2)
        .count() as i32
}


#[cfg(test)]
mod tests {
    use crate::input_handling::example_input;
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = parse_input(&example_input(5)).unwrap();
        assert_eq!(input.as_slice(), &create_example_input());
    }

    #[test]
    fn test_plot_line_overlap_without_diagonals() {
        let expected_plot = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];

        assert_eq!(plot_line_overlap(&create_example_input(), false),expected_plot);
    }

    #[test]
    fn test_changes_iter_increasing() {
        let values: Vec<_> = iter_changes(0, 2).collect();
        assert_eq!(values.as_slice(), &[0, 1, 2]);
    }

    #[test]
    fn test_changes_iter_decreasing() {
        let values: Vec<_> = iter_changes(2, 0).collect();
        assert_eq!(values.as_slice(), &[2, 1, 0]);
    }

    #[test]
    fn test_changes_iter_constant() {
        let values: Vec<_> = iter_changes(0, 0).take(3).collect();
        assert_eq!(values.as_slice(), &[0, 0, 0]);
    }

    #[test]
    fn test_plot_line_overlap_with_diagonals() {
        let expected_plot = vec![
            vec![1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
            vec![0, 1, 1, 1, 0, 0, 0, 2, 0, 0],
            vec![0, 0, 2, 0, 1, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 0, 2, 0, 2, 0, 0],
            vec![0, 1, 1, 2, 3, 1, 3, 2, 1, 1],
            vec![0, 0, 0, 1, 0, 2, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];

        assert_eq!(plot_line_overlap(&create_example_input(), true),expected_plot);
    }

    #[test]
    fn test_count_overlaping_lines() {
        let expected_plot = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];

        assert_eq!(count_overlaping_lines(&expected_plot), 5);
    }

    //
    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1_impl(&create_example_input()), 198);
    // }
    //
    // #[test]
    // fn test_calculate_oxygen_co2_rates() {
    //     assert_eq!(calculate_oxygen_and_co2_rates(&create_example_input()), (23, 10));
    // }
    //
    fn create_example_input() -> Vec<VentLine> {
        vec![
            VentLine::new(VentPoint::new(0, 9), VentPoint::new(5, 9)),
            VentLine::new(VentPoint::new(8, 0), VentPoint::new(0, 8)),
            VentLine::new(VentPoint::new(9, 4), VentPoint::new(3, 4)),
            VentLine::new(VentPoint::new(2, 2), VentPoint::new(2, 1)),
            VentLine::new(VentPoint::new(7, 0), VentPoint::new(7, 4)),
            VentLine::new(VentPoint::new(6, 4), VentPoint::new(2, 0)),
            VentLine::new(VentPoint::new(0, 9), VentPoint::new(2, 9)),
            VentLine::new(VentPoint::new(3, 4), VentPoint::new(1, 4)),
            VentLine::new(VentPoint::new(0, 0), VentPoint::new(8, 8)),
            VentLine::new(VentPoint::new(5, 5), VentPoint::new(8, 2)),
        ]
    }
    //
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2_impl(&create_example_input()), 230);
    // }
}