use crate::types;
use crate::types::{
    Complex as Comp, Constant, Float as Flo, FloatShared as FloSha, IsPrime, Pow, WithVal,
    WithValImag,
};
use std::cmp::Ordering;
use std::f64::consts::{E, PI, TAU};
use std::fmt::{Display, Formatter, LowerExp};
use std::iter::Sum;
use std::ops::*;
#[derive(Clone, Copy, Default)]
pub struct Integer(pub i128);
#[derive(Clone, Copy, Default)]
pub struct Float(pub f64);
#[derive(Clone, Copy, Default)]
pub struct Complex {
    pub real: Float,
    pub imag: Float,
}
impl LowerExp for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:e}{}{:e}i",
            self.real,
            if self.imag.is_sign_positive() {
                "+"
            } else {
                ""
            },
            self.imag
        )
    }
}
impl LowerExp for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}", self.0)
    }
}
impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}i",
            self.real,
            if self.imag.is_sign_positive() {
                "+"
            } else {
                ""
            },
            self.imag
        )
    }
}
impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
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
impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
macro_rules! with_val {
    ($($ty:ty),*) => {
        $(
            impl WithVal<$ty> for Float
            {
                fn with_val(_: u32, val: $ty) -> Self {
                    Self(val as f64)
                }
            }
            impl From<$ty> for Integer
            {
                fn from(value: $ty) -> Self {
                    Self(value as i128)
                }
            }
            impl<'a> WithVal<&'a $ty> for Float
            {
                fn with_val(_: u32, val: &'a $ty) -> Self {
                    Self(*val as f64)
                }
            }
            impl<'a> From<&'a $ty> for Integer
            {
                fn from(value: &'a $ty) -> Self {
                    Self(*value as i128)
                }
            }
        )*
    };
}
with_val!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);
impl WithVal<bool> for Float {
    fn with_val(_: u32, val: bool) -> Self {
        Self(if val { 1.0 } else { 0.0 })
    }
}
impl WithVal<Float> for Float {
    fn with_val(_: u32, val: Float) -> Self {
        val
    }
}
impl WithVal<&Float> for Float {
    fn with_val(_: u32, val: &Float) -> Self {
        *val
    }
}
impl WithVal<Complex> for Complex {
    fn with_val(_: u32, val: Complex) -> Self {
        val
    }
}
impl WithVal<&Complex> for Complex {
    fn with_val(_: u32, val: &Complex) -> Self {
        *val
    }
}
impl From<&Integer> for Integer {
    fn from(value: &Integer) -> Self {
        *value
    }
}
impl WithVal<Integer> for Float {
    fn with_val(_: u32, val: Integer) -> Self {
        Self(val.0 as f64)
    }
}
impl WithVal<&Integer> for Float {
    fn with_val(_: u32, val: &Integer) -> Self {
        Self(val.0 as f64)
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
impl<T> WithVal<T> for Complex
where
    Float: WithVal<T>,
{
    fn with_val(prec: u32, val: T) -> Self {
        Self {
            real: Float::with_val(prec, val),
            imag: Float::default(),
        }
    }
}
impl<T, K> WithVal<(T, K)> for Complex
where
    Float: WithVal<T> + WithVal<K>,
{
    fn with_val(prec: u32, val: (T, K)) -> Self {
        Self {
            real: Float::with_val(prec, val.0),
            imag: Float::with_val(prec, val.1),
        }
    }
}
impl<T> WithValImag<T> for Complex
where
    Float: WithVal<T>,
{
    fn with_val_imag(prec: u32, val: T) -> Self {
        Self {
            real: Float::default(),
            imag: Float::with_val(prec, val),
        }
    }
}
impl Pow<u32> for Integer {
    fn pow(self, rhs: u32) -> Self {
        (*self).pow(rhs).into()
    }
}
impl<T> Pow<T> for Float
where
    Self: WithVal<T>,
{
    fn pow(self, rhs: T) -> Self {
        (*self).powf(*Float::with_val(0, rhs)).into()
    }
}
impl<T> Pow<T> for Complex
where
    Self: WithVal<T>,
{
    fn pow(self, rhs: T) -> Self {
        let rhs = Self::with_val(0, rhs);
        let arg = self.arg().real;
        let hypot = Float(self.real.0.hypot(self.imag.0));
        let mag = hypot.pow(rhs.real) * (-rhs.imag * arg).exp();
        let dir = rhs.imag * hypot.ln() + rhs.real * arg;
        let (imag, real) = dir.sin_cos(Float::default());
        mag * Self { real, imag }
    }
}
impl<T> PartialEq<T> for Integer
where
    Self: for<'a> From<&'a T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(&Self::from(other).0)
    }
}
impl<T> PartialOrd<T> for Integer
where
    Self: for<'a> From<&'a T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(&Self::from(other).0)
    }
}
impl<T> PartialEq<T> for Float
where
    Self: for<'a> WithVal<&'a T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == Self::with_val(0, other).0
    }
}
impl<T> PartialOrd<T> for Float
where
    Self: for<'a> WithVal<&'a T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(&Self::with_val(0, other).0)
    }
}
impl<T> PartialEq<T> for Complex
where
    Float: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.real == *other && self.imag.0 == 0.0
    }
}
impl<T, K> PartialEq<(T, K)> for Complex
where
    Float: PartialEq<T> + PartialEq<K>,
{
    fn eq(&self, other: &(T, K)) -> bool {
        self.real == other.0 && self.imag == other.1
    }
}
impl PartialEq<Complex> for Complex {
    fn eq(&self, other: &Complex) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}
impl<T> Add<T> for Integer
where
    Integer: From<T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + Self::from(rhs).0)
    }
}
impl<T> Sub<T> for Integer
where
    Integer: From<T>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - Self::from(rhs).0)
    }
}
impl<T> Mul<T> for Integer
where
    Integer: From<T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * Self::from(rhs).0)
    }
}
impl<T> Div<T> for Integer
where
    Integer: From<T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / Self::from(rhs).0)
    }
}
impl Neg for Integer {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl<T> Add<T> for Float
where
    Float: WithVal<T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + Self::with_val(0, rhs).0)
    }
}
impl<T> Sub<T> for Float
where
    Float: WithVal<T>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - Self::with_val(0, rhs).0)
    }
}
impl<T> Mul<T> for Float
where
    Float: WithVal<T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * Self::with_val(0, rhs).0)
    }
}
impl<T> Div<T> for Float
where
    Float: WithVal<T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / Self::with_val(0, rhs).0)
    }
}
impl<T> Rem<T> for Float
where
    Float: WithVal<T>,
{
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        Self(self.0 % Self::with_val(0, rhs).0)
    }
}
impl Neg for Float {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl<T> Add<T> for Complex
where
    Self: WithVal<T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = Self::with_val(0, rhs);
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}
impl<T> Sub<T> for Complex
where
    Self: WithVal<T>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = Self::with_val(0, rhs);
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}
impl<T> Mul<T> for Complex
where
    Self: WithVal<T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = Self::with_val(0, rhs);
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}
impl<T> Div<T> for Complex
where
    Self: WithVal<T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = Self::with_val(0, rhs);
        let sqr = rhs.real * rhs.real + rhs.imag * rhs.imag;
        Self {
            real: (self.real * rhs.real + self.imag * rhs.imag) / sqr,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / sqr,
        }
    }
}
impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
macro_rules! ops_assign {
    ($($ty:ty),*) => {
        $(
            impl<T> AddAssign<T> for $ty
            where $ty: Add<T, Output = $ty>
            {
                fn add_assign(&mut self, rhs: T) {
                    *self = *self + rhs;
                }
            }
            impl<T> SubAssign<T> for $ty
            where $ty: Sub<T, Output = $ty>
            {
                fn sub_assign(&mut self, rhs: T) {
                    *self = *self - rhs;
                }
            }
            impl<T> MulAssign<T> for $ty
            where $ty: Mul<T, Output = $ty>
            {
                fn mul_assign(&mut self, rhs: T) {
                    *self = *self * rhs;
                }
            }
            impl<T> DivAssign<T> for $ty
            where $ty: Div<T, Output = $ty>
            {
                fn div_assign(&mut self, rhs: T) {
                    *self = *self / rhs;
                }
            }
            impl<T> RemAssign<T> for $ty
            where $ty: Rem<T, Output = $ty>
            {
                fn rem_assign(&mut self, rhs: T) {
                    *self = *self % rhs;
                }
            }
        )*
    };
}
ops_assign!(Integer, Float, Complex);
impl From<Float> for Complex {
    fn from(value: Float) -> Self {
        Self {
            real: value,
            imag: Float::default(),
        }
    }
}
macro_rules! shift {
    ($t:ty, $($ty:ty),*) => {
        $(
            #[allow(clippy::suspicious_arithmetic_impl)]
            impl Shl<$ty> for $t {
                type Output = Self;
                fn shl(self, value: $ty) -> Self::Output {
                    self * 2.0f64.powi(value as i32)
                }
            }
            #[allow(clippy::suspicious_arithmetic_impl)]
            impl Shr<$ty> for $t {
                type Output = Self;
                fn shr(self, value: $ty) -> Self::Output {
                    self * 2.0f64.powi(-(value as i32))
                }
            }
        )*
    };
}
shift!(Float, u32, i32);
shift!(Complex, u32, i32);
impl<'a> Sum<&'a Self> for Integer {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self(i128::sum(iter.map(|i| i.0)))
    }
}
impl Add<Complex> for Integer {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        rhs + self
    }
}
impl Sub<Complex> for Integer {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        -rhs + self
    }
}
impl Mul<Complex> for Integer {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        rhs * self
    }
}
impl Div<Complex> for Integer {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Self::Output {
        Complex::with_val(0, self) / rhs
    }
}
impl Add<Float> for Integer {
    type Output = Float;
    fn add(self, rhs: Float) -> Self::Output {
        rhs + self
    }
}
impl Sub<Float> for Integer {
    type Output = Float;
    fn sub(self, rhs: Float) -> Self::Output {
        -rhs + self
    }
}
impl Mul<Float> for Integer {
    type Output = Float;
    fn mul(self, rhs: Float) -> Self::Output {
        rhs * self
    }
}
impl Div<Float> for Integer {
    type Output = Float;
    fn div(self, rhs: Float) -> Self::Output {
        Float::with_val(0, self) / rhs
    }
}
impl Add<Complex> for Float {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        rhs + self
    }
}
impl Sub<Complex> for Float {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        -rhs + self
    }
}
impl Mul<Complex> for Float {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        rhs * self
    }
}
impl Div<Complex> for Float {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Self::Output {
        Complex::with_val(0, self) / rhs
    }
}
impl types::Integer<Float, Complex> for Integer {
    fn cmp0(&self) -> Ordering {
        todo!()
    }
    fn div_rem(self, rhs: Self) -> (Self, Self) {
        todo!()
    }
    fn next_prime(self) -> Self {
        todo!()
    }
    fn binomial(self, k: u32) -> Self {
        todo!()
    }
    fn to_u32(&self) -> Option<u32> {
        todo!()
    }
    fn to_usize(&self) -> Option<usize> {
        todo!()
    }
    fn to_u128(&self) -> Option<u128> {
        todo!()
    }
    fn to_i32(&self) -> Option<i32> {
        todo!()
    }
    fn to_isize(&self) -> Option<isize> {
        todo!()
    }
    fn to_i128(&self) -> Option<i128> {
        todo!()
    }
    fn to_string_radix(&self, radix: i32) -> String {
        todo!()
    }
    fn from_str_radix(src: &str, radix: i32) -> Option<Self> {
        todo!()
    }
    fn is_probably_prime(&self, reps: u32) -> IsPrime {
        todo!()
    }
    fn abs(self) -> Self {
        todo!()
    }
    fn new() -> Self {
        todo!()
    }
}
impl types::Float<Integer, Complex> for Float {
    fn erf(self) -> Self {
        todo!()
    }
    fn ai(self) -> Self {
        todo!()
    }
    fn digamma(self) -> Self {
        todo!()
    }
    fn zeta(self) -> Self {
        todo!()
    }
    fn next_up(&mut self) {
        todo!()
    }
    fn next_down(&mut self) {
        todo!()
    }
    fn next_toward(&mut self, to: &Self) {
        todo!()
    }
    fn erfc(self) -> Self {
        todo!()
    }
    fn cmp0(&self) -> Option<Ordering> {
        todo!()
    }
    fn is_nan(&self) -> bool {
        todo!()
    }
    fn is_finite(&self) -> bool {
        todo!()
    }
    fn is_infinite(&self) -> bool {
        todo!()
    }
    fn is_sign_negative(&self) -> bool {
        todo!()
    }
    fn is_sign_positive(&self) -> bool {
        todo!()
    }
    fn to_f64(&self) -> f64 {
        todo!()
    }
    fn fract(self) -> Self {
        todo!()
    }
    fn trunc(self) -> Self {
        todo!()
    }
    fn round(self) -> Self {
        todo!()
    }
    fn gamma(self) -> Self {
        todo!()
    }
    fn floor(self) -> Self {
        todo!()
    }
    fn ceil(self) -> Self {
        todo!()
    }
    fn to_sign_string_exp(
        &self,
        radix: i32,
        num_digits: Option<usize>,
    ) -> (bool, String, Option<i32>) {
        todo!()
    }
    fn to_string_radix(&self, radix: i32, num_digits: Option<usize>) -> String {
        todo!()
    }
    fn to_integer(&self) -> Option<Integer> {
        todo!()
    }
    fn log2(self) -> Self {
        todo!()
    }
}
impl types::FloatShared<Integer, Self, Complex> for Float {
    fn sin_cos(self, cos: Self) -> (Self, Self) {
        todo!()
    }
    fn sin(self) -> Self {
        todo!()
    }
    fn cos(self) -> Self {
        todo!()
    }
    fn tan(self) -> Self {
        todo!()
    }
    fn sinh(self) -> Self {
        todo!()
    }
    fn cosh(self) -> Self {
        todo!()
    }
    fn tanh(self) -> Self {
        todo!()
    }
    fn asin(self) -> Self {
        todo!()
    }
    fn acos(self) -> Self {
        todo!()
    }
    fn atan(self) -> Self {
        todo!()
    }
    fn asinh(self) -> Self {
        todo!()
    }
    fn acosh(self) -> Self {
        todo!()
    }
    fn atanh(self) -> Self {
        todo!()
    }
    fn parse_radix(prec: u32, src: impl AsRef<[u8]>, radix: i32) -> Option<Self> {
        todo!()
    }
    fn parse(prec: u32, src: impl AsRef<[u8]>) -> Option<Self> {
        todo!()
    }
    fn exp(self) -> Self {
        todo!()
    }
    fn new(prec: u32) -> Self {
        todo!()
    }
    fn is_zero(&self) -> bool {
        todo!()
    }
    fn abs(self) -> Self {
        todo!()
    }
    fn recip(self) -> Self {
        todo!()
    }
    fn prec(&self) -> u32 {
        todo!()
    }
    fn ln(self) -> Self {
        todo!()
    }
    fn log10(self) -> Self {
        todo!()
    }
    fn sqrt(self) -> Self {
        todo!()
    }
    fn set_prec(&mut self, prec: u32) {
        todo!()
    }
}
impl types::Complex<Integer, Float> for Complex {
    fn total_cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
    fn real(&self) -> &Float {
        todo!()
    }
    fn imag(&self) -> &Float {
        todo!()
    }
    fn into_real_imag(self) -> (Float, Float) {
        todo!()
    }
    fn conj(self) -> Self {
        todo!()
    }
    fn arg(self) -> Self {
        todo!()
    }
    fn mul_i(self, negative: bool) -> Self {
        todo!()
    }
}
impl types::FloatShared<Integer, Float, Self> for Complex {
    fn sin_cos(self, cos: Self) -> (Self, Self) {
        todo!()
    }
    fn sin(self) -> Self {
        todo!()
    }
    fn cos(self) -> Self {
        todo!()
    }
    fn tan(self) -> Self {
        todo!()
    }
    fn sinh(self) -> Self {
        todo!()
    }
    fn cosh(self) -> Self {
        todo!()
    }
    fn tanh(self) -> Self {
        todo!()
    }
    fn asin(self) -> Self {
        todo!()
    }
    fn acos(self) -> Self {
        todo!()
    }
    fn atan(self) -> Self {
        todo!()
    }
    fn asinh(self) -> Self {
        todo!()
    }
    fn acosh(self) -> Self {
        todo!()
    }
    fn atanh(self) -> Self {
        todo!()
    }
    fn parse_radix(prec: u32, src: impl AsRef<[u8]>, radix: i32) -> Option<Self> {
        todo!()
    }
    fn parse(prec: u32, src: impl AsRef<[u8]>) -> Option<Self> {
        todo!()
    }
    fn exp(self) -> Self {
        todo!()
    }
    fn new(prec: u32) -> Self {
        todo!()
    }
    fn is_zero(&self) -> bool {
        todo!()
    }
    fn abs(self) -> Self {
        todo!()
    }
    fn recip(self) -> Self {
        todo!()
    }
    fn prec(&self) -> u32 {
        todo!()
    }
    fn ln(self) -> Self {
        todo!()
    }
    fn log10(self) -> Self {
        todo!()
    }
    fn sqrt(self) -> Self {
        todo!()
    }
    fn set_prec(&mut self, prec: u32) {
        todo!()
    }
}
