/// Demonstrates Rust's ownership and borrowing rules, including move semantics,
/// immutable references, and mutable references.
pub fn ownership_and_borrowing() {
    let s1 = String::from("hello");
    let _s2 = s1; // s1 is moved to s2
                  // println!("{}", s1); // This would cause an error

    let s3 = String::from("world");
    let s4 = &s3; // Borrowing
    println!("Borrowed value: {}", s4);

    let mut s5 = String::from("mutable");
    let s6 = &mut s5; // Mutable reference
    s6.push_str(" string");
    println!("Modified value: {}", s6);
}
