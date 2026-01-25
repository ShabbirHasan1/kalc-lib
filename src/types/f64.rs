use crate::types;
use crate::types::{
    Complex as Comp, Constant, Float as Flo, FloatShared as FloSha, IsPrime, Pow, WithVal,
    WithValImag,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::f64::consts::{E, PI, TAU};
use std::fmt::{Display, Formatter, LowerExp};
use std::iter::Sum;
use std::ops::*;
#[derive(Clone, Copy, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Integer<T>(pub T);
#[derive(Clone, Copy, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Float<T>(pub T);
#[derive(Clone, Copy, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Complex<T> {
    pub real: Float<T>,
    pub imag: Float<T>,
}
impl LowerExp for Complex<f64> {
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
impl LowerExp for Float<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:e}", self.0)
    }
}
impl Display for Complex<f64> {
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
impl Display for Integer<i128> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Display for Float<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for Integer<i128> {
    type Target = i128;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Integer<i128> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Deref for Float<f64> {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Float<f64> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<f64> for Float<f64> {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
macro_rules! with_val {
    ($($ty:ty),*) => {
        $(
            impl WithVal<$ty> for Float<f64>
            {
                fn with_val(_: u32, val: $ty) -> Self {
                    Self(val as f64)
                }
            }
            impl From<$ty> for Integer<i128>
            {
                fn from(value: $ty) -> Self {
                    Self(value as i128)
                }
            }
            impl<'a> WithVal<&'a $ty> for Float<f64>
            {
                fn with_val(_: u32, val: &'a $ty) -> Self {
                    Self(*val as f64)
                }
            }
            impl<'a> From<&'a $ty> for Integer<i128>
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
impl WithVal<bool> for Float<f64> {
    fn with_val(_: u32, val: bool) -> Self {
        Self(if val { 1.0 } else { 0.0 })
    }
}
impl WithVal<Float<f64>> for Float<f64> {
    fn with_val(_: u32, val: Float<f64>) -> Self {
        val
    }
}
impl WithVal<&Float<f64>> for Float<f64> {
    fn with_val(_: u32, val: &Float<f64>) -> Self {
        *val
    }
}
impl WithVal<Complex<f64>> for Complex<f64> {
    fn with_val(_: u32, val: Complex<f64>) -> Self {
        val
    }
}
impl WithVal<&Complex<f64>> for Complex<f64> {
    fn with_val(_: u32, val: &Complex<f64>) -> Self {
        *val
    }
}
impl From<&Integer<i128>> for Integer<i128> {
    fn from(value: &Integer<i128>) -> Self {
        *value
    }
}
impl WithVal<Integer<i128>> for Float<f64> {
    fn with_val(_: u32, val: Integer<i128>) -> Self {
        Self(val.0 as f64)
    }
}
impl WithVal<&Integer<i128>> for Float<f64> {
    fn with_val(_: u32, val: &Integer<i128>) -> Self {
        Self(val.0 as f64)
    }
}
impl WithVal<Constant> for Float<f64> {
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
impl<T> WithVal<T> for Complex<f64>
where
    Float<f64>: WithVal<T>,
{
    fn with_val(prec: u32, val: T) -> Self {
        Self {
            real: Float::with_val(prec, val),
            imag: Float::default(),
        }
    }
}
impl<T, K> WithVal<(T, K)> for Complex<f64>
where
    Float<f64>: WithVal<T> + WithVal<K>,
{
    fn with_val(prec: u32, val: (T, K)) -> Self {
        Self {
            real: Float::with_val(prec, val.0),
            imag: Float::with_val(prec, val.1),
        }
    }
}
impl<T> WithValImag<T> for Complex<f64>
where
    Float<f64>: WithVal<T>,
{
    fn with_val_imag(prec: u32, val: T) -> Self {
        Self {
            real: Float::default(),
            imag: Float::with_val(prec, val),
        }
    }
}
impl Pow<u32> for Integer<i128> {
    fn pow(self, rhs: u32) -> Self {
        (*self).pow(rhs).into()
    }
}
impl<T> Pow<T> for Float<f64>
where
    Self: WithVal<T>,
{
    fn pow(self, rhs: T) -> Self {
        (*self).powf(*Float::with_val(0, rhs)).into()
    }
}
impl<T> Pow<T> for Complex<f64>
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
impl<T> PartialEq<T> for Integer<i128>
where
    Self: for<'a> From<&'a T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(&Self::from(other).0)
    }
}
impl<T> PartialOrd<T> for Integer<i128>
where
    Self: for<'a> From<&'a T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(&Self::from(other).0)
    }
}
impl<T> PartialEq<T> for Float<f64>
where
    Self: for<'a> WithVal<&'a T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == Self::with_val(0, other).0
    }
}
impl<T> PartialOrd<T> for Float<f64>
where
    Self: for<'a> WithVal<&'a T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(&Self::with_val(0, other).0)
    }
}
impl<T> PartialEq<T> for Complex<f64>
where
    Float<f64>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.real == *other && self.imag.0 == 0.0
    }
}
impl<T, K> PartialEq<(T, K)> for Complex<f64>
where
    Float<f64>: PartialEq<T> + PartialEq<K>,
{
    fn eq(&self, other: &(T, K)) -> bool {
        self.real == other.0 && self.imag == other.1
    }
}
impl PartialEq<Complex<f64>> for Complex<f64> {
    fn eq(&self, other: &Complex<f64>) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}
impl<T> Add<T> for Integer<i128>
where
    Integer<i128>: From<T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + Self::from(rhs).0)
    }
}
impl<T> Sub<T> for Integer<i128>
where
    Integer<i128>: From<T>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - Self::from(rhs).0)
    }
}
impl<T> Mul<T> for Integer<i128>
where
    Integer<i128>: From<T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * Self::from(rhs).0)
    }
}
impl<T> Div<T> for Integer<i128>
where
    Integer<i128>: From<T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / Self::from(rhs).0)
    }
}
impl Neg for Integer<i128> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl<T> Add<T> for Float<f64>
where
    Float<f64>: WithVal<T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + Self::with_val(0, rhs).0)
    }
}
impl<T> Sub<T> for Float<f64>
where
    Float<f64>: WithVal<T>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - Self::with_val(0, rhs).0)
    }
}
impl<T> Mul<T> for Float<f64>
where
    Float<f64>: WithVal<T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * Self::with_val(0, rhs).0)
    }
}
impl<T> Div<T> for Float<f64>
where
    Float<f64>: WithVal<T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / Self::with_val(0, rhs).0)
    }
}
impl<T> Rem<T> for Float<f64>
where
    Float<f64>: WithVal<T>,
{
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        Self(self.0 % Self::with_val(0, rhs).0)
    }
}
impl<T> Rem<T> for Integer<i128>
where
    Self: From<T>,
{
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        Self(self.0 % Self::from(rhs).0)
    }
}
impl Neg for Float<f64> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl<T> Add<T> for Complex<f64>
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
impl<T> Sub<T> for Complex<f64>
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
impl<T> Mul<T> for Complex<f64>
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
impl<T> Div<T> for Complex<f64>
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
impl Neg for Complex<f64> {
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
ops_assign!(Integer<i128>, Float<f64>, Complex<f64>);
impl From<Float<f64>> for Complex<f64> {
    fn from(value: Float<f64>) -> Self {
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
shift!(Float<f64>, u32, i32);
shift!(Complex<f64>, u32, i32);
impl<'a> Sum<&'a Self> for Integer<i128> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self(i128::sum(iter.map(|i| i.0)))
    }
}
impl Add<Complex<f64>> for Integer<i128> {
    type Output = Complex<f64>;
    fn add(self, rhs: Complex<f64>) -> Self::Output {
        rhs + self
    }
}
impl Sub<Complex<f64>> for Integer<i128> {
    type Output = Complex<f64>;
    fn sub(self, rhs: Complex<f64>) -> Self::Output {
        -rhs + self
    }
}
impl Mul<Complex<f64>> for Integer<i128> {
    type Output = Complex<f64>;
    fn mul(self, rhs: Complex<f64>) -> Self::Output {
        rhs * self
    }
}
impl Div<Complex<f64>> for Integer<i128> {
    type Output = Complex<f64>;
    fn div(self, rhs: Complex<f64>) -> Self::Output {
        Complex::with_val(0, self) / rhs
    }
}
impl Add<Float<f64>> for Integer<i128> {
    type Output = Float<f64>;
    fn add(self, rhs: Float<f64>) -> Self::Output {
        rhs + self
    }
}
impl Sub<Float<f64>> for Integer<i128> {
    type Output = Float<f64>;
    fn sub(self, rhs: Float<f64>) -> Self::Output {
        -rhs + self
    }
}
impl Mul<Float<f64>> for Integer<i128> {
    type Output = Float<f64>;
    fn mul(self, rhs: Float<f64>) -> Self::Output {
        rhs * self
    }
}
impl Div<Float<f64>> for Integer<i128> {
    type Output = Float<f64>;
    fn div(self, rhs: Float<f64>) -> Self::Output {
        Float::with_val(0, self) / rhs
    }
}
impl Add<Complex<f64>> for Float<f64> {
    type Output = Complex<f64>;
    fn add(self, rhs: Complex<f64>) -> Self::Output {
        rhs + self
    }
}
impl Sub<Complex<f64>> for Float<f64> {
    type Output = Complex<f64>;
    fn sub(self, rhs: Complex<f64>) -> Self::Output {
        -rhs + self
    }
}
impl Mul<Complex<f64>> for Float<f64> {
    type Output = Complex<f64>;
    fn mul(self, rhs: Complex<f64>) -> Self::Output {
        rhs * self
    }
}
impl Div<Complex<f64>> for Float<f64> {
    type Output = Complex<f64>;
    fn div(self, rhs: Complex<f64>) -> Self::Output {
        Complex::with_val(0, self) / rhs
    }
}
impl types::Integer<Float<f64>, Complex<f64>> for Integer<i128> {
    fn cmp0(&self) -> Ordering {
        self.cmp(&0)
    }
    fn div_rem(self, rhs: Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }
    fn next_prime(self) -> Self {
        //TODO
        self + 1
    }
    #[allow(unused_variables)]
    fn binomial(self, k: u32) -> Self {
        //TODO
        self
    }
    fn to_u32(&self) -> Option<u32> {
        Some(self.0 as u32)
    }
    fn to_usize(&self) -> Option<usize> {
        Some(self.0 as usize)
    }
    fn to_u128(&self) -> Option<u128> {
        Some(self.0 as u128)
    }
    fn to_i32(&self) -> Option<i32> {
        Some(self.0 as i32)
    }
    fn to_isize(&self) -> Option<isize> {
        Some(self.0 as isize)
    }
    fn to_i128(&self) -> Option<i128> {
        Some(self.0)
    }
    fn to_string_radix(&self, radix: i32) -> String {
        let radix = radix.into();
        let (mut div, mut rem) = self.div_rem(radix);
        let mut s = rem.to_string();
        while div != 0 {
            (div, rem) = div.div_rem(radix);
            s.insert(0, rem.to_string().chars().next().unwrap());
        }
        s
    }
    fn from_str_radix(src: &str, radix: i32) -> Option<Self> {
        let neg = src.starts_with('-');
        let len = src.len() - if neg { 1 } else { 0 };
        let mut n = Integer::new();
        for (i, d) in src.chars().skip(if neg { 1 } else { 0 }).enumerate() {
            if !d.is_digit(radix as u32) {
                return None;
            }
            n += Integer::from(radix).pow((len - i) as u32) * d.to_string().parse::<i128>().unwrap()
        }
        Some(n)
    }
    #[allow(unused_variables)]
    fn is_probably_prime(&self, reps: u32) -> IsPrime {
        //TODO
        IsPrime::Yes
    }
    fn abs(self) -> Self {
        Self(self.0.abs())
    }
    fn new() -> Self {
        Self::default()
    }
}
impl types::Float<Integer<i128>, Complex<f64>> for Float<f64> {
    fn erf(self) -> Self {
        //TODO
        self
    }
    fn ai(self) -> Self {
        //TODO
        self
    }
    fn digamma(self) -> Self {
        //TODO
        self
    }
    fn zeta(self) -> Self {
        //TODO
        self
    }
    fn next_up(&mut self) {
        self.0 = self.0.next_up();
    }
    fn next_down(&mut self) {
        self.0 = self.0.next_down();
    }
    fn next_toward(&mut self, to: &Self) {
        match self.total_cmp(to) {
            Ordering::Less => self.next_up(),
            Ordering::Equal => {}
            Ordering::Greater => self.next_down(),
        }
    }
    fn erfc(self) -> Self {
        //TODO
        self
    }
    fn cmp0(&self) -> Option<Ordering> {
        Some(self.total_cmp(&0.0))
    }
    fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }
    fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
    fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }
    fn to_f64(&self) -> f64 {
        self.0
    }
    fn fract(self) -> Self {
        Self(self.0.fract())
    }
    fn trunc(self) -> Self {
        Self(self.0.trunc())
    }
    fn round(self) -> Self {
        Self(self.0.round())
    }
    fn gamma(self) -> Self {
        //TODO
        self
    }
    fn floor(self) -> Self {
        Self(self.0.floor())
    }
    fn ceil(self) -> Self {
        Self(self.0.ceil())
    }
    fn to_sign_string_exp(
        &self,
        radix: i32,
        num_digits: Option<usize>,
    ) -> (bool, String, Option<i32>) {
        let mut s = self.0.abs().to_string();
        s.retain(|c| c != '.');
        while s.starts_with('0') && s.len() != 1 {
            s.remove(0);
        }
        if let Some(num) = num_digits
            && s.len() > num
        {
            s.drain(s.len() - num - 1..);
        }
        (
            self.is_sign_negative(),
            s,
            if self.is_zero() {
                Some(0)
            } else if self.is_finite() {
                Some((self.abs().ln() / Float::with_val(0, radix).ln()).ceil().0 as i32)
            } else {
                None
            },
        )
    }
    #[allow(unused_variables)]
    fn to_string_radix(&self, radix: i32, num_digits: Option<usize>) -> String {
        //TODO
        self.to_string()
    }
    fn to_integer(&self) -> Option<Integer<i128>> {
        if !self.is_finite() {
            return None;
        }
        Some(Integer(self.0 as i128))
    }
    fn log2(self) -> Self {
        Self(self.0.log2())
    }
}
impl types::FloatShared<Integer<i128>, Self, Complex<f64>> for Float<f64> {
    fn sin_cos(self, _: Self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (Self(sin), Self(cos))
    }
    fn sin(self) -> Self {
        Self(self.0.sin())
    }
    fn cos(self) -> Self {
        Self(self.0.cos())
    }
    fn tan(self) -> Self {
        Self(self.0.tan())
    }
    fn sinh(self) -> Self {
        Self(self.0.sinh())
    }
    fn cosh(self) -> Self {
        Self(self.0.cosh())
    }
    fn tanh(self) -> Self {
        Self(self.0.tanh())
    }
    fn asin(self) -> Self {
        Self(self.0.asin())
    }
    fn acos(self) -> Self {
        Self(self.0.acos())
    }
    fn atan(self) -> Self {
        Self(self.0.atan())
    }
    fn asinh(self) -> Self {
        Self(self.0.asinh())
    }
    fn acosh(self) -> Self {
        Self(self.0.acosh())
    }
    fn atanh(self) -> Self {
        Self(self.0.atanh())
    }
    #[allow(unused_variables)]
    fn parse_radix(_: u32, src: &str, radix: i32) -> Option<Self> {
        //TODO
        src.parse().ok().map(Self)
    }
    fn parse(_: u32, src: &str) -> Option<Self> {
        src.parse().ok().map(Self)
    }
    fn exp(self) -> Self {
        Self(self.0.exp())
    }
    fn new(_: u32) -> Self {
        Self::default()
    }
    fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
    fn abs(self) -> Self {
        Self(self.0.abs())
    }
    fn recip(self) -> Self {
        Self(self.0.recip())
    }
    fn prec(&self) -> u32 {
        32
    }
    fn ln(self) -> Self {
        Self(self.0.ln())
    }
    fn log10(self) -> Self {
        Self(self.0.log10())
    }
    fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
    fn set_prec(&mut self, _: u32) {}
}
impl types::Complex<Integer<i128>, Float<f64>> for Complex<f64> {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.real
            .total_cmp(&other.real)
            .then(self.imag.total_cmp(&other.imag))
    }
    fn real(&self) -> &Float<f64> {
        &self.real
    }
    fn imag(&self) -> &Float<f64> {
        &self.imag
    }
    fn into_real_imag(self) -> (Float<f64>, Float<f64>) {
        (self.real, self.imag)
    }
    fn conj(mut self) -> Self {
        self.imag = -self.imag;
        self
    }
    fn arg(self) -> Self {
        Self {
            real: Float(self.imag.atan2(*self.real)),
            imag: Float::default(),
        }
    }
    fn mul_i(self, negative: bool) -> Self {
        self * if negative { (0, -1) } else { (0, 1) }
    }
}
impl types::FloatShared<Integer<i128>, Float<f64>, Self> for Complex<f64> {
    fn sin_cos(self, _: Self) -> (Self, Self) {
        (self.sin(), self.cos())
    }
    fn sin(self) -> Self {
        //TODO
        Self {
            real: self.real.sin(),
            imag: self.imag,
        }
    }
    fn cos(self) -> Self {
        //TODO
        Self {
            real: self.real.cos(),
            imag: self.imag,
        }
    }
    fn tan(self) -> Self {
        //TODO
        Self {
            real: self.real.tan(),
            imag: self.imag,
        }
    }
    fn sinh(self) -> Self {
        //TODO
        Self {
            real: self.real.sinh(),
            imag: self.imag,
        }
    }
    fn cosh(self) -> Self {
        //TODO
        Self {
            real: self.real.cosh(),
            imag: self.imag,
        }
    }
    fn tanh(self) -> Self {
        //TODO
        Self {
            real: self.real.tanh(),
            imag: self.imag,
        }
    }
    fn asin(self) -> Self {
        //TODO
        Self {
            real: self.real.asin(),
            imag: self.imag,
        }
    }
    fn acos(self) -> Self {
        //TODO
        Self {
            real: self.real.acos(),
            imag: self.imag,
        }
    }
    fn atan(self) -> Self {
        //TODO
        Self {
            real: self.real.atan(),
            imag: self.imag,
        }
    }
    fn asinh(self) -> Self {
        //TODO
        Self {
            real: self.real.asinh(),
            imag: self.imag,
        }
    }
    fn acosh(self) -> Self {
        //TODO
        Self {
            real: self.real.acosh(),
            imag: self.imag,
        }
    }
    fn atanh(self) -> Self {
        //TODO
        Self {
            real: self.real.atanh(),
            imag: self.imag,
        }
    }
    #[allow(unused_variables)]
    fn parse_radix(_: u32, src: &str, radix: i32) -> Option<Self> {
        src.parse().ok().map(|real| Self {
            real: Float(real),
            imag: Float::default(),
        })
    }
    fn parse(_: u32, src: &str) -> Option<Self> {
        src.parse().ok().map(|real| Self {
            real: Float(real),
            imag: Float::default(),
        })
    }
    fn exp(self) -> Self {
        let mag = self.real.exp();
        let (sin, cos) = self.imag.sin_cos(Float::default());
        Self {
            real: cos * mag,
            imag: sin * mag,
        }
    }
    fn new(_: u32) -> Self {
        Self::default()
    }
    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }
    fn abs(self) -> Self {
        Self {
            real: Float(self.real.0.hypot(self.imag.0)),
            imag: Float::default(),
        }
    }
    fn recip(self) -> Self {
        let mag = self.real * self.real + self.imag * self.imag;
        self.conj() / mag
    }
    fn prec(&self) -> u32 {
        32
    }
    fn ln(self) -> Self {
        Self {
            real: self.abs().real.ln(),
            imag: self.arg().real,
        }
    }
    fn log10(self) -> Self {
        self.ln() / Float(10.0).ln()
    }
    fn sqrt(self) -> Self {
        self.pow(0.5)
    }
    fn set_prec(&mut self, _: u32) {}
}
