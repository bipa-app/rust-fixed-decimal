// Most of test case are based on rust_decimal test cases
use proptest::prelude::*;
use std::str::FromStr;

use rust_fixed_decimal::{FixedDecimalI128, FixedDecimalU128};

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
    #[ignore]
    fn formats_never_panic_i128(v in any::<i128>()) {
        let _ = format!("{}", FixedDecimalI128::<0>::new(v));
        let _ = format!("{}", FixedDecimalI128::<7>::new(v));
        let _ = format!("{}", FixedDecimalI128::<13>::new(v));
        let _ = format!("{}", FixedDecimalI128::<21>::new(v));
        let _ = format!("{}", FixedDecimalI128::<32>::new(v));
        let _ = format!("{}", FixedDecimalI128::<41>::new(v));
        let _ = format!("{}", FixedDecimalI128::<57>::new(v));
        let _ = format!("{}", FixedDecimalI128::<100>::new(v));
        let _ = format!("{}", FixedDecimalI128::<150>::new(v));
        let _ = format!("{}", FixedDecimalI128::<200>::new(v));
        let _ = format!("{}", FixedDecimalI128::<{u8::MAX}>::new(v));
    }

    #[test]
    #[ignore]
    fn formats_never_panic_u128(v in any::<u128>()) {
        let _ = format!("{}", FixedDecimalU128::<0>::new(v));
        let _ = format!("{}", FixedDecimalU128::<7>::new(v));
        let _ = format!("{}", FixedDecimalU128::<13>::new(v));
        let _ = format!("{}", FixedDecimalU128::<21>::new(v));
        let _ = format!("{}", FixedDecimalU128::<32>::new(v));
        let _ = format!("{}", FixedDecimalU128::<41>::new(v));
        let _ = format!("{}", FixedDecimalU128::<57>::new(v));
        let _ = format!("{}", FixedDecimalU128::<100>::new(v));
        let _ = format!("{}", FixedDecimalU128::<150>::new(v));
        let _ = format!("{}", FixedDecimalU128::<200>::new(v));
        let _ = format!("{}", FixedDecimalU128::<{u8::MAX}>::new(v));
    }
}
