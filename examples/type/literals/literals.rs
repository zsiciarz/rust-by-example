fn main() {
    // Suffixed literals, their types is known at initialization
    let x = 1u8;
    let y = 2u;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // Constraints (summands must have the same type) for `i` and `f`
    let _constraint_1 = x + i;
    let _constraint_2 = z + f;
    // TODO ^ Try commenting out these two lines
}
