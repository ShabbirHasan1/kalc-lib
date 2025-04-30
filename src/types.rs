use rug::ops::{CompleteRound, Pow as RugPow};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
//malachite num maybe
//make real only an option
//maybe box some things to avoid memory bads
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDecimal(Decimal, Decimal);
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct CF64(f64, f64);
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct CF32(f32, f32);
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Decimal {
    D512(fastnum::D512),
    D256(fastnum::D256),
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Complex {
    Rug(rug::Complex),
    Fastnum(CDecimal),
    F64(CF64),
    F32(CF32),
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Float {
    Rug(rug::Float),
    Fastnum(Decimal),
    F64(f64),
    F32(f32),
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Integer {
    Rug(rug::Integer),
    Fastnum(fastnum::I512),
    F64(i128),
    F32(i128),
}
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    Rug,
    Fastnum,
    F64,
    F32,
}
pub trait Prec {
    fn prec(&self) -> u32;
}
pub trait DivFloor {
    fn div_floor(self, rhs: f64) -> Self;
}
impl DivFloor for f64 {
    fn div_floor(self, rhs: f64) -> Self {
        (self / rhs).floor()
    }
}
impl Prec for Decimal {
    fn prec(&self) -> u32 {
        match self {
            Decimal::D512(_) => 512,
            Decimal::D256(_) => 256,
        }
    }
}
impl Prec for CDecimal {
    fn prec(&self) -> u32 {
        self.0.prec()
    }
}
impl Prec for CF64 {
    fn prec(&self) -> u32 {
        64
    }
}
impl Prec for CF32 {
    fn prec(&self) -> u32 {
        32
    }
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
pub trait Parse<T> {
    fn parse(prec: u32, s: T) -> Option<Self>
    where
        Self: Sized;
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
pub trait ParseU<T> {
    fn parse(t: Type, prec: u32, s: T) -> Option<Self>
    where
        Self: Sized;
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
pub trait Special {
    fn pi(prec: u32) -> Self;
    fn nan(prec: u32) -> Self;
    fn inf(prec: u32) -> Self;
}
pub trait SpecialU {
    fn pi(t: Type, prec: u32) -> Self;
    fn nan(t: Type, prec: u32) -> Self;
    fn inf(t: Type, prec: u32) -> Self;
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
impl Special for f64 {
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
impl Special for f32 {
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
impl SpecialU for Float {
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
impl From<f32> for CF32 {
    fn from(value: f32) -> Self {
        Self(value, 0.0)
    }
}
impl From<f64> for CF64 {
    fn from(value: f64) -> Self {
        Self(value, 0.0)
    }
}
impl From<Decimal> for CDecimal {
    fn from(value: Decimal) -> Self {
        Self(value, Decimal::new(value.prec()))
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
macro_rules! impl_sinh_cosh {
    ($($t:ty),*) => {
        $(impl SinhCosh for $t {
    fn sinh_cosh(self) -> (Self, Self) {
        (self.sinh(), self.cosh())
    }
})*
    };
}
macro_rules! impl_pow {
    ($ty:ty, $other:ty, $( ($variant:ident, $cast:expr) ),* ) => {
        impl Pow<$other> for $ty {
            fn pow(self, rhs: $other) -> Self {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a.pow($cast(rhs))),
                    )*
                }
            }
        }
    };
}
macro_rules! impl_with_val_deci {
    ($ty:ty, $other:ty) => {
        impl WithValDeci<$other> for $ty {
            fn with_val(prec: u32, rhs: $other) -> Self {
                match prec.next_power_of_two() {
                    512 => Self::D512(fastnum::D512::from(rhs)),
                    256 => Self::D256(fastnum::D256::from(rhs)),
                    _ => unreachable!(),
                }
            }
        }
    };
}
macro_rules! impl_new_val_deci {
    ($ty:ty) => {
        impl NewDeciVal for $ty {
            fn new(prec: u32) -> Self {
                match prec.next_power_of_two() {
                    512 => Self::D512(fastnum::D512::from(0)),
                    256 => Self::D256(fastnum::D256::from(0)),
                    _ => unreachable!(),
                }
            }
        }
    };
}
macro_rules! impl_with_val_cdeci {
    ($ty:ty, $other:ty) => {
        impl WithValDeci<$other> for $ty {
            fn with_val(prec: u32, rhs: $other) -> Self {
                CDecimal(Decimal::with_val(prec, rhs), Decimal::with_val(prec, 0))
            }
        }
    };
}
macro_rules! impl_new_val_cdeci {
    ($ty:ty) => {
        impl NewDeciVal for $ty {
            fn new(prec: u32) -> Self {
                CDecimal(Decimal::new(prec), Decimal::new(prec))
            }
        }
    };
}
macro_rules! impl_with_val {
    ($ty:ty, $other:ty, $( ($variant:ident, $cast:expr) ),* ) => {
        impl WithVal<$other> for $ty {
            fn with_val(obj: Type, prec: u32, rhs: $other) -> Self {
                match obj {
                    $(
                        Type::$variant => Self::$variant($cast(prec, rhs)),
                    )*
                }
            }
    }
    };
}
macro_rules! impl_new_val {
    ($ty:ty, $( ($variant:ident, $cast:expr) ),* ) => {
        impl NewVal for $ty {
            fn new(obj: Type, prec: u32) -> Self {
                match obj {
                    $(
                        Type::$variant => Self::$variant($cast(prec, 0.0)),
                    )*
                }
            }
        }
    };
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
impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::D512(a) => a.fmt(f),
            Self::D256(a) => a.fmt(f),
        }
    }
}
impl Display for CDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}
impl Display for CF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}
impl Display for CF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}
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
macro_rules! impl_partial_ord {
    ($t:ty,$($variant:ident),*) => {
impl PartialOrd for $t {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            $((Self::$variant(a), Self::$variant(b)) => a.partial_cmp(b),)*
            _=>unreachable!()
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            $((Self::$variant(a), Self::$variant(b)) => a.lt(b),)*
            _=>unreachable!()
        }
    }
    fn le(&self, other: &Self) -> bool {
        match (self, other) {
        $(    (Self::$variant(a), Self::$variant(b)) => a.le(b),)*
            _=>unreachable!()
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
          $(  (Self::$variant(a), Self::$variant(b)) => a.gt(b),)*
            _=>unreachable!()
        }
    }
    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            $((Self::$variant(a), Self::$variant(b)) => a.ge(b),)*
            _=>unreachable!()
        }
    }
}
    };
}
macro_rules! dec_impl {
    ($t:ty,$($variant:ident),*) => {
        impl $t {
            pub fn abs(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.abs()),)*
                }
            }
            pub fn recip(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.recip()),)*
                }
            }
            pub fn sqrt(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sqrt()),)*
                }
            }
            pub fn exp(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.exp()),)*
                }
            }
            pub fn ln(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.ln()),)*
                }
            }
            pub fn log2(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.log2()),)*
                }
            }
            pub fn log10(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.log10()),)*
                }
            }
            pub fn cbrt(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cbrt()),)*
                }
            }
            pub fn sin(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sin()),)*
                }
            }
            pub fn cos(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cos()),)*
                }
            }
            pub fn tan(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.tan()),)*
                }
            }
            pub fn asin(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.asin()),)*
                }
            }
            pub fn acos(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.acos()),)*
                }
            }
            pub fn atan(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.atan()),)*
                }
            }
            pub fn atan2(self, other: Self) -> Self {
                match (self, other) {
                    $( (Self::$variant(a), Self::$variant(b)) => Self::$variant(a.atan2(b)), )*
                    _ => unreachable!(),
                }
            }
            pub fn sinh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sinh()),)*
                }
            }
            pub fn cosh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cosh()),)*
                }
            }
            pub fn tanh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.tanh()),)*
                }
            }
            pub fn asinh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.asinh()),)*
                }
            }
            pub fn acosh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.acosh()),)*
                }
            }
            pub fn atanh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.atanh()),)*
                }
            }
            pub fn round(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.round(0)),)*
                }
            }
            pub fn floor(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.floor()),)*
                }
            }
            pub fn ceil(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.ceil()),)*
                }
            }
            pub fn trunc(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(if a.is_sign_positive(){a.floor()}else{a.ceil()}),)*
                }
            }
            pub fn fract(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a-if a.is_sign_positive(){a.floor()}else{a.ceil()}),)*
                }
            }
            pub fn sin_cos(self) -> (Self, Self) {
                match self {
                    $(Self::$variant(a) => {
                        let (s, c) = a.sin_cos();
                        (Self::$variant(s), Self::$variant(c))
                    },)*
                }
            }
            pub fn sinh_cosh(self) -> (Self, Self) {
                match self {
                    $(Self::$variant(a) => {
                        let (s, c) = a.sinh_cosh();
                        (Self::$variant(s), Self::$variant(c))
                    },)*
                }
            }
            pub fn hypot(self, other: Self) -> Self {
                match (self, other) {
                    $( (Self::$variant(a), Self::$variant(b)) => Self::$variant(a.hypot(b)), )*
                    _ => unreachable!(),
                }
            }
            pub fn is_nan(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_nan(),)*
                }
            }
            pub fn is_infinite(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_infinite(),)*
                }
            }
            pub fn is_finite(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_finite(),)*
                }
            }
            pub fn is_sign_positive(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_sign_positive(),)*
                }
            }
            pub fn is_sign_negative(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_sign_negative(),)*
                }
            }
        }
    };
}
macro_rules! float_impl {
    ($t:ty,$($variant:ident),*) => {
        impl $t {
            pub fn abs(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.abs()),)*
                }
            }
            pub fn recip(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.recip()),)*
                }
            }
            pub fn sqrt(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sqrt()),)*
                }
            }
            pub fn exp(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.exp()),)*
                }
            }
            pub fn ln(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.ln()),)*
                }
            }
            pub fn log2(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.log2()),)*
                }
            }
            pub fn log10(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.log10()),)*
                }
            }
            pub fn cbrt(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cbrt()),)*
                }
            }
            pub fn sin(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sin()),)*
                }
            }
            pub fn cos(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cos()),)*
                }
            }
            pub fn tan(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.tan()),)*
                }
            }
            pub fn asin(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.asin()),)*
                }
            }
            pub fn acos(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.acos()),)*
                }
            }
            pub fn atan(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.atan()),)*
                }
            }
            pub fn sinh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sinh()),)*
                }
            }
            pub fn cosh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cosh()),)*
                }
            }
            pub fn tanh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.tanh()),)*
                }
            }
            pub fn asinh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.asinh()),)*
                }
            }
            pub fn acosh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.acosh()),)*
                }
            }
            pub fn atanh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.atanh()),)*
                }
            }
            pub fn round(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.round()),)*
                }
            }
            pub fn floor(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.floor()),)*
                }
            }
            pub fn ceil(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.ceil()),)*
                }
            }
            pub fn trunc(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.trunc()),)*
                }
            }
            pub fn fract(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.fract()),)*
                }
            }
            pub fn is_nan(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_nan(),)*
                }
            }
            pub fn is_infinite(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_infinite(),)*
                }
            }
            pub fn is_finite(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_finite(),)*
                }
            }
            pub fn is_sign_positive(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_sign_positive(),)*
                }
            }
            pub fn is_sign_negative(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_sign_negative(),)*
                }
            }
        }
    };
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
}
macro_rules! dec_c_impl {
    ($t:ty, $l:ty, $new:expr) => {
        #[allow(clippy::unnecessary_cast)]
        impl $t {
            pub fn abs(self) -> Self {
                Self(
                    (self.0 * self.0 + self.1 * self.1).sqrt(),
                    $new(self.prec(), 0.0),
                )
            }
            pub fn recip(self) -> Self {
                let abs = self.0 * self.0 + self.1 * self.1;
                Self(self.0 / abs, -self.1 / abs)
            }
            pub fn sqrt(self) -> Self {
                self.pow(0.5)
            }
            pub fn exp(self) -> Self {
                let r = self.0.exp();
                let (c, s) = self.1.sin_cos();
                r * Self(c, s)
            }
            pub fn arg(self) -> Self {
                Self(self.1.atan2(self.0), $new(self.prec(), 0.0))
            }
            pub fn ln(self) -> Self {
                let abs = self.0 * self.0 + self.1 * self.1;
                Self(abs.ln() * 0.5, self.1.atan2(self.0))
            }
            pub fn log2(self) -> Self {
                self.ln() / $new(self.prec(), 2.0).ln()
            }
            pub fn log10(self) -> Self {
                self.ln() / $new(self.prec(), 10.0).ln()
            }
            pub fn cbrt(self) -> Self {
                self.root(3)
            }
            pub fn conj(self) -> Self {
                Self(self.0, -self.1)
            }
            pub fn sin(self) -> Self {
                let (a, b) = self.0.sin_cos();
                let (c, d) = self.1.sinh_cosh();
                Self(a * d, b * c)
            }
            pub fn cos(self) -> Self {
                let (a, b) = self.0.sin_cos();
                let (c, d) = self.1.sinh_cosh();
                Self(b * d, a * c)
            }
            pub fn tan(mut self) -> Self {
                self.0 *= 2.0;
                self.1 *= 2.0;
                let (a, b) = self.0.sin_cos();
                let (c, d) = self.1.sinh_cosh();
                Self(a, c) / (b + d)
            }
            pub fn asin(self) -> Self {
                let p: $t = 1.0 - self * self;
                let v = Self(-self.1, self.0) + p.sqrt();
                v.ln()
            }
            pub fn acos(self) -> Self {
                <$l>::pi(self.prec()) / 2.0 - self.asin()
            }
            pub fn atan(self) -> Self {
                let v = Self(-self.1, self.0);
                let a: $t = 1 + v;
                let b: $t = 1 - v;
                (a.arg() - b.arg()) / 2.0
            }
            pub fn atan2(self, other: Self) -> Self {
                let v = (Self(-self.1, self.0) + other) / (self * self + other * other).sqrt();
                v.ln()
            }
            pub fn sinh(self) -> Self {
                let (a, b) = self.0.sinh_cosh();
                let (c, d) = self.1.sin_cos();
                Self(a * d, b * c)
            }
            pub fn cosh(self) -> Self {
                let (a, b) = self.0.sinh_cosh();
                let (c, d) = self.1.sin_cos();
                Self(b * d, a * c)
            }
            pub fn tanh(mut self) -> Self {
                self.0 *= 2.0;
                self.1 *= 2.0;
                let (a, b) = self.0.sinh_cosh();
                let (c, d) = self.1.sin_cos();
                Self(a, c) / (b + d)
            }
            pub fn asinh(self) -> Self {
                let v: $t = 1.0 + self * self;
                (v.sqrt() + self).ln()
            }
            pub fn acosh(self) -> Self {
                let v: $t = self * self - 1.0;
                (v.sqrt() + self).ln()
            }
            pub fn atanh(self) -> Self {
                let a: $t = 1 + self;
                let b: $t = 1 - self;
                (a.ln() - b.ln()) / 2.0
            }
            pub fn sin_cos(self) -> (Self, Self) {
                (self.sin(), self.cos())
            }
            pub fn sinh_cosh(self) -> (Self, Self) {
                (self.sinh(), self.cosh())
            }
            pub fn is_nan(self) -> bool {
                self.0.is_nan() || self.1.is_nan()
            }
            pub fn is_infinite(self) -> bool {
                self.0.is_infinite() || self.1.is_infinite()
            }
            pub fn is_finite(self) -> bool {
                self.0.is_finite() && self.1.is_finite()
            }
        }
    };
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
}
macro_rules! impl_c_pow {
    ($t:ty, $rhs:ty, $cast:expr) => {
        impl Pow<$rhs> for $t {
            fn pow(self, rhs: $rhs) -> Self {
                let rhs = $cast(rhs);
                let t = self.1.atan2(self.0);
                let abs = self.0 * self.0 + self.1 * self.1;
                let r = abs.pow(rhs * 0.5) * rhs;
                let (s, c) = t.sin_cos();
                Self(r * c, r * s)
            }
        }
    };
}
macro_rules! impl_c_rt {
    ($t:ty, $rhs:ty, $cast:expr) => {
        impl Rt<$rhs> for $t {
            fn root(self, rhs: $rhs) -> Self {
                let rhs = $cast(rhs);
                let t = self.1.atan2(self.0);
                let abs = self.0 * self.0 + self.1 * self.1;
                let r = abs.pow(0.5 / rhs) / rhs;
                let (s, c) = t.sin_cos();
                Self(r * c, r * s)
            }
        }
    };
}
macro_rules! impl_c_ops {
    ($t:ty, $enum:ident, $rhs:ty, $cast:expr) => {
        impl std::ops::Add<$rhs> for $t {
            type Output = Self;
            fn add(self, rhs: $rhs) -> Self::Output {
                Self(self.0 + $cast(rhs), self.1)
            }
        }
        impl std::ops::Add<$t> for $rhs {
            type Output = $t;
            fn add(self, rhs: $t) -> Self::Output {
                $enum($cast(self) + rhs.0, rhs.1)
            }
        }
        impl std::ops::AddAssign<$rhs> for $t {
            fn add_assign(&mut self, rhs: $rhs) {
                self.0 += $cast(rhs);
            }
        }
        impl std::ops::Sub<$rhs> for $t {
            type Output = Self;
            fn sub(self, rhs: $rhs) -> Self::Output {
                Self(self.0 - $cast(rhs), self.1)
            }
        }
        impl std::ops::Sub<$t> for $rhs {
            type Output = $t;
            fn sub(self, rhs: $t) -> Self::Output {
                $enum($cast(self) - rhs.0, -rhs.1)
            }
        }
        impl std::ops::SubAssign<$rhs> for $t {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.0 -= $cast(rhs);
            }
        }
        impl std::ops::Mul<$rhs> for $t {
            type Output = Self;
            fn mul(self, rhs: $rhs) -> Self::Output {
                Self(self.0 * $cast(rhs), self.1 * $cast(rhs))
            }
        }
        impl std::ops::Mul<$t> for $rhs {
            type Output = $t;
            fn mul(self, rhs: $t) -> Self::Output {
                $enum($cast(self) * rhs.0, $cast(self) * rhs.1)
            }
        }
        impl std::ops::MulAssign<$rhs> for $t {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.0 *= $cast(rhs);
            }
        }
        impl std::ops::Div<$rhs> for $t {
            type Output = Self;
            fn div(self, rhs: $rhs) -> Self::Output {
                Self(self.0 / $cast(rhs), self.1 / $cast(rhs))
            }
        }
        impl std::ops::Div<$t> for $rhs {
            type Output = $t;
            fn div(self, rhs: $t) -> Self::Output {
                let abs = rhs.0 * rhs.0 + rhs.1 * rhs.1;
                $enum($cast(self) * rhs.0 / abs, $cast(self) * rhs.1 / abs)
            }
        }
        impl std::ops::DivAssign<$rhs> for $t {
            fn div_assign(&mut self, rhs: $rhs) {
                self.0 /= $cast(rhs);
            }
        }
    };
}
macro_rules! impl_self_c_ops {
    ($t:ty) => {
        impl std::ops::Add<$t> for $t {
            type Output = Self;
            fn add(self, rhs: $t) -> Self::Output {
                Self(self.0 + rhs.0, self.1 + rhs.1)
            }
        }
        impl std::ops::AddAssign<$t> for $t {
            fn add_assign(&mut self, rhs: $t) {
                self.0 += rhs.0;
                self.1 += rhs.1;
            }
        }
        impl std::ops::Sub<$t> for $t {
            type Output = Self;
            fn sub(self, rhs: $t) -> Self::Output {
                Self(self.0 - rhs.0, self.1 - rhs.1)
            }
        }
        impl std::ops::SubAssign<$t> for $t {
            fn sub_assign(&mut self, rhs: $t) {
                self.0 -= rhs.0;
                self.1 -= rhs.1;
            }
        }
        impl std::ops::Mul<$t> for $t {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self(
                    self.0 * rhs.0 - self.1 * rhs.1,
                    self.1 * rhs.0 + self.0 * rhs.1,
                )
            }
        }
        impl std::ops::MulAssign<$t> for $t {
            fn mul_assign(&mut self, rhs: $t) {
                *self = Self(
                    self.0 * rhs.0 - self.1 * rhs.1,
                    self.1 * rhs.0 + self.0 * rhs.1,
                )
            }
        }
        impl std::ops::Div<$t> for $t {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                let sq = (self.0 * self.0 + rhs.0 * rhs.0).recip();
                Self(
                    (self.0 * rhs.0 + self.1 * rhs.1) * sq,
                    (self.1 * rhs.0 - self.0 * rhs.1) * sq,
                )
            }
        }
        impl std::ops::DivAssign<$t> for $t {
            fn div_assign(&mut self, rhs: $t) {
                let sq = (self.0 * self.0 + rhs.0 * rhs.0).recip();
                *self = Self(
                    (self.0 / rhs.0 + self.1 / rhs.1) * sq,
                    (self.1 / rhs.0 - self.0 / rhs.1) * sq,
                )
            }
        }
    };
}
macro_rules! impl_ops {
    ($ty:ty, $enum:ident, $other:ty, $( ($variant:ident, $cast:expr, $div:expr) ),* ) => {
        impl std::ops::Add<$other> for $ty {
            type Output = Self;
            fn add(self, rhs: $other) -> Self::Output {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a + $cast(rhs)),
                    )*
                }
            }
        }
        impl std::ops::Add<$ty> for $other {
            type Output = $ty;
            fn add(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $(
                        $enum::$variant(a) => $enum::$variant(a+$cast(self)),
                    )*
                }
            }
        }
        impl std::ops::AddAssign<$other> for $ty {
            fn add_assign(&mut self, rhs: $other) {
                match self {
                    $(
                        Self::$variant(a) => *a += $cast(rhs),
                    )*
                }
            }
        }
        impl std::ops::Sub<$other> for $ty {
            type Output = Self;
            fn sub(self, rhs: $other) -> Self::Output {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a - $cast(rhs)),
                    )*
                }
            }
        }
        impl std::ops::Sub<$ty> for $other {
            type Output = $ty;
            fn sub(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $(
                        $enum::$variant(a) => $enum::$variant($cast(self) - a),
                    )*
                }
            }
        }
        impl std::ops::SubAssign<$other> for $ty {
            fn sub_assign(&mut self, rhs: $other) {
                match self {
                    $(
                        Self::$variant(a) => *a -= $cast(rhs),
                    )*
                }
            }
        }
        impl std::ops::Mul<$other> for $ty {
            type Output = Self;
            fn mul(self, rhs: $other) -> Self::Output {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a * $cast(rhs)),
                    )*
                }
            }
        }
        impl std::ops::Mul<$ty> for $other {
            type Output = $ty;
            fn mul(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $(
                        $enum::$variant(a) => $enum::$variant(a*$cast(self)),
                    )*
                }
            }
        }
        impl std::ops::MulAssign<$other> for $ty {
            fn mul_assign(&mut self, rhs: $other) {
                match self {
                    $(
                        Self::$variant(a) => *a *= $cast(rhs),
                    )*
                }
            }
        }
        impl std::ops::Div<$other> for $ty {
            type Output = Self;
            fn div(self, rhs: $other) -> Self::Output {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a / $cast(rhs)),
                    )*
                }
            }
        }
        impl std::ops::Div<$ty> for $other {
            type Output = $ty;
            fn div(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $(
                        $enum::$variant(a) => $enum::$variant($div($cast(self),a)),
                    )*
                }
            }
        }
        impl std::ops::DivAssign<$other> for $ty {
            fn div_assign(&mut self, rhs: $other) {
                match self {
                    $(
                        Self::$variant(a) => *a /= $cast(rhs),
                    )*
                }
            }
        }
    };
}
macro_rules! impl_int_ops {
    ($ty:ty, $enum:ident, $other:ty) => {
        impl std::ops::Add<$other> for $ty {
            type Output = Self;
            fn add(self, rhs: $other) -> Self::Output {
                match self {
                    Self::Rug(a) => Self::Rug(a + rhs),
                    Self::Fastnum(a) => Self::Fastnum(a + fastnum::I512::from(rhs)),
                    Self::F64(a) => Self::F64(a + rhs as i128),
                    Self::F32(a) => Self::F32(a + rhs as i128),
                }
            }
        }
        impl std::ops::Add<$ty> for $other {
            type Output = $ty;
            fn add(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $enum::Rug(a) => $enum::Rug(self + a),
                    $enum::Fastnum(a) => $enum::Fastnum(fastnum::I512::from(self) + a),
                    $enum::F64(a) => $enum::F64(self as i128 + a),
                    $enum::F32(a) => $enum::F32(self as i128 + a),
                }
            }
        }
        impl std::ops::AddAssign<$other> for $ty {
            fn add_assign(&mut self, rhs: $other) {
                match self {
                    Self::Rug(a) => *a += rhs,
                    Self::Fastnum(a) => *a = fastnum::I512::from(rhs),
                    Self::F64(a) => *a += rhs as i128,
                    Self::F32(a) => *a += rhs as i128,
                }
            }
        }
        impl std::ops::Sub<$other> for $ty {
            type Output = Self;
            fn sub(self, rhs: $other) -> Self::Output {
                match self {
                    Self::Rug(a) => Self::Rug(a - rhs),
                    Self::Fastnum(a) => Self::Fastnum(a - fastnum::I512::from(rhs)),
                    Self::F64(a) => Self::F64(a - rhs as i128),
                    Self::F32(a) => Self::F32(a - rhs as i128),
                }
            }
        }
        impl std::ops::Sub<$ty> for $other {
            type Output = $ty;
            fn sub(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $enum::Rug(a) => $enum::Rug(self - a),
                    $enum::Fastnum(a) => $enum::Fastnum(fastnum::I512::from(self) - a),
                    $enum::F64(a) => $enum::F64(self as i128 - a),
                    $enum::F32(a) => $enum::F32(self as i128 - a),
                }
            }
        }
        impl std::ops::SubAssign<$other> for $ty {
            fn sub_assign(&mut self, rhs: $other) {
                match self {
                    Self::Rug(a) => *a -= rhs,
                    Self::Fastnum(a) => *a -= fastnum::I512::from(rhs),
                    Self::F64(a) => *a -= rhs as i128,
                    Self::F32(a) => *a -= rhs as i128,
                }
            }
        }
        impl std::ops::Mul<$other> for $ty {
            type Output = Self;
            fn mul(self, rhs: $other) -> Self::Output {
                match self {
                    Self::Rug(a) => Self::Rug(a * rhs),
                    Self::Fastnum(a) => Self::Fastnum(a * fastnum::I512::from(rhs)),
                    Self::F64(a) => Self::F64(a * rhs as i128),
                    Self::F32(a) => Self::F32(a * rhs as i128),
                }
            }
        }
        impl std::ops::Mul<$ty> for $other {
            type Output = $ty;
            fn mul(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $enum::Rug(a) => $enum::Rug(self * a),
                    $enum::Fastnum(a) => $enum::Fastnum(fastnum::I512::from(self) * a),
                    $enum::F64(a) => $enum::F64(self as i128 * a),
                    $enum::F32(a) => $enum::F32(self as i128 * a),
                }
            }
        }
        impl std::ops::MulAssign<$other> for $ty {
            fn mul_assign(&mut self, rhs: $other) {
                match self {
                    Self::Rug(a) => *a *= rhs,
                    Self::Fastnum(a) => *a *= fastnum::I512::from(rhs),
                    Self::F64(a) => *a *= rhs as i128,
                    Self::F32(a) => *a *= rhs as i128,
                }
            }
        }
        impl std::ops::Div<$other> for $ty {
            type Output = Self;
            fn div(self, rhs: $other) -> Self::Output {
                match self {
                    Self::Rug(a) => Self::Rug(a / rhs),
                    Self::Fastnum(a) => Self::Fastnum(a / fastnum::I512::from(rhs)),
                    Self::F64(a) => Self::F64(a / rhs as i128),
                    Self::F32(a) => Self::F32(a / rhs as i128),
                }
            }
        }
        impl std::ops::Div<$ty> for $other {
            type Output = $ty;
            fn div(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $enum::Rug(a) => $enum::Rug(self / a),
                    $enum::Fastnum(a) => $enum::Fastnum(fastnum::I512::from(self) / a),
                    $enum::F64(a) => $enum::F64(self as i128 / a),
                    $enum::F32(a) => $enum::F32(self as i128 / a),
                }
            }
        }
        impl std::ops::DivAssign<$other> for $ty {
            fn div_assign(&mut self, rhs: $other) {
                match self {
                    Self::Rug(a) => *a /= rhs,
                    Self::Fastnum(a) => *a /= fastnum::I512::from(rhs),
                    Self::F64(a) => *a /= rhs as i128,
                    Self::F32(a) => *a /= rhs as i128,
                }
            }
        }
    };
}
macro_rules! impl_self_ops {
    ($ty:ty, $( $variant:ident ),* ) => {
        impl std::ops::Add for $ty {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => Self::$variant(a + b),
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::AddAssign for $ty {
            fn add_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => *a += b,
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::Sub for $ty {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => Self::$variant(a - b),
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::SubAssign for $ty {
            fn sub_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => *a -= b,
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::Mul for $ty {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => Self::$variant(a * b),
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::MulAssign for $ty {
            fn mul_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => *a *= b,
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::Div for $ty {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => Self::$variant(a / b),
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::DivAssign for $ty {
            fn div_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => *a /= b,
                    )*
                    _ => unreachable!(),
                }
            }
        }
    }
}
macro_rules! impl_neg {
    ($ty:ty, $( $variant:ident ),* ) => {
        impl std::ops::Neg for $ty {
            type Output = Self;
            fn neg(self) -> Self::Output {
                match self {
                    $(
                    Self::$variant(a) => Self::$variant(-a),
                    )*
                }
            }
        }
    };
}
macro_rules! impl_cneg {
    ($ty:ty) => {
        impl std::ops::Neg for $ty {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0, -self.1)
            }
        }
    };
}
macro_rules! impl_types {
    ($($ty:ty),*) => {
        $(
impl_with_val_deci!(Decimal, $ty);
impl_with_val_cdeci!(CDecimal, $ty);
impl_pow!(
    Decimal,
    $ty,
    (D512, fastnum::D512::from),
    (D256, fastnum::D256::from)
);
impl_pow!(
    Complex,
    $ty,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x)
);
impl_pow!(
    Float,
    $ty,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x as f64),
    (F32, |x| x as f32)
);
impl_c_ops!(CDecimal, CDecimal, $ty, |x| x);
impl_c_ops!(CF64, CF64, $ty, |x| x as f64);
impl_c_ops!(CF32, CF32, $ty, |x| x as f32);
impl_ops!(
    Decimal,
    Decimal,
    $ty,
    (D512, |x| x, |a, b: fastnum::D512| b.recip() * a),
    (D256, |x| x, |a, b: fastnum::D256| b.recip() * a)
);
impl_c_pow!(CDecimal, $ty, |x| x as f64);
impl_c_pow!(CF64, $ty, |x| x as f64);
impl_c_pow!(CF32, $ty, |x| x as f32);
impl_c_rt!(CDecimal, $ty, |x| x as f64);
impl_c_rt!(CF64, $ty, |x| x as f64);
impl_c_rt!(CF32, $ty, |x| x as f32);
impl_ops!(
    Complex,
    Complex,
    $ty,
    (Rug, |x| x, |a, b| a / b),
    (Fastnum, |x| x, |a, b| a / b),
    (F64, |x| x as f64, |a, b| a / b),
    (F32, |x| x as f32, |a, b| a / b)
);
impl_ops!(
    Float,
    Float,
    $ty,
    (Rug, |x| x, |a, b| a / b),
    (Fastnum, |x| x, |a, b| a / b),
    (F64, |x| x as f64, |a, b| a / b),
    (F32, |x| x as f32, |a, b| a / b)
);
impl_with_val!(
    Complex,
    $ty,
    (Rug, rug::Complex::with_val),
    (Fastnum, CDecimal::with_val),
    (F64, |_, x| CF64(x as f64, 0.0)),
    (F32, |_, x| CF32(x as f32, 0.0))
);
impl_with_val!(
    Float,
    $ty,
    (Rug, rug::Float::with_val),
    (Fastnum, Decimal::with_val),
    (F64, |_, x| x as f64),
    (F32, |_, x| x as f32)
);
        )*
    };
}
impl_c_ops!(CDecimal, CDecimal, Decimal, |x| x);
impl_types!(f64, f32, i32, u64, u128);
impl_new_val_deci!(Decimal);
impl_new_val_cdeci!(CDecimal);
impl_partial_ord!(Float, Rug, Fastnum, F64, F32);
impl_partial_ord!(Decimal, D512, D256);
dec_impl!(Decimal, D512, D256);
float_impl!(Float, Rug, Fastnum, F64, F32);
impl_new_val!(
    Complex,
    (Rug, rug::Complex::with_val),
    (Fastnum, CDecimal::with_val),
    (F64, |_, x| CF64(x, 0.0)),
    (F32, |_, x| CF32(x as f32, 0.0))
);
impl_new_val!(
    Float,
    (Rug, rug::Float::with_val),
    (Fastnum, Decimal::with_val),
    (F64, |_, x| x),
    (F32, |_, x| x as f32)
);
dec_c_impl!(CDecimal, Decimal, Decimal::with_val);
dec_c_impl!(CF64, f64, |_, x| x as f64);
dec_c_impl!(CF32, f32, |_, x| x as f32);
impl_cneg!(CDecimal);
impl_cneg!(CF64);
impl_cneg!(CF32);
impl_neg!(Decimal, D512, D256);
impl_neg!(Complex, Rug, Fastnum, F64, F32);
impl_neg!(Float, Rug, Fastnum, F64, F32);
impl_self_c_ops!(CDecimal);
impl_self_c_ops!(CF64);
impl_self_c_ops!(CF32);
impl_self_ops!(Decimal, D512, D256);
impl_self_ops!(Complex, Rug, Fastnum, F64, F32);
impl_self_ops!(Float, Rug, Fastnum, F64, F32);
impl_self_ops!(Integer, Rug, Fastnum, F64, F32);
impl_int_ops!(Integer, Integer, i32);
impl_sinh_cosh!(f64, f32, fastnum::decimal::D512, fastnum::decimal::D256);