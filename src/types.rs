pub mod f64;
pub mod rug;
use std::fmt::Display;
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
    + WithVal<(I, I)>
    + WithVal<F>
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
    fn real(&self) -> &F;
    fn imag(&self) -> &F;
    fn into_real_imag(self) -> (F, F);
    fn conj(self) -> Self;
    fn arg(self) -> Self;
    fn mul_i(self, negative: bool) -> Self;
}
pub trait Float<I: Integer<Self, C>, C: Complex<I, Self>>:
    FloatShared<I, Self, C> + PartialOrd<f64> + Compare + OperatorsOut<C, C> + Pow<Self> + RemOp<usize>
{
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
    + Shr<i32>
    + Shl<i32>
    + Shr<u32>
    + Shl<u32>
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
{
    fn div_rem(self, rhs: Self) -> (Self, Self);
    fn next_prime(self) -> Self;
    fn binomial(self, k: u32) -> Self;
    fn to_u32(&self) -> Option<u32>;
    fn to_usize(&self) -> Option<usize>;
    fn to_u128(&self) -> Option<u128>;
    fn to_i32(&self) -> Option<i32>;
    fn to_isize(&self) -> Option<isize>;
    fn to_i128(&self) -> Option<i128>;
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
pub trait Shared: Operators + Clone {}
impl<T> Shared for T where T: Operators + Clone {}
pub trait Operators:
    Ops<Self>
    + Ops<usize>
    + Ops<u128>
    + Ops<i128>
    + Ops<i32>
    + Ops<u32>
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
    PartialEq + PartialEq<usize> + PartialEq<isize> + PartialEq<u32> + PartialEq<i32> + Sized
{
}
impl<T> Equiv for T where
    T: PartialEq + PartialEq<usize> + PartialEq<isize> + PartialEq<u32> + PartialEq<i32> + Sized
{
}
pub trait Compare:
    Equiv
    + PartialOrd
    + PartialOrd<usize>
    + PartialOrd<isize>
    + PartialOrd<u32>
    + PartialOrd<i32>
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
