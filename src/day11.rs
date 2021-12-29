use std::collections::{HashSet, VecDeque};

use crate::matrix::{Matrix, Point};

impl Matrix {
    #[allow(dead_code)]
    fn total_octopus_flashes(&mut self) -> u32 {
        let generations = 100;
        let mut result = 0;
        let mut q = VecDeque::new();

        for _ in 0..generations {
            let mut clear_q = HashSet::new();
            self.add_to_all(1);
            self.find(10).for_each(|flasher| q.push_back(flasher));
            while let Some(flasher) = q.pop_front() {
                result += 1;
                clear_q.insert(flasher);

                for neighbour in self
                    .neighbours_with_diagonals(flasher)
                    .collect::<Vec<Point>>()
                {
                    if let Some(val) = self.value(&neighbour).copied() {
                        if val <= 9 {
                            self.add(&neighbour, 1);
                        }
                        if val == 9 && !clear_q.contains(&neighbour) {
                            q.push_back(neighbour);
                        }
                    }
                }
            }

            for point in &clear_q {
                self.set(point, 0);
            }
        }

        result
    }

    #[allow(dead_code)]
    fn first_synchronized_flash(&mut self) -> u32 {
        let mut q = VecDeque::new();

        for generation in 1.. {
            let mut clear_q = HashSet::new();
            self.add_to_all(1);
            self.find(10).for_each(|flasher| q.push_back(flasher));
            while let Some(flasher) = q.pop_front() {
                clear_q.insert(flasher);

                for neighbour in self
                    .neighbours_with_diagonals(flasher)
                    .collect::<Vec<Point>>()
                {
                    if let Some(val) = self.value(&neighbour).copied() {
                        if val <= 9 {
                            self.add(&neighbour, 1);
                        }
                        if val == 9 && !clear_q.contains(&neighbour) {
                            q.push_back(neighbour);
                        }
                    }
                }
            }

            for point in &clear_q {
                self.set(point, 0);
            }

            // All elements have flashed
            if clear_q.len() == self.len() {
                return generation;
            }
        }

        unreachable!("Loop must end")
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::matrix::Matrix;

    #[test]
    fn part_1_test() -> Result<()> {
        test(
            "inputs/day11-test.txt",
            &Matrix::total_octopus_flashes,
            1656,
        )
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day11.txt", &Matrix::total_octopus_flashes, 1627)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test(
            "inputs/day11-test.txt",
            &Matrix::first_synchronized_flash,
            195,
        )
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day11.txt", &Matrix::first_synchronized_flash, 329)
    }

    fn test(test_file: &str, function: &dyn Fn(&mut Matrix) -> u32, expected: u32) -> Result<()> {
        let input = crate::files::read_string(test_file)?;
        let mut matrix = Matrix::new(&input)?;
        assert_eq!(function(&mut matrix), expected);
        Ok(())
    }
}
