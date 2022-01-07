use std::{
    collections::HashMap,
    ops::{AddAssign, Mul},
};

#[allow(dead_code)]
fn practice_game(x: u32, y: u32) -> u32 {
    let mut x_position = x - 1;
    let mut y_position = y - 1;
    let mut x_score = 0;
    let mut y_score = 0;
    let mut dice = Dice::new();
    loop {
        let roll = dice.roll_3();
        x_position = (x_position + roll) % 10;
        x_score += x_position + 1;
        if x_score >= 1000 {
            return y_score * dice.rolls;
        }

        let roll = dice.roll_3();
        y_position = (y_position + roll) % 10;
        y_score += y_position + 1;
        if y_score >= 1000 {
            return x_score * dice.rolls;
        }
    }
}

struct Dice {
    current: u32,
    sides: u32,
    rolls: u32,
}

impl Dice {
    fn new() -> Dice {
        Dice {
            current: 1,
            sides: 100,
            rolls: 0,
        }
    }

    fn roll_3(&mut self) -> u32 {
        self.next().unwrap() + self.next().unwrap() + self.next().unwrap()
    }
}

impl Iterator for Dice {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.current;
        self.current += 1;
        if self.current == self.sides + 1 {
            self.current = 1;
        }
        self.rolls += 1;
        Some(temp)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x_pos: u32,
    y_pos: u32,
    x_score: u32,
    y_score: u32,
    turn: Turn,
}

impl State {
    fn play(&mut self, dice_val: u32) {
        match self.turn {
            Turn::X => {
                self.x_pos = (self.x_pos + dice_val) % 10;
                self.x_score += self.x_pos + 1;
                self.turn = Turn::Y;
            }
            Turn::Y => {
                self.y_pos = (self.y_pos + dice_val) % 10;
                self.y_score += self.y_pos + 1;
                self.turn = Turn::X;
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Turn {
    X,
    Y,
}

#[derive(Clone, Copy, Default)]
struct Wins {
    x_wins: u64,
    y_wins: u64,
}

impl AddAssign<Wins> for Wins {
    fn add_assign(&mut self, rhs: Wins) {
        self.x_wins += rhs.x_wins;
        self.y_wins += rhs.y_wins;
    }
}

impl Mul<u64> for Wins {
    type Output = Wins;

    fn mul(self, rhs: u64) -> Self::Output {
        Wins {
            x_wins: self.x_wins * rhs,
            y_wins: self.y_wins * rhs,
        }
    }
}

#[allow(dead_code)]
fn real_game(x_pos: u32, y_pos: u32) -> u64 {
    let mut storage = HashMap::new();
    let initial_state = State {
        x_pos: x_pos - 1,
        y_pos: y_pos - 1,
        x_score: 0,
        y_score: 0,
        turn: Turn::X,
    };
    let wins = solve(initial_state, &mut storage);
    u64::max(wins.x_wins, wins.y_wins)
}

fn solve(state: State, storage: &mut HashMap<State, Wins>) -> Wins {
    if state.x_score >= 21 {
        return Wins {
            x_wins: 1,
            y_wins: 0,
        };
    }
    if state.y_score >= 21 {
        return Wins {
            x_wins: 0,
            y_wins: 1,
        };
    }

    if let Some(wins) = storage.get(&state) {
        return *wins;
    };

    const ROLLS: [(u32, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut combined_wins = Wins::default();

    for (dice_val, times) in ROLLS {
        let mut new_state = state;
        new_state.play(dice_val);
        let wins = solve(new_state, storage) * times;
        combined_wins += wins;
    }
    storage.insert(state, combined_wins);
    combined_wins
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        assert_eq!(super::practice_game(4, 8), 739785);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        assert_eq!(super::practice_game(4, 10), 855624);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        assert_eq!(super::real_game(4, 8), 444356092776315);
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        assert_eq!(super::real_game(4, 10), 187451244607486);
        Ok(())
    }
}
