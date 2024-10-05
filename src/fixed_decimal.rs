use core::fmt;
use std::{ops, str::FromStr};

use crate::error::Error;
use num_traits::ConstZero;

// TODO: make it private
pub trait Num:
    Copy
    + num_traits::Euclid
    + num_traits::Zero
    + num_traits::Pow<u8, Output = Self>
    + From<u8>
    + ops::Sub<Output = Self>
    + num_traits::ConstZero
    + Eq
    + Ord
    + FromStr
    + ops::Mul<Output = Self>
    + fmt::Debug
    + Default
{
    type Unsigned: Num + num_traits::Unsigned;

    fn abs(self) -> u128; // TODO: prober handle this
    fn is_positive(self) -> bool;
    fn is_negative(self) -> bool {
        !self.is_positive()
    }
}

impl Num for i128 {
    type Unsigned = u128;

    fn abs(self) -> Self::Unsigned {
        self.abs() as u128
    }

    fn is_positive(self) -> bool {
        self >= 0
    }
}
impl Num for u128 {
    type Unsigned = Self;

    fn abs(self) -> Self {
        self
    }
    fn is_positive(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct FixedDecimal<T: Num, const SCALE: u8>(T);

impl<T: Num, const SCALE: u8> FixedDecimal<T, SCALE> {
    /// Returns a `FixedDecimal` with `m` representation and corresponding `E` scale.
    ///
    /// # Arguments
    ///
    /// * `num` - An T that represents the `m` portion of the decimal number
    /// * `E` - A i32 representing the `e` portion of the decimal number.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_fixed_decimal::FixedDecimalI128P4;
    /// #
    /// let pi = FixedDecimalI128P4::new(31415);
    /// assert_eq!(pi.to_string(), "3.1415");
    /// ```
    #[must_use]
    pub fn new(num: T) -> Self {
        Self(num)
    }

    #[must_use]
    pub(crate) const fn mantissa(&self) -> T {
        self.0
    }

    #[must_use]
    pub(crate) fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    #[must_use]
    pub(crate) fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl<T: Num, const E: u8> fmt::Display for FixedDecimal<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let (rep, additional) = crate::str::to_str_internal(self, false, f.precision());
        if let Some(additional) = additional {
            let value = [rep.as_str(), "0".repeat(additional).as_str()].concat();
            f.pad_integral(self.is_positive(), "", value.as_str())
        } else {
            f.pad_integral(self.is_positive(), "", rep.as_str())
        }
    }
}

impl<T: Num, const E: u8> fmt::Debug for FixedDecimal<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl<T: Num, const E: u8> FromStr for FixedDecimal<T, E> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::str::parse_str_radix_10(s)
    }
}

impl<T: Num, const E: u8> ops::Add for FixedDecimal<T, E> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Num, const E: u8> ops::Sub for FixedDecimal<T, E> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl<T: Num, const E: u8> num_traits::ConstZero for FixedDecimal<T, E> {
    const ZERO: Self = Self(T::ZERO);
}

impl<T: Num, const E: u8> num_traits::Zero for FixedDecimal<T, E> {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}