mod constants;
mod error;
mod ext_num_traits;
mod fixed_decimal;
mod str;

#[cfg(feature = "serde")]
mod serde;

pub use fixed_decimal::FixedDecimal;

pub type FixedDecimalI128<const E: u8> = FixedDecimal<i128, E>;
pub type FixedDecimalU128<const E: u8> = FixedDecimal<u128, E>;
