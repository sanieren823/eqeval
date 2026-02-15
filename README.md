# eqeval

eqeval converts equations strings to their respective value using the finum crate. We therefore avoid any floating-point errors and are entirely reliant on the precision values provided by the [finum crate](https://github.com/sanieren823/finum.git).

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
eqeval = "0.2.0"
```

## Creating custom variables

Creating custom variables is done via the **Variable struct**. To create an instance of this struct you need:
- a name
- the value (as a FiLong)

## Creating custom functions

In order to create a custom function for the equation evaluator, you have use the **Function struct**.
Each function needs:
- a name
- the actual function (parameters: vector of FiLong; output type: FiLong)
- a special value (this value, usually 0, can provide additional context e. g. trigonometric functions)

|Special Value           |Function                                                 |
|------------------------|---------------------------------------------------------|
|0                       |Base case; no special handeling                          |
|1                       |For trig. function with parameters given in degrees      |
|2                       |For trig. function that return a value in degrees        |

## Potential Issues

- Make sure your custom functions do not include *rad_to_deg*/*deg_to_rad* in its entirety in the name
- Functions musn't include any of the following symbols: #, +, -, *, /, ^, @, !, ? (I did not include a check for that; if you're actually trying to use a operand's symbol in a function name that on you)

# Your first equation

Generally parsing equations is done by using the **eval! macro**. 
You can choose to provide additional context in form of the *deg boolean* or one/both of the two *context vectors* for variables and functions.

## How to write an equation

The Equation must always be provided as a **String**.
These are the symbols for the operands:

|Symbol           |Function                                                 |Example                                                    |
|-----------------|---------------------------------------------------------|-----------------------------------------------------------|
|+                |**Addition**                                             |2 + 4 &rarr; 6                                             |
|-                |**Subtraction** or **Negative Value**                    |7 - 3 &rarr; 4 or -5 &rarr; -5                              |
|*                |**Multiplication**                                       |12 * 4 &rarr; 48                                           |
|/                |**Division**                                             |39 / 13 &rarr; 13                                          |
|^                |**Exponentiation**                                       |2 ^ 8 &rarr; 256                                           |
|@                |**Logarithm**                                            |81 @ 3 &rarr; ${\log_3 81}$ &rarr; 4 |
|!                |**Factorial**                                            |3! &rarr; 1 * 2 * 3 &rarr; 6                               |
|?                |**Termial**                                              |6? &rarr; 1 + 2 + 3 + 4 + 5 + 6 &rarr; 21                  |

Generally there's no limit in equation size (aside from String limits in rust) or in context provided but it'll run faster for smaller equations/fewer context added

## How equations get parsed

The general **Order of Operations**: (lower number &rarr; parsed earlier):
1. parenthesis
2. functions (sin, abs)
3. exponentiation/logarithm
4. factorials/termials
5. modulu
6. multiplication/division
7. addition/subtraction

## An Example Equation

This is an example equation with variables as context:

``` rust
fn main() {
    let mut context: Vec<Variable> = Vec::new();
    context.push(Variable{name: String::from("re"), val: FiLong::two()});
    context.push(Variable{name: String::from("pl"), val: FiLong::six()});
    println!("{}", eval!("re! + (9^0.35) * 1^2!@8 - (4 + 5(3sin(0.213) - 9)) * abs(2.23) + pl? + 7.0cos(-31.9)(3)!^2", vars: context));
}

```
The result should be: **330.40574755336320971885**
