use super::{
    CDecimal, CF32, CF64, Decimal, Float, NewVal, ParseU, Prec, Special, SpecialU, Type,
    WithValDeci,
};
use crate::macros::impls::{impl_neg, impl_new_val, impl_self_ops};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Complex {
    Rug(rug::Complex),
    Fastnum(CDecimal),
    F64(CF64),
    F32(CF32),
}

impl Prec for Complex {
    fn prec(&self) -> u32 {
        match self {
            Self::Rug(a) => a.prec().0,
            Self::Fastnum(a) => a.prec(),
            Self::F64(a) => a.prec(),
            Self::F32(a) => a.prec(),
        }
    }
}

impl ParseU<&str> for Complex {
    fn parse(t: Type, prec: u32, s: &str) -> Option<Self> {
        match t {
            Type::Rug => Float::parse(t, prec, s).map(|a| a.into()),
            Type::Fastnum => Float::parse(t, prec, s).map(|a| a.into()),
            Type::F64 => Float::parse(t, prec, s).map(|a| a.into()),
            Type::F32 => Float::parse(t, prec, s).map(|a| a.into()),
        }
    }
}

impl SpecialU for Complex {
    fn pi(t: Type, prec: u32) -> Self {
        match t {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, rug::float::Constant::Pi)),
            Type::Fastnum => Self::Fastnum(Decimal::pi(prec).into()),
            Type::F64 => Self::F64(f64::pi(prec).into()),
            Type::F32 => Self::F32(f32::pi(prec).into()),
        }
    }
    fn nan(t: Type, prec: u32) -> Self {
        match t {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, rug::float::Special::Nan)),
            Type::Fastnum => Self::Fastnum(Decimal::nan(prec).into()),
            Type::F64 => Self::F64(f64::nan(prec).into()),
            Type::F32 => Self::F32(f32::nan(prec).into()),
        }
    }
    fn inf(t: Type, prec: u32) -> Self {
        match t {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, rug::float::Special::Infinity)),
            Type::Fastnum => Self::Fastnum(Decimal::inf(prec).into()),
            Type::F64 => Self::F64(f64::inf(prec).into()),
            Type::F32 => Self::F32(f32::inf(prec).into()),
        }
    }
}

impl From<Float> for Complex {
    fn from(value: Float) -> Self {
        match value {
            Float::Rug(a) => Complex::Rug(a.into()),
            Float::Fastnum(a) => Complex::Fastnum(a.into()),
            Float::F64(a) => Complex::F64(a.into()),
            Float::F32(a) => Complex::F32(a.into()),
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rug(a) => a.fmt(f),
            Self::Fastnum(a) => a.fmt(f),
            Self::F32(a) => a.fmt(f),
            Self::F64(a) => a.fmt(f),
        }
    }
}

impl_new_val!(
    Complex,
    (Rug, rug::Complex::with_val),
    (Fastnum, CDecimal::with_val),
    (F64, |_, x| CF64(x, 0.0)),
    (F32, |_, x| CF32(x as f32, 0.0))
);

impl_neg!(Complex, Rug, Fastnum, F64, F32);
impl_self_ops!(Complex, Rug, Fastnum, F64, F32);
