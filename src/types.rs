use rug::ops::Pow;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
#[derive(Clone, PartialEq)]
pub enum Complex {
    Rug(rug::Complex),
}
#[derive(Clone, PartialEq)]
pub enum Float {
    Rug(rug::Float),
}
#[derive(Clone, PartialEq)]
pub enum Integer {
    Rug(rug::Integer),
}
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    Rug,
}

impl Complex {
    pub fn pow(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a.pow(b)),
        }
    }
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
    pub fn with_val(obj: Type, prec: u32, val: f64) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, val)),
        }
    }
    pub fn with_valt(obj: Type, prec: u32, val: (f64, f64)) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, val)),
        }
    }
    pub fn with_val32(obj: Type, prec: u32, val: f32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, val)),
        }
    }
    pub fn with_vali32(obj: Type, prec: u32, val: i32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, val)),
        }
    }
    pub fn with_valu32(obj: Type, prec: u32, val: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Complex::with_val(prec, val)),
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
impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rug(a) => a.fmt(f),
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
impl Float {
    pub fn pow(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a.pow(b)),
        }
    }
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
    pub fn with_val(obj: Type, prec: u32, val: f64) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, val)),
        }
    }
    pub fn with_val32(obj: Type, prec: u32, val: f32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, val)),
        }
    }
    pub fn with_vali32(obj: Type, prec: u32, val: i32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, val)),
        }
    }
    pub fn with_valu32(obj: Type, prec: u32, val: u32) -> Self {
        match obj {
            Type::Rug => Self::Rug(rug::Float::with_val(prec, val)),
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
impl Integer {
    pub fn is_probably_prime(&self, reps: u32) -> bool {
        match self {
            Self::Rug(a) => a.is_probably_prime(reps) != rug::integer::IsPrime::No,
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

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a + b),
        }
    }
}
impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a += b,
        }
    }
}
impl Add for Float {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a + b),
        }
    }
}
impl AddAssign for Float {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a += b,
        }
    }
}
impl Add for Integer {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a + b),
        }
    }
}
impl AddAssign for Integer {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a += b,
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a - b),
        }
    }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a -= b,
        }
    }
}
impl Sub for Float {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a - b),
        }
    }
}
impl SubAssign for Float {
    fn sub_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a -= b,
        }
    }
}
impl Sub for Integer {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a - b),
        }
    }
}
impl SubAssign for Integer {
    fn sub_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a -= b,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a * b),
        }
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a *= b,
        }
    }
}
impl Mul for Float {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a * b),
        }
    }
}
impl MulAssign for Float {
    fn mul_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a *= b,
        }
    }
}
impl Mul for Integer {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a * b),
        }
    }
}
impl MulAssign for Integer {
    fn mul_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a *= b,
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a / b),
        }
    }
}
impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a /= b,
        }
    }
}
impl Div for Float {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a / b),
        }
    }
}
impl DivAssign for Float {
    fn div_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a /= b,
        }
    }
}
impl Div for Integer {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => Self::Rug(a / b),
        }
    }
}
impl DivAssign for Integer {
    fn div_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (Self::Rug(a), Self::Rug(b)) => *a /= b,
        }
    }
}
