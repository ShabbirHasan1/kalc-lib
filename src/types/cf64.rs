use serde::{Deserialize, Serialize};
use crate::macros::impls::{ dec_c_impl, impl_cneg, impl_self_c_ops };
use super::{ Prec, Pow, Rt, SinhCosh, Special };
use std::fmt::{ Display, Formatter };

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct CF64(pub f64, pub f64);


impl Prec for CF64 {
    fn prec(&self) -> u32 {
        64
    }
}

impl From<f64> for CF64 {
    fn from(value: f64) -> Self {
        Self(value, 0.0)
    }
}

impl Display for CF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}

dec_c_impl!(CF64, f64, |_, x| x as f64);
impl_cneg!(CF64);
impl_self_c_ops!(CF64);