use std::collections::{HashSet, VecDeque};

struct Matrix {
    storage: Vec<u32>,
    max_x: usize,
    max_y: usize,
}

impl Matrix {
    #[allow(dead_code)]
    fn new(input: &str) -> Option<Matrix> {
        let storage = input
            .split("")
            .filter_map(|d| d.parse::<u32>().ok())
            .collect();
        let max_x = input
            .split_ascii_whitespace()
            .next()?
            .split("")
            .filter_map(|d| d.parse::<u32>().ok())
            .count();
        let max_y = input.split_ascii_whitespace().count();
        Some(Matrix {
            storage,
            max_x,
            max_y,
        })
    }

    #[allow(dead_code)]
    fn low_point_value_sum(&self) -> u32 {
        self.all_points()
            .iter()
            .filter(|(x, y)| self.is_low_point(*x, *y))
            .filter_map(|(x, y)| self.value(*x, *y))
            .map(|val| val + 1)
            .sum()
    }

    fn all_points(&self) -> Vec<(usize, usize)> {
        (0..self.max_x)
            .flat_map(|x| std::iter::repeat(x).zip(0..self.max_y))
            .collect()
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let right = self.is_lower(x, y, x + 1, y);
        let bottom = self.is_lower(x, y, x, y + 1);
        let left = if x > 0 {
            self.is_lower(x, y, x - 1, y)
        } else {
            true
        };
        let top = if y > 0 {
            self.is_lower(x, y, x, y - 1)
        } else {
            true
        };

        right && bottom && left && top
    }

    fn is_lower(&self, x: usize, y: usize, other_x: usize, other_y: usize) -> bool {
        match (self.value(other_x, other_y), self.value(x, y)) {
            (Some(other_val), Some(val)) => other_val > val,
            (_, None) => unreachable!("Element should always be reachable"),
            (None, _) => true,
        }
    }

    #[allow(dead_code)]
    fn top_basin_sizes_product(&self) -> u32 {
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        let mut result = Vec::new();

        for (x, y) in self.all_points() {
            let mut basin_size = 0;
            q.push_back((x, y));
            while !q.is_empty() {
                let (x, y) = match q.pop_front() {
                    Some(point) => point,
                    None => continue,
                };

                if *self.value(x, y).unwrap() == 9 || visited.contains(&(x, y)) {
                    visited.insert((x, y));
                    continue;
                }

                visited.insert((x, y));
                basin_size += 1;

                if x + 1 < self.max_x {
                    q.push_back((x + 1, y));
                }
                if y + 1 < self.max_y {
                    q.push_back((x, y + 1));
                }
                if x > 0 {
                    q.push_back((x - 1, y));
                }
                if y > 0 {
                    q.push_back((x, y - 1));
                }
            }
            result.push(basin_size)
        }

        result.sort_by(|a, b| b.cmp(a));
        result.iter().take(3).product()
    }

    fn value(&self, x: usize, y: usize) -> Option<&u32> {
        self.storage.get(y * self.max_x + x)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day9-test.txt")?;
        let matrix = super::Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        assert_eq!(matrix.low_point_value_sum(), 15);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day9.txt")?;
        let matrix = super::Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        assert_eq!(matrix.low_point_value_sum(), 528);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day9-test.txt")?;
        let matrix = super::Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        assert_eq!(matrix.top_basin_sizes_product(), 1134);
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day9.txt")?;
        let matrix = super::Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        assert_eq!(matrix.top_basin_sizes_product(), 920448);
        Ok(())
    }
}
