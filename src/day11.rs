use std::collections::{HashSet, VecDeque};

use crate::matrix::{Matrix, Point};

impl Matrix {
    #[allow(dead_code)]
    fn total_octopus_flashes(&mut self, generations: u32) -> u32 {
        let mut result = 0;
        let mut q = VecDeque::new();

        for _ in 0..generations {
            let mut clear_q = HashSet::new();
            self.add_to_all(1);
            self.find(10).for_each(|flasher| q.push_back(flasher));
            while !q.is_empty() {
                result += 1;
                let flasher = match q.pop_front() {
                    Some(point) => point,
                    None => continue,
                };
                clear_q.insert(flasher);

                for neighbour in self
                    .neighbours_with_diagonals(flasher)
                    .collect::<Vec<Point>>()
                {
                    match self.value(&neighbour).copied() {
                        Some(val) => {
                            if val <= 9 {
                                self.add(&neighbour, 1);
                            }
                            if val == 9 && !clear_q.contains(&neighbour) {
                                q.push_back(neighbour);
                            }
                        }
                        None => continue,
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
            while !q.is_empty() {
                let flasher = match q.pop_front() {
                    Some(point) => point,
                    None => continue,
                };
                clear_q.insert(flasher);

                for neighbour in self
                    .neighbours_with_diagonals(flasher)
                    .collect::<Vec<Point>>()
                {
                    match self.value(&neighbour).copied() {
                        Some(val) => {
                            if val <= 9 {
                                self.add(&neighbour, 1);
                            }
                            if val == 9 && !clear_q.contains(&neighbour) {
                                q.push_back(neighbour);
                            }
                        }
                        None => continue,
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
    use anyhow::{anyhow, Result};

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day11-test.txt")?;
        let mut matrix = super::Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        assert_eq!(matrix.total_octopus_flashes(10), 204);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day11.txt")?;
        let mut matrix = super::Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        assert_eq!(matrix.total_octopus_flashes(100), 1627);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day11-test.txt")?;
        let mut matrix = super::Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        assert_eq!(matrix.first_synchronized_flash(), 195);
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day11.txt")?;
        let mut matrix = super::Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        assert_eq!(matrix.first_synchronized_flash(), 329);
        Ok(())
    }
}
