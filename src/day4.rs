use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
use std::str::FromStr;
use anyhow::{Context, Result};
use itertools::Itertools;
use crate::day4::BoardNumber::{Drawn, NotDrawn};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Bingo {
    draw_order: Vec<i32>,
    current_draw_index: usize,
    boards: Vec<Board>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum BoardNumber {
    NotDrawn(i32),
    Drawn(i32),
}

impl BoardNumber {
    fn is_drawn(&self) -> bool {
        if let Drawn(_) = self {
            true
        } else {
            false
        }
    }

}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum BoardState {
    Running,
    Bingo,
    AlreadyFinished,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Board {
    numbers: [[BoardNumber; 5]; 5],
    state: BoardState,
}

impl Board {
    fn new(numbers: [[i32; 5]; 5]) -> Self {
        let mut board_numbers = [[NotDrawn(0); 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                board_numbers[i][j] = NotDrawn(numbers[i][j]);
            }
        }

        Self{numbers: board_numbers, state: BoardState::Running}
    }

    fn from_vec(numbers: Vec<Vec<i32>>) -> Self {
        let mut board_numbers = [[NotDrawn(0); 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                board_numbers[i][j] = NotDrawn(numbers[i][j]);
            }
        }

        Self{numbers: board_numbers, state: BoardState::Running}
    }

    fn finished_board(numbers: [[BoardNumber; 5]; 5]) -> Self {
        Self{numbers, state: BoardState::Bingo}
    }

    fn draw(&mut self, drawn_number: i32) -> BoardState {
        let mut position = None;

        'search: for i in 0..5 {
            for j in 0..5 {
                if let NotDrawn(n) = self.numbers[i][j] {
                    if n == drawn_number {
                        self.numbers[i][j] = Drawn(n);
                        position = Some((i, j));
                        break 'search;
                    }
                }
            }
        }

        if self.state != BoardState::Running {
            self.state = BoardState::AlreadyFinished;
            return self.state;
        }

        let (row, column) = if let Some((r, c)) = position {
            (r, c)
        } else {
            return BoardState::Running;
        };

        let mut row_is_completed = true;
        let mut column_is_completed = true;


        // (0..5).map(|j| self.numbers[row][j]).all(|n| n == BoardNumber::Drawn())
        for j in 0..5 {
            if let NotDrawn(_) = self.numbers[row][j] {
                row_is_completed = false;
                break;
            }
        }

        for i in 0..5 {
            if let NotDrawn(_) = self.numbers[i][column] {
                column_is_completed = false;
                break;
            }
        }

        if row_is_completed || column_is_completed {
            self.state = BoardState::Bingo;
        }

        self.state
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for l in &self.numbers {
            write!(f, "[")?;
            for n in l {
                write!(f, "{} ", n)?;
            }
            write!(f, "]")?;
        }

        Ok(())
    }

}

impl Display for BoardNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (prefix, number) = match self {
            Drawn(n) => ("D", n),
            NotDrawn(n) => ("N", n),
        };

        write!(f, "{}({})", prefix, number)
    }
}

pub fn part1(input_path: &str) -> Result<i32> {
    let input = parse_input(input_path)?;
    let (winning_board, last_drawn) = input.run_until_first_completion();
    Ok(Bingo::calculate_score(winning_board, last_drawn))
}

pub fn part2(input_path: &str) -> Result<i32> {
    let input = parse_input(input_path)?;
    let (winning_board, last_drawn) = input.run_until_completion();
    Ok(Bingo::calculate_score(winning_board, last_drawn))
}

fn parse_input(input_path: &str) -> anyhow::Result<Bingo> {
    let file = File::open(input_path).context("Couldn't open file")?;
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let draw_order: Result<Vec<i32>> = first_line.trim().split(",")
        .map(|s| s.parse()
            .map_err(anyhow::Error::from)).collect();

    let draw_order = draw_order?;
    let mut boards = vec![];

    for mut board_input_lines in &reader.lines().into_iter().chunks(6) {
        board_input_lines.next();

        let mut board = vec![];
        for input_line in board_input_lines {
            let input_line = input_line.map_err(anyhow::Error::from)?;
            let line: Result<Vec<_>> = input_line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().map_err(anyhow::Error::from)).collect();
            let line = line?;

            if line.len() != 5 {
                return Err(anyhow::Error::msg("Line doesn't have 5 columns"));
            }

            board.push(line);
        }

        if board.len() != 5 {
            return Err(anyhow::Error::msg("Board doesn't have 5 lines"));
        }

        boards.push(Board::from_vec(board));

    }

    Ok(Bingo::new(draw_order, boards))
}


impl Bingo {

    fn new(draw_order: Vec<i32>, boards: Vec<Board>) -> Self {
        let current_draw_index = 0;
        Bingo{draw_order, current_draw_index, boards}
    }

    fn run_until_first_completion(mut self) -> (Board, i32) {
        for drawn in self.draw_order {
            for board in self.boards.iter_mut() {
                let current_state = board.draw(drawn);
                if let BoardState::Bingo = current_state {
                    return (*board, drawn);
                }
            }
        }

        panic!("Didn't have a winner");
    }

    fn run_until_completion(mut self) -> (Board, i32) {
        let mut completed_boards = 0;
        let total_boards = self.boards.len();

        for drawn in self.draw_order {
            for board in self.boards.iter_mut() {
                let current_state = board.draw(drawn);
                if let BoardState::Bingo = current_state {
                    completed_boards += 1;
                    if total_boards == completed_boards {
                        return (*board, drawn);
                    }
                }
            }
        }

        panic!("Didn't have a winner");
    }

    fn calculate_score(board: Board, last_drawn_number: i32) -> i32 {
        let mut board_score = 0;
        for i in 0..board.numbers.len() {
            for j in 0..board.numbers[i].len() {
                if let NotDrawn(n) = board.numbers[i][j] {
                    board_score += n;
                }
            }
        }

        board_score * last_drawn_number
    }
}


#[cfg(test)]
mod tests {
    use crate::input_handling::example_input;
    use super::*;


    #[test]
    fn test_parse_input() {
        let input_data = parse_input(&example_input(4)).expect("Couldn't parse input");
        let expected = create_example_input();

        assert_eq!(input_data, expected);
    }

    #[test]
    fn test_run_first_completion() {
        let input = create_example_input();

        use BoardNumber::*;

        let completed_board = Board::finished_board([
            [Drawn(14), Drawn(21), Drawn(17), Drawn(24),  Drawn(4)],
            [NotDrawn(10), NotDrawn(16), NotDrawn(15),  Drawn(9), NotDrawn(19)],
            [NotDrawn(18),  NotDrawn(8), Drawn(23), NotDrawn(26), NotDrawn(20)],
            [NotDrawn(22), Drawn(11), NotDrawn(13),  NotDrawn(6),  Drawn(5)],
            [Drawn(2),  Drawn(0), NotDrawn(12),  NotDrawn(3),  Drawn(7)],
        ]);

        let last_drawn_number = 24;

        assert_eq!(input.run_until_first_completion(), (completed_board, last_drawn_number));
    }

    #[test]
    fn test_run_until_completion() {
        let input = create_example_input();

        use BoardNumber::*;

        // 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        let completed_board = Board::finished_board([
            [NotDrawn(3), NotDrawn(15), Drawn(0), Drawn(2), NotDrawn(22)],
            [Drawn(9), NotDrawn(18), Drawn(13), Drawn(17), Drawn(5)],
            [NotDrawn(19), NotDrawn(8), Drawn(7), NotDrawn(25), Drawn(23)],
            [NotDrawn(20), Drawn(11), Drawn(10), Drawn(24), Drawn(4)],
            [Drawn(14), Drawn(21), Drawn(16), NotDrawn(12), NotDrawn(6)],
        ]);

        let last_drawn_number = 13;

        assert_eq!(input.run_until_completion(), (completed_board, last_drawn_number));
    }

    #[test]
    fn test_calculate_final_score() {
        let completed_board = Board::finished_board([
            [Drawn(14), Drawn(21), Drawn(17), Drawn(24),  Drawn(4)],
            [NotDrawn(10), NotDrawn(16), NotDrawn(15),  Drawn(9), NotDrawn(19)],
            [NotDrawn(18),  NotDrawn(8), Drawn(23), NotDrawn(26), NotDrawn(20)],
            [NotDrawn(22), Drawn(11), NotDrawn(13),  NotDrawn(6),  Drawn(5)],
            [Drawn(2),  Drawn(0), NotDrawn(12),  NotDrawn(3),  Drawn(7)],
        ]);

        let last_drawn_number = 24;

        assert_eq!(Bingo::calculate_score(completed_board, last_drawn_number), 4512);
    }

    fn create_example_input() -> Bingo {
        let draw_order = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];

        let board1 = Board::new([[22, 13, 17, 11,  0],
        [ 8,  2, 23,  4, 24],
        [21,  9, 14, 16,  7],
        [ 6, 10,  3, 18,  5],
        [ 1, 12, 20, 15, 19]]);

        let board2 = Board::new([[ 3, 15,  0,  2, 22],
        [ 9, 18, 13, 17,  5],
        [19,  8,  7, 25, 23],
        [20, 11, 10, 24,  4],
        [14, 21, 16, 12,  6]]);

        let board3 = Board::new([[14, 21, 17, 24,  4],
        [10, 16, 15,  9, 19],
        [18,  8, 23, 26, 20],
        [22, 11, 13,  6,  5],
        [2,  0, 12,  3,  7]]);

        Bingo::new(draw_order, (vec![board1, board2, board3]))
    }

}