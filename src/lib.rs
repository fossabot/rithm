#![feature(associated_type_defaults)]

use std::iter::Peekable;
use std::str::Chars;

use num::Zero;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::{PyInt, PyList, PyString};

mod big_int;
mod utils;

type Sign = i8;

const BINARY_SHIFT: usize = (big_int::Digit::BITS - 1) as usize;
const BINARY_BASE: big_int::DoubleDigit = 1 << BINARY_SHIFT;
const BINARY_DIGIT_MASK: big_int::DoubleDigit = BINARY_BASE - 1;
const DECIMAL_SHIFT: usize = utils::floor_log10(BINARY_BASE as usize);
const DECIMAL_BASE: usize = utils::power(10, DECIMAL_SHIFT);
const SEPARATOR: char = '_';
const MAX_REPRESENTABLE_BASE: u8 = 36;
const ASCII_CODES_DIGIT_VALUES: [u8; 256] = [
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 37, 37, 37, 37, 37, 37, 37, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 37, 37, 37, 37, 37, 37, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
    35, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
];

#[pyclass(module = "rithm", subclass)]
struct Int {
    sign: Sign,
    digits: Vec<big_int::Digit>,
}

fn count_digits(mut characters: Peekable<Chars>, base: u8) -> PyResult<usize> {
    let mut result: usize = 0;
    let mut prev: char = SEPARATOR;
    while let Some(character) = characters.next() {
        if character != SEPARATOR {
            if ASCII_CODES_DIGIT_VALUES[character as usize] > base {
                return Err(PyValueError::new_err(format!(
                    "Invalid digit in base {}: {}.",
                    base, character
                )));
            }
            result += 1;
        } else if prev == SEPARATOR {
            return Err(PyValueError::new_err("Consecutive separators found."));
        }
        prev = character;
    }
    Ok(result)
}

#[pymethods]
impl Int {
    #[new]
    fn new(py_string: &PyString, mut base: u8) -> PyResult<Self> {
        let string: String = py_string.extract()?;
        if (base != 0 && base < 2) || base > MAX_REPRESENTABLE_BASE {
            return Err(PyValueError::new_err(format!(
                "Base should be zero or in range from 2 to {}.",
                MAX_REPRESENTABLE_BASE
            )));
        }
        let mut characters = string.trim().chars().peekable();
        let sign: Sign = if characters.peek() == Some(&'-') {
            characters.next();
            -1
        } else if characters.peek() == Some(&'+') {
            characters.next();
            1
        } else {
            1
        };
        if base == 0 {
            base = if characters.peek() != Some(&'0') {
                10
            } else {
                characters.next();
                match characters.peek() {
                    Some(&'b') | Some(&'B') => {
                        characters.next();
                        characters.next_if_eq(&SEPARATOR);
                        2
                    }
                    Some(&'o') | Some(&'O') => {
                        characters.next();
                        characters.next_if_eq(&SEPARATOR);
                        8
                    }
                    Some(&'x') | Some(&'X') => {
                        characters.next();
                        characters.next_if_eq(&SEPARATOR);
                        16
                    }
                    None => {
                        return Ok(Int {
                            sign: 0 as Sign,
                            digits: vec![0],
                        });
                    }
                    _ => 10,
                }
            };
        } else if characters.peek() == Some(&'0') {
            characters.next();
            match characters.peek() {
                Some(&'b') | Some(&'B') => {
                    characters.next();
                    characters.next_if_eq(&SEPARATOR);
                }
                Some(&'o') | Some(&'O') => {
                    characters.next();
                    characters.next_if_eq(&SEPARATOR);
                }
                Some(&'x') | Some(&'X') => {
                    characters.next();
                    characters.next_if_eq(&SEPARATOR);
                }
                None => {
                    return Ok(Int {
                        sign: 0 as Sign,
                        digits: vec![0],
                    });
                }
                _ => {}
            };
        };
        if characters.peek() == Some(&SEPARATOR) {
            return Err(PyValueError::new_err("Should not start with separator."));
        }
        let digits_count = count_digits(characters.clone(), base)?;
        let digits = if base & (base - 1) == 0 {
            parse_binary_base_digits(characters, base, digits_count)?
        } else {
            parse_non_binary_base_digits(characters, base, digits_count)?
        };
        Ok(Int {
            sign: sign * ((digits.len() > 1 || digits[0] != 0) as Sign),
            digits,
        })
    }
}

fn parse_binary_base_digits(
    characters: Peekable<Chars>,
    base: u8,
    digits_count: usize,
) -> PyResult<Vec<big_int::Digit>> {
    let bits_per_character = utils::floor_log2(base) as usize;
    if digits_count > (usize::MAX - (BINARY_SHIFT - 1)) / bits_per_character {
        return Err(PyValueError::new_err("Too many digits."));
    }
    let result_digits_count =
        (digits_count * bits_per_character + (BINARY_SHIFT - 1)) / BINARY_SHIFT;
    let mut digits: Vec<big_int::Digit> = Vec::with_capacity(result_digits_count);
    let mut accumulator: big_int::DoubleDigit = 0;
    let mut bits_in_accumulator: usize = 0;
    let mut reversed_characters = characters.rev();
    while let Some(character) = reversed_characters.next() {
        if character == SEPARATOR {
            continue;
        }
        accumulator |= (ASCII_CODES_DIGIT_VALUES[character as usize] as big_int::DoubleDigit)
            << bits_in_accumulator;
        bits_in_accumulator += bits_per_character;
        if bits_in_accumulator >= BINARY_SHIFT {
            digits.push((accumulator & BINARY_DIGIT_MASK) as big_int::Digit);
            accumulator >>= BINARY_SHIFT;
            bits_in_accumulator -= BINARY_SHIFT;
        }
    }
    if bits_in_accumulator != 0 {
        digits.push(accumulator as big_int::Digit);
    }
    Ok(digits)
}

fn parse_non_binary_base_digits(
    mut characters: Peekable<Chars>,
    base: u8,
    digits_count: usize,
) -> PyResult<Vec<big_int::Digit>> {
    static mut bases_logs: [f64; 37] = [0.0; 37];
    static mut infimum_bases_exponents: [usize; 37] = [0; 37];
    static mut infimum_bases_powers: [usize; 37] = [0; 37];
    unsafe {
        if bases_logs[base as usize] == 0.0 {
            let mut infimum_base_power: usize = base as usize;
            let mut infimum_base_exponent: usize = 1;
            bases_logs[base as usize] = (base as f64).ln() / (BINARY_BASE as f64).ln();
            loop {
                let candidate: usize = infimum_base_power * (base as usize);
                if candidate > (BINARY_BASE as usize) {
                    break;
                }
                infimum_base_power = candidate;
                infimum_base_exponent += 1;
            }
            infimum_bases_powers[base as usize] = infimum_base_power;
            infimum_bases_exponents[base as usize] = infimum_base_exponent;
        }
    }
    let digits_count_upper_bound: f64;
    unsafe {
        digits_count_upper_bound = (digits_count as f64) * bases_logs[base as usize] + 1.0;
    }
    if digits_count_upper_bound > (usize::MAX as f64) / (big_int::Digit::BITS as f64) {
        return Err(PyOverflowError::new_err("Too many digits."));
    }
    let mut digits: Vec<big_int::Digit> = Vec::with_capacity(digits_count_upper_bound as usize);
    let infimum_base_exponent: usize;
    let infimum_base_power: usize;
    unsafe {
        infimum_base_exponent = infimum_bases_exponents[base as usize];
        infimum_base_power = infimum_bases_powers[base as usize];
    }
    while let Some(character) = characters.next() {
        if character == SEPARATOR {
            continue;
        }
        let mut digit: big_int::DoubleDigit =
            ASCII_CODES_DIGIT_VALUES[character as usize] as big_int::DoubleDigit;
        let mut base_exponent: usize = 1;
        while base_exponent < infimum_base_exponent {
            if let Some(character) = characters.next() {
                if character == SEPARATOR {
                    continue;
                }
                base_exponent += 1;
                digit = digit * (base as big_int::DoubleDigit)
                    + (ASCII_CODES_DIGIT_VALUES[character as usize] as big_int::DoubleDigit);
            } else {
                break;
            }
        }
        let base_power: usize = if base_exponent == infimum_base_exponent {
            infimum_base_power
        } else {
            (base as usize).pow(base_exponent as u32)
        };
        for index in 0..digits.len() {
            digit += (digits[index] as big_int::DoubleDigit) * (base_power as big_int::DoubleDigit);
            digits[index] = (digit & BINARY_DIGIT_MASK) as big_int::Digit;
            digit >>= BINARY_SHIFT;
        }
        if !digit.is_zero() {
            digits.push(digit as big_int::Digit);
        }
    }
    if digits.is_empty() {
        digits.push(0);
    }
    Ok(digits)
}

#[pyproto]
impl PyObjectProtocol for Int {
    fn __repr__(&self) -> PyResult<String> {
        let base_digits: Vec<big_int::Digit> =
            big_int::binary_digits_to_non_binary_base(&self.digits, BINARY_SHIFT, DECIMAL_BASE);
        let characters_count: usize = ((self.sign < 0) as usize)
            + (base_digits.len() - 1) * DECIMAL_SHIFT
            + utils::floor_log10(*base_digits.last().unwrap() as usize)
            + 1;
        let mut characters: String = String::with_capacity(characters_count);
        for index in 0..base_digits.len() - 1 {
            let mut remainder = base_digits[index];
            for _ in 0..DECIMAL_SHIFT {
                characters.push((('0' as u8) + ((remainder % 10) as u8)) as char);
                remainder /= 10;
            }
        }
        let mut remainder = *base_digits.last().unwrap();
        while !remainder.is_zero() {
            characters.push((('0' as u8) + ((remainder % 10) as u8)) as char);
            remainder /= 10;
        }
        if self.sign == 0 {
            characters.push('0');
        } else if self.sign < 0 {
            characters.push('-');
        }
        Ok(characters.chars().rev().collect())
    }
}

#[pyfunction]
fn binary_digits_to_binary_base(
    source_digits: &PyList,
    source_shift: &PyInt,
    target_shift: &PyInt,
) -> PyResult<Vec<u32>> {
    Ok(big_int::binary_digits_to_binary_base(
        &source_digits.extract()?,
        source_shift.extract()?,
        target_shift.extract()?,
    ))
}

#[pyfunction]
fn binary_digits_to_non_binary_base(
    source_digits: &PyList,
    source_shift: &PyInt,
    target_shift: &PyInt,
) -> PyResult<Vec<u32>> {
    Ok(big_int::binary_digits_to_non_binary_base(
        &source_digits.extract()?,
        source_shift.extract()?,
        target_shift.extract()?,
    ))
}

#[pymodule]
fn _rithm(_py: Python, module: &PyModule) -> PyResult<()> {
    module.setattr("__version__", "0.0.0")?;
    module.setattr("__doc__", "Arbitrary precision arithmetics.")?;
    module.add_class::<Int>()?;
    Ok(())
}
