use std::{collections::HashMap, num::ParseIntError};

use anyhow::{anyhow, Result};

struct BingoGame {
    boards: Vec<BingoBoard>,
    called_numbers: Vec<u32>,
    completions: Vec<u32>,
}

impl BingoGame {
    #[allow(dead_code)]
    fn new(input: &str) -> Result<BingoGame> {
        let mut parts = input.split("\n\n");
        let first_line = parts.next().ok_or(anyhow!("Invalid input"))?;
        let boards = parts
            .map(BingoBoard::new)
            .collect::<Result<Vec<BingoBoard>>>()?;
        let called_numbers: Vec<u32> = first_line
            .split(',')
            .map(|s| s.parse::<u32>())
            .collect::<core::result::Result<Vec<u32>, ParseIntError>>(
        )?;
        let completions = Vec::new();

        Ok(BingoGame {
            boards,
            called_numbers,
            completions,
        })
    }

    #[allow(dead_code)]
    fn call_all_numbers(&mut self) {
        for number in self.called_numbers.iter() {
            for board in self.boards.iter_mut() {
                if board.completed {
                    continue;
                }
                board.mark(*number);
                if board.completed {
                    self.completions.push(board.unmarked_number_sum() * number);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn winning_board(&self) -> Result<u32> {
        self.completions
            .get(0)
            .copied()
            .ok_or(anyhow!("No winning board"))
    }

    #[allow(dead_code)]
    fn losing_board(&self) -> Result<u32> {
        self.completions
            .iter()
            .last()
            .copied()
            .ok_or(anyhow!("No winning board"))
    }
}

#[derive(Debug)]
struct BingoBoard {
    number_to_cell: HashMap<u32, Cell>,
    row_completions: HashMap<usize, usize>,
    column_completions: HashMap<usize, usize>,
    side_len: usize,
    completed: bool,
}

#[derive(PartialEq, Eq, Debug)]
struct Cell {
    x: usize,
    y: usize,
    number: u32,
    marked: bool,
}

impl BingoBoard {
    fn new(lines: &str) -> Result<BingoBoard> {
        let mut number_to_cell = HashMap::new();
        for (y, line) in lines.split('\n').enumerate() {
            for (x, value) in line.split_ascii_whitespace().enumerate() {
                let number = value.parse::<u32>()?;
                let marked = false;
                let cell = Cell {
                    x,
                    y,
                    number,
                    marked,
                };
                number_to_cell.insert(number, cell);
            }
        }
        let row_completions = HashMap::new();
        let column_completions = HashMap::new();
        let side_len = lines.split('\n').count();
        let completed = false;
        Ok(BingoBoard {
            number_to_cell,
            row_completions,
            column_completions,
            side_len,
            completed,
        })
    }

    fn mark(&mut self, number: u32) {
        let cell = match self.number_to_cell.get_mut(&number) {
            Some(cell) => cell,
            None => return,
        };
        if cell.marked {
            // Cell was already called.
            return;
        }
        cell.marked = true;

        let row = self.row_completions.entry(cell.y).or_default();
        let column = self.column_completions.entry(cell.x).or_default();
        *row += 1;
        *column += 1;

        if *row == self.side_len || *column == self.side_len {
            self.completed = true
        }
    }

    fn unmarked_number_sum(&self) -> u32 {
        self.number_to_cell
            .values()
            .filter(|cell| !cell.marked)
            .map(|cell| cell.number)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::BingoGame;

    #[test]
    fn part_1_test() -> Result<()> {
        test("inputs/day4-test.txt", &BingoGame::winning_board, 4512)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day4.txt", &BingoGame::winning_board, 35711)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test("inputs/day4-test.txt", &BingoGame::losing_board, 1924)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day4.txt", &BingoGame::losing_board, 5586)
    }

    fn test(
        test_file: &str,
        function: &dyn Fn(&BingoGame) -> Result<u32>,
        expected: u32,
    ) -> Result<()> {
        let input = crate::files::read_string(test_file)?;
        let mut bingo_game = BingoGame::new(&input)?;
        bingo_game.call_all_numbers();
        let result = function(&bingo_game)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
