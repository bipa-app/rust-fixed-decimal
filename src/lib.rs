mod constants;
mod error;
mod fixed_decimal;
mod str;

pub use fixed_decimal::FixedDecimal;

pub type FixedDecimalI128<const E: u8> = FixedDecimal<i128, E>;

pub type FixedDecimalI128P0 = FixedDecimal<i128, 0>;
pub type FixedDecimalI128P1 = FixedDecimal<i128, 1>;
pub type FixedDecimalI128P2 = FixedDecimal<i128, 2>;
pub type FixedDecimalI128P3 = FixedDecimal<i128, 3>;
pub type FixedDecimalI128P4 = FixedDecimal<i128, 4>;
pub type FixedDecimalI128P5 = FixedDecimal<i128, 5>;
