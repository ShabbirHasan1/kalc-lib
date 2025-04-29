use rug::ops::Pow as RugPow;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
//TODO malachite num maybe
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
//maybe box some things to avoid memory bads
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
    fn new(obj: Type, prec: u32) -> Self;
}
pub trait WithValDeci<T> {
    fn with_val(prec: u32, val: T) -> Self;
    fn new(prec: u32) -> Self;
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
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x)
);

impl_pow!(
    Decimal,
    i32,
    (D512, fastnum::D512::from),
    (D256, fastnum::D256::from)
);
impl_pow!(
    Decimal,
    f32,
    (D512, fastnum::D512::from),
    (D256, fastnum::D256::from)
);
impl_pow!(
    Decimal,
    f64,
    (D512, fastnum::D512::from),
    (D256, fastnum::D256::from)
);

impl_pow!(
    Complex,
    f32,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x)
);

impl_pow!(
    Complex,
    i32,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x)
);

impl_pow!(
    Float,
    f32,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x as f64),
    (F32, |x| x)
);

impl_pow!(
    Float,
    i32,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x)
);

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

impl_with_val_deci!(Decimal, f64);
impl_with_val_deci!(Decimal, f32);
impl_with_val_deci!(Decimal, i32);
macro_rules! impl_with_val_cdeci {
    ($ty:ty, $other:ty) => {
        impl WithValDeci<$other> for $ty {
            fn with_val(prec: u32, rhs: $other) -> Self {
                CDecimal(Decimal::with_val(prec, rhs), Decimal::with_val(prec, 0))
            }
            fn new(prec: u32) -> Self {
                CDecimal(Decimal::with_val(prec, 0), Decimal::with_val(prec, 0))
            }
        }
    };
}
impl_with_val_cdeci!(CDecimal, f64);
impl_with_val_cdeci!(CDecimal, f32);
impl_with_val_cdeci!(CDecimal, i32);

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
impl_with_val!(
    Complex,
    f64,
    (Rug, rug::Complex::with_val),
    (Fastnum, CDecimal::with_val),
    (F64, |_, x| CF64(x, 0.0)),
    (F32, |_, x| CF32(x as f32, 0.0))
);
impl_with_val!(
    Float,
    f64,
    (Rug, rug::Float::with_val),
    (Fastnum, Decimal::with_val),
    (F64, |_, x| x),
    (F32, |_, x| x as f32)
);

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
impl_partial_ord!(Float, Rug, Fastnum, F64, F32);
impl_partial_ord!(Decimal, D512, D256);
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

            /*pub fn round(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.round()),)*
                }
            }*/

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

            /*pub fn trunc(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.trunc()),)*
                }
            }

            pub fn fract(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.fract()),)*
                }
            }*/

            pub fn sin_cos(self) -> (Self, Self) {
                match self {
                    $(Self::$variant(a) => {
                        let (s, c) = a.sin_cos();
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
dec_impl!(Decimal, D512, D256);
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
macro_rules! impl_pow {
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
impl_ops!(Decimal, f64, (D512, |x| x), (D256, |x| x));
impl_ops!(Decimal, f32, (D512, |x| x), (D256, |x| x));
impl_ops!(Decimal, i32, (D512, |x| x), (D256, |x| x));
impl_pow!(CDecimal, f32, |x| x);
impl_pow!(CF64, f32, |x| x as f64);
impl_pow!(CF32, f32, |x| x);
impl_pow!(CDecimal, f64, |x| x);
impl_pow!(CF64, f64, |x| x);
impl_pow!(CF32, f64, |x| x as f32);
impl_pow!(CDecimal, i32, |x| x as f64);
impl_pow!(CF64, i32, |x| x as f64);
impl_pow!(CF32, i32, |x| x as f32);
impl_ops!(
    Complex,
    f64,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x as f32)
);
impl_ops!(
    Complex,
    f32,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x as f64),
    (F32, |x| x)
);
impl_ops!(
    Complex,
    i32,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x as f64),
    (F32, |x| x as f32)
);
impl_ops!(
    Float,
    f64,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x),
    (F32, |x| x as f32)
);
impl_ops!(
    Float,
    f32,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x as f64),
    (F32, |x| x)
);
impl_ops!(
    Float,
    i32,
    (Rug, |x| x),
    (Fastnum, |x| x),
    (F64, |x| x as f64),
    (F32, |x| x as f32)
);
impl_int_ops!(Integer, i32);
