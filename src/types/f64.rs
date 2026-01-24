use crate::types::{Constant, Pow, WithVal, WithValImag};
use std::cmp::Ordering;
use std::f64::consts::{E, PI, TAU};
use std::ops::*;
use crate::types;
#[derive(Clone, Copy)]
pub struct Complex {
    pub real: Float,
    pub imag: Float,
}
#[derive(Clone, Copy)]
pub struct Integer(pub i128);
impl Deref for Integer {
    type Target = i128;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Integer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Clone, Copy)]
pub struct Float(pub f64);
impl Deref for Float {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
macro_rules! with_val {
    ($ty:ty, $($v:ty),*) => {
        $(
            #[allow(unused_variables)]
            impl WithVal<$v> for $ty {
                fn with_val(prec: u32, val: $v) -> Self {
                    Self(val as f64)
                }
            }
        )*
    };
}
macro_rules! with_val_real {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl WithVal<$v> for $ty {
                fn with_val(prec: u32, val: $v) -> Self {
                    Self {
                        real: Float::with_val(prec, val),
                        imag: Float::with_val(prec, 0.0)
                    }
                }
            }
        )*
    };
}
macro_rules! with_val_complex {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl WithVal<$v> for $ty {
                fn with_val(prec: u32, val: $v) -> Self {
                    Self {
                        real: Float::with_val(prec, val.0),
                        imag: Float::with_val(prec, val.1),
                    }
                }
            }
        )*
    };
}
macro_rules! with_val_imag {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl WithValImag<$v> for $ty {
                fn with_val_imag(prec: u32, val: $v) -> Self {
                    Self {
                        //TODO new
                        real: Float::with_val(prec, 0.0),
                        imag: Float::with_val(prec, val),
                    }
                }
            }
        )*
    };
}
macro_rules! pow {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl Pow<$v> for $ty {
                fn pow(self, val: $v) -> Self {
                    Self(self.powf(*Self::with_val(0, val)))
                }
            }
        )*
    };
}
macro_rules! pow_complex {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl Pow<$v> for $ty {
                fn pow(self, val: $v) -> Self {
                    let mag = self.hypot().pow(val);
                    let dir = Float::with_val(0, val) * self.arg();
                    let (imag,real) = dir.sin_cos();
                    mag * Self{
                        real,imag
                    }
                }
            }
        )*
    };
}
macro_rules! eq {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl PartialEq<$v> for $ty {
                fn eq(&self, val: &$v) -> bool {
                    self.0 == Self::with_val(0, *val).0
                }
            }
        )*
    };
}
macro_rules! ord {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl PartialOrd<$v> for $ty {
                fn partial_cmp(&self, val: &$v) -> Option<Ordering> {
                    self.0.partial_cmp(&Self::with_val(0, *val).0)
                }
            }
        )*
    };
}
macro_rules! eq_complex {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl PartialEq<$v> for $ty {
                fn eq(&self, val: &$v) -> bool {
                    self.imag == 0 && self.real == Float::with_val(0, *val)
                }
            }
        )*
    };
}
macro_rules! from {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl From<$v> for $ty {
                fn from(val: $v) -> Self {
                    Self(val as i128)
                }
            }
        )*
    };
}
macro_rules! eq_int {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl PartialEq<$v> for $ty {
                fn eq(&self, val: &$v) -> bool {
                    self.0 == Self::from(*val).0
                }
            }
        )*
    };
}
macro_rules! ord_int {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl PartialOrd<$v> for $ty {
                fn partial_cmp(&self, val: &$v) -> Option<Ordering> {
                    self.0.partial_cmp(&Self::from(*val).0)
                }
            }
        )*
    };
}
impl PartialEq<Self> for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}
from!(Integer, usize, isize, i32, u32, i128, u128, f64);
eq!(Float, usize, isize, i32, u32, i128, u128, f64, Self);
ord!(Float, usize, isize, i32, u32, i128, u128, f64, Self);
ord_int!(Integer, usize, isize, i32, u32, i128, u128, f64, Self);
eq_int!(Integer, usize, isize, i32, u32, i128, u128, f64, Self);
eq_complex!(Complex, usize, isize, i32, u32, i128, u128, f64);
impl Complex {
    pub fn abs_sqr(self) -> Float {
        self.real * self.real + self.imag * self.imag
    }
    pub fn hypot(self) -> Float {
        self.real.hypot(self.imag)
    }
    pub fn arg(self) -> Float {
        self.imag.atan2(self.real)
    }
}
impl Float {
    pub fn hypot(self, other: Self) -> Self {
        (*self).hypot(*other).into()
    }
    pub fn cos(self) -> Self {
        (*self).cos().into()
    }
    pub fn sin(self) -> Self {
        (*self).sin().into()
    }
    pub fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = (*self).sin_cos();
        (sin.into(), cos.into())
    }
    pub fn atan2(self, other: Self) -> Self {
        (*self).atan2(*other).into()
    }
}
impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self::with_val(0, value)
    }
}
impl Mul<Float> for Float {
    type Output = Float;
    fn mul(self, rhs: Float) -> Self::Output {
        Self::with_val(0, *self * *rhs)
    }
}
impl MulAssign<Float> for Float {
    fn mul_assign(&mut self, rhs: Float) {
        self.0 *= rhs.0;
    }
}
impl Add<Float> for Float {
    type Output = Float;
    fn add(self, rhs: Float) -> Self::Output {
        Self::with_val(0, *self + *rhs)
    }
}
impl AddAssign<Float> for Float {
    fn add_assign(&mut self, rhs: Float) {
        self.0 += rhs.0;
    }
}
impl Div<Float> for Float {
    type Output = Float;
    fn div(self, rhs: Float) -> Self::Output {
        Self::with_val(0, *self / *rhs)
    }
}
impl DivAssign<Float> for Float {
    fn div_assign(&mut self, rhs: Float) {
        self.0 /= rhs.0;
    }
}
impl Sub<Float> for Float {
    type Output = Float;
    fn sub(self, rhs: Float) -> Self::Output {
        Self::with_val(0, *self - *rhs)
    }
}
impl SubAssign<Float> for Float {
    fn sub_assign(&mut self, rhs: Float) {
        self.0 -= rhs.0;
    }
}
impl Mul<Float> for Complex {
    type Output = Complex;
    fn mul(mut self, rhs: Float) -> Self::Output {
        self *= rhs;
        self
    }
}
impl MulAssign<Float> for Complex {
    fn mul_assign(&mut self, rhs: Float) {
        self.real *= rhs;
        self.imag *= rhs;
    }
}
impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(mut self, rhs: Complex) -> Self::Output {
        self *= rhs;
        self
    }
}
impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        let real = self.real * rhs.real - self.imag * rhs.imag;
        let imag = self.real * rhs.imag + self.imag * rhs.real;
        self.real = real;
        self.imag = imag;
    }
}
impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(mut self, rhs: Complex) -> Self::Output {
        self += rhs;
        self
    }
}
impl AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}
impl Sub<Complex> for Complex {
    type Output = Complex;
    fn sub(mut self, rhs: Complex) -> Self::Output {
        self -= rhs;
        self
    }
}
impl SubAssign<Complex> for Complex {
    fn sub_assign(&mut self, rhs: Complex) {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}
impl Div<Complex> for Complex {
    type Output = Complex;
    fn div(mut self, rhs: Complex) -> Self::Output {
        self /= rhs;
        self
    }
}
impl DivAssign<Complex> for Complex {
    fn div_assign(&mut self, rhs: Complex) {
        let r = rhs.abs_sqr();
        let real = self.real * rhs.real + self.imag * rhs.imag;
        let imag = self.imag * rhs.real - self.real * rhs.imag;
        self.real = real / r;
        self.imag = imag / r;
    }
}
impl Neg for Complex {
    type Output = Complex;
    fn neg(mut self) -> Self::Output {
        self.real = -self.real;
        self.imag = -self.imag;
        self
    }
}
impl Neg for Float {
    type Output = Float;
    fn neg(mut self) -> Self::Output {
        self.0 = -self.0;
        self
    }
}
impl Neg for Integer {
    type Output = Integer;
    fn neg(mut self) -> Self::Output {
        self.0 = -self.0;
        self
    }
}
impl Mul<Complex> for Float {
    type Output = Complex;
    fn mul(self, mut rhs: Complex) -> Self::Output {
        rhs *= self;
        rhs
    }
}
pow_complex!(Complex, f64, usize, isize, u32, i32, Float);
pow!(Float, f64, usize, isize, u32, i32, Self);
impl Pow<u32> for Integer {
    fn pow(self, rhs: u32) -> Self {
        Self((*self).pow(rhs))
    }
}
impl Pow<Self> for Complex {
    fn pow(self, _rhs: Self) -> Self {
        todo!()
    }
}
with_val_imag!(
    Complex, f64, i32, usize, isize, i128, u128, u32, Integer, Float, &Float
);
with_val_real!(
    Complex, f64, i32, usize, isize, i128, u128, u32, bool, Integer, &Integer, Float, &Float
);
with_val_complex!(
    Complex,
    (f64, f64),
    (i32, i32),
    (u32, u32),
    (usize, usize),
    (isize, isize),
    (i128, i128),
    (u128, u128),
    (Integer, Integer),
    (&Integer, &Integer),
    (Float, Float),
    (&Float, &Float)
);
impl WithVal<Constant> for Complex {
    fn with_val(prec: u32, val: Constant) -> Self {
        match val {
            Constant::Pi => Self::with_val(prec, PI),
            Constant::E => Self::with_val(prec, E),
            Constant::Infinity => Self::with_val(prec, f64::INFINITY),
            Constant::NegInfinity => Self::with_val(prec, f64::NEG_INFINITY),
            Constant::Nan => Self::with_val(prec, f64::NAN),
            Constant::Tau => Self::with_val(prec, TAU),
        }
    }
}
impl WithVal<(Constant, Constant)> for Complex {
    fn with_val(prec: u32, val: (Constant, Constant)) -> Self {
        <Self as WithVal<Constant>>::with_val(prec, val.0) + Self::with_val_imag(prec, val.1)
    }
}
impl WithValImag<Constant> for Complex {
    fn with_val_imag(prec: u32, val: Constant) -> Self {
        match val {
            Constant::Pi => Self::with_val_imag(prec, PI),
            Constant::E => Self::with_val_imag(prec, E),
            Constant::Infinity => Self::with_val_imag(prec, f64::INFINITY),
            Constant::NegInfinity => Self::with_val_imag(prec, f64::NEG_INFINITY),
            Constant::Nan => Self::with_val_imag(prec, f64::NAN),
            Constant::Tau => Self::with_val_imag(prec, TAU),
        }
    }
}
impl WithVal<Constant> for Float {
    fn with_val(prec: u32, val: Constant) -> Self {
        match val {
            Constant::Pi => Self::with_val(prec, PI),
            Constant::E => Self::with_val(prec, E),
            Constant::Infinity => Self::with_val(prec, f64::INFINITY),
            Constant::NegInfinity => Self::with_val(prec, f64::NEG_INFINITY),
            Constant::Nan => Self::with_val(prec, f64::NAN),
            Constant::Tau => Self::with_val(prec, TAU),
        }
    }
}
#[allow(unused_variables)]
impl WithVal<bool> for Float {
    fn with_val(prec: u32, val: bool) -> Self {
        Self((val as u8) as f64)
    }
}
#[allow(unused_variables)]
impl WithVal<Integer> for Float {
    fn with_val(prec: u32, val: Integer) -> Self {
        Self(val.0 as f64)
    }
}
#[allow(unused_variables)]
impl WithVal<&Integer> for Float {
    fn with_val(prec: u32, val: &Integer) -> Self {
        Self(val.0 as f64)
    }
}
#[allow(unused_variables)]
impl WithVal<Float> for Float {
    fn with_val(prec: u32, val: Float) -> Self {
        val
    }
}
#[allow(unused_variables)]
impl WithVal<&Float> for Float {
    fn with_val(prec: u32, val: &Float) -> Self {
        *val
    }
}
with_val!(Float, f64, i32, u32, usize, isize, i128, u128);
