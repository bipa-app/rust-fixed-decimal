use core::fmt;
use std::{ops, str::FromStr};

use crate::ext_num_traits;

pub struct FixedDecimal<T, const SCALE: u8>(pub(crate) T);

impl<T, const E: u8> FixedDecimal<T, E> {
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
}

#[expect(private_bounds)]
impl<T: ext_num_traits::ConstBound, const E: u8> FixedDecimal<T, E> {
    pub const MAX: Self = Self(T::MAX);
    pub const MIN: Self = Self(T::MIN);
}

impl<T: Copy, const E: u8> FixedDecimal<T, E> {
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
impl<T: Copy, const E: u8> Copy for FixedDecimal<T, E> {}

impl<T: Clone, const E: u8> Clone for FixedDecimal<T, E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Eq, const E: u8> Eq for FixedDecimal<T, E> {}
impl<T: PartialEq, const E: u8> PartialEq for FixedDecimal<T, E> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T: Ord, const E: u8> Ord for FixedDecimal<T, E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T: PartialOrd, const E: u8> PartialOrd for FixedDecimal<T, E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T, const E: u8> fmt::Display for FixedDecimal<T, E>
where
    T: ext_num_traits::UAbs + num_traits::Zero + ext_num_traits::Sign + Copy,
    <T as ext_num_traits::ExtSigned>::Unsigned: ext_num_traits::ILog10
        + ext_num_traits::Ten
        + TryInto<u8>
        + std::fmt::Debug
        + Copy
        + num_traits::Pow<u8, Output = <T as ext_num_traits::ExtSigned>::Unsigned>,
    <<T as ext_num_traits::ExtSigned>::Unsigned as TryInto<u8>>::Error: std::fmt::Debug,
{
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

impl<T, const E: u8> fmt::Debug for FixedDecimal<T, E>
where
    T: ext_num_traits::UAbs + num_traits::Zero + ext_num_traits::Sign + Copy,
    <T as ext_num_traits::ExtSigned>::Unsigned: ext_num_traits::ILog10
        + ext_num_traits::Ten
        + TryInto<u8>
        + std::fmt::Debug
        + Copy
        + num_traits::Pow<u8, Output = <T as ext_num_traits::ExtSigned>::Unsigned>,
    <<T as ext_num_traits::ExtSigned>::Unsigned as TryInto<u8>>::Error: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl<T, const E: u8> FromStr for FixedDecimal<T, E>
where
    T: num_traits::ConstZero
        + num_traits::ConstOne
        + FromStr
        + From<u8>
        + ext_num_traits::ConstTen
        + num_traits::CheckedMul
        + num_traits::CheckedAdd
        + ext_num_traits::NegateIfSigned
        + ext_num_traits::IsSigned
        + Copy
        + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    type Err = crate::str::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::str::parse_str_radix_10_exact(s)
    }
}

impl<T: ops::Add<Output = T>, const E: u8> ops::Add for FixedDecimal<T, E> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl<T: ops::AddAssign, const E: u8> ops::AddAssign for FixedDecimal<T, E> {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0)
    }
}
impl<'a, T: ops::AddAssign + Copy, const E: u8> ops::AddAssign<&'a FixedDecimal<T, E>>
    for FixedDecimal<T, E>
{
    fn add_assign(&mut self, rhs: &'a Self) {
        Self::add_assign(self, *rhs)
    }
}
impl<'a, T: ops::AddAssign, const E: u8> ops::AddAssign<FixedDecimal<T, E>>
    for &'a mut FixedDecimal<T, E>
{
    fn add_assign(&mut self, rhs: FixedDecimal<T, E>) {
        self.0.add_assign(rhs.0)
    }
}

impl<'a, T: ops::AddAssign + Copy, const E: u8> ops::AddAssign<&'a FixedDecimal<T, E>>
    for &'a mut FixedDecimal<T, E>
{
    fn add_assign(&mut self, rhs: &'a FixedDecimal<T, E>) {
        self.0.add_assign(rhs.0)
    }
}

impl<T: num_traits::CheckedAdd, const E: u8> num_traits::CheckedAdd for FixedDecimal<T, E> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        self.0.checked_add(&v.0).map(Self)
    }
}

impl<T: ops::Sub<Output = T>, const E: u8> ops::Sub for FixedDecimal<T, E> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}
impl<T: num_traits::ConstZero, const E: u8> num_traits::ConstZero for FixedDecimal<T, E> {
    const ZERO: Self = Self(T::ZERO);
}

impl<T: num_traits::Zero, const E: u8> num_traits::Zero for FixedDecimal<T, E> {
    fn zero() -> Self {
        Self(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: ops::Neg<Output = T>, const E: u8> ops::Neg for FixedDecimal<T, E> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl<T: ops::Mul<Output = T>, const E: u8> ops::Mul for FixedDecimal<T, E> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

impl<T: ops::Rem<Output = T>, const E: u8> ops::Rem for FixedDecimal<T, E> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0.rem(rhs.0))
    }
}
impl<T: ops::Div<Output = T>, const E: u8> ops::Div for FixedDecimal<T, E> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}

// Trait can't be const yet (https://github.com/rust-lang/rust/issues/67792), so we can't use ext_num_crates for it
macro_rules! impl_const_one {
    ($($tty:ty),+) => {
        $(
            impl<const E: u8> num_traits::ConstOne for FixedDecimal<$tty, E> {
                const ONE: Self = Self((10 as $tty).pow(E as u32));
            }
        )+
    };
}
impl_const_one!(i128, u128, i64, u64, i32, u32, i16, u16, i8, u8);

impl<
        T: ext_num_traits::Ten + num_traits::Pow<u8, Output = T> + ops::Mul<T, Output = T>,
        const E: u8,
    > num_traits::One for FixedDecimal<T, E>
{
    fn one() -> Self {
        Self(T::ten().pow(E))
    }
}
