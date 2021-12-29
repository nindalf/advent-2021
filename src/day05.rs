use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[allow(dead_code)]
fn num_dangerous_points(input: &[String], allow_diagonals: bool) -> usize {
    let lines: Vec<Line> = input
        .iter()
        .filter_map(|l| Line::new(l, allow_diagonals))
        .collect();
    let mut points: HashMap<Point, u32> = HashMap::new();

    for line in lines {
        for point in line {
            *points.entry(point).or_default() += 1;
        }
    }

    points.values().filter(|value| **value >= 2).count()
}

struct Line {
    current: Point,
    end: Point,
    ended: bool,
    step_x: i32,
    step_y: i32,
}

impl Line {
    fn new(line: &str, allow_diagonals: bool) -> Option<Line> {
        let mut parts = line.split(" -> ");
        let current = Point::new(parts.next()).ok()?;
        let end = Point::new(parts.next()).ok()?;

        let ended = false;

        let is_horizontal = end.x - current.x == 0;
        let is_vertical = end.y - current.y == 0;
        let is_diagonal = !is_horizontal && !is_vertical;

        if !allow_diagonals && is_diagonal {
            return None;
        }

        let x_delt = end.x - current.x;
        let y_delt = end.y - current.y;
        let step_x = if x_delt != 0 {
            x_delt / i32::abs(x_delt)
        } else {
            0
        };
        let step_y = if y_delt != 0 {
            y_delt / i32::abs(y_delt)
        } else {
            0
        };

        Some(Line {
            current,
            end,
            ended,
            step_x,
            step_y,
        })
    }

    fn next_point(&self) -> Point {
        let x = self.current.x + self.step_x;
        let y = self.current.y + self.step_y;
        Point { x, y }
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }

        let result = self.current;
        self.current = self.next_point();

        if result == self.end {
            self.ended = true;
        }

        Some(result)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(input: Option<&str>) -> Result<Point> {
        let input = input.ok_or(anyhow!("invalid point input"))?;
        let mut parts = input.split(',');
        let x = parts
            .next()
            .ok_or(anyhow!("invalid point"))?
            .parse::<i32>()?;
        let y = parts
            .next()
            .ok_or(anyhow!("invalid point"))?
            .parse::<i32>()?;
        Ok(Point { x, y })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        test(
            "inputs/day5-test.txt",
            &super::num_dangerous_points,
            false,
            5,
        )
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day5.txt", &super::num_dangerous_points, false, 7269)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test(
            "inputs/day5-test.txt",
            &super::num_dangerous_points,
            true,
            12,
        )
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day5.txt", &super::num_dangerous_points, true, 21140)
    }

    fn test(
        test_file: &str,
        function: &dyn Fn(&[String], bool) -> usize,
        allow_diagonals: bool,
        expected: usize,
    ) -> Result<()> {
        let input = crate::files::read_lines(test_file)?;
        let result = function(&input, allow_diagonals);
        assert_eq!(result, expected);
        Ok(())
    }
}
