use rust_decimal::prelude::*;
use std::str;
use std::vec;


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

#[derive(Debug, Clone)]
enum StackingVec {
    String(String),
    Vector(Box<Vec<StackingVec>>),
}


fn main() {
    println!("Hello, world!");
    let number = dec!(-1.23);
    println!("{:?}", layerize("3 + 9 * 12 - (4 + 5(3(4/2) - 9)) * 2 + 7"));
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
    let tokens: Vec<String> = equation
    .chars()
    .filter(|c| !c.is_whitespace())
    .map(|c| c.to_string())
    .collect();
    let clean_vec: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    println!("{:?}", clean_vec);
    let bracketless: StackingVec = layer_at_bracket(clean_vec);
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

// // might wanna change the type
// fn decimalize(vec_str: Vec<&str>) -> Vec<Decimal> {
//     let mut vec_dec: Vec<Decimal> = Vec::new();
// }

// fn dec_to_frac(dec: Decimal) -> Fraction {
    
// }
