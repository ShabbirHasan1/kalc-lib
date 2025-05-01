// use crate::types::{ NewDeciVal, DivFloor, NewVal, Parse, ParseU, Pow, Prec, Rt, SinhCosh, Special, SpecialU, WithVal, WithValDeci, DivFloor };
macro_rules! impl_new_val_deci {
    ($ty:ty) => {
        impl NewDeciVal for $ty {
            fn new(prec: u32) -> Self {
                match prec.next_power_of_two() {
                    512 => Self::D512(fastnum::D512::from(0)),
                    256 => Self::D256(fastnum::D256::from(0)),
                    _ => unreachable!(),
                }
            }
        }
    };
}

macro_rules! impl_pow {
    ($ty:ty, $other:ty, $( ($variant:ident, $cast:expr) ),* ) => {
        impl Pow<$other> for $ty {
            fn pow(self, rhs: $other) -> Self {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a.pow($cast(rhs))),
                    )*
                }
            }
        }
    };
}

macro_rules! impl_sinh_cosh {
    ($($t:ty),*) => {
        $(impl SinhCosh for $t {
            fn sinh_cosh(self) -> (Self, Self) {
                (self.sinh(), self.cosh())
            }
        })*
    };
}

macro_rules! impl_with_val_deci {
    ($ty:ty, $other:ty) => {
        impl WithValDeci<$other> for $ty {
            fn with_val(prec: u32, rhs: $other) -> Self {
                match prec.next_power_of_two() {
                    512 => Self::D512(fastnum::D512::from(rhs)),
                    256 => Self::D256(fastnum::D256::from(rhs)),
                    _ => unreachable!(),
                }
            }
        }
    };
}

macro_rules! impl_with_val_cdeci {
    ($ty:ty, $other:ty) => {
        impl WithValDeci<$other> for $ty {
            fn with_val(prec: u32, rhs: $other) -> Self {
                CDecimal(Decimal::with_val(prec, rhs), Decimal::with_val(prec, 0))
            }
        }
    };
}

macro_rules! impl_new_val_cdeci {
    ($ty:ty) => {
        impl NewDeciVal for $ty {
            fn new(prec: u32) -> Self {
                CDecimal(Decimal::new(prec), Decimal::new(prec))
            }
        }
    };
}

macro_rules! impl_with_val {
    ($ty:ty, $other:ty, $( ($variant:ident, $cast:expr) ),* ) => {
        impl WithVal<$other> for $ty {
            fn with_val(obj: Type, prec: u32, rhs: $other) -> Self {
                match obj {
                    $(
                        Type::$variant => Self::$variant($cast(prec, rhs)),
                    )*
                }
            }
        }
    };
}

macro_rules! impl_new_val {
    ($ty:ty, $( ($variant:ident, $cast:expr) ),* ) => {
        impl NewVal for $ty {
            fn new(obj: Type, prec: u32) -> Self {
                match obj {
                    $(
                        Type::$variant => Self::$variant($cast(prec, 0.0)),
                    )*
                }
            }
        }
    };
}

macro_rules! impl_partial_ord {
    ($t:ty,$($variant:ident),*) => {
        impl PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                match (self, other) {
                    $((Self::$variant(a), Self::$variant(b)) => a.partial_cmp(b),)*
                    _=>unreachable!()
                }
            }
            fn lt(&self, other: &Self) -> bool {
                match (self, other) {
                    $((Self::$variant(a), Self::$variant(b)) => a.lt(b),)*
                    _=>unreachable!()
                }
            }
            fn le(&self, other: &Self) -> bool {
                match (self, other) {
                $(    (Self::$variant(a), Self::$variant(b)) => a.le(b),)*
                    _=>unreachable!()
                }
            }
            fn gt(&self, other: &Self) -> bool {
                match (self, other) {
                $(  (Self::$variant(a), Self::$variant(b)) => a.gt(b),)*
                    _=>unreachable!()
                }
            }
            fn ge(&self, other: &Self) -> bool {
                match (self, other) {
                    $((Self::$variant(a), Self::$variant(b)) => a.ge(b),)*
                    _=>unreachable!()
                }
            }
        }
    };
}

macro_rules! dec_impl {
    ($t:ty,$($variant:ident),*) => {
        impl $t {
            pub fn abs(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.abs()),)*
                }
            }
            pub fn recip(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.recip()),)*
                }
            }
            pub fn sqrt(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sqrt()),)*
                }
            }
            pub fn exp(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.exp()),)*
                }
            }
            pub fn ln(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.ln()),)*
                }
            }
            pub fn log2(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.log2()),)*
                }
            }
            pub fn log10(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.log10()),)*
                }
            }
            pub fn cbrt(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cbrt()),)*
                }
            }
            pub fn sin(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sin()),)*
                }
            }
            pub fn cos(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cos()),)*
                }
            }
            pub fn tan(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.tan()),)*
                }
            }
            pub fn asin(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.asin()),)*
                }
            }
            pub fn acos(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.acos()),)*
                }
            }
            pub fn atan(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.atan()),)*
                }
            }
            pub fn atan2(self, other: Self) -> Self {
                match (self, other) {
                    $( (Self::$variant(a), Self::$variant(b)) => Self::$variant(a.atan2(b)), )*
                    _ => unreachable!(),
                }
            }
            pub fn sinh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sinh()),)*
                }
            }
            pub fn cosh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cosh()),)*
                }
            }
            pub fn tanh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.tanh()),)*
                }
            }
            pub fn asinh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.asinh()),)*
                }
            }
            pub fn acosh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.acosh()),)*
                }
            }
            pub fn atanh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.atanh()),)*
                }
            }
            pub fn round(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.round(0)),)*
                }
            }
            pub fn floor(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.floor()),)*
                }
            }
            pub fn ceil(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.ceil()),)*
                }
            }
            pub fn trunc(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(if a.is_sign_positive(){a.floor()}else{a.ceil()}),)*
                }
            }
            pub fn fract(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a-if a.is_sign_positive(){a.floor()}else{a.ceil()}),)*
                }
            }
            pub fn sin_cos(self) -> (Self, Self) {
                match self {
                    $(Self::$variant(a) => {
                        let (s, c) = a.sin_cos();
                        (Self::$variant(s), Self::$variant(c))
                    },)*
                }
            }
            pub fn sinh_cosh(self) -> (Self, Self) {
                match self {
                    $(Self::$variant(a) => {
                        let (s, c) = a.sinh_cosh();
                        (Self::$variant(s), Self::$variant(c))
                    },)*
                }
            }
            pub fn hypot(self, other: Self) -> Self {
                match (self, other) {
                    $( (Self::$variant(a), Self::$variant(b)) => Self::$variant(a.hypot(b)), )*
                    _ => unreachable!(),
                }
            }
            pub fn is_nan(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_nan(),)*
                }
            }
            pub fn is_infinite(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_infinite(),)*
                }
            }
            pub fn is_finite(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_finite(),)*
                }
            }
            pub fn is_sign_positive(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_sign_positive(),)*
                }
            }
            pub fn is_sign_negative(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_sign_negative(),)*
                }
            }
        }
    };
}

macro_rules! float_impl {
    ($t:ty,$($variant:ident),*) => {
        impl $t {
            pub fn abs(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.abs()),)*
                }
            }
            pub fn recip(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.recip()),)*
                }
            }
            pub fn sqrt(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sqrt()),)*
                }
            }
            pub fn exp(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.exp()),)*
                }
            }
            pub fn ln(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.ln()),)*
                }
            }
            pub fn log2(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.log2()),)*
                }
            }
            pub fn log10(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.log10()),)*
                }
            }
            pub fn cbrt(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cbrt()),)*
                }
            }
            pub fn sin(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sin()),)*
                }
            }
            pub fn cos(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cos()),)*
                }
            }
            pub fn tan(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.tan()),)*
                }
            }
            pub fn asin(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.asin()),)*
                }
            }
            pub fn acos(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.acos()),)*
                }
            }
            pub fn atan(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.atan()),)*
                }
            }
            pub fn sinh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.sinh()),)*
                }
            }
            pub fn cosh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.cosh()),)*
                }
            }
            pub fn tanh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.tanh()),)*
                }
            }
            pub fn asinh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.asinh()),)*
                }
            }
            pub fn acosh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.acosh()),)*
                }
            }
            pub fn atanh(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.atanh()),)*
                }
            }
            pub fn round(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.round()),)*
                }
            }
            pub fn floor(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.floor()),)*
                }
            }
            pub fn ceil(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.ceil()),)*
                }
            }
            pub fn trunc(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.trunc()),)*
                }
            }
            pub fn fract(self) -> Self {
                match self {
                    $(Self::$variant(a) => Self::$variant(a.fract()),)*
                }
            }
            pub fn is_nan(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_nan(),)*
                }
            }
            pub fn is_infinite(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_infinite(),)*
                }
            }
            pub fn is_finite(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_finite(),)*
                }
            }
            pub fn is_sign_positive(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_sign_positive(),)*
                }
            }
            pub fn is_sign_negative(self) -> bool {
                match self {
                    $(Self::$variant(a) => a.is_sign_negative(),)*
                }
            }
        }
    };
}

macro_rules! dec_c_impl {
    ($t:ty, $l:ty, $new:expr) => {
        #[allow(clippy::unnecessary_cast)]
        impl $t {
            pub fn abs(self) -> Self {
                Self(
                    (self.0 * self.0 + self.1 * self.1).sqrt(),
                    $new(self.prec(), 0.0),
                )
            }
            pub fn recip(self) -> Self {
                let abs = self.0 * self.0 + self.1 * self.1;
                Self(self.0 / abs, -self.1 / abs)
            }
            pub fn sqrt(self) -> Self {
                self.pow(0.5)
            }
            pub fn exp(self) -> Self {
                let r = self.0.exp();
                let (c, s) = self.1.sin_cos();
                r * Self(c, s)
            }
            pub fn arg(self) -> Self {
                Self(self.1.atan2(self.0), $new(self.prec(), 0.0))
            }
            pub fn ln(self) -> Self {
                let abs = self.0 * self.0 + self.1 * self.1;
                Self(abs.ln() * 0.5, self.1.atan2(self.0))
            }
            pub fn log2(self) -> Self {
                self.ln() / $new(self.prec(), 2.0).ln()
            }
            pub fn log10(self) -> Self {
                self.ln() / $new(self.prec(), 10.0).ln()
            }
            pub fn cbrt(self) -> Self {
                self.root(3)
            }
            pub fn conj(self) -> Self {
                Self(self.0, -self.1)
            }
            pub fn sin(self) -> Self {
                let (a, b) = self.0.sin_cos();
                let (c, d) = self.1.sinh_cosh();
                Self(a * d, b * c)
            }
            pub fn cos(self) -> Self {
                let (a, b) = self.0.sin_cos();
                let (c, d) = self.1.sinh_cosh();
                Self(b * d, a * c)
            }
            pub fn tan(mut self) -> Self {
                self.0 *= 2.0;
                self.1 *= 2.0;
                let (a, b) = self.0.sin_cos();
                let (c, d) = self.1.sinh_cosh();
                Self(a, c) / (b + d)
            }
            pub fn asin(self) -> Self {
                let p: $t = 1.0 - self * self;
                let v = Self(-self.1, self.0) + p.sqrt();
                v.ln()
            }
            pub fn acos(self) -> Self {
                <$l>::pi(self.prec()) / 2.0 - self.asin()
            }
            pub fn atan(self) -> Self {
                let v = Self(-self.1, self.0);
                let a: $t = 1 + v;
                let b: $t = 1 - v;
                (a.arg() - b.arg()) / 2.0
            }
            pub fn atan2(self, other: Self) -> Self {
                let v = (Self(-self.1, self.0) + other) / (self * self + other * other).sqrt();
                v.ln()
            }
            pub fn sinh(self) -> Self {
                let (a, b) = self.0.sinh_cosh();
                let (c, d) = self.1.sin_cos();
                Self(a * d, b * c)
            }
            pub fn cosh(self) -> Self {
                let (a, b) = self.0.sinh_cosh();
                let (c, d) = self.1.sin_cos();
                Self(b * d, a * c)
            }
            pub fn tanh(mut self) -> Self {
                self.0 *= 2.0;
                self.1 *= 2.0;
                let (a, b) = self.0.sinh_cosh();
                let (c, d) = self.1.sin_cos();
                Self(a, c) / (b + d)
            }
            pub fn asinh(self) -> Self {
                let v: $t = 1.0 + self * self;
                (v.sqrt() + self).ln()
            }
            pub fn acosh(self) -> Self {
                let v: $t = self * self - 1.0;
                (v.sqrt() + self).ln()
            }
            pub fn atanh(self) -> Self {
                let a: $t = 1 + self;
                let b: $t = 1 - self;
                (a.ln() - b.ln()) / 2.0
            }
            pub fn sin_cos(self) -> (Self, Self) {
                (self.sin(), self.cos())
            }
            pub fn sinh_cosh(self) -> (Self, Self) {
                (self.sinh(), self.cosh())
            }
            pub fn is_nan(self) -> bool {
                self.0.is_nan() || self.1.is_nan()
            }
            pub fn is_infinite(self) -> bool {
                self.0.is_infinite() || self.1.is_infinite()
            }
            pub fn is_finite(self) -> bool {
                self.0.is_finite() && self.1.is_finite()
            }
        }
    };
}

macro_rules! impl_c_pow {
    ($t:ty, $rhs:ty, $cast:expr) => {
        impl Pow<$rhs> for $t {
            fn pow(self, rhs: $rhs) -> Self {
                let rhs = $cast(rhs);
                let t = self.1.atan2(self.0);
                let abs = self.0 * self.0 + self.1 * self.1;
                let r = abs.pow(rhs * 0.5) * rhs;
                let (s, c) = t.sin_cos();
                Self(r * c, r * s)
            }
        }
    };
}

macro_rules! impl_c_rt {
    ($t:ty, $rhs:ty, $cast:expr) => {
        impl Rt<$rhs> for $t {
            fn root(self, rhs: $rhs) -> Self {
                let rhs = $cast(rhs);
                let t = self.1.atan2(self.0);
                let abs = self.0 * self.0 + self.1 * self.1;
                let r = abs.pow(0.5 / rhs) / rhs;
                let (s, c) = t.sin_cos();
                Self(r * c, r * s)
            }
        }
    };
}

macro_rules! impl_c_ops {
    ($t:ty, $enum:ident, $rhs:ty, $cast:expr) => {
        impl std::ops::Add<$rhs> for $t {
            type Output = Self;
            fn add(self, rhs: $rhs) -> Self::Output {
                Self(self.0 + $cast(rhs), self.1)
            }
        }
        impl std::ops::Add<$t> for $rhs {
            type Output = $t;
            fn add(self, rhs: $t) -> Self::Output {
                $enum($cast(self) + rhs.0, rhs.1)
            }
        }
        impl std::ops::AddAssign<$rhs> for $t {
            fn add_assign(&mut self, rhs: $rhs) {
                self.0 += $cast(rhs);
            }
        }
        impl std::ops::Sub<$rhs> for $t {
            type Output = Self;
            fn sub(self, rhs: $rhs) -> Self::Output {
                Self(self.0 - $cast(rhs), self.1)
            }
        }
        impl std::ops::Sub<$t> for $rhs {
            type Output = $t;
            fn sub(self, rhs: $t) -> Self::Output {
                $enum($cast(self) - rhs.0, -rhs.1)
            }
        }
        impl std::ops::SubAssign<$rhs> for $t {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.0 -= $cast(rhs);
            }
        }
        impl std::ops::Mul<$rhs> for $t {
            type Output = Self;
            fn mul(self, rhs: $rhs) -> Self::Output {
                Self(self.0 * $cast(rhs), self.1 * $cast(rhs))
            }
        }
        impl std::ops::Mul<$t> for $rhs {
            type Output = $t;
            fn mul(self, rhs: $t) -> Self::Output {
                $enum($cast(self) * rhs.0, $cast(self) * rhs.1)
            }
        }
        impl std::ops::MulAssign<$rhs> for $t {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.0 *= $cast(rhs);
            }
        }
        impl std::ops::Div<$rhs> for $t {
            type Output = Self;
            fn div(self, rhs: $rhs) -> Self::Output {
                Self(self.0 / $cast(rhs), self.1 / $cast(rhs))
            }
        }
        impl std::ops::Div<$t> for $rhs {
            type Output = $t;
            fn div(self, rhs: $t) -> Self::Output {
                let abs = rhs.0 * rhs.0 + rhs.1 * rhs.1;
                $enum($cast(self) * rhs.0 / abs, $cast(self) * rhs.1 / abs)
            }
        }
        impl std::ops::DivAssign<$rhs> for $t {
            fn div_assign(&mut self, rhs: $rhs) {
                self.0 /= $cast(rhs);
            }
        }
    };
}

macro_rules! impl_self_c_ops {
    ($t:ty) => {
        impl std::ops::Add<$t> for $t {
            type Output = Self;
            fn add(self, rhs: $t) -> Self::Output {
                Self(self.0 + rhs.0, self.1 + rhs.1)
            }
        }
        impl std::ops::AddAssign<$t> for $t {
            fn add_assign(&mut self, rhs: $t) {
                self.0 += rhs.0;
                self.1 += rhs.1;
            }
        }
        impl std::ops::Sub<$t> for $t {
            type Output = Self;
            fn sub(self, rhs: $t) -> Self::Output {
                Self(self.0 - rhs.0, self.1 - rhs.1)
            }
        }
        impl std::ops::SubAssign<$t> for $t {
            fn sub_assign(&mut self, rhs: $t) {
                self.0 -= rhs.0;
                self.1 -= rhs.1;
            }
        }
        impl std::ops::Mul<$t> for $t {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self::Output {
                Self(
                    self.0 * rhs.0 - self.1 * rhs.1,
                    self.1 * rhs.0 + self.0 * rhs.1,
                )
            }
        }
        impl std::ops::MulAssign<$t> for $t {
            fn mul_assign(&mut self, rhs: $t) {
                *self = Self(
                    self.0 * rhs.0 - self.1 * rhs.1,
                    self.1 * rhs.0 + self.0 * rhs.1,
                )
            }
        }
        impl std::ops::Div<$t> for $t {
            type Output = Self;
            fn div(self, rhs: $t) -> Self::Output {
                let sq = (self.0 * self.0 + rhs.0 * rhs.0).recip();
                Self(
                    (self.0 * rhs.0 + self.1 * rhs.1) * sq,
                    (self.1 * rhs.0 - self.0 * rhs.1) * sq,
                )
            }
        }
        impl std::ops::DivAssign<$t> for $t {
            fn div_assign(&mut self, rhs: $t) {
                let sq = (self.0 * self.0 + rhs.0 * rhs.0).recip();
                *self = Self(
                    (self.0 / rhs.0 + self.1 / rhs.1) * sq,
                    (self.1 / rhs.0 - self.0 / rhs.1) * sq,
                )
            }
        }
    };
}

macro_rules! impl_ops {
    ($ty:ty, $enum:ident, $other:ty, $( ($variant:ident, $cast:expr, $div:expr) ),* ) => {
        impl std::ops::Add<$other> for $ty {
            type Output = Self;
            fn add(self, rhs: $other) -> Self::Output {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a + $cast(rhs)),
                    )*
                }
            }
        }
        impl std::ops::Add<$ty> for $other {
            type Output = $ty;
            fn add(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $(
                        $enum::$variant(a) => $enum::$variant(a+$cast(self)),
                    )*
                }
            }
        }
        impl std::ops::AddAssign<$other> for $ty {
            fn add_assign(&mut self, rhs: $other) {
                match self {
                    $(
                        Self::$variant(a) => *a += $cast(rhs),
                    )*
                }
            }
        }
        impl std::ops::Sub<$other> for $ty {
            type Output = Self;
            fn sub(self, rhs: $other) -> Self::Output {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a - $cast(rhs)),
                    )*
                }
            }
        }
        impl std::ops::Sub<$ty> for $other {
            type Output = $ty;
            fn sub(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $(
                        $enum::$variant(a) => $enum::$variant($cast(self) - a),
                    )*
                }
            }
        }
        impl std::ops::SubAssign<$other> for $ty {
            fn sub_assign(&mut self, rhs: $other) {
                match self {
                    $(
                        Self::$variant(a) => *a -= $cast(rhs),
                    )*
                }
            }
        }
        impl std::ops::Mul<$other> for $ty {
            type Output = Self;
            fn mul(self, rhs: $other) -> Self::Output {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a * $cast(rhs)),
                    )*
                }
            }
        }
        impl std::ops::Mul<$ty> for $other {
            type Output = $ty;
            fn mul(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $(
                        $enum::$variant(a) => $enum::$variant(a*$cast(self)),
                    )*
                }
            }
        }
        impl std::ops::MulAssign<$other> for $ty {
            fn mul_assign(&mut self, rhs: $other) {
                match self {
                    $(
                        Self::$variant(a) => *a *= $cast(rhs),
                    )*
                }
            }
        }
        impl std::ops::Div<$other> for $ty {
            type Output = Self;
            fn div(self, rhs: $other) -> Self::Output {
                match self {
                    $(
                        Self::$variant(a) => Self::$variant(a / $cast(rhs)),
                    )*
                }
            }
        }
        impl std::ops::Div<$ty> for $other {
            type Output = $ty;
            fn div(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $(
                        $enum::$variant(a) => $enum::$variant($div($cast(self),a)),
                    )*
                }
            }
        }
        impl std::ops::DivAssign<$other> for $ty {
            fn div_assign(&mut self, rhs: $other) {
                match self {
                    $(
                        Self::$variant(a) => *a /= $cast(rhs),
                    )*
                }
            }
        }
    };
}

macro_rules! impl_int_ops {
    ($ty:ty, $enum:ident, $other:ty) => {
        impl std::ops::Add<$other> for $ty {
            type Output = Self;
            fn add(self, rhs: $other) -> Self::Output {
                match self {
                    Self::Rug(a) => Self::Rug(a + rhs),
                    Self::Fastnum(a) => Self::Fastnum(a + fastnum::I512::from(rhs)),
                    Self::F64(a) => Self::F64(a + rhs as i128),
                    Self::F32(a) => Self::F32(a + rhs as i128),
                }
            }
        }
        impl std::ops::Add<$ty> for $other {
            type Output = $ty;
            fn add(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $enum::Rug(a) => $enum::Rug(self + a),
                    $enum::Fastnum(a) => $enum::Fastnum(fastnum::I512::from(self) + a),
                    $enum::F64(a) => $enum::F64(self as i128 + a),
                    $enum::F32(a) => $enum::F32(self as i128 + a),
                }
            }
        }
        impl std::ops::AddAssign<$other> for $ty {
            fn add_assign(&mut self, rhs: $other) {
                match self {
                    Self::Rug(a) => *a += rhs,
                    Self::Fastnum(a) => *a = fastnum::I512::from(rhs),
                    Self::F64(a) => *a += rhs as i128,
                    Self::F32(a) => *a += rhs as i128,
                }
            }
        }
        impl std::ops::Sub<$other> for $ty {
            type Output = Self;
            fn sub(self, rhs: $other) -> Self::Output {
                match self {
                    Self::Rug(a) => Self::Rug(a - rhs),
                    Self::Fastnum(a) => Self::Fastnum(a - fastnum::I512::from(rhs)),
                    Self::F64(a) => Self::F64(a - rhs as i128),
                    Self::F32(a) => Self::F32(a - rhs as i128),
                }
            }
        }
        impl std::ops::Sub<$ty> for $other {
            type Output = $ty;
            fn sub(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $enum::Rug(a) => $enum::Rug(self - a),
                    $enum::Fastnum(a) => $enum::Fastnum(fastnum::I512::from(self) - a),
                    $enum::F64(a) => $enum::F64(self as i128 - a),
                    $enum::F32(a) => $enum::F32(self as i128 - a),
                }
            }
        }
        impl std::ops::SubAssign<$other> for $ty {
            fn sub_assign(&mut self, rhs: $other) {
                match self {
                    Self::Rug(a) => *a -= rhs,
                    Self::Fastnum(a) => *a -= fastnum::I512::from(rhs),
                    Self::F64(a) => *a -= rhs as i128,
                    Self::F32(a) => *a -= rhs as i128,
                }
            }
        }
        impl std::ops::Mul<$other> for $ty {
            type Output = Self;
            fn mul(self, rhs: $other) -> Self::Output {
                match self {
                    Self::Rug(a) => Self::Rug(a * rhs),
                    Self::Fastnum(a) => Self::Fastnum(a * fastnum::I512::from(rhs)),
                    Self::F64(a) => Self::F64(a * rhs as i128),
                    Self::F32(a) => Self::F32(a * rhs as i128),
                }
            }
        }
        impl std::ops::Mul<$ty> for $other {
            type Output = $ty;
            fn mul(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $enum::Rug(a) => $enum::Rug(self * a),
                    $enum::Fastnum(a) => $enum::Fastnum(fastnum::I512::from(self) * a),
                    $enum::F64(a) => $enum::F64(self as i128 * a),
                    $enum::F32(a) => $enum::F32(self as i128 * a),
                }
            }
        }
        impl std::ops::MulAssign<$other> for $ty {
            fn mul_assign(&mut self, rhs: $other) {
                match self {
                    Self::Rug(a) => *a *= rhs,
                    Self::Fastnum(a) => *a *= fastnum::I512::from(rhs),
                    Self::F64(a) => *a *= rhs as i128,
                    Self::F32(a) => *a *= rhs as i128,
                }
            }
        }
        impl std::ops::Div<$other> for $ty {
            type Output = Self;
            fn div(self, rhs: $other) -> Self::Output {
                match self {
                    Self::Rug(a) => Self::Rug(a / rhs),
                    Self::Fastnum(a) => Self::Fastnum(a / fastnum::I512::from(rhs)),
                    Self::F64(a) => Self::F64(a / rhs as i128),
                    Self::F32(a) => Self::F32(a / rhs as i128),
                }
            }
        }
        impl std::ops::Div<$ty> for $other {
            type Output = $ty;
            fn div(self, rhs: $ty) -> Self::Output {
                match rhs {
                    $enum::Rug(a) => $enum::Rug(self / a),
                    $enum::Fastnum(a) => $enum::Fastnum(fastnum::I512::from(self) / a),
                    $enum::F64(a) => $enum::F64(self as i128 / a),
                    $enum::F32(a) => $enum::F32(self as i128 / a),
                }
            }
        }
        impl std::ops::DivAssign<$other> for $ty {
            fn div_assign(&mut self, rhs: $other) {
                match self {
                    Self::Rug(a) => *a /= rhs,
                    Self::Fastnum(a) => *a /= fastnum::I512::from(rhs),
                    Self::F64(a) => *a /= rhs as i128,
                    Self::F32(a) => *a /= rhs as i128,
                }
            }
        }
    };
}

macro_rules! impl_self_ops {
    ($ty:ty, $( $variant:ident ),* ) => {
        impl std::ops::Add for $ty {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => Self::$variant(a + b),
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::AddAssign for $ty {
            fn add_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => *a += b,
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::Sub for $ty {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => Self::$variant(a - b),
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::SubAssign for $ty {
            fn sub_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => *a -= b,
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::Mul for $ty {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => Self::$variant(a * b),
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::MulAssign for $ty {
            fn mul_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => *a *= b,
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::Div for $ty {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => Self::$variant(a / b),
                    )*
                    _ => unreachable!(),
                }
            }
        }
        impl std::ops::DivAssign for $ty {
            fn div_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    $(
                        (Self::$variant(a), Self::$variant(b)) => *a /= b,
                    )*
                    _ => unreachable!(),
                }
            }
        }
    }
}

macro_rules! impl_neg {
    ($ty:ty, $( $variant:ident ),* ) => {
        impl std::ops::Neg for $ty {
            type Output = Self;
            fn neg(self) -> Self::Output {
                match self {
                    $(
                    Self::$variant(a) => Self::$variant(-a),
                    )*
                }
            }
        }
    };
}

macro_rules! impl_cneg {
    ($ty:ty) => {
        impl std::ops::Neg for $ty {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0, -self.1)
            }
        }
    };
}

macro_rules! impl_types {
    ($($ty:ty),*) => {
        $(
            impl_with_val_deci!(Decimal, $ty);
            impl_with_val_cdeci!(CDecimal, $ty);
            impl_pow!(
                Decimal,
                $ty,
                (D512, fastnum::D512::from),
                (D256, fastnum::D256::from)
            );
            impl_pow!(
                Complex,
                $ty,
                (Rug, |x| x),
                (Fastnum, |x| x),
                (F64, |x| x),
                (F32, |x| x)
            );
            impl_pow!(
                Float,
                $ty,
                (Rug, |x| x),
                (Fastnum, |x| x),
                (F64, |x| x as f64),
                (F32, |x| x as f32)
            );
            impl_c_ops!(CDecimal, CDecimal, $ty, |x| x);
            impl_c_ops!(CF64, CF64, $ty, |x| x as f64);
            impl_c_ops!(CF32, CF32, $ty, |x| x as f32);
            impl_ops!(
                Decimal,
                Decimal,
                $ty,
                (D512, |x| x, |a, b: fastnum::D512| b.recip() * a),
                (D256, |x| x, |a, b: fastnum::D256| b.recip() * a)
            );
            impl_c_pow!(CDecimal, $ty, |x| x as f64);
            impl_c_pow!(CF64, $ty, |x| x as f64);
            impl_c_pow!(CF32, $ty, |x| x as f32);
            impl_c_rt!(CDecimal, $ty, |x| x as f64);
            impl_c_rt!(CF64, $ty, |x| x as f64);
            impl_c_rt!(CF32, $ty, |x| x as f32);
            impl_ops!(
                Complex,
                Complex,
                $ty,
                (Rug, |x| x, |a, b| a / b),
                (Fastnum, |x| x, |a, b| a / b),
                (F64, |x| x as f64, |a, b| a / b),
                (F32, |x| x as f32, |a, b| a / b)
            );
            impl_ops!(
                Float,
                Float,
                $ty,
                (Rug, |x| x, |a, b| a / b),
                (Fastnum, |x| x, |a, b| a / b),
                (F64, |x| x as f64, |a, b| a / b),
                (F32, |x| x as f32, |a, b| a / b)
            );
            impl_with_val!(
                Complex,
                $ty,
                (Rug, rug::Complex::with_val),
                (Fastnum, CDecimal::with_val),
                (F64, |_, x| CF64(x as f64, 0.0)),
                (F32, |_, x| CF32(x as f32, 0.0))
            );
            impl_with_val!(
                Float,
                $ty,
                (Rug, rug::Float::with_val),
                (Fastnum, Decimal::with_val),
                (F64, |_, x| x as f64),
                (F32, |_, x| x as f32)
            );
        )*
    };
}

pub(crate) use {
    dec_c_impl, dec_impl, float_impl, impl_c_ops, impl_c_pow, impl_c_rt, impl_cneg, impl_int_ops,
    impl_neg, impl_new_val, impl_new_val_cdeci, impl_new_val_deci, impl_ops, impl_partial_ord,
    impl_pow, impl_self_c_ops, impl_self_ops, impl_sinh_cosh, impl_types, impl_with_val,
    impl_with_val_cdeci, impl_with_val_deci,
};
