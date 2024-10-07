use std::str::FromStr;

use crate::{
    constants::MAX_STR_BUFFER_SIZE,
    ext_num_traits::{self, ILog10, Ten},
    FixedDecimal,
};

use arrayvec::ArrayString;
use num_traits::Pow;

// impl that doesn't allocate for serialization purposes.
pub(crate) fn to_str_internal<T, const SCALE: u8>(
    value: &FixedDecimal<T, SCALE>,
    append_sign: bool,
    precision: Option<usize>,
) -> (ArrayString<MAX_STR_BUFFER_SIZE>, Option<usize>)
where
    T: ext_num_traits::UAbs + num_traits::Zero + ext_num_traits::Sign + Copy,
    <T as ext_num_traits::ExtSigned>::Unsigned: ext_num_traits::ILog10
        + ext_num_traits::Ten
        + TryInto<u8>
        + std::fmt::Debug
        + Copy
        + num_traits::Pow<u8, Output = <T as ext_num_traits::ExtSigned>::Unsigned>,
    <<T as ext_num_traits::ExtSigned>::Unsigned as TryInto<u8>>::Error: std::fmt::Debug,
{
    let total_len: u8 = if value.0.is_zero() {
        1
    } else {
        (value.0.uabs().ilog10() + 1)
            .try_into()
            .expect("rust add u2048 and it explodes")
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

    if append_sign && !value.0.is_positive() {
        rep.push('-');
    }

    let push_digit = |rep: &mut ArrayString<MAX_STR_BUFFER_SIZE>, i: u8| {
        let digit: u8 = if i >= total_len {
            0
        } else {
            let ten = T::Unsigned::ten();
            ((value.0.uabs() / ten.pow(i)) % ten)
                .try_into()
                .expect("previous mod 10 make it safe")
        };
        rep.push(char::from(b'0' + digit));
    };

    if total_len <= SCALE {
        rep.push('0');
    } else {
        for i in (SCALE..total_len).rev() {
            push_digit(&mut rep, i);
        }
    }
    if prec != 0 {
        rep.push('.');
        for i in (0..SCALE).rev().take(prec) {
            push_digit(&mut rep, i);
        }
        for _ in 0..(prec.saturating_sub(SCALE.into())) {
            rep.push('0');
        }
    }

    (rep, prec_rem)
}

// TODO: add inner types
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ParseError {
    Empty,
    InvalidDigit,
    PosOverflow,
    NegOverflow,
    Underflow,
}

pub(crate) fn parse_str_radix_10_exact<T, const SCALE: u8>(
    str: &str,
) -> Result<FixedDecimal<T, SCALE>, ParseError>
where
    T: num_traits::ConstZero
        + num_traits::ConstOne
        + FromStr
        + From<u8>
        + ext_num_traits::ConstTen
        + num_traits::CheckedMul
        + num_traits::CheckedAdd
        + ext_num_traits::NegateIfSigned
        + ext_num_traits::IsSigned
        + Copy
        + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn digit_to_int(c: &char) -> Result<u8, ParseError> {
        match c {
            '0' => Ok(0u8),
            '1' => Ok(1),
            '2' => Ok(2),
            '3' => Ok(3),
            '4' => Ok(4),
            '5' => Ok(5),
            '6' => Ok(6),
            '7' => Ok(7),
            '8' => Ok(8),
            '9' => Ok(9),
            _ => Err(ParseError::InvalidDigit),
        }
    }

    let mut digits = str.chars().peekable();
    let mut acc = T::ZERO;
    let is_negative = match digits.peek() {
        Some('-') => {
            digits.next();
            if T::IS_SIGNED {
                Ok(true)
            } else {
                Err(ParseError::InvalidDigit)
            }
        }
        Some('+') => {
            digits.next();
            Ok(false)
        }
        Some('.') => Ok(false),
        Some(c) if c.is_digit(10) => {
            acc = acc
                .checked_add(&digit_to_int(c)?.into())
                .ok_or(ParseError::PosOverflow)?;
            digits.next();
            Ok(false)
        }
        None => Err(ParseError::Empty),
        _ => Err(ParseError::InvalidDigit),
    }?;
    let pack = |v: T| Ok(v).map(FixedDecimal::<T, SCALE>::new);
    let overflow_err = || {
        if is_negative {
            ParseError::NegOverflow
        } else {
            ParseError::PosOverflow
        }
    };

    let sign_carry = if is_negative {
        T::ONE.negate_if_signed()
    } else {
        T::ONE
    };
    loop {
        match digits.next() {
            Some('.') => break,
            None => {
                for _ in 0..SCALE {
                    acc = acc.checked_mul(&T::TEN).ok_or_else(overflow_err)?;
                }
                return pack(acc);
            }
            Some(c) => {
                acc = acc.checked_mul(&T::TEN).ok_or_else(overflow_err)?;
                acc = acc
                    .checked_add(&(sign_carry * digit_to_int(&c)?.into()))
                    .ok_or_else(overflow_err)?;
            }
        }
    }
    for _ in 0..SCALE {
        match digits.next() {
            None => return pack(acc),
            Some(c) => {
                acc = acc.checked_mul(&T::TEN).ok_or_else(overflow_err)?;
                acc = acc
                    .checked_add(&(sign_carry * digit_to_int(&c)?.into()))
                    .ok_or_else(overflow_err)?;
            }
        }
    }

    match digits.next() {
        None => pack(acc),
        Some(c) => {
            if digits.chain(std::iter::once(c)).all(|d| d.is_digit(10)) {
                Err(ParseError::Underflow)
            } else {
                Err(ParseError::InvalidDigit)
            }
        }
    }
}
