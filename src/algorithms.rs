use crate::solution::Solution;

pub mod backtracking;
pub use backtracking::Backtracking;

pub trait Algorithm {
    fn solve(solution: &mut Solution);
}
