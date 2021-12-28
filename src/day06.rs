use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[allow(dead_code)]
fn num_lantern_fish(input: &[u32], days: u64) -> Result<u64> {
    let mut fishes = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);
    for fish in input {
        *fishes.entry(*fish).or_default() += 1;
    }
    for _ in 0..days {
        let new_spawn = *fishes.get(&0).ok_or(anyhow!("Hashmap get failed"))?;
        for i in 0..8 {
            let prev_gen = *fishes.get(&(i + 1)).ok_or(anyhow!("Hashmap get failed"))?;
            fishes
                .insert(i, prev_gen)
                .ok_or(anyhow!("Hashmap insert failed"))?;
        }
        fishes.insert(8, new_spawn);
        *fishes.entry(6).or_default() += new_spawn;
    }
    let total_fish = fishes.values().sum();
    Ok(total_fish)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_numbers_one_line("inputs/day6-test.txt")?;
        assert_eq!(super::num_lantern_fish(&input, 18)?, 26);
        assert_eq!(super::num_lantern_fish(&input, 80)?, 5934);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_numbers_one_line("inputs/day6.txt")?;
        assert_eq!(super::num_lantern_fish(&input, 80)?, 352151);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_numbers_one_line("inputs/day6-test.txt")?;
        assert_eq!(super::num_lantern_fish(&input, 256)?, 26984457539);
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_numbers_one_line("inputs/day6.txt")?;
        assert_eq!(super::num_lantern_fish(&input, 256)?, 1601616884019);
        Ok(())
    }
}
