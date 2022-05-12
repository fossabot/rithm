use std::ops::Sub;

use crate::big_int::{AdditiveDigit, BigInt, GcdDigit, MultiplicativeDigit};
use crate::traits::{
    DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Oppositive, SubtractiveMagma,
};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + DivisivePartialMagma
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid
            + SubtractiveMagma,
    > Sub for Fraction<Component>
{
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator * subtrahend.denominator.clone()
                - subtrahend.numerator * self.denominator.clone(),
            self.denominator * subtrahend.denominator,
        );
        Self {
            numerator,
            denominator,
        }
    }
}

impl<
        Component: Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid
            + SubtractiveMagma,
    > Sub<Component> for Fraction<Component>
{
    type Output = Self;

    fn sub(self, other: Component) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator - other * self.denominator.clone(),
            self.denominator,
        );
        Self {
            numerator,
            denominator,
        }
    }
}

impl<
        Digit: AdditiveDigit + GcdDigit + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > Sub<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn sub(self, subtrahend: Fraction<Self>) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self * subtrahend.denominator.clone() - subtrahend.numerator,
            subtrahend.denominator,
        );
        Self::Output {
            numerator,
            denominator,
        }
    }
}

macro_rules! primitive_sub_fraction_impl {
    ($($t:ty)*) => ($(
    impl Sub<Fraction<Self>> for $t {
        type Output = Fraction<Self>;

        fn sub(self, subtrahend: Fraction<Self>) -> Self::Output {
            let (numerator, denominator) = normalize_components_moduli(
                self * subtrahend.denominator - subtrahend.numerator,
                subtrahend.denominator,
            );
            Self::Output {
                numerator,
                denominator,
            }
        }
    }
    )*)
}

primitive_sub_fraction_impl!(i8 i16 i32 i64 i128 isize);