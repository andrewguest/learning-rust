// Function types are EXPLICIT
// You MUST declare the type of the input variable

// Without the 'return' keyword, only the last expression is returned.


fn square(x: f64) -> f64 {
    return x * x;
}

fn main() {
    let number_to_square: f64 = 2.0;
    let result = square(number_to_square);
    println!("Square of {number_to_square} is: {result}")

    // closures (A.K.A anonymous or lambda functions)
    // input parameters are passed inside | | and expression body is wrapped with { }
    // { } are optional for single-lined lambdas (closures)
    let double = |i: i32| -> i32 {
        i * 2
    };
    println!("{}", double(10));
}
