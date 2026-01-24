use crate::complex::NumStr::Vector;
use crate::complex::{NumStr, cubic, quadratic, quartic, unity};
use crate::types::Constant;
use crate::{
    complex::NumStr::{
        Division, Exponent, Func, InternalMultiplication, LeftBracket, LeftCurlyBracket, Minus,
        Multiplication, Num, Plus, RightBracket, RightCurlyBracket,
    },
    math::do_math,
    units::{Number, Options},
};
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
#[derive(Clone, Default)]
struct Polynomial<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> {
    quotient: Vec<Complex>,
    divisor: Vec<Complex>,
    phantom: PhantomData<(Integer, Float)>,
}
fn mul<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    mut lhs: Vec<Complex>,
    rhs: Complex,
) -> Vec<Complex> {
    lhs.iter_mut().for_each(|lhs| *lhs *= rhs.clone());
    lhs
}
fn mul_assign<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    lhs: &mut Vec<Complex>,
    rhs: &[Complex],
) {
    let v =
        vec![Complex::new(rhs.first().map(|a| a.prec()).unwrap_or(1)); rhs.len() + lhs.len() - 1];
    let q = std::mem::replace(lhs, v);
    for (j, b) in rhs.iter().enumerate() {
        if b.is_zero() {
            continue;
        }
        for (i, a) in q.iter().enumerate() {
            if a.is_zero() {
                continue;
            }
            lhs[i + j] += a.clone() * b
        }
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> SubAssign<Complex> for Polynomial<Integer, Float, Complex>
{
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
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> SubAssign<Self> for Polynomial<Integer, Float, Complex>
{
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
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> AddAssign<Complex> for Polynomial<Integer, Float, Complex>
{
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
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> AddAssign<Self> for Polynomial<Integer, Float, Complex>
{
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
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> DivAssign<Complex> for Polynomial<Integer, Float, Complex>
{
    fn div_assign(&mut self, rhs: Complex) {
        self.divisor.iter_mut().for_each(|a| *a *= rhs.clone())
    }
}
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> MulAssign<Complex> for Polynomial<Integer, Float, Complex>
{
    fn mul_assign(&mut self, rhs: Complex) {
        self.quotient.iter_mut().for_each(|a| *a *= rhs.clone())
    }
}
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> MulAssign<&Self> for Polynomial<Integer, Float, Complex>
{
    fn mul_assign(&mut self, rhs: &Self) {
        mul_assign(&mut self.quotient, rhs.quotient.as_slice());
        mul_assign(&mut self.divisor, rhs.divisor.as_slice());
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> DivAssign<Self> for Polynomial<Integer, Float, Complex>
{
    fn div_assign(&mut self, rhs: Self) {
        *self *= &rhs.recip()
    }
}
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> From<Vec<Complex>> for Polynomial<Integer, Float, Complex>
{
    fn from(quotient: Vec<Complex>) -> Self {
        let prec = quotient[0].prec();
        let divisor = vec![Complex::with_val(prec, 1)];
        Polynomial {
            quotient,
            divisor,
            phantom: PhantomData,
        }
    }
}
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> From<(Vec<Complex>, Vec<Complex>)> for Polynomial<Integer, Float, Complex>
{
    fn from(value: (Vec<Complex>, Vec<Complex>)) -> Self {
        let (quotient, divisor) = value;
        Polynomial {
            quotient,
            divisor,
            phantom: PhantomData,
        }
    }
}
impl<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
> Polynomial<Integer, Float, Complex>
{
    fn new(prec: u32) -> Self {
        let divisor = vec![Complex::with_val(prec, 1)];
        Self {
            quotient: Vec::new(),
            divisor,
            phantom: PhantomData,
        }
    }
    fn recip(self) -> Self {
        Polynomial {
            quotient: self.divisor,
            divisor: self.quotient,
            phantom: PhantomData,
        }
    }
    fn simplify(&mut self) {
        while self.quotient.last().map(|a| a.is_zero()).unwrap_or(false) {
            self.quotient.pop();
        }
    }
    fn degree(&self) -> (Option<usize>, Option<usize>) {
        fn last_non_zero<
            I: crate::types::Integer<F, C>,
            F: crate::types::Float<I, C>,
            C: crate::types::Complex<I, F>,
        >(
            a: &[C],
        ) -> Option<usize> {
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
        while let Ok((_, Some(r))) = std::mem::replace(
            &mut self,
            Self {
                quotient: Vec::new(),
                divisor: Vec::new(),
                phantom: PhantomData,
            },
        )
        .div_checked()
        {
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
        func: &[NumStr<Integer, Float, Complex>],
        options: &Options,
        var: &[NumStr<Integer, Float, Complex>],
    ) -> Result<Self, &'static str> {
        if is_interior(func) {
            return Self::get_polynomial(&func[1..func.len() - 1], options, var);
        }
        let mut arr = Polynomial::new(options.prec);
        if is_constant(func, var) {
            arr.quotient
                .push(do_math(func.to_vec(), *options, Vec::new())?.num()?.number);
            return Ok(arr);
        }
        if func == var {
            arr.quotient.push(Complex::new(options.prec));
            arr.quotient.push(Complex::with_val(options.prec, 1));
            return Ok(arr);
        }
        let list = place(func, &Plus, false);
        let is_empty = list.is_empty();
        for p in list {
            poly_add(options, var, &mut arr, p)?;
        }
        if !is_empty {
            return Ok(arr);
        }
        let list = place(func, &Minus, false);
        let is_empty = list.is_empty();
        for (k, p) in list.into_iter().enumerate() {
            if k == 0 {
                arr = Self::get_polynomial(p, options, var)?;
                continue;
            }
            if is_constant(p, var) {
                arr -= do_math(p.to_vec(), *options, Vec::new())?.num()?.number
            } else {
                let q = Self::get_polynomial(p, options, var)?;
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
            poly_mul(options, var, &mut arr, p)?;
        }
        if !is_empty {
            return Ok(arr);
        }
        let list = place(func, &Division, false);
        let is_empty = list.is_empty();
        for (k, p) in list.into_iter().enumerate() {
            if k == 0 {
                arr = Self::get_polynomial(p, options, var)?;
                continue;
            }
            if is_constant(p, var) {
                let d = do_math(p.to_vec(), *options, Vec::new())?.num()?.number;
                if d.is_zero() {
                    return Err("zero divisor");
                }
                arr /= d
            } else {
                let p = Self::get_polynomial(p, options, var)?;
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
            let p = Self::get_polynomial(p, options, var)?;
            let k = do_math(list.remove(0).to_vec(), *options, Vec::new())?
                .num()?
                .number
                .into_real_imag();
            if !k.1.is_zero() || !k.0.clone().fract().is_zero() {
                return Err("non integer exponent");
            }
            let k = k.0.to_integer().unwrap_or_default();
            match k.cmp0() {
                Ordering::Less => {
                    let mut i = Integer::from(1);
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
                    let mut i = Integer::from(1);
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
        Err("not poly")
    }
}
fn is_poly<
    I: crate::types::Integer<F, C>,
    F: crate::types::Float<I, C>,
    C: crate::types::Complex<I, F>,
>(
    func: &[NumStr<I, F, C>],
    var: &[NumStr<I, F, C>],
) -> bool {
    let a = func.len();
    let b = var.len();
    if a >= b {
        let mut i = 0;
        while i <= a - b {
            if &func[i..i + b] == var {
                i += b
            } else {
                if !matches!(
                    func[i],
                    Num(_)
                        | Plus
                        | Multiplication
                        | InternalMultiplication
                        | Minus
                        | Division
                        | Exponent
                        | LeftBracket
                        | RightBracket
                ) {
                    return false;
                }
                i += 1
            }
        }
        true
    } else {
        func.iter().all(|f| {
            matches!(
                f,
                Num(_)
                    | Plus
                    | Multiplication
                    | InternalMultiplication
                    | Minus
                    | Division
                    | Exponent
                    | LeftBracket
                    | RightBracket
            )
        })
    }
}
fn poly_mul<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    options: &Options,
    var: &[NumStr<Integer, Float, Complex>],
    arr: &mut Polynomial<Integer, Float, Complex>,
    p: &[NumStr<Integer, Float, Complex>],
) -> Result<(), &'static str> {
    if is_constant(p, var) {
        *arr *= do_math(p.to_vec(), *options, Vec::new())?.num()?.number
    } else {
        let p = Polynomial::get_polynomial(p, options, var)?;
        *arr *= &p
    }
    Ok(())
}
fn poly_add<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    options: &Options,
    var: &[NumStr<Integer, Float, Complex>],
    arr: &mut Polynomial<Integer, Float, Complex>,
    p: &[NumStr<Integer, Float, Complex>],
) -> Result<(), &'static str> {
    if is_constant(p, var) {
        *arr += do_math(p.to_vec(), *options, Vec::new())?.num()?.number
    } else {
        let q = Polynomial::get_polynomial(p, options, var)?;
        *arr += q
    }
    Ok(())
}
fn place<
    'a,
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    func: &'a [NumStr<Integer, Float, Complex>],
    target: &'a NumStr<Integer, Float, Complex>,
    once: bool,
) -> Vec<&'a [NumStr<Integer, Float, Complex>]> {
    let mut b = 0;
    let mut l = 0;
    let mut vec = Vec::new();
    for (i, n) in func.iter().enumerate() {
        match n {
            LeftBracket | LeftCurlyBracket => b += 1,
            RightBracket | RightCurlyBracket => b -= 1,
            _ if b == 0
                && (n == target
                    || if target == &Multiplication {
                        n == &InternalMultiplication
                    } else {
                        false
                    }) =>
            {
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
fn is_interior<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    func: &[NumStr<Integer, Float, Complex>],
) -> bool {
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
fn is_constant<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    func: &[NumStr<Integer, Float, Complex>],
    var: &[NumStr<Integer, Float, Complex>],
) -> bool {
    let a = func.len();
    let b = var.len();
    if a >= b {
        let mut i = 0;
        while i <= a - b {
            if &func[i..i + b] == var {
                return false;
            }
            i += 1
        }
    }
    true
}
fn get_var<
    'a,
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    func: &'a [NumStr<Integer, Float, Complex>],
    var: &'a [NumStr<Integer, Float, Complex>],
) -> &'a [NumStr<Integer, Float, Complex>] {
    let a = func.len();
    let b = var.len();
    let mut values = Vec::new();
    if a >= b {
        let mut i = 0;
        while i <= a - b {
            if &func[i..i + b] == var {
                values.push(i);
                i += b;
            } else {
                i += 1
            }
        }
    }
    let mut i = 0;
    let mut j = b;
    while values.iter().all(|k| {
        values[0] > i
            && func[k - i - 1] == func[values[0] - i - 1]
            && matches!(func[k - i - 1], LeftBracket)
    }) && values.iter().all(|k| {
        func.len() > k + j
            && func[k + j] == func[values[0] + j]
            && matches!(func[k + j], RightBracket)
    }) {
        i += 1;
        if values.iter().all(|k| {
            values[0] > i
                && func[k - i - 1] == func[values[0] - i - 1]
                && matches!(func[k - i - 1], Func(_))
        }) {
            i += 1;
        }
        j += 1;
    }
    &func[values[0] - i..values[0] + j]
}
fn to_vec<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    a: NumStr<Integer, Float, Complex>,
) -> Vec<Number<Integer, Float, Complex>> {
    match a {
        Num(a) => vec![*a],
        Vector(a) => a,
        _ => unreachable!(),
    }
}
fn inverse<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    func: &[NumStr<Integer, Float, Complex>],
    val: Vec<Number<Integer, Float, Complex>>,
    options: &Options,
) -> Result<Vec<Number<Integer, Float, Complex>>, &'static str> {
    if func.len() > 1 {
        let Func(f) = &func[0] else { unreachable!() };
        let v = match f.as_str() {
            "sin" => {
                let pi = Float::with_val(val[0].number.prec(), Constant::Pi);
                val.into_iter()
                    .flat_map(|a| {
                        let a = a.number.asin();
                        vec![a.clone(), pi.clone() - a]
                    })
                    .map(|a| Number::from(a, None))
                    .collect()
            }
            "cos" => val
                .into_iter()
                .flat_map(|a| {
                    let a = a.number.acos();
                    vec![a.clone(), -a]
                })
                .map(|a| Number::from(a, None))
                .collect(),
            _ => {
                let inv = "a".to_owned() + f;
                let v = vec![Func(inv), LeftBracket, Vector(val), RightBracket];
                to_vec(do_math(v, *options, Vec::new())?)
            }
        };
        if func.len() == 2 {
            Ok(v)
        } else {
            inverse(&func[2..func.len() - 1], v, options)
        }
    } else {
        Ok(val)
    }
}
fn isolate_inner<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    func: &[NumStr<Integer, Float, Complex>],
    options: &Options,
    var: &[NumStr<Integer, Float, Complex>],
) -> Result<Vec<NumStr<Integer, Float, Complex>>, &'static str> {
    if is_interior(func) {
        return isolate_inner(&func[1..func.len() - 1], options, var);
    }
    if matches!(func[0], Func(_)) && func.len() > 1 && is_interior(&func[1..]) {
        let v = to_vec(do_math(
            isolate_inner(&func[2..func.len() - 1], options, var)?,
            *options,
            Vec::new(),
        )?);
        let m = inverse(&func[0..2], v, options)?;
        return Ok(vec![Vector(m)]);
    }
    let var = get_var(func, var);
    if is_poly(func, var) {
        let mut p: Vec<Complex> = Polynomial::get_polynomial(func, options, var)?.compute()?;
        let mut mult = 1;
        let mut r = Vec::with_capacity(p.len());
        while p.len() > 1 && p[0].is_zero() && p[1].is_zero() {
            p.remove(0);
            r.push(Number::new(options))
        }
        if p.len() > 5 {
            let powers = p
                .iter()
                .enumerate()
                .filter_map(|(i, n)| if n.is_zero() && i != 0 { None } else { Some(i) })
                .collect::<Vec<usize>>();
            if powers.len() >= 2 {
                mult = powers[1] - powers[0];
                for (i, p) in powers[1..].iter().enumerate() {
                    if let Some(q) = powers.get(i + 2) {
                        if q - p != mult {
                            mult = 1;
                            break;
                        }
                    }
                }
            }
        }
        let l = p.len().div_ceil(mult);
        let mut p = p
            .into_iter()
            .enumerate()
            .filter_map(|(i, a)| if i % mult == 0 { Some(a) } else { None });
        let n = |c: Complex| Number::from(c, None);
        r.extend(match l {
            0 | 1 => vec![Number::from(
                Complex::with_val(options.prec, Constant::Nan),
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
            _ => {
                //TODO newtons method
                return Err("poly greater then quartic");
            }
        });
        r.sort_unstable_by(|a, b| a.number.total_cmp(&b.number));
        let mut a = Vec::new();
        if mult != 1 {
            let m = Complex::with_val(options.prec, mult);
            for n in r.into_iter() {
                a.extend(unity(n.number, m.clone()))
            }
        } else {
            a = r
        }
        return Ok(vec![Vector(if var.len() == 1 {
            a
        } else {
            inverse(var, a, options)?
        })]);
    }
    let mut v = Vec::new();
    let list = place(func, &Plus, false);
    let mut some = false;
    let empty = list.is_empty();
    for p in list {
        if is_constant(p, var) {
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
            let list = isolate_inner(p, options, var)?;
            if !list.is_empty() && list != var {
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
#[allow(clippy::type_complexity)]
pub fn isolate<
    Integer: crate::types::Integer<Float, Complex>,
    Float: crate::types::Float<Integer, Complex>,
    Complex: crate::types::Complex<Integer, Float>,
>(
    func: &[NumStr<Integer, Float, Complex>],
    func_vars: Vec<(String, Vec<NumStr<Integer, Float, Complex>>)>,
    options: Options,
    var: String,
) -> Result<NumStr<Integer, Float, Complex>, &'static str> {
    if func.iter().all(|f| match f {
        Func(v) => v != &var,
        _ => true,
    }) {
        return Err("nothing to isolate");
    }
    do_math(
        isolate_inner(func, &options, &[Func(var)])?,
        options,
        func_vars,
    )
}
