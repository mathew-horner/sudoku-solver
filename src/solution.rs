use anyhow::Result;

use crate::solution::base::BaseSolution;

pub mod base;
pub mod tui;

/// Implementors of [`Solution`] must either encapsulate a [`BaseSolution`] or
/// be one itself.
///
/// This trait gives those higher level types the ability to hook into `set`
/// calls in order to perform different functions, such as render to a TUI (as
/// [`TuiSolution`] does).
pub trait Solution {
    fn set(&mut self, index: usize, value: Option<u8>) -> Result<()>;

    fn base(&mut self) -> &mut BaseSolution;
}
