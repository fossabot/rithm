use super::digits::{sum_signed_digits, AdditiveDigit};
use super::types::BigInt;
use std::ops::AddAssign;

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> AddAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn add_assign(&mut self, other: &Self) {
        (self.sign, self.digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> AddAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn add_assign(&mut self, other: Self) {
        (self.sign, self.digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
    }
}