use crate::{constants::MAX_STR_BUFFER_SIZE, error::Error, fixed_decimal::Num, FixedDecimal};

use arrayvec::ArrayString;

// impl that doesn't allocate for serialization purposes.
pub(crate) fn to_str_internal<T: Num, const SCALE: u8>(
    value: &FixedDecimal<T, SCALE>,
    append_sign: bool,
    precision: Option<usize>,
) -> (ArrayString<MAX_STR_BUFFER_SIZE>, Option<usize>) {
    let total_len = value.mantissa().abs().ilog10() + 1; // 4
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

pub(crate) fn parse_str_radix_10<T: Num, const SCALE: u8>(
    str: &str,
) -> Result<FixedDecimal<T, SCALE>, Error> {
    let (int, frac) = str
        .split_once('.')
        .map_or((str, None), |(int, frac)| (int, Some(frac)));

    // TODO: proper handle overflow

    let int = str::parse::<T>(int).unwrap_or_else(|_| panic!("fail to parse int part"));
    let frac = frac
        .map_or(Ok(T::ZERO), |v| {
            std::str::FromStr::from_str(&v[..std::cmp::min(v.len(), SCALE.into())])
        })
        .unwrap_or_else(|_| panic!("fail to parse frac part"));
    let shift = Into::<T>::into(10u8).pow(SCALE);

    Ok(FixedDecimal::<T, SCALE>::new(int * shift + frac))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{FixedDecimalI128, FixedDecimalU128};
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
    }

    mod parse_str_radix_10 {
        use super::*;

        #[test]
        fn a() {
            assert_eq!(
                parse_str_radix_10::<_, 2>("1234.56"),
                Ok(FixedDecimalI128::new(123456))
            );
        }
        #[test]
        fn b() {
            assert_eq!(
                parse_str_radix_10::<_, 2>("1234"),
                Ok(FixedDecimalI128::new(123400))
            );
        }
        #[test]
        fn c() {
            assert_eq!(
                parse_str_radix_10::<_, 2>("1234.567"),
                Ok(FixedDecimalI128::new(123456))
            );
        }
    }
}
