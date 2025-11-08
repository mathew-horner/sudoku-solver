use crate::algorithms::Algorithm;
use crate::solution::Solution;

pub struct Backtracking;

impl Algorithm for Backtracking {
    fn solve<T: Solution>(&self, solution: &mut T) {
        let mut pointer = 0;
        let unsolved: Vec<_> =
            solution.base().iter_puzzle().filter(|(_, digit)| digit.is_none()).map(|(idx, _)| idx).collect();

        while pointer < unsolved.len() {
            let idx = unsolved[pointer];
            let base = solution.base().get(idx).unwrap_or(0);
            let mut found_valid = false;

            for cand in (base + 1)..=9 {
                solution.set(idx, Some(cand));
                if solution.base().valid_digit(idx) {
                    found_valid = true;
                    break;
                }
            }

            if found_valid {
                pointer += 1;
            } else {
                solution.set(idx, None);
                if pointer == 0 {
                    panic!("unsolvable");
                }
                pointer -= 1;
            }
        }
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
        Backtracking.solve(&mut solution);

        let expected =
            Puzzle::from_str("158723469367954821294816375619238547485697132732145986976381254841572693523469718")
                .unwrap();

        assert_eq!(solution.puzzle, expected);
    }
}
