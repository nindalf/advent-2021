#[allow(dead_code)]
fn number_of_increases(input: &[i32]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(first, second)| **second > **first)
        .count()
}

#[allow(dead_code)]
fn num_increasing_windows(input: &[i32]) -> usize {
    let window_sums = create_three_element_windows(input);
    number_of_increases(&window_sums)
}

fn create_three_element_windows(input: &[i32]) -> Vec<i32> {
    input
        .iter()
        .zip(input.iter().skip(1).zip(input.iter().skip(2)))
        .map(|(x, (y, z))| *x + *y + *z)
        .collect()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_numbers("inputs/day1-test.txt")?;
        let result = super::number_of_increases(&input);
        assert_eq!(result, 7);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_numbers("inputs/day1.txt")?;
        let result = super::number_of_increases(&input);
        assert_eq!(result, 1709);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_numbers("inputs/day1-test.txt")?;
        let result = super::num_increasing_windows(&input);
        assert_eq!(result, 5);
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_numbers("inputs/day1.txt")?;
        let result = super::num_increasing_windows(&input);
        assert_eq!(result, 1761);
        Ok(())
    }
}
