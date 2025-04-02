/// A macro for performing arithmetic operations and printing the result.
macro_rules! calculate {
    ($a:expr, $b:expr, $op:tt) => {
        println!("Result: {}", $a $op $b);
    };
}

/// Demonstrates the use of macros in Rust.
pub fn macros_example() {
    calculate!(10, 5, +);
    calculate!(10, 5, *);
}
