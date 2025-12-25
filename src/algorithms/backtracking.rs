use std::sync::mpsc::Receiver;

use anyhow::Result;

use crate::algorithms::Algorithm;
use crate::solution::Solution;

pub struct Backtracking;

impl Algorithm for Backtracking {
    fn solve<T: Solution>(&self, solution: &mut T, kill_channel: Option<Receiver<()>>) -> Result<()> {
        let mut pointer = 0;

        // We only need to iterate over the initially empty squares to find a solution.
        let initially_empty: Vec<_> =
            solution.base().iter_puzzle().filter(|(_, digit)| digit.is_none()).map(|(idx, _)| idx).collect();

        while pointer < initially_empty.len() {
            if let Some(channel) = kill_channel.as_ref()
                && let Ok(()) = channel.try_recv()
            {
                return Ok(());
            }

            let idx = initially_empty[pointer];
            let base = solution.base().get(idx).unwrap_or(0);
            let mut found_valid = false;

            // The lower bound of this for loop range is crucial. If the square was empty,
            // base will be 0 here and thus we will start at 1. If the square had a value,
            // we will only consider values higher than it since we've already tried the
            // lower values.
            for cand in (base + 1)..=9 {
                solution.set(idx, Some(cand))?;
                if solution.base().is_valid_digit(idx) {
                    found_valid = true;
                    break;
                }
            }

            if found_valid {
                pointer += 1;
            } else {
                solution.set(idx, None)?;
                if pointer == 0 {
                    // If we ever get here, that means we've exhausted all the candidates in the
                    // first cell, and thus we cannot find a solution.
                    panic!("unsolvable");
                }
                pointer -= 1;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    use crate::puzzle::Puzzle;
    use crate::solution::base::BaseSolution;

    #[test]
    fn test_solve() {
        let puzzle =
            Puzzle::from_str("050703060007000800000816000000030000005000100730040086906000204840572093000409000")
                .unwrap();
        let mut solution = BaseSolution::new(puzzle);
        Backtracking.solve(&mut solution, None).unwrap();

        let expected =
            Puzzle::from_str("158723469367954821294816375619238547485697132732145986976381254841572693523469718")
                .unwrap();

        assert_eq!(solution.puzzle, expected);
    }
}
