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

pub trait DigitChar {
    /// If this number is a single digit, return its ASCII char.
    fn digit_char(self) -> Option<char>;
}

impl DigitChar for u8 {
    fn digit_char(self) -> Option<char> {
        (self < 10).then(|| (self + 48) as char)
    }
}
