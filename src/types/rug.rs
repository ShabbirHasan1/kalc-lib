use crate::types::{Constant, Float, Float1, Float2, Integer, Pow, WithVal};
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
    usize,
    (f64, f64),
    (usize, usize),
    rug::Integer
);
with_val!(rug::Float, f64, usize, rug::Integer);
pow!(rug::Complex, f64, usize);
pow!(rug::Float, f64, usize);
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
}
impl Float<rug::Integer, Self, rug::Complex> for rug::Float {
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
    fn set_prec(&mut self, prec: u32) {
        self.set_prec(prec)
    }
}
impl Float1<rug::Integer, rug::Complex> for rug::Float {
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
    fn to_integer(&self) -> Option<rug::Integer> {
        self.to_integer()
    }
}
impl Float<rug::Integer, rug::Float, Self> for rug::Complex {
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
    fn set_prec(&mut self, prec: u32) {
        self.set_prec(prec)
    }
}
impl Float2<rug::Integer, rug::Float> for rug::Complex {
    fn real(&self) -> &rug::Float {
        self.real()
    }
    fn imag(&self) -> &rug::Float {
        self.imag()
    }
}
