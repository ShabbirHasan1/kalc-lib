use crate::types::{
    Complex, Constant, Float, FloatShared, Integer, IsPrime, Pow, WithVal, WithValImag,
};
use rug::ops::CompleteRound;
use std::cmp::Ordering;
macro_rules! with_val {
    ($ty:ty, $($v:ty),*) => {
        $(
            impl WithVal<$v> for $ty {
                fn with_val(prec: u32, val: $v) -> Self {
                    Self::with_val(prec, val)
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
                    Self::with_val(prec, (0, val))
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
                    rug::ops::Pow::pow(self, val)
                }
            }
        )*
    };
}
with_val_imag!(
    rug::Complex,
    f64,
    i32,
    usize,
    isize,
    i128,
    u128,
    u32,
    rug::Float,
    rug::Integer
);
with_val!(
    rug::Complex,
    f64,
    i32,
    usize,
    isize,
    i128,
    u128,
    u32,
    bool,
    rug::Float,
    &rug::Float,
    rug::Integer,
    &rug::Integer,
    (f64, f64),
    (i32, i32),
    (u32, u32),
    (usize, usize),
    (isize, isize),
    (i128, i128),
    (u128, u128),
    (rug::Float, rug::Float),
    (&rug::Float, &rug::Float),
    (rug::Integer, rug::Integer),
    (&rug::Integer, &rug::Integer)
);
with_val!(
    rug::Float,
    f64,
    i32,
    u32,
    bool,
    usize,
    isize,
    rug::Integer,
    i128,
    u128
);
pow!(rug::Complex, rug::Float, f64, usize, isize, u32, i32, Self);
pow!(rug::Float, f64, usize, isize, u32, i32, Self);
pow!(rug::Integer, u32);
impl WithVal<Constant> for rug::Complex {
    fn with_val(prec: u32, val: Constant) -> Self {
        match val {
            Constant::Pi => Self::with_val(prec, rug::float::Constant::Pi),
            Constant::E => Self::with_val(prec, 1).exp(),
            Constant::Infinity => Self::with_val(prec, rug::float::Special::Infinity),
            Constant::NegInfinity => Self::with_val(prec, rug::float::Special::NegInfinity),
            Constant::Nan => Self::with_val(prec, rug::float::Special::Nan),
            Constant::Tau => Self::with_val(prec, rug::float::Constant::Pi) * 2,
        }
    }
}
impl WithVal<(Constant, Constant)> for rug::Complex {
    fn with_val(prec: u32, val: (Constant, Constant)) -> Self {
        <Self as WithVal<Constant>>::with_val(prec, val.0) + Self::with_val_imag(prec, val.1)
    }
}
impl WithValImag<Constant> for rug::Complex {
    fn with_val_imag(prec: u32, val: Constant) -> Self {
        match val {
            Constant::Pi => Self::with_val(prec, (0, rug::float::Constant::Pi)),
            Constant::E => Self::with_val(prec, 1).exp().mul_i(false),
            Constant::Infinity => Self::with_val(prec, (0, rug::float::Special::Infinity)),
            Constant::NegInfinity => Self::with_val(prec, (0, rug::float::Special::NegInfinity)),
            Constant::Nan => Self::with_val(prec, (0, rug::float::Special::Nan)),
            Constant::Tau => Self::with_val(prec, (0, rug::float::Constant::Pi)) * 2,
        }
    }
}
impl WithVal<Constant> for rug::Float {
    fn with_val(prec: u32, val: Constant) -> Self {
        match val {
            Constant::Pi => Self::with_val(prec, rug::float::Constant::Pi),
            Constant::E => Self::with_val(prec, 1).exp(),
            Constant::Infinity => Self::with_val(prec, rug::float::Special::Infinity),
            Constant::NegInfinity => Self::with_val(prec, rug::float::Special::NegInfinity),
            Constant::Nan => Self::with_val(prec, rug::float::Special::Nan),
            Constant::Tau => Self::with_val(prec, rug::float::Constant::Pi) * 2,
        }
    }
}
impl Integer<rug::Float, rug::Complex> for rug::Integer {
    fn cmp0(&self) -> Ordering {
        self.cmp0()
    }
    fn div_rem(self, rhs: Self) -> (Self, Self) {
        self.div_rem(rhs)
    }
    fn next_prime(self) -> Self {
        self.next_prime()
    }
    fn binomial(self, k: u32) -> Self {
        self.binomial(k)
    }
    fn to_u32(&self) -> Option<u32> {
        self.to_u32()
    }
    fn to_usize(&self) -> Option<usize> {
        self.to_usize()
    }
    fn to_u128(&self) -> Option<u128> {
        self.to_u128()
    }
    fn to_i32(&self) -> Option<i32> {
        self.to_i32()
    }
    fn to_isize(&self) -> Option<isize> {
        self.to_isize()
    }
    fn to_i128(&self) -> Option<i128> {
        self.to_i128()
    }
    fn to_string_radix(&self, radix: i32) -> String {
        self.to_string_radix(radix)
    }
    fn from_str_radix(src: &str, radix: i32) -> Option<Self> {
        Self::from_str_radix(src, radix).ok()
    }
    fn is_probably_prime(&self, reps: u32) -> IsPrime {
        self.is_probably_prime(reps).into()
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn new() -> Self {
        Self::new()
    }
}
impl From<rug::integer::IsPrime> for IsPrime {
    fn from(value: rug::integer::IsPrime) -> Self {
        match value {
            rug::integer::IsPrime::No => IsPrime::No,
            rug::integer::IsPrime::Probably => IsPrime::Probably,
            rug::integer::IsPrime::Yes => IsPrime::Yes,
        }
    }
}
impl FloatShared<rug::Integer, Self, rug::Complex> for rug::Float {
    fn sin_cos(self, cos: Self) -> (Self, Self) {
        self.sin_cos(cos)
    }
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn sinh(self) -> Self {
        self.sinh()
    }
    fn cosh(self) -> Self {
        self.cosh()
    }
    fn tanh(self) -> Self {
        self.tanh()
    }
    fn asin(self) -> Self {
        self.asin()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn atan(self) -> Self {
        self.atan()
    }
    fn asinh(self) -> Self {
        self.asinh()
    }
    fn acosh(self) -> Self {
        self.acosh()
    }
    fn atanh(self) -> Self {
        self.atanh()
    }
    fn parse_radix(prec: u32, src: impl AsRef<[u8]>, radix: i32) -> Option<Self> {
        Some(Self::parse_radix(src, radix).ok()?.complete(prec))
    }
    fn parse(prec: u32, src: impl AsRef<[u8]>) -> Option<Self> {
        Some(Self::parse(src).ok()?.complete(prec))
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn new(prec: u32) -> Self {
        Self::new(prec)
    }
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn recip(self) -> Self {
        self.recip()
    }
    fn prec(&self) -> u32 {
        self.prec()
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn log10(self) -> Self {
        self.log10()
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn set_prec(&mut self, prec: u32) {
        self.set_prec(prec)
    }
}
impl Float<rug::Integer, rug::Complex> for rug::Float {
    fn erf(self) -> Self {
        self.erf()
    }
    fn ai(self) -> Self {
        self.ai()
    }
    fn digamma(self) -> Self {
        self.digamma()
    }
    fn zeta(self) -> Self {
        self.zeta()
    }
    fn next_up(&mut self) {
        self.next_up()
    }
    fn next_down(&mut self) {
        self.next_down()
    }
    fn next_toward(&mut self, to: &Self) {
        self.next_toward(to)
    }
    fn erfc(self) -> Self {
        self.erfc()
    }
    fn cmp0(&self) -> Option<Ordering> {
        self.cmp0()
    }
    fn is_nan(&self) -> bool {
        self.is_nan()
    }
    fn is_finite(&self) -> bool {
        self.is_finite()
    }
    fn is_infinite(&self) -> bool {
        self.is_infinite()
    }
    fn is_sign_negative(&self) -> bool {
        self.is_sign_negative()
    }
    fn is_sign_positive(&self) -> bool {
        self.is_sign_positive()
    }
    fn to_f64(&self) -> f64 {
        self.to_f64()
    }
    fn fract(self) -> Self {
        self.fract()
    }
    fn trunc(self) -> Self {
        self.trunc()
    }
    fn round(self) -> Self {
        self.round()
    }
    fn gamma(self) -> Self {
        self.gamma()
    }
    fn floor(self) -> Self {
        self.floor()
    }
    fn ceil(self) -> Self {
        self.ceil()
    }
    fn to_sign_string_exp(
        &self,
        radix: i32,
        num_digits: Option<usize>,
    ) -> (bool, String, Option<i32>) {
        self.to_sign_string_exp(radix, num_digits)
    }
    fn to_string_radix(&self, radix: i32, num_digits: Option<usize>) -> String {
        self.to_string_radix(radix, num_digits)
    }
    fn to_integer(&self) -> Option<rug::Integer> {
        self.to_integer()
    }
    fn log2(self) -> Self {
        self.log2()
    }
}
impl FloatShared<rug::Integer, rug::Float, Self> for rug::Complex {
    fn sin_cos(self, cos: Self) -> (Self, Self) {
        self.sin_cos(cos)
    }
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn sinh(self) -> Self {
        self.sinh()
    }
    fn cosh(self) -> Self {
        self.cosh()
    }
    fn tanh(self) -> Self {
        self.tanh()
    }
    fn asin(self) -> Self {
        self.asin()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn atan(self) -> Self {
        self.atan()
    }
    fn asinh(self) -> Self {
        self.asinh()
    }
    fn acosh(self) -> Self {
        self.acosh()
    }
    fn atanh(self) -> Self {
        self.atanh()
    }
    fn parse_radix(prec: u32, src: impl AsRef<[u8]>, radix: i32) -> Option<Self> {
        Some(Self::parse_radix(src, radix).ok()?.complete((prec, prec)))
    }
    fn parse(prec: u32, src: impl AsRef<[u8]>) -> Option<Self> {
        Some(Self::parse(src).ok()?.complete((prec, prec)))
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn new(prec: u32) -> Self {
        Self::new(prec)
    }
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn recip(self) -> Self {
        self.recip()
    }
    fn prec(&self) -> u32 {
        self.prec().0
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn log10(self) -> Self {
        self.log10()
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn set_prec(&mut self, prec: u32) {
        self.set_prec(prec)
    }
}
impl Complex<rug::Integer, rug::Float> for rug::Complex {
    fn total_cmp(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
    fn real(&self) -> &rug::Float {
        self.real()
    }
    fn imag(&self) -> &rug::Float {
        self.imag()
    }
    fn into_real_imag(self) -> (rug::Float, rug::Float) {
        self.into_real_imag()
    }
    fn conj(self) -> Self {
        self.conj()
    }
    fn arg(self) -> Self {
        self.arg()
    }
    fn mul_i(self, negative: bool) -> Self {
        self.mul_i(negative)
    }
}
