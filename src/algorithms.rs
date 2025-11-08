use crate::solution::Solution;

pub mod backtracking;
pub use backtracking::Backtracking;

pub trait Algorithm {
    fn solve<T: Solution>(&self, solution: &mut T);
}
