use std::sync::mpsc::Receiver;

use anyhow::Result;

use crate::solution::Solution;

pub mod backtracking;
pub use backtracking::Backtracking;

pub trait Algorithm {
    /// Solve the puzzle; this is done in-place.
    ///
    /// Implementors must listen on `kill_channel` (if provided) and cleanly
    /// cancel their computation if a message is received.
    fn solve<T: Solution>(&self, solution: &mut T, kill_channel: Option<Receiver<()>>) -> Result<()>;
}
