/// Demonstrates core Rust concepts such as data types, control flow, pattern matching,
/// mutable vs immutable bindings, and closures.
pub fn core_concepts() {
    // Data types
    let _integer: f64 = 42.35;
    let boolean: bool = true;
    let _float: f64 = 3.14;
    let _tuple: (i32, f64, &str) = (1, 2.5, "Rust");
    let _array: [i32; 3] = [1, 2, 3];

    // Control flow
    if boolean {
        println!("Boolean is true");
    } else {
        println!("Boolean is false");
    }

    // Pattern matching
    let number: i32 = 2;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),
    }

    // Mutable vs Immutable Bindings
    let mut mutable: i32 = 10;

    mutable += 5;
    println!("Mutable value: {}", mutable);

    // Implicit returns
    let square: fn(i32) -> i32 = |x: i32| x * x;
    println!("Square of 4: {}", square(4));

    // Slices
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array[1..4];
    println!("Slice: {:?}", slice);
}
