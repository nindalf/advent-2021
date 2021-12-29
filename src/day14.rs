use std::collections::HashMap;

use anyhow::{anyhow, Result};

struct Polymer {
    template: HashMap<Pair, u64>,
    rules: HashMap<Pair, (Pair, Pair)>,
    start: char,
    end: char,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pair((char, char));

impl Polymer {
    #[allow(dead_code)]
    fn new(input: &str) -> Result<Polymer> {
        let mut parts = input.split("\n\n");
        let template = parts.next().ok_or(anyhow!("Invalid input"))?;
        let start = template.chars().next().ok_or(anyhow!("Invalid input"))?;
        let end = template.chars().last().ok_or(anyhow!("Invalid input"))?;
        let template = template
            .chars()
            .zip(template.chars().skip(1))
            .map(|p| Pair { 0: p })
            .fold(HashMap::new(), |mut acc, pair| {
                *acc.entry(pair).or_default() += 1;
                acc
            });

        let rules: HashMap<Pair, (Pair, Pair)> = parts
            .next()
            .ok_or(anyhow!("Invalid input"))?
            .split('\n')
            .filter_map(Pair::new_group)
            .collect();

        Ok(Polymer {
            template,
            rules,
            start,
            end,
        })
    }

    #[allow(dead_code)]
    fn pair_insertion_multiple(&mut self, n: u32) {
        for _ in 1..=n {
            self.pair_insertion();
        }
    }

    fn pair_insertion(&mut self) {
        self.template =
            self.template
                .iter()
                .fold(HashMap::new(), |mut acc, (pair, occurrences)| {
                    let new_pairs = self.rules.get(pair).unwrap();
                    *acc.entry(new_pairs.0).or_default() += occurrences;
                    *acc.entry(new_pairs.1).or_default() += occurrences;
                    acc
                });
    }

    #[allow(dead_code)]
    fn quantity_difference(&self) -> Result<u64> {
        let mut char_counts: HashMap<char, u64> =
            self.template
                .iter()
                .fold(HashMap::new(), |mut acc, (pair, count)| {
                    *acc.entry(pair.0 .0).or_default() += count;
                    *acc.entry(pair.0 .1).or_default() += count;
                    acc
                });

        *char_counts.entry(self.start).or_default() += 1;
        *char_counts.entry(self.end).or_default() += 1;

        char_counts.iter_mut().for_each(|(_, value)| {
            *value /= 2;
        });

        Ok(char_counts.values().max().ok_or(anyhow!("Empty values"))?
            - char_counts.values().min().ok_or(anyhow!("Empty values"))?)
    }
}

impl Pair {
    fn new_group(line: &str) -> Option<(Pair, (Pair, Pair))> {
        let mut parts = line.split(" -> ");

        let source = Pair::new(parts.next()?.chars())?;
        let dest = parts.next()?.chars().next()?;
        let dest_one = Pair {
            0: (source.0 .0, dest),
        };
        let dest_two = Pair {
            0: (dest, source.0 .1),
        };
        Some((source, (dest_one, dest_two)))
    }

    fn new(mut c: std::str::Chars) -> Option<Pair> {
        let p = (c.next()?, c.next()?);
        Some(Pair { 0: p })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        test("inputs/day14-test.txt", 10, 1588)
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test("inputs/day14.txt", 10, 3587)
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test("inputs/day14-test.txt", 40, 2188189693529)
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test("inputs/day14.txt", 40, 3906445077999)
    }

    fn test(test_file: &str, iterations: u32, expected: u64) -> Result<()> {
        let input = crate::files::read_string(test_file)?;
        let mut polymer = super::Polymer::new(&input)?;
        polymer.pair_insertion_multiple(iterations);
        assert_eq!(polymer.quantity_difference()?, expected);
        Ok(())
    }
}
