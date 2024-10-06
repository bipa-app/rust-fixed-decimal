use core::fmt;
use std::{ops, str::FromStr};

use num_traits::ConstZero;

// TODO: make it private
pub trait Num:
    Copy
    + num_traits::Euclid
    + num_traits::Zero
    + num_traits::Pow<u8, Output = Self>
    + num_traits::CheckedMul
    + From<u8>
    + ops::Sub<Output = Self>
    + num_traits::ConstZero
    + Eq
    + Ord
    + FromStr<Err = std::num::ParseIntError>
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
    fn force_neg(self) -> Self;
}

impl Num for i128 {
    type Unsigned = u128;

    fn abs(self) -> Self::Unsigned {
        if self == i128::MIN {
            170_141_183_460_469_231_731_687_303_715_884_105_728
        } else {
            self.abs() as u128
        }
    }

    fn is_positive(self) -> bool {
        self >= 0
    }

    fn force_neg(self) -> Self {
        -self
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

    fn force_neg(self) -> Self {
        panic!("cannot neg u128");
    }
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct FixedDecimal<T: Num, const SCALE: u8>(pub(crate) T);

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
    /// # use rust_fixed_decimal::FixedDecimalI128;
    /// #
    /// let pi = FixedDecimalI128::<4>::new(31415);
    /// assert_eq!(pi.to_string(), "3.1415");
    /// ```
    #[must_use]
    pub fn new(num: T) -> Self {
        Self(num)
    }

    /// Cast a `FixedDecimal` scale keeping the mantissa.
    /// "Only move the point separator"
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_fixed_decimal::FixedDecimalI128;
    /// #
    /// type Pennie = FixedDecimalI128<0>;
    /// type Dollar = FixedDecimalI128<2>;
    ///
    /// let amount_cents = Pennie::new(123_45);
    /// let amount: Dollar = amount_cents.with_scale::<2>();
    /// assert_eq!(amount.to_string(), "123.45");
    /// ```
    #[must_use]
    pub fn with_scale<const TARGET_SCALE: u8>(self) -> FixedDecimal<T, TARGET_SCALE> {
        FixedDecimal::<T, TARGET_SCALE>::new(self.0)
    }

    #[must_use]
    pub(crate) const fn mantissa(&self) -> T {
        self.0
    }
}

impl<T: Num, const E: u8> fmt::Display for FixedDecimal<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let (rep, additional) = crate::str::to_str_internal(self, false, f.precision());
        if let Some(additional) = additional {
            let value = [rep.as_str(), "0".repeat(additional).as_str()].concat();
            f.pad_integral(self.0.is_positive(), "", value.as_str())
        } else {
            f.pad_integral(self.0.is_positive(), "", rep.as_str())
        }
    }
}

impl<T: Num, const E: u8> fmt::Debug for FixedDecimal<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl<T: Num, const E: u8> FromStr for FixedDecimal<T, E> {
    type Err = crate::str::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::str::parse_str_radix_10_exact(s)
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
