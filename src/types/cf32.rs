use serde::{Deserialize, Serialize};
use super::{ Prec, Pow, Rt, SinhCosh, Special };
use crate::macros::impls::{ impl_cneg, dec_c_impl, impl_self_c_ops };
use std::fmt::{ Display, Formatter };

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct CF32(pub f32, pub f32);

impl Prec for CF32 {
    fn prec(&self) -> u32 {
        32
    }
}

impl From<f32> for CF32 {
    fn from(value: f32) -> Self {
        Self(value, 0.0)
    }
}

impl Display for CF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}

impl_cneg!(CF32);
dec_c_impl!(CF32, f32, |_, x| x as f32);
impl_self_c_ops!(CF32);