use crate::complex::NumStr;
use crate::{
    complex::NumStr::{
        Comma, Division, Exponent, Func, LeftBracket, Minus, Multiplication, Plus, RightBracket,
    },
    load_vars::get_vars,
    math::do_math,
    parse::input_var,
    units::{Number, Options},
};
use rug::{Complex, float::Constant::Pi};
#[test]
fn test_math() {
    let output = input_var(
        "pi+tau*e/2i^(sqrt(2))/3*3-log(2-2i,-3+i)+sqrt(2)^(sqrt(2))",
        &get_vars(Options::default()),
        &mut Vec::new(),
        &mut 0,
        Options::default(),
        false,
        0,
        Vec::new(),
        false,
        &mut Vec::new(),
        None,
        None,
    )
    .unwrap();
    let expected = vec![
        NumStr::new(Number::from(Complex::with_val(512, Pi), None)),
        Plus,
        NumStr::new(Number::from(2 * Complex::with_val(512, Pi), None)),
        Multiplication,
        NumStr::new(Number::from(Complex::with_val(512, 1).exp(), None)),
        Division,
        NumStr::new(Number::from(Complex::with_val(512, 2), None)),
        Multiplication,
        NumStr::new(Number::from(Complex::with_val(512, (0.0, 1)), None)),
        Exponent,
        LeftBracket,
        Func("sqrt".to_string()),
        LeftBracket,
        NumStr::new(Number::from(Complex::with_val(512, 2), None)),
        RightBracket,
        RightBracket,
        Division,
        NumStr::new(Number::from(Complex::with_val(512, 3), None)),
        Multiplication,
        NumStr::new(Number::from(Complex::with_val(512, 3), None)),
        Minus,
        Func("log".to_string()),
        LeftBracket,
        NumStr::new(Number::from(Complex::with_val(512, 2), None)),
        Minus,
        NumStr::new(Number::from(Complex::with_val(512, 2), None)),
        Multiplication,
        NumStr::new(Number::from(Complex::with_val(512, (0, 1)), None)),
        Comma,
        NumStr::new(Number::from(Complex::with_val(512, -3), None)),
        Plus,
        NumStr::new(Number::from(Complex::with_val(512, (0, 1)), None)),
        RightBracket,
        Plus,
        Func("sqrt".to_string()),
        LeftBracket,
        NumStr::new(Number::from(Complex::with_val(512, 2), None)),
        RightBracket,
        Exponent,
        LeftBracket,
        Func("sqrt".to_string()),
        LeftBracket,
        NumStr::new(Number::from(Complex::with_val(512, 2), None)),
        RightBracket,
        RightBracket,
    ];
    let out = do_math(output.0, Options::default(), Vec::new())
        .unwrap()
        .num()
        .unwrap();
    let answer = do_math(expected, Options::default(), Vec::new())
        .unwrap()
        .num()
        .unwrap();
    assert_eq!(
        out.number.real().to_string(),
        answer.number.real().to_string()
    );
    assert_eq!(
        out.number.imag().to_string(),
        answer.number.imag().to_string()
    );
    assert_eq!(&out.number.real().to_string()[..20], "2.009877988310399125");
    assert_eq!(&out.number.imag().to_string()[..20], "4.535664430265577075");
}
