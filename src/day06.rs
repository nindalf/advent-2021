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
        test("inputs/day6-test.txt", 80, 5934)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day6.txt", 80, 352151)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test("inputs/day6-test.txt", 256, 26984457539)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day6.txt", 256, 1601616884019)
    }

    fn test(test_file: &str, days: u64, expected: u64) -> Result<()> {
        let input = crate::files::read_numbers_one_line(test_file)?;
        assert_eq!(super::num_lantern_fish(&input, days)?, expected);
        Ok(())
    }
}
