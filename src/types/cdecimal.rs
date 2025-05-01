use serde::{ Deserialize, Serialize };
use super::{ Decimal, Prec, NewDeciVal, WithValDeci, Pow, Rt, Special };
use crate::macros::impls::{impl_c_ops, impl_new_val_cdeci, dec_c_impl, impl_cneg, impl_self_c_ops };
use std::fmt::{ Display, Formatter };


#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDecimal(pub Decimal, pub Decimal);

impl Prec for CDecimal {
    fn prec(&self) -> u32 {
        self.0.prec()
    }
}

impl From<Decimal> for CDecimal {
    fn from(value: Decimal) -> Self {
        Self(value, Decimal::new(value.prec()))
    }
}

impl Display for CDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}

impl_c_ops!(CDecimal, CDecimal, Decimal, |x| x);
impl_new_val_cdeci!(CDecimal);
dec_c_impl!(CDecimal, Decimal, Decimal::with_val);
impl_cneg!(CDecimal);
impl_self_c_ops!(CDecimal);