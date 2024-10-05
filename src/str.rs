use std::num::IntErrorKind;

use crate::{constants::MAX_STR_BUFFER_SIZE, fixed_decimal::Num, FixedDecimal};

use arrayvec::ArrayString;
use num_traits::Zero;

// impl that doesn't allocate for serialization purposes.
pub(crate) fn to_str_internal<T: Num, const SCALE: u8>(
    value: &FixedDecimal<T, SCALE>,
    append_sign: bool,
    precision: Option<usize>,
) -> (ArrayString<MAX_STR_BUFFER_SIZE>, Option<usize>) {
    let total_len = if value.is_zero() {
        1
    } else {
        value.mantissa().abs().ilog10() + 1
    };
    let (prec, prec_rem) = match precision {
        Some(prec) => {
            let max: usize = u8::MAX.into();
            if prec > max {
                (max, Some(prec - max))
            } else {
                (prec, None)
            }
        }
        None => (SCALE.into(), None),
    };

    let mut rep = ArrayString::<MAX_STR_BUFFER_SIZE>::new();

    if append_sign && value.is_negative() {
        rep.push('-');
    }

    let push_digit = |rep: &mut ArrayString<MAX_STR_BUFFER_SIZE>, i| {
        let digit = if i >= total_len {
            0
        } else {
            ((value.mantissa().abs() / 10u128.pow(i)) % 10) as u8
        };
        rep.push(char::from(b'0' + digit));
    };

    if total_len <= SCALE.into() {
        rep.push('0');
        rep.push('.');
        for i in (0..SCALE).rev().take(prec) {
            push_digit(&mut rep, i.into());
        }
    } else {
        for i in (SCALE.into()..total_len).rev() {
            push_digit(&mut rep, i);
        }
        if prec != 0 {
            rep.push('.');
            for i in (0u32..SCALE.into()).rev() {
                push_digit(&mut rep, i);
            }
        }
    }
    for _ in 0..(prec.saturating_sub(SCALE.into())) {
        rep.push('0');
    }

    (rep, prec_rem)
}

/// Error type for the library.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ParseError {
    Underflow,
    IntErr(IntErrorKind),
}
pub(crate) fn parse_str_radix_10_exact<T: Num, const SCALE: u8>(
    str: &str,
) -> Result<FixedDecimal<T, SCALE>, ParseError> {
    let is_negative = str.starts_with('-');
    let (int, frac) = str
        .split_once('.')
        .map_or((str, None), |(int, frac)| (int, Some(frac)));

    if frac.is_some_and(|f| f.trim_start_matches('0').len() > SCALE.into()) {
        return Err(ParseError::Underflow);
    }

    let int = str::parse::<T>(int).map_err(|e| ParseError::IntErr(e.kind().clone()))?;
    let frac = frac.map_or(Ok(T::ZERO), |v| {
        str::parse::<T>(v).map_err(|e| ParseError::IntErr(e.kind().clone()))
    })?;
    let high = if !int.is_zero() {
        let shift = Into::<T>::into(10u8).pow(SCALE);
        int.checked_mul(&shift).ok_or_else(|| {
            if int.is_positive() {
                ParseError::IntErr(IntErrorKind::PosOverflow)
            } else {
                ParseError::IntErr(IntErrorKind::PosOverflow)
            }
        })?
    } else {
        T::ZERO
    };

    // TODO: improve gambiarra
    let frac = if is_negative {
        frac.force_neg() // This may panic with "-0".parse::<u128>();
    } else {
        frac
    };

    Ok(FixedDecimal::<T, SCALE>::new(high + frac))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{FixedDecimalI128, FixedDecimalU128};
    use proptest::prelude::*;
    mod to_str_internal {

        use super::*;
        #[test]
        fn a() {
            assert_eq!(
                to_str_internal::<_, 0>(&FixedDecimalI128::new(1234), true, None)
                    .0
                    .as_str(),
                "1234"
            );
        }
        #[test]
        fn b() {
            assert_eq!(
                to_str_internal::<_, 0>(&FixedDecimalI128::new(-1234), true, None)
                    .0
                    .as_str(),
                "-1234"
            );
        }
        #[test]
        fn c() {
            assert_eq!(
                to_str_internal::<_, 1>(&FixedDecimalI128::new(1234), true, None)
                    .0
                    .as_str(),
                "123.4"
            );
        }
        #[test]
        fn d() {
            assert_eq!(
                to_str_internal::<_, 5>(&FixedDecimalI128::new(1234), true, None)
                    .0
                    .as_str(),
                "0.01234"
            );
        }

        #[test]
        fn e() {
            assert_eq!(
                to_str_internal::<_, 4>(&FixedDecimalI128::new(1234), true, None)
                    .0
                    .as_str(),
                "0.1234"
            );
        }

        #[test]
        fn f() {
            assert_eq!(
                to_str_internal::<_, 5>(&FixedDecimalI128::new(1234), true, Some(2))
                    .0
                    .as_str(),
                "0.01"
            );
        }

        #[test]
        fn g() {
            assert_eq!(
                to_str_internal::<_, 0>(&FixedDecimalI128::new(1234), true, Some(2))
                    .0
                    .as_str(),
                "1234.00"
            );
        }

        #[test]
        fn h() {
            assert_eq!(
                to_str_internal::<_, 5>(&FixedDecimalI128::new(1), true, Some(2))
                    .0
                    .as_str(),
                "0.00"
            );
        }

        #[test]
        fn i() {
            assert_eq!(
                to_str_internal::<_, { u8::MAX }>(&FixedDecimalI128::new(1), true, None)
                    .0
                    .as_str(),
                "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"
            );
        }
        #[test]
        fn k() {
            assert_eq!(
                to_str_internal::<_, { u8::MAX }>(&FixedDecimalI128::new(i128::MAX), true, None)
                    .0
                    .as_str(),
                "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000170141183460469231731687303715884105727"
            );
        }
        #[test]
        fn j() {
            assert_eq!(
                to_str_internal::<_, { u8::MAX }>(&FixedDecimalI128::new(i128::MIN), true, None)
                    .0
                    .as_str(),
                "-0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000170141183460469231731687303715884105728"
            );
        }
        #[test]
        fn l() {
            assert_eq!(
                to_str_internal::<_, { u8::MAX }>(&FixedDecimalU128::new(u128::MAX), true, None)
                    .0
                    .as_str(),
                "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000340282366920938463463374607431768211455"
            );
        }
        #[test]
        fn m() {
            let (buffer, extra) =
                to_str_internal::<_, 5>(&FixedDecimalU128::new(1), true, Some(500));
            assert_eq!(buffer.len() + extra.unwrap(), 502);
        }
        #[test]
        fn n() {
            assert_eq!(
                to_str_internal::<_, 0>(&FixedDecimalI128::new(i128::MAX), true, None)
                    .0
                    .as_str(),
                "170141183460469231731687303715884105727"
            );
        }
        #[test]
        fn o() {
            assert_eq!(
                to_str_internal::<_, 0>(&FixedDecimalI128::new(i128::MIN), true, None)
                    .0
                    .as_str(),
                "-170141183460469231731687303715884105728"
            );
        }
        #[test]
        fn p() {
            assert_eq!(
                to_str_internal::<_, 0>(&FixedDecimalU128::new(u128::MAX), true, None)
                    .0
                    .as_str(),
                "340282366920938463463374607431768211455"
            );
        }
        #[test]
        fn q() {
            assert_eq!(
                to_str_internal::<_, 5>(&FixedDecimalU128::new(0), true, None)
                    .0
                    .as_str(),
                "0.00000"
            );
        }
    }

    mod parse_str_radix_10 {
        use super::*;

        #[test]
        fn a() {
            assert_eq!(
                parse_str_radix_10_exact::<_, 2>("1234.56"),
                Ok(FixedDecimalI128::new(123456))
            );
        }
        #[test]
        fn b() {
            assert_eq!(
                parse_str_radix_10_exact::<_, 2>("1234"),
                Ok(FixedDecimalI128::new(123400))
            );
        }
        #[test]
        fn c() {
            assert_eq!(
                parse_str_radix_10_exact::<i128, 2>("1234.567"),
                Err(ParseError::Underflow)
            );
        }
        #[test]
        fn d() {
            let d = FixedDecimalI128::<57>::new(0);
            assert_eq!(parse_str_radix_10_exact(&d.to_string()), Ok(d));
        }
        #[test]
        fn e() {
            assert_eq!(
                parse_str_radix_10_exact::<i128, 2>(""),
                Err(ParseError::IntErr(IntErrorKind::Empty))
            );
        }
        #[test]
        fn f() {
            assert_eq!(
                parse_str_radix_10_exact::<u128, 3>("-0.001"),
                Err(ParseError::IntErr(IntErrorKind::InvalidDigit))
            );
        }

        proptest! {
            #[test]
            fn parse_str_is_the_opposite_of_to_str_i128_0(v in any::<i128>()) {
                let d = FixedDecimalI128::<0>::new(v);
                assert_eq!(parse_str_radix_10_exact(&d.to_string()), Ok(d));
            }
            #[test]
            fn parse_str_is_the_opposite_of_to_str_i128_57(v in any::<i128>()) {
                let d = FixedDecimalI128::<57>::new(v);
                assert_eq!(parse_str_radix_10_exact(&d.to_string()), Ok(d));
            }
            #[test]
            fn parse_str_is_the_opposite_of_to_str_i128_173(v in any::<i128>()) {
                let d = FixedDecimalI128::<173>::new(v);
                assert_eq!(parse_str_radix_10_exact(&d.to_string()), Ok(d));
            }
            #[test]
            fn parse_str_is_the_opposite_of_to_str_i128_255(v in any::<i128>()) {
                let d = FixedDecimalI128::<{u8::MAX}>::new(v);
                assert_eq!(parse_str_radix_10_exact(&d.to_string()), Ok(d));
            }

            #[test]
            fn parse_str_is_the_opposite_of_to_str_u128_0(v in any::<u128>()) {
                let d = FixedDecimalU128::<0>::new(v);
                assert_eq!(parse_str_radix_10_exact(&d.to_string()), Ok(d));
            }
            #[test]
            fn parse_str_is_the_opposite_of_to_str_u128_57(v in any::<u128>()) {
                let d = FixedDecimalU128::<57>::new(v);
                assert_eq!(parse_str_radix_10_exact(&d.to_string()), Ok(d));
            }
            #[test]
            fn parse_str_is_the_opposite_of_to_str_u128_173(v in any::<u128>()) {
                let d = FixedDecimalU128::<173>::new(v);
                assert_eq!(parse_str_radix_10_exact(&d.to_string()), Ok(d));
            }
            #[test]
            fn parse_str_is_the_opposite_of_to_str_u128_255(v in any::<u128>()) {
                let d = FixedDecimalU128::<{u8::MAX}>::new(v);
                assert_eq!(parse_str_radix_10_exact(&d.to_string()), Ok(d));
            }

            #[test]
            fn should_never_panic_i128_7(v in r"-?[0-9]{1,}\.[0-9]{1,}") {
                parse_str_radix_10_exact::<i128, 7>(&v).ok();
            }
            #[test]
            fn should_never_panic_u128_7(v in r"-?[0-9]{1,}\.[0-9]{1,}") {
                parse_str_radix_10_exact::<u128, 7>(&v).ok();
            }
        }
    }
}
