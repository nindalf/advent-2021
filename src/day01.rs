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
        test("inputs/day1-test.txt", &super::number_of_increases, 7)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day1.txt", &super::number_of_increases, 1709)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test("inputs/day1-test.txt", &super::num_increasing_windows, 5)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day1.txt", &super::num_increasing_windows, 1761)
    }

    fn test(
        test_file: &str,
        function: &dyn Fn(&[i32]) -> usize,
        expected_val: usize,
    ) -> Result<()> {
        let input = crate::files::read_numbers(test_file)?;
        let result = function(&input);
        assert_eq!(result, expected_val);
        Ok(())
    }
}
