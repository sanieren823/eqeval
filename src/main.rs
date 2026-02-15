use eqeval::Variable;
use eqeval::Function;
use eqeval::eval;
use finum::finum::FiLong;
fn main() {
    let mut context: Vec<Variable> = Vec::new();
    context.push(Variable{name: String::from("re"), val: FiLong::two()});
    context.push(Variable{name: String::from("pl"), val: FiLong::six()});
    let mut funcs: Vec<Function> = Vec::new();
    funcs.push(Function{name: String::from("new"), fnc: new, special: 0});
    println!("{}", eval!("re! + (9^0.35) * 1^2!@8 - (4 + 5(3sin(0.213) - 9)) * new(2.23, 3, 9, 16) + pl? + 7.0cos(-31.9)(3)!^2", vars: context, func: funcs));
}


fn new(par: Vec<FiLong>) -> FiLong {
    if par.len() != 4 {
        panic!("kasdöf")
    } else {
        par[0].clone() + par[1].clone() + par[2].clone() + par[3].clone()
    }
}








