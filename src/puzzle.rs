use std::fmt;
use std::str::FromStr;

use anyhow::{Error, Result, anyhow};

use crate::PUZZLE_DIGITS;

#[derive(Debug, Eq, PartialEq)]
pub struct Puzzle {
    pub data: [Option<u8>; PUZZLE_DIGITS],
}

impl Puzzle {
    /// Render the puzzle to a single line of 81 digits.
    pub fn serialize(&self) -> String {
        let mut buffer = String::with_capacity(PUZZLE_DIGITS);
        for idx in 0..PUZZLE_DIGITS {
            let char = match self.data[idx] {
                Some(digit) => char::from_digit(digit as u32, 10).unwrap(),
                None => '0',
            };
            buffer.push(char);
        }
        buffer
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self { data: std::array::from_fn(|_| None) }
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        if input.len() != PUZZLE_DIGITS {
            return Err(anyhow!("puzzle must have {PUZZLE_DIGITS} digits"));
        }

        let mut puzzle = Puzzle::default();

        for (idx, char) in input.chars().enumerate() {
            match char.to_digit(10) {
                // We accept both '0' and '.' to mean "empty square"
                Some(0) => {}
                None if char == '.' => {}
                Some(digit) => {
                    puzzle.data[idx] = Some(digit as u8);
                }
                None => return Err(anyhow!("character {char} at index {idx} is not a digit or period")),
            }
        }

        Ok(puzzle)
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Clean this code up.
        let horizontal_bar = str::repeat("-", 13);
        for (idx, digit) in self.data.iter().enumerate() {
            if idx % (9 * 3) == 0 {
                writeln!(f, "{horizontal_bar}")?;
            }
            if idx % 3 == 0 {
                write!(f, "|")?;
            }
            match digit {
                Some(digit) => write!(f, "{digit}")?,
                None => write!(f, ".")?,
            }
            if idx % 9 == 9 - 1 {
                writeln!(f, "|")?;
            }
        }
        writeln!(f, "{horizontal_bar}")?;
        Ok(())
    }
}
