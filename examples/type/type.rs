fn main() {
    // Type annotated variable
    let a_float: f64 = 1.0;

    // This variable is an `int`, the compiler can infer the type for us
    let mut an_integer = 5;

    // Error! The type of a variable can't be changed
    an_integer = true;
}
