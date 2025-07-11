/// A macro for performing arithmetic operations and printing the result.
macro_rules! calculate {
    ($a:expr, $b:expr, +) => {
        println!("Addition: {}", $a + $b);
    };
    ($a:expr, $b:expr, *) => {
        println!("Multiplication: {}", $a * $b);
    };
}

/// Demonstrates the use of macros in Rust.
pub fn macros_example() {
    let a: i32 = 10;
    let b: i32 = 5;
    calculate!(a, b, +);
    calculate!(a, b, *);
}
