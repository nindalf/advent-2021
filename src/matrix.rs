pub struct Matrix {
    storage: Vec<u32>,
    pub max_x: usize,
    pub max_y: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Matrix {
    #[allow(dead_code)]
    pub fn new(input: &str) -> Option<Matrix> {
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
    pub fn construct_extended_matrix(&self) -> Matrix {
        let new_storage = vec![0u32; self.len() * 25];
        let new_max_x = self.max_x * 5;
        let new_max_y = self.max_y * 5;
        let mut new_matrix = Matrix {
            storage: new_storage,
            max_x: new_max_x,
            max_y: new_max_y,
        };

        for p in new_matrix.all_points().collect::<Vec<Point>>() {
            let value = if p.x >= self.max_x {
                let p = Point {
                    x: p.x - self.max_x,
                    y: p.y,
                };
                new_matrix.value(&p).unwrap() + 1
            } else if p.y >= self.max_y {
                let p = Point {
                    x: p.x,
                    y: p.y - self.max_y,
                };
                new_matrix.value(&p).unwrap() + 1
            } else {
                *self.value(&p).unwrap()
            };

            let translated_val = if value <= 9 { value } else { 1 };
            new_matrix.set(&p, translated_val);
        }

        new_matrix
    }

    pub fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.max_x)
            .flat_map(|x| std::iter::repeat(x).zip(0..self.max_y))
            .map(|(x, y)| Point { x, y })
    }

    pub fn find(&self, needle: u32) -> impl Iterator<Item = Point> + '_ {
        self.all_points()
            .filter(move |point| match self.value(point) {
                Some(val) => *val == needle,
                None => false,
            })
    }

    pub fn value(&self, point: &Point) -> Option<&u32> {
        self.storage.get(point.y * self.max_x + point.x)
    }

    pub fn set(&mut self, point: &Point, val: u32) {
        self.storage[(point.y * self.max_x + point.x)] = val;
    }

    pub fn add_to_all(&mut self, n: u32) {
        for x in self.storage.iter_mut() {
            *x += n;
        }
    }

    pub fn add(&mut self, point: &Point, n: u32) {
        self.storage[(point.y * self.max_x + point.x)] += n;
    }

    pub fn neighbours(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        let x = point.x as i32;
        let y = point.y as i32;
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| *x < self.max_x && *y < self.max_y)
            .map(|(x, y)| Point { x, y })
    }

    pub fn neighbours_with_diagonals(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        let x = point.x as i32;
        let y = point.y as i32;
        (x - 1..=x + 1)
            .flat_map(move |x| std::iter::repeat(x).zip(y - 1..=y + 1))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| *x < self.max_x && *y < self.max_y)
            .map(|(x, y)| Point { x, y })
            .filter(move |p| point != *p)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, x) in self.storage.iter().enumerate() {
            if i % self.max_x == 0 && i != 0 {
                f.write_str("\n")?;
            }
            f.write_fmt(format_args!("{} ", x))?;
        }
        f.write_str("\n")?;
        Ok(())
    }
}

impl Point {
    pub fn new(s: &str) -> Option<Point> {
        let mut parts = s.split(',');
        let x = parts.next()?.parse::<usize>().ok()?;
        let y = parts.next()?.parse::<usize>().ok()?;
        Some(Point { x, y })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use anyhow::{anyhow, Result};

    use super::{Matrix, Point};

    #[test]
    fn test_add_to_all() -> Result<()> {
        let input = "00\n00";
        let mut matrix = Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        matrix.add_to_all(1);
        let sum: u32 = matrix.storage.iter().sum();
        assert_eq!(sum, 4);
        Ok(())
    }

    #[test]
    fn test_find() -> Result<()> {
        let input = "01\n22";
        let matrix = Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;
        let mut iterator = matrix.find(0);
        assert_eq!(iterator.next(), Some(Point { x: 0, y: 0 }));
        assert_eq!(iterator.next(), None);

        let mut iterator = matrix.find(1);
        assert_eq!(iterator.next(), Some(Point { x: 1, y: 0 }));
        assert_eq!(iterator.next(), None);

        let mut iterator = matrix.find(2);
        assert_eq!(iterator.next(), Some(Point { x: 0, y: 1 }));
        assert_eq!(iterator.next(), Some(Point { x: 1, y: 1 }));
        assert_eq!(iterator.next(), None);

        let mut iterator = matrix.find(3);
        assert_eq!(iterator.next(), None);
        Ok(())
    }

    #[test]
    fn test_neighbours() -> Result<()> {
        let input = "012\n345\n678";
        let matrix = Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;

        let neighbours = matrix
            .neighbours(Point { x: 0, y: 0 })
            .collect::<HashSet<Point>>();
        assert!(neighbours.contains(&Point { x: 1, y: 0 }));
        assert!(neighbours.contains(&Point { x: 0, y: 1 }));
        assert_eq!(neighbours.len(), 2);

        let neighbours = matrix
            .neighbours(Point { x: 1, y: 0 })
            .collect::<HashSet<Point>>();
        assert!(neighbours.contains(&Point { x: 0, y: 0 }));
        assert!(neighbours.contains(&Point { x: 2, y: 0 }));
        assert!(neighbours.contains(&Point { x: 1, y: 1 }));
        assert_eq!(neighbours.len(), 3);

        let neighbours = matrix
            .neighbours(Point { x: 1, y: 1 })
            .collect::<HashSet<Point>>();
        assert!(neighbours.contains(&Point { x: 1, y: 0 }));
        assert!(neighbours.contains(&Point { x: 0, y: 1 }));
        assert!(neighbours.contains(&Point { x: 2, y: 1 }));
        assert!(neighbours.contains(&Point { x: 1, y: 2 }));
        assert_eq!(neighbours.len(), 4);

        Ok(())
    }

    #[test]
    fn test_neighbours_with_diagonals() -> Result<()> {
        let input = "012\n345\n678";
        let matrix = Matrix::new(&input).ok_or(anyhow!("Could not construct matrix"))?;

        let neighbours = matrix
            .neighbours_with_diagonals(Point { x: 0, y: 0 })
            .collect::<HashSet<Point>>();
        assert!(neighbours.contains(&Point { x: 1, y: 0 }));
        assert!(neighbours.contains(&Point { x: 0, y: 1 }));
        assert!(neighbours.contains(&Point { x: 1, y: 1 }));
        assert_eq!(neighbours.len(), 3);

        let neighbours = matrix
            .neighbours_with_diagonals(Point { x: 1, y: 0 })
            .collect::<HashSet<Point>>();
        assert!(neighbours.contains(&Point { x: 0, y: 0 }));
        assert!(neighbours.contains(&Point { x: 2, y: 0 }));
        assert!(neighbours.contains(&Point { x: 1, y: 1 }));
        assert!(neighbours.contains(&Point { x: 0, y: 1 }));
        assert!(neighbours.contains(&Point { x: 2, y: 1 }));
        assert_eq!(neighbours.len(), 5);

        let neighbours = matrix
            .neighbours_with_diagonals(Point { x: 1, y: 1 })
            .collect::<HashSet<Point>>();
        assert!(neighbours.contains(&Point { x: 0, y: 0 }));
        assert!(neighbours.contains(&Point { x: 0, y: 1 }));
        assert!(neighbours.contains(&Point { x: 0, y: 2 }));
        assert!(neighbours.contains(&Point { x: 1, y: 0 }));
        assert!(neighbours.contains(&Point { x: 1, y: 2 }));
        assert!(neighbours.contains(&Point { x: 2, y: 0 }));
        assert!(neighbours.contains(&Point { x: 2, y: 1 }));
        assert!(neighbours.contains(&Point { x: 2, y: 2 }));
        assert_eq!(neighbours.len(), 8);

        Ok(())
    }
}
