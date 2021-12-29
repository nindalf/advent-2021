#[allow(dead_code)]
fn gamma(input: &[String]) -> anyhow::Result<u32> {
    process(input, &most_common_digit)
}

#[allow(dead_code)]
fn epsilon(input: &[String]) -> anyhow::Result<u32> {
    process(input, &least_common_digit)
}

#[allow(dead_code)]
fn oxygen_rating(input: &[String]) -> anyhow::Result<u32> {
    process_repeatedly(input, &most_common_digit)
}

#[allow(dead_code)]
fn co2_rating(input: &[String]) -> anyhow::Result<u32> {
    process_repeatedly(input, &least_common_digit)
}

fn process(input: &[String], process_fn: &dyn Fn(&[char]) -> char) -> anyhow::Result<u32> {
    let mut binary_digits = String::new();
    let lines = split_lines(input);
    for line in lines {
        let digit = process_fn(&line);
        binary_digits.push(digit);
    }
    let result = u32::from_str_radix(&binary_digits, 2)?;
    Ok(result)
}

fn process_repeatedly(
    input: &[String],
    process_fn: &dyn Fn(&[char]) -> char,
) -> anyhow::Result<u32> {
    let mut temp: Vec<String> = input.iter().map(|s| s.to_owned()).collect();
    for index in 0..input[0].len() {
        if temp.len() == 1 {
            break;
        }
        let line = nth_line(&temp, index);
        let digit = process_fn(&line);
        temp = temp
            .into_iter()
            .filter(|line| match line.chars().nth(index) {
                Some(c) => c == digit,
                None => false,
            })
            .collect();
    }
    let result = u32::from_str_radix(&temp[0], 2)?;
    Ok(result)
}

fn split_lines(input: &[String]) -> Vec<Vec<char>> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut result: Vec<Vec<char>> = Vec::with_capacity(input[0].len());
    for _ in 0..input[0].len() {
        result.push(Vec::with_capacity(input.len()));
    }
    for line in input {
        for (i, character) in line.chars().enumerate() {
            result[i].push(character);
        }
    }
    result
}

fn nth_line(input: &[String], n: usize) -> Vec<char> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut result: Vec<char> = Vec::with_capacity(input.len());
    for line in input {
        if let Some(c) = line.chars().nth(n) {
            result.push(c);
        }
    }
    result
}

fn most_common_digit(input: &[char]) -> char {
    let (zeros, ones) = counts(input);
    if zeros > ones {
        return '0';
    }
    '1'
}

fn least_common_digit(input: &[char]) -> char {
    let (zeros, ones) = counts(input);
    if zeros <= ones {
        return '0';
    }
    '1'
}

fn counts(input: &[char]) -> (u32, u32) {
    input.iter().fold((0, 0), |(zeros, ones), item| {
        if *item == '0' {
            return (zeros + 1, ones);
        }
        (zeros, ones + 1)
    })
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        test(
            "inputs/day3-test.txt",
            &super::gamma,
            22,
            &super::epsilon,
            9,
        )
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day3.txt", &super::gamma, 199, &super::epsilon, 3896)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test(
            "inputs/day3-test.txt",
            &super::oxygen_rating,
            23,
            &super::co2_rating,
            10,
        )
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test(
            "inputs/day3.txt",
            &super::oxygen_rating,
            509,
            &super::co2_rating,
            2693,
        )
    }

    fn test(
        test_file: &str,
        function_one: &dyn Fn(&[String]) -> Result<u32>,
        expected_one: u32,
        function_two: &dyn Fn(&[String]) -> Result<u32>,
        expected_two: u32,
    ) -> Result<()> {
        let input = crate::files::read_lines(test_file)?;
        assert_eq!(function_one(&input)?, expected_one);
        assert_eq!(function_two(&input)?, expected_two);
        Ok(())
    }
}
