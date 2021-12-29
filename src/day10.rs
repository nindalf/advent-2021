// First part - got it right on the first compilation!
// Second part - forgot scores.sort(), and had to replace u32 with u64 (overflow)

#[allow(dead_code)]
fn sum_corruption_points(input: &[String]) -> u64 {
    input.iter().map(|line| corruption_points(line)).sum()
}

fn corruption_points(line: &str) -> u64 {
    let mut stack = Vec::new();
    for char in line.chars() {
        match char {
            '(' | '{' | '<' | '[' => stack.push(char),
            ')' | '}' | '>' | ']' => {
                let opening = stack.pop();
                if is_corrupted(opening, char) {
                    return char_corruption_points(char);
                }
            }
            _ => unreachable!("Illegal character"),
        }
    }
    0
}

fn is_corrupted(opening: Option<char>, closing: char) -> bool {
    match (opening, closing) {
        (Some('('), ')') => false,
        (Some('{'), '}') => false,
        (Some('<'), '>') => false,
        (Some('['), ']') => false,
        (_, _) => true,
    }
}

fn char_corruption_points(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!("Invalid closing tag"),
    }
}

#[allow(dead_code)]
fn median_completion_points(input: &[String]) -> u64 {
    let mut scores: Vec<u64> = input
        .iter()
        .filter(|line| corruption_points(line) == 0)
        .map(|line| completion_points(line))
        .collect();

    // Clippy advice: an unstable sort would perform faster without any
    // observable difference for this data type
    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn completion_points(line: &str) -> u64 {
    let mut stack = Vec::new();
    for char in line.chars() {
        match char {
            '(' | '{' | '<' | '[' => stack.push(char),
            ')' | '}' | '>' | ']' => {
                stack.pop();
            }
            _ => unreachable!("Illegal character"),
        };
    }
    let mut score = 0;
    for char in stack.iter().rev() {
        score = score * 5 + char_completion_points(closing_tag(*char));
    }
    score
}

fn closing_tag(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!("Invalid opening tag"),
    }
}

fn char_completion_points(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!("Invalid closing tag"),
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        test(
            "inputs/day10-test.txt",
            &super::sum_corruption_points,
            26397,
        )
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day10.txt", &super::sum_corruption_points, 290691)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test(
            "inputs/day10-test.txt",
            &super::median_completion_points,
            288957,
        )
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test(
            "inputs/day10.txt",
            &super::median_completion_points,
            2768166558,
        )
    }

    fn test(test_file: &str, function: &dyn Fn(&[String]) -> u64, expected: u64) -> Result<()> {
        let input = crate::files::read_lines(test_file)?;
        assert_eq!(function(&input), expected);
        Ok(())
    }
}
