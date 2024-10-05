use crate::{constants::MAX_STR_BUFFER_SIZE, FixedDecimal, Num};

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
        let digit = ((value.mantissa().abs() / 10u128.pow(i)) % 10) as u8;
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
            for _ in 0..(prec.saturating_sub(SCALE.into())) {
                rep.push('0');
            }
        }
    }

    (rep, prec_rem)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{FixedDecimalI128P0, FixedDecimalI128P1, FixedDecimalI128P4, FixedDecimalI128P5};
    mod to_str_internal {
        use super::*;
        #[test]
        fn a() {
            assert_eq!(
                to_str_internal(&FixedDecimalI128P0::new(1234), true, None)
                    .0
                    .as_str(),
                "1234"
            );
        }
        #[test]
        fn b() {
            assert_eq!(
                to_str_internal(&FixedDecimalI128P0::new(-1234), true, None)
                    .0
                    .as_str(),
                "-1234"
            );
        }
        #[test]
        fn c() {
            assert_eq!(
                to_str_internal(&FixedDecimalI128P1::new(1234), true, None)
                    .0
                    .as_str(),
                "123.4"
            );
        }
        #[test]
        fn d() {
            assert_eq!(
                to_str_internal(&FixedDecimalI128P5::new(1234), true, None)
                    .0
                    .as_str(),
                "0.01234"
            );
        }

        #[test]
        fn e() {
            assert_eq!(
                to_str_internal(&FixedDecimalI128P4::new(1234), true, None)
                    .0
                    .as_str(),
                "0.1234"
            );
        }

        #[test]
        fn f() {
            assert_eq!(
                to_str_internal(&FixedDecimalI128P5::new(1234), true, Some(2))
                    .0
                    .as_str(),
                "0.01"
            );
        }

        #[test]
        fn g() {
            assert_eq!(
                to_str_internal(&FixedDecimalI128P0::new(1234), true, Some(2))
                    .0
                    .as_str(),
                "1234.00"
            );
        }

        #[test]
        fn h() {
            assert_eq!(
                to_str_internal(&FixedDecimalI128P5::new(1), true, Some(2))
                    .0
                    .as_str(),
                "0.00"
            );
        }
    }
}
