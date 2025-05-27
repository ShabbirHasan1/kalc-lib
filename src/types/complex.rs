use super::{
    CDecimal, CF32, CF64, Decimal, Float, NewVal, ParseU, Prec, Special, SpecialValues,
    SpecialValuesDeci, Type, WithVal, WithValDeci,
};
use crate::macros::impls::{
    impl_complex, impl_from_complex_tuple, impl_neg, impl_new_val, impl_self_ops,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};
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
    fn set_prec(&mut self, new_prec: u32) {
        match self {
            Self::Rug(a) => a.set_prec(new_prec),
            Self::Fastnum(a) => a.set_prec(new_prec),
            Self::F64(_) => {}
            Self::F32(_) => {}
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
    fn parse_radix(t: Type, prec: u32, s: &str, base: i32) -> Option<Self> {
        match t {
            Type::Rug => Float::parse_radix(t, prec, s, base).map(|a| a.into()),
            Type::Fastnum => Float::parse_radix(t, prec, s, base).map(|a| a.into()),
            Type::F64 => Float::parse_radix(t, prec, s, base).map(|a| a.into()),
            Type::F32 => Float::parse_radix(t, prec, s, base).map(|a| a.into()),
        }
    }
}

impl SpecialValues for Complex {
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

impl From<(Float, Float)> for Complex {
    fn from(value: (Float, Float)) -> Self {
        match value {
            (Float::Rug(a), Float::Rug(b)) => Complex::Rug((a, b).into()),
            (Float::Fastnum(a), Float::Fastnum(b)) => Complex::Fastnum((a, b).into()),
            (Float::F64(a), Float::F64(b)) => Complex::F64((a, b).into()),
            (Float::F32(a), Float::F32(b)) => Complex::F32((a, b).into()),
            _ => unreachable!(),
        }
    }
}

impl Add<Float> for Complex {
    type Output = Self;
    fn add(self, rhs: Float) -> Self::Output {
        self + Complex::from(rhs)
    }
}
impl Add<Complex> for Float {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        Complex::from(self) + rhs
    }
}
impl Mul<Float> for Complex {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output {
        self * Complex::from(rhs)
    }
}
impl Mul<Complex> for Float {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        Complex::from(self) * rhs
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
impl Complex {
    pub fn real(&self) -> Float {
        match self {
            Self::Rug(a) => Float::Rug(a.real().clone()),
            Self::Fastnum(a) => Float::Fastnum(a.0),
            Self::F64(a) => Float::F64(a.0),
            Self::F32(a) => Float::F32(a.0),
        }
    }
    pub fn imag(&self) -> Float {
        match self {
            Self::Rug(a) => Float::Rug(a.imag().clone()),
            Self::Fastnum(a) => Float::Fastnum(a.1),
            Self::F64(a) => Float::F64(a.1),
            Self::F32(a) => Float::F32(a.1),
        }
    }
    pub fn real_imag(self) -> (Float, Float) {
        match self {
            Self::Rug(a) => {
                let (a, b) = a.into_real_imag();
                (Float::Rug(a), Float::Rug(b))
            }
            Self::Fastnum(a) => (Float::Fastnum(a.0), Float::Fastnum(a.1)),
            Self::F64(a) => (Float::F64(a.0), Float::F64(a.1)),
            Self::F32(a) => (Float::F32(a.0), Float::F32(a.1)),
        }
    }
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Rug(a) => a.is_zero(),
            Self::Fastnum(a) => a.0.is_zero() && a.1.is_zero(),
            Self::F64(a) => a.0 == 0.0 && a.1 == 0.0,
            Self::F32(a) => a.0 == 0.0 && a.1 == 0.0,
        }
    }
    pub fn ftype(&self) -> Type {
        match self {
            Self::Rug(_) => Type::Rug,
            Self::Fastnum(_) => Type::Fastnum,
            Self::F64(_) => Type::F64,
            Self::F32(_) => Type::F32,
        }
    }
    pub fn sin_cos(self) -> (Self, Self) {
        match self {
            Self::Rug(a) => {
                let p = a.prec();
                let (s, c) = a.sin_cos(rug::Complex::new(p));
                (Self::Rug(s), Self::Rug(c))
            }
            Self::Fastnum(a) => {
                let (s, c) = a.sin_cos();
                (Self::Fastnum(s), Self::Fastnum(c))
            }
            Self::F64(a) => {
                let (s, c) = a.sin_cos();
                (Self::F64(s), Self::F64(c))
            }
            Self::F32(a) => {
                let (s, c) = a.sin_cos();
                (Self::F32(s), Self::F32(c))
            }
        }
    }
}

impl PartialEq<f64> for Complex {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Self::Rug(a) => a == other,
            Self::Fastnum(a) => a.0 == *other,
            Self::F64(a) => a.0 == *other,
            Self::F32(a) => a.0 == *other as f32,
        }
    }
}
impl PartialEq<i32> for Complex {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Self::Rug(a) => a == other,
            Self::Fastnum(a) => a.0 == *other,
            Self::F64(a) => a.0 == *other as f64,
            Self::F32(a) => a.0 == *other as f32,
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

impl_from_complex_tuple!(f64, i32);
impl_from_complex_tuple!(i32, f64);
impl_from_complex_tuple!(i32, Special);
impl_from_complex_tuple!(Special, Special);
impl_from_complex_tuple!(Special, i32);
impl_complex!(Complex, Rug, Fastnum, F64, F32);
impl_neg!(Complex, Rug, Fastnum, F64, F32);
impl_self_ops!(Complex, Rug, Fastnum, F64, F32);
