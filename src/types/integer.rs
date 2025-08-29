use super::Type;
use crate::macros::impls::{impl_int_ops, impl_pow, impl_self_ops};
use crate::types::Pow;
use rug::ops::Pow as RugPow;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
#[derive(Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Integer {
    Rug(rug::Integer),
    Fastnum(fastnum::I512),
    F64(i128),
    F32(i128),
}

#[derive(Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Int<T: Add + AddAssign>(T);

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rug(a) => a.fmt(f),
            Self::Fastnum(a) => a.fmt(f),
            Self::F64(a) => a.fmt(f),
            Self::F32(a) => a.fmt(f),
        }
    }
}

impl Integer {
    pub fn is_probably_prime(&self, reps: u32) -> bool {
        match self {
            Self::Rug(a) => a.is_probably_prime(reps) != rug::integer::IsPrime::No,
            Self::Fastnum(_) => false,
            Self::F64(_) => false,
            Self::F32(_) => false,
        }
    }
    pub fn next_prime(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.next_prime()),
            Self::Fastnum(a) => Self::Fastnum(a),
            Self::F64(a) => Self::F64(a),
            Self::F32(a) => Self::F32(a),
        }
    }
    pub fn from(obj: Type, val: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Integer::from(val)),
            Type::Fastnum => Self::Fastnum(fastnum::I512::from(val)),
            Type::F64 => Self::F64(val as i128),
            Type::F32 => Self::F32(val as i128),
        }
    }
    pub fn new(obj: Type) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Integer::new()),
            Type::Fastnum => Self::Fastnum(fastnum::I512::from(0)),
            Type::F64 => Self::F64(0),
            Type::F32 => Self::F32(0),
        }
    }
    pub fn to_i128(self) -> i128 {
        match self {
            Self::Rug(a) => a.to_i128().unwrap_or_default(),
            Self::Fastnum(a) => a.to_string().parse().unwrap_or_default(),
            Self::F64(a) => a,
            Self::F32(a) => a,
        }
    }
    pub fn to_string_radix(self, base: i32) -> String {
        match self {
            Self::Rug(a) => a.to_string_radix(base),
            Self::Fastnum(a) => a.to_str_radix(base as u32),
            Self::F64(a) => a.to_string(),
            Self::F32(a) => a.to_string(),
        }
    }
    pub fn binomial(self, k: u32) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.binomial(k)),
            Self::Fastnum(a) => Self::Fastnum(a),
            Self::F64(a) => Self::F64(a),
            Self::F32(a) => Self::F32(a),
        }
    }
    pub fn div_rem(self, d: Self) -> (Self, Self) {
        match (self, d) {
            (Self::Rug(a), Self::Rug(d)) => {
                let (a, b) = a.div_rem(d);
                (Self::Rug(a), Self::Rug(b))
            }
            (Self::Fastnum(a), Self::Fastnum(d)) => {
                let (a, b) = (a / d, a.rem(d));
                (Self::Fastnum(a), Self::Fastnum(b))
            }
            (Self::F64(a), Self::F64(d)) => {
                let (a, b) = (a / d, a % d);
                (Self::F64(a), Self::F64(b))
            }
            (Self::F32(a), Self::F32(d)) => {
                let (a, b) = (a / d, a % d);
                (Self::F32(a), Self::F32(b))
            }
            _ => unreachable!(),
        }
    }
}

impl_pow!(
    Integer,
    u32,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x)
);
impl_self_ops!(
    Integer,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x)
);
impl_int_ops!(Integer, Integer, i32);
impl_int_ops!(Integer, Integer, u32);
