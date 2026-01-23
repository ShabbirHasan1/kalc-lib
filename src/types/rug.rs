use crate::types::{Complex, Constant, Float, FloatShared, Integer, Pow, WithVal};
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
with_val!(
    rug::Complex,
    f64,
    (f64, f64),
    i32,
    (i32, i32),
    usize,
    (usize, usize),
    isize,
    (isize, isize),
    rug::Float,
    (rug::Float, rug::Float),
    rug::Integer,
    (rug::Integer, rug::Integer)
);
with_val!(rug::Float, f64, i32, usize, isize, rug::Integer);
pow!(rug::Complex, f64, usize, isize, u32, i32, Self);
pow!(rug::Float, f64, usize, isize, u32, i32, Self);
pow!(rug::Integer, u32);
impl WithVal<Constant> for rug::Complex {
    fn with_val(prec: u32, val: Constant) -> Self {
        match val {
            Constant::Pi => Self::with_val(prec, rug::float::Constant::Pi),
            Constant::E => Self::with_val(prec, 1).exp(),
            Constant::Infinity => Self::with_val(prec, rug::float::Special::Infinity),
            Constant::NegInfinity => Self::with_val(prec, rug::float::Special::NegInfinity),
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
        }
    }
}
impl Integer<rug::Float, rug::Complex> for rug::Integer {
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
    fn abs(self) -> Self {
        self.abs()
    }
}
impl FloatShared<rug::Integer, Self, rug::Complex> for rug::Float {
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
    fn set_prec(&mut self, prec: u32) {
        self.set_prec(prec)
    }
}
impl Float<rug::Integer, rug::Complex> for rug::Float {
    fn is_finite(&self) -> bool {
        self.is_finite()
    }
    fn is_sign_negative(&self) -> bool {
        self.is_sign_negative()
    }
    fn is_sign_positive(&self) -> bool {
        self.is_sign_positive()
    }
    fn fract(self) -> Self {
        self.fract()
    }
    fn trunc(self) -> Self {
        self.trunc()
    }
    fn gamma(self) -> Self {
        self.gamma()
    }
    fn floor(self) -> Self {
        self.floor()
    }
    fn to_integer(&self) -> Option<rug::Integer> {
        self.to_integer()
    }
}
impl FloatShared<rug::Integer, rug::Float, Self> for rug::Complex {
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
    fn set_prec(&mut self, prec: u32) {
        self.set_prec(prec)
    }
}
impl Complex<rug::Integer, rug::Float> for rug::Complex {
    fn real(&self) -> &rug::Float {
        self.real()
    }
    fn imag(&self) -> &rug::Float {
        self.imag()
    }
}
