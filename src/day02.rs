use anyhow::{anyhow, Result};

struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
    instructions: Vec<Instruction>,
}

enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Instruction {
    fn new(line: &str) -> Result<Instruction> {
        let mut splits = line.split_ascii_whitespace();
        let direction = splits.next().ok_or(anyhow!("No direction"))?;
        let distance = splits
            .next()
            .ok_or(anyhow!("No distance"))?
            .parse::<i32>()?;
        match direction {
            "forward" => Ok(Instruction::Forward(distance)),
            "up" => Ok(Instruction::Up(distance)),
            "down" => Ok(Instruction::Down(distance)),
            _ => Err(anyhow!("Invalid direction")),
        }
    }
}

impl Submarine {
    #[allow(dead_code)]
    fn new(input: &[String]) -> Result<Submarine> {
        let instructions = input
            .iter()
            .map(|s| Instruction::new(s))
            .collect::<Result<Vec<Instruction>>>()?;
        Ok(Submarine {
            x: 0,
            y: 0,
            aim: 0,
            instructions,
        })
    }

    #[allow(dead_code)]
    fn execute_all_instructions_1(&mut self) {
        self.instructions
            .iter()
            .for_each(|instruction| match instruction {
                Instruction::Forward(distance) => self.x += distance,
                Instruction::Up(distance) => self.y -= distance,
                Instruction::Down(distance) => self.y += distance,
            })
    }

    #[allow(dead_code)]
    fn execute_all_instructions_2(&mut self) {
        self.instructions
            .iter()
            .for_each(|instruction| match instruction {
                Instruction::Forward(distance) => {
                    self.y += self.aim * distance;
                    self.x += distance;
                }
                Instruction::Up(distance) => self.aim -= distance,
                Instruction::Down(distance) => self.aim += distance,
            })
    }

    #[allow(dead_code)]
    fn final_position(&self) -> i32 {
        self.x * self.y
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::day02::Submarine;

    #[test]
    fn part_1_test() -> Result<()> {
        test(
            "inputs/day2-test.txt",
            &super::Submarine::execute_all_instructions_1,
            150,
        )
    }

    #[test]
    fn part_1_real() -> Result<()> {
        test(
            "inputs/day2.txt",
            &super::Submarine::execute_all_instructions_1,
            2322630,
        )
    }

    #[test]
    fn part_2_test() -> Result<()> {
        test(
            "inputs/day2-test.txt",
            &super::Submarine::execute_all_instructions_2,
            900,
        )
    }

    #[test]
    fn part_2_real() -> Result<()> {
        test(
            "inputs/day2.txt",
            &super::Submarine::execute_all_instructions_2,
            2105273490,
        )
    }

    fn test(test_file: &str, function: &dyn Fn(&mut Submarine), expected_val: i32) -> Result<()> {
        let input = crate::files::read_lines(test_file)?;
        let mut submarine = Submarine::new(&input)?;
        function(&mut submarine);
        assert_eq!(submarine.final_position(), expected_val);
        Ok(())
    }
}
