pub mod f64;
pub mod rug;
use std::cmp::Ordering;
use std::fmt::{Display, LowerExp};
use std::iter::Sum;
use std::ops::*;
pub trait Complex<I: Integer<F, Self>, F: Float<I, Self>>:
    FloatShared<I, F, Self>
    + WithVal<(usize, usize)>
    + WithVal<(isize, isize)>
    + WithVal<(i32, i32)>
    + WithVal<(u32, u32)>
    + WithVal<(i128, i128)>
    + WithVal<(u128, u128)>
    + WithVal<(f64, f64)>
    + WithVal<(F, F)>
    + for<'a> WithVal<(&'a F, &'a F)>
    + for<'a> WithVal<(&'a I, &'a I)>
    + WithVal<(I, I)>
    + WithVal<(Constant, Constant)>
    + WithVal<F>
    + for<'a> WithVal<&'a F>
    + for<'a> WithVal<&'a I>
    + WithValImag<usize>
    + WithValImag<i128>
    + WithValImag<u128>
    + WithValImag<isize>
    + WithValImag<f64>
    + WithValImag<i32>
    + WithValImag<u32>
    + WithValImag<I>
    + WithValImag<F>
    + WithValImag<Constant>
    + Equiv
    + Ops<F>
    + for<'a> Ops<&'a F>
    + From<F>
    + Pow<Self>
    + Pow<F>
    + Display
{
    fn total_cmp(&self, other: &Self) -> Ordering;
    fn real(&self) -> &F;
    fn imag(&self) -> &F;
    fn into_real_imag(self) -> (F, F);
    fn conj(self) -> Self;
    fn arg(self) -> Self;
    fn mul_i(self, negative: bool) -> Self;
}
pub trait Float<I: Integer<Self, C>, C: Complex<I, Self>>:
    FloatShared<I, Self, C>
    + PartialOrd<f64>
    + Compare
    + OperatorsOut<C, C>
    + Pow<Self>
    + RemOp<usize>
    + LowerExp
{
    fn erf(self) -> Self;
    fn ai(self) -> Self;
    fn digamma(self) -> Self;
    fn zeta(self) -> Self;
    fn next_up(&mut self);
    fn next_down(&mut self);
    fn next_toward(&mut self, to: &Self);
    fn erfc(self) -> Self;
    fn cmp0(&self) -> Option<Ordering>;
    fn is_nan(&self) -> bool;
    fn is_finite(&self) -> bool;
    fn is_infinite(&self) -> bool;
    fn is_sign_negative(&self) -> bool;
    fn is_sign_positive(&self) -> bool;
    fn to_f64(&self) -> f64;
    fn fract(self) -> Self;
    fn trunc(self) -> Self;
    fn round(self) -> Self;
    fn gamma(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn to_sign_string_exp(
        &self,
        radix: i32,
        num_digits: Option<usize>,
    ) -> (bool, String, Option<i32>);
    fn to_string_radix(&self, radix: i32, num_digits: Option<usize>) -> String;
    fn to_integer(&self) -> Option<I>;
    fn log2(self) -> Self;
}
pub trait FloatShared<I: Integer<F, C>, F: Float<I, C>, C: Complex<I, F>>:
    Shared
    + Pow<usize>
    + Pow<u32>
    + Pow<i32>
    + Pow<isize>
    + Pow<f64>
    + Shr<i32, Output = Self>
    + Shl<i32, Output = Self>
    + Shr<u32, Output = Self>
    + Shl<u32, Output = Self>
    + WithVal<usize>
    + WithVal<i128>
    + WithVal<u128>
    + WithVal<isize>
    + WithVal<f64>
    + WithVal<i32>
    + WithVal<u32>
    + WithVal<bool>
    + WithVal<I>
    + WithVal<Constant>
    + Display
    + Ops<I>
    + Ops<f64>
{
    fn sin_cos(self, cos: Self) -> (Self, Self);
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
    fn parse_radix(prec: u32, src: impl AsRef<[u8]>, radix: i32) -> Option<Self>;
    fn exp(self) -> Self;
    fn new(prec: u32) -> Self;
    fn is_zero(&self) -> bool;
    fn abs(self) -> Self;
    fn recip(self) -> Self;
    fn prec(&self) -> u32;
    fn ln(self) -> Self;
    fn log10(self) -> Self;
    fn sqrt(self) -> Self;
    fn set_prec(&mut self, prec: u32);
}
pub enum Constant {
    Pi,
    Tau,
    E,
    Infinity,
    NegInfinity,
    Nan,
}
pub trait Integer<F: Float<Self, C>, C: Complex<Self, F>>:
    Shared
    + From<usize>
    + From<isize>
    + From<u32>
    + From<i32>
    + From<i32>
    + Pow<u32>
    + OperatorsOutNoDiv<F, F>
    + OperatorsOutNoDiv<C, C>
    + Display
    + Default
    + Compare
    + for<'a> Sum<&'a Self>
{
    fn cmp0(&self) -> Ordering;
    fn div_rem(self, rhs: Self) -> (Self, Self);
    fn next_prime(self) -> Self;
    fn binomial(self, k: u32) -> Self;
    fn to_u32(&self) -> Option<u32>;
    fn to_usize(&self) -> Option<usize>;
    fn to_u128(&self) -> Option<u128>;
    fn to_i32(&self) -> Option<i32>;
    fn to_isize(&self) -> Option<isize>;
    fn to_i128(&self) -> Option<i128>;
    fn to_string_radix(&self, radix: i32) -> String;
    fn from_str_radix(src: &str, radix: i32) -> Option<Self>;
    fn is_probably_prime(&self, reps: u32) -> IsPrime;
    fn abs(self) -> Self;
    fn new() -> Self;
}
#[derive(PartialEq)]
pub enum IsPrime {
    Yes,
    No,
    Probably,
}
pub trait Shared: Operators + Clone + Send + Sync {}
impl<T> Shared for T where T: Operators + Clone + Send + Sync {}
pub trait Operators:
    Ops<Self>
    + Ops<usize>
    + Ops<u128>
    + Ops<i128>
    + Ops<i32>
    + Ops<u32>
    + Ops<i64>
    + Ops<u64>
    + for<'a> Ops<&'a Self>
    + Neg<Output = Self>
    + Sized
{
}
impl<T> Operators for T where
    T: Ops<T>
        + Ops<usize>
        + Ops<u128>
        + Ops<i128>
        + Ops<i32>
        + Ops<u32>
        + Ops<i64>
        + Ops<u64>
        + for<'a> Ops<&'a T>
        + Neg<Output = T>
        + Sized
{
}
pub trait RemOp<T>: Rem<T, Output = Self> + RemAssign<T> {}
impl<K, T> RemOp<T> for K where K: Rem<T, Output = Self> + RemAssign<T> {}
pub trait OperatorsOut<T, K>: OperatorsOutNoDiv<T, K> + Div<T, Output = K> {}
impl<K, T, V> OperatorsOut<T, V> for K where K: OperatorsOutNoDiv<T, V> + Div<T, Output = V> {}
pub trait OperatorsOutNoDiv<T, K>:
    Mul<T, Output = K> + Add<T, Output = K> + Sub<T, Output = K>
{
}
impl<K, T, V> OperatorsOutNoDiv<T, V> for K where
    K: Mul<T, Output = V> + Add<T, Output = V> + Sub<T, Output = V>
{
}
pub trait Ops<T>:
    Mul<T, Output = Self>
    + Add<T, Output = Self>
    + Sub<T, Output = Self>
    + Div<T, Output = Self>
    + MulAssign<T>
    + AddAssign<T>
    + SubAssign<T>
    + DivAssign<T>
{
}
impl<K, T> Ops<T> for K where
    K: Mul<T, Output = Self>
        + Add<T, Output = Self>
        + Sub<T, Output = Self>
        + Div<T, Output = Self>
        + MulAssign<T>
        + AddAssign<T>
        + SubAssign<T>
        + DivAssign<T>
{
}
pub trait Equiv:
    PartialEq
    + PartialEq<usize>
    + PartialEq<isize>
    + PartialEq<u32>
    + PartialEq<i32>
    + PartialEq<f64>
    + Sized
{
}
impl<T> Equiv for T where
    T: PartialEq
        + PartialEq<usize>
        + PartialEq<isize>
        + PartialEq<u32>
        + PartialEq<i32>
        + PartialEq<f64>
        + Sized
{
}
pub trait Compare:
    Equiv
    + PartialOrd
    + PartialOrd<usize>
    + PartialOrd<isize>
    + PartialOrd<u32>
    + PartialOrd<i32>
    + PartialOrd<i128>
    + PartialOrd<u128>
    + PartialOrd<f64>
    + Sized
{
}
impl<T> Compare for T where
    T: Equiv
        + PartialOrd
        + PartialOrd<usize>
        + PartialOrd<isize>
        + PartialOrd<u32>
        + PartialOrd<i32>
        + PartialOrd<i128>
        + PartialOrd<u128>
        + PartialOrd<f64>
        + Sized
{
}
pub trait WithVal<From> {
    fn with_val(prec: u32, val: From) -> Self;
}
pub trait WithValImag<From> {
    fn with_val_imag(prec: u32, val: From) -> Self;
}
pub trait Pow<Rhs> {
    fn pow(self, rhs: Rhs) -> Self;
}
