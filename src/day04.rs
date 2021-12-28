use std::collections::HashMap;

#[allow(dead_code)]
fn winning_and_losing_bingo_combination(input: &str) -> (Option<u32>, Option<u32>) {
    let mut parts = input.split("\n\n");
    let first_line = match parts.next() {
        Some(line) => line,
        None => return (None, None),
    };
    let called_numbers: Vec<u32> = first_line
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let mut boards: Vec<BingoBoard> = parts.filter_map(|s| BingoBoard::new(s).ok()).collect();
    let mut completions = Vec::with_capacity(boards.len());

    for number in called_numbers {
        for board in &mut boards {
            if board.completed {
                continue;
            }
            board.mark(number);
            if board.completed {
                completions.push(board.unmarked_number_sum() * number);
            }
        }
    }

    let winning = completions.get(0).copied();
    if completions.len() < 2 {
        return (winning, None);
    }
    let losing = completions.last().copied();
    (winning, losing)
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
    fn new(lines: &str) -> anyhow::Result<BingoBoard> {
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
    use anyhow::{anyhow, Result};

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day4-test.txt")?;
        let result = super::winning_and_losing_bingo_combination(&input);
        assert_eq!(result.0.ok_or(anyhow!("Failed to find result"))?, 4512);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day4.txt")?;
        let result = super::winning_and_losing_bingo_combination(&input);
        assert_eq!(result.0.ok_or(anyhow!("Failed to find result"))?, 35711);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day4-test.txt")?;
        let result = super::winning_and_losing_bingo_combination(&input);
        assert_eq!(result.1.ok_or(anyhow!("Failed to find result"))?, 1924);
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day4.txt")?;
        let result = super::winning_and_losing_bingo_combination(&input);
        assert_eq!(result.1.ok_or(anyhow!("Failed to find result"))?, 5586);
        Ok(())
    }
}
