use eqeval::Variable;
use eqeval::eval;
use finum::finum::FiLong;
fn main() {
    // let number = dec!(-1.23);
    let mut hashmap = Vec::new();
    hashmap.push(Variable{name: String::from("ser"), val: FiLong::two()});
    hashmap.push(Variable{name: String::from("heal"), val: FiLong::six()});
    println!("{:?}", eval!("ser! + (9^0.35) * 1^2!@8 - (4 + 5(3sin(0.213) - 9)) * abs(2.23) + heal? + 7.0cos(-31.9)(3)!^2", vars: hashmap));
    // println!("{:?}", eval!("sin(0.213)"));
    // println!("{:?}", eval!("cos(-31.9)"));
}











