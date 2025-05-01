use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use super::{ Parse, Prec, Special, NewDeciVal, SinhCosh }; 
use crate::macros::impls::{ impl_new_val_deci, dec_impl, impl_partial_ord, impl_neg, impl_self_ops };
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Decimal {
    D512(fastnum::D512),
    D256(fastnum::D256),
}

impl Prec for Decimal {
    fn prec(&self) -> u32 {
        match self {
            Decimal::D512(_) => 512,
            Decimal::D256(_) => 256,
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
}

impl Special for Decimal {
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

impl_new_val_deci!(Decimal);
impl_partial_ord!(Decimal, D512, D256);
dec_impl!(Decimal, D512, D256);
impl_neg!(Decimal, D512, D256);
impl_self_ops!(Decimal, D512, D256);