fn number_of_increases(input: &[i32]) -> usize {
    input.iter().zip(input.iter().skip(1))
        .filter(|(first, second)| **second > **first)
        .count()
}

fn num_increasing_windows(input: &[i32]) -> usize {
    let window_sums = create_three_element_windows(input);
    number_of_increases(&window_sums)
}

fn create_three_element_windows(input: &[i32]) -> Vec<i32> {
    input.iter().zip(input.iter().skip(1).zip(input.iter().skip(2)))
        .map(|(x, (y, z))| *x + *y + *z)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_test() {
        let input = crate::files::read_numbers("inputs/day1-test.txt").unwrap();
        let result = super::number_of_increases(&input);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_1_real() {
        let input = crate::files::read_numbers("inputs/day1.txt").unwrap();
        let result = super::number_of_increases(&input);
        assert_eq!(result, 1709);
    }

    #[test]
    fn part_2_test() {
        let input = crate::files::read_numbers("inputs/day1-test.txt").unwrap();
        let result = super::num_increasing_windows(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn part_2_real() {
        let input = crate::files::read_numbers("inputs/day1.txt").unwrap();
        let result = super::num_increasing_windows(&input);
        assert_eq!(result, 1761);
    }
}