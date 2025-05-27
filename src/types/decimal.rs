use super::{NewDeciVal, Parse, Prec, SinhCosh, SpecialValuesDeci};
use crate::macros::impls::{
    dec_impl, impl_neg, impl_new_val_deci, impl_partial_ord, impl_rem, impl_self_ops,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Div, Mul};
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Decimal {
    D512(fastnum::D512),
    D256(fastnum::D256),
}

impl Prec for Decimal {
    fn prec(&self) -> u32 {
        match self {
            Self::D512(_) => 512,
            Self::D256(_) => 256,
        }
    }
    fn set_prec(&mut self, _new_prec: u32) {
        match self {
            Self::D512(_) => todo!(),
            Self::D256(_) => todo!(),
        }
    }
}

impl Decimal {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::D512(a) => a.is_zero(),
            Self::D256(a) => a.is_zero(),
        }
    }
    pub fn to_integer(self) -> fastnum::I512 {
        match self {
            Self::D512(_) => todo!(),
            Self::D256(_) => todo!(),
        }
    }
    pub fn to_f64(&self) -> f64 {
        match self {
            Self::D512(_) => todo!(),
            Self::D256(_) => todo!(),
        }
    }
    pub fn to_string_radix(self, _base: i32, _num_digits: Option<usize>) -> String {
        match self {
            Self::D512(a) => a.to_string(),
            Self::D256(a) => a.to_string(),
        }
    }
}

impl Mul<fastnum::I512> for Decimal {
    type Output = Self;
    fn mul(self, rhs: fastnum::I512) -> Self::Output {
        match self {
            Decimal::D512(a) => Decimal::D512(
                a * fastnum::D512::from_str(&rhs.to_string(), fastnum::decimal::Context::default())
                    .unwrap_or_default(),
            ),
            Decimal::D256(a) => Decimal::D256(
                a * fastnum::D256::from_str(&rhs.to_string(), fastnum::decimal::Context::default())
                    .unwrap_or_default(),
            ),
        }
    }
}

impl Div<fastnum::I512> for Decimal {
    type Output = Self;
    fn div(self, rhs: fastnum::I512) -> Self::Output {
        match self {
            Decimal::D512(a) => Decimal::D512(
                a / fastnum::D512::from_str(&rhs.to_string(), fastnum::decimal::Context::default())
                    .unwrap_or_default(),
            ),
            Decimal::D256(a) => Decimal::D256(
                a / fastnum::D256::from_str(&rhs.to_string(), fastnum::decimal::Context::default())
                    .unwrap_or_default(),
            ),
        }
    }
}

impl Parse<&str> for Decimal {
    fn parse(prec: u32, s: &str) -> Option<Self> {
        match prec.next_power_of_two() {
            512 => fastnum::D512::from_str(s, fastnum::decimal::Context::default())
                .ok()
                .map(Self::D512),
            256 => fastnum::D256::from_str(s, fastnum::decimal::Context::default())
                .ok()
                .map(Self::D256),
            _ => unreachable!(),
        }
    }
    fn parse_radix(prec: u32, s: &str, _base: i32) -> Option<Self> {
        match prec.next_power_of_two() {
            512 => fastnum::D512::from_str(s, fastnum::decimal::Context::default())
                .ok()
                .map(Self::D512),
            256 => fastnum::D256::from_str(s, fastnum::decimal::Context::default())
                .ok()
                .map(Self::D256),
            _ => unreachable!(),
        }
    }
}

impl SpecialValuesDeci for Decimal {
    fn pi(prec: u32) -> Self {
        match prec.next_power_of_two() {
            512 => Self::D512(fastnum::D512::PI),
            256 => Self::D256(fastnum::D256::PI),
            _ => unreachable!(),
        }
    }
    fn nan(prec: u32) -> Self {
        match prec.next_power_of_two() {
            512 => Self::D512(fastnum::D512::NAN),
            256 => Self::D256(fastnum::D256::NAN),
            _ => unreachable!(),
        }
    }
    fn inf(prec: u32) -> Self {
        match prec.next_power_of_two() {
            512 => Self::D512(fastnum::D512::INFINITY),
            256 => Self::D256(fastnum::D256::INFINITY),
            _ => unreachable!(),
        }
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::D512(a) => a.fmt(f),
            Self::D256(a) => a.fmt(f),
        }
    }
}
impl PartialEq<f64> for Decimal {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Decimal::D512(a) => &a.to_f64() == other,
            Decimal::D256(b) => &b.to_f64() == other,
        }
    }
}
impl PartialEq<i32> for Decimal {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Decimal::D512(a) => a.to_f64() == *other as f64,
            Decimal::D256(b) => b.to_f64() == *other as f64,
        }
    }
}
impl_rem!(Decimal, (D512, |x| x), (D256, |x| x));
impl_new_val_deci!(Decimal);
impl_partial_ord!(
    Decimal,
    (D512, |x: &fastnum::D512| x.to_f64()),
    (D256, |x: &fastnum::D256| x.to_f64())
);
dec_impl!(Decimal, D512, D256);
impl_neg!(Decimal, D512, D256);
impl_self_ops!(
    Decimal,
    (D512, |x: &fastnum::D512| *x),
    (D256, |x: &fastnum::D256| *x)
);
