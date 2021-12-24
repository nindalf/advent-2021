use anyhow::anyhow;

struct Submarine {
    x: i32,
    y: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine{x: 0, y: 0}
    }

    fn forward(&mut self, i: i32) {
        self.x += i
    }

    fn up(&mut self, j: i32) {
        self.y -= j
    }

    fn down(&mut self, j: i32) {
        self.y += j
    }
}

struct SubmarineVersion2 {
    x: i32,
    y: i32,
    aim: i32,
}

impl SubmarineVersion2 {
    fn new() -> SubmarineVersion2 {
        SubmarineVersion2{x: 0, y: 0, aim: 0}
    }

    fn forward(&mut self, i: i32) {
        self.x += i;
        self.y += self.aim * i;
    }

    fn up(&mut self, j: i32) {
        self.aim -= j
    }

    fn down(&mut self, j: i32) {
        self.aim += j
    }
}

fn final_position_part_1(instructions: &[String]) -> anyhow::Result<Submarine> {
    let mut sub = Submarine::new();
    for instruction in instructions {
        let mut splits = instruction.split_ascii_whitespace();
        let direction = splits.next().ok_or(anyhow!("invalid direction"))?;
        let distance = splits.next().ok_or(anyhow!("invalid distance"))?.parse::<i32>()?;
        match direction {
            "forward" => sub.forward(distance),
            "up" => sub.up(distance),
            "down" => sub.down(distance),
            _ => return Err(anyhow!("invalid direction"))
        }
    }
    Ok(sub)
}

fn final_position_part_2(instructions: &[String]) -> anyhow::Result<SubmarineVersion2> {
    let mut sub = SubmarineVersion2::new();
    for instruction in instructions {
        let mut splits = instruction.split_ascii_whitespace();
        let direction = splits.next().ok_or(anyhow!("invalid direction"))?;
        let distance = splits.next().ok_or(anyhow!("invalid distance"))?.parse::<i32>()?;
        match direction {
            "forward" => sub.forward(distance),
            "up" => sub.up(distance),
            "down" => sub.down(distance),
            _ => return Err(anyhow!("invalid direction"))
        }
    }
    Ok(sub)
}

mod tests {
    #[test]
    fn part_1_test() -> anyhow::Result<()> {
        let input = crate::files::read_lines("inputs/day2-test.txt").unwrap();
        let submarine = super::final_position_part_1(&input)?;
        assert_eq!(submarine.x * submarine.y, 150);
        Ok(())
    }

    #[test]
    fn part_1_real() -> anyhow::Result<()> {
        let input = crate::files::read_lines("inputs/day2.txt").unwrap();
        let submarine = super::final_position_part_1(&input)?;
        assert_eq!(submarine.x * submarine.y, 2322630);
        Ok(())
    }

    #[test]
    fn part_2_test() -> anyhow::Result<()> {
        let input = crate::files::read_lines("inputs/day2-test.txt").unwrap();
        let submarine = super::final_position_part_2(&input)?;
        assert_eq!(submarine.x * submarine.y, 900);
        Ok(())
    }

    #[test]
    fn part_2_real() -> anyhow::Result<()> {
        let input = crate::files::read_lines("inputs/day2.txt").unwrap();
        let submarine = super::final_position_part_2(&input)?;
        assert_eq!(submarine.x * submarine.y, 2105273490);
        Ok(())
    }
}