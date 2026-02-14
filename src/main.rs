use eqeval::Variable;
use eqeval::eval;
use finum::finum::FiLong;
fn main() {
    // let number = dec!(-1.23);
    let mut context: Vec<Variable> = Vec::new();
    context.push(Variable{name: String::from("re"), val: FiLong::two()});
    context.push(Variable{name: String::from("pl"), val: FiLong::six()});
    println!("{}", eval!("re! + (9^0.35) * 1^2!@8 - (4 + 5(3sin(0.213) - 9)) * abs(2.23) + pl? + 7.0cos(-31.9)(3)!^2", vars: context));
}











