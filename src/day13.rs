use std::{
    collections::{HashSet, VecDeque},
    fmt::Write,
};

use anyhow::Result;

use crate::matrix::Point;

// Almost got it right the first time. 2 minor mistakes for the first part
// Passed the wrong test input
// Parsed the folds wrong
// Took 2 tries to get the printing logic right

struct Paper {
    points: HashSet<Point>,
    folds: VecDeque<Fold>,
}

enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn new(s: &str) -> Option<Fold> {
        let mut parts = s.split('=');
        parts.next();
        let index = parts.next()?.parse::<usize>().ok()?;
        match s.chars().nth(11) {
            Some('x') => Some(Fold::X(index)),
            Some('y') => Some(Fold::Y(index)),
            _ => unreachable!("invalid fold"),
        }
    }
}

impl Paper {
    #[allow(dead_code)]
    fn new(input: &str) -> Result<Paper> {
        let mut parts = input.split("\n\n");
        let points = parts
            .next()
            .ok_or(anyhow::anyhow!("Invalid input"))?
            .split_ascii_whitespace()
            .filter_map(Point::new)
            .collect();

        let folds = parts
            .next()
            .ok_or(anyhow::anyhow!("Invalid input"))?
            .split('\n')
            .filter_map(Fold::new)
            .collect();

        Ok(Paper { points, folds })
    }

    #[allow(dead_code)]
    fn fold_once(&mut self) -> Result<()> {
        let fold = self
            .folds
            .pop_front()
            .ok_or(anyhow::anyhow!("no more folds"))?;

        self.points = self
            .points
            .iter()
            .map(|point| point.transpose(&fold))
            .collect();

        Ok(())
    }

    #[allow(dead_code)]
    fn fold_completely(&mut self) -> Result<()> {
        while !self.folds.is_empty() {
            self.fold_once()?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn num_remaining_points(&self) -> usize {
        self.points.len()
    }
}

impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.points.iter().map(|p| p.x).max().unwrap() + 1;
        let max_y = self.points.iter().map(|p| p.y).max().unwrap() + 1;
        for y in 0..max_y {
            for x in 0..max_x {
                if self.points.contains(&Point { x, y }) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Point {
    fn transpose(&self, fold: &Fold) -> Point {
        match fold {
            Fold::X(index) => {
                if self.x < *index {
                    *self
                } else {
                    Point {
                        x: 2 * index - self.x,
                        y: self.y,
                    }
                }
            }
            Fold::Y(index) => {
                if self.y < *index {
                    *self
                } else {
                    Point {
                        x: self.x,
                        y: 2 * index - self.y,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day13-test.txt")?;
        let mut paper = super::Paper::new(&input)?;
        paper.fold_once()?;
        assert_eq!(paper.num_remaining_points(), 17);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day13.txt")?;
        let mut paper = super::Paper::new(&input)?;
        paper.fold_once()?;
        assert_eq!(paper.num_remaining_points(), 802);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day13-test.txt")?;
        let mut paper = super::Paper::new(&input)?;
        paper.fold_completely()?;
        println!("{}", paper);
        assert_eq!(paper.num_remaining_points(), 16);
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day13.txt")?;
        let mut paper = super::Paper::new(&input)?;
        paper.fold_completely()?;
        println!("{}", paper);
        assert_eq!(paper.num_remaining_points(), 103);
        Ok(())
    }
}
