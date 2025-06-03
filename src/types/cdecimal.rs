use super::{Decimal, NewDeciVal, Pow, Prec, Rt, SpecialValuesDeci, WithValDeci};
use crate::macros::impls::{
    dec_c_impl, impl_c_ops, impl_cneg, impl_new_val_cdeci, impl_self_c_ops,
};
use fastnum::I512;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Div, Mul};
#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CDecimal(pub Decimal, pub Decimal);

impl Prec for CDecimal {
    fn prec(&self) -> u32 {
        self.0.prec()
    }
    fn set_prec(&mut self, new_prec: u32) {
        self.0.set_prec(new_prec);
        self.1.set_prec(new_prec);
    }
}

impl From<Decimal> for CDecimal {
    fn from(value: Decimal) -> Self {
        Self(value, Decimal::new(value.prec()))
    }
}

impl From<(Decimal, Decimal)> for CDecimal {
    fn from((a, b): (Decimal, Decimal)) -> Self {
        Self(a, b)
    }
}

impl Display for CDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}

impl Mul<I512> for CDecimal {
    type Output = Self;
    fn mul(mut self, rhs: I512) -> Self::Output {
        self.0 = self.0 * rhs;
        self.1 = self.1 * rhs;
        self
    }
}
impl Div<I512> for CDecimal {
    type Output = Self;
    fn div(mut self, rhs: I512) -> Self::Output {
        self.0 = self.0 / rhs;
        self.1 = self.1 / rhs;
        self
    }
}

impl_c_ops!(CDecimal, CDecimal, Decimal, |x| x);
impl_new_val_cdeci!(CDecimal);
dec_c_impl!(CDecimal, Decimal, Decimal::with_val);
impl_cneg!(CDecimal, CDecimal);
impl_self_c_ops!(CDecimal);
