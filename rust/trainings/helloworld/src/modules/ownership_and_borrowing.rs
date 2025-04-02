/// Demonstrates Rust's ownership and borrowing rules, including move semantics,
/// immutable references, and mutable references.
pub fn ownership_and_borrowing() {
    let s1: String = String::from("hello");
    let _s2: String = s1; // s1 is moved to s2
    // println!("{}", s1); // This would cause an error

    let s3: String = String::from("world");
    let s4: &String = &s3; // Borrowing
    println!("Borrowed value: {}", s4);

    let mut s5: String = String::from("mutable");
    let s6: &mut String = &mut s5; // Mutable reference
    s6.push_str(" string");
    println!("Modified value: {}", s6);
}
