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
        value.mantissa().uabs().ilog10() + 1
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

    if append_sign && value.0.is_negative() {
        rep.push('-');
    }

    let push_digit = |rep: &mut ArrayString<MAX_STR_BUFFER_SIZE>, i| {
        let digit = if i >= total_len {
            0
        } else {
            ((value.mantissa().uabs() / 10u128.pow(i)) % 10) as u8
        };
        rep.push(char::from(b'0' + digit));
    };

    if total_len <= SCALE.into() {
        rep.push('0');
    } else {
        for i in (SCALE.into()..total_len).rev() {
            push_digit(&mut rep, i);
        }
    }
    if prec != 0 {
        rep.push('.');
        for i in (0..SCALE).rev().take(prec) {
            push_digit(&mut rep, i.into());
        }
        for _ in 0..(prec.saturating_sub(SCALE.into())) {
            rep.push('0');
        }
    }

    (rep, prec_rem)
}

// TODO: add inner types
#[derive(Clone, Debug, PartialEq)]
pub struct ParseError;

pub(crate) fn parse_str_radix_10_exact<T: Num, const SCALE: u8>(
    str: &str,
) -> Result<FixedDecimal<T, SCALE>, ParseError> {
    let is_negative = str.starts_with('-');
    let (int, frac) = str
        .split_once('.')
        .map_or((str, None), |(int, frac)| (int, Some(frac)));

    if frac.is_some_and(|f| f.trim_start_matches('0').len() > SCALE.into()) {
        return Err(ParseError);
    }

    let int = if int.is_empty() && !str.is_empty() {
        0u8.into()
    } else {
        str::parse::<T>(int).map_err(|_| ParseError)?
    };
    let frac = frac.map_or(Ok(T::ZERO), |v| str::parse::<T>(v).map_err(|_| ParseError))?;
    let high = if !int.is_zero() {
        let mut shift: T = 1u8.into();
        for _ in 0..SCALE {
            shift = shift
                .checked_mul(&(10u8.into()))
                .ok_or_else(|| ParseError)?;
        }
        int.checked_mul(&shift).ok_or_else(|| ParseError)?
    } else {
        T::ZERO
    };

    // TODO: improve gambiarra
    let frac = if is_negative { frac.force_neg() } else { frac };

    Ok(FixedDecimal::<T, SCALE>::new(high + frac))
}
