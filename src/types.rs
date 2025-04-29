use rug::ops::Pow as RugPow;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::{
    cmp::Ordering,
};
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

pub trait WithVal<T> {
    fn with_val(obj: Type, prec: u32, val: T) -> Self;
}

pub trait Pow<T> {
    fn pow(self, val: T) -> Self;
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
impl_pow!(
    Complex,
    f64,
    Rug,
    Fastnum,
    F64,
    F32
);

impl_pow!(
    Complex,
    i32,
    Rug,
    Fastnum,
    F64,
    F32
);

impl WithVal<f64> for Complex {
    fn with_val(obj: Type, prec: u32, val: f64) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, val)),
        }
    }
}
impl WithVal<i32> for Complex {
    fn with_val(obj: Type, prec: u32, val: i32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, val)),
        }
    }
}
impl WithVal<(f64, f64)> for Complex {
    fn with_val(obj: Type, prec: u32, val: (f64, f64)) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, val)),
        }
    }
}
impl WithVal<(i32, i32)> for Complex {
    fn with_val(obj: Type, prec: u32, val: (i32, i32)) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, val)),
        }
    }
}

impl Pow<f64> for Float {
    fn pow(self, val: f64) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.pow(val)),
        }
    }
}
impl WithVal<f64> for Float {
    fn with_val(obj: Type, prec: u32, val: f64) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, val)),
        }
    }
}
impl Pow<i32> for Float {
    fn pow(self, val: i32) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.pow(val)),
        }
    }
}
impl WithVal<i32> for Float {
    fn with_val(obj: Type, prec: u32, val: i32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, val)),
        }
    }
}
impl Pow<Decimal> for Decimal {
    fn pow(self, val: Decimal) -> Self {
        match (self, val) {
            (Decimal::D512(a), Decimal::D512(b)) => Decimal::D512(a.pow(b)),
        }
    }
}
impl Pow<i32> for Decimal {
    fn pow(self, val: i32) -> Self {
        match self {
            Decimal::D512(a) => Decimal::D512(a.powi(val)),
        }
    }
}
impl Pow<f64> for Decimal {
    fn pow(self, val: f64) -> Self {
        match self {
            Decimal::D512(a) => Decimal::D512(a.pow(fastnum::D512::from(val))),
        }
    }
}

impl Complex {
    pub fn pi(obj: Type, prec: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, rug::float::Constant::Pi)),
        }
    }
    pub fn inf(obj: Type, prec: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, rug::float::Special::Infinity)),
        }
    }
    pub fn nan(obj: Type, prec: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, rug::float::Special::Nan)),
        }
    }
    pub fn new(obj: Type, prec: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::new(prec)),
        }
    }
    pub fn real(&self) -> Float {
        match self {
            Self::Rug(a) => Float::Rug(a.real().clone()),
        }
    }
    pub fn imag(&self) -> Float {
        match self {
            Self::Rug(a) => Float::Rug(a.imag().clone()),
        }
    }
    pub fn into_real(self) -> Float {
        match self {
            Self::Rug(a) => Float::Rug(a.into_real_imag().0),
        }
    }
    pub fn into_imag(self) -> Float {
        match self {
            Self::Rug(a) => Float::Rug(a.into_real_imag().1),
        }
    }
    pub fn into_real_imag(self) -> (Float, Float) {
        match self {
            Self::Rug(a) => {
                let (a, b) = a.into_real_imag();
                (Float::Rug(a), Float::Rug(b))
            }
        }
    }
    pub fn prec(&self) -> u32 {
        match self {
            Self::Rug(a) => a.prec().0,
        }
    }
    pub fn abs(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.abs()),
        }
    }
    pub fn ln(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.ln()),
        }
    }
    pub fn exp(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.exp()),
        }
    }
    pub fn sin(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.sin()),
        }
    }
    pub fn cos(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.cos()),
        }
    }
    pub fn tan(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.tan()),
        }
    }
    pub fn sinh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.sinh()),
        }
    }
    pub fn cosh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.cosh()),
        }
    }
    pub fn tanh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.tanh()),
        }
    }
    pub fn asin(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.asin()),
        }
    }
    pub fn acos(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.acos()),
        }
    }
    pub fn atan(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.atan()),
        }
    }
    pub fn asinh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.asinh()),
        }
    }
    pub fn acosh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.acosh()),
        }
    }
    pub fn atanh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.atanh()),
        }
    }
}
impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rug(a) => a.fmt(f),
        }
    }
}
impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rug(a) => a.fmt(f),
        }
    }
}
impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::D512(a) => a.fmt(f),
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
impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Rug(a), Self::Rug(b)) => a.partial_cmp(b),
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rug(a), Self::Rug(b)) => a.lt(b),
        }
    }
    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rug(a), Self::Rug(b)) => a.le(b),
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rug(a), Self::Rug(b)) => a.gt(b),
        }
    }
    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rug(a), Self::Rug(b)) => a.ge(b),
        }
    }
}
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::D512(a), Self::D512(b)) => a.partial_cmp(b),
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::D512(a), Self::D512(b)) => a.lt(b),
        }
    }
    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::D512(a), Self::D512(b)) => a.le(b),
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::D512(a), Self::D512(b)) => a.gt(b),
        }
    }
    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::D512(a), Self::D512(b)) => a.ge(b),
        }
    }
}
impl Float {
    pub fn pi(obj: Type, prec: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, rug::float::Constant::Pi)),
        }
    }
    pub fn inf(obj: Type, prec: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, rug::float::Special::Infinity)),
        }
    }
    pub fn nan(obj: Type, prec: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, rug::float::Special::Nan)),
        }
    }
    pub fn new(obj: Type, prec: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::new(prec)),
        }
    }
    pub fn prec(&self) -> u32 {
        match self {
            Self::Rug(a) => a.prec(),
        }
    }
    pub fn abs(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.abs()),
        }
    }
    pub fn ln(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.ln()),
        }
    }
    pub fn exp(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.exp()),
        }
    }
    pub fn sin(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.sin()),
        }
    }
    pub fn cos(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.cos()),
        }
    }
    pub fn tan(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.tan()),
        }
    }
    pub fn sinh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.sinh()),
        }
    }
    pub fn cosh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.cosh()),
        }
    }
    pub fn tanh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.tanh()),
        }
    }
    pub fn asin(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.asin()),
        }
    }
    pub fn acos(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.acos()),
        }
    }
    pub fn atan(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.atan()),
        }
    }
    pub fn asinh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.asinh()),
        }
    }
    pub fn acosh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.acosh()),
        }
    }
    pub fn atanh(self) -> Self {
        match self {
            Self::Rug(a) => Self::Rug(a.atanh()),
        }
    }
    pub fn max(self, other: Self) -> Self {
        match (self, other) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a.max(&b)),
        }
    }
    pub fn min(self, other: Self) -> Self {
        match (self, other) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a.min(&b)),
        }
    }
}
impl Decimal {
    pub fn recip(self) -> Self {
        match self {
            Decimal::D512(a) => Decimal::D512(a.recip()),
        }
    }
}
impl Integer {
    pub fn is_probably_prime(&self, reps: u32) -> bool {
        match self {
            Self::Rug(a) => a.is_probably_prime(reps) != rug::integer::IsPrime::No,
            Self::Fastnum(_) => todo!(),
            Self::F64(_) => todo!(),
            Self::F32(_) => todo!(),
        }
    }
    pub fn from(obj: Type, val: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Integer::from(val)),
        }
    }
    pub fn new(obj: Type) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Integer::new()),
        }
    }
}
macro_rules! impl_pow {
    ($t:ty, $rhs:ty, $cast:expr) => {
        impl Pow<$rhs> for $t {
            fn pow(self, rhs: $rhs) -> Self {
                let rhs = $cast(rhs);
                let t = self.1.atan2(self.0);
                let r = rhs * Self::exp(rhs * 0.5 * (self.0 * self.0 + self.1 * self.1).ln());
                let (s,c) = t.sin_cos();
                Self(r*c,r*s)
            }
        }
    }
}
macro_rules! impl_c_ops {
    ($t:ty, $rhs:ty, $cast:expr) => {
        impl std::ops::Add<$rhs> for $t {
            type Output = Self;
            fn add(self, rhs: $rhs) -> Self::Output {
                Self(self.0 + $cast(rhs), self.1)
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
        impl std::ops::SubAssign<$rhs> for $t {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.0 -= $cast(rhs);
            }
        }
        impl std::ops::Mul<$rhs> for $t {
            type Output = Self;
            fn mul(self, rhs: $rhs) -> Self::Output {
                Self(self.0 * $cast(rhs), self.1)
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
                Self(self.0 / $cast(rhs), self.1)
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
    ($ty:ty, $other:ty, $( ($variant:ident, $cast:expr) ),* ) => {
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
    ($ty:ty, $other:ty) => {
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
impl_self_c_ops!(CDecimal);
impl_self_c_ops!(CF64);
impl_self_c_ops!(CF32);
impl_c_ops!(CDecimal, f64, |x| x);
impl_c_ops!(CDecimal, f32, |x| x);
impl_c_ops!(CDecimal, i32, |x| x);
impl_c_ops!(CF64, f64, |x| x);
impl_c_ops!(CF64, f32, |x| x as f64);
impl_c_ops!(CF64, i32, |x| x as f64);
impl_c_ops!(CF32, f64, |x| x as f32);
impl_c_ops!(CF32, f32, |x| x);
impl_c_ops!(CF32, i32, |x| x as f32);
impl_self_ops!(Decimal, D512, D256);
impl_self_ops!(Complex, Rug, Fastnum, F64, F32);
impl_self_ops!(Float, Rug, Fastnum, F64, F32);
impl_self_ops!(Integer, Rug, Fastnum, F64, F32);
impl_ops!(Decimal, f64, (D512, |x|x),(D256, |x|x));
impl_ops!(Decimal, f32, (D512, |x|x),(D256, |x|x));
impl_ops!(Decimal, i32, (D512, |x|x),(D256, |x|x));
impl_pow!(CDecimal, f32, |x| x);
impl_pow!(CF64, f32, |x| x as f64);
impl_pow!(CF32, f32, |x| x);
impl_pow!(CDecimal, f64, |x| x);
impl_pow!(CF64, f64, |x| x as f64);
impl_pow!(CF32, f64, |x| x);
impl_pow!(CDecimal, i32, |x| x);
impl_pow!(CF64, i32, |x| x as i32);
impl_pow!(CF32, i32, |x| x as i32);
impl_ops!(Complex, f64,(Rug,|x|x), (Fastnum,|x|x), (F64,|x|x), (F32,|x|x as f32));
impl_ops!(Complex, f32,(Rug,|x|x), (Fastnum,|x|x), (F64,|x|x as f64), (F32,|x|x));
impl_ops!(Complex, i32,(Rug,|x|x), (Fastnum,|x|x), (F64,|x|x as f64), (F32,|x|x as f32));
impl_ops!(Float, f64,(Rug,|x|x), (Fastnum,|x|x), (F64,|x|x), (F32,|x|x as f32));
impl_ops!(Float, f32,(Rug,|x|x), (Fastnum,|x|x), (F64,|x|x as f64), (F32,|x|x));
impl_ops!(Float, i32,(Rug,|x|x), (Fastnum,|x|x), (F64,|x|x as f64), (F32,|x|x as f32));
impl_int_ops!(Integer, i32);