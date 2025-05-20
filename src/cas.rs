use crate::complex::{NumStr, quartic};
use crate::{
    complex::NumStr::{
        Comma, Division, Exponent, Func, LeftBracket, LeftCurlyBracket, Minus, Multiplication, Num,
        Plus, RightBracket, RightCurlyBracket,
    },
    math::do_math,
    units::{Number, Options},
};
use rug::Complex;
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
        Num(_) | Plus | Multiplication | Minus | Division | Exponent | LeftBracket
        | RightBracket => true,
        _ => false,
    })
}
fn poly_div(
    mut remainder: [Complex; 5],
    divisor: [Complex; 5],
) -> Result<([Complex; 5], Option<[Complex; 5]>), &'static str> {
    fn last_non_zero(a: &[Complex; 5]) -> Result<usize, &'static str> {
        for i in (0..5).rev() {
            if !a[i].is_zero() {
                return Ok(i);
            }
        }
        Err("zero polynomial")
    }
    let prec = remainder[0].prec();
    let zero = || Complex::new(prec);
    let mut quotient = [zero(), zero(), zero(), zero(), zero()];
    let d_div = last_non_zero(&divisor)?;
    let lead_div = divisor[d_div].clone();
    let mut d_rem = match last_non_zero(&remainder) {
        Ok(d) => d,
        Err(_) => return Ok((quotient, None)),
    };
    while d_rem >= d_div {
        let shift = d_rem - d_div;
        let coeff = remainder[d_rem].clone() / lead_div.clone();
        for k in 0..=d_div {
            remainder[k + shift] -= coeff.clone() * divisor[k].clone();
        }
        quotient[shift] = coeff;
        d_rem = match last_non_zero(&remainder) {
            Ok(d) => d,
            Err(_) => {
                return Ok((quotient, None));
            }
        };
    }
    Ok((quotient, Some(remainder)))
}
fn get_polynomial(
    func: &[NumStr],
    options: &Options,
    var: String,
) -> Result<[Complex; 5], &'static str> {
    if is_interior(func) {
        return get_polynomial(&func[1..func.len() - 1], options, var);
    }
    let zero = || Complex::new(options.prec);
    let mut arr = [zero(), zero(), zero(), zero(), zero()];
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
            arr = get_polynomial(p, options, var.clone())?;
            continue;
        }
        if is_constant(p, var.clone()) {
            arr[0] -= do_math(p.to_vec(), *options, Vec::new())?.num()?.number
        } else {
            let q = get_polynomial(p, options, var.clone())?;
            arr.iter_mut().zip(q.into_iter()).for_each(|(a, b)| *a -= b);
        }
    }
    if !is_empty {
        return Ok(arr);
    }
    let list = place(func, &Multiplication, false);
    let is_empty = list.is_empty();
    if !is_empty {
        arr[0] += 1;
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
            arr = get_polynomial(p, options, var.clone())?;
            continue;
        }
        if is_constant(p, var.clone()) {
            for a in arr.iter_mut() {
                *a /= do_math(p.to_vec(), *options, Vec::new())?.num()?.number
            }
        } else {
            let p = get_polynomial(p, options, var.clone())?;
            arr = poly_div(arr, p)?.0;
        }
    }
    if !is_empty {
        return Ok(arr);
    }
    let mut list = place(func, &Exponent, true);
    let is_empty = list.is_empty();
    if !is_empty {
        let p = list.remove(0);
        let p = get_polynomial(p, options, var.clone())?;
        let k = do_math(list.remove(0).to_vec(), *options, Vec::new())?
            .num()?
            .number
            .into_real_imag()
            .0
            .to_integer()
            .unwrap_or_default();
        arr = p.clone();
        if k > 0 {
            let mut i = rug::Integer::from(1);
            while i < k {
                poly_mult(&mut arr, &p)?;
                i += 1;
            }
        }
    }
    if !is_empty {
        return Ok(arr);
    }
    if func[0] == Func(var) {
        arr[1] += 1
    }
    Ok(arr)
}
fn poly_mult(arr: &mut [Complex; 5], p: &[Complex; 5]) -> Result<(), &'static str> {
    let prec = p[0].prec();
    let zero = || Complex::new(prec);
    let q = std::mem::replace(arr, [zero(), zero(), zero(), zero(), zero()]);
    for (j, b) in p.iter().enumerate() {
        if b.is_zero() {
            continue;
        }
        for (i, a) in q.iter().enumerate() {
            if a.is_zero() {
                continue;
            }
            if i + j > 4 {
                return Err("poly too high");
            }
            arr[i + j] += a * b.clone()
        }
    }
    Ok(())
}
fn poly_mul(
    options: &Options,
    var: String,
    arr: &mut [Complex; 5],
    p: &[NumStr],
) -> Result<(), &'static str> {
    if is_constant(p, var.clone()) {
        for a in arr.iter_mut() {
            *a *= do_math(p.to_vec(), *options, Vec::new())?.num()?.number
        }
    } else {
        let p = get_polynomial(p, options, var.clone())?;
        poly_mult(arr, &p)?
    }
    Ok(())
}
fn poly_add(
    options: &Options,
    var: String,
    arr: &mut [Complex; 5],
    p: &[NumStr],
) -> Result<(), &'static str> {
    if is_constant(p, var.clone()) {
        arr[0] += do_math(p.to_vec(), *options, Vec::new())?.num()?.number
    } else {
        let q = get_polynomial(p, options, var.clone())?;
        arr.iter_mut().zip(q.iter()).for_each(|(a, b)| *a += b);
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
        let [e, d, c, b, a] = get_polynomial(func, options, var)?.map(|a| Number::from(a, None));
        let r = quartic(a, b, c, d, e, false);
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
