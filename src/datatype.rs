/// Static keyword is used to define a "global variable" type facility.

/// 2. Variable Shadowing
/// Rust allows use to reclare the same variable name with different data type.
pub(crate) fn variable_shadow(){
    let x: f64 = -26.48;
    let x: i64 = x.floor() as i64;

    print!("{}", x);
    print!("{}", x as i64);
}