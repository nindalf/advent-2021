use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug)]
struct Projectile {
    x: i32,
    y: i32,
    highest_point: i32,
    vel_x: i32,
    vel_y: i32,
    end_x: (i32, i32),
    end_y: (i32, i32),
}

impl Iterator for Projectile {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_in_endzone() || self.is_trending_away() {
            return None;
        }

        self.x += self.vel_x;
        self.y += self.vel_y;
        self.vel_x += match self.vel_x {
            i32::MIN..=-1i32 => 1,
            0 => 0,
            1..=i32::MAX => -1,
        };
        self.vel_y += -1;

        self.highest_point = i32::max(self.highest_point, self.y);

        Some((self.x, self.y))
    }
}

impl Projectile {
    fn is_in_endzone(&self) -> bool {
        self.x >= self.end_x.0
            && self.x <= self.end_x.1
            && self.y >= self.end_y.0
            && self.y <= self.end_y.1
    }

    fn is_trending_away(&self) -> bool {
        (self.y < i32::min(self.end_y.0, self.end_y.1) && self.vel_y < 0) || (self.x > self.end_x.1)
    }
}

#[allow(dead_code)]
fn find_best_point(input: &str) -> Result<(i32, usize)> {
    // target area: x=20..30, y=-10..-5
    let (end_x, end_y) = parse_input(input)?;
    let mut results = Vec::new();
    for vel_x in 0..95 {
        for vel_y in -200..200 {
            let mut p = Projectile {
                x: 0,
                y: 0,
                highest_point: 0,
                vel_x,
                vel_y,
                end_x,
                end_y,
            };

            // reach the last point
            for _ in p.by_ref() {}

            if p.is_in_endzone() {
                results.push(p);
            }
        }
    }

    let highest_point = results
        .iter()
        .map(|p| p.highest_point)
        .max()
        .ok_or(anyhow!("No points"))?;

    Ok((highest_point, results.len()))
}

fn parse_input(input: &str) -> Result<((i32, i32), (i32, i32))> {
    let re: Regex = Regex::new("target area: x=(?P<end_x_0>[-0-9]*)\\.\\.(?P<end_x_1>[-0-9]*), y=(?P<end_y_0>[-0-9]*)\\.\\.(?P<end_y_1>[-0-9]*)").unwrap();
    let caps = re.captures(input).ok_or(anyhow!("Input parsing failed"))?;
    let end_x = (
        caps["end_x_0"].parse::<i32>()?,
        caps["end_x_1"].parse::<i32>()?,
    );
    let end_y = (
        caps["end_y_0"].parse::<i32>()?,
        caps["end_y_1"].parse::<i32>()?,
    );

    Ok((end_x, end_y))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn parse_input() -> Result<()> {
        let input = crate::files::read_string("inputs/day17.txt")?;
        let (end_x, end_y) = super::parse_input(&input)?;

        assert_eq!(end_x, (60, 94));
        assert_eq!(end_y, (-171, -136));
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day17-test.txt")?;
        assert_eq!(super::find_best_point(&input)?.0, 45);

        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day17.txt")?;
        assert_eq!(super::find_best_point(&input)?.0, 14535);

        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_string("inputs/day17-test.txt")?;
        assert_eq!(super::find_best_point(&input)?.1, 112);

        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day17.txt")?;
        assert_eq!(super::find_best_point(&input)?.1, 2270);

        Ok(())
    }
}
