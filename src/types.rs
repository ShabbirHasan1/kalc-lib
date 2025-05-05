mod cdecimal;
mod cf32;
mod cf64;
mod complex;
mod decimal;
mod float;
mod integer;

pub use cdecimal::CDecimal;
pub use cf32::CF32;
pub use cf64::CF64;
pub use complex::Complex;
pub use decimal::Decimal;
pub use float::Float;
pub use integer::Integer;

use rug::ops::Pow as RugPow;

///TODO malachite num maybe
///TODO make real only an option

#[derive(PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Type {
    Rug,
    Fastnum,
    F64,
    F32,
}

pub enum Special {
    Pi,
    Nan,
    Infinity,
}

pub trait Prec {
    fn prec(&self) -> u32;
    fn set_prec(&mut self, new_prec: u32);
}
pub trait DivFloor {
    fn div_floor(self, rhs: f64) -> Self;
}
impl DivFloor for f64 {
    fn div_floor(self, rhs: f64) -> Self {
        (self / rhs).floor()
    }
}
pub trait Parse<T> {
    fn parse(prec: u32, s: T) -> Option<Self>
    where
        Self: Sized;
    fn parse_radix(prec: u32, s: T, base: i32) -> Option<Self>
    where
        Self: Sized;
}
pub trait ParseU<T> {
    fn parse(t: Type, prec: u32, s: T) -> Option<Self>
    where
        Self: Sized;
    fn parse_radix(t: Type, prec: u32, s: T, base: i32) -> Option<Self>
    where
        Self: Sized;
}
pub trait WithVal<T> {
    fn with_val(obj: Type, prec: u32, val: T) -> Self;
}
pub trait NewVal {
    fn new(obj: Type, prec: u32) -> Self;
}
pub trait WithValDeci<T> {
    fn with_val(prec: u32, val: T) -> Self;
}
pub trait NewDeciVal {
    fn new(prec: u32) -> Self;
}
pub trait Pow<T> {
    fn pow(self, val: T) -> Self;
}
pub trait Rt<T> {
    fn root(self, val: T) -> Self;
}
pub trait SinhCosh {
    fn sinh_cosh(self) -> (Self, Self)
    where
        Self: Sized;
}
pub trait SpecialValuesDeci {
    fn pi(prec: u32) -> Self;
    fn nan(prec: u32) -> Self;
    fn inf(prec: u32) -> Self;
}
pub trait SpecialValues {
    fn pi(t: Type, prec: u32) -> Self;
    fn nan(t: Type, prec: u32) -> Self;
    fn inf(t: Type, prec: u32) -> Self;
}

impl SpecialValuesDeci for f64 {
    fn pi(_: u32) -> Self {
        std::f64::consts::PI
    }
    fn nan(_: u32) -> Self {
        f64::NAN
    }
    fn inf(_: u32) -> Self {
        f64::INFINITY
    }
}
impl SpecialValuesDeci for f32 {
    fn pi(_: u32) -> Self {
        std::f32::consts::PI
    }
    fn nan(_: u32) -> Self {
        f32::NAN
    }
    fn inf(_: u32) -> Self {
        f32::INFINITY
    }
}
use crate::macros::impls::*;
impl_types!(f64, f32, i32, u64, u128);
impl_sinh_cosh!(f64, f32, fastnum::decimal::D512, fastnum::decimal::D256);
