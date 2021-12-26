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
    return Ok(result);
}

fn process_repeatedly(
    input: &[String],
    process_fn: &dyn Fn(&[char]) -> char,
) -> anyhow::Result<u32> {
    let mut temp: Vec<String> = input.into_iter().map(|s| s.to_owned()).collect();
    for index in 0..input[0].len() {
        if temp.len() == 1 {
            break;
        }
        let line = nth_line(&temp, index);
        let digit = process_fn(&line);
        temp = temp
            .into_iter()
            .filter(|line| line.chars().nth(index).unwrap() == digit)
            .collect();
    }
    let result = u32::from_str_radix(&temp[0], 2)?;
    return Ok(result);
}

fn split_lines(input: &[String]) -> Vec<Vec<char>> {
    if input.len() < 1 {
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
    return result;
}

fn nth_line(input: &[String], n: usize) -> Vec<char> {
    if input.len() < 1 {
        return Vec::new();
    }
    let mut result: Vec<char> = Vec::with_capacity(input.len());
    for line in input {
        result.push(line.chars().nth(n).unwrap());
    }
    return result;
}

fn most_common_digit(input: &[char]) -> char {
    let (zeros, ones) = counts(input);
    if zeros > ones {
        return '0';
    }
    return '1';
}

fn least_common_digit(input: &[char]) -> char {
    let (zeros, ones) = counts(input);
    if zeros <= ones {
        return '0';
    }
    return '1';
}

fn counts(input: &[char]) -> (u32, u32) {
    input.iter().fold((0, 0), |(zeros, ones), item| {
        if *item == '0' {
            return (zeros + 1, ones);
        }
        return (zeros, ones + 1);
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_test() -> anyhow::Result<()> {
        let input = crate::files::read_lines("inputs/day3-test.txt").unwrap();
        assert_eq!(super::gamma(&input)?, 22);
        assert_eq!(super::epsilon(&input)?, 9);
        Ok(())
    }

    #[test]
    fn part_1_real() -> anyhow::Result<()> {
        let input = crate::files::read_lines("inputs/day3.txt").unwrap();
        assert_eq!(super::gamma(&input)?, 199);
        assert_eq!(super::epsilon(&input)?, 3896);
        Ok(())
    }

    #[test]
    fn part_2_test() -> anyhow::Result<()> {
        let input = crate::files::read_lines("inputs/day3-test.txt").unwrap();
        assert_eq!(super::oxygen_rating(&input)?, 23);
        assert_eq!(super::co2_rating(&input)?, 10);
        Ok(())
    }

    #[test]
    fn part_2_real() -> anyhow::Result<()> {
        let input = crate::files::read_lines("inputs/day3.txt").unwrap();
        assert_eq!(super::oxygen_rating(&input)?, 509);
        assert_eq!(super::co2_rating(&input)?, 2693);
        Ok(())
    }
}
