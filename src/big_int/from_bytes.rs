use traiter::numbers::{Endianness, FromBytes, Unitary, Zeroable};

use crate::traits::HasSignBit;

use super::constants::MIDDLE_BYTE;
use super::contracts::is_valid_digit_bitness;
use super::digits::{
    negate_digits, to_digits_sign, BinaryBaseFromBinaryDigits,
};
use super::types::{BigInt, Sign};

impl<
        Digit: BinaryBaseFromBinaryDigits<u8> + HasSignBit,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > FromBytes for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
        let mut bytes = bytes.to_vec();
        match endianness {
            Endianness::Big => bytes.reverse(),
            Endianness::Little => {}
        }
        debug_assert!(is_valid_digit_bitness::<Digit, DIGIT_BITNESS>());
        let most_significant_byte = bytes[bytes.len() - 1];
        let sign = if most_significant_byte >= MIDDLE_BYTE {
            negate_digits(&mut bytes);
            -Sign::one()
        } else {
            to_digits_sign(&bytes)
        };
        Self {
            sign,
            digits: Digit::binary_base_from_binary_digits(
                &bytes[..bytes.len()
                    - ((bytes.len() > 1 && bytes[bytes.len() - 1].is_zero())
                        as usize)],
                u8::BITS as usize,
                DIGIT_BITNESS,
            ),
        }
    }
}
