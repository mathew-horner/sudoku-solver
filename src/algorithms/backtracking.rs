use crate::algorithms::Algorithm;
use crate::solution::Solution;

pub struct Backtracking;

impl Algorithm for Backtracking {
    fn solve(solution: &mut Solution) {
        let mut pointer = 0;
        let unsolved: Vec<_> =
            solution.iter_puzzle().filter(|(_, digit)| digit.is_none()).map(|(idx, _)| idx).collect();

        fn decrement(pointer: &mut usize) {
            if *pointer == 0 {
                panic!("unsolvable");
            }
            *pointer -= 1;
        }

        while pointer < unsolved.len() {
            let idx = unsolved[pointer];
            let base = solution.get(idx).unwrap_or(0);
            let mut found_valid = false;

            for cand in (base + 1)..=9 {
                solution.set(idx, Some(cand));
                if solution.valid_digit(idx) {
                    found_valid = true;
                    break;
                }
            }

            if found_valid {
                pointer += 1;
            } else {
                solution.set(idx, None);
                decrement(&mut pointer);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    use crate::puzzle::Puzzle;

    #[test]
    fn test_solve() {
        let puzzle =
            Puzzle::from_str("050703060007000800000816000000030000005000100730040086906000204840572093000409000")
                .unwrap();
        let solution = puzzle.solve::<Backtracking>();

        let expected =
            Puzzle::from_str("158723469367954821294816375619238547485697132732145986976381254841572693523469718")
                .unwrap();

        assert_eq!(solution.puzzle, expected);
    }
}
