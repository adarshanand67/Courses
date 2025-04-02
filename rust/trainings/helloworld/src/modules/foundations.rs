/// Prints a brief explanation of why Rust is a powerful programming language.
pub fn foundations() {
    println!("Why Rust? Memory safe, thread safe, low-level systems programming language.");
}

/// Demonstrates Rust's ownership model.
pub fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership is moved to s2
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This would cause a compile-time error
}
