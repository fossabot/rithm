use crate::utils;
use std::ops::{Add, AddAssign, BitOr, Mul, MulAssign, Neg, Shl, ShlAssign, Shr, ShrAssign};

pub trait AdditiveMonoid<Rhs = Self> = Add<Rhs, Output = Self> + Zero;

pub trait AssigningAdditiveMonoid<Rhs = Self> = AdditiveMonoid<Rhs> + AddAssign<Rhs>;

pub trait AssigningMultiplicativeMonoid<Rhs = Self> = MultiplicativeMonoid<Rhs> + MulAssign<Rhs>;

pub trait AssigningShiftingLeftMonoid<Rhs = Self> = ShiftingLeftMonoid<Rhs> + ShlAssign<Rhs>;

pub trait AssigningShiftingRightMonoid<Rhs = Self> = ShiftingRightMonoid<Rhs> + ShrAssign<Rhs>;

pub trait BitwiseOrMonoid<Rhs = Self> = BitOr<Rhs, Output = Self> + Zero;

pub trait MultiplicativeMonoid<Rhs = Self> = Mul<Rhs, Output = Self> + One;

pub trait ShiftingLeftMonoid<Rhs = Self> = Shl<Rhs, Output = Self> + Zero;

pub trait ShiftingRightMonoid<Rhs = Self> = Shr<Rhs, Output = Self> + Zero;

pub trait DoublePrecision: Sized {
    type Type: From<Self>;
}

impl DoublePrecision for i8 {
    type Type = i16;
}

impl DoublePrecision for i16 {
    type Type = i32;
}

impl DoublePrecision for i32 {
    type Type = i64;
}

impl DoublePrecision for i64 {
    type Type = i128;
}

impl DoublePrecision for u8 {
    type Type = u16;
}

impl DoublePrecision for u16 {
    type Type = u32;
}

impl DoublePrecision for u32 {
    type Type = u64;
}

impl DoublePrecision for u64 {
    type Type = u128;
}

pub trait Gcd<Rhs = Self> {
    type Output;

    fn gcd(self, other: Rhs) -> Self::Output;
}

macro_rules! plain_gcd_impl {
    ($($t:ty)*) => ($(
        impl Gcd for $t {
            type Output = Self;

            #[inline]
            fn gcd(self, other: Self) -> Self::Output {
                utils::gcd::<$t>(self, other)
            }
        }
    )*)
}

plain_gcd_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait ModularSub<Rhs = Self> {
    type Output;

    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}


macro_rules! plain_modular_sub_impl {
    ($($t:ty)*) => ($(
        impl ModularSub for $t {
            type Output = $t;

            #[inline]
            fn wrapping_sub(self, rhs: Self) -> Self::Output {
                <$t>::wrapping_sub(self, rhs)
            }
        }
    )*)
}

plain_modular_sub_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait One {
    fn one() -> Self;

    fn is_one(&self) -> bool;
}

macro_rules! plain_one_impl {
    ($($t:ty)*) => ($(
        impl One for $t {
            fn one() -> $t {1}

            #[inline]
            fn is_one(&self) -> bool {
                *self == Self::one()
            }
        }
    )*)
}

plain_one_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait Signed {
    type Type: Neg<Output = Self::Type>;
}

impl Signed for u8 {
    type Type = i8;
}

impl Signed for u16 {
    type Type = i16;
}

impl Signed for u32 {
    type Type = i32;
}

impl Signed for u64 {
    type Type = i64;
}

impl Signed for u128 {
    type Type = i128;
}

impl Signed for i8 {
    type Type = i8;
}

impl Signed for i16 {
    type Type = i16;
}

impl Signed for i32 {
    type Type = i32;
}

impl Signed for i64 {
    type Type = i64;
}

impl Signed for i128 {
    type Type = i128;
}

pub trait Zero {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

macro_rules! plain_zero_impl {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            fn zero() -> $t {0}

            #[inline]
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }
    )*)
}

plain_zero_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub type DoublePrecisionOf<T> = <T as DoublePrecision>::Type;
pub type SignedOf<T> = <T as Signed>::Type;