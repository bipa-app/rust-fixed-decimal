use core::fmt;
use std::{
    ops::{self},
    str::FromStr,
};

use num_traits::ConstZero;

// TODO: make it private
pub trait Num:
    Copy
    + num_traits::Euclid
    + num_traits::Zero
    + num_traits::Pow<u8, Output = Self>
    + num_traits::CheckedMul
    + num_traits::Num
    + From<u8>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Rem<Output = Self>
    + ops::Div<Output = Self>
    + num_traits::ConstZero
    + Eq
    + Ord
    + FromStr<Err = std::num::ParseIntError>
    + fmt::Debug
    + Default
{
    type Unsigned: Num + num_traits::Unsigned;
    const MAX: Self;
    const MIN: Self;

    fn uabs(self) -> u128; // TODO: prober handle this
    fn internal_is_positive(self) -> bool;
    fn is_negative(self) -> bool {
        !self.internal_is_positive()
    }
    fn force_neg(self) -> Self;
}

impl Num for i128 {
    type Unsigned = u128;
    const MAX: Self = i128::MAX;
    const MIN: Self = i128::MIN;

    fn uabs(self) -> Self::Unsigned {
        if self == i128::MIN {
            170_141_183_460_469_231_731_687_303_715_884_105_728
        } else {
            self.abs() as u128
        }
    }

    fn internal_is_positive(self) -> bool {
        self >= 0
    }

    fn force_neg(self) -> Self {
        -self
    }
}
impl Num for u128 {
    type Unsigned = Self;
    const MAX: Self = u128::MAX;
    const MIN: Self = u128::MIN;

    fn uabs(self) -> Self {
        self
    }
    fn internal_is_positive(self) -> bool {
        true
    }

    fn force_neg(self) -> Self {
        panic!("cannot neg u128");
    }
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct FixedDecimal<T: Num, const SCALE: u8>(pub(crate) T);

impl<T: Num, const SCALE: u8> FixedDecimal<T, SCALE> {
    pub const MAX: Self = Self(T::MAX);
    pub const MIN: Self = Self(T::MIN);
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

    /// Returns the mantissa of the decimal number.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_fixed_decimal::FixedDecimalI128;
    /// # use std::str::FromStr;
    ///
    /// let num = FixedDecimalI128::<7>::from_str("-1.2345678").unwrap();
    /// assert_eq!(num.mantissa(), -12345678i128);
    /// ```
    #[must_use]
    pub const fn mantissa(&self) -> T {
        self.0
    }
}

impl<T: Num, const E: u8> fmt::Display for FixedDecimal<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let (rep, additional) = crate::str::to_str_internal(self, false, f.precision());
        if let Some(additional) = additional {
            let value = [rep.as_str(), "0".repeat(additional).as_str()].concat();
            f.pad_integral(self.0.internal_is_positive(), "", value.as_str())
        } else {
            f.pad_integral(self.0.internal_is_positive(), "", rep.as_str())
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
        Self(self.0.add(rhs.0))
    }
}

impl<T: Num, const E: u8> ops::Sub for FixedDecimal<T, E> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
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
        self.0.is_zero()
    }
}

impl<T: Num + ops::Neg<Output = T>, const E: u8> ops::Neg for FixedDecimal<T, E> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl<T: Num, const E: u8> ops::Mul for FixedDecimal<T, E> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

impl<T: Num, const E: u8> ops::Rem for FixedDecimal<T, E> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0.rem(rhs.0))
    }
}
impl<T: Num, const E: u8> ops::Div for FixedDecimal<T, E> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}

impl<T: Num, const E: u8> num_traits::One for FixedDecimal<T, E> {
    fn one() -> Self {
        Self(Into::<T>::into(10u8).pow(E))
    }
}

impl<T: Num, const E: u8> num_traits::Num for FixedDecimal<T, E> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl<T: Num + num_traits::Signed, const E: u8> num_traits::Signed for FixedDecimal<T, E> {
    fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self(self.0.abs_sub(&other.0))
    }

    fn signum(&self) -> Self {
        Self(self.0.signum())
    }

    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}
