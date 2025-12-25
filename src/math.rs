use std::ops::{Div, Rem};

pub trait DivRem<R>: Sized {
    /// Return quotient with remainder.
    fn div_rem(self, divisor: R) -> (Self, Self);
}

impl<T, R> DivRem<R> for T
where
    T: Copy + Div<R, Output = T> + Rem<R, Output = T>,
    R: Copy,
{
    fn div_rem(self, divisor: R) -> (Self, Self) {
        let div = self / divisor;
        let rem = self % divisor;
        (div, rem)
    }
}
