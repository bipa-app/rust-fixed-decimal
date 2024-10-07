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

    if append_sign && value.0.is_negative() {
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
pub struct ParseError;

pub(crate) fn parse_str_radix_10_exact<T, const SCALE: u8>(
    str: &str,
) -> Result<FixedDecimal<T, SCALE>, ParseError> {
    todo!()
    // let is_negative = str.starts_with('-');
    // let (int, frac) = str
    //     .split_once('.')
    //     .map_or((str, None), |(int, frac)| (int, Some(frac)));

    // if frac.is_some_and(|f| f.trim_start_matches('0').len() > SCALE.into()) {
    //     return Err(ParseError);
    // }

    // let int = if int.is_empty() && !str.is_empty() {
    //     0u8.into()
    // } else {
    //     str::parse::<T>(int).map_err(|_| ParseError)?
    // };
    // let frac = frac.map_or(Ok(T::ZERO), |v| str::parse::<T>(v).map_err(|_| ParseError))?;
    // let high = if !int.is_zero() {
    //     let mut shift: T = 1u8.into();
    //     for _ in 0..SCALE {
    //         shift = shift
    //             .checked_mul(&(10u8.into()))
    //             .ok_or_else(|| ParseError)?;
    //     }
    //     int.checked_mul(&shift).ok_or_else(|| ParseError)?
    // } else {
    //     T::ZERO
    // };

    // // TODO: improve gambiarra
    // let frac = if is_negative { frac.force_neg() } else { frac };

    // Ok(FixedDecimal::<T, SCALE>::new(high + frac))
}
