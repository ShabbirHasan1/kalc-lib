pub mod f64;
pub mod rug;
use std::fmt::Display;
use std::ops::*;
pub trait Complex<I: Integer<F, Self>, F: Float<I, Self>>:
    FloatShared<I, F, Self> + WithVal<(usize, usize)> + WithVal<(f64, f64)> + Display
{
    fn real(&self) -> &F;
    fn imag(&self) -> &F;
}
pub trait Float<I: Integer<Self, C>, C: Complex<I, Self>>:
    FloatShared<I, Self, C> + PartialOrd<f64> + Compare
{
    fn is_finite(&self) -> bool;
    fn is_sign_negative(&self) -> bool;
    fn is_sign_positive(&self) -> bool;
    fn fract(self) -> Self;
    fn trunc(self) -> Self;
    fn to_integer(&self) -> Option<I>;
}
pub trait FloatShared<I: Integer<F, C>, F: Float<I, C>, C: Complex<I, F>>:
    Shared
    + Pow<usize>
    + Pow<f64>
    + WithVal<usize>
    + WithVal<f64>
    + WithVal<I>
    + WithVal<Constant>
    + Display
    + OperatorsOwned<I>
    + OperatorsOwned<f64>
{
    fn exp(self) -> Self;
    fn new(prec: u32) -> Self;
    fn is_zero(&self) -> bool;
    fn abs(self) -> Self;
    fn recip(self) -> Self;
    fn prec(&self) -> u32;
    fn set_prec(&mut self, prec: u32);
}
pub enum Constant {
    Pi,
    E,
    Infinity,
    NegInfinity,
}
pub trait Integer<F: Float<Self, C>, C: Complex<Self, F>>:
    Shared + From<usize> + Pow<u32> + Display + Default + Compare
{
    fn div_rem(self, rhs: Self) -> (Self, Self);
    fn next_prime(self) -> Self;
}
pub trait Shared: Operators + Clone {}
impl<T> Shared for T where T: Operators + Clone {}
pub trait Operators:
    OperatorsOwned<Self>
    + OperatorsOwned<usize>
    + for<'a> OperatorsOwned<&'a Self>
    + Neg<Output = Self>
    + Sized
{
}
impl<T> Operators for T where
    T: OperatorsOwned<T>
        + OperatorsOwned<usize>
        + for<'a> OperatorsOwned<&'a T>
        + Neg<Output = T>
        + Sized
{
}
pub trait OperatorsOut<T>:
    Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Div<T, Output = T>
{
}
impl<K, T> OperatorsOut<T> for K where
    K: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Div<T, Output = T>
{
}
pub trait OperatorsOwned<T>:
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
impl<K, T> OperatorsOwned<T> for K where
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
