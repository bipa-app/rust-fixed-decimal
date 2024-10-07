# Fixed Decimal 

A Fixed Decimal number implementation written in pure Rust suitable for financial calculations that require significant
integral and fractional digits explicit round approuch.

The binary representation consists of native rust integer number, a const generic scaling factor used to specify the decimal fraction.

An *n* fixed-point number has `SCALE` digits as fractional
part, and the reminder as integer part. For example,
`FixedDecimalI164::<8>` is a 64 signed fixed-point number in format `IIIIIIIIIII.FFFFFFFF`, storing values between 92233720368.54775807 and -92233720368.54775808.

This library use a mixed aprouch, inspired by `rust-decimal` and `fixed`.

## Installing

```sh
$ cargo add rust_fixed_decimal
```

Alternatively, you can edit your `Cargo.toml` directly and run `cargo update`:

```toml
[dependencies]
rust_fixed_decimal = "0.1"
```

## Usage

Decimal numbers can be created in a few distinct ways. The easiest and most efficient method of creating a Decimal is to
use the procedural macro that can be enabled using the `macros` feature:

```rust
// Import the `rust_fixed_decimal_macros` crate and use the macro directly from there.
use rust_fixed_decimal::dec;

let number: FixedDecimalI128 = dec!(-1.23) + dec!(3.45);
assert_eq!(number, dec!(2.22));
assert_eq!(number.to_string(), "2.22");
```

Alternatively you can also use one of the Decimal number convenience
functions ([see the docs](https://docs.rs/rust_fixed_decimal/) for more details):

```rust
// Using the prelude can help importing trait based functions (e.g. core::str::FromStr).
use rust_fixed_decimal::prelude::*;

// Using an integer followed by the decimal points
let scaled = FixedDecimalI128::<2>::new(202);
assert_eq!("2.02", scaled.to_string());

// From a string representation
let from_string = FixedDecimalI128::<2>::::from_str("2.02").unwrap();
assert_eq!("2.02", from_string.to_string());
```

Once you have instantiated your `FixedDecimal` number you can perform calculations with it just like any other number:

```rust
use rust_fixed_decimal::prelude::*;

let amount: FixedDecimalI128 = dec!(25.120);
let tax_percentage = dec!(0.085);
let total = amount + amount * tax_percentage;
assert_eq!(total, dec!(27.260));
```

## Features
**Serde**

* [serde-str](#serde-str)

### `serde-str`

This is typically useful for `bincode` or `csv` like implementations.

It uses `FromStr` as default method for hanglig `FixedDecimal` numbers for serialization/deserialization rules.

## Building

Please refer to the [Build document](BUILD.md) for more information on building and testing Rust Decimal.

## Minimum Rust Compiler Version

The current _minimum_ compiler version
is [`1.81.0`](https://github.com/rust-lang/rust/releases/tag/1.81.0)
which was released on `2024-10-05`.

This library maintains support for rust compiler versions that are 4 minor versions away from the current stable rust
compiler version.
For example, if the current stable compiler version is `1.90.0` then we will guarantee support up to and
including `1.86.0`.
Of note, we will only update the minimum supported version if and when required.

## Comparison to other Decimal implementations

During the development of this library, there were various design decisions made to ensure that decimal calculations
would be quick, accurate and efficient. Some decisions, however, put limitations on what this library can do and ultimately
what it is suitable for. One such decision was the structure of the internal decimal representation.

This library use the native integer type as mantissa.
This structure allows us to make use of algorithmic optimizations to implement basic arithmetic; ultimately this gives
us the ability  to squeeze out performance and make it one of the fastest implementations available.
The downside of this approach however is that the maximum number of significant digits that can be represented
is roughly 38 base-10 digits (39 in some cases).

While this constraint is not an issue for many applications (e.g. when dealing with money), some applications may
require a higher number of significant digits to be represented. Fortunately,
there are alternative implementations that may be worth investigating, such as:

* [bigdecimal](https://crates.io/crates/bigdecimal)
* [decimal-rs](https://crates.io/crates/decimal-rs)

The library also store the scale in the type system, whitch restrict operations on the same scale and requires explict scale convertion.
The downside is for generic parser you could not know before hand the necessary scale.
Fortunaly, there are alternative implementations there are alternative implementations that may be worth to investigating, such as:

* [decimal-rs](https://crates.io/crates/rust-decimal)

If you have further questions about the suitability of this library for your project, then feel free to either start a
[discussion](https://github.com/bipa-app/rust-fixed-decimal/discussions) or open
an [issue](https://github.com/bipa-app/rust-fixed-decimal/issues) and we'll
do our best to help.
