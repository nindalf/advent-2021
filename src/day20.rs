use std::{collections::HashSet, fmt::Write};

#[allow(dead_code)]
fn csi_enhance(input: &str, times: u32) -> usize {
    let algorithm = Algorithm::from(input);
    let mut image = Image::from(input);

    for _ in 0..times {
        image = image.enhance(&algorithm);
    }

    image.light_pixels.len()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Image {
    light_pixels: HashSet<Point>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

struct Algorithm(Vec<char>);

impl Image {
    fn enhance(&self, algorithm: &Algorithm) -> Self {
        let mut new_image = Image {
            light_pixels: HashSet::new(),
            min_x: self.min_x - 1,
            max_x: self.max_x + 1,
            min_y: self.min_y - 1,
            max_y: self.max_y + 1,
        };
        for x in new_image.min_x..new_image.max_x {
            for y in new_image.min_y..new_image.max_y {
                let point = Point { x, y };
                let index = self.enhanced_index(&point);
                if algorithm.is_light_pixel(index) {
                    new_image.light_pixels.insert(point);
                }
            }
        }

        new_image
    }

    fn get(&self, point: &Point) -> char {
        match self.light_pixels.get(point) {
            Some(_) => '#',
            None => '.',
        }
    }

    fn enhanced_index(&self, point: &Point) -> usize {
        let mut result = 0;
        for y in (point.y - 1)..=(point.y + 1) {
            for x in (point.x - 1)..=(point.x + 1) {
                result <<= 1;
                match self.get(&Point { x, y }) {
                    '#' => result += 1,
                    '.' => {}
                    _ => unreachable!("Invalid character"),
                }
            }
        }

        result
    }
}

impl From<&str> for Image {
    fn from(input: &str) -> Self {
        let mut points = HashSet::new();
        for (y, line) in input.lines().skip(2).enumerate() {
            for (x, val) in line.chars().enumerate() {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };
                if val == '#' {
                    points.insert(point);
                }
            }
        }
        let min_x = 0;
        let max_x = input.lines().nth(2).unwrap().chars().count() as i32;
        let min_y = 0;
        let max_y = input.lines().skip(2).count() as i32;
        Image {
            light_pixels: points,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..self.max_y {
            for x in self.min_x..self.max_x {
                f.write_char(self.get(&Point { x, y }))?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')?;
        Ok(())
    }
}

impl From<&str> for Algorithm {
    fn from(input: &str) -> Self {
        Algorithm(input.lines().next().unwrap().chars().collect::<Vec<char>>())
    }
}

impl Algorithm {
    fn is_light_pixel(&self, index: usize) -> bool {
        self.0[index] == '#'
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_test() -> anyhow::Result<()> {
        let input = crate::files::read_string("inputs/day20-test.txt")?;
        assert_eq!(super::csi_enhance(&input, 2), 35);
        Ok(())
    }

    #[test]
    fn part_1_real() -> anyhow::Result<()> {
        let input = crate::files::read_string("inputs/day20.txt")?;
        assert_eq!(super::csi_enhance(&input, 2), 35);
        Ok(())
    }
}
