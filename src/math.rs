use crate::{
    complex::{
        LimSide::{Both, Left, Right},
        NumStr,
        NumStr::{
            And, Comma, Converse, Conversion, Division, Equal, Exponent, Func, Greater,
            GreaterEqual, Implies, InternalMultiplication, LeftBracket, LeftCurlyBracket, Lesser,
            LesserEqual, Matrix, Minus, Modulo, Multiplication, Nand, NearEqual, Nor, Not,
            NotEqual, Num, Or, Plus, PlusMinus, Range, RightBracket, RightCurlyBracket, Root,
            ShiftLeft, ShiftRight, Tetration, Vector, Xor,
        },
        about_eq, add, and, area, atan, binomial, change_basis, cofactor, coordinate, cube, cubic,
        determinant, digamma, div, eigenvalues, eigenvectors, eq, erf, erfc, eta, euleriannumbers,
        euleriannumbersint, extrema, gamma, gcd, ge, generalized_eigenvectors, gt, hsv2rgb,
        identity, implies, incomplete_beta, incomplete_gamma, inverse, iter, jcf, kernel, lambertw,
        length, limit, lower_incomplete_gamma, minors, mul_units, mvec, nand, ne, nor, not,
        nth_prime, or, pow_nth, prime_factors, quadratic, quartic, rand_gamma, rand_norm, range,
        rcf, recursion, regularized_incomplete_beta, rem, root, rref, shl, shr, slog, slope, solve,
        sort, sort_mat, sqr, sub, subfactorial, sum, surface_area, taylor, tetration, to, to_cyl,
        to_polar, trace, transpose, unity, variance, xor, zeta,
    },
    fraction::{c_to_rational, rationalize},
    misc::do_math_with_var,
    units::{AngleType, Number, Options, Units},
};
use rug::{
    Complex, Float, Integer,
    float::{
        Constant::Pi,
        Special::{Infinity, Nan},
    },
    integer::IsPrime,
    ops::Pow,
};
use std::{cmp::Ordering, ops::Rem, time::SystemTime};
pub fn do_math(
    mut function: Vec<NumStr>,
    options: Options,
    mut func_vars: Vec<(String, Vec<NumStr>)>,
) -> Result<NumStr, &'static str>
{
    if function.is_empty()
    {
        return Err(" ");
    }
    compute_funcvars(&mut function, options, &mut func_vars);
    let mut i = 0;
    while i < function.len()
    {
        match &function[i]
        {
            LeftCurlyBracket =>
            {
                let mut j = i + 1;
                let mut count = 1;
                while count > 0
                {
                    if j >= function.len()
                    {
                        return Err("curly bracket err");
                    }
                    match &function[j]
                    {
                        LeftCurlyBracket => count += 1,
                        RightCurlyBracket => count -= 1,
                        _ =>
                        {}
                    }
                    j += 1;
                }
                if i + 1 == j - 1
                {
                    return Err("no interior vector");
                }
                let mut single = 0;
                let v = &function[i + 1..j - 1];
                let mut vec = Vec::new();
                let mut mat = Vec::<Vec<Number>>::new();
                for (f, n) in v.iter().enumerate()
                {
                    match &n
                    {
                        Comma if count == 0 =>
                        {
                            let z = do_math(v[single..f].to_vec(), options, func_vars.clone())?;
                            match z
                            {
                                Num(n) => vec.push(n),
                                Vector(n) => mat.push(n),
                                _ => return Err("broken matrix"),
                            }
                            single = f + 1;
                        }
                        LeftBracket | LeftCurlyBracket => count += 1,
                        RightBracket | RightCurlyBracket => count -= 1,
                        _ =>
                        {}
                    }
                }
                if single != v.len()
                {
                    let z = do_math(v[single..].to_vec(), options, func_vars.clone())?;
                    match z
                    {
                        Num(n) => vec.push(n),
                        Vector(n) => mat.push(n),
                        _ => return Err("broken matrix"),
                    }
                }
                function.drain(i..j);
                if !mat.is_empty()
                {
                    if vec.is_empty()
                    {
                        function.insert(i, Matrix(mat));
                    }
                    else
                    {
                        return Err("vector err");
                    }
                }
                else
                {
                    function.insert(i, Vector(vec));
                }
            }
            LeftBracket =>
            {
                let mut j = i + 1;
                let mut count = 1;
                while count > 0
                {
                    if j >= function.len()
                    {
                        return Err("round bracket err");
                    }
                    match &function[j]
                    {
                        LeftBracket => count += 1,
                        RightBracket => count -= 1,
                        _ =>
                        {}
                    }
                    j += 1;
                }
                if i + 1 == j - 1
                {
                    return Err("no interior bracket");
                }
                if i != 0
                {
                    if let Func(k) = &function[i - 1]
                    {
                        if matches!(
                            k.as_str(),
                            "next"
                                | "log"
                                | "exp"
                                | "ζ"
                                | "polygamma"
                                | "digamma"
                                | "inter"
                                | "interpolate"
                                | "lobf"
                                | "plane"
                                | "lineofbestfit"
                                | "ψ"
                                | "rotate"
                                | "multinomial"
                                | "gcd"
                                | "gcf"
                                | "lcm"
                                | "ssrt"
                                | "W"
                                | "unity"
                                | "productlog"
                                | "lambertw"
                                | "ceil"
                                | "floor"
                                | "round"
                                | "int"
                                | "trunc"
                                | "frac"
                                | "fract"
                                | "slog"
                                | "root"
                                | "atan"
                                | "arctan"
                                | "atan2"
                                | "normP"
                                | "normD"
                                | "betaP"
                                | "betaC"
                                | "bi"
                                | "binomial"
                                | "angle"
                                | "cross"
                                | "dot"
                                | "part"
                                | "proj"
                                | "project"
                                | "oproj"
                                | "oproject"
                                | "C"
                                | "P"
                                | "Ap"
                                | "An"
                                | "gamma"
                                | "Γ"
                                | "γ"
                                | "lower_gamma"
                                | "ph"
                                | "pochhammer"
                                | "Β"
                                | "B"
                                | "beta"
                                | "I"
                                | "quad"
                                | "quadratic"
                                | "cubic"
                                | "quartic"
                                | "percentilerank"
                                | "percentile"
                                | "eigenvalues"
                                | "eigenvectors"
                                | "generalized_eigenvectors"
                                | "change_basis"
                                | "coordinate"
                                | "mod"
                                | "covariance"
                                | "cov"
                                | "rand_norm"
                                | "rand_uniform"
                                | "rand_int"
                                | "rand_gamma"
                                | "rand_beta"
                                | "gamma_pdf"
                                | "gamma_cdf"
                                | "beta_cdf"
                                | "beta_pdf"
                                | "norm_cdf"
                                | "norm_pdf"
                                | "lognorm_cdf"
                                | "binomial_cdf"
                                | "geometric_cdf"
                                | "lognorm_pdf"
                                | "binomial_pmf"
                                | "geometric_pmf"
                                | "rand_lognorm"
                                | "rand_binomial"
                                | "poisson_pmf"
                                | "poisson_cdf"
                                | "rand_neg_binomial"
                                | "neg_binomial_cdf"
                                | "neg_binomial_pmf"
                                | "hypergeometric_pmf"
                                | "hypergeometric_cdf"
                                | "rand_hypergeometric"
                                | "neg_hypergeometric_pmf"
                                | "neg_hypergeometric_cdf"
                                | "rand_neg_hypergeometric"
                                | "union"
                                | "poly"
                                | "polynomial"
                                | "intersection"
                                | "set_difference"
                                | "symmetric_difference"
                                | "cartesian_product"
                                | "remove"
                                | "extend"
                                | "link"
                                | "subset"
                                | "element"
                        )
                        {
                            function.remove(j - 1);
                            function.remove(i);
                            let v = function.drain(i..j - 2).collect::<Vec<NumStr>>();
                            count = 0;
                            let mut place = Vec::new();
                            for (f, n) in v.iter().enumerate()
                            {
                                match n
                                {
                                    Comma if count == 0 => place.push(f),
                                    LeftBracket | LeftCurlyBracket => count += 1,
                                    RightBracket | RightCurlyBracket => count -= 1,
                                    _ =>
                                    {}
                                }
                            }
                            if !place.is_empty()
                            {
                                let mut func = vec![function[i - 1].clone()];
                                func.push(do_math(
                                    v[..place[0]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?);
                                for (k, l) in place.iter().enumerate()
                                {
                                    func.push(do_math(
                                        v[l + 1
                                            ..if k + 1 != place.len()
                                            {
                                                place[k + 1]
                                            }
                                            else
                                            {
                                                v.len()
                                            }]
                                            .to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?);
                                }
                                function[i - 1] = do_math(func, options, func_vars.clone())?;
                            }
                            else
                            {
                                let v = vec![
                                    function[i - 1].clone(),
                                    do_math(v, options, func_vars.clone())?,
                                ];
                                function[i - 1] = do_math(v, options, func_vars.clone())?;
                            }
                            continue;
                        }
                        else if matches!(
                            k.as_str(),
                            "sum"
                                | "area"
                                | "surfacearea"
                                | "sarea"
                                | "solve"
                                | "∫"
                                | "length"
                                | "slope"
                                | "taylor"
                                | "iter"
                                | "extrema"
                                | "summation"
                                | "prod"
                                | "product"
                                | "Σ"
                                | "Π"
                                | "vec"
                                | "mat"
                                | "piecewise"
                                | "pw"
                                | "D"
                                | "integrate"
                                | "arclength"
                                | "lim"
                                | "limit"
                                | "set"
                        )
                        {
                            i = j - 1;
                            continue;
                        }
                        else
                        {
                            function.remove(j - 1);
                            function.remove(i);
                            function[i - 1] = do_math(
                                vec![
                                    function[i - 1].clone(),
                                    do_math(
                                        function.drain(i..j - 2).collect::<Vec<NumStr>>(),
                                        options,
                                        func_vars.clone(),
                                    )?,
                                ],
                                options,
                                func_vars.clone(),
                            )?;
                            continue;
                        }
                    }
                }
                function.remove(j - 1);
                function[i] = do_math(
                    function.drain(i + 1..j - 1).collect::<Vec<NumStr>>(),
                    options,
                    func_vars.clone(),
                )?;
            }
            _ =>
            {}
        }
        i += 1;
    }
    if function.len() == 1
    {
        if let Func(s) = &function[0]
        {
            if !matches!(s.as_str(), "rnd" | "rand" | "epoch")
            {
                return Ok(function[0].clone());
            }
        }
        else
        {
            return Ok(function[0].clone());
        }
    }
    i = 0;
    let to_deg = match options.angle
    {
        AngleType::Degrees => 180 / Complex::with_val(options.prec, Pi),
        AngleType::Radians => Complex::with_val(options.prec, 1),
        AngleType::Gradians => 200 / Complex::with_val(options.prec, Pi),
    };
    while i < function.len().saturating_sub(1)
    {
        if let Func(s) = &function[i].clone()
        {
            if !matches!(s.as_str(), "epoch" | "rnd" | "rand")
            {
                if matches!(
                    s.as_str(),
                    "sum"
                        | "area"
                        | "sarea"
                        | "surfacearea"
                        | "solve"
                        | "∫"
                        | "length"
                        | "slope"
                        | "taylor"
                        | "iter"
                        | "extrema"
                        | "product"
                        | "prod"
                        | "summation"
                        | "Σ"
                        | "Π"
                        | "vec"
                        | "mat"
                        | "piecewise"
                        | "pw"
                        | "D"
                        | "integrate"
                        | "arclength"
                        | "lim"
                        | "limit"
                        | "set"
                )
                {
                    let mut place = Vec::new();
                    let mut count = 0;
                    let mut count2 = 0;
                    for (f, n) in function[i + 2..].iter().enumerate()
                    {
                        if s == "piecewise" || s == "pw"
                        {
                            match n
                            {
                                Comma if (count == 0 || count == 1) && count2 == 0 =>
                                {
                                    place.push(f + i + 2);
                                }
                                LeftBracket =>
                                {
                                    count += 1;
                                    count2 += 1;
                                }
                                LeftCurlyBracket => count += 1,
                                RightCurlyBracket =>
                                {
                                    if count == 0
                                    {
                                        place.push(f + i + 2);
                                        break;
                                    }
                                    count -= 1;
                                }
                                RightBracket =>
                                {
                                    if count == 0
                                    {
                                        place.push(f + i + 2);
                                        break;
                                    }
                                    count -= 1;
                                    count2 -= 1;
                                }
                                _ =>
                                {}
                            }
                        }
                        else
                        {
                            match n
                            {
                                Comma if count == 0 =>
                                {
                                    place.push(f + i + 2);
                                }
                                LeftBracket | LeftCurlyBracket =>
                                {
                                    count += 1;
                                }
                                RightBracket | RightCurlyBracket =>
                                {
                                    if count == 0
                                    {
                                        place.push(f + i + 2);
                                        break;
                                    }
                                    count -= 1;
                                }
                                _ =>
                                {}
                            }
                        }
                    }
                    match (
                        s.as_str(),
                        if place.is_empty() || place[0] - 1 == i + 1
                        {
                            Func(String::new())
                        }
                        else
                        {
                            function[place[0] - 1].clone()
                        },
                    )
                    {
                        ("iter", Func(var)) if place.len() == 4 || place.len() == 5 =>
                        {
                            function[i] = iter(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?,
                                do_math(
                                    function[place[2] + 1..place[3]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?
                                .number
                                .real()
                                .clone(),
                                place.len() == 5,
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("extrema", Func(var)) if place.len() == 2 || place.len() == 3 =>
                        {
                            function[i] = extrema(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                if place.len() == 3
                                {
                                    do_math(
                                        function[place[1] + 1..place[2]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                }
                                else
                                {
                                    Number::from(Complex::new(options.prec), None)
                                },
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("solve", Func(var)) if place.len() == 2 || place.len() == 3 =>
                        {
                            function[i] = solve(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                if place.len() == 3
                                {
                                    do_math(
                                        function[place[1] + 1..place[2]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                }
                                else
                                {
                                    Number::from(Complex::new(options.prec), None)
                                },
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("set", Func(var)) if place.len() == 3 =>
                        {
                            function[i] = do_math_with_var(
                                function[place[0] + 1..place[1]].to_vec(),
                                options,
                                func_vars.clone(),
                                &var,
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?,
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("lim" | "limit", Func(var)) if place.len() == 3 || place.len() == 4 =>
                        {
                            function[i] = limit(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?,
                                if place.len() == 4
                                {
                                    match do_math(
                                        function[place[2] + 1..place[3]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .number
                                    .real()
                                    .cmp0()
                                    {
                                        Some(Ordering::Less) => Left,
                                        Some(Ordering::Greater) => Right,
                                        _ => Both,
                                    }
                                }
                                else
                                {
                                    Both
                                },
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("surfacearea" | "sarea", Func(var)) if place.len() == 7 =>
                        {
                            if let Func(var2) = &function[place[0] + 1]
                            {
                                function[i] = Num(surface_area(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    func_vars.clone(),
                                    options,
                                    var.to_string(),
                                    do_math(
                                        function[place[4] + 1..place[5]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .number,
                                    do_math(
                                        function[place[5] + 1..place[6]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?,
                                    var2.to_string(),
                                    function[place[2] + 1..place[3]].to_vec(),
                                    function[place[3] + 1..place[4]].to_vec(),
                                )?);
                                function.drain(i + 1..=*place.last().unwrap());
                            }
                            else
                            {
                                return Err("bad var");
                            }
                        }
                        ("length" | "arclength", Func(var)) if place.len() == 4 =>
                        {
                            function[i] = Num(length(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?
                                .number,
                                do_math(
                                    function[place[2] + 1..place[3]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?,
                            )?);
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("∫" | "area" | "integrate", Func(var))
                            if place.len() == 4 || place.len() == 5 || place.len() == 6 =>
                        {
                            function[i] = area(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?
                                .number,
                                do_math(
                                    function[place[2] + 1..place[3]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?,
                                if place.len() == 5
                                {
                                    do_math(
                                        function[place[3] + 1..place[4]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .number
                                }
                                else
                                {
                                    Complex::with_val(options.prec, 1)
                                },
                                place.len() != 6,
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("slope" | "D", Func(var))
                            if place.len() == 3 || place.len() == 4 || place.len() == 5 =>
                        {
                            function[i] = slope(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?,
                                place.len() != 5,
                                if place.len() >= 4
                                {
                                    do_math(
                                        function[place[2] + 1..place[3]].to_vec(),
                                        options,
                                        func_vars.clone(),
                                    )?
                                    .num()?
                                    .number
                                    .real()
                                    .to_integer()
                                    .unwrap_or_default()
                                    .to_u32()
                                    .unwrap_or_default()
                                }
                                else
                                {
                                    1
                                },
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("taylor", Func(var)) if place.len() == 4 || place.len() == 5 =>
                        {
                            function[i] = taylor(
                                function[place[0] + 1..place[1]].to_vec(),
                                func_vars.clone(),
                                options,
                                var.to_string(),
                                if place.len() == 5
                                {
                                    Some(
                                        do_math(
                                            function[place[3] + 1..place[4]].to_vec(),
                                            options,
                                            func_vars.clone(),
                                        )?
                                        .num()?,
                                    )
                                }
                                else
                                {
                                    None
                                },
                                do_math(
                                    function[place[1] + 1..place[2]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?,
                                do_math(
                                    function[place[2] + 1..place[3]].to_vec(),
                                    options,
                                    func_vars.clone(),
                                )?
                                .num()?
                                .number
                                .real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_usize()
                                .unwrap_or_default(),
                            )?;
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        ("pw" | "piecewise", _) if !place.is_empty() =>
                        {
                            let mut ans = None;
                            let mut start = i + 3;
                            for (i, end) in place[0..if place.len() % 2 == 1
                            {
                                place.len()
                            }
                            else
                            {
                                place.len().saturating_sub(1)
                            }]
                                .iter()
                                .enumerate()
                            {
                                if i + 1 == place.len()
                                    || (i % 2 == 0
                                        && do_math(
                                            function[*end + 1..place[i + 1] - 1].to_vec(),
                                            options,
                                            func_vars.clone(),
                                        )?
                                        .num()?
                                        .number
                                        .real()
                                            == &1.0)
                                {
                                    ans = Some(recursion(
                                        func_vars.clone(),
                                        function[if i + 1 == place.len()
                                        {
                                            start.saturating_sub(1)
                                        }
                                        else
                                        {
                                            start
                                        }..*end]
                                            .to_vec(),
                                        options,
                                    )?);
                                    break;
                                }
                                else
                                {
                                    start = end + 2;
                                }
                            }
                            function[i] = if let Some(n) = ans
                            {
                                n
                            }
                            else
                            {
                                Num(Number::from(Complex::with_val(options.prec, Nan), None))
                            };
                            function.drain(i + 1..=*place.last().unwrap());
                        }
                        (
                            "sum" | "product" | "prod" | "summation" | "Σ" | "Π" | "vec" | "mat",
                            Func(var),
                        ) if place.len() == 4 =>
                        {
                            let start = do_math(
                                function[place[1] + 1..place[2]].to_vec(),
                                options,
                                func_vars.clone(),
                            )?
                            .num()?
                            .number;
                            let end = do_math(
                                function[place[2] + 1..place[3]].to_vec(),
                                options,
                                func_vars.clone(),
                            )?
                            .num()?
                            .number;
                            if !start.imag().is_zero() || !end.imag().is_zero()
                            {
                                return Err("imag start/end");
                            }
                            if !start.real().clone().fract().is_zero()
                                || !end.real().clone().fract().is_zero()
                            {
                                return Err("fractional start/end");
                            }
                            let start = start.real();
                            let end = end.real();
                            function[i] = match s.as_str()
                            {
                                "vec" | "mat" => mvec(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    func_vars.clone(),
                                    &var,
                                    start
                                        .to_integer()
                                        .unwrap_or_default()
                                        .to_isize()
                                        .unwrap_or_default(),
                                    end.to_integer()
                                        .unwrap_or_default()
                                        .to_isize()
                                        .unwrap_or_default(),
                                    s == "vec",
                                    options,
                                )?,
                                _ => sum(
                                    function[place[0] + 1..place[1]].to_vec(),
                                    func_vars.clone(),
                                    &var,
                                    start.clone(),
                                    end.clone(),
                                    !(s == "sum" || s == "summation" || s == "Σ"),
                                    options,
                                )?,
                            };
                            function.drain(i + 1..=place[3]);
                        }
                        ("sum" | "summation" | "Σ", _) if place.len() <= 1 =>
                        {
                            function[i] = match if place.is_empty()
                            {
                                Ok(function.remove(i + 1).clone())
                            }
                            else
                            {
                                do_math(
                                    function.drain(i + 1..=place[0]).collect::<Vec<NumStr>>(),
                                    options,
                                    func_vars.clone(),
                                )
                            }
                            {
                                Ok(Num(a)) => Num(a.clone()),
                                Ok(Vector(a)) => Num(Number::from(
                                    a.iter().fold(Complex::new(options.prec), |sum, val| {
                                        sum + val.number.clone()
                                    }),
                                    None,
                                )),
                                Ok(Matrix(a)) => Num(Number::from(
                                    a.iter()
                                        .flatten()
                                        .fold(Complex::new(options.prec), |sum, val| {
                                            sum + val.number.clone()
                                        }),
                                    None,
                                )),
                                _ => return Err("sum err"),
                            };
                        }
                        ("product" | "prod" | "Π", _) if place.len() <= 1 =>
                        {
                            function[i] = match if place.is_empty()
                            {
                                Ok(function.remove(i + 1).clone())
                            }
                            else
                            {
                                do_math(
                                    function.drain(i + 1..=place[0]).collect::<Vec<NumStr>>(),
                                    options,
                                    func_vars.clone(),
                                )
                            }
                            {
                                Ok(Num(a)) => Num(a.clone()),
                                Ok(Vector(a)) => Num(Number::from(
                                    a.iter()
                                        .fold(Complex::with_val(options.prec, 1), |sum, val| {
                                            sum * val.number.clone()
                                        }),
                                    None,
                                )),
                                Ok(Matrix(a)) => Num(Number::from(
                                    a.iter()
                                        .flatten()
                                        .fold(Complex::with_val(options.prec, 1), |sum, val| {
                                            sum * val.number.clone()
                                        }),
                                    None,
                                )),
                                _ => return Err("prod err"),
                            };
                        }
                        (_, _) => return Err("arg/var err with sum/prod/vec/slope or similar"),
                    }
                }
                else
                {
                    let arg = function.remove(i + 1);
                    function[i] = match arg.clone()
                    {
                        Matrix(a) => match s.as_str()
                        {
                            "plane" =>
                            {
                                if a.len() != 3 || a.iter().any(|a| a.len() != 3)
                                {
                                    return Err("dimensions too high");
                                }
                                let x1 = &a[0][0].number;
                                let y1 = &a[0][1].number;
                                let z1 = &a[0][2].number;
                                let x2 = &a[1][0].number;
                                let y2 = &a[1][1].number;
                                let z2 = &a[1][2].number;
                                let x3 = &a[2][0].number;
                                let y3 = &a[2][1].number;
                                let z3 = &a[2][2].number;
                                let t1 = z2 - z3.clone() * y2 / y3;
                                let t2 = x2 - x3.clone() * y2 / y3;
                                let t3 = t1 / t2.clone();
                                let c1 = z1 - x1 * t3.clone() - y1.clone() / y3 * (z3 - x3 * t3);
                                let c2 = 1 - x1 / t2.clone() + x1.clone() * y2 / y3 / t2.clone()
                                    - y1.clone() / y3
                                    + x3.clone() * y1 / y3 / t2.clone()
                                    - x3.clone() * y1 * y2 / y3 / y3 / t2.clone();
                                let c: Complex = c1 / c2;
                                let b = (z3
                                    - c.clone()
                                    - x3 * (z2 - c.clone() - y2.clone() / y3 * (z3 - c.clone()))
                                        / t2)
                                    / y3;
                                let a = (z2 - c.clone() - b.clone() * y2) / x2;
                                if function.len() > i + 2
                                {
                                    let x = function.remove(i + 1).num()?.number;
                                    let y = function.remove(i + 1).num()?.number;
                                    Num(Number::from(a * x + b * y + c, None))
                                }
                                else
                                {
                                    Vector(vec![
                                        Number::from(a, None),
                                        Number::from(b, None),
                                        Number::from(c, None),
                                    ])
                                }
                            }
                            "lobf" | "lineofbestfit" =>
                            {
                                if a.is_empty() || a.iter().any(|a| a.len() != 2)
                                {
                                    return Err("dimensions too high");
                                }
                                let mut xsum = Complex::new(options.prec);
                                let mut ysum = Complex::new(options.prec);
                                let mut xxsum = Complex::new(options.prec);
                                let mut xysum = Complex::new(options.prec);
                                for row in &a
                                {
                                    let x = row[0].number.clone();
                                    let y = row[1].number.clone();
                                    xsum += x.clone();
                                    ysum += y.clone();
                                    xxsum += sqr(x.clone());
                                    xysum += x * y;
                                }
                                let m: Complex = (a.len() * xysum - xsum.clone() * ysum.clone())
                                    / (a.len() * xxsum - sqr(xsum.clone()));
                                let b = (ysum - m.clone() * xsum) / a.len();
                                if function.len() > i + 1
                                {
                                    let x = function.remove(i + 1).num()?.number;
                                    Num(Number::from(m * x + b, a[0][1].units))
                                }
                                else
                                {
                                    Vector(vec![Number::from(m, None), Number::from(b, None)])
                                }
                            }
                            "inter" | "interpolate" =>
                            {
                                if function.len() > i + 1
                                {
                                    if !a.is_empty() && a.iter().all(|a| a.len() == 2)
                                    {
                                        let x = function.remove(i + 1).num()?.number;
                                        let mut sum = Complex::new(options.prec);
                                        for i in 0..a.len()
                                        {
                                            let mut prod = Complex::with_val(options.prec, 1);
                                            for j in 0..a.len()
                                            {
                                                if j != i
                                                {
                                                    prod *= (x.clone() - a[j][0].number.clone())
                                                        / (a[i][0].number.clone()
                                                            - a[j][0].number.clone())
                                                }
                                            }
                                            sum += prod * a[i][1].number.clone()
                                        }
                                        Num(Number::from(sum, a[0][1].units))
                                    }
                                    else
                                    {
                                        return Err("dimensions too high");
                                    }
                                }
                                else
                                {
                                    return Err("no x value given");
                                }
                            }
                            "sort" => Matrix(sort_mat(a, options.prec)),
                            "max" =>
                            {
                                let mut vec = Vec::new();
                                for j in a
                                {
                                    let mut max = j[0].clone();
                                    for i in j
                                    {
                                        if i.number.real() > max.number.real()
                                        {
                                            max = i
                                        }
                                    }
                                    vec.push(max)
                                }
                                Vector(vec)
                            }
                            "min" =>
                            {
                                let mut vec = Vec::new();
                                for j in a
                                {
                                    let mut min = j[0].clone();
                                    for i in j
                                    {
                                        if i.number.real() < min.number.real()
                                        {
                                            min = i
                                        }
                                    }
                                    vec.push(min)
                                }
                                Vector(vec)
                            }
                            "flatten" => Vector(a.into_iter().flatten().collect::<Vec<Number>>()),
                            "cofactor" | "cofactors" | "cof" => Matrix(cofactor(&a)?),
                            "minor" | "minors" => Matrix(minors(&a)?),
                            "adjugate" | "adj" => Matrix(transpose(&cofactor(&a)?)),
                            "inverse" | "inv" => Matrix(inverse(&a)?),
                            "transpose" | "trans" => Matrix(transpose(&a)),
                            "len" =>
                            {
                                Num(Number::from(Complex::with_val(options.prec, a.len()), None))
                            }
                            "wid" | "width" => Num(Number::from(
                                Complex::with_val(options.prec, a[0].len()),
                                None,
                            )),
                            "tr" | "trace" => Num(trace(&a)),
                            "det" | "norm" | "determinant" => Num(determinant(&a)?),
                            "part" =>
                            {
                                if function.len() > i + 2
                                {
                                    match (function.remove(i + 1), function.remove(i + 1))
                                    {
                                        (Num(b), Num(c)) =>
                                        {
                                            let b = b.number;
                                            let c = c.number;
                                            let n1 =
                                                b.clone().real().to_integer().unwrap_or_default();
                                            let getcol = n1 == -1;
                                            let n1 = n1.to_usize().unwrap_or_default();
                                            let n2 = c
                                                .clone()
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            if getcol
                                            {
                                                if a.iter().all(|a| n2 < a.len())
                                                {
                                                    Vector(
                                                        a.iter()
                                                            .map(|a| a[n2].clone())
                                                            .collect::<Vec<Number>>(),
                                                    )
                                                }
                                                else
                                                {
                                                    return Err("out of range");
                                                }
                                            }
                                            else if n1 < a.len() && n2 < a[n1].len()
                                            {
                                                Num(a[n1][n2].clone())
                                            }
                                            else
                                            {
                                                return Err("not in matrix");
                                            }
                                        }
                                        (Num(b), Vector(c)) =>
                                        {
                                            let b = b.number;
                                            let n1 =
                                                b.clone().real().to_integer().unwrap_or_default();
                                            let getcol = n1 == -1;
                                            let n1 = n1.to_usize().unwrap_or_default();
                                            if getcol
                                            {
                                                let mut mat = Vec::new();
                                                for n in c
                                                {
                                                    let n = n
                                                        .number
                                                        .clone()
                                                        .real()
                                                        .to_integer()
                                                        .unwrap_or_default()
                                                        .to_usize()
                                                        .unwrap_or_default();
                                                    if a.iter().all(|a| n < a.len())
                                                    {
                                                        mat.push(
                                                            a.iter()
                                                                .map(|a| a[n].clone())
                                                                .collect::<Vec<Number>>(),
                                                        )
                                                    }
                                                    else
                                                    {
                                                        return Err("out of range");
                                                    }
                                                }
                                                Matrix(transpose(&mat))
                                            }
                                            else
                                            {
                                                let mut vec = Vec::new();
                                                for n in c
                                                {
                                                    let n2 = n
                                                        .number
                                                        .clone()
                                                        .real()
                                                        .to_integer()
                                                        .unwrap_or_default()
                                                        .to_usize()
                                                        .unwrap_or_default();
                                                    if n1 < a.len() && n2 < a[n1].len()
                                                    {
                                                        vec.push(a[n1][n2].clone())
                                                    }
                                                    else
                                                    {
                                                        return Err("not in matrix");
                                                    }
                                                }
                                                Vector(vec)
                                            }
                                        }
                                        (Vector(b), Num(c)) =>
                                        {
                                            let c = c.number;
                                            let n2 = c
                                                .clone()
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            let mut vec = Vec::new();
                                            for n in b
                                            {
                                                let n1 = n
                                                    .number
                                                    .clone()
                                                    .real()
                                                    .to_integer()
                                                    .unwrap_or_default()
                                                    .to_usize()
                                                    .unwrap_or_default();
                                                if n1 < a.len() && n2 < a[n1].len()
                                                {
                                                    vec.push(a[n1][n2].clone())
                                                }
                                                else
                                                {
                                                    return Err("not in matrix");
                                                }
                                            }
                                            Vector(vec)
                                        }
                                        (Vector(b), Vector(c)) =>
                                        {
                                            let mut mat = Vec::new();
                                            for g in b
                                            {
                                                let mut vec = Vec::new();
                                                let n1 = g
                                                    .number
                                                    .clone()
                                                    .real()
                                                    .to_integer()
                                                    .unwrap_or_default()
                                                    .to_usize()
                                                    .unwrap_or_default();
                                                for n in c.clone()
                                                {
                                                    let n2 = n
                                                        .number
                                                        .clone()
                                                        .real()
                                                        .to_integer()
                                                        .unwrap_or_default()
                                                        .to_usize()
                                                        .unwrap_or_default();
                                                    if n1 < a.len() && n2 < a[n1].len()
                                                    {
                                                        vec.push(a[n1][n2].clone())
                                                    }
                                                    else
                                                    {
                                                        return Err("not in matrix");
                                                    }
                                                }
                                                mat.push(vec);
                                            }
                                            Matrix(mat)
                                        }
                                        _ => return Err("wrong part num"),
                                    }
                                }
                                else if function.len() > i + 1
                                {
                                    match function.remove(i + 1)
                                    {
                                        Num(b) =>
                                        {
                                            let b = b.number;
                                            let n = b
                                                .clone()
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            if n < a.len()
                                            {
                                                Vector(a[n].clone())
                                            }
                                            else
                                            {
                                                return Err("out of range");
                                            }
                                        }
                                        Vector(b) =>
                                        {
                                            let mut vec = Vec::new();
                                            for i in b
                                            {
                                                let n = i
                                                    .number
                                                    .clone()
                                                    .real()
                                                    .to_integer()
                                                    .unwrap_or_default()
                                                    .to_usize()
                                                    .unwrap_or_default();
                                                if n < a.len()
                                                {
                                                    vec.push(a[n].clone());
                                                }
                                                else
                                                {
                                                    return Err("out of range");
                                                }
                                            }
                                            Matrix(vec)
                                        }
                                        _ => return Err("non num/vec"),
                                    }
                                }
                                else
                                {
                                    return Err("no arg");
                                }
                            }
                            "weighted_mean" =>
                            {
                                if a.iter().any(|a| a.len() != 2)
                                {
                                    return Err("bad data");
                                }
                                Num(Number::from(
                                    a.iter().fold(Complex::new(options.prec), |sum, val| {
                                        sum + val[0].number.clone() * val[1].number.clone()
                                    }) / a.iter().fold(Complex::new(options.prec), |sum, val| {
                                        sum + val[1].number.clone()
                                    }),
                                    a[0][0].units,
                                ))
                            }
                            "mean" | "μ" => Num(Number::from(
                                a.iter()
                                    .flatten()
                                    .fold(Complex::new(options.prec), |sum, val| {
                                        sum + val.number.clone()
                                    })
                                    / a.iter().fold(0, |sum, a| sum + a.len()),
                                a[0][0].units,
                            )),
                            "mode" =>
                            {
                                let mut most = (Vec::new(), 0);
                                for i in a.iter().flatten()
                                {
                                    let mut count = 0;
                                    for j in a.iter().flatten()
                                    {
                                        if i == j
                                        {
                                            count += 1;
                                        }
                                    }
                                    if count > most.1
                                    {
                                        most = (vec![i.clone()], count);
                                    }
                                    if count == most.1 && !most.0.iter().any(|j| i == j)
                                    {
                                        most.0.push(i.clone())
                                    }
                                }
                                if most.0.len() == 1
                                {
                                    Num(most.0[0].clone())
                                }
                                else
                                {
                                    Vector(most.0)
                                }
                            }
                            "median" =>
                            {
                                let a = sort(a.iter().flatten().cloned().collect::<Vec<Number>>());
                                if a.len() % 2 == 0
                                {
                                    Vector(vec![a[a.len() / 2 - 1].clone(), a[a.len() / 2].clone()])
                                }
                                else
                                {
                                    Num(a[a.len() / 2].clone())
                                }
                            }
                            "all" =>
                            {
                                let mut res = true;
                                for a in a.iter().flatten()
                                {
                                    if !(a.number.imag().is_zero() && a.number.real() == &1)
                                    {
                                        res = false
                                    }
                                }
                                Num(Number::from(
                                    Complex::with_val(options.prec, res as u8),
                                    None,
                                ))
                            }
                            "any" =>
                            {
                                let mut res = false;
                                for a in a.iter().flatten()
                                {
                                    if a.number.imag().is_zero() && a.number.real() == &1
                                    {
                                        res = true
                                    }
                                }
                                Num(Number::from(
                                    Complex::with_val(options.prec, res as u8),
                                    None,
                                ))
                            }
                            "eigenvalues" =>
                            {
                                if function.len() > i + 1 && !matches!(&function[i + 1], Func(_))
                                {
                                    function.remove(i + 1);
                                    eigenvalues(&a, true)?
                                }
                                else
                                {
                                    eigenvalues(&a, false)?
                                }
                            }
                            "eigenvectors" =>
                            {
                                if function.len() > i + 1 && !matches!(&function[i + 1], Func(_))
                                {
                                    function.remove(i + 1);
                                    eigenvectors(&a, true)?
                                }
                                else
                                {
                                    eigenvectors(&a, false)?
                                }
                            }
                            "rref" => Matrix(rref(a)?),
                            "ker" => Matrix(kernel(a)?),
                            "ran" => Matrix(range(a)?),
                            "generalized_eigenvectors" =>
                            {
                                if function.len() > i + 1 && !matches!(&function[i + 1], Func(_))
                                {
                                    function.remove(i + 1);
                                    generalized_eigenvectors(&a, true)?
                                }
                                else
                                {
                                    generalized_eigenvectors(&a, false)?
                                }
                            }
                            "rcf" => rcf(a)?,
                            "jcf" => jcf(a)?,
                            "change_basis" =>
                            {
                                if function.len() > i + 1 && !matches!(&function[i + 1], Func(_))
                                {
                                    let beta = function.remove(i + 1).mat()?;
                                    if function.len() > i + 1
                                        && !matches!(&function[i + 1], Func(_))
                                    {
                                        change_basis(a, &beta, &function.remove(i + 1).mat()?)?
                                    }
                                    else
                                    {
                                        let i = identity(a.len(), a[0][0].number.prec().0);
                                        change_basis(a, &i, &beta)?
                                    }
                                }
                                else
                                {
                                    return Err("missing arg");
                                }
                            }
                            "coordinate" =>
                            {
                                if function.len() > i + 1 && !matches!(&function[i + 1], Func(_))
                                {
                                    let v = function.remove(i + 1).vec()?;
                                    coordinate(v, a)?
                                }
                                else
                                {
                                    return Err("missing arg");
                                }
                            }
                            "rank" => Num(Number::from(
                                Complex::with_val(a[0][0].number.prec(), range(a)?.len()),
                                None,
                            )),
                            "null" => Num(Number::from(
                                Complex::with_val(a[0][0].number.prec(), kernel(a)?.len()),
                                None,
                            )),
                            "to_list" =>
                            {
                                let mut vec = Vec::new();
                                for a in a
                                {
                                    if a.len() != 2
                                    {
                                        return Err("bad list");
                                    }
                                    for _ in 0..a[1]
                                        .number
                                        .real()
                                        .to_integer()
                                        .unwrap_or_default()
                                        .to_usize()
                                        .unwrap_or_default()
                                    {
                                        vec.push(a[0].clone())
                                    }
                                }
                                if vec.is_empty()
                                {
                                    return Err("bad list");
                                }
                                Vector(sort(vec))
                            }
                            "to_freq" =>
                            {
                                if a.is_empty()
                                {
                                    return Err("bad list");
                                }
                                let mut a = sort(a.into_iter().flatten().collect::<Vec<Number>>());
                                let mut last = a[0].clone();
                                let mut count = 1;
                                a.remove(0);
                                let mut mat = Vec::new();
                                for a in a
                                {
                                    if a != last
                                    {
                                        mat.push(vec![
                                            last.clone(),
                                            Number::from(
                                                Complex::with_val(options.prec, count),
                                                None,
                                            ),
                                        ]);
                                        last = a;
                                        count = 0;
                                    }
                                    count += 1;
                                }
                                mat.push(vec![
                                    last.clone(),
                                    Number::from(Complex::with_val(options.prec, count), None),
                                ]);
                                Matrix(mat)
                            }
                            "rand_weighted" =>
                            {
                                if a.iter().any(|a| a.len() != 2)
                                {
                                    return Err("bad data");
                                }
                                let mut sum = Integer::new();
                                for i in &a
                                {
                                    sum += i[1].number.real().to_integer().unwrap_or_default();
                                }
                                let n = sum.to_u128().unwrap_or_default();
                                if n == 0
                                {
                                    return Err("bad data");
                                }
                                let max = u128::MAX - u128::MAX.rem(n);
                                let mut rnd = u128::MAX;
                                while rnd >= max
                                {
                                    rnd = fastrand::u128(..);
                                }
                                rnd = rnd.rem(n) + 1;
                                let mut num =
                                    Number::from(Complex::with_val(options.prec, Nan), None);
                                for i in &a
                                {
                                    rnd = rnd.saturating_sub(
                                        i[1].number
                                            .real()
                                            .to_integer()
                                            .unwrap_or_default()
                                            .to_u128()
                                            .unwrap_or_default(),
                                    );
                                    if rnd == 0
                                    {
                                        num = i[0].clone();
                                        break;
                                    }
                                }
                                Num(num)
                            }
                            "roll" =>
                            {
                                let mut sum: Integer = Integer::new();
                                for i in a
                                {
                                    if i.len() != 2
                                    {
                                        return Err("bad dice data");
                                    }
                                    let a = i[0].number.real().to_integer().unwrap_or_default();
                                    if a > u128::MAX || a == 0
                                    {
                                        return Err("dice too large or bad dice data");
                                    }
                                    let n = a.to_u128().unwrap_or_default();
                                    let max = u128::MAX - u128::MAX.rem(n);
                                    let end = i[1]
                                        .number
                                        .real()
                                        .to_integer()
                                        .unwrap_or_default()
                                        .to_u128()
                                        .unwrap_or_default();
                                    let mut i = 0;
                                    while i < end
                                    {
                                        let rnd = fastrand::u128(..);
                                        if rnd < max
                                        {
                                            sum += rnd.rem(n) + 1;
                                            i += 1;
                                        }
                                    }
                                }
                                Num(Number::from(Complex::with_val(options.prec, sum), None))
                            }
                            "dice" =>
                            {
                                let mut faces = Vec::new();
                                for a in a
                                {
                                    if a.len() != 2
                                    {
                                        return Err("bad list");
                                    }
                                    for _ in 0..a[1]
                                        .number
                                        .real()
                                        .to_integer()
                                        .unwrap_or_default()
                                        .to_usize()
                                        .unwrap_or_default()
                                    {
                                        faces.push(
                                            a[0].number
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default(),
                                        )
                                    }
                                }
                                if faces.iter().any(|c| c == &0)
                                {
                                    return Err("bad face value");
                                }
                                if faces.is_empty()
                                {
                                    Vector(vec![Number::from(
                                        Complex::with_val(options.prec, 1),
                                        None,
                                    )])
                                }
                                else if faces.len() == 1
                                {
                                    Vector(vec![
                                        Number::from(
                                            Complex::with_val(options.prec, 1),
                                            None
                                        );
                                        faces[0]
                                    ])
                                }
                                else
                                {
                                    let mut last = vec![Integer::from(1); faces[0]];
                                    let mut current = last.clone();
                                    for i in 1..faces.len()
                                    {
                                        current = Vec::new();
                                        for p in 0..=faces[0..=i].iter().sum::<usize>() - i - 1
                                        {
                                            let value = last[(p + 1).saturating_sub(faces[i])
                                                ..=p.min(faces[0..i].iter().sum::<usize>() - i)]
                                                .iter()
                                                .sum::<Integer>();
                                            current.push(value)
                                        }
                                        last.clone_from(&current)
                                    }
                                    Vector(
                                        current
                                            .iter()
                                            .map(|a| {
                                                Number::from(
                                                    Complex::with_val(options.prec, a),
                                                    None,
                                                )
                                            })
                                            .collect::<Vec<Number>>(),
                                    )
                                }
                            }
                            "poly" | "polynomial" =>
                            {
                                if i + 1 < function.len()
                                {
                                    if a.is_empty()
                                    {
                                        Num(Number::from(Complex::new(options.prec), None))
                                    }
                                    else
                                    {
                                        let x = function.remove(i + 1).num()?;
                                        let mut sum =
                                            vec![
                                                Number::from(Complex::new(options.prec), None);
                                                a.len()
                                            ];
                                        for (i, v) in transpose(&a).iter().rev().enumerate()
                                        {
                                            for (s, a) in sum.iter_mut().zip(v.iter().map(|a| {
                                                Number::from(
                                                    a.number.clone() * x.number.clone().pow(i),
                                                    mul_units(
                                                        a.units,
                                                        Some(
                                                            x.units
                                                                .unwrap_or_default()
                                                                .pow(i as f64),
                                                        ),
                                                    ),
                                                )
                                            }))
                                            {
                                                *s = add(s, &a)
                                            }
                                        }
                                        Vector(sum)
                                    }
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "norm_combine" =>
                            {
                                if a.is_empty()
                                {
                                    return Err("empty vec");
                                }
                                let mut mu = Complex::new(options.prec);
                                let mut var = Complex::new(options.prec);
                                let mut ws = Complex::new(options.prec);
                                for d in &a
                                {
                                    if d.len() < 2
                                    {
                                        return Err("not enough data");
                                    }
                                    if d.len() == 2
                                    {
                                        mu += d[0].number.clone();
                                        var += d[1].number.clone().pow(2);
                                        ws += 1;
                                    }
                                    else
                                    {
                                        mu += d[0].number.clone() * d[2].number.clone();
                                        var += d[1].number.clone().pow(2) * d[2].number.clone();
                                        ws += d[2].number.clone();
                                    }
                                }
                                mu /= ws.clone();
                                for d in &a
                                {
                                    if d.len() == 2
                                    {
                                        var += (mu.clone() - d[0].number.clone()).pow(2);
                                    }
                                    else
                                    {
                                        var += (mu.clone() - d[0].number.clone()).pow(2)
                                            * d[2].number.clone();
                                    }
                                }
                                var /= ws;
                                Vector(vec![Number::from(mu, None), Number::from(var.sqrt(), None)])
                            }
                            _ => do_functions(arg, options, &mut function, i, &to_deg, s)?,
                        },
                        Vector(a) => match s.as_str()
                        {
                            "transpose" | "trans" => Matrix(transpose(&[a])),
                            "to_list" =>
                            {
                                let mut vec = Vec::new();
                                for (i, a) in a.iter().enumerate()
                                {
                                    let num =
                                        Number::from(Complex::with_val(options.prec, i + 1), None);
                                    for _ in 1..=a
                                        .number
                                        .real()
                                        .to_integer()
                                        .unwrap_or_default()
                                        .to_usize()
                                        .unwrap_or_default()
                                    {
                                        vec.push(num.clone())
                                    }
                                }
                                if vec.is_empty()
                                {
                                    return Err("bad list");
                                }
                                Vector(sort(vec))
                            }
                            "roll" =>
                            {
                                let mut sum: Integer = Integer::new();
                                let mut i = 0;
                                while i < a.len()
                                {
                                    let a = a[i].number.real().to_integer().unwrap_or_default();
                                    if a > u128::MAX as f64 || a == 0
                                    {
                                        return Err("dice too large or bad dice data");
                                    }
                                    let n = a.to_u128().unwrap_or_default();
                                    let max = u128::MAX - u128::MAX.rem(n);
                                    let rnd = fastrand::u128(..);
                                    if rnd < max
                                    {
                                        sum += rnd.rem(n) + 1;
                                        i += 1;
                                    }
                                }
                                Num(Number::from(Complex::with_val(options.prec, sum), None))
                            }
                            "dice" =>
                            {
                                let faces = a
                                    .iter()
                                    .map(|c| {
                                        c.number
                                            .real()
                                            .to_integer()
                                            .unwrap_or_default()
                                            .to_usize()
                                            .unwrap_or_default()
                                    })
                                    .collect::<Vec<usize>>();
                                if faces.iter().any(|c| c == &0)
                                {
                                    return Err("bad face value");
                                }
                                let mut last = vec![Integer::from(1); faces[0]];
                                if a.is_empty()
                                {
                                    Vector(vec![Number::from(
                                        Complex::with_val(options.prec, 1),
                                        None,
                                    )])
                                }
                                else if faces.len() == 1
                                {
                                    Vector(
                                        last.iter()
                                            .map(|a| {
                                                Number::from(
                                                    Complex::with_val(options.prec, a),
                                                    None,
                                                )
                                            })
                                            .collect::<Vec<Number>>(),
                                    )
                                }
                                else
                                {
                                    let mut current = Vec::new();
                                    for i in 1..faces.len()
                                    {
                                        current = Vec::new();
                                        for p in 0..=faces[0..=i].iter().sum::<usize>() - i - 1
                                        {
                                            let value = last[(p + 1).saturating_sub(faces[i])
                                                ..=p.min(faces[0..i].iter().sum::<usize>() - i)]
                                                .iter()
                                                .sum::<Integer>();
                                            current.push(value)
                                        }
                                        last.clone_from(&current);
                                    }
                                    Vector(
                                        current
                                            .iter()
                                            .map(|a| {
                                                Number::from(
                                                    Complex::with_val(options.prec, a),
                                                    None,
                                                )
                                            })
                                            .collect::<Vec<Number>>(),
                                    )
                                }
                            }
                            "quartiles" =>
                            {
                                if a.len() < 2
                                {
                                    return Err("not enough data");
                                }
                                let units = a[0].units;
                                let a = sort(a);
                                let half1 = &a[0..a.len() / 2];
                                let half2 = if a.len() % 2 == 0
                                {
                                    &a[a.len() / 2..a.len()]
                                }
                                else
                                {
                                    &a[a.len() / 2 + 1..a.len()]
                                };
                                if half1.len() % 2 == 0
                                {
                                    Vector(vec![
                                        Number::from(
                                            (half1[half1.len() / 2 - 1].number.clone()
                                                + half1[half1.len() / 2].number.clone())
                                                / 2,
                                            units,
                                        ),
                                        Number::from(
                                            if a.len() % 2 == 0
                                            {
                                                (half1[half1.len().saturating_sub(1)]
                                                    .number
                                                    .clone()
                                                    + half2[0].number.clone())
                                                    / 2
                                            }
                                            else
                                            {
                                                a[a.len() / 2].number.clone()
                                            },
                                            units,
                                        ),
                                        Number::from(
                                            (half2[half2.len() / 2 - 1].number.clone()
                                                + half2[half2.len() / 2].number.clone())
                                                / 2,
                                            units,
                                        ),
                                    ])
                                }
                                else
                                {
                                    Vector(vec![
                                        Number::from(half1[half1.len() / 2].number.clone(), units),
                                        Number::from(
                                            if a.len() % 2 == 0
                                            {
                                                (half1[half1.len().saturating_sub(1)]
                                                    .number
                                                    .clone()
                                                    + half2[0].number.clone())
                                                    / 2
                                            }
                                            else
                                            {
                                                a[a.len() / 2].number.clone()
                                            },
                                            units,
                                        ),
                                        Number::from(half2[half2.len() / 2].number.clone(), units),
                                    ])
                                }
                            }
                            "percentile" =>
                            {
                                if function.len() <= i + 1
                                {
                                    return Err("not enough input");
                                }
                                let b = function.remove(i + 1).num()?.number;
                                let r: Float = (b.real().clone() / 100) * a.len();
                                let r = r
                                    .ceil()
                                    .to_integer()
                                    .unwrap_or_default()
                                    .to_usize()
                                    .unwrap_or_default();
                                if r > a.len()
                                {
                                    return Err("bad input");
                                }
                                Num(sort(a)[r.saturating_sub(1)].clone())
                            }
                            "percentilerank" =>
                            {
                                if function.len() <= i + 1
                                {
                                    return Err("not enough input");
                                }
                                let mut cf = 0;
                                let mut f = 0;
                                let b = function.remove(i + 1).num()?.number;
                                for a in sort(a.clone())
                                {
                                    if a.number.real() < b.real()
                                    {
                                        cf += 1;
                                    }
                                    else if a.number == b
                                    {
                                        f += 1;
                                    }
                                    else
                                    {
                                        break;
                                    }
                                }
                                Num(Number::from(
                                    100 * (cf + Complex::with_val(options.prec, f) / 2) / a.len(),
                                    None,
                                ))
                            }
                            "to_freq" =>
                            {
                                if a.is_empty()
                                {
                                    return Err("bad list");
                                }
                                let mut a = sort(a);
                                let mut last = a[0].clone();
                                let mut count = 1;
                                a.remove(0);
                                let mut mat = Vec::new();
                                for a in a
                                {
                                    if a != last
                                    {
                                        mat.push(vec![
                                            last.clone(),
                                            Number::from(
                                                Complex::with_val(options.prec, count),
                                                None,
                                            ),
                                        ]);
                                        last = a;
                                        count = 0;
                                    }
                                    count += 1;
                                }
                                mat.push(vec![
                                    last.clone(),
                                    Number::from(Complex::with_val(options.prec, count), None),
                                ]);
                                Matrix(mat)
                            }
                            "kurtosis" =>
                            {
                                fn quar(z: Complex) -> Complex
                                {
                                    if z.imag().is_zero()
                                    {
                                        z.pow(4)
                                    }
                                    else
                                    {
                                        z.clone() * &z * &z * z
                                    }
                                }
                                let mean = a.iter().fold(Complex::new(options.prec), |sum, val| {
                                    sum + val.number.clone()
                                }) / a.len();
                                Num(Number::from(
                                    a.iter().fold(Complex::new(options.prec), |sum, val| {
                                        sum + quar(val.number.clone() - mean.clone())
                                    }) / a.len()
                                        / sqr(variance(&a, Some(mean.clone()), options.prec).number)
                                        - 3,
                                    None,
                                ))
                            }
                            "skew" | "skewness" =>
                            {
                                let mean = a.iter().fold(Complex::new(options.prec), |sum, val| {
                                    sum + val.number.clone()
                                }) / a.len();
                                Num(Number::from(
                                    a.iter().fold(Complex::new(options.prec), |sum, val| {
                                        sum + cube(val.number.clone() - mean.clone())
                                    }) / a.len()
                                        / cube(
                                            variance(&a, Some(mean.clone()), options.prec)
                                                .number
                                                .sqrt(),
                                        ),
                                    None,
                                ))
                            }
                            "sd" | "standarddeviation" | "σ" => Num(Number::from(
                                variance(&a, None, options.prec).number.sqrt(),
                                a[0].units,
                            )),
                            "variance" | "var" => Num(variance(&a, None, options.prec)),
                            "covariance" | "cov" =>
                            {
                                let mut sum = Complex::new(options.prec);
                                if function.len() <= i + 1
                                {
                                    return Err("not enough input");
                                }
                                let b = function.remove(i + 1).vec()?;
                                if a.len() != b.len()
                                {
                                    return Err("different sized data sets");
                                }
                                let ma = a.iter().fold(Complex::new(options.prec), |sum, val| {
                                    sum + val.number.clone()
                                }) / a.len();
                                let mb = b.iter().fold(Complex::new(options.prec), |sum, val| {
                                    sum + val.number.clone()
                                }) / b.len();
                                for (a, b) in a.iter().zip(b.iter())
                                {
                                    sum += (a.number.clone() - ma.clone())
                                        * (b.number.clone() - mb.clone());
                                }
                                Num(Number::from(
                                    sum / (a.len() - 1),
                                    mul_units(a[0].units, b[0].units),
                                ))
                            }
                            "all" =>
                            {
                                let mut res = true;
                                for a in a
                                {
                                    if !(a.number.imag().is_zero() && a.number.real() == &1)
                                    {
                                        res = false
                                    }
                                }
                                Num(Number::from(
                                    Complex::with_val(options.prec, res as u8),
                                    None,
                                ))
                            }
                            "any" =>
                            {
                                let mut res = false;
                                for a in a
                                {
                                    if a.number.imag().is_zero() && a.number.real() == &1
                                    {
                                        res = true
                                    }
                                }
                                Num(Number::from(
                                    Complex::with_val(options.prec, res as u8),
                                    None,
                                ))
                            }
                            "sort" => Vector(sort(a)),
                            "hsv_to_rgb" =>
                            {
                                if a.len() == 3
                                {
                                    Vector(hsv2rgb(
                                        a[0].number.real().clone(),
                                        a[1].number.real().clone(),
                                        a[2].number.real().clone(),
                                    ))
                                }
                                else
                                {
                                    return Err("not 3 length");
                                }
                            }
                            "mean" | "μ" => Num(Number::from(
                                a.iter().fold(Complex::new(options.prec), |sum, val| {
                                    sum + val.number.clone()
                                }) / a.len(),
                                a[0].units,
                            )),
                            "geo_mean" => Num(Number::from(
                                pow_nth(
                                    a.iter()
                                        .fold(Complex::with_val(options.prec, 1), |sum, val| {
                                            sum * val.number.clone()
                                        }),
                                    Complex::with_val(options.prec, 1) / a.len(),
                                ),
                                a[0].units,
                            )),
                            "median" =>
                            {
                                let a = sort(a);
                                if a.len() % 2 == 0
                                {
                                    Num(Number::from(
                                        (a[a.len() / 2 - 1].number.clone()
                                            + a[a.len() / 2].number.clone())
                                            / 2,
                                        a[0].units,
                                    ))
                                }
                                else
                                {
                                    Num(Number::from(a[a.len() / 2].number.clone(), a[0].units))
                                }
                            }
                            "mode" =>
                            {
                                let mut most = (Vec::new(), 0);
                                for i in &a
                                {
                                    let mut count = 0;
                                    for j in &a
                                    {
                                        if i == j
                                        {
                                            count += 1;
                                        }
                                    }
                                    if count > most.1
                                    {
                                        most = (vec![i.clone()], count);
                                    }
                                    if count == most.1 && !most.0.iter().any(|j| i == j)
                                    {
                                        most.0.push(i.clone())
                                    }
                                }
                                if most.0.len() == 1
                                {
                                    Num(most.0[0].clone())
                                }
                                else
                                {
                                    Vector(most.0)
                                }
                            }
                            "max" =>
                            {
                                let mut max = a[0].clone();
                                for i in a
                                {
                                    if i.number.real() > max.number.real()
                                    {
                                        max = i
                                    }
                                }
                                Num(max)
                            }
                            "min" =>
                            {
                                let mut min = a[0].clone();
                                for i in a
                                {
                                    if i.number.real() < min.number.real()
                                    {
                                        min = i
                                    }
                                }
                                Num(min)
                            }
                            "reverse" => Vector(a.iter().rev().cloned().collect()),
                            "len" =>
                            {
                                Num(Number::from(Complex::with_val(options.prec, a.len()), None))
                            }
                            "norm" =>
                            {
                                let units = a[0].units;
                                let mut n = Complex::new(options.prec);
                                for i in a
                                {
                                    n += sqr(i.number.abs());
                                }
                                Num(Number::from(n.sqrt(), units))
                            }
                            "normalize" =>
                            {
                                let mut n = Complex::new(options.prec);
                                for i in a.clone()
                                {
                                    n += sqr(i.number);
                                }
                                Vector(
                                    a.iter()
                                        .map(|x| {
                                            Number::from(x.number.clone() / n.clone().sqrt(), None)
                                        })
                                        .collect(),
                                )
                            }
                            "car" | "cartesian" =>
                            {
                                if a.len() == 2
                                {
                                    let t = a[1].number.clone() / to_deg.clone();
                                    Vector(vec![
                                        Number::from(
                                            a[0].number.clone() * t.clone().cos(),
                                            a[0].units,
                                        ),
                                        Number::from(
                                            a[0].number.clone() * t.clone().sin(),
                                            a[0].units,
                                        ),
                                    ])
                                }
                                else if a.len() == 3
                                {
                                    let t1 = a[1].number.clone() / to_deg.clone();
                                    let t2 = a[2].number.clone() / to_deg.clone();
                                    Vector(vec![
                                        Number::from(
                                            a[0].number.clone()
                                                * t1.clone().sin()
                                                * t2.clone().cos(),
                                            a[0].units,
                                        ),
                                        Number::from(
                                            a[0].number.clone()
                                                * t1.clone().sin()
                                                * t2.clone().sin(),
                                            a[0].units,
                                        ),
                                        Number::from(
                                            a[0].number.clone() * t1.clone().cos(),
                                            a[0].units,
                                        ),
                                    ])
                                }
                                else
                                {
                                    return Err("incorrect polar form");
                                }
                            }
                            "polar" | "pol" => Vector(to_polar(a.clone(), to_deg.clone())),
                            "cyl" | "cylinder" => Vector(to_cyl(a.clone(), to_deg.clone())),
                            "angle" =>
                            {
                                if function.len() > i + 1
                                {
                                    let b = function.remove(i + 1).vec()?;
                                    if a.len() == 3 && b.len() == 3
                                    {
                                        let c: Complex = sqr(a[0].number.clone())
                                            + sqr(a[1].number.clone())
                                            + sqr(a[2].number.clone());
                                        let d: Complex = sqr(b[0].number.clone())
                                            + sqr(b[1].number.clone())
                                            + sqr(b[2].number.clone());
                                        Num(Number::from(
                                            ((a[0].number.clone() * b[0].number.clone()
                                                + a[1].number.clone() * b[1].number.clone()
                                                + a[2].number.clone() * b[2].number.clone())
                                                / (c.sqrt() * d.sqrt()))
                                            .acos()
                                                * to_deg.clone(),
                                            Some(Units {
                                                angle: 1.0,
                                                ..Units::default()
                                            }),
                                        ))
                                    }
                                    else if a.len() == 2 && b.len() == 2
                                    {
                                        let c: Complex =
                                            sqr(a[0].number.clone()) + sqr(a[1].number.clone());
                                        let d: Complex =
                                            sqr(b[0].number.clone()) + sqr(b[1].number.clone());
                                        Num(Number::from(
                                            ((a[0].number.clone() * b[0].number.clone()
                                                + a[1].number.clone() * b[1].number.clone())
                                                / (c.sqrt() * d.sqrt()))
                                            .acos()
                                                * to_deg.clone(),
                                            Some(Units {
                                                angle: 1.0,
                                                ..Units::default()
                                            }),
                                        ))
                                    }
                                    else
                                    {
                                        return Err("cant decern angles");
                                    }
                                }
                                else
                                {
                                    let vec = to_polar(a, to_deg.clone());
                                    if vec.len() == 3
                                    {
                                        Vector(vec[1..=2].to_vec())
                                    }
                                    else if vec.len() == 2
                                    {
                                        Num(vec[1].clone())
                                    }
                                    else
                                    {
                                        return Err("cant decern angles");
                                    }
                                }
                            }
                            "cross" =>
                            {
                                if function.len() > i + 1
                                {
                                    let b = function.remove(i + 1).vec()?;
                                    let units = mul_units(a[0].units, b[0].units);
                                    if a.len() == 3 && b.len() == 3
                                    {
                                        Vector(vec![
                                            Number::from(
                                                a[1].number.clone() * &b[2].number
                                                    - a[2].number.clone() * &b[1].number,
                                                units,
                                            ),
                                            Number::from(
                                                a[2].number.clone() * &b[0].number
                                                    - a[0].number.clone() * &b[2].number,
                                                units,
                                            ),
                                            Number::from(
                                                a[0].number.clone() * &b[1].number
                                                    - a[1].number.clone() * &b[0].number,
                                                units,
                                            ),
                                        ])
                                    }
                                    else if a.len() == 2 && b.len() == 2
                                    {
                                        Num(Number::from(
                                            a[0].number.clone() * &b[1].number
                                                - a[1].number.clone() * &b[0].number,
                                            units,
                                        ))
                                    }
                                    else
                                    {
                                        return Err("cant cross");
                                    }
                                }
                                else
                                {
                                    return Err("no args");
                                }
                            }
                            "project" | "proj" =>
                            {
                                if function.len() > i + 1
                                {
                                    let b = function.remove(i + 1).clone().vec()?;
                                    if b.len() == a.len()
                                    {
                                        let mut dot = Complex::new(options.prec);
                                        for i in a
                                            .iter()
                                            .zip(b.iter())
                                            .map(|(a, b)| a.number.clone() * b.number.clone())
                                        {
                                            dot += i;
                                        }
                                        let mut norm = Complex::new(options.prec);
                                        for i in &b
                                        {
                                            norm += sqr(i.number.clone().abs());
                                        }
                                        Vector(
                                            b.iter()
                                                .map(|n| {
                                                    Number::from(
                                                        dot.clone() / norm.clone()
                                                            * n.number.clone(),
                                                        n.units,
                                                    )
                                                })
                                                .collect::<Vec<Number>>(),
                                        )
                                    }
                                    else
                                    {
                                        return Err("cant project");
                                    }
                                }
                                else
                                {
                                    return Err("no args");
                                }
                            }
                            "oproject" | "oproj" =>
                            {
                                if function.len() > i + 1
                                {
                                    let b = function.remove(i + 1).clone().vec()?;
                                    if b.len() == a.len()
                                    {
                                        let mut dot = Complex::new(options.prec);
                                        for i in a
                                            .iter()
                                            .zip(b.iter())
                                            .map(|(a, b)| a.number.clone() * b.number.clone())
                                        {
                                            dot += i;
                                        }
                                        let mut norm = Complex::new(options.prec);
                                        for i in &b
                                        {
                                            norm += sqr(i.number.clone().abs());
                                        }
                                        Vector(
                                            b.iter()
                                                .zip(a.iter())
                                                .map(|(n, a)| {
                                                    Number::from(
                                                        a.number.clone()
                                                            - dot.clone() / norm.clone()
                                                                * n.number.clone(),
                                                        a.units,
                                                    )
                                                })
                                                .collect::<Vec<Number>>(),
                                        )
                                    }
                                    else
                                    {
                                        return Err("cant project");
                                    }
                                }
                                else
                                {
                                    return Err("no args");
                                }
                            }
                            "dot" =>
                            {
                                if function.len() > i + 1
                                {
                                    let mut n = Complex::new(options.prec);
                                    let b = function.remove(i + 1).vec()?;
                                    for i in a
                                        .iter()
                                        .zip(b.iter())
                                        .map(|(a, b)| a.number.clone() * b.number.clone())
                                    {
                                        n += i;
                                    }
                                    Num(Number::from(n, mul_units(a[0].units, b[0].units)))
                                }
                                else
                                {
                                    return Err("no args");
                                }
                            }
                            "part" =>
                            {
                                if function.len() > i + 1
                                {
                                    match function.remove(i + 1)
                                    {
                                        Num(b) =>
                                        {
                                            let b = b.number;
                                            let n = b
                                                .clone()
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            if n < a.len()
                                            {
                                                Num(a[n].clone())
                                            }
                                            else
                                            {
                                                return Err("out of range");
                                            }
                                        }
                                        Vector(b) =>
                                        {
                                            let mut vec = Vec::new();
                                            for i in b
                                            {
                                                let n = i
                                                    .number
                                                    .clone()
                                                    .real()
                                                    .to_integer()
                                                    .unwrap_or_default()
                                                    .to_usize()
                                                    .unwrap_or_default();
                                                if n < a.len()
                                                {
                                                    vec.push(a[n].clone());
                                                }
                                                else
                                                {
                                                    return Err("out of range");
                                                }
                                            }
                                            Vector(vec)
                                        }
                                        _ => return Err("non num/vec"),
                                    }
                                }
                                else
                                {
                                    return Err("no args");
                                }
                            }
                            "split" => Matrix(
                                a.iter()
                                    .map(|a| {
                                        vec![
                                            Number::from(a.number.real().clone().into(), None),
                                            Number::from(a.number.imag().clone().into(), None),
                                        ]
                                    })
                                    .collect::<Vec<Vec<Number>>>(),
                            ),
                            "uniq" =>
                            {
                                let mut a = a;
                                a.dedup();
                                Vector(a)
                            }
                            "factors" | "factor" =>
                            {
                                let mut fail = false;
                                let mut mat = Vec::new();
                                for num in a
                                {
                                    let num = num.number;
                                    if num.imag().clone().is_zero()
                                    {
                                        if num.real().clone().fract().is_zero()
                                        {
                                            let mut vec = Vec::new();
                                            let n = num
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            for i in 1..=n
                                            {
                                                if n % i == 0
                                                {
                                                    vec.push(Number::from(
                                                        Complex::with_val(options.prec, i),
                                                        None,
                                                    ));
                                                }
                                            }
                                            mat.push(vec);
                                        }
                                        else
                                        {
                                            fail = true;
                                            break;
                                        }
                                    }
                                    else
                                    {
                                        fail = true;
                                        break;
                                    }
                                }
                                if fail
                                {
                                    Num(Number::from(Complex::with_val(options.prec, Nan), None))
                                }
                                else
                                {
                                    Matrix(mat)
                                }
                            }
                            "union" =>
                            {
                                if function.len() > i + 1
                                {
                                    if let Vector(b) = function.remove(i + 1)
                                    {
                                        let mut a = a;
                                        a.extend(b);
                                        a = sort(a);
                                        a.dedup();
                                        Vector(a)
                                    }
                                    else
                                    {
                                        return Err("arg not vector");
                                    }
                                }
                                else
                                {
                                    return Err("arg not vector");
                                }
                            }
                            "intersection" =>
                            {
                                if function.len() > i + 1
                                {
                                    if let Vector(b) = function.remove(i + 1)
                                    {
                                        let mut v = Vec::new();
                                        'main: for n1 in a
                                        {
                                            for n2 in &b
                                            {
                                                if &n1 == n2
                                                {
                                                    v.push(n2.clone());
                                                    continue 'main;
                                                }
                                            }
                                        }
                                        Vector(v)
                                    }
                                    else
                                    {
                                        return Err("arg not vector");
                                    }
                                }
                                else
                                {
                                    return Err("arg not vector");
                                }
                            }
                            "set_difference" =>
                            {
                                if function.len() > i + 1
                                {
                                    if let Vector(b) = function.remove(i + 1)
                                    {
                                        let mut v = Vec::new();
                                        'main: for n1 in a
                                        {
                                            for n2 in &b
                                            {
                                                if &n1 == n2
                                                {
                                                    continue 'main;
                                                }
                                            }
                                            v.push(n1);
                                        }
                                        Vector(v)
                                    }
                                    else
                                    {
                                        return Err("arg not vector");
                                    }
                                }
                                else
                                {
                                    return Err("arg not vector");
                                }
                            }
                            "symmetric_difference" =>
                            {
                                if function.len() > i + 1
                                {
                                    if let Vector(b) = function.remove(i + 1)
                                    {
                                        let mut a1 = Vec::new();
                                        'main: for n1 in a.clone()
                                        {
                                            for n2 in &b
                                            {
                                                if &n1 == n2
                                                {
                                                    continue 'main;
                                                }
                                            }
                                            a1.push(n1);
                                        }
                                        let mut a2 = Vec::new();
                                        'main: for n1 in b.clone()
                                        {
                                            for n2 in &a
                                            {
                                                if &n1 == n2
                                                {
                                                    continue 'main;
                                                }
                                            }
                                            a2.push(n1);
                                        }
                                        a2.extend(a1);
                                        let mut u = sort(a2);
                                        u.dedup();
                                        Vector(u)
                                    }
                                    else
                                    {
                                        return Err("arg not vector");
                                    }
                                }
                                else
                                {
                                    return Err("arg not vector");
                                }
                            }
                            "cartesian_product" =>
                            {
                                if function.len() > i + 1
                                {
                                    if let Vector(b) = function.remove(i + 1)
                                    {
                                        let mut m = Vec::new();
                                        for n1 in a
                                        {
                                            for n2 in b.clone()
                                            {
                                                m.push(vec![n1.clone(), n2])
                                            }
                                        }
                                        Matrix(m)
                                    }
                                    else
                                    {
                                        return Err("arg not vector");
                                    }
                                }
                                else
                                {
                                    return Err("arg not vector");
                                }
                            }
                            "power_set" =>
                            {
                                let mut m = Vec::new();
                                for i in 0..1 << a.len()
                                {
                                    let mut v = Vec::new();
                                    for (j, n) in a.iter().enumerate()
                                    {
                                        if (i >> j) & 1 == 1
                                        {
                                            v.push(n.clone())
                                        }
                                    }
                                    m.push(v);
                                }
                                Matrix(m)
                            }
                            "set_fix" =>
                            {
                                let mut a = sort(a);
                                a.dedup();
                                Vector(a)
                            }
                            "subset" =>
                            {
                                if function.len() > i + 1
                                {
                                    if let Vector(b) = function.remove(i + 1)
                                    {
                                        let mut ainb = true;
                                        'main: for n1 in b
                                        {
                                            for n2 in &a
                                            {
                                                if &n1 == n2
                                                {
                                                    continue 'main;
                                                }
                                            }
                                            ainb = false;
                                        }
                                        Num(Number::from(
                                            Complex::with_val(options.prec, ainb as u8),
                                            None,
                                        ))
                                    }
                                    else
                                    {
                                        return Err("arg not vector");
                                    }
                                }
                                else
                                {
                                    return Err("arg not vector");
                                }
                            }
                            "element" =>
                            {
                                if function.len() > i + 1
                                {
                                    if let Num(b) = function.remove(i + 1)
                                    {
                                        let mut ainb = false;
                                        for n2 in &a
                                        {
                                            if &b == n2
                                            {
                                                ainb = true;
                                            }
                                        }
                                        Num(Number::from(
                                            Complex::with_val(options.prec, ainb as u8),
                                            None,
                                        ))
                                    }
                                    else
                                    {
                                        return Err("arg not vector");
                                    }
                                }
                                else
                                {
                                    return Err("arg not vector");
                                }
                            }
                            "extend" | "link" =>
                            {
                                if function.len() > i + 1
                                {
                                    match function.remove(i + 1)
                                    {
                                        Num(n) =>
                                        {
                                            let mut a = a;
                                            a.push(n);
                                            Vector(a)
                                        }
                                        Vector(v) =>
                                        {
                                            let mut a = a;
                                            a.extend(v);
                                            Vector(a)
                                        }
                                        _ => return Err("bad arg"),
                                    }
                                }
                                else
                                {
                                    return Err("bad arg");
                                }
                            }
                            "remove" =>
                            {
                                if function.len() > i + 1
                                {
                                    match function.remove(i + 1)
                                    {
                                        Num(n) =>
                                        {
                                            let mut a = a;
                                            let n = n
                                                .number
                                                .real()
                                                .to_integer()
                                                .unwrap_or_default()
                                                .to_usize()
                                                .unwrap_or_default();
                                            if n >= a.len()
                                            {
                                                return Err("bad range");
                                            }
                                            a.remove(n);
                                            Vector(a)
                                        }
                                        Vector(v) =>
                                        {
                                            let mut a = a;
                                            for n in v
                                            {
                                                let n = n
                                                    .number
                                                    .real()
                                                    .to_integer()
                                                    .unwrap_or_default()
                                                    .to_usize()
                                                    .unwrap_or_default();
                                                if n >= a.len()
                                                {
                                                    return Err("bad range");
                                                }
                                                a.remove(n);
                                            }
                                            Vector(a)
                                        }
                                        _ => return Err("bad range"),
                                    }
                                }
                                else
                                {
                                    return Err("bad range");
                                }
                            }
                            "rationalize" => Matrix(
                                a.iter()
                                    .map(|c| c_to_rational(c.number.clone(), options))
                                    .collect::<Vec<Vec<Number>>>(),
                            ),
                            "prime_factors" =>
                            {
                                if a.len() != 2
                                {
                                    return Err("expected vector length 2");
                                }
                                let mut p1 = prime_factors(
                                    a[0].number.real().to_integer().unwrap_or_default(),
                                );
                                let p2 = prime_factors(
                                    a[1].number.real().to_integer().unwrap_or_default(),
                                );
                                for p in p1.iter_mut()
                                {
                                    for m in &p2
                                    {
                                        if p.0 == m.0
                                        {
                                            p.1 -= m.1
                                        }
                                    }
                                }
                                for m in p2.iter()
                                {
                                    if !p1.iter().any(|p| p.0 == m.0)
                                    {
                                        p1.push((m.0.clone(), -m.1));
                                    }
                                }
                                Matrix(
                                    p1.iter()
                                        .map(|p| {
                                            vec![
                                                Number::from(
                                                    Complex::with_val(options.prec, p.0.clone()),
                                                    None,
                                                ),
                                                Number::from(
                                                    Complex::with_val(options.prec, p.1),
                                                    None,
                                                ),
                                            ]
                                        })
                                        .collect::<Vec<Vec<Number>>>(),
                                )
                            }
                            "poly" | "polynomial" =>
                            {
                                if i + 1 < function.len()
                                {
                                    let x = function.remove(i + 1).num()?;
                                    let mut sum = Number::from(Complex::new(options.prec), None);
                                    for (i, a) in a.iter().rev().enumerate()
                                    {
                                        let n = Number::from(
                                            a.number.clone() * x.number.clone().pow(i),
                                            mul_units(
                                                a.units,
                                                Some(x.units.unwrap_or_default().pow(i as f64)),
                                            ),
                                        );
                                        if i == 0 { sum = n } else { sum = add(&sum, &n) }
                                    }
                                    Num(sum)
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            _ => do_functions(arg, options, &mut function, i, &to_deg, s)?,
                        },
                        _ => match s.as_str()
                        {
                            "rationalize" => Vector(c_to_rational(arg.num()?.number, options)),
                            "domain_coloring_rgb" =>
                            {
                                let pi = Float::with_val(options.prec, Pi);
                                let num = arg.num()?.number;
                                let hue: Float = 1 + (-num.clone()).arg().real().clone() / &pi;
                                let sat: Float = (1 + num.clone().abs().real().clone().fract()) / 2;
                                let val: Float = {
                                    let (r, i) = (num * &pi).into_real_imag();
                                    let t1: Float = r.sin();
                                    let t2: Float = i.sin();
                                    (t1 * t2).abs().pow(0.125)
                                };
                                Vector(hsv2rgb(3 * hue, sat, val))
                            }
                            "multinomial" =>
                            {
                                let mut numerator: Complex = arg.num()?.number + 1;
                                let mut divisor = gamma(numerator.clone());
                                while i + 1 < function.len() && !matches!(&function[i + 1], Func(_))
                                {
                                    let temp = function.remove(i + 1).num()?.number;
                                    numerator += temp.clone();
                                    let temp = temp.clone() + 1;
                                    divisor *= gamma(temp);
                                }
                                Num(Number::from(gamma(numerator) / divisor, None))
                            }
                            "Β" | "B" | "beta" =>
                            {
                                if i + 1 < function.len()
                                {
                                    let a = arg.num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    if i + 1 < function.len()
                                    {
                                        let x = function.remove(i + 1).num()?.number;
                                        Num(Number::from(incomplete_beta(a, b, x), None))
                                    }
                                    else if a.imag().is_zero() && b.imag().is_zero()
                                    {
                                        Num(Number::from(
                                            gamma(a.clone()) * gamma(b.clone())
                                                / gamma(a + b.clone()),
                                            None,
                                        ))
                                    }
                                    else
                                    {
                                        Num(Number::from(
                                            incomplete_beta(
                                                Complex::with_val(options.prec, 1),
                                                a,
                                                b,
                                            ),
                                            None,
                                        ))
                                    }
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "gamma_pdf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let x = arg.num()?.number;
                                    let a = function.remove(i + 1).num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    Num(Number::from(
                                        pow_nth(b.clone(), -a.clone()) / gamma(a.clone())
                                            * pow_nth(x.clone(), a - 1)
                                            * (-x / b).exp(),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "gamma_cdf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let x = arg.num()?.number;
                                    let a = function.remove(i + 1).num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    Num(Number::from(
                                        1 - incomplete_gamma(a.clone(), x / b) / gamma(a),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "I" | "betaC" | "beta_cdf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let x = arg.num()?.number;
                                    let a = function.remove(i + 1).num()?.number;
                                    let b = function.remove(i + 1).num()?.number;
                                    if i + 1 < function.len()
                                    {
                                        let z = function.remove(i + 1).num()?.number;
                                        Num(Number::from(
                                            regularized_incomplete_beta(a, b.clone(), z.clone())
                                                - regularized_incomplete_beta(x, b, z),
                                            None,
                                        ))
                                    }
                                    else
                                    {
                                        Num(Number::from(
                                            regularized_incomplete_beta(x, a, b),
                                            None,
                                        ))
                                    }
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "betaP" | "beta_pdf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let x = arg.num()?.number;
                                    let alpha = function.remove(i + 1).num()?.number;
                                    let beta = function.remove(i + 1).num()?.number;
                                    let c: Complex = 1 - x.clone();
                                    Num(Number::from(
                                        gamma(alpha.clone() + beta.clone())
                                            * pow_nth(x, alpha.clone() - 1)
                                            * pow_nth(c, beta.clone() - 1)
                                            / (gamma(alpha) * gamma(beta)),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "normP" | "norm_pdf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let x = arg.num()?.number;
                                    let mu = function.remove(i + 1).num()?.number;
                                    let sigma = function.remove(i + 1).num()?.number;
                                    let n: Complex = sqr(x - mu);
                                    let n: Complex = -n / (2 * sqr(sigma.clone()));
                                    let tau: Complex = 2 * Complex::with_val(options.prec, Pi);
                                    Num(Number::from(n.exp() / (sigma * tau.sqrt()), None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "normD" | "norm_cdf" =>
                            {
                                let mut a = arg.num()?.number;
                                if i + 2 < function.len()
                                {
                                    a -= function.remove(i + 1).num()?.number;
                                    a /= function.remove(i + 1).num()?.number;
                                }
                                if a.imag().is_zero()
                                {
                                    let two = Float::with_val(options.prec, 2);
                                    Num(Number::from(
                                        ((-a / two.clone().sqrt()).real().clone().erfc() / two)
                                            .into(),
                                        None,
                                    ))
                                }
                                else
                                {
                                    let two = Float::with_val(options.prec, 2);
                                    Num(Number::from(erf(-a / two.clone().sqrt()) / two, None))
                                }
                            }
                            "lognorm_cdf" =>
                            {
                                let mut a = arg.num()?.number.ln();
                                if i + 2 < function.len()
                                {
                                    a -= function.remove(i + 1).num()?.number;
                                    a /= function.remove(i + 1).num()?.number;
                                }
                                if a.imag().is_zero()
                                {
                                    let two = Float::with_val(options.prec, 2);
                                    Num(Number::from(
                                        ((-a / two.clone().sqrt()).real().clone().erfc() / two)
                                            .into(),
                                        None,
                                    ))
                                }
                                else
                                {
                                    let two = Float::with_val(options.prec, 2);
                                    Num(Number::from(erf(-a / two.clone().sqrt()) / two, None))
                                }
                            }
                            "lognorm_pdf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let x = arg.num()?.number;
                                    let mu = function.remove(i + 1).num()?.number;
                                    let sigma = function.remove(i + 1).num()?.number;
                                    let n: Complex = sqr(x.clone().ln() - mu);
                                    let n: Complex = -n / (2 * sqr(sigma.clone()));
                                    let tau: Complex = 2 * Complex::with_val(options.prec, Pi);
                                    Num(Number::from(n.exp() / (sigma * tau.sqrt() * x), None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "poisson_pmf" =>
                            {
                                if i + 1 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let l = function.remove(i + 1).num()?.number;
                                    Num(Number::from(
                                        pow_nth(l.clone(), k.clone()) * (-l).exp() / gamma(k + 1),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "poisson_cdf" =>
                            {
                                if i + 1 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let l = function.remove(i + 1).num()?.number;
                                    Num(Number::from(
                                        incomplete_gamma(k.clone() + 1, l) / gamma(k + 1),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "binomial_cdf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let n = function.remove(i + 1).num()?.number;
                                    let p = function.remove(i + 1).num()?.number;
                                    let q: Complex = 1 - p.clone();
                                    Num(Number::from(
                                        regularized_incomplete_beta(q, n - k.clone(), 1 + k),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "binomial_pmf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let n = function.remove(i + 1).num()?.number;
                                    let p = function.remove(i + 1).num()?.number;
                                    let q: Complex = 1 - p.clone();
                                    Num(Number::from(
                                        binomial(n.clone(), k.clone())
                                            * pow_nth(p, k.clone())
                                            * pow_nth(q, n - k),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "neg_binomial_cdf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let r = function.remove(i + 1).num()?.number;
                                    let p = function.remove(i + 1).num()?.number;
                                    Num(Number::from(
                                        regularized_incomplete_beta(p, r, k + 1),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "neg_binomial_pmf" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let r = function.remove(i + 1).num()?.number;
                                    let p = function.remove(i + 1).num()?.number;
                                    let q: Complex = 1 - p.clone();
                                    Num(Number::from(
                                        binomial(k.clone() + r.clone() - 1, k.clone())
                                            * pow_nth(p, r)
                                            * pow_nth(q, k),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "hypergeometric_cdf" =>
                            {
                                if i + 3 < function.len()
                                {
                                    let mut k = arg.num()?.number.real().clone().floor();
                                    let pop = function.remove(i + 1).num()?.number;
                                    let success = function.remove(i + 1).num()?.number;
                                    let draws = function.remove(i + 1).num()?.number;
                                    let mut sum = Complex::new(options.prec);
                                    while k >= 0
                                    {
                                        sum += binomial(success.clone(), k.clone().into())
                                            * binomial(
                                                pop.clone() - success.clone(),
                                                draws.clone() - k.clone(),
                                            )
                                            / binomial(pop.clone(), draws.clone());
                                        k -= 1
                                    }
                                    Num(Number::from(Complex::with_val(options.prec, sum), None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "hypergeometric_pmf" =>
                            {
                                if i + 3 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let pop = function.remove(i + 1).num()?.number;
                                    let success = function.remove(i + 1).num()?.number;
                                    let draws = function.remove(i + 1).num()?.number;
                                    Num(Number::from(
                                        binomial(success.clone(), k.clone())
                                            * binomial(
                                                pop.clone() - success.clone(),
                                                draws.clone() - k,
                                            )
                                            / binomial(pop, draws),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "rand_hypergeometric" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let mut pop = arg.num()?.number.real().clone();
                                    let mut success =
                                        function.remove(i + 1).num()?.number.real().clone();
                                    let mut draws =
                                        function.remove(i + 1).num()?.number.real().clone();
                                    let mut sum = Integer::new();
                                    while draws > 0
                                    {
                                        if success.clone() / pop.clone()
                                            > Float::with_val(options.prec, fastrand::u128(..))
                                                / u128::MAX
                                        {
                                            sum += 1;
                                            success -= 1;
                                        }
                                        pop -= 1;
                                        draws -= 1
                                    }
                                    Num(Number::from(Complex::with_val(options.prec, sum), None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "neg_hypergeometric_cdf" =>
                            {
                                if i + 3 < function.len()
                                {
                                    let mut k = arg.num()?.number.real().clone().floor();
                                    let pop = function.remove(i + 1).num()?.number;
                                    let success = function.remove(i + 1).num()?.number;
                                    let fails = function.remove(i + 1).num()?.number;
                                    let mut sum = Complex::new(options.prec);
                                    while k >= 0
                                    {
                                        sum += binomial(
                                            k.clone() + fails.clone() - 1,
                                            k.clone().into(),
                                        ) * binomial(
                                            pop.clone() - k.clone() - fails.clone(),
                                            success.clone() - k.clone(),
                                        ) / binomial(pop.clone(), success.clone());
                                        k -= 1
                                    }
                                    Num(Number::from(Complex::with_val(options.prec, sum), None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "neg_hypergeometric_pmf" =>
                            {
                                if i + 3 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let pop = function.remove(i + 1).num()?.number;
                                    let success = function.remove(i + 1).num()?.number;
                                    let fails = function.remove(i + 1).num()?.number;
                                    Num(Number::from(
                                        binomial(k.clone() + fails.clone() - 1, k.clone())
                                            * binomial(
                                                pop.clone() - k.clone() - fails,
                                                success.clone() - k,
                                            )
                                            / binomial(pop, success),
                                        None,
                                    ))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "rand_neg_hypergeometric" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let mut pop = arg.num()?.number.real().clone();
                                    let mut success =
                                        function.remove(i + 1).num()?.number.real().clone();
                                    let mut fails =
                                        function.remove(i + 1).num()?.number.real().clone();
                                    let mut sum = Integer::new();
                                    while fails > 0
                                    {
                                        if success.clone() / pop.clone()
                                            > Float::with_val(options.prec, fastrand::u128(..))
                                                / u128::MAX
                                        {
                                            sum += 1;
                                            success -= 1;
                                        }
                                        else
                                        {
                                            fails -= 1
                                        }
                                        pop -= 1;
                                    }
                                    Num(Number::from(Complex::with_val(options.prec, sum), None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "geometric_cdf" =>
                            {
                                if i + 1 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let p = function.remove(i + 1).num()?.number;
                                    let q: Complex = 1 - p.clone();
                                    Num(Number::from(1 - pow_nth(q, k), None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "geometric_pmf" =>
                            {
                                if i + 1 < function.len()
                                {
                                    let k = arg.num()?.number;
                                    let p = function.remove(i + 1).num()?.number;
                                    let q: Complex = 1 - p.clone();
                                    Num(Number::from(pow_nth(q, k - 1) * p, None))
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "quartic" =>
                            {
                                if i + 5 < function.len()
                                {
                                    let a = arg.num()?;
                                    let b = function.remove(i + 1).num()?;
                                    let c = function.remove(i + 1).num()?;
                                    let d = function.remove(i + 1).num()?;
                                    let e = function.remove(i + 1).num()?;
                                    function.remove(i + 1);
                                    let n = quartic(a, b, c, d, e, true);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 4 < function.len()
                                {
                                    let a = arg.num()?;
                                    let b = function.remove(i + 1).num()?;
                                    let c = function.remove(i + 1).num()?;
                                    let d = function.remove(i + 1).num()?;
                                    let e = function.remove(i + 1).num()?;
                                    let n = quartic(a, b, c, d, e, false);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 3 < function.len()
                                {
                                    let b = arg.num()?;
                                    let c = function.remove(i + 1).num()?;
                                    let d = function.remove(i + 1).num()?;
                                    let e = function.remove(i + 1).num()?;
                                    let n = quartic(
                                        Number::from(Complex::with_val(options.prec, 1), None),
                                        b,
                                        c,
                                        d,
                                        e,
                                        false,
                                    );
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "cubic" =>
                            {
                                if i + 4 < function.len()
                                {
                                    let a = arg.num()?;
                                    let b = function.remove(i + 1).num()?;
                                    let c = function.remove(i + 1).num()?;
                                    let d = function.remove(i + 1).num()?;
                                    function.remove(i + 1);
                                    let n = cubic(a, b, c, d, true);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 3 < function.len()
                                {
                                    let a = arg.num()?;
                                    let b = function.remove(i + 1).num()?;
                                    let c = function.remove(i + 1).num()?;
                                    let d = function.remove(i + 1).num()?;
                                    let n = cubic(a, b, c, d, false);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 2 < function.len()
                                {
                                    let b = arg.num()?;
                                    let c = function.remove(i + 1).num()?;
                                    let d = function.remove(i + 1).num()?;
                                    let n = cubic(
                                        Number::from(Complex::with_val(options.prec, 1), None),
                                        b,
                                        c,
                                        d,
                                        false,
                                    );
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "quad" | "quadratic" =>
                            {
                                if i + 3 < function.len()
                                {
                                    let a = arg.num()?;
                                    let b = function.remove(i + 1).num()?;
                                    let c = function.remove(i + 1).num()?;
                                    function.remove(i + 1);
                                    let n = quadratic(a, b, c, true);
                                    if n.is_empty()
                                    {
                                        Num(Number::from(
                                            Complex::with_val(options.prec, Nan),
                                            None,
                                        ))
                                    }
                                    else if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 2 < function.len()
                                {
                                    let a = arg.num()?;
                                    let b = function.remove(i + 1).num()?;
                                    let c = function.remove(i + 1).num()?;
                                    let n = quadratic(a, b, c, false);
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else if i + 1 < function.len()
                                {
                                    let b = arg.num()?;
                                    let c = function.remove(i + 1).num()?;
                                    let n = quadratic(
                                        Number::from(Complex::with_val(options.prec, 1), None),
                                        b,
                                        c,
                                        false,
                                    );
                                    if n.len() == 1
                                    {
                                        Num(n[0].clone())
                                    }
                                    else
                                    {
                                        Vector(n)
                                    }
                                }
                                else
                                {
                                    return Err("not enough args");
                                }
                            }
                            "cossin" =>
                            {
                                let (a, b) = arg.num()?.number.sin_cos(Complex::new(options.prec));
                                Vector(vec![Number::from(b, None), Number::from(a, None)])
                            }
                            "sincos" =>
                            {
                                let (a, b) = arg.num()?.number.sin_cos(Complex::new(options.prec));
                                Vector(vec![Number::from(a, None), Number::from(b, None)])
                            }
                            "split" =>
                            {
                                let a = arg.num()?.number;
                                Vector(vec![
                                    Number::from(a.real().clone().into(), None),
                                    Number::from(a.imag().clone().into(), None),
                                ])
                            }
                            "ceil" =>
                            {
                                let a = arg.num()?;
                                let units = a.units;
                                let m = if i + 1 < function.len()
                                {
                                    Complex::with_val(a.number.prec(), options.base.1)
                                        .pow(function.remove(i + 1).num()?.number)
                                }
                                else
                                {
                                    Complex::with_val(a.number.prec(), 1)
                                };
                                let a = a.number * m.clone();
                                Num(Number::from(
                                    Complex::with_val(
                                        options.prec,
                                        (a.real().clone().ceil(), a.imag().clone().ceil()),
                                    ) / m,
                                    units,
                                ))
                            }
                            "floor" =>
                            {
                                let a = arg.num()?;
                                let units = a.units;
                                let m = if i + 1 < function.len()
                                {
                                    Complex::with_val(a.number.prec(), options.base.1)
                                        .pow(function.remove(i + 1).num()?.number)
                                }
                                else
                                {
                                    Complex::with_val(a.number.prec(), 1)
                                };
                                let a = a.number * m.clone();
                                Num(Number::from(
                                    Complex::with_val(
                                        options.prec,
                                        (a.real().clone().floor(), a.imag().clone().floor()),
                                    ) / m,
                                    units,
                                ))
                            }
                            "round" =>
                            {
                                let a = arg.num()?;
                                let units = a.units;
                                let m = if i + 1 < function.len()
                                {
                                    Complex::with_val(a.number.prec(), options.base.1)
                                        .pow(function.remove(i + 1).num()?.number)
                                }
                                else
                                {
                                    Complex::with_val(a.number.prec(), 1)
                                };
                                let a = a.number * m.clone();
                                Num(Number::from(
                                    Complex::with_val(
                                        options.prec,
                                        (a.real().clone().round(), a.imag().clone().round()),
                                    ) / m,
                                    units,
                                ))
                            }
                            "int" | "trunc" =>
                            {
                                let a = arg.num()?;
                                let units = a.units;
                                let m = if i + 1 < function.len()
                                {
                                    Complex::with_val(a.number.prec(), options.base.1)
                                        .pow(function.remove(i + 1).num()?.number)
                                }
                                else
                                {
                                    Complex::with_val(a.number.prec(), 1)
                                };
                                let a = a.number * m.clone();
                                Num(Number::from(
                                    Complex::with_val(
                                        options.prec,
                                        (a.real().clone().trunc(), a.imag().clone().trunc()),
                                    ) / m,
                                    units,
                                ))
                            }
                            "frac" | "fract" =>
                            {
                                let a = arg.num()?;
                                let units = a.units;
                                let m = if i + 1 < function.len()
                                {
                                    Complex::with_val(a.number.prec(), options.base.1)
                                        .pow(function.remove(i + 1).num()?.number)
                                }
                                else
                                {
                                    Complex::with_val(a.number.prec(), 1)
                                };
                                let a = a.number * m.clone();
                                Num(Number::from(
                                    Complex::with_val(
                                        options.prec,
                                        (a.real().clone().fract(), a.imag().clone().fract()),
                                    ) / m,
                                    units,
                                ))
                            }
                            "iden" | "identity" => Matrix(identity(
                                arg.num()?
                                    .number
                                    .real()
                                    .to_integer()
                                    .unwrap_or_default()
                                    .to_usize()
                                    .unwrap_or_default(),
                                options.prec,
                            )),
                            "rotate" =>
                            {
                                if i + 2 < function.len()
                                {
                                    let (sina, cosa) = (arg.num()?.number / to_deg.clone())
                                        .sin_cos(Complex::new(options.prec));
                                    let (sinb, cosb) = (function.remove(i + 1).num()?.number
                                        / to_deg.clone())
                                    .sin_cos(Complex::new(options.prec));
                                    let (sinc, cosc) = (function.remove(i + 1).num()?.number
                                        / to_deg.clone())
                                    .sin_cos(Complex::new(options.prec));
                                    Matrix(vec![
                                        vec![
                                            Number::from(cosa.clone() * cosb.clone(), None),
                                            Number::from(
                                                cosa.clone() * sinb.clone() * sinc.clone()
                                                    - sina.clone() * cosc.clone(),
                                                None,
                                            ),
                                            Number::from(
                                                cosa.clone() * sinb.clone() * cosc.clone()
                                                    + sina.clone() * sinc.clone(),
                                                None,
                                            ),
                                        ],
                                        vec![
                                            Number::from(sina.clone() * cosb.clone(), None),
                                            Number::from(
                                                sina.clone() * sinb.clone() * sinc.clone()
                                                    + cosa.clone() * cosc.clone(),
                                                None,
                                            ),
                                            Number::from(
                                                sina.clone() * sinb.clone() * cosc.clone()
                                                    - cosa.clone() * sinc.clone(),
                                                None,
                                            ),
                                        ],
                                        vec![
                                            Number::from(-sinb.clone(), None),
                                            Number::from(cosb.clone() * sinc.clone(), None),
                                            Number::from(cosb.clone() * cosc.clone(), None),
                                        ],
                                    ])
                                }
                                else
                                {
                                    let (sin, cos) = (arg.num()?.number / to_deg.clone())
                                        .sin_cos(Complex::new(options.prec));
                                    Matrix(vec![
                                        vec![
                                            Number::from(cos.clone(), None),
                                            Number::from(-sin.clone(), None),
                                        ],
                                        vec![Number::from(sin, None), Number::from(cos, None)],
                                    ])
                                }
                            }
                            "prime_factors" =>
                            {
                                let a = arg.num()?.number;
                                if a.imag().is_zero()
                                {
                                    if a.real().clone().fract().is_zero()
                                    {
                                        let n = a.real().to_integer().unwrap_or_default();
                                        let m = prime_factors(n);
                                        if m.is_empty()
                                        {
                                            Num(Number::from(
                                                Complex::with_val(options.prec, Nan),
                                                None,
                                            ))
                                        }
                                        else
                                        {
                                            Matrix(
                                                m.iter()
                                                    .map(|(a, b)| {
                                                        vec![
                                                            Number::from(
                                                                Complex::with_val(options.prec, a),
                                                                None,
                                                            ),
                                                            Number::from(
                                                                Complex::with_val(options.prec, b),
                                                                None,
                                                            ),
                                                        ]
                                                    })
                                                    .collect::<Vec<Vec<Number>>>(),
                                            )
                                        }
                                    }
                                    else if let Some(a) = rationalize(a.real().clone(), options)
                                    {
                                        let mut p1 = prime_factors(a.0);
                                        let p2 = prime_factors(a.1);
                                        for p in p1.iter_mut()
                                        {
                                            for m in &p2
                                            {
                                                if p.0 == m.0
                                                {
                                                    p.1 -= m.1
                                                }
                                            }
                                        }
                                        for m in p2.iter()
                                        {
                                            if !p1.iter().any(|p| p.0 == m.0)
                                            {
                                                p1.push((m.0.clone(), -m.1));
                                            }
                                        }
                                        Matrix(
                                            p1.iter()
                                                .map(|p| {
                                                    vec![
                                                        Number::from(
                                                            Complex::with_val(
                                                                options.prec,
                                                                p.0.clone(),
                                                            ),
                                                            None,
                                                        ),
                                                        Number::from(
                                                            Complex::with_val(options.prec, p.1),
                                                            None,
                                                        ),
                                                    ]
                                                })
                                                .collect::<Vec<Vec<Number>>>(),
                                        )
                                    }
                                    else
                                    {
                                        Num(Number::from(
                                            Complex::with_val(options.prec, Nan),
                                            None,
                                        ))
                                    }
                                }
                                else
                                {
                                    Num(Number::from(Complex::with_val(options.prec, Nan), None))
                                }
                            }
                            "factors" | "factor" =>
                            {
                                let a = arg.num()?.number;
                                if a.imag().clone().is_zero()
                                {
                                    if a.real().clone().fract().is_zero()
                                    {
                                        let mut vec = Vec::new();
                                        let n = a
                                            .real()
                                            .to_integer()
                                            .unwrap_or_default()
                                            .to_usize()
                                            .unwrap_or_default();
                                        for i in 1..=n
                                        {
                                            if n % i == 0
                                            {
                                                vec.push(Number::from(
                                                    Complex::with_val(options.prec, i),
                                                    None,
                                                ));
                                            }
                                        }
                                        Vector(vec)
                                    }
                                    else
                                    {
                                        Num(Number::from(
                                            Complex::with_val(options.prec, Nan),
                                            None,
                                        ))
                                    }
                                }
                                else
                                {
                                    Num(Number::from(Complex::with_val(options.prec, Nan), None))
                                }
                            }
                            "unity" =>
                            {
                                let vec = if i + 1 < function.len()
                                    && !matches!(&function[i + 1], Func(_))
                                {
                                    unity(
                                        arg.num()?.number.ln(),
                                        function.remove(i + 1).num()?.number,
                                    )
                                }
                                else
                                {
                                    unity(Complex::new(options.prec), arg.num()?.number)
                                };
                                if vec.is_empty()
                                {
                                    Num(Number::from(Complex::with_val(options.prec, Nan), None))
                                }
                                else
                                {
                                    Vector(vec)
                                }
                            }
                            _ => do_functions(arg, options, &mut function, i, &to_deg, s)?,
                        },
                    }
                }
            }
        }
        i += 1;
    }
    i = 0;
    while i < function.len()
    {
        if let Func(s) = &function[i]
        {
            function[i] = match s.as_str()
            {
                "rnd" | "rand" => Num(Number::from(
                    Complex::with_val(options.prec, fastrand::u128(..)) / u128::MAX,
                    None,
                )),
                "epoch" => Num(Number::from(
                    Complex::with_val(
                        options.prec,
                        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                        {
                            Ok(n) => n.as_nanos(),
                            _ => return Err("epoch fail"),
                        },
                    ) / 1000000000,
                    Some(Units {
                        second: 1.0,
                        ..Units::default()
                    }),
                )),
                _ =>
                {
                    i += 1;
                    continue;
                }
            }
        }
        else
        {
            i += 1;
        }
    }
    i = 1;
    while i < function.len().saturating_sub(1)
    {
        function[i] = match &function[i]
        {
            Modulo => function[i - 1].func(&function[i + 1], rem)?,
            Range => to(&function[i - 1], &function[i + 1])?,
            _ =>
            {
                i += 1;
                continue;
            }
        };
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = function.len().saturating_sub(2);
    while i != 0
    {
        function[i] = match &function[i]
        {
            Exponent => function[i - 1].pow(&function[i + 1])?,
            Tetration => function[i - 1].func(&function[i + 1], tetration)?,
            Root => function[i - 1].func(&function[i + 1], root)?,
            _ =>
            {
                i -= 1;
                continue;
            }
        };
        function.remove(i + 1);
        function.remove(i - 1);
        i = i.saturating_sub(2);
    }
    i = 1;
    while i < function.len().saturating_sub(1)
    {
        function[i] = match &function[i]
        {
            InternalMultiplication => function[i - 1].mul(&function[i + 1])?,
            _ =>
            {
                i += 1;
                continue;
            }
        };
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len().saturating_sub(1)
    {
        function[i] = match &function[i]
        {
            Multiplication => function[i - 1].mul(&function[i + 1])?,
            Division => function[i - 1].func(&function[i + 1], div)?,
            _ =>
            {
                i += 1;
                continue;
            }
        };
        function.remove(i + 1);
        function.remove(i - 1);
    }
    i = 1;
    while i < function.len().saturating_sub(1)
    {
        function[i] = match &function[i]
        {
            PlusMinus => function[i - 1].pm(&function[i + 1])?,
            Plus => function[i - 1].func(&function[i + 1], add)?,
            Minus => function[i - 1].func(&function[i + 1], sub)?,
            _ =>
            {
                i += 1;
                continue;
            }
        };
        function.remove(i + 1);
        function.remove(i - 1);
    }
    if options.units
    {
        i = 1;
        while i < function.len().saturating_sub(1)
        {
            function[i] = match &function[i]
            {
                Conversion => function[i - 1].func(&function[i + 1], div)?,
                _ =>
                {
                    i += 1;
                    continue;
                }
            };
            function.remove(i + 1);
            function.remove(i - 1);
        }
    }
    i = 1;
    while i < function.len().saturating_sub(1)
    {
        function[i] = match &function[i]
        {
            Lesser =>
            {
                if i + 3 < function.len()
                    && matches!(
                        &function[i + 2],
                        Lesser | Greater | Equal | NearEqual | LesserEqual | GreaterEqual
                    )
                {
                    function[i - 1] = function[i + 1].func(&function[i - 1], gt)?;
                    function[i] = And;
                    continue;
                }
                function[i + 1].func(&function[i - 1], gt)?
            }
            LesserEqual =>
            {
                if i + 3 < function.len()
                    && matches!(
                        &function[i + 2],
                        Lesser | Greater | Equal | NearEqual | LesserEqual | GreaterEqual
                    )
                {
                    function[i - 1] = function[i + 1].func(&function[i - 1], ge)?;
                    function[i] = And;
                    continue;
                }
                function[i + 1].func(&function[i - 1], ge)?
            }
            Greater =>
            {
                if i + 3 < function.len()
                    && matches!(
                        &function[i + 2],
                        Lesser | Greater | Equal | NearEqual | LesserEqual | GreaterEqual
                    )
                {
                    function[i - 1] = function[i - 1].func(&function[i + 1], gt)?;
                    function[i] = And;
                    continue;
                }
                function[i - 1].func(&function[i + 1], gt)?
            }
            GreaterEqual =>
            {
                if i + 3 < function.len()
                    && matches!(
                        &function[i + 2],
                        Lesser | Greater | Equal | NearEqual | LesserEqual | GreaterEqual
                    )
                {
                    function[i - 1] = function[i - 1].func(&function[i + 1], ge)?;
                    function[i] = And;
                    continue;
                }
                function[i - 1].func(&function[i + 1], ge)?
            }
            Equal =>
            {
                if i + 3 < function.len()
                    && matches!(
                        &function[i + 2],
                        Lesser | Greater | Equal | NearEqual | LesserEqual | GreaterEqual
                    )
                {
                    function[i - 1] = function[i - 1].func(&function[i + 1], eq)?;
                    function[i] = And;
                    continue;
                }
                function[i - 1].func(&function[i + 1], eq)?
            }
            NotEqual =>
            {
                if i + 3 < function.len()
                    && matches!(
                        &function[i + 2],
                        Lesser | Greater | Equal | NearEqual | LesserEqual | GreaterEqual
                    )
                {
                    function[i - 1] = function[i - 1].func(&function[i + 1], ne)?;
                    function[i] = And;
                    continue;
                }
                function[i - 1].func(&function[i + 1], ne)?
            }
            NearEqual =>
            {
                if i + 3 < function.len()
                    && matches!(
                        &function[i + 2],
                        Lesser | Greater | Equal | NearEqual | LesserEqual | GreaterEqual
                    )
                {
                    function[i - 1] = function[i - 1].func(&function[i + 1], about_eq)?;
                    function[i] = And;
                    continue;
                }
                function[i - 1].func(&function[i + 1], about_eq)?
            }
            ShiftRight => function[i - 1].func(&function[i + 1], shr)?,
            ShiftLeft => function[i - 1].func(&function[i + 1], shl)?,
            _ =>
            {
                i += 1;
                continue;
            }
        };
        function.remove(i + 1);
        function.remove(i - 1);
    }
    if !function.is_empty() && function[0] == Not
    {
        function[0] = not(&function.remove(1))?;
    }
    i = 1;
    while i < function.len().saturating_sub(1)
    {
        function[i] = match &function[i]
        {
            And => function[i - 1].func(&function[i + 1], and)?,
            Or => function[i - 1].func(&function[i + 1], or)?,
            Xor => function[i - 1].func(&function[i + 1], xor)?,
            Implies => function[i - 1].func(&function[i + 1], implies)?,
            Nand => function[i - 1].func(&function[i + 1], nand)?,
            Nor => function[i - 1].func(&function[i + 1], nor)?,
            Converse => function[i + 1].func(&function[i - 1], implies)?,
            Not =>
            {
                function[i] = not(&function.remove(i + 1))?;
                continue;
            }
            _ =>
            {
                i += 1;
                continue;
            }
        };
        function.remove(i + 1);
        function.remove(i - 1);
    }
    if function.len() == 1
    {
        Ok(function[0].clone())
    }
    else
    {
        Err("failed to compute")
    }
}
fn do_functions(
    a: NumStr,
    options: Options,
    function: &mut Vec<NumStr>,
    k: usize,
    to_deg: &Complex,
    s: &str,
) -> Result<NumStr, &'static str>
{
    if function.len() > k + 1
    {
        match (a.clone(), function[k + 1].clone())
        {
            (Num(a), Num(b)) =>
            {
                function.remove(k + 1);
                return Ok(Num(functions(a, Some(b), to_deg.clone(), s, options)?));
            }
            (Vector(a), Vector(b)) =>
            {
                function.remove(k + 1);
                let mut mat = Vec::new();
                for a in a
                {
                    let mut vec = Vec::new();
                    for b in &b
                    {
                        vec.push(functions(
                            a.clone(),
                            Some(b.clone()),
                            to_deg.clone(),
                            s,
                            options,
                        )?)
                    }
                    mat.push(vec);
                }
                return Ok(Matrix(mat));
            }
            (Matrix(a), Matrix(b))
                if a.len() == b.len() && (0..a.len()).all(|i| a[i].len() == b[i].len()) =>
            {
                function.remove(k + 1);
                let mut mat = Vec::new();
                for i in 0..a.len()
                {
                    let mut vec = Vec::new();
                    for j in 0..a[i].len()
                    {
                        vec.push(functions(
                            a[i][j].clone(),
                            Some(b[i][j].clone()),
                            to_deg.clone(),
                            s,
                            options,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            (Num(a), Vector(b)) =>
            {
                function.remove(k + 1);
                let mut vec = Vec::new();
                for i in b
                {
                    vec.push(functions(a.clone(), Some(i), to_deg.clone(), s, options)?)
                }
                return Ok(Vector(vec));
            }
            (Vector(a), Num(b)) =>
            {
                function.remove(k + 1);
                let mut vec = Vec::new();
                for i in a
                {
                    vec.push(functions(i, Some(b.clone()), to_deg.clone(), s, options)?)
                }
                return Ok(Vector(vec));
            }
            (Num(a), Matrix(b)) =>
            {
                function.remove(k + 1);
                let mut mat = Vec::new();
                for i in b
                {
                    let mut vec = Vec::new();
                    for j in i
                    {
                        vec.push(functions(a.clone(), Some(j), to_deg.clone(), s, options)?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            (Matrix(a), Num(b)) =>
            {
                function.remove(k + 1);
                let mut mat = Vec::new();
                for i in a
                {
                    let mut vec = Vec::new();
                    for j in i
                    {
                        vec.push(functions(j, Some(b.clone()), to_deg.clone(), s, options)?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            (Matrix(a), Vector(b)) if a.len() == b.len() =>
            {
                function.remove(k + 1);
                let mut mat = Vec::new();
                for i in 0..a.len()
                {
                    let mut vec = Vec::new();
                    for j in 0..a[i].len()
                    {
                        vec.push(functions(
                            a[i][j].clone(),
                            Some(b[i].clone()),
                            to_deg.clone(),
                            s,
                            options,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            (Vector(a), Matrix(b)) if a.len() == b.len() =>
            {
                function.remove(k + 1);
                let mut mat = Vec::new();
                for i in 0..b.len()
                {
                    let mut vec = Vec::new();
                    for j in 0..b[i].len()
                    {
                        vec.push(functions(
                            a[i].clone(),
                            Some(b[i][j].clone()),
                            to_deg.clone(),
                            s,
                            options,
                        )?)
                    }
                    mat.push(vec.clone());
                }
                return Ok(Matrix(mat));
            }
            _ =>
            {}
        }
    }
    match a
    {
        Matrix(a) =>
        {
            let mut mat = Vec::new();
            for i in a
            {
                let mut vec = Vec::new();
                for j in i
                {
                    vec.push(functions(j, None, to_deg.clone(), s, options)?)
                }
                mat.push(vec.clone());
            }
            Ok(Matrix(mat))
        }
        Vector(a) =>
        {
            let mut vec = Vec::new();
            for i in a
            {
                vec.push(functions(i, None, to_deg.clone(), s, options)?)
            }
            Ok(Vector(vec))
        }
        Num(a) => Ok(Num(functions(a, None, to_deg.clone(), s, options)?)),
        _ => Err("str err1"),
    }
}
fn functions(
    mut a: Number,
    mut c: Option<Number>,
    to_deg: Complex,
    s: &str,
    options: Options,
) -> Result<Number, &'static str>
{
    if a.number.imag().is_zero() && !a.number.imag().is_sign_positive()
    {
        a.number = Complex::with_val(a.number.prec(), a.number.real())
    }
    let n = if matches!(
        s,
        "root"
            | "sqrt"
            | "asquare"
            | "exp"
            | "aln"
            | "square"
            | "asqrt"
            | "cube"
            | "acbrt"
            | "asin"
            | "arcsin"
            | "acsc"
            | "arccsc"
            | "acos"
            | "arccos"
            | "asec"
            | "arcsec"
            | "atan2"
            | "atan"
            | "arctan"
            | "acot"
            | "arccot"
            | "ceil"
            | "floor"
            | "round"
            | "frac"
            | "fract"
            | "cbrt"
            | "acube"
            | "units"
            | "int"
            | "trunc"
            | "recip"
            | "abs"
            | "norm"
            | "onlyreal"
            | "onlyre"
            | "ore"
            | "onlyimag"
            | "onlyim"
            | "oim"
            | "re"
            | "real"
            | "im"
            | "imag"
            | "next"
            | "rand_norm"
            | "rand_uniform"
            | "rand_int"
    )
    {
        if let Some(ref b) = c
        {
            if b.number.imag().is_zero() && !b.number.imag().is_sign_positive()
            {
                c = Some(Number::from(
                    Complex::with_val(b.number.prec(), b.number.real()),
                    b.units,
                ))
            }
        }
        match s
        {
            "sqrt" | "asquare" => Number::from(a.number.sqrt(), a.units.map(|a| a.root(2.0))),
            "root" =>
            {
                if let Some(b) = c
                {
                    root(&a, &b)
                }
                else
                {
                    Number::from(a.number.sqrt(), a.units.map(|a| a.root(2.0)))
                }
            }
            "exp" | "aln" =>
            {
                if let Some(b) = c
                {
                    Number::from(
                        pow_nth(a.number, b.number.clone()),
                        a.units.map(|a| a.pow(b.number.real().to_f64())),
                    )
                }
                else
                {
                    Number::from(a.number.exp(), None)
                }
            }
            "square" | "asqrt" => Number::from(sqr(a.number), a.units.map(|a| a.pow(2.0))),
            "cube" | "acbrt" => Number::from(cube(a.number), a.units.map(|a| a.pow(3.0))),
            "asin" | "arcsin" => Number::from(
                a.number.clone().asin() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "acsc" | "arccsc" => Number::from(
                a.number.clone().recip().asin() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "acos" | "arccos" => Number::from(
                a.number.clone().acos() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "asec" | "arcsec" => Number::from(
                a.number.clone().recip().acos() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "atan2" =>
            {
                if let Some(b) = c
                {
                    Number::from(
                        atan(b.number, a.number) * to_deg,
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    )
                }
                else
                {
                    return Err("not enough args");
                }
            }
            "atan" | "arctan" =>
            {
                if let Some(b) = c
                {
                    Number::from(
                        atan(a.number, b.number) * to_deg,
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    )
                }
                else
                {
                    Number::from(
                        a.number.atan() * to_deg,
                        Some(Units {
                            angle: 1.0,
                            ..Units::default()
                        }),
                    )
                }
            }
            "acot" | "arccot" => Number::from(
                a.number.recip().atan() * to_deg,
                Some(Units {
                    angle: 1.0,
                    ..Units::default()
                }),
            ),
            "cbrt" | "acube" => Number::from(
                if a.number.imag().is_zero()
                {
                    if a.number.real().is_zero()
                    {
                        Complex::new(options.prec)
                    }
                    else if a.number.real().is_sign_positive()
                    {
                        pow_nth(a.number, Complex::with_val(options.prec, 3).recip())
                    }
                    else
                    {
                        -pow_nth(-a.number, Complex::with_val(options.prec, 3).recip())
                    }
                }
                else
                {
                    pow_nth(a.number, Complex::with_val(options.prec, 3).recip())
                },
                a.units.map(|a| a.root(3.0)),
            ),
            "abs" | "norm" => Number::from(a.number.abs(), a.units),
            "recip" => Number::from(a.number.recip(), a.units.map(|a| a.pow(-1.0))),
            "units" => Number::from(Complex::with_val(options.prec, 1), a.units),
            "onlyreal" | "onlyre" | "ore" =>
            {
                if -a.number.imag().clone().abs().log10() > a.number.prec().0 / 4
                {
                    Number::from(a.number.real().clone().into(), a.units)
                }
                else
                {
                    Number::from(Complex::with_val(options.prec, Nan), None)
                }
            }
            "onlyimag" | "onlyim" | "oim" =>
            {
                if -a.number.real().clone().abs().log10() > a.number.prec().0 / 4
                {
                    Number::from(a.number.imag().clone().into(), a.units)
                }
                else
                {
                    Number::from(Complex::with_val(options.prec, Nan), None)
                }
            }
            "re" | "real" => Number::from(a.number.real().clone().into(), a.units),
            "im" | "imag" => Number::from(a.number.imag().clone().into(), a.units),
            "next" =>
            {
                if let Some(b) = c
                {
                    let mut real: Float = a.number.real().clone();
                    let imag: Float = a.number.imag().clone();
                    if b.number.real().is_infinite()
                    {
                        if b.number.real().is_sign_positive()
                        {
                            real.next_up()
                        }
                        else
                        {
                            real.next_down()
                        }
                    }
                    else
                    {
                        real.next_toward(b.number.real());
                    }
                    Number::from(Complex::with_val(options.prec, (real, imag)), a.units)
                }
                else
                {
                    let mut real: Float = a.number.real().clone();
                    let imag: Float = a.number.imag().clone();
                    real.next_up();
                    Number::from(Complex::with_val(options.prec, (real, imag)), a.units)
                }
            }
            "rand_norm" =>
            {
                if let Some(b) = c
                {
                    Number::from(rand_norm(a.number, b.number), a.units)
                }
                else
                {
                    return Err("not enough args");
                }
            }
            "rand_uniform" =>
            {
                if let Some(b) = c
                {
                    let units = a.units;
                    let a = a.number;
                    let b = b.number;
                    Number::from(
                        (b.clone() - a.clone()) * Float::with_val(options.prec, fastrand::u128(..))
                            / u128::MAX
                            + if a.real() < b.real() { a } else { b },
                        units,
                    )
                }
                else
                {
                    return Err("not enough args");
                }
            }
            "rand_int" =>
            {
                if let Some(b) = c
                {
                    let units = a.units;
                    let ar = a
                        .number
                        .real()
                        .to_integer()
                        .unwrap_or_default()
                        .to_i128()
                        .unwrap_or_default();
                    let br = b
                        .number
                        .real()
                        .to_integer()
                        .unwrap_or_default()
                        .to_i128()
                        .unwrap_or_default();
                    let ai = a
                        .number
                        .imag()
                        .to_integer()
                        .unwrap_or_default()
                        .to_i128()
                        .unwrap_or_default();
                    let bi = b
                        .number
                        .imag()
                        .to_integer()
                        .unwrap_or_default()
                        .to_i128()
                        .unwrap_or_default();
                    Number::from(
                        Complex::with_val(
                            options.prec,
                            (
                                fastrand::i128(ar.min(br)..=br.max(ar)),
                                fastrand::i128(ai.min(bi)..=bi.max(ai)),
                            ),
                        ),
                        units,
                    )
                }
                else
                {
                    return Err("not enough args");
                }
            }
            _ => return Err("unreachable"),
        }
    }
    else
    {
        let a = a.number;
        let mut d = None;
        if let Some(ref b) = c
        {
            if b.number.imag().is_zero() && !b.number.imag().is_sign_positive()
            {
                d = Some(Complex::with_val(b.number.prec(), b.number.real()))
            }
            else
            {
                d = Some(b.number.clone())
            }
        }
        Number::from(
            match s
            {
                "sin" => (a / to_deg).sin(),
                "csc" => (a / to_deg).sin().recip(),
                "cos" => (a / to_deg).cos(),
                "sec" => (a / to_deg).cos().recip(),
                "tan" => (a / to_deg).tan(),
                "cot" => (a / to_deg).tan().recip(),
                "sinh" => a.sinh(),
                "csch" => a.sinh().recip(),
                "cosh" => a.cosh(),
                "sech" => a.cosh().recip(),
                "tanh" => a.tanh(),
                "coth" => a.tanh().recip(),
                "asinh" | "arcsinh" => a.asinh(),
                "acsch" | "arccsch" => a.recip().asinh(),
                "acosh" | "arccosh" => a.acosh(),
                "asech" | "arcsech" => a.clone().recip().acosh(),
                "atanh" | "arctanh" => a.clone().atanh(),
                "acoth" | "arccoth" => a.clone().recip().atanh(),
                "cis" =>
                {
                    let b = a / to_deg.clone();
                    b.clone().cos() + b.sin() * Complex::with_val(options.prec, (0.0, 1.0))
                }
                "ln" | "aexp" => a.ln(),
                "W" | "productlog" | "lambertw" =>
                {
                    if let Some(b) = d
                    {
                        lambertw(b, a.real().to_integer().unwrap_or_default())
                    }
                    else
                    {
                        lambertw(a, Integer::new())
                    }
                }
                "log" =>
                {
                    let a = a.ln();
                    if let Some(b) = d
                    {
                        let b = b.ln();
                        if a.is_zero()
                        {
                            if b.is_zero()
                            {
                                Complex::with_val(options.prec, Nan)
                            }
                            else
                            {
                                Complex::with_val(options.prec, Infinity)
                            }
                        }
                        else if b.real().is_infinite()
                        {
                            -Complex::with_val(options.prec, Infinity)
                        }
                        else
                        {
                            b / a
                        }
                    }
                    else
                    {
                        a
                    }
                }
                "ssrt" =>
                {
                    if let Some(b) = d
                    {
                        let b = b.ln();
                        b.clone() / lambertw(b, a.real().to_integer().unwrap_or_default())
                    }
                    else
                    {
                        let a = a.ln();
                        a.clone() / lambertw(a, Integer::new())
                    }
                }
                "slog" =>
                {
                    if let Some(b) = d
                    {
                        if a.real() > &1
                        {
                            slog(&a, &b)
                        }
                        else
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                    }
                    else
                    {
                        return Err("no args");
                    }
                }
                "Ap" =>
                {
                    if let Some(b) = d
                    {
                        if a.real().is_sign_negative()
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                        else
                        {
                            pub fn pow_n(z: Complex, n: usize) -> Complex
                            {
                                if !z.imag().is_zero() && n <= 256
                                {
                                    let mut p = z.clone();
                                    for _ in 1..n
                                    {
                                        p *= &z;
                                    }
                                    p
                                }
                                else
                                {
                                    z.pow(n)
                                }
                            }
                            let mut sum = Complex::new(options.prec);
                            let n = a
                                .real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_u32()
                                .unwrap_or_default();
                            for k in 0..=n
                            {
                                sum += pow_n(b.clone(), k as usize) * euleriannumbersint(n, k)
                            }
                            sum
                        }
                    }
                    else
                    {
                        return Err("no args");
                    }
                }
                "An" =>
                {
                    if let Some(b) = d
                    {
                        euleriannumbers(
                            a,
                            b.real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_i32()
                                .unwrap_or_default(),
                        )
                    }
                    else
                    {
                        return Err("no args");
                    }
                }
                "P" =>
                {
                    if let Some(b) = d
                    {
                        if a.real().is_sign_negative()
                            && a.real().clone().fract().is_zero()
                            && a.imag().is_zero()
                            && b.imag().is_zero()
                        {
                            let a = a + Complex::with_val(
                                options.prec,
                                (0, Float::with_val(options.prec, 0.5).pow(options.prec / 2)),
                            );
                            (gamma(a.clone() + 1) / gamma(a.clone() - b + 1))
                                .real()
                                .clone()
                                .into()
                        }
                        else
                        {
                            gamma(a.clone() + 1) / gamma(a.clone() - b + 1)
                        }
                    }
                    else
                    {
                        return Err("no args");
                    }
                }
                "C" | "bi" | "binomial" =>
                {
                    if let Some(b) = d
                    {
                        binomial(a, b)
                    }
                    else
                    {
                        return Err("no args");
                    }
                }
                "pochhammer" | "ph" =>
                {
                    if let Some(b) = d
                    {
                        if !a.real().is_sign_positive() && a.imag().is_zero() && b.imag().is_zero()
                        {
                            pow_nth(Complex::with_val(options.prec, -1), b.clone())
                                * gamma(1 - a.clone())
                                / gamma(1 - a - b)
                        }
                        else
                        {
                            gamma(b.clone() + a.clone()) / gamma(a.clone())
                        }
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "lower_gamma" | "γ" =>
                {
                    if let Some(b) = d
                    {
                        lower_incomplete_gamma(a, b)
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "gamma" | "Γ" =>
                {
                    if let Some(b) = d
                    {
                        incomplete_gamma(a, b)
                    }
                    else
                    {
                        gamma(a)
                    }
                }
                "sgn" | "sign" =>
                {
                    if a.is_zero()
                    {
                        Complex::new(options.prec)
                    }
                    else
                    {
                        a.clone() / a.abs()
                    }
                }
                "arg" => a.arg(),
                "doublefact" | "doublefactorial" =>
                {
                    let two = Complex::with_val(options.prec, 2);
                    let pi = Complex::with_val(options.prec, Pi);
                    two.pow(a.clone() / 2 + (1 - (pi.clone() * a.clone()).cos()) / 4)
                        * pi.clone().pow(((pi * a.clone()).cos() - 1) / 4)
                        * gamma(a.clone() / 2 + 1)
                }
                "fact" | "factorial" => gamma(a.clone() + 1),
                "subfact" | "subfactorial" =>
                {
                    if !a.imag().is_zero()
                        || a.real().is_sign_negative()
                        || !a.real().clone().fract().is_zero()
                    {
                        subfactorial(a)
                    }
                    else if a.real().is_zero()
                    {
                        Complex::with_val(options.prec, 1)
                    }
                    else
                    {
                        (gamma(a.clone() + 1) / Float::with_val(options.prec, 1).exp())
                            .real()
                            .clone()
                            .round()
                            .into()
                    }
                }
                "sinc" => a.clone().sin() / a,
                "conj" | "conjugate" => a.conj(),
                "erf" =>
                {
                    if a.imag().is_zero()
                    {
                        a.real().clone().erf().into()
                    }
                    else
                    {
                        erf(a)
                    }
                }
                "erfc" =>
                {
                    if a.imag().is_zero()
                    {
                        a.real().clone().erfc().into()
                    }
                    else
                    {
                        erfc(a)
                    }
                }
                "erfi" =>
                {
                    let i = Complex::with_val(options.prec, (0, 1));
                    -i.clone() * erf(i * a)
                }
                "ai" =>
                {
                    if a.imag().is_zero()
                    {
                        a.real().clone().ai().into()
                    }
                    else
                    {
                        Complex::with_val(options.prec, Nan)
                    }
                }
                "trigamma" => digamma(a, 1),
                "digamma" | "polygamma" | "ψ" =>
                {
                    if let Some(b) = d
                    {
                        digamma(
                            b,
                            a.real()
                                .to_integer()
                                .unwrap_or_default()
                                .to_u32()
                                .unwrap_or_default(),
                        )
                    }
                    else if a.imag().is_zero()
                    {
                        if a.real().is_sign_negative() && a.real().clone().fract().is_zero()
                        {
                            Complex::with_val(options.prec, Infinity)
                        }
                        else
                        {
                            a.real().clone().digamma().into()
                        }
                    }
                    else
                    {
                        digamma(a, 0)
                    }
                }
                "eta" | "η" => eta(a),
                "zeta" | "ζ" =>
                {
                    if a.imag().is_zero()
                    {
                        a.real().clone().zeta().into()
                    }
                    else
                    {
                        zeta(a)
                    }
                }
                "nth_prime" | "prime" =>
                {
                    if a.imag().is_zero() && a.real().clone().fract().is_zero()
                    {
                        Complex::with_val(
                            options.prec,
                            nth_prime(a.real().to_integer().unwrap_or_default()),
                        )
                    }
                    else
                    {
                        Complex::with_val(options.prec, Nan)
                    }
                }
                "mod" =>
                {
                    if let Some(b) = d
                    {
                        let c = a.clone() / b.clone();
                        let c = Complex::with_val(
                            options.prec,
                            (c.real().clone().floor(), c.imag().clone().floor()),
                        );
                        a - b * c
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "lcm" =>
                {
                    if let Some(b) = d
                    {
                        if !a.real().is_finite()
                            || !b.real().is_finite()
                            || b.real().is_zero()
                            || a.real().is_zero()
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                        else
                        {
                            let a = a.real().clone().abs().to_integer().unwrap_or_default();
                            let b = b.real().clone().abs().to_integer().unwrap_or_default();
                            Complex::with_val(options.prec, a.clone() * b.clone() / gcd(a, b))
                        }
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "gcd" | "gcf" =>
                {
                    if let Some(b) = d
                    {
                        if !a.real().is_finite()
                            || !b.real().is_finite()
                            || b.real().is_zero()
                            || a.real().is_zero()
                        {
                            Complex::with_val(options.prec, Nan)
                        }
                        else
                        {
                            Complex::with_val(
                                options.prec,
                                gcd(
                                    a.real().clone().abs().to_integer().unwrap_or_default(),
                                    b.real().clone().abs().to_integer().unwrap_or_default(),
                                ),
                            )
                        }
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "is_nan" => Complex::with_val(options.prec, a.real().is_nan() as u8),
                "is_inf" => Complex::with_val(options.prec, a.real().is_infinite() as u8),
                "is_fin" | "is_finite" =>
                {
                    Complex::with_val(options.prec, a.real().is_finite() as u8)
                }
                "isprime" | "is_prime" =>
                {
                    if a.imag().is_zero()
                        && a.real().clone().fract().is_zero()
                        && a.real().is_finite()
                    {
                        Complex::with_val(
                            options.prec,
                            (a.real()
                                .to_integer()
                                .unwrap_or_default()
                                .is_probably_prime(100)
                                != IsPrime::No) as u8,
                        )
                    }
                    else
                    {
                        Complex::new(options.prec)
                    }
                }
                "rand_gamma" =>
                {
                    if let Some(b) = d
                    {
                        rand_gamma(a.real().clone(), b.real().clone()).into()
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "rand_beta" =>
                {
                    if let Some(b) = d
                    {
                        let x = rand_gamma(a.real().clone(), Float::with_val(options.prec, 1));
                        let y = rand_gamma(b.real().clone(), Float::with_val(options.prec, 1));
                        (x.clone() / (x + y)).into()
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "rand_lognorm" =>
                {
                    if let Some(b) = d
                    {
                        rand_norm(a, b).exp()
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "rand_bernoulli" =>
                {
                    if *a.real() > Float::with_val(options.prec, fastrand::u128(..)) / u128::MAX
                    {
                        Complex::with_val(options.prec, 1)
                    }
                    else
                    {
                        Complex::new(options.prec)
                    }
                }
                "rand_binomial" =>
                {
                    if let Some(b) = d
                    {
                        let p = b.real();
                        let mut n = a.real().to_integer().unwrap_or_default();
                        let mut sum = Integer::new();
                        while n > 0
                        {
                            if *p > Float::with_val(options.prec, fastrand::u128(..)) / u128::MAX
                            {
                                sum += 1;
                            }
                            n -= 1
                        }
                        Complex::with_val(options.prec, sum)
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "rand_neg_binomial" =>
                {
                    if let Some(b) = d
                    {
                        let p = b.real();
                        if *p <= 0
                        {
                            return Err("p must be greater then 0");
                        }
                        let mut r = a.real().to_integer().unwrap_or_default();
                        let mut sum = Integer::new();
                        while r > 0
                        {
                            if *p > Float::with_val(options.prec, fastrand::u128(..)) / u128::MAX
                            {
                                r -= 1
                            }
                            else
                            {
                                sum += 1;
                            }
                        }
                        Complex::with_val(options.prec, sum)
                    }
                    else
                    {
                        return Err("not enough args");
                    }
                }
                "rand_geometric" =>
                {
                    let q: Float = 1 - a.real().clone();
                    let n: Float = (Float::with_val(options.prec, fastrand::u128(..)) / u128::MAX)
                        .ln()
                        / q.ln();
                    n.ceil().into()
                }
                "rand_poisson" =>
                {
                    let mut prod = Complex::with_val(options.prec, 1);
                    let lim = (-a).exp();
                    let mut n = Integer::new();
                    while lim.real() < prod.real()
                    {
                        prod *= Float::with_val(options.prec, fastrand::u128(..)) / u128::MAX;
                        n += 1;
                    }
                    Complex::with_val(options.prec, n - 1)
                }
                _ =>
                {
                    return Err("wrong input type");
                }
            },
            None,
        )
    };
    if n.number.imag().is_zero() && !n.number.imag().is_sign_positive()
    {
        Ok(Number::from(n.number.real().clone().into(), n.units))
    }
    else
    {
        Ok(n)
    }
}
pub fn compute_funcvars(
    function: &mut [NumStr],
    options: Options,
    func_vars: &mut Vec<(String, Vec<NumStr>)>,
)
{
    let mut i = 0;
    while i < func_vars.len()
    {
        let v = func_vars[i].clone();
        let mut cont = false;
        for f in function.iter_mut()
        {
            if let Func(s) = &f
            {
                if *s == v.0
                {
                    cont = true;
                    break;
                }
            }
        }
        if !cont && i + 1 < func_vars.len()
        {
            for fv in func_vars[i + 1..].iter_mut()
            {
                for f in fv.1.iter_mut()
                {
                    if let Func(s) = &f
                    {
                        if *s == v.0
                        {
                            cont = true;
                            break;
                        }
                    }
                }
            }
        }
        if cont
            && (v.1.len() != 1
                || (if let Func(s) = &v.1[0]
                {
                    matches!(s.as_str(), "rnd" | "rand" | "epoch")
                }
                else
                {
                    false
                }))
            && !v.0.ends_with(')')
        {
            if let Ok(n) = do_math(v.1.clone(), options, func_vars[..i].to_vec())
            {
                for f in function.iter_mut()
                {
                    if let Func(s) = &f
                    {
                        if *s == v.0
                        {
                            *f = n.clone();
                        }
                    }
                }
                if i + 1 < func_vars.len()
                {
                    for fv in func_vars[i + 1..].iter_mut()
                    {
                        for f in fv.1.iter_mut()
                        {
                            if let Func(s) = &f
                            {
                                if *s == v.0
                                {
                                    *f = n.clone();
                                }
                            }
                        }
                    }
                }
                func_vars.remove(i);
                continue;
            }
        }
        i += 1;
    }
}
