use core::fmt;
use std::{marker::PhantomData, str::FromStr};

use crate::{ext_num_traits, FixedDecimal};

/// Serialize/deserialize Decimals as strings.
///
/// ```
/// # use serde::{Deserialize};
/// # use rust_fixed_decimal::FixedDecimalI128;
/// # use serde_json::json;
///
/// #[derive(Deserialize)]
/// pub struct StringExample {
///     value: FixedDecimalI128::<3>,
/// }
///
/// #[derive(serde::Deserialize, PartialEq, Debug)]
/// struct A {
///     value: FixedDecimalI128<2>,
/// }
///
/// let json = json!({
///     "value": "123.45"
/// });
/// assert_eq!(
///     A {
///         value: FixedDecimalI128::<2>::new(12345)
///     },
///     serde_json::from_value(json).unwrap()
/// );
///
/// ```
impl<'de, T: FromStr, const E: u8> serde::Deserialize<'de> for FixedDecimal<T, E>
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
    fn deserialize<D>(deserializer: D) -> Result<FixedDecimal<T, E>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(FixedDecimalVisitor::<T, E>(PhantomData))
    }
}

#[derive(Default)]
struct FixedDecimalVisitor<T, const E: u8>(PhantomData<T>);

impl<'de, T, const E: u8> serde::de::Visitor<'de> for FixedDecimalVisitor<T, E>
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
    type Value = FixedDecimal<T, E>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a FixedDecimal type representing with at most {E} decimal digits"
        )
    }

    fn visit_str<Err>(self, value: &str) -> Result<Self::Value, Err>
    where
        Err: serde::de::Error,
    {
        Self::Value::from_str(value)
            .map_err(|_| Err::invalid_value(serde::de::Unexpected::Str(value), &self))
    }
}
