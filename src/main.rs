use rust_decimal::prelude::*;
use std::str;
use std::vec;
use std::string;


// "23 - 7 * 3^4 / 4(17 - 5 * 3)"
// "4 * 3 % 9 + 13"


// general order of operations:
// 1. parenthesis
// 2. function (sin, abs)
// 3. exponentiation/logarithm
// 4. factorials
// 5. modulu
// 6. multiplication/division
// 7. addition/subtraction


// @ for logarithmic calculations??


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
    println!("{:?}", layerize("3 + 9 * 12 - (4 + 5(3sin(0.213) - 9)) * 2.23 + 7"));
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
    let clean_vec: Vec<char> = equation
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect();
    println!("{:?}", clean_vec);
    let strings = merge(&clean_vec);
    let merged_vec: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    println!("{:?}", merged_vec);
    let bracketless: StackingVec = layer_at_bracket(merged_vec);
    bracketless
}

fn layer_at_bracket(vec: Vec<&str>) -> StackingVec {
    let sep1 = "(";
    let sep2 = ")";
    let mut return_vec = Vec::new();
    let mut forbidden = Vec::new();
    for (i, el) in vec.iter().enumerate() {
        if forbidden.contains(&i) {

        } else if el == &sep1 {
            let output = layer_at_bracket(vec[i+1..].to_vec());
            let mut count = 0;
            if let StackingVec::Vector(boxed_vec) = output.clone() {
                let vec = *boxed_vec;
                
                for el in vec {
                    count += count_recursive(&el);
                }
                
            }
            println!("Count: {count}");
            forbidden = (i..i+count+2).collect();
            return_vec.push(output);
        } else if el == &sep2 {
            return StackingVec::Vector(Box::new(return_vec));
        } else {
            return_vec.push(StackingVec::String(el.to_string()));
            println!("{el}");
        }
    }
    return StackingVec::Vector(Box::new(return_vec));
}

fn count_recursive(sv: &StackingVec) -> usize {
    match sv {
        StackingVec::String(_) => 1,
        StackingVec::Vector(boxed_vec) => {
            2 + boxed_vec.iter().map(count_recursive).sum::<usize>()
        }
    }
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
        if type1 == Type::Variable {
            if !var_sub.is_empty() { 
                new_vec.push(var_sub); 
                var_sub = String::new(); 
            }
            var = true;
            var_sub.push('#');
        }
        if var {
            match type1 {
                Type::Number => var_sub.push(window[0]),
                _ => {
                    new_vec.push(var_sub);
                    var_sub = String::new();
                    var = false;
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
                    if type2 != Type::Operator && type2 != Type::BrClosed && type2 != Type::Number {
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
                    if type2 != Type::Operator && type2 != Type::BrClosed {
                        new_vec.push("*".to_string());
                    }
                },
                Type::Variable => (),
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
                    new_vec.push(substr.clone()); 
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
                    new_vec.push(substr.clone()); 
                    new_vec.push(last.to_string()); 
                }
            }
        },
        Type::BrClosed | Type::Undefined => new_vec.push(last.to_string()),
        _ => (),

    }
    new_vec
}

fn get_type(c: char) -> Type{
    match c {
        c if c.is_ascii_digit() || c == '.' => Type::Number,
        c if c.is_alphabetic() => Type::Letter,
        '+' | '-' | '*' | '/' | '^' | '@' => Type::Operator,
        '(' => Type::BrOpen,
        ')' => Type::BrClosed,
        '#' => Type::Variable,
        _ => Type::Undefined,
    }
}
// // might wanna change the type
// fn decimalize(vec_str: Vec<&str>) -> Vec<Decimal> {
//     let mut vec_dec: Vec<Decimal> = Vec::new();
// }

// fn dec_to_frac(dec: Decimal) -> Fraction {
    
// }
