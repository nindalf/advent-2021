use std::collections::HashSet;

#[allow(dead_code)]
fn count_1478s(input: &[String]) -> u32 {
    input
        .iter()
        .filter_map(|s| {
            let mut parts = s.split("|");
            parts.next();
            parts.next()
        })
        .flat_map(|s| s.split_ascii_whitespace())
        .filter(|s| match s.len() {
            // Length scorrespond to 1, 7, 4, 8
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count() as u32
}

#[allow(dead_code)]
fn decode_book(input: &[String]) -> u32 {
    input.iter().filter_map(|line| decode_line(line)).sum()
}

fn decode_line(line: &str) -> Option<u32> {
    let mut parts = line.split("|");
    let signals = parts.next()?;
    let one = Digit::new(
        signals
            .split_ascii_whitespace()
            .filter(|s| s.len() == 2)
            .next()?,
    );
    let four = Digit::new(
        signals
            .split_ascii_whitespace()
            .filter(|s| s.len() == 4)
            .next()?,
    );
    let output = parts.next()?;
    let digit_values: Vec<u32> = output
        .split_ascii_whitespace()
        .map(|s| Digit::new(s))
        .map(|d| d.decode(&one, &four))
        .rev()
        .collect();
    let mut result = 0;
    for (i, value) in digit_values.iter().enumerate() {
        result += value * u32::pow(10, i as u32);
    }
    Some(result)
}

#[derive(Debug)]
struct Digit {
    letter_set: HashSet<char>,
}

impl Digit {
    fn new(letters: &str) -> Digit {
        let letter_set: HashSet<char> = letters.chars().collect();
        Digit { letter_set }
    }

    fn contains_fully(&self, other: &Digit) -> bool {
        self.letter_set.intersection(&other.letter_set).count() == other.letter_set.len()
    }

    fn intersection(&self, other: &Digit) -> u32 {
        self.letter_set.intersection(&other.letter_set).count() as u32
    }

    /*
        0 = 6 (6 elements, contains 1 but doesn't contain 4)
        1 = 2 (ez)
        2 = 5 (doesn't contain 1, 2 elements from 4)
        3 = 5 (contains 1)
        4 = 4 (ez)
        5 = 5 (doesn't contain 1, 3 elements from 4)
        6 = 6 (6 elements, doesn't contain 1)
        7 = 3 (ez)
        8 = 7 (ez)
        9 = 6 (6 elements, contains 1, contains 4)
    */
    fn decode(&self, one: &Digit, four: &Digit) -> u32 {
        match self.letter_set.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            // 2, 3, 5
            5 => {
                if self.contains_fully(one) {
                    3
                } else if self.intersection(four) == 2 {
                    2
                } else if self.intersection(four) == 3 {
                    5
                } else {
                    unreachable!("invalid set of letters")
                }
            }
            // 0, 6 or 9
            6 => {
                if !self.contains_fully(one) {
                    6
                } else if self.contains_fully(one) && !self.contains_fully(four) {
                    0
                } else if self.contains_fully(one) && self.contains_fully(four) {
                    9
                } else {
                    unreachable!("invalid set of letters")
                }
            }
            7 => 8,
            _ => unreachable!("invalid number of letters"),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_lines("inputs/day8-test.txt")?;
        assert_eq!(super::count_1478s(&input), 26);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_lines("inputs/day8.txt")?;
        assert_eq!(super::count_1478s(&input), 519);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let input = crate::files::read_lines("inputs/day8-test.txt")?;
        assert_eq!(super::decode_book(&input), 61229);
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_lines("inputs/day8.txt")?;
        assert_eq!(super::decode_book(&input), 1027483);
        Ok(())
    }
}
