use rust_decimal::prelude::*;
use std::str;
use std::vec;
use std::string;


// "23 - 7 * 3^4 / 4(17 - 5 * 3)"
// "4 * 3 % 9 + 13"


// general order of operations:
// 1. parenthesis
// 2. functions (sin, abs)
// 3. exponentiation/logarithm
// 4. factorials
// 5. modulu
// 6. multiplication/division
// 7. addition/subtraction


// @ for logarithmic calculations??
// "3^4@8.9^3"
// "sin()"


// struct Atom {
//     left: Particle,
//     right: Particle,
//     op: Operator,
// }

// // all calculations should be done with fractions (if at least one division/fraction is in the equation)
// struct Fraction {
//     numerator: Particle,
//     denominator: Particle,  
// }

// enum Particle {
//     Num(Number),
//     Fnc(Function, Vec<Decimal>),
//     Res(Atom),
// }

// enum Operator {
//     Addition,
//     Subtraction, // do we even need subtraction?
//     Multiplication,
//     Division,
//     Modulu,
//     Exponentiation,
//     Logarithm

// }
// enum Number {
//     Frc(Fraction),
//     Num(Decimal),
// }

// enum Function {
//     Sine,
//     Cosine,
//     Tangent,
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Type {
    Number,
    Letter, 
    Operator,
    Factorial,
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


fn main() {
    println!("Hello, world!");
    let number = dec!(-1.23);
    println!("{:?}", layerize("#32! + (9^0.35) * 1^2@8 - (4 + 5(3sin(0.213) - 9)) * abs(2.23) + #0 + 7.0cos(-31.9)(3)!"));
}


// fn eval_str(equation: &str) -> &str {
//     let res: Decimal = eval_as_decimal(equation);
//     let res_string: String = Decimal::into(res).to_string();
//     &res_string
// }

// fn eval_as_decimal(equation: &str) -> Decimal {
//     let vec_str: Vec<&str> = layerize(equation);
//     let vec_dec: Vec<Decimal> = decimalize(vec_str);
// }


fn layerize(equation: &str) -> StackingVec {
    // replace variables
    let clean_vec: Vec<char> = equation
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect();
    println!("{:?}", clean_vec);
    let strings = merge(&clean_vec);
    let merged_vec: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    println!("{:?}", merged_vec);
    let stacking_vec: StackingVec = convert_stacking(merged_vec);
    println!("{:?}", stacking_vec);
    let bracketless: StackingVec = layer_at_bracket(stacking_vec);
    println!("{:?}", bracketless);
    let functionless: StackingVec = layer_at_fn(bracketless);
    println!("{:?}", functionless);
    let exponentialless: StackingVec = layer_at_exp(functionless);
    println!("{:?}", exponentialless);
    let factorialless: StackingVec = layer_at_fact(exponentialless);
    println!("{:?}", factorialless);
    let moduloless: StackingVec = layer_at_mod(factorialless);
    println!("{:?}", moduloless);
    let fully_split: StackingVec = layer_at_mplv(moduloless);
    println!("{:?}", fully_split);
    fully_split
}

// // might wanna change the type
// fn decimalize(vec_str: Vec<&str>) -> Vec<Decimal> {
//     let mut vec_dec: Vec<Decimal> = Vec::new();
// }

// fn dec_to_frac(dec: Decimal) -> Fraction {
    
// }







impl StackingVec {
    fn unwrap_sv_string(self) -> String {
        match self {
            StackingVec::String(string) => string.clone(),
            StackingVec::Vector(_) => panic!("Called unwrap_sv_string on Vec variant"),
        }
    }
    
    fn unwrap_sv_vec(self) -> Vec<StackingVec> {
        match self {
            StackingVec::String(_) => panic!("Called unwrap_sv_vec on String variant"),
            StackingVec::Vector(vec) => *vec.clone(),
        }
    }
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

// TODO: convert to parameter type: StackingVec
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
        StackingVec::String(String) => (),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else {
                    match el {
                        StackingVec::String(string) => {
                            if string.clone().chars().all(|chr| chr.is_alphabetic()) {
                                if i < vec.len() - 1 {
                                    return_vec.push(StackingVec::Vector(Box::new(vec![StackingVec::String(string.clone()), vec[i + 1].clone()])));
                                    skip = true;
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(vec) => return_vec.push(layer_at_fn(el.clone())),
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
        StackingVec::String(String) => (),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else { 
                    match el {
                        StackingVec::String(string) => {
                            if ["^", "@"].contains(&string.as_str()) {
                                if i > 0 && i < vec.len() - 1 {
                                    let last = return_vec.last().unwrap().clone();
                                    return_vec.pop();
                                    return_vec.push(StackingVec::Vector(Box::new(vec![last, StackingVec::String(string.clone()), vec[i + 1].clone()])));
                                    skip = true;
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(vecn) => return_vec.push(layer_at_exp(el.clone())),
                    }
                    
                }
                

            }
        },
    }
    StackingVec::Vector(Box::new(return_vec))
}

fn layer_at_fact(sv: StackingVec) -> StackingVec {
    let mut return_vec: Vec<StackingVec> = Vec::new();
    let mut skip: bool = false;
    match sv {
        StackingVec::String(String) => (),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                match el {
                    StackingVec::String(string) => {
                        if "!" == string.to_string() {
                            if i > 0 {
                                let last = return_vec.last().unwrap().clone();
                                return_vec.pop();
                                return_vec.push(StackingVec::Vector(Box::new(vec![last, StackingVec::String(string.clone())])));
                            }
                        } else {
                            return_vec.push(el.clone());
                        }
                    },
                    StackingVec::Vector(vec) => return_vec.push(layer_at_fact(el.clone())),
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
        StackingVec::String(String) => (),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else { 
                    match el {
                        StackingVec::String(string) => {
                            if "%" == string.to_string() {
                                if i > 0 && i < vec.len() - 1 {
                                    let last = return_vec.last().unwrap().clone();
                                    return_vec.pop();
                                    return_vec.push(StackingVec::Vector(Box::new(vec![last, StackingVec::String(string.clone()), vec[i + 1].clone()])));
                                    skip = true;
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(vec) => return_vec.push(layer_at_mod(el.clone())),
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
        StackingVec::String(String) => (),
        StackingVec::Vector(vec) => {
            for (i, el) in vec.iter().enumerate() {
                if skip {
                    skip = false;
                } else { 
                    match el {
                        StackingVec::String(string) => {
                            if ["*", "/"].contains(&string.as_str()) {
                                if i > 0 && i < vec.len() - 1 {
                                    let last = return_vec.last().unwrap().clone();
                                    return_vec.pop();
                                    return_vec.push(StackingVec::Vector(Box::new(vec![last, StackingVec::String(string.clone()), vec[i + 1].clone()])));
                                    skip = true;
                                }
                            } else {
                                return_vec.push(el.clone());
                            }
                        },
                        StackingVec::Vector(vec) => return_vec.push(layer_at_mplv(el.clone())),
                    }
                    
                }
                

            }
        },
    }
    StackingVec::Vector(Box::new(return_vec))
}

fn merge(vec: &Vec<char>) -> Vec<String> {
    let mut substr: String = String::new();;
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
                    if type2 != Type::Operator && type2 != Type::BrClosed && type2 != Type::Number && type2 != Type::Factorial{
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
                Type::Operator => {
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
                    if type2 != Type::Operator && type2 != Type::BrClosed && type2 != Type::Factorial {
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

fn get_type(c: char) -> Type{
    match c {
        c if c.is_ascii_digit() || c == '.' => Type::Number,
        c if c.is_alphabetic() => Type::Letter,
        '+' | '-' | '*' | '/' | '^' | '@' => Type::Operator,
        '!' => Type::Factorial,
        '(' => Type::BrOpen,
        ')' => Type::BrClosed,
        '#' => Type::Variable,
        _ => Type::Undefined,
    }
}

