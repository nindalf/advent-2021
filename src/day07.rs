use std::collections::HashMap;

#[allow(dead_code)]
fn min_fuel(input: &[u32], fuel_cost_fn: &dyn Fn(u32, u32) -> u32) -> Option<u32> {
    let mut positions = HashMap::new();
    let max_position = *input.iter().max()?;
    for position in 0..=max_position {
        for crab in input {
            let fuel = fuel_cost_fn(position, *crab);
            *positions.entry(position).or_default() += fuel;
        }
    }
    positions.values().min().copied()
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
    use anyhow::{anyhow, Result};

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_numbers_one_line("inputs/day7-test.txt")?;
        assert_eq!(
            super::min_fuel(&input, &super::fuel_cost_simple).ok_or(anyhow!("Empty data"))?,
            37
        );
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_numbers_one_line("inputs/day7.txt")?;
        assert_eq!(
            super::min_fuel(&input, &super::fuel_cost_simple).ok_or(anyhow!("Empty data"))?,
            356179
        );
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_numbers_one_line("inputs/day7-test.txt")?;
        assert_eq!(
            super::min_fuel(&input, &super::fuel_cost_complex).ok_or(anyhow!("Empty data"))?,
            168
        );
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_numbers_one_line("inputs/day7.txt")?;
        assert_eq!(
            super::min_fuel(&input, &super::fuel_cost_complex).ok_or(anyhow!("Empty data"))?,
            99788435
        );
        Ok(())
    }
}
