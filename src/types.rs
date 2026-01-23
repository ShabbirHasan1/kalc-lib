pub mod f64;
pub mod rug;
use std::fmt::Display;
use std::ops::*;
pub trait Complex<I: Integer<F, Self>, F: Float<I, Self>>:
    FloatShared<I, F, Self>
    + WithVal<(usize, usize)>
    + WithVal<(isize, isize)>
    + WithVal<(i32, i32)>
    + WithVal<(f64, f64)>
    + WithVal<(F, F)>
    + WithVal<(I, I)>
    + WithVal<F>
    + Ops<F>
    + From<F>
    + Pow<Self>
    + Display
{
    fn real(&self) -> &F;
    fn imag(&self) -> &F;
}
pub trait Float<I: Integer<Self, C>, C: Complex<I, Self>>:
    FloatShared<I, Self, C> + PartialOrd<f64> + Compare + OperatorsOut<C, C> + Pow<Self>
{
    fn is_finite(&self) -> bool;
    fn is_sign_negative(&self) -> bool;
    fn is_sign_positive(&self) -> bool;
    fn fract(self) -> Self;
    fn trunc(self) -> Self;
    fn gamma(self) -> Self;
    fn floor(self) -> Self;
    fn to_integer(&self) -> Option<I>;
}
pub trait FloatShared<I: Integer<F, C>, F: Float<I, C>, C: Complex<I, F>>:
    Shared
    + Pow<usize>
    + Pow<u32>
    + Pow<i32>
    + Pow<isize>
    + Pow<f64>
    + WithVal<usize>
    + WithVal<isize>
    + WithVal<f64>
    + WithVal<i32>
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
    fn set_prec(&mut self, prec: u32);
}
pub enum Constant {
    Pi,
    E,
    Infinity,
    NegInfinity,
}
pub trait Integer<F: Float<Self, C>, C: Complex<Self, F>>:
    Shared + From<usize> + From<isize> + From<i32> + Pow<u32> + Display + Default + Compare
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
    fn abs(self) -> Self;
}
pub trait Shared: Operators + Clone {}
impl<T> Shared for T where T: Operators + Clone {}
pub trait Operators:
    Ops<Self> + Ops<usize> + for<'a> Ops<&'a Self> + Neg<Output = Self> + Sized
{
}
impl<T> Operators for T where T: Ops<T> + Ops<usize> + for<'a> Ops<&'a T> + Neg<Output = T> + Sized {}
pub trait OperatorsOut<T, K>:
    Mul<T, Output = K> + Add<T, Output = K> + Sub<T, Output = K> + Div<T, Output = K>
{
}
impl<K, T, V> OperatorsOut<T, V> for K where
    K: Mul<T, Output = V> + Add<T, Output = V> + Sub<T, Output = V> + Div<T, Output = V>
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
pub trait Compare: PartialOrd + PartialEq + PartialOrd<usize> + Sized {}
impl<T> Compare for T where T: PartialOrd + PartialEq + PartialOrd<usize> + PartialOrd<f64> + Sized {}
pub trait WithVal<From> {
    fn with_val(prec: u32, val: From) -> Self;
}
pub trait Pow<Rhs> {
    fn pow(self, rhs: Rhs) -> Self;
}
