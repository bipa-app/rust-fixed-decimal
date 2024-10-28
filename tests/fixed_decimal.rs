// Most of test case are based on rust_decimal test cases
use num_traits::*;
use proptest::prelude::*;
use std::str::FromStr;

use rust_fixed_decimal::{FixedDecimal, FixedDecimalI128, FixedDecimalI8, FixedDecimalU128};
macro_rules! d5 {
    ($v:literal) => {
        FixedDecimalI128::<6>::from_str($v).unwrap()
    };
}

// Consts
#[test]
fn it_consts_bounds() {
    assert_eq!(FixedDecimal::<i128, 0>::MAX.mantissa(), i128::MAX);
    assert_eq!(FixedDecimal::<i128, 0>::MIN.mantissa(), i128::MIN);
    assert_eq!(FixedDecimal::<u128, 0>::MAX.mantissa(), u128::MAX);
    assert_eq!(FixedDecimal::<u128, 0>::MIN.mantissa(), u128::MIN);
}

#[test]
fn it_consts_one() {
    assert_eq!(
        FixedDecimalI128::<0>::ONE,
        FixedDecimalI128::from_str("1").unwrap()
    );
    assert_eq!(
        FixedDecimalI128::<7>::ONE,
        FixedDecimalI128::from_str("1").unwrap()
    );
}

#[test]
fn it_zero() {
    assert_eq!(FixedDecimalI128::<0>::zero(), FixedDecimalI128::new(0));
    assert_eq!(FixedDecimalI128::<7>::zero(), FixedDecimalI128::new(0));
}

#[test]
fn it_clones() {
    let d = FixedDecimalI128::<7>::from_str("1").unwrap();
    assert_eq!(d, d.clone());
}

// Formatting

#[test]
fn it_formats() {
    let a = FixedDecimalI128::<6>::from_str("233.323223").unwrap();
    assert_eq!(format!("{a}"), "233.323223");
    assert_eq!(format!("{a:.9}"), "233.323223000");
    assert_eq!(format!("{a:.0}"), "233");
    assert_eq!(format!("{a:.2}"), "233.32");
    assert_eq!(format!("{a:010.2}"), "0000233.32");
    assert_eq!(format!("{a:0<10.2}"), "233.320000");
    assert_eq!(format!("{a:+}"), "+233.323223");
    assert_eq!(format!("{a:.300}"), "233.323223000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(format!("{a:?}"), "233.323223");
}
#[test]
fn it_formats_neg() {
    let a = FixedDecimalI128::<6>::from_str("-233.323223").unwrap();
    assert_eq!(format!("{a}"), "-233.323223");
    assert_eq!(format!("{a:.9}"), "-233.323223000");
    assert_eq!(format!("{a:.0}"), "-233");
    assert_eq!(format!("{a:.2}"), "-233.32");
    assert_eq!(format!("{a:010.2}"), "-000233.32");
    assert_eq!(format!("{a:0<10.2}"), "-233.32000");
}
#[test]
fn it_formats_small() {
    let a = FixedDecimalI128::<4>::from_str("0.2223").unwrap();
    assert_eq!(format!("{a}"), "0.2223");
    assert_eq!(format!("{a:.9}"), "0.222300000");
    assert_eq!(format!("{a:.0}"), "0");
    assert_eq!(format!("{a:.2}"), "0.22");
    assert_eq!(format!("{a:010.2}"), "0000000.22");
    assert_eq!(format!("{a:0<10.2}"), "0.22000000");
}
#[test]
fn it_formats_small_leading_zeros() {
    let a = FixedDecimalI128::<16>::from_str("0.0023554701772169").unwrap();
    assert_eq!(format!("{a}"), "0.0023554701772169");
    assert_eq!(format!("{a:.9}"), "0.002355470");
    assert_eq!(format!("{a:.0}"), "0");
    assert_eq!(format!("{a:.2}"), "0.00");
    assert_eq!(format!("{a:010.2}"), "0000000.00");
    assert_eq!(format!("{a:0<10.2}"), "0.00000000");
}
#[test]
fn it_formats_small_neg() {
    let a = FixedDecimalI128::<4>::from_str("-0.2223").unwrap();
    assert_eq!(format!("{a}"), "-0.2223");
    assert_eq!(format!("{a:.9}"), "-0.222300000");
    assert_eq!(format!("{a:.0}"), "-0");
    assert_eq!(format!("{a:.2}"), "-0.22");
    assert_eq!(format!("{a:010.2}"), "-000000.22");
    assert_eq!(format!("{a:0<10.2}"), "-0.2200000");
}

#[test]
fn it_formats_zero() {
    let a = FixedDecimalI128::<0>::from_str("0").unwrap();
    assert_eq!(format!("{a}"), "0");
    assert_eq!(format!("{a:.9}"), "0.000000000");
    assert_eq!(format!("{a:.0}"), "0");
    assert_eq!(format!("{a:.2}"), "0.00");
    assert_eq!(format!("{a:010.2}"), "0000000.00");
    assert_eq!(format!("{a:0<10.2}"), "0.00000000");
}

#[test]
fn it_formats_int() {
    let a = FixedDecimalI128::<0>::from_str("5").unwrap();
    assert_eq!(format!("{a}"), "5");
    assert_eq!(format!("{a:.9}"), "5.000000000");
    assert_eq!(format!("{a:.0}"), "5");
    assert_eq!(format!("{a:.2}"), "5.00");
    assert_eq!(format!("{a:010.2}"), "0000005.00");
    assert_eq!(format!("{a:0<10.2}"), "5.00000000");
}

#[test]
fn it_formats_big_scale() {
    let a = FixedDecimalI128::<{ u8::MAX }>::new(1);
    assert_eq!(format!("{a}"), "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
    let a = FixedDecimalI128::<{ u8::MAX }>::new(i128::MAX);
    assert_eq!(format!("{a}"), "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000170141183460469231731687303715884105727");
    let a = FixedDecimalI128::<{ u8::MAX }>::new(i128::MIN);
    assert_eq!(format!("{a}"), "-0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000170141183460469231731687303715884105728");
    let a = FixedDecimalU128::<{ u8::MAX }>::new(u128::MAX);
    assert_eq!(format!("{a}"), "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000340282366920938463463374607431768211455");
}

#[test]
fn it_formats_big_int() {
    let a = FixedDecimalI128::<0>::new(i128::MAX);
    assert_eq!(format!("{a}"), i128::MAX.to_string());
    let a = FixedDecimalI128::<0>::new(i128::MIN);
    assert_eq!(format!("{a}"), i128::MIN.to_string());
    let a = FixedDecimalU128::<0>::new(u128::MAX);
    assert_eq!(format!("{a}"), u128::MAX.to_string());
}

proptest! {
    #[test]
    fn formats_never_panic_i128(v in any::<i128>()) {
        FixedDecimalI128::<0>::new(v).to_string();
        FixedDecimalI128::<7>::new(v).to_string();
        FixedDecimalI128::<13>::new(v).to_string();
        FixedDecimalI128::<21>::new(v).to_string();
        FixedDecimalI128::<32>::new(v).to_string();
        FixedDecimalI128::<41>::new(v).to_string();
        FixedDecimalI128::<57>::new(v).to_string();
        FixedDecimalI128::<100>::new(v).to_string();
        FixedDecimalI128::<150>::new(v).to_string();
        FixedDecimalI128::<200>::new(v).to_string();
        FixedDecimalI128::<{u8::MAX}>::new(v).to_string();
    }

    #[test]
    fn formats_never_panic_u128(v in any::<u128>()) {
        FixedDecimalU128::<0>::new(v).to_string();
        FixedDecimalU128::<7>::new(v).to_string();
        FixedDecimalU128::<13>::new(v).to_string();
        FixedDecimalU128::<21>::new(v).to_string();
        FixedDecimalU128::<32>::new(v).to_string();
        FixedDecimalU128::<41>::new(v).to_string();
        FixedDecimalU128::<57>::new(v).to_string();
        FixedDecimalU128::<100>::new(v).to_string();
        FixedDecimalU128::<150>::new(v).to_string();
        FixedDecimalU128::<200>::new(v).to_string();
        FixedDecimalU128::<{u8::MAX}>::new(v).to_string();
    }
}

// Parsing

#[test]
fn it_parses_empty_string() {
    assert!(FixedDecimalI128::<3>::from_str("").is_err());
    assert!(FixedDecimalI128::<3>::from_str(" ").is_err());
}

#[test]
fn it_parses_positive_int_string() {
    let a = FixedDecimalI128::<0>::from_str("233").unwrap();
    // assert!(a.is_positive());
    assert_eq!(233, a.mantissa());
    assert_eq!("233", a.to_string());
}

#[test]
fn it_parses_explicitly_positive_int_string() {
    let a = FixedDecimalI128::<0>::from_str("+233").unwrap();
    // assert!(a.is_positive());
    assert_eq!(233, a.mantissa());
    assert_eq!("233", a.to_string());
}

#[test]
fn it_parses_negative_int_string() {
    let a = FixedDecimalI128::<0>::from_str("-233").unwrap();
    // assert!(a.is_negative());
    assert_eq!(-233, a.mantissa());
    assert_eq!("-233", a.to_string());
}

#[test]
fn it_parses_positive_float_string() {
    let a = FixedDecimalI128::<6>::from_str("233.323223").unwrap();
    // assert!(a.is_positive());
    assert_eq!(233323223, a.mantissa());
    assert_eq!("233.323223", a.to_string());
}

#[test]
fn it_parses_negative_float_string() {
    let a = FixedDecimalI128::<5>::from_str("-233.32322").unwrap();
    // assert!(a.is_negative());
    assert_eq!(-23332322, a.mantissa());
    assert_eq!("-233.32322", a.to_string());
}

#[test]
fn it_parses_positive_tiny_float_string() {
    let a = FixedDecimalI128::<6>::from_str(".000001").unwrap();
    // assert!(a.is_positive());
    assert_eq!(1, a.mantissa());
    assert_eq!("0.000001", a.to_string());
}

#[test]
fn it_parses_negative_tiny_float_string() {
    let a = FixedDecimalI128::<6>::from_str("-0.000001").unwrap();
    // assert!(a.is_negative());
    assert_eq!(-1, a.mantissa());
    assert_eq!("-0.000001", a.to_string());
}

#[test]
fn it_parses_big_integer_string() {
    let a = FixedDecimalI128::<0>::from_str("170141183460469231731687303715884105727").unwrap();
    assert_eq!(FixedDecimalI128::<0>::MAX, a);
    assert_eq!(i128::MAX.to_string(), a.to_string());

    let a = FixedDecimalI128::<0>::from_str("-170141183460469231731687303715884105728").unwrap();
    assert_eq!(FixedDecimalI128::<0>::MIN, a);
    assert_eq!(i128::MIN.to_string(), a.to_string());
}

#[test]
fn it_parses_big_float_string() {
    let a = FixedDecimalI128::<37>::from_str("17.0141183460469231731687303715884105727").unwrap();
    assert_eq!("17.0141183460469231731687303715884105727", a.to_string());
}

#[test]
fn it_parses_big_scale() {
    let s = "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001";
    let a = FixedDecimalI128::<{ u8::MAX }>::from_str(s).expect("1");
    assert_eq!(s, a.to_string());

    let s = "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000170141183460469231731687303715884105727";
    let a = FixedDecimalI128::<{ u8::MAX }>::from_str(s).expect("i128::MAX");
    assert_eq!(s, a.to_string());

    let s = "-0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000170141183460469231731687303715884105728";
    let a = FixedDecimalI128::<{ u8::MAX }>::from_str(s).expect("i128::MIN");
    assert_eq!(s, a.to_string());

    let s = "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000340282366920938463463374607431768211455";
    let a = FixedDecimalU128::<{ u8::MAX }>::from_str(s).expect("u128::MAX");
    assert_eq!(s, a.to_string());
}

#[test]
fn it_parses_positive_int_string_with_extra_right_zeros() {
    let a = FixedDecimalI128::<3>::from_str("98.1").unwrap();
    // assert!(a.is_positive());
    assert_eq!(98100, a.mantissa());
    assert_eq!("98.100", a.to_string());
}

#[test]
fn it_doesnt_parses_invalid_digit() {
    assert!(FixedDecimalI128::<2>::from_str("&123.45").is_err());
    assert!(FixedDecimalI128::<2>::from_str("1&23.45").is_err());
    assert!(FixedDecimalI128::<2>::from_str("12&3.45").is_err());
    assert!(FixedDecimalI128::<2>::from_str("123&.45").is_err());
    assert!(FixedDecimalI128::<2>::from_str("123.&45").is_err());
    assert!(FixedDecimalI128::<2>::from_str("123.4&5").is_err());
    assert!(FixedDecimalI128::<2>::from_str("123.45&").is_err());

    assert!(FixedDecimalI128::<3>::from_str("&123.45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("1&23.45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("12&3.45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("123&.45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("123.&45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("123.4&5").is_err());
    assert!(FixedDecimalI128::<3>::from_str("123.45&").is_err());
}

#[test]
fn it_doesnt_parses_invalid_unicode_digit() {
    assert!(FixedDecimalI128::<3>::from_str("ç123.45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("1ç23.45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("12ç3.45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("123ç.45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("123.ç45").is_err());
    assert!(FixedDecimalI128::<3>::from_str("123.4ç5").is_err());
    assert!(FixedDecimalI128::<3>::from_str("123.45ç").is_err());
}

#[test]
fn it_doesnt_parses_huge_decimal_cases() {
    assert!(FixedDecimal::<i16, 3>::from_str("32.767").is_ok());
    assert!(FixedDecimal::<i16, 3>::from_str("32.768").is_err());
    assert!(FixedDecimal::<i16, 0>::from_str("32768").is_err());

    assert!(FixedDecimal::<i16, 3>::from_str("33.7").is_err());
    assert!(FixedDecimal::<i16, 10>::from_str("10").is_err());
}

proptest! {
    #[test]
    fn formats_and_parses_give_same_result_i128(v in any::<i128>()) {
        let d = FixedDecimalI128::<0>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));

        let d = FixedDecimalI128::<27>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));

        let d = FixedDecimalI128::<57>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));

        let d = FixedDecimalI128::<173>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));

        let d = FixedDecimalI128::<{u8::MAX}>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));
    }

    #[test]
    fn formats_and_parses_give_same_result_u128(v in any::<u128>()) {
        let d = FixedDecimalU128::<0>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));

        let d = FixedDecimalU128::<27>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));

        let d = FixedDecimalU128::<57>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));

        let d = FixedDecimalU128::<173>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));

        let d = FixedDecimalU128::<{u8::MAX}>::new(v);
        assert_eq!(d.to_string().parse(), Ok(d));
    }

    #[test]
    fn parses_never_panic_i128(v in r"-?[0-9]{1,200}\.[0-9]{1,200}") {
        let _ = FixedDecimalI128::<0>::from_str(&v);
        let _ = FixedDecimalI128::<7>::from_str(&v);
        let _ = FixedDecimalI128::<13>::from_str(&v);
        let _ = FixedDecimalI128::<21>::from_str(&v);
        let _ = FixedDecimalI128::<32>::from_str(&v);
        let _ = FixedDecimalI128::<41>::from_str(&v);
        let _ = FixedDecimalI128::<57>::from_str(&v);
        let _ = FixedDecimalI128::<100>::from_str(&v);
        let _ = FixedDecimalI128::<150>::from_str(&v);
        let _ = FixedDecimalI128::<200>::from_str(&v);
        let _ = FixedDecimalI128::<{u8::MAX}>::from_str(&v);

        let _ = FixedDecimalU128::<0>::from_str(&v);
        let _ = FixedDecimalU128::<7>::from_str(&v);
        let _ = FixedDecimalU128::<13>::from_str(&v);
        let _ = FixedDecimalU128::<21>::from_str(&v);
        let _ = FixedDecimalU128::<32>::from_str(&v);
        let _ = FixedDecimalU128::<41>::from_str(&v);
        let _ = FixedDecimalU128::<57>::from_str(&v);
        let _ = FixedDecimalU128::<100>::from_str(&v);
        let _ = FixedDecimalU128::<150>::from_str(&v);
        let _ = FixedDecimalU128::<200>::from_str(&v);
        let _ = FixedDecimalU128::<{u8::MAX}>::from_str(&v);
    }

}
// Negation
#[test]
fn it_negates_decimals() {
    let a = FixedDecimalI128::<8>::from_str("11.81512605").unwrap();
    let b = FixedDecimalI128::<8>::from_str("-11.81512605").unwrap();

    assert_eq!(-a, b);
    assert_eq!(-b, a);
}

proptest! {
    #[test]
    fn negates_works_as_internal(v in any::<i128>()) {
        assert_eq!(-FixedDecimalI128::<7>::new(v), FixedDecimalI128::<7>::new(-v));
    }
}

// Addition

#[test]
fn it_can_add_simple() {
    // This is the most basic test for addition, intended largely for micro-optimization.
    let two = FixedDecimalI128::<2>::ONE + FixedDecimalI128::<2>::ONE;
    assert_eq!(two, FixedDecimalI128::<2>::new(200));
}

#[test]
fn it_can_addassign() {
    let mut a = FixedDecimalI128::<2>::from_str("1.01").unwrap();
    let b = FixedDecimalI128::<2>::from_str("0.99").unwrap();
    a += b;
    assert_eq!("2.00", a.to_string());

    a += &b;
    assert_eq!("2.99", a.to_string());

    let mut c = &mut a;
    c += b;
    assert_eq!("3.98", a.to_string());

    let mut c = &mut a;
    c += &b;
    assert_eq!("4.97", a.to_string());
}

#[test]
#[should_panic]
fn it_panics_on_add_overflow() {
    let a = FixedDecimalI8::<2>::new(100);
    let b = FixedDecimalI8::<2>::new(100);

    let _ = a + b;
}

proptest! {
    #[test]
    fn adds_works_as_internal(a in any::<i128>(), b in any::<i128>()) {
        if a.checked_add(b).is_some() {
            assert_eq!(FixedDecimalI128::<7>::new(a) + FixedDecimalI128::<7>::new(b), FixedDecimalI128::<7>::new(a + b))
        }
    }

    #[test]
    fn checked_adds_works_as_internal(a in any::<i128>(), b in any::<i128>()) {
        assert_eq!(FixedDecimalI128::<7>::new(a).checked_add(&FixedDecimalI128::<7>::new(b)), a.checked_add(b).map(FixedDecimalI128::<7>::new))
    }
}

// Sub

#[test]
fn it_can_sub_simple() {
    let zero = FixedDecimalI128::<2>::ONE - FixedDecimalI128::<2>::ONE;
    assert!(zero.is_zero());
    let minus_one = zero - FixedDecimalI128::<2>::ONE;
    assert_eq!(minus_one, FixedDecimalI128::<2>::from_str("-1").unwrap());
}

// Mult

#[test]
fn it_can_mult_simple() {
    let one = FixedDecimalI128::<2>::ONE * 3i128;
    assert_eq!(one, FixedDecimalI128::<2>::from_str("3").unwrap());
}

// Ord
proptest! {
    #[test]
    fn ord_works_as_internal(a in any::<i128>(), b in any::<i128>()) {
        assert_eq!(FixedDecimalI128::<7>::new(a).cmp(&FixedDecimalI128::<7>::new(b)), a.cmp(&b))
    }

    #[test]
    fn partial_ord_works_as_internal(a in any::<i128>(), b in any::<i128>()) {
        assert_eq!(FixedDecimalI128::<7>::new(a).partial_cmp(&FixedDecimalI128::<7>::new(b)), a.partial_cmp(&b))
    }
}

// (Try)From integers
#[test]
fn it_converts_to_f64() {
    assert_eq!(d5!("5").to_f64(), Some(5f64));
    assert_eq!(d5!("-5").to_f64(), Some(-5f64));
    assert_eq!(d5!("0.1").to_f64(), Some(0.1));
    assert_eq!(d5!("0.0").to_f64(), Some(0f64));
    assert_eq!(d5!("-0.0").to_f64(), Some(0f64));
    assert_eq!(d5!("0.00025").to_f64(), Some(0.00025));
    assert_eq!(d5!("100000").to_f64(), Some(1e5));
}

/*
#[test]
fn it_converts_to_f64_try() {
    let tests = &[
        ("5", Some(5f64)),
        ("-5", Some(-5f64)),
        ("0.1", Some(0.1f64)),
        ("0.0", Some(0f64)),
        ("-0.0", Some(0f64)),
        ("0.0000000000025", Some(0.25e-11f64)),
        ("1000000.0000000000025", Some(1e6f64)),
        ("0.000000000000000000000000025", Some(0.25e-25_f64)),
        (
            "2.1234567890123456789012345678",
            Some(2.1234567890123456789012345678_f64),
        ),
        ("21234567890123456789012345678", Some(21234567890123458000000000000_f64)),
        (
            "-21234567890123456789012345678",
            Some(-21234567890123458000000000000_f64),
        ),
        ("1.59283191", Some(1.59283191_f64)),
    ];
    for &(value, expected) in tests {
        let value = Decimal::from_str(value).unwrap().try_into().ok();
        assert_eq!(expected, value);
    }
}
*/

#[test]
fn it_converts_to_i64() {
    assert_eq!(d5!("5").to_i64(), Some(5i64));
    assert_eq!(d5!("-5").to_i64(), Some(-5i64));
    assert_eq!(d5!("5.12345").to_i64(), Some(5i64));
    assert_eq!(d5!("-5.12345").to_i64(), Some(-5i64));
    assert_eq!(d5!("-9223372036854775808").to_i64(), Some(i64::MIN));
    assert_eq!(d5!("9223372036854775807").to_i64(), Some(i64::MAX));
    assert_eq!(d5!("-9223372036854775809").to_i64(), None);
    assert_eq!(d5!("9223372036854775808").to_i64(), None);
}

#[test]
fn it_converts_to_u64() {
    assert_eq!(d5!("5").to_u64(), Some(5u64));
    assert_eq!(d5!("-5").to_u64(), None);
    assert_eq!(d5!("5.12345").to_u64(), Some(5u64));
    assert_eq!(d5!("18446744073709551615").to_u64(), Some(u64::MAX));
    assert_eq!(d5!("18446744073709551616").to_u64(), None);
}

#[test]
fn it_converts_to_i128() {
    assert_eq!(d5!("5").to_i128(), Some(5i128));
    assert_eq!(d5!("-5").to_i128(), Some(-5i128));
    assert_eq!(d5!("5.12345").to_i128(), Some(5i128));
    assert_eq!(d5!("-5.12345").to_i128(), Some(-5i128));
    assert_eq!(
        d5!("-170141183460469231731687303715884").to_i128(),
        Some(-170141183460469231731687303715884)
    );
    assert_eq!(
        d5!("170141183460469231731687303715884").to_i128(),
        Some(170141183460469231731687303715884)
    );
}

#[test]
fn it_converts_to_u128() {
    assert_eq!(d5!("5").to_u128(), Some(5u128));
    assert_eq!(d5!("-5").to_u128(), None);
    assert_eq!(d5!("5.12345").to_u128(), Some(5u128));
    assert_eq!(
        d5!("170141183460469231731687303715884").to_u128(),
        Some(170141183460469231731687303715884)
    );
}

/*
#[test]
fn it_converts_from_i128() {
    let tests: &[(i128, Option<&str>)] = &[
        (5, Some("5")),
        (-5, Some("-5")),
        (0x7FFF_FFFF_FFFF_FFFF, Some("9223372036854775807")),
        (92233720368547758089, Some("92233720368547758089")),
        (0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF, Some("79228162514264337593543950335")),
        (0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, None),
        (i128::MIN, None),
        (i128::MAX, None),
    ];
    for (value, expected) in tests {
        let from_i128 = num_traits::FromPrimitive::from_i128(*value);

        match expected {
            Some(expected_value) => {
                let decimal = Decimal::from_str(expected_value).unwrap();
                assert_eq!(from_i128, Some(decimal));
            }
            None => assert!(from_i128.is_none()),
        }
    }
}

#[test]
fn it_converts_from_u128() {
    let tests: &[(u128, Option<&str>)] = &[
        (5, Some("5")),
        (0xFFFF_FFFF_FFFF_FFFF, Some("18446744073709551615")),
        (0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF, Some("79228162514264337593543950335")),
        (0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, None),
        (u128::MAX, None),
    ];
    for (value, expected) in tests {
        let from_u128 = num_traits::FromPrimitive::from_u128(*value);

        match expected {
            Some(expected_value) => {
                let decimal = Decimal::from_str(expected_value).unwrap();
                assert_eq!(from_u128, Some(decimal));
            }
            None => assert!(from_u128.is_none()),
        }
    }
}

#[test]
fn it_converts_from_str() {
    assert_eq!(Decimal::try_from("1").unwrap(), Decimal::ONE);
    assert_eq!(Decimal::try_from("10").unwrap(), Decimal::TEN);
}

#[test]
fn it_converts_from_f32() {
    use num_traits::FromPrimitive;

    let tests = [
        (0.1_f32, "0.1"),
        (1_f32, "1"),
        (0_f32, "0"),
        (0.12345_f32, "0.12345"),
        (0.1234567800123456789012345678_f32, "0.12345678"),
        (0.12345678901234567890123456789_f32, "0.12345679"),
        (0.00000000000000000000000000001_f32, "0"),
        (5.1_f32, "5.1"),
    ];

    for &(input, expected) in &tests {
        assert_eq!(
            expected,
            Decimal::from_f32(input).unwrap().to_string(),
            "from_f32({input})"
        );
        assert_eq!(
            expected,
            Decimal::try_from(input).unwrap().to_string(),
            "try_from({input})"
        );
    }
}

#[test]
fn it_converts_from_f32_limits() {
    use num_traits::FromPrimitive;

    assert!(Decimal::from_f32(f32::NAN).is_none(), "from_f32(f32::NAN)");
    assert!(Decimal::from_f32(f32::INFINITY).is_none(), "from_f32(f32::INFINITY)");
    assert!(Decimal::try_from(f32::NAN).is_err(), "try_from(f32::NAN)");
    assert!(Decimal::try_from(f32::INFINITY).is_err(), "try_from(f32::INFINITY)");

    // These overflow
    assert!(Decimal::from_f32(f32::MAX).is_none(), "from_f32(f32::MAX)");
    assert!(Decimal::from_f32(f32::MIN).is_none(), "from_f32(f32::MIN)");
    assert!(Decimal::try_from(f32::MAX).is_err(), "try_from(f32::MAX)");
    assert!(Decimal::try_from(f32::MIN).is_err(), "try_from(f32::MIN)");
}

#[test]
fn it_converts_from_f32_retaining_bits() {
    let tests = [
        (0.1_f32, "0.100000001490116119384765625"),
        (2_f32, "2"),
        (4.000_f32, "4"),
        (5.1_f32, "5.099999904632568359375"),
    ];

    for &(input, expected) in &tests {
        assert_eq!(
            expected,
            Decimal::from_f32_retain(input).unwrap().to_string(),
            "from_f32_retain({input})"
        );
    }
}

#[test]
fn it_converts_from_f64() {
    use num_traits::FromPrimitive;

    let tests = [
        (0.1_f64, "0.1"),
        (1_f64, "1"),
        (0_f64, "0"),
        (0.12345_f64, "0.12345"),
        (0.1234567890123456089012345678_f64, "0.1234567890123456"),
        (0.12345678901234567890123456789_f64, "0.1234567890123457"),
        (0.00000000000000000000000000001_f64, "0"),
        (0.6927_f64, "0.6927"),
        (0.00006927_f64, "0.00006927"),
        (0.000000006927_f64, "0.000000006927"),
        (5.1_f64, "5.1"),
    ];

    for &(input, expected) in &tests {
        assert_eq!(
            expected,
            Decimal::from_f64(input).unwrap().to_string(),
            "from_f64({input})"
        );
        assert_eq!(
            expected,
            Decimal::try_from(input).unwrap().to_string(),
            "try_from({input})"
        );
    }
}

#[test]
fn it_converts_from_f64_limits() {
    use num_traits::FromPrimitive;

    assert!(Decimal::from_f64(f64::NAN).is_none(), "from_f64(f64::NAN)");
    assert!(Decimal::from_f64(f64::INFINITY).is_none(), "from_f64(f64::INFINITY)");
    assert!(Decimal::try_from(f64::NAN).is_err(), "try_from(f64::NAN)");
    assert!(Decimal::try_from(f64::INFINITY).is_err(), "try_from(f64::INFINITY)");

    // These overflow
    assert!(Decimal::from_f64(f64::MAX).is_none(), "from_f64(f64::MAX)");
    assert!(Decimal::from_f64(f64::MIN).is_none(), "from_f64(f64::MIN)");
    assert!(Decimal::try_from(f64::MAX).is_err(), "try_from(f64::MIN)");
    assert!(Decimal::try_from(f64::MIN).is_err(), "try_from(f64::MAX)");
}

#[test]
fn it_converts_from_f64_dec_limits() {
    use num_traits::FromPrimitive;

    // Note Decimal MAX is: 79_228_162_514_264_337_593_543_950_335
    let over_max = 79_228_162_514_264_355_185_729_994_752_f64;
    let max_plus_one = 79_228_162_514_264_337_593_543_950_336_f64;
    let under_max = 79_228_162_514_264_328_797_450_928_128_f64;

    assert!(
        Decimal::from_f64(over_max).is_none(),
        "from_f64(79_228_162_514_264_355_185_729_994_752_f64) -> none (too large)"
    );
    assert!(
        Decimal::from_f64(max_plus_one).is_none(),
        "from_f64(79_228_162_514_264_337_593_543_950_336_f64) -> none (too large)"
    );
    assert_eq!(
        "79228162514264328797450928128",
        Decimal::from_f64(under_max).unwrap().to_string(),
        "from_f64(79_228_162_514_264_328_797_450_928_128_f64) -> some (inside limits)"
    );
}

#[test]
fn it_converts_from_f64_retaining_bits() {
    let tests = [
        (0.1_f64, "0.1000000000000000055511151231"),
        (2_f64, "2"),
        (4.000_f64, "4"),
        (5.1_f64, "5.0999999999999996447286321175"),
    ];

    for &(input, expected) in &tests {
        assert_eq!(
            expected,
            Decimal::from_f64_retain(input).unwrap().to_string(),
            "from_f64_retain({input})"
        );
    }
}

#[test]
fn it_converts_to_integers() {
    assert_eq!(i64::try_from(Decimal::ONE), Ok(1));
    assert_eq!(i64::try_from(Decimal::MAX), Err(Error::ConversionTo("i64".to_string())));
    assert_eq!(u128::try_from(Decimal::ONE_HUNDRED), Ok(100));
}
*/

// Serde
#[cfg(feature = "serde")]
mod _serde {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_can_deserilize_str() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct A {
            value: FixedDecimalI128<2>,
        }

        assert_eq!(
            A {
                value: FixedDecimalI128::<2>::from_str("123.45").unwrap()
            },
            serde_json::from_value(json!({
                "value": "123.45"
            }))
            .unwrap()
        );

        assert_eq!(
            A {
                value: FixedDecimalI128::<2>::from_str("123.40").unwrap()
            },
            serde_json::from_value(json!({
                "value": "123.4"
            }))
            .unwrap()
        );
    }

    #[test]
    fn it_cant_if_is_invalid() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct A {
            value: FixedDecimalI128<2>,
        }

        assert!(serde_json::from_value::<A>(json!({
            "value": "123.459"
        }))
        .is_err());

        assert!(serde_json::from_value::<A>(json!({
            "value": "a lot"
        }))
        .is_err());
    }
}
