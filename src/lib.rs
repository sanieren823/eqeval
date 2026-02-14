use std::str;
use std::vec;
use std::collections::HashMap;
use finum::finum::*;
use finum::operations::math::*;
use finum::operations::arithm::*;

// "23 - 7 * 3^4 / 4(17 - 5 * 3)"
// "4 * 3 % 9 + 13"


// general order of operations:
// 1. parenthesis
// 2. functions (sin, abs)
// 3. exponentiation/logarithm
// 4. factorials/termials
// 5. modulu
// 6. multiplication/division
// 7. addition/subtraction


// @ for logarithmic calculations??
// "3^4@8.9^3"
// "sin()"



// why strings?
// floats can be impresice, hence this crate uses decimals 
// to avoid carring on the floating point errors strings are never/should never be converted to floats 
// by converting the strings to decimals directly we avoid any kind of deviation from the actual number

#[derive(Debug, Clone)]
struct Atom {
    left: Box<Particle>,
    right: Box<Particle>,
    op: Operation,
}

#[derive(Debug, Clone)]
struct Action {
    x: Box<Particle>,
    fnc: String,
}

#[derive(Debug, Clone)]
// all calculations should be done with fractions (if at least one division/fraction is in the equation)
struct Fraction {
    numerator: Box<Particle>,
    denominator: Box<Particle>,  
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub fnc: fn(Vec<FiLong>) -> FiLong,
    pub special: u8, // 0: default, 1: trig_receiving (like sin, cos, tan), 2: trig_emitting (like the inverses (arcsin, arccos, arctan))
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub val: FiLong,
}

#[derive(Debug, Clone)]
enum Input {
    Fnc(Function),
    Var(Variable),
}

#[derive(Debug, Clone)]
enum Particle {
    Num(Box<Number>),
    Fnc(Box<Action>),
    Res(Box<Atom>),
}

#[derive(Debug, Clone)]
enum Operation {
    Addition,
    Subtraction, // do we even need subtraction?
    Multiplication,
    Division, // and division?
    Modulu,
    Exponentiation,
    Logarithm

}

#[derive(Debug, Clone)]
enum Number {
    Frc(Box<Fraction>),
    Num(FiLong),
    Var(usize),
}

// enum Function {
//     Sine,
//     Cosine,
//     Tangent,
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Type {
    Number,
    Letter, 
    Operation,
    Factorial,
    Termial,
    BrOpen,
    BrClosed,
    Variable,
    Undefined,
}

#[derive(Debug, Clone)]
enum StackingVec {
    String(String),
    Vector(Box<Vec<StackingVec>>),
}

#[macro_export]
macro_rules! eval {
    ($equation: expr) => {
        eval($equation, false, Vec::new(), Vec::new())
    };
    ($equation: expr, true) => {
        eval($equation, true, Vec::new(), Vec::new())
    };
    ($equation: expr, false) => {
        eval($equation, false, Vec::new(), Vec::new())
    };
    ($equation: expr, vars: $vars: expr) => {
        eval($equation, false, $vars, Vec::new())
    };
    ($equation: expr, func: $func: expr) => {
        eval($equation, false, Vec::new(), $func)
    };
    ($equation: expr, true, vars: $vars: expr) => {
        eval($equation, true, $vars, Vec::new())
    };
    ($equation: expr, false, vars: $vars: expr) => {
        eval($equation, false, $vars, Vec::new())
    };
    ($equation: expr, true, func: $func: expr) => {
        eval($equation, true, Vec::new(), $func)
    };
    ($equation: expr, false, func: $func: expr) => {
        eval($equation, false, Vec::new(), $func)
    };
    ($equation: expr, vars: $vars: expr, func: $func: expr) => {
        eval($equation, false, $vars, $func)
    };
    ($equation: expr, $deg: expr, $vars: expr, $func: expr) => {
        eval($equation, $deg, $vars, $func)
    };
    ($equation: expr, $deg: expr, vars: $vars: expr, func: $func: expr) => {
        eval($equation, $deg, $vars, $func)
    };
    ($equation: expr, deg: $deg: expr, vars: $vars: expr, func: $func: expr) => {
        eval($equation, $deg, $vars, $func)
    };
    
}

fn get_type(c: char) -> Type{
    match c {
        c if c.is_ascii_digit() || c == '.' => Type::Number,
        c if c.is_alphabetic() => Type::Letter,
        '+' | '-' | '*' | '/' | '^' | '@' => Type::Operation,
        '!' => Type::Factorial,
        '?' => Type::Termial,
        '(' => Type::BrOpen,
        ')' => Type::BrClosed,
        '#' => Type::Variable,
        _ => Type::Undefined,
    }
}

impl StackingVec {
    fn unwrap_sv_string(self) -> String {
        match self {
            StackingVec::String(string) => string.clone(),
            StackingVec::Vector(vec) => panic!("Called unwrap_sv_string on Vec variant. This is the vector {:?}", vec),
        }
    }
    
    // fn unwrap_sv_vec(self) -> Vec<StackingVec> {
    //     match self {
    //         StackingVec::String(_) => panic!("Called unwrap_sv_vec on String variant"),
    //         StackingVec::Vector(vec) => *vec.clone(),
    //     }
    // }
}




// fn eval_str(equation: &str) -> &str {
//     let res: Decimal = eval_as_decimal(equation);
//     let res_string: String = Decimal::into(res).to_string();
//     &res_string
// }

// fn eval_as_decimal(equation: &str) -> Decimal {
//     let vec_str: Vec<&str> = layerize(equation);
//     let vec_dec: Vec<Decimal> = particlelize(vec_str);
// }

pub fn eval(equation: &str, deg: bool, custom_vars: Vec<Variable>, custom_func: Vec<Function>) -> String { // i'd change the parameter type from vec to a hashmap?? (kinda depends on the implementation) // equation: the equation that should be evaluated; deg: whether the degrees should be used to solve trigonometric functions; custom_vars: custom variables added by the user
    let mut vars: Vec<Variable> = custom_vars;
    vars.push(Variable{name: String::from("e"), val: FiLong::e()});
    vars.push(Variable{name: String::from("pi"), val: FiLong::pi()});
    let mut funcs: Vec<Function> = base_funcs();
    funcs.extend(custom_func);
    let mut final_vars = HashMap::new();
    let injected: String = inject(equation, deg, &vars, &funcs, &mut final_vars);
    funcs.push(Function{name: String::from("deg_to_rad"), fnc: deg_to_rad, special: 0});
    funcs.push(Function{name: String::from("rad_to_deg"), fnc: rad_to_deg, special: 0});
    let layerized: StackingVec = layerize(&injected);
    let cleaned: StackingVec = clean(layerized);
    // println!("{:?}", cleaned);
    let particlelized: Particle = particlelize(cleaned);
    // println!("{:?}", particlelized);
    let res: FiLong = realize(particlelized, &final_vars,  &funcs);
    res.to_string()
}

fn inject(equation: &str, deg: bool, custom_vars: &Vec<Variable>, custom_func: &Vec<Function>, final_vars: &mut HashMap<u64, Variable>) -> String { // custom_func: ??
    let digits = (custom_vars.len() + custom_func.len()).ilog10(); // it's actually one lower since we do not need zeros if the number has the maximum number of digits
    let mut string = equation.to_string();
    let mut counter: u64 = 1;
    let mut funcs: HashMap<u64, Function> = HashMap::new();
    let queue: Vec<Input> = transform(custom_vars, custom_func);
    let mut zeros = String::new();
    for _ in 0..digits {
        zeros += "0";
    }
    let mut base = 10;
    for el in queue.iter().rev() {
        string = match el {
            Input::Fnc(function) => {
                funcs.insert(counter, function.clone()); 
                if deg {
                    match function.special {
                        1 => string.replace(&function.name, &format!("deg_to_rad(#{}{})", zeros, counter)),
                        2 => string.replace(&function.name, &format!("rad_to_deg(#{}{})", zeros, counter)),
                        _ => string.replace(&function.name, &format!("#{}{}", zeros, counter)),
                    }
                } else {
                    string.replace(&function.name, &format!("#{}{}", zeros, counter))
                }
                
            },
            Input::Var(variable) => {
                final_vars.insert(counter, variable.clone()); 
                string.replace(&variable.name, &format!("#{}{}", zeros, counter))
                
            },
        };
        counter += 1;
        if counter >= base {
            base *= 10;
            zeros.pop();
        }
    }
    for el in funcs.keys().into_iter() {
        let num_zeros = digits - el.ilog10(); // maximum number of zeros that can be added - the number of zeros it doesn't need because it of a significant size
        let mut zeros = String::new();
        for _ in 0..num_zeros {
            zeros += "0";
        }
        string = string.replace(&format!("#{}{}", zeros, *el), &funcs[el].name);
    }
    string
}

fn transform(custom_vars: &Vec<Variable>, custom_func: &Vec<Function>) -> Vec<Input> {
    let mut vec = Vec::new();
    let mut hashmap: HashMap<usize, Vec<Input>> = HashMap::new();
    for var in custom_vars {
        if let Some(val) = hashmap.get_mut(&var.name.len()) {
            val.push(Input::Var(var.clone()));
        } else {
            hashmap.insert(var.name.len(), vec![Input::Var(var.clone())]);
        }
    }
    for func in custom_func {
        if let Some(val) = hashmap.get_mut(&func.name.len()) {
            val.push(Input::Fnc(func.clone()));
        } else {
            hashmap.insert(func.name.len(), vec![Input::Fnc(func.clone())]);
        }
    }
    let mut keys_sorted = hashmap.keys().collect::<Vec<&usize>>();
    keys_sorted.sort();
    for key in keys_sorted.iter() {
        for el in hashmap[key].iter() {
            vec.push(el.clone());
        }
    }
    vec
}

macro_rules! convert_method_func_par_1 {
    ($name: ident) => {
        fn $name(pars: Vec<FiLong>) -> FiLong {
            if pars.len() > 1 {
                panic!("This function accepts only one input parameter.")
            } else {
                (&pars[0]).$name()
            }
        }
    };
}

macro_rules! convert_method_func_par_2 {
    ($name: ident) => {
        fn $name(pars: Vec<FiLong>) -> FiLong {
            if pars.len() != 2 {
                panic!("Make sure the function receive exactly TWO input parameters.")
            } else {
                (&pars[0]).$name(&pars[1])
            }
        }
    };
}

fn abs(pars: Vec<FiLong>) -> FiLong {
    if pars.len() > 1 {
        panic!("This function accepts only one input parameter.")
    } else {
        pars[0].clone().abs()
    }
}

fn neg(pars: Vec<FiLong>) -> FiLong {
    if pars.len() > 1 {
        panic!("This function accepts only one input parameter.")
    } else {
        -pars[0].clone()
    }
}

fn round_n(pars: Vec<FiLong>) -> FiLong {


    if pars.len() > 1 {
        panic!("This function accepts only one input parameter.")
    } else {
        let acc = pars[1].parse::<usize>();
        match acc {
            Ok(val) => (&pars[0]).round_n(val),
            Err(e) => panic!("{}", e.msg()),
        }
        
    }
}

fn deg_to_rad(deg: Vec<FiLong>) -> FiLong {
    if deg.len() > 1 {
        panic!("This function accepts only one input parameter.")
    } else {
        deg[0].clone() * FiLong::pi() / FiLong::from(180)
    } 
}

fn rad_to_deg(rad: Vec<FiLong>) -> FiLong {
    if rad.len() > 1 {
        panic!("This function accepts only one input parameter.")
    } else {
        rad[0].clone() * FiLong::from(180) / FiLong::pi()
    } 
}

fn base_funcs() -> Vec<Function> {
    convert_method_func_par_1!(sin);
    convert_method_func_par_1!(cos);
    convert_method_func_par_1!(tan);
    convert_method_func_par_1!(arcsin);
    convert_method_func_par_1!(arccos);
    convert_method_func_par_1!(arctan);
    convert_method_func_par_1!(sinh);
    convert_method_func_par_1!(cosh);
    convert_method_func_par_1!(tanh);
    convert_method_func_par_1!(coth);
    convert_method_func_par_1!(sech);
    convert_method_func_par_1!(csch);
    convert_method_func_par_1!(arcsinh);
    convert_method_func_par_1!(arccosh);
    convert_method_func_par_1!(arctanh);
    convert_method_func_par_1!(arccoth);
    convert_method_func_par_1!(arcsech);
    convert_method_func_par_1!(arccsch);
    convert_method_func_par_1!(cot);
    convert_method_func_par_1!(sec);
    convert_method_func_par_1!(csc);
    convert_method_func_par_1!(arccot);
    convert_method_func_par_1!(arcsec);
    convert_method_func_par_1!(arccsc);
    convert_method_func_par_1!(versin);
    convert_method_func_par_1!(coversin);
    convert_method_func_par_1!(vercos);
    convert_method_func_par_1!(covercos);
    convert_method_func_par_1!(exsec);
    convert_method_func_par_1!(excsc);

    convert_method_func_par_1!(log2);
    convert_method_func_par_1!(ln);
    convert_method_func_par_1!(log10);
    convert_method_func_par_1!(fact);
    convert_method_func_par_1!(term);
    convert_method_func_par_1!(sqrt);
    convert_method_func_par_1!(pot);
    convert_method_func_par_1!(exp);
    convert_method_func_par_1!(round);
    convert_method_func_par_1!(ceil);
    convert_method_func_par_1!(floor);


    convert_method_func_par_2!(log);
    convert_method_func_par_2!(pow_int);
    convert_method_func_par_2!(pow_r);
    convert_method_func_par_2!(pow);
    convert_method_func_par_2!(root);

    let mut funcs: Vec<Function> = Vec::new();
    funcs.push(Function{name: String::from("sin"), fnc: sin, special: 1});
    funcs.push(Function{name: String::from("cos"), fnc: cos, special: 1});
    funcs.push(Function{name: String::from("tan"), fnc: tan, special: 1});
    funcs.push(Function{name: String::from("arcsin"), fnc: arcsin, special: 2});
    funcs.push(Function{name: String::from("arccos"), fnc: arccos, special: 2});
    funcs.push(Function{name: String::from("arctan"), fnc: arctan, special: 2});
    funcs.push(Function{name: String::from("sinh"), fnc: sinh, special: 1});
    funcs.push(Function{name: String::from("cosh"), fnc: cosh, special: 1});
    funcs.push(Function{name: String::from("tanh"), fnc: tanh, special: 1});
    funcs.push(Function{name: String::from("coth"), fnc: coth, special: 1});
    funcs.push(Function{name: String::from("sech"), fnc: sech, special: 1});
    funcs.push(Function{name: String::from("csch"), fnc: csch, special: 1});
    funcs.push(Function{name: String::from("arcsinh"), fnc: arcsinh, special: 2});
    funcs.push(Function{name: String::from("arccosh"), fnc: arccosh, special: 2});
    funcs.push(Function{name: String::from("arctanh"), fnc: arctanh, special: 2});
    funcs.push(Function{name: String::from("arccoth"), fnc: arccoth, special: 2});
    funcs.push(Function{name: String::from("arcsech"), fnc: arcsech, special: 2});
    funcs.push(Function{name: String::from("arccsch"), fnc: arccsch, special: 2});
    funcs.push(Function{name: String::from("cot"), fnc: cot, special: 1});
    funcs.push(Function{name: String::from("sec"), fnc: sec, special: 1});
    funcs.push(Function{name: String::from("csc"), fnc: csc, special:1});
    funcs.push(Function{name: String::from("arccot"), fnc: arccot, special: 2});
    funcs.push(Function{name: String::from("arcsec"), fnc: arcsec, special: 2});
    funcs.push(Function{name: String::from("arccsc"), fnc: arccsc, special: 2});
    funcs.push(Function{name: String::from("versin"), fnc: versin, special: 2});
    funcs.push(Function{name: String::from("coversin"), fnc: coversin, special: 1});
    funcs.push(Function{name: String::from("vercos"), fnc: vercos, special: 1});
    funcs.push(Function{name: String::from("covercos"), fnc: covercos, special: 1});
    funcs.push(Function{name: String::from("exsec"), fnc: exsec, special: 1});
    funcs.push(Function{name: String::from("excsc"), fnc: excsc, special: 1});

    funcs.push(Function{name: String::from("log2"), fnc: log2, special: 0});
    funcs.push(Function{name: String::from("ln"), fnc: ln, special: 0});
    funcs.push(Function{name: String::from("log10"), fnc: log10, special: 0});
    funcs.push(Function{name: String::from("fact"), fnc: fact, special: 0});
    funcs.push(Function{name: String::from("term"), fnc: term, special: 0});
    funcs.push(Function{name: String::from("sqrt"), fnc: sqrt, special: 0});
    funcs.push(Function{name: String::from("pot"), fnc: pot, special: 0});
    funcs.push(Function{name: String::from("exp"), fnc: exp, special: 0});
    funcs.push(Function{name: String::from("round"), fnc: round, special: 0});
    funcs.push(Function{name: String::from("ceil"), fnc: ceil, special: 0});
    funcs.push(Function{name: String::from("floor"), fnc: floor, special: 0});
    funcs.push(Function{name: String::from("neg"), fnc: neg, special: 0});

    funcs.push(Function{name: String::from("log"), fnc: log, special: 0});
    funcs.push(Function{name: String::from("pow_int"), fnc: pow_int, special: 0});
    funcs.push(Function{name: String::from("pow_r"), fnc: pow_r, special: 0});
    funcs.push(Function{name: String::from("pow"), fnc: pow, special: 0});
    funcs.push(Function{name: String::from("root"), fnc: root, special: 0});
    funcs.push(Function{name: String::from("abs"), fnc: abs, special: 0});
    funcs.push(Function{name: String::from("round_n"), fnc: round_n, special: 0});
    funcs
}

fn layerize(equation: &str) -> StackingVec {
    let clean_vec: Vec<char> = equation
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect();
    // println!("{:?}", clean_vec);
    let strings = merge(&clean_vec);
    let merged_vec: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    // println!("{:?}", merged_vec);
    let stacking_vec: StackingVec = convert_stacking(merged_vec);
    // println!("{:?}", stacking_vec);
    let bracketless: StackingVec = layer_at_bracket(stacking_vec);
    // println!("{:?}", bracketless);
    let functionless: StackingVec = layer_at_fn(bracketless);
    // println!("{:?}", functionless);
    let exponentialless: StackingVec = layer_at_exp(functionless);
    // println!("{:?}", exponentialless);
    let factorialless: StackingVec = layer_at_fact(exponentialless);
    // println!("{:?}", factorialless);
    let moduloless: StackingVec = layer_at_mod(factorialless);
    // println!("{:?}", moduloless);
    let multipilicativeless: StackingVec = layer_at_mplv(moduloless);
    // println!("{:?}", multipilicativeless);
    let subtractionless: StackingVec = layer_at_neg(multipilicativeless);
    // println!("{:?}", subtractionless);
    let additionless: StackingVec = layer_at_add(subtractionless);
    // println!("{:?}", additionless);
    additionless
}


fn clean(sv: StackingVec) -> StackingVec {
    match sv {
        StackingVec::String(string) => return StackingVec::String(string),
        StackingVec::Vector(vec) => {
            match vec.len() {
                1 => clean(vec[0].clone()),
                _ => {
                    let mut return_vec: Vec<StackingVec> = Vec::new();
                    for el in vec.iter() {
                        return_vec.push(clean(el.clone()));
                    }
                    StackingVec::Vector(Box::new(return_vec))
                },
            }
        },
    }
    
}






// remove custom_func
fn particlelize(sv: StackingVec) -> Particle { // replace with hashmap
    // println!("sv: {:?}", sv);
    match sv {
        StackingVec::String(string) => Particle::Num(Box::new(replace_var(string))), // replace variables
        StackingVec::Vector(vec) => {
            let len = vec.len();
            match len {
                2 => { // cleanup
                    let first = vec[0].clone();
                    let second = vec[1].clone();
                    match second {
                        StackingVec::String(string) => {
                            if string == "!".to_string() {
                                return Particle::Fnc(Box::new(Action{fnc: String::from("fact"), x: Box::new(particlelize(first))}));
                            } else if string == "?".to_string() {
                                return Particle::Fnc(Box::new(Action{fnc: String::from("term"), x: Box::new(particlelize(first))}));
                            } else {
                                let first_val = first.unwrap_sv_string();
                                if first_val == String::from("-") {
                                    return Particle::Fnc(Box::new(Action{fnc: String::from("neg"), x: Box::new(Particle::Num(Box::new(replace_var(string))))}));
                                } else {
                                    return Particle::Fnc(Box::new(Action{fnc: first_val, x: Box::new(Particle::Num(Box::new(replace_var(string))))}));
                                }
                            }
                            
                        },
                        _ => {
                            let first_val = first.unwrap_sv_string();
                            if first_val == String::from("-") {
                                return Particle::Fnc(Box::new(Action{fnc: String::from("neg"), x: Box::new(particlelize(second))}));
                            } else {
                                return Particle::Fnc(Box::new(Action{fnc: first_val, x: Box::new(particlelize(second))}));
                            }
                        },
                    }
                },
                3 => {
                    match vec[1].clone().unwrap_sv_string() {
                        val if val == "+".to_string() => Particle::Res(Box::new(Atom{left: Box::new(particlelize(vec[0].clone())), right: Box::new(particlelize(vec[2].clone())), op: Operation::Addition})),
                        val if val == "*".to_string() => Particle::Res(Box::new(Atom{left: Box::new(particlelize(vec[0].clone())), right: Box::new(particlelize(vec[2].clone())), op: Operation::Multiplication})),
                        val if val == "/".to_string() => Particle::Num(Box::new(Number::Frc(Box::new(Fraction{numerator: Box::new(particlelize(vec[0].clone())), denominator:Box::new(particlelize(vec[2].clone()))})))),
                        val if val == "%".to_string() => Particle::Res(Box::new(Atom{left: Box::new(particlelize(vec[0].clone())), right: Box::new(particlelize(vec[2].clone())), op: Operation::Modulu})),
                        val if val == "^".to_string() => Particle::Res(Box::new(Atom{left: Box::new(particlelize(vec[0].clone())), right: Box::new(particlelize(vec[2].clone())), op: Operation::Exponentiation})),
                        val if val == "@".to_string() => Particle::Res(Box::new(Atom{left: Box::new(particlelize(vec[0].clone())), right: Box::new(particlelize(vec[2].clone())), op: Operation::Logarithm})),
                        _ => Particle::Num(Box::new(Number::Num(FiLong::new()))),
                    }
                }
                _ => Particle::Num(Box::new(Number::Num(FiLong::new()))),
            }
        }
    }
}

// fn dec_to_frac(dec: Decimal) -> Fraction {
    
// }

// fn dec(string: &str) -> FiLong {
//     let parts = string.split(".").collect::<Vec<&str>>();
//     if parts.len() == 1 {
//         Decimal::new(string.parse::<i64>().unwrap(), 0)
//     } else {
//         let decimals: u32 = parts[1].chars().count().try_into().unwrap();
//         let int: i64 = parts.join("").parse::<i64>().unwrap();
//         Decimal::new(int, decimals)
//     }
// }

fn replace_var(string: String) -> Number {
    if is_numeric(string.clone()) {
        Number::Num(FiLong::from(string))
    } else {
        let mut chars = string.chars();
        chars.next();
        let num_var = chars.as_str().parse::<usize>();
        match num_var {
            Ok(val) => Number::Var(val),
            Err(_) => panic!{"The sequence '{:?}' could not be parsed into a number. It tried to crete a variable out of the sequence. Make sure that you don't use the hashtag '#' in any form in the equation", chars.as_str()},
        }
        
    }

}

fn is_numeric(string: String) -> bool {
    for el in string.chars() {
        match el {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' | '.' => (),
            _ => return false,
        }
    }
    true
}

// fn is_var(string: String) -> bool {
//     match string.chars().next() {
//         Some(c) => {
//             if c == '#' {
//                 true
//             } else {
//                 false
//             }
//         }
//         None => false
//     }

// }

fn realize(p: Particle, custom_vars: &HashMap<u64, Variable>, functions: &Vec<Function>) -> FiLong {
    match p {
        Particle::Res(atom) => {
            match atom.op {
                Operation::Addition => realize(*atom.left, custom_vars, functions) + realize(*atom.right, custom_vars, functions),
                Operation::Subtraction => realize(*atom.left, custom_vars, functions) - realize(*atom.right, custom_vars, functions),
                Operation::Multiplication => realize(*atom.left, custom_vars, functions) * realize(*atom.right, custom_vars, functions),
                Operation::Division => realize(*atom.left, custom_vars, functions) / realize(*atom.right, custom_vars, functions),
                Operation::Modulu => realize(*atom.left, custom_vars, functions) % realize(*atom.right, custom_vars, functions),
                Operation::Exponentiation => realize(*atom.left, custom_vars, functions).pow(realize(*atom.right, custom_vars, functions)),
                Operation::Logarithm => realize(*atom.left, custom_vars, functions).log(realize(*atom.right, custom_vars, functions)),

            }
        },
        Particle::Fnc(action) => get_function(functions, action.fnc)(vec![realize(*action.x, custom_vars, functions)]),
        Particle::Num(num) => return parse_num(*num, custom_vars, functions),
    }
}

fn get_function(functions: &Vec<Function>, desired: String) -> fn(Vec<FiLong>) -> FiLong {
    for function in functions.iter() {
        if function.name == desired {
            return function.fnc;
        }
    }
    panic!("No function with the name {:?} was provided. Only provided functions can be used in evaluation", desired)
}


fn parse_num(num: Number, custom_vars: &HashMap<u64, Variable>, functions: &Vec<Function>) -> FiLong {
    match num {
        Number::Frc(fraction) => as_quotient(*fraction, custom_vars, functions),
        Number::Num(number) => number,
        Number::Var(var) => custom_vars[&(var as u64)].val.clone(),
    }
}

fn as_quotient(frc: Fraction, custom_vars: &HashMap<u64, Variable>, functions: &Vec<Function>) -> FiLong {
    realize(*frc.denominator, custom_vars, functions) / realize(*frc.numerator, custom_vars, functions)
}











fn count_recursive(sv: &StackingVec) -> usize {
    match sv {
        StackingVec::String(_) => 1,
        StackingVec::Vector(boxed_vec) => {
            2 + boxed_vec.iter().map(count_recursive).sum::<usize>()
        }
    }
    
}

fn convert_stacking(vec: Vec<&str>) -> StackingVec {
    let mut return_vec: Vec<StackingVec> = Vec::new();
    for el in vec.iter() {
        return_vec.push(StackingVec::String(el.to_string()));
    }
    StackingVec::Vector(Box::new(return_vec))
}

fn merge(vec: &Vec<char>) -> Vec<String> {
    let mut substr: String = String::new();
    let mut char_type: bool = true; // true: Type::Number; false: Type::Letter
    let mut var_sub: String = String::new();
    let mut new_vec: Vec<String> = Vec::new();
    let mut var: bool = false;
    for window in vec.windows(2) {
        let type1: Type = get_type(window[0]);
        let type2: Type = get_type(window[1]);
        let string: String = window[0].to_string();
        // if type1 == Type::Variable {
        //     if !var_sub.is_empty() { 
        //         new_vec.push(var_sub); 
        //         var_sub = String::new(); 
        //     }
        //     var = true;
        //     var_sub.push('#');
        // } 
        if var {
            match type1 {
                Type::Number => var_sub.push(window[0]),
                Type::Variable => {
                    if !var_sub.is_empty() { 
                        new_vec.push(var_sub); 
                        var_sub = String::new(); 
                    }
                    var_sub.push('#');
                },
                _ => {
                    new_vec.push(var_sub);
                    var_sub = String::new();
                    var = false;
                    new_vec.push(string);
                },
            }
        } else {
            match type1 {
                Type::Number => {
                    char_type = true;
                    if !char_type {
                        new_vec.push(substr.clone());
                        substr = String::new();
                    }
                    substr.push(window[0]);
                    if type2 != Type::Operation && type2 != Type::BrClosed && type2 != Type::Number && type2 != Type::Factorial{
                        new_vec.push(substr.clone());
                        new_vec.push("*".to_string());
                        substr = String::new(); 
                        
                    }
                    
                },
                Type::Letter => {
                    char_type = false;
                    if char_type {
                        new_vec.push(substr.clone());
                        substr = String::new();
                        char_type ^= true;
                    } 
                    substr.push(window[0])
                },
                Type::Operation => {
                    if !substr.is_empty() { 
                        new_vec.push(substr.clone()); 
                        substr = String::new(); 
                    }
                    new_vec.push(string);
                },
                Type::Factorial => {
                    if !substr.is_empty() { 
                        new_vec.push(substr.clone()); 
                        substr = String::new(); 
                    }
                    new_vec.push(string);
                },
                Type::Termial => {
                    if !substr.is_empty() { 
                        new_vec.push(substr.clone()); 
                        substr = String::new(); 
                    }
                    new_vec.push(string);
                },
                Type::BrOpen => {
                    if !substr.is_empty() { 
                        new_vec.push(substr.clone()); 
                        substr = String::new(); 
                    }
                    new_vec.push(string);
                },
                Type::BrClosed => {
                    if !substr.is_empty() { 
                        new_vec.push(substr.clone()); 
                        substr = String::new(); 
                    }
                    new_vec.push(string);
                    if type2 != Type::Operation && type2 != Type::BrClosed && type2 != Type::Factorial {
                        new_vec.push("*".to_string());
                    }
                },
                Type::Variable => {
                    if !var_sub.is_empty() { 
                        new_vec.push(var_sub); 
                        var_sub = String::new(); 
                    }
                    var = true;
                    var_sub.push('#');
                },
                Type::Undefined => {
                    if !substr.is_empty() { 
                        new_vec.push(substr.clone()); 
                        substr = String::new(); 
                    }
                    new_vec.push(string);
                },
               
            }
        }
    }
    let last;
    match vec.last() {
        Some(el) => last = *el,
        None => return new_vec,
    } 
    match get_type(last) {
        Type::Number => {
            if var {
                var_sub.push(last);
                new_vec.push(var_sub); 
            } else {
                if char_type {
                    substr.push(last);
                    new_vec.push(substr.clone()); 
                } else {
                    if !substr.is_empty() {
                        new_vec.push(substr.clone()); 
                    }
                    new_vec.push(last.to_string()); 
                }
            }
        },
        Type::Letter => {
            if var {
                new_vec.push(var_sub); 
                new_vec.push(last.to_string()); 
            } else {
                if !char_type {
                    substr.push(last);
                    new_vec.push(substr.clone()); 
                } else {
                    if !substr.is_empty() {
                        new_vec.push(substr.clone()); 
                    }
                    new_vec.push(last.to_string()); 
                }
            }
        },
        Type::Factorial => {
            if !var_sub.is_empty() {
                new_vec.push(var_sub.clone());
            }
            new_vec.push("!".to_string());
        },
        Type::Termial => {
            if !var_sub.is_empty() {
                new_vec.push(var_sub.clone());
            }
            new_vec.push("?".to_string());
        }
        Type::BrClosed | Type::Undefined => {
            if !substr.is_empty() { 
                new_vec.push(substr.clone()); 
            }
            new_vec.push(last.to_string());
        },
        _ => (),

    
    }
    
    new_vec
}

fn layer_at_bracket(sv: StackingVec) -> StackingVec {
    let sep1 = "(";
    let sep2 = ")";
    let mut return_vec: Vec<StackingVec> = Vec::new();
    let mut forbidden = Vec::new();
    match sv {
        StackingVec::String(ref string) => StackingVec::String(string.clone()),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if forbidden.contains(&i) {
        
                } else if el.clone().unwrap_sv_string() == sep1.to_string() {
                    let output = layer_at_bracket(StackingVec::Vector(Box::new(vec[i+1..].to_vec())));
                    let mut count = 0;
                    if let StackingVec::Vector(boxed_vec) = output.clone() {
                        let vec = *boxed_vec;
                        
                        for el in vec {
                            count += count_recursive(&el);
                        }
                        
                    }
                    forbidden = (i..i+count+2).collect();
                    return_vec.push(output);
                } else if el.clone().unwrap_sv_string() == sep2.to_string() {
                    return StackingVec::Vector(Box::new(return_vec));
                } else {
                    return_vec.push(el.clone());
                }
            }
            StackingVec::Vector(Box::new(return_vec))
        }
    }
    
}

fn layer_at_fn(sv: StackingVec) -> StackingVec {
    let mut return_vec: Vec<StackingVec> = Vec::new();
    let mut skip: bool = false;
    match sv {
        StackingVec::String(string) => return StackingVec::String(string),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else {
                    match el {
                        StackingVec::String(string) => {
                            if string.clone().chars().all(|chr| chr.is_alphabetic()) {
                                if i < vec.len() - 1 {
                                    return_vec.push(StackingVec::Vector(Box::new(vec![StackingVec::String(string.clone()), layer_at_fn(vec[i + 1].clone())])));
                                    skip = true;
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(_) => return_vec.push(layer_at_fn(el.clone())),
                    }
                }
                

            }
        },
    }
    StackingVec::Vector(Box::new(return_vec))
}

fn layer_at_exp(sv: StackingVec) -> StackingVec {
    let mut return_vec: Vec<StackingVec> = Vec::new();
    let mut skip: bool = false;
    match sv {
        StackingVec::String(string) => return StackingVec::String(string),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else { 
                    match el {
                        StackingVec::String(string) => {
                            if ["^", "@"].contains(&string.as_str()) {
                                if i > 0 && i < vec.len() - 1 {
                                    let mut last = match return_vec.pop() {
                                        Some(val) => val,
                                        None => panic!("The vector couldn't be popped probably because the vector is empty"),
                                    };
                                    match last {
                                        StackingVec::String(ref string) => {
                                            if string == "!" {
                                                let prev = match return_vec.pop() {
                                                    Some(val) => val,
                                                    None => panic!("The vector couldn't be popped probably because the vector is empty"),
                                                };
                                                last = StackingVec::Vector(Box::new(vec![prev, StackingVec::String("!".to_string())]));
                                            } else if string == "?" {
                                                let prev = match return_vec.pop() {
                                                    Some(val) => val,
                                                    None => panic!("The vector couldn't be popped probably because the vector is empty"),
                                                };
                                                last = StackingVec::Vector(Box::new(vec![prev, StackingVec::String("?".to_string())]));
                                            }
                                        },
                                        StackingVec::Vector(_) => (),
                                    }
                                    return_vec.push(StackingVec::Vector(Box::new(vec![last, StackingVec::String(string.clone()), layer_at_exp(vec[i + 1].clone())])));
                                    skip = true;
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(_) => return_vec.push(layer_at_exp(el.clone())),
                    }
                    
                }
                

            }
        },
    }
    StackingVec::Vector(Box::new(return_vec))
}

fn layer_at_fact(sv: StackingVec) -> StackingVec {
    let mut return_vec: Vec<StackingVec> = Vec::new();
    // let mut skip: bool = false;
    match sv {
        StackingVec::String(string) => return StackingVec::String(string),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                match el {
                    StackingVec::String(string) => {
                        if "!" == string.to_string() || "?" == string.to_string() {
                            if i > 0 {
                                let last = match return_vec.pop() {
                                    Some(val) => val,
                                    None => panic!("The vector couldn't be popped probably because the vector is empty"),
                                };
                                return_vec.push(StackingVec::Vector(Box::new(vec![last, StackingVec::String(string.clone())])));
                            }
                        } else {
                            return_vec.push(el.clone());
                        }
                    },
                    StackingVec::Vector(_) => return_vec.push(layer_at_fact(el.clone())),
                }
                    
                

            }
        },
    }
    StackingVec::Vector(Box::new(return_vec))
}

fn layer_at_mod(sv: StackingVec) -> StackingVec {
    let mut return_vec: Vec<StackingVec> = Vec::new();
    let mut skip: bool = false;
    match sv {
        StackingVec::String(string) => return StackingVec::String(string),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else { 
                    match el {
                        StackingVec::String(string) => {
                            if "%" == string.to_string() {
                                if i > 0 && i < vec.len() - 1 {
                                    let last = match return_vec.pop() {
                                        Some(val) => val,
                                        None => panic!("The vector couldn't be popped probably because the vector is empty"),
                                    };
                                    return_vec.push(StackingVec::Vector(Box::new(vec![last, StackingVec::String(string.clone()), layer_at_mod(vec[i + 1].clone())])));
                                    skip = true;
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(_) => return_vec.push(layer_at_mod(el.clone())),
                    }
                    
                }
                

            }
        },
    }
    StackingVec::Vector(Box::new(return_vec))
}

fn layer_at_mplv(sv: StackingVec) -> StackingVec {
    let mut return_vec: Vec<StackingVec> = Vec::new();
    let mut skip: bool = false;
    match sv {
        StackingVec::String(string) => return StackingVec::String(string),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else { 
                    match el {
                        StackingVec::String(string) => {
                            if ["*", "/"].contains(&string.as_str()) {
                                if i > 0 && i < vec.len() - 1 {
                                    let last = match return_vec.pop() {
                                        Some(val) => val,
                                        None => panic!("The vector couldn't be popped probably because the vector is empty"),
                                    };
                                    return_vec.push(StackingVec::Vector(Box::new(vec![last, StackingVec::String(string.clone()), layer_at_mplv(vec[i + 1].clone())])));
                                    skip = true;
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(_) => return_vec.push(layer_at_mplv(el.clone())),
                    }
                    
                }
                

            }
        },
    }
    StackingVec::Vector(Box::new(return_vec))
}

fn layer_at_neg(sv: StackingVec) -> StackingVec {
    let mut return_vec: Vec<StackingVec> = Vec::new();
    let mut skip: bool = false;
    match sv {
        StackingVec::String(string) => return StackingVec::String(string),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else {
                    match el {
                        StackingVec::String(string) => {
                            if "-" == string.to_string() {
                                if i > 0 && i < vec.len() - 1 {
                                    return_vec.push(StackingVec::String("+".to_string()));
                                    return_vec.push(StackingVec::Vector(Box::new(vec![StackingVec::String(string.clone()), layer_at_neg(vec[i + 1].clone())])));
                                    skip = true;
                                } else if i == 0 {
                                    return_vec.push(StackingVec::String("-".to_string()));
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(_) => return_vec.push(layer_at_neg(el.clone())),
                    }
                }
                

            }
        },
    }
    StackingVec::Vector(Box::new(return_vec))
}

fn layer_at_add(sv: StackingVec) -> StackingVec {
    let mut return_vec: Vec<StackingVec> = Vec::new();
    let mut skip: bool = false;
    match sv {
        StackingVec::String(string) => return StackingVec::String(string),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else { 
                    match el {
                        StackingVec::String(string) => {
                            if "+" == string.to_string() {
                                if i > 0 && i < vec.len() - 1 {
                                    let last = match return_vec.pop() {
                                        Some(val) => val,
                                        None => panic!("The vector couldn't be popped probably because the vector is empty"),
                                    };
                                    return_vec.push(StackingVec::Vector(Box::new(vec![last, StackingVec::String(string.clone()),  layer_at_add(vec[i + 1].clone())])));
                                    skip = true;
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(_) => return_vec.push(layer_at_add(el.clone())),
                    }
                    
                }
                

            }
        },
    }
    StackingVec::Vector(Box::new(return_vec))
}
