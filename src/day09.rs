use std::collections::{HashSet, VecDeque};

use crate::matrix::{Matrix, Point};

impl Matrix {
    #[allow(dead_code)]
    fn low_point_value_sum(&self) -> u32 {
        self.all_points()
            .filter(|p| self.is_low_point(p))
            .filter_map(|p| self.value(&p))
            .map(|val| val + 1)
            .sum()
    }

    fn is_low_point(&self, point: &Point) -> bool {
        self.neighbours(*point)
            .all(|neighbour| self.is_lower(point, &neighbour))
    }

    fn is_lower(&self, point: &Point, other: &Point) -> bool {
        match (self.value(other), self.value(point)) {
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

        for point in self.all_points() {
            let mut basin_size = 0;
            q.push_back(point);
            while let Some(point) = q.pop_front() {
                if *self.value(&point).unwrap() == 9 || visited.contains(&point) {
                    visited.insert(point);
                    continue;
                }

                visited.insert(point);
                basin_size += 1;

                for neighbour in self.neighbours(point) {
                    q.push_back(neighbour);
                }
            }
            result.push(basin_size)
        }

        result.sort_by(|a, b| b.cmp(a));
        result.iter().take(3).product()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::matrix::Matrix;

    #[test]
    fn part_1_test() -> Result<()> {
        test("inputs/day9-test.txt", &Matrix::low_point_value_sum, 15)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day9.txt", &Matrix::low_point_value_sum, 528)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test(
            "inputs/day9-test.txt",
            &Matrix::top_basin_sizes_product,
            1134,
        )
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day9.txt", &Matrix::top_basin_sizes_product, 920448)
    }

    fn test(test_file: &str, function: &dyn Fn(&Matrix) -> u32, expected: u32) -> Result<()> {
        let input = crate::files::read_string(test_file)?;
        let matrix = super::Matrix::new(&input)?;
        assert_eq!(function(&matrix), expected);
        Ok(())
    }
}
