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
pub type FixedDecimalI64<const E: u8> = FixedDecimal<i64, E>;
pub type FixedDecimalU64<const E: u8> = FixedDecimal<u64, E>;
pub type FixedDecimalI32<const E: u8> = FixedDecimal<i32, E>;
pub type FixedDecimalU32<const E: u8> = FixedDecimal<u32, E>;
pub type FixedDecimalI16<const E: u8> = FixedDecimal<i16, E>;
pub type FixedDecimalU16<const E: u8> = FixedDecimal<u16, E>;
pub type FixedDecimalI8<const E: u8> = FixedDecimal<i8, E>;
pub type FixedDecimalU8<const E: u8> = FixedDecimal<u8, E>;
