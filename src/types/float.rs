use super::{
    Decimal, Integer, NewVal, Parse, ParseU, Prec, SinhCosh, Special, SpecialValues,
    SpecialValuesDeci, Type, WithVal, WithValDeci,
};
use crate::macros::impls::{
    float_impl, impl_neg, impl_new_val, impl_partial_ord, impl_rem, impl_self_ops,
};
use rug::ops::CompleteRound;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Div, Mul};
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::{Display, Formatter},
};
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Float {
    Rug(rug::Float),
    Fastnum(Decimal),
    F64(f64),
    F32(f32),
}

impl Prec for Float {
    fn prec(&self) -> u32 {
        match self {
            Self::Rug(a) => a.prec(),
            Self::Fastnum(a) => a.prec(),
            Self::F64(_) => 64,
            Self::F32(_) => 32,
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
impl Float {
    pub fn cmp0(&self) -> Option<Ordering> {
        self.partial_cmp(&Float::new(self.ftype(), self.prec()))
    }
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Rug(a) => a.is_zero(),
            Self::Fastnum(a) => a.is_zero(),
            Self::F64(a) => a == &0.0,
            Self::F32(a) => a == &0.0,
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
    pub fn to_f64(&self) -> f64 {
        match self {
            Float::Rug(a) => a.to_f64(),
            Float::Fastnum(a) => a.to_f64(),
            Float::F64(a) => *a,
            Float::F32(a) => *a as f64,
        }
    }
    pub fn to_string_radix(self, base: i32, num_digits: Option<usize>) -> String {
        match self {
            Self::Rug(a) => a.to_string_radix(base, num_digits),
            Self::Fastnum(a) => a.to_string_radix(base, num_digits),
            Self::F64(a) => a.to_string(),
            Self::F32(a) => a.to_string(),
        }
    }
}
impl ParseU<&str> for Float {
    fn parse(t: Type, prec: u32, s: &str) -> Option<Self> {
        match t {
            Type::Rug => rug::Float::parse(s)
                .ok()
                .map(|a| Float::Rug(a.complete(prec))),
            Type::Fastnum => Decimal::parse(prec, s).map(Float::Fastnum),
            Type::F64 => s.parse().ok().map(Float::F64),
            Type::F32 => s.parse().ok().map(Float::F32),
        }
    }
    fn parse_radix(t: Type, prec: u32, s: &str, base: i32) -> Option<Self> {
        match t {
            Type::Rug => rug::Float::parse_radix(s, base)
                .ok()
                .map(|a| Float::Rug(a.complete(prec))),
            Type::Fastnum => Decimal::parse_radix(prec, s, base).map(Float::Fastnum),
            Type::F64 => s.parse().ok().map(Float::F64),
            Type::F32 => s.parse().ok().map(Float::F32),
        }
    }
}
impl Mul<Integer> for Float {
    type Output = Self;
    fn mul(self, rhs: Integer) -> Self::Output {
        match (self, rhs) {
            (Float::Rug(a), Integer::Rug(b)) => Float::Rug(a * b),
            (Float::Fastnum(a), Integer::Fastnum(b)) => Float::Fastnum(a * b),
            (Float::F64(a), Integer::F64(b)) => Float::F64(a * b as f64),
            (Float::F32(a), Integer::F32(b)) => Float::F32(a * b as f32),
            _ => unreachable!(),
        }
    }
}
impl Div<Integer> for Float {
    type Output = Self;
    fn div(self, rhs: Integer) -> Self::Output {
        match (self, rhs) {
            (Float::Rug(a), Integer::Rug(b)) => Float::Rug(a / b),
            (Float::Fastnum(a), Integer::Fastnum(b)) => Float::Fastnum(a / b),
            (Float::F64(a), Integer::F64(b)) => Float::F64(a / b as f64),
            (Float::F32(a), Integer::F32(b)) => Float::F32(a / b as f32),
            _ => unreachable!(),
        }
    }
}
impl Mul<Float> for Integer {
    type Output = Float;
    fn mul(self, rhs: Float) -> Self::Output {
        match (rhs, self) {
            (Float::Rug(a), Integer::Rug(b)) => Float::Rug(a * b),
            (Float::Fastnum(a), Integer::Fastnum(b)) => Float::Fastnum(a * b),
            (Float::F64(a), Integer::F64(b)) => Float::F64(a * b as f64),
            (Float::F32(a), Integer::F32(b)) => Float::F32(a * b as f32),
            _ => unreachable!(),
        }
    }
}
impl Div<Float> for Integer {
    type Output = Float;
    fn div(self, rhs: Float) -> Self::Output {
        match (rhs, self) {
            (Float::Rug(a), Integer::Rug(b)) => Float::Rug(a / b),
            (Float::Fastnum(a), Integer::Fastnum(b)) => Float::Fastnum(a / b),
            (Float::F64(a), Integer::F64(b)) => Float::F64(a / b as f64),
            (Float::F32(a), Integer::F32(b)) => Float::F32(a / b as f32),
            _ => unreachable!(),
        }
    }
}
impl SpecialValues for Float {
    fn pi(t: Type, prec: u32) -> Self {
        match t {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, rug::float::Constant::Pi)),
            Type::Fastnum => Self::Fastnum(Decimal::pi(prec)),
            Type::F64 => Self::F64(f64::pi(prec)),
            Type::F32 => Self::F32(f32::pi(prec)),
        }
    }
    fn nan(t: Type, prec: u32) -> Self {
        match t {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, rug::float::Special::Nan)),
            Type::Fastnum => Self::Fastnum(Decimal::nan(prec)),
            Type::F64 => Self::F64(f64::nan(prec)),
            Type::F32 => Self::F32(f32::nan(prec)),
        }
    }
    fn inf(t: Type, prec: u32) -> Self {
        match t {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, rug::float::Special::Infinity)),
            Type::Fastnum => Self::Fastnum(Decimal::inf(prec)),
            Type::F64 => Self::F64(f64::inf(prec)),
            Type::F32 => Self::F32(f32::inf(prec)),
        }
    }
}

impl WithVal<Special> for Float {
    fn with_val(obj: Type, prec: u32, val: Special) -> Self {
        match val {
            Special::Pi => Float::pi(obj, prec),
            Special::Nan => Float::nan(obj, prec),
            Special::Infinity => Float::inf(obj, prec),
        }
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rug(a) => a.fmt(f),
            Self::Fastnum(a) => a.fmt(f),
            Self::F32(a) => a.fmt(f),
            Self::F64(a) => a.fmt(f),
        }
    }
}

impl Float {
    pub fn sin_cos(self) -> (Self, Self) {
        match self {
            Self::Rug(a) => {
                let p = a.prec();
                let (s, c) = a.sin_cos(rug::Float::new(p));
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
    pub fn sinh_cosh(self) -> (Self, Self) {
        match self {
            Self::Rug(a) => {
                let p = a.prec();
                let (s, c) = a.sinh_cosh(rug::Float::new(p));
                (Self::Rug(s), Self::Rug(c))
            }
            Self::Fastnum(a) => {
                let (s, c) = a.sinh_cosh();
                (Self::Fastnum(s), Self::Fastnum(c))
            }
            Self::F64(a) => {
                let (s, c) = a.sinh_cosh();
                (Self::F64(s), Self::F64(c))
            }
            Self::F32(a) => {
                let (s, c) = a.sinh_cosh();
                (Self::F32(s), Self::F32(c))
            }
        }
    }
    pub fn atan2(self, other: Self) -> Self {
        match (self, other) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a.atan2(&b)),
            (Self::Fastnum(a), Self::Fastnum(b)) => Self::Fastnum(a.atan2(b)),
            (Self::F64(a), Self::F64(b)) => Self::F64(a.atan2(b)),
            (Self::F32(a), Self::F32(b)) => Self::F32(a.atan2(b)),
            _ => unreachable!(),
        }
    }
    pub fn hypot(self, other: Self) -> Self {
        match (self, other) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a.hypot(&b)),
            (Self::Fastnum(a), Self::Fastnum(b)) => Self::Fastnum(a.hypot(b)),
            (Self::F64(a), Self::F64(b)) => Self::F64(a.hypot(b)),
            (Self::F32(a), Self::F32(b)) => Self::F32(a.hypot(b)),
            _ => unreachable!(),
        }
    }
    pub fn to_integer(self) -> Integer {
        match self {
            Float::Rug(a) => Integer::Rug(a.to_integer().unwrap()),
            Float::Fastnum(a) => Integer::Fastnum(a.to_integer()),
            Float::F64(a) => Integer::F64(a as i128),
            Float::F32(a) => Integer::F32(a as i128),
        }
    }
    pub fn gamma(self) -> Self {
        match self {
            Float::Rug(a) => Self::Rug(a.gamma()),
            Float::Fastnum(a) => Self::Fastnum(a),
            Float::F64(a) => Self::F64(a),
            Float::F32(a) => Self::F32(a),
        }
    }
}

impl PartialEq<f64> for Float {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Float::Rug(a) => a == other,
            Float::Fastnum(a) => a == other,
            Float::F64(a) => a == other,
            Float::F32(a) => *a == *other as f32,
        }
    }
}
impl PartialEq<i32> for Float {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Float::Rug(a) => a == other,
            Float::Fastnum(a) => a == other,
            Float::F64(a) => *a == *other as f64,
            Float::F32(a) => *a == *other as f32,
        }
    }
}
impl PartialOrd<i32> for Float {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        match self {
            Float::Rug(a) => a.partial_cmp(other),
            Float::Fastnum(a) => a.to_integer().partial_cmp(&fastnum::I512::from(*other)),
            Float::F64(a) => a.partial_cmp(&(*other as f64)),
            Float::F32(a) => a.partial_cmp(&(*other as f32)),
        }
    }
}

impl_new_val!(
    Float,
    (Rug, rug::Float::with_val),
    (Fastnum, Decimal::with_val),
    (F64, |_, x| x),
    (F32, |_, x| x as f32)
);
impl_rem!(
    Float,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x as f64),
    (F32, |x| x as f32)
);
float_impl!(Float, Rug, Fastnum, F64, F32);
impl_partial_ord!(
    Float,
    (Rug, |x: &rug::Float| x.to_f64()),
    (Fastnum, |x: &Decimal| x.to_f64()),
    (F64, |x: &f64| *x),
    (F32, |x: &f32| *x as f64)
);
impl_neg!(Float, Rug, Fastnum, F64, F32);
impl_self_ops!(
    Float,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x)
);
