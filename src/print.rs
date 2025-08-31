use crate::{
    complex::{
        NumStr,
        NumStr::{Matrix, Num, Vector},
        to_polar,
    },
    fraction::fraction,
    help::help_for,
    load_vars::set_commands_or_vars,
    math::do_math,
    misc::{
        clear, get_terminal_dimensions, handle_err, insert_last, no_col, no_col_len,
        parsed_to_string, prompt, to_output,
    },
    options::{equal_to, list_vars, silent_commands},
    parse::input_var,
    units::{
        AngleType, Auto, Colors, HowGraphing, Notation, Notation::Normal, Number, Options, Units,
        Variable,
    },
};
use rug::{
    Complex, Float, Integer,
    float::Constant::Pi,
    ops::{CompleteRound, Pow},
};
use std::cmp::Ordering;
#[allow(clippy::too_many_arguments)]
pub fn print_concurrent(
    unmodified_input: &[char],
    last: &[char],
    vars: &[Variable],
    mut options: Options,
    mut colors: Colors,
    start: usize,
    end: usize,
    long_output: bool,
) -> (usize, HowGraphing, bool, bool) {
    let mut vars = vars;
    if unmodified_input.starts_with(&['#']) || unmodified_input.is_empty() {
        clear(unmodified_input, vars, start, end, options, &colors);
        return (0, HowGraphing::default(), false, false);
    }
    if matches!(
        unmodified_input.iter().collect::<String>().as_str(),
        "vars" | "variables"
    ) {
        let list = list_vars(vars, &options, &colors);
        let (width, height) = get_terminal_dimensions();
        let n = no_col(&list, options.color == Auto::True)
            .iter()
            .collect::<String>()
            .split("\n")
            .map(|a| a.len().div_ceil(width))
            .sum();
        return if n + 1 >= height {
            print!(
                "\x1b[J\x1b[G\ntoo long, will print on enter\x1b[G\x1b[A\x1b[K{}{}{}",
                prompt(options, &colors),
                to_output(
                    &unmodified_input[start..end],
                    vars,
                    options.color == Auto::True,
                    &colors
                ),
                if options.color == Auto::True {
                    "\x1b[0m"
                } else {
                    ""
                },
            );
            (0, HowGraphing::default(), true, false)
        } else {
            print!(
                "\x1b[G\n\x1b[J{}\x1b[{}A\x1b[K{}{}{}",
                list,
                n + 1,
                prompt(options, &colors),
                to_output(
                    &unmodified_input[start..end],
                    vars,
                    options.color == Auto::True,
                    &colors
                ),
                if options.color == Auto::True {
                    "\x1b[0m"
                } else {
                    ""
                }
            );
            (n, HowGraphing::default(), false, false)
        };
    }
    if unmodified_input.iter().collect::<String>() == "version" {
        print!(
            "\x1b[J\x1b[G\n{} {}\x1b[G\x1b[A\x1b[K{}{}{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            prompt(options, &colors),
            to_output(
                &unmodified_input[start..end],
                vars,
                options.color == Auto::True,
                &colors
            ),
            if options.color == Auto::True {
                "\x1b[0m"
            } else {
                ""
            },
        );
        return (1, HowGraphing::default(), false, false);
    }
    let mut var;
    let mut unparsed = unmodified_input;
    {
        if !unmodified_input.contains(&'#') {
            let split = unmodified_input.split(|c| c == &';');
            let count = split.clone().count();
            if count != 1 {
                unparsed = split.clone().next_back().unwrap();
                var = vars.to_vec();
                for (i, s) in split.enumerate() {
                    if i == count - 1 {
                        break;
                    }
                    silent_commands(
                        &mut options,
                        &s.iter()
                            .copied()
                            .filter(|&c| !c.is_ascii_whitespace())
                            .collect::<Vec<char>>(),
                    );
                    if s.contains(&'=') {
                        if let Err(s) = set_commands_or_vars(&mut colors, &mut options, &mut var, s)
                        {
                            handle_err(s, vars, unmodified_input, options, &colors, start, end);
                            return (1, HowGraphing::default(), false, false);
                        }
                    }
                }
                vars = &var;
            }
        }
        let mut tempinput = unparsed.iter().collect::<String>();
        if tempinput.starts_with("help ") {
            let message = help_for(tempinput.splitn(2, ' ').last().unwrap());
            let mut num = message.chars().filter(|c| c == &'\n').count();
            {
                let width = get_terminal_dimensions().0;
                let mut len = 0;
                for i in no_col(&message, options.color == Auto::True) {
                    len += 1;
                    if i == '\n' {
                        len = 0
                    } else if len > width {
                        len = 0;
                        num += 1;
                    }
                }
            }
            print!(
                "\x1b[G\x1b[J\n{}\x1b[G\x1b[{}A{}{}{}",
                message,
                num + 1,
                prompt(options, &colors),
                to_output(
                    &unmodified_input[start..end],
                    vars,
                    options.color == Auto::True,
                    &colors
                ),
                if options.color == Auto::True {
                    "\x1b[0m"
                } else {
                    ""
                },
            );
            return (num, HowGraphing::default(), false, false);
        } else if tempinput.ends_with('=') {
            let (width, height) = get_terminal_dimensions();
            let mut wrap = 0;
            tempinput.pop();
            let mut out = String::new();
            let split = tempinput.split('#');
            {
                if tempinput.contains(';') {
                    let mut options = options;
                    let mut colors = colors.clone();
                    let mut vars = vars.to_vec();
                    for input in split {
                        if !input.is_empty() {
                            let mut input = input;
                            let split = input.split(';');
                            let count = split.clone().count();
                            if count != 1 {
                                input = split.clone().next_back().unwrap();
                                for (i, s) in split.enumerate() {
                                    if i == count - 1 {
                                        break;
                                    }
                                    silent_commands(
                                        &mut options,
                                        &s.chars()
                                            .filter(|c| !c.is_ascii_whitespace())
                                            .collect::<Vec<char>>(),
                                    );
                                    if s.contains('=') {
                                        if let Err(s) = set_commands_or_vars(
                                            &mut colors,
                                            &mut options,
                                            &mut vars,
                                            &s.chars().collect::<Vec<char>>(),
                                        ) {
                                            handle_err(
                                                s,
                                                &vars,
                                                unmodified_input,
                                                options,
                                                &colors,
                                                start,
                                                end,
                                            );
                                            return (1, HowGraphing::default(), false, false);
                                        }
                                    }
                                }
                            }
                            let n = &equal_to(
                                options,
                                &colors,
                                &vars,
                                input,
                                &last.iter().collect::<String>(),
                            );
                            let len = no_col_len(n, options.color == Auto::True);
                            wrap += len.saturating_sub(1) / width + 1;
                            out += n;
                            out += "\x1b[G\n"
                        }
                    }
                } else {
                    for input in split {
                        if !input.is_empty() {
                            let n = &equal_to(
                                options,
                                &colors,
                                vars,
                                input,
                                &last.iter().collect::<String>(),
                            );
                            let len = no_col_len(n, options.color == Auto::True);
                            wrap += len.saturating_sub(1) / width + 1;
                            out += n;
                            out += "\x1b[G\n"
                        }
                    }
                }
            }
            out.pop();
            return if !out.is_empty() {
                if wrap > height.saturating_sub(1) {
                    if long_output {
                        print!(
                            "\x1b[G\n\x1b[J{}{}",
                            out,
                            if options.color == Auto::True {
                                "\x1b[0m"
                            } else {
                                ""
                            }
                        );
                        (wrap, HowGraphing::default(), false, false)
                    } else {
                        print!(
                            "\x1b[J\x1b[G\ntoo long, will print on enter\x1b[G\x1b[A\x1b[K{}{}{}",
                            prompt(options, &colors),
                            to_output(
                                &unmodified_input[start..end],
                                vars,
                                options.color == Auto::True,
                                &colors
                            ),
                            if options.color == Auto::True {
                                "\x1b[0m"
                            } else {
                                ""
                            },
                        );
                        (0, HowGraphing::default(), true, false)
                    }
                } else {
                    print!(
                        "\x1b[G\n\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                        out,
                        if wrap == 0 {
                            String::new()
                        } else {
                            format!("\x1b[{wrap}A")
                        },
                        prompt(options, &colors),
                        to_output(
                            &unmodified_input[start..end],
                            vars,
                            options.color == Auto::True,
                            &colors
                        ),
                        if options.color == Auto::True {
                            "\x1b[0m"
                        } else {
                            ""
                        }
                    );
                    (wrap, HowGraphing::default(), false, false)
                }
            } else {
                clear(unmodified_input, vars, start, end, options, &colors);
                (0, HowGraphing::default(), false, false)
            };
        }
    }
    let mut var = false;
    let mut input = match input_var(
        &insert_last(unparsed, last.iter().collect::<String>().as_str()),
        vars,
        &mut Vec::new(),
        &mut 0,
        options,
        false,
        0,
        Vec::new(),
        false,
        &mut Vec::new(),
        None,
        None,
    ) {
        Ok(f) => f,
        Err(s) => {
            handle_err(s, vars, unmodified_input, options, &colors, start, end);
            return (1, HowGraphing::default(), false, false);
        }
    };
    {
        if input.3 && !unmodified_input.contains(&'#') {
            let n = unparsed.iter().position(|c| c == &'=').unwrap_or(0) + 1;
            let mut inputs = unparsed[n..].iter().collect::<String>();
            let func = unparsed[..n - 1].to_vec();
            if matches!(
                func.iter().collect::<String>().as_str(),
                "recol"
                    | "imcol"
                    | "angle"
                    | "notation"
                    | "graph"
                    | "saveto"
                    | "textc"
                    | "promptc"
                    | "imagc"
                    | "scic"
                    | "unitsc"
                    | "bracketc"
                    | "default_units"
                    | "label"
                    | "point"
                    | "points"
                    | "line"
                    | "lines"
                    | "color"
                    | "colour"
            ) {
                print!(
                    "\x1b[G\n\x1b[J{}\x1b[A\x1b[G\x1b[K{}{}{}",
                    inputs,
                    prompt(options, &colors),
                    to_output(
                        &unmodified_input[start..end],
                        vars,
                        options.color == Auto::True,
                        &colors
                    ),
                    if options.color == Auto::True {
                        "\x1b[0m"
                    } else {
                        ""
                    }
                );
                return (1, input.2, false, true);
            } else {
                let mut func_vars: Vec<(isize, String)> = Vec::new();
                let mut def = false;
                if func.contains(&'(') {
                    def = true;
                    let mut func = func.clone();
                    func.drain(0..=func.iter().position(|c| c == &'(').unwrap());
                    func.pop();
                    for i in func.split(|c| c == &',') {
                        func_vars.push((-1, i.iter().collect()));
                    }
                } else if inputs.contains(',')
                    && matches!(
                        func.iter().collect::<String>().as_str(),
                        "base"
                            | "ticks"
                            | "xr"
                            | "yr"
                            | "zr"
                            | "vxr"
                            | "vyr"
                            | "vzr"
                            | "3d"
                            | "windowsize"
                            | "range"
                            | "vrange"
                    )
                {
                    let mut brackets = 0;
                    for c in inputs.chars() {
                        match c {
                            '(' => brackets += 1,
                            ')' => brackets -= 1,
                            ',' if brackets == 0 => {
                                inputs.insert(0, '{');
                                inputs.push('}');
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                if inputs.contains(':') {
                    let inp = inputs;
                    let mut split = inp.split(':').collect::<Vec<&str>>();
                    inputs = split.pop().unwrap().to_string();
                    for i in split {
                        if i.contains('=') {
                            let mut split = i.splitn(2, '=');
                            func_vars.push((-1, split.next().unwrap().to_string()));
                        }
                    }
                }
                let tinput = input_var(
                    &inputs,
                    vars,
                    &mut func_vars,
                    &mut 0,
                    options,
                    false,
                    0,
                    func,
                    false,
                    &mut Vec::new(),
                    None,
                    None,
                );
                if def {
                    let out = match tinput {
                        Ok(n) if !n.0.is_empty() => {
                            parsed_to_string(n.0, vars, n.1, &options, &colors)
                        }
                        Err(s) => {
                            handle_err(s, vars, unmodified_input, options, &colors, start, end);
                            return (1, HowGraphing::default(), false, false);
                        }
                        _ => {
                            handle_err(
                                "equal sign on right side of definition",
                                vars,
                                unmodified_input,
                                options,
                                &colors,
                                start,
                                end,
                            );
                            return (1, HowGraphing::default(), false, false);
                        }
                    };
                    if out.is_empty() {
                        clear(unmodified_input, vars, start, end, options, &colors);
                        return (0, HowGraphing::default(), false, true);
                    }
                    let (width, height) = get_terminal_dimensions();
                    let len = no_col_len(&out, options.color == Auto::True);
                    let wrap = len.saturating_sub(1) / width + 1;
                    return if len > width * height.saturating_sub(1) {
                        if long_output {
                            print!(
                                "\x1b[G\n\x1b[J{}{}",
                                out,
                                if options.color == Auto::True {
                                    "\x1b[0m"
                                } else {
                                    ""
                                }
                            );
                            (wrap, input.2, false, true)
                        } else {
                            print!(
                                "\x1b[J\x1b[G\ntoo long, will print on enter\x1b[G\x1b[A\x1b[K{}{}{}",
                                prompt(options, &colors),
                                to_output(
                                    &unmodified_input[start..end],
                                    vars,
                                    options.color == Auto::True,
                                    &colors
                                ),
                                if options.color == Auto::Auto {
                                    "\x1b[0m"
                                } else {
                                    ""
                                },
                            );
                            (0, input.2, true, true)
                        }
                    } else {
                        print!(
                            "\x1b[G\n\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                            out,
                            if wrap == 0 {
                                String::new()
                            } else {
                                format!("\x1b[{wrap}A")
                            },
                            prompt(options, &colors),
                            to_output(
                                &unmodified_input[start..end],
                                vars,
                                options.color == Auto::True,
                                &colors
                            ),
                            if options.color == Auto::True {
                                "\x1b[0m"
                            } else {
                                ""
                            }
                        );
                        (wrap, input.2, false, true)
                    };
                } else {
                    var = true;
                    input = match tinput {
                        Ok(f) if !f.0.is_empty() => f,
                        Err(s) => {
                            handle_err(s, vars, unmodified_input, options, &colors, start, end);
                            return (1, HowGraphing::default(), false, false);
                        }
                        _ => {
                            handle_err(
                                "equal sign on right side of definition",
                                vars,
                                unmodified_input,
                                options,
                                &colors,
                                start,
                                end,
                            );
                            return (1, HowGraphing::default(), false, false);
                        }
                    };
                }
            };
        }
    }
    if input.2.graph || unmodified_input.contains(&'#') {
        return if unmodified_input.contains(&'#') {
            input.2.graph = true;
            let mut out = String::new();
            {
                let split = unmodified_input.iter().collect::<String>();
                let split = split.split('#');
                if unmodified_input.contains(&';') {
                    let mut options = options;
                    let mut colors = colors.clone();
                    let mut vars = vars.to_vec();
                    for input in split {
                        if !input.is_empty() {
                            let mut input = input;
                            let split = input.split(';');
                            let count = split.clone().count();
                            if count != 1 {
                                input = split.clone().next_back().unwrap();
                                for (i, s) in split.enumerate() {
                                    if i == count - 1 {
                                        break;
                                    }
                                    silent_commands(
                                        &mut options,
                                        &s.chars()
                                            .filter(|c| !c.is_ascii_whitespace())
                                            .collect::<Vec<char>>(),
                                    );
                                    if s.contains('=') {
                                        if let Err(s) = set_commands_or_vars(
                                            &mut colors,
                                            &mut options,
                                            &mut vars,
                                            &s.chars().collect::<Vec<char>>(),
                                        ) {
                                            handle_err(
                                                s,
                                                &vars,
                                                unmodified_input,
                                                options,
                                                &colors,
                                                start,
                                                end,
                                            );
                                            return (1, HowGraphing::default(), false, false);
                                        }
                                    }
                                }
                            }
                            out += &equal_to(
                                options,
                                &colors,
                                &vars,
                                input,
                                &last.iter().collect::<String>(),
                            );
                            out += "\n"
                        }
                    }
                } else {
                    for input in split {
                        if !input.is_empty() {
                            out += &equal_to(
                                options,
                                &colors,
                                vars,
                                input,
                                &last.iter().collect::<String>(),
                            );
                            out += "\n"
                        }
                    }
                }
            }
            out.pop();
            let width = get_terminal_dimensions().0;
            let no_col = no_col(&out, options.color == Auto::True);
            let wrap: usize = no_col
                .split(|c| c == &'\n')
                .map(|i| {
                    if i.is_empty() {
                        1
                    } else {
                        (i.len().saturating_sub(1)) / width + 1
                    }
                })
                .sum();
            let out = out.replace('\n', "\x1b[G\n");
            if no_col.split(|&c| c == '\n').any(|s| s.len() > width) {
                print!(
                    "\x1b[J\x1b[G\ntoo long, append '=' to see parsed input\x1b[G\x1b[A\x1b[K{}{}{}",
                    prompt(options, &colors),
                    to_output(
                        &unmodified_input[start..end],
                        vars,
                        options.color == Auto::True,
                        &colors
                    ),
                    if options.color == Auto::Auto {
                        "\x1b[0m"
                    } else {
                        ""
                    },
                );
                (1, input.2, true, false)
            } else {
                print!(
                    "\x1b[G\n\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                    out,
                    if wrap == 0 {
                        String::new()
                    } else {
                        format!("\x1b[{wrap}A")
                    },
                    prompt(options, &colors),
                    to_output(
                        &unmodified_input[start..end],
                        vars,
                        options.color == Auto::True,
                        &colors
                    ),
                    if options.color == Auto::True {
                        "\x1b[0m"
                    } else {
                        ""
                    }
                );
                (wrap, input.2, false, false)
            }
        } else {
            let out = equal_to(
                options,
                &colors,
                vars,
                &unparsed.iter().collect::<String>(),
                &last.iter().collect::<String>(),
            );
            let width = get_terminal_dimensions().0;
            let len = no_col_len(&out, options.color == Auto::True);
            let wrap = len.saturating_sub(1) / width + 1;
            if len > width {
                print!(
                    "\x1b[J\x1b[G\ntoo long, append '=' to see parsed input\x1b[G\x1b[A\x1b[K{}{}{}",
                    prompt(options, &colors),
                    to_output(
                        &unmodified_input[start..end],
                        vars,
                        options.color == Auto::True,
                        &colors
                    ),
                    if options.color == Auto::Auto {
                        "\x1b[0m"
                    } else {
                        ""
                    },
                );
                (1, input.2, true, false)
            } else {
                print!(
                    "\x1b[G\n\x1b[J{}{}\x1b[G\x1b[K{}{}{}",
                    out,
                    if wrap == 0 {
                        String::new()
                    } else {
                        format!("\x1b[{wrap}A")
                    },
                    prompt(options, &colors),
                    to_output(
                        &unmodified_input[start..end],
                        vars,
                        options.color == Auto::True,
                        &colors
                    ),
                    if options.color == Auto::True {
                        "\x1b[0m"
                    } else {
                        ""
                    }
                );
                (wrap, input.2, false, false)
            }
        };
    }
    let num = match do_math(input.0, options, input.1) {
        Ok(n) => n,
        Err(s) => {
            return if s == " " {
                clear(unmodified_input, vars, start, end, options, &colors);
                (0, HowGraphing::default(), false, false)
            } else {
                handle_err(s, vars, unmodified_input, options, &colors, start, end);
                (1, HowGraphing::default(), false, false)
            };
        }
    };
    let mut frac = 0;
    let mut long = false;
    match num {
        Num(n) => {
            let n = custom_units(*n, options, &colors);
            let mut output = get_output(options, &colors, &n);
            let (mut frac_a, frac_b) = if options.frac.num {
                let n = n.number;
                let fa = fraction(n.real().clone(), options, &colors, 0);
                let fb = fraction(n.imag().clone(), options, &colors, 0);
                let sign = if !output.0.is_empty() && n.imag().is_sign_positive() {
                    "+"
                } else {
                    ""
                }
                .to_string();
                match (!fa.is_empty(), !fb.is_empty()) {
                    (true, true) => {
                        frac = 1;
                        (
                            if n.real().is_zero() && !n.imag().is_zero() {
                                String::new()
                            } else {
                                fa
                            },
                            if n.imag().is_zero() {
                                String::new()
                            } else {
                                sign + fb.as_str()
                                    + &if options.color == Auto::True {
                                        format!("{}i\x1b[0m", &colors.imag)
                                    } else {
                                        "i".to_string()
                                    }
                            },
                        )
                    }
                    (true, _) => {
                        frac = 1;
                        (
                            if n.real().is_zero() && !n.imag().is_zero() {
                                String::new()
                            } else {
                                fa
                            },
                            if n.imag().is_zero() {
                                String::new()
                            } else {
                                output.1.clone()
                                    + if options.color == Auto::True {
                                        "\x1b[0m"
                                    } else {
                                        ""
                                    }
                            },
                        )
                    }
                    (_, true) => {
                        frac = 1;
                        (
                            if n.real().is_zero() && !n.imag().is_zero() {
                                String::new()
                            } else {
                                output.0.clone()
                            },
                            if n.imag().is_zero() {
                                String::new()
                            } else {
                                sign + fb.as_str()
                                    + &if options.color == Auto::True {
                                        format!("{}i\x1b[0m", &colors.imag)
                                    } else {
                                        "i".to_string()
                                    }
                            },
                        )
                    }
                    _ => (String::new(), String::new()),
                }
            } else {
                (String::new(), String::new())
            };
            let (width, height) = get_terminal_dimensions();
            if let Some(st) = output.2 {
                output.0 += &st;
                frac_a += &st;
            }
            let len1 = no_col_len(&output.0, options.color == Auto::True);
            let len2 = no_col_len(&output.1, options.color == Auto::True);
            if (frac == 1 && !options.frac.num)
                || (frac_a.len() + frac_b.len()
                    - if options.color == Auto::True && !frac_b.is_empty() {
                        5
                    } else {
                        0
                    })
                    > width
            {
                frac = 0;
            }
            if len1 + len2 > width * height.saturating_sub(1) {
                if long_output {
                    let num = len1.div_ceil(width).saturating_sub(1)
                        + len2.saturating_sub(1).div_ceil(width).saturating_sub(1);
                    print!(
                        "\x1b[G\n\x1b[J{}\x1b[G{}{}{}{}",
                        if frac == 1 {
                            format!("{frac_a}{frac_b}\x1b[G\n")
                        } else {
                            String::new()
                        },
                        output.0,
                        if len1 != 0 && len2 != 0 {
                            "\x1b[G\n"
                        } else {
                            ""
                        },
                        &output.1.replace('+', ""),
                        if options.color == Auto::True {
                            "\x1b[0m"
                        } else {
                            ""
                        },
                    );
                    frac += num + if len1 != 0 && len2 != 0 { 1 } else { 0 };
                } else {
                    print!(
                        "\x1b[G\x1b[J{}\x1b[G\ntoo long, will print on enter{}\x1b[G\x1b[A{}{}{}",
                        if frac == 1 {
                            format!("\x1b[G\n{frac_a}{frac_b}")
                        } else {
                            String::new()
                        },
                        if frac == 1 { "\x1b[A" } else { "" },
                        prompt(options, &colors),
                        to_output(
                            &unmodified_input[start..end],
                            vars,
                            options.color == Auto::True,
                            &colors
                        ),
                        if options.color == Auto::True {
                            "\x1b[0m"
                        } else {
                            ""
                        },
                    );
                    long = true;
                }
            } else if len1 + len2 > width {
                let num = len1.div_ceil(width).saturating_sub(1)
                    + len2.saturating_sub(1).div_ceil(width).saturating_sub(1);
                let temp = (num + frac).saturating_sub(if len1 == 0 || len2 == 0 { 1 } else { 0 });
                print!(
                    "\x1b[G\x1b[J{}\x1b[G\n{}{}{}{}\x1b[A\x1b[G\x1b[A{}{}{}",
                    if frac == 1 {
                        format!("\x1b[G\n{frac_a}{frac_b}")
                    } else {
                        String::new()
                    },
                    output.0,
                    if len1 != 0 && len2 != 0 {
                        "\x1b[G\n"
                    } else {
                        ""
                    },
                    &output.1.replace('+', ""),
                    if temp == 0 {
                        String::new()
                    } else {
                        format!("\x1b[{temp}A")
                    },
                    prompt(options, &colors),
                    to_output(
                        &unmodified_input[start..end],
                        vars,
                        options.color == Auto::True,
                        &colors
                    ),
                    if options.color == Auto::True {
                        "\x1b[0m"
                    } else {
                        ""
                    },
                );
                frac += num + if len1 != 0 && len2 != 0 { 1 } else { 0 };
            } else {
                print!(
                    "\x1b[G{}{}\x1b[K{}\x1b[G\n{}{}{}\x1b[J\x1b[G{}{}\x1b[A\x1b[{}C{}",
                    prompt(options, &colors),
                    to_output(
                        &unmodified_input[start..end],
                        vars,
                        options.color == Auto::True,
                        &colors
                    ),
                    if frac == 1 {
                        format!("\x1b[G\n{frac_a}{frac_b}\x1b[K")
                    } else {
                        String::new()
                    },
                    output.0,
                    output.1,
                    if len1 == width || len2 == width {
                        " "
                    } else {
                        ""
                    },
                    if frac == 1 { "\x1b[A" } else { "" },
                    if len1 == width || len2 == width {
                        "\x1b[A"
                    } else {
                        ""
                    },
                    if options.prompt { 2 } else { 0 } + (end - start),
                    if options.color == Auto::True {
                        "\x1b[0m"
                    } else {
                        ""
                    }
                )
            }
        }
        Vector(mut v) => {
            if options.polar {
                v = to_polar(
                    v,
                    match options.angle {
                        AngleType::Degrees => 180 / Complex::with_val(options.prec, Pi),
                        AngleType::Radians => Complex::with_val(options.prec, 1),
                        AngleType::Gradians => 200 / Complex::with_val(options.prec, Pi),
                    },
                );
            }
            let mut output = if options.color == Auto::True {
                colors.brackets[0].clone()
            } else {
                String::new()
            } + if options.polar { "[" } else { "{" }
                + if options.color == Auto::True {
                    &colors.text
                } else {
                    ""
                };
            let mut frac_out = output.clone();
            let mut out;
            let mut frac_temp;
            let (width, height) = get_terminal_dimensions();
            frac = if options.frac.vec { 1 } else { 0 };
            for (k, i) in v.iter().enumerate() {
                let i = custom_units(i.clone(), options, &colors);
                out = get_output(options, &colors, &i);
                let i = &i.number;
                if frac == 1 {
                    frac_temp = fraction(i.real().clone(), options, &colors, 1);
                    frac_out += if !frac_temp.is_empty() {
                        &frac_temp
                    } else {
                        &out.0
                    };
                    frac_temp = fraction(i.imag().clone(), options, &colors, 1);
                    frac_out += &if !frac_temp.is_empty() {
                        format!(
                            "{}{}{}",
                            if !out.0.is_empty()
                                && !i.imag().is_zero()
                                && i.imag().is_sign_positive()
                            {
                                "+"
                            } else {
                                ""
                            },
                            frac_temp,
                            if !i.imag().is_zero() {
                                if options.color == Auto::True {
                                    format!("{}i", &colors.imag)
                                } else {
                                    "i".to_string()
                                }
                            } else {
                                String::new()
                            }
                        )
                    } else {
                        out.clone().1
                    };
                    if no_col_len(&frac_out, options.color == Auto::True) > width {
                        frac = 0
                    }
                }
                output += &out.0;
                output += &out.1;
                if let Some(st) = out.2 {
                    output += &st;
                    if frac == 1 {
                        frac_out += &st;
                    }
                }
                if options.color == Auto::True {
                    output += &colors.text;
                    if frac == 1 {
                        frac_out += &colors.text;
                    }
                }
                if k == v.len().saturating_sub(1) {
                    if options.color == Auto::True {
                        output += &colors.brackets[0].clone()
                    }
                    output += if options.polar { "]" } else { "}" };
                    if options.color == Auto::True {
                        output += &colors.text
                    }
                    if frac == 1 {
                        if options.color == Auto::True {
                            frac_out += &colors.brackets[0].clone()
                        }
                        frac_out += if options.polar { "]" } else { "}" };
                        if options.color == Auto::True {
                            frac_out += &colors.text
                        }
                    }
                } else {
                    output += ",";
                    if frac == 1 {
                        frac_out += ",";
                    }
                }
                if !long_output
                    && no_col_len(&output, options.color == Auto::True)
                        > width * height.saturating_sub(1)
                {
                    break;
                }
            }
            let length = no_col_len(&output, options.color == Auto::True);
            if frac_out == output
                || !options.frac.vec
                || no_col_len(&frac_out, options.color == Auto::True) > width
                || length > width
            {
                frac = 0;
            }
            if length > width * height.saturating_sub(1) {
                if long_output {
                    let num = length.saturating_sub(1) / width;
                    print!(
                        "\x1b[G\n\x1b[J{}\x1b[G{}{}",
                        if frac == 1 && options.frac.vec {
                            format!("{frac_out}\x1b[G\n")
                        } else {
                            String::new()
                        },
                        output,
                        if options.color == Auto::True {
                            "\x1b[0m"
                        } else {
                            ""
                        }
                    );
                    frac += num;
                } else {
                    print!(
                        "\x1b[G\x1b[J\ntoo long, will print on enter\x1b[G\x1b[A{}{}{}",
                        prompt(options, &colors),
                        to_output(
                            &unmodified_input[start..end],
                            vars,
                            options.color == Auto::True,
                            &colors
                        ),
                        if options.color == Auto::True {
                            "\x1b[0m"
                        } else {
                            ""
                        },
                    );
                    long = true;
                    frac = 0;
                }
            } else {
                let num = length.saturating_sub(1) / width;
                print!(
                    "\x1b[G{}{}\x1b[K{}\x1b[G\n{}\x1b[J{}{}\x1b[G\x1b[A\x1b[{}C{}",
                    prompt(options, &colors),
                    to_output(
                        &unmodified_input[start..end],
                        vars,
                        options.color == Auto::True,
                        &colors
                    ),
                    if frac == 1 {
                        format!("\x1b[G\n{frac_out}\x1b[K")
                    } else {
                        String::new()
                    },
                    output,
                    if num == 0 {
                        String::new()
                    } else {
                        format!("\x1b[{num}A")
                    },
                    if frac == 1 { "\x1b[A" } else { "" },
                    if options.prompt { 2 } else { 0 } + (end - start),
                    if options.color == Auto::True {
                        "\x1b[0m"
                    } else {
                        ""
                    }
                );
                frac += num;
            }
        }
        Matrix(v) => {
            let mut output = if options.color == Auto::True {
                colors.brackets[0].clone()
            } else {
                String::new()
            } + if options.multi { "" } else { "{" }
                + if options.color == Auto::True {
                    &colors.text
                } else {
                    ""
                };
            let mut frac_out = output.clone();
            let mut out;
            let mut frac_temp;
            let mut num = 0;
            let (width, height) = get_terminal_dimensions();
            frac = if options.frac.mat { 1 } else { 0 };
            for (l, j) in v.iter().enumerate() {
                if !options.multi {
                    if options.color == Auto::True {
                        output += &(colors.brackets[1 % colors.brackets.len()].clone()
                            + "{"
                            + &colors.text);
                        if frac == 1 {
                            frac_out += &(colors.brackets[1 % colors.brackets.len()].clone()
                                + "{"
                                + &colors.text);
                        }
                    } else {
                        output += "{";
                        if frac == 1 {
                            frac_out += "{";
                        }
                    }
                }
                for (k, i) in j.iter().enumerate() {
                    let i = custom_units(i.clone(), options, &colors);
                    out = get_output(options, &colors, &i);
                    let i = &i.number;
                    if frac == 1 {
                        frac_temp = fraction(i.real().clone(), options, &colors, 2);
                        frac_out += if !frac_temp.is_empty() {
                            &frac_temp
                        } else {
                            &out.0
                        };
                        frac_temp = fraction(i.imag().clone(), options, &colors, 2);
                        frac_out += &if !frac_temp.is_empty() {
                            format!(
                                "{}{}{}",
                                if !out.0.is_empty()
                                    && !i.imag().is_zero()
                                    && i.imag().is_sign_positive()
                                {
                                    "+"
                                } else {
                                    ""
                                },
                                frac_temp,
                                if !i.imag().is_zero() {
                                    if options.color == Auto::True {
                                        format!("{}i", &colors.imag)
                                    } else {
                                        "i".to_string()
                                    }
                                } else {
                                    String::new()
                                }
                            )
                        } else {
                            out.clone().1
                        };
                        if no_col_len(&frac_out, options.color == Auto::True) > width {
                            frac = 0
                        }
                    }
                    output += &out.0;
                    output += &out.1;
                    if let Some(st) = out.2 {
                        output += &st;
                        if frac == 1 {
                            frac_out += &st;
                        }
                    }
                    if options.color == Auto::True {
                        output += &colors.text;
                        if frac == 1 {
                            frac_out += &colors.text;
                        }
                    }
                    if k == j.len().saturating_sub(1) {
                        if !options.multi {
                            if options.color == Auto::True {
                                output += &(colors.brackets[1 % colors.brackets.len()].clone()
                                    + "}"
                                    + &colors.text);
                                if frac == 1 {
                                    frac_out += &(colors.brackets[1 % colors.brackets.len()]
                                        .clone()
                                        + "}"
                                        + &colors.text);
                                }
                            } else {
                                output += "}";
                                if frac == 1 {
                                    frac_out += "}";
                                }
                            }
                        }
                    } else if options.tabbed {
                        output += "\t";
                        if frac == 1 {
                            frac_out += "\t";
                        }
                    } else {
                        output += ",";
                        if frac == 1 {
                            frac_out += ",";
                        }
                    }
                }
                if l != v.len().saturating_sub(1) {
                    if options.multi {
                        num += 1;
                        output += "\x1b[K\x1b[G\n";
                        if frac == 1 {
                            frac_out += "\x1b[K\x1b[G\n";
                        }
                    } else {
                        output += ",";
                        if frac == 1 {
                            frac_out += ",";
                        }
                    }
                }
            }
            if !options.multi {
                if options.color == Auto::True {
                    output += &(colors.brackets[0].clone() + "}" + &colors.text);
                    if frac == 1 {
                        frac_out += &(colors.brackets[0].clone() + "}" + &colors.text);
                    }
                } else {
                    output += "}";
                    if frac == 1 {
                        frac_out += "}";
                    }
                }
            }
            let length = no_col_len(&output, options.color == Auto::True).saturating_sub(1);
            if frac_out == output && frac == 1 {
                frac = 0;
            }
            if !options.multi {
                num += length.saturating_sub(1) / width;
                if !options.frac.mat
                    || no_col_len(&frac_out, options.color == Auto::True) > width
                    || length > width
                {
                    frac = 0;
                }
            } else {
                let mut len = 0;
                for i in no_col(&frac_out, options.color == Auto::True) {
                    len += 1;
                    if i == '\n' {
                        len = 0
                    } else if len > width {
                        frac = 0;
                        break;
                    }
                }
                len = 0;
                for i in no_col(&output, options.color == Auto::True) {
                    len += 1;
                    if i == '\n' {
                        len = 0
                    } else if len > width {
                        len = 0;
                        num += 1;
                    }
                }
                frac_out += "\x1b[K\x1b[G\n\x1b[K";
            }
            if length > width * height.saturating_sub(1) || num > height.saturating_sub(2) {
                if long_output {
                    print!(
                        "\x1b[G\n\x1b[J{}\x1b[G{}{}",
                        if frac == 1 && options.frac.mat {
                            num *= 2;
                            if options.multi {
                                num += 1;
                            }
                            format!("{frac_out}\x1b[G\n")
                        } else {
                            String::new()
                        },
                        output,
                        if options.color == Auto::True {
                            "\x1b[0m"
                        } else {
                            ""
                        }
                    );
                    frac += num;
                } else {
                    print!(
                        "\x1b[G\x1b[J\ntoo long, will print on enter\x1b[G\x1b[A{}{}{}",
                        prompt(options, &colors),
                        to_output(
                            &unmodified_input[start..end],
                            vars,
                            options.color == Auto::True,
                            &colors
                        ),
                        if options.color == Auto::True {
                            "\x1b[0m"
                        } else {
                            ""
                        },
                    );
                    long = true;
                    frac = 0;
                }
            } else {
                if !options.frac.mat {
                    frac = 0;
                }
                print!(
                    "\x1b[G{}{}\x1b[K{}\x1b[G\n{}\x1b[J{}\x1b[G\x1b[A\x1b[{}C{}",
                    prompt(options, &colors),
                    to_output(
                        &unmodified_input[start..end],
                        vars,
                        options.color == Auto::True,
                        &colors
                    ),
                    if frac == 1 {
                        num *= 2;
                        if options.multi {
                            num += 1;
                        }
                        format!("\x1b[G\n{frac_out}\x1b[K")
                    } else {
                        String::new()
                    },
                    output,
                    if num + frac == 0 {
                        String::new()
                    } else {
                        format!("\x1b[{}A", num + frac)
                    },
                    if options.prompt { 2 } else { 0 } + (end - start),
                    if options.color == Auto::True {
                        "\x1b[0m"
                    } else {
                        ""
                    }
                );
                frac += num;
            }
        }
        _ => handle_err(
            "str err",
            vars,
            unmodified_input,
            options,
            &colors,
            start,
            end,
        ),
    }
    (frac + 1, HowGraphing::default(), long, var)
}
pub fn print_answer(num: NumStr, options: Options, colors: &Colors) {
    match num {
        Num(n) => {
            let n = custom_units(*n, options, colors);
            let a = get_output(options, colors, &n);
            print!(
                "{}{}{}{}",
                a.0,
                a.2.unwrap_or_default(),
                a.1,
                if options.color == Auto::True {
                    "\x1b[0m"
                } else {
                    ""
                }
            );
        }
        Vector(mut v) => {
            if options.polar {
                v = to_polar(
                    v,
                    match options.angle {
                        AngleType::Degrees => 180 / Complex::with_val(options.prec, Pi),
                        AngleType::Radians => Complex::with_val(options.prec, 1),
                        AngleType::Gradians => 200 / Complex::with_val(options.prec, Pi),
                    },
                );
            }
            let mut output = if options.color == Auto::True {
                colors.brackets[0].clone()
            } else {
                String::new()
            } + if options.polar { "[" } else { "{" }
                + if options.color == Auto::True {
                    &colors.text
                } else {
                    ""
                };
            let mut out;
            for (k, i) in v.iter().enumerate() {
                let i = custom_units(i.clone(), options, colors);
                out = get_output(options, colors, &i);
                output += out.0.as_str();
                output += out.1.as_str();
                output += &out.2.unwrap_or_default();
                if options.color == Auto::True {
                    output += &colors.text;
                }
                if k == v.len().saturating_sub(1) {
                    if options.color == Auto::True {
                        output += &colors.brackets[0].clone()
                    }
                    output += if options.polar { "]" } else { "}" };
                    if options.color == Auto::True {
                        output += &colors.text
                    }
                } else {
                    output += ",";
                }
            }
            print!(
                "{}{}",
                output,
                if options.color == Auto::True {
                    "\x1b[0m"
                } else {
                    ""
                }
            );
        }
        Matrix(v) => {
            let mut output = if options.multi {
                String::new()
            } else if options.color == Auto::True {
                colors.brackets[0].clone() + "{" + &colors.text
            } else {
                "{".to_string()
            };
            let mut out;
            for (l, j) in v.iter().enumerate() {
                if !options.multi {
                    if options.color == Auto::True {
                        output += &format!(
                            "{}{{{}",
                            colors.brackets[1 % colors.brackets.len()].clone(),
                            colors.text
                        );
                    } else {
                        output += "{"
                    };
                }
                for (k, i) in j.iter().enumerate() {
                    let i = custom_units(i.clone(), options, colors);
                    out = get_output(options, colors, &i);
                    output += out.0.as_str();
                    output += out.1.as_str();
                    output += &out.2.unwrap_or_default();
                    if options.color == Auto::True {
                        output += &colors.text;
                    }
                    if k == j.len().saturating_sub(1) {
                        if !options.multi {
                            if options.color == Auto::True {
                                output += &format!(
                                    "{}}}{}",
                                    colors.brackets[1 % colors.brackets.len()].clone(),
                                    colors.text
                                );
                            } else {
                                output += "}"
                            };
                        }
                    } else if options.tabbed {
                        output += "\t";
                    } else {
                        output += ",";
                    }
                }
                if l != v.len().saturating_sub(1) {
                    if options.multi {
                        output += "\n";
                    } else {
                        output += ",";
                    }
                }
            }
            if !options.multi {
                if options.color == Auto::True {
                    output += &format!("{}}}{}", colors.brackets[0].clone(), colors.text);
                } else {
                    output += "}"
                };
            }
            print!(
                "{}{}",
                output,
                if options.color == Auto::True {
                    "\x1b[0m"
                } else {
                    ""
                }
            );
        }
        _ => {}
    }
}
pub fn custom_units(mut number: Number, options: Options, colors: &Colors) -> Number {
    if !colors.default_units.is_empty() {
        let mut meter = Complex::with_val(options.prec, 1);
        let mut second = Complex::with_val(options.prec, 1);
        let mut kilogram = Complex::with_val(options.prec, 1);
        let mut ampere = Complex::with_val(options.prec, 1);
        let mut mole = Complex::with_val(options.prec, 1);
        let mut candela = Complex::with_val(options.prec, 1);
        let mut byte = Complex::with_val(options.prec, 1);
        let mut usd = Complex::with_val(options.prec, 1);
        let mut unit = Complex::with_val(options.prec, 1);
        let m = Units {
            meter: 1.0,
            ..Units::default()
        };
        let s = Units {
            second: 1.0,
            ..Units::default()
        };
        let kg = Units {
            kilogram: 1.0,
            ..Units::default()
        };
        let a = Units {
            ampere: 1.0,
            ..Units::default()
        };
        let mol = Units {
            mole: 1.0,
            ..Units::default()
        };
        let cd = Units {
            candela: 1.0,
            ..Units::default()
        };
        let b = Units {
            byte: 1.0,
            ..Units::default()
        };
        let us = Units {
            usd: 1.0,
            ..Units::default()
        };
        let un = Units {
            unit: 1.0,
            ..Units::default()
        };
        for du in &colors.default_units {
            let u = du.1.units.unwrap_or_default();
            if u == m {
                meter = du.1.number.clone()
            } else if u == s {
                second = du.1.number.clone()
            } else if u == kg {
                kilogram = du.1.number.clone()
            } else if u == a {
                ampere = du.1.number.clone()
            } else if u == mol {
                mole = du.1.number.clone()
            } else if u == cd {
                candela = du.1.number.clone()
            } else if u == b {
                byte = du.1.number.clone()
            } else if u == us {
                usd = du.1.number.clone()
            } else if u == un {
                unit = du.1.number.clone()
            }
        }
        if let Some(u) = number.units {
            number.number /= meter.pow(u.meter);
            number.number /= second.pow(u.second);
            number.number /= kilogram.pow(u.kilogram);
            number.number /= ampere.pow(u.ampere);
            number.number /= mole.pow(u.mole);
            number.number /= candela.pow(u.candela);
            number.number /= byte.pow(u.byte);
            number.number /= usd.pow(u.usd);
            number.number /= unit.pow(u.unit);
        }
    }
    number
}
pub fn get_output(
    options: Options,
    colors: &Colors,
    number: &Number,
) -> (String, String, Option<String>) {
    let num = number.number.clone();
    let units = number.units;
    if options.notation != Normal {
        if options.base.1 != 10 {
            let sign = if num.imag().is_sign_positive() && !num.real().is_zero() {
                "+"
            } else {
                ""
            }
            .to_string();
            (
                if !num.real().is_zero() {
                    let n = num
                        .real()
                        .to_string_radix(options.base.1, Some(options.decimal_places));
                    if n.chars().filter(|c| *c == '@').count() == 1 {
                        n.replace('@', &notate(options, colors))
                            + if options.color == Auto::True {
                                "\x1b[0m"
                            } else {
                                ""
                            }
                    } else if n.contains('e') && options.base.1 < 16 {
                        n.replace("e0", "").replace('e', &notate(options, colors))
                            + if options.color == Auto::True {
                                "\x1b[0m"
                            } else {
                                ""
                            }
                    } else {
                        n.trim_end_matches('0').trim_end_matches('.').to_string()
                    }
                } else if num.imag().is_zero() {
                    "0".to_string()
                } else {
                    String::new()
                },
                if !num.imag().is_zero() {
                    let n = num
                        .imag()
                        .to_string_radix(options.base.1, Some(options.decimal_places));
                    sign + &if n.chars().filter(|c| *c == '@').count() == 1 {
                        n.replace('@', &notate(options, colors))
                            + if options.color == Auto::True {
                                "\x1b[0m"
                            } else {
                                ""
                            }
                    } else if n.contains('e') && options.base.1 < 16 {
                        n.replace("e0", "").replace('e', &notate(options, colors))
                            + if options.color == Auto::True {
                                "\x1b[0m"
                            } else {
                                ""
                            }
                    } else {
                        n.trim_end_matches('0').trim_end_matches('.').to_string()
                    } + &if options.color == Auto::True {
                        format!("{}i", &colors.imag)
                    } else {
                        "i".to_string()
                    }
                } else {
                    String::new()
                },
                if options.units && num.imag().is_zero() {
                    units.map(|units| {
                        format!(
                            "{}{}{}",
                            if options.color == Auto::True {
                                &colors.units
                            } else {
                                ""
                            },
                            units.to_string(options, colors),
                            if options.color == Auto::True {
                                "\x1b[0m"
                            } else {
                                ""
                            }
                        )
                    })
                } else {
                    None
                },
            )
        } else {
            let sign = if num.imag().is_sign_positive() && !num.real().is_zero() {
                "+"
            } else {
                ""
            }
            .to_string();
            (
                if num.real().is_zero() && !num.imag().is_zero() {
                    String::new()
                } else {
                    if options.comma {
                        add_commas(&remove_trailing_zeros(
                            &format!("{:e}", num.real()),
                            options,
                        ))
                    } else {
                        remove_trailing_zeros(&format!("{:e}", num.real()), options)
                    }
                    .replace("e0", "")
                    .replace('e', &notate(options, colors))
                        + if options.color == Auto::True {
                            "\x1b[0m"
                        } else {
                            ""
                        }
                },
                if num.imag().is_zero() {
                    String::new()
                } else if num.imag() == &1 {
                    sign + &if options.color == Auto::True {
                        format!("{}i", &colors.imag)
                    } else {
                        "i".to_string()
                    }
                } else if num.imag() == &-1 {
                    sign + &if options.color == Auto::True {
                        format!("-{}i", &colors.imag)
                    } else {
                        "i".to_string()
                    }
                } else {
                    if options.comma {
                        add_commas(
                            &(sign + &remove_trailing_zeros(&format!("{:e}", num.imag()), options)),
                        )
                    } else {
                        sign + &remove_trailing_zeros(&format!("{:e}", num.imag()), options)
                    }
                    .replace("e0", "")
                    .replace('e', &notate(options, colors))
                        + &if options.color == Auto::True {
                            format!("{}i", &colors.imag)
                        } else {
                            "i".to_string()
                        }
                },
                if options.units && num.imag().is_zero() {
                    units.map(|units| {
                        format!(
                            "{}{}{}",
                            if options.color == Auto::True {
                                &colors.units
                            } else {
                                ""
                            },
                            units.to_string(options, colors),
                            if options.color == Auto::True {
                                "\x1b[0m"
                            } else {
                                ""
                            }
                        )
                    })
                } else {
                    None
                },
            )
        }
    } else {
        let mut re = if options.comma {
            add_commas(&to_string(num.real(), options, false, options.base.1))
        } else {
            to_string(num.real(), options, false, options.base.1)
        };
        let mut im = if options.comma {
            add_commas(&to_string(num.imag(), options, true, options.base.1))
        } else {
            to_string(num.imag(), options, true, options.base.1)
        };
        if re == "-0" {
            re.remove(0);
        }
        if im == "-0" {
            im.remove(0);
        }
        let sign = if num.imag().is_sign_positive() && re != "0" {
            "+"
        } else {
            ""
        }
        .to_string();
        (
            if re == "0" && im != "0" {
                String::new()
            } else {
                re
            },
            if im == "0" {
                String::new()
            } else if im == "1" {
                sign + &if options.color == Auto::True {
                    format!("{}i", &colors.imag)
                } else {
                    "i".to_string()
                }
            } else if im == "-1" {
                sign + &if options.color == Auto::True {
                    format!("-{}i", &colors.imag)
                } else {
                    "-i".to_string()
                }
            } else {
                sign + &im
                    + &if options.color == Auto::True {
                        format!("{}i", &colors.imag)
                    } else {
                        "i".to_string()
                    }
            },
            if options.units && im == "0" {
                units.map(|units| {
                    format!(
                        "{}{}{}",
                        if options.color == Auto::True {
                            &colors.units
                        } else {
                            ""
                        },
                        units.to_string(options, colors),
                        if options.color == Auto::True {
                            "\x1b[0m"
                        } else {
                            ""
                        }
                    )
                })
            } else {
                None
            },
        )
    }
}
fn to_string(num: &Float, options: Options, imag: bool, radix: i32) -> String {
    let (neg, mut str, exp) = num.to_sign_string_exp(radix, None);
    let mut neg = if neg { "-" } else { "" };
    if exp.is_none() {
        return format!(
            "{}{}{}",
            neg,
            str,
            if options.keep_zeros && options.decimal_places < usize::MAX - 2 {
                format!(".{}", "0".repeat(options.decimal_places))
            } else {
                String::new()
            }
        );
    }
    let exp = exp.unwrap();
    let width = get_terminal_dimensions().0;
    let decimals = if options.decimal_places == usize::MAX - 1 && (width as i32) > (2i32 + exp) {
        (width as i32
            - match exp.cmp(&0) {
                Ordering::Equal => 2i32,
                Ordering::Less => 3i32,
                Ordering::Greater => 1i32 + exp,
            }
            - if imag { 1 } else { 0 }
            - if !neg.is_empty() { 1 } else { 0 }) as usize
    } else {
        options.decimal_places
    };
    if str.len() as i32 == exp {
        return format!("{neg}{str}");
    }
    if exp > str.len() as i32 {
        str.push_str(&"0".repeat(exp as usize - str.len()));
    }
    let mut zeros = String::new();
    if -exp as i128 > decimals as i128 {
        return format!(
            "{}0{}",
            neg,
            if options.keep_zeros && options.decimal_places < usize::MAX - 2 {
                format!(".{}", "0".repeat(options.decimal_places))
            } else {
                String::new()
            }
        );
    }
    if exp < 0 {
        zeros = "0".repeat(-exp as usize);
        str.insert_str(0, &zeros);
        str.insert(1, '.');
    } else {
        str.insert(exp as usize, '.');
    }
    let mut split = str.split('.');
    let mut l = split.next().unwrap().to_string();
    let mut r = split.next().unwrap().to_string();
    if r.is_empty() {
        return format!("{neg}{l}");
    }
    if r.len() > decimals {
        if !zeros.is_empty() {
            r.insert(decimals.saturating_sub(1), '.');
        } else {
            r.insert(decimals, '.');
        }
    }
    let mut d = Float::parse_radix(&r, radix)
        .unwrap()
        .complete(num.prec())
        .to_integer()
        .unwrap();
    if exp > 0 {
        zeros = "0".repeat(r.to_string().len() - r.to_string().trim_start_matches('0').len());
        if d.to_string()
            == (radix as f64)
                .powi((decimals as i32).saturating_sub(1))
                .to_string()
        {
            zeros.pop();
        }
    }
    if radix == 10
        && d.to_string().trim_end_matches('0') == "1"
        && d.to_string().len() + zeros.len() > decimals
        && r.trim_start_matches('0')
            .trim_start_matches('.')
            .starts_with('9')
    {
        if zeros.is_empty() {
            let t: Float = Float::parse_radix(if l.is_empty() { "0" } else { &l }, radix)
                .unwrap()
                .complete(num.prec())
                + 1;
            l = t.to_integer().unwrap_or_default().to_string();
            d = Integer::new();
        } else {
            zeros.pop();
        }
    }
    if d.to_string() == "0" && (l.is_empty() || l == "0") {
        neg = ""
    }
    if zeros.len() == decimals {
        zeros.pop();
    }
    if decimals == 0 {
        if zeros.is_empty()
            && d.to_string_radix(radix)
                .chars()
                .next()
                .unwrap()
                .to_digit(radix as u32)
                .unwrap()
                == 1
        {
            format!(
                "{}{}",
                neg,
                Integer::from_str_radix(&l, radix).unwrap_or_default() + 1
            )
        } else {
            format!("{}{}", neg, if l.is_empty() { "0" } else { &l })
        }
    } else if options.keep_zeros {
        format!(
            "{}{}.{}{}",
            neg,
            if l.is_empty() { "0" } else { &l },
            zeros,
            d.to_string_radix(radix)
        )
    } else {
        format!(
            "{}{}.{}{}",
            neg,
            if l.is_empty() { "0" } else { &l },
            zeros,
            d.to_string_radix(radix)
        )
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
    }
}
fn add_commas(input: &str) -> String {
    let mut split = input.split('.');
    let mut result = String::new();
    let mut count = 0;
    let mut exp = false;
    for c in split.next().unwrap().chars().rev() {
        if c == '-' {
            count -= 1;
        }
        if c == 'e' {
            exp = true;
        }
        if count == 3 && !exp {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }
    if split.clone().count() == 1 {
        let mut result = result.chars().rev().collect::<String>();
        result.push('.');
        count = 0;
        for c in split.next().unwrap().chars() {
            if c == '-' {
                count -= 1;
            }
            if c == 'e' {
                exp = true;
            }
            if count == 3 && !exp {
                result.push(',');
                count = 0;
            }
            result.push(c);
            count += 1;
        }
        return result;
    }
    result.chars().rev().collect::<String>()
}
fn remove_trailing_zeros(input: &str, options: Options) -> String {
    let pos = match input.find('e') {
        Some(n) => n,
        None => {
            let s = if options.keep_zeros {
                input.to_string()
            } else {
                input
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            };
            return if s.is_empty() || s == "-" || s == "-0" || s == "0" {
                format!(
                    "{}0{}",
                    if s == "-" { "-" } else { "" },
                    if options.keep_zeros && options.decimal_places < usize::MAX - 2 {
                        format!(".{}", "0".repeat(options.decimal_places - 1))
                    } else {
                        String::new()
                    }
                )
            } else {
                s
            };
        }
    };
    let dec = if options.decimal_places == usize::MAX - 1 {
        get_terminal_dimensions().0
            - (if &input[pos..] == "e0" {
                2
            } else {
                input.len() - pos + 2
            })
            - if input.starts_with('-') { 1 } else { 0 }
    } else if options.decimal_places == 0 {
        1
    } else {
        options.decimal_places
    };
    if dec > pos {
        if options.keep_zeros {
            input.to_string()
        } else {
            input[..pos]
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
                + &input[pos..]
        }
    } else {
        let mut sign = String::new();
        let mut num = if input.starts_with('-') {
            sign = "-".to_string();
            input[1..pos].to_string()
        } else {
            input[0..pos].to_string()
        };
        num.remove(1);
        if dec >= num.len() {
            if options.keep_zeros {
                input.to_string()
            } else {
                input[..pos]
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
                    + &input[pos..]
            }
        } else {
            num.insert(dec, '.');
            num = Float::parse(num)
                .unwrap()
                .complete(options.prec)
                .to_integer()
                .unwrap()
                .to_string();
            num.insert(1, '.');
            sign + if options.keep_zeros {
                &num
            } else {
                num.trim_end_matches('0').trim_end_matches('.')
            } + "e"
                + &(input[pos + 1..].parse::<isize>().unwrap()
                    + if num.len().saturating_sub(1) > dec {
                        1
                    } else {
                        0
                    })
                .to_string()
        }
    }
}
fn notate(options: Options, colors: &Colors) -> String {
    format!(
        "{}{}",
        if options.color == Auto::True {
            colors.sci.to_string()
        } else {
            String::new()
        },
        match options.notation {
            Notation::SmallEngineering => "e",
            Notation::LargeEngineering => "E",
            Notation::Scientific => "*10^",
            Notation::Normal => "",
        },
    )
}
