use traiter::numbers::CheckedDivRem;

use super::digits::CheckedDivRemComponents;
use super::types::BigInt;

impl<
        Digit: CheckedDivRemComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDivRem for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem(self, divisor: Self) -> Self::Output {
        Digit::checked_div_rem_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(
                quotient_sign,
                quotient_digits,
                remainder_sign,
                remainder_digits,
            )| {
                (
                    Self {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    Self {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<
        Digit: CheckedDivRemComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDivRem<&Self> for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem(self, divisor: &Self) -> Self::Output {
        Digit::checked_div_rem_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(
                quotient_sign,
                quotient_digits,
                remainder_sign,
                remainder_digits,
            )| {
                (
                    Self {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    Self {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<
        Digit: CheckedDivRemComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDivRem<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>
    for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
        BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
    )>;

    fn checked_div_rem(
        self,
        divisor: BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
    ) -> Self::Output {
        Digit::checked_div_rem_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(
                quotient_sign,
                quotient_digits,
                remainder_sign,
                remainder_digits,
            )| {
                (
                    BigInt::<Digit, SEPARATOR, DIGIT_BITNESS> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, SEPARATOR, DIGIT_BITNESS> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<
        Digit: CheckedDivRemComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDivRem for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
        BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
    )>;

    fn checked_div_rem(self, divisor: Self) -> Self::Output {
        Digit::checked_div_rem_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(
                quotient_sign,
                quotient_digits,
                remainder_sign,
                remainder_digits,
            )| {
                (
                    BigInt::<Digit, SEPARATOR, DIGIT_BITNESS> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, SEPARATOR, DIGIT_BITNESS> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}
