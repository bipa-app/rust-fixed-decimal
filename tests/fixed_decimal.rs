// Most of test case are based on rust_decimal test cases
use num_traits::{CheckedAdd, ConstOne, Signed};
use proptest::prelude::*;
use std::str::FromStr;

use rust_fixed_decimal::{FixedDecimal, FixedDecimalI128, FixedDecimalU128};

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
    assert!(a.is_positive());
    assert_eq!(233, a.mantissa());
    assert_eq!("233", a.to_string());
}

#[test]
fn it_parses_negative_int_string() {
    let a = FixedDecimalI128::<0>::from_str("-233").unwrap();
    assert!(a.is_negative());
    assert_eq!(-233, a.mantissa());
    assert_eq!("-233", a.to_string());
}

#[test]
fn it_parses_positive_float_string() {
    let a = FixedDecimalI128::<6>::from_str("233.323223").unwrap();
    assert!(a.is_positive());
    assert_eq!(233323223, a.mantissa());
    assert_eq!("233.323223", a.to_string());
}

#[test]
fn it_parses_negative_float_string() {
    let a = FixedDecimalI128::<5>::from_str("-233.32322").unwrap();
    assert!(a.is_negative());
    assert_eq!(-23332322, a.mantissa());
    assert_eq!("-233.32322", a.to_string());
}

#[test]
fn it_parses_positive_tiny_float_string() {
    let a = FixedDecimalI128::<6>::from_str(".000001").unwrap();
    assert!(a.is_positive());
    assert_eq!(1, a.mantissa());
    assert_eq!("0.000001", a.to_string());
}

#[test]
fn it_parses_negative_tiny_float_string() {
    let a = FixedDecimalI128::<6>::from_str("-0.000001").unwrap();
    assert!(a.is_negative());
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

    // TODO:
    // let s = "-0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000170141183460469231731687303715884105728";
    // let a = FixedDecimalI128::<{ u8::MAX }>::from_str(s).expect("i128::MIN");
    // assert_eq!(s, a.to_string());

    let s = "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000340282366920938463463374607431768211455";
    let a = FixedDecimalU128::<{ u8::MAX }>::from_str(s).expect("u128::MAX");
    assert_eq!(s, a.to_string());
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
