/// Error type for the library.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// A generic error from Rust Decimal with the `String` containing more information as to what
    /// went wrong.
    /// The value provided exceeds `Self::MAX`.
    ExceedsMaximumPossibleValue,
    /// The value provided is less than `Self::MIN`.
    LessThanMinimumPossibleValue,
    /// An underflow is when there are more fractional digits than can be represented within `Decimal`.
    Underflow,
    /// The scale provided exceeds the maximum scale that `Decimal` can represent.
    ScaleExceedsMaximumPrecision(u32),
    /// Represents a failure to convert to/from `Decimal` to the specified type. This is typically
    /// due to type constraints (e.g. `Decimal::MAX` cannot be converted into `i32`).
    ConversionTo(String),
}
