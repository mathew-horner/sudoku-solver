use crate::solution::base::BaseSolution;

pub mod base;
pub mod tui;

pub trait Solution {
    fn set(&mut self, index: usize, value: Option<u8>);

    fn base(&mut self) -> &mut BaseSolution;
}
