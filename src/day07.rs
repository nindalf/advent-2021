use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[allow(dead_code)]
fn min_fuel(input: &[u32], fuel_cost_fn: &dyn Fn(u32, u32) -> u32) -> Result<u32> {
    let mut positions = HashMap::new();
    let max_position = *input.iter().max().ok_or(anyhow!("Empty input"))?;
    for position in 0..=max_position {
        for crab in input {
            let fuel = fuel_cost_fn(position, *crab);
            *positions.entry(position).or_default() += fuel;
        }
    }
    positions
        .values()
        .min()
        .copied()
        .ok_or(anyhow!("Empty positions"))
}

#[allow(dead_code)]
fn fuel_cost_simple(position: u32, crab: u32) -> u32 {
    i32::abs(crab as i32 - position as i32) as u32
}

#[allow(dead_code)]
fn fuel_cost_complex(position: u32, crab: u32) -> u32 {
    let n = i32::abs(crab as i32 - position as i32) as u32;
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        test("inputs/day7-test.txt", &super::fuel_cost_simple, 37)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day7.txt", &super::fuel_cost_simple, 356179)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test("inputs/day7-test.txt", &super::fuel_cost_complex, 168)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day7.txt", &super::fuel_cost_complex, 99788435)
    }

    fn test(test_file: &str, function: &dyn Fn(u32, u32) -> u32, expected: u32) -> Result<()> {
        let input = crate::files::read_numbers_one_line(test_file)?;
        assert_eq!(super::min_fuel(&input, function)?, expected);
        Ok(())
    }
}
