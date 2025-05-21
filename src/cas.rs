use crate::complex::{NumStr, cubic, quadratic, quartic};
use crate::{
    complex::NumStr::{
        Comma, Division, Exponent, Func, InternalMultiplication, LeftBracket, LeftCurlyBracket,
        Minus, Multiplication, Num, Plus, RightBracket, RightCurlyBracket,
    },
    math::do_math,
    units::{Number, Options},
};
use rug::Complex;
use std::cmp::Ordering;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
#[derive(Clone, Default)]
struct Polynomial {
    quotient: Vec<Complex>,
    divisor: Vec<Complex>,
}
fn mul(mut lhs: Vec<Complex>, rhs: Complex) -> Vec<Complex> {
    lhs.iter_mut().for_each(|lhs| *lhs *= rhs.clone());
    lhs
}
fn mul_assign(lhs: &mut Vec<Complex>, rhs: &[Complex]) {
    let v =
        vec![Complex::new(rhs.first().map(|a| a.prec().0).unwrap_or(1)); rhs.len() + lhs.len() - 1];
    let q = std::mem::replace(lhs, v);
    for (j, b) in rhs.iter().enumerate() {
        if b.is_zero() {
            continue;
        }
        for (i, a) in q.iter().enumerate() {
            if a.is_zero() {
                continue;
            }
            lhs[i + j] += a * b.clone()
        }
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl SubAssign<Complex> for Polynomial {
    fn sub_assign(&mut self, rhs: Complex) {
        let prec = rhs.prec();
        let divisor = mul(self.divisor.clone(), rhs);
        self.quotient.extend(vec![
            Complex::new(prec);
            (divisor.len() + 1).saturating_sub(self.quotient.len())
        ]);
        self.quotient
            .iter_mut()
            .zip(divisor)
            .for_each(|(a, b)| *a -= b);
        self.simplify()
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl SubAssign<Self> for Polynomial {
    fn sub_assign(&mut self, mut rhs: Self) {
        mul_assign(&mut self.quotient, rhs.divisor.as_slice());
        mul_assign(&mut rhs.quotient, self.divisor.as_slice());
        self.quotient.extend(vec![
            Complex::new(self.divisor[0].prec());
            (rhs.quotient.len() + 1)
                .saturating_sub(self.quotient.len())
        ]);
        self.quotient
            .iter_mut()
            .zip(rhs.quotient)
            .for_each(|(a, b)| *a -= b);
        mul_assign(&mut self.divisor, rhs.divisor.as_slice());
        self.simplify()
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<Complex> for Polynomial {
    fn add_assign(&mut self, rhs: Complex) {
        let prec = rhs.prec();
        let divisor = mul(self.divisor.clone(), rhs);
        self.quotient.extend(vec![
            Complex::new(prec);
            (divisor.len() + 1).saturating_sub(self.quotient.len())
        ]);
        self.quotient
            .iter_mut()
            .zip(divisor)
            .for_each(|(a, b)| *a += b);
        self.simplify()
    }
}
impl AddAssign<Self> for Polynomial {
    fn add_assign(&mut self, mut rhs: Self) {
        mul_assign(&mut self.quotient, rhs.divisor.as_slice());
        mul_assign(&mut rhs.quotient, self.divisor.as_slice());
        self.quotient.extend(vec![
            Complex::new(self.divisor[0].prec());
            (rhs.quotient.len() + 1)
                .saturating_sub(self.quotient.len())
        ]);
        self.quotient
            .iter_mut()
            .zip(rhs.quotient)
            .for_each(|(a, b)| *a += b);
        mul_assign(&mut self.divisor, rhs.divisor.as_slice());
        self.simplify()
    }
}
impl DivAssign<Complex> for Polynomial {
    fn div_assign(&mut self, rhs: Complex) {
        self.divisor.iter_mut().for_each(|a| *a *= rhs.clone())
    }
}
impl MulAssign<Complex> for Polynomial {
    fn mul_assign(&mut self, rhs: Complex) {
        self.quotient.iter_mut().for_each(|a| *a *= rhs.clone())
    }
}
impl MulAssign<&Self> for Polynomial {
    fn mul_assign(&mut self, rhs: &Self) {
        mul_assign(&mut self.quotient, rhs.quotient.as_slice());
        mul_assign(&mut self.divisor, rhs.divisor.as_slice());
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl DivAssign<Self> for Polynomial {
    fn div_assign(&mut self, rhs: Self) {
        *self *= &rhs.recip()
    }
}
impl From<Vec<Complex>> for Polynomial {
    fn from(quotient: Vec<Complex>) -> Self {
        let prec = quotient[0].prec();
        let divisor = vec![Complex::with_val(prec, 1)];
        Polynomial { quotient, divisor }
    }
}
impl From<(Vec<Complex>, Vec<Complex>)> for Polynomial {
    fn from(value: (Vec<Complex>, Vec<Complex>)) -> Self {
        let (quotient, divisor) = value;
        Polynomial { quotient, divisor }
    }
}
impl Polynomial {
    fn new(prec: u32) -> Self {
        let divisor = vec![Complex::with_val(prec, 1)];
        Self {
            quotient: Vec::new(),
            divisor,
        }
    }
    fn recip(self) -> Self {
        Polynomial {
            quotient: self.divisor,
            divisor: self.quotient,
        }
    }
    fn simplify(&mut self) {
        while self.quotient.last().map(|a| a.is_zero()).unwrap_or(false) {
            self.quotient.pop();
        }
    }
    fn degree(&self) -> (Option<usize>, Option<usize>) {
        fn last_non_zero(a: &[Complex]) -> Option<usize> {
            if a.is_empty() {
                None
            } else {
                Some(a.len() - 1)
            }
        }
        (last_non_zero(&self.quotient), last_non_zero(&self.divisor))
    }
    fn div_checked(mut self) -> Result<(Vec<Complex>, Option<Vec<Complex>>), &'static str> {
        let (d_rem, Some(d_div)) = self.degree() else {
            return Err("zero divisor");
        };
        if d_div == 0 {
            return Ok((self.quotient, None));
        }
        let lead_div = self.divisor[d_div].clone();
        let mut d_rem = match d_rem {
            Some(d) => d,
            None => return Ok((Vec::new(), None)),
        };
        let mut quotient =
            vec![Complex::new(self.quotient[0].prec()); (d_rem + 1).saturating_sub(d_div)];
        while d_rem >= d_div {
            let shift = d_rem - d_div;
            let coeff = self.quotient[d_rem].clone() / lead_div.clone();
            for k in 0..d_div {
                self.quotient[k + shift] -= coeff.clone() * self.divisor[k].clone();
            }
            self.quotient.pop();
            self.simplify();
            quotient[shift] = coeff;
            d_rem = match self.degree().0 {
                Some(d) => d,
                None => {
                    return Ok((quotient, None));
                }
            };
        }
        Ok((self.divisor, Some(self.quotient)))
    }
    fn gcd(mut self) -> Result<Vec<Complex>, &'static str> {
        let mut d = self.divisor.clone();
        while let Ok((_, Some(r))) = std::mem::take(&mut self).div_checked() {
            self.quotient = d;
            self.divisor = r.clone();
            d = r;
        }
        Ok(d)
    }
    fn compute(mut self) -> Result<Vec<Complex>, &'static str> {
        self.divisor = self.clone().gcd()?;
        Ok(self.div_checked()?.0)
    }
    fn get_polynomial(
        func: &[NumStr],
        options: &Options,
        var: String,
    ) -> Result<Self, &'static str> {
        if is_interior(func) {
            return Self::get_polynomial(&func[1..func.len() - 1], options, var);
        }
        let mut arr = Polynomial::new(options.prec);
        if is_constant(func, var.clone()) {
            arr.quotient
                .push(do_math(func.to_vec(), *options, Vec::new())?.num()?.number);
            return Ok(arr);
        }
        if func == vec![Func(var.clone())] {
            arr.quotient.push(Complex::new(options.prec));
            arr.quotient.push(Complex::with_val(options.prec, 1));
            return Ok(arr);
        }
        let list = place(func, &Plus, false);
        let is_empty = list.is_empty();
        for p in list {
            poly_add(options, var.clone(), &mut arr, p)?;
        }
        if !is_empty {
            return Ok(arr);
        }
        let list = place(func, &Minus, false);
        let is_empty = list.is_empty();
        for (k, p) in list.into_iter().enumerate() {
            if k == 0 {
                arr = Self::get_polynomial(p, options, var.clone())?;
                continue;
            }
            if is_constant(p, var.clone()) {
                arr -= do_math(p.to_vec(), *options, Vec::new())?.num()?.number
            } else {
                let q = Self::get_polynomial(p, options, var.clone())?;
                arr -= q;
            }
        }
        if !is_empty {
            return Ok(arr);
        }
        let list = place(func, &Multiplication, false);
        let is_empty = list.is_empty();
        if !is_empty {
            arr.quotient.push(Complex::with_val(options.prec, 1));
        }
        for p in list {
            poly_mul(options, var.clone(), &mut arr, p)?;
        }
        if !is_empty {
            return Ok(arr);
        }
        let list = place(func, &Division, false);
        let is_empty = list.is_empty();
        for (k, p) in list.into_iter().enumerate() {
            if k == 0 {
                arr = Self::get_polynomial(p, options, var.clone())?;
                continue;
            }
            if is_constant(p, var.clone()) {
                let d = do_math(p.to_vec(), *options, Vec::new())?.num()?.number;
                if d.is_zero() {
                    return Err("zero divisor");
                }
                arr /= d
            } else {
                let p = Self::get_polynomial(p, options, var.clone())?;
                arr /= p;
            }
        }
        if !is_empty {
            return Ok(arr);
        }
        let mut list = place(func, &Exponent, true);
        let is_empty = list.is_empty();
        if !is_empty {
            let p = list.remove(0);
            let p = Self::get_polynomial(p, options, var.clone())?;
            let k = do_math(list.remove(0).to_vec(), *options, Vec::new())?
                .num()?
                .number
                .into_real_imag()
                .0
                .to_integer()
                .unwrap_or_default();
            match k.cmp0() {
                Ordering::Less => {
                    let mut i = rug::Integer::from(1);
                    let k = -k;
                    arr = p.clone();
                    while i < k {
                        arr *= &p;
                        i += 1;
                    }
                    arr = arr.recip();
                }
                Ordering::Equal => {
                    arr.quotient.push(Complex::with_val(options.prec, 1));
                }
                Ordering::Greater => {
                    arr = p.clone();
                    let mut i = rug::Integer::from(1);
                    while i < k {
                        arr *= &p;
                        i += 1;
                    }
                }
            }
        }
        if !is_empty {
            return Ok(arr);
        }
        Ok(arr)
    }
}
fn place<'a>(func: &'a [NumStr], target: &'a NumStr, once: bool) -> Vec<&'a [NumStr]> {
    let mut b = 0;
    let mut l = 0;
    let mut vec = Vec::new();
    for (i, n) in func.iter().enumerate() {
        match n {
            LeftBracket | LeftCurlyBracket => b += 1,
            RightBracket | RightCurlyBracket => b -= 1,
            _ if b == 0 && n == target => {
                vec.push(&func[l..i]);
                l = i + 1;
                if once {
                    vec.push(&func[l..]);
                    return vec;
                }
            }
            _ => {}
        }
    }
    if l != 0 {
        vec.push(&func[l..]);
    }
    vec
}
fn is_interior(func: &[NumStr]) -> bool {
    let mut b = 0;
    if func[0] == LeftBracket && func[func.len() - 1] == RightBracket {
        for n in func {
            match n {
                LeftBracket => b += 1,
                RightBracket => b -= 1,
                _ if b == 0 => return false,
                _ => {}
            }
        }
        true
    } else {
        false
    }
}
fn is_constant(func: &[NumStr], var: String) -> bool {
    !func.contains(&Func(var))
}
fn is_poly(func: &[NumStr], var: &str) -> bool {
    func.iter().all(|f| match f {
        Func(a) => a == var,
        Num(_)
        | Plus
        | Multiplication
        | InternalMultiplication
        | Minus
        | Division
        | Exponent
        | LeftBracket
        | RightBracket => true,
        _ => false,
    })
}
fn poly_mul(
    options: &Options,
    var: String,
    arr: &mut Polynomial,
    p: &[NumStr],
) -> Result<(), &'static str> {
    if is_constant(p, var.clone()) {
        *arr *= do_math(p.to_vec(), *options, Vec::new())?.num()?.number
    } else {
        let p = Polynomial::get_polynomial(p, options, var.clone())?;
        *arr *= &p
    }
    Ok(())
}
fn poly_add(
    options: &Options,
    var: String,
    arr: &mut Polynomial,
    p: &[NumStr],
) -> Result<(), &'static str> {
    if is_constant(p, var.clone()) {
        *arr += do_math(p.to_vec(), *options, Vec::new())?.num()?.number
    } else {
        let q = Polynomial::get_polynomial(p, options, var.clone())?;
        *arr += q
    }
    Ok(())
}
fn isolate_inner(
    func: &[NumStr],
    options: &Options,
    var: String,
) -> Result<Vec<NumStr>, &'static str> {
    if is_interior(func) {
        return isolate_inner(&func[1..func.len() - 1], options, var);
    }
    if is_poly(func, &var) {
        let p: Vec<Complex> = Polynomial::get_polynomial(func, options, var)?.compute()?;
        let l = p.len();
        let mut p = p.into_iter();
        let n = |c: Complex| Number::from(c, None);
        let r = match l {
            0 | 1 => vec![Number::from(
                Complex::with_val(options.prec, rug::float::Special::Nan),
                None,
            )],
            2 => {
                let a = p.next().unwrap();
                let b = p.next().unwrap();
                vec![Number::from(-a / b, None)]
            }
            3 => {
                let c = p.next().unwrap();
                let b = p.next().unwrap();
                let a = p.next().unwrap();
                quadratic(n(a), n(b), n(c), false)
            }
            4 => {
                let d = p.next().unwrap();
                let c = p.next().unwrap();
                let b = p.next().unwrap();
                let a = p.next().unwrap();
                cubic(n(a), n(b), n(c), n(d), false)
            }
            5 => {
                let e = p.next().unwrap();
                let d = p.next().unwrap();
                let c = p.next().unwrap();
                let b = p.next().unwrap();
                let a = p.next().unwrap();
                quartic(n(a), n(b), n(c), n(d), n(e), false)
            }
            _ => return Err("poly greater then quartic"),
        };
        let mut v = Vec::new();
        v.push(LeftCurlyBracket);
        for o in r.into_iter().map(|a| Num(Box::new(a))) {
            v.push(o);
            v.push(Comma)
        }
        v.pop();
        v.push(RightCurlyBracket);
        return Ok(v);
    }
    let mut v = Vec::new();
    let list = place(func, &Plus, false);
    let mut some = false;
    let empty = list.is_empty();
    for p in list {
        if is_constant(p, var.clone()) {
            v.push(LeftBracket);
            v.push(Num(Box::new(Number::new(options))));
            v.push(Minus);
            v.push(LeftBracket);
            v.extend_from_slice(p);
            v.push(RightBracket);
            v.push(RightBracket);
            v.push(Plus);
            some = true;
        } else {
            let list = isolate_inner(p, options, var.clone())?;
            if !list.is_empty() && list != vec![Func(var.clone())] {
                v.extend(list);
                v.push(Plus);
                some = true;
            }
        }
    }
    if some {
        v.pop();
    }
    if !empty {
        return Ok(v);
    }
    Ok(v)
}
pub fn isolate(
    func: &[NumStr],
    func_vars: Vec<(String, Vec<NumStr>)>,
    options: Options,
    var: String,
) -> Result<NumStr, &'static str> {
    if func.is_empty() {
        return Err("nothing to isolate");
    }
    do_math(isolate_inner(func, &options, var)?, options, func_vars)
}
