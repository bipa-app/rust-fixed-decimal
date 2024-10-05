mod constants;
mod str;

use core::fmt;
use std::ops;

use num_traits::ConstZero;

// TODO: make it private
pub trait Num:
    Copy
    + num_traits::Euclid
    + num_traits::Zero
    + From<u8>
    + ops::Sub<Output = Self>
    + num_traits::ConstZero
    + Eq
    + Ord
{
    type Unsigned: Num + num_traits::Unsigned;

    fn abs(self) -> u128;
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

/// `FixedDecimal` represents a 128 or 64 bits representation of a fixed-precision decimal number.
/// The finite set of values of type `FixedDecimal` are of the form m / 10<sup>E</sup>,
/// where m is an integer `T`, and e is an i32
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct FixedDecimal<T: Num, const SCALE: u8>(T);

/// `Decimal` represents a 128 bit representation of a fixed-precision decimal number.
/// The finite set of values of type `Decimal` are of the form m / 10<sup>e</sup>,
/// where m is an integer such that -2<sup>96</sup> < m < 2<sup>96</sup>, and e is i32.
pub type FixedDecimalI128<const E: i32> = FixedDecimal<i128, E>;

pub type FixedDecimalI128P2 = FixedDecimal<i128, 2>;
pub type FixedDecimalI128P4 = FixedDecimal<i128, 4>;

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
